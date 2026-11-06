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
  const stateFilter = document.querySelector("#state-filter");
  stateFilter.value = new URL(location.href).searchParams.get("state") || "";
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
        item.hidden = (category.value && category.value !== group.dataset.category) || (stateFilter.value && stateFilter.value !== item.dataset.state) || !terms.every(term => normalize(item.dataset.search).includes(term));
        if (!item.hidden) visible++;
      }
      group.hidden = visible === 0;
      visible && (total += visible);
    }
    status.textContent = `${total} matching entries out of ${groups.reduce((sum, group) => sum + group.querySelectorAll("[data-search]").length, 0)}.`;
    document.querySelector("#no-results").hidden = total !== 0;
    document.querySelector(".category-nav").hidden = terms.length > 0 || category.value !== "" || stateFilter.value !== "";
  }
  filter.addEventListener("input", applyFilter);
  category.addEventListener("change", applyFilter);
  stateFilter.addEventListener("change", applyFilter);
  applyFilter();
}

const assetBase = new URL(".", document.currentScript.src);
const siteBase = new URL("..", assetBase);
function storedLanguage() { try { return localStorage.getItem("efh-language") || ""; } catch { return ""; } }
function storeLanguage(tag) { try { if (tag) localStorage.setItem("efh-language", tag); else localStorage.removeItem("efh-language"); } catch { /* the page works without storage */ } }

// Instant search: loads a static script, so it also runs from file:// and needs no server.
const fold = value => value.normalize("NFKD").replace(/[\u0300-\u036f]/g, "").toLowerCase();
let searchIndex = null, searchLoading = null;
function loadSearchIndex() {
  if (searchIndex) return Promise.resolve(searchIndex);
  searchLoading ??= new Promise((resolve, reject) => {
    const script = document.createElement("script");
    script.src = new URL("search-index.js", assetBase);
    script.onload = () => {
      const tags = window.EFH_SEARCH.locales;
      searchIndex = window.EFH_SEARCH.rows.map(([name, path, category, summary, id, locals]) => {
        const pairs = locals ? locals.split("|").map(item => { const at = item.indexOf("="); return [at > 0 ? tags[Number(item.slice(0, at))] : "", item.slice(at + 1)]; }) : [];
        const names = [...new Set(pairs.map(pair => pair[1]))];
        return { name, path, category, summary, id, names, byLocale: Object.fromEntries(pairs.filter(pair => pair[0])), foldedName: fold(name), foldedLocals: names.map(fold), foldedText: fold(`${summary} ${category}`), foldedId: fold(id) };
      });
      resolve(searchIndex);
    };
    script.onerror = () => reject(new Error("search index unavailable"));
    document.head.append(script);
  });
  return searchLoading;
}
function scoreEntry(entry, terms, query) {
  let score = 0;
  if (entry.foldedName === query || entry.foldedId === query) score = 1000;
  else if (entry.foldedName.startsWith(query)) score = 800 - entry.name.length;
  else if (entry.foldedName.includes(query)) score = 600 - entry.name.length;
  const exact = entry.foldedLocals.findIndex(name => name === query);
  const prefix = entry.foldedLocals.findIndex(name => name.startsWith(query));
  const inner = entry.foldedLocals.findIndex(name => name.includes(query));
  const hit = exact >= 0 ? exact : prefix >= 0 ? prefix : inner;
  const localScore = exact >= 0 ? 900 : prefix >= 0 ? 700 : inner >= 0 ? 500 : 0;
  if (localScore > score) score = localScore;
  if (!score && terms.every(term => entry.foldedName.includes(term) || entry.foldedText.includes(term) || entry.foldedLocals.some(name => name.includes(term)))) score = 100;
  return { score, local: hit >= 0 ? entry.names[hit] : "" };
}
function attachSearch(input) {
  let panel = input.form.querySelector(".search-results");
  if (!panel) { panel = document.createElement("div"); panel.className = "search-results"; panel.id = `${input.id}-results`; panel.hidden = true; input.form.append(panel); }
  let results = [], active = -1;
  input.setAttribute("role", "combobox");
  input.setAttribute("aria-expanded", "false");
  input.setAttribute("aria-autocomplete", "list");
  input.setAttribute("aria-controls", panel.id);
  panel.setAttribute("role", "listbox");
  panel.setAttribute("aria-label", "Search results");
  const close = () => { panel.hidden = true; active = -1; input.setAttribute("aria-expanded", "false"); input.removeAttribute("aria-activedescendant"); };
  const mark = () => {
    [...panel.querySelectorAll("[role=option]")].forEach((option, index) => {
      option.setAttribute("aria-selected", String(index === active));
      if (index === active) { option.scrollIntoView({ block: "nearest" }); input.setAttribute("aria-activedescendant", option.id); }
    });
  };
  async function run() {
    const query = fold(input.value).trim();
    if (!query) { close(); return; }
    let rows;
    try { rows = await loadSearchIndex(); } catch { close(); return; }
    if (fold(input.value).trim() !== query) return;
    const terms = query.split(/\s+/);
    results = rows.map(entry => ({ entry, ...scoreEntry(entry, terms, query) })).filter(row => row.score > 0).sort((a, b) => b.score - a.score || a.entry.name.localeCompare(b.entry.name)).slice(0, 12);
    panel.replaceChildren(...results.map(({ entry, local }, position) => {
      const link = document.createElement("a");
      link.className = "search-result"; link.id = `${panel.id}-${position}`; link.setAttribute("role", "option");
      link.href = new URL(entry.path, siteBase);
      const name = document.createElement("strong"); name.textContent = entry.name; link.append(name);
      const preferred = entry.byLocale[storedLanguage()];
      if (preferred && !local) { const tag = document.createElement("span"); tag.className = "local"; tag.textContent = preferred; link.append(tag); }
      if (local && fold(local) !== entry.foldedName) { const tag = document.createElement("span"); tag.className = "local"; tag.textContent = local; link.append(tag); }
      const detail = document.createElement("span"); detail.className = "small"; detail.textContent = [entry.summary, entry.category].filter(Boolean).join(" · "); link.append(detail);
      return link;
    }));
    if (!results.length) { const empty = document.createElement("p"); empty.className = "search-empty"; empty.textContent = "No matching function."; panel.append(empty); }
    active = results.length ? 0 : -1;
    panel.hidden = false; input.setAttribute("aria-expanded", "true"); mark();
  }
  input.addEventListener("input", run);
  input.addEventListener("focus", () => { loadSearchIndex().catch(() => {}); if (input.value.trim()) run(); });
  input.addEventListener("keydown", event => {
    if (event.key === "Escape") { close(); return; }
    if (panel.hidden || !results.length) return;
    if (event.key === "ArrowDown") { event.preventDefault(); active = (active + 1) % results.length; mark(); }
    else if (event.key === "ArrowUp") { event.preventDefault(); active = (active - 1 + results.length) % results.length; mark(); }
    else if (event.key === "Enter" && active >= 0) { event.preventDefault(); panel.querySelectorAll("[role=option]")[active].click(); }
  });
  document.addEventListener("click", event => { if (!input.form.contains(event.target)) close(); });
}
document.querySelectorAll("[data-instant-search]").forEach(attachSearch);

