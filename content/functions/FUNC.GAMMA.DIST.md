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

### FN-GAMMA.DIST-002: Incomplete gamma at half-integer shape starts from sqrt(pi) to binary64 and the public ERFC.PRECISE value

*Detail for matching Excel's bits; measured; recorded 2026-11-11.* Applies to CHIDIST, CHIINV, CHISQ.INV, GAMMA.DIST, GAMMA.INV.

For a shape a = n + 1/2 with a < x, the incomplete gamma sums a finite series. The series
begins with the term t = EXP(-x) / (sqrt(pi) * SQRT(x)) and the starting value Q(0.5, x) =
erfc(SQRT(x)). With sqrt(pi) typed as the 15-digit 1.77245385090552 (about 20 units of 2^-53
below the true value) the cumulative answers sat low: on Excel 20131 rows CHIDIST(8, 5) was 12
units low, and GAMMA.DIST(3, 2.5, 1, TRUE) and GAMMA.DIST(4, 2.5, 1, TRUE) 6 and 4 units low.
With the binary64 value 1.772453850905516 DNA Calc returns Excel's recorded bits on all three.
Over the full bank (all functions) that constant and the F.TEST change landed with it moved
3,889,035 to 3,889,056 matched rows (CHIDIST 371 to 372, CHIINV 30 to 37, CHISQ.INV 13 to 14,
GAMMA.DIST 77,804 to 77,808, GAMMA.INV 63 to 66, one CHIINV row lost). A later change takes the
starting value Q(0.5, x) from the public ERFC.PRECISE function instead of the internal
incomplete gamma at 0.5: CHIDIST 378 to 380, GAMMA.DIST 77,812 to 77,814, CHIINV 37 to 36 (one
row, (0.1, 3), lost).

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `CHIDIST(8, 5)` | x `8` (`0x4020000000000000`); deg_freedom `5` (`0x4014000000000000`) (cells) | `0.1562356275777223` (`0x3fc3ff876f754b0e`) | Excel 16.0 build 20131, Windows; bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics |
| `GAMMA.DIST(3, 2.5, 1, TRUE)` | x `3` (`0x4008000000000000`); alpha `2.5` (`0x4004000000000000`); beta `1` (`0x3ff0000000000000`); cumulative `1` (`0x3ff0000000000000`) (cells) | `0.6937810815867216` (`0x3fe6337461fff386`) | Excel 16.0 build 20131, Windows; bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics |

Basis (measured): Three Excel build 20131 rows (CHIDIST oc65, GAMMA.DIST oc73 and oc75) are
reproduced by the unit test with the binary64 constant. The ERFC.PRECISE start rests on net row
counts only (+3 net over the full bank), not on a located row.

Inferred, not measured: That Excel's series starts from the public ERFC.PRECISE value is
inferred from the +3 net; the cause of the 15-digit constant's error being absent in Excel is a
reading of the match, not of Excel's code.

