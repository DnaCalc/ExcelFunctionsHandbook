---
schema: efh.function-page/v1
function_id: FUNC.ROUND
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft — WorksheetFunction.Round method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.round"
    role: "the three documented remarks about num_digits greater than, equal to and less than zero; note that no tie-breaking rule is stated"
  - work: "Microsoft Support — ROUND function"
    locator: "https://support.microsoft.com/en-us/office/round-function-c018c5d8-40fb-4053-90b1-b3e7f61a213c"
    role: "the worksheet-surface documentation page named in the projection; not retrievable for this pass"
  - work: "Steele & White, How to print floating-point numbers accurately"
    locator: "PLDI 1990"
    role: "why decimal rounding of a binary value is a conversion problem, not an arithmetic one"
  - work: "Gay, Correctly rounded binary-decimal and decimal-binary conversions"
    locator: "AT&T Bell Labs Numerical Analysis Manuscript 90-10, 1990"
    role: "the reference algorithm for exact decimal rounding of a binary64 value"
  - work: "IEEE 754-2019, clause 5.9 and clause 9"
    locator: "roundToIntegralTiesToEven; the decimal rounding attributes"
    role: "the standard's own rounding directions, against which Excel's tie rule is undocumented"
episodes: []
body_sections:
  - "What it does"
  - "Try it"
  - "Quirks"
  - "How to implement"
  - "Version differences"
  - "Alternatives"
  - "Notes"
  - "Sources"
family: round_fn
role_in_family: "The nearest-value rounder; the middle member of the three-way ROUND / ROUNDDOWN / ROUNDUP split by direction."
---

# ROUND

## What it does

`ROUND(number, num_digits)` rounds a number to the requested decimal place. Positive
`num_digits` keeps places after the decimal point, zero rounds to an integer, and negative
values round to tens, hundreds and larger places. Both arguments are required.

The observed tie rule is away from zero: `ROUND(2.5,0)` is 3 and `ROUND(-2.5,0)` is -3 on the
named build below. Excel chooses the digit from a 15-significant-digit reading of the input.
That matters for familiar decimal examples: `ROUND(1.005,2)` is 1.01 and `ROUND(2.675,2)` is 2.68,
even though their stored binary values lie just below the corresponding decimal halfway points.

The result is a number, not formatted text. Changing a cell's displayed decimal places does
not perform this calculation or change the number used by later formulas.

## Try it

Start with `=ROUND(1.005,2)`. Compare `=ROUND(-2.5,0)`, `=ROUND(1234.5,-2)` and
`=ROUND(2.675,2)`. The browser computes with OneCalc; the Excel readings below remain a separate
comparison, with their own build and platform.

## Quirks

### The stored binary value is not the whole rounding rule

Decimal inputs usually have no finite binary64 representation. Rounding the exact stored value
to the nearest decimal step can therefore disagree with Excel's 15-digit reading. Replacing
ROUND with a language's default `round` is unsafe: that routine may use ties to even, and it may
choose the digit directly from the stored value.

### Precision arguments can wrap

On Excel 16.0 build 20430, `num_digits` is cut toward zero and limited to ±2147483647. A signed
32-bit calculation of the retained significant-digit count can wrap. Consequently an enormous
positive precision can return zero rather than leave the input alone. The boundary moves with
the input's decimal exponent; the recorded examples below give the calls and bits.

### Values near the largest finite number

The recorded build 20527 can return all-ones-exponent binary64 patterns which ordinary IEEE
arithmetic calls NaNs, while Excel's cell machinery treats them as numbers and displays a huge
decimal. Saving the workbook writes these values as `#NUM!`; recalculating can restore the
patterns. They must not be silently interpreted as ordinary finite results. The detailed
examples below distinguish cell inspection, arithmetic and file round trips.

## How to implement

For an ordinary mathematical decimal rounder, decompose the binary64 input into an exact
integer significand and power of two. Apply the decimal scale as integer powers of 2 and 5,
compare the remainder with halfway, choose a declared tie rule, then convert the resulting
decimal rational back to binary64. This avoids overflow and spurious ties from a floating-point
multiply followed by a divide. It intentionally does not reproduce Excel's preliminary reading.

To target the measured Excel route, preserve its separate stages: normalize the magnitude to
15 significant decimal digits using the observed x87 intermediate precision; apply that stage's
tie rule; truncate and clamp the precision argument; form the significant-digit count with
32-bit wrapping; then round at the requested digit away from zero and restore the sign.
The preliminary half rule for ROUND differs from the one observed for ROUNDDOWN, ROUNDUP and
TRUNC. One shared generic rounding instruction does not describe both stages.

Probe every transition in decimal exponent, halfway inputs and their immediate binary64
neighbours, large positive and negative precision, zero, and the upper finite boundary. A
scale-and-round formula alone does not establish agreement on those regions. Matching the
near-overflow cell encodings also requires their publication and file-serialization rules.

## Version differences

The observations below name builds 20430 and 20527, including a full 16.0.20527.20088 capture
where available. They cover different experiments, so differences in the recorded examples
must not be read as a demonstrated change between those builds. CPU information is included
only where the capture records it. Other platforms and versions still need corresponding probes.

## Alternatives

[ROUNDDOWN](FUNC.ROUNDDOWN.md) chooses toward zero; [ROUNDUP](FUNC.ROUNDUP.md) chooses away from
zero. [TRUNC](FUNC.TRUNC.md) discards decimal places. [MROUND](FUNC.MROUND.md) rounds to a stated
multiple; its halfway behaviour has its own numerical questions. A number format changes only
presentation. For a new mathematical API, an explicitly specified decimal or rational rounding
algorithm may be preferable to reproducing Excel's preliminary 15-digit reading.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `ROUND` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-ROUND-001: ROUND counts digits in 32 bits, and the count wraps

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.*

