---
schema: efh.function-page/v1
function_id: FUNC.FORECAST.ETS
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "FORECAST.ETS function"
    locator: "https://support.microsoft.com/en-us/excel/functions/forecast-ets-function"
    role: "public source read for this page"
episodes: []
body_sections:
  - What it does
  - Try it
  - Quirks
  - How to implement
  - Version differences
  - Alternatives
  - Sources
family: null
role_in_family: null
---

# FORECAST.ETS

## What it does

`FORECAST.ETS` predicts a future value from a timed series, using the AAA version of exponential smoothing.

`FORECAST.ETS(target_date, values, timeline, [seasonality], [data_completion], [aggregation])`

- **`target_date`** is the point to predict, a date or number after the end of the history.
- **`values`** is the history.
- **`timeline`** is the matching dates or numbers, with a constant nonzero step. It need not be sorted.
- **`seasonality`** is 1 to detect the pattern (default), 0 for none, or a positive pattern length up to 8,760.
- **`data_completion`** is 1 to fill missing points with the average of their neighbours (default), or 0 to treat
  them as zeros. Up to 30% of points may be missing.
- **`aggregation`** combines values sharing a timestamp: 0 AVERAGE (default), then SUM, COUNT, COUNTA, MIN, MAX and MEDIAN.

## Try it

## Quirks

- `#NUM!`: the target precedes the end of the history, no constant step exists, or `seasonality` is invalid or above 8,760.
- `#N/A`: `values` and `timeline` differ in size.
- `#VALUE!`: Microsoft's timeline section lists this for duplicate timeline values, while its aggregation
  section says duplicates are combined. The two statements conflict, and no reading is recorded here to settle it.

## How to implement

No implementation exists yet. It needs the AAA exponential smoothing fit, the detection of a constant timeline step, seasonality
detection, the data-completion and aggregation rules, and recorded Excel answers to fix the optimiser's stopping
rules. Forecast values depend on those rules, so they must be measured and not assumed.

## Version differences

Microsoft says these functions are not available in Excel for the web, iOS or Android. No Excel answers for this function are recorded here, so no comparison between Excel builds exists.

## Alternatives

- `FORECAST.LINEAR` for a straight-line forecast.
- `TREND` and `GROWTH` for fitted series.
- The Forecast Sheet command, which builds on the same family.

## Sources

- [FORECAST.ETS function](https://support.microsoft.com/en-us/excel/functions/forecast-ets-function)
