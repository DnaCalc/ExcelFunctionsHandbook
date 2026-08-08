---
schema: efh.function-page/v1
function_id: FUNC.WORKDAY
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: WORKDAY function"
    locator: "https://support.microsoft.com/en-us/office/workday-function-f764a5b7-05fc-4494-9486-60d494efbf33"
    role: "documented syntax, the days truncation rule, and both error conditions"
  - work: "Microsoft 365 support: WORKDAY.INTL function"
    locator: "https://support.microsoft.com/en-us/office/workday-intl-function-a378391c-9ba7-4678-8a39-39611a9bf81d"
    role: "the generalized successor, with the weekend argument and a fuller error table"
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
role_in_family: "Working-day offsetting with the weekend fixed at Saturday and Sunday; the inverse
  operation to NETWORKDAYS."
---

## What it computes

`WORKDAY` returns the date that lies a given number of **working days** before or after a start
date. Weekends — fixed at Saturday and Sunday — and any dates listed in `holidays` are skipped and
do not consume the count.

Operationally: step day by day away from `start_date` in the direction given by the sign of `days`,
decrementing a counter each time the day stepped onto is a working day, and stop when the counter
reaches zero. The result is that day's serial.

The subtlety that this phrasing makes visible, and that the documentation does not settle, is
whether `start_date` itself can be the answer. Microsoft's page does not state whether the start
date participates in the count. The behaviour readers rely on — and the one that makes `WORKDAY`
the inverse of [NETWORKDAYS](FUNC.NETWORKDAYS.md) — is that `days = 0` returns `start_date`
unchanged and that counting begins from the *next* day. **This Handbook has not verified it**, and
it is the first probe listed below. It matters: the two conventions differ by one working day on
every call, which is a schedule that misses its deadline.

## Arguments

`WORKDAY(start_date, days, [holidays])` — two required arguments and one optional.

**start_date** — "a date that represents the start date".

**days** — "the number of nonweekend and nonholiday days before or after start_date. A positive
value for days yields a future date; a negative value yields a past date." Microsoft states the
conversion rule explicitly: **"If days is not an integer, it is truncated."** Truncation toward
zero, on the ordinary reading — so `-1.9` becomes `-1`, not `-2`. That negative branch is worth
testing, since languages disagree about it.

**holidays** — "an optional list of one or more dates to exclude from the working calendar, such as
state and federal holidays and floating holidays". A range or array argument in an otherwise scalar
signature; see [the call pipeline](../model/03-call-pipeline.md) for how such positions are
prepared.

The misunderstood argument is `days`, in two ways: readers pass a *calendar* day count and expect
weekends to be skipped on top of it (they are not — `days` is already in working days), and readers
assume `days = 1` means "tomorrow if tomorrow is a workday" without checking whether the start day
itself counts.

## Result and edge cases

Returns a `Number`: a whole date serial.

- **`days = 0`** is the case that reveals the counting convention. Under the expected reading it
  returns `start_date` — even when `start_date` is a Saturday, which is the input that
  distinguishes "return the start date" from "snap to the nearest working day".
- **A `start_date` that is itself a weekend or holiday** is the other diagnostic input, and for the
  same reason.
- **Negative `days`** walks backwards; the same convention questions apply mirrored.
- **Holidays outside the traversed span** are irrelevant. Duplicates and weekend-falling holidays
  should contribute nothing, since the operation is exclusion — undocumented.
- **Blank cells in a `holidays` range** raise the Empty-versus-Missing question from
  [the value universe](../model/01-value-universe.md), and serial 0 lies inside the representable
  range, so a blank read as a number is a date. Undocumented, and a genuine hazard because holiday
  ranges are usually over-sized.
- **The result carries no time component.**

## Errors

As documented on the Microsoft page:

- `#VALUE!` "if any argument is not a valid date".
- `#NUM!` "if start_date plus days yields an invalid date" — that is, when the offset walks off
  either end of the representable serial range.

The generalized sibling [WORKDAY.INTL](FUNC.WORKDAY.INTL.md) documents a fuller table, including
`#NUM!` for an out-of-range date in `holidays` and `#VALUE!` for an invalid weekend string. Whether
`WORKDAY` also errors on an out-of-range holiday is not stated on its own page and is not asserted
here.

## Relationships

