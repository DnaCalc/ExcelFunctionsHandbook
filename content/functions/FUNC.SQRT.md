---
schema: efh.function-page/v1
function_id: FUNC.SQRT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — SQRT function"
    locator: "https://support.microsoft.com/en-us/office/sqrt-function-654975c2-05c4-4831-9a24-2c65e4040fdf"
    role: "documented signature, the #NUM! condition for negative arguments, and the ABS workaround"
  - work: "IEEE 754-2019, Standard for Floating-Point Arithmetic"
    locator: "clause 5.4.1 (squareRoot)"
    role: "the requirement that square root be correctly rounded, which is what makes this function special"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 3"
    locator: "3.7 (roots and powers)"
    role: "elementary algebraic identities and the series used in cancellation-avoiding rewrites"
  - work: "Muller et al., Handbook of Floating-Point Arithmetic"
    locator: "chapter on square root and division algorithms"
    role: "Newton-Raphson and Goldschmidt square root, and the last-bit rounding argument"
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
family: sqrt_fn
role_in_family: >-
  The sole member: the non-negative square root, and the one elementary function in the category
  that IEEE 754 requires to be correctly rounded.
---

## What it computes

`SQRT(number)` is the **principal** (non-negative) square root: the unique `y >= 0` with
`y^2 = x`.

- **Domain**: `[0, +infinity)` over the reals. The equation `y^2 = x` has no real solution for
  `x < 0`, which is the mathematical reason the negative case is an error and not a convention.
- **Range**: `[0, +infinity)`.
- **Branch**: the principal branch. Every positive `x` has two real square roots, `+sqrt(x)` and
  `-sqrt(x)`; `SQRT` returns the positive one. Extended to the complex plane, the square root
  has a branch point at `0` and the conventional branch cut along the negative real axis — which
  is exactly the ray on which `SQRT` reports `#NUM!`. Excel's real-only surface makes the cut
  visible as an error condition.
- **Monotonicity and shape**: strictly increasing, concave, with `sqrt(0) = 0`, `sqrt(1) = 1`,
  and derivative `1/(2 sqrt x)` — unbounded at the origin, which is why `SQRT` is
  ill-conditioned in *absolute* terms near zero and perfectly conditioned in relative terms
  everywhere.
- **Relative conditioning**: the relative condition number is exactly `1/2` for all `x > 0`.
  Square root *halves* relative error. It is one of the very few operations that improves the
  accuracy of its input, and that fact is worth knowing when assembling a formula.
- **Identities**: `sqrt(x*y) = sqrt(x)*sqrt(y)` for non-negative arguments; `sqrt(x) = x^(1/2)`;
  `sqrt(x^2) = |x|`, not `x`.

Microsoft's page states the signature and one remark: "If number is negative, SQRT returns the
#NUM! error value", with the documented workaround `=SQRT(ABS(A2))`.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The number whose square root is wanted. Required, must be non-negative. | — |

One argument; the reference engine records an arity of exactly one. Ordinary numeric slot under
the shared coercion rules — see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

The commonly misunderstood position is the negative case, and specifically the `ABS` workaround
Microsoft documents. `SQRT(ABS(x))` is **not** the square root of `x`; it is the modulus of the
principal complex square root. If the negative sign is meaningful, the workaround silently
destroys it. Where a genuine complex root is wanted, the engineering category's `IMSQRT` is the
function that computes one.

## Result and edge cases

Returns `Number`.

- **Zero.** `SQRT(0)` is `0`. In IEEE 754 arithmetic the square root of negative zero is
  negative zero — a sign that a worksheet will not display and that may or may not survive to
  the published result. Whether Excel preserves it is unverified here and is on the probe list.
- **Exact squares.** Because IEEE 754 requires square root to be correctly rounded, `SQRT` of an
  exactly representable perfect square is exact: `SQRT(4)` is exactly `2`, and there is no
  last-bit wobble in the answer. This is not true of `POWER(x, 0.5)` in general, and that
  difference is the subject of the *Relationships* and *Numerical notes* sections below.