ROUND cuts num_digits toward zero to a 32-bit integer D, limited to +/-2147483647, and keeps k
= D + e + 1 significant digits, where e is the decimal exponent of |number| and the sum is
computed in 32 bits. k of 15 or more keeps the 15-digit reading of the number; k below 0 gives
0. A large positive num_digits therefore wraps to no digits at all: for a number near 1.1E14,
2147483617 returns the number and 2147483635 returns 0. The boundaries move by exactly -e on
both the +2^31 and -2^31 sides, for e from -308 to 308. num_digits of 2^31 or more acts as
+2147483647 and -2^31 or less as -2147483647.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `ROUND(-112500000000000, 2147483635)` | number `-112500000000000` (`0xc2d9945ca2620000`); num_digits `2147483635` (`0x41dffffffcc00000`) (cells) | `0` (`0x0000000000000000`) | `-112500000000000` (`0xc2d9945ca2620000`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `ROUND(-112500000000000, 2147483617)` | number `-112500000000000` (`0xc2d9945ca2620000`); num_digits `2147483617` (`0x41dffffff8400000`) (cells) | `-112500000000000` (`0xc2d9945ca2620000`) | — | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |

Correct value: -112500000000000 is an integer, so rounding it to any non-negative number of
places returns it.

Basis (measured): Under the family rules the recorded rows match ROUND 36,821 of 36,945,
ROUNDUP 52,022 of 52,841, ROUNDDOWN 59,182 of 59,995 and TRUNC 50,504 of 51,317, with no row
lost and each remaining failure classified in the write-up. The wrap boundary was read at every
decimal exponent in the recorded rows.

Evidence:

- DnaCalc `crates/calc-func/src/catalogue/rounding.rs` at `ab94fb91a5` — the rules in the product; the commit message carries the row counts
- [`content/notes/evidence/2026-11-04/rounding-EVIDENCE.md`](../../content/notes/evidence/2026-11-04/rounding-EVIDENCE.md) — research write-up: full-bank result, the rules, what stays open
- CalcFunc-Evidence `runs/w111-rounding-count-refinement-20260929/answers-round.json.gz`, w111roundcount-ROUND-01694 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `runs/w111-rounding-count-refinement-20260929/answers-round.json.gz`, w111roundcount-ROUND-01682 at `4d5320dfdc` (readings)

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

### FN-ROUND-004: ROUND takes ties away from zero on the 15-digit reading, so 1.005 rounds up

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.*

A tie in ROUND goes away from zero, not to even: ROUND(2.5,0) is 3 and ROUND(-2.5,0) is -3. The
tie is judged on the 15-digit reading of FN-ROUND-002, not on the stored value. So 1.005 and
2.675, whose binary64 values lie just below the decimal tie, count as ties: ROUND(1.005,2) is
1.01 and ROUND(2.675,2) is 2.68, where rounding the stored value would give 1 and 2.67.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `=ROUND(2.5,0)` | as typed | `3` (`0x4008000000000000`) | — | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=ROUND(-2.5,0)` | as typed | `-3` (`0xc008000000000000`) | — | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=ROUND(2.675,2)` | as typed | `2.68` (`0x400570a3d70a3d71`) | `2.67` (`0x40055c28f5c28f5c`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=ROUND(1.005,2)` | as typed | `1.01` (`0x3ff028f5c28f5c29`) | `1` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Correct value: The stored value lies below the decimal tie, so the nearest multiple of 0.01 is
the lower one; nearest binary64.

Basis (measured): The readings below, typed as literals, and the family rule of FN-ROUND-002
over the recorded ROUND rows.

Inferred, not measured: That every tie follows this is inferred from the rule's fit to the
recorded rows; the literal ties read directly are the ones below.

Evidence:

- DnaCalc `crates/calc-func/tests/captures/cfunc_m3_2/oracle.capture.json` at `282f03144b` — the literal readings
- [`content/notes/evidence/2026-11-04/rounding-EVIDENCE.md`](../../content/notes/evidence/2026-11-04/rounding-EVIDENCE.md) — the fifteen-digit reading
- DnaCalc `crates/calc-func/tests/captures/cfunc_m3_2/oracle.capture.json`, sheet rounding_boundaries, row 8, column 4 at `282f03144b` (readings)
- DnaCalc `crates/calc-func/tests/captures/cfunc_m3_2/oracle.capture.json`, sheet rounding_boundaries, row 9, column 4 at `282f03144b` (readings)
- DnaCalc `crates/calc-func/tests/captures/cfunc_m3_2/oracle.capture.json`, sheet rounding_boundaries, row 19, column 4 at `282f03144b` (readings)
- DnaCalc `crates/calc-func/tests/captures/cfunc_m3_2/oracle.capture.json`, sheet rounding_boundaries, row 21, column 4 at `282f03144b` (readings)

<!-- efh:notes end -->

## Sources

- [Microsoft: ROUND function](https://support.microsoft.com/en-us/excel/functions/round-function).
- [Precision count](../notes/FN-ROUND-001.json), [15-digit reading](../notes/FN-ROUND-002.json),
  [near-overflow encodings](../notes/FN-ROUND-003.json) and
  [tie examples](../notes/FN-ROUND-004.json).
- Gay, *Correctly rounded binary-decimal and decimal-binary conversions*, Numerical Analysis
  Manuscript 90-10 (1990), for the mathematical conversion problem.
