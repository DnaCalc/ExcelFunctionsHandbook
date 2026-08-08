---
schema: efh.function-page/v1
function_id: FUNC.COTH
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0010
open_problems: []
references:
  - work: "Microsoft Support — COTH function"
    locator: "https://support.microsoft.com/en-us/office/coth-function-2e0b4cb6-0ba0-403e-aed4-deaa71b49df5"
    role: "documented signature, the magnitude constraint, the #NUM! and #VALUE! conditions, and the cosh/sinh equation"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 4.5 (Hyperbolic Functions), 4.5.65-4.5.67 for the coth expansions"
    role: "definition, the Laurent expansion at the origin, and the identity set"
  - work: "fdlibm (Sun Microsystems freely distributable libm)"
    locator: "s_tanh.c"
    role: "the expm1-based tanh whose reciprocal is the stable route to coth"
  - work: "Cephes mathematical library"
    locator: "tanh.c"
    role: "an independent branch structure for the same problem"
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
family: coth
role_in_family: >-
  The hyperbolic cotangent — the surface whose recorded large-argument behaviour is saturation
  to the sign of the argument rather than an error, and the one place in this batch where the
  reference engine's domain-guard axis openly disagrees with Microsoft's documented constraint.
---

# COTH

## What it computes

`COTH(number)` returns the hyperbolic cotangent. Microsoft's page gives the equation as
`cosh(x)/sinh(x)` and describes the function as the hyperbolic analogue of the ordinary
cotangent.

    coth x = cosh x / sinh x = 1 / tanh x = (e^{2x} + 1) / (e^{2x} - 1)

| Property | Statement |
|---|---|
| Domain | all real `x` except `x = 0` |
| Range | `(-∞, -1) ∪ (1, ∞)` — the value 1 is a limit, never attained |
| Parity | odd: `coth(-x) = -coth(x)` |
| Pole | one, simple, at `x = 0`, with residue 1 |
| Laurent series at 0 | `coth x = 1/x + x/3 - x³/45 + 2x⁵/945 - …` (A&S 4.5.67) |
| Asymptote | `coth x → ±1` as `x → ±∞`, exponentially fast: `coth x = 1 + 2e^{-2x} + …` |
| Fundamental identity | `coth²x - csch²x = 1` |
| Derivative | `d/dx coth x = -csch²x = 1 - coth²x` |
| Relation to circular | `coth x = i·cot(ix)` |
| Complex structure | poles at every `x = ikπ`; periodic with period `πi` |

The shape is worth holding in mind: two branches, each strictly decreasing, each hugging the
line `y = ±1` from outside. Because the approach is exponential, `coth x` becomes
indistinguishable from `1` in binary64 once `x` is above about 19 — `2e^{-2x}` falls below half
an ulp of 1 there. **From roughly `x = 19` upward the mathematically correct answer already
*is* exactly 1.0.** That fact does most of the work in the evidence discussion below.

## Arguments

| Argument | Meaning | Constraint (as documented) |
|---|---|---|
| `number` | The hyperbolic angle. Required. | `ABS(number) < 2^27` |

**A recorded divergence.** Microsoft's `COTH` page documents the `2^27` magnitude constraint and
`#NUM!` when it is exceeded — the same constraint template as [COT](FUNC.COT.md),
[CSC](FUNC.CSC.md) and `SEC`. The reference engine's real-result policy for this surface
records `arg_domain_guard=none`. Those two statements cannot both describe the same behaviour
at `ABS(x) ≥ 2^27`: the documentation says `#NUM!`, the recorded axis says no argument guard
applies. The Handbook publishes the divergence and has not resolved it.

The constraint is also, on its face, unnecessary here in a way it is not for the circular
functions: `coth` needs no argument reduction, and its correct value at `2^27` is exactly 1.0.
A magnitude bound on a hyperbolic function looks like a documentation template applied
uniformly across the 2013 trigonometric batch rather than a numerical requirement — but that
reading is a hypothesis, not a finding, and it is on the probe list.

## Result and edge cases

Returns `Number` with absolute value strictly greater than 1.

- **`COTH(0)`** — mathematically a pole. Microsoft's `COTH` page, as fetched, states the
  magnitude constraint and the non-numeric condition but **does not state what happens at
  zero**, where [COT](FUNC.COT.md)'s page does state `#DIV/0!`. That asymmetry is in the
  documentation; the Handbook has not checked what Excel returns.
