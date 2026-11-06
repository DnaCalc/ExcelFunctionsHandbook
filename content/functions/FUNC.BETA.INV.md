---
schema: efh.function-page/v1
function_id: FUNC.BETA.INV
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
family: beta_gamma_stats_family
role_in_family: >-
  The family's beta quantile: defined by inversion of BETA.DIST rather than by a formula of its
  own, and therefore the surface whose accuracy is bounded by the forward function's.
---

# BETA.INV

## What it does

`BETA.INV(probability,alpha,beta,[A],[B])` returns the value whose beta CDF is the requested probability. Shapes alpha and beta are positive. Bounds default to zero and one; supplying other bounds rescales the value interval, while probability remains on zero to one.

## Try it

`=BETA.INV(0.25,2,1,2,6)` is about `4`. On the standard interval, these shapes give CDF `z^2`; probability one quarter therefore selects z one half. Rescaling gives `2+0.5*4=4`.

`=BETA.INV(0.25,1,1,2,6)` is about `3`: the uniform shape-one case has CDF z itself.

## Quirks

Microsoft documents `#VALUE!` for nonnumeric arguments and `#NUM!` for nonpositive shapes or probability at/below zero or above one. The current implementation also rejects unordered bounds.

The current helper’s validation admits zero probability but has no zero-endpoint shortcut before searching adjacent binary64 values. On the standard interval this can leave a subnormal value that its publisher refuses; on two to six the affine addition rounds it to two. This code-path behaviour is not a measurement of Excel’s endpoint.

The legacy BETAINV shares the current helper. Shared code does not establish that all Excel builds compute the two names identically.

## How to implement

1. Apply optional bounds and validate finite positive shapes, finite ordered bounds and probability under the chosen endpoint contract.
2. Use the current regularized lower beta callback F(z), including its stored complement `1-z` and internal beta-ratio approximation.
3. Search z between zero and one by bisecting the ordered nonnegative binary64 encodings. At each midpoint, keep the upper half when F(z) is below p and the lower half otherwise. Return the upper endpoint once adjacent, as in [BETAINV’s detailed search recipe](FUNC.BETAINV.md#how-to-implement).
4. Store `width=B-A`, `scaled=z*width` and `result=A+scaled` with ordinary binary64 arithmetic and separate multiplication/addition. Apply the worksheet’s numeric publication rule.

In the first example, z is about one half and the last three stages give width four, scaled two and result four. An arithmetic-midpoint search, a residual tolerance, an FMA or an explicit endpoint shortcut can change stored results and boundary behaviour.

At p=0 the usual strict lower-bracket invariant already fails at z=0. The current search can select the smallest positive binary64 value rather than exactly zero. A mathematical inverse can handle that endpoint explicitly; reproducing this code instead requires retaining its search and publication stages.

Inverse accuracy depends on the forward surface being searched. A round trip through that same surface can hide a shared error. The beta approximation, extreme affine scaling and remaining inverse mismatches are still open; there is no published whole-function Handbook suite.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[BETA.DIST](FUNC.BETA.DIST.md) evaluates the forward CDF or density. [BETAINV](FUNC.BETAINV.md) is the compatibility spelling. [F.INV](FUNC.F.INV.md) and [T.INV](FUNC.T.INV.md) use related beta-ratio mathematics with distinct wrappers.

## Sources

- [Microsoft: BETA.INV function](https://support.microsoft.com/en-us/excel/functions/beta-inv-function), opened 7 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/beta_gamma_stats_family.rs); implementation steps and the limits identified in the text.
- [Beta callback and ordered-value inverse](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/special_math_common.rs); `regularized_beta` and `bisect_inverse`.
