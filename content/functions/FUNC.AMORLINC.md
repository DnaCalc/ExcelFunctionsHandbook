---
schema: efh.function-page/v1
function_id: FUNC.AMORLINC
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn: WorksheetFunction.AmorLinc method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.amorlinc"
    role: "the parameter list, the four-row basis table, the proration sentence, and the French-accounting provenance"
  - work: "Microsoft Learn: WorksheetFunction.AmorDegrc method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.amordegrc"
    role: "the sibling's coefficient rule, cited for the comparison that defines this function's role"
  - work: "Microsoft 365 support: AMORLINC function"
    locator: "https://support.microsoft.com/en-us/office/amorlinc-function-7d417b45-f7f5-4dba-a0a5-3451a81079a8"
    role: "the worksheet-surface page; cited but not retrieved — the host returned HTTP 403 to the Handbook's fetch"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: amor_depreciation_family
role_in_family: >-
  The linear member: a flat annual charge of cost times rate, with only the first period
  prorated by a day-count fraction — the function AMORDEGRC reduces to when its coefficient
  is one.
---

## What it computes

`AMORLINC` returns the depreciation charge for **one accounting period** under the French
straight-line regime. Microsoft's Learn page states both the provenance — "This function is
provided for the French accounting system" — and the whole of the rule in one sentence: "If an
asset is purchased in the middle of the accounting period, the prorated depreciation is taken
into account."

The schedule is as simple as depreciation gets:

>     annual charge      = cost × rate
>     first charge (period 0) = cost × rate × f,  where f = fraction of the first period held
>     every later charge = cost × rate,  until the depreciable basis (cost − salvage) runs out
>     final charge       = whatever is left of the depreciable basis

`f` is a day-count fraction from `date_purchased` to `first_period`, computed on the chosen
`basis`. When the asset is bought exactly on `first_period` the fraction is 1 and the first
period is a full one.

The only genuinely interesting quantity is `f`, and `f` is a [YEARFRAC](FUNC.YEARFRAC.md)
computation under a different name. Everything else is a constant repeated until the money runs
out.

`period` is **0-based** — period 0 is the prorated first period. This is the same convention as
[AMORDEGRC](FUNC.AMORDEGRC.md) and the opposite of [DB](FUNC.DB.md) and [DDB](FUNC.DDB.md),
which count from 1. Mixing the two conventions in one model is the most common way to get a
plausible but wrong depreciation schedule.

## Arguments

`AMORLINC(cost, date_purchased, first_period, salvage, period, rate, [basis])`

| Argument | Meaning | Required? |
|---|---|---|
| `cost` | The cost of the asset. | Required |
| `date_purchased` | The date of purchase. | Required |
| `first_period` | The date of the **end of the first period**. | Required |
| `salvage` | The salvage value at the end of the asset's life. | Required |
| `period` | The period. **0-based**: 0 is the prorated first period. | Required |
| `rate` | The annual rate of depreciation. | Required |
| `basis` | The year basis. | Optional, defaults to 0 |

`first_period` is the argument readers misread most often. It is not the *length* of the first
period and it is not the start of anything: it is the date on which the first accounting period
**ends**. The prorated fraction is measured from `date_purchased` up to it.

**The basis table for this function has four rows, not five.** Microsoft's Learn page gives:

| `basis` | Date system |
|---|---|
| 0 or omitted | 360 days (NASD method) |
| 1 | Actual |
| 3 | 365 days in a year |
| 4 | 360 days in a year (European method) |

`basis` = 2 (actual/360 elsewhere in the category) is **absent**, exactly as on the
[AMORDEGRC](FUNC.AMORDEGRC.md) page and unlike every other financial function that takes a
`basis`. The reference engine agrees and rejects 2 with `#NUM!`. Code that validates `basis`
once, centrally, as "an integer in 0…4" will admit a value these two functions do not accept.

Date arguments are Excel date serials in numeric slots. Microsoft's page carries the standard
warning that dates should come from `DATE` or from other formulas rather than being typed as
text; ordinary coercion rules are in
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns a `Number`: the charge for the requested period, in the units of `cost`.

- **Period 0 is the prorated one.** Period 1 is a full annual charge even when the asset was
  bought a week before `first_period`.
