---
schema: efh.function-page/v1
function_id: FUNC.GCD
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-STRUCT-0007
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
family: gcd_fn
role_in_family: >-
  Sole member of its module; the meet operation of the divisibility lattice, and LCM's
  dual — the two are separate modules in the reference engine despite sharing one identity.
---

## What it computes

`GCD(number1, [number2], ...)` returns the greatest common divisor of its arguments after each
has been truncated to an integer.

The mathematics is the oldest algorithm in the book. For integers *a*, *b* not both zero,
gcd(*a*, *b*) is the largest positive integer dividing both. The definition extends to any
number of arguments by associativity:

    gcd(a, b, c) = gcd(gcd(a, b), c)

so the *n*-argument function is a left fold of the binary one, in any order — gcd is
commutative and associative, which is why Excel can accept up to 255 arguments without
declaring an evaluation order.

The structural facts worth carrying:

- **gcd(*a*, 0) = *a***, and **gcd(0, 0) = 0**. Zero is the identity element: every integer
  divides zero, so zero contributes nothing to the meet. This is not a special case bolted on;
  it is what makes gcd a well-defined operation on the whole of the non-negative integers.
- **gcd(*a*, 1) = 1.** Microsoft's page states the same fact in its own words — "One divides any
  value evenly" — and notes that "A prime number has only itself and one as even divisors."
- **gcd and lcm are dual.** Under divisibility, the non-negative integers form a lattice in
  which gcd is the meet and lcm the join. For exactly two arguments,
  gcd(*a*, *b*) · lcm(*a*, *b*) = *a* · *b*. **That identity does not extend to three or more
  arguments**, and readers who assume it does are the main source of wrong `LCM` formulas.
- **Bézout.** gcd(*a*, *b*) is the smallest positive integer expressible as *ax* + *by*. Excel
  exposes no function that returns the Bézout coefficients, so the extended algorithm is not
  reachable from the worksheet.

Domain: non-negative integers after truncation. Range: non-negative integers.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `number1` | The first value. | Yes |
| `number2`, … | Further values. | No |

Microsoft's page states the count as "1 to 255 values" and the truncation rule as: "If any
value is not an integer, it is truncated." The declared arity in the projection agrees: one to
255 argument slots.

Two properties of the truncation rule deserve emphasis.

1. **Truncation happens before the arithmetic, not after.** `GCD(2.5, 5)` is the gcd of 2 and
   5, not a rounding of some fractional gcd. Fractional arguments are silently reinterpreted;
   there is no warning.
2. **The documented rule says "truncated", not "rounded".** For non-negative inputs — the only
   ones `GCD` accepts — truncation toward zero and floor agree, so the two readings cannot be
   distinguished on the admissible domain. The distinction that bites elsewhere in the math
   category (see [INT](FUNC.INT.md)) is invisible here.

Argument slots are numeric and subject to ordinary to-number coercion; see
[coercion and lifting](../model/02-coercion-and-lifting.md). Range arguments are scanned rather
than lifted — `GCD` is declared with a custom coercion profile rather than the scalar-lift
profile used by single-argument math functions.

## Result and edge cases

Returns `Number` — a non-negative integer value.

The reference engine's published battery covers the interesting corners, and the generator
renders those rows beside this page; read them there rather than from prose. The behaviours
they exhibit, stated qualitatively:

- **Zero alone** yields zero, consistent with the identity-element reading above.
- **A logical argument** is accepted and converted; **numeric text** is accepted and then
  truncated. Neither case is covered by Microsoft's page, which speaks only of "nonnumeric".
- **An inline array argument** is consumed as a set of values and reduced to one result, not
  lifted elementwise. This is the aggregate shape, not the scalar-kernel shape.
- **An empty range** produces `#VALUE!` rather than the zero that the identity element would
  suggest.
- **A subnormal magnitude** truncates to zero and therefore contributes nothing.
- **The largest finite double** does *not* produce `#NUM!` in the reference engine, even though
  Microsoft documents `#NUM!` for any parameter at or above 2^53. See the divergence note under
  "Errors".

