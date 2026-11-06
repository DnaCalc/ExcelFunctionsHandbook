#!/usr/bin/env node
// Node 24+, no third-party runtime packages. Local by default; no hosting setup.
import { createServer } from "node:http";
import { Worker } from "node:worker_threads";
import { DatabaseSync } from "node:sqlite";
import { readFile, stat, realpath, mkdir, chmod } from "node:fs/promises";
import { resolve, join, extname, dirname, sep } from "node:path";
import { fileURLToPath } from "node:url";
import { randomUUID } from "node:crypto";

export class RateLimiter {
  constructor(limit, windowMs = 60000, maxClients = 10000) {
    this.limit = limit; this.windowMs = windowMs; this.maxClients = maxClients; this.clients = new Map();
  }
  take(key, now = Date.now()) {
    let slot = this.clients.get(key);
    if (!slot || now >= slot.until) {
      for (const [id, entry] of this.clients) if (now >= entry.until) this.clients.delete(id);
      if (this.clients.size >= this.maxClients) return false;
      slot = { count: 0, until: now + this.windowMs }; this.clients.set(key, slot);
    }
    return ++slot.count <= this.limit;
  }
}

export function evaluate(engine, request, timeoutMs = 3000) {
  return new Promise((resolveResult, reject) => {
    const worker = new Worker(new URL("eval-worker.mjs", import.meta.url), {
      workerData: { engine, request }, resourceLimits: { maxOldGenerationSizeMb: 48, stackSizeMb: 4 },
    });
    const timer = setTimeout(() => { worker.terminate(); reject(Object.assign(new Error("Calculation exceeded 3 seconds."), { status: 408 })); }, timeoutMs);
    function finish(callback, value) { clearTimeout(timer); worker.terminate(); callback(value); }
    worker.once("message", value => finish(resolveResult, value));
    worker.once("error", () => finish(reject, Object.assign(new Error("Engine worker failed."), { status: 503 })));
    worker.once("exit", code => { if (code !== 0) finish(reject, Object.assign(new Error("Engine worker stopped."), { status: 503 })); });
  });
}

async function body(req, maxBytes = 32768) {
  if (!(req.headers["content-type"] || "").startsWith("application/json")) throw Object.assign(new Error("Use Content-Type: application/json."), { status: 415 });
  let size = 0; const chunks = [];
  for await (const chunk of req) {
    size += chunk.length;
    if (size > maxBytes) { req.resume(); throw Object.assign(new Error("Request exceeds 32768 bytes."), { status: 413 }); }
    chunks.push(chunk);
  }
  try { return JSON.parse(Buffer.concat(chunks).toString("utf8")); }
  catch { throw Object.assign(new Error("Invalid JSON."), { status: 400 }); }
}

