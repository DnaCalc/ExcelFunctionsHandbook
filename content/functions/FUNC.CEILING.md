---
schema: efh.function-page/v1
function_id: FUNC.CEILING
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

# CEILING

## What it does

`CEILING(number, significance)` chooses a multiple at or above the input under its sign and mode rules. `significance` is a step size, such as two or 0.05, rather than a count of decimal places. Both arguments are required.

## Try it

`=CEILING(-8.1,2)` returns `-8`, while `=CEILING(-8.1,-2)` returns `-10`. Changing the significance sign changes the negative-input direction.

## Quirks

Legacy CEILING rejects a positive number with negative significance using `#NUM!`; a negative number can use either significance sign. The current route returns zero for zero significance. Quotients close to an integer first pass through a fifteen-significant-digit reading, so a bare language ceil/floor of the unadjusted quotient can disagree.

## How to implement

Validate and coerce numeric arguments. Retain the significance sign and enforce the legacy sign restriction. The current helper forms a binary64 quotient and rounds its decimal representation to fifteen significant digits before choosing an integer multiple. Take the ceiling of the adjusted `number/significance`, then multiply by significance under the arithmetic-site policy. In the negative 8.1 example with step two, the magnitude quotient is 4.05; choosing four versus five gives minus eight versus minus ten. Retain decimal-reading and final-product rounding as separate steps. Extreme quotient and nonfinite publication policies still need the host’s explicit error/refusal handling.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,335 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.CEILING.json).

## Alternatives

[MROUND](FUNC.MROUND.md) chooses a nearest multiple; [ROUNDUP](FUNC.ROUNDUP.md) and [ROUNDDOWN](FUNC.ROUNDDOWN.md) address decimal positions. [FLOOR.MATH](FUNC.FLOOR.MATH.md) provides the other directional family.

## Sources

- [Microsoft: CEILING function](https://support.microsoft.com/en-us/excel/functions/ceiling-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/rounding.rs); implementation steps and the limits identified in the text.
- [Quotient reading and sign helpers](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/ceiling_floor_family.rs).
