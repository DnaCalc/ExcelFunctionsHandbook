---
schema: efh.function-page/v1
function_id: FUNC.CHISQ.TEST
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

# CHISQ.TEST

## What it does

`CHISQ.TEST(actual_range, expected_range)` returns a chi-square right-tail test probability from observed and expected frequencies. Supply the expected counts explicitly; the function does not infer them from labels.

## Try it

`=CHISQ.TEST({12,8},{10,10})` is about `0.37109337`. The statistic is `(12-10)^2/10+(8-10)^2/10=0.8`, with one degree of freedom for a one-row, two-category comparison.

## Quirks

The two arrays need matching data-point shapes; differing counts give `#N/A`. A one-cell comparison is not admitted. Expected zero values make the statistic’s division undefined; the current implementation returns an error. The output is a tail probability, not the chi-square statistic itself.

## How to implement

Validate the aligned arrays and accumulate `sum((actual-expected)^2/expected)`. For r rows and c columns use `(r-1)*(c-1)` degrees of freedom when both exceed one; for a single row use `c-1`, and for a single column `r-1`. Pass the statistic to the chi-square right-tail routine. The example contributes 0.4 from each category. Keep shape/admission rules separate from summation and tail evaluation; the numerical implementation still has recorded residuals.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 5 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.CHISQ.TEST.json).

## Alternatives

[CHISQ.DIST.RT](FUNC.CHISQ.DIST.RT.md) evaluates a supplied statistic; [CHISQ.INV.RT](FUNC.CHISQ.INV.RT.md) finds a critical value. [F.TEST](https://handbook.dnacalc.com/functions/F.TEST.html) compares sample variances.

## Sources

- [Microsoft: CHISQ.TEST function](https://support.microsoft.com/en-us/excel/functions/chisq-test-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/statistical_tests_family.rs); implementation steps and the limits identified in the text.
