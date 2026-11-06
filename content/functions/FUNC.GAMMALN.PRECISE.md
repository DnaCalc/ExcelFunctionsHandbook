---
schema: efh.function-page/v1
function_id: FUNC.GAMMALN.PRECISE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0013
  - EV-MATH-0014
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.GammaLn_Precise method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.gammaln_precise"
    role: "documented description, the two documented failure conditions (worded as 'generates an error'), and the exp(GAMMALN(i)) = (i-1)! remark"
  - work: "Microsoft Learn — WorksheetFunction.GammaLn method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.gammaln"
    role: "the legacy surface, whose replacement banner names this function"
  - work: "Microsoft Support — GAMMALN.PRECISE function"
    locator: "https://support.microsoft.com/en-us/office/gammaln-precise-function-5cdfe601-4e1e-4189-9d74-241ef1caa599"
    role: "the worksheet-surface documentation page; not retrievable at curation time (the host refused the request)"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 6"
    locator: "6.1.40 ff — Stirling's asymptotic series for ln Γ"
    role: "the mathematics, shared with GAMMALN"
  - work: "W. J. Cody and K. E. Hillstrom, Chebyshev approximations for ln Γ (1967); SPECFUN ALGAMA/DLGAMA"
    locator: null
    role: "the minimax rational design family named in the upstream identification"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - The pair, stated precisely
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Notes
  - Page vocabulary
  - Sources
family: special_dist_family
role_in_family: >-
  The modern log-gamma spelling; a one-line delegation to the GAMMALN kernel in the reference
  engine, and the spelling through which both of the family's held-out tests were captured.
---

## What it computes

`GAMMALN.PRECISE(x)` returns the natural logarithm of the gamma function, ln Γ(x), for x > 0.

The mathematics is set out in full on the [GAMMALN](FUNC.GAMMALN.md) page — the integral
definition, the recurrence ln Γ(x+1) = ln x + ln Γ(x), strict convexity on (0, ∞), the two zeros
at x = 1 and x = 2, the single minimum near x ≈ 1.4616, and Stirling's asymptotic series — and is
not repeated here. Microsoft's description of this surface is word-for-word the description of
the legacy one: "Returns the natural logarithm of the gamma function, Γ(x)."

The interesting content of *this* page is not the mathematics. It is the pair relationship, and
what the word "PRECISE" in the name does and does not promise.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `x` | The value at which to evaluate ln Γ. Required. | — |

One argument; the reference engine declares an arity of exactly 1. The slot is numeric under
ordinary to-number coercion, and the reference engine's source records that this family accepts
logicals — `GAMMALN.PRECISE(TRUE)` gives 0, since Γ(1) = 1 — while the ERF/ERFC family in the
same module rejects them. That note is attributed upstream to an empirical Excel sweep and has
no documented counterpart.

## The pair, stated precisely

The `.PRECISE` suffix belongs to the Excel 2010 renaming of the statistical function set, and in
the general case the Handbook's rule holds: **a modern dotted name is not guaranteed to be the same
computation as its legacy counterpart, and proving identity requires evidence.** For this
particular pair, unusually, some of that evidence exists — upstream's, not the Handbook's — and
it is worth laying out with each claim's warrant attached.

1. **Documented supersession.** Microsoft's `GammaLn` page carries the replacement banner: the
   function "has been replaced with one or more new functions that may provide improved accuracy
   and whose names better reflect their usage", and it names `GammaLn_Precise`. Note the modal
   verb — *may* provide improved accuracy. The banner is boilerplate carried by every function
   in that 2010 renaming, and it promises nothing measurable.
2. **Observed identity, upstream, on one build.** `EV-MATH-0014` records an Excel-versus-Excel
   check in which both surfaces resolved to the same implementation and published the same
   results at every probed point. The record is explicit that this is an identity check and not
   a pass rate, and that the source publishes no numerator or denominator for it.
3. **Structural identity in the reference engine.** OxFunc's `gammaln_precise_kernel` is a
   one-line delegation to `gammaln_kernel`. The two surfaces cannot differ there by construction.
4. **The direction of inheritance runs toward the legacy name, not away from it.**
   `EV-MATH-0013` records that both of the family's held-out tests were captured through
   *this* spelling, so a figure measured on `GAMMALN.PRECISE` transfers to `GAMMALN` by that
   argument and not the reverse. That is the opposite of the usual assumption, in which the
   older, better-studied surface is the measured one.
