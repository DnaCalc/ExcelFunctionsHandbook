---
schema: efh.function-page/v1
function_id: FUNC.GAMMALN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0012
  - EV-MATH-0013
  - EV-MATH-0014
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.GammaLn method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.gammaln"
    role: "documented description, the replacement banner naming GammaLn_Precise, the #VALUE! and #NUM! conditions, and the exp(GAMMALN(i)) = (i-1)! remark"
  - work: "Microsoft Support — GAMMALN function"
    locator: "https://support.microsoft.com/en-us/office/gammaln-function-b838c48b-c65f-484f-9e1d-141c55470eb9"
    role: "the worksheet-surface documentation page; not retrievable at curation time (the host refused the request)"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 6"
    locator: "6.1.40 ff — Stirling's asymptotic series for ln Γ"
    role: "the asymptotic expansion every implementation uses at large argument"
  - work: "W. J. Cody and K. E. Hillstrom, 'Chebyshev approximations for the natural logarithm of the gamma function' (1967); SPECFUN ALGAMA/DLGAMA"
    locator: null
    role: "the minimax rational design named in the upstream identification of this surface"
  - work: "fdlibm e_lgamma_r.c"
    locator: null
    role: "the reference open-source treatment, including the special handling near the two zeros"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: special_dist_family
role_in_family: >-
  The log-gamma surface that received the module's identified Excel-shaped kernel; paired with
  GAMMALN.PRECISE, which delegates to it, and deliberately not wired to GAMMA.
---

## What it computes

`GAMMALN(x)` returns the natural logarithm of the gamma function:

    GAMMALN(x)  =  ln Γ(x)  =  ln ∫₀^∞ t^{x−1} e^{−t} dt        for x > 0

**Domain:** the positive reals. Excel does not expose the negative-argument branch (where Γ
alternates sign and its logarithm is complex), so the documented domain is x > 0 with `#NUM!`
outside it. Note this makes `GAMMALN` a *restriction* of `LN(GAMMA(x))`, not an equivalent: the
latter is defined on more of the line and overflows on far less of it.

**Range and shape.** ln Γ is strictly convex on (0, ∞) — this is the Bohr–Mollerup
characterisation, and it is the property that makes ln Γ the natural object rather than Γ. It
falls from +∞ at x → 0⁺ to a single minimum near x ≈ 1.4616 (value ≈ −0.1214) and rises
thereafter without bound. It has **exactly two zeros**, at x = 1 and x = 2, because Γ(1) =
Γ(2) = 1. Those two zeros are the source of the function's only real accuracy trap; see
**Numerical notes**.

**The identities.**

    ln Γ(x + 1)  =  ln x  +  ln Γ(x)                      (recurrence)
    ln Γ(n)      =  ln (n − 1)!                           for positive integers n
    ln Γ(x)      ~ (x − ½) ln x − x + ½ ln(2π) + 1/(12x) − 1/(360x³) + …   (Stirling, A&S 6.1.40)

Microsoft states the integer relation from the other side: "The number e raised to the
GAMMALN(i) power, where i is an integer, returns the same result as (i − 1)!" That is exact as
mathematics and, as with `GAMMA`, is a floating-point *claim about the round trip* rather than a
guarantee — exp(lnΓ(i)) need not land on the integer.

**Why this function exists at all.** Γ overflows binary64 a little above x ≈ 171.6; ln Γ stays
comfortably finite until x is astronomically large. Every ratio of gamma functions — binomial
coefficients, beta functions, distribution normalising constants, multinomials — should be
computed as a difference of log-gammas and exponentiated once at the end, if at all. That is
the entire practical argument for the function, and it is why `GAMMALN` appears far more often
than `GAMMA` in careful spreadsheet work.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `x` | The value at which to evaluate ln Γ. Required. | — |

