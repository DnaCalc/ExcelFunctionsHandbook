---
schema: efh.function-page/v1
function_id: FUNC.ROUND
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft — WorksheetFunction.Round method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.round"
    role: "the three documented remarks about num_digits greater than, equal to and less than zero; note that no tie-breaking rule is stated"
  - work: "Microsoft Support — ROUND function"
    locator: "https://support.microsoft.com/en-us/office/round-function-c018c5d8-40fb-4053-90b1-b3e7f61a213c"
    role: "the worksheet-surface documentation page named in the projection; not retrievable for this pass"
  - work: "Steele & White, How to print floating-point numbers accurately"
    locator: "PLDI 1990"
    role: "why decimal rounding of a binary value is a conversion problem, not an arithmetic one"
  - work: "Gay, Correctly rounded binary-decimal and decimal-binary conversions"
    locator: "AT&T Bell Labs Numerical Analysis Manuscript 90-10, 1990"
    role: "the reference algorithm for exact decimal rounding of a binary64 value"
  - work: "IEEE 754-2019, clause 5.9 and clause 9"
    locator: "roundToIntegralTiesToEven; the decimal rounding attributes"
    role: "the standard's own rounding directions, against which Excel's tie rule is undocumented"
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
family: round_fn
role_in_family: "The nearest-value rounder; the middle member of the three-way ROUND / ROUNDDOWN / ROUNDUP split by direction."
---

# ROUND

## What it computes

`ROUND(number, num_digits)` rounds `number` to `num_digits` decimal places.

    ROUND(x, d) = the multiple of 10^-d nearest to x

with a tie rule that Microsoft's documentation does not state — see below. Positive `d` rounds to
the right of the decimal point, `d = 0` rounds to an integer, negative `d` rounds to the left.
Microsoft's VBA reference gives exactly those three remarks and nothing else:

> If num_digits is greater than 0 (zero), number is rounded to the specified number of decimal
> places. If num_digits is 0, number is rounded to the nearest integer. If num_digits is less than
> 0, number is rounded to the left of the decimal point.

**The mathematics has a trap built into its own statement.** "The multiple of 10⁻ᵈ nearest to `x`"
is a *decimal* operation on a *binary* value. For `d > 0`, `10⁻ᵈ` is not representable in binary64
— 0.1, 0.01, 0.001 are all infinite binary fractions — so the set of "multiples of 10⁻ᵈ" that the
function is supposed to round to does not exist in the number system it is computing in. The
result of `ROUND(x, 2)` is not a multiple of 0.01; it is the binary64 value nearest to the multiple
of 0.01 nearest to `x`. Two roundings, not one, and the second one is invisible.

This is not a defect in Excel. It is what decimal rounding of a binary number *is*, and every
system that offers this function has to decide how carefully to do it.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The value to round. Required. | — |
| `num_digits` | The number of decimal places. Required — not optional. | — |

Both are numeric slots under ordinary to-number coercion; see
[Coercion and lifting](../model/02-coercion-and-lifting.md). Note that `num_digits` is
**required**: `ROUND(1.5)` is refused at formula entry, not defaulted to zero. This is one of the
few places where Excel demands an argument that most languages default.

**Non-integral `num_digits` is undocumented.** The reference engine truncates it toward zero, so
`ROUND(1.5, 0.9)` behaves as `ROUND(1.5, 0)`. Rounding it, or refusing it, would both be
defensible; nothing documented decides.

## Result and edge cases

Returns `Number`.

- **The tie rule is not documented.** Microsoft's VBA remarks say "rounded to the nearest" and
  stop. The reference engine rounds **half away from zero** — `1.25 → 1.3`, `−1.25 → −1.3` — which
  is the convention conventionally attributed to Excel and is *not* the IEEE 754 default
  (ties-to-even). The Handbook records the tie rule as **undocumented on the page it could read**,
  implemented one way in the reference engine, and unverified against Excel.
- **Ties mostly do not exist.** For `d > 0` an exact tie requires `x` to be exactly half a
  multiple of 10⁻ᵈ, which requires `x` to be exactly representable — true for values like 1.25 and
  0.5, false for 2.675, 1.005, 8.835 and almost every other "obvious" tie a user will type. Those
  values are *not* ties: their binary64 representations sit strictly below the halfway point, so
  "correct" rounding takes them down, while a naive scale-and-round can take them up. Which
  behaviour a system exhibits is the single most-asked question about `ROUND` in any spreadsheet,
  and this page does not answer it for Excel because the Handbook has not measured it.
