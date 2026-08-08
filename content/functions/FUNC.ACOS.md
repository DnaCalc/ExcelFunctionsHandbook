---
schema: efh.function-page/v1
function_id: FUNC.ACOS
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Acos method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.acos"
    role: "documented description, the [-1, 1] argument constraint, the 0-to-pi range, and the radians-to-degrees remark"
  - work: "Microsoft Support — ACOS function"
    locator: "https://support.microsoft.com/en-us/office/acos-function-cb73173f-d089-4582-afa1-76e5524b5d5b"
    role: "the canonical worksheet article; not retrieved for this pass (fetch returned HTTP 403)"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 4 (Elementary Transcendental Functions), section 4.4"
    locator: "4.4.1-4.4.40, inverse circular functions"
    role: "definitions, principal ranges, the complementary identity, series and the relations between arcsine, arccosine and arctangent"
  - work: "Cody & Waite, Software Manual for the Elementary Functions"
    locator: "chapter on ASIN/ACOS"
    role: "the classical square-root reduction that moves the hard end of the domain onto the easy one"
  - work: "fdlibm, e_acos.c"
    locator: null
    role: "the reference implementation of the near-one branch and its half-angle reduction"
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
family: acos
role_in_family: >-
  The principal arccosine on [-1, 1] with range [0, pi]; the decreasing member of the inverse
  circular trio, and the one whose hard ends are both endpoints rather than one.
---

## What it computes

`ACOS(number)` is the principal inverse cosine: the angle in radians whose cosine is *number*.

    acos: [-1, 1] -> [0, pi],    cos(acos x) = x

- **Domain**: the closed interval `[-1, 1]`. Outside it there is no real angle whose cosine is
  *number*, and the function is undefined over the reals.
- **Range**: `[0, pi]`, the principal branch. Microsoft's Learn reference states this range
  explicitly: the returned angle is given in radians in the range 0 to pi.
- **Monotonicity**: strictly *decreasing*. `acos(-1) = pi`, `acos(0) = pi/2`, `acos(1) = 0`. It is
  the only one of the three inverse circular primaries that runs downhill, which is a frequent
  source of sign errors when it is substituted for `ASIN`.
- **Reflection**: `acos(-x) = pi - acos(x)`. The function is neither even nor odd; it is
  antisymmetric about the point `(0, pi/2)`.
- **Complementary identity**: `acos(x) + asin(x) = pi/2` exactly, for every `x` in the domain.
  This is the single most useful relation on the page and it is also, in floating point, a
  liability — see the numerical notes.
- **Derivative**: `d/dx acos x = -1 / sqrt(1 - x^2)`, which is finite in the interior and blows up
  at both endpoints. The graph meets `x = 1` and `x = -1` with a vertical tangent.
- **Endpoint behaviour**: near `x = 1`, `acos(x) = sqrt(2(1-x)) * (1 + (1-x)/12 + ...)`. The
  square root is the whole story of this function's numerics: the answer near the endpoints is
  governed by the square root of a quantity that itself has to be computed by subtraction.
- **Series about zero**: `acos x = pi/2 - (x + x^3/6 + 3x^5/40 + ...)`, that is, `pi/2` minus the
  arcsine series. Convergence is slow near the endpoints, which is why nobody implements it this
  way.
- **Complex continuation** (relevant to the `IM*` family, not to this surface): `acos` extends to
  the complex plane with branch cuts along `(-infinity, -1]` and `[1, +infinity)`. The real
  function's domain is exactly the complement of those cuts on the real axis, which is why the
  domain restriction is not arbitrary.

Abramowitz & Stegun treat the inverse circular functions in chapter 4 section 4.4, including the
principal-range conventions used above.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The cosine of the angle you want. Required. | — |

Microsoft's Learn reference describes the argument as "the cosine of the angle you want, and must
be from -1 to 1".

One argument; the reference engine records an arity of exactly one and classifies the surface as
a unary numeric scalar-or-array kernel, so arrays lift elementwise. Ordinary numeric slot under the
shared coercion rules — see [Coercion and lifting](../model/02-coercion-and-lifting.md).

The commonly misunderstood point is not the slot but the units of the *result*: `ACOS` returns
radians. Readers who work in degrees need `DEGREES(ACOS(x))`, and the Learn reference gives the
same advice in the form "multiply it by 180/PI()".

## Result and edge cases

Returns `Number` — an angle in radians in `[0, pi]`.

- **The endpoints** `-1` and `1` are *in* the domain and return `pi` and `0` respectively. This is
  a closed interval; an implementation that tests `|x| < 1` rather than `|x| <= 1` gets both
  endpoints wrong, and they are the two arguments users are most likely to pass.
