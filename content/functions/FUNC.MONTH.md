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
