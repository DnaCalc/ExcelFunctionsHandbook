---
schema: efh.function-page/v1
function_id: FUNC.DOLLARFR
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-STRUCT-0006
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
  The decimal-to-fraction direction: packs the fractional remainder back into a digit field over the
  given denominator. DOLLARDE's inverse, and the member the reference engine declares with an
  explicit two-position array broadcast.
---

# DOLLARFR

## What it computes

`DOLLARFR(decimal_dollar, fraction)` converts an ordinary decimal price into **fractional
notation** — the market form in which the digits after the decimal point are a numerator over a
stated denominator rather than a decimal fraction.

`DOLLARFR(1.0625, 32)` returns `1.02`, which reads as *one and two thirty-seconds*. The `.02` is
not two hundredths; it is a two-digit numerator field.

The transformation is the exact inverse of [DOLLARDE](FUNC.DOLLARDE.md):

    whole     = trunc(decimal_dollar)
    remainder = decimal_dollar − whole
    result    = whole + remainder × fraction / scale

where `fraction` is the denominator (truncated to an integer) and `scale` is the power of ten wide
enough to hold a numerator up to `fraction − 1`. As on the `DOLLARDE` page, **`scale` is fixed by
the number of decimal digits in the denominator**, and that is the family's one real trap:

| `fraction` | Digits | `scale` | `DOLLARFR(1.125, fraction)` |
|---|---|---|---|
| 8 | 1 | 10 | `1 + 0.125·8/10 = 1.1` |
| 16 | 2 | 100 | `1 + 0.125·16/100 = 1.02` |
| 32 | 2 | 100 | `1 + 0.125·32/100 = 1.04` |

The result is **not a number you may do arithmetic on**. `DOLLARFR(1.0625, 32) + DOLLARFR(1.03125,
32)` is `1.02 + 1.01 = 2.03`, which reads as two and three thirty-seconds — and the true sum is
`2.09375`, which is two and three thirty-seconds. It happens to work here because the numerators
did not carry. Add two quotes whose numerators sum past the denominator and the notation breaks
silently: `1.31 + 1.02` gives `2.33` in thirty-seconds, which is not a valid quote at all. Convert
back with `DOLLARDE` before doing anything arithmetic.

Range: for `remainder` in `[0, 1)` the output's fractional field lies in `[0, fraction/scale)`,
which for the market denominators (8, 16, 32, 64) is a proper digit field. Nothing enforces it.

Negative inputs work by symmetry: both the truncation and the subtraction go toward zero, so
`DOLLARFR(−1.125, 16) = −1.02`.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `decimal_dollar` | A decimal number. Required. | — |
| `fraction` | The integer to use in the denominator of the fraction. Required. | — |

Microsoft documents that a non-integer `fraction` is **truncated**, and the same two error
conditions as for `DOLLARDE` — a negative denominator is `#NUM!`, a denominator in `[0, 1)` is
`#DIV/0!`.

`fraction` is not restricted to powers of two. Denominators such as 3 or 7 are accepted and produce
consistent arithmetic that corresponds to no quotation convention.

## Result and edge cases

Returns `Number` — a number whose decimal digits are to be *read*, not evaluated.

- **The digit-width rule** above governs the shape of the output: a sixty-fourths quote occupies a
  two-digit field, so `DOLLARFR(1.015625, 64)` is `1.01`, and a reader who strips the leading zero
  has changed the price.
- **Rounding is not applied.** `DOLLARFR` does not snap the input to the nearest representable
  quote: a decimal price that is not an exact multiple of `1/fraction` produces a fractional field
  with digits beyond the numerator, e.g. a third-of-a-thirty-second appears as trailing decimals.
  If you want the nearest tradeable quote you must round first — `DOLLARFR(ROUND(p*32,0)/32, 32)`.
  This is a real source of "why does my quote have six decimal places" confusion.
