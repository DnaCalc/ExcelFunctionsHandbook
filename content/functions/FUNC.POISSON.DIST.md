---
schema: efh.function-page/v1
function_id: FUNC.POISSON.DIST
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0030
  - EV-DIST-0033
open_problems: []
references:
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 26, 26.4.21 and 26.1.22 (Poisson sums and the incomplete gamma / chi-square identity)"
    role: "The identity turning the Poisson CDF into a regularized incomplete gamma function"
  - work: "C. Loader, Fast and Accurate Computation of Binomial Probabilities"
    locator: "2000; the saddle-point dpois/dbinom used by R"
    role: "The modern accurate route to discrete densities, avoiding overflow and cancellation"
  - work: "A. R. DiDonato and A. H. Morris, Algorithm 654: computation of the incomplete gamma function ratios"
    locator: "ACM TOMS 12 (1986) 377-393 (GRATIO)"
    role: "The reference implementation of the incomplete gamma ratios behind the Poisson CDF"
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
family: discrete_dist_family
role_in_family: >-
  The Poisson member of the discrete distribution family; the surface whose k=0 window is the
  best-evidenced point anywhere in the family and whose behaviour away from that window is
  openly worse.
---

# POISSON.DIST

## What it does

`POISSON.DIST(x,mean,cumulative)` evaluates a Poisson event count with the supplied expected count mean. FALSE returns the mass for exactly x events; TRUE returns the probability of zero through x events, inclusive. The mathematical mass at nonnegative integer k is `exp(-mean)*mean^k/k!`.

## Try it

`=POISSON.DIST(0,2,FALSE)` is about `0.1353352832`, the probability of no events when two are expected. The cumulative call at zero has the same value. `=POISSON.DIST(4,3,FALSE)` is about `0.1680313557`; the corresponding cumulative call also includes counts zero through three.

## Quirks

The count is truncated, but an original negative fraction is rejected before it can become zero. The mean must be nonnegative in Microsoft’s documented domain. Nonnumeric count or mean gives `#VALUE!`.

Mean zero is a point mass: zero events has probability one and positive counts have mass zero. For a positive mean, a cumulative result includes the requested count. “More than k” therefore uses `1-CDF(k)`, while “at least k” uses the CDF at k-1.

The current k=0 branch runs before its negative-mean check and can return EXP of a positive number for that otherwise invalid argument. This is the code’s branch order, separate from Microsoft’s stated negative-mean error. It also retains a subnormal exponential where the positive-count routes flush below normal.

## How to implement