Arity failures at both ends — no arguments, or more than 255 — surface as `#VALUE!` in the
reference engine; in Excel the zero-argument case is expected to be refused at formula entry
instead ([the call pipeline](../model/03-call-pipeline.md)).

## Errors

As documented on Microsoft's `GCD` page:

| Error | Documented condition |
|---|---|
| `#VALUE!` | "If any argument is nonnumeric, GCD returns the #VALUE! error value." |
| `#NUM!` | "If any argument is less than zero, GCD returns the #NUM! error value." |
| `#NUM!` | "If a parameter to GCD is >=2^53, GCD returns the #NUM! error value." |

Error values in any argument propagate under the shared coercion discipline.

**Documented-versus-reference-engine divergence, recorded here as a finding.** The third row is
a hard documented ceiling: at or above 2^53, `GCD` is documented to fail. The reference
engine's own published battery row for the largest finite double returns a finite number
instead of `#NUM!`. Since 2^53 is exactly where binary64 stops representing consecutive
integers, the documented ceiling is not arbitrary — above it, "the integer this double
represents" and "the integer the user meant" part company, and refusing is a defensible design.
Which of the two behaviours Excel actually exhibits has not been observed by the Handbook, and
the answer is the single most valuable probe on this page.

## Relationships

- **[LCM](FUNC.LCM.md)** — the dual operation, and a separate module in the reference engine
  despite the shared identity gcd·lcm = product for two arguments. Their documented error
  ceilings are worded differently: `GCD`'s is on any *parameter*, `LCM`'s is on the *result*.
  That asymmetry is real and follows from the mathematics — a gcd never exceeds its inputs,
  while an lcm routinely does.
- **[MOD](FUNC.MOD.md)** — the primitive the Euclidean algorithm is built from. `GCD` is what
  repeated `MOD` converges to.
- **[QUOTIENT](FUNC.QUOTIENT.md)** and **[INT](FUNC.INT.md)** — the other truncating integer
  operations; each truncates by a different rule, and only `INT` floors.
- **[MULTINOMIAL](FUNC.MULTINOMIAL.md)** — the other variadic integer-combinatorial function in
  this category, and one that shares `GCD`'s group structural evidence record.
- Readers reach for `GCD` most often to reduce a fraction: `n/GCD(n,d)` over `d/GCD(n,d)`. That
  usage never approaches the documented 2^53 ceiling, which is why the ceiling is so rarely
  noticed.

## Numerical notes

`GCD` is one of the few functions in the math category with **no floating-point error at all**
in its intended domain — every intermediate value is an exact integer — and yet it has a
genuine numerical hazard, which is the representation boundary rather than rounding.

**The 2^53 wall.** Arguments arrive as binary64. Below 2^53 every integer is exactly
representable and truncation recovers exactly the integer the user typed. At or above 2^53 the
doubles are spaced two or more apart, so the value that arrives is not necessarily the integer
that was written, and a gcd computed from it is the gcd of a rounded neighbour. Microsoft's
documented `#NUM!` at 2^53 is precisely a refusal to answer a question that can no longer be
asked. An implementation that instead converts to a 64-bit integer inherits a *different*
boundary at 2^63, and above that the conversion saturates or wraps — which is how a function
with no rounding error still produces a wrong answer.

**Algorithm choice.** Euclid's algorithm with remainder (Knuth, *TAOCP* volume 2, section 4.5.2)
runs in O(log min(*a*, *b*)) divisions; the binary GCD of Stein replaces division with shifts
and subtraction and is usually faster on hardware without a fast integer divide. Both are exact,
so the choice is performance-only — unusually for this category, there is no accuracy trade-off
to weigh. The variadic fold benefits from an early exit: once the running gcd reaches 1 no later
argument can change it, and the remaining arguments need only be validated, not divided.

**What a careful implementation does about the boundary**, in order of decreasing preference:
guard the documented domain explicitly and return `#NUM!`; or, if a wider domain is wanted,
carry the reduction in a type wide enough to hold every finite double's integer part and state
the widened domain as a deliberate, documented departure. What it must not do is convert to a
fixed-width integer and let the conversion decide.

