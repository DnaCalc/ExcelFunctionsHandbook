---
schema: efh.function-page/v1
function_id: FUNC.FORECAST.ETS.CONFINT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "FORECAST.ETS.CONFINT function"
    locator: "https://support.microsoft.com/en-us/excel/functions/forecast-ets-confint-function"
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

# FORECAST.ETS.CONFINT

## What it does

`FORECAST.ETS.CONFINT` gives the width of a confidence interval around a `FORECAST.ETS` prediction.

`FORECAST.ETS.CONFINT(target_date, values, timeline, [confidence_level], [seasonality], [data_completion], [aggregation])`

The arguments are those of `FORECAST.ETS`, with `confidence_level` added. It takes the same timeline rules.

## Try it

## Quirks

The series rules and errors follow [FORECAST.ETS](FUNC.FORECAST.ETS.md). The two share one fit, so a change to any shared argument moves both results.

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

- [FORECAST.ETS.CONFINT function](https://support.microsoft.com/en-us/excel/functions/forecast-ets-confint-function)
- [FORECAST.ETS function](https://support.microsoft.com/en-us/excel/functions/forecast-ets-function)
