---
schema: efh.function-page/v1
function_id: FUNC.GAMMA.INV
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0003
  - EV-DIST-0016
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Gamma_Inv method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.gamma_inv"
    role: "documented parameters, the round-trip statement, the error conditions, and the iterative-search / 100-iteration / #N/A remark"
  - work: "Microsoft Support — GAMMA.INV function"
    locator: "https://support.microsoft.com/en-us/office/gamma-inv-function-74991443-c2b0-4be5-aaab-1aa4d71fbb18"
    role: "the worksheet-surface documentation page; not retrievable at curation time (the host refused the request)"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapters 6 and 26"
    locator: "6.5 — incomplete gamma; 26.4 — chi-square percentage points"
    role: "the forward function this one inverts, and the classical quantile relations"
  - work: "D. J. Best and D. E. Roberts, Algorithm AS 91: the percentage points of the chi-squared distribution (1975)"
    locator: null
    role: "the classical seeded Newton scheme for gamma/chi-square quantiles"
  - work: "DCDFLIB / CDFLIB (Brown, Lovato, Russell)"
    locator: null
    role: "the standard inverse-by-bracketed-search design for distribution quantiles"
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
  The gamma quantile: a root-finder over the module's own forward cumulative surface, and the
  member whose accuracy is documented to be bounded by that forward's accuracy.
---

# GAMMA.INV

## What it does

`GAMMA.INV(probability,alpha,beta)` returns a gamma-distribution quantile for positive shape alpha and positive scale beta. It seeks the x whose lower cumulative probability is the requested p. Beta scales x’s units, rather than supplying an exponential rate.

## Try it

`=GAMMA.INV(0.5,1,2)` is about `1.3862943611`. The mathematical shape-one CDF is `1-exp(-x/2)`, so its median solves `x=-2*ln(0.5)`.

`=GAMMA.INV(0.5,1,4)` is about `2.7725887222`. Doubling the scale doubles the mathematical quantile, though the current search has its own stored path.

## Quirks

Microsoft documents `#VALUE!` for a text argument, `#NUM!` for nonpositive shape or scale and for p outside zero to one. The current helper returns zero at p=0 and `#NUM!` at p=1, where there is no finite mathematical gamma quantile.

The opened modern page documents an iterative search limit of 64 tries followed by `#N/A`; the legacy GAMMAINV page describes 100 tries. This documentation difference is not a measured change between named Excel builds, and the current helper uses its own bracket and adjacent-value search.

The current inverse callback is the generic lower gamma ratio. It does not call the public GAMMA.DIST wrapper, whose half-shape and shape-one cases have separate ERF and exponential paths.

## How to implement

1. Validate finite positive alpha and beta and finite p. Return zero for p=0; reject p<0 or p>=1 under the current endpoint rule.
2. With ordinary binary64 operations, initialize `lo=0` and `hi=beta*max(alpha+10*sqrt(alpha)+10,1)`.
3. Evaluate `F(x)=regularized_gamma_p(alpha,D(x/beta))`. While F(hi) is below p, double hi. The current source permits 200 expansion checks; extreme-parameter bracketing and nonfinite callbacks remain explicit limitations.
4. Bisect the nonnegative binary64 encodings of lo and hi, as in the [shared ordered-value inverse recipe](FUNC.CHISQ.INV.RT.md#how-to-implement). If F(mid)>=p keep the midpoint as the upper endpoint; otherwise keep it as lower. Return the upper value when the endpoint encodings are adjacent.
5. Apply worksheet numeric publication. The candidate searches x on its scaled interval; it does not simply solve once at scale one and multiply an already rounded answer by beta.

For alpha=1, beta=2 and p=0.5, the mathematical equation gives about `1.38629436111989`. That explains the example. Its numerical candidate still depends on the generic forward callback and upper-adjacent return, rather than an exact inverse exponential formula.

A bracketed Newton method or a residual-based stopping rule is another reasonable mathematical solver. Changing the callback to the public CDF’s special branches is also a separate numerical candidate. Forward-ratio errors, conditioning in the tails, bracket extremes and remaining inverse mismatches remain open; no whole-function Handbook suite is published.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 204 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.GAMMA.INV.json).

## Alternatives

[GAMMA.DIST](FUNC.GAMMA.DIST.md) supplies the forward distribution. [GAMMAINV](FUNC.GAMMAINV.md) is the compatibility spelling. [CHISQ.INV](FUNC.CHISQ.INV.md) uses gamma shape df/2 and scale two; [BETA.INV](FUNC.BETA.INV.md) inverts a bounded distribution.

## Sources

- [Microsoft: GAMMA.INV function](https://support.microsoft.com/en-us/excel/functions/gamma-inv-function), opened 7 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/beta_gamma_stats_family.rs); implementation steps and the limits identified in the text.
- [Generic gamma ratio and ordered-value search](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/special_math_common.rs); callback and adjacent binary64 return.
- [Remaining numerical failures](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/docs/generated/problem-functions.md).
