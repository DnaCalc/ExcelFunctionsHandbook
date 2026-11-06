---
schema: efh.function-page/v1
function_id: FUNC.CEILING.PRECISE
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

# CEILING.PRECISE

## What it does

`CEILING.PRECISE(number,[significance])` rounds to a multiple of the absolute step, toward positive infinity in the mathematical convention. The default step is one. The significance sign is ignored and this spelling has no mode argument. Its current quotient-reading approximation is described below.

## Try it

`=CEILING.PRECISE(-10.3,-3)` returns `-9`. Changing the significance to positive three gives the same value: the step magnitude controls the grid, while the function chooses the direction.

`=CEILING.PRECISE(31.2,3)` returns `33`. These ordinary examples separate the direction from the significance sign.

## Quirks

The absolute significance is used, including for negative numbers. A zero number or zero significance returns zero. The current wrapper uses a default of one when significance is omitted at the end of the call; an explicitly empty significance slot is read as zero.

The PRECISE spelling does not establish a correctly rounded mathematical result on every input. The current helper applies a fifteen-digit reading to the quotient before choosing the integer count. The legacy FLOOR measurements do not establish all transferred modern-family boundaries; preserve that limitation.

## How to implement

1. Read the number and apply the omitted significance default of one. Set `s=abs(significance)` and return positive zero for s equal to zero.
2. Compute the current [fifteen-significant-digit quotient reading](FUNC.CEILING.md#how-to-implement) using ordinary binary64 division, nearest/even decimal quantization and conversion back to binary64.
3. Take the ceiling of the signed quotient `number/s`. The current helper expresses the negative branch using a positive magnitude quotient and the opposite integer direction, then restores the sign.
4. Multiply the selected count by s with the ported helper’s ordinary binary64 multiplication, normalize zero and apply worksheet number publication.

For `number=-10.3,s=3`, the magnitude quotient is near `3.43333333333333`. Choosing three steps gives `-9`. Do not infer the significance sign from the input’s sign; this spelling discards it.

This is a complete branch outline for the current scalar helper, with the quotient-reading primitive linked above. The exact modern-spelling edge transfer remains open in [FN-FLOOR-002](../notes/FN-FLOOR-002.json); a shared helper does not prove that Excel privately uses the same program for every spelling.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,335 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.CEILING.PRECISE.json).

## Alternatives

[MROUND](FUNC.MROUND.md) chooses a nearest multiple; [ROUNDUP](FUNC.ROUNDUP.md) and [ROUNDDOWN](FUNC.ROUNDDOWN.md) address decimal positions. [FLOOR.MATH](FUNC.FLOOR.MATH.md) provides the other directional family.

## Sources

- [Microsoft: CEILING.PRECISE function](https://support.microsoft.com/en-us/excel/functions/ceiling-precise-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/rounding.rs); implementation steps and the limits identified in the text.
- [Quotient reading and sign helpers](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/ceiling_floor_family.rs).
- [Modern wrapper defaults](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/rounding_more.rs).
