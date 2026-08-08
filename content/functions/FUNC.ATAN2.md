---
schema: efh.function-page/v1
function_id: FUNC.ATAN2
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0007
  - EV-MISC-0013
  - EV-STRUCT-0011
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Atan2 method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.atan2"
    role: "the x-first argument order, the (-pi, pi] range statement, the five documented quadrant identities, and the statement that both arguments zero returns an error value"
  - work: "Microsoft Support — ATAN2 function"
    locator: "https://support.microsoft.com/en-us/office/atan2-function-c04592ab-b9e3-4908-b428-c96b3a565033"
    role: "the canonical worksheet article; not retrieved for this pass (fetch returned HTTP 403)"
  - work: "IEEE Std 754-2019, Standard for Floating-Point Arithmetic"
    locator: "clause 9.2, atan2 and the signed-zero conventions"
    role: "the standard two-argument arctangent, including its total behaviour at zeros and infinities"
  - work: "Kahan, Branch Cuts for Complex Elementary Functions"
    locator: null
    role: "why the two-argument form exists at all, and why signed zero determines which side of the cut a point lies on"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 4 (Elementary Transcendental Functions), section 4.4"
    locator: "4.4.x, arctangent and the quadrant conventions"
    role: "the underlying arctangent and the quadrant adjustments the two-argument form encodes"
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
family: atan2
role_in_family: >-
  The four-quadrant arctangent, with Excel's x-first argument order; the surface on which a
  documented identity that forms the y/x ratio is itself the mechanism of an identified departure
  from the mathematical function.
---

## What it computes

`ATAN2(x_num, y_num)` is the four-quadrant inverse tangent: the angle from the positive x-axis to
the ray through the origin and the point `(x, y)`.

    atan2: R^2 \ {(0,0)} -> (-pi, pi]

Microsoft's Learn reference states the geometry and the range directly: the arctangent is the angle
from the x-axis to a line containing the origin and a point with coordinates `(x_num, y_num)`, and
the angle is given in radians between `-pi` and `pi`, excluding `-pi`.

### The argument order is the trap

**Excel takes `x` first.** Nearly every other implementation of this function in nearly every other
language — C, Python, Java, JavaScript, Fortran, Rust — takes `y` first, as `atan2(y, x)`. Excel's
signature is `ATAN2(x_num, y_num)`. Microsoft's Learn parameter table is unambiguous: `Arg1` is the
x-coordinate, `Arg2` is the y-coordinate.

A formula ported in either direction without swapping the arguments computes the reflection of the
intended angle about the line `y = x`, which is wrong by an amount that varies with the point and
is therefore not detectable by a spot check. This is the single most consequential fact on the
page and it is why the Handbook states it before the mathematics.

### The mathematics

The function is the **argument** (phase) of the point, the same object as `arg(x + iy)` in the
complex plane. Microsoft's Learn reference documents it as a set of quadrant identities:

| Condition | Documented identity |
|---|---|
| `x > 0` | `ATAN2(x,y) = ATAN(y/x)` |
| `y >= 0`, `x < 0` | `ATAN2(x,y) = ATAN(y/x) + PI()` |
| `y < 0`, `x < 0` | `ATAN2(x,y) = ATAN(y/x) - PI()` |
| `y > 0`, `x = 0` | `ATAN2(x,y) = PI()/2` |
| `y < 0`, `x = 0` | `ATAN2(x,y) = -PI()/2` |
| `x = 0` and `y = 0` | returns an error value |

Structural properties:

- **Domain**: the punctured plane. Every point except the origin has a well-defined argument;
  at the origin the angle is genuinely undefined, not merely inconvenient.
- **Range**: `(-pi, pi]`. The value `pi` is attained (on the negative x-axis with `y >= 0`) and
  `-pi` is not — the half-open convention that makes the function single-valued.
- **Branch cut**: the negative x-axis. Crossing it, the value jumps by `2pi`. `ATAN2` is
  continuous on the plane minus that ray and discontinuous across it; this is not an artefact but
  the unavoidable consequence of representing a circle-valued quantity by a real number.
- **Homogeneity**: `atan2(t*x, t*y) = atan2(x, y)` for every `t > 0`. The function depends only on
  the *direction* of the point, not on its magnitude. **This is the property that matters most
  numerically, and it is the one the departure described below violates.**
- **Symmetries**: `atan2(x, -y) = -atan2(x, y)` when `y != 0`; and `atan2(-x, y) = pi - atan2(x, y)`
  for `y > 0`.
