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