One argument; the reference engine declares an arity of exactly 1. The slot is numeric under
ordinary to-number coercion, and the reference engine's source records that the GAMMA/GAMMALN
family **accepts logicals** — `GAMMALN.PRECISE(TRUE)` gives 0, since Γ(1) = 1 — while the
ERF/ERFC family in the same module rejects them with `#VALUE!`. Upstream attributes that split
to an empirical sweep against Excel 16.0; it is not in the documentation.

## Result and edge cases

Returns `Number`.

- **x = 1 and x = 2** are the exact zeros. An implementation should return an exact zero at both,
  and the sign of that zero is itself an observable — the reference engine's battery records a
  negative zero at one of them.
- **Small positive x** behaves like −ln x: ln Γ(x) = −ln x − γx + O(x²). At the bottom of the
  double range the value is large and finite, which is the good news `GAMMALN` exists to
  deliver — where `GAMMA` overflows, `GAMMALN` still answers.
- **Large x** follows Stirling and grows like x ln x. It overflows binary64 only for absurdly
  large arguments — and here the reference engine has a gap: its guard tests the *input* for
  finiteness and its domain, but nothing tests the *output*, so at the top of the double range
  the projected battery records a non-finite number rather than an error. Recorded as an
  observation about the reference engine: a raw infinity escapes as a `Number`. Whether Excel
  publishes a non-finite number anywhere is a separate question this record does not answer.
- **x ≤ 0** is documented `#NUM!`, including the poles and the whole negative axis where the
  real logarithm of Γ does not exist.
- **Array arguments** lift elementwise through the module's broadcast helper.

## Errors

As documented by Microsoft:

| Error | Condition |
|---|---|
| `#VALUE!` | `x` is nonnumeric |
| `#NUM!` | `x` ≤ 0 |

The reference engine adds a non-finite-input `#NUM!` with no documented counterpart, and — as
noted above — does *not* guard the output.

## Relationships

- **[GAMMALN.PRECISE](FUNC.GAMMALN.PRECISE.md)** — the documented replacement. This is one of
  the better-evidenced alias relationships in the Handbook and it deserves precision:
  - Microsoft's VBA page for `GammaLn` carries the standard replacement banner: the function
    "has been replaced with one or more new functions that may provide improved accuracy and
    whose names better reflect their usage", and it names `GammaLn_Precise`.
  - `EV-MATH-0014` records an upstream **Excel-versus-Excel identity check**, on one named
    build, in which both surfaces resolved to the same implementation and published the same
    results at every probed point. The record is careful that this is an identity check and not
    a pass rate, and that the source publishes no numerator or denominator for it.
  - The reference engine mirrors the identity structurally: its `GAMMALN.PRECISE` kernel is a
    one-line delegation to the `GAMMALN` kernel.
  - **The supersession did not move the category.** Both surfaces are classified under
    **Statistical functions** in the catalogue projection; `GAMMALN` was not moved to
    **Compatibility**, unlike `GAMMADIST` and `GAMMAINV`, which were. Recorded as a
    documentation-versus-catalogue mismatch: the documentation says replaced, the category says
    current.
  - So the pair is *documented* as a replacement, *observed* identical on one build by upstream,
    and *structurally* identical in the reference engine — and none of that is a Handbook
    measurement.
- **[GAMMA](FUNC.GAMMA.md)** — the value rather than its logarithm. The two share a module and
  **not** a numeric path: upstream's landed log-gamma kernel is wired only to the two `GAMMALN`
  surfaces, and the source says so in as many words. `LN(GAMMA(x))` and `GAMMALN(x)` are
  therefore different computations with different accuracy and different domains, and the
  first overflows where the second does not.
- **`FACT`, `COMBIN`, `MULTINOMIAL`** — combinatorial quantities better computed through this
  function than through `GAMMA` or `FACT` when the intermediate factorials would overflow.
  `COMBIN(n,k)` as `EXP(GAMMALN(n+1) − GAMMALN(k+1) − GAMMALN(n−k+1))` is the standard rewrite,
  and it is *less* accurate for small n than an exact integer route — a real trade, not a free
  improvement.
