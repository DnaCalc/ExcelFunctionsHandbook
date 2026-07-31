# RV-normal-discrete — independent re-derivation of measurement attributions

Slice: **normal-discrete**. Surfaces: NORMDIST, NORMSDIST, NORMINV, NORMSINV,
NORM.S.DIST, LOGNORM.DIST, LOGNORMDIST, PHI, GAUSS, ERF, ERFC, ERF.PRECISE,
ERFC.PRECISE, BINOMDIST, BINOM.DIST, NEGBINOMDIST, NEGBINOM.DIST, POISSON.DIST,
WEIBULL, WEIBULL.DIST, EXPONDIST, EXPON.DIST.

Method: STEP 1 (below) was derived **only** from OxFunc primary sources. FOUNDATION.md
was not opened until STEP 1 was complete and written.

Sources read in STEP 1 (all under `C:/Work/DnaCalc/OxFunc/`, read-only):
- `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md`
- `docs/function-lane/DISCREPANCY_RULED_OUT_LEDGER.csv`
- `docs/bugs/streams/BUG-FUNC-013_normal_distribution_exact_value_accuracy_gap.md`
- `docs/bugs/streams/BUG-FUNC-021_w090_statistical_numeric_exactness_drift.md`
- `docs/function-lane/ERFC_EXCEL_EMULATION.md`
- `docs/EXCEL_MATH_DEVIATION_CATALOG.md`
- plus the primaries those cite: `docs/function-lane/W109_CAMPAIGN_RESUME_20260718.md`,
  `docs/function-lane/W109_G3-01_GRATIO_IDENTIFICATION_20260716.md`,
  `docs/function-lane/W109_WALL_CLUES_LEDGER.md`,
  `docs/bugs/streams/BUG-FUNC-018_successor_scalar_parameter_array_lift_gap.md`,
  `docs/bugs/streams/BUG-FUNC-033_erf_erfc_over_coerce_logical.md`,
  `docs/KNOWN_EXACTNESS_DEVIATIONS.md`,
  `smart-fuzzer/planning/W097-R-GH-closed-streams-cell-ref-resweep.md`,
  `smart-fuzzer/planning/W097-R-D-stat-distribution-cell-ref-resweep.md`,
  `docs/function-lane/W109_G6_PMT_RESUME_20260723.md`,
  `.tmp/w62-statistical-distributions-compat-b-results.csv`

---

## 0. The four measurement-kind buckets used below

- **production-oxfunc** — current shipped OxFunc kernel vs live Excel.
- **research-model-or-candidate** — a racer/candidate/model score, not the shipped kernel.
- **excel-vs-truth** — Excel vs the correctly-rounded/mathematical value (no OxFunc involved).
- **excel-vs-excel-identity** — one Excel surface vs another Excel surface.
- **instrument-validation** — a distribution surface used as a *read-out* for an internal
  primitive (exp/ln/expm1); the number scores the primitive, not the surface.
- **internal-regression** — local test-suite / pin counts.

---

## 1. The three headline questions, answered from source first

### 1.1 "NORMDIST 8/10 numeric versus 10/10 structural"

**Verdict: both figures are real records, but NEITHER is a NORMDIST measurement.
The 8/10 record does not contain NORMDIST at all.**

**The 10/10 (structural, build 20026).** BUG-FUNC-018, the *array-lift / array-admission*
stream:

> "Signed off against **live Excel 16.0 build 20026** (workbook compatibility 2). All ten
> representative reopened scalar-parameter array-lift formulas were re-evaluated in Excel and
> compared bit-for-bit against the current OxFunc surface output"
> — `docs/bugs/streams/BUG-FUNC-018_successor_scalar_parameter_array_lift_gap.md:11`

> "`10/10` exact typed bit matches; `0` mismatches. The array-admission class this stream tracks
> is resolved on the current baseline. Numeric exactness inside the lifted statistical kernels
> remains tracked separately under `BUG-FUNC-021`."
> — `…BUG-FUNC-018…md:29-31`

The ten rows are ten *different functions* (BINOMDIST, NORMDIST, COMPLEX, DOLLARFR,
SWITCH ×2, IFS ×2, ADDRESS ×2). NORMDIST's own row is:

> "| `=NORMDIST(42,40,1.5,{TRUE,TRUE})` | `array:1x2:[0x3fed14cc3547f8da ×2]` |"
> — `…BUG-FUNC-018…md:19`

So the NORMDIST-specific evidence is **1 row, 1/1, structural (array admission), build 20026**.
`10/10` is a **group total across ten surfaces**, and the stream itself says numeric exactness
is explicitly *out of scope* for it (line 30-31). Note also a build split inside the same file:
the underlying W090 tranches ran on a different build —

> "**Ref notes**: W090 successor tranches used live Excel COM on Excel `16.0`
> build `19929`, workbook Compatibility Version `2`"
> — `…BUG-FUNC-018…md:39-41`

— so 20026 is the *sign-off* build and 19929 the *generation* build. BINOMDIST is the other
slice member in that ten (`…:18`).

**The 8/10 (numeric, build 19929).** It is the BUG-FUNC-013 row of the W097 R-G closed-stream
cell-ref re-sweep:

> "| `BUG-FUNC-013` |  `10` |   `8` |   `2` |  `0` |    `0`  |"
> — `smart-fuzzer/planning/W097-R-GH-closed-streams-cell-ref-resweep.md:36`

> "- Cases: `15` (4 BUG-FUNC-005 + 10 BUG-FUNC-013 + 1 BUG-FUNC-014)
>  - Rollup: matches `13`, drifts `2`, kind drift `0`, blocked `0`
>  - Excel environment: `16.0` build `19929`"
> — `…W097-R-GH…md:29-31`

The ten witnesses are: NORM.DIST, NORM.INV, NORMSDIST, NORMSINV, NORM.S.DIST, NORM.S.INV,
ERF(1), ERFC(1) — all eight bit-exact — plus GAUSS(1) at 2 ULP and PHI(0) at 1 ULP
(`…W097-R-GH…md:60-76`; mirrored in `BUG-FUNC-013…md:134-156`). **NORMDIST and NORMINV are
absent from this set.** The stream doc restates the split:

