---
schema: efh.function-page/v1
function_id: FUNC.HARMEAN
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

# HARMEAN

## What it does

`HARMEAN(number1, [number2], ...)` is the reciprocal of the mean of reciprocals. It is useful when equal-sized portions contribute rates, such as two equal distances travelled at different speeds.

## Try it

`=HARMEAN(30,60)` returns `40`. One distance at 30 and the same distance at 60 take a combined time proportional to `1/30+1/60=1/20`; two distances divided by that time give 40.

## Quirks

Every admitted value must be positive; zero or a negative value gives `#NUM!`. Referenced text, logical values and blanks are ignored, while numeric zero is included and then rejected. Direct numeric text and logical constants can be admitted.

## How to implement

Collect the admitted numbers and reject any value at or below zero. Sum their reciprocals, then divide the count by that sum. For `30,60`, the reciprocal sum is `0.05`, so `2/0.05=40`. Very small inputs can make reciprocals overflow; a scaled reciprocal sum is a mathematical robustness option, but requires fresh comparisons before being presented as an Excel compatibility route.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 19,214 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.HARMEAN.json).

## Alternatives

[AVERAGE](FUNC.AVERAGE.md) suits equally weighted observations; [GEOMEAN](FUNC.GEOMEAN.md) suits multiplicative factors. Use a weighted harmonic mean when the portions are unequal.

## Sources

- [Microsoft: HARMEAN function](https://support.microsoft.com/en-us/excel/functions/harmean-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/statistics.rs); implementation steps and the limits identified in the text.
