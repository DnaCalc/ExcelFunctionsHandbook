---
schema: efh.function-page/v1
function_id: FUNC.DEGREES
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0008
open_problems: []
references:
  - work: "Microsoft Support — DEGREES function"
    locator: "https://support.microsoft.com/en-us/office/degrees-function-4d6ec4db-e694-4b94-ace0-1cc3f61f9ba1"
    role: "documented signature and the radians-to-degrees description"
  - work: "Muller et al., Handbook of Floating-Point Arithmetic"
    locator: "the chapters on correctly rounded scaling and on double rounding"
    role: "why x*(a/b) and (x*a)/b are different functions in binary64"
  - work: "Goldberg, What Every Computer Scientist Should Know About Floating-Point Arithmetic"
    locator: "the sections on rounding error in a single operation"
    role: "the one-rounding-versus-two argument in its standard form"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 4.3 (Circular Functions)"
    role: "the radian measure this function converts out of"
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
family: degrees
role_in_family: >-
  The radians-to-degrees converter — arithmetically the simplest surface in the trigonometric
  neighbourhood, and one whose entire content is the choice of a single multiply-or-divide.
---

# DEGREES

## What it computes

`DEGREES(angle)` converts an angle from radians to degrees. Microsoft's page says exactly that
and gives the canonical example, `π` radians to 180 degrees.

    DEGREES(x) = x · 180 / π

| Property | Statement |
|---|---|
| Domain | all real `x` |
| Range | all real values the format can hold; the map is a bijection on the reals |
| Linearity | `DEGREES(ax + by) = a·DEGREES(x) + b·DEGREES(y)` — exactly, in the mathematics |
| Parity | odd: `DEGREES(-x) = -DEGREES(x)` |
| Fixed point | only `x = 0` |
| Inverse | `RADIANS(y) = y · π / 180` |
| Scale factor | `180/π = 57.29577951308232…`, an irrational, hence never exactly a double |
| Overflow | the true value exceeds the double range once `ABS(x)` passes roughly `3.14 × 10^306` |

There is no analysis here at all: it is one multiplication by a constant. What makes the page
worth writing is that a single multiplication by an *irrational* constant is exactly the
situation in which two obviously-equivalent formulas stop being equivalent, and this function is
the cleanest example of it on the entire worksheet surface.

## Arguments

| Argument | Meaning | Notes |
|---|---|---|
| `angle` | The angle in radians. Required. | Microsoft documents no constraint |

Ordinary to-number coercion applies; the reference engine declares the surface a scalar kernel
that lifts elementwise over arrays. See
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`.

- **Zero** — `DEGREES(0)` is `0`, exactly, and it is the only exactly-preserved value other
  than the sign structure.
- **`DEGREES(PI())`** — not exactly `180`. `PI()` is the double nearest `π`, not `π`, so the
  product is the scaled image of a slightly wrong angle. Whether it rounds *to* `180` anyway is
  a question about the rounding of one multiplication, and the answer depends on which of the
  two op-graphs below is used. This is the single most-noticed behaviour of this function.
- **`DEGREES(RADIANS(x))`** — not `x` in general. Two roundings by reciprocal irrational factors
  do not compose to the identity, and the round trip loses up to about one ulp.
- **Overflow** — `180/π ≈ 57.3`, so the product overflows for `ABS(x)` above roughly
  `3.1 × 10^306`, well inside the double range of the *argument*. `EV-MATH-0008` names a
  very large `DEGREES` argument among its witnesses for the convention that a non-finite real
  result surfaces as an error rather than as an infinity, and the reference engine carries
  `non_finite=num` for this surface accordingly. The record publishes **no count**; it
  establishes where an error code appears, with witnesses and no denominator.
- **Underflow** — for subnormal arguments the product is scaled *up*, so `DEGREES` of a
  subnormal is generally a normal number, and the operation recovers precision rather than
  losing it. This direction is harmless.

## Errors

Microsoft's `DEGREES` page documents no error conditions.

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | Non-numeric argument | Shared call model, not this page |
| `#NUM!` | The scaled result is not finite | `EV-MATH-0008`, which names `DEGREES` — a kind and error-code convention, no count |

## Relationships

- **`RADIANS`** — the inverse, `y · π / 180`. The pair is the standard round-trip probe and
  neither direction is exact.
- **[COS](FUNC.COS.md), `SIN`, `TAN`** — the functions that make this one necessary, all of
  which take radians. Microsoft's `COS` page recommends `RADIANS()` or multiplication by
  `PI()/180` for exactly this reason.
- **`PI()`** — the constant this function is scaled by, and the source of its irreducible error.
- **`ACOS`, `ASIN`, `ATAN`, `ATAN2`** — the inverse trigonometric functions, which return
  radians and are therefore the usual callers of `DEGREES`.
- **`CONVERT`** — the general unit converter, which also handles angle units. Whether
  `CONVERT(x, "rad", "deg")` and `DEGREES(x)` agree bitwise is a natural question and is
  unchecked here.

## Numerical notes

This function has exactly one design decision, and it decides the last bit of every result.

**Two op-graphs, two different functions.**

    (A)  result = x * (180/π)        — one rounding of the constant, one rounding of the product
    (B)  result = (x * 180) / π      — `x*180` is exact when it does not overflow, then one division
    (C)  result = x / (π/180)        — a third variant, with the constant rounded the other way

Form (A) rounds `180/π` to a double first — an error of up to half an ulp in the *constant*,
which becomes a relative error of up to half an ulp in every result — and then rounds the
product. Two roundings, so up to one ulp of error, and the error is systematic: it has the same
sign for every argument, because the constant is wrong in one fixed direction.