- **Very large arguments.** `SQRT` cannot overflow: the square root of the largest finite double
  is comfortably in range. It also cannot underflow to zero from a non-zero input, because the
  square root of the smallest subnormal is a normal number. `SQRT` is the rare function with no
  representation hazards on the positive axis at all.
- **Negative arguments.** `#NUM!`, as documented.

The reference engine classifies `SQRT` as a unary numeric scalar-or-array kernel
(`UnaryNumericScalarOrArrayElementwise`) with `kernel_signature_class: Custom`, so arrays lift
elementwise.

**A divergence worth recording.** The reference engine's projected `real_result_policy` for
`SQRT` reads `arg_domain_guard=none` with `non_finite=allow`. Microsoft documents a domain
restriction — negative arguments are `#NUM!` — so the declared axis does not, on its face,
express the documented behaviour. The most likely reading is that the guard lives inside the
`Custom` kernel rather than in the declared axis vocabulary, in which case the axis value is a
classification gap rather than a behavioural claim. The Handbook records the mismatch and does
not resolve it: the documentation says there is a domain restriction, and the projected axis
says there is no argument-domain guard.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | `number` is negative | Documented by Microsoft |
| `#VALUE!` | The argument does not convert to a number | Shared coercion rules |
| propagated | An error value in the argument is returned unchanged | Shared coercion rules |

## Relationships

- **`POWER(x, 0.5)` and `x^0.5`** — mathematically the same value, computationally not the same
  function. `SQRT` is required by IEEE 754 to be correctly rounded; a general `pow` is not, and
  in practice is not. Two expressions that look interchangeable can differ in the last bit, and
  the difference is not symmetric: `SQRT` is the accurate one. Prefer `SQRT` whenever the
  exponent is literally one half.
- **`SQRTPI`** — `sqrt(number * pi)`. Related in name, and the Handbook's evidence for `SQRTPI`
  makes precisely the `SQRT`-versus-`pow` point above; see that page.
- **`ABS`** — the documented workaround for negative input, with the caveat stated above.
- **`IMSQRT`** — the complex square root, for when the negative case is meaningful rather than
  a mistake. Note it returns text in the engineering complex-number format, not a number.
- **`EXP` / `LN`** — `SQRT` is emphatically *not* to be computed as `exp(ln(x)/2)`; that
  composition loses roughly half the significand and can overflow the intermediate.
- **Confused with**: `POWER` with a fractional exponent, and with `SUMSQ`'s inverse (there is
  no such function; the Euclidean norm is `SQRT(SUMSQ(...))`).

## Numerical notes

`SQRT` is the best-behaved elementary function in the entire category, and it is worth being
explicit about why, because the reasons are unusual.

**IEEE 754 mandates correct rounding.** Clause 5.4.1 of the standard lists `squareRoot` among
the operations that must return the correctly rounded result — the same tier as addition,
multiplication and division, and a tier that `exp`, `log`, `pow` and every trigonometric
function do *not* occupy. There is exactly one right answer for every input, every conforming
platform produces it, and there is no "quality of implementation" question. That is why `SQRT`
is not a place where two spreadsheets can honestly disagree, and why any observed disagreement
would be a finding rather than a tolerance.

**Why correct rounding is achievable here and not for `exp`.** The table-maker's dilemma — the
possibility that the true value sits so close to a rounding boundary that arbitrarily many bits
are needed to decide — does not bite for square root. The worst case is bounded: because the
square of a `p`-bit number has at most `2p` bits, the true square root of a binary64 number can
never be exactly halfway between two representable values (except in the exact-square case,
where it is representable outright), and the number of extra bits needed to round correctly is
small and known. That structural fact is what puts `sqrt` in the correctly rounded tier.

**How it is computed.** Modern hardware has a square-root instruction; where it does not, the
standard software route is a Newton-Raphson or Goldschmidt iteration on the *reciprocal* square
root — `y_{n+1} = y_n (3 - x y_n^2)/2`, which converges quadratically and uses no division —
followed by a multiplication by `x` and a final correctly rounding step. The exponent is halved
by integer arithmetic first so that the iteration runs on a significand in `[1, 4)`. Muller et
al. give the full treatment including the residual-based final rounding.

