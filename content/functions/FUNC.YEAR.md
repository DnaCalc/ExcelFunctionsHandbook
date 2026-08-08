---
schema: efh.function-page/v1
function_id: FUNC.YEAR
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: YEAR function"
    locator: "https://support.microsoft.com/en-us/office/year-function-c64f017a-1354-490d-981f-578e8ec8d3b9"
    role: "documented behaviour of the serial_number argument and the returned range"
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
role_in_family: "Extracts the Gregorian year component from a date serial; the coarsest of the
  three calendar decomposers."
---

## What it computes

`YEAR` projects a date serial onto its calendar-year component. Given a serial `s`, it discards
the fractional part, finds the calendar day that `⌊s⌋` denotes under the workbook's date system,
and returns that day's year as an integer in the range 1900–9999 (1904–9999 in a 1904-system
workbook).

It is a projection, not a computation: there is no arithmetic in it beyond the serial-to-calendar
mapping described on [FUNC.DATE](FUNC.DATE.md). Everything interesting about `YEAR` is
consequently a question about that mapping's edges rather than about `YEAR` itself.

## Arguments

`YEAR(serial_number)` — one required argument.

**serial_number** — the date whose year is wanted. Microsoft's documentation is emphatic across
this whole category that dates should be supplied as serials, from `DATE`, from another formula,
or from a cell already holding a date, and *not* as text, because text dates are locale-dependent
and can be misparsed. `YEAR` accepts what the ordinary scalar coercion rules accept (see
[coercion and lifting](../model/02-coercion-and-lifting.md)): numbers, numeric text, logicals.

The commonly misunderstood point is that `serial_number` is a *number*, not a "date object". Any
number in range is a legal argument, including numbers that no user would call a date.

## Result and edge cases

Returns a `Number`: a four-digit year.

- **Fractional serials.** The time-of-day component is discarded. `YEAR(43831.75)` is the same as
  `YEAR(43831)`.
- **Serial 0.** Under the 1900 system, serial 0 is the phantom "0 January 1900". Whether Excel
  reports 1900 for it is an artefact question, not a calendrical one — see below.
- **Serial 60.** The phantom 29 February 1900. `YEAR` returns 1900 for it under any sane reading,
  so it is the *least* diagnostic of the three decomposers here; [DAY](FUNC.DAY.md) is where the
  artefact shows.
- **Very large serials.** The reference-engine battery on this page includes a maximum-double
  argument and returns a number rather than an error. Whether Excel does the same, or enforces the
  31 December 9999 ceiling with `#NUM!`, has not been checked.

## Errors

As documented for this category, and on the page linked below:

- `#NUM!` when `serial_number` is out of range for the workbook's date-base value — negative
  serials being the clear case.
- `#VALUE!` when the argument cannot be converted to a number (ordinary coercion behaviour, not a
  `YEAR`-specific rule).
- An error value passed in propagates.

## Relationships

- **[MONTH](FUNC.MONTH.md), [DAY](FUNC.DAY.md)** — the other two components of the same
  decomposition. Together with `YEAR` they invert [DATE](FUNC.DATE.md).
- **[DATE](FUNC.DATE.md)** — the constructor. `DATE(YEAR(s),MONTH(s),DAY(s))` should return
  `⌊s⌋`; that round trip is the obvious metamorphic test for this family.
- **[YEARFRAC](FUNC.YEARFRAC.md)** — despite the shared prefix, unrelated in kind: `YEARFRAC`
  measures an interval in years, `YEAR` names a year.
- **[DATEDIF](FUNC.DATEDIF.md)** with unit `"Y"` — counts *complete years between* two dates, which
  is not `YEAR(b) − YEAR(a)`. The difference of years and the number of elapsed years disagree
  whenever the anniversary has not yet passed; this is the most frequent age-calculation bug in
  spreadsheets.

## Notes for implementers

1. **Floor, do not truncate.** The fractional part must be removed by flooring so that the
   behaviour is defined the same way for the (error-bound) negative region as for the positive
   one. Truncation and flooring differ exactly where the range check is supposed to fire.
2. **The year is not derivable by division.** No `1900 + s/365.25`-style shortcut is correct; the
   mapping has to go through a real calendar, adjusted for the leap-year artefact.
3. **The date system is context, not a constant.** See [FUNC.DATE](FUNC.DATE.md).

## What has not been checked

No Handbook vector suite exists for `YEAR`, and no Excel-comparison evidence is recorded; the
battery shown on this page is the reference engine's own answer set, produced without Excel. The
first probes worth running:

- **`YEAR(0)` and `YEAR(0.5)`** — does the phantom day 0 report 1900, and does a fractional serial
  below 1 behave the same way?
- **`YEAR(2958465)` and `YEAR(2958466)`** — the documented ceiling (31 December 9999) and the first
  serial past it. This is where a `#NUM!` boundary should be, if there is one.
- **`YEAR(1.7976931348623157E+308)`** — the reference engine returns a number here; if Excel
  returns `#NUM!`, that is a real divergence and worth recording as one.
- **A 1904-system workbook** — every statement on this page about 1904 is inherited from
  Microsoft's date-systems documentation, not observed.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| date serial | The `Number` denoting a calendar day; see [FUNC.DATE](FUNC.DATE.md) |
| decomposer | A function projecting a serial onto one calendar component |
| date-base value | The workbook's date-system origin (1900 or 1904) |

## Sources

- Microsoft 365 support, **YEAR function** —
  <https://support.microsoft.com/en-us/office/year-function-c64f017a-1354-490d-981f-578e8ec8d3b9>.
- [FUNC.DATE](FUNC.DATE.md) — the serial-number model, the leap-year artefact and the 1900/1904
  systems, with their own Microsoft citations.
- Handbook call-model chapter [02 coercion and lifting](../model/02-coercion-and-lifting.md).
- `data/functions/FUNC.YEAR.json`, `data/presence/FUNC.YEAR.json`, `data/battery/FUNC.YEAR.json`.
