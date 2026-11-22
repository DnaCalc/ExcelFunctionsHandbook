---
schema: efh.function-page/v1
function_id: FUNC.EDATE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: EDATE function"
    locator: "https://support.microsoft.com/en-us/office/edate-function-3c920eb2-6e66-44e7-a1f5-753ae47ee4f5"
    role: "documented syntax, months truncation rule, and the #VALUE! condition"
  - work: "Microsoft 365 support: EOMONTH function"
    locator: "https://support.microsoft.com/en-us/office/eomonth-function-7314ffa1-2bc9-4005-9d66-f49db127d628"
    role: "the sibling function whose documented error behaviour differs from EDATE's"
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
family: date_week_family
role_in_family: "Month-offset arithmetic: the same day-of-month a whole number of months away."
---

## What it computes

`EDATE` moves a date by a whole number of months, keeping the day-of-month. The serial it returns
denotes the day with the same day-of-month, in the month `months` steps away from `start_date`'s
month.

The interesting part of that sentence is what it does *not* say, and the documentation does not say
it either. When the target month has no such day-of-month — 31 January offset by one month, or 29
February offset by twelve — some rule has to apply. Calendars generally clamp to the last day of
the target month, and `EDATE` is widely understood to clamp. **Microsoft's `EDATE` page does not
state a rule for that case, and this Handbook has not verified Excel's behaviour.** It is stated
here as the expected behaviour and nothing stronger.

That gap is the whole reason `EDATE` is not simply `DATE(YEAR(s), MONTH(s)+n, DAY(s))`. `DATE`
carries excess days *forward* — its documented rule turns a 31st in a 30-day month into the 1st of
the next month. If `EDATE` clamps, the two expressions disagree on exactly the dates people build
payment schedules from. If it does not clamp, `EDATE` is redundant. Either way, the answer is
load-bearing and it is not written down.

## Arguments

`EDATE(start_date, months)` — both required.

**start_date** — the date to offset. Microsoft's page repeats the category guidance: supply dates
"by using the DATE function, or as results of other formulas or functions", not as text.

**months** — "the number of months before or after start_date. A positive value for months yields
a future date; a negative value yields a past date." The page states the conversion rule
explicitly: **"If months is not an integer, it is truncated."** Truncation, not rounding — so
`EDATE(s, 1.9)` is one month, and `EDATE(s, -1.9)` is, on the ordinary meaning of truncation, minus
one month rather than minus two. The behaviour of truncation on negative values is the part worth
testing, because languages disagree about it.

## Result and edge cases

Returns a `Number`: a whole date serial, with no time component.

- **Day-of-month preservation is the defining behaviour** — and the end-of-month case is the
  undocumented one described above.
- **`months = 0`** returns the start date's own serial (with any time component dropped), which
  makes `EDATE(s,0)` a serviceable "floor to whole day" idiom.
- **Fractional `months`** is truncated, per the documented rule.
- **Fractional `start_date`** — the page does not say whether the time component is truncated
  before or after the offset. For whole-month arithmetic the two agree, so this is a low-stakes
  gap, but it is a gap.
- **The 1900 boundary.** An offset that lands below serial 1 is out of range; see
  [FUNC.DATE](FUNC.DATE.md) for the serial model, and note that months landing in January or
  February 1900 interact with the phantom leap day.

## Errors

As documented on the Microsoft `EDATE` page:

- `#VALUE!` when `start_date` is not a valid date.

That is the only condition the page states — and it is worth noticing that the sibling
[EOMONTH](FUNC.EOMONTH.md) page documents `#NUM!` for the corresponding situation, plus a second
`#NUM!` when `start_date` plus `months` yields an invalid date. Two functions of identical shape,
documented with different error values for the same failure. Whether Excel actually behaves
differently, or whether one of the two pages is simply wrong, is **an open documentation
discrepancy that nobody has resolved against a running Excel for this Handbook.** It is exactly the
kind of divergence [claim-language rule 5](../model/06-claim-language.md) exists to keep visible
rather than silently harmonized.

`EDATE`'s page does not document what happens when the *result* is out of range, though `EOMONTH`'s
does for the same operation.

## Relationships

