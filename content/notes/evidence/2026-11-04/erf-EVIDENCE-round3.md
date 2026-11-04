# ERF below 0.5: the full stored rational (round 3, 2026-11-04)

Supersedes the higher-order parts of `EVIDENCE-round2.md`. Everything measured here uses Excel
16.0 build 20527 through `bulk/bulk_runner.ps1` (own instance, blank workbook, doubles written
by array through Value2 and read back as exact bits; 20,000 calls in 12 seconds).

## Result

For `|x| < 0.5`, Excel's ERF is a (4, 5) rational in compiled double arithmetic on the x87 at
precision 64. Every operation is `RN53(RN64(..))`:

```text
t = x*x
N = Horner(t; n4..n0)     D = Horner(t; d5..d0)     each product and each sum stored
p = x*N                   y = p/D
```

| Order | N | D |
|---:|---|---|
| 0 | `0x3ff1df5e70c7956b` | `0x3fefada15778ece6` |
| 1 | `0x3fc201d8627a7d2c` | `0x3fdd191aca8d2499` |
| 2 | `0x3fa6f289d165b303` | `0x3fb79f85ba68803e` |
| 3 | `0x3f5d93a5813e372c` | `0x3f856de5da86af12` |
| 4 | `0x3f2932767df264b2` | `0x3f4609751b5b6488` |
| 5 | | `0x3ef4f20a473a5ab0` |

Round 2's linear terms (`...7d2d`, `...2496`) were one and three units off: that bracket assumed
the product `c1*t` is exact, and it is stored.

## How it was read

1. **Site boxes** (`dec/rects.rs`, `dec/climb3.py`). At a site, 2,000 doubles at a stride that puts
   about 9.38 rows on each D step. Each row's answer leaves a few (N, D) double pairs, each pair
   a rectangle in offsets from the prediction; the intersection fixes the real N and D at the site
   to about 0.001 ulp, with zero conflicting rows up to `x = 0.05`. A run of consecutive doubles
   alone cannot do this: it sees N only through one phase, so the rows must be spread.
2. **Degree.** Fits over the sites give `N` degree 4 and `D` degree 5; a further term fits to zero.
3. **Store placement** (`dec/refine.py`), on 19 sites from 0.09 to 0.4999, 38,000 rows:

   | t | products | sums | rows unexplained |
   |---|---|---|---:|
   | stored | stored | stored | 223 |
   | stored | stored | register | 568 |
   | stored | register | stored | 1,058 |
   | register | register | register | 1,152 |
   | stored | register | register | 1,244 |

4. **Coefficients** (`dec/scan.rs`, `dec/perSite.py`, `dec/perSite2.py`, `dec/sol*.py`). Runs of
   20,000 consecutive doubles at 12 sites from 0.06 to 0.49 (`dec/dense/`). Moving one coefficient
   and counting unexplained rows gives a sharp V at the value that the site sees, because a wrong
   coefficient scrambles the rounding of the stored Horner step. The per-site offsets, fitted over
   `t`, give each order's correction in integer units.

   This is a measurement against a zero target on fresh rows, not a fit to the bank: the bank was
   only scored. Five of the sites reached zero misses in 20,000 rows at once.

   How well each coefficient is known (scan on 300,000 rows unless stated):
   - N1, D1: exact. One unit off breaks 500 to 1,100 of 160,000 rows.
   - N2, D2: at the V minimum (159 misses; D2 +-1 gives 281 and 311, N2 +-1 gives 182 and 185).
   - D3: at the minimum at a step of 40 units (274 and 308 at +-40).
   - N3: a flat valley from -120 to +320 units.
   - N4, D4, D5: not scanned one at a time. Their last bits are open.

   `dec/lp2.py` (an LP on the rows) made things worse (59,712 of 60,000 on the hold-out) and
   was dropped.