5. **The supersession did not move the category.** Both surfaces are classified under
   **Statistical functions** in the catalogue projection. `GAMMALN` was *not* moved to
   **Compatibility**, unlike `GAMMADIST` and `GAMMAINV`. Recorded here as a
   documentation-versus-catalogue mismatch.

So the honest summary is: **documented as a replacement, observed identical once by upstream on
one named build, structurally identical in the reference engine, and agreeing at every shared
recorded input tried (FN-GAMMALN-001).** If the two surfaces are the same computation in every build
— which is what the
evidence so far suggests — then the "PRECISE" in the name describes an intention from 2010
rather than a current difference, and a reader choosing between them is choosing a spelling.

## Result and edge cases

Returns `Number`. The edge-case inventory is the one on the
[GAMMALN page](FUNC.GAMMALN.md#result-and-edge-cases): exact zeros at x = 1 and x = 2, the
−ln x behaviour as x → 0⁺, Stirling growth at large x, and `#NUM!` for x ≤ 0.

One item belongs here because it is an observation about the reference engine and it applies to
both spellings: the domain guard tests the **input** for finiteness, and nothing tests the
**output**, so at the top of the double range the projected battery records a non-finite number
escaping as a `Number` rather than an error. No Excel was involved in producing that battery.

## Errors

Microsoft's documentation for this surface is worded differently from the legacy one, and the
difference is worth noting rather than smoothing over:

| Condition | `GAMMALN.PRECISE` documentation | `GAMMALN` documentation |
|---|---|---|
| `x` is nonnumeric | "generates an error" | `#VALUE!` |
| `x` ≤ 0 | "generates an error" | `#NUM!` |

**The modern page does not name the error values; the legacy page does.** The two pages describe
the same two conditions, but only one of them tells you what lands in the cell. A reader writing
an `IFERROR`/`IFNA` guard needs the code, and the documentation for the *recommended* surface is
the one that omits it. Recorded as a documentation gap.

The reference engine returns the legacy page's codes for both surfaces, by construction, since
one delegates to the other. That is a fact about the reference engine.

## Relationships

- **[GAMMALN](FUNC.GAMMALN.md)** — the legacy spelling; the full pair analysis is above, and the
  numerical treatment lives on that page.
- **[GAMMA](FUNC.GAMMA.md)** — the value rather than its logarithm, in the same module and
  **not** on the same numeric path: upstream's landed log-gamma kernel is wired only to the two
  `GAMMALN` surfaces. `LN(GAMMA(x))` and `GAMMALN.PRECISE(x)` are different computations with
  different domains, and the first overflows where the second does not.
- **[GAMMA.DIST](FUNC.GAMMA.DIST.md), `BETA.DIST`, `POISSON.DIST`, `COMBIN`, `MULTINOMIAL`** —
  consumers of log-gamma, directly or through their normalising constants.
- **`ERF.PRECISE` / `ERFC.PRECISE`** — module siblings that carry the same `.PRECISE` suffix from
  the same renaming. Whether *those* pairs are identical computations is a separate question
  with its own record, and nothing about this pair transfers to them.

## Numerical notes

The algorithmic content — the three-region split (recurrence for small x, minimax rational in
the middle, Stirling above a seam), the unbounded relative error in the two narrow bands around
the zeros, and the identified structure with its threshold and seam — is set out on the
[GAMMALN page](FUNC.GAMMALN.md#numerical-notes).

Two points belong specifically here.

**"Precise" is a name, not a measurement.** Nothing in Microsoft's documentation quantifies the
improvement the 2010 renaming promised, and the banner's own wording is "may provide improved
accuracy". Where a `.PRECISE` surface has been checked against its legacy counterpart in this
domain, the result so far has been identity rather than improvement. A reader should not choose
this spelling expecting better answers; they should choose it because it is the current name.

**The shared-kernel blindness.** Because the reference engine implements one surface as a
delegation to the other, **it cannot exhibit a difference between them**. Any comparison run
against OxFunc reports identity by construction. The only thing that can settle whether Excel's
two surfaces agree is a probe that compares them in Excel, with no reference engine in the loop
— and upstream's identity check is exactly that, on one build. FN-GAMMALN-001 adds agreement at
every shared recorded input tried. A full sweep on more than one build is the cheapest useful work
available on this page.

## What has not been checked

Two records name `GAMMALN.PRECISE` as a subject.

`EV-MATH-0013` records the two held-out tests for the landed kernel and three caveats that
travel with them: the tests were held out from the **coefficient fits** and not from the
structural identification, so the threshold, the seam and the form were pinned on batteries
surrounding those rows; the arguments in both tests are `.PRECISE` captures, which is why the
inheritance direction runs from this surface toward `GAMMALN`; and OxFunc contradicts itself on
one of the figures, publishing two different values in the same file, with the weaker carried
forward. `EV-MATH-0014` records the Excel-versus-Excel alias identity on one named build. All
figures render mechanically beside this page; this prose deliberately does not restate them.

Note what is *not* here: `EV-MATH-0012`, the fresh never-probed corpus, names `GAMMALN` alone.
This surface has no fresh-corpus figure of its own.

What does not exist: any Handbook vector suite; any identification of the rationals below 3 (the
open item of FN-GAMMALN-001); any Handbook-side check of the alias identity beyond that note's
shared recorded inputs; any measurement on more than one Excel build; any concentrated measurement
in the two bands around the zeros, where the mathematics predicts the worst relative error.

Inputs I would probe first, and why:

1. **`GAMMALN.PRECISE` against `GAMMALN`, cell by cell, over a wide sweep including both zeros
   and the seam.** This is the highest-value probe on the page: it needs no external oracle, it
   tests the one claim the page's whole structure rests on, and either outcome is publishable.
   Agreement over a broad sweep strengthens the identity; a single disagreement overturns it and
   makes every inherited figure on both pages suspect.
2. **The same comparison on a second Excel build.** Upstream's identity is single-build, and
   build-scoped identity is the kind of fact that quietly stops being true.
3. **Dense sweeps in (1 ± 2⁻¹⁰) and (2 ± 2⁻¹⁰)** — the zeros, where relative error is unbounded
   by construction and where a uniform sweep contains almost no points.
4. **The seam neighbourhood near x = 8 and the threshold near x = 0.7**, from both sides, since
   those are the identified structural features and a seam is the strongest fingerprint an
   implementation has.
5. **`GAMMALN.PRECISE(TRUE)`** — the logical-coercion behaviour recorded in the reference
   engine's source but absent from the documentation.
6. **The top of the range**, where the reference engine's missing output guard lets a non-finite
   number escape.
7. **`EXP(GAMMALN.PRECISE(i))` against `FACT(i−1)`** for small integers i, the round trip
   Microsoft's own remark invites.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `GAMMALN.PRECISE` that an implementer matching Excel, or a reader
explaining an unexpected Excel result, needs to know. Each note gives exact binary64 inputs,
Excel's answer and where it was recorded; "measured" means the statement is read off recorded
Excel answers, "inferred" marks what goes beyond them. Where older text on this page says a
behaviour was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

### FN-GAMMALN-001: GAMMALN from 3 to 8 is a stored recurrence, and GAMMALN(3) is one unit above ln 2

*Detail for matching Excel's bits; measured; recorded 2026-11-04.* Applies to GAMMALN, GAMMALN.PRECISE.

With RN(v) meaning v rounded to 64 bits on the x87 unit and then stored to binary64, write x =
n + y with n the integer part. For 3 < x < 8, Excel forms z = (y+2)(y+3)...(y+n-1) from the
left with every product RN, and GAMMALN(x) is RN(RN(ln z) + GAMMALN(2+y)). So answers on this
range carry the rounding of a value on [2,3]. x = 3 itself belongs to the band on [2,3], not to
this route, and returns 0x3fe62e42fefa39f0, one unit above the stored ln 2, although GAMMALN(3)
is exactly ln 2.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `GAMMALN(3)` | x `3` (`0x4008000000000000`) (cells) | `0.6931471805599454` (`0x3fe62e42fefa39f0`) | `0.6931471805599453` (`0x3fe62e42fefa39ef`) | Excel 16.0 build 20131, platform and bitness not recorded, AMD Ryzen 7 4800H |
| `GAMMALN.PRECISE(3)` | x `3` (`0x4008000000000000`) (cells) | `0.6931471805599454` (`0x3fe62e42fefa39f0`) | `0.6931471805599453` (`0x3fe62e42fefa39ef`) | Excel 16.0 build 20131, platform and bitness not recorded, AMD Ryzen 7 4800H |
| `GAMMALN(2.020222407046596)` | x `2.020222407046596` (`0x4000296a5d874ad0`) (cells) | `0.008681034861562235` (`0x3f81c75cc69b54fc`) | — | Excel 16.0 build 20527, Windows; bitness and CPU not recorded in this reading |
| `GAMMALN(6.020222407046596)` | x `6.020222407046596` (`0x401814b52ec3a568`) (cells) | `4.8220305792560145` (`0x401349c26258ddb7`) | `4.822030579256015` (`0x401349c26258ddb8`) | Excel 16.0 build 20527, Windows; bitness and CPU not recorded in this reading |

Correct value: ln 2, correctly rounded.
Correct value: mpmath loggamma at 800 bits from the binary64 input, correctly rounded.

Basis (measured): 6,000 of 6,000 fractions at each n from 3 to 7, 30,000 answers, from Excel's
own GAMMALN(2+y) (build 20527). The descending product fits 5,430 to 5,527 of 6,000. Replayed
for this note in exact rational arithmetic with a correctly rounded logarithm on 300 of those
fractions: 300 of 300 at n = 3 to 6, and 299 of 300 at n = 7. Recorded rows match GAMMALN 2,106
of 4,538 and GAMMALN.PRECISE 10,627 of 14,278, with stand-ins for the bands below 3.

Inferred, not measured: That GAMMALN.PRECISE takes the same route is inferred: the 30,000
answers are GAMMALN, and the two functions agree at every shared recorded input tried,
including 3.

- Open: The rationals below 3. They are rationals in x itself, [1.5,2) and [2,3] are separate bands, and their tables are not identified: no rescaled Cody-Hillstrom table and no fresh minimax table tried reproduces the steps of their mean error.

Evidence:

- [`content/notes/evidence/2026-11-04/gamma-EVIDENCE.md`](../../content/notes/evidence/2026-11-04/gamma-EVIDENCE.md) — research write-up: how the stored stages were read
- DnaCalc `crates/calc-func/src/ported/excel_numeric/gammaln.rs` at `1c7a4ab86b` — the routes in the product, with stand-ins where the code says so; the commit message carries the row counts
- DnaCalc `crates/calc-func/witness-status.tsv` at `af74247e1e` — rows matched per function
- CalcFunc-Evidence `cache/oracle/build-20131/GAMMALN.jsonl.gz`, line 273, key GAMMALN|0x4008000000000000 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `cache/oracle/build-20131/GAMMALN.PRECISE.jsonl.gz`, line 22, key GAMMALN.PRECISE|0x4008000000000000 at `4d5320dfdc` (readings)
- [`content/notes/evidence/2026-11-04/gamma-readings.tsv`](../../content/notes/evidence/2026-11-04/gamma-readings.tsv), GAMMALN 4000296a5d874ad0 (readings)
- [`content/notes/evidence/2026-11-04/gamma-readings.tsv`](../../content/notes/evidence/2026-11-04/gamma-readings.tsv), GAMMALN 401814b52ec3a568 (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| `.PRECISE` suffix | The 2010 renaming convention; a name, not a quantified accuracy claim |
| replacement banner | Microsoft's standard supersession notice, worded "may provide improved accuracy" |
| inheritance direction | Which spelling a shared measurement can legitimately be attributed to |
| shared-kernel blindness | The inability of a delegating implementation to exhibit a difference between two surfaces |
| the two zeros | x = 1 and x = 2, where ln Γ vanishes and relative error is unbounded |

## Sources

- Microsoft Learn, "WorksheetFunction.GammaLn_Precise method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.gammaln_precise>
  (the description, the two conditions worded as "generates an error" without naming a code, and
  the exp(GAMMALN(i)) = (i − 1)! remark).
- Microsoft Learn, "WorksheetFunction.GammaLn method (Excel)" — the legacy surface, its
  replacement banner naming this function, and the named `#VALUE!`/`#NUM!` codes. The
  worksheet-surface pages at `support.microsoft.com` were not retrievable at curation time.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, §6.1; Cody & Hillstrom (1967);
  SPECFUN `ALGAMA`/`DLGAMA`; fdlibm `e_lgamma_r.c`.
- Handbook evidence records `EV-MATH-0013` (the held-out tests, their scope caveats, the capture
  spelling, and the upstream self-contradiction) and `EV-MATH-0014` (the Excel-versus-Excel alias
  identity on one named build).
- Handbook, [GAMMALN](FUNC.GAMMALN.md) — the shared mathematics and numerical treatment.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.GAMMALN.PRECISE.json` (arity, category **Statistical
  functions**) and `data/presence/FUNC.GAMMALN.PRECISE.json` (the `special_dist_family` module).
- OxFunc `crates/oxfunc_core/src/functions/special_dist_family.rs` at commit `473efa3` — the
  one-line `gammaln_precise_kernel` delegation, the input-only domain guard, and the
  logical-acceptance comment quoted under **Arguments**.