- **Relation to the one-argument form**: `atan2(x, y) = atan(y/x)` only for `x > 0`. The whole
  reason the two-argument function exists is that the ratio `y/x` discards the information needed
  to place the quadrant — and, as the next section shows, discards more than that.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `x_num` | The **x**-coordinate of the point. Required. | — |
| `y_num` | The **y**-coordinate of the point. Required. | — |

Two arguments; the reference engine records an arity of exactly two, a `NumsToNum` kernel
signature and a `Custom` coercion-lift profile. Ordinary numeric slots under the shared coercion
rules — see [Coercion and lifting](../model/02-coercion-and-lifting.md).

The misunderstood position is, of course, the order — see above. A secondary one: these are
*coordinates*, not a slope and a length. `ATAN2(1, 1)` is `pi/4` because the point is on the
diagonal, not because `1/1 = 1`.

## Result and edge cases

Returns `Number` — an angle in radians in `(-pi, pi]`.

- **The origin.** Microsoft's Learn reference says that if both `x` and `y` are 0, the function
  "returns an error value" — **without naming which**. The reference engine returns `#DIV/0!`. The
  Handbook records the code as the reference engine's and the *existence* of the error as
  documented; which code Excel actually returns is not established here, and the discrepancy
  between "an error value" and a specific code is exactly the sort of documentation gap this
  Handbook publishes rather than guesses past.
- **On the axes.** `x = 0` with `y` nonzero gives `±pi/2`, documented. `y = 0` with `x > 0` gives
  `0`; `y = 0` with `x < 0` gives `pi` — note that the documented identity table routes this
  through the `y >= 0, x < 0` row, so `pi` and not `-pi`, consistent with the half-open range.
- **Signed zero.** The IEEE two-argument arctangent uses the *sign* of a zero argument to decide
  which side of the branch cut a point lies on: `atan2(-1, +0)` is `pi` and `atan2(-1, -0)` is
  `-pi`. Whether Excel's value model can express that distinction at all is an open question — see
  [The value universe](../model/01-value-universe.md) — and it interacts with the fact that Excel's
  range excludes `-pi`.
- **Equal-magnitude arguments** give the diagonal angles exactly, or as exactly as the constants
  allow.
- **Arrays.** Both argument positions broadcast; the reference engine routes this surface through a
  binary numeric broadcast evaluator, and the evidence record `EV-STRUCT-0011` concerns exactly
  that routing.

### The ratio-overflow departure

This is the substantive finding on the page, and it follows directly from the documented identity
table.

Mathematically, the angle of a point is finite for *every* point except the origin, and depends
only on the point's direction. A genuine two-argument arctangent never forms `y/x`; it compares
magnitudes and works with the smaller-over-larger ratio, which is always at most 1. Forming `y/x`
directly reintroduces exactly the overflow the two-argument form exists to avoid: if `y` is huge
and `x` is tiny, the quotient overflows to infinity even though the answer is an ordinary angle
very close to `±pi/2`.

**`EV-MATH-0007` and `EV-MISC-0013` both carry `FUNC.ATAN2` as a subject**, and both concern the
same upstream math-deviation row: an error is published exactly when `x` is nonzero and the
implicit `y/x` ratio overflows, while the axis case `x = 0` — where the ratio is undefined rather
than overflowing — keeps its `±pi/2` value. `EV-MISC-0013`'s own status text puts it plainly: the
four-quadrant angle is finite everywhere off the origin, so this is the ratio being formed first
and the overflow being inherited; a two-argument `atan2` would never do it.

Both records carry counts, and both carry a warning the Handbook must repeat rather than
paraphrase away: **what was counted is an error-placement boundary, not the bits of the returned
angle.** `EV-MISC-0013` states that it publishes the weaker reading — the boundary was measured,
not the angle — and that its count rides on the numeric axis only by the register's convention.
The figures are rendered with the records and are not restated here. **No claim about the accuracy
of the angle itself follows from either record**, and this page makes none.

The reference engine reproduces the boundary deliberately: its kernel guards `x != 0` and a
non-finite `y/x`, and returns `#NUM!`. That is a compatibility choice, not an endorsement.

**The documentation predicts the departure.** Microsoft's own identity table says
`ATAN2(x,y) = ATAN(y/x)` for `x > 0`. Taken as a specification of the *evaluation*, that identity
requires forming the ratio, and forming it in binary64 requires it to be representable. So the
documented identity and the identified behaviour agree with each other — and both depart from the
mathematical function they claim to compute. That is an unusually clean example of a specification
written in mathematics being implemented as arithmetic.

## Errors

