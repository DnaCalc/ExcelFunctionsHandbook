---
schema: efh.function-page/v1
function_id: FUNC.DAYS360
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: DAYS360 function"
    locator: "https://support.microsoft.com/en-us/office/days360-function-b9a509fd-49ef-407e-94df-0cbda5718c2a"
    role: "documented syntax and the verbatim US (NASD) and European method rules"
  - work: "Microsoft 365 support: YEARFRAC function"
    locator: "https://support.microsoft.com/en-us/office/yearfrac-function-3844141e-c76d-4143-82b6-208454ddc6a8"
    role: "the sibling carrying the same two 30/360 conventions as basis 0 and basis 4"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
family: date_value_family
role_in_family: "Interval measurement on the accounting 30/360 convention, in two mutually
  incompatible national variants."
---

## What it computes

`DAYS360` returns the number of days between two dates **as counted by a 360-day year of twelve
30-day months** — the day-count convention used in bond and accrual arithmetic, not the calendar.

The computation has two steps. First, each endpoint's day-of-month is adjusted according to the
selected method; then the count is taken as

> `360·(y₂ − y₁) + 30·(m₂ − m₁) + (d₂ − d₁)`

on the adjusted components. All of the difficulty is in the adjustment, and the adjustment is
where the two methods disagree.

### The two methods, as documented

Microsoft states them verbatim:

**US (NASD) method — `method` FALSE or omitted.** "If the starting date is the last day of a month,
it becomes equal to the 30th day of the same month. If the ending date is the last day of a month
and the starting date is earlier than the 30th day of a month, the ending date becomes equal to the
1st day of the next month; otherwise the ending date becomes equal to the 30th day of the same
month."

**European method — `method` TRUE.** "Starting dates and ending dates that occur on the 31st day of
a month become equal to the 30th day of the same month."

Read side by side, the difference is sharper than it first appears:

- The **European** rule is symmetric, unconditional and touches only the 31st. February is never
  special: 28 February stays 28.
- The **US** rule is asymmetric and conditional. It treats *any* month end (including 28 or 29
  February) as the 30th when it is the start date; and for the end date it makes the outcome depend
  on the start date, which is why the US method is not a function of each endpoint independently.
  The "otherwise" branch — pushing a month-end end-date to the 1st of the next month — is the
  clause that produces the results people find surprising.

The consequence is that the same pair of dates yields **different answers** under the two methods,
and that is not a bug in either. Any workbook that computes accruals has to know which convention
its contracts use.

## Arguments

`DAYS360(start_date, end_date, [method])` — two required arguments and one optional.

**start_date** — the beginning of the period. Microsoft's page carries the category's strongest
version of the text warning: "Problems can occur if dates are entered as text", with the advice to
use `DATE` or the results of other formulas instead.

**end_date** — the end of the period. As with plain subtraction, an end before the start gives a
negative count; the documentation does not describe an error for that case, unlike
[DATEDIF](FUNC.DATEDIF.md).

**method** — optional logical. FALSE or omitted selects the US (NASD) method; TRUE selects the
European method.

The misunderstood argument is `method`, in a specific and consequential way: it is a *logical*, and
its default is the US convention. A European workbook that omits it silently gets American
accrual counts. Because the two methods agree on most ordinary dates and differ only near month
ends, the mistake survives casual testing.

## Result and edge cases

Returns a `Number`: a signed integer count of 30/360 days.

- **Month ends are the entire subject.** For dates that are neither the 30th nor the 31st nor a
  February month end, the two methods and the plain formula all agree.
- **February under the US method** is the distinctive case: 28 February (or 29 in a leap year) as a
  *start date* is treated as the 30th, so February is stretched to a full 30-day month. The
  European method leaves it alone.
- **The US end-date rule is order-dependent** — it consults the start date — so `DAYS360` is not
  antisymmetric under the US method: swapping the arguments need not simply negate the result. That
  is a genuinely unusual property for an interval function and is worth testing directly.
- **The 1900 artefact is largely invisible here**, because the computation runs on (year, month,
  day) components rather than on serial differences — but the components themselves come from the
  serial, so February 1900 remains a curiosity.

## Errors

Microsoft's `DAYS360` page describes the methods and the text-entry hazard, but does not publish an
enumerated error table of the kind [WEEKDAY](FUNC.WEEKDAY.md) or
[NETWORKDAYS.INTL](FUNC.NETWORKDAYS.INTL.md) carry. What can be stated honestly:

- `#VALUE!` for an argument that will not coerce to a number, and propagation of an error argument,
  are the engine-level behaviours from
  [coercion and lifting](../model/02-coercion-and-lifting.md).
- Out-of-range serials are `#NUM!` across this category; the page does not state it for this
  function specifically, and it is not asserted here as a `DAYS360` fact.

