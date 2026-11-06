---
schema: efh.function-page/v1
function_id: FUNC.HOUR
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: HOUR function"
    locator: "https://support.microsoft.com/en-us/office/hour-function-a3afa879-86cb-4339-b1b5-2dd2d7310ac7"
    role: "documented behaviour of the serial_number argument and the 0-23 return range"
episodes: []
body_sections:
  - "What it does"
  - "Try it"
  - "Quirks"
  - "How to implement"
  - "Version differences"
  - "Alternatives"
  - "Sources"
family: date_parts_family
role_in_family: "Extracts the hour component from the fractional part of a serial, as an integer
  0 through 23."
---

# HOUR

## What it does

`HOUR(serial_number)` returns the hour of the day, as an integer from 0 through 23.
Excel represents a day by one unit, so 0.5 is noon. A full date-time serial can be supplied;
this function extracts a clock component rather than an elapsed duration.

The argument may be a serial number, a time produced by another formula, or time text that the
locale can parse. Text parsing and component extraction are separate operations.

## Try it

Start with `=HOUR(0.5)`. Compare `=HOUR(TIME(12,34,56))`, `=MINUTE(TIME(12,34,56))` and
`=SECOND(TIME(12,34,56))` to see the three components of the same time.

## Quirks

A day has 24 hours and each hour and minute has 60 subdivisions. The component wraps; for total
elapsed hours, minutes or seconds, use arithmetic on the duration rather than this extractor.

On Excel 16.0.20527.20088 for Windows, `HOUR(0.999999)` returns 0, carrying a time just before midnight into the next day.

The documentation describes the returned component, but does not specify every binary64 boundary
or intermediate rounding. A simple floor of a scaled fractional day is therefore an incomplete
compatibility specification. Invalid date/time input and negative serial handling also need the
target workbook's date-system rules.

## How to implement

Separate argument coercion and range checks from the clock calculation. Preserve the numeric
serial as binary64. OneCalc's compatibility candidate first adds `0.5/86400` to the serial,
then takes its fractional day. It multiplies by 24, takes the integer hour, multiplies the
remaining fraction by 60, takes the integer minute, and multiplies the next remainder by 60 to
obtain the integer second, reduced modulo 60.

That operation order matters near a boundary. Computing a single rounded total-second integer
and splitting it is a sensible mathematical design, but is not established as interchangeable
with this sequence. Test a value and its adjacent binary64 neighbours around each second, minute,
hour and midnight transition. Do not let date formatting decide the numeric result.

## Version differences

The observations below are scoped to their captured Excel build, platform and input expression.
No comparison across builds establishes a change in the component-rounding rule here. The
1900/1904 date system is workbook context, separate from the Excel product version.

## Alternatives

Use [TIME](FUNC.TIME.md) to construct a clock value and [TIMEVALUE](FUNC.TIMEVALUE.md) to parse
text. For elapsed time, multiplying a duration by 24, 1440 or 86400 retains the full interval.
[TEXT](FUNC.TEXT.md) or a cell format controls presentation without extracting a numeric component.

## Sources

- [Microsoft: HOUR function](https://support.microsoft.com/en-us/excel/functions/hour-function).
