---
schema: efh.function-page/v1
function_id: FUNC.MONTH
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: MONTH function"
    locator: "https://support.microsoft.com/en-us/office/month-function-579a2881-199b-48b2-ab90-ddba0eba86e8"
    role: "documented behaviour of the serial_number argument and the 1-12 return range"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
  - Notes
  - Page vocabulary
  - Sources
family: date_parts_family
role_in_family: "Extracts the month-of-year component from a date serial, as an integer 1 through
  12."
---

## What it computes

`MONTH` projects a date serial onto its month-of-year component: it discards the fractional part,
maps `⌊serial⌋` to a calendar day under the workbook's date system (see
[FUNC.DATE](FUNC.DATE.md)), and returns that day's month as an integer from 1 (January) to 12
(December).

There is no month arithmetic here and no locale sensitivity: the result is a number, never a month
*name*. Producing "January" is the `TEXT` function's job, and it is locale-dependent where `MONTH`
is not.

## Arguments

`MONTH(serial_number)` — one required argument, the date whose month is wanted.

Microsoft's guidance for the whole category applies: supply the date as a serial — from
[DATE](FUNC.DATE.md), from another formula, or from a cell that already holds a date — rather than
as text, because text dates depend on the locale's date order and can be misread. Coercion of the
argument follows the ordinary scalar rules in
[coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns a `Number` in 1…12.

- **Fractional serials** are floored; the time of day is irrelevant.
- **The phantom days.** Serial 0 ("0 January 1900") and serial 60 (the non-existent 29 February
  1900) both live inside months that do exist, so `MONTH` returns an unremarkable 1 and 2
  respectively under any consistent reading. `MONTH` is therefore the least diagnostic member of
  the decomposition trio for the leap-year artefact — but it is also the one whose *neighbours*
  reveal it: `MONTH(59)`, `MONTH(60)` and `MONTH(61)` should read 2, 2, 3.
- **Out-of-range serials.** Negative serials are the documented `#NUM!` case. The upper end is
  discussed under "What has not been checked".

## Errors

As documented for this category:

- `#NUM!` when `serial_number` is out of range for the workbook's date-base value.
- `#VALUE!` when the argument will not convert to a number — the ordinary coercion outcome rather
  than a rule specific to `MONTH`.
- An error argument propagates.

## Relationships

- **[YEAR](FUNC.YEAR.md), [DAY](FUNC.DAY.md)** — the other two projections; together they invert
  [DATE](FUNC.DATE.md).
- **[EOMONTH](FUNC.EOMONTH.md)** — the natural companion when the question is "the end of *this*
  month". `DATE(YEAR(s), MONTH(s)+1, 0)` is the classic hand-rolled equivalent, and relies on
  `DATE`'s day-0 rollover rather than on `MONTH` itself.
- **[EDATE](FUNC.EDATE.md)** — month *offsetting*, which is a different operation from month
  *extraction* and is frequently confused with `MONTH`-based arithmetic. `DATE(YEAR(s),
  MONTH(s)+1, DAY(s))` and `EDATE(s,1)` are believed to disagree at end-of-month — the former
  rolls 31 January into 3 March by `DATE`'s documented day carry, while `EDATE` is generally
  understood to clamp. `EDATE`'s Microsoft page does not state the clamping rule, and this
  Handbook has not verified it.
- **[DATEDIF](FUNC.DATEDIF.md)** with unit `"M"` — complete months *between* two dates, which is
  not a difference of `MONTH` values.

## Notes for implementers

1. **Floor before mapping**, for the same reason as [YEAR](FUNC.YEAR.md): truncation and flooring
   disagree precisely in the region the range check is meant to reject.
2. **Share one serial→(y,m,d) routine** across `YEAR`, `MONTH` and `DAY`. Three independent
   derivations are three chances to disagree at serial 60. The reference engine implements this
   family in one module, which is the right shape.
3. **Never format inside the kernel.** The month number is the value; month names belong to the
   presentation layer and to `TEXT`.

## What has not been checked

No Handbook vector suite exists for `MONTH`, and no Excel-comparison evidence is recorded. The
battery panel on this page shows the reference engine's own answers, obtained without Excel. Worth
probing first:

- **`MONTH(59)`, `MONTH(60)`, `MONTH(61)`** — the three serials straddling the phantom leap day.
  These are cheap and settle whether the artefact is implemented as a coordinate shift.
- **`MONTH(0)`** — the "0 January 1900" phantom.
- **The upper boundary** — `MONTH(2958465)` versus `MONTH(2958466)`, and a maximum-double
  argument, for which the reference engine returns a number rather than an error.
- **A 1904-system workbook**, where the whole serial-to-month mapping shifts by 1,462 days.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `MONTH` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-DATE-001: In the 1900 date system, serial 0 is 1900-01-00 and serial 60 is the fictitious 1900-02-29

*Quirk; measured; recorded 2026-11-22.* Applies to DATE, DAY, DAYS, EDATE, EOMONTH, MONTH, WEEKDAY, YEAR.

In Excel 16.0.20527.20088 under the 1900 date system, serial 0 formats as 1900/1/0
(YEAR(0)=1900, MONTH(0)=1, DAY(0)=0), and 1900 is treated as a leap year so DATE(1900,2,29)
returns 60, WEEKDAY(60,2) returns 3 (Wednesday), and DATE(1900,3,1)-DATE(1900,2,28) returns 2.
EOMONTH(60,0) returns 59 (1900-02-28) rather than 60, whereas EDATE(60,12) returns 425
(1901-02-28).

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=YEAR(0)&"/"&MONTH(0)&"/"&DAY(0)` | as typed | `1900/1/0` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=DATE(1900,2,29)` | as typed | `60.0` (`0x404e000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=EOMONTH(60,0)` | as typed | `59.0` (`0x404d800000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=EDATE(60,12)` | as typed | `425.0` (`0x407a900000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=WEEKDAY(60,2)` | as typed | `3.0` (`0x4008000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=DAYS(DATE(1904,1,1),0)` | as typed | `1462.0` (`0x4096d80000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Measured on all 16 sheets (q001-q016) of fn5_dates_1900 in Excel
16.0.20527.20088.

Evidence:

- DnaCalc `crates/calc-func/tests/captures/fn5_dates_1900/oracle.capture.json`, sheets q001 to q016 at `4fb9b728df` — Excel 16.0.20527.20088 1900 leap-year and serial-0 readings
- DnaCalc `crates/calc-func/src/ported/date_kernels.rs`, ymd_1900; serial_unbounded_1900 at `4fb9b728df` — 1900-system serial 0 and 60 date arithmetic
- DnaCalc `crates/calc-func/tests/captures/fn5_dates_1900/oracle.capture.json`, sheet q001 row 20 column 10 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn5_dates_1900/oracle.capture.json`, sheet q005 row 20 column 10 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn5_dates_1900/oracle.capture.json`, sheet q009 row 20 column 10 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn5_dates_1900/oracle.capture.json`, sheet q008 row 20 column 10 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn5_dates_1900/oracle.capture.json`, sheet q012 row 20 column 10 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn5_dates_1900/oracle.capture.json`, sheet q015 row 20 column 10 at `4fb9b728df` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| date serial | The `Number` denoting a calendar day; see [FUNC.DATE](FUNC.DATE.md) |
| decomposer | A function projecting a serial onto one calendar component |
| month extraction vs month offsetting | Reading a serial's month, versus moving a date by whole months (`EDATE`) |

## Sources

- Microsoft 365 support, **MONTH function** —
  <https://support.microsoft.com/en-us/office/month-function-579a2881-199b-48b2-ab90-ddba0eba86e8>.
- [FUNC.DATE](FUNC.DATE.md) — serial-number model, leap-year artefact, 1900/1904 systems.
- Handbook call-model chapter [02 coercion and lifting](../model/02-coercion-and-lifting.md).
- `data/functions/FUNC.MONTH.json`, `data/presence/FUNC.MONTH.json`,
  `data/battery/FUNC.MONTH.json`.