export async function createHandbookServer({ root, store, evalLimit = 30, contributionLimit = 5, maxConcurrent = 2, timeoutMs = 3000 } = {}) {
  root = await realpath(resolve(root || "site/build"));
  store = resolve(store || ".tmp/handbook/contributions.sqlite");
  if (store.startsWith(root + sep) || store === root) throw new Error("The private contribution store must be outside the public root.");
  await mkdir(dirname(store), { recursive: true, mode: 0o700 });
  const database = new DatabaseSync(store);
  await chmod(store, 0o600);
  database.exec("PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL; CREATE TABLE IF NOT EXISTS contributions (id TEXT PRIMARY KEY, created_at TEXT NOT NULL, status TEXT NOT NULL DEFAULT 'pending', function_id TEXT NOT NULL, kind TEXT NOT NULL, excel_build TEXT NOT NULL, details TEXT NOT NULL, contact TEXT NOT NULL)");
  const save = database.prepare("INSERT INTO contributions (id, created_at, function_id, kind, excel_build, details, contact) VALUES (?, ?, ?, ?, ?, ?, ?)");
  const catalogue = JSON.parse(await readFile(join(root, "api/index.json"), "utf8"));
  const ids = new Set(catalogue.rows.map(row => row.function_id));
  const evaluations = new RateLimiter(evalLimit);
  const contributions = new RateLimiter(contributionLimit);
  const globalLimit = new RateLimiter(120);
  const engine = join(root, "assets/onecalc");
  let active = 0;
  const mime = { ".html": "text/html; charset=utf-8", ".css": "text/css", ".js": "text/javascript", ".mjs": "text/javascript", ".json": "application/json", ".txt": "text/plain; charset=utf-8", ".wasm": "application/wasm", ".svg": "image/svg+xml" };
  const server = createServer(async (req, res) => {
    function json(status, data, extra = {}) {
      res.writeHead(status, { "Content-Type": "application/json", "Cache-Control": "no-store", ...extra }); res.end(JSON.stringify(data));
    }
    res.setHeader("X-Content-Type-Options", "nosniff");
    res.setHeader("Referrer-Policy", "no-referrer");
    try {
      const url = new URL(req.url, "http://localhost");
      const api = ["/api/eval", "/api/contributions"].includes(url.pathname);
      if (api) {
        if (req.method !== "POST") { json(405, { message: "Use POST." }, { Allow: "POST" }); return; }
        const origin = req.headers.origin;
        if (origin && origin !== `http://${req.headers.host}` && origin !== `https://${req.headers.host}`) { json(403, { message: "Cross-origin submissions are not allowed." }); return; }
        // Direct peer address only. A future proxy must enforce limits at its own edge;
        // untrusted X-Forwarded-For never selects a rate-limit bucket.
        const client = req.socket.remoteAddress;
        const limiter = url.pathname === "/api/eval" ? evaluations : contributions;
        if (!limiter.take(client) || !globalLimit.take("all")) { json(429, { message: "Too many requests. Try again in a minute." }, { "Retry-After": "60" }); return; }
        const data = await body(req);
        if (!data || typeof data !== "object" || Array.isArray(data)) { json(400, { message: "Request must be a JSON object." }); return; }
        if (url.pathname === "/api/eval") {
          if (active >= maxConcurrent) { json(503, { message: "Calculator busy. Try again shortly." }, { "Retry-After": "1" }); return; }
          active++;
          try {
            const response = await evaluate(engine, data, timeoutMs);
            json(response.status === "invalid-request" ? 400 : response.status === "engine-error" ? 503 : 200, response);
          } finally { active--; }
        } else {
          for (const [field, max] of [["function_id",100],["kind",30],["excel_build",200],["details",20000],["contact",320]]) {
            if (data[field] === undefined && ["excel_build","contact"].includes(field)) data[field] = "";
            if (typeof data[field] !== "string" || data[field].length > max) throw Object.assign(new Error(`Invalid ${field}.`), { status: 400 });
          }
          data.function_id = data.function_id.trim().toUpperCase();
          if (!data.function_id.startsWith("FUNC.")) data.function_id = "FUNC." + data.function_id;
          if (!ids.has(data.function_id) || !["correction","observation","implementation","variant"].includes(data.kind) || !data.details.trim()) { json(400, { message: "Choose a catalogue function, contribution kind, and nonempty details." }); return; }
          const id = randomUUID();
          save.run(id, new Date().toISOString(), data.function_id, data.kind, data.excel_build, data.details, data.contact);
          json(201, { id, status: "pending", message: "Saved privately for review." });
        }
        return;
      }
      if (!["GET","HEAD"].includes(req.method)) { json(405, { message: "Use GET or HEAD." }, { Allow: "GET, HEAD" }); return; }
      if (url.pathname === "/favicon.ico") { res.writeHead(204); res.end(); return; }
      let file = resolve(root, "." + decodeURIComponent(url.pathname));
      if (file !== root && !file.startsWith(root + sep)) throw Object.assign(new Error("Not found."), { status: 404 });
      if ((await stat(file)).isDirectory()) file = join(file, "index.html");
      file = await realpath(file);
      if (!file.startsWith(root + sep)) throw Object.assign(new Error("Not found."), { status: 404 });
      if (![".html",".css",".js",".mjs",".json",".txt",".wasm",".svg"].includes(extname(file))) throw Object.assign(new Error("Not found."), { status: 404 });
      const content = await readFile(file);
      res.writeHead(200, { "Content-Type": mime[extname(file)] || "application/octet-stream", "Cache-Control": "no-store" });
      res.end(req.method === "HEAD" ? undefined : content);
    } catch (error) {
      if (res.headersSent) { res.end(); return; }
      json(error.status || (error.code === "ENOENT" ? 404 : 500), { message: error.status ? error.message : error.code === "ENOENT" ? "Not found." : "Request could not be handled." });
    }
  });
  server.requestTimeout = 10000; server.headersTimeout = 12000; server.keepAliveTimeout = 5000;
  server.on("close", () => database.close());
  return server;
}

if (process.argv[1] && fileURLToPath(import.meta.url) === resolve(process.argv[1])) {
  const options = { root: "site/build", store: ".tmp/handbook/contributions.sqlite", host: "127.0.0.1", port: 8787 };
  for (let i = 2; i < process.argv.length; i += 2) {
    const key = process.argv[i].replace(/^--/, "");
    if (!Object.hasOwn(options, key) || !process.argv[i+1]) throw new Error("Usage: node server.mjs [--root PATH] [--store PATH] [--host HOST] [--port PORT]");
    options[key] = key === "port" ? Number(process.argv[i+1]) : process.argv[i+1];
  }
  const server = await createHandbookServer(options);
  server.listen(options.port, options.host, () => process.stdout.write(`Handbook local server: http://${options.host}:${server.address().port}\n`));
  for (const signal of ["SIGTERM","SIGINT"]) process.once(signal, () => { server.close(); server.closeAllConnections(); });
}
