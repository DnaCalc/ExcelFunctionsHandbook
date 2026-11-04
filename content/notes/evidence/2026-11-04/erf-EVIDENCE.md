# ERF body: the tiny route is identified (evidence, 2026-11-04)

## What changes

`erf_approx` (the shared ERF body behind ERF, ERF.PRECISE, the below-half ERFC complement and
GAUSS's tiny route) now computes, for normal `x` with `|x| < 2^-28`:

```text
p = RN53(RN64(x * P0))     x87 precision 64, product stored to binary64
y = RN53(RN64(p / Q0))     quotient stored to binary64
P0 = 0x3ff1df5e70c7956b = RN(3.20937758913846947e3 / s)    Cody 1969, SPECFUN CALERF A(4)
Q0 = 0x3fefada15778ece6 = RN(2.84423683343917062e3 / s)    Cody 1969, SPECFUN CALERF B(4)
s  = 2873.12583276792695... (one real scale for both; s/B(4) = 1.01015703017031...)
```

That is Cody's `|x| <= 0.5` rational `x*P(t)/Q(t)` at `t ~ 0`, with both polynomials divided
by a common factor, evaluated on the x87 with the product stored before the divide. Elsewhere
(`|x| >= 2^-28`, zero, subnormals) the body is unchanged (`libm::erf`).

Patch: `erf-tiny-route.patch` (against DnaCalc `6073fff1`, `crates/calc-func` only; applies
cleanly, `git apply --check`). It uses the existing `excel_x87_mul` / `excel_x87_div`
helpers (hardware x87 on x86_64, software x87 elsewhere).

## Before / after (full bank, every function)

Command, run in a source snapshot of DnaCalc `6073fff1` (not the orchestrator's clone), with
the evidence repository beside it:

```text
cargo xtask witness judge --bank full --trace full-before.trace     (unpatched)
cargo xtask witness judge --bank full --trace full-after.trace      (patched)
cargo xtask witness compare --before full-before.trace --after full-after.trace
```

| Function | Rows | Before | After | Gains | Losses |
|---|---:|---:|---:|---:|---:|
| ERF.PRECISE | 618,437 | 252,580 | 386,870 | 134,290 | 0 |
| GAUSS | 9,728 | 8,569 | 9,247 | 678 | 0 |
| every other function | | unchanged | unchanged | 0 | 0 |
| **All** | 3,909,807 | 3,267,143 | 3,402,111 | 134,968 | **0** |

Compare output: "3909691 decoded rows; all input/selection/answer/source/environment fields
unchanged"; "+134968 / -0". ERFC, ERFC.PRECISE, NORM.S.DIST, NORMSDIST, CHIDIST and the GAMMA
family calculate the same bits on every recorded row (the compare reports changed actuals
only for ERF.PRECISE and GAUSS).

Not run: the workbook corpus (`cargo xtask corpus` captures with Excel unless told not to; I
did not drive Excel) and the OneCalc replays. The corpus has no ERF/ERFC/GAUSS mismatch rows
today; the change only touches `0 < |x| < 2^-28`. Run the corpus once before landing.

Tests (failing first): `normal_dist_common::tests::erf_tiny_route_reproduces_recorded_excel_answers`
(24 recorded answers: 13 libm misses, 11 libm already matched) failed on the old body
(`ERF.PRECISE(0x3d70000000000000)`: calculated `...9b6d`, Excel `...9b6e`) and passes after,
natively and with `--features soft-x87`; `erf_tiny_constants_are_the_scaled_cody_pair` checks
that the two constants share one scale. Logs: `failing-first.log`, `tests-after.log`.
The wasm32 browser run was not done.

## How it was found (exact reasoning, no fitted constant)

1. The 618k ERF.PRECISE rows are 249k below `2^-27` and 360k in the binades `2^-25`, `2^-20`,
   `2^-15`; all mismatches against libm are below 0.5.
2. Below `2^-27` Excel's output mantissa depends only on the input mantissa (49,152 shared
   mantissas across binades, 0 disagreements), so the route is a chain of products/quotients
   with intermediate roundings; no exp/log is involved. No single constant fits (empty
   interval intersection).
3. Fourier analysis of the residual on the uniform `2^-14` sweeps gives two sawtooth lines
   (0.33466 and 0.09389 per step, summing to the output rounding frequency 0.428528); on two
   other step sizes the same holds. The line that is not the output rounding is the rounding
   of an intermediate `x*A`: it fixes the low 14 bits of A's significand and two further
   residues. An exhaustive search over every binary64 `A` in `[1.1037, 1.1429]` (the range
   where the intermediate crosses its binade with the output; `solve.rs`) leaves 55,240
   candidates; exact interval feasibility on recorded rows leaves one,
   `A = 0x3ff1df5e70c7956b`.
4. With A fixed, the second factor is not a binary64 multiplier but a division by the binary64
   `0x3fefada15778ece6` (exact per-row feasibility, then a 7-value neighbourhood check).
5. `A/Q` is `A(4)/B(4)` of Cody's published table to 1e-16, and both are `RN(coef/s)` for one
   real `s` when the published decimals are divided; dividing the binary64-rounded coefficients
   by any real `s` cannot give both, so the scaling was done at higher precision.
6. Precision race on the 131,255 rows of the `2^-40` binade: per-op binary64 131,201; product
   at PC64 then stored, divide at PC64 then stored: **131,255/131,255**. Over every binade
   below `2^-27`: 249,300/249,300, zero exceptions, negative arguments included.

Sources: Cody, W. J., "Rational Chebyshev approximations for the error function", Math. Comp.
23 (1969) (tier 1), coefficient values as in SPECFUN CALERF (netlib, tier 2); Intel SDM x87
precision control (tier 1); Excel's recorded answers in OxFunc-Evidence. No code was copied;
only two published coefficient values appear (scaled). `s` itself is not explained by a
source: it is determined by the two constants, and both are needed. Treat it as a measured
normalisation (open question 1 below).

Why the cut is `2^-28` and not `2^-27`: below `2^-28`, `t < 2^-56`, and the `t` terms of
Cody's rational sit below the last place of both constant terms whatever precision the
polynomials are evaluated in, so the reduced graph is the whole graph. The data show the
reduced graph also holds in `[2^-28, 2^-27)` (214 rows, 142 more gains) but fails from
`2^-27` (55 of 176 rows). I left those 142 out rather than set the cut from the data.

## Open: the body above `2^-28` (about 231k ERF.PRECISE failures remain)

Lead, not landed: the full scaled Cody rational (`A(i)/s`, `B(i)/s`, SPECFUN Horner order,
x87 PC64 with `N`, `D` stored to binary64, then the same stored product and divide) matches
174/176 rows in `[2^-27, 2^-26)` and 138/154 in `[2^-26, 2^-25)`, but only ~55% at `2^-15`.
The two misses at `2^-27` (`0x3e47c00000000000`, `0x3e47e00000000000`, toothB bank) sit in
the "−1 window" the W109 swarm described at `t in [2.2, 4.5)*2^-54`. That window starts where
`RN53(Q0 + B3'*t)` first rounds up (`B3'*t = half an ulp of Q0` at `t = 2.24*2^-54`), so the
next object is how the denominator's (and numerator's) `t` terms are formed and stored.
Four scaled coefficients are ambiguous by one unit (A1, A5, B3, the leading `1/s`).