- **Just outside the endpoints.** A value one ulp beyond `1` is a domain failure. This matters far
  more than it sounds, because the natural producer of `ACOS` arguments is a normalised dot
  product, and rounding routinely pushes such a quotient a bit past `1`. Formulas that compute an
  angle between vectors need to clamp before calling `ACOS`; without a clamp they fail on exactly
  the parallel-vector case they were written for.
- **Zero** returns `pi/2`, and the smallest subnormals also return `pi/2` — the function is flat
  enough there that every subnormal maps to the same double.
- **Large magnitudes** are domain failures, including every value that a text-to-number conversion
  might plausibly produce.
- **Arrays** lift elementwise, and an out-of-domain element produces an element-local error rather
  than collapsing the whole result — the provisional scalar-lift policy described in the coercion
  chapter, with `ACOS` named there as a canonical example of it.

The reference engine's projected `real_result_policy` for `ACOS` records `non_finite=allow` and
`arg_domain_guard=none`; the domain restriction is enforced inside the kernel rather than by a
declared guard axis. The kernel rejects anything outside `[-1, 1]` before evaluating.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | *number* lies outside `[-1, 1]` | Reference engine's kernel; Microsoft's Learn page states the constraint but does not name an error code |
| `#VALUE!` | The argument does not convert to a number | Shared coercion rules |
| propagated | An error value in the argument is returned unchanged | Shared coercion rules |

**This is a documentation gap the Handbook records rather than papers over.** The Learn reference
states that the argument "must be from -1 to 1" and stops there: it does not say what happens when
it is not. The error code in the table above is the reference engine's choice, not a documented
one, and Microsoft's worksheet article — which may well name it — was not retrieved for this pass.

## Relationships

- **`ASIN`** — the complementary partner, via `ACOS(x) + ASIN(x) = PI()/2`. Where `ASIN` is odd
  and increasing, `ACOS` is neither odd nor increasing; the pair is not interchangeable with a
  sign flip.
- **`ATAN`** and **`ATAN2`** — the other route to an angle. `acos(x) = atan2(x, sqrt(1-x^2))` in
  Excel's `x`-first argument order, and that form is better conditioned near the endpoints than
  `ACOS` computed from a series.
- **`COS`** — the forward function. `COS(ACOS(x)) = x` holds to rounding on the whole domain;
  `ACOS(COS(t))` recovers `t` only for `t` in `[0, pi]`, and folds every other `t` into that
  interval.
- **`ACOSH`** — the hyperbolic namesake, and a genuinely different function: its domain is
  `[1, +infinity)`, exactly the region where `ACOS` stops being defined. The two domains touch at
  the single point `x = 1`, where both are zero.
- **`DEGREES`** — the usual consumer of the result.
- **`IMCOS`** and the complex family — where the branch cuts stop being a footnote.
- **Confused with**: `ACOT`, whose range is also stated as 0 to pi. Two different functions with
  the same stated range is a real trap, and the discriminator is the domain: `ACOS` takes
  `[-1, 1]`, `ACOT` takes everything.

## Numerical notes

`ACOS` is a well-conditioned function in the middle of its domain and an ill-conditioned one at
both ends, and every implementation question is about the ends.

**The endpoint problem, stated exactly.** Write `x = 1 - t`. Then `acos(x) ~ sqrt(2t)`. A relative
error `eps` in `t` becomes a relative error `eps/2` in the answer — that part is benign. The
problem is upstream: when `x` is a double near `1`, forming `1 - x` is exact by Sterbenz's lemma,
but *the information was already lost* if `x` itself came from a computation. The spacing of
doubles near `1` is about `2^-52`, so the smallest nonzero `acos` reachable from a double argument
is around `2^-26` — roughly `1.5e-8`. Half the exponent range of the answer is simply unreachable.
This is intrinsic to the argument being a cosine, not a defect of any implementation, and it is
why numerical geometry code computes angles from `ATAN2` of a cross product and dot product rather
than from `ACOS` of a normalised dot product.

**The complementary identity as a trap.** `acos(x) = pi/2 - asin(x)` is exact in mathematics and
catastrophic in floating point for `x` near `1`: `asin(x)` is then near `pi/2`, and the
subtraction of two nearly equal quantities destroys the relative accuracy of a result that is
itself near zero. An implementation of `ACOS` built on `ASIN` this way is accurate near `x = -1`
and wrong near `x = +1`. The mirror trap catches `ASIN` built on `ACOS`. Cody & Waite state the
rule: compute whichever of the pair is *large* directly, and get the small one another way.

**The standard remedies**, in the form the reference implementations use:

1. **Split the domain at `|x| = 1/2`.** For `|x| <= 1/2`, evaluate a rational (minimax)
   approximation to `asin` and set `acos(x) = pi/2 - asin(x)`; here both terms are comparable in
   size and the subtraction is harmless.