- **Fractional `period`.** The reference engine treats a `period` strictly between 0 and 1 as
  period 1 — that is, it rounds *up* into the first full period rather than flooring to 0 — and
  floors any `period` of 1 or more. Its sibling `AMORDEGRC` instead returns zero for a
  fractional period below 1. The documentation consulted says nothing about fractional periods,
  and the two siblings do not agree; treat this as unspecified and avoid non-integer `period`.
- **The schedule terminates on the depreciable basis.** Charges continue at `cost × rate` until
  the cumulative total reaches `cost − salvage`; the final period takes only the remainder, and
  every period after that returns 0 rather than an error.
- **`salvage` = `cost`** gives a depreciable basis of zero, so period 0 returns 0.
- **`date_purchased` = `first_period`** gives a fraction of exactly 1 — a full first period,
  with no proration.
- **`date_purchased` after `first_period`** is rejected by the reference engine.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md).

Unlike its sibling, the reference engine applies **no rounding** to `AMORLINC`'s per-period
charges. That asymmetry between two functions sharing one module and one argument list is worth
knowing before comparing their schedules.

## Errors

**Microsoft's Learn page documents no error conditions for this function at all** — it carries
the parameter table, the basis table, the proration sentence and the date-system notes, and
stops. Its sibling's page documents a `#NUM!` band; this one documents nothing. That silence is
itself a finding: there is no published statement of what `AMORLINC` does with a negative
`cost`, a `salvage` above `cost`, or a `basis` of 2, even though the basis table's omission of 2
implies the last one.

The reference engine's conditions, recorded as the reference engine's and not as Excel's:

| Error | Condition (reference engine) |
|---|---|
| `#NUM!` | `cost` ≤ 0, `salvage` < 0, or `salvage` > `cost` |
| `#NUM!` | `rate` ≤ 0 |
| `#NUM!` | `cost`, `salvage` or `rate` is not finite |
| `#NUM!` | `basis` truncates to anything other than 0, 1, 3 or 4 — **including 2** |
| `#NUM!` | `period` is negative or not finite |
| `#NUM!` | `date_purchased` is later than `first_period` |
| `#VALUE!` | A date argument is not finite, or truncates outside the representable date range |
| propagated | An error value in any argument surfaces as that error |

## Relationships

- **[AMORDEGRC](FUNC.AMORDEGRC.md)** — the accelerated sibling, with the same seven arguments,
  the same proration and the same four-row basis table. Microsoft describes the difference in
  one clause: `AMORDEGRC` applies "a depreciation coefficient … depending on the life of the
  assets". Set that coefficient to 1 and remove the end-of-life 50/100 per cent steps and you
  have this function.
- **`SLN`** — the plain straight-line function. `SLN(cost, salvage, life)` is a flat
  `(cost − salvage)/life`. `AMORLINC` differs on three axes at once: it charges `cost × rate`
  rather than `(cost − salvage) × rate`, it prorates the first period by calendar date, and it
  truncates the last charge at the depreciable basis instead of dividing it evenly.
- **[DB](FUNC.DB.md)** — the other function in Excel that prorates a partial first year, but by
  a `month` count rather than by a date pair, and on a declining rather than a linear balance.
- **[DDB](FUNC.DDB.md)**, `VDB`, `SYD` — the rest of the depreciation category, all 1-based in
  `period` and none of them date-aware.
- **[YEARFRAC](FUNC.YEARFRAC.md)** — the standalone day-count engine that computes the same
  first-period fraction this function needs.
- **Confused with**: `SLN` (above). The two answer the same accounting question and give
  different numbers, because `SLN` spreads `cost − salvage` while `AMORLINC` charges `rate` on
  `cost` and stops when it hits `salvage`.

## Numerical notes

1. **The only computed quantity is the first-period fraction.** Everything downstream is
   repeated multiplication and subtraction of a constant, so an implementation's accuracy is
   entirely the accuracy of its day count.
2. **Two 30/360 routines live in this reference engine, and they disagree.** This family's own
   US 30/360 routine collapses an end day of 31 to 30 *in place*, and only when the (already
   adjusted) start day is 30 or more. The bond and coupon families share a different routine
   that rolls the end date into the following month when the end day is 31 and the start day is
   below 30. On that input class the two return different day counts. So `AMORLINC`'s basis-0
   proration and [COUPDAYBS](FUNC.COUPDAYBS.md)'s basis-0 accrual are **not** guaranteed to be
   the same computation, even though both are "US 30/360". The Handbook's research record
   documents precisely this class of divergence arising elsewhere in the same engine.
