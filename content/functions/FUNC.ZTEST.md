---
schema: efh.function-page/v1
function_id: FUNC.ZTEST
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

# ZTEST

## What it does

`ZTEST(array,x,[sigma])` returns a one-tailed normal-test probability for a hypothesised population mean x. Supply a known population standard deviation sigma, or let the function use sample standard deviation. This is the compatibility name.

## Try it

`=ZTEST({1,2,3},2,1)` returns `0.5`: the sample mean equals the hypothesis. If the sample mean exceeds x, this one-tail result is below one half.

## Quirks

The public page documents `#N/A` for an empty array. The current helper has its own empty/nonfinite validation, so retain that error distinction in comparisons. A supplied scale must be positive, and an omitted scale needs enough sample data. A two-tailed test uses `2*MIN(p,1-p)` under the stated worksheet composition, not simply `2*p` in every case.

## How to implement

Compute the numeric count and mean. If sigma is omitted, obtain sample standard deviation from the recovered variance family. The recorded current combination is `z=(x-mean)*sqrt(n)/sigma`, then the standard-normal **left CDF** at z. Do not rewrite it as a right-tail subtraction on `(mean-x)/(sigma/sqrt(n))`: mathematically equivalent forms have different stored results. The equal-mean example gives z=0 and probability one half. Inner normal tails still depend on the open ERFC factor.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[Z.TEST](FUNC.Z.TEST.md) is the modern spelling; [T.TEST](FUNC.T.TEST.md) supplies a t-based comparison; [NORM.S.DIST](FUNC.NORM.S.DIST.md) evaluates a normal CDF directly.

## Sources

- [Microsoft: ZTEST function](https://support.microsoft.com/en-us/excel/functions/ztest-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/confidence_test_family.rs); implementation steps and the limits identified in the text.