> "The four direct closure witnesses (`NORM.DIST`, `NORM.INV`, `NORMSDIST`, `NORMSINV`) plus
> the two `NORM.S.*` aliases plus `ERF(1)` / `ERFC(1)` all match Excel bit-for-bit."
> — `docs/bugs/streams/BUG-FUNC-013…md:145-148`

The build is **not restated inside BUG-FUNC-013** for the W097 R-G table; it lives only in the
tranche record (19929). All ten rows are single pinned witnesses, one per surface — this is a
1-point-per-surface record, not a corpus pass rate.

**Consequence:** attributing "8/10" to NORMDIST is a wrong-attribution error twice over
(wrong surface set membership, and a group total read as a per-surface rate). Attributing
"10/10" to NORMDIST is a group total read as a per-surface rate (NORMDIST's own datum is 1/1).

### 1.2 POISSON's 34,000

**Verdict: the split into 30,000 + 4,000 is real and correctly described in source, but the
34,000 is NOT a POISSON.DIST surface pass rate. It is the internal-exp primitive's evidence
grade, read out through the POISSON.DIST k=0 window only. The 30,000 is the IDENTIFICATION
corpus; only the 4,000 is a held-out gate on production.**

The join is stated once, in the GRATIO note:

> "- **b26P POISSON: 4,000/4,000 through the production RN-chain path** — with
>   b23's 30,000 that is 34,000 consecutive fresh rows, zero misses. The exp
>   primitive + POISSON.DIST(0, lambda) are sign-off grade."
> — `docs/function-lane/W109_G3-01_GRATIO_IDENTIFICATION_20260716.md:935-937`

Note the surface is spelled out as **`POISSON.DIST(0, lambda)`** — k=0 only.

The 30,000's role is unambiguous — it is the corpus the internal-exp identity was *fitted
against* (the 153-row off-CR fingerprint set inside it is the discriminator):

> "- POISSON fingerprint (30k rows): x87-f2xm1-naive reproduces the off-CR set
>   **153/153 EXACTLY** (29,997/30,000 overall, 3 false-positives)."
> — `…GRATIO…:781-782`

> "**INTERNAL EXP = ONE near-CR routine; publication varies by site.**
> - Direct read (POISSON k=0, 30k rows): 99.490% == RN(CR), 0.457% CR-1,
>   0.053% CR+1 — BIAS-LOW"
> — `…GRATIO…:729-731`

That is an **instrument-validation** read: POISSON.DIST k=0 is being used as a window onto
Excel's exp, because at k=0 the pmf is `exp(-λ)`. The 30,000 got a *second* score after the
hardware chain landed:

> "- Hardware verification first: RN53(real chain) = 30,000/30,000 on the
>   POISSON channel"
> — `…GRATIO…:820-821`

and again in the staging note:

> "**Site-dependent publication RE-CONFIRMED on both sides** … POISSON direct
> store = RN53 (chain RN 29,997/30,000 vs chop 49.9% …)"
> — `…GRATIO…:917-920`

So 30,000 carries three different numbers in three different roles (29,997 idealized model;
30,000 real hardware; the same corpus as chop-vs-RN discriminator). Only b26P is fresh:

> "b26 batteries designed for the clean held-out gate … b26P POISSON re-confirm."
> — `…GRATIO…:924-926`

The resume attributes the 34,000 to the **primitive row**, not to POISSON:

> "| Internal exp | x87 F2XM1 chain … | excel_exp / excel_exp_rz (excel_numeric); all 49
> substrate sites | **34,000/34,000 held-out (POISSON) — sign-off grade** |"
> — `docs/function-lane/W109_CAMPAIGN_RESUME_20260718.md:18`

and the corpus rollup line scopes it to k=0 explicitly:

> "POISSON k=0 34,000+ consecutive exact"
> — `…W109_CAMPAIGN_RESUME…:35-36`

> "b26 POISSON 4,000/4,000" (post-lane-3 standing numbers) — `…GRATIO…:1104`
> "POISSON k=0 window 4,000/4,000" — `…GRATIO…:1082`

POISSON k≥1 is explicitly OPEN and much worse:

> "- POISSON pmf: k=1 = extended-composed direct product (exact at large
>   λ); k≥2 = **Loader saddle-point dpois bit-exact at λ≳14**; small-λ
>   staging + branch structure open."
> — `…W109_CAMPAIGN_RESUME…:69-71`

> "Direct product at k=1 scores 25.7% with ±10-ULP tails."  — `…GRATIO…:1015`
> "- Small-λ (all k): neither model as staged; mask families … cap at 70/43/41%." — `…GRATIO…:1028-1029`

And the withdrawal of the previous claim is itself a finding:

> "The "POISSON direct-product route proven, ~21% unexplained at k=1" claim
> is WRONG and is hereby withdrawn: the k=0 window is ROUTE-BLIND"
> — `…GRATIO…:1011-1012`

**Build near the figure: not restated.** The only build statement in the GRATIO note is for
battery B1: "## The multi-view collapse (battery B1, 829 probes, build 20131)"
(`…GRATIO…:15`). b23/b26P/b28/b28c/b36 do not restate it.

### 1.3 WEIBULL.DIST / EXPON.DIST sign-off at 99.983% / 100.000%

**Verdict: real, held-out, production-kernel, and correctly attributed to the two MODERN
surfaces. The legacy aliases WEIBULL and EXPONDIST were NOT separately measured.**

> "- **b28 held-out (fresh 6,000 rows, production kernel): 5,999/6,000 =
>   99.983%**, sole miss -2 ULP (chain-microdetail class, see clue ledger).
>   LANDED in `weibull_dist_kernel`."
> — `…GRATIO…:990-992`

> "**EXPON.DIST (b28b + b28c):** body is the same legacy x87 per-op-DR class:
> inner `lambda*x` 14/14 DR, pdf outer `lambda*e` 24/24 DR, twins 40/40.
> Landed (`excel_x87_mul` at both sites). **b28c held-out (fresh 4,000 rows,
> production): 4,000/4,000 = 100.000%.**"
> — `…GRATIO…:994-997`

