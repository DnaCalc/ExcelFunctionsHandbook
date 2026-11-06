---
schema: efh.function-page/v1
function_id: FUNC.BETADIST
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

# BETADIST

## What it does

`BETADIST(x, alpha, beta, [A], [B])` returns the cumulative beta probability up to x. The two positive shape parameters control the distribution, and optional bounds A and B default to zero and one. This is a compatibility name.

## Try it

`=BETADIST(3,1,1,2,6)` is about `0.25`. The shape-one beta is uniform on two to six, and three is one quarter of the way across.

`=BETADIST(4,2,1,2,6)` is also about `0.25`. Here four is halfway across the interval, but the transformed CDF is `z^2`, rather than z. Shape parameters change the distribution within the same bounds.

## Quirks

Nonpositive shapes and invalid bounds give `#NUM!`; nonnumeric arguments give `#VALUE!`. An x outside the bounds gives `#NUM!`. This old spelling has no density/CDF switch: it is cumulative.

A and B change the value interval, rather than the probability scale. A CDF still ranges from zero to one, and the uniform example’s density on two to six would be one quarter, not one. The current affine normalization uses ordinary binary64 subtraction and division; extreme finite bounds can overflow an intermediate width.

## How to implement

1. Validate positive finite alpha and beta and finite ordered bounds `B>A`. Apply omitted defaults A=0 and B=1. Require a finite x inside those bounds under the current helper’s rules.
2. With D meaning ordinary binary64 storage, form `width=D(B-A)`, `offset=D(x-A)` and `z=D(offset/width)`.
3. Evaluate the regularized lower beta ratio at z. The current `regularized_beta` helper returns exact zero and one for z=0 and z=1; otherwise it computes `w=D(1-z)` and calls `bratio(alpha,beta,z,w)`.
4. Return the helper’s first ratio under the worksheet number-publication policy. Keep that lower/upper-tail convention explicit.

For the uniform example, width is four, offset one and z one quarter. Shapes one and one give CDF z. For the second example, z is one half and shapes two and one give CDF `z^2`, one quarter.

This wrapper constructs its complement from the already stored z. It does not independently form `(B-x)/(B-A)` as some other distribution wrappers do for their paired arguments. A scaled affine transformation can improve extreme-input range, but changes the stored history.

The current code routes integer shapes through the same beta-ratio body as fractional shapes; its retained comparisons did not justify replacing them with an integer binomial-sum shortcut. The inner beta approximation, extreme affine cases and the remaining problem-report mismatches remain open. The wrapper recipe does not identify every private Excel approximation.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 28 |
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 294 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.BETADIST.json).

## Alternatives

[BETA.DIST](FUNC.BETA.DIST.md) is the current spelling. [BETAINV](FUNC.BETAINV.md) reverses the value/probability direction.

## Sources

- [Microsoft: BETADIST function](https://support.microsoft.com/en-us/excel/functions/betadist-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/beta_gamma_stats_family.rs); implementation steps and the limits identified in the text.
- [Beta ratio and ordered-value search](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/special_math_common.rs); `regularized_beta` and `bisect_inverse`.