- Open: Row CHIINV oc3 (the inverse search lands one double above Excel's 0x403044280e2bdaad) and CHIINV oc21 (0.1, 3) were lost by these two changes.
- Open: Nine bank rows at x >= 10 (R6 of the harvest) are unmatched.
- Open: A fitted 6c constant scored +9 rows and was not taken because it is fitted.

Evidence:

- DnaCalc `crates/calc-func/src/ported/special_math_common.rs`, GRATIO_RTPI_FULL; half_integer_finite_sum_uses_binary64_sqrt_pi at `4cc4ba08f1` — constant and unit test
- DnaCalc `crates/calc-func/src/ported/special_math_common.rs`, gratio (statement 220) at `eac227d3b8` — start from erfc_precise_kernel
- DnaCalc `crates/calc-func/witness-status.tsv`, CHIDIST; GAMMA.DIST; CHIINV at `eac227d3b8` — matched rows per function
- CalcFunc-Evidence `cache/oracle/build-20131/CHIDIST.jsonl.gz`, CHIDIST|0x4020000000000000|0x4014000000000000 at `b4818a6a23` (readings)
- CalcFunc-Evidence `cache/oracle/build-20131/GAMMA.DIST.jsonl.gz`, GAMMA.DIST|0x4008000000000000|0x4004000000000000|0x3ff0000000000000|0x3ff0000000000000 at `b4818a6a23` (readings)

### FN-GAMMA.DIST-003: Incomplete gamma at integer shape 2 to 20 is a stored polynomial of reciprocal factorials times EXP(-x)

*Detail for matching Excel's bits; inferred; recorded 2026-11-11.* Applies to CHIDIST, CHIINV, CHISQ.INV, CHISQ.TEST, GAMMA.DIST, GAMMA.INV, POISSON.DIST.

For an integer shape a from 2 to 20 and a < x <= 30, DNA Calc computes the upper tail as Q(a,
x) = S(EXP(-x) * H), with H a Horner polynomial whose coefficients are 1/k! rounded once to
binary64 (k = 0 to 19) and with every Horner multiply and add stored to binary64 before the
next step; the result is EXP(-x) times the stored polynomial, and P = 1 - Q. The boundaries
were located with adjacent-double probes: a == x and x > 30 do not take this route (they take
the ordinary gamma-density route), and a divided Horner form, a forward term sum, and an
extension to a >= 21 were each refuted. Over the full bank this matched 20 more rows net
(3,889,422 to 3,889,442): CHIDIST +6, CHISQ.TEST +1, GAMMA.DIST +5, POISSON.DIST +3, GAMMA.INV
+6 and -1, with four rows lost: CHIINV (0.001, 10), CHISQ.INV (0.6, 4) and (0.85, 4), GAMMA.INV
(0.75, 5, 1); the first three are exact-inverse pins whose gains in the same functions offset
them. On the oracle corpus the passed count went from 76,759 to 76,765. Measured and not taken:
for large-mean POISSON, one stored product M*u*A gave +79 and -73, and M = RN64(V/D) resident
into st(M*u) then st(T*A) gave +47 and -39 on the 542 large-mean POISSON misses, with gains and
losses interleaved inside the same (k, mean) groups so no clean rule; a stored generic GAMMA
seed fixes GAMMA(-0.5) and GAMMA(-1.5) but breaks GAMMA(-2.5), which matches today.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `GAMMA.DIST(5, 3, 1, TRUE)` | x `5` (`0x4014000000000000`); alpha `3` (`0x4008000000000000`); beta `1` (`0x3ff0000000000000`); cumulative `1` (`0x3ff0000000000000`) (cells) | `0.8753479805169189` (`0x3fec02d9c49e12f0`) | Excel 16.0 build 20131, Windows; bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics |
| `GAMMA.DIST(4, 2, 1, TRUE)` | x `4` (`0x4010000000000000`); alpha `2` (`0x4000000000000000`); beta `1` (`0x3ff0000000000000`); cumulative `1` (`0x3ff0000000000000`) (cells) | `0.9084218055563291` (`0x3fed11ca9b3ace7a`) | Excel 16.0 build 20131, Windows; bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics |

Basis (inferred): Counts and boundaries are quoted from the DnaCalc commit message, code
comments and bead comment (full-bank per-row traces); the two cited build 20131 rows are
readings at integer shapes 2 and 3 inside the region, not a test of the polynomial's stores.

Inferred, not measured: That Excel evaluates this polynomial with these coefficient loads and
stores is inferred from the match and from adjacent-double probes; the coefficient loads are
described in the code as conditional on the other loads and on EXP.

- Open: Nine x >= 10 rows (R6) and the large-mean POISSON misses remain unmatched.
- Open: GAMMALN near 2.5 and 2.75 cause 32 of 58 GAMMA.DIST cumulative misses (tracked as dc-6332.7).

Evidence:

- DnaCalc `crates/calc-func/src/ported/special_math_common.rs`, GRATIO_RECIP_FACTORIAL; gratio_integer_sum at `a93fc3af73` — the route as implemented
- DnaCalc `crates/calc-func/tests/f1_stats.rs`, integer-shape tests at `a93fc3af73` — tests added with the commit
- DnaCalc `.beads/issues.jsonl`, dc-6332.3 at `b8738ae6a6` — landed items and the large-mean POISSON negative results
- CalcFunc-Evidence `cache/oracle/build-20131/GAMMA.DIST.jsonl.gz`, GAMMA.DIST|0x4014000000000000|0x4008000000000000|0x3ff0000000000000|0x3ff0000000000000 at `b4818a6a23` (readings)
- CalcFunc-Evidence `cache/oracle/build-20131/GAMMA.DIST.jsonl.gz`, GAMMA.DIST|0x4010000000000000|0x4000000000000000|0x3ff0000000000000|0x3ff0000000000000 at `b4818a6a23` (readings)

### FN-GAMMA.DIST-004: Forward incomplete gamma runs finite sums strictly on a < x <= 30, drops ln(10) and x0=31 splits for a <= 20, and loops forward for a > 20

*Detail for matching Excel's bits; inferred; recorded 2026-11-22.* Applies to CHIDIST, CHISQ.DIST, CHISQ.DIST.RT, GAMMA.DIST, GAMMADIST, POISSON, POISSON.DIST.

In Excel 16.0.20527, the shared forward incomplete-gamma kernel gratio(a, x) removes three
splits of NSWC TOMS 708. First, for integer shapes a in 2..=20 and half-integer shapes a in
1.5..=19.5, the finite-sum branch runs strictly on a < x <= 30.0: at x == a Excel runs the
lower Taylor series (matched on 42 of 42 adjacent-double boundary calls), and for half-integers
Excel evaluates a stored x87 PC64 Horner polynomial over RN53(2^(k+1) / (2k+1)!!) scaled by
exp(S(-x - S(0.5 * ln(x)))) * (1 / sqrt(pi)) * x plus ERFC.PRECISE(sqrt(x)). Second, for 1 < a
<= 20 outside the finite sums, Excel has no ln(10) switch and no x0 = 31.0 asymptotic switch: x
<= a runs the forward Taylor series and x > a stays in the continued fraction down toward
DBL_MIN. Third, for a > 20 and x / a >= 1.3 (all x, with no x0 = 31.0 threshold), Excel
computes the upper tail by a forward x87-stored loop (amn = S(amn - 1), nt = S(t * S(amn / x)),
stopping at |nt| > |t| or |t|/s <= 2^-52) multiplied by S(pm * S(S(a / x) * s)) using
poisson_mass(a, x) (Loader's saddle-point prefactor for a > 1 and x > 1.1). See also the
DnaCalc last-bit article
https://github.com/DnaCalc/DnaCalc/blob/344d70dcea3ea77cbbeee1595f96694630a9610a/docs/articles/last-bit-gamma.md.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `CHIDIST(18.162619486466507, 15)` | x `18.162619486466507` (`0x403229a16e4010e3`); deg_freedom `15.0` (`0x402e000000000000`) (cells) | `0.25421272406815476` (`0x3fd0450572066312`) | Excel 16.0.20527.20196, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics |
| `POISSON.DIST(26, 62.46781908508943, TRUE)` | x `26.0` (`0x403a000000000000`); mean `62.46781908508943` (`0x404f3be17eeb73ae`); cumulative `TRUE` (cells) | `1.5066926709301541e-07` (`0x3e8438f4e39171f3`) | Excel 16.0.20527.20196, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics |
| `GAMMA.DIST(1.4337904022671917, 1.3878205276776927, 1, TRUE)` | x `1.4337904022671917` (`0x3ff6f0ce3470e67e`); alpha `1.3878205276776927` (`0x3ff634834c317d8e`); beta `1.0` (`0x3ff0000000000000`); cumulative `TRUE` (cells) | `0.627034000116316` (`0x3fe410a99b7f5931`) | Excel 16.0.20527.20196, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics |

Basis (inferred): Measured across 5,820 cb7 live Excel 16.0.20527 calls and two prospective
frozen holdout suites: cb7 frozen_cb7_predictions.jsonl matched 461 of 800 unseen rows (318 of
320 on a > 20, x/a >= 1.3; 240 of 240 for x >= 31) and nx6 frozen_predictions_nx6.jsonl matched
252 of 300 (60 of 60 half-integer forward, 40 of 40 asymptotic forward). Offline across 12
gamma/Poisson/chi witness banks (162,739 rows), landing nx6 moved total matches from 160,287 to
161,032 (+745 net: CHIDIST pinned 120 of 120, POISSON +199, POISSON.DIST +182).

Inferred, not measured: That Excel executes these exact polynomial, forward-loop, and
branch-boundary expressions internally is inferred from the zero-ULP match across frozen
holdouts and adjacent-double sweeps; Excel's source was not read.

- Open: 46 CHISQ.DIST cumulative rows (45 in a > 20, y < 1.1 and 1 tie at a = 104.5, y = 50) remain unmatched (dc-j9k.59).
- Open: 4 POISSON Temme rows (a > 48) and 3 build-20131 POISSON.DIST rows remain 1 ULP off (dc-j9k.42); forward distribution remainder is tracked in dc-ymjf.2.27.

Evidence:

- DnaCalc `crates/calc-func/src/ported/special_math_common.rs`, gratio; gratio_half_integer_sum; gratio_after40 at `88b8861796` — Carbon cb7 forward gratio landing (nx6)
- DnaCalc `docs/articles/last-bit-gamma.md`, Stage 2 (cb7 / nx6) at `344d70dcea` — Last-bit article on the gamma family
- CalcFunc-Evidence `readings/excel-20527/studies/cb7.json.gz` at `4ef2c244d7` — 5,744 live Excel cb7 study rows
- CalcFunc-Evidence `readings/excel-20527/studies/nx6.json.gz` at `4ef2c244d7` — 300 frozen nx6 holdout rows
- [`content/notes/evidence/2026-11-22/frozen-holdouts-provenance.tsv`](../../content/notes/evidence/2026-11-22/frozen-holdouts-provenance.tsv), cb7; nx6 — Pre-ask SHA-256 hashes and frozen holdout scores
- CalcFunc-Evidence `readings/excel-20527/studies/nx6.json.gz`, $.rows[7] at `4ef2c244d7` (readings)
- CalcFunc-Evidence `readings/excel-20527/studies/nx6.json.gz`, $.rows[67] at `4ef2c244d7` (readings)
- CalcFunc-Evidence `readings/excel-20527/studies/nx6.json.gz`, $.rows[104] at `4ef2c244d7` (readings)

### FN-GAMMA.DIST-005: For a < 1 and x < 1.1, Excel uses TOMS 654 double-precision st200 branch bounds with shared Kahan expm1, forcing even LSBs when P < 0.5

*Detail for matching Excel's bits; inferred; recorded 2026-11-22.* Applies to CHISQ.DIST, GAMMA.DIST, GAMMA.INV, GAMMADIST, GAMMAINV.

In Excel 16.0.20527, when 0 < a < 1.0 and 0 < x < 1.1 (unit scale x / beta), gratio computes z
= S(a * ln(x)) with x87 PC64 operations and selects the upper-complement branch st200 using
TOMS 654's double-precision (iop = 1) bounds: z >= -ln(2) (-0x1.62e42fefa39efp-1,
0xbfe62e42fefa39ef) when x <= 0.5, and S(a - x) < 0.25 when x > 0.5. Single-bit straddles at x
= 0x3fd965fea53d6e3c vs 0x3fd965fea53d6e3d (a = 0.75) and x = 0.5 vs next_up(0.5) (a = 0.8125)
establish both seams with zero bits of slack. Inside st200, Excel computes expm1(z) via its
shared Kahan excel_expm1_internal(z) routine (not Morris's gratio_rexp rational) and forms P =
0.5 + (0.5 - qans), which by Sterbenz's lemma gives an even LSB (bit 0 = 0) on every output
with P < 0.5 (85 of 85 across consecutive-x sweeps). Separately, for CHISQ.DIST(2*y, 2*a, TRUE)
at a > 20 and y < 1.1, Excel exits the anomalous a > 20 lower-series branch at y == 1.1
(0x3ff199999999999a) while poisson_mass keeps compensated exp until y > 1.1.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `GAMMA.DIST(0.39685026299204984, 0.75, 1, TRUE)` | x `0.39685026299204984` (`0x3fd965fea53d6e3c`); alpha `0.75` (`0x3fe8000000000000`); beta `1.0` (`0x3ff0000000000000`); cumulative `TRUE` (cells) | `0.4621377428146077` (`0x3fdd93aa2ee8b435`) | Excel 16.0.20527.20196, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics |
| `GAMMA.DIST(0.3968502629920499, 0.75, 1, TRUE)` | x `0.3968502629920499` (`0x3fd965fea53d6e3d`); alpha `0.75` (`0x3fe8000000000000`); beta `1.0` (`0x3ff0000000000000`); cumulative `TRUE` (cells) | `0.46213774281460807` (`0x3fdd93aa2ee8b43c`) | Excel 16.0.20527.20196, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics |
| `GAMMA.DIST(0.45449360831744195, 0.875, 1, TRUE)` | x `0.45449360831744195` (`0x3fdd166c5bfdb97f`); alpha `0.875` (`0x3fec000000000000`); beta `1.0` (`0x3ff0000000000000`); cumulative `TRUE` (cells) | `0.4293230191680172` (`0x3fdb7a0741afc88a`) | Excel 16.0.20527.20196, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics |
| `CHISQ.DIST(0.5, 38, TRUE)` | x `0.5` (`0x3fe0000000000000`); deg_freedom `38.0` (`0x4043000000000000`); cumulative `TRUE` (cells) | `2.3585848943481354e-29` (`0x39fde60bab853da6`) | Excel 16.0.20527.20196, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics |

Basis (inferred): Measured in cb11 (1,391 live Excel 16.0.20527.20196 calls,
frozen_cb11_predictions.jsonl 141 of 240 vs 105 of 240 on prior master) and nx9 (nx9_probe1:
582 of 1,478 combined matches vs 525 for gratio_rexp and 464 for prior master). Landing nx9 on
full witness banks raised GAMMA.DIST by +656 (78,209 -> 78,865 of 83,312), GAMMADIST by +15
(3,019 -> 3,034 of 3,139), GAMMA.INV by +567 (4,336 -> 4,903 of 10,860), and GAMMAINV by +149
(4,106 -> 4,255 of 6,163) with zero per-function regressions.

Inferred, not measured: The exact internal code structure of Excel's a < 1, x < 1.1 st190
primary branch (which still shows a non-monotonic period-16/32 sawtooth) and the a > 20, y <
1.1 CHISQ.DIST branch remain inferred in part and are open.

- Open: a < 1, x < 1.1 st190 primary-branch period-16/32 sawtooth (-11..+2 ULP) and residual +-1..2 ULP st200 offset (dc-j9k.59).
- Open: CHISQ.DIST a > 20, y < 1.1 lower-series staging (45 bank misses; dc-j9k.59).

Evidence:

- DnaCalc `crates/calc-func/src/ported/special_math_common.rs`, gratio; gratio_gam1 at `0707a18dea` — iop=1 st200 branch and Kahan excel_expm1_internal (nx9)
- CalcFunc-Evidence `readings/excel-20527/studies/cb11.json.gz` at `4ef2c244d7` — 241 cb11 study rows
- [`content/notes/evidence/2026-11-22/cb11-seam-witnesses.json`](../../content/notes/evidence/2026-11-22/cb11-seam-witnesses.json) — Single-bit z=-ln(2) and x=0.5 st200 seam witnesses
- [`content/notes/evidence/2026-11-22/nx9-GAMMA.DIST-witnesses.json`](../../content/notes/evidence/2026-11-22/nx9-GAMMA.DIST-witnesses.json) — 79 differing nx9 GAMMA.DIST witnesses
- [`content/notes/evidence/2026-11-22/nx9-probe-summary.md`](../../content/notes/evidence/2026-11-22/nx9-probe-summary.md) — nx9 expm1 proof summary and witness-bank counts
- [`content/notes/evidence/2026-11-22/cb11-seam-witnesses.json`](../../content/notes/evidence/2026-11-22/cb11-seam-witnesses.json), cb11-GAMMA.DIST-00907 (readings)
- [`content/notes/evidence/2026-11-22/cb11-seam-witnesses.json`](../../content/notes/evidence/2026-11-22/cb11-seam-witnesses.json), cb11-GAMMA.DIST-00908 (readings)
- [`content/notes/evidence/2026-11-22/nx9-GAMMA.DIST-witnesses.json`](../../content/notes/evidence/2026-11-22/nx9-GAMMA.DIST-witnesses.json), nx9-GAMMA.DIST-00001 (readings)
- CalcFunc-Evidence `readings/excel-20527/studies/cb11.json.gz`, $.witnesses[0] at `4ef2c244d7` (readings)

### FN-POISSON.DIST-001: Upper-tail incomplete gamma for a <= 20 runs uncontracted Modified Lentz with TINY=1e-30, and poisson_mass switches at <= 1.1

*Detail for matching Excel's bits; inferred; recorded 2026-11-22.* Applies to CHIDIST, CHISQ.DIST, CHISQ.DIST.RT, GAMMA.DIST, GAMMADIST, POISSON, POISSON.DIST.

In Excel 16.0.20527, all upper-tail incomplete-gamma calls with a <= 20.0 (both a < 1.0, x >=
1.1 and 1.0 < a <= 20.0, x > a outside finite sums) evaluate the uncontracted Legendre
continued fraction using Thompson-Barnett Modified Lentz from b_0 = 0 (f = c = TINY, d = 0.0)
with Numerical Recipes TINY = 1.0e-30 (0x39b08b2a2c280291), storing each arithmetic operation
to binary64 at x87 PC64, stopping at k > 0 && del == 1.0, and multiplying S(pm * f) directly
without factoring a out of the fraction. At k = 0, c_0 = S(x + S(a / TINY)) rounds a * 10^30 +
x into binary64, so changing TINY to 1e-50 or 1e-100 breaks about half of 10,000 tested rows.
In poisson_mass(a, x), Excel switches between the compensated exponential and Loader's
saddle-point expansion inclusively at x <= 1.1 || a <= 1.1 (0x3ff199999999999a, verified by
single-bit bisection at 1.1 vs next_up(1.1)). POISSON(x, mean, cum) and POISSON.DIST(x, mean,
cum) match across all 278 paired probes, and POISSON.DIST(k, m, TRUE) equals CHIDIST(2*m,
2*(k+1)) and CHISQ.DIST.RT(2*m, 2*(k+1)). See
https://github.com/DnaCalc/DnaCalc/blob/344d70dcea3ea77cbbeee1595f96694630a9610a/docs/articles/last-bit-gamma.md.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `POISSON(1, 32.00000000000001, TRUE)` | x `1.0` (`0x3ff0000000000000`); mean `32.00000000000001` (`0x4040000000000001`); cumulative `TRUE` (cells) | `4.1791746312010515e-13` (`0x3d5d68881f1c6f57`) | Excel 16.0.20527.20196, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics |
| `POISSON.DIST(1, 32.00000000000001, TRUE)` | x `1.0` (`0x3ff0000000000000`); mean `32.00000000000001` (`0x4040000000000001`); cumulative `TRUE` (cells) | `4.1791746312010515e-13` (`0x3d5d68881f1c6f57`) | Excel 16.0.20527.20196, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics |
| `GAMMA.DIST(1.1, 4.143399358773536, 1, FALSE)` | x `1.1` (`0x3ff199999999999a`); alpha `4.143399358773536` (`0x401092d74810cc80`); beta `1.0` (`0x3ff0000000000000`); cumulative `FALSE` (cells) | `0.06233902817180819` (`0x3fafeae6ae865147`) | Excel 16.0.20527.20196, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics |

Basis (inferred): Measured across cb8's 1,325 live Excel 16.0.20527 calls and
frozen_cb8_predictions.jsonl (560 of 560 unseen forward calls across 6 functions matched with 0
ULP max error; 599 of 640 overall), plus nx7's frozen_nx7_proof2_pred.jsonl (50 of 50 on the
1.1 inclusive boundary). On full witness banks after nx7, POISSON reached 1,556 of 1,560,
POISSON.DIST 49,707 of 49,710 decoded (0 cumulative CF/asymp/Temme misses), CHIDIST 3,241 of
3,241, and CHISQ.DIST.RT 2,858 of 2,859.

Inferred, not measured: That Excel executes this exact Modified Lentz recurrence and TINY
constant is inferred from matching 10,000 of 10,000 offline controls, targeted del==1.0 vs
nf==f tie-breakers (2 of 2), and 560 of 560 prospective forward holdouts.

- Open: 4 remaining 1-ULP POISSON Temme rows (a > 48) and 3 remaining 1-ULP build-20131 POISSON.DIST rows plus 4 excluded input-changed rows (dc-j9k.42).

Evidence:

- DnaCalc `crates/calc-func/src/ported/special_math_common.rs`, gratio_cf at `de7cbede41` — Unified Modified Lentz CF with TINY=1e-30 (nx7)
- DnaCalc `crates/calc-func/src/ported/discrete_dist.rs`, poisson_mass; poisson_compensation_error at `de7cbede41` — Inclusive <= 1.1 threshold
- DnaCalc `docs/articles/last-bit-gamma.md`, Stage 3 (cb8 / nx7) at `344d70dcea` — Last-bit article on the gamma family
- CalcFunc-Evidence `readings/excel-20527/studies/cb8.json.gz` at `4ef2c244d7` — 1,230 live Excel cb8 study rows
- CalcFunc-Evidence `readings/excel-20527/studies/nx7.json.gz` at `4ef2c244d7` — 265 live Excel nx7 proof rows
- [`content/notes/evidence/2026-11-22/frozen-holdouts-provenance.tsv`](../../content/notes/evidence/2026-11-22/frozen-holdouts-provenance.tsv), cb8; nx7 — Pre-ask SHA-256 hashes and frozen holdout scores
- CalcFunc-Evidence `readings/excel-20527/studies/cb8.json.gz`, $[50] at `4ef2c244d7` (readings)
- CalcFunc-Evidence `readings/excel-20527/studies/cb8.json.gz`, $[0] at `4ef2c244d7` (readings)
- CalcFunc-Evidence `readings/excel-20527/studies/nx7.json.gz`, $.rows[40] at `4ef2c244d7` (readings)

<!-- efh:notes end -->

## Sources

- [Microsoft: GAMMA.DIST function](https://support.microsoft.com/en-us/excel/functions/gamma-dist-function), opened 7 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/beta_gamma_stats_family.rs); implementation steps and the limits identified in the text.
- [Current exponential difference candidate](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/excel_numeric/x87_chains.rs); `excel_exp_difference` and its resident difference.
- [Remaining numerical failures](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/docs/generated/problem-functions.md); counts keep their source scopes.
