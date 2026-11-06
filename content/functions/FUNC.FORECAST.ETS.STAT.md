---
schema: efh.function-page/v1
function_id: FUNC.FORECAST.ETS.STAT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "FORECAST.ETS.STAT function"
    locator: "https://support.microsoft.com/en-us/excel/functions/forecast-ets-stat-function"
    role: "public source read for this page"
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

# FORECAST.ETS.STAT

## What it does

`FORECAST.ETS.STAT` returns one statistic of the exponential smoothing fit.

`FORECAST.ETS.STAT(values, timeline, statistic_type, [seasonality], [data_completion], [aggregation])`

Microsoft lists eight statistics in this order, for `statistic_type` 1 to 8:

| Type | Statistic |
|---|---|
| 1 | Alpha, the level smoothing parameter |
| 2 | Beta, the trend smoothing parameter |
| 3 | Gamma, the seasonal smoothing parameter |
| 4 | MASE |
| 5 | SMAPE |
| 6 | MAE |
| 7 | RMSE |
| 8 | Step size detected in the timeline |

## Try it

## Quirks

The page lists the statistics without numbering them, so the numbers above come from the list order and the stated range. It also describes MAE in the words it uses for SMAPE. The series rules follow [FORECAST.ETS](FUNC.FORECAST.ETS.md).

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

- [FORECAST.ETS.STAT function](https://support.microsoft.com/en-us/excel/functions/forecast-ets-stat-function)
- [FORECAST.ETS function](https://support.microsoft.com/en-us/excel/functions/forecast-ets-function)