Questions that would settle it (all are ERF.PRECISE readings; predictions can be produced
from the scripts before asking):

1. A walk of `x` across `t = x*x` from `2.0*2^-54` to `5.0*2^-54` in steps of `2^-60` relative
   (about 64 cells): it locates both edges of the −1 window to one input ulp and says which of
   `N`, `D` moves first, and at which bit.
2. The same walk across `t = 7.6*2^-54` (where `RN53(P0 + A3'*t)` first leaves `P0`): it tells
   whether the numerator is stored to binary64 (an edge appears) or kept at PC64 (no edge).
3. Four subnormal arguments (`2^-1074`, `2^-1060`, `0x000fffffffffffff`, `-2^-1050`): whether
   the tiny route still applies there (x87 keeps the exponent; the stored product rounds as a
   subnormal). The patch keeps libm for subnormals until this is known.
4. Eight arguments in `(0.46875, 0.5]` and eight in `[0.5, 0.53125)`: whether this rational
   is used up to 0.5 (Excel's measured complement flip) or Cody's own cut at 0.46875.

## Not started

GAMMALN / GAMMALN.PRECISE, ERFC (above 0.5), NORM.S.DIST body and GAMMA: no work this round.

## Files

- `erf-tiny-route.patch`: the patch.
- `compare-summary.txt`, `judge-full-before.log`, `judge-full-after.log`, `compare.log`: bank
  evidence (the traces `full-before.trace` / `full-after.trace`, 1.5 to 1.7 GB, stay in the
  scratch folder).
- `failing-first.log`, `tests-after.log`: test runs.
- `scripts/`: extraction (`extract.py`, `trace2tsv.py`), spectral analysis (`spec2.py`),
  constant search (`solve.rs`, `verA.py`), precision race (`tinyvar.py`, `rnlib.py`), coverage
  by binade (`tinyall.py`), mid-range leads (`model6.py`, `mid2.py`).
