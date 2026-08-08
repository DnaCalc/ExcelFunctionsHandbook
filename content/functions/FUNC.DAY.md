---
schema: efh.function-page/v1
function_id: FUNC.DAY
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: DAY function"
    locator: "https://support.microsoft.com/en-us/office/day-function-8a7d1cbb-6c7d-4ba1-8aea-25c134d03101"
    role: "documented behaviour of the serial_number argument and the 1-31 return range"
  - work: "Microsoft Learn: Excel incorrectly assumes that the year 1900 is a leap year"
    locator: "https://learn.microsoft.com/en-us/office/troubleshoot/excel/wrongly-assumes-1900-is-leap-year"
    role: "the phantom 29 February 1900 that makes serial 60 the diagnostic input for this function"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
family: date_parts_family
role_in_family: "Extracts the day-of-month component from a date serial; the decomposer on which
  the 1900 leap-year artefact is directly observable."
---

## What it computes

`DAY` projects a date serial onto its day-of-month component: floor the serial, map `⌊serial⌋` to
a calendar day under the workbook's date system (see [FUNC.DATE](FUNC.DATE.md)), and return that
day's day-of-month as an integer, documented as 1 through 31.

Of the three decomposers this is the one where Excel's serial line stops agreeing with the
calendar. Serial 60 denotes 29 February 1900, a date that did not exist; the day-of-month it
reports is 29 in a February that had 28 days. Serial 0 denotes a "0 January 1900" that is not a
day at all. So the documented 1–31 range is the range for *real* days; the phantoms sit outside
the calendar the range describes.

## Arguments

`DAY(serial_number)` — one required argument, the date whose day-of-month is wanted.

Microsoft's category-wide guidance applies: pass a serial from [DATE](FUNC.DATE.md), from another
formula, or from a cell already holding a date, rather than text, whose interpretation depends on
the locale's date order. Argument coercion is the ordinary scalar behaviour described in
[coercion and lifting](../model/02-coercion-and-lifting.md).

Note that `DAY` takes a *date*, not a day count. `DAY(45)` is the day-of-month of 14 February
1900, not the number 45 reduced somehow — a confusion that shows up whenever someone reaches for
`DAY` when they wanted [DAYS](FUNC.DAYS.md).

## Result and edge cases

Returns a `Number`, documented as 1…31 for real calendar days.

- **Fractional serials** are floored; time of day is discarded.
- **Serial 60** is the diagnostic input for this whole category. If the implementation carries the
  phantom leap day, `DAY(60)` is 29 and `DAY(59)`/`DAY(61)` are 28 and 1. If instead it delegates
  to a real calendar without the shift, the answers slide by one. Nothing else in the date
  category exposes the artefact this directly.
- **Serial 0** is the other phantom. The reference-engine battery row on this page returns 0 for
  it — a value outside the documented 1–31 range, which is exactly what "0 January 1900" would
  imply. Whether Excel agrees has not been checked here.
- **Very large serials.** The battery's maximum-double row returns a number rather than an error;
  see below.

## Errors

As documented for this category:

- `#NUM!` when `serial_number` is out of range for the workbook's date-base value; negative
  serials are the unambiguous case.
- `#VALUE!` when the argument will not convert to a number — ordinary coercion, not a `DAY` rule.
- An error argument propagates.

## Relationships

- **[YEAR](FUNC.YEAR.md), [MONTH](FUNC.MONTH.md)** — the sibling projections; together they invert
  [DATE](FUNC.DATE.md).
- **[DAYS](FUNC.DAYS.md)** — the confusable. `DAY` reads a component of one date; `DAYS` measures
  the interval between two. The names are one character apart and the meanings are unrelated.
- **[EOMONTH](FUNC.EOMONTH.md)** — `DAY(EOMONTH(s,0))` is the idiomatic "how many days are in this
  month" formula, and is the standard way people discover the 1900 artefact by accident:
  `DAY(EOMONTH(DATE(1900,2,1),0))` asks a question with two possible right answers.
- **[WEEKDAY](FUNC.WEEKDAY.md)** — day *of week* rather than day *of month*, and the function
  Microsoft's leap-year article names as the one that returns incorrect values before 1 March
  1900.

## Notes for implementers

1. **Implement the artefact as a coordinate shift, once.** Convert serial to a proleptic Gregorian
   day number by subtracting 1 for serials ≥ 61, do real calendar arithmetic, and special-case
   serials 0 and 60 on the way out. Trying to teach a calendar library that 1900 was a leap year
   corrupts every other computation that library performs.
2. **Share the serial→(y,m,d) routine** with `YEAR` and `MONTH`; three separate derivations will
   eventually disagree at serial 60.
3. **Decide what `DAY(60)` returns and write it down.** It is the one value in this function that
   a reasonable engineer could get "right" in two different ways, and it is therefore the first
   thing a compatibility test should pin.

## What has not been checked

No Handbook vector suite exists for `DAY`, and no Excel-comparison evidence is recorded. The
battery on this page is the reference engine's own answers; no Excel was involved. The probes that
matter, in order:

- **`DAY(59)`, `DAY(60)`, `DAY(61)`** — the phantom-leap-day window. This is the highest-value
  three-row experiment in the entire date category, because the answer tells you how the
  implementation models the artefact and therefore predicts its behaviour everywhere else before
  1 March 1900.
- **`DAY(0)`** — whether Excel also reports 0, i.e. whether the "0 January 1900" phantom is
  observable through this function.
- **`DAY(2958465)` and `DAY(2958466)`** — the documented ceiling and the first serial past it.
- **A maximum-double argument** — the reference engine returns a number; if Excel returns `#NUM!`
  that is a genuine divergence to record.
- **A 1904-system workbook**, in which serial 60 is an ordinary day and the artefact is absent
  entirely.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| date serial | The `Number` denoting a calendar day; see [FUNC.DATE](FUNC.DATE.md) |
| phantom day | A serial denoting no real calendar day: 0 ("0 January 1900") and 60 (29 February 1900) |
| coordinate shift | Implementing the artefact by offsetting serials ≥ 61, rather than by changing the calendar |

## Sources

- Microsoft 365 support, **DAY function** —
  <https://support.microsoft.com/en-us/office/day-function-8a7d1cbb-6c7d-4ba1-8aea-25c134d03101>.
- Microsoft Learn, **Excel incorrectly assumes that the year 1900 is a leap year** —
  <https://learn.microsoft.com/en-us/office/troubleshoot/excel/wrongly-assumes-1900-is-leap-year>.
- [FUNC.DATE](FUNC.DATE.md) — the serial-number model and the 1900/1904 systems.
- Handbook call-model chapter [02 coercion and lifting](../model/02-coercion-and-lifting.md).
- `data/functions/FUNC.DAY.json`, `data/presence/FUNC.DAY.json`, `data/battery/FUNC.DAY.json`.