## Relationships

- **[YEARFRAC](FUNC.YEARFRAC.md)** — carries the same two conventions as `basis` 0 (US 30/360) and
  `basis` 4 (European 30/360), expressed as a fraction of a year rather than a day count.
  `DAYS360(a, b, FALSE)/360` and `YEARFRAC(a, b, 0)` are the same quantity on the documented
  reading — and Microsoft separately warns that `YEARFRAC` "may return an incorrect result when
  using the US (NASD) 30/360 basis, and the start_date is the last day in February". Whether
  `DAYS360` carries the same defect, or whether the two disagree on those inputs, is exactly the
  kind of question a cross-function suite would answer. Nobody has answered it here.
- **[DAYS](FUNC.DAYS.md)** and the `−` operator — actual calendar days. Substituting one for the
  other changes the financial answer, not just the units.
- **[DATEDIF](FUNC.DATEDIF.md)** with `"D"` — calendar days again, with a documented `#NUM!` for
  reversed arguments that `DAYS360` does not appear to share.
- **Confusable.** `DAYS360` and `DAYS` differ by four characters and by a whole convention;
  `DAYS360(a,b)` and `DAYS360(a,b,TRUE)` differ by a national standard.

## Notes for implementers

1. **Adjust, then subtract — and adjust in the documented order.** The US end-date rule reads the
   *unadjusted* start date's day-of-month in its condition ("the starting date is earlier than the
   30th day of a month"). An implementation that adjusts the start date first and then evaluates
   the condition against the adjusted value will get a different answer, because the start
   adjustment can move a 28 to a 30 and flip the test. This is the most likely place for two
   good-faith implementations to diverge.
2. **Do not assume antisymmetry.** Under the US method, `f(a,b) ≠ −f(b,a)` in general. Any
   optimization that sorts the arguments and negates is wrong.
3. **Treat "last day of a month" correctly for February**, including 29 February in leap years and
   the phantom 29 February 1900 (see [FUNC.DATE](FUNC.DATE.md)).
4. **`method` is a logical, coerced by the ordinary rules** — a numeric 1 or 0, or numeric text,
   will reach it through the standard to-logical path described in
   [coercion and lifting](../model/02-coercion-and-lifting.md). The Handbook's chapter notes that
   to-logical behaviour is per-family; this page does not claim to know `DAYS360`'s.

## What has not been checked

No Handbook vector suite exists for `DAYS360`, and no Excel-comparison evidence is recorded. The
battery on this page is the reference engine answering its own probes; no Excel was involved. The
probes that would settle real questions:

- **Every (month-end → month-end) pair for a two-year window, under both methods.** A few thousand
  rows, and it characterizes the entire disputed region. Everything else in this function is
  arithmetic that cannot go wrong.
- **The US "otherwise" branch specifically** — start dates on the 28th, 29th, 30th and 31st paired
  with end dates that are month ends. This is the clause the documentation words most awkwardly and
  the one implementations most often get wrong.
- **February start dates under the US method** — 28 and 29 February, leap and non-leap.
- **Argument order** — `DAYS360(a,b,FALSE)` against `−DAYS360(b,a,FALSE)`, to confirm or refute the
  asymmetry predicted above. This one needs no Excel oracle to be interesting.
- **`DAYS360(a,b,FALSE)*1/360` against `YEARFRAC(a,b,0)`** — a cross-function consistency check
  touching Microsoft's own documented `YEARFRAC` caveat.
- **`method` supplied as 1, 0, `"TRUE"`, or a number other than 0/1.**

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| 30/360 | Day-count convention treating every month as 30 days and every year as 360 |
| US (NASD) method | The default, asymmetric convention; the end-date rule depends on the start date |
| European method | The symmetric convention; only the 31st is adjusted |
| antisymmetry | The property `f(a,b) = −f(b,a)`, which the US method is not expected to have |

## Sources

- Microsoft 365 support, **DAYS360 function** —
  <https://support.microsoft.com/en-us/office/days360-function-b9a509fd-49ef-407e-94df-0cbda5718c2a>.
  Source of the syntax, the two method rules quoted verbatim above, and the text-entry warning.
- Microsoft 365 support, **YEARFRAC function** —
  <https://support.microsoft.com/en-us/office/yearfrac-function-3844141e-c76d-4143-82b6-208454ddc6a8>.
  Source of the basis table and the last-day-of-February caveat referred to above.
- [FUNC.DATE](FUNC.DATE.md) — the serial-number model and the 1900 artefact.
- Handbook call-model chapter [02 coercion and lifting](../model/02-coercion-and-lifting.md).
- `data/functions/FUNC.DAYS360.json`, `data/presence/FUNC.DAYS360.json`,
  `data/battery/FUNC.DAYS360.json`.
