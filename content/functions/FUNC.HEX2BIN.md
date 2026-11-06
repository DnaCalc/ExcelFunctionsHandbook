---
schema: efh.function-page/v1
function_id: FUNC.HEX2BIN
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

# HEX2BIN

## What it does

`HEX2BIN(number, [places])` converts hexadecimal to binary. The result is text, with optional leading zeros controlled by `places`.

## Try it

`=HEX2BIN("2D",8)` returns `"00101101"`. All of these represent the same integer: decimal 45, binary `101101`, octal `55`, and hexadecimal `2D`.

For a negative value, `=HEX2BIN("FFFFFFFFD3",4)` returns `"1111010011"`. Decode `1099511627731-2^40=-45`, then encode `1024-45=979` in ten binary digits. A positive source such as `"200"` is `512`, outside the signed binary range, and gives `#NUM!`.

## Quirks

These engineering conversions use signed fixed-width encodings, not arbitrary-length integers. The source accepts at most ten digits; a full ten-digit encoding uses 40-bit two’s complement. `FFFFFFFFFF` represents minus one. Invalid source digits or an oversized input give `#NUM!`. For a positive result a width too small gives `#NUM!`. A negative result uses ten digits; the current code still validates an explicitly supplied width before emitting it, despite documentation describing the width as ignored.

## How to implement

Separate input parsing, signed interpretation and output formatting. Uppercase the source, validate its digit set and length, and accumulate `raw=raw*16+digit` with checked integer arithmetic. Only a full ten-digit source with its top bit set is negative: subtract `2^40` to obtain signed integer `v`. Check `v` against the 10-bit signed target range. If negative, encode `2^10+v` in base 2 using ten digits. Otherwise use repeated division by 2, reverse the remainders and left-pad to the requested width. Truncate the width and validate it in the current supported range one through ten. Return text; a decimal number formatted to look similar is a different value kind.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,214 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.HEX2BIN.json).

## Alternatives

[BIN2HEX](FUNC.BIN2HEX.md) converts in the reverse direction. [BASE](FUNC.BASE.md) and [DECIMAL](FUNC.DECIMAL.md) provide general radix conversion with different sign rules. A full-width negative engineering encoding is not a minus-prefixed numeral; choose the pair whose sign convention fits your input.

## Sources

- [Microsoft: HEX2BIN function](https://support.microsoft.com/en-us/excel/functions/hex2bin-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/engineering_radix_family.rs); implementation steps and the limits identified in the text.
