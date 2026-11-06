---
schema: efh.function-page/v1
function_id: FUNC.ROUNDDOWN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft — WorksheetFunction.RoundDown method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.rounddown"
    role: "the documented \"rounds a number down, toward 0 (zero)\" definition and the three num_digits remarks"
  - work: "Microsoft Support — ROUNDDOWN function"
    locator: "https://support.microsoft.com/en-us/office/rounddown-function-2ec94c73-241f-4b01-8c6f-17e6d7968f53"
    role: "the worksheet-surface documentation page named in the projection; not retrievable for this pass"
  - work: "Gay, Correctly rounded binary-decimal and decimal-binary conversions"
    locator: "AT&T Bell Labs Numerical Analysis Manuscript 90-10, 1990"
    role: "the reference algorithm for deciding a decimal digit exactly rather than by scaling"
  - work: "IEEE 754-2019, clause 4.3.2"
    locator: "roundTowardZero"
    role: "the directed rounding attribute this function implements at a decimal place"
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
family: rounddown_fn
role_in_family: "The directed toward-zero decimal rounder; TRUNC supplies the same operation with an optional digit count."
---

# ROUNDDOWN

## What it does

`ROUNDDOWN(number, num_digits)` rounds toward zero by discarding the unwanted decimal part. Positive digit counts address places after the decimal point, zero the integer position, and negative counts places to its left.

## Try it

`=ROUNDDOWN(-7.29,1)` returns `-7.2`; `=ROUNDDOWN(729,-1)` returns `720`. “Down” means toward zero.

## Quirks

The recorded functions first read fifteen significant decimal digits. Asking to retain more digits can therefore change an input that mathematically needs no rounding. The recorded `ROUNDDOWN(283025097725723.5,0)` returns `283025097725724`, showing that the preliminary reading can move the number upward before truncation. Huge digit counts have recorded integer-conversion behaviour rather than a simple “keep everything” rule. Near the normal-range boundary and overflow, worksheet publication needs separate handling.


