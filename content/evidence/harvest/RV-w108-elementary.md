# RV — w108-elementary: independent re-derivation of measurement attributions

Slice: `EXP`, `LN`, `LOG10`, `LOG`, `POWER`, `OP_POWER` (`^`), the trig six (`SIN`, `COS`, `TAN`,
`SEC`, `CSC`, `COT`), and `SINH`, `COSH`, `COTH`, `ASINH`, `ATANH`, `ACOTH`, `ATAN2`, `DEGREES`.

Method, in order actually executed:

1. Read the primary OxFunc sources cold — the W108 workset, the discrepancy catalog's W108
   paragraph and G4 rows, the POWER spec, BUG-FUNC-042, the W109 trig and ATANH identification
   notes, the math-deviation catalog, the fix learning log, and the `excel_numeric` sources
   (`mod.rs`, `x87.rs`, `x87_excel_ground_truth.tsv`, `cot.rs`/`csc.rs`/`sec.rs`, `acoth.rs`,
   `asinh.rs`, `atan2.rs`, `sinh.rs`, `cosh.rs`, `coth.rs`, `degrees.rs`).
2. Wrote down, per surface, the figure and the sentence that carries it, with path:line.
3. Only then opened `FOUNDATION.md` §2.5 / §3.2 / §3.4 / §3.6, and — where a disagreement
   appeared — the harvest files `C6-verification-record.json`, `C5-identifications.json`,
   `C3-excel-math-deviations.json` to locate at which layer the divergence was introduced.

Everything below cites `C:/Work/DnaCalc/OxFunc` (read-only) unless marked otherwise.

---

## PART 1 — Independent derivation, surface by surface

### 1.1 The W108-resolved paragraph, read literally

`docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:77-84`:

> "W108 resolved (bit-exact via the x87 backend, removed from tracking): `EXP`, `LN`, `LOG10`,
> `LOG(x, base)`, and `POWER` — 64-bit Excel computes these with the legacy x87 CRT
> transcendental chain (`87tran.asm`, CW `0x133F`), reproduced bit-for-bit by
> [`crate::excel_numeric::x87`] on the reference x86-64 host. `POWER` (BUG-FUNC-042, signed
> off) is the fractional-path `exp(y·ln x)` with the `y<0` reciprocal staging and the
> `|y|==0.5→sqrt` special case (715/715 live rows). `EXP`/`LN`/`LOG10`/`LOG` were never catalog
> rows (W108-A research findings)."

Two things this paragraph does NOT do:

- It gives **one** figure — `715/715` — and attaches it to **POWER only**. There is no figure for
  EXP, LN, LOG10 or LOG in this paragraph. Anyone inheriting "bit-exact via the x87 backend"
  as a per-surface count for the four log/exp surfaces would be inventing it here.
- It names no Excel build. Build `20131` appears at `:13` (a *previous-reconcile* line about the
  trig row) and at `:4-12`, not on or near the 715 figure.

**The "never catalog rows" claim is not literally true for EXP.** The earliest committed version
of this same catalog (`git show fc24e93:docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md`, G1 section)
carries the row:

> "| EXP (+ DEGREES/RADIANS/FACT/FACTDOUBLE audit) | overflow → `+Inf` vs Excel `#NUM!`; same
> `finite_or_num` pattern as the fixed SINH/COSH | STR | M1 | `oxf-vgxs` |"

So EXP *did* hold a catalog row — a **structural** one (G1, error-domain) — and `LOG` held a G2
coercion/array-lift row in the same version. What is true is that none of the four ever held a
**numeric-exactness** row. The current sentence is an unqualified over-generalisation of a
correct narrower fact. This matters for the slice because it is exactly the sentence a reviewer
would quote to justify "no divergence was ever measured on EXP".

### 1.2 EXP, LN, LOG10, LOG — what the evidence actually is

The only per-surface figures anywhere in the primary sources are in the W108 Phase-A section.

`docs/worksets/W108_EXCEL_NUMERIC_CORE_AND_FINANCIAL_POWER_EXACTNESS.md:252-254`:

> "- Validation: 249/249 in-crate corpus (`x87_excel_ground_truth.tsv`) + a fresh 396-row
> live Excel sweep (`x87lab`): EXP 24/24, LN 19/19, LOG10 120/120, LOG 218/218 bit-exact
> (excl. one subnormal-domain edge where Excel flushes `5e-324` to 0 → `#NUM!`)."

Independent findings from that one sentence:

- **Per-surface**: EXP `24/24`, LN `19/19`, LOG10 `120/120`, LOG `218/218`. These are genuinely
  per-surface — the sentence names each surface with its own numerator/denominator.
