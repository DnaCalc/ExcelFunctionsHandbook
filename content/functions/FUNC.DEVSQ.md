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

`DEVSQ(number1, [number2], ...)` returns the sum of squared distances from the data’s mean. It supplies a centered variance numerator without choosing either the sample or population divisor. If the observations are lengths, the result has squared-length units.

## Try it

`=DEVSQ(2,4,6)` returns `8`: the mean is four and the squared deviations are four, zero and four.

## Quirks

Referenced or array text, logical values and blanks are skipped; a numeric zero is an observation. Direct logical values and numeric text can be converted. Worksheet errors propagate, and unconvertible direct text gives `#VALUE!`. In the current code, direct scalar errors are checked before errors reached through a reference or array.

The current implementation returns `#NUM!` for no collected numbers. A single number has zero deviation from its own mean and returns zero. Adding a constant to all observations leaves the mathematical answer unchanged, but a floating-point implementation can lose that property at extreme magnitudes.

## How to implement

The current code uses a centered pass with a particular mixture of operations:

1. Collect the numeric observations in their argument and traversal order. Reject an empty collection with `#NUM!`.
2. Sum the observations in ordinary binary64 and divide that sum by `n` in binary64 to obtain `mean`.
3. For each observation, store the ordinary binary64 subtraction `d=x-mean`.
4. Compute the square using `R(d*d)`, where `R` rounds to a 64-bit significand and then stores to binary64. Replace a square below the minimum normal binary64 value by zero.
5. Add these squares in ordinary binary64 and publish the sum. Do not divide it by `n` or `n-1`.

For `2,4,6`, the sum is 12 and the mean is four. Differences `-2,0,2` give squares `4,0,4` and total eight. By comparison, `SUMSQ(2,4,6)=56` squares the uncentered inputs.

A direct calculation `sum(x*x)-sum(x)^2/n` is mathematically equivalent but can subtract two large, nearly equal quantities. It also has a different operation tree from this code. The stated mixed-store route is an implementation recipe; no whole-domain Excel stage identification is claimed.

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