Resume table rows name the surfaces and the landed kernels explicitly
(`…W109_CAMPAIGN_RESUME…:25` WEIBULL.DIST → `weibull_dist_kernel`; `:26` EXPON.DIST →
`expon_dist_kernel`). Catalog header says the same:

> "WEIBULL.DIST + EXPON.DIST bodies identified as legacy x87 per-op-double-rounded units
> and SIGNED OFF held-out 99.983% / 100.000%"
> — `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:5-7`

The identification corpora (fitted, distinct from the gates) are b27/b27b:
"WEIBULL.DIST identification (b27 pdf corpus 5,400 rows + b27b + b28)" (`…GRATIO…:974`);
"the round-6 exhaustive tree×spill-mask race put `T3|SS` at 1,600/1,600" (`…GRATIO…:979-981`);
"b27b D1 48/48: outer ops are x87-DR" (`…GRATIO…:988`).

Legacy aliases: nothing in W109 measures **WEIBULL** or **EXPONDIST** separately. The only
WEIBULL (legacy) datum anywhere is a rounded-decimal witness from 2026-03-18:
"`WEIBULL(2,3,4,TRUE)` -> `0.117503097415405`"
(`docs/function-lane/W24_BATCH06_SPECIAL_DIST_EXECUTION_RECORD.md:44`). The contract slices
record them as alias/lane-followers only:
"`EXPONDIST` is the compatibility alias of `EXPON.DIST`"
(`docs/function-lane/FUNCTION_SLICE_STATISTICAL_DISTRIBUTIONS_AND_COMPAT_A_CONTRACT_PRELIM.md:51`).
Contrast: the *gamma-side* legacy≡modern collapse WAS measured, but it lists a different set of
surfaces and does not include WEIBULL/EXPON —
"Legacy ≡ modern bit-for-bit everywhere probed: CHIDIST≡CHISQ.DIST.RT, FDIST≡F.DIST.RT,
TDIST(·,·,1)≡T.DIST.RT, GAMMADIST≡GAMMA.DIST, BETADIST≡BETA.DIST." (`…GRATIO…:17-19`).
So **inheriting 99.983% / 100.000% onto WEIBULL / EXPONDIST is a sibling-inheritance claim
with no measurement behind it.**

**Third-number conflict on EXPON.DIST.** Three different EXPON.DIST-adjacent numbers exist and
must not be conflated:
- `4,000/4,000 = 100.000%` — b28c held-out, EXPON.DIST production surface (`…GRATIO…:997`).
- `17,996/18,000 (99.978%)` — the **expm1 primitive**, read at EXPON/WEIBULL cdf sites
  (`…W109_CAMPAIGN_RESUME…:19`).
- `99.96%` — a later PMT-lane aside: "touches the shared excel_expm1_internal
  (serves EXPON.DIST at 99.96% SSE2)" (`docs/function-lane/W109_G6_PMT_RESUME_20260723.md:107`).
- `232/234` — "EXPON.DIST … the **statistical** `expm1` — all-double Kahan, **232/234** |
  direct oracle" (`docs/function-lane/W109_G6_PMT_TAKEOVER_BRIEF.md:172`) — an
  instrument-validation read of Excel's expm1 through EXPON.DIST, not an EXPON.DIST pass rate.

### 1.4 PHI's 764/764 and whether it is on an erf substrate

**Verdict: 764/764 is real, and PHI is explicitly NOT on an erf substrate. Held-out status is
NOT stated; the number reads as the identification round's answered-rows total.**

> "PHI is resolved out of this row (W109 2026-07-11: `RN53(RN64(x·x))` -> x87 EXP ->
> `RN53(RN64(e·RN(1/sqrt(2π))))` with a live-pinned subnormal publication flush;
> `764/764` answered rows, see the ruled-out ledger and `smart-fuzzer/work/w109/G3-07-phi`)."
> — `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:119`

That op-graph is the standard-normal **pdf** — a square, an exp, and a multiply by
`1/sqrt(2π)`. No erf, no erfc, no CDF. GAUSS, by contrast, is the one that needs erf and is
still open in the same row: "Standard-normal `Phi(z)-0.5` drift, `2` ULP on the stable
witness; needs the erf/CDF substrate (Phase-5 adjacent)" (same line, `:119`). Row maturity `M1`.

Held-out: the phrase used is "answered rows", not "held-out"/"fresh"/"never-probed", and the
ruled-out ledger shows candidates were killed *on those same live rounds*:

> "\"G3-07-gauss-phi\",\"w109-phi\",\"PHI as division by RN(sqrt(2pi)); strict (single-rounded)
> square or final multiply; platform exp\",\"live rounds: divide staging killed on r1; strict
> stagings killed on 39 constructed window rows; flush band pinned at bit resolution\""
> — `docs/function-lane/DISCREPANCY_RULED_OUT_LEDGER.csv:22`

So 764/764 is fitted-corpus (identification), not a held-out gate. Build not restated near it.
Chronology check: PHI(0) was 1 ULP off on 2026-05-10 (build 19929, `BUG-FUNC-013…md:156`) and
the identification landed 2026-07-11 — consistent, not contradictory.

---

## 2. Per-surface findings

### NORMDIST (compatibility alias of NORM.DIST)
- Structural, **1/1**, build **20026**: `=NORMDIST(42,40,1.5,{TRUE,TRUE})` →
  `array:1x2:[0x3fed14cc3547f8da ×2]`, OxFunc/Excel identical (`BUG-FUNC-018…md:19`; the
  10/10 total at `:29` is a ten-surface group total).
- Numeric, rounded-decimal only, build **16.0.19822.20114**:
  `"W62-NORM-005","normal_log","=ROUND(NORMDIST(42,40,1.5,TRUE),9)","0.908789","0.90878878","0.90878878","True"`
  (`.tmp/w62-statistical-distributions-compat-b-results.csv:2`). This is a ROUND(...,9)
  comparison — **not** bit-exact evidence.
- **No bit-exact numeric pass rate exists for NORMDIST.** It is absent from BUG-FUNC-013's
  10-witness set, absent from BUG-FUNC-021's residual list and from the W097 R-D histogram,
  and absent from every W109 corpus.

