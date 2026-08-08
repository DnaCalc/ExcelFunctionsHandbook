---
schema: efh.function-page/v1
function_id: FUNC.DOLLARDE
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
family: dollar_fraction_family
role_in_family: >-
  The fraction-to-decimal direction: reads the digits after the decimal point as a numerator over
  the given denominator. DOLLARFR's inverse, and the member whose scaling rule is the family's one
  real trap.
---

# DOLLARDE

## What it computes

`DOLLARDE(fractional_dollar, fraction)` converts a price written in **fractional notation** — the
form US bond and note markets have quoted in for two centuries — into an ordinary decimal number.

A quote of `1.02` in thirty-seconds is not "one and two hundredths". It is *one dollar and two
thirty-seconds*, and the `.02` is a two-digit field holding the numerator. `DOLLARDE(1.02, 32)`
is therefore `1 + 2/32 = 1.0625`.

The transformation is:

    whole      = trunc(fractional_dollar)
    frac_field = fractional_dollar − whole
    result     = whole + frac_field × scale / fraction

where `fraction` is the denominator (truncated to an integer) and `scale` is the power of ten that
shifts the digit field up to a whole numerator. **The value of `scale` is determined by how many
decimal digits the denominator has**, and that is the whole subtlety of the function:

| `fraction` | Digits | `scale` | `DOLLARDE(1.02, fraction)` |
|---|---|---|---|
| 8 | 1 | 10 | `1 + 0.2/8 = 1.025` |
| 16 | 2 | 100 | `1 + 2/16 = 1.125` |
| 32 | 2 | 100 | `1 + 2/32 = 1.0625` |

So the number of digits you write after the point is not free notation — it must match the width
the denominator implies. `DOLLARDE(1.2, 32)` is `1 + 20/32`, not `1 + 2/32`, because the field is
two digits wide and `.2` means `20`. This is the single most common misreading of the function, and
it produces a plausible wrong answer rather than an error.

Range: for a well-formed quote the fractional field is in `[0, fraction)` and the result lies
between `whole` and `whole + 1`. Nothing enforces that. `DOLLARDE(1.99, 32)` computes
`1 + 99/32 ≈ 4.094` without complaint: the "fraction" was larger than the denominator, and the
function has no notion of a malformed quote.

Negative inputs work by symmetry, because both the truncation and the subtraction are toward zero:
`DOLLARDE(−1.02, 32) = −1.0625`.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `fractional_dollar` | A number expressed as an integer part and a fraction part, separated by a decimal symbol. Required. | — |
| `fraction` | The integer to use in the denominator of the fraction. Required. | — |

Microsoft documents that **if `fraction` is not an integer, it is truncated**. In practice this
means `DOLLARDE(1.02, 16.9)` is read as sixteenths.

`fraction` is not restricted to powers of two by anything in the function, even though halves,
quarters, eighths, sixteenths, thirty-seconds and sixty-fourths are the only denominators the
market convention uses. A denominator of 3 or 7 is accepted and produces arithmetic that is
internally consistent but does not correspond to any quotation.

Both slots are numeric and go through this family's own coercion, which is narrower than the
general rule — see *Result and edge cases*.

## Result and edge cases

Returns `Number`.

- **The digit-width rule above is the edge case that bites.** A quote whose fractional field is
  narrower than the denominator's digit count is a different price, not a formatting variant.
- **Fractional field larger than the denominator** is accepted and rolls over into the whole part.
- **`fraction` is truncated, not rounded**, so `15.99` reads as `15`, not as `16`. Truncation is
  toward zero.
- **Coercion in this family is not the general rule.** In the reference engine at commit `473efa3`
  the two argument slots accept numbers and numeric text, treat a blank cell as `0`, return `#N/A`
  for an omitted-slot Missing marker, propagate error values — and **reject logical values**
  outright rather than converting `TRUE` to 1. That last one is a documented departure from the
  general to-number rule in
  [Coercion and lifting](../model/02-coercion-and-lifting.md), which converts `TRUE → 1`. It is a
  per-family policy in the reference engine; whether Excel does the same is unverified.
