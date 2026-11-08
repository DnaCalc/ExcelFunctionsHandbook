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
  - Notes
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

The shared current model rounds the quotient to fifteen significant decimal digits before choosing a count. On Excel 16.0 build 20527, writing a subnormal binary64 value (`±5e-324` through `±(2^-1022 - 5e-324)`) to a cell via `Range.Value2` flushes the cell to `+0`, so a cell-written subnormal `number` or `significance` returns `0` ([sweep-questions-1-ANSWERS.md](../notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md), `C:\Work\DnaCalc\codex-runs\sweep-questions-1\Q1-subnormal-inputs.json`). On a true computed subnormal `x` in a formula (`EXP(-745)`, `EXP(-730)`, `EXP(-709)`), `CEILING.MATH(x, 1)` returns `1` and `CEILING.MATH(0 - x, 1)` returns `0`, while a subnormal quotient `number / s` flushes to `0` ([FN-ABS-002](../notes/FN-ABS-002.json)).

## How to implement

1. Apply omitted defaults: significance one and mode zero. Coerce numeric arguments, retain the distinction between an omitted tail and an explicitly empty slot, and set `s=abs(significance)`. Return zero if s is zero.
2. For nonnegative input, obtain the current [fifteen-digit quotient reading](FUNC.CEILING.md#how-to-implement) of `number/s`, then take its ceiling.
3. For negative input, read the positive quotient `abs(number)/s`. Take its floor when mode is zero, or its ceiling when mode is nonzero; restore the negative sign.
4. The ported modern helper multiplies that count by s with ordinary binary64 arithmetic and normalizes zero. This is distinct from assuming every family spelling uses the native legacy wrapper’s final stored-x87 site. Apply the surrounding number publication rule.

For `-10.3` and step three, the positive quotient reads near `3.43333333333333`. Default mode chooses three and returns `-9`; nonzero mode chooses four and returns `-12`. The current fractional mode `0.5` follows the nonzero branch too.

The quotient helper and branch recipe describe the current implementation. [FN-FLOOR-002](../notes/FN-FLOOR-002.json) explicitly leaves transfer of the isolated legacy quotient rule to modern spellings open. With cell `Range.Value2` subnormal flushing modelled at the witness boundary (`crates/calc-func/src/witness_exec.rs`, commit `c0b94f88ab4e3df5b518329d08ea03cf481abdbe`) and true subnormal `number` inputs accepted (`crates/calc-func/src/catalogue/math.rs`, commit `e042619e35426ffac3d8cfaaa2dd2961abe3d02c`), DNA Calc matches all 1,362 of 1,362 recorded `CEILING.MATH` bank rows (`C:\Work\DnaCalc\codex-runs\sweep-report-2.md`).

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

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `CEILING.MATH` that an implementer matching Excel, or a reader explaining
an unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's
answer and where it was recorded; "measured" means the statement is read off recorded Excel
answers, "inferred" marks what goes beyond them. Where older text on this page says a behaviour
was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

### FN-ABS-002: Subnormal cell writes and *, /, ^ underflows flush to +0, while +, - and EXP(-745) keep subnormals

*Quirk; measured; recorded 2026-11-08.* Applies to ABS, CEILING, CEILING.MATH, CEILING.PRECISE, DELTA, EVEN, EXP, FLOOR, FLOOR.MATH, FLOOR.PRECISE, GESTEP, INT, ISEVEN, ISO.CEILING, ISODD, ODD, QUOTIENT, SIGN, SQRT.

Writing any positive or negative subnormal binary64 value or -0 to a cell via Range.Value2
stores +0 (0x0000000000000000), while normal extremes 2^-1022 (0x0010000000000000) and ±DBL_MAX
(0x7fefffffffffffff) read back intact. In the 430 recorded bank rows with subnormal or
maximum-finite cell arguments, all 179 subnormal-input rows read back as +0 and match the +0
control (SQRT(-5e-324) is 0, SIGN(±5e-324) is 0, ODD(±5e-324) is 1, EVEN(±5e-324) is 0,
ISEVEN(±5e-324) is TRUE, ISODD(±5e-324) is FALSE, GESTEP(±5e-324) is 1, DELTA(±5e-324) is 1,
and a subnormal divisor or significance in QUOTIENT, CEILING or FLOOR flushes to 0). Formula
literals below 2.2250738585072014E-308 are refused at entry or parsed to 0. Products (*),
quotients (/) and probed powers (^) that fall in the subnormal range flush to +0, whereas + and
- retain subnormals and EXP(-745) holds 0x0000000000000001 in a formula cell. When fed a true
computed subnormal x from EXP(-745), EXP(-730) or EXP(-709) (and 0-x for negative), ABS(±x) is
x, SQRT(+x) is 2^-537, SQRT(-x) is #NUM!, EVEN(±x) is ±2, ODD(±x) is ±1, ISEVEN(±x) is TRUE,
ISODD(±x) is FALSE, CEILING(+x,1) is 1, FLOOR(-x,1) is -1, QUOTIENT(±x,1) is 0, DELTA(x,0) is
0, DELTA(x,x) is 1, GESTEP(+x,0) is 1 and GESTEP(-x,0) is 0, while SIGN(+x) and INT(-x)
transition inside the subnormal range (0 at 0x00000000001c7ea3 and ±1 at 0x0008bfe58468888d).

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `SQRT(-5e-324)` | number `-5e-324` (`0x8000000000000001`) (cells) | `0` (`0x0000000000000000`) | Excel 16.0.20527.20150, Windows 64-bit, AMD Ryzen 7 4800H, en-ZA |
| `=EXP(-745)` | as typed | `5e-324` (`0x0000000000000001`) | Excel 16.0.20527.20150, Windows 64-bit, AMD Ryzen 7 4800H, en-ZA |
| `=SQRT(EXP(-745))` | as typed | `2.2227587494850775e-162` (`0x1e60000000000000`) | Excel 16.0.20527.20150, Windows 64-bit, AMD Ryzen 7 4800H, en-ZA |
| `=SQRT(0-EXP(-745))` | as typed | `#NUM!` | Excel 16.0.20527.20150, Windows 64-bit, AMD Ryzen 7 4800H, en-ZA |
| `=EVEN(EXP(-745))` | as typed | `2` (`0x4000000000000000`) | Excel 16.0.20527.20150, Windows 64-bit, AMD Ryzen 7 4800H, en-ZA |
| `=SIGN(EXP(-730))` | as typed | `0` (`0x0000000000000000`) | Excel 16.0.20527.20150, Windows 64-bit, AMD Ryzen 7 4800H, en-ZA |
| `=SIGN(EXP(-709))` | as typed | `1` (`0x3ff0000000000000`) | Excel 16.0.20527.20150, Windows 64-bit, AMD Ryzen 7 4800H, en-ZA |
| `=INT(0-EXP(-730))` | as typed | `0` (`0x0000000000000000`) | Excel 16.0.20527.20150, Windows 64-bit, AMD Ryzen 7 4800H, en-ZA |
| `=INT(0-EXP(-709))` | as typed | `-1` (`0xbff0000000000000`) | Excel 16.0.20527.20150, Windows 64-bit, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): 430 bank rows replayed with Value2 input read-back and +0 controls, 22
subnormal bit-pattern cell-write probes, 16 literal-acceptance probes, 45 arithmetic underflow
probes, and 180 true-subnormal function calls in Excel 16.0 build 20527 on 2026-11-08.

