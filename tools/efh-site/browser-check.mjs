#!/usr/bin/env node
// Node >=22, installed Chrome. Uses the Chrome DevTools Protocol; no npm packages.
import { createServer, request as proxyRequest } from "node:http";
import { createHandbookServer } from "../efh-api/server.mjs";
import { DatabaseSync } from "node:sqlite";
import { spawn } from "node:child_process";
import { mkdtemp, readFile, writeFile, mkdir, rm, stat } from "node:fs/promises";
import { tmpdir } from "node:os";
import { resolve, join, extname, sep } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import assert from "node:assert/strict";

const repo = fileURLToPath(new URL("../..", import.meta.url));
const args = process.argv.slice(2);
const options = { out: join(repo, "site/build"), screenshots: join(repo, ".tmp/site-browser"), chrome: process.env.CHROME || "google-chrome" };
for (let i = 0; i < args.length; i += 2) {
  const key = args[i].slice(2);
  if (!Object.hasOwn(options, key) || !args[i + 1]) throw new Error("Usage: node browser-check.mjs [--out PATH] [--screenshots PATH] [--chrome PATH]");
  options[key] = args[i + 1];
}
options.out = resolve(options.out);
options.screenshots = resolve(options.screenshots);
const catalogue = JSON.parse(await readFile(join(options.out, "api/index.json"), "utf8"));
let thinPath;
for (const id of new Set(["FUNC.IMLOG10", ...catalogue.rows.map(row => row.function_id)])) {
  const candidate = `functions/${id.replace(/^FUNC\./, "")}.html`;
  if ((await readFile(join(options.out, candidate), "utf8")).includes("Detailed argument guidance is still being written.")) { thinPath = candidate; break; }
}
assert.ok(thinPath, "No brief page remains: revise the thin-page case for the new curation state");
await mkdir(options.screenshots, { recursive: true });
const prefix = "/preview/handbook/";
const mime = { ".html": "text/html; charset=utf-8", ".css": "text/css", ".js": "text/javascript", ".json": "application/json", ".svg": "image/svg+xml", ".wasm": "application/wasm", ".txt": "text/plain" };
const privateStore = join(options.screenshots, "browser-contributions.sqlite");
const apiServer = await createHandbookServer({ root: options.out, store: privateStore });
await new Promise(resolve => apiServer.listen(0, "127.0.0.1", resolve));
const server = createServer(async (req, res) => {
  try {
    const path = new URL(req.url, "http://localhost").pathname;
    if (path === "/favicon.ico") { res.writeHead(204); res.end(); return; }
    if (!path.startsWith(prefix)) throw new Error("outside preview prefix");
    if (["api/eval", "api/contributions"].includes(path.slice(prefix.length))) {
      const upstream = proxyRequest({ host: "127.0.0.1", port: apiServer.address().port, path: "/" + path.slice(prefix.length), method: req.method, headers: req.headers }, response => {
        res.writeHead(response.statusCode, response.headers); response.pipe(res);
      });
      upstream.on("error", () => { res.writeHead(502); res.end(); });
      req.pipe(upstream); return;
    }
    let file = resolve(options.out, decodeURIComponent(path.slice(prefix.length)));
    if (file !== options.out && !file.startsWith(options.out + sep)) throw new Error("outside output");
    if ((await stat(file)).isDirectory()) file = join(file, "index.html");
    res.writeHead(200, { "Content-Type": mime[extname(file)] || "application/octet-stream", "Cache-Control": "no-store" });
    res.end(await readFile(file));
  } catch { res.writeHead(404); res.end("Not found"); }
});
await new Promise(resolve => server.listen(0, "127.0.0.1", resolve));
const origin = `http://127.0.0.1:${server.address().port}`;
const profile = await mkdtemp(join(tmpdir(), "efh-chrome-"));
const chrome = spawn(options.chrome, ["--headless=new", "--no-sandbox", "--disable-dev-shm-usage", "--disable-background-networking", "--disable-component-update", "--no-first-run", "--no-default-browser-check", "--remote-debugging-port=0", `--user-data-dir=${profile}`, "about:blank"], { stdio: ["ignore", "ignore", "pipe"] });
let stderr = "";
const debugUrl = await new Promise((resolve, reject) => {
  const timeout = setTimeout(() => reject(new Error(`Chrome did not start: ${stderr.slice(-2000)}`)), 15000);
  chrome.once("error", reject);
  chrome.once("exit", code => { clearTimeout(timeout); reject(new Error(`Chrome exited ${code}: ${stderr.slice(-2000)}`)); });
  chrome.stderr.on("data", chunk => {
    stderr += chunk;
    const match = stderr.match(/DevTools listening on (ws:\/\/\S+)/);
    if (match) { clearTimeout(timeout); resolve(match[1]); }
  });
});