- **The four figures sum to 381, not 396.** `24+19+120+218 = 381`. The sentence calls the sweep
  "396-row". The remaining 15 rows are unaccounted for in the source. This is a source ambiguity,
  not a transcription slip: the per-surface split does not exhaust the corpus it is drawn from.
- **Held-out wording**: the exact word used is **"fresh"** ("a fresh 396-row live Excel sweep").
  The model had already been fixed by the 294-row out-of-repo reverse-engineering pass
  (`:226`, `:237`: "294 live-Excel rows over 3 adversarial rounds", "reference impl 294/294 live
  Excel"), so the 396-row sweep is held out relative to the fit. The source never uses the phrase
  "held out" for it.
- **The 249-row in-crate corpus is EXP and LN ONLY.** Verified by reading the file, not the prose:
  `crates/oxfunc_core/src/excel_numeric/x87_excel_ground_truth.tsv` has 249 lines, and the op
  column is 136 `EXP` + 113 `LN` — zero LOG10, LOG or POWER rows. `x87.rs:14-16` states this in
  prose: "it reproduces live Excel **bit-for-bit on 249/249** harvested `EXP`/`LN` witnesses".
  So `249/249` is a **group total over EXP+LN**, and it is **instrument validation** of the x87
  backend module, not a worksheet-surface pass rate. Attributing `249/249` to LOG10, LOG or POWER
  would be wrong; attributing it per-surface to EXP or LN would also be wrong.
- **`294/294`** (`:237`, and re-stated `:329` "EXP/LN x87 reference: `294/294` exact") is the
  **out-of-repo reference implementation** score over EXP+LN — again a group total, and a
  research-instrument measurement of a harness that is not in the repo.
- **LOG's evidence is a separate 218-row sweep with an identification content**, not just a pass
  rate. `:249-251`: "**`LOG(x, base)` = `ln(x)/ln(base)` for EVERY base** (dropped the wrong
  base-2/base-10 special-casing). Confirmed by a 218-row live sweep, incl.
  `LOG(1000,10)=2.9999999999999996` (Excel's own imprecision) while dedicated `LOG10(1000)=3` —
  genuinely different paths." So LOG's 218/218 doubles as an Excel-vs-mathematical-truth
  observation (Excel is imprecise at `LOG(1000,10)`).
- **Build near the figure**: the Phase-A section header is `## 10. W108 Phase A (2026-07-04)`;
  build 20131 is stated at `:22` ("live Excel 16.0 b20131, 64-bit") for the *2026-07-03*
  investigation, far above. The Phase-A validation line itself does not restate a build. The
  learning log restatement (`docs/OXFUNC_FIX_LEARNING_LOG.md:117-119`, "validated 249/249 + a
  fresh 396-row live-Excel sweep") also names no build.
- **CPU-scoped**: `:236-237` "The `F2XM1`/`FYL2X` last bit is CPU microcode → on the hardest
  ~1-in-30 rows, parity is a host-CPU property (validated: AMD Zen2…)". Every one of these four
  surfaces inherits that caveat.

**A source contradiction that touches LOG.** `docs/EXCEL_MATH_DEVIATION_CATALOG.md:262-265`
(the "Inverse class & pending candidates" section) still says:

> "- `LOG(x, 10)` / `LOG(x, 2)` — Excel uses dedicated `log10`/`log2` (more accurate than naive
> `ln(x)/ln(base)`); OxFunc matches by using `log10`/`log2` directly. (Already matched — Excel is
> the accurate party.)"

That is the exact claim the 218-row W108 sweep **refuted** ("dropped the wrong base-2/base-10
special-casing"). The deviation catalog was not updated. Two OxFunc documents contradict each
other about LOG's algorithm; the workset is the later and the empirically-backed one.

### 1.3 POWER — the 715 rows, and how much of it is held out

`docs/bugs/streams/BUG-FUNC-042_power_fractional_exponent_exp_of_y_lnx.md:10-11`:

> "`POWER` is now bit-exact (715/715 live rows: 315 reverse-engineering ground truth +
> 400 fresh confirmation)."

Build restated on the same page at `:6`: "Reproduced on: live Excel 16.0 build 20131, 64-bit,
AMD Zen2, Value2 cell-ref plumbing." This is the one figure in the slice whose build, bitness,
CPU and plumbing are all restated adjacent to it.

`docs/EXCEL_POWER_SPEC_AND_TEST_CASES.md:24-25` says the same, and `:235-236` repeats it.

**The held-out question, answered from the source and not from the summary.** The spec document
splits the two corpora by role:

- `:86` — "This model scored **315/315** across the reverse-engineering ground truth." The 315
  rows are the corpus the `y<0` reciprocal staging (fix **a**) was reverse-engineered against.
  They are the repair's own target.
- `:88-89` — "**(b) exponent `0.5` is `sqrt`.** A fresh confirmation sweep found the one remaining
  class: Excel evaluates `POWER(x, 0.5)` as the **correctly-rounded hardware `sqrt(x)`**…"
- `:94-95` — "With (a)+(b) the full algorithm reproduces live Excel **400/400** on a fresh sweep
  spanning both signs, negative-base roots, integer, subnormal, and error rows."

So the 400-row sweep is where fix **(b)** was **discovered** and then **also** where it was
scored. For the final `(a)+(b)` algorithm — the algorithm the `715/715` figure describes —
**neither half of the corpus is cleanly held out**: 315 is (a)'s fit target and 400 is (b)'s.
"400 held out" is true only of the intermediate model. `BUG-FUNC-042:17-19` states the discovery
provenance explicitly: the `0.5→sqrt` case was "found by an OxFunc live sweep the reference
missed (it only tested `0.5` on negative bases)".

**Scope caveats that belong to the figure**, from `EXCEL_POWER_SPEC_AND_TEST_CASES.md:97-98`:
"The only residual caveat is the shared x87 `exp`/`ln` per-CPU-family microcode on the hardest
~1-in-2000 general-fractional inputs (same as `EXP`/`LN`); validated on AMD Zen2." And `:227-229`
— the `0.5→sqrt` and integer paths are CPU-independent, the general fractional path is not.

**One half of the corpus is not reconstructible.** `BUG-FUNC-042:16` cites the 315 rows to
`C:/Temp/ExcelExpFunction/POWER_REPORT.md` — outside the repo. The in-repo run directory
`smart-fuzzer/runs/w108-power-phaseD/` contains only `README.md` + a 3.3 KB `power_witnesses.tsv`
and its README is still headed `Status: open_puzzle` with the 220-row bake-off — it is the
*pre*-resolution artifact, not the 715-row sign-off corpus.

### 1.4 OP_POWER (`^`) — measured, or inherited?

**Inherited. No `^` pass rate exists anywhere in the primary sources.**

- `EXCEL_POWER_SPEC_AND_TEST_CASES.md:13` — "Excel's `POWER(x, y)` (and the `^` operator) is
  **not** a single library `pow`." The operator is named in the *claim*.
- But the corpus is POWER worksheet calls: `:220-221` — "Read `=POWER(A1, B1)` back as its 64-bit
  pattern." Every test-case table in §5A–5E is written as `POWER(...)`. There is no `^` column
  and no `^` row count.
- `BUG-FUNC-042:30-31` — "`POWER(x, y)` (and `x ^ y`) for a **fractional exponent with a positive
  base** currently calls `f64::powf` (UCRT)." Again a shared-kernel statement, not a measurement.
- The mechanism of the inheritance is a shared kernel, confirmed in code:
  `crates/oxfunc_core/src/functions/surface_dispatch.rs:2934` — `FUNC_ID_OP_POWER => power_kernel(lhs, rhs)`;
  and `surface_dispatch_unary_numeric_spec_generator.rs:346` — "OP_POWER and POWER both bind
  `power_kernel`". The only in-repo POWER/OP_POWER joint check is a unit test over a single
  error row (`surface_dispatch.rs:3725-3726`, `for function_id in [FUNC_ID_OP_POWER, FUNC_ID_POWER]`
  asserting `0^0 → #NUM!`).
- The W108 plan *intended* to measure the operator — `W108_…:102-103`, "**B3 POWER/operator**:
  `POWER` and `^` over integer/fractional/negative exponent and base grids" — but no `w108-b3`
  run exists (`ls smart-fuzzer/runs/` shows `w108-b2-financial`, `w108-power-phaseD`,
  `w108a-elementary-cr-vs-svml`, `w108a-reference-hunt`; no b3). **Planned, not executed.**
- `^` *was* probed live, once, for a different question: `W108_…:36` uses "`=A1^2-B1`" as one of
  the discriminators proving Excel's arithmetic is pure SSE2. That is an instrument/discriminator
  probe at integer exponent 2, not a conformance count.
- `^` also holds genuine *witness-level* Excel-departure evidence of its own:
  `EXCEL_MATH_DEVIATION_CATALOG.md:117` (XMD-004, "`POWER(x, n)` / the `^` operator", integer
  exponents via repeated multiplication), `:182-184` (XMD-008, "the overflow arm of `POWER`/`^`"),
  `:204` (XMD-009, "`POWER(x, p)` and `^`"). All three cite live Excel **build 20026** — a
  different build from POWER's 20131 sign-off — and **none of them publishes a count**.

### 1.5 The trig six — is 1020/1044 per-surface or a group total?

**Per-surface. The source says so with the word "each".**

`docs/function-lane/W109_TRIG_IDENTIFICATION_20260711.md:45-49`, the whole Sign-off block:

> "- Per-function unique survivors over the live rounds; validation sweeps
> (560 discovery + 460 held-out each): SIN/TAN/COT/CSC/SEC **1020/1020**,
> COS **1044/1044** (incl. the threshold ladder), max ULP 0.
> - Production-kernel replay over every deduplicated answered witness:
> **5425/5425 bit-exact** (`verify_trig_promotion`)."

Independent readings:

- `560 + 460 = 1020` and the modifier is "**each**". So the sweeps are per-function, the held-out
  half is **460 rows per surface**, and `1020/1020` is a per-surface figure that happens to be
  identical across five surfaces. COS is `1044/1044` = the same 1020 plus 24 threshold-ladder
  rows ("incl. the threshold ladder"). Arithmetic is consistent.
- `5425/5425` is a **group total across all six**, and it is a different *subject*: the
  **production-kernel replay**, versus the 1020/1044 which are the **surviving candidate graphs**
  validated during identification. `1020×5 + 1044 = 6144`, deduplicated down to 5425 — consistent
  with "every deduplicated answered witness".
- The catalog's restatement collapses the distinction:
  `OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:137-139` — "Validated `5425/5425` live rows (all six
  functions, incl. held-out sweeps and a bit-resolution COS threshold ladder)." Read alone, that
  sentence is a **group total** and gives no per-surface number at all. A reviewer starting from
  the catalog would conclude the trig six have only a group figure. They do not — the per-surface
  figures are in the identification note.
