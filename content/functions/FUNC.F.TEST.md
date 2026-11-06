---
schema: efh.function-page/v1
function_id: FUNC.F.TEST
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

# F.TEST

## What it does

`F.TEST(array1,array2)` returns a two-tailed F-test probability for comparing sample variances. It returns a probability, rather than the variance ratio statistic.

## Try it

`=F.TEST({1,2,3},{2,3,4})` returns `1` under the current route. Both sample variances are one, so their ratio is one with equal degree counts.

## Quirks

Each sample needs at least two numeric observations and a nonzero variance, otherwise `#DIV/0!`. Text, logical values and blanks in references are ignored; zeros count. Equal sample variances do not imply equal sample means or identical observations.

## How to implement

Compute each sample variance and its count, put the larger variance in the numerator, and retain its matching degree count `n-1`. Evaluate the F right tail of the ratio, double it and cap at one. For the example both variances are one and both degree counts two. The current helper uses its own centered binary64 sample-variance calculation; it does not automatically inherit the separately recovered VAR.S computation tree. Tail and reduction discrepancies remain recorded.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 3 |
| 20228 | Windows, 64-bit-bit, CPU not recorded | 75 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.F.TEST.json).

## Alternatives

[F.DIST.RT](FUNC.F.DIST.RT.md) accepts a supplied ratio; [VAR.S](FUNC.VAR.S.md) reports sample variance; [T.TEST](FUNC.T.TEST.md) compares means under its selected model.

## Sources

- [Microsoft: F.TEST function](https://support.microsoft.com/en-us/excel/functions/f-test-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/statistical_tests_family.rs); implementation steps and the limits identified in the text.