### NORMSDIST (compatibility alias of NORM.S.DIST)
- Production per-surface, build **19929** (build stated in the tranche record, not in
  BUG-FUNC-021): `| `NORMSDIST`    |   `3` |   `1` |    `3` |     `3` |       `47` | `47`  |`
  — `docs/bugs/streams/BUG-FUNC-021…md:147`. i.e. 3 sampled rows, 1 match, ULP 3..47.
  Corpus: "`1,000,000` local cases, seed `17`, `800` Excel-sampled candidates"
  (`smart-fuzzer/planning/W097-R-D-stat-distribution-cell-ref-resweep.md:43`); build at `:46`.
  **Internal inconsistency in the source table**: `total 3`, `match 1`, `drifts 3` — drifts
  equals total on every row of that histogram, which cannot be reconciled with match=1.
  Record as ambiguity, not resolved.
- Single-witness bit-exact: `=NORMSDIST(0)` → `0x3fe0000000000000` both sides
  (`BUG-FUNC-013…md:138`), one of the 8/10.
- Named in the open KED-STAT-001 / BUG-FUNC-021 residual list (`BUG-FUNC-021…md:105`).

### NORMSINV (compatibility alias of NORM.S.INV)
- Production per-surface, build **19929**:
  `| `NORMSINV`     |   `2` |   `1` |    `2` |    `21` |       `21` | `21` |`
  — `BUG-FUNC-021…md:148`. 2 sampled, 1 match, 21 ULP.
- Single-witness bit-exact `=NORMSINV(0.975)` → `0x3fff5c0331eeff82` (`BUG-FUNC-013…md:139`).
- Open residual (`BUG-FUNC-021…md:105`, `KNOWN_EXACTNESS_DEVIATIONS.md:96-97`).

### NORMINV (compatibility alias of NORM.INV)
- **No figure.** Only a rounded-decimal witness:
  `"W62-NORM-006","=ROUND(NORMINV(0.9,40,1.5),9)","41.92233",…,"True"` (w62 csv:3), and a
  similar-risk-scan mention as an alias of the closed NORM.INV lane
  (`BUG-FUNC-013…md:85`). Not in the 8/10 set. Not in BUG-FUNC-021.

### NORM.S.DIST
- Single-witness bit-exact, build **19929**: `=NORM.S.DIST(0, TRUE)` → `0x3fe0000000000000`
  (`BUG-FUNC-013…md:140`; `W097-R-GH…md:66`). One of the 8/10.
- `=NORM.S.DIST(1.25,TRUE)` is listed as a *representative witness of an open residual*
  under KED-STAT-001 (`docs/KNOWN_EXACTNESS_DEVIATIONS.md:122`) — so the surface is not
  uniformly clean; the 0-point witness is a special value (exactly 0.5).

### LOGNORM.DIST
- **No bit-exact pass rate.** Rounded-decimal witness, build 16.0.19822.20114:
  `"W62-LOG-001","=ROUND(LOGNORM.DIST(4,1.2,0.4,TRUE),9)","0.679298",…,"True"` (w62 csv:6).
- The only W109 number touching "LOGNORM" is an **instrument-validation** read of the
  internal ln, and the surface is written unqualified:
  > "**INTERNAL LOG = CORRECTLY ROUNDED (RN53), bit-for-bit.** Proven two ways:
  > LOGNORM candidate-matched decode (b24: 2,151 RN-exact rows all delta 0; 849
  > interval rows all contain 0; zero routing surprises; …)"
  > — `…GRATIO…:718-722`
  Whether those 2,151 rows are LOGNORM.DIST or LOGNORMDIST is **ambiguous in source**, and
  the figure scores the internal ln, not the LOGNORM surface.
- Error lane: `=LOGNORM.DIST(0,1.2,0.4,TRUE)` → `#NUM!` matched (w62 csv:13).

### LOGNORMDIST (compatibility alias)
- **No bit-exact pass rate.** Rounded witness `"W62-LOG-003"` True (w62 csv:8). Listed in the
  BUG-FUNC-018 affected-surface sample (`BUG-FUNC-018…md:192`) but not among the ten signed
  rows. Contract: "`LOGNORMDIST` follows the cumulative lane of `LOGNORM.DIST`"
  (`FUNCTION_SLICE_STATISTICAL_DISTRIBUTIONS_AND_COMPAT_B_CONTRACT_PRELIM.md:51`).

### PHI
- **764/764 answered rows**, identification (fitted) corpus, build not restated
  (`OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:119`, quoted in §1.4). NOT an erf substrate.
- Prior state: 1 ULP drift at PHI(0), build 19929 (`BUG-FUNC-013…md:156`).
- Also flagged as a 1-ULP class under legacy plumbing in BUG-FUNC-027 (`:307-308`).

### GAUSS
- **No pass-rate figure.** Open G3-07: "Standard-normal `Phi(z)-0.5` drift, `2` ULP on the
  stable witness; needs the erf/CDF substrate" (`…CATALOG.md:119`), severity NUM-S, maturity M1.
- Witness, build 19929: `=GAUSS(1)` local `0x3fd5d897a241a6fa` vs Excel `0x3fd5d897a241a6fc`,
  2 ULP (`BUG-FUNC-013…md:155`; `W097-R-GH…md:75`). One of the 2 drifts in the 8/10.
- Predicted to close with the erf sub-lane: "same sub-lane closes ERF.PRECISE/GAUSS
  G4-04/G3-07" (`…CATALOG.md:113`).

### ERF
- Single-witness bit-exact, build 19929: `=ERF(1)` → `0x3feaf767a741088b` both sides
  (`BUG-FUNC-013…md:142`). One of the 8/10.
- Structural, build **20026**: logical-operand rejection fixed —
  "`ERF`, `ERF.PRECISE`, `ERFC`, `ERFC.PRECISE` coerced a logical operand to a number
  (TRUE→1) and computed, where Excel returns `#VALUE!`"
  (`docs/bugs/streams/BUG-FUNC-033_erf_erfc_over_coerce_logical.md:20-23`); validation
  "all four ERF/ERFC `arg0_logical` probes now `error:Value` (match Excel)" (`:47-49`);
  build "Excel `16.0` build `20026`" (`:16`).