class CDP {
  constructor(socket) {
    this.socket = socket; this.next = 0; this.pending = new Map(); this.listeners = new Map();
    socket.addEventListener("message", event => {
      const message = JSON.parse(event.data);
      if (message.id) {
        const pending = this.pending.get(message.id);
        if (!pending) return;
        clearTimeout(pending.timeout); this.pending.delete(message.id);
        if (message.error) pending.reject(new Error(JSON.stringify(message.error)));
        else pending.resolve(message.result);
      } else this.listeners.get(message.method)?.forEach(callback => callback(message.params));
    });
  }
  static async open(url) {
    const socket = new WebSocket(url);
    await new Promise((resolve, reject) => { socket.addEventListener("open", resolve, { once: true }); socket.addEventListener("error", reject, { once: true }); });
    return new CDP(socket);
  }
  send(method, params = {}) {
    const id = ++this.next;
    return new Promise((resolve, reject) => {
      const timeout = setTimeout(() => { this.pending.delete(id); reject(new Error(`CDP timeout: ${method}`)); }, 15000);
      this.pending.set(id, { resolve, reject, timeout });
      this.socket.send(JSON.stringify({ id, method, params }));
    });
  }
  on(method, callback) { const list = this.listeners.get(method) || []; list.push(callback); this.listeners.set(method, list); }
  async evaluate(expression) {
    const result = await this.send("Runtime.evaluate", { expression, returnByValue: true, awaitPromise: true });
    if (result.exceptionDetails) throw new Error(JSON.stringify(result.exceptionDetails));
    return result.result.value;
  }
  close() { this.socket.close(); }
}

