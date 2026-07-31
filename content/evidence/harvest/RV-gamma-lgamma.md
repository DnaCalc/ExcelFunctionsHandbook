# RV-gamma-lgamma — independent re-derivation of measurement attributions

Slice: **gamma-lgamma**.
Surfaces: GAMMALN, GAMMALN.PRECISE, GAMMA, COMBIN, COMBINA, FACT, FACTDOUBLE, MULTINOMIAL, PERMUT,
PERMUTATIONA, SQRTPI.

Method discipline followed: every figure below was located in the OxFunc primary sources **before**
FOUNDATION.md §2.5 was opened. Where the sources contradict themselves I did not guess — I
re-executed the measurement. Four independent re-measurements were run against the in-repo
oracle cache and the in-repo production kernels (scripts in the scratchpad, not written into
OxFunc, which is read-only). They are labelled **[RE-MEASURED]** and are the load-bearing part of
this review, because the sources do contradict each other on the single most-cited figure in the
slice.

Basis: OxFunc working tree at the session's HEAD; oracle cache
`smart-fuzzer/cache/oracle/build-20131/*.jsonl` (every row carries
`"excel_build":"20131"`, `"cpu_id":"AMD Ryzen 7 4800H …"`).

---

## 0. Executive verdict

Five findings, in descending severity.

1. **`0/79` (worst 1,370 ULP) is GAMMA's figure, not GAMMALN's.** Three OxFunc documents
   re-attribute it to `GAMMALN/GAMMALN.PRECISE`. I reproduced the number exactly by re-running
   production against the corpus: GAMMA positive side `0/79`, worst `−1,370` ULP at `x=108.637`,
   negatives worst `−810,173` ULP. Pre-landing GAMMALN on *its* own 79-row corpus was `16/79`,
   worst `−19,943` — a different measurement entirely. The coincidence that both corpora contain
   exactly 79 positive rows is what baked the error in.
