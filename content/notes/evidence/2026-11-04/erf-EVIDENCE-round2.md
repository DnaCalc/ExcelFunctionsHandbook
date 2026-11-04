# ERF body, round 2: structure of `|x| <= 0.5`, landing below 2^-27 (2026-11-04)

Supersedes the "Cody scaled by s" reading in `EVIDENCE.md`.

## 1. The structure (proven on recorded rows)

Excel evaluates ERF for small arguments as `x * N(t) / D(t)`, `t = x*x`, on the x87 at precision
64, with **N and D stored to binary64**, then the product stored, then the quotient stored:

```text
N = RN53(RN64(N(t)))        D = RN53(RN64(D(t)))
p = RN53(RN64(x * N))       y = RN53(RN64(p / D))
N(0) = 0x3ff1df5e70c7956b   D(0) = 0x3fefada15778ece6     (N(0)/D(0) = 2/sqrt(pi) to 1 ulp)
```

Evidence: with only the linear terms `N(t) = N(0) + a*t`, `D(t) = D(0) + b*t` and
`a = 0.1406813127`, `b = 0.4546572666`, the graph reproduces every sampled recorded row in every
binade from `2^-28` to `2^-14` (3,000 per large binade; all rows in the small ones): 214/214,
176/176, 154/154, 3000/3000, 37/37, 26/26, 48/48, 32/32, 3000/3000, 2093/2093, 42/42, 44/44,
41/41, 2999/3000. Above `2^-14` the `t^2` terms start to show (39/42 at `2^-13`).
Script: `scripts/evalab.py` (`python evalab.py 0.1406813127 0.4546572666`).

How a and b were found: exact interval feasibility per row (`scripts/ascan3.py`). Each recorded
row admits a few `(N, D)` binary64 pairs, and each pair is a rectangle in `(a, b)`. The
intersection over 113,223 rows of the `2^-15` binade leaves
`a in [0.140681312752, 0.140681312755]`, `b ~= 0.454657266654`, with 113 rows conflicting
(0.1%, consistent with the `t^2` terms moving the stored N or D). The stored/extended race
(`scripts/ascan4.py`) leaves only "both stored": 1 conflict in 3,000 rows, against more than
1,700 for every other combination.

These are effective linear coefficients (they absorb `t^2` effects of about 1e-10), not exact
constants, and they are **not** landed.

## 2. What this says about the source table

- **Cody 1969 / SPECFUN CALERF is refuted.** Its normalised linear terms are `A3/A4 = 0.117618` and
  `B3/B4 = 0.450956`. Excel's are `a/N(0) = 0.125942` and `b/D(0) = 0.459276`. The earlier
  "Cody divided by s" match was only the constant-term ratio, which every accurate table shares
  (it is 2/sqrt(pi)), so it was not evidence for Cody. The scale `s` has no meaning and needs no
  explanation.
- Cephes `ndtr.c` erf (`|x| < 1`, T/U) is close but different: its normalised `T3/T4 = 0.125977`
  and `U4/U5 = 0.459310`.
- fdlibm's `|x| < 0.84375` form gives `0.0646` and `0.3979` for the same quantities. Not it.
- No local file or web search finds the stored constants. The table is unidentified. Hart et al.
  1968 (ERF tables) is the next published candidate; I have no copy.

## 3. What landed: the route below 2^-27

For normal `|x| < 2^-27`: `t < 2^-54`, so `a*t < 2^-55`, below half an ulp of `N(0)` (`2^-53`),
and `b*t < 2^-55`, below half an ulp of `D(0)` (`2^-54`). The stored N and D are then exactly
their constant terms, and the route is fully determined by the two stored constants. The bound
holds for any `|a| < 1` and `|b| < 1/2`, so it does not depend on the measured values. The
data agree: the route holds on all 249,260 recorded rows below `2^-27` and fails from `2^-27`
on (55 of 176 rows in `[2^-27, 2^-26)`, where D first rounds up at `b*t = 2^-54`).

### Bank (full witness bank, every function)

`cargo xtask witness judge --bank full --trace ...` before and after, then
`cargo xtask witness compare --before ... --after ...`, run in a source snapshot with
the evidence repository beside it:

| Function | Rows | Before | After | Gains | Losses |
|---|---:|---:|---:|---:|---:|
| ERF.PRECISE | 618,437 | 252,580 | 387,012 | 134,432 | 0 |
| GAUSS | 9,728 | 8,569 | 9,247 | 678 | 0 |
| all others | | unchanged actuals | | 0 | 0 |
| **All** | 3,909,807 | 3,267,143 | 3,402,253 | 135,110 | **0** |

### Corpus (offline, committed oracle captures, Excel not started)

`cargo xtask corpus --oracle --no-capture --cache <empty dir> --out <dir>` (the route
`racer/corpus_cases.py` uses): before 21,556 / 22,830 (`corpus-before/summary.md`); after: see
`corpus-after/summary.md` and the commit message.

### Pinned tests

`crates/calc-func/tests/witnesses/ERF.PRECISE.json` (37 rows) and `GAUSS.json` (11 rows) were
re-pinned from the after trace: every changed row is a gain, now equal to Excel.
`tests/f1_stats.rs::tiny_erf_records_known_gaps` pinned the old wrong results; it is now
`tiny_erf_matches_recorded_answers` with exact Excel values (ERF, ERF.PRECISE, GAUSS).

## 4. Excel session (own instance, blank workbook, Value2 doubles)

