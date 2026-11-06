---
schema: efh.function-page/v1
function_id: FUNC.GAMMA.DIST
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0002
  - EV-DIST-0011
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Gamma_Dist method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.gamma_dist"
    role: "documented parameters, the cumulative flag, the three error conditions, the alpha=1 exponential case, the chi-square identity, and the Erlang remark"
  - work: "Microsoft Support — GAMMA.DIST function"
    locator: "https://support.microsoft.com/en-us/office/gamma-dist-function-9b6f1538-d11c-4d5f-8966-21f6a2201def"
    role: "the worksheet-surface documentation page; not retrievable at curation time (the host refused the request)"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapters 6 and 26"
    locator: "6.5 — incomplete gamma function; 26.4 — the chi-square distribution and its relation to the incomplete gamma"
    role: "the mathematics of the regularized incomplete gamma function and the chi-square link"
  - work: "W. Gautschi, 'A computational procedure for incomplete gamma functions' (TOMS, 1979)"
    locator: null
    role: "the standard series/continued-fraction split for the incomplete gamma"
  - work: "NSWC Library / DCDFLIB GRATIO"
    locator: null
    role: "the incomplete-gamma-ratio routine family named in the upstream identification work"
  - work: "L. Knüsel and B. D. McCullough, published assessments of Excel's statistical distribution functions"
    locator: null
    role: "the standing external record on Excel's distribution accuracy"
episodes: []
body_sections:
  - What it does
  - Try it
  - Quirks
  - How to implement
  - Version differences
  - Alternatives
  - Notes
  - Sources
family: beta_gamma_stats_family
role_in_family: >-
  The forward gamma distribution, both density and cumulative, and the surface the module's
  gamma inverse roots; its cumulative mode routes through the shared regularized incomplete
  gamma while its density mode does not.
---

# GAMMA.DIST

## What it does

`GAMMA.DIST(x,alpha,beta,cumulative)` evaluates a gamma distribution with positive shape alpha and positive scale beta at nonnegative x. TRUE returns a cumulative probability; FALSE returns a density. Scale beta has the same units as x.

Shape one has the exponential distribution with rate `1/beta`. A shape `n/2` for positive integer n, with scale two, has the chi-square distribution with n degrees of freedom. These are mathematical relations; particular worksheet compositions still have their own stores.

## Try it

`=GAMMA.DIST(2,1,2,TRUE)` is about `0.6321205588`. The scale ratio is one, giving the exponential CDF `1-exp(-1)`.

`=GAMMA.DIST(2,1,2,FALSE)` is about `0.1839397206`. That is a density, mathematically `exp(-1)/2`; it is not the probability of observing exactly the real-valued point two.

## Quirks

Microsoft documents `#VALUE!` for nonnumeric x or shape/scale, and `#NUM!` for negative x or a nonpositive shape or scale. Beta is a scale, so replacing it by an exponential rate without taking a reciprocal changes the distribution.

At x=0 the mathematical density’s right-hand limit is zero for alpha>1, `1/beta` for alpha=1 and unbounded for smaller positive alpha. The current fallback produces zero in the first case but `#NUM!` for the other two. Those are code facts; a mathematical limit alone is not evidence of Excel’s actual boundary return.

The cumulative half-shape branch currently uses an ordinary binary64 square root after dividing by beta. Its retained public-composition controls do not settle every root precision boundary. Large finite parameters can also overflow an intermediate ratio; finite inputs do not by themselves prove a successful candidate result.

## How to implement

Use D for ordinary binary64 storage and [R](../model/09-staged-arithmetic.md) for the stored 64-bit-significand operation.

For the current cumulative route, validate x and the two positive parameters, then store `y=D(x/beta)`:

1. At alpha=0.5, compute the ordinary binary64 square root of y and pass it to ERF.PRECISE. Keep divide before square root. Do not silently replace this code’s root by a separately staged public SQRT call at unmeasured boundaries.
2. At alpha=1, call EXPON.DIST with `(y,1,TRUE)`, retaining its corrected exponential-difference stages. Calling it with `(x,1/beta,TRUE)` changes a ratio into a stored reciprocal/product.
3. For other shapes, evaluate the generic regularized lower gamma ratio `P(alpha,y)`. Integer alpha does not select a finite-sum shortcut in the current code.

The source retains 68 half-shape composition controls on scales one, two and four, and shape-one controls including non-dyadic scales three and five. Those observations constrain the named compositions, rather than every inner approximation or scale.

For the current density candidate at positive x and alpha other than one:

```text
y = R(x/beta)
t = R(alpha*LN(y))
G = stored GAMMALN(alpha+1)
prefix = current exponential-of-difference candidate(t,y,G)
ratio = R(alpha/x)
density = R(prefix*ratio)
```

The prefix corresponds mathematically to `exp(t-y-G)`. Its producer and return stage are not fully identified; the current source retains a particular extended difference/exponential candidate. The independently stored alpha/x ratio and final multiplication are a separately measured consumer. Shapes one and x=0 retain a log-density fallback, rather than this branch.

For the cumulative example, y is exactly one; the CDF is about `1-exp(-1)`. Its density is a separate calculation. Do not transfer a supported CDF composition into the density merely because both come from the same distribution.

The notes retain earlier integer-shape prefactor findings and their scope. General gamma ratios, GAMMALN, the density prefix, endpoint publication and remaining report mismatches remain open. No published Handbook suite establishes whole-function agreement.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 77,653 |
| 20430 | Windows, 64-bit, CPU not recorded | 194 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.GAMMA.DIST.json).