- Only three kernels exist; COT/CSC/SEC are reciprocals of the published primaries
  (`W109_TRIG_…:20-23`; `functions/cot.rs:31-38`, `csc.rs:29-37`, `sec.rs:29-37`, each
  `excel_x87_recip` of `excel_tan`/`excel_sin`/`excel_cos`). Nevertheless the identification note
  scores COT/CSC/SEC **individually** at 1020/1020, so they are measured surfaces, not inherited
  ones. This is the opposite of the OP_POWER situation and the distinction is load-bearing.
- **Build near the figure**: the note states "live Excel 16.0 build 20131, x86-64" in the
  section header at `:9`, two sections above the Sign-off block; the Sign-off block itself does
  not restate it. The code doc-comment does:
  `crates/oxfunc_core/src/excel_numeric/mod.rs:426-427` — "Excel `SIN` — bit-exact to 64-bit
  Excel on `x86_64` (W109 G4-01, validated 1020/1020 live rows incl. held-out, build 20131)".
  `mod.rs:454` gives COS "1044/1044 incl. the … threshold ladder"; `mod.rs:493` gives TAN
  "1020/1020". COT/CSC/SEC carry no figure in their own source files.
- **CPU-scoped**: the reduction uses `FPREM1`/`FSIN`/`FPTAN` microcode, so the same host-CPU
  caveat applies. The note does not restate it; `x87.rs:20-27` and the harvest do.