- **Coercion in this family is narrower than the general rule.** In the reference engine at commit
  `473efa3` both slots accept numbers and numeric text, map a blank cell to `0`, return `#N/A` for
  an omitted-slot Missing marker, propagate error values, and **reject logical values** instead of
  converting `TRUE` to 1 — a departure from the general to-number rule in
  [Coercion and lifting](../model/02-coercion-and-lifting.md). Whether Excel does the same is
  unverified.
- **A blank denominator** reaches the kernel as `0` and yields `#DIV/0!`.
- **Array arguments.** The reference engine declares `DOLLARFR` as *scalar-shaped by index*,
  broadcasting over **both** argument positions, while `DOLLARDE` is declared as lifting natively.
  The module's own comment attributes this asymmetry to live Excel 16.0 build 20026. Two functions
  that are exact inverses of one another carrying different declared lift shapes is a structural
  fact worth stating plainly, and it is the axis the evidence record below sits on.

## Errors

As documented on Microsoft's `DOLLARFR` page, matching the `DOLLARDE` pair:

| Error | Condition |
|---|---|
| `#NUM!` | `fraction < 0` |
| `#DIV/0!` | `fraction ≥ 0` and `fraction < 1` |

The `#NUM!` test applies to the **untruncated** argument, so `−0.5` is `#NUM!`; the `#DIV/0!` test
covers exactly the range whose truncation is zero. The reference engine implements the two guards
in that order with those codes.

Outside the documented pair: non-numeric text and unsupported kinds surface as `#VALUE!` under this
family's coercion, error values propagate, and an omitted slot yields `#N/A` in the reference
engine. None of that is documented and none of it has been observed in Excel by the Handbook.

## Relationships

- **[DOLLARDE](FUNC.DOLLARDE.md)** — the exact inverse and the family's only other member.
  `DOLLARDE(DOLLARFR(x, f), f)` should return `x` up to floating-point rounding; the round trip is
  the natural self-test, and it is also the test most likely to expose the scaling ambiguity
  described below.
- **`DOLLAR`** — unrelated. It formats a number as currency *text*. The name collision is
  historical.
- **`TEXT`** and Excel's `# ?/32` number formats — the display route. If nothing downstream needs
  to compute on the fractional form, formatting is safer than `DOLLARFR`, because a formatted cell
  still holds the true decimal value.
- **`ROUND` / `MROUND`** — the pre-step that turns an arbitrary decimal into an exact multiple of
  `1/fraction` before conversion.
- **`PRICE` / `PRICEDISC` / `PRICEMAT`** — bond prices per 100 in decimal, the usual input.

## Numerical notes

**The power-of-ten ambiguity.** The reference engine derives `scale` from the number of decimal
digits in the truncated denominator (`len("32") = 2 → 100`). An alternative formulation used
elsewhere is `10^ceil(log10(fraction))`. The two agree on every denominator that is not an exact
power of ten and disagree on `10`, `100`, `1000`. For `DOLLARFR(1.5, 10)` the digit-count rule gives
`1 + 0.5·10/100 = 1.05` and the `ceil(log10)` rule gives `1 + 0.5·10/10 = 1.5`. Which one Excel
implements is **not settled by this page**; it is the cheapest and highest-value probe here. The
ambiguity survives because power-of-ten denominators are outside market convention.

**Exactness and the reverse direction.** `remainder × fraction / scale` divides by a power of ten,
which is not exactly representable in binary64 for `scale ≥ 10`. So `DOLLARFR` returns the binary64
*nearest* to the intended quote, not the quote itself, and the round trip through `DOLLARDE`
accumulates two such roundings. For the small integers involved in real quotes the round trip
recovers the input, but that is a property of the numbers being small, not a guarantee. An
implementation aiming for the mathematically correct answer would form the integer numerator by
rounding `remainder × fraction` before scaling; the reference engine does not.

**The `as i32` conversion.** The reference engine truncates the denominator and converts to a
32-bit integer with a saturating cast, so a denominator above 2³¹−1 becomes `i32::MAX` rather than
wrapping. An implementation fact with an observable consequence at absurd inputs.

