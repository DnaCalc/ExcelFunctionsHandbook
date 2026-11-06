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

The arrays need matching shapes; a mismatch gives `#N/A` in the current wrapper. A one-cell comparison also gives `#N/A` because it supplies no positive degree count. The output is a tail probability, rather than the chi-square statistic itself.

The current wrapper requires a numeric value at every paired position and returns `#VALUE!` for text, logical or blank cells; it does not compact those cells into a shorter sample. It propagates a direct argument error before comparing shapes, then checks cell errors during traversal. A nonpositive expected count gives `#NUM!` in this code.

For an independence table, calculate expected counts from row total times column total divided by grand total before calling the function. The function takes those expected values as inputs. Its p-value is conditional on the chosen model and expected table.

## How to implement

1. Resolve the two input grids and retain their row and column counts. Propagate a direct scalar argument error before a shape mismatch. Reject unequal shapes.
2. Set degrees of freedom to `(rows-1)*(columns-1)` when both dimensions exceed one, `columns-1` for one row or `rows-1` for one column. Reject zero degrees.
3. Traverse paired positions in row order. Propagate a cell error, require two numbers and reject a nonpositive expected value under the current rules.
4. With ordinary binary64 stores, form `delta=actual-expected`, `square=delta*delta`, `contribution=square/expected` and `statistic=statistic+contribution` from an initial zero.
5. Evaluate the chi-square right tail at that stored statistic and degree count.

The starter example contributes `4/10=0.4` from each category, giving statistic 0.8 and one degree. For an independence layout `{12,8;8,12}`, both row totals and both column totals are twenty and the grand total forty, so every expected count is ten. Its four contributions total 1.6, still with one degree because `(2-1)*(2-1)=1`.

Keep admission, statistic reduction and the numerical right-tail routine as separate stages. The loop describes current code; the forward distribution’s last bits and other recorded residuals remain unresolved.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 5 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.CHISQ.TEST.json).

## Alternatives

[CHISQ.DIST.RT](FUNC.CHISQ.DIST.RT.md) evaluates a supplied statistic; [CHISQ.INV.RT](FUNC.CHISQ.INV.RT.md) finds a critical value. [F.TEST](FUNC.F.TEST.md) compares sample variances.

## Sources

- [Microsoft: CHISQ.TEST function](https://support.microsoft.com/en-us/excel/functions/chisq-test-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/statistical_tests_family.rs); implementation steps and the limits identified in the text.
