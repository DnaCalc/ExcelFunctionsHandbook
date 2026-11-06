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

## Quirks

Recorded worksheet SQRT uses a root rounded to a 64-bit significand followed by a binary64 store. That can differ by a final bit from one correctly rounded binary64 square root. In the exact-cell control `0x3fefffffffffffff` (`0.9999999999999999`), Excel returns one; a single-rounding mathematical reference returns the preceding binary64 value.

The smallest-subnormal reproduction returned zero, but its cell input was not read back. That observation does not establish a square-root underflow rule: input transfer may already have changed it to zero. A positive finite root itself fits the binary64 normal range even for the smallest positive binary64 input.

Compositions need their own operation order. Squaring two large components before SQRT can overflow when a scaled hypotenuse would be finite. A general POWER route or a logarithm/exponential composition also has different intermediate rounding.

## How to implement

For the finite nonnegative scalar route, reject negative inputs before the root and return zero for zero. For positive x:

1. Find the exponent of the root by halving `floor(log2(x))`, rounding downward. Obtain the input exponent from its bits or exact comparisons.
2. Round the exact square root to a 64-bit significand, ties-to-even. The [portable staged-root recipe](../model/09-staged-arithmetic.md#square-root-without-an-extended-host-type) finds this with an integer square root and an exact comparison against a squared midpoint. It needs no host extended floating-point type.
3. Store that rounded root to binary64, again choosing nearest/even. Keep this second step; directly calling a binary64 sqrt removes the recorded first rounding.

Worked boundary: `x=1-2^-53`. The midpoint between its two possible binary64 root results is `m=1-2^-54`. Since `m*m=x+2^-108`, the true root is slightly below m. Rounding it first to 64 significant bits gives m; the binary64 midpoint tie then selects one, whose low significand bit is even. A direct binary64 root instead selects `1-2^-53`.

FN-SQRT-001 reports 41,897 matching distinct normal-positive recorded inputs, including all 164 controls where the staged and single-rounding roots differ. Its reproductions name Excel build 20430 on Windows 64-bit. These results support the stated staged route on that evidence, with no published Handbook suite or whole-domain equality claim. The input route and worksheet publication remain separate from the root primitive.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 42 |
| 20430 | Windows, 64-bit, CPU not recorded | 42,803 |

- XC-SQRT-001: build 19929 versus build 20430; 2 same-input pairs, 0 differing answers. Compared axes: excel-build. Numeric answers were compared by their recorded binary64 bits; other answers by typed value.

Agreement on these compared inputs does not establish agreement on unmeasured inputs.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.SQRT.json).

## Alternatives

[IMSQRT](FUNC.IMSQRT.md) accepts complex inputs. [SQRTPI](FUNC.SQRTPI.md) includes a multiplication by pi, and [IMABS](FUNC.IMABS.md) uses a hypotenuse route for complex magnitude. [POWER](FUNC.POWER.md) accepts other exponents but must retain its own numerical graph.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `SQRT` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

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
