---
schema: efh.function-page/v1
function_id: FUNC.DATE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: DATE function"
    locator: "https://support.microsoft.com/en-us/office/date-function-e36c0c8c-4104-49da-ab83-82328b832349"
    role: "documented argument rules for year, month and day, including out-of-range rollover"
  - work: "Microsoft Learn: Excel incorrectly assumes that the year 1900 is a leap year"
    locator: "https://learn.microsoft.com/en-us/office/troubleshoot/excel/wrongly-assumes-1900-is-leap-year"
    role: "documented origin and consequences of the phantom 29 February 1900"
  - work: "Microsoft 365 support: Date systems in Excel"
    locator: "https://support.microsoft.com/en-us/office/date-systems-in-excel-e7fe7167-48a9-4b96-bb53-5612a800b487"
    role: "documented 1900 vs 1904 date systems and the 1,462-day offset"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
family: date_fn
role_in_family: "The constructor: builds a date serial from a year/month/day triple, and is the
  page that carries this Handbook's account of the serial-number model itself."
---

## What it computes

`DATE` is the constructor of the date category. It takes three numbers — a year, a month and a
day — and returns the **date serial number** that denotes the calendar day those three numbers
name. Everything else in the category either consumes a serial (`YEAR`, `MONTH`, `WEEKDAY`,
`EOMONTH`) or produces one (`TODAY`, `EDATE`, `WORKDAY`). `DATE` is where the mapping is defined.

The important thing to understand is that `DATE` is **not** a validator. It is a normalizing
constructor: it does not reject 14 as a month or 35 as a day, it carries them. The documented
rule is arithmetic, not calendrical. Given `(y, m, d)`, Excel first resolves the year–month pair
by ordinary integer division — month 14 of 2008 means month 2 of 2009, month −3 of 2008 means
month 9 of 2007 — and then counts `d` days from the day before the first of that resolved month.
So `DATE(2008,1,35)` is 4 February 2008 and `DATE(2008,1,-15)` is 16 December 2007, both of which
Microsoft's page gives as worked examples.

Read that way, `DATE(y, m, d)` is the serial of

> the first day of month `m` of year `y`, resolved by carrying month overflow into the year,
> then advanced by `d − 1` days.

That single sentence explains all four documented rollover cases at once, and it is why
`DATE(2008,2,30)` quietly becomes 1 March 2008 rather than an error.

### The serial-number model

A date serial is a plain `Number` (see [the value universe](../model/01-value-universe.md)); the
date-ness lives in the cell format, not in the value. Under the default **1900 date system**,
Microsoft documents that 1 January 1900 is serial 1 and 1 January 2008 is serial 39448. Those two
anchors, together with the leap-year artefact below, pin the whole mapping:

| Serial | Calendar day |
|---|---|
| 0 | no real day — displays as "0 January 1900" |
| 1 … 59 | 1 January … 28 February 1900 (serial = days after 31 December 1899) |
| 60 | **29 February 1900 — a day that never existed** |
| 61 … 2958465 | 1 March 1900 … 31 December 9999 (serial = days after 30 December 1899) |

The fractional part of a serial carries time of day: 0.5 is noon. `DATE` returns a whole number,
so its result has no time component. See [TIME](FUNC.TIME.md) for the other half of that model.

### The 1900 leap-year artefact

1900 was not a leap year — it is divisible by 100 and not by 400 — but Excel's serial line
contains a 29 February 1900 anyway. Microsoft documents both the fact and the reason: Lotus 1-2-3
made the assumption first, and Multiplan and Excel adopted it so that serial numbers would be
interchangeable between the programs. Microsoft's article is explicit that the behaviour is
retained deliberately and that only the year 1900 is affected; century years such as 2100 are
handled correctly.

Two consequences bite in this category:

1. **Every serial from 61 upwards is one greater than the true day count from 1 January 1900.**
   This is invisible in day-difference arithmetic as long as both endpoints are on the same side
   of the discontinuity, and wrong by one when they straddle it.
2. **Weekday alignment before 1 March 1900 is off by one.** Microsoft's own article names this as
   the single residual problem: `WEEKDAY` "returns incorrect values for dates before March 1,
   1900". See [WEEKDAY](FUNC.WEEKDAY.md).

Whether `DATE(1900,2,29)` returns serial 60 rather than rolling into 1 March, and what
`DATE(1900,2,30)` does, are exactly the questions the artefact makes interesting. Neither has
been checked against Excel for this Handbook.

### The 1900 and 1904 date systems

