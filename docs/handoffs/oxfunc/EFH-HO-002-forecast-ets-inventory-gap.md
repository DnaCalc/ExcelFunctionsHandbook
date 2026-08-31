# EFH-HO-002 — FORECAST.ETS family missing from the OxFunc/Handbook spine

Status: `open handoff`
Owner requested: OxFunc function-inventory lane
Observed: 2026-08-24

## Finding

The current Handbook spine contains 518 functions. A row-by-row comparison with Microsoft's live
English alphabetical function table found four Microsoft-listed worksheet functions that are not
present in `data/index.json`, any `data/functions/FUNC.*.json` record, or OxFunc's corrected
catalogue/export spine:

| Missing function | Version | Official one-line help | Microsoft Support target |
|---|---|---|---|
| `FORECAST.ETS` | 2016 | Returns a future value based on existing historical values using the AAA form of exponential smoothing. | `https://support.microsoft.com/en-us/office/forecasting-functions-reference-897a2fe9-6595-4680-a0b0-93e0308d5f6e#_FORECAST.ETS` |
| `FORECAST.ETS.CONFINT` | 2016 | Returns a confidence interval for the forecast value at the requested target date. | `https://support.microsoft.com/en-us/office/forecasting-functions-reference-897a2fe9-6595-4680-a0b0-93e0308d5f6e#_FORECAST.ETS.CONFINT` |
| `FORECAST.ETS.SEASONALITY` | 2016 | Returns the length of the repetitive pattern Excel detects in the time series. | `https://support.microsoft.com/en-us/office/forecasting-functions-reference-897a2fe9-6595-4680-a0b0-93e0308d5f6e#_FORECAST.ETS.SEASONALITY` |
| `FORECAST.ETS.STAT` | 2016 | Returns a statistic produced by time-series forecasting. | `https://support.microsoft.com/en-us/office/forecasting-functions-reference-897a2fe9-6595-4680-a0b0-93e0308d5f6e#_FORECAST.ETS.STAT` |

The descriptions above are close paraphrases for this handoff. The exact harvested display strings
remain in the named Foundation raw source and on Microsoft Support; they should be projected
verbatim by the eventual mechanical catalogue writer, not copied from this prose.

## Existing local evidence

This is not a new-web-only discovery. Foundation's earlier fetch already contains all four rows:

- `research/runs/20260228-130325-excel-compat-spec-index-pass-01/inputs/raw/source_2.html`
- source SHA-256: `41c7a49914f239e3a0a38bd30697eaac68f316b1c112327947f2f2c871d53b01`
- row region in that file: the alphabetical table entries carrying the shared forecasting article
  GUID `897a2fe9-6595-4680-a0b0-93e0308d5f6e` and the four anchors listed above.

Foundation's ECS-008 raw standards material also names all four future-function calls. The defect is
therefore downstream of source acquisition: the four rows were dropped before
`function_catalog_full.csv`, OxFunc's corrected catalogue, and the OxFunc export spine were formed.
OxFunc's W28 localization seed contains no `FORECAST.ETS*` rows, so this local corpus cannot supply
their translated labels or one-line help; that absence must remain explicit until a new localized
intake is harvested.

Microsoft's live alphabetical table was re-read on 2026-08-24. After splitting the seven combined
name rows and correcting the table's known `BETA.INVn` display typo only for comparison, it carries
521 distinct function names. The Handbook spine carries 518; the exact set difference is these four
Microsoft rows minus `COPILOT`. `COPILOT` remains in the pinned W28 harvest with an official article
and must not be deleted merely because the current alphabetical table omits it; that is a separate
scope/version decision.

## Requested upstream action

1. Add stable IDs and catalogue/export rows for all four `FORECAST.ETS*` functions.
2. Preserve the common Support article with its four distinct fragments; do not collapse the four
   function identities into one row.
3. Decide their registry/admission status explicitly, even if implementation is deferred.
4. Re-cut the corrected catalogue/export and localization intake from a named clean source state.
5. Notify the Handbook so its canonical denominator, schemas, F0/F1 projection, F6 page inventory,
   and rubric baseline can be migrated together rather than locally patched.

## Handbook boundary

The Handbook does not write OxFunc. Until this handoff lands upstream, the four help summaries and
links above are retained here and in the completeness audit, but no fake F1 record or F6 function
page is created. The published rubric therefore continues to state the currently decided 541-entry
denominator and must not be described as complete against Microsoft's live function table.