5. **Cut.** 2,000 consecutive doubles below 0.5: 1,988 match. 2,000 from 0.5 up: 858, chance
   level. So the rational stops at 0.5 (`dec/half.tsv`). 0.46875 is not a cut. At `x = 0.5` itself
   the rational and `libm::erf` give the same bits, so that one point does not say which branch
   owns it.

## Independent checks

- `N/D` equals `erf(x)/x` to a flat offset of 9e-17 from 0 to 0.5 and climbs at once past 0.5
  (2e-16 at 0.55, 7e-16 at 0.6): the design interval is `[0, 0.5]`. The Taylor ratios match to 9e-17,
  4e-16, 2e-13, 3e-11, as a minimax rational should (they were not imposed).
- The bank was not used to read the coefficients: `c8` matches all 618,225 rows below 0.5.
- Fresh hold-out, 60,000 random doubles below 0.5 (half log-uniform from 2^-16, half
  uniform), asked after `c8` was fixed: 59,976 match. Misses: 23 in `[0.25, 0.5)`, 1 in `[0.125, 0.25)`.

## The source table

Not identified. The ratios are close to Cephes' erf (`|x| < 1`) but not equal: `D/D5` is 49558.8,
22761.1, 4619.6, 523.8, 33.67 against 49267.4, 22629.0, 4594.3, 521.4, 33.56. The same shape on a
narrower interval is the likely reading. Cody is refuted. Hart's ERF tables remain unchecked: no
copy. The normalisation (`D(t) = 1` near `t = 0.0221`, no coefficient equal to 1) is unexplained.

## Open

- Last bits of N3, N4, D4, D5. They only show above 0.25, at about 0.15% of rows there. More
  consecutive runs above 0.25 with per-coefficient scans will close them. N3's per-site offset has
  a parabolic trend in `t` I cannot explain from degree 4 alone; it is small (0.005 ulp of N).
- Upper branch: `x >= 0.5`. See the bead note. The grid in `erfc/grid.tsv` shows the noise doubling
  at each binade of `x*x` (at 2, 2.83, 4, 5.66, 8, ...), as an unsplit `exp(-RN53(x*x))` does.

## Final state (supersedes the numbers above where they differ)

Landed as `36a7f3d7` (merge `d6798160` on master), constants `c14` in the table at the top.

`c8` to `c14`: paired scans of N3 with N4, D3 with D4, D4 with D5 on rows above 0.25 (`grid2.py`)
followed long correlated valleys. The floor on those rows fell from 130 to about 25 of 100,000.
It stops at about 30 of 120,000 and no pair lowers it further.

| Check | `c8` | `c14` |
|---|---:|---:|
| Bank rows below 0.5 (618,225) | all | all |
| Hold-out 1, 60,000 random (seen while choosing) | 59,976 | 59,996 |
| Hold-out 2, 60,000 random, 40,000 of them above 0.25 (asked after `c14` was fixed) | 59,937 | 59,982 |

Hold-out 2 misses: 17 of 50,001 in `[0.25, 0.5)`, 1 of 4,987 in `[0.125, 0.25)`.

Bank by row against master before this step (`cmp3.py`, on rows in both traces): +77,213 / -0.
`c14` against `c8` alone is +1 / -2, all NORM.S.DIST at `z` near 0.283: rows `c8` gained and
`c14` gives back, in the open last bits class. Baseline 3,652,294; after 3,729,477 of 3,910,007
(incoming commits moved the bank scope by 43 rows, so `witness compare` refuses the pair).

By band, after: ERF, ERF.PRECISE, ERFC, ERFC.PRECISE below 0.5: every recorded row, positive
and negative. Above 0.5:

| Band | ERF.PRECISE | ERFC.PRECISE | ERFC |
|---|---:|---:|---:|
| `[0.5, 1)` | 31 / 47 | 46 / 110 | 0 / 7 |
| `[1, 2)` | 48 / 50 | 41 / 143 | 4 / 14 |
| `[2, 4)` | 64 / 64 | 33 / 210 | 6 / 37 |
| `[4, 8)` | 51 / 51 | 14 / 224 | 1 / 59 |
| `>= 8` | | 84 / 508 | 64 / 365 |

