---
schema: efh.function-page/v1
function_id: FUNC.ROUNDUP
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft — WorksheetFunction.RoundUp method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.roundup"
    role: "the documented \"rounds a number up, away from 0 (zero)\" definition and the three num_digits remarks"
  - work: "Microsoft Support — ROUNDUP function"
    locator: "https://support.microsoft.com/en-us/office/roundup-function-f8bc9b23-e795-47db-8703-db171d0c42a7"
    role: "the worksheet-surface documentation page named in the projection; not retrievable for this pass"
  - work: "Gay, Correctly rounded binary-decimal and decimal-binary conversions"
    locator: "AT&T Bell Labs Numerical Analysis Manuscript 90-10, 1990"
    role: "the reference algorithm for deciding a decimal digit exactly rather than by scaling"
  - work: "IEEE 754-2019, clause 4.3.2"
    locator: "roundTowardPositive / roundTowardNegative"
    role: "the directed attributes this function combines into an away-from-zero rule"
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
family: roundup_fn
role_in_family: "The away-from-zero member of the directed-rounding trio; the conservative-overstatement counterpart of ROUNDDOWN."
---

# ROUNDUP

## What it computes

`ROUNDUP(number, num_digits)` rounds away from zero to `num_digits` decimal places.

    ROUNDUP(x, d) = sign(x) · ⌈|x| · 10^d⌉ / 10^d

Microsoft's VBA reference summarises it as "Rounds a number up, away from 0 (zero)" and then
defines it by difference: "**RoundUp** behaves like **Round**, except that it always rounds a
number up". The parenthetical is doing real work — **`ROUNDUP` is not the ceiling.** For positive
values the two agree; for negative values they do not. `ROUNDUP(-1.2, 0)` is `-2`, while the
ceiling of `-1.2` is `-1`.

The same three `num_digits` remarks apply as on [ROUND](FUNC.ROUND.md): greater than zero rounds to
that many decimal places, zero rounds to an integer, less than zero rounds to the left of the
decimal point.

**The properties that make it useful.** Rounding away from zero never decreases magnitude and is
idempotent:

    |ROUNDUP(x, d)| ≥ |x|,   and   ROUNDUP(ROUNDUP(x, d), d) = ROUNDUP(x, d)

so a `ROUNDUP` result is a *safe overestimate* of magnitude. It is the correct choice for anything
that must not fall short — boxes needed, staff required, buffer to allocate. It is also, for the
same reason, the wrong choice for anything summed over many rows, because the bias accumulates: a
column of `ROUNDUP(x, 2)` overstates its total by up to one cent per row, systematically and in one
direction.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | Any real number to be rounded up. Required. | — |
| `num_digits` | The number of decimal places. Required. | — |

Both are numeric slots under ordinary to-number coercion — see
[Coercion and lifting](../model/02-coercion-and-lifting.md). As with the rest of the family,
`num_digits` is **required**, and the reference engine truncates a non-integral `num_digits` toward
zero. Neither fact is documented.

## Result and edge cases

Returns `Number`.

- **Sign is preserved and magnitude grows.** `ROUNDUP(-1.2, 0)` is `-2`. This is the whole
  difference from `CEILING.MATH`'s default direction.
- **Zero is a fixed point.** `ROUNDUP(0, d)` is `0` for every `d` — the reference engine special-
  cases it, and it has to, because the sign extraction has no meaningful answer at zero.
- **There are no ties.** As with [ROUNDDOWN](FUNC.ROUNDDOWN.md), the direction is fixed, so the
  tie-rule question that dominates [ROUND](FUNC.ROUND.md) does not arise.
- **Any nonzero fraction promotes.** `ROUNDUP(1.0000000000000002, 0)` is `2`. The function is
  extremely sensitive to the last bit: a value that is one ulp above an integer rounds up a whole
  unit. Since arithmetic routinely produces values one ulp off an intended integer, `ROUNDUP` on a
  computed quantity is a well-known source of off-by-one results — the value that "should be" 3.0
  is stored as 3.0000000000000004 and comes back as 4.
