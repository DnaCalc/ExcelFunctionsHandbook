---
schema: efh.function-page/v1
function_id: FUNC.ISOWEEKNUM
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: ISOWEEKNUM function"
    locator: "https://support.microsoft.com/en-us/office/isoweeknum-function-1c2d0afe-d25b-4ab1-8894-8d0520e90e0e"
    role: "documented syntax and error conditions"
  - work: "Microsoft 365 support: WEEKNUM function"
    locator: "https://support.microsoft.com/en-us/office/weeknum-function-e5c43a03-b4ab-426c-b411-b18c13c75340"
    role: "Microsoft's own statement of the ISO 8601 week-1 rule, as WEEKNUM's System 2"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
family: date_week_family
role_in_family: "The ISO 8601 week number, as a dedicated function rather than a WEEKNUM option."
---

## What it computes

`ISOWEEKNUM` returns the ISO 8601 week number of the week that a date falls in.

Microsoft states the ISO rule on the [WEEKNUM](FUNC.WEEKNUM.md) page, as that function's System 2:
"The week containing the first Thursday of the year is the first week of the year, and is numbered
as week 1. This system is the methodology specified in ISO 8601, which is commonly known as the
European week numbering system."

The full ISO definition has three parts, all of which follow from that rule and are worth stating
because they are what makes the function non-trivial:

1. **Weeks always start on Monday and always contain seven days.** There are no stub weeks.
2. **Week 1 is the week containing the year's first Thursday** — equivalently, the week containing
   4 January, equivalently the first week with the majority of its days in the new year.
3. **A calendar year has 52 or 53 ISO weeks**, and the days at either end of the calendar year can
   belong to the neighbouring ISO year. 31 December 2019 is ISO week 1 (of 2020); 1 January 2021 is
   ISO week 53 (of 2020).

Point 3 is the one that bites. `ISOWEEKNUM` returns a bare week number with **no ISO year
attached**, and Excel provides no function that returns the ISO year. A worksheet that pairs
`ISOWEEKNUM(d)` with `YEAR(d)` produces a wrong label for exactly the boundary days ISO exists to
handle — reporting "week 53 of 2021" for 1 January 2021, which no ISO calendar contains. The
correct ISO year has to be derived by hand, typically via the Thursday of the same week:
`YEAR(d - WEEKDAY(d, 3) + 3)`.

## Arguments

`ISOWEEKNUM(date)` — one required argument, the date whose ISO week number is wanted.

There is no `return_type`: ISO 8601 admits no variants, which is the point of having a separate
function. Argument coercion is the ordinary scalar behaviour described in
[coercion and lifting](../model/02-coercion-and-lifting.md); as everywhere in this category,
Microsoft's guidance is to supply serials from [DATE](FUNC.DATE.md) or another formula rather than
text.

`ISOWEEKNUM` is one of the newer members of the category — it does not appear in the oldest
function sets, and workbooks that must open in very old Excel versions use a `WEEKNUM(d, 21)` or a
hand-rolled expression instead. The Handbook's version-history data, rendered elsewhere on this
page, is the authority on exactly which release introduced it; this prose does not restate it.

## Result and edge cases

Returns a `Number` in 1…53.

- **The boundary days are the whole story.** Late December and early January are where the ISO week
  number stops agreeing with intuition, and where implementations diverge.
- **53-week years.** A year has 53 ISO weeks when it starts on a Thursday, or is a leap year
  starting on a Wednesday. Any suite for this function must contain several of them.
- **The missing ISO year.** As above: the result is incomplete information at year boundaries, by
  design of the function rather than by defect.
- **Early 1900.** ISO week numbers are computed from weekday positions, so the region below serial
  61 inherits the phase problem Microsoft documents for [WEEKDAY](FUNC.WEEKDAY.md). The
  reference-engine battery on this page answers the serial-0 probe with a week number in the 50s,
  which is what a "belongs to the previous ISO year" reading would give. Excel's answer there is
  unchecked.

## Errors

As documented for this function:

- `#NUM!` when the argument is out of range for the current date-base value.
- `#VALUE!` when the argument is not a valid date — the ordinary coercion outcome.

An error value passed in propagates.

## Relationships

- **[WEEKNUM](FUNC.WEEKNUM.md) with `return_type` 21** — the same definition reached through the
  general function. Whether the two agree on every input is an equivalence that is widely assumed,
  easy to test, and **not verified here**.
- **[WEEKDAY](FUNC.WEEKDAY.md) with `return_type` 3** — Monday-zero-based day numbering, the
  natural building block for hand-rolled ISO arithmetic and for the ISO-year expression above.
- **[YEAR](FUNC.YEAR.md)** — the trap: calendar year is not ISO year.
- **Confusable.** `ISOWEEKNUM` and `WEEKNUM` with its default `return_type` disagree for a large
  share of January and December dates, and the difference is routinely reported as a bug in one or
  the other.

## Notes for implementers

1. **Compute via the Thursday.** Shift the date to the Thursday of its ISO week
   (`d − isoweekday(d) + 4`, with Monday = 1), take that Thursday's calendar year and its
   day-of-year, and the week number is `⌈doy/7⌉`. This formulation has no year-boundary special
   cases at all, which is why it is the one to use.
2. **Do not compute week numbers by dividing the day-of-year.** Every naive variant needs
   corrections at both ends of the year, and the corrections are where the bugs live.
3. **Share the weekday kernel** with `WEEKDAY` and `WEEKNUM`, so the 1900 phase behaviour is
   identical across the three by construction.
4. **Resist the urge to return an ISO year.** The function's contract is a bare number; adding a
   year would be a different function.

## What has not been checked

No Handbook vector suite exists for `ISOWEEKNUM`, and no Excel-comparison evidence is recorded. The
battery on this page is the reference engine's own answers, obtained without Excel. What to probe
first, and why:

- **28 December through 4 January for every year 1990–2040.** ISO week numbering is correct or
  incorrect entirely at these boundaries; a suite that omits them proves nothing, and one that
  includes them proves almost everything. Roughly 400 rows.
- **Years with 53 ISO weeks** — 2004, 2009, 2015, 2020, 2026 among others — checked at both ends.
- **`ISOWEEKNUM(d)` against `WEEKNUM(d, 21)`** across the same corpus, to settle the assumed
  equivalence.
- **The January–February 1900 region**, where the documented weekday phase break sits.
- **The upper range boundary**, serials 2958465 and 2958466.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| ISO 8601 week | Seven days beginning Monday; week 1 contains the year's first Thursday |
| ISO year | The year an ISO week belongs to; not exposed by any Excel function |
| 53-week year | A calendar year containing 53 ISO weeks |
| the Thursday method | Computing the week number from the Thursday of the same week, avoiding boundary cases |

## Sources

- Microsoft 365 support, **ISOWEEKNUM function** —
  <https://support.microsoft.com/en-us/office/isoweeknum-function-1c2d0afe-d25b-4ab1-8894-8d0520e90e0e>.
- Microsoft 365 support, **WEEKNUM function** —
  <https://support.microsoft.com/en-us/office/weeknum-function-e5c43a03-b4ab-426c-b411-b18c13c75340>.
  Source of the quoted ISO 8601 / System 2 definition.
- [FUNC.WEEKDAY](FUNC.WEEKDAY.md) — the weekday kernel and Microsoft's documented pre-March-1900
  phase problem.
- [FUNC.DATE](FUNC.DATE.md) — the serial-number model.
- `data/functions/FUNC.ISOWEEKNUM.json`, `data/presence/FUNC.ISOWEEKNUM.json`,
  `data/battery/FUNC.ISOWEEKNUM.json`.
