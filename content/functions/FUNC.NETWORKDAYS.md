---
schema: efh.function-page/v1
function_id: FUNC.NETWORKDAYS
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: NETWORKDAYS function"
    locator: "https://support.microsoft.com/en-us/office/networkdays-function-48e717bf-a7a3-495f-969e-5005e3eb18e7"
    role: "documented syntax, the holidays argument, and the #VALUE! condition"
  - work: "Microsoft 365 support: NETWORKDAYS.INTL function"
    locator: "https://support.microsoft.com/en-us/office/networkdays-intl-function-a9b26239-4f20-46a1-9ab8-4e925bfd5e28"
    role: "the generalized successor, whose weekend table and error conditions are documented in more detail"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
family: workday_networkdays_family
role_in_family: "Counts working days in an interval with the weekend fixed at Saturday and Sunday;
  the original, un-generalized member of the family."
---

## What it computes

`NETWORKDAYS` counts the whole working days in the interval from `start_date` to `end_date`,
excluding weekends and any dates supplied in `holidays`. The weekend is **fixed at Saturday and
Sunday** and cannot be changed — that limitation is the entire reason
[NETWORKDAYS.INTL](FUNC.NETWORKDAYS.INTL.md) exists.

The computation is a filtered count: enumerate the days of the interval, drop those falling on
Saturday or Sunday, drop those appearing in `holidays`, and return how many remain.

Two properties of that count matter and are not settled by the documentation page:

1. **Inclusivity.** The catalogue description is "the number of whole workdays between two dates",
   and the function's ordinary use — counting the working days of a month by passing the first and
   last of it — depends on both endpoints being counted. The Microsoft page linked below does not
   state the rule either way. It is recorded here as the expected behaviour, unverified.
2. **Reversed intervals.** Whether an `end_date` before `start_date` gives a negative count, a
   positive one, or an error is likewise not stated on that page. The `.INTL` sibling's page is no
   more explicit. Unverified here.

Both are one-line experiments, and both are listed below.

## Arguments

`NETWORKDAYS(start_date, end_date, [holidays])` — two required arguments and one optional.

**start_date** — "a date that represents the start date".

**end_date** — "a date that represents the end date".

**holidays** — optional: "an optional range of one or more dates to exclude from the working
calendar, such as state and federal holidays and floating holidays. The list can be either a range
of cells that contains the dates or an array constant of the serial numbers that represent the
dates."

Three things about `holidays` that the sentence above implies and readers routinely miss:

- **It is a range or array argument in a function whose other arguments are scalars.** That makes
  `NETWORKDAYS` an aggregate-shaped call in its third position; see
  [the call pipeline](../model/03-call-pipeline.md) for how such positions are prepared.
- **Duplicate holidays should not double-count**, since the operation is set exclusion rather than
  subtraction — but the documentation does not say so.
- **Holidays falling on a weekend are already excluded** and should contribute nothing, for the
  same reason. Also not documented.

Microsoft's page carries the category's standard warning that "problems can occur if dates are
entered as text" and recommends `DATE` or formula results instead.

## Result and edge cases

Returns a `Number`: a count of working days.

- **A same-day interval** (`start_date` equal to `end_date`) is 1 if that day is a working day and
  0 otherwise — under the inclusive reading above.
- **An interval containing no working days** returns 0.
- **Holidays outside the interval** are irrelevant and should be ignored.
- **Empty cells inside a `holidays` range** raise the ordinary Empty-versus-Missing question
  described in [the value universe](../model/01-value-universe.md); whether they are skipped or
  treated as serial 0 (which is inside the representable range and would be a "holiday") is not
  documented and not checked here. It is a real hazard, because holiday ranges are usually sized
  generously and left partly blank.
- **Fractional serials** — whether time components are truncated before counting is undocumented.

## Errors

As documented on the Microsoft page:

- `#VALUE!` "if any argument is not a valid date".

Note the contrast with the generalized sibling: [NETWORKDAYS.INTL](FUNC.NETWORKDAYS.INTL.md)'s page
documents `#NUM!` when a date is out of range for the date-base value and `#VALUE!` only for an
invalid weekend string. Two functions computing the same thing, documented with different error
values for an out-of-range date. This Handbook does not harmonize the two; the divergence is
recorded, and resolving it against a running Excel is one of the experiments listed below. See
[claim language](../model/06-claim-language.md), rule 5.