- **[NETWORKDAYS](FUNC.NETWORKDAYS.md)** — the inverse question: given two dates, how many working
  days. The pair should round-trip — `NETWORKDAYS(a, WORKDAY(a, n))` ought to be a simple function
  of `n` — but the exact relation depends on both functions' inclusivity conventions, neither of
  which is documented. Determining the constant is a one-experiment job and would settle both
  conventions at once.
- **[WORKDAY.INTL](FUNC.WORKDAY.INTL.md)** — the generalization with a `weekend` argument.
  `WORKDAY(a, n, h)` should equal `WORKDAY.INTL(a, n, 1, h)`; unverified here.
- **[EDATE](FUNC.EDATE.md)** — offsetting by months rather than working days, and documented with
  the same "if it is not an integer, it is truncated" rule.
- **[WEEKDAY](FUNC.WEEKDAY.md)** — the underlying day-of-week notion.
- **Confusable.** `WORKDAY(a, n)` and `a + n` differ by however many weekends and holidays fall in
  between, which is precisely why the function exists.

## Notes for implementers

1. **Do not step day by day.** With a fixed Saturday/Sunday weekend the answer has a closed form:
   divide `days` by 5 to get whole weeks, handle the remainder against the start day's weekday
   position, then apply holidays iteratively — and note that applying holidays can push the result
   into a new week, so the holiday pass has to iterate to a fixed point rather than run once.
2. **The holiday fixed-point is the real algorithm.** Adding a holiday moves the target, which may
   land on another holiday. An implementation that adjusts once is wrong for consecutive holidays,
   which is exactly what Christmas and Easter are.
3. **Truncate `days` toward zero**, per the documented word.
4. **Pin the `days = 0` convention** and keep it identical in `WORKDAY.INTL`.
5. **Normalize `holidays` to a set**, dropping weekend and out-of-span entries first.

## What has not been checked

No Handbook vector suite exists for `WORKDAY`, and no Excel-comparison evidence is recorded. The
battery on this page is the reference engine answering its own probes; no Excel was involved. The
experiments that matter:

- **`WORKDAY(friday, 0)`, `WORKDAY(saturday, 0)`, `WORKDAY(sunday, 0)`** — three rows that settle
  the counting convention completely, including the "does it snap" question. This is the highest-
  value probe on the page and everything else depends on it.
- **`WORKDAY(friday, 1)` and `WORKDAY(monday, -1)`** — the direction and off-by-one behaviour at a
  weekend boundary.
- **Consecutive holidays** — a start date immediately before two adjacent holiday dates, testing
  the fixed-point requirement above.
- **`WORKDAY(a, -1.9)` and `WORKDAY(a, 1.9)`** — confirms truncation toward zero.
- **An out-of-range holiday**, where `WORKDAY.INTL` documents `#NUM!` and `WORKDAY`'s page is
  silent.
- **`WORKDAY(a, n, h)` against `WORKDAY.INTL(a, n, 1, h)`** across a few hundred cases.
- **`NETWORKDAYS(a, WORKDAY(a, n))` for a range of `n`** — the round trip, which needs no Excel to
  be informative about internal consistency and is decisive with one.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| working day | A day that is neither a weekend day nor listed in `holidays` |
| counting convention | Whether `start_date` itself participates; not stated on the Microsoft page |
| holiday fixed-point | Re-applying holiday exclusion until the result stops moving |
| truncated toward zero | The documented conversion applied to a non-integer `days` |

## Sources

- Microsoft 365 support, **WORKDAY function** —
  <https://support.microsoft.com/en-us/office/workday-function-f764a5b7-05fc-4494-9486-60d494efbf33>.
  Source of the syntax, the `days` description and truncation rule, and both documented error
  conditions. It does not state whether the start date is counted.
- Microsoft 365 support, **WORKDAY.INTL function** —
  <https://support.microsoft.com/en-us/office/workday-intl-function-a378391c-9ba7-4678-8a39-39611a9bf81d>.
  Source of the fuller error table referred to above.
- Handbook call-model chapters [01 value universe](../model/01-value-universe.md) and
  [03 call pipeline](../model/03-call-pipeline.md).
- `data/functions/FUNC.WORKDAY.json`, `data/presence/FUNC.WORKDAY.json`,
  `data/battery/FUNC.WORKDAY.json`.
