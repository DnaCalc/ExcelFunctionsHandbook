---
schema: efh.function-page/v1
function_id: FUNC.TRUNC
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — TRUNC function"
    locator: "https://support.microsoft.com/en-us/office/trunc-function-8b86a64c-3127-43db-ba14-aa5ceb292721"
    role: "documented signature, the num_digits default of 0, and the stated contrast with INT"
  - work: "IEEE 754-2019, Standard for Floating-Point Arithmetic"
    locator: "roundToIntegralTowardZero; the five rounding-direction attributes"
    role: "the standard name for this rounding mode and its distinction from the others"
  - work: "Goldberg, What Every Computer Scientist Should Know About Floating-Point Arithmetic"
    locator: "sections on rounding and on decimal conversion"
    role: "why scaled decimal truncation is not exact and where the representable-decimal problem comes from"
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
family: trunc_fn
role_in_family: >-
  The sole member: rounding toward zero, with an optional decimal-place argument that turns a
  clean integral operation into a scaled one.
---

## What it computes

`TRUNC(number, [num_digits])` discards digits beyond a given decimal position, rounding **toward
zero**.

With `num_digits = 0` — the documented default — the mathematical statement is

    TRUNC(x) = sgn(x) * floor(|x|)

which is IEEE 754's `roundToIntegralTowardZero`. With a general `d`, the intended value is

    TRUNC(x, d) = sgn(x) * floor(|x| * 10^d) / 10^d

so positive `d` keeps `d` decimal places and negative `d` zeroes out digits to the left of the
decimal point.

- **Domain**: all real numbers, for both arguments.
- **Range**: for `d = 0`, the integers; in general, the multiples of `10^-d` — except that most
  of those multiples are not representable in binary64, which is the whole numerical content of
  this page.
- **Shape**: a monotone non-decreasing step function, constant on each half-open interval
  between consecutive multiples of `10^-d`, with the intervals oriented **away from zero on both
  sides** — `[0, s)` maps to `0` and `(-s, 0]` maps to `0`, so the step at the origin is twice
  as wide as every other step. That double-width step at zero is the geometric statement of
  "toward zero" and it is what distinguishes this function from `INT` and from `FLOOR`.
- **Identities**: `TRUNC(x) = INT(x)` for `x >= 0`; `TRUNC(-x) = -TRUNC(x)` (odd, unlike `INT`);
  `x = TRUNC(x) + frac(x)` where `frac` has the same sign as `x`.

Microsoft's page states the contrast directly: "TRUNC simply removes the fractional part of the
number, whereas INT returns the nearest lower integer (not necessarily smaller in magnitude)",
with the worked case that `TRUNC(-4.3)` is `-4` while `INT(-4.3)` is `-5`.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The number to truncate. Required. | — |
| `num_digits` | The precision of the truncation. Optional. | `0` |

The reference engine records an arity of one to two, consistent with the documented optional
second argument.

The commonly misunderstood position is `num_digits`, in two ways:

1. **Negative values are meaningful.** `TRUNC(1234, -2)` truncates to hundreds. Readers who
   assume the argument is a count of decimals in the ordinary sense often do not realise the
   left-hand side is reachable.
2. **It is not a rounding argument.** `ROUND` and `TRUNC` take the same-shaped second argument
   and do different things with it; a formula ported from one to the other changes behaviour
   silently on exactly half the inputs.

Whether `num_digits` is itself truncated when non-integral is not stated on Microsoft's page,
and the Handbook does not assert an answer.

## Result and edge cases

Returns `Number`.

- **Negative numbers** truncate toward zero — the documented `TRUNC(-4.3) = -4`. This is the
  single most important edge and the reason the function exists alongside `INT`.
- **Values already integral** are returned unchanged.
- **Zero** returns zero. Whether a negative input in `(-1, 0)` returns `+0` or `-0` is
  unspecified by the documentation and is on the probe list; the mathematical answer is `0` and
  IEEE's `roundToIntegralTowardZero` preserves the sign of the operand, which would give `-0`.