- **[GAMMA.DIST](FUNC.GAMMA.DIST.md), `BETA.DIST`, `POISSON.DIST`** — consumers: their
  normalising constants are log-gammas.
- **Module siblings**: `ERF`, `ERF.PRECISE`, `ERFC`, `ERFC.PRECISE`, `GAMMA`, `WEIBULL`,
  `WEIBULL.DIST`.

## Numerical notes

**The classical design is a three-region split.** Every serious implementation of ln Γ does the
same three things, and their boundaries are the implementation's fingerprint:

1. **Small x** — use the recurrence to lift x into the well-behaved region:
   ln Γ(x) = ln Γ(x + 1) − ln x. This is essential because the Lanczos and rational forms both
   degrade as x → 0, and the reciprocal singularity is best handled by an explicit `ln x` term.
2. **The middle**, roughly [1, 8] — a minimax rational or Chebyshev approximation. Cody &
   Hillstrom's 1967 Chebyshev approximations are the classical answer and the ancestor of
   SPECFUN's `ALGAMA`/`DLGAMA`, of fdlibm's `lgamma`, and of most libm implementations.
3. **Large x** — Stirling's asymptotic series (A&S 6.1.40), typically with terms through 1/x⁷ or
   so, above a seam around x = 8 to 12.

**The two zeros are the trap.** Because ln Γ vanishes at x = 1 and x = 2, the *relative* error
near those points is unbounded for any implementation that computes the value directly: the
answer is a small number obtained by cancelling large terms, and the surviving digits are few.
fdlibm handles this by evaluating a polynomial in (x − 1) or (x − 2) directly near each zero,
so that the small quantity is the *argument* rather than a difference. An implementation that
skips that step will produce residuals that spike sharply in two narrow bands and look fine
everywhere else — which is exactly the pattern that makes a coarse accuracy sweep miss the
problem.

**Argument reduction is the other half.** The recurrence multiplies by x, so lifting a small
argument requires computing `ln x` accurately and subtracting; near x = 1 that subtraction is
itself a cancellation. The interaction of the reduction seam with the zero at x = 1 is the
subtlest part of the whole function.

**What upstream identified.** The evidence records describe an identification campaign that
pinned a structural form for Excel's surface — a threshold near 0.7, a seam at 8.0, and a
Stirling form beyond it — and then fitted coefficients, with two held-out gates plus one fresh
never-probed corpus used to decide which candidate landed. Two properties of that campaign
matter to a reader and are stated in the records themselves. First, the held-out gates were held
out from the **coefficient fits**, not from the structural identification: the threshold, the
seam and the form were pinned on batteries surrounding those very rows. Second, the arguments in
both held-out gates are `GAMMALN.PRECISE` captures, so the inheritance runs from `.PRECISE` to
`GAMMALN` and not the other way — the surface with the fresh corpus of its own is this one, and
the record says so explicitly.

**Recommended practice.** Prefer `GAMMALN` over `LN(GAMMA(x))` unconditionally: it is defined on
a larger useful range, it does not overflow where `GAMMA` does, and it avoids the
exp-then-log round trip. Where a *ratio* of gammas is wanted, keep it in log space to the last
step. Where an exact small factorial is wanted, use the integer route, not this one.

## What has not been checked

Three records name `GAMMALN` as a subject, and together they describe a surface that is
unusually well studied and still unfinished.

`EV-MATH-0012` records the only fresh-corpus, per-surface figure this function has, on a corpus
whose arguments are `GAMMALN` captures and which the source calls "fresh" and "never-probed";
it is also the gate that decided which candidate landed. `EV-MATH-0013` records the two held-out
gates and their two caveats — held out from the fits and not from the structural
identification, and captured through the `.PRECISE` spelling — and additionally records that
**OxFunc contradicts itself** on one of the held-out numbers, publishing two different values in
the same file, with the weaker one carried forward. `EV-MATH-0014` records the Excel-versus-Excel
alias identity check on one named build. All figures render mechanically beside this page; this
prose deliberately does not restate them.

