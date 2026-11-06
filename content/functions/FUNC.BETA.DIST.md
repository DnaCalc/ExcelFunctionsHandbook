---
schema: efh.function-page/v1
function_id: FUNC.BETA.DIST
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0005
  - EV-DIST-0011
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
  The family's forward beta CDF and density, and the substrate the whole module rests on: the
  regularized incomplete beta function that F, t and the binomial identities are all expressed
  through.
---

# BETA.DIST

## What it does

`BETA.DIST(x,alpha,beta,cumulative,[A],[B])` evaluates a beta distribution between optional bounds A and B, defaulting to zero and one. The two shape parameters are positive. TRUE selects its cumulative probability; FALSE selects its density.

Changing the interval rescales the density by its width. It does not multiply a CDF by that width.

## Try it

`=BETA.DIST(5,2,1,TRUE,2,6)` is about `0.5625`. Five maps to `z=(5-2)/(6-2)=0.75`, and these shapes have CDF `z^2`.

`=BETA.DIST(5,2,1,FALSE,2,6)` is about `0.375`. The standard density `2*z` is divided by width four, giving `1.5/4`. The density is a different quantity from the cumulative probability.

## Quirks

Microsoft documents `#VALUE!` for nonnumeric arguments and `#NUM!` for nonpositive shapes, x outside the interval or equal bounds. The current helper requires ordered finite bounds `B>A`.

At an interval endpoint, mathematical density limits need explicit treatment. The current log-density body has no general endpoint shortcut: even the uniform shape-one density can encounter `0*log(0)` and return `#NUM!`, although its mathematical density is finite. This is a code limitation, rather than an additional Excel observation.

The current ordinary binary64 width calculation can overflow for extreme finite bounds. Keeping finite parameter validation separate from intermediate arithmetic is necessary for explaining that case.

## How to implement

Use D for ordinary binary64 storage.

1. Apply optional bounds and validate positive finite shapes, finite x and ordered finite bounds. Require x inside the interval.
2. Store `width=D(B-A)`, `offset=D(x-A)` and `z=D(offset/width)`.
3. For cumulative mode, call the regularized lower beta helper. It handles z=0 and z=1 explicitly; otherwise it supplies `bratio(alpha,beta,z,D(1-z))` and returns the first ratio.
4. For density mode, the current body uses runtime logarithms and exponentials and the internal log-gamma routine. It computes `log_norm=logGamma(alpha)+logGamma(beta)-logGamma(alpha+beta)`, then `(alpha-1)*log(z)+(beta-1)*log(1-z)-log_norm-log(width)` with binary64 stores in that order, and exponentiates. These are internal leaves, not unconditional calls to the worksheet LN, EXP or GAMMALN wrappers.
5. Apply the finite-result and worksheet publication rules. Preserve the density’s current endpoint behaviour when describing this code; handling mathematical limits is a separate implementation choice.

For the examples z is three quarters. The beta(2,1) CDF is `z^2=9/16`; the density on the chosen interval is `2*z/4=3/8`. Stored numerical approximations can differ in their low bits from those rational values.

Integer shapes use the same current beta-ratio route, rather than a binomial-sum shortcut. The complement construction, log-density arithmetic, endpoint cases and inner approximations have distinct numerical scopes. The problem report still records beta-distribution mismatches; a general beta library does not by itself reproduce Excel’s private program.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 2,805 |
| 20430 | Windows, 64-bit, CPU not recorded | 17 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.BETA.DIST.json).

## Alternatives

[BETADIST](FUNC.BETADIST.md) is the compatibility CDF-only spelling; [BETA.INV](FUNC.BETA.INV.md) maps probability to a value. [F.DIST](FUNC.F.DIST.md) and [T.DIST](FUNC.T.DIST.md) use related beta ratios with different argument constructions.

## Sources

- [Microsoft: BETA.DIST function](https://support.microsoft.com/en-us/excel/functions/beta-dist-function), opened 7 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/beta_gamma_stats_family.rs); implementation steps and the limits identified in the text.
- [Beta ratio and internal log-gamma](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/special_math_common.rs); numerical helpers used by the wrapper.
- [Remaining numerical failures](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/docs/generated/problem-functions.md).
