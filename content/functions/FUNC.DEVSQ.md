---
schema: efh.function-page/v1
function_id: FUNC.DEVSQ
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

# DEVSQ

## What it does

`DEVSQ(number1, [number2], ...)` returns the sum of squared distances from the data’s mean. It is the unnormalised numerator of centered variance.

## Try it

`=DEVSQ(2,4,6)` returns `8`: the mean is four and the squared deviations are four, zero and four.

## Quirks

Referenced text, logical values and blanks are skipped; referenced zero counts. Direct logical values and numeric text can be admitted. An error or unconvertible direct text can produce an error. The sum has squared input units.

## How to implement

Collect numeric observations, compute their mean, subtract it from each value, square each difference, and add. Do not divide by the count. In DnaCalc’s current route the subtraction and accumulation use ordinary double operations; each square uses a stored x87 operation, and a square below the minimum normal binary64 value is replaced by zero. This is an implementation recipe, not a proof that all Excel input routes share those stages.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 23,096 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.DEVSQ.json).

## Alternatives

[VAR.S](FUNC.VAR.S.md) divides a variance numerator by `n-1`; [VAR.P](FUNC.VAR.P.md) uses `n`. [SUMSQ](FUNC.SUMSQ.md) squares the original values without centering.

## Sources

- [Microsoft: DEVSQ function](https://support.microsoft.com/en-us/excel/functions/devsq-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/statistics.rs); implementation steps and the limits identified in the text.
