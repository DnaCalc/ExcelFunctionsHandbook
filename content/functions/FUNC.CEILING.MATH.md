---
schema: efh.function-page/v1
function_id: FUNC.CEILING.MATH
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

# CEILING.MATH

## What it does

`CEILING.MATH(number,[significance],[mode])` rounds to a multiple of the step’s absolute value, which defaults to one. By default, negative numbers move toward zero and positive numbers away from zero. A nonzero mode reverses the negative-input direction. Significance supplies a multiple, rather than a number of decimal places.

## Try it

`=CEILING.MATH(-10.3,3)` returns `-9`; `=CEILING.MATH(-10.3,3,1)` returns `-12`. The nonzero mode changes negative inputs to rounding away from zero. Both answers are multiples of three.

## Quirks

The significance sign is ignored. Mode affects only negative numbers, and the current helper tests zero versus any nonzero mode without truncating the flag. Its zero-significance route returns zero.

The opened Microsoft page’s argument summary calls significance a count of digits, while its remarks and worked examples use a multiple. Use the multiple interpretation shown here. The current wrapper defaults significance to one when the call ends before it; an explicitly empty slot is read as zero instead.

The shared current model rounds the quotient to fifteen significant decimal digits before choosing a count. Recorded scope for the legacy FLOOR rule does not by itself establish every modern variant’s quotient edge or last bit.

## How to implement

1. Apply omitted defaults: significance one and mode zero. Coerce numeric arguments, retain the distinction between an omitted tail and an explicitly empty slot, and set `s=abs(significance)`. Return zero if s is zero.
2. For nonnegative input, obtain the current [fifteen-digit quotient reading](FUNC.CEILING.md#how-to-implement) of `number/s`, then take its ceiling.
3. For negative input, read the positive quotient `abs(number)/s`. Take its floor when mode is zero, or its ceiling when mode is nonzero; restore the negative sign.
4. The ported modern helper multiplies that count by s with ordinary binary64 arithmetic and normalizes zero. This is distinct from assuming every family spelling uses the native legacy wrapper’s final stored-x87 site. Apply the surrounding number publication rule.

For `-10.3` and step three, the positive quotient reads near `3.43333333333333`. Default mode chooses three and returns `-9`; nonzero mode chooses four and returns `-12`. The current fractional mode `0.5` follows the nonzero branch too.

The quotient helper and branch recipe describe the current implementation. [FN-FLOOR-002](../notes/FN-FLOOR-002.json) explicitly leaves transfer of the isolated legacy quotient rule to modern spellings open. Exact edge behaviour and a whole-domain modern-family identification are not established by the shared code alone.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,362 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.CEILING.MATH.json).

## Alternatives

For the ordinary input `-10.3` on a grid of magnitude three:

| Function/call | Result |
|---|---:|
| `CEILING(-10.3,3)` | -9 |
| `CEILING(-10.3,-3)` | -12 |
| `CEILING.PRECISE(-10.3,-3)` | -9 |
| `CEILING.MATH(-10.3,3,1)` | -12 |
| `FLOOR.PRECISE(-10.3,-3)` | -12 |

[CEILING.PRECISE](FUNC.CEILING.PRECISE.md) and [ISO.CEILING](FUNC.ISO.CEILING.md) have the fixed upward convention. [FLOOR.MATH](FUNC.FLOOR.MATH.md) selects the other directional default. [MROUND](FUNC.MROUND.md) chooses a nearest multiple.

## Sources

- [Microsoft: CEILING.MATH function](https://support.microsoft.com/en-us/excel/functions/ceiling-math-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/rounding.rs); implementation steps and the limits identified in the text.
- [Quotient reading and sign helpers](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/ceiling_floor_family.rs).
- [Modern wrapper defaults](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/rounding_more.rs).