## Alternatives

[GAMMADIST](FUNC.GAMMADIST.md) is the compatibility spelling. [GAMMA.INV](FUNC.GAMMA.INV.md) finds a quantile; [EXPON.DIST](FUNC.EXPON.DIST.md) covers shape one; [CHISQ.DIST](FUNC.CHISQ.DIST.md) selects shape n/2 and scale two. [GAMMA](FUNC.GAMMA.md) and [GAMMALN](FUNC.GAMMALN.md) evaluate related special functions rather than a CDF.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `GAMMA.DIST` that an implementer matching Excel, or a reader explaining
an unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's
answer and where it was recorded; "measured" means the statement is read off recorded Excel
answers, "inferred" marks what goes beyond them. Where older text on this page says a behaviour
was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

### FN-GAMMA.DIST-001: GAMMA.DIST's prefactor carries GAMMALN(a+1) in the exponent, with its error

*Excel differs from the mathematical answer; inferred; recorded 2026-11-04.*

With RN(v) meaning v rounded to 64 bits on the x87 unit and then stored to binary64, the series
arm and the density of GAMMA.DIST at integer shape a from 2 to 6 share r = RN(exp(t - x -
GAMMALN(a+1))), with ln x stored, t = RN(a*ln x), and the difference held in the 64-bit
registers. The cumulative answer is RN(r*S) with S the stored forward sum, and the density
RN(r*a)/x, the division last; with a scale, y = RN(x/beta) comes first. So GAMMALN's own error
is in every answer: at shape 2 the constant is GAMMALN(3), one unit above stored ln 2
(FN-GAMMALN-001).

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `GAMMA.DIST(0.010000000000003386, 2, 1, TRUE)` | x `0.010000000000003386` (`0x3f847ae147ae1c1b`); alpha `2` (`0x4000000000000000`); beta `1` (`0x3ff0000000000000`); cumulative `TRUE` (cells) | `4.9667913340299445e-05` (`0x3f0a0a5081f5d243`) | `4.966791334029942e-05` (`0x3f0a0a5081f5d23f`) | Excel 16.0 build 20527, Windows; bitness and CPU not recorded in this reading |
| `GAMMA.DIST(0.5000000000001851, 3, 1, TRUE)` | x `0.5000000000001851` (`0x3fe0000000000683`); alpha `3` (`0x4008000000000000`); beta `1` (`0x3ff0000000000000`); cumulative `TRUE` (cells) | `0.01438767796698471` (`0x3f8d774972ab2b85`) | `0.014387677966984718` (`0x3f8d774972ab2b89`) | Excel 16.0 build 20527, Windows; bitness and CPU not recorded in this reading |

Correct value: mpmath regularized lower incomplete gamma at 800 bits, correctly rounded.

Basis (inferred): Noise lines on windows of consecutive doubles (Excel build 20527) put ln x
stored and multiplied by a, not a power of x. Storing t - x adds a slow line of amplitude 5,
and leaving it unstored takes the noise at small x from 13 units of 2^-53 to 1.5. A scan of one
constant factor per shape peaks at -0.875, +0.25, -1.25, +1.625 and -2 of 2^-53 for a = 2 to 6,
and the first is the error of GAMMALN(3). This form matches 15,364 of 26,776 rows at a = 2,
15,588 of 26,770 at a = 3, 1,088 of 2,051 at a = 4, 654 of 1,142 at a = 5, 691 of 1,151 at a =
6, and over all shapes 47,647 of 77,975 recorded rows.

Inferred, not measured: The form is read from noise and scored in part: one stored stage of the
prefactor or the sum is still missing, with a root mean square of about 0.8 of 2^-53 at small
x. Non-integer shapes depend on GAMMALN below 3, which is not identified. An earlier reading of
a chopped exponential in this function was this constant.

- Open: The missing stored stage of the prefactor or the sum.
- Open: Shapes outside 2 to 6, and the upper arm of the distribution.

Evidence:

- [`content/notes/evidence/2026-11-04/gamma-EVIDENCE.md`](../../content/notes/evidence/2026-11-04/gamma-EVIDENCE.md) — research write-up: how the stored stages were read
- DnaCalc `crates/calc-func/src/ported/special_math_common.rs` at `78de0f0a49` — the prefactor in the product; the commit message carries the row counts
- DnaCalc `crates/calc-func/witness-status.tsv` at `af74247e1e` — rows matched per function
- [`content/notes/evidence/2026-11-04/gamma-readings.tsv`](../../content/notes/evidence/2026-11-04/gamma-readings.tsv), GAMMA.DIST 3f847ae147ae1c1b 4000000000000000 (readings)
- [`content/notes/evidence/2026-11-04/gamma-readings.tsv`](../../content/notes/evidence/2026-11-04/gamma-readings.tsv), GAMMA.DIST 3fe0000000000683 4008000000000000 (readings)

<!-- efh:notes end -->

## Sources

- [Microsoft: GAMMA.DIST function](https://support.microsoft.com/en-us/excel/functions/gamma-dist-function), opened 7 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/beta_gamma_stats_family.rs); implementation steps and the limits identified in the text.
- [Current exponential difference candidate](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/excel_numeric/x87_chains.rs); `excel_exp_difference` and its resident difference.
- [Remaining numerical failures](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/docs/generated/problem-functions.md); counts keep their source scopes.