- The former catalog row for this lane, before sign-off, covered **five** functions, not six:
  `git show fc24e93:…CATALOG.md` G4 — "| TAN, SIN, COT, SEC, CSC | moderate-large
  argument-reduction drift (Cody-Waite vs extended-π; up to `~3.3E12` ULP) |". COS was added later.

### 1.6 ATANH — the figure the catalog publishes is not a surface total

`docs/function-lane/W109_ATANH_IDENTIFICATION_20260712.md` publishes three numbers:

- `:24` — "ln via the x87 CRT chain. **163/163 bit-exact.** PROMOTED into atanh.rs." (region C,
  `|x| >= ~1.25e-4`)
- `:27-28` — "extended temporaries with a SINGLE final store — **175/175 bit-exact** on every live
  region-B row." (region B, `|x| <= ~9.0e-5`)
- `:56-60` — "Piecewise (x87 pair below T, ratio-log above): best `344/350` at
  `T≈1.0e-4..1.05e-4`, the 6 residuals all in the switch band. … Full corpus `344/350`; the only
  open rows are the 6 band rows at `+1` ULP."

So the **surface-level figure for ATANH is `344/350`**. `163/163` and `175/175` are *per-region
sub-figures* of that same surface, not independent measurements, and `163+175 = 338` is the count
of rows in the two promoted regions — a subtotal, never published as a score against a
denominator.

Corpus provenance, `:3-4`: "Corpora: G4-hyp (107) + G4-02 band (77) + gap (146) + switch (42) =
~350 distinct rows." Note the source's own arithmetic: `107+77+146+42 = 372`, and it writes
"~350" with a tilde. The denominator is approximate in the source. The word "held out" does not
appear anywhere in the file; the corpus is the identification's own discovery corpus (a
multi-agent racing lane that overturned the prior scoping). Build: `:3` "Live oracle Excel 16.0
build 20131" — stated in the file header, not restated at `:56`.