2. **The `316/400` held-out gate was captured on the GAMMALN.PRECISE surface, not GAMMALN.** All
   400 held2 arguments are present in `GAMMALN.PRECISE.jsonl` and in **zero** rows of
   `GAMMALN.jsonl`. The direction of inheritance in FOUNDATION §2.5 ("the 316/400 gate is
   GAMMALN's") is inverted.
3. **`316/400 = 79.0%` describes the superseded first landing, not the shipped kernel.** The
   shipped kernel (B2 re-landed continuous + LM coefficients, lane 4) scores **314/400 (78.5%)**
   on that same gate and **851/1,200** on the round-2 gate — both slightly *lower* than the
   figures the catalog headlines, while being higher on the fresh corpus. OxFunc's own campaign
   resume publishes `314/400`; the catalog and the resume's own summary table publish `79.0%`.
4. **COMBIN does have a per-surface production figure (`6/16`)** — the catalog states Excel matches
   *OxFunc's* multiply-first kernel on 6 of 16 discriminating pairs, and `combinations_of_int` is
   exactly that kernel. FOUNDATION carries COMBIN as `named-but-not-measured` with
   `passed:null,total:null`. That is an underclaim.
5. **SQRTPI has a counted figure (`30/30`)** in XMD-003. FOUNDATION carries SQRTPI at W3 /
   N4 "witness exists, no count". Underclaim, with a genuine scope ambiguity about whether the 30
   inputs were SQRTPI calls or bare `pow` calls.

---

## 1. GAMMALN

### 1.1 The corpus geography (established first, because everything else depends on it)

> `Corpus: answers-gammaln.json (79) + ../G4-04-combin/answers-gammaln.json`
> `(14) + answers-r2.json (274 dense). GAMMA: answers-r0.json (156).`
— `smart-fuzzer/work/w109/G3-02-gamma/NEXT-STEPS.md:21-22`

So there are **two** 79-row objects in this lane: a 79-row **GAMMALN** corpus, and a 156-row
**GAMMA** corpus. **[RE-MEASURED]** `answers-r0.json` declares `"function": "GAMMA"` and splits
79 positive / 77 negative. `answers-gammaln.json` declares `"function": "GAMMALN"`, 79 rows, all
positive. This collision is the mechanical cause of finding 1.

Surface tag of every corpus in the work dir **[RE-MEASURED]** (read from each file's `function`
field):

| file | declared surface | rows |
|---|---|---|
| `answers-gammaln.json` | GAMMALN | 79 |
| `answers-r2.json` | GAMMALN | 274 |
| `answers-dense1.json` | GAMMALN | 1,016 |
| `answers-g12dense.json` | GAMMALN | 1,468 |
| `answers-peel.json` | GAMMALN | 399 |
| `answers-b32-gammaln.json` | GAMMALN | 1,600 (b32B1 400 + b32B2 1,200) |
| `answers-precise.json` | **GAMMALN.PRECISE** | 17 |
| `answers-L-boundary.json` | **GAMMALN.PRECISE** | 1,793 |
| `answers-L-core.json` | **GAMMALN.PRECISE** | 9,642 (incl. `held-` 1,200) |
| `answers-L-round3.json` | **GAMMALN.PRECISE** | 2,795 (incl. `held2-` 400) |
| `answers-r0.json` | **GAMMA** | 156 |

The entire structural identification campaign (boundary + core + round-3 = 14,230 rows) **and both
held-out gates** ran on GAMMALN.PRECISE captures. Only the early corpora and the lane-4 fresh b32
gate ran on GAMMALN.

### 1.2 The figures the source publishes

**(a) The headline held-out figure — 316/400.**

> `**KERNEL LANDED (commit 223cfa5, agent port bit-identical to reference on 17,003 rows):`
> `GAMMALN/GAMMALN.PRECISE 0/79 (worst 1,370) → held-out 316/400 = 79.0% (worst 5).**`
— `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:114` (G3-02 row, Round-3 clause)

> `held2  (round-3 sweep, 400 rows) : 316 exact (79.0%), worst 5`
— `smart-fuzzer/work/w109/G3-02-gamma/agentL_landing_spec.md:161`

> `held2 400 → 316 (79.0%) worst 5; b1 zone 8/25, b2 zone 44/80, composed 18/21,`
> `b4lo 102/129, stirl8 144/145. held 855/1200 (71.2%). All match the pinned spec.`
— `smart-fuzzer/work/w109/G3-02-gamma/agentS_results.md:11-12`

> `- GAMMALN kernel landed same commit (see the G3-02 note): held-out 79.0%`
> `  worst 5, from 0/79 worst 1,370.`
— `docs/function-lane/W109_G3-01_GRATIO_IDENTIFICATION_20260716.md:836-837`

**(b) The same gate, published as 314/400 — twice.**

> `- **Composite promotion (held2, never fitted): 314/400 = 78.5% exact, worst 4**`
— `docs/function-lane/W109_GAMMALN_IDENTIFICATION_20260711.md:396`

> `GAMMALN held2 314/400 + fresh-b32 b2 549/1,200 (B2 continuous re-landing:`
— `docs/function-lane/W109_CAMPAIGN_RESUME_20260718.md:34`, under the heading
`Corpus scores after all landings (lane-3 re-verified 2026-07-18, production routing ≡ identified
substrate bit-for-bit …)`

The resume's own summary table, in the same file, still says `79.0%`:

> `| Published GAMMALN | … | excel_numeric/gammaln.rs → GAMMALN/GAMMALN.PRECISE | port`
> `bit-identical to ref on 17,003 rows; held-out 79.0% worst 5 |`
— `docs/function-lane/W109_CAMPAIGN_RESUME_20260718.md:23`

**(c) The fresh-corpus figure.**

> `**Lane-4 (2026-07-18, agent-S + fresh b32 gate): B2 RE-IDENTIFIED as fully-continuous x87`
> `(same class as B4; noise floor 1.077 vs spill 1.113) and RE-LANDED with LM-refit coefficients`
> `— fresh never-probed b32: 549 vs 518/1,200 …**`
— `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:114` (G3-02, Lane-4 clause)

> `**B1 [0.7,1.5): confirmed at its form-family wall.** Every refit overfits`
> `(held-out drops); the published 1967 n=7 plain-double stands. Fresh b32:`
> `123/400 (30.8%), consistent.`
— `docs/function-lane/W109_GAMMALN_IDENTIFICATION_20260711.md:425-427` (lane-4 section). Note the
b1 fresh figure appears **only** in the identification note, not in `agentS_results.md`, which reports
b1 only as held-set scores (`agentS_results.md:120-130`).

**(d) The band figure that is genuinely near-closed.**

> `**5739/5749 = 99.83% bit-exact over x in [8, 1030], worst 2.**`
— `docs/function-lane/W109_GAMMALN_IDENTIFICATION_20260711.md:384`

**(e) The round-2 gate.**

> `- **Held-out promotion (golden-ratio sweep, never fitted): composite best`
> `  model 644/1199 = 53.7% exact, worst 4**`
— `docs/function-lane/W109_GAMMALN_IDENTIFICATION_20260711.md:361-362`
(this is the pre-round-3 model on the round-2 `held-` set; the landing spec's counterpart figure for
the landed kernel is `held (round-2 sweep, 1200 rows): 855 exact (71.2%)`, `agentL_landing_spec.md:162`)

### 1.3 Held out from what — the exact word

> `All fits use the CORRECTED fit split (excludes BOTH `held-` 1200 and`
> `held2-` 400 …) … Held gates untouched by all fits.`
— `smart-fuzzer/work/w109/G3-02-gamma/agentS_results.md:4-7`

So `held-` (1,200) and `held2-` (400) are held out **from the coefficient fits**, not from the
structural identification: the 0.7 threshold, the 8.0 seam, the band skeleton and the Stirling form
were all pinned on the boundary/core/round-3 batteries that surround these rows. The word used for
b32 is stronger:

> `the fresh`
> `never-probed b32 corpus (1,600 rows) ruled: that candidate 505/1,200 —`
> `WORSE than the landing 518 (selection leakage, exactly the MINVERSE lesson)`
— `docs/function-lane/W109_GAMMALN_IDENTIFICATION_20260711.md:417-420`

`fresh` / `never-probed` for b32; `held-out` / `never fitted` for held- and held2.

### 1.4 [RE-MEASURED] What the shipped kernel actually scores

I transcribed `crates/oxfunc_core/src/excel_numeric/gammaln.rs` (the shipped kernel, including the
x87-continuous B2/B4 emulated at 64-bit mantissa) into Python and scored it against the banked live
captures. The replica is faithful: it reproduces four of agentS's five zone counts exactly
(composed 18/21, b4 102/129, stirl 144/145, b1 8/25) and both lane-4 b32 band counts exactly
(b1 123/400 = 30.8%, b2 549/1,200 = 45.8%). The only zone that differs from agentS's sanity gate is
B2 — which is precisely the band lane 4 re-landed.

| gate | source figure | shipped kernel, re-measured | surface the rows were captured on |
|---|---|---|---|
| held2 (400) | **316** (79.0%) worst 5 | **314** (78.5%) worst 5; b2 42/80 not 44/80 | GAMMALN.PRECISE |
| held- (1,200) | **855** (71.2%) worst 5 | **851** (70.9%) worst +5 | GAMMALN.PRECISE |
| b32 B2 (1,200) | **549** vs 518 | **549** (45.8%) | GAMMALN |
| b32 B1 (400) | **123** (30.8%) | **123** (30.8%) | GAMMALN |
| b32 all (1,600) | *not published* | **672** (42.0%) worst +7 | GAMMALN |
| all 14,230 .PRECISE rows | *not published* | **10,268** (72.2%) | GAMMALN.PRECISE |

**Verdict.** `316/400` is a production figure for the **223cfa5** kernel. The kernel in the tree is
the lane-4 re-landing, and its held-out figure is `314/400`. The catalog's `→ held-out 316/400 =
79.0%` is stale by one landing. OxFunc itself states the corrected number at
`W109_CAMPAIGN_RESUME_20260718.md:34`.

### 1.5 [RE-MEASURED] Where 0/79 (worst 1,370) really comes from

I reconstructed the pre-landing kernel — `ln_gamma_positive` (Lanczos g=7, coefficients at
`crates/oxfunc_core/src/functions/special_dist_family.rs:85-96`, body at `:121-141`) and
`gamma_kernel` (`:252-283`, `exp(ln_gamma)` with the `x<0.5` sine reflection) — and scored both:

| candidate | corpus | exact | worst ULP |
|---|---|---|---|
| `gamma_kernel` (Lanczos+exp) | `answers-r0.json` **positive 79** | **0/79** | **−1,370** at x=108.637 |
| `gamma_kernel` | `answers-r0.json` negative 77 | 1/77 | **−810,173** at x=−147.000014 |
| `ln_gamma_positive` (Lanczos) | `answers-gammaln.json` **79** | 16/79 | −19,943 at x=1.00012 |
| `ln_gamma_positive` | b32 (1,600) | 58/1,600 | +8,139 |

The GAMMA row reproduces the catalog's 2026-07-11 sentence *number for number*, including the "810k":

> `a fresh 156-row live sweep shows the POSITIVE side is `0/79` exact (up to `1370` ULP at large x)`
> `and negatives reach `810k` ULP`
— `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:114` (G3-02, 2026-07-11 clause)

The GAMMALN row does not, and cannot: Excel's own GAMMALN sits within 2.03 ULP of the true
`loggamma` on that corpus (**[RE-MEASURED]** against mpmath at 60 dps), so a 1,370-ULP GAMMALN
deviation "at large x" is arithmetically unavailable. Where the Lanczos GAMMALN *is* badly wrong it
is at `x=1.00012` (a zero of lgamma), not at large x, and the magnitude is 19,943 ULP.

**Verdict.** `0/79 worst 1,370` is GAMMA's positive-side production baseline. Three places
re-attribute it to GAMMALN — and one of them shows the copy happening:

> `Production baseline for comparison (catalog G3-02): current Lanczos`
> `log-domain kernel is 0/79 exact on positives with errors to 1370 ULP; this`
> `composite is a material improvement on every band.`
— `smart-fuzzer/work/w109/G3-02-gamma/agentL_landing_spec.md:168-170`

It cites *the catalog* (i.e. the GAMMA sentence) as the GAMMALN baseline. The identification note
repeats it at `:398` and the GRATIO note at `:837`. **This is a wrong-attribution in the OxFunc
sources, inherited unchanged into the harvest and into FOUNDATION.**

### 1.6 Excel build, at the figures

`docs/function-lane/W109_GAMMALN_IDENTIFICATION_20260711.md:3` — `Live oracle: Excel 16.0 build
20131, x86-64 AMD host.` — is stated once at the top of the document, in the context of the 2026-07-11
361-row corpus. It is **not restated near** the Round-3 `316/400`, the Lane-4 `549/1,200`, or the
`99.83%` figures. `agentS_results.md:3-4` says the lane-4 work was `Offline against banked corpora
only; no live Excel.` The build is nevertheless recoverable per-row from the cache
(`"excel_build":"20131"`, `"cpu_id":"AMD Ryzen 7 4800H …"`), which is a stronger provenance than the
prose. Correct label: **build not restated near the figure; recoverable from the cache as 20131.**

---

## 2. GAMMALN.PRECISE

### 2.1 What the source says

> `- **Target:** Excel `GAMMALN` (≡ `GAMMALN.PRECISE` on build 20131, verified bit-identical)`
— `docs/function-lane/W109_GAMMALN_RESUME.md:7`

> `- **`GAMMALN.PRECISE` == legacy `GAMMALN`** on modern build 20131: identical bits at`
> `  every probed point, both ±3 ULP from true. Excel 2010 introduced `.PRECISE`, but on`
> `  this build BOTH resolve to the SAME custom Cody-family implementation`
— `docs/function-lane/W109_GAMMALN_PUBLISHED_COEFFICIENTS.md:104-107`

That is an **Excel-vs-Excel identity check**. The source gives no numerator/denominator for it —
"identical bits at every probed point" — and the dedicated probe file is 17 rows
(`answers-precise.json`, ids `gp-000..gp-016`).

Production side, the alias is structural, not measured:

```
pub fn gammaln_precise_kernel(x: f64) -> Result<f64, WorksheetErrorCode> {
    gammaln_kernel(x)
}
```
— `crates/oxfunc_core/src/functions/special_dist_family.rs:297-299`

### 2.2 [RE-MEASURED] The identity, and the direction of inheritance

Two independent checks:

- Work-dir corpora: 91 arguments appear in both a GAMMALN-tagged and a GAMMALN.PRECISE-tagged
  answers file. **0 bit mismatches.**
- Oracle cache: `GAMMALN.jsonl` 4,450 rows, `GAMMALN.PRECISE.jsonl` 14,262 rows, **109 shared
  arguments, 0 bit mismatches** (kind and `bits_hex` both compared).

So the identity is solid, and better evidenced than the source's 17-row probe.

But the **direction** is the opposite of what FOUNDATION records:

| gate | in `GAMMALN.jsonl` | in `GAMMALN.PRECISE.jsonl` |
|---|---|---|
| the 400 held2 rows (the `316/400` gate) | **0** | **400** |
| the 1,600 b32 rows (the lane-4 fresh gate) | **1,600** | **0** |

The `316/400` gate is a **GAMMALN.PRECISE** measurement. GAMMALN's own per-surface, fresh-corpus
measurement is b32. Both surfaces are separately probed live; neither is a pure alias inheritance in
the evidence, even though production aliases them.

**Verdict.** GAMMALN.PRECISE is **separately measured on live Excel** (17 dedicated rows + the whole
14,230-row identification campaign + 14,262 cached rows), and the headline held-out gate is *its*
capture. The correct label is `excel-vs-excel-identity` for the equivalence plus
`measured-for-this-surface` for the 400-row gate — not `alias-sibling-inherited`.

---

## 3. GAMMA

### 3.1 The figure

> `W109 re-scoping (2026-07-11): the row was under-scoped — a fresh 156-row live sweep shows the`
> `POSITIVE side is `0/79` exact (up to `1370` ULP at large x) and negatives reach `810k` ULP; the`
> `recon corpus had only probed two negative points.`
— `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:114`

Per-surface (row header is `G3-02 — GAMMA (+ GAMMALN substrate)`), production-vs-Excel, **not** held
out (a discovery re-scoping sweep), 2026-07-11, build named in the catalog header for that reconcile
date but not on the row.

### 3.2 Is it real? Is it current? Did the landed kernel change it?

**Real — [RE-MEASURED] and exact.** `gamma_kernel` reproduced in Python: 0/79 on the positive side,
worst −1,370 ULP; 1/77 on the negative side, worst −810,173 ULP.

**Current — yes, by code inspection.** The landed kernel is wired only to the two GAMMALN surfaces:

> `// Published GAMMALN / GAMMALN.PRECISE surface: the identified Excel op-graph`
> `// kernel (W109 G3-02) for positive arguments. … only the positive-x numeric path changes.`
> `// GAMMA and the shared internal lgamma are unaffected.`
— `crates/oxfunc_core/src/functions/special_dist_family.rs:286-290`