Corpus 21,583 / 22,830 before and after. `cargo xtask check` passed on the clean Linux clone on
the merged tree; one further incoming commit (collation, 2 files) came in by merge just before the
push. Browser wasm and the OneCalc replays were not run.

The `erf` scan of the upper branch so far: `bulk/erfc/grid.tsv` (40,000 points) and six 4,000-row
consecutive runs. Noise doubles at the binades of `x*x`. Dividing out `exp(-x*x)`: at 1.3 a stored
`z = RN(x*x)` and stored exponential leave the least noise (2.44 against 3.59 for exact `x*x`); at
2.5 and 5.5 exact `x*x` leaves less (3.83 against 5.17, 10.7 against 13.2). So the staging
differs by band, and the swarm's single unsplit form is not the whole story. Open.

# Round 4: ERFC above 0.5 (2026-11-04)

Landed: `erfc_upper`, `Q = E*F`, `z = RN53(RN64(x*x))`, `E = exp(-z)` stored (`excel_exp`). `F` is
Cody's published rational with stored arithmetic, a stand-in, not Excel's.

How `E` was read (`bulk/erfc/band.py`, `res2.py`): residual of `Q` over `E * true erfcx` on 220,000
answers in `[0.5, 26.5]`.

| Staging | sd at x = 24 | Max residual |
|---|---:|---:|
| exact `exp(-x*x)` | 287 | |
| Cody `AINT(16x)/16` split | 287 | |
| fdlibm low-word split | 287 | |
| plain binary64 `z`, stored `E` | 2 with outliers | 1,024 |
| `RN53(RN64(x*x))`, stored `E` | 2 | 10 |

Bank: see the commit message. Losses are all last-bit rows of one class (stand-in `F`).

## What is known about Excel's F

- `bulk/erfc/kspec.py`, `track.py`, `shape3.py`, `lines1.txt`. Residual spectrum in input-ulp space
  (20,000 random doubles from a 2^18 range at each site): a dozen or more sharp lines. A stored smooth
  `G(x)` gives a line at `frac(G'(x)*2^j)`. Lines tracked from 0.60 to 0.90 are smooth curves.
- `G''/G'` of the tracks at 0.75 falls near the Horner levels of Cephes' P and Q (0.137 against
  0.1358; 0.30 to 0.33 against 0.287; 0.44 to 0.47 against 0.47; 0.75 to 0.80 against 0.71 to
  0.73; 1.41 and 1.63 against 1.53 and 1.62) and not Cody's (0.116 for the first). So degree about
  8 in `x`, Cephes-like, as ERF's (4, 5) was. Other readings are not excluded: this is shape only.
- Mean steps (`cpd.py`, `stepbis.py`, `delta.py`). The mean of the residual is piecewise flat with
  sharp steps; one was bisected to within 8,793 doubles at `x = 1.1156792580` and measures
  `0.41 +- 0.02` units. Flats are flat to under 0.05 units across lengths up to 0.3, so the
  rational's own error there is below that, and about 80 steps cover `[0.5, 27]`, some toggling
  within 0.02. No stored polynomial value varies that slowly. Unexplained, and the first thing to
  resolve: it says something near-constant is stored.
- Not a closed form of stored `F`: `y = RN(E * RN(F))` with any offset matches 40% at best.

Why ERF's climb does not carry over: ERF started from `t -> 0`, where N and D reduce to constants
and the rows fix them absolutely. Here no such limit exists, and a run sees N only modulo its own
phase. The next step needs an absolute reading of N and D: for instance the line tracks taken
over `[0.5, 8]` with longer spans, then polynomial fits to pin scale, then boxes.