let browser;
const report = { cases: [], checks: [], failures: [], screenshots: options.screenshots, thinRoute: thinPath };
try {
  browser = await CDP.open(debugUrl);
  report.browser = await browser.send("Browser.getVersion");
  const port = new URL(debugUrl).port;
  async function withPage(path, width, mode, callback, nojs = false) {
    const { targetId } = await browser.send("Target.createTarget", { url: "about:blank" });
    let cdp;
    let loadTimer;
    const errors = [];
    try {
      const targets = await (await fetch(`http://127.0.0.1:${port}/json/list`)).json();
      cdp = await CDP.open(targets.find(target => target.id === targetId).webSocketDebuggerUrl);
      for (const name of ["Page.enable", "Runtime.enable", "Network.enable"]) await cdp.send(name);
      cdp.on("Runtime.exceptionThrown", event => errors.push(`exception: ${event.exceptionDetails.text}`));
      cdp.on("Runtime.consoleAPICalled", event => { if (event.type === "error") errors.push(`console: ${JSON.stringify(event.args)}`); });
      cdp.on("Network.loadingFailed", event => errors.push(`request: ${event.errorText}`));
      cdp.on("Network.responseReceived", event => { if (event.response.status >= 400) errors.push(`HTTP ${event.response.status}: ${event.response.url}`); });
      cdp.on("Network.requestWillBeSent", event => { if (/^https?:/.test(event.request.url) && !event.request.url.startsWith(origin + "/")) errors.push(`external request: ${event.request.url}`); });
      await cdp.send("Emulation.setDeviceMetricsOverride", { width, height: 1000, deviceScaleFactor: 1, mobile: false });
      await cdp.send("Emulation.setEmulatedMedia", { features: [{ name: "prefers-color-scheme", value: mode }] });
      if (nojs) await cdp.send("Emulation.setScriptExecutionDisabled", { value: true });
      const loaded = new Promise((resolve, reject) => {
        loadTimer = setTimeout(() => reject(new Error("page load timeout")), 15000);
        cdp.on("Page.loadEventFired", () => { clearTimeout(loadTimer); resolve(); });
      });
      const navigation = await cdp.send("Page.navigate", { url: path.startsWith("file:") ? path : origin + prefix + path });
      if (navigation.errorText) throw new Error(navigation.errorText);
      await loaded;
      const value = await callback(cdp);
      assert.deepEqual(errors, [], `${path} runtime and request failures`);
      return value;
    } finally { clearTimeout(loadTimer); cdp?.close(); await browser.send("Target.closeTarget", { targetId }); }
  }
  const routes = [["home", "index.html"], ["round", "functions/ROUND.html"], ["thin", thinPath], ["notes", "notes/index.html"], ["coverage", "coverage/index.html"], ["catalogue", "functions/index.html"], ["series", "lastbit/index.html"], ["contribute", "contribute/index.html"], ["lambda", "functions/LAMBDA.html"], ["operator", "functions/OP_TRIM_REF_BOTH.html"], ["parity", "functions/ISEVEN.html"]];
  for (const width of [390, 1440]) for (const mode of ["light", "dark"]) for (const [name, path] of routes) {
    const result = await withPage(path, width, mode, async cdp => {
      const metrics = await cdp.evaluate(`({width: innerWidth, client: document.documentElement.clientWidth, scroll: document.documentElement.scrollWidth, heading: document.querySelector('h1').innerText, h1s: document.querySelectorAll('h1').length, scheme: getComputedStyle(document.body).backgroundColor, bytes: performance.getEntriesByType('resource').reduce((sum,r)=>sum+r.transferSize,0), duration: performance.getEntriesByType('navigation')[0].duration})`);
      assert.equal(metrics.width, width);
      assert.equal(metrics.scroll, metrics.client, `${path} document overflow at ${width}`);
      assert.equal(metrics.h1s, 1, `${path} heading hierarchy`);
      if (name === "thin") assert.ok(await cdp.evaluate(`document.body.innerText.includes('Detailed argument guidance is still being written.')`), "thin-page case must retain its stated scope");
      for (const type of ["keyDown", "keyUp"]) await cdp.send("Input.dispatchKeyEvent", { type, key: "Tab", code: "Tab", windowsVirtualKeyCode: 9 });
      assert.ok(await cdp.evaluate(`document.activeElement.classList.contains('skip')`), "Tab reaches skip link first");
      for (const type of ["keyDown", "keyUp"]) await cdp.send("Input.dispatchKeyEvent", { type, key: "Enter", code: "Enter", windowsVirtualKeyCode: 13 });
      assert.equal(await cdp.evaluate(`document.activeElement.id`), "main", "skip link must move focus");
      // Stress text width, then traverse the whole page, including wide tables.
      const stress = await cdp.evaluate(`(() => { const style=document.createElement('style'); style.id='stress'; style.textContent='* {letter-spacing:.04em!important}'; document.head.append(style); return document.documentElement.scrollWidth-document.documentElement.clientWidth; })()`);
      assert.equal(stress, 0, `${path} text expansion overflow`);
      await cdp.evaluate(`document.querySelector('#stress').remove(); window.scrollTo(0,0);`);
      const focus = await cdp.evaluate(`(() => { const b=document.querySelector('.lensbar').hidden ? document.querySelector('.mainnav a') : document.querySelector('[data-lens=explicit]'); b.focus(); const s=getComputedStyle(b); return {style:s.outlineStyle,width:parseFloat(s.outlineWidth)}; })()`);
      assert.ok(focus.style !== "none" && focus.width >= 2, "visible keyboard focus");
      if (await cdp.evaluate(`!document.querySelector('.lensbar').hidden`)) {
        const before = await cdp.evaluate(`document.querySelector('h1').innerText`);
        await cdp.evaluate(`document.querySelector('[data-lens=explicit]').click()`);
        assert.ok(await cdp.evaluate(`document.body.classList.contains('explicit') && document.querySelector('[data-lens=explicit]').getAttribute('aria-pressed')==='true'`));
        assert.ok(await cdp.evaluate(`[...document.querySelectorAll('.explain')].every(el=>getComputedStyle(el).display!=='none')`));
        assert.equal(await cdp.evaluate(`document.querySelector('h1').innerText`), before);
        await cdp.evaluate(`document.querySelector('[data-lens=working]').click();`);
      }
      await cdp.evaluate(`document.activeElement.blur();`);
      const height = await cdp.evaluate(`document.documentElement.scrollHeight`);
      for (let y = 0; y < height; y += 750) await cdp.evaluate(`window.scrollTo(0,${y});`);
      await cdp.evaluate(`window.scrollTo(0,0);`);
      if (["home", "round", "thin", "lambda", "operator", "parity"].includes(name)) {
        const file = `${name}-${width}-${mode}.png`;
        const screenshot = await cdp.send("Page.captureScreenshot", { format: "png", captureBeyondViewport: false });
        await writeFile(join(options.screenshots, file), Buffer.from(screenshot.data, "base64"));
        if (name === "home") {
          const full = await cdp.send("Page.captureScreenshot", { format: "png", captureBeyondViewport: true, clip: { x: 0, y: 0, width, height, scale: 1 } });
          await writeFile(join(options.screenshots, `home-full-${width}-${mode}.png`), Buffer.from(full.data, "base64"));
        }
        if (name === "round") {
          await cdp.evaluate(`window.scrollTo(0,document.querySelector('.note').getBoundingClientRect().top+scrollY-20);`);
          const note = await cdp.send("Page.captureScreenshot", { format: "png", captureBeyondViewport: false });
          await writeFile(join(options.screenshots, `round-note-${width}-${mode}.png`), Buffer.from(note.data, "base64"));
        }
      }
      return metrics;
    });
    report.cases.push({ name, path, width, mode, ...result });
    process.stdout.write(`OK ${name} ${width}px ${mode}\n`);
  }
  await withPage("functions/index.html?q=ROUND", 390, "light", async cdp => {
    assert.ok(await cdp.evaluate(`document.querySelector('#function-filter').value==='ROUND' && [...document.querySelectorAll('[data-search]')].some(el=>!el.hidden && el.innerText.includes('ROUND'))`));
    await cdp.evaluate(`const f=document.querySelector('#function-filter'); f.value='no-such-function-xyz'; f.dispatchEvent(new Event('input'));`);
    assert.ok(await cdp.evaluate(`!document.querySelector('#no-results').hidden && [...document.querySelectorAll('#catalogue section')].every(el=>el.hidden)`));
    await cdp.evaluate(`const f2=document.querySelector('#function-filter'); f2.value='arrondi'; f2.dispatchEvent(new Event('input'));`);
    assert.ok(await cdp.evaluate(`[...document.querySelectorAll('[data-search]')].some(el=>!el.hidden && el.innerText.includes('ROUND'))`));
    await cdp.evaluate(`const f3=document.querySelector('#function-filter'); f3.value=''; f3.dispatchEvent(new Event('input')); const s=document.querySelector('#category-filter'); s.value='Financial functions'; s.dispatchEvent(new Event('change'));`);
    assert.ok(await cdp.evaluate(`[...document.querySelectorAll('#catalogue section')].filter(el=>!el.hidden).every(el=>el.dataset.category==='Financial functions')`));
  });
  report.checks.push("search query, empty results, localized names, category filter");
  await withPage("functions/ROUND.html", 390, "dark", async cdp => {
    assert.ok(await cdp.evaluate(`document.querySelectorAll('.note').length>=3 && document.body.innerText.includes('0xc2d9945ca2620000')`));
    assert.deepEqual(await cdp.evaluate(`[...document.querySelectorAll('.function-section > h2')].map(el=>el.textContent)`), ["What it does","Try it","Quirks","How to implement","Version differences","Alternatives"]);
    assert.ok(await cdp.evaluate(`!document.body.innerText.match(/mixed-vintage|pre-ledger|export commit|OxFunc admission/i)`));
    await cdp.evaluate(`document.querySelector('.source-footer details').open=true`);
    assert.ok(await cdp.evaluate(`document.querySelector('.source-footer').innerText.includes('Function JSON')`));
  });
  report.checks.push("six reader sections, ROUND quirks and raw bits, compact source footer");
  for (const [formula, expectedStatus, expectedText, inputs] of [
    ["=ROUND(1.005,2)","calculated","1.01",{}],
    ["=SUM(B2:B3)","calculated","7",{B2:3,B3:4}],
    ["=SEQUENCE(2,3)","calculated","2 × 3",{}],
    ["=1/0","worksheet-error","#DIV/0!",{}],
    ["=B2","refused","could not calculate",{}],
    ["=LAMBDA(x,x+1)(4)","calculated","5",{}],
  ]) await withPage("functions/ROUND.html", 390, "light", async cdp => {
    const result = await cdp.evaluate(`new Promise(resolve=>{
      const form=document.querySelector('[data-evaluator]'),out=form.querySelector('output');
      form.elements.formula.value=${JSON.stringify(formula)};
      form.elements.inputs.value=${JSON.stringify(JSON.stringify(inputs))};
      const observer=new MutationObserver(()=>{if(out.dataset.status && out.dataset.status!=='pending'){observer.disconnect();resolve({status:out.dataset.status,text:out.textContent});}});
      observer.observe(out,{childList:true,subtree:true,attributes:true});form.requestSubmit();
    })`);
    assert.equal(result.status,expectedStatus,JSON.stringify(result));assert.ok(result.text.includes(expectedText),JSON.stringify(result));
    const response=await cdp.evaluate(`fetch(new URL('../api/eval',location.href),{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({formula:${JSON.stringify(formula)},inputs:${JSON.stringify(inputs)}})}).then(r=>r.json())`);
    assert.equal(response.status,result.status);
    if(response.value?.bits) assert.ok(result.text.includes(response.value.bits));
    await cdp.evaluate(`const edited=document.querySelector('#formula'); edited.value='=2+3'; edited.dispatchEvent(new Event('input'));`);
    assert.equal(await cdp.evaluate(`document.querySelector('#eval-result').dataset.status`),"stale");
    assert.ok(await cdp.evaluate(`document.querySelector('#eval-result').textContent.includes('Calculate to update')`));
    await cdp.evaluate(`document.querySelector('[data-reset]').click()`);
    assert.equal(await cdp.evaluate(`document.querySelector('#formula').value`),"=ROUND(1.005,2)");
  });
  report.checks.push("browser wasm calculates formulas, cells, arrays, LAMBDA, errors and refusals; numeric bits agree with API");
  await withPage("contribute/index.html?function=FUNC.ROUND",390,"light",async cdp=>{
    assert.equal(await cdp.evaluate(`document.querySelector('#contribution-function').value`),"FUNC.ROUND");
    const message=await cdp.evaluate(`new Promise(resolve=>{
      const form=document.querySelector('#contribution-form'),out=form.querySelector('output');
      form.elements.details.value='Browser test contribution: ROUND example checked locally.';
      const observer=new MutationObserver(()=>{if(out.textContent.startsWith('Saved for review.')){observer.disconnect();resolve(out.textContent);}});
      observer.observe(out,{childList:true,subtree:true});form.requestSubmit();
    })`);
    assert.equal(await cdp.evaluate(`document.querySelector('#contribution-function').value`),"FUNC.ROUND");
    const id=message.split('Reference: ')[1];const db=new DatabaseSync(privateStore);
    assert.equal(db.prepare('SELECT function_id FROM contributions WHERE id=?').get(id).function_id,'FUNC.ROUND');db.close();
  });
  report.checks.push("contribution form saves through API into private SQLite");
  await withPage("functions/index.html", 390, "light", async cdp => {
    assert.equal(await cdp.evaluate(`document.querySelectorAll('[data-search]').length`), 541);
    assert.ok(await cdp.evaluate(`[...document.querySelectorAll('.filter-controls')].every(el=>getComputedStyle(el).display==='none')`));
    assert.ok(await cdp.evaluate(`document.querySelector('section[data-category]')!==null`));
  }, true);
  report.checks.push("541 catalogue links without JavaScript");
  await withPage("functions/ROUND.html", 390, "light", async cdp => {
    assert.ok(await cdp.evaluate(`[...document.querySelectorAll('.explain')].every(el=>getComputedStyle(el).display!=='none')`));
  }, true);
  await withPage("functions/ROUND.html",390,"light",async cdp=>{
    assert.ok(await cdp.evaluate(`document.querySelector('[data-evaluator] [type=submit]').disabled`));
  },true);
  await withPage("contribute/index.html",390,"light",async cdp=>{
    assert.ok(await cdp.evaluate(`document.querySelector('#contribution-form [type=submit]').disabled`));
  },true);
  report.checks.push("scope explanations visible without JavaScript; interactive forms cannot fall back to GET");
  for (const width of [320, 375]) for (const [, path] of routes) await withPage(path, width, "light", async cdp => {
    const overflow = await cdp.evaluate(`({overflow:document.documentElement.scrollWidth-document.documentElement.clientWidth,elements:[...document.querySelectorAll('main *')].filter(e=>!e.closest('.table-scroll,pre')&&e.getBoundingClientRect().right>document.documentElement.clientWidth+1).map(e=>({tag:e.tagName,class:e.className,text:e.innerText?.slice(0,100),right:e.getBoundingClientRect().right})).slice(0,10)})`);
    assert.equal(overflow.overflow, 0, `${path} narrow-width overflow: ${JSON.stringify(overflow.elements)}`);
    assert.equal(await cdp.evaluate(`(() => { const s=document.createElement('style');s.textContent='*{letter-spacing:.04em!important}';document.head.append(s);return document.documentElement.scrollWidth-document.documentElement.clientWidth; })()`), 0, `${path} narrow-width text stress`);
  });
  report.checks.push("320px and 375px overflow and text-width stress across eleven routes");
  await withPage(pathToFileURL(join(options.out, "index.html")).href, 390, "light", async cdp => {
    assert.ok(await cdp.evaluate(`document.body.classList.contains('js') && document.querySelector('h1').innerText==='The Handbook of Excel Functions'`));
  });
  report.checks.push("homepage and its local assets under file://");
} catch (error) {
  report.failures.push(error.stack || String(error));
  process.exitCode = 1;
} finally {
  browser?.close();
  try {
    const exited = new Promise(resolve => { if (chrome.exitCode !== null || chrome.signalCode !== null) resolve(); else chrome.once("exit", resolve); });
    chrome.kill("SIGTERM");
    await exited;
    await new Promise(resolve => server.close(resolve));
    apiServer.closeAllConnections(); await new Promise(resolve => apiServer.close(resolve));
    for (const suffix of ["", "-shm", "-wal"]) await rm(privateStore+suffix,{force:true});
    // Chrome helper processes can finish profile writes just after the parent exits.
    await rm(profile, { recursive: true, force: true, maxRetries: 10, retryDelay: 100 });
  } catch (error) { report.failures.push(`Browser cleanup: ${error.stack || error}`); process.exitCode = 1; }
  await writeFile(join(options.screenshots, "report.json"), JSON.stringify(report, null, 2));
}
console.log(JSON.stringify({ cases: report.cases.length, checks: report.checks, failures: report.failures, screenshots: report.screenshots }, null, 2));