Contrast the earlier, reverted candidate for the same surface, so the numbers are not confused:
`W108_…:355-359` — "`ATANH(x)` decomposes bit-for-bit as `0.5 * LN((1+x)/(1-x))` on the two
bounded witnesses, but the required expanded sweep rejected that as a global kernel: `297/368`
exact and `71` regressions". `297/368` is a **refuted research candidate**, not production.

### 1.7 ACOTH

`docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md:126`:

> "**W109 (2026-07-12): two-regime x87 form PROMOTED — strict improvement (35→53/56, 0
> regressions, +19 rows).**"

Restated in code, `crates/oxfunc_core/src/functions/acoth.rs:44-46`: "Strictly dominates the prior
platform-ln1p form (0 regressions, +19 rows on the 56-row live corpus; 53/56). Residual: 3
pair-branch rows (±5, +8.1) at +-1..2 ULP and the exact switch double remain open".

Per-surface, production, denominator 56, three open residual rows. No held-out language; the
56-row corpus is the identification's own corpus (`W109_SWEEP_20260712.md:28` — "107+57 fresh
rows" for ATANH+ACOTH; the word is again "fresh", and note the 57 vs 56 slip between documents).
Build not restated on the catalog line; `W109_SWEEP_20260712.md:3-4` says "Live oracle build
20131" for the sweep, while `acoth.rs:87-90` cites build **b20026** for a different (near-1)
witness row. Row status is **M3 fixed-unsigned**, i.e. not signed off.

### 1.8 SINH, COSH, COTH, DEGREES — no numeric figure exists

Searched: all `docs/*.md`, `docs/function-lane/`, `docs/bugs/streams/`, and each surface's kernel
file. What exists for these four is **structural / error-domain** evidence only, with witnesses
and no counts:

- `docs/bugs/streams/BUG-FUNC-027_broad_scalar_invocation_space_findings.md:79-85` — "CLASS-A3:
  SINH / COSH overflow does not map to #NUM! … **Witness**: `=SINH(-326648.33)` local `-Inf`,
  Excel `#NUM!`; `=COSH(-24230)` local `+Inf`, Excel `#NUM!`."
- `docs/EXCEL_MATH_DEVIATION_CATALOG.md:182-197` — XMD-008, functions "`EXP`, `SINH`, `COSH`,
  `FACT`, `FACTDOUBLE`, `DEGREES`, `PERMUTATIONA`"; evidence line `:196` — "live Excel 16.0 build
  20026; BUG-FUNC-027 CLASS-A3/A4/A5, bead `oxf-vgxs`, commit `b0b2419`." **No count.**
- COTH: `crates/oxfunc_core/src/functions/coth.rs:24-25` — "for large |n|, cosh/sinh = Inf/Inf =
  NaN; Excel saturates COTH to `sign(n)` i.e. `±1`. Verified live Excel 16.0 b20026:
  COTH(800)=1." One witness, no count.
- DEGREES: `crates/oxfunc_core/src/functions/degrees.rs:61` — "live Excel DEGREES(1E307)=#NUM!
  (overflow guard on surface)." One witness, no count.

None of the four has any numeric-bits comparison count. Their absence from a count table is
correct; what would be wrong is any label implying a numeric bit-exactness measurement.

### 1.9 ASINH — a witness-level boundary identification, no count

`docs/EXCEL_MATH_DEVIATION_CATALOG.md:61-78` (XMD-001) identifies the mechanism —
"Excel evaluates the literal `ln(x + √(x²+1))`; the `x²` (or `x²+1`) intermediate overflows
`f64` → `#NUM!`" — and its evidence line `:77-78` is "live Excel 16.0 build 20026
(`.tmp/asinh-sqrtpi-oracle2.ps1`); bead `oxf-7m1k`, commit `77431cf`." **No count.**
`crates/oxfunc_core/src/functions/asinh.rs:37-38` claims the numeric side qualitatively:
"Below the threshold Excel publishes the finite value sign(x) * ln(|x| + hypot(x, 1)) (bit-exact
on the disputed lanes where platform libm `asinh` differs by 1+ ULP)" — a bit-exactness claim
with **no denominator**. The historical 1-ULP divergence is on record at
`docs/function-lane/W53_NUMERIC_FORENSICS_20260326.md:23` ("`ASINH`: `1` ULP") against build not
stated in that line.

### 1.10 ATAN2 — a figure DOES exist, on the structural axis, on a different build

`docs/EXCEL_MATH_DEVIATION_CATALOG.md:178-179` (XMD-007 evidence line):

