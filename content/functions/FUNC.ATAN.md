---
schema: efh.function-page/v1
function_id: FUNC.ATAN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — ATAN function"
    locator: "https://support.microsoft.com/en-us/office/atan-function-50746fa8-630a-406b-81d0-4a2aed395543"
    role: "the canonical worksheet article; not retrieved for this pass (fetch returned HTTP 403)"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 4 (Elementary Transcendental Functions), section 4.4"
    locator: "4.4.1-4.4.42, the arctangent series, addition formula and continued fraction"
    role: "definition, principal range, the addition formula used for argument reduction, and the series"
  - work: "Cody & Waite, Software Manual for the Elementary Functions"
    locator: "chapter on ATAN"
    role: "the classical four-interval reduction against tan(pi/12) and tan(pi/6), and the reciprocal reduction for large arguments"
  - work: "fdlibm, s_atan.c"
    locator: null
    role: "the published reference implementation of that reduction"
  - work: "Muller, Elementary Functions: Algorithms and Implementation"
    locator: "chapters on polynomial approximation and correctly rounded elementary functions"
    role: "the modern account of minimax approximation and last-bit behaviour"
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
family: atan
role_in_family: >-
  The principal arctangent on all of R with range (-pi/2, pi/2): the unrestricted, well-conditioned
  member of the inverse circular trio, and the substrate every other inverse circular surface is
  built on.
---

## What it computes

`ATAN(number)` is the principal inverse tangent: the angle in radians whose tangent is *number*.

    atan: R -> (-pi/2, pi/2),    tan(atan x) = x

- **Domain**: all real numbers. `tan` maps each open interval `(k*pi - pi/2, k*pi + pi/2)` onto the
  whole real line, so the inverse needs a branch choice but no domain restriction.
- **Range**: the *open* interval `(-pi/2, pi/2)`. The endpoints are approached and never attained;
  `atan(x) -> ±pi/2` only in the limit.
- **Parity**: odd, exactly. `atan(-x) = -atan(x)`.
- **Monotonicity**: strictly increasing on the whole line, and bounded — one of the few elementary
  functions that is both.
- **Derivative**: `d/dx atan x = 1/(1 + x^2)`. Bounded by 1, never zero, smooth everywhere.
  `ATAN` has no singularities, no poles, no branch points on the real line, and no domain edge.
  It is the best-conditioned of the inverse circular functions and that is why it is the one
  everything else is built from.
- **Series about zero**: `atan x = x - x^3/3 + x^5/5 - x^7/7 + ...` for `|x| <= 1` (the Gregory
  series). It converges glacially near `|x| = 1` — at `x = 1` it is the Leibniz series for `pi/4`,
  famous for needing hundreds of terms per digit — which is why no implementation uses it without
  reduction.
- **Asymptotics**: `atan x = pi/2 - 1/x + 1/(3x^3) - ...` for large positive `x`.
- **Key identities**:
  - `atan(x) + atan(1/x) = pi/2` for `x > 0`, and `-pi/2` for `x < 0`. This is the reciprocal
    reduction, and it is what makes large arguments cheap.
  - The addition formula `atan(u) + atan(v) = atan((u+v)/(1-uv))` modulo `pi`, which is the basis
    of every classical range reduction and of the Machin-like formulas for `pi`.
  - `atan(x) = asin(x / sqrt(1 + x^2))`.
- **Complex continuation**: branch cuts along the imaginary axis outside `[-i, i]` — nothing on the
  real line is cut, which is the exact statement of the unrestricted real domain.

Abramowitz & Stegun cover the arctangent in chapter 4 section 4.4, including the addition formula
and the continued fraction.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The tangent of the angle you want. Required. | — |

One argument; the reference engine records an arity of exactly one, a `NumToNum` kernel signature,
and a unary numeric scalar-or-array lift profile, so arrays lift elementwise. Ordinary numeric slot
under the shared coercion rules — see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

Microsoft's projection description for this surface is "Returns the arctangent of a number", and
that one line is the only documented text the Handbook holds for `ATAN`: unlike its siblings,
`ATAN` has no `WorksheetFunction` page on Microsoft Learn (the VBA-visible name for this operation
is the language intrinsic `Atn`), and the worksheet article was not retrievable for this pass. So
the documentary basis for this page is thinner than for `ASIN` or `ACOS`, and the page says so
rather than borrowing their statements.

The misunderstood point is the *quadrant*. `ATAN` cannot tell you which quadrant a point is in,
because it receives only a ratio: `ATAN(y/x)` gives the same answer for `(1, 1)` and `(-1, -1)`.
That is what `ATAN2` is for, and it is the single most common reason `ATAN` is the wrong function.