- **Very large numbers.** Above `2^52` every binary64 value is already an integer, so `TRUNC`
  with `d = 0` is the identity there. For positive `d` the scaling `10^d` can overflow for large
  `d`; for negative `d` and large `|x|` the same is true in the other direction.
- **The general `d` case is not exact.** `10^-d` is not representable for any `d` except zero
  and small negative values, so `TRUNC(x, d)` for `d > 0` cannot in general return a number
  whose decimal expansion terminates where the caller asked. The returned double is the nearest
  representable neighbour of the intended decimal value. This is not an implementation defect;
  it is a property of binary floating point. It does mean that `TRUNC(x, 2)` followed by a
  comparison against a literal with two decimals is not a reliable equality test.

The reference engine classifies `TRUNC` with `kernel_signature_class: NumsToNum`,
`coercion_lift_profile: Custom` and `error_collapse_profile: None`, and its projected
`real_result_policy` reads `arg_domain_guard=none` with `non_finite=allow` — no domain
restriction, consistent with a function that is total on the reals.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | Either argument does not convert to a number | Shared coercion rules |
| propagated | An error value in either argument is returned unchanged | Shared coercion rules |

Microsoft's page lists no error conditions for `TRUNC`, and the function has no domain
restriction. What happens when the internal scaling by `10^d` overflows is not documented.

## Relationships

- **`INT`** — the documented contrast. `INT` rounds toward negative infinity (the floor);
  `TRUNC` rounds toward zero. They agree on non-negative numbers and differ on every negative
  non-integer. `INT` also has no second argument.
- **`ROUND`, `ROUNDDOWN`, `ROUNDUP`** — the decimal-place family. **`ROUNDDOWN` is the
  near-duplicate**: rounding "down" in Excel's naming means toward zero, not toward negative
  infinity, so `ROUNDDOWN(x, d)` and `TRUNC(x, d)` are the same operation under two names. The
  documented difference is that `TRUNC`'s second argument is optional and `ROUNDDOWN`'s is not.
  A reader choosing between them is choosing a name, not a behaviour.
- **`FLOOR.MATH` / `CEILING.MATH`** — the significance-based family, which rounds to a multiple
  of an arbitrary step rather than a power of ten, and which carries an explicit mode argument
  for the negative direction. Where `TRUNC`'s negative-number convention is fixed,
  `FLOOR.MATH`'s is a parameter.
- **`MOD` and `QUOTIENT`** — `QUOTIENT` is integer division and is `TRUNC(a/b)` in intent;
  `MOD`'s sign convention follows the floor, not the truncation, which is why `MOD` and
  `QUOTIENT` do not compose the way a C programmer expects.
- **`SIGN`** — the parity relation `TRUNC(-x) = -TRUNC(x)` is exactly the statement that `TRUNC`
  commutes with `SIGN`.
- **Confused with**: `INT` (the documented case), `ROUND` (which rounds to nearest, not toward
  zero), and cell *formatting*, which changes display without changing the stored value.

## Numerical notes

The `d = 0` case is exact and trivial: rounding toward zero is one of IEEE 754's five rounding
directions, hardware implements it directly, and there is no error. Everything below concerns
the general case.

**The scaling problem.** The obvious implementation of `TRUNC(x, d)` is
`trunc(x * 10^d) / 10^d`. It has three separate defects:

1. **`10^d` is inexact for `d > 22`.** Powers of ten are exactly representable in binary64 only
   up to `10^22`; beyond that the constant itself is a rounded value, so the scaling introduces
   error before any truncation happens.
2. **The multiplication rounds.** Even with an exact `10^d`, `x * 10^d` rounds, and the rounding
   can cross an integer boundary. A value that is a hair below `k` mathematically can round *up*
   to exactly `k`, whereupon `trunc` keeps `k` and the caller sees a digit they asked to have
   discarded. The failure is one-sided and rare, which is what makes it a durable bug.
3. **The division rounds again.** The final quotient is the nearest double to the intended
   decimal, not the decimal itself.

