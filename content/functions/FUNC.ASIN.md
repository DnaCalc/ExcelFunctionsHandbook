---
schema: efh.function-page/v1
function_id: FUNC.ASIN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Asin method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.asin"
    role: "documented description, the [-1, 1] argument constraint, the -pi/2 to pi/2 range, and the degrees remark"
  - work: "Microsoft Support — ASIN function"
    locator: "https://support.microsoft.com/en-us/office/asin-function-81fb95e5-6d6f-48c4-bc45-58f955c6d347"
    role: "the canonical worksheet article; not retrieved for this pass (fetch returned HTTP 403)"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 4 (Elementary Transcendental Functions), section 4.4"
    locator: "4.4.1-4.4.40, inverse circular functions"
    role: "definitions, principal ranges, series, and the identities relating arcsine to arctangent and arccosine"
  - work: "Cody & Waite, Software Manual for the Elementary Functions"
    locator: "chapter on ASIN/ACOS"
    role: "the |x| = 1/2 domain split and the square-root reduction of the outer band"
  - work: "fdlibm, e_asin.c"
    locator: null
    role: "the published reference implementation of that split, including the compensated square root near the endpoints"
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
family: asin
role_in_family: >-
  The principal arcsine on [-1, 1] with range [-pi/2, pi/2]: the odd, increasing member of the
  inverse circular trio, and the coercion chapter's worked example of element-local domain errors
  under array lifting.
---

## What it computes

`ASIN(number)` is the principal inverse sine: the angle in radians whose sine is *number*.

    asin: [-1, 1] -> [-pi/2, pi/2],    sin(asin x) = x

- **Domain**: the closed interval `[-1, 1]`. Outside it no real angle has that sine.
- **Range**: `[-pi/2, pi/2]`, the principal branch. Microsoft's Learn reference states this range
  explicitly.
- **Parity**: odd. `asin(-x) = -asin(x)`, exactly. An implementation should compute on `|x|` and
  restore the sign, which makes the identity hold to the last bit for free.
- **Monotonicity**: strictly increasing, from `-pi/2` at `x = -1` through `0` at the origin to
  `pi/2` at `x = 1`.
- **Derivative**: `d/dx asin x = 1 / sqrt(1 - x^2)` — finite in the interior, infinite at both
  endpoints. Vertical tangents at `x = ±1`.
- **Series about zero**:
  `asin x = x + x^3/6 + 3x^5/40 + 15x^7/336 + ...`, converging on `|x| < 1` but slowly near the
  endpoints. For small `x`, `asin x -> x` with leading correction `x^3/6`, so subnormal arguments
  pass through unchanged.
- **Endpoint behaviour**: with `x = 1 - t`, `asin(x) = pi/2 - sqrt(2t)(1 + t/12 + ...)`. The
  square-root approach to the endpoint is the same phenomenon as in `ACOS`, but here it appears as
  a correction to a value near `pi/2` rather than as the value itself — which changes everything
  about the numerics.
- **Complementary identity**: `asin(x) + acos(x) = pi/2` exactly.
- **Relation to arctangent**: `asin(x) = atan(x / sqrt(1 - x^2))` on the open interval, and
  `asin(x) = atan2(x, sqrt(1-x^2))` in Excel's `x`-first order. The `atan2` form is the one that
  survives the endpoints.
- **Complex continuation**: branch cuts along `(-infinity, -1]` and `[1, +infinity)`, the same
  cuts as `acos` — the real domain is exactly their complement on the real axis.

Abramowitz & Stegun cover the inverse circular functions in chapter 4 section 4.4.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The sine of the angle you want. Required. | — |

Microsoft's Learn reference describes it as "the sine of the angle that you want; must be from -1
to 1".

One argument; the reference engine records an arity of exactly one and a unary numeric
scalar-or-array lift profile. Ordinary numeric slot under the shared coercion rules — see
[Coercion and lifting](../model/02-coercion-and-lifting.md), where **`ASIN` is the worked example**
for the rule that a domain failure inside a lifted array stays element-local instead of collapsing
the whole result.

The misunderstood point is the unit of the result: radians. The Learn reference gives the standard
remedy — multiply by 180/PI(), or use `DEGREES`.

## Result and edge cases

Returns `Number` — an angle in radians in `[-pi/2, pi/2]`.

- **The endpoints** `-1` and `1` are in the domain and return `-pi/2` and `pi/2`. Closed interval;
  an implementation testing `|x| < 1` gets both wrong, and they are the two arguments users pass
  most often.
