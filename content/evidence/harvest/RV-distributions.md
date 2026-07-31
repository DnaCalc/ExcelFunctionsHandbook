# RV-distributions — independent re-derivation of measurement attributions

Slice: **distributions** (G3-01 / G3-05 / G3-06 cluster).
Method: STEP 1 derived from OxFunc primary sources ONLY, before opening
FOUNDATION.md or any harvest record. STEP 2 compares against FOUNDATION.md §2.5 /
§3.2 / §3.4 / §3.6.

Primary sources read (all under `C:/Work/DnaCalc/OxFunc/` — read-only):

- `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md` (G3-01 = **one single line, 113**;
  G3-05 = 117, G3-06 = 118, G3-07 = 119; header/reconcile block lines 1-99)
- `docs/function-lane/W109_G3-01_GRATIO_IDENTIFICATION_20260716.md` (1,495 lines)
- `docs/function-lane/W109_CAMPAIGN_RESUME_20260718.md` (128 lines)
- `docs/function-lane/W109_WALL_CLUES_LEDGER.md` (215 lines)
- `docs/function-lane/W109_CHISQ_FTEST_DECOMPOSITION_20260712.md` (46 lines)
- `docs/bugs/streams/BUG-FUNC-021_w090_statistical_numeric_exactness_drift.md`
- `docs/KNOWN_EXACTNESS_DEVIATIONS.md` (KED-STAT-001, lines 92-131)

---

## 0. Structural facts that govern every attribution in this slice

**(a) The G3-01 row header names TEN surfaces + two unprobed ones.**

> `| G3-01 — BETAINV, CHIDIST, CHIINV, FDIST, FINV, GAMMAINV, HYPGEOMDIST,
> NEGBINOMDIST, TDIST, TINV (+ CONFIDENCE.T, Z.TEST unprobed) | Distribution
> scalar numeric drift, `1`-`28` ULP.`
> — `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:113`

Note what is **absent** from the row header but present in the row body and in my
slice: `GAMMA.DIST`, `BETA.DIST`, `F.DIST.RT`, `T.DIST.RT`, `CHISQ.DIST.RT`,
`GAMMADIST`, `BETADIST`, `CHISQ.INV`, `GAMMA.INV`. Most of the row's *figures*
are published against the modern names (`GAMMA.DIST`, `BETA.DIST`) that the row
header does not list. Membership by header ≠ membership by figure.

**(b) 12 surfaces were deliberately collapsed into ONE measurement.** This is the
single most important attribution fact in the slice — most W109 figures are
measurements of *one internal kernel* taken *through* whichever surface was the
cheapest window, not per-surface pass rates:

> `## The multi-view collapse (battery B1, 829 probes, build 20131)`
> `- Legacy ≡ modern bit-for-bit everywhere probed: CHIDIST≡CHISQ.DIST.RT,`
> `  FDIST≡F.DIST.RT, TDIST(·,·,1)≡T.DIST.RT, GAMMADIST≡GAMMA.DIST, BETADIST≡BETA.DIST.`
> — `W109_G3-01_GRATIO_IDENTIFICATION_20260716.md:15-18`

and

> `- **Multi-view exact-transform probing** (CHIDIST(2x,2a) vs GAMMA.DIST(x,a,1) vs`
> `  GAMMA.DIST(2x,a,2)) collapses 12 surfaces to one kernel measurement and isolates`
> — `W109_G3-01_GRATIO_IDENTIFICATION_20260716.md:376-377`