- **No production numeric pass rate for legacy ERF.** ERF is not in the G4-04 row
  (which names ERF.PRECISE/ERFC.PRECISE only, `…CATALOG.md:127`).

### ERFC
- Single-witness bit-exact, build 19929: `=ERFC(1)` → `0x3fc4226162fbddd5` (`BUG-FUNC-013…md:143`).
- **20/48 on a FITTED witness set** (the only corpus-scale ERFC number):
  > "| **correction-fit kernel (this commit)** | **20** | cross-platform, 0 regressions |"
  > — `docs/function-lane/ERFC_EXCEL_EMULATION.md:44`, under the heading
  > "## Evidence summary (widened 48-point positive witness set)" (`:38`).
  The fit was *trained on those very points*, with anchors force-fitted:
  "matched-anchor points carry weight 1e12 so the fit is forced through them exactly"
  (`:29-31`). 28 points remain blocked (`:55-63`), worst residual 6 ULP (`:66`).
  Which of ERFC vs ERFC.PRECISE the 48 points were captured on is **ambiguous in source** —
  the doc title covers both and the witness table is not per-surface.
- The math-deviation catalog restates it as partial and jointly attributed:
  "**Status: PARTIALLY reproduced** (~20/48 positive witnesses bit-exact; ~28 still open in
  the discrepancy catalog)" for "`ERFC(x)`, `ERFC.PRECISE(x)`, positive tail `x ≥ 1.25`"
  (`docs/EXCEL_MATH_DEVIATION_CATALOG.md:239-250`).
- **Staleness flag:** the ERFC_EXCEL_EMULATION correction-polynomial model is superseded by
  W109, which found no coefficient tables at all:
  "**W109 2026-07-17 — NO coefficient tables exist: ERF/ERFC.PRECISE ARE the NSWC gratio a<1
  branches themselves**" (`…CATALOG.md:127`). The 20/48 therefore describes a kernel design
  the project has since abandoned.
- Structural: same BUG-FUNC-033 fix (build 20026). Residual noted:
  "Remaining ERFC `arg0_text_number` 2-ULP drift" (`BUG-FUNC-033…md:60`).

### ERF.PRECISE
- **Excel-vs-truth**, not a pass rate:
  "6. **Excel's erf is near-CR** (ERF.PRECISE 158/176 CR-exact, ±1-2 tails)" (`…GRATIO…:50`).
  Same 176-row corpus used to refute published implementations bit-exactly: NSWC 113/176,
  Cody CALERF 121/176, fdlibm s_erf 160/176, Boost 155-157/176, UCRT erf 146/176
  (`…GRATIO…:50-52`, `:75-78`).
- **Excel-vs-Excel identity**, 160/160:
  > "**Wiring PROVEN by cross-view**: `GAMMA.DIST(k²/1024, ½, 1, TRUE) ≡ ERF.PRECISE(k/32)`
  > and `CHIDIST(k²/512, 1) ≡ ERFC.PRECISE(k/32)` — **160/160 AND 160/160 bit-exact**"
  > — `…GRATIO…:68-70`
- **Research model**, not production: "true-x87 Rust race (`check_erf190.rs`, Ext80 fFEXP/fFLN,
  512 spill configs): 663/1218 on z<0.5 (misses ±1, one ±2) … ~92% of rows within ±1"
  (`…GRATIO…:127-131`); "C10r composite 67.65%" (`…W109_CAMPAIGN_RESUME…:49`); j-scan
  "windowed **1,154/1,508 = 76.5%**" on a *development* corpus (`…GRATIO…:1203`).
- **Reserved held-out gate, never raced**: "**b9heldout (256 rows) NEVER RACED — the promotion
  gate.**" (`…W109_CAMPAIGN_RESUME…:53`); "b9heldout (256 rows) remains the reserved unraced
  promotion gate." (`W109_WALL_CLUES_LEDGER.md:182`).
- Row status: G4-04, NUM-S, M2, open (`…CATALOG.md:127`). No production OxFunc pass rate exists.

### ERFC.PRECISE
- **Excel-vs-Excel identity** 160/160 (`…GRATIO…:68-70`, above).
- Excel-vs-published-candidate refutations: Cody CALERF "56/176 erfc" (`…GRATIO…:51-52`);
  "erfc side = the a<1 CF with unsplit exp argument, proven by messy-grid regression slope
  +0.95" (`…CATALOG.md:127`).
- 352 captured ladder points shared with ERF.PRECISE: "352 ladder points captured
  (`answers-erfp/erfcp.json`)" (`…GRATIO…:52-53`).
- Shares the 20/48 fitted number with ERFC (ambiguous attribution, see ERFC).
- No production pass rate.

### BINOMDIST (compatibility alias of BINOM.DIST)
- Structural, **1/1**, build 20026: `=BINOMDIST(2,4,0.25,{FALSE,FALSE})` →
  `array:1x2:[0x3fcb000000000001 ×2]` (`BUG-FUNC-018…md:18`).
- Numeric claim of exactness, no count: "`BINOM.DIST` / `BINOMDIST` retain their prior
  log/exp path because the W090 replay showed those rows were already exact there"
  (`BUG-FUNC-021…md:81-83`). "those rows" = the W090 replay rows only.
- **Not separately measured in W109.** Lane 8 names BINOM.DIST.

### BINOM.DIST
- **Production, held-out gate, build not restated near the figure**:
  > "Lane-8 (2026-07-18, agent-T/U): BINOM.DIST pmf IDENTIFIED as R dbinom_raw … and LANDED
  > (85e91e4): b29 8.76%->49.81%, fresh b36 gate 49.59% overall (k=0 81.6%, k=n 96.4%,
  > general-k 37.9% with quantified n-dependence)."
  > — `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:113`
  Resume restates: "Scores: b29 8.76→49.81%, b34 52.17%, b35 75.51%, FRESH b36 gate
  49.59% overall (k=n 96.4%, k=0 81.6%, general-k 37.9% …)"
  (`…W109_CAMPAIGN_RESUME…:82-84`). **b29/b34/b35 are the fitted corpora; only b36 is fresh.**
