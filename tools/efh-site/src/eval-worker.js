// One worker per calculation: terminating it also discards a timed-out wasm instance.
import init, { evaluate } from "./onecalc/engine.js";
const ready = init();
self.onmessage = async ({ data }) => {
  try {
    await ready;
    self.postMessage(JSON.parse(evaluate(JSON.stringify(data))));
  } catch {
    self.postMessage({ status: "engine-error", message: "The calculation exceeded the engine's limits or the calculator is unavailable in this preview." });
  }
};
