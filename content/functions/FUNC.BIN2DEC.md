---
schema: efh.function-page/v1
function_id: FUNC.BIN2DEC
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

# BIN2DEC

## What it does

`BIN2DEC(number)` converts binary to decimal. The result is a numeric worksheet value.

## Try it

`=BIN2DEC("101101")` returns `45`. All of these represent the same integer: decimal 45, binary `101101`, octal `55`, and hexadecimal `2D`.

## Quirks

These engineering conversions use signed fixed-width encodings, not arbitrary-length integers. The source accepts at most ten digits; a full ten-digit encoding uses 10-bit two’s complement. `1111111111` represents minus one. Invalid source digits or an oversized input give `#NUM!`. Quote source digits to keep leading zeros and prevent worksheet number parsing from changing the input.

## How to implement

Separate input parsing, signed interpretation and output formatting. Uppercase the source, validate its digit set and length, and accumulate `raw=raw*2+digit` with checked integer arithmetic. Only a full ten-digit source with its top bit set is negative: subtract `2^10` to obtain signed integer `v`. Return `v` as a number. The 45 example is an exact integer at every step. Do not infer sign merely from the first source character of a short encoding.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit-bit, CPU not recorded | 313 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.BIN2DEC.json).

## Alternatives

[BASE](FUNC.BASE.md) and [DECIMAL](FUNC.DECIMAL.md) provide general radix conversion with different sign conventions. [DEC2BIN](https://handbook.dnacalc.com/functions/DEC2BIN.html) is the reverse direction.

## Sources

- [Microsoft: BIN2DEC function](https://support.microsoft.com/en-us/excel/functions/bin2dec-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/engineering_radix_family.rs); implementation steps and the limits identified in the text.