- **Large positive `x`** — the correct answer is 1.0 exactly, for every `x` above about 19.
- **Large negative `x`** — the correct answer is −1.0 exactly, symmetrically.
- **Saturation** — `EV-MATH-0010` records a single verified live witness in which `COTH`
  returns the sign of its argument at a large magnitude, and identifies the mechanism: computed
  as `cosh/sinh`, both terms overflow to infinity and the ratio becomes NaN, so the
  implementation saturates to `sign(n)` instead. The reference engine carries
  `non_finite=saturate_sign` for this surface, matching. The record publishes **no count** and
  says so; it lives in a code comment rather than in a catalogue row.
- **A subtlety the saturation hides.** Because the true value is already exactly `±1` well
  before any overflow, saturation and correct evaluation agree on the *value* everywhere the
  overflow could occur. The saturation is therefore not an approximation — it is the right
  answer reached by an unusual route. What it does hide is whether the documented `2^27` bound
  fires at all.
- **Near zero** — `coth x ≈ 1/x`, so `COTH` of a tiny argument is a huge finite number, and
  overflows only for `ABS(x)` down in the subnormal range.

## Errors

As documented on Microsoft's `COTH` page:

| Error | Condition |
|---|---|
| `#NUM!` | `ABS(number)` is not less than `2^27` |
| `#VALUE!` | `number` is non-numeric |

Not documented on that page, and therefore not stated here as behaviour: what `COTH(0)`
returns. `1/TANH(0)` and `COSH(0)/SINH(0)` are both divisions by zero, so `#DIV/0!` is the
expected outcome by analogy with `COT`, but expectation is not evidence and Microsoft's page
does not say it.

## Relationships

- **`TANH`** — the reciprocal, and the numerically well-behaved member of the pair. Any
  accurate `COTH` is `1/TANH` computed carefully, or the Laurent series near zero.
- **`SINH`, [COSH](FUNC.COSH.md)** — the ratio the documentation names. Evaluating `COTH` that
  way is what produces the `Inf/Inf` the saturation exists to catch.
- **[CSCH](FUNC.CSCH.md)** — the other hyperbolic reciprocal in the same 2013 batch, sharing the
  pole at zero, the documented constraint, and the `arg_domain_guard=none` axis value. Read the
  two pages together: they diverge from their documentation in the same way.
- **`ACOTH`** — not an Excel function. There is no inverse hyperbolic cotangent on the
  worksheet surface; `ATANH(1/x)` is the standard substitute and inherits `ATANH`'s own
  accuracy problem near `±1`.
- **[COT](FUNC.COT.md)** — the circular partner, `coth x = i·cot(ix)`. Same `1/x` leading term,
  same documented bound, and the only member of the pair whose zero case is documented.
- **`IMCOT`** — the complex cotangent; there is no `IMCOTH`.

## Numerical notes

Three regimes, and the middle one is the only place a naive route is safe.

**Near zero.** `1/tanh x` is well conditioned as a *value* — `tanh x → x`, so `coth x → 1/x` —
but the interesting quantity is often `coth x - 1/x`, which cancels catastrophically. The
remedy is the Laurent series `1/x + x/3 - x³/45 + …` (A&S 4.5.67), used below roughly `2^-13`,
where it is both cheaper and more accurate than any division.

**Moderate.** `coth x = 1/tanh x`, with `tanh` computed by the standard `expm1` form: for
`t = expm1(2·ABS(x))`, `tanh(ABS(x)) = t/(t + 2)`, hence `coth(ABS(x)) = (t + 2)/t`, sign
restored afterwards. This is fdlibm's `s_tanh.c` inverted, and it never forms `e^{2x}`
directly, so it never overflows on the way to a value near 1. Alternatively
`coth x = 1 + 2/(e^{2x} - 1)` isolates the deviation from the asymptote — the form to use if
what you want is `coth x - 1`.

**Large.** The mathematically correct answer is exactly `sign(x)` once `2e^{-2ABS(x)}` drops
below half an ulp of 1, which happens around `ABS(x) ≈ 19`. Any branch structure that returns
`sign(x)` above that threshold is correct, whatever route it took to get there. The
`cosh/sinh` route, by contrast, produces `Inf/Inf = NaN` at around `ABS(x) ≈ 710` — three
orders of magnitude *past* the point where the answer stopped changing. The recorded
saturation is a repair applied at the overflow boundary rather than a branch taken at the
mathematical one, and both give the same answer; the difference is only visible if you ask what
happens in between, which nothing on record does.