**Where `SQRT` participates in someone else's cancellation.** `SQRT` itself does not cancel,
but it appears in two classic formulas that do, and a page about square roots should name them:

1. **The quadratic formula.** `(-b + sqrt(b^2 - 4ac)) / (2a)` cancels catastrophically when
   `b^2 >> 4ac` and `b > 0`. The standard fix is to compute the root of larger magnitude with
   the sign-matched numerator and obtain the other from `x1 * x2 = c/a`.
2. **`sqrt(x^2 + 1) - x`** for large `x`, which appears inside `ASINH` and in vector-length
   work. Rewrite as `1 / (sqrt(x^2 + 1) + x)`.

In both cases the defect is in the surrounding expression, not in `SQRT` — which is exactly the
point: because `SQRT` is exact to the last bit, any error you see around it came from somewhere
else.

**Hypotenuse.** `SQRT(x^2 + y^2)` overflows when `x` alone is large, even though the result is
representable. There is no `HYPOT` in Excel; the scaled form `SQRT((x/m)^2 + (y/m)^2) * m` with
`m = MAX(ABS(x), ABS(y))` is the workaround, and it also fixes the underflow case.

## What has not been checked

No Handbook evidence record lists `FUNC.SQRT` in its subjects, and no Handbook vector suite
exists for `SQRT`. **Nobody has checked this function against Excel within the Handbook's
record.** The `#NUM!` condition for negative arguments is documented by Microsoft and has not
been observed in Excel by the Handbook.

Probes worth running first:

1. **`SQRT` against `POWER(x, 0.5)` and `x^0.5`** over a broad sample, especially near the top
   and bottom of the exponent range. If they ever differ, `SQRT` is using the correctly rounded
   route and the power operator is not — which is the expected outcome, and confirming it is the
   single most informative probe on this page. The `SQRTPI` evidence records make exactly this
   distinction for a neighbouring surface, which raises the prior that it holds here too.
2. **Perfect squares** across the exponent range — `4`, `1e100` when exactly representable,
   `2^2k` — where the correct answer is exact and any deviation is a defect rather than a
   rounding choice.
3. **Negative zero.** `SQRT(-0)` and whether a signed zero survives to the published result, and
   whether `-0` takes the documented `#NUM!` branch (it should not; `-0` is not negative under
   IEEE comparison).
4. **The boundary of the negative branch**: the largest negative subnormal, and text and logical
   arguments that coerce to a negative number.
5. **Subnormal inputs**, whose square roots are normal — a place where a poorly written
   scaling step fails.
6. **Array arguments**, to confirm elementwise lifting and element-local `#NUM!`.
7. **The reference engine's `arg_domain_guard=none` classification** against actual behaviour on
   a negative argument, which is the divergence recorded above.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| principal square root | The non-negative root; the branch `SQRT` returns |
| correctly rounded | The result is the true value rounded once to the nearest double; required by IEEE 754 for square root |
| table-maker's dilemma | The problem of not knowing in advance how many bits are needed to round a transcendental correctly |
| relative condition number | Amplification of relative input error into relative output error; exactly `1/2` here |

## Sources

- Microsoft, "SQRT function" —
  <https://support.microsoft.com/en-us/office/sqrt-function-654975c2-05c4-4831-9a24-2c65e4040fdf>
  (signature; "If number is negative, SQRT returns the #NUM! error value"; the `ABS` workaround).
- IEEE 754-2019, clause 5.4.1 — `squareRoot` among the correctly rounded operations.
- Muller et al., *Handbook of Floating-Point Arithmetic* — Newton-Raphson and Goldschmidt square
  root, the bounded worst case, and the final rounding step.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 3 — elementary algebraic
  identities used in the cancellation-avoiding rewrites.
- Handbook page [SQRTPI](FUNC.SQRTPI.md) — where the `sqrt`-versus-`pow` distinction is on the
  record for a neighbouring surface.
- Handbook projections `data/functions/FUNC.SQRT.json` (arity, `kernel_signature_class: Custom`,
  `real_result_policy` with `arg_domain_guard=none`) and `data/presence/FUNC.SQRT.json`.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md).