Form (B) multiplies by `180` first. Since `180 = 4 · 45` and `45` needs six significand bits,
`x * 180` is **exact** whenever it does not overflow — no rounding at all. Then one division by
the rounded `π`. That is one rounding of the constant and one of the division, so nominally the
same count, but the errors do not align the same way and the results differ on a substantial
fraction of inputs. Form (B) also overflows earlier, at `ABS(x)` near `10^306`, because the
intermediate `x*180` is larger than the result.

Form (C) divides by a differently-rounded constant again.

None of the three is "wrong"; they are three functions that agree to within an ulp and disagree
in the last bit. Which one a spreadsheet uses is a fact about the spreadsheet, and it is exactly
the kind of fact the Handbook exists to record. **The Handbook has not determined which form
Excel uses**, and this page will not guess.

**Why it is worth determining.** `DEGREES` is a scaling, so its error is purely relative and
constant across the range — there is no argument reduction, no cancellation, no conditioning
problem anywhere. That makes it an unusually clean probe: any disagreement between two
implementations is entirely attributable to the op-graph, with no confounding effects. It is
the ideal warm-up target for op-graph identification, and a resolved answer here would be a
model for the harder surfaces in this batch.

**The correctly-rounded target.** A `math-correct` implementation computes `x · 180/π` with the
constant to double-double precision and rounds once. That is achievable with a two-term split of
`180/π` and one fused multiply-add, and it differs from all three forms above on the arguments
where they are worst. Muller et al.'s *Handbook of Floating-Point Arithmetic* gives the
technique and the analysis; Goldberg's paper gives the underlying one-rounding-versus-two
argument in its standard form.

## What has not been checked

`EV-MATH-0008` names this surface. It is a kind-and-error-code convention record with named
witnesses and, in its own words, **no row count anywhere**: it establishes that a non-finite
real result surfaces as an error, with a large `DEGREES` argument among the witnesses, and it
carries no numeric-bits comparison count for any of its surfaces. That is the whole of the
Handbook's evidence for `DEGREES`.

No Handbook vector suite exists for `DEGREES`. Microsoft's page documents the conversion and
nothing else — no error conditions, no formula, no constraint. **Nobody has checked this
function's values against Excel within the Handbook's record**, and its op-graph is
undetermined.

Inputs I would probe first:

1. **`DEGREES(PI())`, and whether it is exactly 180.** The cheapest probe on the page and
   already discriminating: the three op-graphs do not all agree here.
2. **The op-graph separation set.** Sweep `x` and compare `DEGREES(x)` against `x*(180/PI())`,
   `(x*180)/PI()` and `x/(PI()/180)` computed as worksheet formulas. Because the function is a
   pure scaling, the arguments where the three forms differ can be enumerated in advance, and a
   few hundred of them decide the question outright. This is the identification most likely to
   succeed of anything on this page.
3. **The overflow boundary, bisected**: `DEGREES(1E306)`, `DEGREES(3.1E306)`, `DEGREES(1E307)`.
   Form (B) overflows about two hundred-fold earlier than forms (A) and (C), so the first
   argument returning `#NUM!` separates them by kind, with no bit comparison needed. This is the
   strongest single experiment on the page.
4. **The round trip** `DEGREES(RADIANS(x)) - x` across the range, and `RADIANS(DEGREES(x)) - x`.
   Both are bounded by about one ulp; the *sign pattern* of the residual is a fingerprint of the
   constant's rounding direction.
5. **Exactly-representable results**: `DEGREES(x)` where the true answer is a small integer —
   for `x = PI()/2`, `PI()/4`, `PI()/6`. Whether `90`, `45` and `30` come back exactly says more
   about `PI()` than about `DEGREES`, and separating those two contributions is the point.
6. **Subnormal arguments**, where the scaling moves the value back into the normal range.
7. **`CONVERT(x, "rad", "deg")` against `DEGREES(x)`** — two surfaces that should agree and may
   not.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| op-graph | The exact sequence of floating-point operations an implementation performs |
| scale factor | The constant `180/π`, irrational and therefore never exactly a double |
| exact multiplication | A product that needs no rounding; `x·180` is one, when it does not overflow |
| one rounding | The `math-correct` target: compute in extra precision, round a single time |
| systematic error | Error with the same sign at every argument, from a constant rounded one way |
| FINITE convention | The recorded convention that a non-finite real result surfaces as an error |

## Sources

- Microsoft, "DEGREES function" —
  <https://support.microsoft.com/en-us/office/degrees-function-4d6ec4db-e694-4b94-ace0-1cc3f61f9ba1>
  (fetched at curation: signature and the radians-to-degrees description with the `π` → 180
  example. No error conditions, formula or constraint are documented there).
- Handbook evidence record `EV-MATH-0008` — the FINITE error-code convention with named
  witnesses including a large `DEGREES` argument, and its explicit statement that it publishes
  no count. Read its reader warning.
- Muller et al., *Handbook of Floating-Point Arithmetic* — correctly rounded scaling by an
  irrational constant; double rounding.
- Goldberg, *What Every Computer Scientist Should Know About Floating-Point Arithmetic*.
- Abramowitz & Stegun, chapter 4.3 — radian measure.
- Handbook, [COS](FUNC.COS.md) — where the converted angle usually goes next;
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.DEGREES.json` (the `non_finite=num` axis value) and
  `data/presence/FUNC.DEGREES.json`.
