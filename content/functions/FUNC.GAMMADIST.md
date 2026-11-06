---
schema: efh.function-page/v1
function_id: FUNC.GAMMADIST
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

# GAMMADIST

## What it does

`GAMMADIST(x,alpha,beta,cumulative)` evaluates a gamma distribution with positive shape alpha and positive **scale** beta. TRUE returns the probability up to nonnegative `x`; FALSE returns a density. Beta has the same units as `x`. This is the compatibility spelling of GAMMA.DIST.

Shape one is the exponential waiting-time distribution with rate `1/beta`. Shapes `n/2` and scale two describe a chi-squared distribution with `n` degrees of freedom; these are mathematical relations, whose worksheet compositions can have different rounding histories.

## Try it

`=GAMMADIST(2,1,2,TRUE)` is about `0.632120558828558`. With shape one it is an exponential waiting-time CDF with scale two.

## Quirks

Negative `x` or nonpositive shape or scale gives `#NUM!`; nonnumeric numeric arguments give `#VALUE!`. A scale of two is not an exponential rate of two: for shape one it corresponds to rate one half.

The zero-input density needs a separate boundary decision. Mathematically its right-hand limit is zero for `alpha>1`, `1/beta` for `alpha=1`, and unbounded for `0<alpha<1`. The current density fallback evaluates a log expression at zero and can give `#NUM!`, including at shape one. Do not use a mathematical boundary value as an uncited claim about Excel’s actual return.

## How to implement

The current shared CDF route first forms the binary64 scale ratio `y=x/beta`:

1. At shape `alpha=0.5`, it calls the ERF.PRECISE body at the current binary64 `sqrt(y)`.
2. At shape one, it calls the exponential CDF with arguments `(y,1)`. Computing `EXPON.DIST(x,1/beta,TRUE)` instead forms a different rounded ratio/product.
3. Other shapes call the regularized lower gamma ratio `P(alpha,y)`.

The source retains modern-spelling composition controls for the first two branches, including non-dyadic scales. They do not identify every general-shape branch, and the shape-one **density** is not established by the CDF identity.

For positive `x` with shape other than one, the current density candidate forms stored `y=R(x/beta)` and `t=R(alpha*LN(y))`, then obtains an exponential prefix corresponding to `exp(t-y-logGamma(alpha+1))`. It finally multiplies that prefix by the separately stored ratio `R(alpha/x)`, with another stored multiplication. The outer ratio/product consumer has discriminating controls; the prefix producer and its return stage remain open. Shape one and zero input have separate fallback code.

For the example `x=2,alpha=1,beta=2`, `y=1`, so the mathematical CDF is `1-exp(-1)`, about `0.632120558828558`. That reduction explains the probability; substituting an integer-shape finite sum, a host gamma library or a reciprocal scale is another numerical candidate. The problem report retains general distribution mismatches, and no whole-function Handbook suite is published.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 6 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.GAMMADIST.json).

## Alternatives

[GAMMA.DIST](FUNC.GAMMA.DIST.md) is the current spelling; [EXPON.DIST](FUNC.EXPON.DIST.md) handles the shape-one waiting time; [CHISQ.DIST](FUNC.CHISQ.DIST.md) is a shape/scale specialization.

## Sources

- [Microsoft: GAMMADIST function](https://support.microsoft.com/en-us/excel/functions/gammadist-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/beta_gamma_stats_family.rs); implementation steps and the limits identified in the text.