## Relationships

- **[NETWORKDAYS.INTL](FUNC.NETWORKDAYS.INTL.md)** — the generalization. `NETWORKDAYS(a, b, h)`
  should equal `NETWORKDAYS.INTL(a, b, 1, h)`, since weekend code 1 is documented as
  Saturday/Sunday. That equivalence is the cleanest cross-function test in this family and is
  **not verified here**. `NETWORKDAYS` is not a Compatibility-category function and Microsoft has
  not superseded it; both remain current, with the `.INTL` form recommended when the weekend is not
  Saturday/Sunday.
- **[WORKDAY](FUNC.WORKDAY.md)** — the inverse operation: given a start and a count of working
  days, find the date. `NETWORKDAYS` and `WORKDAY` should compose, and their inclusivity
  conventions differ (see that page), which is the classic off-by-one in scheduling formulas.
- **[DAYS](FUNC.DAYS.md)** — the unfiltered calendar count.
- **[WEEKDAY](FUNC.WEEKDAY.md)** — the underlying notion of "which day is Saturday", and the
  function Microsoft documents as returning incorrect values before 1 March 1900.

## Notes for implementers

1. **Do not enumerate day by day for large intervals.** The closed form is
   `7·⌊d/7⌋`-style arithmetic on whole weeks plus a small table for the partial week at each end;
   an implementation that loops will be correct and unusably slow for century-scale intervals,
   which are reachable given the serial range.
2. **Normalize `holidays` to a set first**, filtering out weekend dates and out-of-interval dates,
   so that duplicates and stray blanks cannot affect the count.
3. **Decide the reversed-interval policy explicitly**, and note that whatever `NETWORKDAYS` does,
   `.INTL` must do the same or the equivalence above fails.
4. **Share the weekday kernel** with [WEEKDAY](FUNC.WEEKDAY.md) so that the documented pre-March-1900
   phase behaviour is inherited rather than reinvented.

## What has not been checked

No Handbook vector suite exists for `NETWORKDAYS`, and no Excel-comparison evidence is recorded.
The battery panel on this page is the reference engine answering its own probes; no Excel was
involved. The experiments that would settle this page's open questions, in order of value:

- **`NETWORKDAYS(monday, monday)` and `NETWORKDAYS(saturday, saturday)`** — settles inclusivity in
  two rows. Everything else on this page depends on the answer.
- **`NETWORKDAYS(b, a)` with `a < b`** — settles the reversed-interval policy.
- **`NETWORKDAYS(a, b, h)` against `NETWORKDAYS.INTL(a, b, 1, h)`** over a few hundred date pairs —
  tests the equivalence with the generalized form, without needing to know which answer is right.
- **A `holidays` range containing blanks, duplicates, weekend dates, and dates outside the
  interval** — four hazards, one experiment.
- **An out-of-range date** — resolves the documented `#VALUE!`/`#NUM!` discrepancy with the `.INTL`
  page.
- **Intervals spanning February 1900**, where the weekday phase break lives.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| working day | A day that is neither a weekend day nor listed in `holidays` |
| inclusivity | Whether the endpoints themselves are counted; not stated on the Microsoft page |
| the `.INTL` generalization | The sibling function that makes the weekend definition an argument |
| holiday set | The normalized, de-duplicated set of excluded dates an implementation should build |

## Sources

- Microsoft 365 support, **NETWORKDAYS function** —
  <https://support.microsoft.com/en-us/office/networkdays-function-48e717bf-a7a3-495f-969e-5005e3eb18e7>.
  Source of the syntax, the `holidays` description quoted above, the text-entry warning and the
  `#VALUE!` condition. It does not state inclusivity or reversed-interval behaviour.
- Microsoft 365 support, **NETWORKDAYS.INTL function** —
  <https://support.microsoft.com/en-us/office/networkdays-intl-function-a9b26239-4f20-46a1-9ab8-4e925bfd5e28>.
  Source of the contrasting documented error conditions.
- Handbook call-model chapters [01 value universe](../model/01-value-universe.md) and
  [03 call pipeline](../model/03-call-pipeline.md).
- `data/functions/FUNC.NETWORKDAYS.json`, `data/presence/FUNC.NETWORKDAYS.json`,
  `data/battery/FUNC.NETWORKDAYS.json`.