- Sub-branch fitted score: "b29b (BINOM.DIST(0, n, p<0.1), 400 fresh rows): … `exp(-bd0(n,nq) - np)`
  matches 383/400" (`…GRATIO…:1063-1066`) — described as fresh, and it is a *k=0 branch*
  control, not the surface.
- Row OPEN: "(i) the 72/475 second lc-flip source …; (ii) small-operand bd0-direct smooth
  bodies" (`…W109_CAMPAIGN_RESUME…:84-86`).
- Beware: 49.59% is the *whole-surface* fresh gate; 96.4%/81.6%/37.9% are k-strata **within**
  it, not separate surfaces.

### NEGBINOMDIST (compatibility alias)
- **Production per-surface, build 19929**:
  `| `NEGBINOMDIST` | `138` |   `7` |  `138` |     `1` |       `20` | `237` |`
  — `BUG-FUNC-021…md:146`. 138 sampled, 7 match, ULP 1..237. Same drifts=total anomaly.
- "`NEGBINOMDIST` scalar mass remains one ULP high in the W089 seed replay"
  (`BUG-FUNC-021…md:83-84`).
- Named member of open row G3-01 (`…CATALOG.md:113`) and of KED-STAT-001
  (`KNOWN_EXACTNESS_DEVIATIONS.md:96`).
- Structural: in the BUG-FUNC-018 affected-surface sample (`:192`), not in the signed ten.

### NEGBINOM.DIST
- **No count.** One witness only: "`NEGBINOM.DIST`: the small finite cumulative witness now
  matches Excel bits" (`BUG-FUNC-021…md:83`).
- The W109 NEGBINOM figures do **not** name which surface:
  "NEGBINOM measured NOT to inherit (10.4%) - separate route" (`…CATALOG.md:113`);
  "(iii) NEGBINOM does NOT inherit (10.4% raced) — own route hunt"
  (`…W109_CAMPAIGN_RESUME…:87`); "NEGBINOM: Loader dnbinom refuted (8.4%)" (`…GRATIO…:1053`);
  corpus "b29 (BINOM 1,062 + NEGBINOM 1,800, banked)" (`…GRATIO…:1009`).
  **Ambiguous in source** whether NEGBINOM.DIST or NEGBINOMDIST (or both) was probed. Also
  note 10.4% and 8.4% are *candidate-model* scores, not production pass rates.

### POISSON.DIST
- **4,000/4,000 held-out on production, k=0 window only** (`…GRATIO…:935-937`, §1.2).
- 30,000: identification/instrument corpus for the internal exp; 29,997/30,000 idealized
  model, 30,000/30,000 real hardware chain (`…GRATIO…:782`, `:820-821`, `:919`).
- 34,000 = 30,000 + 4,000, and is the **internal exp** row's evidence grade
  (`…W109_CAMPAIGN_RESUME…:18`), scoped "POISSON k=0" at `:35-36`.
- k≥1 OPEN, materially worse (25.7% at k=1 direct product; small-λ masks 70/43/41%).
- Build not restated near any of these figures.

### POISSON (compatibility alias)
- **No figure.** Rounded witness `"W62-DISC-001","=ROUND(POISSON(3,2.5,TRUE),9)",…,"True"`
  (w62 csv:11). In the BUG-FUNC-018 affected-surface sample (`:193`). The sign-off sentence
  names `POISSON.DIST(0, lambda)`, not POISSON.

### WEIBULL.DIST
- **5,999/6,000 = 99.983% held-out, fresh 6,000 rows, production kernel** (`…GRATIO…:990-992`).
- Identification (fitted): b27 5,400-row pdf corpus, T3|SS 1,600/1,600, b27b D1 48/48, D2 2/2
  (`…GRATIO…:974-988`).
- Build not restated near the figures.

### WEIBULL (compatibility alias)
- **No figure.** Rounded witness `WEIBULL(2,3,4,TRUE) -> 0.117503097415405`
  (`W24_BATCH06_SPECIAL_DIST_EXECUTION_RECORD.md:44`, 2026-03-18). Any 99.983% claim on
  WEIBULL is sibling inheritance, unmeasured.

### EXPON.DIST
- **4,000/4,000 = 100.000% held-out, fresh, production** (`…GRATIO…:997`).
- Identification (fitted): b28b 14/14 + 24/24 + twins 40/40 (`…GRATIO…:994-996`).
- Conflicting adjacent numbers: 99.978% (expm1 primitive), 99.96% (PMT-lane aside),
  232/234 (instrument read). See §1.3.
- Build not restated near the figures.

### EXPONDIST (compatibility alias)
- **No figure.** Alias per contract
  (`FUNCTION_SLICE_STATISTICAL_DISTRIBUTIONS_AND_COMPAT_A_CONTRACT_PRELIM.md:51`); in the
  BUG-FUNC-018 affected-surface sample (`:187`). Any 100.000% claim on EXPONDIST is
  unmeasured inheritance.

---

## 3. STEP 2 — comparison against FOUNDATION.md §2.5 / §3.2 / §3.4 / §3.6

Read after §1–§2 above were written. Also opened, to characterise each disagreement precisely:
`dossier/C6-verification-record.json` records 9, 11, 12, 22, 23, 24, 34, 38;
`dossier/C3-excel-math-deviations.json` (XMD-011); `dossier/C4-bug-streams.json`
(BUG-FUNC-013, BUG-FUNC-033).

### 3.0 Where the foundation is right (recorded so the disagreement list is readable)

- **PHI is not on an erf substrate.** FOUNDATION:72 (A2-S1) already applied this as a data
  correction, citing catalog:119 and the non-erf chain. Matches my §1.4 exactly.
- **The 34,000 is not a POISSON.DIST surface pass rate.** FOUNDATION:73 (A2-S2) splits it into
  b23's 30,000 identification corpus and b26P's fresh 4,000 gate, and records the source's own
  phrase "consecutive fresh rows" as inaccurate as to b23. Matches my §1.2.
- **WEIBULL and EXPONDIST are `alias-sibling-inherited`** (§2.5 records 11, 12). Matches my §1.3:
  neither legacy surface is separately counted anywhere.
