---
schema: efh.function-page/v1
function_id: FUNC.GROWTH
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
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

# GROWTH

## What it does

`GROWTH(known_ys,[known_xs],[new_xs],[const])` fits an exponential relationship and returns predicted y values. In one variable the model is `y=b*m^x`; multiple predictors multiply corresponding exponential factors. With omitted x values it uses positions one, two and so on.

## Try it

`=GROWTH({3,9,27},{1,2,3},{4,5})` returns approximately `{81,243}`. The model is `3^x`, so each step triples the prediction. The result is an array even though only two forecasts are requested.

## Quirks

Known y values must be positive; zero or a negative value gives `#NUM!`. Predictor and response shapes must agree. Const FALSE fixes b at one, which means fixing the intercept at zero in log space. It does not fix a slope at one. Omitted new_xs predicts at the supplied training x values.

## How to implement

Take the logarithm of admitted positive y values, form the design matrix with an intercept when requested, and solve the least-squares problem in log space. Use a rank-aware QR-style solver for a numerical implementation rather than unstable normal equations; then form each fitted log prediction and exponentiate. The current regression family retains specific centering, scaling, rank and exponential choices. Those stages and remaining last-bit discrepancies are not solved by the mathematical transformation alone. The example has exact log-linear structure, making it a useful control without establishing a general regression route.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20228 | Windows, 64-bit, CPU not recorded | 180 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.GROWTH.json).

## Alternatives

[LOGEST](FUNC.LOGEST.md) returns exponential-model coefficients; [TREND](FUNC.TREND.md) fits an additive linear model; [FORECAST.LINEAR](FUNC.FORECAST.LINEAR.md) makes a one-predictor linear forecast.

## Sources

- [Microsoft: GROWTH function](https://support.microsoft.com/en-us/excel/functions/growth-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/regression_forecast_family.rs); implementation steps and the limits identified in the text.