- **Large `num_digits`.** The reference engine returns `number` unchanged once `num_digits` reaches
  308, and returns a signed zero once it reaches −308. Those clamps are implementation limits, not
  documented behaviour.
- **Negative zero.** Rounding a small negative number to a coarse place produces `−0` in the
  reference engine — a value that displays as `0` but is not the same bit pattern, and which
  behaves differently under `SIGN`-style inspection and under text conversion in some contexts.
  See the raw-versus-published discussion in
  [The value universe](../model/01-value-universe.md).
- **Arrays.** The projection declares `UnaryNumericScalarOnly` with `surface_native` lifting.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | Either argument does not convert to a number | shared coercion model |
| propagated | An error value in either argument | shared coercion model |
| `#VALUE!` | Wrong argument count (`num_digits` omitted) | arity, refused at entry |

Microsoft's VBA page states no error condition for this method.

## Relationships

- **[ROUNDDOWN](FUNC.ROUNDDOWN.md)** and **[ROUNDUP](FUNC.ROUNDUP.md)** — the same operation with
  the direction fixed: toward zero and away from zero. The three together are a directed-rounding
  family, and the missing members are instructive: there is no "round toward negative infinity at
  `d` places" (that is `FLOOR.MATH` with a significance of `10^-d`, on a different argument
  convention) and no ties-to-even variant at all.
- **`TRUNC`** — `TRUNC(x, d)` is `ROUNDDOWN(x, d)`, with `d` optional where `ROUNDDOWN`'s is
  required. Two functions, one operation, different argument requirements.
- **`INT`** — floors rather than truncates, so `INT(-1.5)` is `-2` while `ROUND(-1.5, 0)` and
  `TRUNC(-1.5)` are `-2` and `-1` respectively. Three functions, three answers, one input.
- **`MROUND`, `CEILING.MATH`, `FLOOR.MATH`** — round to a multiple of an arbitrary
  *significance* rather than to a decimal place. `MROUND(x, 0.01)` and `ROUND(x, 2)` are the same
  intent by different routes, and they do not always agree, because `0.01` as a significance is a
  binary64 value slightly different from one hundredth.
- **Cell display formatting** — the other thing that "rounds" a number, and the reason `ROUND` is
  reached for so often. A displayed value is a rendering; `ROUND` changes the stored number. So
  does the workbook-level *Precision as displayed* setting, which changes it globally and
  destructively.

## Numerical notes

**The scale-round-unscale algorithm is the source of every anomaly.** The obvious implementation is

    ROUND(x, d) ≈ round_to_integer(x · 10^d) / 10^d

and it rounds three times: once forming `10^d` (exact only for `d` ≤ 22), once in the
multiplication, once in the division. The multiplication is the dangerous one. Take a value whose
decimal literal appears to be an exact tie: its binary64 neighbour is slightly below the tie, but
multiplying by `10^d` can round the product *up* to exactly the halfway integer, at which point the
tie rule fires and the result goes up — producing an answer that looks right to a user reading
decimal and is wrong with respect to the actual stored value. The reverse also happens. This is why
`ROUND` in different systems disagrees on inputs like 2.675 and 1.005, and why the disagreement is
about the *representation*, not about the rounding rule.

**The correct algorithm does not multiply.** The exact approach treats the problem as what it is —
a binary-to-decimal conversion — and decides the digit by comparing exactly, using the same
machinery as correctly rounded printing: Steele & White's Dragon4, Gay's `gdtoa`, or a modern
Grisu/Ryū with a fallback. Round the *decimal representation* of `x` at position `d`, then convert
back once. That gives a single rounding and a result that matches what a user reading the decimal
expansion of the stored value would compute by hand. It is more code and it is the right answer.

**A cheaper middle route** is to compute the naive result and then verify it: form the candidate,
compute the exact residual `x − candidate` using a fused multiply-add or a two-product, and check
which side of the halfway point `x` actually lies on, correcting by one ulp of `10^-d` if the naive
step chose wrong. Two extra operations, and it removes the multiplication-induced misrounding
without a full decimal conversion.