- **A blank denominator** therefore reaches the kernel as `0` and produces `#DIV/0!` rather than a
  missing-argument diagnosis.
- Array arguments: `DOLLARDE` is declared as lifting natively over arrays, while its inverse
  `DOLLARFR` is declared with an explicit broadcast over both argument positions. See
  [DOLLARFR](FUNC.DOLLARFR.md) — the asymmetry between two functions that are inverses of each
  other is itself a finding.

## Errors

As documented on Microsoft's `DOLLARDE` page:

| Error | Condition |
|---|---|
| `#NUM!` | `fraction < 0` |
| `#DIV/0!` | `fraction ≥ 0` and `fraction < 1` |

Note the shape of that pair: the `#NUM!` test is on the **untruncated** value, so `−0.5` is
`#NUM!`, while the `#DIV/0!` test catches everything from `0` up to but not including `1` — which
is exactly the range whose truncation is zero. The reference engine implements the two guards in
that order and with those codes.

Outside the documented pair, non-numeric text and unsupported value kinds surface as `#VALUE!`
under this family's coercion, error values propagate, and an omitted argument slot yields `#N/A` in
the reference engine. None of that is documented and none of it has been observed in Excel by the
Handbook.

## Relationships

- **[DOLLARFR](FUNC.DOLLARFR.md)** — the exact inverse, and the only other member of this family.
  `DOLLARFR(DOLLARDE(x, f), f)` should return `x` up to floating-point rounding, and the round trip
  is a good self-test of the scaling rule.
- **`DOLLAR`** — unrelated despite the name. `DOLLAR` formats a number as currency *text*;
  `DOLLARDE` returns a number. They share four letters and nothing else.
- **`TEXT`** — the general formatter, and the tool you actually want if the goal is to *display* a
  price in fractional notation rather than to compute with one. Excel's `# ?/32` number formats
  cover the display direction.
- **`PRICE` / `PRICEDISC`** — the bond-pricing functions whose output is a decimal price per 100.
  `DOLLARFR` is what turns their answer into a market quote; `DOLLARDE` turns a market quote into
  something they can consume.

## Numerical notes

The arithmetic is exact for the cases that matter, and the one interesting question is not
rounding but *specification*.

**The digit count of the denominator is a decision, and there is more than one way to make it.**
The reference engine computes `scale` as `10` raised to the **number of decimal digits in the
decimal representation of the truncated denominator** — `len("16") = 2 → 100`. An alternative
formulation that appears in other implementations is `10^ceil(log10(fraction))`. The two agree on
every denominator that is *not* an exact power of ten, and disagree on `10`, `100`, `1000`:

| `fraction` | digit-count rule | `ceil(log10)` rule |
|---|---|---|
| 16 | `scale = 100` | `scale = 100` |
| 10 | `scale = 100` | `scale = 10` |
| 100 | `scale = 1000` | `scale = 100` |

For `DOLLARDE(1.5, 10)` the first rule gives `1 + 0.5·100/10 = 6` and the second gives
`1 + 0.5·10/10 = 1.5`. That is not a last-bit difference; it is a different function. Which one
Excel implements is **not settled by this page**, and `DOLLARDE(1.5, 10)` is the cheapest probe in
this whole batch. Power-of-ten denominators are outside market convention, which is presumably why
the ambiguity has survived.

**Exactness.** When the denominator is a power of two and the numerator field is a small integer,
`frac_field × scale` is not exact — `1.02 − 1.0` is not exactly `0.02` in binary64 — so the result
is not the exactly-representable dyadic rational the market quote denotes. `DOLLARDE(1.02, 32)`
returns the binary64 nearest to `1.0625`, but it arrives there through an inexact intermediate, and
a differently-ordered evaluation can land one ULP away. An implementation aiming for the
mathematically correct answer would recover the integer numerator by rounding
`frac_field × scale` to the nearest integer before dividing; the reference engine does not, and
neither, as far as this page can say, does anything else.