- **WEIBULL.DIST N6 5999/6000, W5 held-out**; **EXPON.DIST N5 4000/4000, W5 held-out**. Correct.
- **ERF.PRECISE / ERFC.PRECISE → `model-or-candidate-score`** (§2.5 record 24, the 663/1218
  check_erf190 model). Correct; and W3's `figure-is-a-model-or-candidate-score` is the right
  `why_no_count`.
- **GAUSS N1 + W3 (no count)**, and `named-but-not-measured` for record 24. Correct.
- **BINOM.DIST W3 with only-percentages-published.** Correct — b36 publishes 49.59%, not rows.
- **NORMSDIST / NORMSINV at N1** rather than clean. Correct (KED-STAT-001 members with measured
  ULP drift).

### 3.1 The load-bearing disagreement: record 34's 8/10

FOUNDATION:599 —

> "**NORMDIST is the worked example.** It renders `N6` (8 of 10 counted numeric rows, C6 record 34,
> builds 19929 **and** 20131 — `build_ambiguity: two-builds-named`) **and** `S5` (10 of 10 counted
> structural rows, C6 record 38, build 20026)."

and FOUNDATION:584 —

> "- **N6 (6)** — LOGNORM.DIST, LOGNORMDIST, NORM.S.DIST, NORMDIST, NORMINV, WEIBULL.DIST."

The 8/10 belongs to the BUG-FUNC-013 row of the W097 R-G re-sweep. Its ten witnesses are
NORM.DIST, NORM.INV, NORMSDIST, NORMSINV, NORM.S.DIST, NORM.S.INV, ERF(1), ERFC(1) (all eight
matched) plus GAUSS(1) at 2 ULP and PHI(0) at 1 ULP
(`smart-fuzzer/planning/W097-R-GH-closed-streams-cell-ref-resweep.md:60-76`;
`docs/bugs/streams/BUG-FUNC-013…md:134-156`).

Therefore:

- **NORMDIST, NORMINV, LOGNORM.DIST, LOGNORMDIST are not in the measured set.** Four of the six
  N6 entries carry a shortfall from a corpus that never touched them.
- **NORM.S.DIST is in the set and it matched.** Assigning it "a measured shortfall of 2" moves
  the shortfall onto a surface whose own row is a pass. The two misses are GAUSS and PHI, and
  record 34 does not list either.
- **NORM.DIST, NORM.INV and NORM.S.INV — the record's own direct closure witnesses — do not
  appear anywhere in FOUNDATION.md** (word-boundary check: 0 occurrences each). The three
  surfaces that were measured are absent; four that were not are labelled with the result.