The exact-cell minimum normal input `0x0010000000000000` has a recorded early return of `0x000ffffffffffffa`, even at zero requested digits. That return happens before ordinary digit rounding. At the upper end, the current model can publish a tagged NaN bit pattern as a worksheet number. See the [packing rules and worked encodings](../model/09-staged-arithmetic.md#decimal-family-packing-exceptions).

## How to implement

The current worksheet wrapper reads numeric arguments in order and flushes subnormal numeric operands to zero. Return zero for a zero `number` before finding its decimal exponent. The following steps start with a finite, nonzero admitted input. The [stored-arithmetic chapter](../model/09-staged-arithmetic.md) explains exact binary decoding and significand rounding.

First construct the number’s fifteen-significant-digit **reading**. Extract the exact binary64 value and find its decimal exponent `e` by comparisons with exact powers of ten, so `10^e <= abs(x) < 10^(e+1)`. Put `s=14-e`. When `abs(s)<=27`, evaluate `abs(x)*10^s` (or `abs(x)/10^(-s)`) with a 64-bit significand and extended exponent range, rounding to nearest/even. **Do not store that scaled value to binary64 before the integer decision.** Round it to an integer: exact halves go down for ROUND’s reading, up for the directed functions’ readings. Retain the sign and the exact decimal value `integer*10^(-s)`.

This reading step is a measured reconstruction with a known counterexample even inside the stated scale range. For `abs(s)>27`, Excel’s scaling is still open; the current code falls back to exact binary64-to-decimal rounding to fifteen digits. That fallback is a candidate, not an identified Excel stage.

1. Truncate the magnitude of `num_digits` and retain its low sixteen bits, then restore the sign. Magnitudes at least `2^32`, or low bits at least `32768`, select an out-of-range sentinel of 100 places with the original sign.
2. Before digit rounding, check the special initial-reading return described below. If it applies, return its packed result immediately. Otherwise truncate the exact decimal reading at unit `10^(-D)`, then convert that truncation to nearest binary64, using the [overflow packing exception](../model/09-staged-arithmetic.md#decimal-family-packing-exceptions) when required.
3. Return that stored truncation without adding a unit. Flush final subnormal results to zero, apart from the special initial-reading path below.

Worked: `x=-7.29`, `D=1` retains 72 tenths of the magnitude and restores the negative sign, giving `-7.2`. A count of `65537` uses low bits equal to one, so it selects the same decimal position. This conversion rule is distinct from ROUND’s signed 32-bit count.

The current boundary reconstruction handles a normal input below twice the minimum normal value whose initial reading becomes subnormal: convert twice the reading (a normal number) and decrement its encoded exponent field by one, retaining the sign and significand. This reproduces the observed packing rather than an ordinary subnormal conversion. Other final subnormals still flush. The large-scale reading and remaining last-bit discrepancies stay open; consult the dated failure breakdown.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 112 |
| 20430 | Windows, 64-bit, AMD Ryzen 7 4800H with Radeon Graphics | 3,454 |
| 20430 | Windows, 64-bit, CPU not recorded | 56,541 |

- XC-ROUNDDOWN-001: build 19929 versus build 20430; 3 same-input pairs, 0 differing answers. Compared axes: excel-build. Numeric answers were compared by their recorded binary64 bits; other answers by typed value.

Agreement on these compared inputs does not establish agreement on unmeasured inputs.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.ROUNDDOWN.json).

## Alternatives

[ROUND](FUNC.ROUND.md) chooses a nearest decimal; [MROUND](FUNC.MROUND.md) chooses a multiple. [INT](FUNC.INT.md) floors negatives; [TRUNC](FUNC.TRUNC.md) removes their fraction toward zero.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `ROUNDDOWN` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-ROUND-002: The rounding functions read 15 digits first, so ROUNDDOWN can round up

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.* Applies to ROUND, ROUNDDOWN, ROUNDUP, TRUNC.

Before choosing a digit these functions read the number to 15 significant digits on the x87
unit: with e the decimal exponent and n = 14 - e, y = RN64(|x| * 10^n), or RN64(|x| / 10^-n)
when n is negative, while 10^|n| is exact in 64 bits (|n| <= 27). A half in y goes up for
ROUNDDOWN, ROUNDUP and TRUNC and down for ROUND. So digits beyond the 15th are lost even when
the requested precision keeps them: ROUND(-1594628480062905,14) loses its last digit, and
ROUNDDOWN(283025097725723.5,0) returns 283025097725724, larger than its input.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `ROUND(-1594628480062905, 14)` | number `-1594628480062905` (`0xc316a939806956e4`); num_digits `14` (`0x402c000000000000`) (cells) | `-1594628480062900` (`0xc316a939806956d0`) | `-1594628480062905` (`0xc316a939806956e4`) | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |
| `ROUNDDOWN(283025097725723.5, 0)` | number `283025097725723.5` (`0x42f0168ea69971b8`); num_digits `0` (`0x0000000000000000`) (cells) | `283025097725724` (`0x42f0168ea69971c0`) | `283025097725723` (`0x42f0168ea69971b0`) | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |

Correct value: An integer rounded to 14 places is itself.
Correct value: ROUNDDOWN drops the fraction .5.

Basis (measured): Under the family rules the recorded rows match ROUND 36,821 of 36,945,
ROUNDUP 52,022 of 52,841, ROUNDDOWN 59,182 of 59,995 and TRUNC 50,504 of 51,317, with no row
lost and each remaining failure classified in the write-up.

- Open: How the scale is formed for |n| > 27: a single correctly rounded 64-bit power and the chained multiplications tried do not fit; 123 ROUND, 548 ROUNDUP, 556 ROUNDDOWN and 555 TRUNC rows remain.
- Open: The reading step itself: one control at n = -10 misses the single RN64 step by just over half a unit, so that step is an approximation of the real one even for |n| <= 27. It stays because it gains rows and loses none.

Evidence:

- DnaCalc `crates/calc-func/src/catalogue/rounding.rs` at `ab94fb91a5` — the rules in the product; the commit message carries the row counts
- [`content/notes/evidence/2026-11-04/rounding-EVIDENCE.md`](../../content/notes/evidence/2026-11-04/rounding-EVIDENCE.md) — research write-up: full-bank result, the rules, what stays open
- CalcFunc-Evidence `cache/oracle-20430/build-20430/ROUND.jsonl.gz`, line 3758, key ROUND|0xc316a939806956e4|0x402c000000000000 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `cache/oracle-20430/build-20430/ROUNDDOWN.jsonl.gz`, line 753, key ROUNDDOWN|0x42f0168ea69971b8|0x0000000000000000 at `4d5320dfdc` (readings)

### FN-ROUND-003: Near DBL_MAX, ROUND returns NaN bit patterns that Excel treats as numbers

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.* Applies to ROUND, ROUNDDOWN, ROUNDUP, TRUNC.

ROUND of a number near 1.8E308 can return a binary64 with an all-ones exponent field. A decimal
result of 2^1024 or more keeps its 53-bit significand, and its exponent is carried into the
exponent field without a check, so the bits are a NaN. ROUND, ROUNDDOWN and TRUNC return those
bits; ROUNDUP gives #NUM!. The recorded rows hold 430 such ROUND rows, 102 ROUNDDOWN and 95
TRUNC, and all 627 follow this. Even ROUND(DBL_MAX,0), whose answer is DBL_MAX, returns
0x7ff000000000000a. The cell shows 2E+308 or 1.8E+308, and a pattern with its sign bit set
shows 2E+308 too, with no minus. ISNUMBER gives TRUE, ISERROR FALSE, TYPE 1, ERROR.TYPE #N/A;
A1=A1 and A1>0 are TRUE. =A1, N(A1), MAX(A1), ABS(A1) and IFERROR(A1,...) pass the pattern on,
as does =-A1 with its sign unchanged. Arithmetic on it (A1+0, A1*1, A1-A1, SUM, ROUND) gives
#NUM!, and TEXT(A1,"0") gives #VALUE!. Saved to .xlsx, each such cell is written as the error
#NUM!; reopened, the cells read #NUM! until a full recalculation brings the patterns back.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `ROUND(1.7976931348623157E+308, 0)` | number `1.7976931348623157e+308` (`0x7fefffffffffffff`); num_digits `0` (`0x0000000000000000`) (cells) | `NaN bit pattern, shown as 1.8E+308` (`0x7ff000000000000a`) | `1.7976931348623157e+308` (`0x7fefffffffffffff`) | Excel 16.0 build 20527, Windows; bitness and CPU not recorded in this reading |
| `ROUND(1.7976931348623157E+308, -307)` | number `1.7976931348623157e+308` (`0x7fefffffffffffff`); num_digits `-307` (`0xc073300000000000`) (cells) | `NaN bit pattern, shown as 1.8E+308` (`0x7ff005419221015d`) | `#NUM!` | Excel 16.0 build 20527, Windows; bitness and CPU not recorded in this reading |
| `ROUND(1.7976931348623157E+308, -308)` | number `1.7976931348623157e+308` (`0x7fefffffffffffff`); num_digits `-308` (`0xc073400000000000`) (cells) | `NaN bit pattern, shown as 2E+308` (`0x7ff1ccf385ebc8a0`) | `#NUM!` | Excel 16.0 build 20527, Windows; bitness and CPU not recorded in this reading |

Correct value: DBL_MAX is an integer, so rounding it to 0 places returns it.
Correct value: The rounded value exceeds the largest binary64; Excel reports overflow in its
own arithmetic as #NUM!.

Basis (measured): One Excel workbook read cell by cell through Value2 on 2026-11-04, saved,
reopened and recalculated; rows from the rounding write-up, 627 of 627.

Inferred, not measured: That one decimal-to-binary conversion routine is the source of all
these patterns is inferred from the exponent-overflow encoding.

- Open: How these patterns sort among themselves and with other numbers, and display width, are not recorded.

Evidence:

- DnaCalc `crates/calc-func/src/catalogue/rounding.rs` at `ab94fb91a5` — the rules in the product; the commit message carries the row counts
- [`content/notes/evidence/2026-11-04/round-nan-downstream.json`](../../content/notes/evidence/2026-11-04/round-nan-downstream.json), cells A1..A4, B1..B24, reopened, recalculated — cell-by-cell Excel reading of the NaN patterns
- [`content/notes/evidence/2026-11-04/round-nan-downstream.ps1`](../../content/notes/evidence/2026-11-04/round-nan-downstream.ps1) — the script that took the reading
- [`content/notes/evidence/2026-11-04/round-nan-downstream-sheet1.xml`](../../content/notes/evidence/2026-11-04/round-nan-downstream-sheet1.xml) — the sheet as Excel saved it
- [`content/notes/evidence/2026-11-04/rounding-EVIDENCE.md`](../../content/notes/evidence/2026-11-04/rounding-EVIDENCE.md) — research write-up: full-bank result, the rules, what stays open
- [`content/notes/evidence/2026-11-04/round-nan-downstream.json`](../../content/notes/evidence/2026-11-04/round-nan-downstream.json), cells.A3 (readings)
- [`content/notes/evidence/2026-11-04/round-nan-downstream.json`](../../content/notes/evidence/2026-11-04/round-nan-downstream.json), cells.A2 (readings)
- [`content/notes/evidence/2026-11-04/round-nan-downstream.json`](../../content/notes/evidence/2026-11-04/round-nan-downstream.json), cells.A1 (readings)

### FN-ROUNDUP-001: ROUNDUP, ROUNDDOWN and TRUNC read num_digits from its low 16 bits

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.* Applies to ROUNDDOWN, ROUNDUP, TRUNC.

ROUNDUP, ROUNDDOWN and TRUNC take the digit count from the low 16 bits of the integer part of
|num_digits|, with the sign of num_digits. If |num_digits| is 2^32 or more, or those low bits
are 32768 or more, the truncation happens at 100 places (10^-100, or 10^100 for a negative
count). So 65536 acts as 0, 65537 as 1, 32768 to 65535 as 100, and -2147483748, whose low 16
bits are 100, as -100. In the out-of-range case ROUNDUP adds no unit for a positive count, and
gives #NUM! for a negative count when a digit is dropped.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `ROUNDUP(1.234567890123456e-200, 65536)` | number `1.234567890123456e-200` (`0x166e3d71f622be8f`); num_digits `65536` (`0x40f0000000000000`) (cells) | `1` (`0x3ff0000000000000`) | `1.234567890123456e-200` (`0x166e3d71f622be8f`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `ROUNDUP(1.234567890123456e-200, 65537)` | number `1.234567890123456e-200` (`0x166e3d71f622be8f`); num_digits `65537` (`0x40f0001000000000`) (cells) | `0.1` (`0x3fb999999999999a`) | `1.234567890123456e-200` (`0x166e3d71f622be8f`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `ROUNDUP(1.234567890123456e-200, 32768)` | number `1.234567890123456e-200` (`0x166e3d71f622be8f`); num_digits `32768` (`0x40e0000000000000`) (cells) | `0` (`0x0000000000000000`) | `1.234567890123456e-200` (`0x166e3d71f622be8f`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `TRUNC(1.234567890123456e-200, 65536)` | number `1.234567890123456e-200` (`0x166e3d71f622be8f`); num_digits `65536` (`0x40f0000000000000`) (cells) | `0` (`0x0000000000000000`) | `1.234567890123456e-200` (`0x166e3d71f622be8f`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |

Correct value: The exact decimal expansion of this binary64 number has fewer than 32768 places,
so a truncation at num_digits places returns the number itself.

Basis (measured): Under the family rules the recorded rows match ROUND 36,821 of 36,945,
ROUNDUP 52,022 of 52,841, ROUNDDOWN 59,182 of 59,995 and TRUNC 50,504 of 51,317, with no row
lost and each remaining failure classified in the write-up. All 13,914 informative ROUNDDOWN
and TRUNC rows with large num_digits follow the rule.

Evidence:

- DnaCalc `crates/calc-func/src/catalogue/rounding.rs` at `ab94fb91a5` — the rules in the product; the commit message carries the row counts
- [`content/notes/evidence/2026-11-04/rounding-EVIDENCE.md`](../../content/notes/evidence/2026-11-04/rounding-EVIDENCE.md) — research write-up: full-bank result, the rules, what stays open
- CalcFunc-Evidence `runs/w111-rounding-count-refinement-20260929/answers-roundup.json.gz`, w111roundcount-ROUNDUP-00041 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `runs/w111-rounding-count-refinement-20260929/answers-roundup.json.gz`, w111roundcount-ROUNDUP-00043 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `runs/w111-rounding-count-refinement-20260929/answers-roundup.json.gz`, w111roundcount-ROUNDUP-00033 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `runs/w111-rounding-count-refinement-20260929/answers-trunc.json.gz`, w111roundcount-TRUNC-00041 at `4d5320dfdc` (readings)

### FN-TRUNC-001: TRUNC of DBL_MIN returns a subnormal, not 0

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.* Applies to ROUNDDOWN, ROUNDUP, TRUNC.

With DBL_MIN (0x0010000000000000) as number, TRUNC, ROUNDDOWN and ROUNDUP return the subnormal
0x000ffffffffffffa at each num_digits recorded, so TRUNC(DBL_MIN,0) is not 0. That value is 6
steps below DBL_MIN and 3 steps below 0x000ffffffffffffd, the binary64 nearest the 15-digit
reading 2.22507385850720E-308. The recorded rows hold 256 such cases for each of the three
functions. ROUND gives 0 on the same inputs.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `TRUNC(2.2250738585072014e-308, 0)` | number `2.2250738585072014e-308` (`0x0010000000000000`); num_digits `0` (`0x0000000000000000`) (cells) | `2.2250738585071984e-308` (`0x000ffffffffffffa`) | `0` (`0x0000000000000000`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `TRUNC(2.2250738585072014e-308, 1)` | number `2.2250738585072014e-308` (`0x0010000000000000`); num_digits `1` (`0x3ff0000000000000`) (cells) | `2.2250738585071984e-308` (`0x000ffffffffffffa`) | `0` (`0x0000000000000000`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |

Correct value: DBL_MIN is below 1, so truncating it at 0 or 1 places gives 0.

Basis (measured): Recorded rows of the broad capture; counts from the rounding write-up.

- Open: Why this one input converts 3 steps below nearest.

Evidence:

- [`content/notes/evidence/2026-11-04/rounding-EVIDENCE.md`](../../content/notes/evidence/2026-11-04/rounding-EVIDENCE.md) — research write-up: full-bank result, the rules, what stays open
- CalcFunc-Evidence `runs/w111-broad-20260929/answers/answers-trunc.json.gz`, w111broad-20260929-TRUNC-00026 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `runs/w111-broad-20260929/answers/answers-trunc.json.gz`, w111broad-20260929-TRUNC-00027 at `4d5320dfdc` (readings)

<!-- efh:notes end -->

## Excel versions, platforms and CPUs

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.ROUNDDOWN.json; edits here are overwritten -->

Readings of ROUNDDOWN with a recorded Excel build: 60,107 input(s) over 3 recorded
environment(s), Excel 16.0 build(s) 19929, 20430, all on Windows; recorded CPUs: AMD Zen 2. 1
same-input comparison(s) between environments over 3 input-pair(s): no result differs. No
reading from a Mac, Excel for the web, 32-bit Excel or an Intel CPU is recorded.

Readings of `ROUNDDOWN` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 112 | date not recorded |
| 16.0 build 20430 | Windows, 64-bit | AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | 3,454 | 2026-09-25 |
| 16.0 build 20430 | Windows, 64-bit | CPU not recorded | 56,541 | 2026-09-29 |

Compared on the same inputs:

| Check | Differs in | Environments | Inputs compared | Inputs with different results |
|---|---|---|---:|---:|
| XC-ROUNDDOWN-001 | excel-build | 16.0 build 19929 vs 16.0 build 20430 | 3 | 0 |

Differences observed: none.

Evidence for every row: [`content/evidence/environments/FUNC.ROUNDDOWN.json`](../../content/evidence/environments/FUNC.ROUNDDOWN.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Sources

- [Microsoft: ROUNDDOWN function](https://support.microsoft.com/en-us/excel/functions/rounddown-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/rounding.rs); implementation steps and the limits identified in the text.
- [Rounding research](../notes/evidence/2026-11-04/rounding-EVIDENCE.md): measured reading, count rules, overflow and remaining failures.
