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
  - "What it does"
  - "Try it"
  - "Quirks"
  - "How to implement"
  - "Version differences"
  - "Alternatives"
  - "Sources"
family: round_fn
role_in_family: "The nearest-value rounder; the middle member of the three-way ROUND / ROUNDDOWN / ROUNDUP split by direction."
---

# ROUND

## What it does

`ROUND(number, num_digits)` rounds a number to the requested decimal place. Positive
`num_digits` keeps places after the decimal point, zero rounds to an integer, and negative
values round to tens, hundreds and larger places. Both arguments are required.

The observed tie rule is away from zero: `ROUND(2.5,0)` is 3 and `ROUND(-2.5,0)` is -3 on the
named build below. Excel chooses the digit from a 15-significant-digit reading of the input.
That matters for familiar decimal examples: `ROUND(1.005,2)` is 1.01 and `ROUND(2.675,2)` is 2.68,
even though their stored binary values lie just below the corresponding decimal halfway points.

The result is a number, not formatted text. Changing a cell's displayed decimal places does
not perform this calculation or change the number used by later formulas.

## Try it

Start with `=ROUND(1.005,2)`. Compare `=ROUND(-2.5,0)`, `=ROUND(1234.5,-2)` and
`=ROUND(2.675,2)`. The browser computes with OneCalc; the Excel readings below remain a separate
comparison, with their own build and platform.

## Quirks

### The stored binary value is not the whole rounding rule

Decimal inputs usually have no finite binary64 representation. Rounding the exact stored value
to the nearest decimal step can therefore disagree with Excel's 15-digit reading. Replacing
ROUND with a language's default `round` is unsafe: that routine may use ties to even, and it may
choose the digit directly from the stored value.

### Precision arguments can wrap

On Excel 16.0 build 20430, `num_digits` is cut toward zero and limited to ±2147483647. A signed
32-bit calculation of the retained significant-digit count can wrap. Consequently an enormous
positive precision can return zero rather than leave the input alone. The boundary moves with
the input's decimal exponent; the recorded examples below give the calls and bits.

### Values near the largest finite number

The recorded build 20527 can return all-ones-exponent binary64 patterns which ordinary IEEE
arithmetic calls NaNs, while Excel's cell machinery treats them as numbers and displays a huge
decimal. Saving the workbook writes these values as `#NUM!`; recalculating can restore the
patterns. They must not be silently interpreted as ordinary finite results. The detailed
examples below distinguish cell inspection, arithmetic and file round trips.

## How to implement

For an ordinary mathematical decimal rounder, decompose the binary64 input into an exact
integer significand and power of two. Apply the decimal scale as integer powers of 2 and 5,
compare the remainder with halfway, choose a declared tie rule, then convert the resulting
decimal rational back to binary64. This avoids overflow and spurious ties from a floating-point
multiply followed by a divide. It intentionally does not reproduce Excel's preliminary reading.

To target the measured Excel route, preserve its separate stages: normalize the magnitude to
15 significant decimal digits using the observed x87 intermediate precision; apply that stage's
tie rule; truncate and clamp the precision argument; form the significant-digit count with
32-bit wrapping; then round at the requested digit away from zero and restore the sign.
The preliminary half rule for ROUND differs from the one observed for ROUNDDOWN, ROUNDUP and
TRUNC. One shared generic rounding instruction does not describe both stages.

Probe every transition in decimal exponent, halfway inputs and their immediate binary64
neighbours, large positive and negative precision, zero, and the upper finite boundary. A
scale-and-round formula alone does not establish agreement on those regions. Matching the
near-overflow cell encodings also requires their publication and file-serialization rules.

## Version differences

The observations below name builds 20430 and 20527, including a full 16.0.20527.20088 capture
where available. They cover different experiments, so differences in the recorded examples
must not be read as a demonstrated change between those builds. CPU information is included
only where the capture records it. Other platforms and versions still need corresponding probes.

## Alternatives

[ROUNDDOWN](FUNC.ROUNDDOWN.md) chooses toward zero; [ROUNDUP](FUNC.ROUNDUP.md) chooses away from
zero. [TRUNC](FUNC.TRUNC.md) discards decimal places. [MROUND](FUNC.MROUND.md) rounds to a stated
multiple; its halfway behaviour has its own numerical questions. A number format changes only
presentation. For a new mathematical API, an explicitly specified decimal or rational rounding
algorithm may be preferable to reproducing Excel's preliminary 15-digit reading.

## Sources

- [Microsoft: ROUND function](https://support.microsoft.com/en-us/excel/functions/round-function).
- [Precision count](../notes/FN-ROUND-001.json), [15-digit reading](../notes/FN-ROUND-002.json),
  [near-overflow encodings](../notes/FN-ROUND-003.json) and
  [tie examples](../notes/FN-ROUND-004.json).
- Gay, *Correctly rounded binary-decimal and decimal-binary conversions*, Numerical Analysis
  Manuscript 90-10 (1990), for the mathematical conversion problem.