`gamma_kernel` (`:252-283`) still calls `ln_gamma_positive`, the Lanczos routine. `gammaln.rs`'s own
header says the same: `only the GAMMALN / GAMMALN.PRECISE surfaces route here`
(`crates/oxfunc_core/src/excel_numeric/gammaln.rs:6-7`). The catalog's forward plan confirms GAMMA is
future work: `then GAMMA = exp composition (+ sin reflection), COMBIN, G3-01 fractional-a re-race`
(`docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:114`).

So the answer to "did the landed kernel change GAMMA" is **no** — and this now settles the open
question the harvest itself flagged (C6 record 21: *"Whether the 2026-07-18 GAMMALN landing improved
GAMMA has not been re-measured in any record I found."*).

### 3.3 [RE-MEASURED] A much larger GAMMA capture exists and has never been scored

`GAMMA.jsonl` holds **12,312** cached live rows. Production scores **410/12,312 = 3.33%** exact
over the whole shard (positives only: 365/11,636 = 3.14%), worst finite −810,173 ULP. No record I
found publishes any figure over this corpus. The published `0/79` is therefore *conservative but
corpus-specific*: on the broader cached corpus production is ~3% rather than 0%. Anyone rendering
"0 of 79" should not be told that 0% generalises; it does not, quite.

---

## 4. COMBIN

### 4.1 The per-surface production figure the source does publish

> `COMBIN = multiplicative product but **NOT bit-exact** (cycle-2 design-for-divergence capture`
> `CORRECTED the earlier "bit-exact <2^53" over-claim, which rested on a non-discriminating`
> `7-point corpus where all forms agree below ~2^40): on 16 discriminating `(n,k)` with`
> `representable results, Excel matches OxFunc's multiply-first `(acc*(n-k+i))/i` only `6/16`,`
> `ratio-first `2/16`, exact-integer `6/16`, and NEITHER `8/16`; Excel sits `1`-`3` ULP BELOW the`
> `multiply-first product on larger `n,k`.`
— `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:127` (G4-04 row)

`(acc*(n-k+i))/i` is verbatim the shipped kernel:

```
let k = k.min(n - k);
let mut acc = 1.0;
for i in 1..=k { acc *= (n - k + i) as f64; acc /= i as f64; }
```
— `crates/oxfunc_core/src/functions/combinatorics_common.rs:8-12`

So `6/16` is **production OxFunc vs Excel, on COMBIN, per-surface, 16 rows, not held out**. It is
small and it is a design-for-divergence corpus (deliberately adversarial), but it is a measurement of
this surface.

### 4.2 The candidate score, which is a different thing

> `Ruled out (505-row live corpus; see the ruled-out ledger):`
> `- every product-loop family … (best candidate 82/505);`
— `docs/function-lane/W109_PERMUT_COMBIN_FINDINGS_20260711.md:19-22`

`82/505` is a **research-candidate score** over the identification corpus, not a production pass
rate. The positive finding on the same page is structural:

> `Positive identification: **Excel reduces `k -> min(k, n-k)`** —`
> `COMBIN(23,13)` publishes bit-identical results to `COMBIN(23,10)`.`
— `docs/function-lane/W109_PERMUT_COMBIN_FINDINGS_20260711.md:16-17`

### 4.3 Divergence witnesses (uncounted, and one group count)

> `Newly-visible 1-ULP rows include `=COMBIN(23, 10)` where OxFunc returns`
> `the exact integer `1144066.0` and Excel returns `1144066.0000000002``
— `docs/bugs/streams/BUG-FUNC-027_broad_scalar_invocation_space_findings.md:305-307`

> ``26` rows across the six cycles are combinatorial functions where OxFunc returns the exact`
> `integer and Excel returns the integer `±1` ULP`
— `docs/bugs/streams/BUG-FUNC-027_broad_scalar_invocation_space_findings.md:351-356`

That `26` is a **group divergence count** over COMBIN + COMBINA (examples given for both), on Excel
16.0 build **20026** cell-ref plumbing — a different build from the W109 figures. It is a count of
mismatches, never of passes.

### 4.4 Was COMBIN measured, or only implicated via the internal-lgamma substrate?

**Measured** — `6/16` per-surface, plus a 505-row live corpus (**[RE-MEASURED]**: `COMBIN.jsonl`
holds 503 cached rows, three short of the "505-row" prose; production scores **70/503 = 13.9%**
exact over it, worst +36 ULP at `(600,366)` — a figure no record publishes). The *substrate*
attribution is the hypothesis, not the measurement:

> `leading hypothesis is an internal extended lgamma/exp substrate (Phase-5 lane)`
— `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:127`

So: COMBIN is genuinely measured on its own surface; it is *additionally* implicated in the
internal-lgamma wall. The two must not be conflated in either direction.

---

## 5. COMBINA

> `COMBINA = **`exp(gammaln)` substrate, NOT a product** — CONFIRMED:`
> ``COMBINA(20,7)=C(26,7)` returns `657799.9999999999`, 1 ULP BELOW the exact integer `657800``
> `(impossible for a product); reduces to the **GAMMALN/x87 wall** (crack GAMMALN → COMBINA free).`
— `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:127`

This is a **single-witness Excel-vs-mathematical-truth** observation used as a structural
discriminator. It is *not* an OxFunc pass/fail count, and there is no denominator. The recipe line
confirms the reduction is designed, not executed:

> `Recipe: capture `GAMMALN(n+k)/(k+1)/(n)` + `EXP` at the COMBINA arg-triples to formally reduce`
> `it to GAMMALN.`
— `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:127`

Production is a product (`combina_kernel` → `combinations_of_int(n+k-1, k)`,
`crates/oxfunc_core/src/functions/combina.rs:28-37`), i.e. production does **not** implement the
identified substrate. Additional uncounted divergence witnesses at build 20026 in BUG-FUNC-027:
`=COMBINA(41,16) → 41,648,951,840,265` local vs `…,265.01` in Excel (`:355-356`).

**Verdict: no per-surface pass count exists for COMBINA.** One truth-witness, several divergence
witnesses, one group count of 26 mismatches shared with COMBIN.

---

## 6. FACT

No numeric-bits comparison figure exists anywhere in the primary sources. The only record is the
error-surface convention:

> `- **Functions:** `EXP`, `SINH`, `COSH`, `FACT`, `FACTDOUBLE`, `DEGREES`, `PERMUTATIONA` (and the`
> `  overflow arm of `POWER`/`^`) — the `ExcelRealPolicy::FINITE` family.` … `Excel **never publishes`
> `  `±Inf`/`NaN``… → **`#NUM!`** (`EXP(1000)=#NUM!`, `FACT(171)=#NUM!` vs `FACT(170)` finite)`
— `docs/EXCEL_MATH_DEVIATION_CATALOG.md:181-188` (XMD-008)

That is a **kind/error-code** reproduction with two witnesses (`FACT(170)`, `FACT(171)`) on build
**20026**, no count, and it belongs on the admission axis, not the numeric axis.
`FUNCTION_LANE_EVIDENCE_ID_REGISTRY.md` carries no FACT numeric baseline.

**Verdict: no-figure-exists on the numeric-bits axis.**

---

## 7. FACTDOUBLE

> `FACTDOUBLE bit-exact (7/7)`
— `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:127` (inside the G4-04 row)

Per-surface, production-vs-Excel, 7 rows, 2026-07-14 W109 sweep. Held-out status is **not stated**.
Build is **not restated** on this clause (the G4-04 row and the sweep sit on build 20131 by
context). Seven rows is a spot check; FACTDOUBLE also appears in XMD-008's `FINITE` family
(`docs/EXCEL_MATH_DEVIATION_CATALOG.md:182`) with no count. Notably, FACTDOUBLE sits inside a row
whose header lists it among **open** discrepancies, yet its own clause reports full agreement — a
scope trap for anyone reading the row header as the status of every member.

---

## 8. MULTINOMIAL

No numeric Excel comparison figure exists. The only evidence is a 2026-03-15 COM spot probe pinning
domain/truncation:

> `Native Excel COM spot probes on `2026-03-15` pinned the truncation and domain lanes for the`
> `combinatorics batch, including `COMBINA(0,0) -> 1`, `COMBINA(0,1) -> #NUM!`, and`
> `MULTINOMIAL(0,0) -> 1`.`
— `docs/function-lane/FUNCTION_LANE_EVIDENCE_ID_REGISTRY.md:49` (`W16-BATCH8-COMBINATORICS-20260315`)

Worth flagging for the substrate story: MULTINOMIAL carries its **own private Lanczos lgamma**
(`MULTINOMIAL_LANCZOS_G`/`MULTINOMIAL_LANCZOS_COEFFS`,
`crates/oxfunc_core/src/functions/multinomial.rs:41-42`, used at `:110-119`), i.e. it sits on the
same class of substrate as the GAMMA/COMBIN wall — yet it appears in **no** discrepancy row and has
no numeric measurement. That is an evidence gap, not a clean result.

**Verdict: no-figure-exists on the numeric-bits axis; domain/admission probes only.**

---

## 9. PERMUT

> `PERMUT(n,k)` is the **ascending legacy x87 spill-loop product**: … Unique surviving candidate`
> `out of 6 stagings (strict/spill/extended × forward/reverse) over 402 live witnesses; production`
> `kernel (`permut_fn::permut_kernel`) verified **702/702 bit-exact** across discovery + fresh`
> `held-out sweeps (build 20131).`
— `docs/function-lane/W109_PERMUT_COMBIN_FINDINGS_20260711.md:5-9`

Per-surface, production-vs-Excel, 702 rows, **build restated inline (20131)**, held-out component
present ("discovery + fresh held-out sweeps") but the source does **not split** how many of the 702
were held out. **[RE-MEASURED]** `PERMUT.jsonl` holds exactly **702** cached rows — the claimed total
equals the entire cached corpus, so the "402 live witnesses" identification corpus is a subset of
the 702, and the held-out portion is at most 300.

Restated in the catalog: `PERMUT resolved out (W109 2026-07-11: ascending x87 spill-loop product,
`702/702` live rows …)` — `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:127`.

---

## 10. PERMUTATIONA

No numeric comparison figure. Two records, both kind/error:

> `### CLASS-A5: PERMUTATIONA overflow not mapped to #NUM!` … `- **Witness**:`
> `=PERMUTATIONA(163, 150)` local `+Inf`, Excel `#NUM!`.`
— `docs/bugs/streams/BUG-FUNC-027_broad_scalar_invocation_space_findings.md:99-103`
(landed and verified on build 20026, `:395`)

and XMD-008's `FINITE` family membership (`docs/EXCEL_MATH_DEVIATION_CATALOG.md:182`). Plus a
2026-03-15 domain probe: `PERMUTATIONA(0,0) -> 1`, `PERMUTATIONA(0,1) -> 0`
(`docs/function-lane/FUNCTION_LANE_EVIDENCE_ID_REGISTRY.md:64`).

**Verdict: no-figure-exists on the numeric-bits axis; single-witness error-code reproduction only.**

---

## 11. SQRTPI

Two Excel-math-deviation entries, and one of them **is counted**:

> `- **OxFunc reproduction:** `sqrtpi_kernel` uses `(n·π).powf(0.5)` (Rust's `powf` reproduces`
> `  Excel's `pow` bit-for-bit: 30/30 sampled inputs).`
— `docs/EXCEL_MATH_DEVIATION_CATALOG.md:110-111` (XMD-003)

Build **20026**, evidence `.tmp/sqrtpi-broad-oracle.ps1 + sqrtpi-boundary-oracle.ps1`
(`:112-113`). Held-out: not stated.

**Ambiguity I will not resolve by guessing:** the sentence measures *"Rust's `powf` reproduces
Excel's `pow`"*, which could be a `pow`-vs-`powf` race rather than 30 SQRTPI worksheet calls. It sits
inside the SQRTPI entry, with SQRTPI-named oracle scripts, so the natural reading is 30 SQRTPI
inputs — but the subject of the clause is `pow`. Label: **ambiguous-in-source, leaning per-surface.**

XMD-002 is the uncounted companion (`#NUM!` once `n·π` overflows, `:79-92`), an admission-axis
witness pair.

The same entry also states the *direction* of the deviation, which matters for any "matched Excel"
label: at `n·π == f64::MAX`, correctly-rounded `sqrt` is the **more** accurate value and Excel's
`pow` is 1 ULP high (`:104-107`). OxFunc matches Excel, i.e. matches the less accurate party, by
policy.

---

## 12. Comparison against FOUNDATION.md (opened only after §§1-11 were written)

Sources read: FOUNDATION.md §2.5 attribution table + per-surface count table (`:183-297`), §3.2
warrant membership (`:444-516`), §3.4 axis membership (`:543-608`), §3.5 cross-tab (`:609-626`),
§3.6 counters (`:627-665`), and `FOUNDATION-assignment.json` entries for all 11 surfaces.

What FOUNDATION currently records for this slice:

| surface | warrant | numeric | count carried | attribution |
|---|---|---|---|---|
| GAMMALN | W5 | N1 | 316/400 per-surface, held_out=true | measured-for-this-surface |
| GAMMALN.PRECISE | W3 | N1 | null/null, group | **alias-sibling-inherited** |
| GAMMA | W4 | N1 | 0/79 per-surface, held_out=false | measured-for-this-surface |
| COMBIN | W3 | N1 | **null/null**, group of 5 | **named-but-not-measured** |
| COMBINA | W3 | N1 | null/null, group of 5 | named-but-not-measured |
| FACT | W3 | N4 | none | — (XMD-008) |
| FACTDOUBLE | W4 | N3 | 7/7 per-surface | measured-for-this-surface |
| MULTINOMIAL | W4 | N8 | 46/47 group of 14, structural | measured-for-this-surface |
| PERMUT | W5 | N5 | 702/702 per-surface, held_out=true | measured-for-this-surface |
| PERMUTATIONA | W3 | N4 | none | — (XMD-008) |
| SQRTPI | W3 | N4 | **none** | — (XMD-002/003) |

### Disagreements

**D1 — wrong-attribution (severe). FOUNDATION §2.5 line: "20 | `GAMMALN.PRECISE` |
`alias-sibling-inherited` | the 316/400 gate is GAMMALN's".** The inheritance runs the other way.
All 400 held2 rows are GAMMALN.PRECISE captures; none is a GAMMALN capture. GAMMALN's own fresh
per-surface gate is b32 (549/1,200 + 123/400), which FOUNDATION does not carry at all.
Consequences: GAMMALN.PRECISE is under-warranted (W3, `passed:null`) when it holds a 400-row
held-out live capture and a 14,262-row cached shard; GAMMALN's carried figure is borrowed from its
sibling without saying so. The harvest's own C6 record 20 *names* the two shards and their sizes
("GAMMALN.jsonl 4,450 + GAMMALN.PRECISE.jsonl 14,262"), so the information was in hand and the
inference went the wrong way.

**D2 — stale figure (wrong-figure). GAMMALN 316/400 = 79.0%.** The shipped kernel scores 314/400
(78.5%) on that gate and 851/1,200 on the round-2 gate. `316` belongs to the superseded 223cfa5
landing (gn2/spill B2); lane 4 re-landed B2 continuous with LM coefficients and traded 2 held2 rows
for +31 fresh b32 rows. OxFunc publishes the corrected number at
`W109_CAMPAIGN_RESUME_20260718.md:34`; C6 record 20 saw it and demoted it to "a companion figure …
for the same gate class". It is not a companion — it is the current one.

**D3 — inherited wrong-attribution (severe, and it is upstream's fault first).** FOUNDATION's
GAMMALN record carries the catalog clause `GAMMALN/GAMMALN.PRECISE 0/79 (worst 1,370) → held-out
316/400`, i.e. it inherits OxFunc's own conflation of GAMMA's baseline with GAMMALN's. C6 record 20's
`what_the_rows_are` states it explicitly: *"up from 0/79 exact (worst 1,370 ULP) before"*. That
"before" figure is GAMMA's. FOUNDATION simultaneously and correctly assigns `0/79` to GAMMA under
record 21 — so the same 79 rows are attributed to two different surfaces in the same document. Only
one can be right, and it is GAMMA (reproduced numerically: 0/79 worst −1,370; pre-landing GAMMALN
was 16/79 worst −19,943). GAMMALN's real pre-landing baseline is unpublished anywhere.

**D4 — underclaim. COMBIN carried as `named-but-not-measured`, `passed:null,total:null`.** A
per-surface production figure exists: `6/16`, Excel vs OxFunc's multiply-first kernel, on 16
design-for-divergence pairs (`docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:127`). FOUNDATION's own
justification for the override — *"the record's COMBIN content is 6/16 and 'best 82/505'
refutations"* — treats `6/16` as a refutation rather than as a measurement, but `(acc*(n-k+i))/i` is
verbatim the shipped `combinations_of_int`. `82/505` is indeed a candidate score and correctly
excluded. Correct handling: one per-surface production count of 6/16 (16 rows, not held out,
adversarial corpus) **and** a separate candidate score of 82/505.

**D5 — underclaim. SQRTPI carried with no count.** XMD-003 publishes `30/30`
(`docs/EXCEL_MATH_DEVIATION_CATALOG.md:110-111`, build 20026). FOUNDATION puts SQRTPI at W3 /
N4 "witness exists, no count". The count exists; its *subject* is what is ambiguous (`pow` vs SQRTPI
calls). The honest record is a count with `measured_on_this_surface: ambiguous-in-source`, not the
absence of a count.

**D6 — cosmetic / modelling. FACT and PERMUTATIONA at N4 on the numeric-bits axis.** Their only
evidence (XMD-008) is an error-code convention — `+Inf → #NUM!` — which is admission behaviour, not
numeric bits. FOUNDATION's §3.4 rule ("the Excel-math-deviation catalogue is a numeric register, so
S1–S4 cannot arise") forces this placement, so it is a declared modelling choice rather than a
mis-reading. Flagging it because the rendered label will read as numeric-bits evidence where none
exists.

**D7 — cosmetic. GAMMA's `0/79` presented without its corpus caveat.** Correct as published, but a
12,312-row GAMMA shard exists on which production is 3.33% exact, and no record scores it. "0 of 79"
is true and should not be generalised to 0%.

### Agreements worth recording

- GAMMA at W4 with an open measured divergence, and FOUNDATION §3.5's own remark *"GAMMA is W4 on
  0/79. That is why warrant may never be read as quality"* — correct, and now confirmed to be
  **current**, not stale: the landed GAMMALN kernel does not touch `gamma_kernel`.
- PERMUT 702/702 per-surface with a held-out component and the build restated inline — correct. The
  unstated split (how many of 702 were held out) is a real limit; the cache shows 702 total, so
  ≤300 held out.
- FACTDOUBLE 7/7 per-surface, N3 — correct; the "spot check, not a sweep" caveat in C6 record 14 is
  the right one.
- MULTINOMIAL W4 / N8 / S6 on a 14-way structural group total — correct, and the "40 entries at W4
  with numeric state N8" caveat covers it. Add that MULTINOMIAL carries its own Lanczos lgamma and
  so is silently exposed to the same substrate wall with zero numeric evidence.
- COMBINA `named-but-not-measured` for a *pass count* — correct. Its content is one truth-witness
  and a designed-but-unexecuted reduction recipe.

---

## 13. Reproduction notes

Scripts live in
`C:/Users/GovertvanDrimmelen/AppData/Local/Temp/claude/C--Work-DnaCalc-ExcelFunctionsHandbook/89f0ac21-1f61-47b9-b4f5-113a2e0f9f56/scratchpad/`
(`score.py`, `score2.py`, `score3.py`). Nothing was written under `C:/Work/DnaCalc/OxFunc`.

Replica fidelity evidence, so a third party can judge the re-measurements:

- The landed-kernel replica reproduces agentS's held2 zone counts exactly except B2 (composed 18/21,
  b1 8/25, b4lo 102/129, stirl8 144/145 — all identical), and reproduces both lane-4 b32 band counts
  exactly (123/400, 549/1,200). B2 differs by exactly the band lane 4 re-landed, in the expected
  direction (44/80 → 42/80).
- The GAMMA replica reproduces the published `0/79`, `1,370` and `810k` figures simultaneously from
  one corpus, which is stronger than matching any one of them.
- Excel-side sanity: Excel's own GAMMALN is within 2.03 ULP of `mpmath.loggamma` on the 79-row
  corpus, and Excel's own GAMMA is within 54.72 ULP of `mpmath.gamma` on the 79 positives — which is
  what makes the 1,370-ULP figure attributable to GAMMA and unattributable to GAMMALN.
