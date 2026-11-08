---
schema: efh.function-page/v1
function_id: FUNC.SQRT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — SQRT function"
    locator: "https://support.microsoft.com/en-us/office/sqrt-function-654975c2-05c4-4831-9a24-2c65e4040fdf"
    role: "documented signature, the #NUM! condition for negative arguments, and the ABS workaround"
  - work: "IEEE 754-2019, Standard for Floating-Point Arithmetic"
    locator: "clause 5.4.1 (squareRoot)"
    role: "the requirement that square root be correctly rounded, which is what makes this function special"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 3"
    locator: "3.7 (roots and powers)"
    role: "elementary algebraic identities and the series used in cancellation-avoiding rewrites"
  - work: "Muller et al., Handbook of Floating-Point Arithmetic"
    locator: "chapter on square root and division algorithms"
    role: "Newton-Raphson and Goldschmidt square root, and the last-bit rounding argument"
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
family: sqrt_fn
role_in_family: "The principal real square root, with a measured 64-significand-bit root followed by a binary64 store."
---

# SQRT

## What it does

`SQRT(number)` returns the nonnegative real square root. A positive input has two mathematical real roots; this function chooses the positive one. Zero returns zero. Microsoft documents `#NUM!` for a negative input.

## Try it

`=SQRT(81)` returns `9`; `=SQRT(2)` is about `1.414213562373095`. Squaring the returned binary64 value need not recover the input exactly.

`=SQRT(-9)` gives `#NUM!`. `=SQRT(ABS(-9))` returns three because it changes the input to positive nine. Use IMSQRT when a complex root of the original negative value is intended.

The retained build 20430 controls give `GEOMEAN(1,2)=1.414213562373095` and `SQRT(2)=1.4142135623730951`, adjacent binary64 values. [FN-GEOMEAN-001](../notes/FN-GEOMEAN-001.json) retains the exact cell inputs and both answers. Their mathematical values agree, but the recorded calculation routes do not publish the same bits.

## Quirks

Recorded worksheet SQRT uses a root rounded to a 64-bit significand followed by a binary64 store. That can differ by a final bit from one correctly rounded binary64 square root. In the exact-cell control `0x3fefffffffffffff` (`0.9999999999999999`), Excel returns one; a single-rounding mathematical reference returns the preceding binary64 value.

Writing a positive or negative subnormal (`±5e-324` through `±(2^-1022 - 5e-324)`) or `-0` to a cell via `Range.Value2` flushes the stored cell value to `+0` (`0x0000000000000000`) on Excel 16.0 build 20527, which is why the recorded cell-input rows `SQRT(5e-324)` and `SQRT(-5e-324)` return `0` ([sweep-questions-1-ANSWERS.md](../notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md), `C:\Work\DnaCalc\codex-runs\sweep-questions-1\Q1-subnormal-inputs.json`). When a true subnormal reaches `SQRT` inside a formula (`EXP(-745) = 0x0000000000000001`, `EXP(-730) = 0x00000000001c7ea3`, `EXP(-709) = 0x0008bfe58468888d`), Excel computes the positive normal root (`SQRT(EXP(-745)) = 2^-537 = 2.2227587494850775e-162`, `0x1e60000000000000`) and rejects a negative computed subnormal (`SQRT(0 - EXP(-745)) = #NUM!`) ([FN-ABS-002](../notes/FN-ABS-002.json)).

Compositions need their own operation order. Squaring two large components before SQRT can overflow when a scaled hypotenuse would be finite. A general POWER route or a logarithm/exponential composition also has different intermediate rounding.

## How to implement

For the finite nonnegative scalar route, reject negative inputs before the root and return zero for zero. For positive x:

