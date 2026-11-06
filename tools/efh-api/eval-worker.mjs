import { parentPort, workerData } from "node:worker_threads";
import { readFile } from "node:fs/promises";
import { pathToFileURL } from "node:url";
import { join } from "node:path";
try {
  const { default: init, evaluate } = await import(pathToFileURL(join(workerData.engine, "engine.js")));
  await init({ module_or_path: await readFile(join(workerData.engine, "engine_bg.wasm")) });
  parentPort.postMessage(JSON.parse(evaluate(JSON.stringify(workerData.request))));
} catch {
  parentPort.postMessage({ status: "engine-error", message: "The engine could not calculate this request within its limits." });
}