| Error | Condition | Source |
|---|---|---|
| an error value | `x_num` and `y_num` are both zero | Microsoft Learn, which does not name the code; the reference engine returns `#DIV/0!` |
| `#NUM!` | `x_num` is nonzero and the implicit `y/x` ratio overflows | `EV-MATH-0007` / `EV-MISC-0013` and the reference engine's kernel — **documented nowhere** |
| `#VALUE!` | An argument does not convert to a number | Shared coercion rules |
| propagated | An error value in either argument is returned unchanged | Shared coercion rules |

The second row is the interesting one: an error condition that exists on record and appears in no
documentation the Handbook has retrieved.

## Relationships

- **`ATAN`** — the one-argument form. `ATAN2(x, y) = ATAN(y/x)` for `x > 0` only, and the
  restriction is the reason `ATAN2` exists. Note that `ATAN` itself never overflows, because the
  caller has already done the division and taken the consequences.
- **`ACOT`** — `ACOT(x) = ATAN2(x, 1)` in Excel's order, and that formulation is better conditioned
  at large `|x|` than `PI()/2 - ATAN(x)`. See the [ACOT](FUNC.ACOT.md) page.
- **`IMARGUMENT`** — the complex-number surface computing the same quantity, `arg(x + iy)`. If the
  two disagree anywhere, one of them is wrong, and that is a cheap cross-check the Handbook has not
  run.
- **`ASIN` / `ACOS`** — both are better computed through `ATAN2` than through their own closed
  forms when the endpoints matter: `asin(t) = ATAN2(SQRT(1-t^2), t)` and
  `acos(t) = ATAN2(t, SQRT(1-t^2))` in Excel's order.
- **`DEGREES`** — the usual consumer; the Learn reference gives the 180/PI() remark.
- **`SQRT`, `SUMSQ`** — the magnitude half of a polar conversion, of which `ATAN2` is the angle
  half.
- **Confused with**: `ATAN`, whenever the quadrant matters; and with every other language's
  `atan2`, whose argument order is reversed.

## Numerical notes

**How a correct two-argument arctangent is built.** The standard construction has three steps and
never forms a quotient larger than 1:

1. Compare `|x|` and `|y|`. Compute `r = min(|x|,|y|) / max(|x|,|y|)`, which lies in `[0, 1]` and
   cannot overflow. If both are zero, the angle is undefined — that is the only true failure.
2. Evaluate `atan(r)`, which is the accurate, well-conditioned part.
3. Reflect into the correct octant and quadrant using the signs of `x` and `y` and whether the
   swap happened, adding `pi/2`, `pi` or negating as required.

The construction is total on the punctured plane, never overflows, never underflows in a way that
matters, and is accurate for every direction including the extremely elongated ones. IEEE 754
clause 9.2 specifies the behaviour including at zeros and infinities.

**Why the ratio form is worse than it looks.** Beyond the overflow, `y/x` loses the homogeneity
property: `atan2(t*x, t*y)` should be independent of `t`, and forming a quotient makes it depend on
whether that quotient happens to be representable. So a formula that scales its coordinates — a
completely ordinary thing to do — can change its answer from an angle to an error. There is no
scaling that fixes it in general, because the failing cases are exactly the ones with extreme
aspect ratios.

**Underflow is the mirror hazard.** If `y` is tiny and `x` is huge, `y/x` underflows to zero and
the angle comes back as exactly `0` or `pi` rather than as a tiny nonzero angle. That is the same
defect at the other end, and it produces a *wrong number* rather than an error — the worse of the
two outcomes. Whether it occurs is a separate question from the overflow, and the Handbook does not
know the answer for Excel; no record addresses it.

**The `pi` constant appears twice.** The quadrant adjustments add `pi` or `pi/2`. As with `ATAN`,
a careful implementation uses a two-part split constant so the addition does not eat the accuracy
of a result that may be small. An angle just below `pi` computed as `pi_double - small` inherits
the representation error of `pi_double`.

**Signed zero decides the cut.** `atan2(-1, +0) = pi` and `atan2(-1, -0) = -pi` under IEEE; this is
how the standard lets a computation stay on the correct side of the branch cut. Excel's documented
range excludes `-pi`, which means either that the distinction is not expressible or that the
documented range is approximate. Nobody has checked which.

## What has not been checked

Three evidence records name `FUNC.ATAN2` as a subject, and it is worth being precise about what
each does and does not give.

- **`EV-MATH-0007`** and **`EV-MISC-0013`** both derive from the same upstream math-deviation row
  and both carry a per-surface count with the Excel build restated on the same line — which the
  latter record notes is rare in this evidence base. **Both carry the warning that what was counted
  is the error-placement boundary and not the returned angle.** The Handbook therefore has evidence
  about *where an error appears* and no evidence at all about *how accurate the angle is*.