1. Find the exponent of the root by halving `floor(log2(x))`, rounding downward. Obtain the input exponent from its bits or exact comparisons.
2. Round the exact square root to a 64-bit significand, ties-to-even. The [portable staged-root recipe](../model/09-staged-arithmetic.md#square-root-without-an-extended-host-type) finds this with an integer square root and an exact comparison against a squared midpoint. It needs no host extended floating-point type.
3. Store that rounded root to binary64, again choosing nearest/even. Keep this second step; directly calling a binary64 sqrt removes the recorded first rounding.

Worked boundary: `x=1-2^-53`. The midpoint between its two possible binary64 root results is `m=1-2^-54`. Since `m*m=x+2^-108`, the true root is slightly below m. Rounding it first to 64 significant bits gives m; the binary64 midpoint tie then selects one, whose low significand bit is even. A direct binary64 root instead selects `1-2^-53`.

FN-SQRT-001 reports 41,897 matching distinct normal-positive recorded inputs, including all 164 controls where the staged and single-rounding roots differ. Its reproductions name Excel build 20430 on Windows 64-bit. With cell `Range.Value2` subnormal flushing modelled at the witness boundary (`crates/calc-func/src/witness_exec.rs`, commit `c0b94f88ab4e3df5b518329d08ea03cf481abdbe`) and `SQRT` keeping true positive and negative subnormal handling in the kernel (`crates/calc-func/src/catalogue/math.rs`, commit `e042619e35426ffac3d8cfaaa2dd2961abe3d02c`), DNA Calc matches all 42,803 of 42,803 recorded `SQRT` bank rows (`C:\Work\DnaCalc\codex-runs\sweep-report-2.md`).

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 42 |
| 20430 | Windows, 64-bit, CPU not recorded | 42,803 |

- XC-SQRT-001: build 19929 versus build 20430; 2 same-input pairs, 0 differing answers. Compared axes: excel-build. Numeric answers were compared by their recorded binary64 bits; other answers by typed value.

Agreement on these compared inputs does not establish agreement on unmeasured inputs.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.SQRT.json).

Retained controls from the indexed captures:

