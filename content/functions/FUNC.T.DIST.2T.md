---
schema: efh.function-page/v1
function_id: FUNC.T.DIST.2T
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
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
family: chi_f_t_family
role_in_family: >-
  The two-tail Student-t surface: the p-value shape that most t-tests actually want, defined
  only for non-negative x and inverted by T.INV.2T rather than by T.INV.
---

# T.DIST.2T

## What it computes

`T.DIST.2T(x, deg_freedom)` returns the **two-tailed** area of Student's *t*-distribution with
ν degrees of freedom:

    T.DIST.2T(x, ν) = P(|T| > x) = 2 · P(T > x) = 2 · (1 − F(x; ν))

for `x ≥ 0`, where `F` is the cumulative distribution function given on
[T.DIST](FUNC.T.DIST.md#what-it-computes). Because the t-distribution is symmetric about zero,
the two tails are equal and the factor of two is exact rather than an approximation.

In terms of the regularized incomplete beta function, again exactly:

    T.DIST.2T(x, ν) = I_{ν/(ν+x²)}(ν/2, ½)

— note that the `½` prefactor that appears in the one-tail form has cancelled against the
doubling. The two-tail area is, in the beta parameterization, the *simpler* object of the three
t surfaces.

Domain and range: `x ≥ 0` (Microsoft documents `#NUM!` below zero), ν > 0. The value falls
strictly from 1 at `x = 0` to 0 as `x → ∞`. `T.DIST.2T(0, ν) = 1` exactly, for every ν: the
whole distribution is more than zero away from zero in absolute value, apart from a null set.

Elementary closed forms at the small integer degrees of freedom:

| ν | `T.DIST.2T(x, ν)` for `x ≥ 0` |
|---|---|
| 1 | `1 − 2·arctan(x)/π` |
| 2 | `1 − x / sqrt(x² + 2)` |
| ν → ∞ | `2(1 − Φ(x))` = `ERFC(x/sqrt 2)` |

That ν = 2 row is worth pausing on: the two-tail t-distribution at two degrees of freedom is an
*elementary rational-algebraic* function, computable to the last bit with a square root and a
division. Anything an implementation publishes there can be checked against arithmetic.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `x` | The numeric value at which the distribution is evaluated. Documented as non-negative. | Yes |
| `deg_freedom` | ν, "an integer indicating the number of degrees of freedom" (Microsoft). | Yes |

Exactly two arguments; the projection records an arity of exactly two. No `cumulative` flag —
this surface is a tail area only.

`x < 0` is documented as `#NUM!`. This is the one member of the modern t trio that keeps the
legacy `TDIST` domain restriction, and it is worth knowing *why* it is not merely legacy
baggage: `P(|T| > x)` for negative `x` is identically 1, so the natural extension is a constant
and carries no information. Refusing it is a defensible design choice rather than an oversight —
though it does mean `T.DIST.2T` is not a total function on the reals the way `T.DIST.RT` is
documented to be.

The page describes `deg_freedom` as an integer and does not say what a non-integer value does.
The inverse pages (`T.INV`, `T.INV.2T`) document truncation for the same parameter; the three
`T.DIST*` pages are silent.

Numeric slots take ordinary to-number coercion; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`, a probability in `(0, 1]`.

- **`x = 0`** is exactly 1 — one of only two exact values this surface has, and free to check.
- **Large `x`** gives a small `p`-value decaying like `x^(−ν)`. As with `T.DIST.RT`, the value
  stays representable and informative deep into the tail *provided* it is not formed by
  subtraction from one.
- **Small `x`**, near zero, is the opposite hazard: here the answer is close to 1, and the
  interesting quantity is `1 − T.DIST.2T(x, ν)`, which no Excel surface exposes directly.
  A reader wanting the central probability accurately should use
  `T.DIST(x, ν, TRUE) − T.DIST(−x, ν, TRUE)`, or, better, `2·T.DIST(x, ν, TRUE) − 1` — and
  should know that all three routes lose digits near `x = 0`.
- **The doubling identity.** `T.DIST.2T(x, ν) = 2 · T.DIST.RT(x, ν)` is exact in real
  arithmetic, and multiplication by two is exact in binary floating point. So if Excel computes
  the two surfaces from one internal quantity, the published bits must satisfy the identity
  exactly, and if they do not, the two surfaces have different internals. That is a rare
  situation: a metamorphic probe whose *failure* is as informative as its success, requiring no
  external oracle.
- **Arrays.** The lift axis is projected as `surface_native` with `default-unexamined`
  provenance, and the reference engine's own battery beside this page refuses an inline array
  argument. Unsettled.

One observation about the reference engine, stated as such and not as anything about Excel: at
ν = 1 the exact answer at `x = 1` is `1 − 2·arctan(1)/π = ½`, and the value the reference-engine
battery beside this page publishes for that input is not exactly `½`. That is the engine
answering its own question, with no Excel involved; it is recorded here because it is a
one-input demonstration that the generic beta path does not reproduce the Cauchy closed form
exactly, which is the sort of thing a vector suite would need to decide about deliberately.

## Errors

As documented on Microsoft's `T.DIST.2T` page:

| Error | Documented condition |
|---|---|
| `#VALUE!` | Any argument is nonnumeric. |
| `#NUM!` | `deg_freedom < 1`. |
| `#NUM!` | `x < 0`. |

Errors in either argument propagate under the universal coercion rule. The Handbook has not
verified any of this against Excel.

## Relationships

- **`TDIST(x, ν, 2)`** is the legacy spelling of this surface. Caution: the Handbook's
  alias-pairing record for the legacy/modern collapse names `TDIST` and `T.DIST.RT` as one of
  its five pairs — that is the **one-tail** correspondence. `T.DIST.2T` is not a subject of that
  record, and no evidence in the Handbook's record bears on whether `TDIST(x, ν, 2)` and
  `T.DIST.2T(x, ν)` publish the same bits. They are documented to compute the same number; that
  is not the same statement.
- **[T.DIST.RT](FUNC.T.DIST.RT.md)** — exactly half of this value for `x ≥ 0`.
- **[T.DIST](FUNC.T.DIST.md)** — the left-tail CDF and the density.
- **[T.INV.2T](FUNC.T.INV.2T.md)** is this function's inverse, and Microsoft's `T.INV.2T` page
  says so explicitly: it seeks the `x` with `T.DIST.2T(x, ν) = probability`, and states that
  `T.INV.2T`'s precision depends on `T.DIST.2T`'s. **[T.INV](FUNC.T.INV.md)** inverts `T.DIST`
  instead; confusing the two is the single most common error in this corner of Excel, because
  both are called "the inverse of the t-distribution" in ordinary speech.
- **`F.DIST.RT(x², 1, ν) = T.DIST.2T(x, ν)`** for `x ≥ 0`, from `T² ~ F(1, ν)`. Both surfaces
  are implemented in one module in the reference engine.
- **`CHISQ.DIST.RT`** is the same shape of object for a different distribution, and
  **`ERFC`** is the ν → ∞ limit up to the `sqrt 2` scaling.
- **`T.TEST`** with `tails = 2` is the packaged consumer of this quantity.

## Numerical notes

The two-tail area is the cleanest of the three t surfaces to compute, because in the beta
parameterization it *is* `I_{ν/(ν+x²)}(ν/2, ½)` with no prefactor and no complementation. An
implementation that computes it directly inherits the full relative accuracy of its incomplete
beta routine across the whole tail.

The traps are the ones the identity invites rather than the ones the formula contains:

1. **Do not compute it as `2 * (1 − T.DIST(x, ν, TRUE))`.** The inner subtraction destroys the
   tail before the doubling can restore anything.
2. **Do not compute the central probability as `1 − T.DIST.2T(x, ν)`** for small `x`. That is
   the same mistake mirrored, and it is the one a spreadsheet user is likelier to make, because
   Excel offers no surface for the central probability.
3. **Argument reduction.** `z = ν/(ν + x²)` should be formed so that neither `x²` overflows nor
   the small complement `x²/(ν + x²)` is lost. Keeping the complement and calling a
   complementary incomplete-beta entry point handles both.

For the continued-fraction evaluation of `I_z(a, b)` the standard reference is Numerical
Recipes' `betacf`, with the reflection applied so the fraction converges quickly; Cephes'
`incbet` and Boost.Math's `ibeta`/`ibetac` are the production treatments worth reading. For
integer ν, A&S 26.7.3–26.7.4 give finite closed forms, and A&S 26.7 also tabulates the
percentage points that a spreadsheet is standing in for.

Nothing here is a claim about Excel's algorithm. The Handbook does not know it.

## What has not been checked

No Handbook vector suite exists for `T.DIST.2T`, and no Handbook evidence record lists this
surface among its subjects. **Nobody has checked `T.DIST.2T` against Excel within the Handbook's
record.**

The nearest evidence belongs to a sibling and must not be read onto this page: the alias-pairing
record that pairs the legacy `TDIST` spelling with `T.DIST.RT` covers the one-tail
correspondence. This surface is not one of its subjects. The reference-engine battery rendered
beside this page is the engine's own answers, with no Excel involved.

Documentation gaps this page could not close: what a non-integer `deg_freedom` does, and whether
the `x < 0` refusal is enforced as documented.

Inputs worth probing first, in the order the Handbook would run them:

1. **`T.DIST.2T(0, ν)`** for many ν — must be exactly 1. One of only two exactly known values,
   and a published result that is not exactly 1 would be immediately diagnostic.
2. **`2 * T.DIST.RT(x, ν)` against `T.DIST.2T(x, ν)`** on a grid. Doubling is exact in binary
   floating point, so this is a pure self-consistency test of Excel against Excel: no oracle
   needed, and a mismatch proves the two surfaces have separate internals.
3. **`T.DIST.2T(x, 2)` against `1 − x/SQRT(x²+2)`** — an elementary closed form, so this
   compares Excel against arithmetic. `ν = 1` against `1 − 2·ATAN(x)/PI()` is the companion.
4. **`TDIST(x, ν, 2)` against `T.DIST.2T(x, ν)`** — the legacy/modern pairing that the existing
   alias record does *not* cover. This is the probe that would extend that record's five pairs
   by one, and it is cheap.
5. **`T.DIST.2T(−1, 5)`** — whether the documented `#NUM!` is actually raised.
6. **`T.INV.2T(T.DIST.2T(x, ν), ν)` against `x`** — the round trip, which measures the inverse's
   documented dependence on this surface directly.
7. **Deep tail** (`x` at 10, 30, 100 across ν) and **near zero** (`x` at `1e-8`), the two ends
   where the two failure modes above bite.
8. **Non-integer and out-of-range `deg_freedom`.**

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| two-tail area | `P(\|T\| > x)` — the quantity this surface returns |
| doubling identity | `T.DIST.2T(x, ν) = 2·T.DIST.RT(x, ν)`, exact in binary floating point |
| central probability | `P(\|T\| ≤ x)` — the complement, which no Excel surface exposes directly |
| regularized incomplete beta | `I_z(a,b)`; this surface equals it with no prefactor |
| `default-unexamined` | Axis-provenance marker: the projected value is a default, not a finding |

## Sources

- Microsoft, "T.DIST.2T function" —
  <https://support.microsoft.com/en-us/office/t-dist-2t-function-198e9340-e360-4230-bd21-f52f22ff5c28>
  (syntax, both arguments required, and the three documented error conditions: `#VALUE!` for
  nonnumeric arguments, `#NUM!` for `deg_freedom < 1`, `#NUM!` for `x < 0`). Retrieved for this
  page.
- Microsoft, "T.INV.2T function" — the statement that `T.INV.2T` searches iteratively for the
  `x` with `T.DIST.2T(x, ν) = probability` and that its precision depends on this surface's.
- Handbook evidence record `EV-DIST-0011` — cited here only to establish that its five-pair
  legacy/modern collapse covers `TDIST`/`T.DIST.RT` and **not** this surface.
- M. Abramowitz and I. A. Stegun, *Handbook of Mathematical Functions*, section 26.7.
- W. H. Press et al., *Numerical Recipes*, incomplete beta function; Cephes `incbet`;
  Boost.Math `ibeta` / `ibetac`.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.T.DIST.2T.json`, `data/presence/FUNC.T.DIST.2T.json`
  and `data/battery/FUNC.T.DIST.2T.json`.