**What careful implementations do.** The defensible approaches, in increasing order of effort:
scale with an exact power-of-ten table for the representable range and fall back to a
higher-precision path beyond it; or do the work in a decimal representation (`decimal128`, or a
big-integer significand with a decimal exponent) so that "keep `d` decimal digits" is exact by
construction; or use a correctly rounded decimal-conversion routine (the Steele-White / Ryu
line) to produce the digit string and rebuild from it. Spreadsheet implementations historically
take the first route. The Handbook makes no claim about which route Excel takes.

**Why this matters more here than for `ROUND`.** Round-to-nearest is *self-correcting* about
boundary crossings: if the scaled product rounds slightly, the nearest integer is usually still
the right one, because the decision boundary is at the half-way point, far from where the
product lands. Truncation puts the decision boundary exactly at the integer, which is precisely
where the scaling error can push a value across. **Truncation is the more fragile of the two
operations under scaling**, and that is not obvious from the definitions.

**Repeated truncation is not idempotent across precisions.** `TRUNC(TRUNC(x, 4), 2)` need not
equal `TRUNC(x, 2)` once the scaled values are inexact. Chained rounding in financial worksheets
is where this surfaces.

## What has not been checked

No Handbook evidence record lists `FUNC.TRUNC` in its **subjects**. `FUNC.TRUNC` appears in the
group membership of `EV-STRUCT-0011`, an array-lift tranche whose counting is at group scope —
so **the family was measured and this surface was not measured separately**. The Handbook does
not attribute that record's figures to `TRUNC`, and this page claims nothing from it.

No Handbook vector suite exists for `TRUNC`. Beyond the group-scope record just named, nobody
has checked this function against Excel within the Handbook's record.

Probes worth running first:

1. **The scaling-boundary cases.** Values engineered to sit a fraction of an ULP below a
   `d`-decimal boundary — for example a value just under `1.005` truncated to two places, and
   the analogous constructions at several `d`. This is the probe that distinguishes an exact
   decimal implementation from a scaled binary one, and it is the highest-value probe here.
2. **`d > 22`**, where the power of ten stops being representable. A pure table-based
   implementation changes character at that point.
3. **Negative `d`** across magnitudes, including `d` large enough that the answer is zero, and
   `d` negative enough that the scaling underflows.
4. **`TRUNC` against `ROUNDDOWN`** at the same inputs across a broad sample. If they ever
   differ, they are not the same code path, which would be a finding in itself given how
   interchangeable the documentation makes them look.
5. **Negative-zero production**: `TRUNC(-0.5)` — whether the published result carries a sign.
6. **Non-integer `num_digits`**, e.g. `TRUNC(1.23456, 2.7)`, which the documentation does not
   cover.
7. **Very large `number` with positive `d`**, where the internal scaling would overflow.
8. **Array arguments in both positions**, given the group-scope array-lift record named above.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| toward zero | The rounding direction that discards magnitude; IEEE's `roundToIntegralTowardZero` |
| `num_digits` | The decimal position at which truncation happens; negative values act left of the point |
| scaling | Multiplying by `10^d` before the integral operation and dividing after |
| double-width step | The interval `(-s, s)` that all maps to zero, unique to toward-zero rounding |

## Sources

- Microsoft, "TRUNC function" —
  <https://support.microsoft.com/en-us/office/trunc-function-8b86a64c-3127-43db-ba14-aa5ceb292721>
  (signature, `num_digits` default of zero, and the stated contrast with `INT` including the
  `-4.3` example; no error conditions are listed).
- IEEE 754-2019 — `roundToIntegralTowardZero` and the rounding-direction attributes.
- Goldberg, *What Every Computer Scientist Should Know About Floating-Point Arithmetic* — why
  scaled decimal operations are inexact.
- Handbook evidence record `EV-STRUCT-0011` — cited only to record that `FUNC.TRUNC` appears
  there in group membership, which forbids per-surface attribution.
- Handbook projections `data/functions/FUNC.TRUNC.json` (arity 1-2,
  `kernel_signature_class: NumsToNum`, `coercion_lift_profile: Custom`, `real_result_policy`
  with `arg_domain_guard=none`) and `data/presence/FUNC.TRUNC.json`.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md).
