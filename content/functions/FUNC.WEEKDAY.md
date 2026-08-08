---
schema: efh.function-page/v1
function_id: FUNC.WEEKDAY
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: WEEKDAY function"
    locator: "https://support.microsoft.com/en-us/office/weekday-function-60e44483-2ed1-439f-8bd0-e404c190949a"
    role: "documented return_type table and both #NUM! conditions"
  - work: "Microsoft Learn: Excel incorrectly assumes that the year 1900 is a leap year"
    locator: "https://learn.microsoft.com/en-us/office/troubleshoot/excel/wrongly-assumes-1900-is-leap-year"
    role: "Microsoft's statement that WEEKDAY returns incorrect values for dates before 1 March 1900"
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
role_in_family: "Maps a date serial to its day of the week under one of ten documented numbering
  conventions; the function Microsoft itself names as damaged by the 1900 leap-year artefact."
---

## What it computes

`WEEKDAY` returns which day of the week a date falls on, as a number. The mapping from serial to
weekday is fixed by the serial line; what `return_type` selects is only the *numbering* — which
day counts as 1, and whether the range starts at 0 or 1.

Underneath the ten documented conventions there is one function and two knobs: an origin day and a
zero-or-one base. Everything in the table below is that single rotation.

| `return_type` | Numbers returned |
|---|---|
| 1 or omitted | 1 (Sunday) through 7 (Saturday) |
| 2 | 1 (Monday) through 7 (Sunday) |
| 3 | **0 (Monday) through 6 (Sunday)** |
| 11 | 1 (Monday) through 7 (Sunday) |
| 12 | 1 (Tuesday) through 7 (Monday) |
| 13 | 1 (Wednesday) through 7 (Tuesday) |
| 14 | 1 (Thursday) through 7 (Wednesday) |
| 15 | 1 (Friday) through 7 (Thursday) |
| 16 | 1 (Saturday) through 7 (Friday) |
| 17 | 1 (Sunday) through 7 (Saturday) |

Three features of that table are worth pointing at, because they are the source of most `WEEKDAY`
bugs:

1. **`return_type` 3 is the only zero-based option.** Formulas that switch between 2 and 3 without
   adjusting are off by one everywhere.
2. **2 and 11 are duplicates**, and **1 and 17 are duplicates**. The 11–17 block is the later,
   regular family (start day = `return_type` − 10, counting Monday as 1); 1, 2 and 3 are the
   legacy options that predate it.
3. **There is no `return_type` for ISO week semantics.** `WEEKDAY(s, 2)` is the ISO-compatible
   day numbering (Monday = 1), but week *numbering* is a separate function — see
   [ISOWEEKNUM](FUNC.ISOWEEKNUM.md).

### The 1900 artefact, in Microsoft's own words

Weekday is where Excel's phantom 29 February 1900 becomes an observable wrong answer rather than a
harmless coordinate shift. Because serial 60 occupies a slot that no real day occupies, every
serial from 1 to 59 sits one position out of phase with the true weekday of the day it names.
Microsoft's leap-year article states this plainly as the one residual problem of not fixing the
bug: `WEEKDAY` "returns incorrect values for dates before March 1, 1900". Dates from 1 March 1900
onwards are unaffected.

This is not a defect this Handbook is alleging; it is documented by the vendor. What has *not* been
checked here is exactly where the phase break falls in a running Excel — see below.

## Arguments

`WEEKDAY(serial_number, [return_type])` — one required argument and one optional.

**serial_number** — "a sequential number that represents the date of the day you are trying to
find". Microsoft's page repeats the category advice to use `DATE` rather than text, since text
dates may be interpreted differently.

**return_type** — optional; the numbering convention, per the table above. Omitted means 1 (Sunday
= 1). Values outside the documented set are an error, not a silent default.

The commonly misunderstood position is `return_type` itself: readers assume the number names the
*first day of the week* directly, which is true for 11–17 (offset by 10) and false for 1, 2 and 3.

## Result and edge cases

Returns a `Number` in 1…7, or 0…6 for `return_type` 3.

- **Fractional serials** are floored; time of day is irrelevant.
- **Serials below 61** are in the region Microsoft's own article says returns incorrect values.
- **Serial 60** — the phantom day itself. There is no true weekday for a day that did not exist,
  so whatever Excel returns here is a convention, not a fact.
- **A 1904-system workbook** has no phase problem at all, because 1904 was genuinely a leap year;
  the same calendar day also has a different serial, so a formula that hard-codes serials and a
  formula that uses `DATE` behave differently across the two systems.

## Errors

As documented on the Microsoft page:

