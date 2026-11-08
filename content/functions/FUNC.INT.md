---
schema: efh.function-page/v1
function_id: FUNC.INT
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
  - Excel versions, platforms and CPUs
  - Sources
family: int_fn
role_in_family: >-
  Sole member of its module; the floor function, and the one rounding primitive in the math
  category that rounds toward negative infinity rather than toward zero.
---

# INT

## What it does

`INT(number)` rounds down to an integer, toward negative infinity. This makes negative fractions more negative: it is a different direction from TRUNC. The recorded compatibility route also has a preliminary decimal reading near integer boundaries.

## Try it

`=INT(-6.8)` returns `-7`, while `=TRUNC(-6.8)` returns `-6`. `=INT(6.8)` returns `6`. For an ordinary positive amount, subtracting its INT result extracts the fractional part.

A boundary control from the source is different from mathematical floor of the incoming binary value: an exact cell value `2.9999999999999982` gives `3` on the recorded build. Entering a decimal formula literal can change the value before INT sees it; retain the exact cell input when reproducing that control.

## Quirks

The fifteen-digit reading can cross an integer. Calling a language's floor on the incoming double therefore misses recorded answers. The current implementation applies this reading only below magnitude `1e15`; larger values use binary64 floor directly. For example, the recorded `4503599627370495.5` floors to `4503599627370495`, while its fifteen-digit decimal reading would have crossed upward.

INT has no digit-count argument. The direction for ordinary negative fractions is toward negative infinity; ROUNDDOWN and TRUNC use toward zero. Numeric conversion, errors and array handling belong to the worksheet wrapper, separately from this finite scalar recipe.

When a subnormal binary64 value (`±5e-324` through `±(2^-1022 - 5e-324)`) is written to a cell via `Range.Value2`, Excel 16.0 build 20527 stores `+0` (`0x0000000000000000`), so `INT` on a cell-written negative subnormal returns `0` ([sweep-questions-1-ANSWERS.md](../notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md), `C:\Work\DnaCalc\codex-runs\sweep-questions-1\Q1-subnormal-inputs.json`). When a true computed subnormal reaches `INT` in a formula (`x` from `EXP(-745) = 0x0000000000000001`, `EXP(-730) = 0x00000000001c7ea3`, or `EXP(-709) = 0x0008bfe58468888d`), `INT(+x)` is `0` at all three magnitudes, whereas `INT(0 - x)` transitions inside the subnormal range: `INT(0 - EXP(-745)) = 0` and `INT(0 - EXP(-730)) = 0`, while `INT(0 - EXP(-709)) = -1` ([FN-ABS-002](../notes/FN-ABS-002.json)). Normal extremes `±DBL_MAX` (`0x7fefffffffffffff` / `0xffefffffffffffff`) return `±DBL_MAX`.

## How to implement

The current code separates decimal reading from binary floor:

1. If `abs(x)>=1e15`, floor the exact binary64 value. A value already integral is unchanged; for a negative value with a fractional part, the result is one less than its toward-zero truncation.
2. Below that threshold, obtain the shortest decimal that converts back to the input binary64. If it has at most fifteen significant digits, use that exact decimal; otherwise decode the exact binary64 as a rational or exact decimal.
3. Round that reading to fifteen significant decimal digits, choosing the larger magnitude at an exact decimal half. When this did not change the reading, the current code falls back to floor of the incoming binary64.
4. When the reading changed, truncate its decimal fraction toward zero and convert the whole value to binary64. Also convert the full reading. If the full reading is below the truncated whole value, subtract one; otherwise retain the whole value.

For `2.9999999999999982`, the shortened fifteen-digit reading is `3`, so the answer is three. For `-6.8`, the integral truncation is minus six and the number lies below it, so floor gives minus seven. Do not reuse ROUND's preliminary x87 scale operation: INT's current decimal helper is a separate route.

The calculation source reports 109 COM controls on build 20527, including both sides of small integers and the large-value exception. Those controls support this implementation on their stated inputs; they do not identify every conversion or workbook route. For subnormal inputs (`crates/calc-func/src/catalogue/math.rs`, commit `e042619e35426ffac3d8cfaaa2dd2961abe3d02c`), DNA Calc returns `0` for positive subnormals and declines negative computed subnormals with a typed not-modelled code while the interior threshold between `0x00000000001c7ea3` and `0x0008bfe58468888d` remains open, matching 124,401 of 124,403 recorded `INT` bank rows (`C:\Work\DnaCalc\codex-runs\sweep-report-2.md`). The [stored-arithmetic chapter](../model/09-staged-arithmetic.md) explains exact input decoding when a host decimal representation is unsuitable.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 40 |
| 20430 | Windows, 64-bit, CPU not recorded | 124,403 |

