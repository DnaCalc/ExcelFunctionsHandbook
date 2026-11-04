---
schema: efh.function-page/v1
function_id: FUNC.WEEKNUM
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: WEEKNUM function"
    locator: "https://support.microsoft.com/en-us/office/weeknum-function-e5c43a03-b4ab-426c-b411-b18c13c75340"
    role: "documented return_type table, the System 1 / System 2 definitions, and both #NUM! conditions"
  - work: "Microsoft 365 support: ISOWEEKNUM function"
    locator: "https://support.microsoft.com/en-us/office/isoweeknum-function-1c2d0afe-d25b-4ab1-8894-8d0520e90e0e"
    role: "the dedicated ISO 8601 week-number function this one overlaps at return_type 21"
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
role_in_family: "Numbers the week of the year that a date falls in, under either of two
  incompatible week-numbering systems."
---

## What it computes

`WEEKNUM` returns the ordinal number of the week within the year that a date falls in. The
function carries **two genuinely different definitions**, selected by `return_type`, and confusing
them is the source of nearly every disagreement between spreadsheets about "which week is it".

Microsoft states the two systems as follows:

- **System 1** — "The week containing January 1 is the first week of the year, and is numbered
  week 1." Under this system week 1 can be a stub of a single day, and a year can contain 54
  partial weeks' worth of numbering in the worst case.
- **System 2** — "The week containing the first Thursday of the year is the first week of the year,
  and is numbered as week 1. This system is the methodology specified in ISO 8601, which is
  commonly known as the European week numbering system."

The documented `return_type` table:

| `return_type` | Week begins on | System |
|---|---|---|
| 1 or omitted | Sunday | 1 |
| 2 | Monday | 1 |
| 11 | Monday | 1 |
| 12 | Tuesday | 1 |
| 13 | Wednesday | 1 |
| 14 | Thursday | 1 |
| 15 | Friday | 1 |
| 16 | Saturday | 1 |
| 17 | Sunday | 1 |
| 21 | Monday | 2 |

Note the shape: **nine of the ten options are System 1** and differ only in which weekday starts
the week; the tenth, 21, is the ISO system and is the only one where the week-1 rule itself
changes. `return_type` 2 and 11 are duplicates, as are 1 and 17 — the same redundancy the
[WEEKDAY](FUNC.WEEKDAY.md) table has, for the same historical reason.

Under System 1, the algorithm is: week `n` runs from the `n`-th week-start day on or before the
first week-start day following 1 January, with the partial week containing 1 January counted as
week 1. Under System 2 the algorithm is the ISO 8601 one, in which weeks always have seven days,
always begin on Monday, and the last days of December can belong to week 1 of the *following*
year — which is why an ISO week number alone is ambiguous without its ISO year.

## Arguments

`WEEKNUM(serial_number, [return_type])` — one required argument and one optional.

**serial_number** — "a date within the week". Microsoft's page repeats the serial-number model:
"Excel stores dates as sequential serial numbers so they can be used in calculations. By default,
January 1, 1900 is serial number 1." Supply dates from `DATE` or from another formula rather than
as text.

**return_type** — optional; the week-start day and system, per the table. Omitted means 1 (weeks
begin Sunday, System 1).

The misunderstood argument is again `return_type`, in three distinct ways: readers assume its
values match `WEEKDAY`'s (they overlap but do not mean the same thing), assume 21 is "just Monday"
(it changes the entire week-1 rule), and assume there is a System 2 variant for other start days
(there is not — ISO weeks are Monday-based by definition).

## Result and edge cases

Returns a `Number`: a week ordinal.

- **Early January.** This is where the two systems visibly disagree. A 1 January falling on a
  Friday is week 1 under System 1 and week 53 of the *previous* year under ISO.
- **Late December.** Under System 1 a year can reach week 53 or 54 depending on the start day;
  under ISO the last days of December may be week 1 of the next year.
- **The result carries no year.** `WEEKNUM` returns a number, not a (year, week) pair, so ISO week
  numbers obtained from it must be paired with an ISO year computed separately — Excel provides no
  ISO-year function.