- **Just outside the endpoints** — one ulp beyond — is a domain failure. As with `ACOS`, this
  matters because the natural producer of an `ASIN` argument is a ratio that ought to be at most
  one and, after rounding, sometimes is not. Clamp before calling.
- **Zero** returns zero, and the sign of zero should survive an odd implementation.
- **Subnormals** pass through unchanged: `asin(x) = x` to within rounding for arguments that small.
- **Large magnitudes**, including anything a text conversion is likely to produce, are domain
  failures.
- **Arrays** lift elementwise, with element-local errors on out-of-domain elements.

The reference engine's projected `real_result_policy` records `arg_domain_guard=none` and
`non_finite=allow`; the `[-1, 1]` test lives in the kernel. The presence projection records that
this surface's module carries its own declared artefacts and appears in the upstream coercion
scenario manifest — `ASIN` is one of the surfaces the coercion model was actually written against,
which is why the model chapter cites it.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | *number* lies outside `[-1, 1]` | Reference engine's kernel; Microsoft's Learn page states the constraint and names no error code |
| `#VALUE!` | The argument does not convert to a number | Shared coercion rules |
| propagated | An error value in the argument is returned unchanged | Shared coercion rules |

**The Learn reference documents a constraint without a consequence.** It says the argument must be
from -1 to 1 and does not say what happens when it is not. The error code above is the reference
engine's; Microsoft's worksheet article, which may name it, was not retrieved for this pass.

## Relationships

- **`ACOS`** — the complementary partner, `ASIN(x) + ACOS(x) = PI()/2`. Same domain, mirrored
  range, opposite monotonicity. The pair is not interchangeable by a sign flip.
- **`ATAN`** — the unrestricted-domain member of the trio. `ASIN(x) = ATAN(x/SQRT(1-x^2))` inside
  the open interval, and that identity is the usual way `ASIN` gets implemented on top of `ATAN`
  when only one of the two is available.
- **`ATAN2`** — `ASIN(x) = ATAN2(SQRT(1-x^2), x)` in Excel's `x`-first order, and this is the form
  that behaves at the endpoints where the plain `ATAN` identity divides by something approaching
  zero.
- **`SIN`** — the forward function. `SIN(ASIN(x)) = x` to rounding across the domain;
  `ASIN(SIN(t))` folds `t` into `[-pi/2, pi/2]`.
- **`ASINH`** — the hyperbolic namesake, defined on *all* reals. Where `ASIN` is bounded in both
  domain and range, `ASINH` is bounded in neither; they share a name and nothing else.
- **`DEGREES`** — the usual consumer.
- **Confused with**: `1/SIN(x)`, which is `CSC`; and with `ACSC`, which Excel does not have.

## Numerical notes

`ASIN`'s error behaviour is the mirror of `ACOS`'s, and the mirror is the point: they are hard in
opposite places, which is why neither can be built naively on the other.

**Where `ASIN` is easy.** Near zero the function is essentially the identity, and any decent
approximation gets full relative accuracy. Near the endpoints the *value* is near `±pi/2`, which is
comfortably far from zero, so the relative accuracy of the answer is not destroyed by the
square-root singularity — the singularity is in the derivative, not in the value. This is exactly
where `ACOS` is hard, because there the answer *is* near zero.

**Where `ASIN` is hard.** The absolute accuracy near the endpoints still depends on computing
`sqrt(1 - x^2)` well, and that is a subtraction of nearly equal quantities:

- Do **not** compute `1 - x*x`. The product rounds first, and near `|x| = 1` that rounding is a
  full ulp of `1`, which is much larger than the quantity being kept.
- Do compute `(1 - x)(1 + x)`. Both factors are exact for doubles near the endpoints (Sterbenz for
  the first, exactness of the sum for the second), and the product loses only its own final
  rounding.

**The standard branch structure**, as in Cody & Waite and `fdlibm`'s `e_asin.c`:

1. For `|x| <= 1/2`, evaluate a minimax rational approximation to `asin` directly. The argument is
   comfortably away from the singularity and the polynomial does all the work.
2. For `1/2 < |x| <= 1`, reduce with the half-angle identity
   `asin(x) = pi/2 - 2*asin(sqrt((1-x)/2))`. The inner argument is at most `1/2`, so branch 1
   evaluates it; the outer subtraction is safe because both terms are of order one and the answer
   is of order one.
3. Restore the sign at the end. Never branch on sign inside the kernel.