## What has not been checked

No Handbook vector suite exists for `DOLLARFR`; `vectors/` publishes nothing at this revision, so no
suite-scoped claim exists for it.

One Excel-comparison record does name `DOLLARFR` in its subjects: **EV-STRUCT-0006**, a
structural-verification record covering scalar-parameter array lift across a group of functions, in
which `DOLLARFR` appears as one subject among several. Its figures are group totals over a set of
formulas spanning several functions and do **not** decompose onto `DOLLARFR`; the record therefore
establishes that `DOLLARFR` was inside a live-Excel structural comparison, not a per-function
result, and its own reader warning states that it says nothing about numeric exactness inside the
lifted kernels. Read the record for its scope, its build caveats and its caveats about
architecture. Nothing about the *value* `DOLLARFR` computes has been compared against Excel within
the Handbook's record.

The implementing module carries one open upstream defect stream touching this surface,
`BUG-FUNC-018`, on successor scalar-parameter array-lift gaps — the same axis the evidence record
sits on.

Inputs worth probing first:

1. **`DOLLARFR(1.5, 10)` and `DOLLARFR(1.5, 100)`** — the power-of-ten scaling ambiguity. Two cells
   decide between two different functions.
2. **`DOLLARDE(DOLLARFR(x, 32), 32)` over a spread of `x`** — the round trip, which localizes any
   scaling or rounding disagreement to one of the two members.
3. **`DOLLARFR(1.0625, 32)` and `DOLLARFR(1.015625, 64)`** — the digit-width rule at the two
   denominators the market actually uses.
4. **A price that is not an exact multiple of `1/fraction`**, e.g. `DOLLARFR(1.07, 32)` — confirms
   that no rounding to the nearest quote happens.
5. **`DOLLARFR(TRUE, 16)`** — the logical-rejection policy, which contradicts the general coercion
   rule.
6. **An omitted second argument versus a blank-cell second argument** — the `#N/A` versus `#DIV/0!`
   split between Missing and Empty.
7. **Array arguments in each position separately**, which is the declared asymmetry with
   `DOLLARDE` and the live axis of both the evidence record and the open defect stream.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| fractional notation | A price written as whole units plus a numerator field after the decimal point |
| digit field | The decimal digits after the point, to be read as an integer numerator |
| scale | The power of ten wide enough to hold a numerator up to `fraction − 1` |
| denominator digit count | The number of decimal digits in the truncated `fraction`, which fixes `scale` |
| round trip | `DOLLARDE(DOLLARFR(x, f), f)`, the family's self-inverse check |

## Sources

- Microsoft, "DOLLARFR function" —
  <https://support.microsoft.com/en-us/office/dollarfr-function-0835d163-3023-4a33-9824-3042c5d4f495>
  (syntax, argument descriptions, the truncation remark, and the `#NUM!` and `#DIV/0!` conditions).
- Microsoft, "DOLLARDE function" —
  <https://support.microsoft.com/en-us/office/dollarde-function-db85aab0-1677-428a-9dfd-a38476693427>
  (the inverse direction, whose two error conditions were read verbatim for this batch).
- Handbook evidence record `EV-STRUCT-0006` — the structural-verification record naming `DOLLARFR`
  among its subjects; group totals, with its own reader warning.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [The value universe](../model/01-value-universe.md).
- OxFunc `crates/oxfunc_core/src/functions/dollar_fraction_family.rs` at commit `473efa3` — the
  `dollarfr_kernel`, the `decimal_scale` digit-count rule, the declared two-position lift broadcast
  and its build attribution, and the family's argument coercion.
- Handbook projections `data/functions/FUNC.DOLLARFR.json` and `data/presence/FUNC.DOLLARFR.json`
  (arity, classification axes, implementing module, sibling, and the `BUG-FUNC-018` defect stream).