- **[EOMONTH](FUNC.EOMONTH.md)** — the same month offset, but landing on the last day of the target
  month instead of preserving the day-of-month. `EOMONTH(s, n)` and `EDATE(s, n)` coincide exactly
  when `s` is already a month end and the target month is no shorter.
- **[DATE](FUNC.DATE.md)** — the constructor whose month rollover is the *carrying* alternative to
  `EDATE`'s (expected) clamping.
- **[WORKDAY](FUNC.WORKDAY.md)** — offsetting by working days rather than months.
- **[DATEDIF](FUNC.DATEDIF.md) with unit `"M"`** — the inverse question: how many whole months lie
  between two dates.
- **Confusable.** `EDATE` and `EOMONTH` are near-interchangeable in appearance and never in
  meaning; and `EDATE(s, 12)` is not the same as `DATE(YEAR(s)+1, MONTH(s), DAY(s))` if the
  clamping hypothesis holds and `s` is 29 February.

## Notes for implementers

1. **Write the end-of-month rule down explicitly**, whichever way you implement it. It is the one
   behaviour of this function that a reader cannot infer from the documentation, and therefore the
   one a compatibility consumer most needs stated.
2. **Truncate `months` toward zero**, matching the documented word "truncated" — do not floor.
   `-1.9` truncates to `-1` and floors to `-2`; the two give different answers for every negative
   fractional offset.
3. **Do the arithmetic in (year, month) space**, not by adding approximate day counts. Convert the
   serial to `(y, m, d)`, add `months` to a zero-based month index with a floor-division carry,
   then reconstruct — clamping `d` at the target month's length if that is the chosen rule.
4. **Do not assume the sibling's error value.** Until the `#VALUE!`/`#NUM!` discrepancy above is
   resolved, an implementation that shares one error path between `EDATE` and `EOMONTH` is making
   an unverified compatibility bet.

## What has not been checked

No Handbook vector suite exists for `EDATE`, and no Excel-comparison evidence is recorded. The
battery panel on this page shows the reference engine's own answers, produced without Excel. The
probes that would settle the open questions:

- **`EDATE(DATE(2024,1,31), 1)`** — the single most important input for this function. Clamping
  gives 29 February 2024; carrying gives 2 March 2024. The documentation does not choose.
- **`EDATE(DATE(2024,2,29), 12)`** — the leap-day anniversary case, which distinguishes clamping to
  28 February from any other rule.
- **`EDATE(s, -1.9)` and `EDATE(s, 1.9)`** — confirms truncate-toward-zero for negatives.
- **`EDATE("not a date", 1)` versus `EOMONTH("not a date", 1)`** — resolves the documented
  `#VALUE!`/`#NUM!` discrepancy between the two pages, in one experiment.
- **Offsets landing before serial 1 and after serial 2958465** — where the range errors begin, which
  `EDATE`'s page does not document at all.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `EDATE` that an implementer matching Excel, or a reader explaining an
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
| month offset | Moving a date by whole months, keeping the day-of-month where possible |
| clamping | Landing on the target month's last day when the day-of-month does not exist there |
| carrying | `DATE`'s documented alternative: excess days spill into the following month |
| truncated toward zero | The documented conversion applied to a non-integer `months` |

## Sources

- Microsoft 365 support, **EDATE function** —
  <https://support.microsoft.com/en-us/office/edate-function-3c920eb2-6e66-44e7-a1f5-753ae47ee4f5>.
  Source of the syntax, the `months` truncation rule, and the `#VALUE!` condition; it documents no
  end-of-month rule and no result-out-of-range condition.
- Microsoft 365 support, **EOMONTH function** —
  <https://support.microsoft.com/en-us/office/eomonth-function-7314ffa1-2bc9-4005-9d66-f49db127d628>.
  Cited here for the contrasting documented error values.
- [FUNC.DATE](FUNC.DATE.md) — serial model and the documented day-carry rule.
- Handbook chapter [06 claim language](../model/06-claim-language.md), rule 5 (documentation and
  behaviour shown side by side rather than harmonized).
- `data/functions/FUNC.EDATE.json`, `data/presence/FUNC.EDATE.json`,
  `data/battery/FUNC.EDATE.json`.