The legacy≡modern statements are **Excel-vs-Excel identity checks** ("everywhere
probed", 829 probes). They license transferring a figure from `CHIDIST` to
`CHISQ.DIST.RT`, but the figure was still *measured on* `CHIDIST`.

**(c) Build.** `20131` is stated exactly once inside the identification note, in
the B1 battery heading (`:15`), and in the catalog's reconcile/recon header
(`:13`, `:89` — "live Excel 16.0 build 20131 result bits", which is the 48-case
2026-07-10 recon corpus, not the W109 batteries). It is **not restated near any
of the b14/b19/b20/b21/b22/b26/b28/b30/b31 figures**. For every W109 per-surface
figure below the honest answer is *"not restated near the figure"* — the build is
inherited from the document header at best.

**(d) Corpus name ≠ surface name.** `b22` and `b26` are batteries, and the row
publishes battery-level totals in the same breath as surface-level figures:

> `production ≡ identified substrate bit-for-bit — GAMMA.DIST 337/446, b26
> 1,615/4,100 (worst −10), b22 293/671.`
> — `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:113` (Lane-3 sentence)

`b22 293/671` is a **beta-substrate battery total spanning BETA.DIST + FDIST +
TDIST**; it is not a figure for any one surface (proof below). `b26 1,615/4,100`
IS resolvable to GAMMA.DIST integer-a blocks (proof below) — the catalog dropped
the qualifier that makes it resolvable.

---

## 1. GAMMA-SIDE SURFACES

### 1.1 CHIDIST — current production figure **152/195**

Superseded chain, all three figures inside the same row:

1. `12/195` (pre-campaign baseline)
2. `144/195` after the GRATIO port
3. **`152/195`** after the chopped-exp landing — still current at lane-3.

> `**GRATIO KERNEL PORTED+LANDED** (c71cde5/fa275e0): CHIDIST 12→144/195 exact
> (catastrophics eliminated), GAMMA.DIST 64→137/268 max 21 ULP, 1507 tests green.`
> — `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:113`

> `Landed as `exp_rd` (double-double, validated 0/25k vs floor-exp) → **CHIDIST
> 152/195, GAMMA.DIST 159/268** (b20 held-out gate: +3/111, fresh a-slices).`
> — `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:113`

Corroborated per-surface in the identification note:

> `Corpus: CHIDIST 148 -> **152**/195, GAMMA.DIST 151 -> **159**/268 (from`
> `12/195 and 64/268 pre-campaign). Full suite green (1604).`
> — `W109_G3-01_GRATIO_IDENTIFICATION_20260716.md:497-498`

and re-verified as *production routing* at lane-3:

> `**Post-lane-3 standing numbers** (production routing, bit-verified equal to`
> `the identified substrate paths): CHIDIST 152/195; GAMMA.DIST modern corpus`
> `293→**337/446**; b26 integer-a 331→**1,615/4,100** (worst −10); b22`
> `**293/671** (integer rows now scored; BETA.DIST 136/288); b26 POISSON`
> `4,000/4,000; WEIBULL b28 5,999/6,000; EXPON b28c 4,000/4,000.`
> — `W109_G3-01_GRATIO_IDENTIFICATION_20260716.md:1101-1105`

**Verdict.** `152/195` = production OxFunc pass rate, measured on the CHIDIST
surface, per-surface, 195-row corpus. Corpus is the campaign's own working
CHIDIST corpus — **fitted**, not held out (the row's held-out gate for this
landing is b20 `+3/111`, a *different* corpus). Note `144/195` and `12/195` are
STALE. Build not restated near the figure.

Also on CHIDIST, three non-pass-rate figures that must not be confused with it:

- Excel-vs-Excel identity: `CHIDIST is NOT RN53(1 − published P) (16 eq / 17 ne)`
  (`:21`) — a publication-structure probe, not a pass rate.
- Cross-view wiring identity: `CHIDIST(k²/512, 1) ≡ ERFC.PRECISE(k/32)` —
  `**160/160 AND 160/160 bit-exact**` (`:69-70`) — Excel-vs-Excel.
- A refuted historical claim: `The catastrophic 6224-ULP CHIDIST row is OxFunc's
  plain-double NR complement, not an Excel-side extended CF` (`:60-62`).

### 1.2 CHISQ.DIST.RT — **no own figure**; inherits CHIDIST by proven identity

Only statement: `Legacy ≡ modern bit-for-bit everywhere probed:
CHIDIST≡CHISQ.DIST.RT` (`:17`). No count is published against the
`CHISQ.DIST.RT` surface itself. Attribution = sibling-inherited via an
Excel-vs-Excel identity check on 829 B1 probes.

### 1.3 GAMMA.DIST (cdf) — current production figure **337/446**

Superseded chain: `64/268` → `137/268` → `159/268` → (corpus expanded)
`293/446` → **`337/446`**.

> `Lane-3 (2026-07-18): BOTH OxFunc-side integer-shape fast paths REMOVED (gamma
> one catastrophic ±4,400 ULP, silently overriding the landed GRATIO; b30 proves
> Excel has NO integer beta path; A/B-bounds staging broadly confirmed);
> production ≡ identified substrate bit-for-bit — GAMMA.DIST 337/446, b26
> 1,615/4,100 (worst −10), b22 293/671.`
> — `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:113`

Note the **denominator change** 268 → 446: `159/268` is not merely improved to
`337/446`, the corpus grew ("GAMMA.DIST modern corpus 293→337/446",
identification `:1102-1103`). `159/268` and `137/268` and `64/268` are STALE.

**Verdict.** `337/446` = production OxFunc pass rate on the GAMMA.DIST surface,
per-surface, fitted working corpus (not labelled fresh/held-out), build not
restated.

### 1.4 GAMMA.DIST integer-a held-out gate — **b26 1,615/4,100 IS GAMMA.DIST**

The catalog prints `b26 1,615/4,100` with no surface. It is resolvable, and it is
GAMMA.DIST:

> `- b26A GAMMA.DIST (production path, integer-a moderate-y uniform grids):`
> `  a=2 795/1600 (worst 4), a=3 544/1600 (worst 7), a=4 276/900 (worst 10).`
> — `W109_G3-01_GRATIO_IDENTIFICATION_20260716.md:938-939`

795+544+276 = **1,615**; 1600+1600+900 = **4,100**. Exact reconciliation, so
`b26 1,615/4,100` = the b26A GAMMA.DIST integer-a blocks, and it is **held out**:

> `## b26 held-out gate (2026-07-18) — POISSON signed off; series staging
> held-out-confirmed with known ceiling`
> — `W109_G3-01_GRATIO_IDENTIFICATION_20260716.md:933`

> `- b26 batteries designed for the clean held-out gate (captured; scoring
>   next): b26A integer-a moderate-y series gate, b26X cross-view, b26P
>   POISSON re-confirm.` — `:929-931`

Caution: `b26` also contains `b26P POISSON: 4,000/4,000` (`:935`) and a reserved
unraced `b26X` (`W109_CAMPAIGN_RESUME_20260718.md:128`). So "b26" as a bare label
is a multi-surface battery; only b26A is GAMMA.DIST. The catalog's unqualified
`b26 1,615/4,100` is *recoverable* but under-labelled; the resume does label it:
`b26 integer-a 1,615/4,100 (worst −10)`
(`W109_CAMPAIGN_RESUME_20260718.md:32-33`).

### 1.5 GAMMA.DIST **pdf** — separate, much worse, production figure **16.1%**

This is a distinct surface-mode with its own measurement and it is easy to
mis-merge with the cdf figures:

> `- **GAMMA.DIST pdf MEASURED (b31, 4,750 rows banked) — new named wall:`
> `  the closed-form-pdf extended-composition body class.** Triage REFUTED at`
> `  the exact-bit level: production log-composed (16.1%), direct separate-pows`
> `  (18%), ratio forms (22%), and R's dgamma-via-dpois structure (20% — NOT`
> `  R's dgamma, unlike POISSON which IS Loader at k≥2).`
> — `W109_WALL_CLUES_LEDGER.md:201-205`

`16.1%` is the **production** score (log-composed pdf is what production runs);
`18% / 22% / 20%` are **research-model candidate** scores; `42.3%` (`:207`) is a
research-model score on the a=1 sub-slice. The catalog compresses all of this to
`Lane-3b (b31): GAMMA.DIST pdf ≠ log-composed ≠ R dgamma` — **no figure at all in
the catalog row**. Also note the row banked, one day earlier, `Clue banked:
GAMMA.DIST **pdf** (cumulative=FALSE) remains unmeasured` (identification
`:1107`) — that clue is now STALE, superseded by b31.

### 1.6 GAMMADIST — **no own figure**; inherits GAMMA.DIST by proven identity

`GAMMADIST≡GAMMA.DIST` (`:18`). The header of G3-01 lists neither `GAMMADIST` nor
`GAMMA.DIST`… it lists `GAMMAINV`. The only per-surface `GAMMADIST` count in the
whole corpus of primary sources is the pre-W109 W097 histogram (§4 below):
`36/89`, median 7 ULP.

### 1.7 GAMMA.INV — current figure **18/60** (b14, fitted), held-out b19 hint only

> `  b14 effect: GAMMA.INV 8->18/60 exact, worst +880,380 -> -16 ULP;`
> `  BETAINV 2->4/30, worst +1,910,580 -> +13 ULP (residual = the pre-BRATIO`
> `  beta forward, a separate lane); publication-rule race: hi 18/60 vs`
> `  closest 17/60 vs lo 7/60 (gamma side) — hi retained.`
> — `W109_G3-01_GRATIO_IDENTIFICATION_20260716.md:433-436`

b14 is explicitly the corpus the inverter was *validated on*, i.e. the fitted
target, with b19 as the separate held-out capture:

> `Landed in production (validated on the b14 corpora, held-out b19 captured
> separately)` — `:427-428`

b19's GAMMA.INV content is only a sub-promotion-bar hint, with no pass rate:

> `GAMMA.INV z-space vs x-space:`
> `+4 rows on 48 discriminators (beta=3) for z-space — a hint, below promotion`
> `bar; x-space retained.` — `:514-516`

**Verdict.** GAMMA.INV: `18/60`, production, per-surface, **fitted corpus (b14)**.
No post-BRATIO / post-chain-exp re-score of GAMMA.INV exists — a real gap, since
the forward it roots was replaced twice afterwards. `8/60` is the stale
pre-lattice figure. Build not restated.

### 1.8 CHIINV — current figure **held-out b19 15/40** (and fitted b14 16/60)

> `- `chisq_inv_rt_kernel` (chi_f_t_family.rs): CHIINV now inverts the PUBLISHED`
> `  right-tail surface Q directly (negated-forward convention) instead of P at`
> `  1-p. The 1-p staging carries a systematic -5..-33 ULP bias (rounding loss in`
> `  1-p); Q-direct: 10->16/60 exact, residuals collapse to +-1..5 (worst -91 at`
> `  one deep-tail row).` — `:437-441`

> `b19 (fresh rows, never raced): CHIINV Q-direct CONFIRMED held-out (15/40 vs`
> `6/40 for P at 1-p, same systematic negative bias on fresh rows).` — `:509-510`

The catalog carries only the held-out pair:
`CHIINV roots Q directly, held-out-confirmed b19 15/40 vs 6/40`
(`docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:113`).

**Verdict.** Two co-current CHIINV figures on two corpora: `16/60` on b14
(fitted) and `15/40` on b19 (**held out** — exact words "fresh rows, never
raced"). `15/40 vs 6/40` is an A/B *staging* comparison, both arms production
candidates. `10/60` and `6/40` are the losing/stale arms. Build not restated.

### 1.9 CHISQ.INV — **no figure; and the one mention is ambiguous**

Single mention in the entire W109 corpus:

> `BETAINV/CHISQ.INV inverter-limited no longer; forward error dominates.`
> — `W109_G3-01_GRATIO_IDENTIFICATION_20260716.md:516-517`

Ambiguity, stated as a finding: the kernel actually described in this section is
`chisq_inv_rt_kernel` (`:437`), i.e. the **right-tail** inverse
(`CHIINV`/`CHISQ.INV.RT`), whereas `CHISQ.INV` is the *left-tail* modern
function. The source does not say which surface it means, and no count is
attached either way. `CHISQ.INV` is **not** in the G3-01 header list. Verdict:
no figure exists for CHISQ.INV; the qualitative claim is ambiguous in source.

---

## 2. BETA-SIDE SURFACES

### 2.1 BETA.DIST — current production figure **136/288** (inside b22); b21 tail 4/127

> `b22` `**293/671**` `(integer rows now scored; BETA.DIST 136/288)`
> — `W109_G3-01_GRATIO_IDENTIFICATION_20260716.md:1103-1104`

This is the sentence that proves `b22` is a battery total and not a BETA.DIST
figure: BETA.DIST is 288 of b22's 671 rows. The remaining ~383 rows are the
F- and T-surface blocks (see 2.4/2.6). The resume calls the total "beta b22",
i.e. beta-*substrate*, not the BETA.DIST *surface*:

> `beta b22 293/671` — `W109_CAMPAIGN_RESUME_20260718.md:33`

Deep-tail BETA.DIST corpus b21, a discriminator battery of 127 live BETA.DIST rows:

> `- b21 (127 BETA.DIST rows): beta-tail discriminator battery (agent-H spec:`
> `  GRATIO-substitution vs Boost small-b-large-a series vs CR) — scoring in`
> `  flight.` — `:525-527`

> `- b21 deep-tail corpus: old **0/127, worst 8,848 ULP** -> new 4/127, worst`
> `  56. The catastrophic tail class is eliminated.` — `:593-594`

And on the same b21 rows, an **Excel-vs-mathematical-truth** measurement that is
NOT a pass rate and is adjacent to the pass rate in the catalog:

> `- **FAMILY PROVEN = DiDonato-Morris TOMS-708 Eq-9 bgrat expansion.** Decisive`
> `  signature: at k=2, a=118/200, Excel sits +41..+63 ULP from the TRUE value`
> `  yet within +-7 ULP of every Eq-9-family realization across 25 rows.`
> — `:535-538`

Plus a research-model refutation ledger on the same corpus (`NSWC-double grat1
(4/127, max 56), GRATIO-sub ... (8-10/127, max 37), Boost 1.35-1.42 (6/127 ...)`,
`:542-545`) — note `4/127` appears **twice with different meanings**: as the
production post-port score (`:593`) and as the NSWC-double research-model score
(`:542`). That collision is a real trap in this row.

**Verdict.** BETA.DIST: `136/288` production, per-surface, on the held-out b22
battery; `4/127` production on the b21 deep-tail battery (fitted/discriminator,
described as a designed discriminator battery, not held out). Build not restated.

### 2.2 BETADIST — **no own figure**; `BETADIST≡BETA.DIST` (`:18`). Pre-W109 W097
histogram only: `13/28`.

### 2.3 BETAINV — current figure **12/30 worst +5** — and the catalog row is STALE

The catalog G3-01 row publishes only:

> `b14: GAMMA.INV 8→18/60 worst −16, BETAINV 2→4/30 worst +13`
> — `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:113`

but the identification note and the resume both carry a later value:

> `- Gates: suite 1,606 green; gratio corpus stable (152/195, 159/268); b22`
> `  stable (285/655); **b14 BETAINV 4 -> 12/30, worst +13 -> +5** (chain exp`
> `  through the beta forward).`
> — `W109_G3-01_GRATIO_IDENTIFICATION_20260716.md:828-830`

> `BETAINV 12/30 worst +5` — `W109_CAMPAIGN_RESUME_20260718.md:35`

**Verdict.** Current BETAINV = `12/30` worst +5, production, per-surface, on the
**fitted** b14 corpus, after the F2XM1-chain-exp landing (commit 223cfa5). The
catalog's `4/30 worst +13` is STALE, and `2/30` doubly so. Prompt's "BETAINV
12/30" is therefore the *correct current* figure and the catalog row is the wrong
place to read it from. Build not restated near the figure.

Also note in the same `:828-829` sentence the *stale-in-place* pair
`gratio corpus stable (152/195, 159/268)` and `b22 stable (285/655)`: as of
2026-07-18 lane-3 those became `337/446` and `293/671`.

### 2.4 FDIST — no current count; only worst-ULP movement and research-model scores

> `Held-out gate b22 (671 fresh live rows, disjoint values): old NR kernel`
> `167/655 exact (worst +-145) -> BRATIO **285/655** (worst 126, one`
> `bgrat-wall row; FDIST worst +37 -> -15, TDIST worst +-88 -> +17).`
> — `W109_G3-01_GRATIO_IDENTIFICATION_20260716.md:587-589`

FDIST gets a **worst-ULP** delta only, never a count, in the b22 gate. Its
research-model figures:

> `Decisive branch-differential: **bpser in plain double`
> `BEATS correctly-rounded betainc on FDIST/TDIST** (11/6, 5/3, 8/4) — literal code`
> `identity; F.DIST.RT bpser 5/5.` — `:199-201`

`(11/6, 5/3, 8/4)` are model-vs-model win counts (bpser-double vs CR) on
FDIST/TDIST rows. Which pair belongs to which surface is **ambiguous in source**
(three pairs, two named surfaces). These are research-model (Python emulator
`agentA_bratio.py`) measurements, not production.

**Verdict.** FDIST: no production pass-rate count on the current build exists;
only `worst +37 → −15` on held-out b22, plus ambiguous research-model win counts.
Pre-W109 W097 histogram: `8/43`.

### 2.5 F.DIST.RT — one own figure, research-model: **5/5**

`F.DIST.RT bpser 5/5` (`:201`) — a research-model branch-differential on 5 rows,
NOT a production pass rate. Otherwise inherits FDIST via `FDIST≡F.DIST.RT`
(`:18`). Also named as a landing site: `FDIST/F.DIST x=d2/den y=d1F/den`
(`:597`).

### 2.6 TDIST — current figure **14/60** (inside held-out b22)

> `TDIST/T.DIST.RT/2T + t_cdf + TTEST x=df/den y=t^2/den; all F/T inverter`
> `closures root the same staged forwards. b22 effect: TDIST 6->14/60 exact.`
> — `W109_G3-01_GRATIO_IDENTIFICATION_20260716.md:598-599`

**Verdict.** `14/60`, production, per-surface, on the **held-out** b22 battery
(`671 fresh live rows, disjoint values`, `:587`). `6/60` is the stale pre-staging
arm. Not in the catalog row at all — the catalog gives only
`TDIST worst ±88 → +17`. Build not restated. Also: `one-tail=0.5·two-tail
bit-exact` (catalog `:113`) and `x=df/den, y=t²/den, one-tail = 0.5·two-tail
bit-exact` (`:198-199`) are Excel-vs-Excel identity checks, not pass rates.

### 2.7 T.DIST.RT — **no own figure**; `TDIST(·,·,1)≡T.DIST.RT` (`:18`), staging
landed at `:598`. Sibling-inherited from TDIST.

### 2.8 FINV — current figure **held-out b19 3/32**

> `invert-the-published-surface staging decisively improves FINV (0/32 -> 3/32,`
> `small-p bias -60 -> +2) and TINV (residuals -4..-238 -> mostly +-1..7):`
> `LANDED for f_inv_rt_kernel (roots f_dist_rt's accurate complement form) and`
> `t_inv_2t_kernel (roots t_dist_2t's surface).` — `:511-514`

Section heading establishes hold-out: `## *INV published-surface principle
extended (b19 held-out)` (`:507`) and `b19 (fresh rows, never raced)` (`:509`).

**Verdict.** FINV `3/32`, production, per-surface, **held out** (b19). `0/32` is
the stale pre-staging arm. Build not restated. Pre-W109 W097: `1/80`.

### 2.9 TINV — **no count at all; residual band only**

`TINV (residuals -4..-238 -> mostly +-1..7)` (`:512`) and catalog
`TINV roots the two-tail surface, residuals −238→±7`. No numerator/denominator is
ever published for TINV in W109. Its only per-surface count anywhere is the
pre-W109 W097 histogram `0/14`.

---

## 3. TEST-STATISTIC AND DISCRETE SURFACES

### 3.1 CHISQ.TEST / CHITEST (G3-05) — identity check only, drift inherited

> `**W109 sweep (2026-07-12): decomposition unblock proven** — `CHISQ.TEST(o,e)
> == CHIDIST(S, df)` BIT-EXACTLY for a specific stored double S (tail cancels in
> the comparison), so the internal statistic is directly measurable without the
> gamma substrate: the internal statistic is IDENTIFIED as the plain-double
> ROW-MAJOR `Σ(o-e)²/e` (offset 0 on the two injective-tail tables of a 4-table
> live set); the CHIDIST tail half remains on the G3-01 substrate, so ALL
> CHISQ.TEST drift is inherited.`
> — `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:117`

> `Proven on live bits, 4 contingency tables (2x2, 2x3, 2x5):`
> `    CHISQ.TEST(obs, exp) == CHIDIST(S, df) exactly,  df = (r-1)(c-1)`
> — `W109_CHISQ_FTEST_DECOMPOSITION_20260712.md:9-11`

**Verdict.** The only figures are `4 tables` / `two injective-tail tables` /
`offset 0` — an **Excel-vs-Excel identity/decomposition** measurement. There is
**no pass-rate figure** for CHISQ.TEST or CHITEST. Any attribution of `152/195`
to CHISQ.TEST would be wrong in kind: the source says drift is *inherited*, which
is a causal claim, not a measured pass rate on this surface. CHITEST is named in
the row title and shares CHISQ.TEST's identity claim with no separate probe.
Severity `NUM-L`, maturity `M1`.

### 3.2 F.TEST / FTEST (G3-06) — identity check only, drift inherited

> `**W109 sweep (2026-07-12): decomposed.** `F.TEST(a,b) == 2·FDIST(F, df_hi,
> df_lo)` BIT-EXACTLY (3 live sets, df to (5,6)); F = larger-var/smaller-var
> (unbiased, n-1 divisor), 2× exact. Statistic layer identified; one
> variance-accumulation ULP detail open; the tail is FDIST -> all drift inherited
> from the G3-01 incomplete-beta substrate.`
> — `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:118`

> `one set matched at statistic offset -1 ULP, isolating a minor
> variance-accumulation-order detail in var()`
> — `W109_CHISQ_FTEST_DECOMPOSITION_20260712.md:31-33`

**Verdict.** `3 live sets` — Excel-vs-Excel identity check. No pass rate.
Severity `NUM-S` (note: *different* severity from G3-05's `NUM-L`, despite the
identical "all drift inherited" logic — an internal inconsistency worth flagging).
FTEST has no separate probe.

### 3.3 HYPGEOMDIST — in the G3-01 header, **untouched by all of W109**

`HYPGEOMDIST` appears in the G3-01 header list (`:113`) and nowhere in the body
of that row, nowhere in the identification note, nowhere in the resume, nowhere
in the wall-clues ledger (verified by grep across `docs/**.md`). It is a discrete
hypergeometric point mass; it shares neither the gamma nor the beta substrate
that every W109 figure measures. Its only per-surface figure is pre-W109:

> `| `HYPGEOMDIST`  |  `46` |   `0` |   `46` |     `2` |      `105` | `6.08E3`  |`
> — `docs/bugs/streams/BUG-FUNC-021_w090_statistical_numeric_exactness_drift.md:144`

plus a structural sub-class:

> `- **NEW HYPGEOMDIST domain sub-class**: `15` rows return `#NUM!`
>   locally where Excel returns a finite probability, all with inputs
>   inside Excel's documented domain.`
> — `BUG-FUNC-021...md:168-170`

**Verdict.** HYPGEOMDIST = `0/46` (2026-05-10 W097 R-D cell-ref re-sweep,
1M local cases / 800 Excel-sampled, seed 17), production, per-surface. **No W109
figure applies to it.** Any inheritance of `152/195`, `337/446`, `b22`, or `b26`
onto HYPGEOMDIST would be a false attribution.

### 3.4 CONFIDENCE.T — explicitly **unprobed** by W109

Row header: `(+ CONFIDENCE.T, Z.TEST unprobed)` (`:113`). Only figure:

> `| `CONFIDENCE.T` |  `87` |   `1` |   `87` |     `1` |      `538` | `3.12E4` |`
> — `BUG-FUNC-021...md:139`

> `- **`CHIDIST`** and **`CONFIDENCE.T`**: largest non-saturating
>   median drift (`739` and `538` ULP). Repair priority: high.`
> — `BUG-FUNC-021...md:163-164`

**Verdict.** `1/87`, production, per-surface, pre-W109 (2026-05-10), sampled
corpus. Explicitly *unprobed* by the W109 campaign, so no W109 figure attaches.

### 3.5 Z.TEST — explicitly **unprobed** by W109

> `| `Z.TEST`       |   `9` |   `4` |    `9` |     `1` |      `167` | `9.07E2` |`
> — `BUG-FUNC-021...md:154`

**Verdict.** `4/9`, production, per-surface, pre-W109. Unprobed by W109.

---

## 4. The pre-W109 per-surface baseline table (the only source for several surfaces)

`BUG-FUNC-021...md:133-154`, W097 R-D cell-ref re-sweep, 2026-05-10:

> `Per-distribution ULP histogram (from `1,000,000` local cases, `800`
> Excel-sampled candidates, seed `17`)` — `:130-131`

| Surface | match/total | median ULP | max ULP |
|---|---|---|---|
| BETADIST | 13/28 | 16 | 4.5E3 |
| BETAINV | 4/71 | 29 | 6.78E17 (sat) |
| CHIDIST | 0/41 | 739 | 1.38E19 (sat) |
| CHIINV | 0/14 | 16 | 4.10E6 |
| CONFIDENCE.T | 1/87 | 538 | 3.12E4 |
| FDIST | 8/43 | 32 | 3.33E3 |
| FINV | 1/80 | 95 | 3.48E7 |
| GAMMADIST | 36/89 | 7 | 1.85E4 |
| GAMMAINV | 2/82 | 31 | 1.45E18 (sat) |
| HYPGEOMDIST | 0/46 | 105 | 6.08E3 |
| TDIST | 6/17 | 38 | 1.23E3 |
| TINV | 0/14 | 13 | 189 |
| Z.TEST | 4/9 | 167 | 9.07E2 |

Two cautions I record as findings:

1. The table's `drifts` column equals `total` on every row (e.g. BETADIST total
   28, match 13, drifts 28) — internally inconsistent, so `match/total` is the
   only trustworthy reading. The build is **not stated anywhere in this
   document**; only `seed 17` and the sampling design are.
2. These are superseded for every surface W109 later landed (CHIDIST, GAMMA.DIST,
   CHIINV, GAMMA.INV, BETAINV, BETA.DIST, TDIST, FINV) but are the ONLY
   per-surface figures for HYPGEOMDIST, CONFIDENCE.T, Z.TEST, GAMMADIST, BETADIST
   and the only count ever published for TINV.

### 4.1 KED-STAT-001 — carries no per-surface figures, and one stale claim

`docs/KNOWN_EXACTNESS_DEVIATIONS.md:108-116` publishes only **whole-run** counts
(`139 cases, 102 exact matches, 37 unexpected mismatches`; `339 cases, 294 exact
matches, 42 unexpected mismatches`) over the mixed W090/W089 replays — these are
**run totals over a 15-function family**, not per-surface figures, and they must
never be attributed to a single surface.

STALE claim found in a primary source:

> `- **Partial repairs already landed**: `BETA.DIST` / `BETADIST` integer-shape
>   CDF, ...` — `docs/KNOWN_EXACTNESS_DEVIATIONS.md:125-126`

Lane-3 **removed** that integer-shape path on 2026-07-18 and proved Excel has no
such path (`b30 capture (768 integer-shape rows): bratio 344/768 vs the shortcut
254/768`, identification `:1092-1093`). KED-STAT-001 was not updated.

---

## 5. Non-pass-rate figures in the G3-01 row that are routinely misread

Recording these explicitly, because they sit in the same sentence stream as the
production pass rates:

| Figure | Quote source | What it actually measures |
|---|---|---|
| `~67% CR-exact overall`, `\|δ\| up to 47` | ident `:26` | **Excel vs mathematical truth**, multi-surface 692-row corpus |
| `416/692` GRATIO transcription | ident `:36-37` | **research-model** vs Excel, multi-surface (12 collapsed surfaces) |
| `closed-int 199/218 = 91%`, `asymp 4/6`, `temme 5/5`, `taylor 48%` | ident `:38-41`, catalog `:113` | **research-model per-branch** of the internal P(a,x); NOT per-surface |
| `179/205 vs 143/205` (a=1 wrapper) | ident `:33-34` | research-model A/B on the a=1 slice |
| `38/45` floor-exp vs `25` CR vs `28` fdlibm | catalog `:113`, ident `:468` | research-model on the **implied-exp** corpus (internal primitive) |
| `0/25k` exp_rd vs floor-exp | catalog `:113`, ident `:490-491` | **instrument validation** of the ported primitive |
| `20,008/20,008` BRATIO vs spec | catalog `:113`, ident `:584` | **instrument validation** (port ≡ Python spec), NOT vs Excel |
| `1,507` / `1,604` / `1,606` / `1,509` tests green | catalog `:113`, ident `:230,:498,:828` | **internal regression** suite |
| `160/160 AND 160/160`, `16 eq / 17 ne`, `33/33` β-scaling, `4 tables`, `3 live sets` | ident `:69-70,:21,:19-20`; decomposition `:9,:24` | **Excel-vs-Excel identity** checks |
| `33,145/33,145` (b24), `113/113` (b27D) | catalog `:113`, ident `:958,:966-968` | research-model→landed **primitive** (distribution pow), not a distribution surface |
| `5,999/6,000` / `4,000/4,000` | catalog `:113` | WEIBULL.DIST / EXPON.DIST — **outside this slice**, adjacent in the same row |
| `34,000/34,000`, `17,996/18,000` | resume `:18-19` | POISSON channel / expm1 — internal primitives |
| `344/768` vs `254/768` (b30) | ident `:1092-1093` | research-model routing discriminator on BETA.DIST integer-shape rows |
| `230/600` (b30 Z-block) | ident `:1097-1098` | "wall-class rate", A/B-bounds staging probe |

---

## 6. STEP-1 verdict table (my independent answer, before reading FOUNDATION.md)

| Surface | Current figure | Measured on this surface? | Kind | Corpus / held-out | Stale figures to retire |
|---|---|---|---|---|---|
| CHIDIST | 152/195 | yes | production | campaign CHIDIST corpus, fitted | 12/195, 144/195, 148/195 |
| CHISQ.DIST.RT | none | no — sibling-inherited from CHIDIST | — | B1 identity (829 probes) | — |
| CHIINV | 15/40 (b19 held-out) + 16/60 (b14 fitted) | yes | production | b19 fresh/never-raced; b14 fitted | 6/40, 10/60 |
| CHISQ.INV | none | no figure; the one mention is ambiguous (RT vs left-tail) | — | — | — |
| CHISQ.TEST | none (identity only) | no figure exists | Excel-vs-Excel identity | 4 tables | — |
| CHITEST | none | no figure exists | — | — | — |
| GAMMA.DIST (cdf) | 337/446 | yes | production | modern corpus, fitted; b26A 1,615/4,100 held-out | 64/268, 137/268, 159/268, 293/446 |
| GAMMA.DIST (pdf) | 16.1% (b31) | yes | production (log-composed) | b31 4,750 rows, triage | "pdf remains unmeasured" clue |
| GAMMADIST | none in W109; 36/89 pre-W109 | no — sibling-inherited from GAMMA.DIST | production (pre-W109) | W097 sampled | — |
| GAMMA.INV | 18/60 | yes | production | b14 **fitted** | 8/60 |
| GAMMAINV | none in W109; 2/82 pre-W109 | ambiguous: header says GAMMAINV, all figures say GAMMA.INV | production | b14 / W097 | — |
| BETA.DIST | 136/288 (in b22 held-out); 4/127 (b21) | yes | production | b22 fresh/disjoint = held out; b21 discriminator = fitted | 0/127 |
| BETADIST | none in W109; 13/28 pre-W109 | no — sibling-inherited from BETA.DIST | production (pre-W109) | W097 sampled | — |
| BETAINV | 12/30 worst +5 | yes | production | b14 **fitted** | 2/30, 4/30 (catalog is stale) |
| FDIST | no count; worst +37→−15 | worst-ULP only | production (worst only) | b22 held-out | — |
| F.DIST.RT | 5/5 | yes but research-model (bpser branch) | research-model | agentA emulator | — |
| FINV | 3/32 | yes | production | b19 **held out** | 0/32 |
| F.TEST | none (identity only) | no figure exists | Excel-vs-Excel identity | 3 live sets | — |
| FTEST | none | no figure exists | — | — | — |
| TDIST | 14/60 | yes | production | b22 **held out** | 6/60 |
| T.DIST.RT | none | no — sibling-inherited from TDIST | — | B1 identity | — |
| TINV | no count; residuals −238→±7 | residual band only | production (band only) | b19 held out | — |
| CONFIDENCE.T | 1/87 | yes | production, pre-W109 | W097 sampled; **W109-unprobed** | — |
| Z.TEST | 4/9 | yes | production, pre-W109 | W097 sampled; **W109-unprobed** | — |
| HYPGEOMDIST | 0/46 | yes | production, pre-W109 | W097 sampled; **W109 never touched it** | — |

Battery totals that belong to NO single surface:
- **`b22 293/671`** — beta-substrate battery total (BETA.DIST 288 rows + F/T
  blocks). Earlier form `285/655` is stale (denominator changed when integer rows
  were scored).
- **`b26 1,615/4,100`** — resolvable to GAMMA.DIST integer-a (b26A) by exact
  arithmetic, but printed unqualified in the catalog; `b26` as a label also
  contains POISSON `4,000/4,000` and the reserved-unraced `b26X`.
- **`+3/111` (b20)** — held-out gamma-series gate; the surface the rows were
  captured through is **ambiguous in source** ("held-out gamma series (fresh a)",
  `:524`; "fresh a in {1.75,2.25,3.25,4.5,5.5,8,12}, df-truncation corrected",
  `:479-480` — the "df-truncation" wording points at CHIDIST rows while "fresh a"
  points at GAMMA.DIST).
- KED-STAT-001's `102/139` and `294/339` — 15-function run totals.

---

## 7. Late addition to Step 1 (found while checking the tracked recon corpus)

`docs/function-lane/DISCREPANCY_RECON_RESULTS_20260710.csv` is one of only two
version-controlled comparison corpora. Its per-surface rows for my slice:

| row_id | function | class | ULP |
|---|---|---|---|
| G3-01-distributions | BETAINV | numeric_drift_gt1ulp | 2 |
| G3-01-distributions | CHIINV | numeric_drift_gt1ulp | 3 |
| G3-05-chisq-test | CHISQ.TEST | numeric_drift_gt1ulp | 8 |
| G3-05-chisq-test | CHITEST | numeric_drift_gt1ulp | 7 |
| G3-06-f-test | F.TEST | numeric_drift_1ulp | 1 |
| G3-06-f-test | FTEST | numeric_drift_1ulp | 1 |

Two consequences:

1. G3-01's *tracked* evidence covers exactly **two** of its ~21 surfaces
   (BETAINV, CHIINV) — "two exact-input test cases" per row, per
   `OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:88-90`, which is also the only place the
   build is attached to a corpus: *"live Excel 16.0 build 20131 result bits"*.
2. CHISQ.TEST / CHITEST / F.TEST / FTEST **do** each have a per-surface
   measured figure after all — but it is a **1-row ULP distance**, not a
   pass/total. A schema whose count element is `{passed, total}` cannot express
   it; it must render as a divergence, not a count.

---

## STEP 2 — comparison against FOUNDATION.md §2.5 / §3.2 / §3.4 / §3.6

What the foundation says about my slice:

- §2.5 attribution table, row `22` (= G3-01): `17 of 21 members` →
  `named-but-not-measured`, justified by *"only CHIDIST, GAMMA.DIST, BETAINV,
  BETA.DIST carry per-surface figures; 'CONFIDENCE.T and Z.TEST are recorded as
  UNPROBED'"* (`FOUNDATION.md:271`).
- §2.5 per-surface count table, row `22`: `CHIDIST 152/195; GAMMA.DIST 337/446;
  BETAINV 12/30; BETA.DIST 293/671 | numeric; BETA.DIST true, rest
  source-does-not-state` (`FOUNDATION.md:292`).
- §3.2 warrant: W4 (counted, none held out) = BETAINV, CHIDIST, GAMMA.DIST.
  W5 (held out) = BETA.DIST. W3 (no count extracted) = BETADIST, CHIINV,
  CHISQ.DIST.RT, CHISQ.INV, CHISQ.TEST, CHITEST, CONFIDENCE.T, F.DIST.RT,
  F.TEST, FDIST, FINV, FTEST, GAMMA.INV, GAMMADIST, GAMMAINV, HYPGEOMDIST,
  T.DIST.RT, TDIST, TINV, Z.TEST (`FOUNDATION.md:478-494`).
- §3.4 numeric axis: N1 (shortfall measured) = BETA.DIST, BETAINV, CHIDIST,
  CHIINV, CHISQ.TEST, CHITEST, CONFIDENCE.T, F.TEST, FDIST, FINV, FTEST,
  GAMMA.DIST, GAMMADIST, GAMMAINV, HYPGEOMDIST, TDIST, TINV, Z.TEST.
  N7 (comparison on record, no count) = BETADIST, CHISQ.DIST.RT, CHISQ.INV,
  F.DIST.RT, GAMMA.INV, T.DIST.RT (`FOUNDATION.md:572-587`).
- §3.6: `entries whose counted corpus was the target of the repair it scores: 18`;
  `entries with ≥1 held-out counted record: 19` (`FOUNDATION.md:637-638`).

### What the foundation gets right (worth recording, since I audited it hostilely)

1. `BETAINV 12/30` — **correct and better than the catalog**. The catalog row
   still says `4/30`; the foundation used the live figure from
   `W109_G3-01_GRATIO_IDENTIFICATION_20260716.md:829` /
   `W109_CAMPAIGN_RESUME_20260718.md:35`. It did not inherit the row's staleness.
2. `CHIDIST 152/195` and `GAMMA.DIST 337/446` — both current, both per-surface,
   both correctly typed as production numeric counts, and the stale
   `12/195 → 144/195` and `64/268 → 137/268 → 159/268` chains were not used.
3. CONFIDENCE.T and Z.TEST correctly flagged as UNPROBED by the row.
4. `GAMMAINV` (legacy) is left uncounted rather than fed GAMMA.INV's figure —
   which is right for a reason the foundation does not state: the B1 multi-view
   collapse proves legacy≡modern for the **five forward CDFs only**
   (`CHIDIST≡CHISQ.DIST.RT, FDIST≡F.DIST.RT, TDIST(·,·,1)≡T.DIST.RT,
   GAMMADIST≡GAMMA.DIST, BETADIST≡BETA.DIST`, ident `:17-18`). **No inverse pair
   is proven identical anywhere in the sources.** No `*INV` figure may be
   sibling-inherited.

### Disagreements

**D1 — `BETA.DIST 293/671` is a battery total, not a BETA.DIST count.**
Foundation `:292` lists it as one of record 22's per-surface counts. Source:
`b22 **293/671** (integer rows now scored; BETA.DIST 136/288)` — ident `:1103-04`.
BETA.DIST is 288 of the 671 rows; the rest are the F- and T-surface blocks
(`FDIST worst +37 -> -15, TDIST worst +-88 -> +17`, ident `:589`; `b22 effect:
TDIST 6->14/60`, ident `:599`). The resume calls the total `beta b22 293/671`
(resume `:33`) — beta *substrate*, not the BETA.DIST *surface*. Correct
per-surface figure = **136/288**. Severity: **wrong-figure** (and, if 293/671 is
retained, `count_scope` must be `group` with `group_members` = BETA.DIST, FDIST,
TDIST — the foundation's own §3.6 rule at `:507`).

**D2 — "only CHIDIST, GAMMA.DIST, BETAINV, BETA.DIST carry per-surface figures"
is false; at least four more do.** All three of the following are inside the
catalog G3-01 row itself, i.e. inside record 22's own anchor text:
`CHIINV roots Q directly, held-out-confirmed b19 15/40 vs 6/40`;
`FINV roots the FDIST complement form 0→3/32 with small-p bias collapsed`;
`b14: GAMMA.INV 8→18/60 worst −16` — all `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:113`.
And in the identification note the row cites: `b22 effect: TDIST 6->14/60 exact`
(ident `:599`). Severity: **wrong-attribution** — `named-but-not-measured` is
wrong for CHIINV, FINV, GAMMA.INV and TDIST.

**D3 — CHIINV should be a held-out counted entry (W5), not W3.**
`b19 (fresh rows, never raced): CHIINV Q-direct CONFIRMED held-out (15/40 vs
6/40 for P at 1-p, same systematic negative bias on fresh rows)` — ident
`:509-510`; heading `## *INV published-surface principle extended (b19 held-out)`
`:507`. Exact hold-out words present: *"fresh rows, never raced"*, *"held-out"*.
Severity: **underclaim**.

**D4 — FINV should be a held-out counted entry (W5), not W3.** Same b19 section:
`the same invert-the-published-surface staging decisively improves FINV (0/32 ->
3/32, small-p bias -60 -> +2)` — ident `:511-512`. Severity: **underclaim**.

**D5 — GAMMA.INV has a per-surface count (18/60) and is placed at N7 "no row
count was extracted".** `b14 effect: GAMMA.INV 8->18/60 exact, worst +880,380 ->
-16 ULP` — ident `:433`. Corpus is **the repair's own target**: `Landed in
production (validated on the b14 corpora, held-out b19 captured separately)`
ident `:427-428`. Severity: **underclaim** (with `corpus_was_repair_target:
true`).

**D6 — TDIST has a per-surface count (14/60) on a held-out battery.**
`b22 effect: TDIST 6->14/60 exact` — ident `:599`; b22 is
`Held-out gate b22 (671 fresh live rows, disjoint values)` — ident `:587`.
Severity: **underclaim**.

**D7 — GAMMA.DIST has a held-out per-surface gate the foundation does not carry.**
Foundation: GAMMA.DIST is W4, "none held out", `held_out: source-does-not-state`.
Source: `## b26 held-out gate (2026-07-18)` (ident `:933`) →
`- b26A GAMMA.DIST (production path, integer-a moderate-y uniform grids):
a=2 795/1600 (worst 4), a=3 544/1600 (worst 7), a=4 276/900 (worst 10)`
(ident `:938-939`), which sums exactly to the catalog's `b26 1,615/4,100`.
4,100 fresh integer-a rows, production path, per-surface. GAMMA.DIST is
**W5-eligible**. Severity: **underclaim**.

**D8 — GAMMA.DIST's pdf mode is a separate, far worse production figure and is
absent.** `**GAMMA.DIST pdf MEASURED (b31, 4,750 rows banked) ...** Triage
REFUTED at the exact-bit level: production log-composed (16.1%) ...` —
`W109_WALL_CLUES_LEDGER.md:201-205`. Rendering `337/446` as *the* GAMMA.DIST
figure, with no pdf figure anywhere, overstates the surface: the same function's
`cumulative=FALSE` mode scores 16.1% on 4,750 rows. Severity: **overclaim**.
(Also: the identification note's `Clue banked: GAMMA.DIST **pdf**
(cumulative=FALSE) remains unmeasured`, ident `:1107`, is itself stale — b31
measured it the next lane.)

**D9 — five alias siblings are typed `named-but-not-measured` when the source
proves them bit-identical to a measured sibling.** `Legacy ≡ modern bit-for-bit
everywhere probed: CHIDIST≡CHISQ.DIST.RT, FDIST≡F.DIST.RT, TDIST(·,·,1)≡T.DIST.RT,
GAMMADIST≡GAMMA.DIST, BETADIST≡BETA.DIST.` — ident `:17-18` (battery B1, 829
probes, build 20131). The schema's correct enum for CHISQ.DIST.RT, F.DIST.RT,
T.DIST.RT, GAMMADIST, BETADIST is `alias-sibling-inherited` (the value the
foundation itself uses for WEIBULL, EXPONDIST, GAMMALN.PRECISE at `:268-270`),
not `named-but-not-measured`. Severity: **wrong-attribution**. Note the identity
is an **Excel-vs-Excel** check, so the inherited count keeps its own
`measurement_subject` but gains `attribution: alias-sibling-inherited`.

**D10 — BETAINV's `held_out: source-does-not-state` understates what the source
says.** The source states the corpus status explicitly: b14 is what the inverter
was `validated on`, with `held-out b19 captured separately` (ident `:427-428`);
the 12/30 is a b14 re-score after the chain-exp landing (`**b14 BETAINV 4 ->
12/30, worst +13 -> +5**`, ident `:829`). So `held_out: false` +
`corpus_was_repair_target: true`. Severity: **underclaim**.

**D11 — CHISQ.INV is placed at N7, "a numeric comparison is on record"; I can
find no sentence that supports it.** The only occurrence in the entire W109
corpus is `BETAINV/CHISQ.INV inverter-limited no longer; forward error dominates.`
(ident `:516-517`) — no figure, no corpus, and **ambiguous in source** about which
surface it means: the section's kernel is `chisq_inv_rt_kernel`
(ident `:437`), i.e. the right-tail inverse (CHIINV / CHISQ.INV.RT), whereas
`CHISQ.INV` is the left-tail modern function. `CHISQ.INV` also does not appear in
the G3-01 header list. Severity: **overclaim** (N2 "named in an open divergence
row; no measurement of this surface is published in it" is the supportable state).

**D12 — four surfaces have a real per-surface count that W109 never superseded,
and the foundation carries none of it.** `BUG-FUNC-021` — cited in G3-01's own
Evidence cell and by KED-STAT-001 — publishes a per-distribution histogram
(`from 1,000,000 local cases, 800 Excel-sampled candidates, seed 17`,
`BUG-FUNC-021...md:130-131`): HYPGEOMDIST `0/46`, CONFIDENCE.T `1/87`,
Z.TEST `4/9`, TINV `0/14`. W109 never touched any of these four (grep-verified:
HYPGEOMDIST appears nowhere in the W109 notes; CONFIDENCE.T and Z.TEST are
"unprobed" per the row; TINV has only a residual band `-4..-238 -> mostly +-1..7`,
ident `:512`, never a count). So these are their **only and current** counts, and
"no row count was extracted" is an extraction gap, not a source gap.
Severity: **underclaim**. (The same table's BETADIST 13/28, FDIST 8/43,
GAMMADIST 36/89, GAMMAINV 2/82, CHIINV 0/14, TDIST 6/17, BETAINV 4/71,
CHIDIST 0/41 are genuinely superseded for the W109-landed surfaces — those I
would leave out, but the reason must be *superseded*, not *absent*.)

**D13 — CHISQ.TEST / CHITEST / F.TEST / FTEST at N1 "a shortfall was measured":
defensible, but the measurement is a 1-row ULP witness and the count schema
cannot hold it.** Their only per-surface measurements are
`DISCREPANCY_RECON_RESULTS_20260710.csv` rows (CHISQ.TEST 8 ULP, CHITEST 7 ULP,
F.TEST 1 ULP, FTEST 1 ULP) plus **Excel-vs-Excel identity** decompositions —
`CHISQ.TEST(obs, exp) == CHIDIST(S, df) exactly` on `4 contingency tables
(2x2, 2x3, 2x5)` and `F.TEST(a, b) == 2 · FDIST(F, df_hi, df_lo) exactly` on
`3 two-sample sets` (`W109_CHISQ_FTEST_DECOMPOSITION_20260712.md:9-11, 24-26`).
No pass rate exists for any of the four, and "all drift inherited from
CHIDIST/FDIST" (catalog `:117`, `:118`) is a **causal** claim — it must never be
turned into an inherited pass rate. Severity: **cosmetic** (state is right;
the gloss must say ULP-witness, not count). Sub-note: G3-05 carries `NUM-L` and
G3-06 carries `NUM-S` (catalog `:117`, `:118`) despite identical inheritance
logic — an upstream inconsistency the handbook should surface, not smooth.

**D14 — build provenance: `single-build-not-restated-on-the-scored-line` is the
only honest value for every W109 figure in this slice.** `20131` appears once in
the identification note (`## The multi-view collapse (battery B1, 829 probes,
build 20131)`, ident `:15`) and in the catalog's recon header (`live Excel 16.0
build 20131 result bits`, catalog `:89`, which describes the **48-case 2026-07-10
recon corpus**, not the b14/b19/b20/b21/b22/b26/b30/b31 batteries). It is not
restated near 152/195, 337/446, 12/30, 136/288, 293/671, 1,615/4,100, 15/40,
3/32, 18/60 or 14/60. The W097 histogram (`BUG-FUNC-021`) names **no build at
all** — only `seed 17`. Severity: **cosmetic**, but it must not be silently
upgraded to `single-build`.

### Corrected record-22 member partition (my answer)

21 members, partitioned by what the sources actually support:

- **measured per-surface, production (8)**: CHIDIST 152/195; GAMMA.DIST 337/446
  (+ held-out b26A 1,615/4,100 integer-a; + pdf 16.1% b31); BETA.DIST 136/288
  (+ b21 4/127); BETAINV 12/30; CHIINV 15/40 (b19 held out) & 16/60 (b14);
  FINV 3/32 (b19 held out); GAMMA.INV 18/60 (b14, repair target);
  TDIST 14/60 (b22 held out).
- **alias-sibling-inherited (5)**: CHISQ.DIST.RT, F.DIST.RT (own figure is a
  model score, bpser 5/5), T.DIST.RT, GAMMADIST, BETADIST.
- **named, no count on the current build (8)**: FDIST (worst-ULP only,
  +37→−15), TINV (residual band only), GAMMAINV (no proven inverse alias
  identity → nothing may be inherited), CHISQ.INV (ambiguous mention, no
  figure), NEGBINOMDIST (measured NOT to inherit, 10.4%, catalog `:113`),
  HYPGEOMDIST (0/46 pre-W109 only), CONFIDENCE.T (1/87 pre-W109 only, unprobed),
  Z.TEST (4/9 pre-W109 only, unprobed).

