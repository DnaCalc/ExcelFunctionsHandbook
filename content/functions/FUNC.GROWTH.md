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

`GROWTH(known_ys,[known_xs],[new_xs],[const])` fits an exponential relationship and returns a prediction for each new observation. With one predictor the model is `y=b*m^x`; with several, it is `y=b*m1^x1*m2^x2*...`.

Omitting known x values substitutes positions one, two and so on. Omitting new x values predicts at the training positions. This second use returns fitted values, which need not equal noisy observed y values.

## Try it

`=GROWTH({3,9,27},{1,2,3},{4,5})` returns approximately `{81,243}`. The fitted mathematical model is `3^x`, so each step triples the prediction.

To see the role of the constant, use `=GROWTH({6,18,54},{1,2,3},{0,4})`. The mathematical model is `2*3^x`, giving approximately `{2,162}`. With the final `const` argument FALSE, the prediction at x=0 is one because the fitted multiplier b is fixed at one.

A two-predictor example is `=GROWTH({2;6;10;30},{0,0;1,0;0,1;1,1},{2,1})`, approximately `90`. The observations fit `2*3^x1*5^x2`; at `(2,1)`, the prediction is `2*9*5`.

## Quirks

Known y values must be positive; zero or a negative observation gives `#NUM!`. The predictor layout matters when there is more than one independent variable:

| Known y layout | Predictor layout |
|---|---|
| One column of observations | One predictor per column of known/new x data |
| One row of observations | One predictor per row of known/new x data |

With multiple predictors, known y must be a vector. New data needs the same number of predictors as training data. For one predictor, Microsoft also allows equally shaped known x and y ranges of other shapes.

`const=FALSE` fits a new model with b fixed at one: it sets the log-space intercept to zero and adjusts the slopes. It does not take a fit with an intercept and simply remove its constant afterward. Duplicate or nearly dependent predictor columns require a rank decision, which can affect coefficients and predictions.

## How to implement

1. Prepare the observation vector, predictor columns and requested output layout. Apply omitted-x defaults before fitting.
2. Validate positive finite y values and obtain each stored public `LN(y)` result.
3. Fit a linear least-squares model to those logarithms. With a constant, the current code centres predictor columns and the transformed response on their forward-sum means. It uses a pivoted Householder QR fit, selecting the largest remaining predictor-column squared norm first; the constant column keeps its own position. Without a constant, it fits the raw transformed data with zero intercept.
4. For each new predictor vector, start `P=1`. In predictor order, compute the ordinary binary64 product `u_j=coefficient_j*x_j`, obtain the public `EXP(u_j)`, and multiply it into P with ordinary binary64 multiplication.
5. Return `EXP(intercept)*P`, in the requested shape. Do not replace this prediction tree by `EXP(intercept+sum(coefficient_j*x_j))`: it stores different intermediates.

For the two-predictor example, the mathematical log coefficients are `LN(2)`, `LN(3)` and `LN(5)`. At `(2,1)`, the separate exponentials represent factors two, nine and five, whose product is ninety. The calculated coefficients and exponentials can differ in their last bits from those mathematical values.

The source retains specific reflector stores and structural/numerical rank decisions, with remaining accumulator-layout questions. A generic QR library supplies a mathematical alternative, not the complete measured Excel fit. The outer prediction tree above is distinct from those unresolved fitting details, and no whole-function Handbook suite is published.

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