- **Large and small `num_digits`.** The reference engine returns `number` unchanged once
  `num_digits` reaches 308, and a signed zero once it reaches −308. Implementation limits, not
  documented behaviour.
- **Arrays.** The projection declares a `Custom` coercion profile with `surface_native` lifting.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | Either argument does not convert to a number | shared coercion model |
| propagated | An error value in either argument | shared coercion model |
| `#VALUE!` | Wrong argument count (`num_digits` omitted) | arity, refused at entry |

Microsoft's VBA page states no error condition for this method. In particular it does not say what
happens when rounding up at a coarse place would exceed the representable range — `ROUNDUP` of a
value near the top of the double range at negative `num_digits` has to either saturate or fail, and
nothing documents which.

## Relationships

- **`CEILING.MATH`** — rounds toward positive infinity by default, with a `mode` argument that
  switches negative numbers to away-from-zero behaviour. `CEILING.MATH(x, 10^-d, 1)` is the
  equivalent of `ROUNDUP(x, d)`; the default `mode` is *not*. Reach for `CEILING.MATH` when the
  direction must be explicit and for a significance that is not a power of ten.
- **`ISO.CEILING` and `CEILING.PRECISE`** — the always-toward-positive-infinity variants, which
  differ from `ROUNDUP` on exactly the negative half-line.
- **[ROUND](FUNC.ROUND.md)** and **[ROUNDDOWN](FUNC.ROUNDDOWN.md)** — the other two directions.
  `ROUNDDOWN(x,d) ≤ ROUND(x,d) ≤ ROUNDUP(x,d)` in magnitude, always.
- **`ROUNDUP(x/n, 0)`** — the idiomatic "how many containers" calculation, and the one that meets
  the last-bit sensitivity above head-on: when `x` is an exact multiple of `n`, the quotient may
  still land one ulp high and produce one container too many. Where the operands are integers,
  `QUOTIENT` plus a remainder test is the safer construction; see [QUOTIENT](FUNC.QUOTIENT.md).
- **`ROUNDUP` is odd:** `ROUNDUP(-x, d) = -ROUNDUP(x, d)`. `CEILING.MATH` in its default mode is
  not.

## Numerical notes

**No ties, but the sharpest boundary sensitivity in the family.** `ROUNDUP` promotes on *any*
excess, however small, which makes it the function most exposed to the accumulated error of
whatever computed its argument. `ROUND` needs an error of half a decimal unit to change its
answer; `ROUNDUP` needs one ulp. That is a difference of roughly fifteen orders of magnitude in
required perturbation, and it is why `ROUNDUP` results are so often described as "wrong" when the
arithmetic upstream is what moved.

**Scaling can overshoot in the harmless direction and undershoot in the harmful one.** With the
naive `⌈|x|·10^d⌉/10^d`, if the product rounds *up* across an integer that the exact product falls
short of, the ceiling is already that integer and the answer is correct. But if the product rounds
*down* onto an integer that the exact product slightly exceeds, the ceiling stops there and the
result is one unit too *small* — violating the function's defining guarantee that the result never
falls short. For a function whose entire contract is "never understate", that is the failure mode
to guard.

**The guard is one exact residual test.** Form the candidate, compute `|x| − candidate` exactly
with a fused multiply-add, and if it is positive, increment by one unit in the last decimal place.
There is no tie case, so one correction step is provably sufficient. As on the sibling pages: a
`natural-best` implementation should do this, and an `excel-bitexact` implementation should not
until Excel's own boundary behaviour is pinned — see
[About implementation options](../model/07-implementation-options.md).

