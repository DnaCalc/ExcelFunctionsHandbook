---
schema: efh.function-page/v1
function_id: FUNC.HEX2DEC
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

# HEX2DEC

## What it does

`HEX2DEC(number)` converts hexadecimal to decimal. The result is a numeric worksheet value.

## Try it

`=HEX2DEC("2D")` returns `45`. All of these represent the same integer: decimal 45, binary `101101`, octal `55`, and hexadecimal `2D`.

For a negative value, `=HEX2DEC("FFFFFFFFD3")` returns `-45`. Accumulation gives `1099511627731`; the full-width sign bit is set, so subtract `2^40` to get `-45`. The shorter source `"FFD3"` is positive `65491`.

## Quirks

These engineering conversions use signed fixed-width encodings, not arbitrary-length integers. The source accepts at most ten digits; a full ten-digit encoding uses 40-bit two’s complement. `FFFFFFFFFF` represents minus one. Invalid source digits or an oversized input give `#NUM!`. Quote source digits to keep leading zeros and prevent worksheet number parsing from changing the input.

## How to implement

Separate input parsing, signed interpretation and output formatting. Uppercase the source, validate its digit set and length, and accumulate `raw=raw*16+digit` with checked integer arithmetic. Only a full ten-digit source with its top bit set is negative: subtract `2^40` to obtain signed integer `v`. Return `v` as a number. The 45 example is an exact integer at every step. Do not infer sign merely from the first source character of a short encoding.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,025 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.HEX2DEC.json).

## Alternatives

[DEC2HEX](FUNC.DEC2HEX.md) converts in the reverse direction. [BASE](FUNC.BASE.md) and [DECIMAL](FUNC.DECIMAL.md) provide general radix conversion with different sign rules. A full-width negative engineering encoding is not a minus-prefixed numeral; choose the pair whose sign convention fits your input.

## Sources

- [Microsoft: HEX2DEC function](https://support.microsoft.com/en-us/excel/functions/hex2dec-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/engineering_radix_family.rs); implementation steps and the limits identified in the text.