> "- **Evidence:** live Excel 16.0 build 20026 (18/18 bit-exact); BUG-FUNC-027 B3, commit
> `8dea9cd`. Confidence **high**."

This is the only surface in the "no-figure" tail of my slice that in fact carries a count. It is:
per-surface (ATAN2 alone), `18/18`, **build 20026 restated on the same line**, and it measures
the `y/x`-overflow → `#NUM!` **error-placement boundary** — i.e. structural admission, not the
numeric bits of the angle. `crates/oxfunc_core/src/functions/atan2.rs:50-53,130-136` pins the
same boundary rows. There is no numeric-exactness count for ATAN2 anywhere.

---

## PART 2 — Comparison against FOUNDATION.md (read only after Part 1 was written)

Read: `FOUNDATION.md` §2.5 (attribution table `:265-279`, per-surface count table `:282-296`),
§3.2 (`:444-494`), §3.4 (`:543-597`), §3.6 (`:627-664`), plus `:65` (A1-S8) and `:785` (OT-13)
and `:1184` (build guard T4(d)).

### 2.1 Where the foundation and I agree — and these are the non-trivial ones

**Trig six, per-surface, 460 held out each.** `FOUNDATION.md:284` — "| 0 | SIN/TAN/COT/CSC/SEC
1020/1020; COS 1044/1044 | numeric; `partial`, 460 held out per surface |". This is exactly what
`W109_TRIG_IDENTIFICATION_20260711.md:45-47` says, including the crucial "per surface". The
foundation did **not** fall into the trap the catalog's 5425 sentence sets. Confirmed correct.
§3.2 backs it: all six sit at W5 (`:493-494`) and the group total is preserved in the harvest
(`C6-verification-record.json`, record 0, `passed: 5425` with the per-function sweeps quoted in
`what_the_rows_are`). §3.7's "per-surface counts shadow group counts" rule is the right
disposition here.

