"use strict";
document.body.classList.add("js");
const lensButtons = [...document.querySelectorAll("[data-lens]")];
document.querySelector(".lensbar").hidden = !document.querySelector(".explain");
function setLens(lens, updateUrl = true) {
  document.body.classList.toggle("explicit", lens === "explicit");
  lensButtons.forEach(button => button.setAttribute("aria-pressed", String(button.dataset.lens === lens)));
  if (updateUrl) {
    const url = new URL(location.href);
    if (lens === "explicit") url.searchParams.set("view", "explicit");
    else url.searchParams.delete("view");
    history.replaceState(null, "", url);
  }
}
lensButtons.forEach(button => button.addEventListener("click", () => setLens(button.dataset.lens)));
setLens(new URL(location.href).searchParams.get("view") === "explicit" ? "explicit" : "working", false);
const filter = document.querySelector("#function-filter");
if (filter) {
  document.querySelector(".filter-controls").hidden = false;
  const category = document.querySelector("#category-filter");
  const status = document.querySelector("#filter-status");
  const groups = [...document.querySelectorAll("#catalogue section")];
  const normalize = value => value.normalize("NFKD").replace(/[\u0300-\u036f]/g, "").toLowerCase();
  filter.value = new URL(location.href).searchParams.get("q") || "";
  function applyFilter() {
    const terms = normalize(filter.value).trim().split(/\s+/).filter(Boolean);
    let total = 0;
    for (const group of groups) {
      let visible = 0;
      for (const item of group.querySelectorAll("[data-search]")) {
        item.hidden = (category.value && category.value !== group.dataset.category) || !terms.every(term => normalize(item.dataset.search).includes(term));
        if (!item.hidden) visible++;
      }
      group.hidden = visible === 0;
      visible && (total += visible);
    }
    status.textContent = `${total} matching entries out of 541.`;
    document.querySelector("#no-results").hidden = total !== 0;
    document.querySelector(".category-nav").hidden = terms.length > 0 || category.value !== "";
  }
  filter.addEventListener("input", applyFilter);
  category.addEventListener("change", applyFilter);
  applyFilter();
}

const assetBase = new URL(".", document.currentScript.src);
const calculator = document.querySelector("[data-evaluator]");
if (calculator) {
  const formula = calculator.elements.formula;
  const result = calculator.querySelector("output");
  const initialFormula = formula.value;
  let worker = null;
  let timer;
  function stopCalculation() {
    clearTimeout(timer);
    worker?.terminate();
    worker = null;
    calculator.querySelector('[type="submit"]').disabled = false;
    calculator.removeAttribute("aria-busy");
  }
  function showResult(response) {
    if (response.status === "calculated" || response.status === "worksheet-error") {
      const value = response.value;
      result.textContent = value.kind === "array"
        ? `${value.rows} × ${value.columns}\n${value.cells.map(row => row.map(cell => cell.display).join("\t")).join("\n")}${value.truncated ? "\nShowing the first 32 rows and 16 columns." : ""}`
        : `${value.display}${value.bits ? `\nBinary64: ${value.bits}` : ""}`;
      result.textContent += `\n${response.status === "worksheet-error" ? "Worksheet error" : "Calculated by OneCalc"}`;
    } else {
      result.textContent = [response.status === "refused" ? "OneCalc could not calculate this formula." : response.message, ...(response.diagnostics || [])].filter(Boolean).join("\n");
    }
    if (response.diagnostics?.length && response.status === "worksheet-error") result.textContent += `\n${response.diagnostics.join("\n")}`;
    result.dataset.status = response.status;
  }
  calculator.addEventListener("submit", event => {
    event.preventDefault();
    stopCalculation();
    let inputs;
    try { inputs = JSON.parse(calculator.elements.inputs?.value || "{}"); }
    catch { result.textContent = "Input cells must be valid JSON, for example {\"B2\": 3}."; return; }
    const request = { formula: formula.value, inputs, now: (Date.now() - new Date().getTimezoneOffset() * 60000) / 86400000 + 25569 };
    result.textContent = "Calculating…";
    result.dataset.status = "pending";
    calculator.setAttribute("aria-busy", "true");
    calculator.querySelector('[type="submit"]').disabled = true;
    try {
      worker = new Worker(new URL("eval-worker.js", assetBase), { type: "module" });
      worker.onmessage = ({ data }) => { stopCalculation(); showResult(data); };
      worker.onerror = event => {
        event.preventDefault(); stopCalculation();
        showResult({ status: "engine-error", message: "Calculator unavailable in this preview." });
      };
      worker.postMessage(request);
      timer = setTimeout(() => {
        stopCalculation(); showResult({ status: "timeout", message: "Calculation stopped after 5 seconds. Try a smaller input." });
      }, 5000);
    } catch {
      stopCalculation(); showResult({ status: "engine-error", message: "Calculator unavailable in this preview." });
    }
  });
  calculator.querySelector("[data-reset]").addEventListener("click", () => {
    stopCalculation(); formula.value = initialFormula;
    if (calculator.elements.inputs) calculator.elements.inputs.value = calculator.elements.inputs.defaultValue;
    result.textContent = "Enter a formula, then calculate."; delete result.dataset.status;
  });
  addEventListener("pagehide", stopCalculation);
}

const contribution = document.querySelector("#contribution-form");
if (contribution) {
  const initialFunction = new URL(location.href).searchParams.get("function");
  if (initialFunction) contribution.elements.function_id.value = initialFunction;
  contribution.addEventListener("submit", async event => {
    event.preventDefault();
    const output = contribution.querySelector("output");
    const button = contribution.querySelector("button");
    button.disabled = true;
    output.textContent = "Sending…";
    try {
      const response = await fetch(new URL("../api/contributions", location.href), {
        method: "POST", headers: { "Content-Type": "application/json" },
        body: JSON.stringify(Object.fromEntries(new FormData(contribution))), signal: AbortSignal.timeout(10000),
      });
      const data = await response.json();
      if (!response.ok) throw new Error(data.message || "Submission could not be saved.");
      output.textContent = `Saved for review. Reference: ${data.id}`;
      contribution.reset();
    } catch (error) {
      output.textContent = `Your contribution has not been saved. ${error.message}`;
    } finally { button.disabled = false; }
  });
}