- **`EV-STRUCT-0011`** is a structural-verification record covering an array-lift tranche.
  `FUNC.ATAN2` is a named subject with its own witness row about array-shaped arguments, but the
  record's counts are **group totals across many surfaces with no per-surface split**, and the
  record says so. The array-lift behaviour of this surface was fixed and re-measured as part of a
  group; it was not measured separately.

So the honest summary: the error boundary is on record; the array-lift routing is on record as part
of a group; **the numeric accuracy of the angle is not on record at all**, and no Handbook vector
suite exists for `ATAN2`.

The documented statements above come from Microsoft's Learn `WorksheetFunction.Atan2` reference,
which was retrieved. Microsoft's worksheet article was not (HTTP 403).

Probes worth running first:

1. **`ATAN2(1, 0)`, `ATAN2(0, 1)`, `ATAN2(-1, 0)`, `ATAN2(0, -1)`** — the four axis directions,
   which pin the argument order beyond doubt in four cheap probes. If `ATAN2(0, 1)` is `pi/2`, the
   x-first order is confirmed.
2. **`ATAN2(0, 0)`** — to learn *which* error code the documentation declines to name.
3. **The overflow boundary from both sides**, at several different magnitudes of `x`, to confirm
   that the trigger really is the ratio and not the individual arguments. Two points with the same
   direction and different scales should agree; if they do not, homogeneity is broken and the ratio
   is identified.
4. **The underflow mirror**: `y` tiny, `x` huge. Whether the result is a small angle or exactly
   zero is unaddressed by any record, and a wrong number is worse than an error.
5. **Accuracy of the angle itself** across a grid of directions and magnitudes against a
   high-precision reference. This is the whole unmeasured axis and it is where a vector suite would
   start.
6. **Signed-zero inputs**, if they can be constructed, to test the branch-cut side and the excluded
   `-pi`.
7. **`ATAN2` against `IMARGUMENT`** on the same points — an internal cross-check requiring no
   oracle.
8. **Array arguments in both positions, including mismatched shapes**, given the broadcast routing
   recorded in `EV-STRUCT-0011`.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| four-quadrant arctangent | The two-argument form that recovers the quadrant the ratio `y/x` discards |
| x-first order | Excel's `ATAN2(x_num, y_num)`, the reverse of the near-universal `atan2(y, x)` |
| homogeneity | `atan2(t*x, t*y) = atan2(x, y)` for `t > 0`; the property a ratio-forming implementation loses |
| branch cut | The negative x-axis, across which the value jumps by `2pi` |
| error-placement boundary | Where an implementation switches from a value to an error; what the attached records measured |
| group total | An evidence count spanning many surfaces, from which no per-surface figure may be read |

## Sources

- Microsoft Learn, "WorksheetFunction.Atan2 method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.atan2> (retrieved: the
  x-first parameter table, the `(-pi, pi]` range, the five quadrant identities including
  `ATAN2(x,y) = ATAN(y/x)` for `x > 0`, and the unnamed error value at the origin).
- Microsoft, "ATAN2 function" —
  <https://support.microsoft.com/en-us/office/atan2-function-c04592ab-b9e3-4908-b428-c96b3a565033>
  — the canonical worksheet article. **Not retrieved for this pass** (HTTP 403).
- IEEE Std 754-2019, clause 9.2 — the standard two-argument arctangent and its signed-zero
  conventions.
- Kahan, *Branch Cuts for Complex Elementary Functions* — why the two-argument form exists and how
  signed zero selects the side of the cut.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 4 section 4.4 — the
  arctangent and the quadrant adjustments.
- Handbook evidence records `EV-MATH-0007` and `EV-MISC-0013` (subject `FUNC.ATAN2`) — the
  `y/x`-overflow error boundary, each with its own warning that the boundary and not the angle was
  what was counted; and `EV-STRUCT-0011` (subjects `FUNC.ATAN2`, `FUNC.BASE`) — the array-lift
  tranche, whose counts are group totals with no per-surface split.
- Handbook projections `data/functions/FUNC.ATAN2.json` (arity 2, `NumsToNum`, `Custom` lift
  profile) and `data/presence/FUNC.ATAN2.json` (implementing module; math-deviation entries; the
  `BUG-FUNC-017` and `BUG-FUNC-027` defect streams).
- OxFunc `crates/oxfunc_core/src/functions/atan2.rs` at commit `473efa3` — the kernel guard that
  reproduces the boundary.
- Handbook [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md); related page
  [ATAN](FUNC.ATAN.md).