**OP_POWER is inherited, not measured.** `FOUNDATION.md:267` — "| 1 | `OP_POWER` (`^`) |
`alias-sibling-inherited` | the 715 rows are POWER worksheet calls; no `^` split is published |".
That is precisely my finding in §1.4, including the reason. Consistent with `:580` (OP_POWER at
N4, "a witness exists, no count") and `:590` (S5 — its counted evidence is the operator-family
structural group, not POWER's 715). The apparent tension with OP_POWER remaining in W4 (`:490`)
resolves correctly: its W4 warrant is structural, its numeric state is N4. No disagreement.

**EXP/LN/LOG10/LOG per-surface figures.** `FOUNDATION.md:286` — "| 2 | EXP 24/24; LN 19/19;
LOG10 120/120; LOG 218/218 |" — matches `W108_…:252-254` verbatim, and the harvest
(`C6-verification-record.json`, record 2) even reconciles the 396→381 gap: "reported per-surface
as EXP 24/24, LN 19/19, LOG10 120/120, LOG 218/218 (sum 381)". Good.

**The 249-row corpus is scoped to EXP/LN.** `FOUNDATION.md:242` — "`x87_excel_ground_truth.tsv`
(249 rows, EXP/LN)". Independently verified against the file itself (136 EXP + 113 LN). Correct,
and correctly not attributed to LOG10/LOG/POWER.

**LOG's departure is source-derived, not a catalog XMD.** `FOUNDATION.md:579` puts LOG at N3
("matches Excel by reproducing a documented Excel departure"), and the harvest justifies it with
`C3-excel-math-deviations.json` entry `"SRC-LOG-001 (NOT a catalog XMD id -- source-derived;
contradicts the catalog's Inverse-class row)"`. That is the same contradiction I found
independently in §1.2, correctly labelled. Good.

**ACOTH 53/56.** `FOUNDATION.md:293` matches `acoth.rs:44-46` and catalog `:126`.

**SINH/COSH/COTH/DEGREES/ASINH have no count.** `FOUNDATION.md:478-484` places ASINH, COSH, COTH,
DEGREES, SINH at W3 ("an Excel comparison is on record; no row count was extracted") and `:580`
at N4 ("a witness exists, no count"). Matches §1.8–1.9 exactly.

### 2.2 Disagreement 1 — ATANH `338/344` is a figure that exists in no source

`FOUNDATION.md:293` — "| 26 | ATANH 338/344; ACOTH 53/56 | numeric; `false` |" — and the same
string is hard-wired into the build guard at `:1184`: "The §2.5 per-surface table is applied:
… ATANH 338/344, ACOTH 53/56 …".

The source publishes `344/350` (`W109_ATANH_IDENTIFICATION_20260712.md:56` and `:60`). `338` is
`163 + 175`, the row count of the two promoted regions — a subtotal that the source never uses as
a numerator, and `344` is the source's **numerator**, not its denominator. So the foundation has
composed a fraction out of a subtotal over a numerator: it understates the corpus by 6 rows and
converts a surface with 6 known open `+1` ULP residual rows into a surface with 6 residuals out of
a *different* denominator. Read off the page, "338 of 344" also implies a 98.3% figure where the
source's own is 98.3% of 350 — the ratio is coincidentally close, which is exactly why this kind
of error survives review.

This was introduced **at the foundation layer**: the harvest is right.
`C5-identifications.json` (ATANH entry, `current_production_score`) says: "344/350 on the full
~350-row corpus (region B 175/175 and region C 163/163, both promoted). The six residual rows are
all +1 ULP and all lie in the B-to-C transition band." `grep -l "338/344"` across the whole
dossier returns `FOUNDATION.md` and nothing else. Severity: **wrong-figure**, and load-bearing
because guard T4(d) will enforce it into every emitted record.

Secondary note on the same row: the foundation records ATANH `held_out: false`. The source uses no
held-out language at all, so `source-does-not-state` is the honest value. The harvest goes the
other way and asserts more than the source: "with 77 ATANH band rows banked as a held-out set; no
separate held-out score is quoted" — the source's `:3` only lists "G4-02 band (77)" as one of four
corpora and never calls it held out. The foundation's `false` is closer to the source than the
harvest's prose, but neither is the source's actual silence.

### 2.3 Disagreement 2 — POWER's "400 held out" overclaims, because fix (b) was fitted to those 400 rows

`FOUNDATION.md:285` — "| 1 | POWER 715/715 | numeric; `partial`, 400 held out |"; `:65` — "POWER
renders 715 of which 400 were held out"; `:785` (overclaim test OT-13) — "`W5` '715 counted rows
of which 400 were held out'".

The source is explicit that the 400-row "fresh confirmation" sweep is where the second of the two
corrections was **found**: `EXCEL_POWER_SPEC_AND_TEST_CASES.md:88-89` — "**(b) exponent `0.5` is
`sqrt`.** A fresh confirmation sweep found the one remaining class"; and `BUG-FUNC-042:17-19` —
"exponent `|y| == 0.5` is the correctly-rounded hardware **`sqrt`** … found by an OxFunc live
sweep the reference missed". The `400/400` at `:94-95` is then scored on that same sweep. For the
final `(a)+(b)` algorithm the 400 rows are the repair's own target, exactly as the 315 rows are
(a)'s. The honest reading is: **`held_out: partial` with `held_out_rows: null` and
`corpus_was_repair_target: true` for both halves**, plus a sentence saying which fix each half
scored.

Layer: introduced in the harvest and carried forward. `C6-verification-record.json` record 1 says
"The 400 fresh rows are the only held-out half" with `held_out: true`; the foundation inherits it.
Severity: **overclaim** — and it is the single held-out claim in this slice that a reader would
most reasonably rely on, since POWER is one of the 19 W5 entries whose label promises held-out
rows.

### 2.4 Disagreement 3 — ATAN2's 18/18 exists; the foundation records ATAN2 as having no count

`FOUNDATION.md:478-480` puts ATAN2 at **W3** (`why_no_count`) and `:580` at **N4** ("a witness
exists, no count"); ATAN2 appears in none of S5/S6/S7 (`:588-597`), so its structural axis falls
to S8, "no structural comparison record".

But `EXCEL_MATH_DEVIATION_CATALOG.md:178` publishes "live Excel 16.0 build 20026 (18/18
bit-exact)" for XMD-007, per-surface to ATAN2. Under the foundation's own §3.4 convention that
"the Excel-math-deviation catalogue [is a] numeric register, so those states cannot arise on the
structural axis" (`:567-568`), that count is routed to the numeric axis, where the correct state
is **N3** — "matches Excel by reproducing a documented Excel departure from exact mathematics
({n} counted rows, {axis}, Excel {builds}, {cpu})", n=18, builds=20026 — not N4. Correspondingly
ATAN2 should be W4, not W3.

Note the second-order problem this exposes, which I flag rather than resolve: `18/18` measures an
**error-placement boundary** (`#NUM!` when `y/x` overflows), which is structurally-shaped evidence
being carried on the numeric axis by a routing rule. Under the foundation's convention it becomes
a numeric-axis count with a *structural* meaning, and the label would then read "{n} counted rows"
next to numeric prose. The foundation forbids the words *bit*, *bit-exact*, *exact* in structural
labels (`:510`) precisely to prevent this confusion — but the source sentence itself says
"18/18 **bit-exact**" about an error-code boundary. **Ambiguous in source**, and the honest
disposition is to carry the count with an explicit note that what was counted is the boundary,
not the angle. Severity of the foundation's current state: **underclaim** (a real, per-surface,
build-restated figure is recorded as absent).

