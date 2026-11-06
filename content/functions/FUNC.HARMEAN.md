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

`HARMEAN(number1, [number2], ...)` returns `n / sum(1/x)` for positive observations. It is useful for rates attached to equal-sized portions. Two equal distances travelled at different speeds have equal distance weights, not equal time weights; averaging their speeds arithmetically answers a different question.

## Try it

`=HARMEAN(30,60)` returns `40`. Travelling 60 km at 30 km/h takes two hours; another 60 km at 60 km/h takes one hour. Total distance 120 km divided by total time three hours is 40 km/h.

`=AVERAGE(30,60)` returns `45`. That is the average speed for equal amounts of time at the two speeds. The difference comes from what receives equal weight.

## Quirks

Every admitted value must be positive. A numeric zero or negative value gives `#NUM!`; a blank cell is skipped rather than treated as zero. References and arrays also skip logical values and text, including numeric-looking text. Direct numeric text and logical constants can be converted, so direct `FALSE` becomes a rejected zero.

Worksheet errors propagate, and unconvertible direct text gives `#VALUE!`. The current code checks direct scalars before reading held values, returns `#N/A` when collection finds no numbers, and gives `#NUM!` for a nonfinite reciprocal sum. Those last two choices are implementation behaviour, not claims established by Microsoft’s published error list.

## How to implement

The mathematical identity `n/sum(1/x)` is not the current operation tree. Use `R` for the [stored 64-bit-significand operation](../model/09-staged-arithmetic.md).

1. Preserve whether each value is direct, a reference cell or an array element; collect it using the rules above. Reject any admitted `x<=0`.
2. Starting from `S=0`, visit the admitted values in order. Store `r=R(1/x)`, then store `S=R(S+r)` for each one.
3. After the empty-set and nonfinite-sum checks, store the reciprocal mean `m=R(S/n)`.
4. Return `R(1/m)`, subject to worksheet number publication. Replacing these last two divisions by `R(n/S)` changes the intermediate rounding history.

For `30,60`, the mathematical reciprocal sum is `1/20`; the reciprocal mean is `1/40`, whose reciprocal is 40. The current staged route also publishes 40. These worked numbers explain the route; they do not identify all Excel rounding stages on every input.

Small positive inputs can make a reciprocal overflow. Scaling the reciprocal sum can help a mathematical implementation, but its different arithmetic needs separate Excel comparisons before being offered as a compatibility route.

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
