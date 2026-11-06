---
schema: efh.function-page/v1
function_id: FUNC.TTEST
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

# TTEST

## What it does

`TTEST(array1,array2,tails,type)` returns a t-test probability for two samples. Type one is paired, two assumes equal variances, and three uses the unequal-variance model. Tails is one or two. This is the compatibility name.

## Try it

`=TTEST({1,2,3},{0,2,4},2,3)` returns `1` under the current route: both means are two, giving t=0 with a positive standard error. One tail returns one half.

## Quirks

Type and tail selectors are truncated. Invalid selectors give `#NUM!`; nonnumeric selectors give `#VALUE!`. A paired test needs matched data-point counts or gives `#N/A`. The result is a probability associated with the observed statistic, not the probability that the underlying means are equal.

## How to implement

Collect numeric samples while preserving paired positions for type one. For paired data compute differences, their mean and sample standard error. For type two use a pooled variance and degrees `n1+n2-2`; for type three use separate variance/count terms and Welch’s degree formula. Form the absolute t statistic and evaluate the one- or two-tail t probability. The current helper uses its own centered binary64 variance calculation, with partly unresolved beta-ratio leaves. Zero standard error and too-small samples need explicit errors.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[T.TEST](FUNC.T.TEST.md) is the modern spelling; [TDIST](FUNC.TDIST.md) accepts a supplied t statistic; [F.TEST](FUNC.F.TEST.md) compares variances.

## Sources

- [Microsoft: TTEST function](https://support.microsoft.com/en-us/excel/functions/ttest-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/statistical_tests_family.rs); implementation steps and the limits identified in the text.