The general lesson, and it is a good one: a reciprocal built from `cosh/sinh` fails not because
the values are wrong but because the *representation* fails first. Reformulating in terms of
`expm1` moves the overflow out of the computation entirely.

## What has not been checked

`EV-MATH-0010` names this surface. It records a single verified live witness of the saturation
behaviour and states in its own reader warning that one witness is not a count and that **no
numeric-bits comparison count exists for `COTH` anywhere** in the upstream record. The finding
lives in a source-code comment rather than in a catalogue row.

No Handbook vector suite exists for `COTH`. The magnitude constraint, the `#NUM!` and `#VALUE!`
conditions and the `cosh/sinh` equation are Microsoft's; the saturation is on record from
OxFunc with one witness; the zero case is documented nowhere; and the conflict between the
documented constraint and the recorded `arg_domain_guard=none` axis is unresolved.

Inputs I would probe first:

1. **`COTH(2^27)` and `COTH(2^27 - 1)`.** This is the single most valuable probe on the page,
   because the two competing statements give different *kinds*: the documentation says `#NUM!`,
   the recorded axis and the saturation witness together say `1`. One experiment decides it.
2. **`COTH(0)`, `COTH(-0)`.** The undocumented pole. `#DIV/0!` is expected; nothing establishes
   it.
3. **The saturation onset**: `COTH(18)`, `COTH(19)`, `COTH(20)`, `COTH(25)`, `COTH(800)`. The
   first argument at which the returned value is exactly 1.0 separates "correct value, reached
   normally" from "saturation applied late", and it locates the branch boundary without an
   oracle — the mathematics fixes what the right answer is.
4. **`COTH(x) · TANH(x) - 1`** across the range, which tests the reciprocal relation, and
   `COTH(x) - COSH(x)/SINH(x)` computed in the worksheet, which tests the documented equation
   against the shipped one. Where the second diverges and the first does not, the implementation
   is not doing what the documentation says.
5. **Near zero**: `COTH(2^-20)` down to `COTH(2^-1070)`, testing the `1/x` regime and the
   subnormal boundary — and `COTH(x) - 1/x` against the series `x/3 - x³/45`, which is the
   cancellation probe.
6. **The oddness identity** `COTH(-x) = -COTH(x)` bitwise, which any sign-restoring
   implementation satisfies exactly and a series-based one might not.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| saturation | Returning the asymptotic value `±1` in place of an unrepresentable computation |
| the `Inf/Inf` failure | `cosh/sinh` at large argument: both terms overflow, the ratio is NaN |
| `expm1` form | Computing via `e^t - 1` so the overflow never enters the expression |
| asymptote-exact region | Arguments above which the true value is already exactly `±1` |
| Laurent series | The expansion `1/x + x/3 - …` valid near the pole at the origin |
| domain-guard axis | The recorded `arg_domain_guard` value on the surface's real-result policy |

## Sources

- Microsoft, "COTH function" —
  <https://support.microsoft.com/en-us/office/coth-function-2e0b4cb6-0ba0-403e-aed4-deaa71b49df5>
  (fetched at curation: signature, the `2^27` magnitude constraint, the `#NUM!` and `#VALUE!`
  conditions, and the `cosh(x)/sinh(x)` equation. No zero-argument behaviour is documented).
- Handbook evidence record `EV-MATH-0010` — the single live saturation witness, the `Inf/Inf`
  mechanism, the `SATURATE_SIGN` substrate, and the explicit statement that no count exists.
  Read its reader warning.
- Abramowitz & Stegun, chapter 4.5 — hyperbolic identities and the `coth` expansions.
- fdlibm `s_tanh.c` and Cephes `tanh.c` — the `expm1`-based route this page recommends.
- Handbook, [COT](FUNC.COT.md) and [CSCH](FUNC.CSCH.md) — the documentation-template siblings;
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.COTH.json` (the `arg_domain_guard=none` and
  `non_finite=saturate_sign` axis values) and `data/presence/FUNC.COTH.json`.