A workbook carries a date-system flag. Microsoft documents the **1900 date system** (the default)
and the **1904 date system** (the default in earlier versions of Excel for Mac), and states that
the difference between the two is 1,462 days — "four years and one day (including one leap day)" —
so that the same calendar day has a serial 1,462 higher in the 1900 system than in the 1904
system. Microsoft's worked example is 5 July 2011: serial 40729 in the 1900 system, 39267 in the
1904 system.

`DATE` is the function where this flag is consumed rather than merely observed: the same
`DATE(2011,7,5)` call produces a different number in the two systems. Note that the 1904 system
has no leap-year artefact, because 1904 genuinely was a leap year.

One honest inconsistency in the documentation, worth naming because implementers trip on it:
Microsoft's date-systems page describes a serial as "the number of days elapsed since January 1,
1900", while the `DATE`, `WEEKDAY` and `WEEKNUM` pages state that 1 January 1900 *is* serial 1.
Those two statements do not reconcile — "days elapsed since 1 January 1900" would make that day
serial 0. The serial-1 anchor is the one consistent with the documented 39448 for 1 January 2008,
so it is the one this Handbook uses.

## Arguments

`DATE(year, month, day)` — all three required, no optional argument, no repeats.

**year** — Microsoft documents a three-branch rule, and it is the argument that surprises people:

| Value of `year` | Documented interpretation |
|---|---|
| 0 … 1899 | 1900 is **added**: `DATE(108,1,2)` is 2 January 2008 |
| 1900 … 9999 | used as the year directly |
| < 0 or ≥ 10000 | `#NUM!` |

The documentation also says Excel "interprets the year argument according to the date system your
computer is using", which is the hook for 1904-system workbooks. The practical advice that follows
from the table: always pass a four-digit year, because `DATE(24,1,1)` is 1 January 1924, not 2024.

**month** — an integer, documented as "positive or negative", nominally 1–12. Out-of-range values
carry into the year as described above.

**day** — an integer, documented as "positive or negative", nominally 1–31. Out-of-range values
carry into the month, and through the month into the year.

Non-integer arguments are the commonly misunderstood case that the documentation does not settle
on this page. `YEARFRAC`'s page states the truncation rule for *its* arguments explicitly; the
`DATE` page does not, and this Handbook has not checked whether `DATE(2008.9, 1.9, 1.9)`
truncates, rounds, or does something else.

## Result and edge cases

Returns a `Number`: a whole date serial. The cell will usually display it as a date, because Excel
applies a date format to the result — that is host-side adaptation, not part of the value (see
[the call pipeline](../model/03-call-pipeline.md), stage 4).

Argument coercion follows the ordinary scalar rules of
[coercion and lifting](../model/02-coercion-and-lifting.md): numeric text converts, logicals
convert to 1 and 0, error inputs propagate. Nothing on this page overrides those.

The reference engine's own answers for the standard probe inputs — zero, negative arguments, empty
text, logicals, arrays — are rendered by the battery panel above and are OxFunc's answers, not
Excel's.

## Errors

As documented on the Microsoft page linked below:

- `#NUM!` when `year` is negative or 10000 or greater.

Beyond that single documented condition, the errors reachable from `DATE` are the ordinary ones
that any numeric-argument function can produce: `#VALUE!` from an argument that cannot convert to
a number, and any error value passed in as an argument propagating out. Microsoft's page does not
enumerate a `#NUM!` for a *resulting* date outside 1 January 1900 – 31 December 9999 — for example
`DATE(1900,1,-5)`, which resolves below serial 0 — and this Handbook has not checked what Excel
returns there.

## Relationships

- **Inverses.** [YEAR](FUNC.YEAR.md), [MONTH](FUNC.MONTH.md) and [DAY](FUNC.DAY.md) decompose a
  serial back into the three components `DATE` composes. `DATE(YEAR(s), MONTH(s), DAY(s))` should
  return the whole-day part of `s` for any in-range serial; whether it does across the leap-year
  discontinuity is a natural metamorphic probe nobody has run here.
- **Sibling constructor.** [TIME](FUNC.TIME.md) builds the fractional part the same way `DATE`
  builds the integer part, and with the same normalizing (rather than validating) philosophy.
- **Text parser.** [DATEVALUE](FUNC.DATEVALUE.md) reaches the same serials from text instead of
  from three numbers, and is locale-dependent where `DATE` is not.
- **Month arithmetic.** [EDATE](FUNC.EDATE.md) and [EOMONTH](FUNC.EOMONTH.md) offset by whole
  months, which is *not* what `DATE`'s month rollover does. `DATE(2008,1,31)` rewritten as
  `DATE(2008,2,31)` gives 2 March 2008, because `DATE` carries the excess days forward; `EDATE`
  is widely understood to clamp instead, though its Microsoft page does not state the clamping
  rule and this Handbook has not verified it. This is the most common confusion in the category.