- **Serials in 1900.** Week numbering near the start of the serial line inherits the phase problem
  described on [WEEKDAY](FUNC.WEEKDAY.md), because week boundaries are weekday boundaries.
- **Serial 0.** The reference-engine battery on this page answers the zero probe with a week number
  in the 50s, consistent with treating serial 0 as belonging to a week at the end of 1899. Whether
  Excel agrees has not been checked.

## Errors

As documented on the Microsoft page:

- `#NUM!` when `serial_number` is out of range for the current date-base value.
- `#NUM!` when `return_type` is outside the range specified in the table.

Excel does not always follow the second of these: for an undocumented `return_type` it usually
gives `#NUM!`, sometimes a number, and the same call can answer differently (FN-WEEKNUM-001).

Ordinary coercion failures give `#VALUE!`, and an error argument propagates; those are engine
behaviours (see [coercion and lifting](../model/02-coercion-and-lifting.md)), not `WEEKNUM` rules.

## Relationships

- **[ISOWEEKNUM](FUNC.ISOWEEKNUM.md)** — the dedicated ISO 8601 function, which on the documented
  reading computes the same thing as `WEEKNUM(s, 21)`. Having two spellings of one definition is
  itself a fact about the surface, and whether they agree on every input is a natural equivalence
  test that nobody has run for this Handbook.
- **[WEEKDAY](FUNC.WEEKDAY.md)** — the day-of-week function whose `return_type` table looks similar
  and is not interchangeable.
- **[DATE](FUNC.DATE.md)** — the recommended way to construct the argument.
- **Confusable.** `WEEKNUM(s,1)` and `WEEKNUM(s,21)` differ for a large fraction of January and
  December dates; reports that "Excel gives the wrong week number" are almost always this.

## Notes for implementers

1. **Implement System 1 and System 2 as separate algorithms.** They are not the same computation
   with a different start day; System 2's week 1 is defined by the first Thursday, System 1's by 1
   January. A single parameterized routine that tries to cover both usually gets the year boundary
   wrong in one of them.
2. **Validate `return_type` against the exact set** {1, 2, 11, 12, 13, 14, 15, 16, 17, 21}. It is
   not a range; 3 is valid for `WEEKDAY` and invalid here, which is a genuinely easy mistake given
   how the two tables are usually read side by side.
3. **Do not compute ISO weeks by rounding day-of-year.** The standard `⌊(doy + 6 − isoweekday)/7⌋`
   style formula needs its year-boundary corrections; get them from the ISO definition rather than
   by patching failures.
4. **Reuse the weekday kernel** shared with `WEEKDAY`, so that the 1900 phase behaviour is the same
   in both functions by construction rather than by coincidence.

## What has not been checked

No Handbook vector suite exists for `WEEKNUM`. The Excel readings on this page are in
FN-WEEKNUM-001, on undocumented `return_type` values. The battery on this page is the reference
engine answering its own probes; no Excel was involved. The probes that would settle real questions:

- **Every 1 January and 31 December from 1990 to 2040, across all ten `return_type` values.** Under
  a thousand rows, and it characterizes both systems at exactly the boundaries where they are hard.
  This is the highest-value cheap suite in the week family.
- **Years beginning on each of the seven weekdays**, so that all System 1 stub-week shapes and all
  ISO 52/53-week years are represented.
- **`WEEKNUM(s, 21)` against `ISOWEEKNUM(s)` on the same inputs** — an equivalence claim that is
  widely assumed and, here, unverified.
- **`WEEKNUM(0)` and the January 1900 region**, where the leap-year phase problem meets week
  boundaries.
- **`return_type` 3, 18, 22.** Excel usually gives `#NUM!` for an undocumented value and sometimes
  a number (FN-WEEKNUM-001). What decides the answer is not known, and the raw readings of build
  20527 are not yet in a published capture.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `WEEKNUM` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-WEEKNUM-001: WEEKNUM with an invalid return_type can answer differently each time it is called

*Quirk; measured; recorded 2026-11-04.*