## What has not been checked

No Handbook vector suite exists for `GCD`; `vectors/` publishes nothing for this function.

One evidence record lists `GCD` among its subjects: **EV-STRUCT-0007**, a structural-admission
resweep against a single named live Excel build. Its own reader warning governs how it may be
read here — the record's figure is a **group** total shared across roughly twenty surfaces,
each contributing one or two probe cases, and it may not be rendered as a pass rate for `GCD`.
What it supports is narrow: the array-shape and coercion-placement case or cases `GCD`
contributed to that group matched. It establishes nothing whatever about `GCD`'s *values*.
`GCD` also appears inside the group membership of a second structural record without being one
of its subjects; that record's counts are not claimed here.

So: argument shape has been touched once, in a group, at one build. Nobody has checked this
function's results against Excel.

Inputs I would probe first:

1. **`GCD(2^53)` and `GCD(2^53 - 1)`, then `GCD(9007199254740994, 4)`.** The documented `#NUM!`
   ceiling against the reference engine's finite answer. This is the page's one live
   documentation-versus-implementation contradiction and the cheapest to settle; the pair
   straddling 2^53 exactly locates the boundary if Excel has one.
2. **`GCD(TRUE)`, `GCD(FALSE)` and `GCD("2.5")`.** The documentation says "nonnumeric" and says
   nothing about logicals or numeric text. The reference engine accepts all three. Two cells
   each.
3. **`GCD(0)` and `GCD(0, 0)`.** The identity-element reading predicts zero. It is worth
   confirming rather than assuming, because "greatest common divisor of nothing" is exactly the
   kind of corner where a shipping implementation quietly returns 1.
4. **`GCD(A1)` with `A1` blank, and `GCD(A1:A3)` with the range empty.** The reference engine
   returns `#VALUE!` for the empty range, where the scan convention elsewhere in the aggregate
   families would skip and leave the identity.
5. **`GCD(-0)` and `GCD(2.5, -0.5)`.** Whether the documented "less than zero" test is applied
   before or after truncation decides whether −0.5 is a `#NUM!` or a zero contribution.
6. **A 255-argument call and a 256-argument call**, to confirm the documented count is the
   admission boundary rather than a documentation-only figure.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| meet | The greatest-lower-bound operation of the divisibility lattice; here, gcd |
| truncation | Dropping the fractional part before the integer arithmetic begins |
| the 2^53 wall | The magnitude above which consecutive integers are no longer distinct doubles |
| group figure | A count measured across several surfaces jointly; never a per-function rate |
| variadic fold | Reducing many arguments by repeated application of the binary operation |

## Sources

- Microsoft, "GCD function" —
  <https://support.microsoft.com/en-us/office/gcd-function-d5107a51-69e3-461f-8e4c-ddfc21b5073a>.
  Retrieved for this page: the syntax, the "1 to 255 values" count, the truncation rule, the
  `#VALUE!` nonnumeric remark, the `#NUM!` negative remark, the `#NUM!` at 2^53 remark, and the
  "One divides any value evenly" / prime-divisor notes.
- Handbook evidence record `EV-STRUCT-0007` — group structural-admission resweep at one named
  Excel build; carries a reader warning against per-surface rates.
- Handbook, [the value universe](../model/01-value-universe.md) and
  [coercion and lifting](../model/02-coercion-and-lifting.md) — value kinds, to-number outcomes,
  error propagation, and the aggregate-versus-lift distinction.
- D. E. Knuth, *The Art of Computer Programming*, volume 2, section 4.5.2 — Euclid's algorithm,
  its complexity, and the binary (Stein) variant.
- `data/functions/FUNC.GCD.json` — identity (`xlfGcd`, code 473), signature, arity 1–255, and
  the declared axes, as projected at OxFunc `473efa3`.
- `data/presence/FUNC.GCD.json` — the implementing module, its Lean companion, and the
  `BUG-FUNC-028` defect stream that the structural record above resolves.