2. **For `x` in `(1/2, 1]`, reduce by the half-angle identity**: `acos(x) = 2 * asin(sqrt((1-x)/2))`.
   The argument of the inner `asin` is small, so the accurate small-argument branch does the work,
   and the square root supplies the endpoint behaviour exactly.
3. **For `x` in `[-1, -1/2)`, use the reflection** `acos(x) = pi - acos(-x)` to move onto the
   branch above. This subtraction is safe because the result is near `pi`, not near zero.
4. **Never evaluate `sqrt(1 - x*x)`** near the endpoints. The product `x*x` rounds, and the
   subtraction then loses bits that `(1-x)*(1+x)` retains. The factored form is the standard
   substitute and costs one extra multiply.

`fdlibm`'s `e_acos.c` is the canonical published implementation of exactly this structure.

**Correct rounding.** `ACOS` is not correctly rounded in any mainstream library; the usual quality
target is under one ulp over the domain, with the last-bit behaviour near the endpoints being the
part that differs between vendors. That is where a comparison against Excel would be expected to
find its residuals, and no such comparison exists here.

**Symmetry as a self-test.** `ACOS(x) + ACOS(-x)` should be `PI()` and `ACOS(x) + ASIN(x)` should
be `PI()/2`. Neither will hold to the last bit for every `x` in any implementation — but *how*
they fail is a fingerprint of the branch structure, and it is measurable without an oracle.

## What has not been checked

**No evidence record in the Handbook names `FUNC.ACOS`**, and no Handbook vector suite exists for
it. Nobody has checked this function against Excel within the Handbook's record. The battery
rendered beside this page is the reference engine's own output, with no Excel involved, and it is
labelled as such.

The documented statements above — the `[-1, 1]` constraint, the `0` to `pi` range, and the
degrees remark — come from Microsoft's Learn `WorksheetFunction.Acos` reference, which was
retrieved. Microsoft's worksheet-function article for `ACOS` was **not** retrieved (HTTP 403), so
anything it says about error codes is unknown to this page.

Probes worth running first:

1. **`ACOS(1)` and `ACOS(-1)`** — the closed-interval endpoints, which decide whether the
   implementation uses `<=` or `<`. Two probes, and they settle the most common failure.
2. **One ulp outside each endpoint** — `1 + 2^-52` and `-1 - 2^-52` — to pin the exact
   domain boundary and to confirm the error code, which is documented nowhere.
3. **A sweep just inside `x = 1`** at `1 - 2^-k` for increasing `k`, compared against a
   high-precision reference. This is the single most informative probe on the page: it exposes
   whether the implementation uses the half-angle reduction or the naive `pi/2 - asin` form, and
   the two produce visibly different error profiles.
4. **`ACOS(x) + ASIN(x) - PI()/2`** across the domain — a metamorphic probe needing no oracle,
   whose residual pattern identifies the branch split point.
5. **`ACOS(0)` and `ACOS` of the smallest subnormal**, to confirm both land on the same
   `pi/2` double.
6. **Array arguments with a mix of in-domain and out-of-domain elements**, to confirm that the
   element-local error policy holds here as the coercion chapter's provisional rule states.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| principal branch | The choice of range `[0, pi]` that makes the inverse single-valued |
| complementary identity | `acos(x) + asin(x) = pi/2` |
| half-angle reduction | Rewriting `acos(x)` as `2*asin(sqrt((1-x)/2))` to move the endpoint onto an accurate branch |
| endpoint conditioning | The square-root behaviour at `x = ±1` that costs half the significant digits |
| element-local error | A lifted array result in which one bad element errors without collapsing the array |

## Sources

- Microsoft Learn, "WorksheetFunction.Acos method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.acos> (retrieved:
  description, the `-1` to `1` argument constraint, the `0` to `pi` radian range, and the
  degrees-conversion remark).
- Microsoft, "ACOS function" —
  <https://support.microsoft.com/en-us/office/acos-function-cb73173f-d089-4582-afa1-76e5524b5d5b>
  — the canonical worksheet article. **Not retrieved for this pass** (HTTP 403); no statement here
  rests on it.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 4 section 4.4 — inverse
  circular functions: principal ranges, the complementary and reflection identities, and series.
- Cody & Waite, *Software Manual for the Elementary Functions*, chapter on `ASIN`/`ACOS` — the
  domain split and the square-root reduction.
- `fdlibm` `e_acos.c` — the published reference implementation of the near-endpoint branch.
- Handbook projections `data/functions/FUNC.ACOS.json` (arity,
  `UnaryNumericScalarOrArrayElementwise` lift profile, `real_result_policy` with
  `arg_domain_guard=none;non_finite=allow`) and `data/presence/FUNC.ACOS.json`.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md) — the elementwise lift and
  the element-local error policy, where `ASIN`-style domain errors are the worked example.