- **Confusable.** `DATEVALUE` and `DATE` are frequently interchanged in advice; so are `DATE` and
  the `TEXT` function, which formats rather than constructs.

## Notes for implementers

1. **Do the month arithmetic before the day arithmetic, and do it in integers.** Resolve
   `(y, m)` to a normalized year and 1–12 month with a floor-division carry (not truncating
   division — `month = -3` must go *backwards*), then add `day − 1` days. Getting the negative
   branch right is the whole trick.
2. **The leap-year artefact is a coordinate shift, not a calendar rule.** The cleanest
   implementation computes a proper proleptic Gregorian day number and then adds 1 for days on or
   after 1 March 1900. Trying to make a real calendar library believe in 29 February 1900 is the
   route to subtle bugs.
3. **The date system is workbook state, not function state.** A kernel that hard-codes the 1900
   origin cannot serve a 1904 workbook. Model the origin as an input to the function, sourced from
   the execution context.
4. **`year` in 0…1899 is add-1900, not a two-digit-window rule.** It is not the sliding pivot
   convention used elsewhere in date parsing; 99 means 1999 and 5 means 1905.

## What has not been checked

There is no Handbook vector suite for `DATE`, and no recorded Excel-comparison evidence for it.
The reference-engine battery rendered on this page is OxFunc answering its own probes; no Excel
was involved. Nobody has checked any of the following against Excel, and each would settle a
distinct question:

- **`DATE(1900,2,29)` and `DATE(1900,2,30)`** — does the constructor produce serial 60, and does
  the phantom day absorb one day of overflow? This is the single most diagnostic input for the
  leap-year artefact.
- **`DATE(1900,1,0)`, `DATE(1900,1,-5)`** — what happens when the construction lands at or below
  serial 0. Whether the "0 January 1900" phantom is reachable through `DATE` is unknown here.
- **Non-integer arguments** — `DATE(2008.9, 1.9, 1.9)`. Truncation is the plausible rule by
  analogy with `YEARFRAC`'s documented behaviour, but analogy is not evidence.
- **The 1904 system** — every claim on this page about 1904 comes from Microsoft's date-systems
  page; no 1904 workbook has been probed for this Handbook.
- **Very large and very small `year`/`month`/`day`** — the documented `#NUM!` boundary is stated
  for `year` only. Where the *composed* result leaves the representable range, the behaviour is
  undocumented on that page and unverified here.

A suite for `DATE` would be cheap and high-value, because the function is a pure integer kernel
with no floating-point subtlety: a few thousand triples covering the four rollover branches, the
serial 0–61 window, and the year-interpretation table would characterize it almost completely.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| date serial | The `Number` Excel uses to denote a calendar day; integer part is the day, fractional part the time |
| 1900 date system | The default workbook date system; 1 January 1900 is serial 1 |
| 1904 date system | The alternative system, default in earlier Excel for Mac; serials are 1,462 lower |
| the leap-year artefact | Serial 60 denoting 29 February 1900, a day that did not exist |
| normalizing constructor | A constructor that carries out-of-range components into the next unit rather than rejecting them |

## Sources

- Microsoft 365 support, **DATE function** —
  <https://support.microsoft.com/en-us/office/date-function-e36c0c8c-4104-49da-ab83-82328b832349>.
  Source of the year/month/day rules, the rollover examples, and the `#NUM!` condition.
- Microsoft Learn, **Excel incorrectly assumes that the year 1900 is a leap year** —
  <https://learn.microsoft.com/en-us/office/troubleshoot/excel/wrongly-assumes-1900-is-leap-year>.
  Source of the Lotus 1-2-3 compatibility explanation and of the statement that `WEEKDAY` returns
  incorrect values before 1 March 1900.
- Microsoft 365 support, **Date systems in Excel** —
  <https://support.microsoft.com/en-us/office/date-systems-in-excel-e7fe7167-48a9-4b96-bb53-5612a800b487>.
  Source of the 1900/1904 definitions, the 1,462-day offset, and the 5 July 2011 example.
- Handbook call-model chapters
  [01 value universe](../model/01-value-universe.md),
  [02 coercion and lifting](../model/02-coercion-and-lifting.md),
  [03 call pipeline](../model/03-call-pipeline.md) — for the value kinds, coercion rules and
  publication stage this page refers to rather than restates.
- `data/functions/FUNC.DATE.json`, `data/presence/FUNC.DATE.json`, `data/battery/FUNC.DATE.json` —
  the projected identity, implementation-presence and reference-engine battery data rendered
  elsewhere on this page.
