---
schema: efh.function-page/v1
function_id: FUNC.BITAND
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
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
family: null
role_in_family: null
---

# BITAND

## What it does

`BITAND(number1, number2)` combines two nonnegative integers bit by bit. A result bit is one when both input bits are one. The returned value is a decimal worksheet number.

## Try it

`=BITAND(12,10)` returns `8`. In four bits the inputs are `1100` and `1010`; applying AND gives `1000`.

## Quirks

Both inputs must already be integers in `0..2^48-1` (`281474976710655`). Fractions are rejected with `#NUM!`, rather than truncated. Nonnumeric arguments give `#VALUE!`. Negative numbers do not denote a signed two’s-complement operand in this family.

## How to implement

Coerce and validate the two numbers before integer conversion. Use a type able to retain all 48 admitted bits, then compute `left & right` and convert the result to binary64. Every admitted integer and result is exactly representable in binary64. Languages whose bitwise operators first narrow to 32 bits need a wider integer or arbitrary-precision integer path. The four-bit example uses no approximation; validation and error ordering are the separate worksheet obligations.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 5,989 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.BITAND.json).

## Alternatives

[BITAND](https://handbook.dnacalc.com/functions/BITAND.html) masks bits, [BITOR](https://handbook.dnacalc.com/functions/BITOR.html) sets selected bits and [BITXOR](https://handbook.dnacalc.com/functions/BITXOR.html) toggles them. [BITLSHIFT](https://handbook.dnacalc.com/functions/BITLSHIFT.html) moves bit positions.

## Sources

- [Microsoft: BITAND function](https://support.microsoft.com/en-us/excel/functions/bitand-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/bit_common.rs); implementation steps and the limits identified in the text.