## Result and edge cases

Returns `Number` — an angle in radians, strictly between `-pi/2` and `pi/2`.

- **Zero** returns zero; an odd implementation preserves the sign of zero.
- **Subnormals and very small arguments** pass through unchanged, since `atan x = x` to within
  rounding there.
- **`ATAN(1)`** is `pi/4` and **`ATAN(-1)`** is `-pi/4`; these are the reduction anchors and are
  worth checking in any implementation.
- **Very large arguments** approach `pi/2`. Because the range is open, no argument should return
  exactly the double nearest `pi/2`... except that, in binary64, the correctly rounded value of
  `atan(x)` *is* that double for all sufficiently large `x`. The mathematical openness of the range
  and the representable range therefore disagree: the function saturates. That is not a defect, it
  is what rounding to a finite grid means, but it is worth stating because it is what makes `ACOT`
  built as `pi/2 - ATAN(x)` fail — see the [ACOT](FUNC.ACOT.md) page.
- **The largest finite double** returns the double nearest `pi/2`.
- **Arrays** lift elementwise.

The reference engine's projected `real_result_policy` records `arg_domain_guard=none` and
`non_finite=allow`; there is no domain to guard.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | The argument does not convert to a number | Shared coercion rules |
| propagated | An error value in the argument is returned unchanged | Shared coercion rules |

There is no domain error and no overflow error. `ATAN` is one of the very few surfaces in this
category whose only failures are conversion failures — which is a consequence of a bounded range
over an unrestricted domain.

## Relationships

- **`ATAN2`** — the four-quadrant version, and the one you usually want. Note Excel's unusual
  argument order: `ATAN2(x_num, y_num)` takes **x first**, the reverse of the C convention. See
  that page.
- **`TAN`** — the forward function. `TAN(ATAN(x)) = x` to rounding for every `x`;
  `ATAN(TAN(t))` folds `t` into `(-pi/2, pi/2)`.
- **`ACOT`** — `ACOT(x) = PI()/2 - ATAN(x)` on Excel's documented continuous branch. That identity
  is exact mathematics and a numerically poor program for large `x`; the `ACOT` page is where that
  is worked out.
- **`ASIN`** and **`ACOS`** — both expressible through `ATAN`, and both harder, because they carry
  a domain restriction and an endpoint singularity that `ATAN` does not.
- **`ATANH`** — the hyperbolic namesake, and structurally the opposite: bounded domain `(-1, 1)`,
  unbounded range, poles at the ends. `ATAN` is bounded in range and unbounded in domain. The pair
  is a good illustration that a hyperbolic namesake inverts the geometry.
- **`DEGREES`** — the usual consumer of the result.
- **`PI`** — the reduction constants, and the reason a two-part split constant appears in the
  numerical notes.
- **Confused with**: `1/TAN(x)`, which is `COT`; and with `ATAN2`, when the quadrant matters.

## Numerical notes

`ATAN` is the easy one, and the reason is structural: the derivative is bounded by 1, so the
function never amplifies an input error, and the range is bounded, so the answer is never so small
that relative accuracy becomes precarious — except near zero, where the function is the identity
and therefore trivially accurate. There is no cancellation anywhere and no overflow anywhere.

The whole implementation problem is **approximation efficiency**, not stability.

**The Gregory series is unusable directly.** Its convergence rate is governed by `|x|`, and at
`|x| = 1` it needs a prohibitive number of terms. So every implementation reduces first.

**The two reductions, both from the identities above:**

1. **Reciprocal reduction**, for `|x| > 1`:
   `atan(x) = sign(x) * pi/2 - atan(1/x)`.
   This maps the whole outer half of the domain onto `[-1, 1]`. The subtraction here is safe
   because the answer is near `±pi/2`, not near zero — the exact opposite of the `ACOT` situation.
2. **Addition-formula reduction**, for the remaining `[0, 1]`:
   split at `tan(pi/12) = 2 - sqrt(3)` and `tan(pi/6)`, and for arguments above the split use
   `atan(x) = pi/6 + atan((x*sqrt(3) - 1)/(x + sqrt(3)))`
   or its `pi/4` analogue. Each application shrinks the working interval, so a short polynomial
   suffices. Cody & Waite give the classical four-interval version; `fdlibm`'s `s_atan.c` is the
   published implementation.

**The split-constant technique.** The reductions add `pi/2`, `pi/4` or `pi/6` at the end. Each of
those is irrational and its double approximation carries an error of about `2^-53` relative — which
is comparable to the final rounding of the answer and would therefore dominate the error budget.
The standard fix is to store the constant as an exact-plus-tail pair, `c = c_hi + c_lo`, with
`c_hi` having trailing zero bits so that additions involving it are exact, and to add `c_lo` last.
This is the same device that makes trigonometric argument reduction work, and it is the reason
implementations of `ATAN` contain two constants where the mathematics has one.

