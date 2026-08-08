---
schema: efh.function-page/v1
function_id: FUNC.INT
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
family: int_fn
role_in_family: >-
  Sole member of its module; the floor function, and the one rounding primitive in the math
  category that rounds toward negative infinity rather than toward zero.
---

## What it computes

`INT(number)` is the **floor** function: the greatest integer not exceeding its argument.

    INT(x) = ⌊x⌋ = max{ n ∈ ℤ : n ≤ x }

Domain: all real numbers. Range: the integers. The function is non-decreasing, and it is
**discontinuous at every integer** — a jump of exactly one, from the left. That discontinuity is
the whole of its numerical character, and everything in "Numerical notes" below follows from it.

Two identities are worth having in front of you:

- **x = ⌊x⌋ + {x}**, where {x} ∈ [0, 1) is the fractional part. Microsoft's page gives exactly
  this as its worked example: `A2-INT(A2)` extracts the decimal portion.
- **⌊−x⌋ = −⌈x⌉.** Floor and ceiling are reflections of one another through zero, which is why
  `INT` and [ISO.CEILING](FUNC.ISO.CEILING.md) are not independent functions but two views of the
  same operation.

The point every reader eventually trips over is what "round down" means below zero. Microsoft
states it explicitly: "Rounding a negative number down rounds it away from 0", with the worked
example `INT(-8.9)` returning −9, not −8. **`INT` rounds toward negative infinity, not toward
zero.** Truncation toward zero is a different function, and Excel publishes it separately as
`TRUNC`. On the non-negative reals the two agree; on the negative reals they differ by one at
every non-integer, and that is where migrated formulas break.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `number` | "The real number you want to round down to an integer." | Yes |

Exactly one argument; the declared arity is one to one, and there is no optional digits
parameter — `INT` floors to a unit, always. To floor to another multiple you need
[FLOOR.MATH](FUNC.FLOOR.MATH.md) or an explicit scaling.

The slot is numeric and subject to ordinary to-number coercion; see
[coercion and lifting](../model/02-coercion-and-lifting.md). The declared coercion profile is
the unary numeric scalar-or-array-elementwise one, so `INT` is a scalar kernel that lifts over
array arguments elementwise, with element failures staying element-local.

## Result and edge cases

Returns `Number` — always an integer value, never a fraction.

The reference engine's published battery is rendered beside this page by the generator. The
behaviours it exhibits, stated qualitatively:

- **A logical argument** converts and floors; **numeric text** converts and floors. Microsoft's
  page addresses neither.
- **An inline array** produces an array of the same shape, floored elementwise — the scalar-lift
  behaviour, in contrast to the aggregate shape of [GCD](FUNC.GCD.md) and [LCM](FUNC.LCM.md).
- **An empty range** produces `#VALUE!`.
- **A subnormal positive magnitude** floors to zero, as it must.
- **The largest finite double** is returned **unchanged**. This is not a special case: every
  binary64 of magnitude at or above 2^52 is already an exact integer, so `INT` is the identity on
  the entire upper half of the floating-point range. Above 2^52 the function has no work to do,
  and no rounding to get wrong.
- **Negative zero.** Floor of −0 is −0 mathematically and by IEEE 754's `roundToIntegralTowardNegative`;
  whether that sign survives to the worksheet is a publication question, not an arithmetic one.

Because `INT` never changes a value at or above 2^52, its interesting domain is bounded, and a
test suite that samples uniformly over the doubles will spend almost all of its cells in the
region where the function is the identity. That is worth knowing before designing one.

## Errors

Microsoft's `INT` page documents **no error return of its own**. What errors can appear comes
entirely from the shared call model:

| Error | Condition |
|---|---|
| `#VALUE!` | The argument does not convert to a number under the shared to-number rules |
| propagated | An error value in the argument surfaces as that error |

The reference engine additionally reports `#VALUE!` for arity failures — a call with no argument
or with two. In Excel the zero-argument case is expected to be refused at formula entry rather
than evaluated ([the call pipeline](../model/03-call-pipeline.md)); this is a difference of
surface, not of semantics.