**The `as i32` conversion.** The reference engine truncates the denominator and converts it to a
32-bit integer. Rust's `as` cast saturates, so a denominator above 2³¹−1 becomes `i32::MAX` rather
than overflowing — an implementation fact with an observable consequence at absurd inputs, and a
probe target.

## What has not been checked

No Handbook vector suite exists for `DOLLARDE`; `vectors/` publishes nothing at this revision. No
Excel-comparison evidence record names `DOLLARDE` in its subjects — it appears only inside the body
of a record whose subjects are other functions, which the Handbook does not treat as evidence about
this surface. **Nobody has checked this function against Excel within the Handbook's record.** The
two error rows and the truncation remark are Microsoft's documented statements; the scaling rule,
the coercion policy and the integer conversion are read from the reference engine's source at
commit `473efa3`.

The implementing module carries one open upstream defect stream touching this surface,
`BUG-FUNC-028`, concerning text, date, array-lift and coercion gaps — so the coercion behaviour
described above should be read as unsettled even within the reference engine.

Inputs worth probing first:

1. **`DOLLARDE(1.5, 10)` and `DOLLARDE(1.5, 100)`** — the power-of-ten ambiguity. Two cells decide
   between two genuinely different functions, and no other probe on this page matters as much.
2. **`DOLLARDE(1.2, 32)` against `DOLLARDE(1.02, 32)`** — the digit-width rule, the mistake real
   users make.
3. **`DOLLARDE(1.99, 32)`** — a fractional field exceeding the denominator, to confirm it rolls
   over rather than erroring.
4. **`DOLLARDE(TRUE, 16)` and `DOLLARDE(1.02, TRUE)`** — the logical-rejection policy, which
   contradicts the general coercion rule and is therefore the highest-value coercion probe.
5. **`DOLLARDE(1.02, )` with an empty slot, and `DOLLARDE(1.02, A1)` with `A1` blank** — the
   `#N/A` versus `#DIV/0!` split between Missing and Empty.
6. **`DOLLARDE(1.02, −0.5)`** — `#NUM!` on the untruncated negative, distinguishing the order of
   the two documented guards.
7. **`DOLLARDE(−1.02, 32)`** — sign symmetry through the truncation.
8. **Array arguments in each position**, given the declared lift asymmetry with `DOLLARFR` and the
   open defect stream.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| fractional notation | A price written as whole units plus a numerator field after the decimal point |
| digit field | The decimal digits after the point, read as an integer numerator |
| scale | The power of ten that lifts the digit field to a whole numerator |
| denominator digit count | The number of decimal digits in the truncated `fraction`, which fixes `scale` |
| round trip | `DOLLARFR(DOLLARDE(x, f), f)`, the family's self-inverse check |

## Sources

- Microsoft, "DOLLARDE function" —
  <https://support.microsoft.com/en-us/office/dollarde-function-db85aab0-1677-428a-9dfd-a38476693427>
  (syntax, both argument descriptions, the truncation remark, and the `#NUM!` and `#DIV/0!`
  conditions, quoted above in the form the page states them).
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — the general to-number
  rule this family departs from, and the Missing/Empty distinction.
- Handbook, [The value universe](../model/01-value-universe.md).
- OxFunc `crates/oxfunc_core/src/functions/dollar_fraction_family.rs` at commit `473efa3` — the
  `dollarde_kernel`, the `decimal_scale` digit-count rule, the denominator guards and the family's
  argument coercion.
- Handbook projections `data/functions/FUNC.DOLLARDE.json` and `data/presence/FUNC.DOLLARDE.json`
  (arity, classification axes, implementing module, sibling, and the `BUG-FUNC-028` defect stream).
