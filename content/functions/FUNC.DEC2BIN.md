---
schema: efh.function-page/v1
function_id: FUNC.DEC2BIN
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

# DEC2BIN

## What it does

`DEC2BIN(number, [places])` converts decimal to binary. The result is text, with optional leading zeros controlled by `places`.

## Try it

`=DEC2BIN(45,8)` returns `"00101101"`. All of these represent the same integer: decimal 45, binary `101101`, octal `55`, and hexadecimal `2D`.

## Quirks

These engineering conversions use signed fixed-width encodings, not arbitrary-length integers. The destination’s numeric range is -512 through 511. The decimal input is truncated toward zero. For a positive result a width too small gives `#NUM!`. A negative result uses ten digits; the current code still validates an explicitly supplied width before emitting it, despite documentation describing the width as ignored.

## How to implement

Separate input parsing, signed interpretation and output formatting. Coerce the decimal number and truncate it toward zero to obtain integer `v`. Check `v` against the 10-bit signed target range. If negative, encode `2^10+v` in base 2 using ten digits. Otherwise use repeated division by 2, reverse the remainders and left-pad to the requested width. Truncate the width and validate it in the current supported range one through ten. Return text; a decimal number formatted to look similar is a different value kind.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit-bit, CPU not recorded | 4,807 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.DEC2BIN.json).

## Alternatives

[BASE](FUNC.BASE.md) and [DECIMAL](FUNC.DECIMAL.md) provide general radix conversion with different sign conventions. [BIN2DEC](FUNC.BIN2DEC.md) is the reverse direction.

## Sources

- [Microsoft: DEC2BIN function](https://support.microsoft.com/en-us/excel/functions/dec2bin-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/engineering_radix_family.rs); implementation steps and the limits identified in the text.