The absence of a documented domain error is itself informative: `INT` is total on the reals, so
there is nothing to refuse.

## Relationships

- **`TRUNC`** — truncation toward zero. Identical to `INT` for non-negative arguments and
  different by one for every negative non-integer. `TRUNC` also takes an optional digits
  argument, which `INT` does not. If a formula was written by someone thinking in C or in most
  BASIC dialects, they meant `TRUNC`.
- **[ISO.CEILING](FUNC.ISO.CEILING.md)** — the reflection, ⌈x⌉ = −⌊−x⌋.
- **[FLOOR.MATH](FUNC.FLOOR.MATH.md)** and **[CEILING.MATH](FUNC.CEILING.MATH.md)** — the
  generalizations to an arbitrary multiple, with an explicit mode argument for the direction on
  negatives. `INT(x)` is `FLOOR.MATH(x)` with default arguments.
- **[MOD](FUNC.MOD.md)** — Microsoft's `MOD` page defines the remainder *in terms of this
  function*: `MOD(n, d) = n - d*INT(n/d)`. `INT` is therefore the primitive under Excel's
  remainder, and `MOD`'s documented sign rule ("the result has the same sign as divisor") is a
  consequence of `INT` flooring rather than truncating.
- **`ROUND`, `ROUNDDOWN`, `ROUNDUP`** — the decimal-place rounding family. `ROUNDDOWN` truncates
  toward zero, so it is `TRUNC`'s sibling rather than `INT`'s, despite the name.
- **`EVEN` and [ODD](FUNC.ODD.md)** — the parity-rounding pair, which round *away* from zero.
- **[ISEVEN](FUNC.ISEVEN.md) / `ISODD`** — parity tests, which truncate toward zero rather than
  flooring. Yet another member of this category with the other convention.

The math category contains at least four distinct rounding conventions — floor, truncate, away
from zero, half away from zero — and the function names do not reliably tell you which is which.
That is the practical reason this page insists on the word *floor*.

## Numerical notes

`INT` performs no arithmetic and introduces no rounding error. Every hazard on this page comes
from the fact that **floor is a discontinuous function of a value that has already been rounded**.

**The amplification.** If `x` is the exactly-correct result of a computation and `x̂` is what the
arithmetic actually produced, then `|x − x̂|` may be a fraction of an ULP — and `|⌊x⌋ − ⌊x̂⌋|` is
either 0 or **1**. A relative error of 10^−16 becomes an absolute error of one whole unit
whenever the true value sits on an integer and the computed value lands on the other side. The
classic worksheet symptom is `INT(0.1*30)` disagreeing with `INT(3)`, or a currency conversion
producing 4 units where 5 were expected. Nothing is wrong with `INT`; the error was already
present in its argument, and `INT` merely made it visible at full size.

The remedies are all upstream of the function, and they are the standard ones for discontinuous
post-processing (Higham, *Accuracy and Stability of Numerical Algorithms*, chapter 2, on the
distinction between the error in a computed value and the error in a decision made from it):

1. **Round before flooring** when the argument is conceptually an integer — `INT(ROUND(x, 0))`,
   or better, restructure so no rounding is needed.
2. **Scale to exact integers first.** Money in cents, not in fractional units.
3. **Never floor a difference of two nearly-equal quantities.** Cancellation there is unbounded
   in ULP terms, so the floor is unbounded in units.

**The 2^52 identity region.** For |x| ≥ 2^52 every double is an integer, so `INT` returns its
argument. An implementation that routes through a fixed-width integer type — `x as i64`, then
back — is correct only below 2^63, and above that the conversion saturates or wraps, turning the
identity into a wrong answer at the top of the range. The correct primitive is IEEE 754's
`roundToIntegralTowardNegative` (C99 `floor`, Rust `f64::floor`), which is exact for every finite
double including the ones no integer type can hold, and which is a required correctly-rounded
operation in the standard. There is no accuracy argument for the integer-cast route; it is
strictly a smaller domain for no gain.