What does not exist: any Handbook vector suite for `GAMMALN`; any Handbook-side reproduction of
the alias identity; any measurement in the two narrow bands around the zeros, which is where the
mathematics predicts the worst relative error and where none of the named corpora is described
as concentrating; any measurement at the top of the range, where the reference engine returns a
non-finite number.

Inputs I would probe first, and why:

1. **Dense sweeps in (1 ± 2⁻¹⁰) and (2 ± 2⁻¹⁰).** The two zeros are the only places where
   relative error is unbounded by construction, and they are narrow enough that a uniform sweep
   over the domain will contain almost no points in them. This is the highest-value probe on the
   page and the cheapest to construct.
2. **`GAMMALN(1)` and `GAMMALN(2)` exactly**, including the sign of the returned zero.
3. **The seam neighbourhood near x = 8**, from both sides. If the identified structure is right,
   the residual pattern changes character there, and a seam is the single most identifying
   feature of an implementation.
4. **The threshold neighbourhood near x = 0.7**, likewise.
5. **Small x down to the subnormals**, where the recurrence and the `ln x` term dominate and
   where `GAMMA` cannot answer at all.
6. **The top of the range**, to locate the argument at which the result stops being finite and
   to determine whether Excel errors there or publishes something else — the case where the
   reference engine's output guard is missing.
7. **`GAMMALN` against `GAMMALN.PRECISE` on the Handbook's own probes.** Upstream's identity
   check is on one build; a second independent check is cheap and would turn an inherited result
   into a Handbook one.
8. **`EXP(GAMMALN(i))` against `FACT(i−1)`** for i = 1…20, the round trip Microsoft's own remark
   invites, which needs no oracle.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| ln Γ | The natural logarithm of the gamma function; the whole of this surface |
| Bohr–Mollerup | The convexity characterisation that makes ln Γ the natural object |
| the two zeros | x = 1 and x = 2, where ln Γ vanishes and relative error is unbounded |
| Stirling's series | The large-argument asymptotic expansion, A&S 6.1.40 |
| seam | The argument at which an implementation switches between approximation regions |
| held out from the fits | Rows excluded from coefficient fitting but not from structural identification |
| replacement banner | Microsoft's standard notice that a function has been superseded |

## Sources

- Microsoft Learn, "WorksheetFunction.GammaLn method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.gammaln>
  (the description, the replacement banner naming `GammaLn_Precise`, the `#VALUE!` nonnumeric
  and `#NUM!` x ≤ 0 conditions, and the exp(GAMMALN(i)) = (i − 1)! remark). The worksheet-surface
  page at `support.microsoft.com` was not retrievable at curation time.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, §6.1 — the recurrence, the two
  zeros, the location of the minimum, and Stirling's asymptotic series at 6.1.40.
- W. J. Cody & K. E. Hillstrom, "Chebyshev approximations for the natural logarithm of the gamma
  function", *Math. Comp.* 21 (1967); SPECFUN `ALGAMA`/`DLGAMA`; fdlibm `e_lgamma_r.c` — the
  three-region design and the special handling near the zeros.
- Handbook evidence records `EV-MATH-0012` (the fresh per-surface gate), `EV-MATH-0013` (the two
  held-out gates, their scope caveats, and the upstream self-contradiction on one figure) and
  `EV-MATH-0014` (the Excel-versus-Excel alias identity on one named build).
- Handbook, [GAMMA](FUNC.GAMMA.md) — the value form, and the record that the landed kernel does
  not touch it.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.GAMMALN.json` (arity, category **Statistical
  functions**) and `data/presence/FUNC.GAMMALN.json` (the `special_dist_family` module).
- OxFunc `crates/oxfunc_core/src/functions/special_dist_family.rs` at commit `473efa3` — the
  `gammaln_kernel` delegating to the identified `gammaln_excel` path, its input-only domain
  guard, the note that GAMMA and the shared internal lgamma are unaffected, and the
  logical-acceptance comment quoted under **Arguments**.
