---
schema: efh.function-page/v1
function_id: FUNC.ROUNDDOWN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft — WorksheetFunction.RoundDown method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.rounddown"
    role: "the documented \"rounds a number down, toward 0 (zero)\" definition and the three num_digits remarks"
  - work: "Microsoft Support — ROUNDDOWN function"
    locator: "https://support.microsoft.com/en-us/office/rounddown-function-2ec94c73-241f-4b01-8c6f-17e6d7968f53"
    role: "the worksheet-surface documentation page named in the projection; not retrievable for this pass"
  - work: "Gay, Correctly rounded binary-decimal and decimal-binary conversions"
    locator: "AT&T Bell Labs Numerical Analysis Manuscript 90-10, 1990"
    role: "the reference algorithm for deciding a decimal digit exactly rather than by scaling"
  - work: "IEEE 754-2019, clause 4.3.2"
    locator: "roundTowardZero"
    role: "the directed rounding attribute this function implements at a decimal place"
episodes: []
body_sections:
  - "What it computes"
  - "Arguments"
  - "Result and edge cases"
  - "Errors"
  - "Relationships"
  - "Numerical notes"
  - "What has not been checked"
  - "Page vocabulary"
  - "Sources"
family: rounddown_fn
role_in_family: "The toward-zero member of the directed-rounding trio; ROUND's floor-side sibling, and TRUNC by another name."
---

# ROUNDDOWN

## What it computes

`ROUNDDOWN(number, num_digits)` rounds toward zero to `num_digits` decimal places.

    ROUNDDOWN(x, d) = sign(x) · ⌊|x| · 10^d⌋ / 10^d

Microsoft's VBA reference defines it by difference: "**RoundDown** behaves like **Round**, except
that it always rounds a number down", where "down" is immediately qualified in the summary line as
"down, toward 0 (zero)". The distinction is essential and is the most common misreading on this
page: **`ROUNDDOWN` is not the floor.** It is truncation. For positive values the two agree; for
negative values they do not. `ROUNDDOWN(-1.7, 0)` is `-1`, while the floor of `-1.7` is `-2`.

The same three remarks as [ROUND](FUNC.ROUND.md) apply to `num_digits`: greater than zero rounds
to that many decimal places, zero rounds to an integer, less than zero rounds to the left of the
decimal point.

**Monotonicity, and the property this function has that `ROUND` does not.** Truncation toward zero
is idempotent and never increases magnitude:

    |ROUNDDOWN(x, d)| ≤ |x|,   and   ROUNDDOWN(ROUNDDOWN(x, d), d) = ROUNDDOWN(x, d)

so a `ROUNDDOWN` result is always a *conservative* estimate of magnitude. That is why it is the
correct choice for anything that must not overstate — available inventory, billable whole units,
capacity that must not be exceeded.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | Any real number to be rounded down. Required. | — |
| `num_digits` | The number of decimal places. Required. | — |

Both are numeric slots under ordinary to-number coercion — see
[Coercion and lifting](../model/02-coercion-and-lifting.md). As with `ROUND`, `num_digits` is
**required**, and the reference engine truncates a non-integral `num_digits` toward zero. Neither
fact is documented.

## Result and edge cases

Returns `Number`.

- **Sign is preserved exactly.** `ROUNDDOWN(-1.7, 0)` is `-1`, not `-2`. This is the whole
  difference from `INT` and `FLOOR.MATH`.
- **There are no ties.** Because the direction is fixed, `ROUNDDOWN` never has to break one — the
  entire tie-rule question that dominates [ROUND](FUNC.ROUND.md) simply does not arise. That makes
  this the *better-behaved* function of the two, and the one whose only remaining hazard is the
  representation issue below.
- **Large and small `num_digits`.** The reference engine returns `number` unchanged once
  `num_digits` reaches 308, and a signed zero once it reaches −308. Implementation limits, not
  documented behaviour.
- **Negative zero.** Rounding a small negative number down to a coarse place produces `−0` in the
  reference engine. It displays as `0`; it is not the same bit pattern. See
  [The value universe](../model/01-value-universe.md).
- **Arrays.** The projection declares a `Custom` coercion profile with `surface_native` lifting.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | Either argument does not convert to a number | shared coercion model |
| propagated | An error value in either argument | shared coercion model |
| `#VALUE!` | Wrong argument count (`num_digits` omitted) | arity, refused at entry |

Microsoft's VBA page states no error condition for this method.

## Relationships

- **`TRUNC`** — the same operation. `TRUNC(x, d)` and `ROUNDDOWN(x, d)` compute the same thing;
  `TRUNC`'s second argument is optional and defaults to zero, while `ROUNDDOWN`'s is required.
  Excel therefore ships two functions for one operation, differing only in argument admission.
  That is worth knowing when reading someone else's workbook: a `TRUNC` and a `ROUNDDOWN` in
  adjacent columns are not doing different things.
- **`INT`** — floors. `INT(-1.7)` is `-2`; `ROUNDDOWN(-1.7, 0)` is `-1`. The two agree on
  non-negative input and diverge on every negative non-integer, which is the classic silent bug in
  a model that changes sign.
- **`FLOOR.MATH`** — rounds toward negative infinity by default, with an explicit `mode` argument
  that switches negative numbers to toward-zero behaviour. `FLOOR.MATH` is the function to reach
  for when the direction must be *stated*; `ROUNDDOWN` is the one where it is baked in.