The documentation says an out-of-range return_type gives #NUM!. Excel usually does, but not
always, and the same call can give different answers. In recorded captures of build 20430, 479
(serial_number, return_type) pairs were read with different results: usually #NUM!, sometimes a
number (0, 1, 9, 10, 53 or 54), and some pairs differ within one run file. In build 20527 a
matrix of 1,926 pairs (serial_number 0, 1, 59, 60, 61 and 43831, return_type -32 to 288) was
read twice in one request, and 623 pairs differed between the two readings: 619 between a
number and #NUM!, and 4 between 9 and 10. Every difference has an undocumented return_type. The
ten documented return_types (1, 2, 11 to 17 and 21) gave the same answer both times. No rule
for the undocumented values is claimed.

Documentation: If Return_type is out of the range specified in the table above, a #NUM! error
is returned.
(https://support.microsoft.com/en-us/office/weeknum-function-e5c43a03-b4ab-426c-b411-b18c13c75340)

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `WEEKNUM(0, 23)` | serial_number `0` (`0x0000000000000000`); return_type `23` (`0x4037000000000000`) (cells) | `#NUM!` | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `WEEKNUM(0, 23)` | serial_number `0` (`0x0000000000000000`); return_type `23` (`0x4037000000000000`) (cells) | `1` (`0x3ff0000000000000`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `WEEKNUM(0, 33)` | serial_number `0` (`0x0000000000000000`); return_type `33` (`0x4040800000000000`) (cells) | `0` (`0x0000000000000000`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `WEEKNUM(0, 33)` | serial_number `0` (`0x0000000000000000`); return_type `33` (`0x4040800000000000`) (cells) | `#NUM!` | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |

Basis (measured): The readings conflict with each other. The 479 comes from the DnaCalc witness
differences file. The 1,926 pairs come from one request of 3,852 calls, with exact input
read-backs, compared pair by pair.

- Open: What decides the answer for an undocumented return_type is not known.
- Open: The raw readings of build 20527 are not yet in a published capture. Their counts are recorded in the DnaCalc issue tracker.

Evidence:

- DnaCalc `crates/calc-func/witness-differences.tsv`, WEEKNUM at `9c23dde1df` — index of the conflicting readings
- DnaCalc `.beads/issues.jsonl`, dc-zmk.2 at `2d0eddcac8` — the repeat comparison of 3,852 calls in build 20527
- OxFunc-Evidence `runs/w111-date-boundaries-20260929/answers/answers-weeknum.json.gz`, date-boundary-WEEKNUM-00188 at `e26b7bddb8` (readings)
- OxFunc-Evidence `runs/w111-date-conversion-20260929/answers/answers-weeknum.json.gz`, date-conversion-WEEKNUM-01331 at `e26b7bddb8` (readings)
- OxFunc-Evidence `runs/w111-date-boundaries-20260929/answers/answers-weeknum.json.gz`, date-boundary-WEEKNUM-00940 at `e26b7bddb8` (readings)
- OxFunc-Evidence `runs/w111-date-conversion-20260929/answers/answers-weeknum.json.gz`, date-conversion-WEEKNUM-00133 at `e26b7bddb8` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| System 1 | Week 1 is the week containing 1 January; week start day is selectable |
| System 2 | ISO 8601: week 1 is the week containing the first Thursday; weeks always start Monday |
| stub week | A System 1 week 1 shorter than seven days |
| ISO year | The year an ISO week belongs to, which can differ from the calendar year; Excel exposes no function for it |

## Sources

- Microsoft 365 support, **WEEKNUM function** —
  <https://support.microsoft.com/en-us/office/weeknum-function-e5c43a03-b4ab-426c-b411-b18c13c75340>.
  Source of the `return_type` table, the System 1 and System 2 definitions quoted above, the
  serial-number remark, and both `#NUM!` conditions.
- Microsoft 365 support, **ISOWEEKNUM function** —
  <https://support.microsoft.com/en-us/office/isoweeknum-function-1c2d0afe-d25b-4ab1-8894-8d0520e90e0e>.
- [FUNC.WEEKDAY](FUNC.WEEKDAY.md) — the weekday kernel and the documented 1900 phase problem.
- [FUNC.DATE](FUNC.DATE.md) — the serial-number model.
- `data/functions/FUNC.WEEKNUM.json`, `data/presence/FUNC.WEEKNUM.json`,
  `data/battery/FUNC.WEEKNUM.json`.