**The sign of zero.** `floor(−0.5)` is −1; `floor(−0.0)` is −0. IEEE 754 preserves the sign of a
zero argument. Whether a worksheet ever shows that is a publication-boundary question — see
[the value universe](../model/01-value-universe.md) — but an implementation that normalizes −0 to
+0 inside the kernel has made a decision, and it should be a recorded one.

## What has not been checked

No Handbook vector suite exists for `INT`; `vectors/` publishes nothing for this function, and
**no evidence record names `INT` among its subjects**. Nobody has checked this function against
Excel within the Handbook's record.

Everything above marked as documented comes from Microsoft's `INT` page: the signature, the
argument description, the round-down-away-from-zero rule for negatives, and the fractional-part
example. Everything else — the coercion behaviour, the array lifting, the arity failures, the
sign of zero — is either shared call-model behaviour or the reference engine's declaration, and
none of it has been compared against Excel here.

Inputs I would probe first:

1. **`INT(-8.9)`, `INT(-0.5)`, `INT(-1)`.** The floor-versus-truncate distinction, which is the
   only thing about this function that readers get wrong. Cheap, and it anchors every other
   probe on the page.
2. **`INT(-0)` and `INT(-0.0)` fed into `SIGN`** — whether a negative zero survives.
3. **The 2^52 boundary**: `INT(2^52 - 0.5)`, `INT(2^52)`, `INT(2^52 + 1)` and the largest finite
   double. This locates the identity region and would expose any implementation that routes
   through a fixed-width integer.
4. **`INT(TRUE)` and `INT("2.5")`** — undocumented conversions the reference engine accepts.
5. **`INT(A1)` with `A1` blank**, against the reference engine's `#VALUE!` for an empty range;
   the shared model would predict zero for a blank *cell*, and the two cases must be separated.
6. **`INT(0.1*3*10)` and `INT(1/3*3)`** — the amplification described above, as observed through
   Excel's own arithmetic rather than through a library's. This is the probe that tells a reader
   something they can act on.
7. **An array argument in the single slot**, to confirm the elementwise lift and that a
   non-convertible element stays element-local rather than collapsing the result.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| floor | The greatest integer not exceeding the argument; rounds toward −∞ |
| truncation | Dropping the fractional part; rounds toward zero. Not what `INT` does |
| fractional part | `{x} = x − ⌊x⌋`, always in [0, 1) |
| identity region | Magnitudes at or above 2^52, where every double is already an integer |
| amplification | A sub-ULP error in the argument becoming a full unit error in the result |

## Sources

- Microsoft, "INT function" —
  <https://support.microsoft.com/en-us/office/int-function-a6c4af9e-356d-4369-ab6a-cb1fd9d343ef>.
  Retrieved for this page: the syntax, the `number` argument description, the statement that
  rounding a negative number down rounds it away from zero, and the `A2-INT(A2)` fractional-part
  example.
- Microsoft, "MOD function" —
  <https://support.microsoft.com/en-us/office/mod-function-9b6cd169-b6ee-406a-a97b-edf2a9dc24f3>.
  Retrieved for the identity `MOD(n, d) = n - d*INT(n/d)`, which makes `INT` the primitive under
  Excel's remainder.
- IEEE 754-2019, `roundToIntegralTowardNegative` — the exact, correctly-rounded floor operation
  for every finite binary64.
- N. J. Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd edition, chapter 2 — error
  in a computed value versus error in a decision taken from it; the source of the amplification
  argument above.
- Handbook, [the value universe](../model/01-value-universe.md) and
  [coercion and lifting](../model/02-coercion-and-lifting.md) — value kinds, to-number outcomes,
  scalar-kernel lifting, error propagation.
- `data/functions/FUNC.INT.json` — identity, signature `INT(number)`, arity 1–1, the
  `UnaryNumericScalarOrArrayElementwise` coercion profile and the declared axes, as projected at
  OxFunc `473efa3`; `data/presence/FUNC.INT.json` — implementing module, Lean companion, and the
  `BUG-FUNC-027` defect stream.