- Open: The exact subnormal threshold between 0x00000000001c7ea3 (EXP(-730)) and 0x0008bfe58468888d (EXP(-709)) where SIGN(+x) changes from 0 to 1 and INT(-x) changes from 0 to -1 is not yet pinned.

Evidence:

- [`content/notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md`](../../content/notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md), Q1: Subnormal and maximum-finite inputs — summary of the build 20527 subnormal and max-finite measurements
- DnaCalc `crates/calc-func/src/witness_exec.rs`, flush_cell_value2_number at `c0b94f88ab` — cell Value2 input flushing in witness replay
- DnaCalc `crates/calc-func/src/catalogue/math.rs` at `e042619e35` — true-subnormal handling across math, rounding and comparison functions
- [`content/notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md`](../../content/notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md), Q1: Subnormal and maximum-finite inputs (readings)

<!-- efh:notes end -->

## Sources

- [Microsoft: CEILING.MATH function](https://support.microsoft.com/en-us/excel/functions/ceiling-math-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/rounding.rs); implementation steps and the limits identified in the text.
- [Quotient reading and sign helpers](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/ceiling_floor_family.rs).
- [Modern wrapper defaults](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/rounding_more.rs).
- [sweep-questions-1-ANSWERS.md](../notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md) (`C:\Work\DnaCalc\codex-runs\sweep-questions-1\ANSWERS.md`, `Q1-subnormal-inputs.json`) and `C:\Work\DnaCalc\codex-runs\sweep-report-2.md`: build 20527 subnormal cell-write flushing, true computed subnormal `CEILING.MATH` probes, and DNA Calc bank comparison (`c0b94f88ab`, `e042619e35`).