**The target grid is still not representable.** For `d > 0` the multiples of `10^-d` do not exist
in binary64, so the returned value is the nearest double to the rounded-up decimal, and that
nearest double can be *below* the decimal it represents. Consequently `ROUNDUP(x, 2) ≥ x` holds as
a comparison of stored doubles in practice but is not guaranteed by the specification; the
guarantee lives in decimal, and the value lives in binary.

**Accumulated bias is a modelling concern, not an implementation one.** A column of `n` values each
rounded up at `d` places overstates its total by an expected `n·10^-d/2`. If the total matters,
round the total rather than the parts — the classic penny-allocation problem, and the reason
`ROUNDUP` should never be applied row-wise before a `SUM` that is meant to reconcile.

## What has not been checked

No Handbook vector suite exists for `ROUNDUP`, and **no evidence record lists this surface among
its subjects**. `ROUNDUP` appears inside the group membership of an upstream array-lift measurement
(`EV-STRUCT-0011`) whose subjects are other surfaces; that record's reader warning forbids reading
its group total as any individual surface's rate. The honest statement is therefore: **the rounding
family was measured as a group for structural array admission; this surface was not measured
separately, and its arithmetic was not measured at all.**

The presence projection records the defect stream `BUG-FUNC-017` against this module.

Inputs I would probe first:

1. **`ROUNDUP(-1.2, 0)` beside `CEILING.MATH(-1.2)` and `ISO.CEILING(-1.2)`** — three cells that
   settle the away-from-zero-versus-toward-positive-infinity question.
2. **`ROUNDUP(3.0000000000000004, 0)` and `ROUNDUP(0.1+0.2, 1)`** — the last-bit sensitivity, using
   values that arithmetic actually produces rather than values typed by hand.
3. **A boundary case engineered to expose scaling undershoot**, where the exact product `|x|·10^d`
   slightly exceeds an integer but rounds down onto it. If the result equals that integer divided
   by the scale, the implementation is violating its own never-understate contract.
4. **`ROUNDUP(0, 5)` and `ROUNDUP(-0, 5)`** — the fixed point and its sign.
5. **`ROUNDUP(1.7976931348623157E+308, -1)`** — the overflow question the documentation does not
   address: saturate, error, or wrap.
6. **`ROUNDUP(1.5, 0.9)`** — non-integral `num_digits`, truncated in the reference engine and
   undocumented.
7. **Array arguments in both slots**, given the array-lift defect stream on this module.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| away from zero | The magnitude never decreases and the sign never changes |
| ceiling | Rounding toward positive infinity, which this function does **not** do |
| promotion | Advancing to the next multiple because of any nonzero excess, however small |
| scaling undershoot | The rounded product landing on an integer the exact product exceeds |
| accumulated bias | The systematic one-direction error of rounding many values up before summing |

## Sources

- Microsoft, "WorksheetFunction.RoundUp method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.roundup> ("Rounds a
  number up, away from 0 (zero)", the behaves-like-Round sentence, and the three `num_digits`
  remarks; no error conditions are stated).
- Microsoft Support, "ROUNDUP function" —
  <https://support.microsoft.com/en-us/office/roundup-function-f8bc9b23-e795-47db-8703-db171d0c42a7>
  (retrieval refused with HTTP 403 during this pass; nothing here is sourced from it).
- Gay, "Correctly rounded binary-decimal and decimal-binary conversions", Bell Labs NAM 90-10,
  1990; IEEE 754-2019 clause 4.3.2.
- Handbook, [ROUND](FUNC.ROUND.md) — the shared decimal-rounding analysis;
  [ROUNDDOWN](FUNC.ROUNDDOWN.md); [QUOTIENT](FUNC.QUOTIENT.md);
  [About implementation options](../model/07-implementation-options.md);
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.ROUNDUP.json` (`Custom` coercion,
  `precision_rounding_profile=default`) and `data/presence/FUNC.ROUNDUP.json` (own module; defect
  stream `BUG-FUNC-017`).
