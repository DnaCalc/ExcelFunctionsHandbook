---
schema: efh.function-page/v1
function_id: FUNC.COVARIANCE.P
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

# COVARIANCE.P

## What it does

`COVARIANCE.P(array1, array2)` measures population covariance: whether paired observations tend to move in the same direction. It averages products of deviations using divisor `n`. Covariance carries the product of the two input units.

## Try it

`=COVARIANCE.P({1,2,3},{2,4,6})` returns approximately `1.33333333333333`. The means are two and four; the paired deviation products are two, zero and two.

With an excluded position, `=COVARIANCE.P({1,"missing",3},{2,4,6})` returns `2` under the current paired collection rule. The second position is skipped on both sides; the remaining pairs are `(1,2)` and `(3,6)`.

## Quirks

The original arguments need the same number of cells; otherwise the current collector returns `#N/A`. It flattens them in traversal order and preserves positional alignment. If either side of a position is text, logical or blank, that position supplies no pair. Numeric zero still participates. Worksheet errors are read and propagated rather than silently removed with a nonnumeric partner.

An empty set of eligible pairs gives `#DIV/0!`; one pair has zero population covariance. Do not filter each side independently and zip the remaining lists: that can pair observations that never belonged together.

## How to implement

The current paired collector checks equal original cell counts, then walks both arguments together and retains only positions where both values are numeric. It checks the first argument’s value before the second’s at each position. This admission step belongs before either mean is calculated.

1. Let `n` be the number of retained pairs. Apply the empty-set and sample-count checks above.
2. Accumulate separate binary64 sums `Sx` and `Sy`, then store `mx=Sx/n` and `my=Sy/n`.
3. Starting from `C=0`, visit each retained pair and add the ordinary binary64 product `(x-mx)*(y-my)`.
4. Return `C/n`. The current helper uses ordinary binary64 operations here; no stored-stage identification for the whole Excel route is claimed.

For the excluded-position example, the retained means are two and four. Products `(-1)*(-2)` and `1*2` total four. Divide by two to obtain population covariance two. The count is two, not the three original positions.

Covariance retains both input scales: multiplying every `y` by ten multiplies the mathematical covariance by ten. [CORREL](FUNC.CORREL.md) normalizes that scale away. The exact-arithmetic relation does not certify the last bits of either implementation.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[CORREL](FUNC.CORREL.md) removes units and scales to a correlation coefficient. [COVARIANCE.S](FUNC.COVARIANCE.S.md) changes the denominator.

## Sources

- [Microsoft: COVARIANCE.P function](https://support.microsoft.com/en-us/excel/functions/covariance-p-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc pair collector](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/kit.rs), `pairs` and `PairScalars::SkipNonnumeric`.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/paired_stats_common.rs); implementation steps and the limits identified in the text.