`excel-q1/questions.json` (793 calls with predictions) and `excel-q1/answers.json` (Excel 16.0
build 20527, started and quit by the script).

- **Subnormal arguments** cannot be asked through a cell: Excel reads each written subnormal
  back as 0 (7 input mismatches), and ERF.PRECISE then returns 0. DBL_MIN and 2*DBL_MIN follow
  the tiny route exactly (`0x00120dd750429b6e`, `0x00220dd750429b6e`; libm gives `...6d`). The
  `is_normal()` guard is therefore unobservable from cells and stays as written.
- **Near 0.5** (16 points from 0.46875 - 2^-20 to 0.53125, plus -0.49): recorded for the full
  model. They cannot discriminate yet without the higher coefficients.
- **Edge grids** (N's first step at `a*t = 2^-53`, D's at `b*t = 2^-54` and `3*2^-54`): 256
  points each, about 2^13 x-ulps wide. Excel stays on one side throughout, so the edges lie
  outside the windows. The windows were sized from the effective `a, b`, whose `t^2` bias
  (relative 2^-32) is far wider. The recorded rows put D's first edge between
  `t = 2.180*2^-54` and `2.2034*2^-54` (predicted `2.2005`). A second round should centre on
  that bank bracket and bisect.

## 5. Next

1. Bisect the three edges in Excel (about 5 rounds of 256 cells each). This gives `a` and `b`
   to about 2^-52 relative at tiny `t`, where the `t^2` terms cannot reach, and so the stored
   linear coefficients. Their binary64 values, with `N(0)` and `D(0)`, are what a table search
   (Hart's ERF tables) can be matched against.
2. Fit nothing above `2^-14`. The higher terms need either the table or the same edge
   bisection at larger `t`.
3. The losses classification the owner asked for is not needed for the landed part: it has
   none. For the full rational, the first expected loss class is rows where `t^2` terms move the
   stored N or D (0.1% of the `2^-15` binade).

## 6. Update after the edge bisection (this supersedes sections 3 to 5 where they differ)

The second Excel run (`excel-edges/`, `bisect_edges.py`, `excel-edges/result.json`) centred on
the bank brackets and bisected each edge. Every query was answered by one of the two hypotheses
(`other 0`) and in order (`out-of-order 0`):

| Edge | x bracket | Reads as |
|---|---|---|
| D 0 to 1 | `0x3e47bc15e9263272` / `...73` | `D1 in [0.45465726642994159, 0.45465726642994175]` |
| D 1 to 2 | `0x3e548c550652f541` / `...f55a` | `D1 in [0.45465726642993831, 0.45465726642994225]` |
| N 0 to 1 | `0x3e5e280f6c1e72dc` / `...dd` | `N1 in [0.14068131264228026, 0.14068131264228029]` |

The two D brackets agree only with the 64-bit register's half unit on each edge. Without it they
read 0.454435 and 0.454731. That is direct evidence of `RN53(RN64(...))` on the stored D.
`N1/N0 - D1/D0 + 1/3 = 1.2e-16`: the measured coefficients recover erf's Taylor coefficient,
which nothing imposed. The earlier values in section 1 were biased by the `t^2` terms.

Stored values: `N1 = 0x3fc201d8627a7d2d` (the only double in its bracket), `D1` =
`0x3fdd191aca8d2496` or `...97`. Both candidates match 250,554 of 250,554 recorded rows in
`[2^-27, 2^-16)` (`linear.py`), so the landed `...96` is not yet distinguished from `...97`.

Landed in two commits on master: `92234dc8` (below 2^-27) and `f1bf1f02` (linear route below
2^-16).

| Bank | Before | After 92234dc8 | After f1bf1f02 |
|---|---:|---:|---:|
| ERF.PRECISE (618,437) | 252,580 | 387,012 | 542,098 |
| GAUSS (9,728) | 8,569 | 9,247 | 9,247 |
| All (3,909,807) | 3,267,143 | 3,402,253 | 3,557,339 |

Losses: zero at each step (`compare2.log`, `compare3a.log`). The offline corpus stays 21,556 /
22,830 at each step (`corpus-before`, `corpus-after`, `corpus-after2`).

Check status: on a clean snapshot of `fe89d40e` plus the change, the only failures were
`dc_xb26__spill_shape` (`link.exe` 1104 from the long scratch path; it passed in the clone's run)
and, on the last run, `small_erf_records_known_polynomial_gaps`, which pinned the old wrong
values. I fixed that test and re-ran `f1_stats` (38 passed) and the `normal_dist_common` tests,
but did not repeat the whole check after that one test edit.

Counting correction: rows below `2^-27` are 249,260, not 249,300 or 249,514 as `EVIDENCE.md` and
the message of `92234dc8` say.

Next, `t^2`. `p2scan.py` with `D2` tied by erf's third Taylor coefficient gives
`N2 = 0.0448 +- 0.0001`, which clears all 574 rows in `[2^-16, 2^-11)` that the linear route
misses. It did not rescore the rows the linear route already matches, so it is a lead only.
Measure `N2` and `D2` the same way: edges at larger `t`, predicted from the terms already known,
checked by the next Taylor coefficient (1/10, then 1/42, 1/216). Above about `2^-9`, N steps every
few input ulps, so consecutive runs of about 128 doubles at about ten sites replace bisection.
`excel-q1/` holds 16 points around 0.46875 and 0.5 for the cutoff question once `N` and `D` are
complete.
