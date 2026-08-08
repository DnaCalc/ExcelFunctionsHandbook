---
schema: efh.function-page/v1
function_id: FUNC.EOMONTH
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: EOMONTH function"
    locator: "https://support.microsoft.com/en-us/office/eomonth-function-7314ffa1-2bc9-4005-9d66-f49db127d628"
    role: "documented syntax, months truncation rule, and both #NUM! conditions"
  - work: "Microsoft 365 support: EDATE function"
    locator: "https://support.microsoft.com/en-us/office/edate-function-3c920eb2-6e66-44e7-a1f5-753ae47ee4f5"
    role: "the sibling function whose documented error value for the same failure differs"
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
role_in_family: "Month-offset arithmetic landing on the last day of the target month; the
  end-of-month counterpart to EDATE."
---

## What it computes

`EOMONTH` offsets a date by a whole number of months and returns the serial of the **last day of
the resulting month**. The day-of-month of `start_date` is discarded entirely: only its year and
month matter.

Precisely: let `(y, m)` be the year and month of `start_date`; add `months` to `m` with carry into
`y`; return the serial of the last day of that `(y, m)`. `EOMONTH(s, 0)` is therefore "the end of
this month" and `EOMONTH(s, -1)` is "the end of last month", which is why the function turns up in
almost every period-boundary formula in finance.

Two identities follow, and both are used constantly:

- `DAY(EOMONTH(s, 0))` is the number of days in `s`'s month.
- `EOMONTH(s, -1) + 1` is the first day of `s`'s month.

The second is the idiomatic "start of month" expression precisely because Excel has no `BOMONTH`.

## Arguments

`EOMONTH(start_date, months)` — both required.

**start_date** — "a date that represents the starting date". Microsoft repeats the category
guidance that dates "should be entered by using the DATE function, or as results of other formulas
or functions" rather than as text.

**months** — "the number of months before or after start_date. A positive value for months yields a
future date; a negative value yields a past date." The page states the conversion rule explicitly:
**"If months is not an integer, it is truncated."** Truncation is toward zero on the ordinary
reading of the word, which makes the negative fractional case (`months = -1.5`) the one worth
testing.

The argument that is misunderstood is `start_date`, and specifically the belief that its day-of-
month matters. It does not: `EOMONTH(DATE(2024,3,1), 0)` and `EOMONTH(DATE(2024,3,31), 0)` are the
same value.

## Result and edge cases

Returns a `Number`: a whole date serial with no time component.

- **February.** `EOMONTH` is one of the few places in the category where correct leap-year handling
  is directly observable in ordinary use — and where the 1900 artefact is directly reachable.
  `EOMONTH` of a February 1900 date asks whether the implementation believes in the phantom 29
  February; see [FUNC.DATE](FUNC.DATE.md).
- **`months = 0`** does not return `start_date`; it returns the month end. Any time component of
  `start_date` is not carried into the result.
- **Result out of range** is documented as an error, unlike on the `EDATE` page — see below.
- **Very large `months`** simply carries through years; the range check is on the result.

## Errors

As documented on the Microsoft `EOMONTH` page:

- `#NUM!` when `start_date` is not a valid date.
- `#NUM!` when `start_date` plus `months` yields an invalid date.

Note the discrepancy with the sibling: [EDATE](FUNC.EDATE.md)'s page documents **`#VALUE!`** for an
invalid `start_date`, and documents no result-out-of-range condition at all. Two functions of the
same shape, taking the same arguments, documented with different error values for the same failure.
This Handbook does not silently pick a winner: the divergence is stated, and resolving it against a
running Excel is listed below as an unrun experiment. See
[claim language](../model/06-claim-language.md), rule 5.

`#VALUE!` from an argument that will not coerce, and propagation of an error argument, are the
ordinary behaviours described in
[coercion and lifting](../model/02-coercion-and-lifting.md).

## Relationships

- **[EDATE](FUNC.EDATE.md)** — the same month arithmetic preserving the day-of-month rather than
  jumping to the month end. The two coincide when `start_date` is already a month end and the
  target month is no shorter.