**The unscaling is a second, unavoidable rounding.** Even a perfect decimal round has to return a
binary64 value, and the nearest binary64 to `1.23` is not `1.23`. So `ROUND(x, 2) * 100` is not
generally an integer, and a chain of rounded values does not sum to the rounded sum. Financial
workbooks that need exact cent arithmetic should carry integer cents, not rounded decimals — this
is the practical consequence, and it is worth stating on the page that people arrive at while
trying to fix exactly that problem.

**Directed rounding at negative `d` is exact more often than at positive `d`,** because `10^-d` for
`d < 0` is a positive power of ten, and positive powers of ten up to 10²² are exactly representable
in binary64. `ROUND(x, -2)` therefore has one fewer source of error than `ROUND(x, 2)`.

## What has not been checked

No Handbook vector suite exists for `ROUND`, and **no evidence record lists this surface among its
subjects**. `ROUND` does appear inside the group membership of an upstream array-lift measurement
(`EV-STRUCT-0011`), but that record's subjects are other surfaces and its reader warning forbids
reading its group total as any individual surface's result. The honest statement is therefore:
**the rounding family was measured as a group for structural array admission; this surface was not
measured separately, and its arithmetic was not measured at all.**

The presence projection records the defect stream `BUG-FUNC-017` (a math scalar/array-lift gap)
against this module.

Inputs I would probe first:

1. **`ROUND(2.675, 2)`, `ROUND(1.005, 2)`, `ROUND(8.835, 2)`** — the three canonical
   near-tie decimals. Each stored value lies strictly below its apparent halfway point, so a
   correctly rounding implementation goes down and a scale-and-round implementation goes up. Three
   cells characterise the algorithm.
2. **`ROUND(1.25, 1)` and `ROUND(-1.25, 1)`, `ROUND(0.5, 0)` and `ROUND(1.5, 0)` and
   `ROUND(2.5, 0)`** — genuine exact ties, which is the only way to observe the tie rule. If 2.5
   rounds to 2, the rule is ties-to-even and this page's assumption is wrong.
3. **`ROUND(-1, -1)`** and `1/ROUND(-1,-1)` — whether a negative zero survives to the sheet, and
   whether it is observable at all.
4. **`ROUND(1.5, 0.9)` and `ROUND(1.5, -0.9)`** — non-integral `num_digits`, truncated in the
   reference engine and undocumented everywhere.
5. **`ROUND(x, 400)` and `ROUND(x, -400)`** — the clamp boundaries, which are pure implementation
   limits with no documented basis.
6. **`ROUND(1.7976931348623157E+308, -1)`** — whether rounding away from zero at the top of the
   range overflows to an error or saturates.
7. **`ROUND(A1:A3, B1:B3)` with array arguments**, given the array-lift defect stream on this
   module.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| decimal place | The exponent `d` in `10^-d`; not a representable binary quantity for `d > 0` |
| tie | An input exactly halfway between two representable multiples of `10^-d` |
| near-tie | An input whose decimal literal looks like a tie but whose stored value is not one |
| scale-round-unscale | The naive algorithm, and the source of near-tie disagreement |
| negative zero | The signed zero produced by rounding a small negative value to a coarse place |

## Sources

- Microsoft, "WorksheetFunction.Round method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.round> (the three
  documented remarks; **no tie rule and no error condition are stated**).
- Microsoft Support, "ROUND function" —
  <https://support.microsoft.com/en-us/office/round-function-c018c5d8-40fb-4053-90b1-b3e7f61a213c>
  (retrieval refused with HTTP 403 during this pass; nothing here is sourced from it).
- Steele & White, "How to print floating-point numbers accurately", PLDI 1990; Gay, "Correctly
  rounded binary-decimal and decimal-binary conversions", Bell Labs NAM 90-10, 1990 — the exact
  decimal-rounding machinery.
- IEEE 754-2019 — the rounding-direction attributes, including ties-to-even as the default that
  Excel's documented text neither adopts nor rejects.
- Handbook, [The value universe](../model/01-value-universe.md),
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.ROUND.json` (`UnaryNumericScalarOnly`,
  `precision_rounding_profile=default`) and `data/presence/FUNC.ROUND.json` (own module; defect
  stream `BUG-FUNC-017`).