**Where implementations differ.** Since there is no instability, differences between
implementations of `ATAN` are last-bit differences arising from the polynomial coefficients, the
placement of the interval boundaries, and whether intermediate arithmetic happened in extended
precision. Those are exactly the kinds of difference that identification work in this Handbook's
upstream record has found elsewhere in the elementary set, and they are invisible to any test that
compares to fewer than about sixteen digits.

**Symmetry.** `atan` is odd, so an implementation should fold the sign and get exact antisymmetry.
`ATAN(x) + ATAN(-x)` being exactly zero is a cheap and complete test of that.

## What has not been checked

**No evidence record in the Handbook names `FUNC.ATAN`**, and no Handbook vector suite exists for
it. Nobody has checked this function against Excel within the Handbook's record. The presence
projection records no entries for this surface in the discrepancy catalogue, the math-deviation
catalogue, or any defect stream.

The documentary basis is unusually thin: **no Microsoft Learn `WorksheetFunction` page exists for
`ATAN`**, and Microsoft's worksheet article was not retrieved for this pass (HTTP 403). The only
documented text this page rests on is the one-line English description carried verbatim in the
Handbook's own projection. Everything mathematical above is stated from the literature, and
everything structural is read from the projections.

The battery rendered beside this page is the reference engine's own output, no Excel involved, as
its own label states.

Probes worth running first:

1. **`ATAN(1)` and `ATAN(-1)`** against `PI()/4` — the reduction anchors, and the cheapest check of
   the split constants.
2. **A logarithmic sweep across `|x|` from the subnormal floor to the largest finite double**
   against a high-precision reference. Because `ATAN` is stable everywhere, any structure in the
   residual profile is a direct picture of the interval boundaries — this is the single most
   informative probe on the page, and it is exactly the kind of plate the ULP Atlas is for.
3. **Arguments straddling `|x| = 1`**, where the reciprocal reduction switches. A step in the
   residual there identifies the reduction.
4. **Arguments near `tan(pi/12)` and `tan(pi/6)`**, the classical inner boundaries.
5. **`ATAN(x) + ATAN(-x)`** across the range — exact oddness, or not.
6. **The saturation point**: bisect for the smallest `x` whose `ATAN` is the double nearest `pi/2`.
   That single number is what governs the accuracy of `ACOT` if `ACOT` is built on `ATAN`, so the
   probe pays for two pages.
7. **Array arguments**, to confirm elementwise lifting.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| principal branch | The choice of range `(-pi/2, pi/2)` that makes the inverse single-valued |
| Gregory series | `x - x^3/3 + x^5/5 - ...`, the arctangent series; convergent but far too slow near `|x| = 1` |
| reciprocal reduction | `atan(x) = ±pi/2 - atan(1/x)`, mapping `|x| > 1` into the unit interval |
| addition-formula reduction | Using `atan(u) + atan(v) = atan((u+v)/(1-uv))` to shrink the working interval |
| split constant | Storing an irrational constant as an exact head plus a tail, so additions do not lose bits |
| saturation | The point above which the correctly rounded result is the double nearest `pi/2` |

## Sources

- Microsoft, "ATAN function" —
  <https://support.microsoft.com/en-us/office/atan-function-50746fa8-630a-406b-81d0-4a2aed395543>
  — the canonical worksheet article. **Not retrieved for this pass** (HTTP 403). There is no
  Microsoft Learn `WorksheetFunction.Atan` page; the projection
  `data/functions/FUNC.ATAN.json` carries Microsoft's English one-line description verbatim and is
  the only documented text this page uses.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 4 section 4.4 — the
  arctangent: principal range, series, addition formula and continued fraction.
- Cody & Waite, *Software Manual for the Elementary Functions*, chapter on `ATAN` — the classical
  interval reduction and the split constants.
- `fdlibm` `s_atan.c` — the published reference implementation.
- Muller, *Elementary Functions: Algorithms and Implementation* — minimax approximation and
  last-bit behaviour.
- Handbook projections `data/functions/FUNC.ATAN.json` (arity, `NumToNum` kernel signature,
  `real_result_policy` with `arg_domain_guard=none;non_finite=allow`) and
  `data/presence/FUNC.ATAN.json` (implementing module; no discrepancy, math-deviation or defect
  entries).
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md); related pages
  [ATAN2](FUNC.ATAN2.md) and [ACOT](FUNC.ACOT.md).
