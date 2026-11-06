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

Type and tail selectors are truncated. Invalid selectors give `#NUM!`; nonnumeric selectors give `#VALUE!`. A paired test needs matched data-point counts or gives `#N/A`. Two independent-sample tests may use different counts.

TTEST forms a nonnegative statistic. Its one-tail result is the upper probability for that magnitude, so it is half the two-tail result; swapping the samples does not select a different directional hypothesis in this function. The output is not the probability that the population means are equal.

The current numerical helpers return `#DIV/0!` for too few admitted observations or zero standard error. Type three retains its fractional Welch degree count internally; passing that count through the public TDIST wrapper would truncate it and change the calculation.

## How to implement

Collect numeric observations, retaining aligned pairs for type one. Let `mean`, `s²` and n denote each sample’s arithmetic mean, sample variance and count. The current test helpers use forward binary64 means and centered squared-deviation sums; they do not call the separately recovered VAR.S computation tree.

| Type | Standard error SE | Degrees v |
|---|---|---|
| 1, paired differences d | `sqrt(s_d²/n)` | `n-1` |
| 2, pooled | `sqrt(s_p²*(1/n1+1/n2))`, with `s_p²=((n1-1)*s1²+(n2-1)*s2²)/(n1+n2-2)` | `n1+n2-2` |
| 3, unequal variances | `sqrt(a+b)`, with `a=s1²/n1,b=s2²/n2` | `(a+b)²/(a²/(n1-1)+b²/(n2-1))` |

For paired data set `t=abs(mean(d)/SE)`; otherwise set `t=abs((mean1-mean2)/SE)`. Reject zero SE. Retain fractional v for type three.

The current inner tail forms `t2=t*t`, `den=v+t2`, then separately stores `z=v/den` and `w=t2/den`. Evaluate `bratio(v/2,0.5,z,w).first`. This is the two-tail probability; divide it by two for one tail. Its paired beta arguments are not formed by computing `w=1-z`.

For a worked unequal-variance case, samples `{1,2,3}` and `{2,4,6}` have means two and four, variances one and four, and counts three each. Then `a=1/3,b=4/3`, SE is about `1.2909944487`, t about `1.5491933385`, and v is `50/17`, about `2.9411764706`. This shows why rounding v to an integer is a different test. In the starter example the means are equal, so t is zero and the two-tail probability one.

These formulas specify the current composition and operation order. The beta-ratio approximation and remaining numerical discrepancies remain open; a mathematical t-test implementation alone does not identify Excel’s private program.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[T.TEST](FUNC.T.TEST.md) is the modern spelling; [TDIST](FUNC.TDIST.md) accepts a supplied t statistic; [F.TEST](FUNC.F.TEST.md) compares variances.

## Sources

- [Microsoft: TTEST function](https://support.microsoft.com/en-us/excel/functions/ttest-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/statistical_tests_family.rs); implementation steps and the limits identified in the text.
