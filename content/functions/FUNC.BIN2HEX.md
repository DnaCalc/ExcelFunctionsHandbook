---
schema: efh.function-page/v1
function_id: FUNC.BIN2HEX
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

# BIN2HEX

## What it does

`BIN2HEX(number, [places])` converts binary to hexadecimal. The result is text, with optional leading zeros controlled by `places`.

## Try it

`=BIN2HEX("101101",4)` returns `"002D"`. All of these represent the same integer: decimal 45, binary `101101`, octal `55`, and hexadecimal `2D`.

## Quirks

These engineering conversions use signed fixed-width encodings, not arbitrary-length integers. The source accepts at most ten digits; a full ten-digit encoding uses 10-bit two’s complement. `1111111111` represents minus one. Invalid source digits or an oversized input give `#NUM!`. For a positive result a width too small gives `#NUM!`. A negative result uses ten digits; the current code still validates an explicitly supplied width before emitting it, despite documentation describing the width as ignored.

## How to implement

Separate input parsing, signed interpretation and output formatting. Uppercase the source, validate its digit set and length, and accumulate `raw=raw*2+digit` with checked integer arithmetic. Only a full ten-digit source with its top bit set is negative: subtract `2^10` to obtain signed integer `v`. Check `v` against the 40-bit signed target range. If negative, encode `2^40+v` in base 16 using ten digits. Otherwise use repeated division by 16, reverse the remainders and left-pad to the requested width. Truncate the width and validate it in the current supported range one through ten. Return text; a decimal number formatted to look similar is a different value kind.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 748 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.BIN2HEX.json).

## Alternatives

[BASE](FUNC.BASE.md) and [DECIMAL](FUNC.DECIMAL.md) provide general radix conversion with different sign conventions. [DEC2BIN](https://handbook.dnacalc.com/functions/DEC2BIN.html) is the reverse direction.

## Sources

- [Microsoft: BIN2HEX function](https://support.microsoft.com/en-us/excel/functions/bin2hex-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/engineering_radix_family.rs); implementation steps and the limits identified in the text.
