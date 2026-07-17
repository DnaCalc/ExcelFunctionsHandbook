# H2 ingest findings — for OxFunc

Date: 2026-07-18 · Found while building `tools/efh-ingest` against OxFunc commit 937f198.
Read-only observation; outbound handoff per OPERATIONS section 1.

1. **Combined `surface_stable_id` values violate the locked id pattern.** Seven rows in
   `docs/function-lane/OXFUNC_LIBRARY_CONTEXT_SNAPSHOT_EXPORT_V1.csv` carry ids like
   `FUNC.FIND, FINDB` (comma and space inside the id). The downstream metadata contract
   (section 3.1) declares `surface_stable_id` stable with locked pattern `FUNC.<NAME>` /
   `OP.<NAME>`; these rows match neither, and the runtime registry models the pairs as
   fourteen separate ids (`FUNC.FIND`, `FUNC.FINDB`, …). Downstream joins on
   `surface_stable_id` silently miss all fourteen registry entries. The Handbook works around
   this by splitting combined rows (recorded in `data/README.md`), but the export and the
   registry should agree on row identity.

2. **W28 localization seed has no rows for IMAGINARY.** The harvest CSV
   (`W28_FUNCTION_NAME_LOCALIZATION_LIBRARY_SEED.csv`) contains zero rows for the IMAGINARY
   function's article (GUID `dd5952fd-473d-44d9-95a1-9a17b23e428a`) in any locale — the only
   function on the published surface with no localized-name coverage. Likely a harvest gap
   rather than a Microsoft gap.

3. **`LET`, `LAMBDA`, and `OP_IMPLICIT_INTERSECTION` are published but absent from
   `builtin_registry()`.** They appear in the snapshot export (and `LET`/`LAMBDA` are
   `function-phase-complete` per the admission policy docs) but have no `FunctionMeta` in
   `FUNCTION_CATALOG`, presumably because their binding semantics are owned by the formula
   layer. If deliberate, the downstream metadata contract's "the registry is the canonical
   consumption path" guidance deserves a note that 16 published rows are export-only (these 3
   plus the 13 deferred functions that have no registry entry; the other 4 deferred rows are
   registry-backed), so consumers know the registry alone under-covers the published surface.
