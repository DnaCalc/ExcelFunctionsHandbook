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
  - Notes
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

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `GAMMA.INV` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

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

<!-- efh:notes end -->

## Sources

- [Microsoft: GAMMA.INV function](https://support.microsoft.com/en-us/excel/functions/gamma-inv-function), opened 7 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/beta_gamma_stats_family.rs); implementation steps and the limits identified in the text.
- [Generic gamma ratio and ordered-value search](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/special_math_common.rs); callback and adjacent binary64 return.
- [Remaining numerical failures](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/docs/generated/problem-functions.md).
