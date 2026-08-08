---
schema: efh.function-page/v1
function_id: FUNC.T.INV
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
  The left-tail Student-t quantile function: the inverse of T.DIST's cumulative form, and a
  surface with no legacy predecessor — the legacy TINV inverts the two-tail area instead.
---

# T.INV

## What it computes

`T.INV(probability, deg_freedom)` returns the **left-tail quantile** of Student's
*t*-distribution with ν degrees of freedom: the value `t` for which

    P(T ≤ t) = probability,        i.e.   t = F⁻¹(p; ν)

where `F` is the cumulative distribution function described on
[T.DIST](FUNC.T.DIST.md#what-it-computes). It is the exact functional inverse of
`T.DIST(·, ν, TRUE)`.

`F(·; ν)` is continuous and strictly increasing on the whole real line, so the inverse exists,
is unique, and is single-valued for every `p` in the **open** interval `(0, 1)`. It is not
defined at the endpoints: `F⁻¹(p) → −∞` as `p → 0⁺` and `→ +∞` as `p → 1⁻`. Those are genuine
poles of the quantile function, not removable singularities, and they are where every
implementation difficulty lives.

Symmetry gives the identity that halves the work and doubles the accuracy:

    T.INV(p, ν) = − T.INV(1 − p, ν)

so `T.INV(½, ν) = 0` exactly for every ν, and the whole function is determined by its values on
`p ∈ (0, ½]`. There is no closed form for general ν, but there are two:

| ν | `T.INV(p, ν)` |
|---|---|
| 1 | `tan(π(p − ½))` |
| 2 | `(2p − 1) · sqrt( 2 / (4p(1 − p)) )` |
| ν → ∞ | `NORM.S.INV(p)` |

The ν = 2 row is exact and elementary — write `u = 2p − 1`; then `t = u·sqrt(2/(1 − u²))`. Along
with the ν = 1 tangent form, it gives two full parameter slices where an implementation can be
checked against arithmetic rather than against another approximation.

A useful asymptotic for the deep tail: as `p → 0⁺`, `t ~ −(ν/(2p·B(ν/2, ½)·... )` — more
usefully stated as `t ≈ −ν^{1/2} · (2p·B)^{−1/ν}` with `B = B(ν/2, ½)` the beta function, which
says the quantile blows up like a *power* `p^{−1/ν}` rather than like the normal's
`sqrt(−2 ln p)`. For small ν the growth is violent: at ν = 1 the quantile is proportional to
`1/p`. That is why t-quantile routines need a starting bracket that is aware of ν and why a
normal-based initial guess is a bad one in the tail.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `probability` | "The probability associated with the Student's t-distribution" — the left-tail cumulative probability. | Yes |
| `deg_freedom` | ν, the number of degrees of freedom. Non-integer values are documented as truncated. | Yes |

Exactly two arguments; the projection records an arity of exactly two.

Microsoft documents the admissible range as `0 < probability ≤ 1` — `#NUM!` when
`probability ≤ 0` **or** `probability > 1`. Read literally, `probability = 1` is admitted. The
mathematics says `F⁻¹(1) = +∞`, which is not a double. **This is a documentation-versus-
mathematics divergence and the Handbook records it rather than resolving it:** either Excel
returns some finite saturation value, or it returns `#NUM!` in a case the documentation admits,
or it returns an infinity that the value model has no place for
([The value universe](../model/01-value-universe.md) has no infinity kind). Nobody has checked
which. It is the first probe on the list below.

Unlike the three `T.DIST*` pages, this page **does** document what happens to a non-integer
`deg_freedom`: "if `deg_freedom` is not an integer, it is truncated". That asymmetry in the
documentation — the inverses specify truncation and the forwards say nothing — is itself worth
recording.

Numeric slots take ordinary to-number coercion; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`, a real quantile.

- **`probability = ½`** gives exactly 0 for every ν. Free, exact, and diagnostic.
- **`probability` near 0 or near 1** is where the quantile is largest in magnitude and where its
  *relative* accuracy is hardest to hold, because the forward CDF is flat there: a change of one
  ulp in `p` moves `t` by an amount inversely proportional to the density, which is tiny in the
  tail. This is the fundamental conditioning statement for any quantile function, and it means
  a t-quantile in the far tail cannot be accurate to the last bit no matter how good the
  algorithm — the information is not in the input.
- **`probability = 1`** — see Arguments. Documented as admissible, mathematically infinite,
  behaviour unknown.
- **Small ν** makes the tails heavier and the quantile larger: at ν = 1 the quantile grows like
  `1/p`, so `T.INV(1e-300, 1)` should be an enormous but finite double, and `T.INV(1e-320, 1)`
  should overflow. Where the overflow boundary actually sits, and what Excel returns at it, is
  unprobed.
- **Round trips.** `T.DIST(T.INV(p, ν), ν, TRUE)` should return `p`, and
  `T.INV(T.DIST(x, ν, TRUE), ν)` should return `x`. Neither round trip can be exact in floating
  point, and the *size* of the discrepancy is a direct measurement of the pair's joint quality.
  No such measurement exists in the Handbook.
- **Arrays.** The lift axis is projected as `surface_native` with `default-unexamined`
  provenance; the reference-engine battery beside this page refuses an inline array argument.
  Unsettled.

## Errors

As documented on Microsoft's `T.INV` page:

| Error | Documented condition |
|---|---|
| `#VALUE!` | Either argument is nonnumeric. |
| `#NUM!` | `probability ≤ 0` or `probability > 1`. |
| `#NUM!` | `deg_freedom < 1`. |

Errors in either argument propagate under the universal coercion rule. The Handbook has not
verified any of this against Excel.

## Relationships

- **[T.DIST](FUNC.T.DIST.md)** is the forward function this inverts, in its cumulative form.
- **[T.INV.2T](FUNC.T.INV.2T.md)** is the *other* inverse, and the distinction is the single
  most consequential fact in this corner of Excel. `T.INV.2T(p, ν)` returns the `t` with
  `P(|T| > t) = p`; `T.INV(p, ν)` returns the `t` with `P(T ≤ t) = p`. They are related by

        T.INV.2T(p, ν) = T.INV(1 − p/2, ν) = − T.INV(p/2, ν)

  so for the usual 5% two-sided critical value you want `T.INV.2T(0.05, ν)`, which equals
  `T.INV(0.975, ν)` and **not** `T.INV(0.05, ν)` — the latter is negative and is the 5%
  *one-sided lower* critical value.
- **`TINV` is the legacy spelling of `T.INV.2T`, not of `T.INV`.** `T.INV` has no legacy
  predecessor: before the dotted names arrived, Excel offered no left-tail t quantile at all.
  A mechanical rewrite of `TINV` to `T.INV` silently changes the answer, and changes its sign
  for `p > ½`. This is the sharpest name-versus-semantics trap in the statistical category.
- **`NORM.S.INV`** is the ν → ∞ limit and the usual source of an initial guess.
- **`F.INV.RT`**, **`CHISQ.INV`** and **`BETA.INV`** are the sibling quantile surfaces; all of
  them, in the reference engine, sit in the same implementing module as this one.
- Readers confuse `T.INV` with `T.INV.2T`, and both with `T.TEST` (which returns a `p`-value,
  not a critical value).

## Numerical notes

A quantile function is an inverse problem, and there are exactly three ways to attack it: invert
the reduction analytically, invert an incomplete-beta inverse, or iterate on the forward
function. Excel's own documentation says, for the two-tail sibling, that it iterates.

**The analytic route.** The t-quantile reduces to the inverse of the regularized incomplete
beta: from `T²/(T² + ν) ~ Beta(½, ν/2)`, if `y = I⁻¹_{2p'}(ν/2, ½)` for the appropriate tail
probability `p'`, then `t = ±sqrt(ν(1 − y)/y)`. Any library with a good `ibeta_inv` — Boost.Math
has one; Cephes' `incbi` is the classical implementation; DCDFLIB and the TOMS algorithms in
that lineage are the other standard sources — gets the t-quantile for free and with a stated
error bound. This is the route a `natural-best` implementation should take.

**The iterative route.** Newton's method on `F(t) − p = 0` has the derivative available in
closed form (it is the density), so each step is cheap, and the function is monotone, so
bisection is always available as a safeguard. The difficulties are the starting bracket — the
Cornish–Fisher expansion for the t quantile in terms of the normal quantile, given by Hill's
classic algorithm and reproduced in A&S 26.7.5, is the standard seed — and the stopping rule.
Iterating to a fixed *absolute* tolerance on `t` is wrong in the tail, where `t` is large;
iterating to a fixed tolerance on `p` is wrong in the centre, where the density is large. A
tolerance stated in the wrong variable is the most common defect in spreadsheet quantile
functions, and it is precisely what makes a documented statement like "precision depends on the
precision of the forward function" true rather than merely modest.

**Conditioning.** The derivative of the quantile function is `1/f(t; ν)`, so the condition number
of the problem blows up wherever the density is small — which is the entire tail. No algorithm
can beat this; it is a property of the question. What an implementation *can* do is avoid
adding error of its own, which means: never form `1 − p` when `p` is near 1 (use a complement
entry point), never evaluate the forward CDF by subtraction inside the iteration, and use the
symmetry `T.INV(p, ν) = −T.INV(1 − p, ν)` to move the work into the half where the argument is
better conditioned.

None of this is a claim about Excel's internals. Microsoft documents an iterative seek for
`T.INV.2T`; it does not say what `T.INV` does, and the Handbook does not know.

## What has not been checked

No Handbook vector suite exists for `T.INV`, and no Handbook evidence record lists this surface
among its subjects. **Nobody has checked `T.INV` against Excel within the Handbook's record.**

Two nearby records exist and neither reaches this page. The Handbook's alias-pairing record for
the legacy/modern distribution collapse covers **forward CDFs only**, and states explicitly that
no inverse pair has ever been probed for identity — a limit that applies here doubly, since
`T.INV` has no legacy counterpart in the first place. A separate per-surface histogram record
carries a row for the legacy `TINV`; `TINV` is `T.INV.2T`'s ancestor, not this surface's, so
that row does not travel here either. The reference-engine battery beside this page is the
engine answering its own questions.

Inputs worth probing first:

1. **`T.INV(1, ν)`.** Documented as admissible; mathematically infinite. One call decides
   whether the documentation, the value model, or the implementation is the thing that has to
   give. This is the highest-information probe on the page.
2. **`T.INV(0.5, ν)`** across ν — must be exactly 0.
3. **`T.INV(p, 1)` against `TAN(PI()*(p − 0.5))`**, and **`T.INV(p, 2)` against
   `(2p−1)·SQRT(2/(4p(1−p)))`** — the two elementary slices, checking Excel against arithmetic.
4. **`T.INV(p, ν) + T.INV(1 − p, ν)`** — must be exactly 0 by symmetry. A non-zero result
   measures the asymmetry of the search directly and needs no oracle.
5. **`T.DIST(T.INV(p, ν), ν, TRUE)` against `p`** — the round trip, run at `p` near ½, at
   `p = 0.975`, and at `p = 1e-10`, which samples the three conditioning regimes.
6. **`T.INV(1 − p/2, ν)` against `T.INV.2T(p, ν)`** — the exact cross-surface identity, and the
   probe that would catch a sign or tail confusion in either.
7. **Deep tail at small ν**: `T.INV(1e-300, 1)` and nearby, to find the overflow boundary.
8. **Non-integer `deg_freedom`** — `T.INV(0.975, 2.9)` against `T.INV(0.975, 2)`, which
   confirms or refutes the documented truncation in one call.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| quantile | The inverse of a cumulative distribution function |
| left-tail quantile | The `t` with `P(T ≤ t) = p` — what this surface returns |
| conditioning | `dt/dp = 1/f(t; ν)`; large in the tail, so tail quantiles are intrinsically hard |
| complement entry point | Computing `1 − p` quantities directly rather than by subtraction |
| Cornish–Fisher seed | The normal-quantile-based starting approximation for an iterative search |
| `default-unexamined` | Axis-provenance marker: the projected value is a default, not a finding |

## Sources

- Microsoft, "T.INV function" —
  <https://support.microsoft.com/en-us/office/t-inv-function-2908272b-4e61-4942-9df9-a25fec9b0e2e>
  (syntax; `#VALUE!` for nonnumeric arguments; `#NUM!` for `probability ≤ 0` or
  `probability > 1`; `#NUM!` for `deg_freedom < 1`; and the statement that a non-integer
  `deg_freedom` is truncated). Retrieved for this page. The page states no iteration count,
  tolerance, or accuracy claim.
- Microsoft, "T.INV.2T function" — for the contrast: that page *does* describe an iterative seek
  and does state that precision depends on the forward function.
- Handbook evidence record `EV-DIST-0011` — cited for its explicit limit that the legacy/modern
  collapse covers forward CDFs only and that no inverse pair has been probed for identity.
- M. Abramowitz and I. A. Stegun, *Handbook of Mathematical Functions*, section 26.7, including
  26.7.5 (the Cornish–Fisher-style expansion of the t percentage point in terms of the normal
  deviate).
- Cephes `incbi` (inverse incomplete beta) and `stdtri`; Boost.Math `ibeta_inv` and
  `students_t_distribution` quantile; DCDFLIB and the TOMS incomplete-beta-inverse lineage — the
  standard analytic routes to a t quantile.
- Handbook [The value universe](../model/01-value-universe.md) (no infinity kind) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.T.INV.json`, `data/presence/FUNC.T.INV.json`.
