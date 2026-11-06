---
schema: efh.function-page/v1
function_id: FUNC.CHISQ.INV.RT
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

# CHISQ.INV.RT

## What it does

`CHISQ.INV.RT(probability, deg_freedom)` finds the chi-square value whose right-tail probability is the requested value.

## Try it

`=CHISQ.INV.RT(0.5,2)` is about `1.38629436111989`. At two degrees of freedom, a half right tail solves `exp(-x/2)=0.5`; the same point also has a half left CDF.

## Quirks

Degrees of freedom are truncated. Nonnumeric arguments give `#VALUE!`; invalid degrees or probabilities give `#NUM!`. The current helper accepts truncated degrees from one through `10^10` and requires `0<p<1`; it rejects both probability endpoints.

The opened Microsoft page describes an out-of-range probability test below zero or above one, and degrees below one. It also documents `#N/A` after 64 unsuccessful search iterations. Those statements are documentation, rather than evidence that the current shared search has the same guards or iteration counter. The companion legacy CHIINV page documents 100 iterations; this wording difference does not establish a measured change between Excel builds.

## How to implement

The current candidate inverts the decreasing regularized upper gamma ratio `Q(k/2,x/2)` directly. Do not first replace p by `1-p`: a tiny p can disappear in that subtraction.

1. Truncate and validate k and require `0<p<1`. Let `H(x)=-Q(k/2,x/2)` and `target=-p`, making H increasing.
2. Set `lo=0,hi=max(k,1)`. While `H(hi)<target`, double hi, with the source’s limit of 200 expansion checks.
3. For nonnegative finite binary64 values, their unsigned 64-bit encodings have the same order as their values. Let L and U be the encodings of lo and hi. While `U-L>1`, take the integer midpoint `M=L+(U-L)//2`, decode it to x, and evaluate H(x). If `H(x)>=target`, set U=M; otherwise set L=M.
4. Decode and return U. This chooses the upper endpoint of the maintained adjacent-value bracket, rather than a floating arithmetic midpoint or a nearest-root rounding rule.

At k=2 the mathematical right tail is `exp(-x/2)`. Setting it to one half gives about `1.38629436111989`. In the current code, however, this inverse callback remains the generic upper-gamma ratio; it does not call the public right-tail wrapper’s separate df=1 ERFC and df=2 EXP branches.

This is an implementable description of the current search, conditional on its numerical forward callback. That callback’s approximation and monotonic behaviour, its remaining mismatches and whole-domain Excel return choices are separate unresolved questions.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[CHISQ.DIST.RT](FUNC.CHISQ.DIST.RT.md) supplies the forward probability; [CHISQ.TEST](FUNC.CHISQ.TEST.md) returns a test p-value directly.

## Sources

- [Microsoft: CHISQ.INV.RT function](https://support.microsoft.com/en-us/excel/functions/chisq-inv-rt-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/chi_f_t_family.rs); implementation steps and the limits identified in the text.
- [Ordered-value inverse search](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/special_math_common.rs); `bisect_inverse`, upper-gamma callback and numerical limits.
