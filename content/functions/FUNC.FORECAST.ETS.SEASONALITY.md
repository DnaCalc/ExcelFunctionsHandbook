---
schema: efh.function-page/v1
function_id: FUNC.FORECAST.ETS.SEASONALITY
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "FORECAST.ETS.SEASONALITY function"
    locator: "https://support.microsoft.com/en-us/excel/functions/forecast-ets-seasonality-function"
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

# FORECAST.ETS.SEASONALITY

## What it does

`FORECAST.ETS.SEASONALITY` returns the length of the repeating pattern that exponential smoothing detects in a timed series.

`FORECAST.ETS.SEASONALITY(values, timeline, [data_completion], [aggregation])`

Use it to see what `FORECAST.ETS` would detect by default, or to set `seasonality` explicitly.

## Try it

## Quirks

The timeline, data-completion and aggregation rules follow [FORECAST.ETS](FUNC.FORECAST.ETS.md).

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

- [FORECAST.ETS.SEASONALITY function](https://support.microsoft.com/en-us/excel/functions/forecast-ets-seasonality-function)
- [FORECAST.ETS function](https://support.microsoft.com/en-us/excel/functions/forecast-ets-function)
