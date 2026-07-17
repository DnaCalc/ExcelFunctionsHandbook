# H1 evaluation-context review — findings summary for OxFunc

Date: 2026-07-18 · Reviewed at OxFunc commit 937f198 · Read-only review; nothing in OxFunc
was modified. Detail in the three per-scope registers:

- `H1_FINDINGS_VALUE_COERCION.md` — 13 findings
- `H1_FINDINGS_PIPELINE_CONTEXT.md` — 12 findings
- `H1_FINDINGS_VERSION_AXES.md` — 11 findings

Total: 36. Most are spec/code naming drift or half-migrated documents; the ones below likely
warrant OxFunc-side action.

## Candidate behavior bugs (verify against Excel, then fix or reclassify)

1. **Aggregate scan policy sums numeric-looking text.**
   `AggregateScanPolicy::IgnoreTextAndEmpty` (`crates/oxfunc_core/src/coercion.rs`,
   `aggregate_scan_sum`) skips only unparsable text; `"2"` in a scanned range coerces and is
   summed. Excel's range-scan rule ignores all text/logicals regardless of parseability —
   seed scenario CO4-011 would yield 3 vs Excel's 1. Either a placeholder awaiting the
   per-family matrix, or a live divergence in the flagship direct-vs-scan behavior.

2. **Empty-through-reference cannot reach the EmptyCell outcome.**
   Direct Empty → `CoercionError::EmptyCell`, but a reference resolving to Empty falls into
   `coerce_eval_to_number`'s catch-all → `UnsupportedValueKind`. Policies keyed on EmptyCell
   (`=SIN(A1)` with blank A1 → 0) cannot fire for the most common arrival path of emptiness.

## Data-quality issues that block downstream ingestion

3. **W28 localization seed scrape artifact.** All 36 `BETA.INVn` rows in
   `docs/function-lane/W28_FUNCTION_NAME_LOCALIZATION_LIBRARY_SEED.csv` carry a systematic
   scrape artifact (see H1_FINDINGS_VERSION_AXES.md for detail). The Handbook ingests this
   CSV in its next phase; a corrected regeneration or a documented exclusion row would help.

4. **CSV hygiene.** `COERCION_DECISION_TABLE.csv` row CO-POL-002 has an unquoted comma in the
   notes field (13 fields vs 12-column header); naive parsers mis-split.

## Documentation/register drift worth an editing pass

5. "Current baseline" build number drifts across docs (19725 / 19929 / 20026 / 20131) with no
   single registry of record; consumers cannot tell which build a bare "current baseline"
   claim refers to.
6. `EvalValue` naming survives in the value-universe and coercion specs though code has moved
   to `CalcValue`/`CoreValue`; the spec's 12-tag algebra vs the 13-variant `ValueTag`; seven
   boundary sets in spec vs six in `ValueBoundary`.
7. The adapter-layering spec names axes (`error_policy_class`, `compile_eval_class`) that do
   not exist in `function.rs`.
8. A stale accuracy paragraph in `crates/oxfunc_core/src/excel_numeric/mod.rs` predates the
   x87 backend's current status.
9. `#BUSY!`/`#GETTING_DATA` are unclassified in the error-registry split; codes outside the
   enum (e.g. `#PYTHON!`) have no recorded deferral row.
10. Callable admission at the published-result boundary is ambiguous between spec (not
    publishable as such) and `ValueBoundary::allows` (admits `Callable`).

## Open decisions the chapters had to write around

D-004 (aggregate coercion conflicts), D-005 (argument gaps), D-010 (compatibility-version
policy; the value "2" is referenced but undefined in the read sources), D-017 (admission
boundary), D-018 (selective dereference). The chapters state these as open, with the pinned
seed anchors as the only firm ground.

## Handling

Per Handbook OPERATIONS section 1, this is an outbound handoff: OxFunc decides what to adopt.
Items 1–2 deserve an Excel-oracle probe before any code change (they may be intended
placeholders). Items 3–4 affect the Handbook's H2 ingest and will otherwise be worked around
with documented exclusions.