// The language picker: remembers one language, shows its name on this page and in the lists.
const picker = document.querySelector("#language-picker");
function showLanguage(tag) {
  const line = document.querySelector("#local-name");
  if (line) {
    const row = tag && document.querySelector(`#local-names [data-locale="${tag}"]`);
    line.hidden = !row;
    if (row) { line.textContent = `${row.cells[0].textContent}: ${row.cells[1].textContent}`; line.lang = tag; }
    document.querySelectorAll("#local-names tr").forEach(item => item.classList.toggle("chosen", Boolean(tag) && item.dataset.locale === tag));
  }
  const rows = document.querySelectorAll("[data-entry]");
  document.querySelectorAll(".entry-local").forEach(item => item.remove());
  if (!tag || !rows.length) return;
  loadSearchIndex().then(index => {
    const byId = new Map(index.map(entry => [entry.id, entry]));
    rows.forEach(row => {
      const name = byId.get(row.dataset.entry)?.byLocale[tag];
      if (name) { const label = document.createElement("span"); label.className = "entry-local"; label.lang = tag; label.textContent = name; row.querySelector("a").after(label); }
    });
  }).catch(() => {});
}
if (picker) {
  picker.closest("label").hidden = false;
  picker.value = storedLanguage();
  picker.addEventListener("change", () => { storeLanguage(picker.value); showLanguage(picker.value); });
  showLanguage(picker.value);
}

// A link to the code opens it; the section stays collapsed until asked for.
function openFragment() {
  const target = location.hash && document.querySelector(`${CSS.escape(location.hash.slice(1)) ? "#" + CSS.escape(location.hash.slice(1)) : ""}`);
  if (target?.id === "rust-code") target.nextElementSibling.open = true;
}
addEventListener("hashchange", openFragment);
openFragment();
addEventListener("keydown", event => {
  if (event.key !== "/" || event.ctrlKey || event.metaKey || event.altKey) return;
  const target = event.target;
  if (target instanceof HTMLElement && (target.isContentEditable || ["INPUT", "TEXTAREA", "SELECT"].includes(target.tagName))) return;
  event.preventDefault();
  const box = document.querySelector("#site-search");
  box.focus(); box.select();
});
const calculator = document.querySelector("[data-evaluator]");
if (calculator) {
  const formula = calculator.elements.formula;
  const result = calculator.querySelector("output");
  const initialFormula = formula.value;
  calculator.querySelector('[type="submit"]').disabled = false;
  calculator.querySelector('[data-reset]').disabled = false;
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
    result.textContent = "Loading calculator…";
    result.dataset.status = "pending";
    calculator.setAttribute("aria-busy", "true");
    calculator.querySelector('[type="submit"]').disabled = true;
    try {
      worker = new Worker(new URL("eval-worker.js", assetBase), { type: "module" });
      const job = worker;
      worker.onmessage = ({ data }) => {
        if (worker !== job) return;
        if (data.phase === "ready") {
          clearTimeout(timer); result.textContent = "Calculating…";
          timer = setTimeout(() => {
            stopCalculation(); showResult({ status: "timeout", message: "Calculation stopped after 5 seconds. Try a smaller input." });
          }, 5000);
          return;
        }
        stopCalculation(); showResult(data);
      };
      worker.onerror = event => {
        event.preventDefault(); if (worker !== job) return; stopCalculation();
        showResult({ status: "engine-error", message: "Calculator unavailable in this preview." });
      };
      worker.postMessage(request);
      timer = setTimeout(() => {
        stopCalculation(); showResult({ status: "timeout", message: "Calculator could not load. Check your connection and try again." });
      }, 30000);
    } catch {
      stopCalculation(); showResult({ status: "engine-error", message: "Calculator unavailable in this preview." });
    }
  });
  for (const field of [formula, calculator.elements.inputs].filter(Boolean)) field.addEventListener("input", () => {
    stopCalculation();
    result.textContent = "Formula or inputs changed. Calculate to update the result.";
    result.dataset.status = "stale";
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
  contribution.querySelector('[type="submit"]').disabled = false;
  const initialFunction = new URL(location.href).searchParams.get("function");
  if (initialFunction) contribution.elements.function_id.defaultValue = initialFunction;
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