- The harvest record's own prose contradicts the label it feeds:
  > "no per-surface OxFunc-vs-Excel pass rate is published for any of them"
  > — `dossier/C6-verification-record.json` record 34, `counts.what_the_rows_are`
  Yet FOUNDATION renders it as NORMDIST's per-surface numeric state, and A1-F5 (FOUNDATION:47)
  institutionalises it:
  > "the state is per-axis, so NORMDIST now renders `S5` (10/10 structural, record 38) **and**
  > `N6` (8/10 numeric, record 34) simultaneously"
  A1-F5 fixed a *neighbouring* misattribution in the same breath ("`KED-STAT-001` names NORMSDIST
  and NORMSINV, not NORMDIST") while cementing the deeper one.
- **Build.** The 8/10 is measured on build **19929** only (`…W097-R-GH…md:31`). The 20131 material
  in record 34 is the NORM.S.DIST / LOGNORM oracle-cache instrumentation shard, which the record
  itself says carries no pass rate. `two-builds-named` is therefore the wrong code for the
  *scored* line; the scored line has a single, unambiguous build.
- What NORMDIST's own evidence actually is: **1 structural row, 1/1, build 20026**
  (`BUG-FUNC-018…md:19`) and a ROUND(…,9) witness on build 16.0.19822.20114
  (`.tmp/w62-…-results.csv:2`). No bit-exact numeric datum exists for NORMDIST at all.

### 3.2 Second disagreement: ERF and the four ERF/ERFC surfaces on the structural axis

FOUNDATION §3.4 lists no `ERF` in N1–N7 → ERF is N8, "no numeric comparison record in the six
sources listed under Sources"; and none of ERF, ERF.PRECISE, ERFC, ERFC.PRECISE appears in
S5/S6/S7 → all four are S8, "no structural comparison record in the six sources".

Both statements are false against the primaries:
- `=ERF(1)` → `0x3feaf767a741088b` on both sides, live Excel, build 19929
  (`BUG-FUNC-013…md:142`; `…W097-R-GH…md:68`). The harvest carries it (C4 BUG-FUNC-013's
  `verification` names ERF(1) and ERFC(1) among the eight bit-for-bit rows).
- BUG-FUNC-033 is a live-Excel COM structural differential on **build 20026** covering exactly
  these four surfaces: "all four ERF/ERFC `arg0_logical` probes now `error:Value` (match Excel)"
  (`docs/bugs/streams/BUG-FUNC-033…md:47-49`; build at `:16`). The harvest carries it in
  C4-bug-streams; it never reached C6, so no `S*` state was assigned.

### 3.3 Third disagreement: ERFC's N4

FOUNDATION §3.4 N4 = "matches Excel by reproducing a documented Excel departure; a witness
exists, no count", and ERFC is in N4 (13). The source publishes a count and it is a shortfall:

> "| **correction-fit kernel (this commit)** | **20** | cross-platform, 0 regressions |"
> — `docs/function-lane/ERFC_EXCEL_EMULATION.md:44` (heading `:38` = "widened 48-point positive
> witness set"), with 28 points still blocked (`:55-63`) and worst residual 6 ULP (`:66`)

and the math-deviation catalogue says so in the same words:
"**Status: PARTIALLY reproduced** (~20/48 positive witnesses bit-exact; ~28 still open…)"
(`docs/EXCEL_MATH_DEVIATION_CATALOG.md:249-250`). C3's XMD-011 harvest also records "PARTIAL".
So "matches Excel" is an overclaim, and "no count" is inaccurate — the count exists, is a
20-of-48 shortfall, and was **fitted to those very 48 points** with matched anchors weighted 1e12
(`ERFC_EXCEL_EMULATION.md:29-31`). The honest state is a measured shortfall on a repair-target
corpus, with the additional flag that the whole correction-polynomial design is superseded by
W109's "NO coefficient tables exist" finding (`…CATALOG.md:127`).

### 3.4 Fourth disagreement: POISSON.DIST's held-out gate is dropped entirely

FOUNDATION:73 — "Neither figure is a POISSON.DIST surface pass rate … POISSON.DIST stays at
W3/N1." The correction over-corrects. The source says:

> "- **b26P POISSON: 4,000/4,000 through the production RN-chain path**"
> — `…GRATIO…:935`; and "POISSON k=0 window 4,000/4,000" (`…GRATIO…:1082`)

That is a production-kernel, fresh, held-out pass rate **on POISSON.DIST(0, λ)**. What it is not
is a whole-surface rate and it is not route-discriminating. The correct record is a per-surface
count with an explicit input-window scope, `held_out: true`, `held_out_rows: 4000`; dropping it
also understates §3.6's "entries with ≥1 held-out counted record = **19**" by at least one.

### 3.5 Fifth disagreement: NEGBINOMDIST's per-surface count, and which NEGBINOM was raced

- NEGBINOMDIST sits at W3 (`no row count was extracted`) and is on FOUNDATION:497's list of the
  42 dropped by the attribution gate, via §2.5 record 22's `named-but-not-measured`. But a
  per-surface count exists in a primary source: `| `NEGBINOMDIST` | `138` | `7` | `138` | `1` |
  `20` | `237` |` (`BUG-FUNC-021…md:146`), i.e. 7 of 138 on the W097 R-D explorer, build 19929
  (`W097-R-D…md:46`). The *same table* is what puts NORMSDIST and NORMSINV at W4, so the
  treatment is internally inconsistent.
- The W109 "NEGBINOM measured NOT to inherit (10.4%)" figure (`…CATALOG.md:113`) names no
  surface. C6 record 23 assigns it to **NEGBINOM.DIST**, while its own oracle citation is
  `smart-fuzzer/cache/oracle/build-20131/NEGBINOMDIST.jsonl` (1,804 rows) and the source's corpus
  line is "b29 (BINOM 1,062 + NEGBINOM 1,800, banked)" (`…GRATIO…:1009`). The evidence points at
  the **legacy** surface. This is ambiguous in source and should be recorded as such, not resolved.

### 3.6 Build-restatement disagreements (a class, not a one-off)

C6 records 9, 11, 12, 23 all assert a flat `16.0 build 20131` for PHI 764/764, WEIBULL.DIST
5999/6000, EXPON.DIST 4000/4000 and the BINOM/POISSON/NEGBINOM lane. **None of those figures has
the build restated on or near its line in OxFunc.** The single build statement in the GRATIO note
is scoped to one battery: "## The multi-view collapse (battery B1, 829 probes, build 20131)"
(`…GRATIO…:15`). FOUNDATION's own oracle enum has the right value for this
(`single-build-not-restated-on-the-scored-line`) and §3.6 counts only **20** entries with a
non-single build; these five should carry the not-restated code.

### 3.7 Minor / cosmetic

- **PHI, `corpus_was_repair_target`.** The ruled-out ledger shows PHI's rival stagings were killed
  on the same live rounds that produced the 764: "divide staging killed on r1; strict stagings
  killed on 39 constructed window rows"
  (`docs/function-lane/DISCREPANCY_RULED_OUT_LEDGER.csv:22`). Combined with the record's own
  concession that "answered rows" is "not a pre-registered held-out split", PHI belongs in
  §3.6's "corpus was the target of the repair it scores" set (**18**), and N5's "matched Excel on
  every one of 764 counted rows" should not read as a clean-corpus claim.
- **The 30,000's two scores.** FOUNDATION:73 records the identification corpus as 30,000/30,000.
  The source publishes both 29,997/30,000 (idealised model) and 30,000/30,000 (real hardware
  chain) — `…GRATIO…:782` and `:820-821`. Quoting only the hardware number silently picks the
  more favourable of two figures on the same corpus.
- **EXPON.DIST among the 12 "numeric-clean".** The source elsewhere publishes 99.96% for
  EXPON.DIST (`docs/function-lane/W109_G6_PMT_RESUME_20260723.md:107`) and 99.978% for the shared
  expm1 primitive it depends on (`…W109_CAMPAIGN_RESUME…:19`). §7.3 already warns about the 12;
  this specific numeric conflict is not among the warnings and should be.
- **BINOMDIST numeric state.** N8 ("no numeric comparison record"), but BUG-FUNC-021 asserts a
  live-Excel numeric result without a count: "`BINOM.DIST` / `BINOMDIST` retain their prior
  log/exp path because the W090 replay showed those rows were already exact there"
  (`BUG-FUNC-021…md:81-83`). N7 is the honest state.
- **POISSON (legacy) and WEIBULL/EXPONDIST rounded-witness records.** The only evidence for
  POISSON, and the only evidence for legacy WEIBULL, is a `ROUND(…,9)` comparison
  (`.tmp/w62-…-results.csv:11`; `W24_BATCH06…md:44`) — neither of FOUNDATION's two
  `comparison_predicate` values covers a 9-decimal rounding, so these should be recorded as
  no-comparison rather than folded into either predicate.

### 3.8 Verdict

The foundation handles my slice's two most-publicised traps correctly (POISSON's 34,000, PHI's
substrate) — both were caught by its own audit rows A2-S1/A2-S2. The failure is concentrated in
one record: **C6 record 34's 8/10 has been distributed across a seven-surface list that shares only
three members with the ten rows actually measured**, and FOUNDATION §3.4 promotes that
distribution to its worked example. Four surfaces (NORMDIST, NORMINV, LOGNORM.DIST, LOGNORMDIST)
are labelled with a shortfall they were never measured for; one (NORM.S.DIST) is labelled with a
shortfall it passed; three (NORM.DIST, NORM.INV, NORM.S.INV) that were measured are missing from
the document. A secondary cluster underclaims: ERF's live witness, four surfaces' structural
differential, ERFC's 20/48, POISSON.DIST's held-out 4,000, and NEGBINOMDIST's 7/138.