- XC-INT-001: build 19929 versus build 20430; 2 same-input pairs, 0 differing answers. Compared axes: excel-build. Numeric answers were compared by their recorded binary64 bits; other answers by typed value.

Agreement on these compared inputs does not establish agreement on unmeasured inputs.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.INT.json).


The current source also retains a later build-20527 COM group of 109 INT readings from 30 October 2026. It records `INT(2.9999999999999982)=3` and the large-input exception `INT(4503599627370495.5)=4503599627370495`. These are additional controls, not a matched-input comparison with the earlier environments.

Retained controls from the indexed captures:

| Build and recorded CPU | Captured call or arguments | Excel result |
|---|---|---|
| 19929, CPU not recorded | [`=INT(0)`](https://github.com/DnaCalc/CalcFunc-Evidence/blob/4287ba192d2e3a7fd54671ea1cee1c34cb2e84e2/runs/W097-R-B-validation-cellref-module/comparisons/excel_sample_comparisons.jsonl.gz) | `0.0` (`0x0000000000000000`) |
| 20430, CPU not recorded | [`INT(1)`](https://github.com/DnaCalc/CalcFunc-Evidence/blob/4287ba192d2e3a7fd54671ea1cee1c34cb2e84e2/cache/w111-broad-regenerated-20260929/answers-int.json.gz) | `1` (`0x3ff0000000000000`) |

These are earlier recorded answers, not new Excel runs or an additional cross-build comparison. The function JSON retains each source locator, exact argument bits where available, and input-route status. Call notation does not imply that captured binary64 values were entered as decimal formula literals.

## Alternatives

[TRUNC](FUNC.TRUNC.md) removes a fraction toward zero. [FLOOR.MATH](FUNC.FLOOR.MATH.md) chooses a multiple with explicit negative-number mode, and [ROUND](FUNC.ROUND.md) chooses a nearest decimal position. [MOD](FUNC.MOD.md) has a documented floor identity but a separately recovered numerical route.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `INT` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

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

## Excel versions, platforms and CPUs

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.INT.json; edits here are overwritten -->

Readings of INT with a recorded Excel build: 124,443 input(s) over 2 recorded environment(s),
Excel 16.0 build(s) 19929, 20430, all on Windows; none of these captures records the CPU. 1
same-input comparison(s) between environments over 2 input-pair(s): no result differs. No
reading from a Mac, Excel for the web, 32-bit Excel or an Intel CPU is recorded.

Readings of `INT` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 40 | date not recorded |
| 16.0 build 20430 | Windows, 64-bit | CPU not recorded | 124,403 | 2026-09-29 |

Compared on the same inputs:

| Check | Differs in | Environments | Inputs compared | Inputs with different results |
|---|---|---|---:|---:|
| XC-INT-001 | excel-build | 16.0 build 19929 vs 16.0 build 20430 | 2 | 0 |

Differences observed: none.

Evidence for every row: [`content/evidence/environments/FUNC.INT.json`](../../content/evidence/environments/FUNC.INT.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Sources

- [Microsoft: INT function](https://support.microsoft.com/en-us/excel/functions/int-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/rounding.rs); implementation steps and the limits identified in the text.
- [Exact decimal reading and integer parts](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/decimal.rs).
- [Recorded acceptance control](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/tests/acceptance.rs), INT at cell A5 in the rounding tests.
- [sweep-questions-1-ANSWERS.md](../notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md) (`C:\Work\DnaCalc\codex-runs\sweep-questions-1\ANSWERS.md`, `Q1-subnormal-inputs.json`) and `C:\Work\DnaCalc\codex-runs\sweep-report-2.md`: build 20527 subnormal cell-write flushing, true computed subnormal `INT(0-EXP(-730))` / `INT(0-EXP(-709))` transition probes, and DNA Calc bank comparison (`c0b94f88ab`, `e042619e35`).