- `#NUM!` when `serial_number` is out of range for the current date-base value.
- `#NUM!` when `return_type` is outside the range in the table above.

`#VALUE!` from an argument that will not coerce to a number, and propagation of an error argument,
are the ordinary behaviours from
[coercion and lifting](../model/02-coercion-and-lifting.md), not `WEEKDAY`-specific rules.

## Relationships

- **[WEEKNUM](FUNC.WEEKNUM.md)** — the week-of-year counterpart, with its own and differently
  organized `return_type` table (1, 2, 11–17, 21). The two tables look similar and mean different
  things; the `return_type` values are *not* interchangeable between the functions.
- **[ISOWEEKNUM](FUNC.ISOWEEKNUM.md)** — ISO 8601 week numbering, equivalent to `WEEKNUM(s, 21)`
  on the documented reading.
- **[NETWORKDAYS](FUNC.NETWORKDAYS.md) and [WORKDAY](FUNC.WORKDAY.md)** — the functions that
  consume the same weekday notion implicitly, with Saturday and Sunday hard-wired as the weekend;
  their `.INTL` variants make the weekend definition explicit instead.
- **[DAY](FUNC.DAY.md)** — day *of month*, the routine confusion.
- **The `TEXT` function** with a `"ddd"`/`"dddd"` format code returns the weekday's *name*, and is
  locale-dependent where `WEEKDAY` is not.

## Notes for implementers

1. **Implement one rotation, not ten branches.** Compute a canonical weekday index from the serial
   (for example Monday = 0), then map through a small table of (origin, base) pairs keyed by
   `return_type`. Ten independent branches is ten chances to mis-key a row.
2. **Validate `return_type` against the exact documented set** — {1, 2, 3, 11, 12, 13, 14, 15, 16,
   17}. It is not a range: 4 through 10 are invalid, and so is 18.
3. **Derive the weekday from the serial, not from a calendar library.** This is the function where
   the leap-year artefact must survive rather than be corrected; a real calendar consulted for
   "what weekday was 15 January 1900" gives the true answer, which is the answer Excel's own
   documentation says Excel does not give.
4. **Beware `MOD` with negative operands** if the canonical index is computed by modular
   arithmetic; the serial range is guarded, but intermediate offsets need not be.

## What has not been checked

No Handbook vector suite exists for `WEEKDAY`, and no Excel-comparison evidence is recorded. The
battery on this page shows the reference engine's own answers, produced without Excel. The probes
that matter:

- **`WEEKDAY(59)`, `WEEKDAY(60)`, `WEEKDAY(61)`** — locates the phase break exactly and shows what
  convention Excel applies to the phantom day. Microsoft documents *that* pre-March-1900 values are
  wrong but not *how* they are wrong.
- **All ten `return_type` values on one known date** — a single Wednesday, say, checked across the
  whole table. Ten rows that pin the entire numbering surface.
- **`return_type` 4, 10, 18 and 0** — confirms that the documented set really is a set and that the
  `#NUM!` fires.
- **Serial 2958465 and 2958466** — the upper range boundary.
- **A 1904-system workbook** on the same calendar dates.

Because the kernel is pure integer arithmetic with no floating-point content, a `WEEKDAY` suite of
a few thousand rows could characterize the function essentially completely. Nothing of the kind
exists yet.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| return_type | The argument selecting which day numbers 1 (or 0) and where the week starts |
| the 11–17 block | The regular later family, start day = `return_type` − 10 with Monday as 1 |
| phase break | The point (serial 60) at which Excel's weekday sequence diverges from the true calendar |
| date-base value | The workbook's date-system origin (1900 or 1904) |

## Sources

- Microsoft 365 support, **WEEKDAY function** —
  <https://support.microsoft.com/en-us/office/weekday-function-60e44483-2ed1-439f-8bd0-e404c190949a>.
  Source of the complete `return_type` table and both `#NUM!` conditions.
- Microsoft Learn, **Excel incorrectly assumes that the year 1900 is a leap year** —
  <https://learn.microsoft.com/en-us/office/troubleshoot/excel/wrongly-assumes-1900-is-leap-year>.
  Source of the statement that `WEEKDAY` returns incorrect values for dates before 1 March 1900.
- [FUNC.DATE](FUNC.DATE.md) — the serial model and the 1900/1904 systems.
- Handbook call-model chapter [02 coercion and lifting](../model/02-coercion-and-lifting.md).
- `data/functions/FUNC.WEEKDAY.json`, `data/presence/FUNC.WEEKDAY.json`,
  `data/battery/FUNC.WEEKDAY.json`.