Note the asymmetry with `ACOS`: `ASIN` can afford the `pi/2 - ...` subtraction at the endpoints and
`ACOS` cannot, because `ASIN`'s answer there is large and `ACOS`'s is small. Two functions, one
identity, opposite verdicts. This is the single most useful thing to understand about the pair.

**Intrinsic information loss.** The same caution as on the `ACOS` page applies from the other side:
if `x` arrived as the result of a computation, the accuracy of `asin(x)` near the endpoints is
limited by how accurately `x` itself represents the intended sine, and the derivative blows up
there. No implementation can recover information the argument does not carry. Numerical geometry
code that needs a small angle accurately should get it from `ATAN2` of a cross product, not from
`ASIN` of a ratio.

**Self-tests without an oracle.** `ASIN(x) + ASIN(-x)` must be exactly zero if the sign is handled
by symmetry. `ASIN(x) + ACOS(x) - PI()/2` across the domain exposes the branch split of both
functions at once. `SIN(ASIN(x)) - x` is a cheap round trip whose residual profile shows where the
square root is being computed badly.

## What has not been checked

**No evidence record in the Handbook names `FUNC.ASIN`**, and no Handbook vector suite exists for
it. Nobody has checked this function against Excel within the Handbook's record. The presence
projection records no entries for this surface in the discrepancy catalogue, the math-deviation
catalogue, or any defect stream.

The battery rendered beside this page is the reference engine's own output, with no Excel involved,
as its own label states.

The documented statements above — the `[-1, 1]` constraint, the `-pi/2` to `pi/2` range, and the
degrees remark — come from Microsoft's Learn `WorksheetFunction.Asin` reference, which was
retrieved. Microsoft's worksheet article was not retrieved for this pass (HTTP 403).

Probes worth running first:

1. **`ASIN(1)` and `ASIN(-1)`** — the closed endpoints, and the cheapest test of `<=` versus `<`.
2. **One ulp outside each endpoint**, to pin the domain boundary and the undocumented error code.
3. **A sweep at `x = 1 - 2^-k`** for increasing `k` against a high-precision reference. The
   residual profile distinguishes the half-angle reduction from a naive `sqrt(1 - x*x)` form, and
   it is the probe most likely to find a last-bit disagreement.
4. **`ASIN(x) + ASIN(-x)`** across the domain — exact oddness, or not. A nonzero result identifies
   an implementation that does not fold the sign, and that is a defect worth knowing about because
   downstream code relies on the parity.
5. **`ASIN(x) + ACOS(x) - PI()/2`** — the complementary identity as a joint fingerprint of the two
   surfaces' branch structures.
6. **Subnormal and very small arguments**, to confirm the pass-through.
7. **A mixed array**, one element in domain and one out, to confirm the element-local error policy
   that the coercion chapter states provisionally and names `ASIN` as the example of.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| principal branch | The choice of range `[-pi/2, pi/2]` that makes the inverse single-valued |
| half-angle reduction | Rewriting `asin(x)` as `pi/2 - 2*asin(sqrt((1-x)/2))` for `|x| > 1/2` |
| factored square root | Computing `sqrt((1-x)(1+x))` rather than `sqrt(1 - x*x)` |
| element-local error | A lifted array result in which one bad element errors without collapsing the array |
| exact oddness | `f(-x) = -f(x)` holding bit for bit, achieved by computing on `|x|` and restoring the sign |

## Sources

- Microsoft Learn, "WorksheetFunction.Asin method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.asin> (retrieved:
  description, the `-1` to `1` constraint, the `-pi/2` to `pi/2` range, and the degrees remark).
- Microsoft, "ASIN function" —
  <https://support.microsoft.com/en-us/office/asin-function-81fb95e5-6d6f-48c4-bc45-58f955c6d347>
  — the canonical worksheet article. **Not retrieved for this pass** (HTTP 403).
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 4 section 4.4 — inverse
  circular functions: ranges, series and identities.
- Cody & Waite, *Software Manual for the Elementary Functions*, chapter on `ASIN`/`ACOS` — the
  `|x| = 1/2` split and the square-root reduction.
- `fdlibm` `e_asin.c` — the published reference implementation.
- Handbook projections `data/functions/FUNC.ASIN.json` (arity, lift profile, `real_result_policy`
  with `arg_domain_guard=none;non_finite=allow`) and `data/presence/FUNC.ASIN.json` (implementing
  module with declared artefacts present; appearance in the upstream coercion scenario manifest; no
  discrepancy, math-deviation or defect entries).
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md) — the array-lift baseline,
  which uses `ASIN`'s domain error as its worked example of element-local failure.