3. **Basis 1 ("Actual") is under-specified**, as everywhere in this category. The reference
   engine splits the interval at calendar-year boundaries and divides each piece by that year's
   own length; other constructions are defensible and give different fractions across a leap
   day. See [YEARFRAC](FUNC.YEARFRAC.md).
4. **The schedule must be iterated.** The last charge depends on the cumulative total, so there
   is no closed form for period *n*; the cost is `O(period)`, and an error in the first fraction
   propagates to the termination point of the whole schedule.
5. **No rounding, unlike the sibling.** If you are porting this pair, do not factor the rounding
   into a shared helper: only `AMORDEGRC` uses it in the reference engine.

## What has not been checked

No Handbook vector suite exists for `AMORLINC`, and **no evidence record lists this surface in
its subjects** — nor does any record name its family module. Nobody has checked this function
against Excel within the Handbook's record. The battery on this page is the reference engine
answering its own probes; no Excel was involved.

The most consequential unknowns here are not numerical. They are: what Excel does with a
fractional `period` (where the two siblings already disagree), and whether the basis-0 fraction
is the same 30/360 day count the bond family uses.

Inputs worth probing first:

1. **`period` = 0.5.** The reference engine's `AMORLINC` treats it as period 1 while its
   `AMORDEGRC` returns 0. Nothing documented covers it, and the two answers are far apart.
2. **`date_purchased` on the 31st with `first_period` on a month end**, on `basis` 0, compared
   against `COUPDAYBS`/`YEARFRAC` for the same dates. This is the probe that would expose the
   two-30/360-routines issue, and it is the highest-value single experiment on this page.
3. **`basis` = 2**, which the documentation omits and the reference engine rejects. One cell.
4. **A full schedule read out period by period** — 0, 1, 2, … past the termination point —
   checking where the last partial charge lands and whether later periods return 0 or `#NUM!`.
5. **`date_purchased` = `first_period`**, confirming a fraction of exactly 1 and no proration.
6. **`salvage` > `cost`, `cost` = 0, `rate` = 0**, none of which the documentation addresses,
   and all of which the reference engine rejects.
7. **Basis 1 across a leap day**, which is the only case where the actual/actual construction is
   observable.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| proration | Scaling the first period's charge by the fraction of that period the asset was held |
| first-period fraction | The day-count fraction from `date_purchased` to `first_period` on the chosen basis |
| depreciable basis | `cost − salvage`; the total the schedule may charge before it stops |
| 0-based period | `period` 0 is the first (prorated) period, unlike `DB` and `DDB` |
| day-count basis | The `basis` argument's convention — here a four-value set, with no 2 |

## Sources

- Microsoft Learn, **WorksheetFunction.AmorLinc method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.amorlinc>. Source of
  the parameter table, the four-row basis table (with no `basis` = 2), the proration sentence,
  the French-accounting provenance, and the date-entry warning. This page documents no error
  conditions.
- Microsoft Learn, **WorksheetFunction.AmorDegrc method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.amordegrc>. Cited
  for the sentence describing the two functions' relationship and for the sibling's documented
  error band.
- Microsoft 365 support, **AMORLINC function** —
  <https://support.microsoft.com/en-us/office/amorlinc-function-7d417b45-f7f5-4dba-a0a5-3451a81079a8>.
  The worksheet-surface page; **not retrieved** for this entry (the host returned HTTP 403), so
  nothing here is quoted from it.
- Handbook, [YEARFRAC](FUNC.YEARFRAC.md) — the day-count basis axis and the 30/360 rules.
- Handbook, `content/lastbit/MATERIAL_W109_PRIMITIVES.md` §21 — the recorded case of two 30/360
  routines in one engine disagreeing on month-end pairs.
- Handbook chapters [01 the value universe](../model/01-value-universe.md),
  [02 coercion and lifting](../model/02-coercion-and-lifting.md),
  [06 claim language](../model/06-claim-language.md).
- OxFunc `crates/oxfunc_core/src/functions/amor_depreciation_family.rs` and
  `crates/oxfunc_core/src/functions/day_count_common.rs` at commit `473efa3` — the reference
  engine's kernel, validation, period normalization and the two distinct US 30/360 routines.
- `data/functions/FUNC.AMORLINC.json`, `data/presence/FUNC.AMORLINC.json`,
  `data/battery/FUNC.AMORLINC.json`.