| Build and recorded CPU | Captured call or arguments | Excel result |
|---|---|---|
| 19929, CPU not recorded | [`=SQRT(0)`](https://github.com/DnaCalc/CalcFunc-Evidence/blob/4287ba192d2e3a7fd54671ea1cee1c34cb2e84e2/runs/broad-scalar-cycle-011-cellref/comparisons/excel_sample_comparisons.jsonl.gz) | `0.0` (`0x0000000000000000`) |
| 20430, CPU not recorded | [`SQRT(2)`](https://github.com/DnaCalc/CalcFunc-Evidence/blob/4287ba192d2e3a7fd54671ea1cee1c34cb2e84e2/cache/w111-broad-regenerated-20260929/answers-sqrt.json.gz) | `1.4142135623730951` (`0x3ff6a09e667f3bcd`) |

These are earlier recorded answers, not new Excel runs or an additional cross-build comparison. The function JSON retains each source locator, exact argument bits where available, and input-route status. Call notation does not imply that captured binary64 values were entered as decimal formula literals.

## Alternatives

[IMSQRT](FUNC.IMSQRT.md) accepts complex inputs. [SQRTPI](FUNC.SQRTPI.md) includes a multiplication by pi, and [IMABS](FUNC.IMABS.md) uses a hypotenuse route for complex magnitude. [POWER](FUNC.POWER.md) accepts other exponents but must retain its own numerical graph.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `SQRT` that an implementer matching Excel, or a reader explaining an
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

### FN-GEOMEAN-001: GEOMEAN(1,2) and SQRT(2) return adjacent values in retained build 20430 controls

*Excel differs from the mathematical answer; measured; recorded 2026-11-06.* Applies to GEOMEAN, SQRT.

In two retained build 20430 controls, GEOMEAN(1,2) returned 1.414213562373095
(0x3ff6a09e667f3bcc), while SQRT(2) returned 1.4142135623730951 (0x3ff6a09e667f3bcd). Both
functions mathematically return the positive square root of two; the SQRT result is the nearest
binary64 value. A fifteen-significant-digit display hides the difference.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `GEOMEAN(1,2)` | number1 `1` (`0x3ff0000000000000`); number2 `2` (`0x4000000000000000`) (cells) | `1.414213562373095` (`0x3ff6a09e667f3bcc`) | `1.4142135623730951` (`0x3ff6a09e667f3bcd`) | Excel 16.0 build 20430, Windows 64-bit (NT 10.00), Workbook Compatibility Version 2; CPU not recorded. Captured 29 September 2026. |
| `SQRT(2)` | number `2` (`0x4000000000000000`) (cells) | `1.4142135623730951` (`0x3ff6a09e667f3bcd`) | — | Excel 16.0 build 20430, Windows 64-bit (NT 10.00), Workbook Compatibility Version 2; CPU not recorded. Captured 29 September 2026. |

Correct value: Exact rational comparison: 2 lies strictly between the squares of the lower and
upper binary64 midpoints around 0x3ff6a09e667f3bcd. Therefore that value is the nearest
binary64 square root of 2.

Basis (measured): Two rows read directly from the landed regenerated W111 capture files. Both
record Excel build 20430, Windows 64-bit, Compatibility Version 2 and exact-double bulk Value2
inputs, on 29 September 2026. The calls are to different functions in the same recorded
environment class; this is not a comparison between builds and no fresh Excel calls were made.

- Open: Whether other inputs or environments retain the same difference; the two rows do not certify whole-function agreement or identify private Excel code.

Evidence:

- CalcFunc-Evidence `cache/w111-broad-regenerated-20260929/answers-geomean.json.gz`, w111broad-20260929-GEOMEAN-00003 at `4287ba192d` — Recorded GEOMEAN(1,2) answer and capture provenance.
- CalcFunc-Evidence `cache/w111-broad-regenerated-20260929/answers-sqrt.json.gz`, w111broad-20260929-SQRT-00014 at `4287ba192d` — Recorded SQRT(2) answer and capture provenance.
- DnaCalc `crates/calc-func/src/ported/excel_numeric/x87_chains.rs`, excel_root_x87 at `da3ecc4fb1` — Current normalized LN, reciprocal and EXP root reconstruction.
- CalcFunc-Evidence `cache/w111-broad-regenerated-20260929/answers-geomean.json.gz`, w111broad-20260929-GEOMEAN-00003 at `4287ba192d` — Capture provenance records exact-double bulk Value2 cell input. (readings)
- CalcFunc-Evidence `cache/w111-broad-regenerated-20260929/answers-sqrt.json.gz`, w111broad-20260929-SQRT-00014 at `4287ba192d` — Capture provenance records exact-double bulk Value2 cell input. (readings)

### FN-SQRT-001: SQRT is rounded twice, once to 64 bits and once to 53

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.*

The worksheet SQRT is RN53(RN64(sqrt(x))): the root is rounded to the x87 unit's 64-bit
significand and then stored to binary64. On inputs where that double rounding lands differently
from a single rounding, Excel takes the double-rounded value: SQRT(0.9999999999999999) is 1. A
subnormal written to a cell reads back as 0, so the 0 for the smallest subnormal below may
record that and not SQRT.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `SQRT(0.9999999999999999)` | number `0.9999999999999999` (`0x3fefffffffffffff`) (cells) | `1` (`0x3ff0000000000000`) | `0.9999999999999999` (`0x3fefffffffffffff`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `SQRT(1.7976931348623157e+308)` | number `1.7976931348623157e+308` (`0x7fefffffffffffff`) (cells) | `1.3407807929942597e+154` (`0x5ff0000000000000`) | `1.3407807929942596e+154` (`0x5fefffffffffffff`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `SQRT(5e-324)` | number `5e-324` (`0x0000000000000001`) (cells) | `0` (`0x0000000000000000`) | — | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |

Correct value: Exact square root at 800 bits, correctly rounded.

Basis (measured): 41,897 of 41,897 distinct normal positive recorded rows match. On the 164
rows where double and correct rounding differ, Excel follows double rounding every time. The
subnormal row is 1 row, and no read-back of its input is recorded.

Evidence:

- DnaCalc `crates/calc-func/src/ported/sqrt_fn.rs`, excel_sqrt_staged at `b5ce0759bc` — the kernel in the product
- CalcFunc-Evidence `runs/w111-broad-20260929/answers/answers-sqrt.json.gz`, w111broad-20260929-SQRT-00016 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `runs/w111-broad-20260929/answers/answers-sqrt.json.gz`, w111broad-20260929-SQRT-00025 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `runs/w111-broad-20260929/answers/answers-sqrt.json.gz`, w111broad-20260929-SQRT-00002 at `4d5320dfdc` (readings)

### FN-VALUE-001: Text to number reads Arabic, Persian, Devanagari and Thai digits, but not fullwidth digits

*Quirk; measured; recorded 2026-11-04.* Applies to DATEVALUE, SQRT, VALUE.

In the recorded en-ZA settings, converting text to a number accepts Arabic-Indic (U+0660 to
U+0669), extended Arabic-Indic (U+06F0 to U+06F9), Devanagari (U+0966 to U+096F) and Thai
(U+0E50 to U+0E59) digits. They work alone, in runs, with a decimal point, in the exponent,
with a sign or percent, and mixed with ASCII digits or with each other: VALUE("1٢3") is 123,
and SQRT of a four-digit string using four scripts is the root of 1234. Fullwidth digits
(U+FF10 to U+FF19) give #VALUE! alone or mixed. DATEVALUE reads the four accepted scripts in a
date written year-month-day.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=VALUE("١")` | as typed | `1` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("١٢٣")` | as typed | `123` (`0x405ec00000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("1٢3")` | as typed | `123` (`0x405ec00000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("١e2")` | as typed | `100` (`0x4059000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("१")` | as typed | `1` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("๑")` | as typed | `1` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("１")` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("1２3")` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=SQRT("۱٢३๔")` | as typed | `35.12833614050059` (`0x4041906d51932c26`) | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `=DATEVALUE("٢٠٢٤-٠١-٠٢")` | as typed | `45293` (`0x40e61da000000000`) | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |

Basis (measured): 66 recorded VALUE readings (all ten digits of each of the five scripts, then
composed forms), 10 SQRT and 4 DATEVALUE readings.

Inferred, not measured: That other text-to-number routes, such as arithmetic operators, treat
digits the same way is inferred from VALUE and SQRT agreeing.

- Open: Other digit blocks (Bengali, Tibetan, Khmer and so on) have not been read.
- Open: Whether the set depends on the Windows or workbook locale has not been read.

Evidence:

- DnaCalc `apps/dna-onecalc/tests/acceptance.rs`, delivered_fn4_digit_scripts_and_numeric_date_widths at `55697ab4f7` — recorded answers pinned
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheets qt031 to qt096 at `2d0eddcac8` — 66 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/SQRT.reading.json` at `2d0eddcac8` — 10 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/DATEVALUE.reading.json` at `2d0eddcac8` — 6 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt042, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt081, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt084, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt088, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt062, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt072, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt032, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt093, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/SQRT.reading.json`, fn7-SQRT-digits-6 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/DATEVALUE.reading.json`, fn7-DATEVALUE-digits-1 at `2d0eddcac8` (readings)

<!-- efh:notes end -->

## Excel versions, platforms and CPUs

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.SQRT.json; edits here are overwritten -->

Readings of SQRT with a recorded Excel build: 42,845 input(s) over 2 recorded environment(s),
Excel 16.0 build(s) 19929, 20430, all on Windows; none of these captures records the CPU. 1
same-input comparison(s) between environments over 2 input-pair(s): no result differs. No
reading from a Mac, Excel for the web, 32-bit Excel or an Intel CPU is recorded.

Readings of `SQRT` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 42 | date not recorded |
| 16.0 build 20430 | Windows, 64-bit | CPU not recorded | 42,803 | 2026-09-29 |

Compared on the same inputs:

| Check | Differs in | Environments | Inputs compared | Inputs with different results |
|---|---|---|---:|---:|
| XC-SQRT-001 | excel-build | 16.0 build 19929 vs 16.0 build 20430 | 2 | 0 |

Differences observed: none.

Evidence for every row: [`content/evidence/environments/FUNC.SQRT.json`](../../content/evidence/environments/FUNC.SQRT.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Sources

- [Microsoft: SQRT function](https://support.microsoft.com/en-us/excel/functions/sqrt-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/excel_numeric/mod.rs); implementation steps and the limits identified in the text.
- [FN-SQRT-001](../notes/FN-SQRT-001.json): exact inputs, recorded results and the subnormal transfer limit.
- [Software extended root and binary64 store](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/excel_numeric/soft87.rs).
- [FN-GEOMEAN-001](../notes/FN-GEOMEAN-001.json): two retained build 20430 controls, with a rational check of the nearest square root.
- [sweep-questions-1-ANSWERS.md](../notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md) (`C:\Work\DnaCalc\codex-runs\sweep-questions-1\ANSWERS.md`, `Q1-subnormal-inputs.json`) and `C:\Work\DnaCalc\codex-runs\sweep-report-2.md`: build 20527 subnormal cell-write flushing versus true computed subnormal `SQRT(EXP(-745))` / `SQRT(0-EXP(-745))`, and DNA Calc bank comparison (`c0b94f88ab`, `e042619e35`).
