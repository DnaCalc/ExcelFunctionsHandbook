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
  - What it does
  - Try it
  - Quirks
  - How to implement
  - Version differences
  - Alternatives
  - Sources
family: dollar_fraction_family
role_in_family: "The decimal-to-fraction direction: it encodes a fractional numerator in decimal digits over the supplied denominator."
---

# DOLLARFR

## What it does

`DOLLARFR(decimal_dollar,fraction)` converts an ordinary decimal amount to Excel’s fractional-quote numeric encoding. The digits after the decimal point encode a numerator over the supplied denominator; they are not an ordinary decimal fraction of the amount.

## Try it

`=DOLLARFR(12.375,32)` returns about `12.12`. The fraction 0.375 is twelve thirty-seconds, so the quote is twelve and 12/32. Reading `12.12` as an ordinary decimal amount would give a different value.

## Quirks

The denominator is truncated. A negative denominator gives `#NUM!`; a denominator that truncates to zero gives `#DIV/0!`. The output is a number, not text containing a slash. Numerator digit width depends on the denominator’s decimal scale, and negative amounts need a consistent toward-zero integer part.

## How to implement

Split the admitted decimal amount into its toward-zero integer part and signed fractional part. For denominator f choose the kernel’s decimal scale `10^ceil(log10(f))`; multiply the fractional part by f, divide by that scale and add the integer part. For f=32 the scale is 100, so `12+(0.375*32)/100=12.12`. The current helper constructs the scale by multiplying by ten until it is at least the denominator, rather than taking a floating-point logarithm; use its retained code for those boundaries rather than assuming all algebraic orderings agree.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, AMD Ryzen 7 4800H with Radeon Graphics | 2,406 |
| 20430 | Windows, 64-bit, CPU not recorded | 1,235 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.DOLLARFR.json).

## Alternatives

[DOLLARDE](FUNC.DOLLARDE.md) converts the quote back to an ordinary amount; [TEXT](FUNC.TEXT.md) can display a conventional written fraction when no numeric quote encoding is needed.

## Sources

- [Microsoft: DOLLARFR function](https://support.microsoft.com/en-us/excel/functions/dollarfr-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/dollar_fraction_family.rs); implementation steps and the limits identified in the text.
