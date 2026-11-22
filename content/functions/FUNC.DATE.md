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
  - Notes
  - Page vocabulary
  - Sources
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

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `DATE` that an implementer matching Excel, or a reader explaining an
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

### FN-DATE-002: Excel 16.0.20527 rejects DATE with year >= 10000 even when negative month/day offsets roll back below 10000, differing from build 20430

*Quirk; measured; recorded 2026-11-22.*

Across 22 witness rows in crates/calc-func/witness-differences.tsv, Excel 16.0.20430 rolled
negative month or day offsets back from year 10000 (or 9999.9999999) into 9999 and returned a
valid serial (for example DATE(10000, 0, 0) = 2958434, 9999-11-30), whereas Excel 16.0.20527
rejects year >= 10000 before month/day normalization and returns #NUM! on all 22 rows.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `DATE(10000, 0, 0)` | year `10000.0` (`0x40c3880000000000`); month `0.0` (`0x0000000000000000`); day `0.0` (`0x0000000000000000`) (cells) | `#NUM!` | Excel 16.0.20527, Windows (bitness not recorded), AMD Ryzen 7 4800H with Radeon Graphics (Excel 16.0.20430 returned 2958434, 0x4146923100000000) |
| `DATE(9999.9999999, 0, -1)` | year `9999.9999999` (`0x40c387ffffff2940`); month `0.0` (`0x0000000000000000`); day `-1.0` (`0xbff0000000000000`) (cells) | `#NUM!` | Excel 16.0.20527, Windows (bitness not recorded), AMD Ryzen 7 4800H with Radeon Graphics (Excel 16.0.20430 returned 2958433, 0x4146923080000000) |

Basis (measured): Measured across all 22 DATE differences in
crates/calc-func/witness-differences.tsv comparing Excel 16.0.20430 against Excel 16.0.20527
(dc-iuwct.36; full bank 22,250 of 22,250 matched on 20527).

Evidence:

- DnaCalc `crates/calc-func/witness-differences.tsv`, DATE rows 2 to 23 at `4fb9b728df` — 22 DATE differences between Excel 16.0.20430 and Excel 16.0.20527
- DnaCalc `crates/calc-func/src/ported/date_kernels.rs`, date at `4fb9b728df` — Excel 20527 original_year >= 10000 #NUM! check
- DnaCalc `crates/calc-func/witness-status.tsv`, DATE at `4fb9b728df` — 22,250 of 22,250 full-bank DATE rows matched
- DnaCalc `crates/calc-func/witness-differences.tsv`, DATE row 4 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/witness-differences.tsv`, DATE row 2 at `4fb9b728df` (readings)

<!-- efh:notes end -->

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