- **[ROUND](FUNC.ROUND.md)** and **[ROUNDUP](FUNC.ROUNDUP.md)** — the other two directions.
  `ROUNDDOWN(x,d) ≤ ROUND(x,d) ≤ ROUNDUP(x,d)` in magnitude, always, with equality when `x` is
  already a multiple of `10^-d`.
- **`ROUNDDOWN(x, 0) + ROUNDUP(x, 0)`** is not generally `2x`, and `ROUNDDOWN(-x, d)` is
  `-ROUNDDOWN(x, d)` — the function is odd, which `INT` is not. Symmetry about zero is the
  cleanest way to remember which member of the family does what.

## Numerical notes

**The tie problem disappears; the representation problem does not.** `ROUNDDOWN` still has to
decide whether the stored value is above or below a decimal boundary, and the naive
scale-and-truncate route still gets that wrong when the scaling rounds across the boundary. If
`x·10^d` rounds *up* to exactly an integer that the exact product falls short of, truncation keeps
that integer and the result is one unit too large — which for a function whose entire purpose is to
never overstate is the worst possible failure direction. The failure is rare and completely silent.

**The remedy is the same as for `ROUND` and is cheaper here.** Compute the candidate, then check
the sign of the exact residual `|x| − candidate` using a fused multiply-add; if it is negative, the
scaling overshot and the candidate must be decremented by one unit in the last decimal place. There
is no tie case to worry about, so a single correction step is provably sufficient. An implementation
that wants the mathematically correct answer should do exactly this; an implementation targeting
compatibility should not, until Excel's own behaviour on the boundary is pinned.

**Negative `num_digits` is the well-behaved half.** Powers of ten up to 10²² are exactly
representable in binary64, so for `d ≤ 0` the scaling factor `10^-d` is exact and only the division
and truncation can misbehave. For `d > 0` the factor `10^d` is exact too (same reason, up to 22),
but the *target grid* — multiples of `10^-d` — is not representable at all, so the returned value
is the nearest binary64 to the truncated decimal rather than the truncated decimal itself. The
function cannot return what its own specification names, and this is unavoidable in binary
arithmetic.

**Repeated application is safe.** Unlike the round-to-nearest case, where chained rounding at
different precisions can drift upward, truncation is monotone and idempotent, so
`ROUNDDOWN(ROUNDDOWN(x, 4), 2) = ROUNDDOWN(x, 2)` exactly. That property makes `ROUNDDOWN`
composable in a way `ROUND` is not, and it is worth exploiting in staged calculations.

## What has not been checked

No Handbook vector suite exists for `ROUNDDOWN`, and **no evidence record lists this surface among
its subjects**. `ROUNDDOWN` appears inside the group membership of an upstream array-lift
measurement (`EV-STRUCT-0011`) whose subjects are other surfaces; that record's reader warning
forbids reading its group total as any individual surface's rate. The honest statement is
therefore: **the rounding family was measured as a group for structural array admission; this
surface was not measured separately, and its arithmetic was not measured at all.**

The presence projection records the defect stream `BUG-FUNC-017` against this module.

Inputs I would probe first:

1. **`ROUNDDOWN(-1.7, 0)` beside `INT(-1.7)` and `TRUNC(-1.7)`** — three cells that settle the
   toward-zero-versus-floor question and confirm the `TRUNC` identity in the same glance.
2. **A boundary case engineered to expose scaling overshoot**, such as `ROUNDDOWN(0.29, 1)` and
   `ROUNDDOWN(4.35, 1)` and `ROUNDDOWN(1.4999999999999998, 0)` — inputs whose exact product with
   the scale sits just below an integer. If any of them returns a value one unit high, the
   implementation is scaling without correction.
3. **`ROUNDDOWN(x, d)` against `TRUNC(x, d)` swept over a few thousand random inputs and
   `d ∈ [-5, 5]`** — the cheapest way to establish, or refute, that the two are the same function.
4. **`ROUNDDOWN(-0.4, 0)`** and its reciprocal — the negative-zero question.
5. **`ROUNDDOWN(1.5, 0.9)`** — non-integral `num_digits`, truncated in the reference engine and
   undocumented.
6. **`ROUNDDOWN(x, 400)` and `ROUNDDOWN(x, -400)`** — the clamp boundaries.
7. **Array arguments in both slots**, given the array-lift defect stream on this module.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| toward zero | Truncation: the magnitude never increases and the sign never changes |
| floor | Rounding toward negative infinity, which this function does **not** do |
| idempotent | Applying the operation twice gives the same result as applying it once |
| scaling overshoot | The rounded product `x·10^d` crossing an integer the exact product does not reach |

## Sources

- Microsoft, "WorksheetFunction.RoundDown method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.rounddown> ("Rounds a
  number down, toward 0 (zero)", the behaves-like-Round sentence, and the three `num_digits`
  remarks; no error conditions are stated).
- Microsoft Support, "ROUNDDOWN function" —
  <https://support.microsoft.com/en-us/office/rounddown-function-2ec94c73-241f-4b01-8c6f-17e6d7968f53>
  (retrieval refused with HTTP 403 during this pass; nothing here is sourced from it).
- Gay, "Correctly rounded binary-decimal and decimal-binary conversions", Bell Labs NAM 90-10,
  1990; IEEE 754-2019 clause 4.3.2 (`roundTowardZero`).
- Handbook, [ROUND](FUNC.ROUND.md) — the shared decimal-rounding analysis;
  [The value universe](../model/01-value-universe.md);
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.ROUNDDOWN.json` (`Custom` coercion,
  `precision_rounding_profile=default`) and `data/presence/FUNC.ROUNDDOWN.json` (own module;
  defect stream `BUG-FUNC-017`).
