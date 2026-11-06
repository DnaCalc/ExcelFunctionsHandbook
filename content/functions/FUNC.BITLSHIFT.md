---
schema: efh.function-page/v1
function_id: FUNC.BITLSHIFT
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

# BITLSHIFT

## What it does

`BITLSHIFT(number, shift_amount)` moves an integer’s binary digits left. A negative count reverses the direction. The ordinary left shift corresponds to multiplication by a power of two.

## Try it

`=BITLSHIFT(13,2)` returns `52`. Binary `1101` becomes `110100` after appending two zeros. A count of `-2` shifts right.

## Quirks

The number must be an integer in `0..2^48-1`; fractional numbers give `#NUM!`. The count is truncated. Documentation rejects count magnitudes above 53. Recorded build-20430 rules add exceptions: zero bypasses the finite count-range check, and admitted counts of magnitude 49 through 53 return zero in either direction.

## How to implement

Validate the number first and require a finite count. Return zero immediately for a zero number. Otherwise truncate the count and reject magnitudes above 53; magnitudes 49 through 53 return zero. Convert the function and count sign to a left/right direction. Before a left shift of `k`, check `number <= (2^48-1)>>k`; reject overflow with `#NUM!`. A right shift discards the low bits. Return the resulting integer as a worksheet number. These shortcuts are recorded compatibility behaviour, and should not be replaced by a host language’s masked shift-count semantics.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit-bit, CPU not recorded | 8,131 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.BITLSHIFT.json).

## Alternatives

[BITRSHIFT](https://handbook.dnacalc.com/functions/BITRSHIFT.html) changes the default direction. [BITAND](FUNC.BITAND.md) extracts a bit field after shifting.

## Sources

- [Microsoft: BITLSHIFT function](https://support.microsoft.com/en-us/excel/functions/bitlshift-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/bit_common.rs); implementation steps and the limits identified in the text.