Use [D, E and R](../model/09-staged-arithmetic.md#retain-an-extended-intermediate-when-the-recipe-requires-it) for binary64 storage, resident 64-bit-significand arithmetic and a 64-bit operation followed by a binary64 store. The following finite-input recipe describes the newer cited implementation.

1. Reject an original negative count before truncation. Set k to its truncated value. If k=0, return the measured public `EXP(-mean)` result before the positive-count mean checks. This route retains its subnormal exponential.
2. For k>0 reject a negative mean. At mean zero, return zero for a mass or one for a cumulative probability.
3. For cumulative probability evaluate the generic upper gamma ratio `Q(k+1,mean)`. It is the sum through k, despite being an upper gamma ratio. This avoids starting a recurrence at a mass that has already underflowed.
4. For a mass with k=1 or `mean<1.1`, form `t=R(k*LN(mean))` and obtain the stored internal/public GAMMALN result G at k+1. Subtract mean and G with the compensated-exponential recipe below.
5. For other positive-count masses, compute Stirling correction s(k) and deviance d(k,mean). Store `exponent=R(-s-d)`, obtain stored public EXP(exponent), and divide it with R by `D(sqrt(D(D(2*pi)*k)))`, using the stored binary64 pi. The source uses an ordinary binary64 square root at this normalizer.
6. The current positive-count publisher flushes subnormal results to zero and rejects nonfinite results. Keep this policy distinct from the k=0 return.

### Exponential of two subtractions

Start `a=t`. For each term in the ordered list `[mean,G]`, form the resident difference `e=E(a-term)`, store `a=D(e)`, store `lost=D(E(e-a))`, and keep `factor=R(1+lost)`. After both subtractions, obtain public EXP(a), then multiply successively by the two factors using R at each multiplication. A single call to EXP of an algebraically combined difference loses these stores and corrections. GAMMALN’s inner approximation remains a separate open leaf.

### Stirling correction

For integer k=0 through six, the current table is:

| k | Stored decimal coefficient |
|---:|---:|
| 0 | 0 |
| 1 | 0.0810614667953272 |
| 2 | 0.0413406959554092 |
| 3 | 0.0276779256849983 |
| 4 | 0.0207906721037650 |
| 5 | 0.0166446911898211 |
| 6 | 0.0138761288230707 |

Convert each decimal to nearest binary64. The source describes these as values cut to sixteen decimal places; its observed k=4 value is one representable step above the table entry and remains unexplained.

For k>=7, first round each of these ten coefficients to binary64, in ascending power order:

```text
1/12, -1/360, 1/1260, -1/1680, 1/1188,
-691/360360, 1/156, -3617/122400, 43867/244188, -174611/125400
```

Form `u=E(1/E(k*k))`. Starting with the last coefficient, run Horner with resident multiplication and addition, `v=E(E(v*u)+coefficient)`. Return `D(E(v/k))`. Intermediate polynomial values are retained in E; storing each one to binary64 is a different model.

### Deviance

The mathematical quantity is `d(x,m)=x*ln(x/m)+m-x`. Outside the current series condition `abs(D(x-m))<D(0.2*D(x+m))`, form `ratio=R(x/m)`, obtain stored LN(ratio), then return `R(R(x*LN(ratio))+R(m-x))`.

Inside that condition, the current finite reconstruction is:

```text
delta = R(x-m)
v = E(delta / R(x+m))
w = E(v*v)
power = v; tail = 0; j = 1
repeat:
    power = E(power*w)
    next = E(tail + E(power/(2*j+1)))
    if next == tail: stop
    tail = next; j = j+1
V = D(v)
S = D(E(V+tail))
twice_x = D(2*x)
A = E(R(m-x) + R(twice_x*S))
A = E(A - R(twice_x*V))
A = E(D(A) + delta)
return D(E(A + R(delta*V)))
```

The series condition is the implementation’s floating comparison. Its mathematical ratio interval is `(2/3,3/2)`, wider than the published starting method’s interval. Do not change the threshold or intermediate stores solely from that algebraic equivalence.

Worked mass at k=4, mean three: the deviance reconstruction gives `0.15072828980712366`; the table supplies s(4) about `0.020790672103765`. The stored exponent is about `-0.17151896191088867`, and the normalizer is the binary64 square root of `8*pi`. This recipe gives about `0.16803135574154085`. These are worked reconstruction values, rather than additional Excel captures.

The mathematical starting point is Loader’s published saddle-point method; the project then isolates arithmetic sites from recorded answers. The source records 1,000 of 1,000 series controls at x=100 and 1,199 of 1,200 at x=1024. The latter remaining disagreement, the small-table exception, GAMMALN and other report failures are still open. The 1.1 split is recorded at x=2,3,7 and does not establish every global route boundary. The separately pinned interactive calculator can predate these later stages. No published Handbook suite establishes whole-function agreement.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 45,997 |
| 20430 | Windows, 64-bit, CPU not recorded | 3,520 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.POISSON.DIST.json).

## Alternatives

[BINOM.DIST](FUNC.BINOM.DIST.md) models a fixed number of independent trials. [CHISQ.DIST.RT](FUNC.CHISQ.DIST.RT.md) exposes the related gamma-tail family; [EXP](FUNC.EXP.md) supplies the zero-count stage.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `POISSON.DIST` that an implementer matching Excel, or a reader explaining
an unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's
answer and where it was recorded; "measured" means the statement is read off recorded Excel
answers, "inferred" marks what goes beyond them. Where older text on this page says a behaviour
was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

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

- [Microsoft: POISSON.DIST function](https://support.microsoft.com/en-us/excel/functions/poisson-dist-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [R31 discrete distribution implementation](https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/crates/calc-func/src/ported/discrete_dist.rs); guards, route split, Stirling table, deviance and mass composition.
- [Resident series and compensated exponential](https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/crates/calc-func/src/ported/excel_numeric/x87_chains.rs); explicit operation and store order.
- [Loader, Fast and Accurate Computation of Binomial Probabilities](https://www.r-project.org/doc/reports/CLoader-dbinom-2002.pdf), opened 6 November 2026; published saddle-point starting method and Poisson extension. The implementation’s measured stages are a separate claim.
- [Remaining numerical failures](https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/docs/generated/problem-functions.md); source-specific counts, including still unresolved Poisson answers.