### 2.5 Disagreement 4 — EXP/LN/LOG10/LOG held-out state discards the source's word "fresh"

`FOUNDATION.md:286` records record 2 as held-out `source-does-not-state`.

The source's word is **"fresh"**: "249/249 in-crate corpus … + **a fresh 396-row live Excel
sweep**" (`W108_…:252-253`), repeated in the learning log (`OXFUNC_FIX_LEARNING_LOG.md:119`,
"validated 249/249 + a fresh 396-row live-Excel sweep"). The four per-surface figures come from
that fresh sweep, and the algorithm they score was already fully determined by the earlier
294-row reverse-engineering pass. Under the standing instruction to capture the exact word —
"held-out", "fresh", "never-probed", "b<N> gate" — this is a held-out signal that the foundation
dropped. Correct value: `held_out: true` (or `partial`), `held_out_rows: null` with the rendered
note that the source does not split, `corpus_was_repair_target: false`, and the word "fresh"
quoted. Severity: **underclaim**. Consequence: these four surfaces sit at W4 ("none held out",
`:452`, `:486-487`) when at least the log/exp evidence warrants the W5 label.

### 2.6 Disagreement 5 — the trig per-surface figures are candidate-survivor scores; only 5425 is the production replay

`FOUNDATION.md:284` carries `1020/1020` and `1044/1044` in the per-surface table with no
`measurement_subject` override, i.e. defaulting to `production-oxfunc`.

The source splits the two subjects in adjacent bullets: `1020`/`1044` are "**Per-function unique
survivors** over the live rounds; validation sweeps", whereas `5425/5425` is "**Production-kernel
replay** over every deduplicated answered witness (`verify_trig_promotion`)"
(`W109_TRIG_…:45-49`). The surviving candidate graph *is* what landed — `mod.rs:426-427` attaches
`1020/1020` to the production `excel_sin` — so the practical difference is nil. But under the
foundation's own `measurement_subject` enum the per-surface figures are candidate-validation
scores and the group figure is the production replay, and a record that flattens the two loses the
one distinction the enum exists to make. Severity: **cosmetic**, recorded because the same
flattening is a wrong-attribution elsewhere in the dossier.

### 2.7 Disagreement 6 — the W108 "never catalog rows" sentence is inherited without the correction

The foundation's §3.6 places LN, LOG, LOG10 in the 12 "numeric-clean, no open catalogue row, no
open defect stream" entries (`:649`, `:658-660`) and EXP in the 25 (`:662-663`, excluded from the
12 only for its Excel-math-deviation reproduction). Nothing in §2.5/§3.x records that EXP once
held a **structural** catalogue row (G1 overflow → `#NUM!`) and LOG a G2 coercion row, per
`git show fc24e93:docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md`. Both rows were resolved, so the
current state is right; what is missing is that the upstream sentence the foundation leans on
("were never catalog rows") is an over-generalisation. Severity: **cosmetic**, but worth a
retraction-style note on those records, since "never had a row" and "has no open row" are
different claims and the four surfaces in question are among the dossier's small set of clean
entries — the claim carries more weight there than anywhere else.

---

## Verdict

- The foundation gets right the two hardest questions in this slice: the trig six really are
  measured **per surface** (1020/1044, 460 held out each — not a group total), and OP_POWER really
  is **inherited** from POWER's shared kernel with no `^` figure in existence.
- One **wrong figure**: ATANH `338/344`. The source and the harvest both say `344/350`; the
  foundation composed the fraction from a region subtotal and a numerator, and wired it into a
  build guard.
- One **overclaim**: POWER's "400 held out". The source shows the 400-row sweep is where the
  `0.5→sqrt` correction was discovered, so for the final model no half of the 715 is cleanly
  held out.
- Two **underclaims**: ATAN2 has a real per-surface `18/18` on build 20026 that the foundation
  records as absent; and EXP/LN/LOG10/LOG's corpus is called "fresh" in the source, which the
  foundation reduces to `source-does-not-state`, costing those four surfaces the W5 label.
- Ambiguities that should be published as ambiguities, not resolved: the 396-row sweep whose
  per-surface figures sum to 381; ATANH's "~350" denominator whose corpus parts sum to 372;
  ATAN2's `18/18` described as "bit-exact" about an error-code boundary; and the 56-vs-57 row
  slip for ACOTH between `acoth.rs` and `W109_SWEEP_20260712.md`.
- One upstream sentence should not be repeated unqualified: "`EXP`/`LN`/`LOG10`/`LOG` were never
  catalog rows" — EXP and LOG did hold structural/coercion rows in the earliest catalog; neither
  ever held a numeric-exactness row.