- **[DATE](FUNC.DATE.md)** — the hand-rolled equivalent is `DATE(YEAR(s), MONTH(s)+n+1, 0)`, which
  works by `DATE`'s documented day-underflow rule (day 0 of a month is the last day of the
  previous one). Anyone who has written that expression has been using `DATE` as an `EOMONTH`.
- **[DAY](FUNC.DAY.md)** — `DAY(EOMONTH(s,0))` is the days-in-month idiom.
- **[YEARFRAC](FUNC.YEARFRAC.md)** — shares the end-of-February sensitivity: Microsoft documents
  that `YEARFRAC` "may return an incorrect result when using the US (NASD) 30/360 basis, and the
  start_date is the last day in February", which is exactly the kind of date `EOMONTH` produces.
  The two functions are frequently composed, so the interaction is real.
- **[WORKDAY](FUNC.WORKDAY.md)** — month ends are frequently adjusted to business days afterwards.

## Notes for implementers

1. **Compute in (year, month) space and take the month length last.** Adding `months` to a
   zero-based month index with floor-division carry, then looking up the target month's length, is
   both simpler and less error-prone than any day-count approach.
2. **February 1900 is the special case.** If the implementation models the leap-year artefact as a
   coordinate shift (the recommendation on [FUNC.DAY](FUNC.DAY.md)), then the "length of February
   1900" question has to be answered deliberately: the real calendar says 28, Excel's serial line
   contains a 29th.
3. **Truncate `months` toward zero**, per the documented word.
4. **Do not share an error path with `EDATE`** until the documented `#NUM!`/`#VALUE!` discrepancy is
   resolved by observation.

## What has not been checked

No Handbook vector suite exists for `EOMONTH`, and no Excel-comparison evidence is recorded. The
battery on this page is the reference engine's own answers; no Excel was involved. Probes worth
running, and what each settles:

- **`EOMONTH(DATE(1900,2,1), 0)`** — does Excel report 28 or 29 February 1900? The single most
  diagnostic input for this function, because it forces the leap-year artefact into an
  ordinary-looking answer.
- **`EOMONTH("not a date", 1)` beside `EDATE("not a date", 1)`** — resolves the documented error
  discrepancy between the sibling pages in one experiment.
- **`EOMONTH(s, -1.5)` and `EOMONTH(s, 1.5)`** — confirms truncation toward zero.
- **`EOMONTH(1, -1)`** — an offset landing below the representable range, where the documented
  second `#NUM!` should fire.
- **Leap-year Februaries generally** — 2000 (a leap year despite being a century year) and 2100
  (not a leap year), which Microsoft's leap-year article says Excel handles correctly.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| month end | The last calendar day of a given year-month |
| month offset | Moving by whole months with carry into the year |
| days-in-month idiom | `DAY(EOMONTH(s,0))` |
| day-underflow rule | `DATE`'s documented behaviour where day 0 denotes the previous month's last day |

## Sources

- Microsoft 365 support, **EOMONTH function** —
  <https://support.microsoft.com/en-us/office/eomonth-function-7314ffa1-2bc9-4005-9d66-f49db127d628>.
  Source of the syntax, the `months` truncation rule and both documented `#NUM!` conditions.
- Microsoft 365 support, **EDATE function** —
  <https://support.microsoft.com/en-us/office/edate-function-3c920eb2-6e66-44e7-a1f5-753ae47ee4f5>.
  Cited for the contrasting documented error value.
- Microsoft 365 support, **YEARFRAC function** —
  <https://support.microsoft.com/en-us/office/yearfrac-function-3844141e-c76d-4143-82b6-208454ddc6a8>.
  Source of the last-day-of-February caveat quoted above.
- [FUNC.DATE](FUNC.DATE.md) — serial model, the day-underflow rule and the leap-year artefact.
- `data/functions/FUNC.EOMONTH.json`, `data/presence/FUNC.EOMONTH.json`,
  `data/battery/FUNC.EOMONTH.json`.
