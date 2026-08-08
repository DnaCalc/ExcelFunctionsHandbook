---
schema: efh.function-page/v1
function_id: FUNC.AMORDEGRC
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn: WorksheetFunction.AmorDegrc method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.amordegrc"
    role: "the parameter list, the four-row basis table, the depreciation-coefficient table, the 50/100 per cent end-of-life rule, and the documented #NUM! life ranges"
  - work: "Microsoft Learn: WorksheetFunction.AmorLinc method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.amorlinc"
    role: "the sibling's parameter list and identical basis table, cited for the comparison"
  - work: "Microsoft 365 support: AMORDEGRC function"
    locator: "https://support.microsoft.com/en-us/office/amordegrc-function-a14d0ca1-64a4-42eb-9b3d-b0dededf9e51"
    role: "the worksheet-surface page; cited but not retrieved — the host returned HTTP 403 to the Handbook's fetch"
episodes: []
body_sections:
  - What it computes
  - The depreciation coefficient
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
  The accelerated member: the same prorated French accounting schedule as AMORLINC, multiplied
  by a life-dependent coefficient and forced to 50 and 100 per cent in the last two periods.
---

## What it computes

`AMORDEGRC` returns the depreciation charge for **one accounting period** of an asset, under
the French *amortissement dégressif* rules. Microsoft's Learn page states the provenance
plainly: "This function is provided for the French accounting system."

Two ideas define it, and the second is what separates it from its sibling:

1. **Proration of the first period.** If the asset is bought part-way through the first
   accounting period, the first charge is scaled by the fraction of that period the asset was
   held. That fraction is a day-count computation on the chosen `basis`, from
   `date_purchased` to `first_period`.
2. **A declining-balance charge with a statutory coefficient.** The annual rate is not `rate`
   but `rate × coefficient`, where the coefficient is a step function of the asset's life; and
   in the last two periods the rate is forced up so the book value is fully written off.

Sketching the schedule the way the documentation describes it:

>     effective rate  = rate × coefficient(1 / rate)
>     first charge    = cost × effective rate × first-period fraction
>     later charges   = book value × effective rate
>     second-to-last  = book value × 50 %
>     last            = the whole remaining book value

Microsoft states the last two lines directly: "The depreciation rate will grow to 50 percent for
the period preceding the last period and will grow to 100 percent for the last period." And it
states the stopping rule: "This function will return the depreciation until the last period of
the life of the assets or until the cumulated value of depreciation is greater than the cost of
the assets minus the salvage value."

`period` is **0-based** here — period 0 is the prorated first period — which is unusual in
Excel's depreciation category and is the single most common source of an off-by-one answer.

The reference engine additionally rounds each period's charge to a whole unit, half away from
zero, before carrying the book value forward. That rounding is not in the documentation
consulted; it is recorded here as reference-engine behaviour, and it is a *semantic* choice, not
a display choice, because it changes every subsequent period.

## The depreciation coefficient

Microsoft's Learn page publishes this table:

| Life of assets (1/rate) | Depreciation coefficient |
|---|---|
| Between 3 and 4 years | 1.5 |
| Between 5 and 6 years | 2 |
| More than 6 years | 2.5 |

and immediately below it, this sentence:

> "If the life of assets is between 0 (zero) and 1, 1 and 2, 2 and 3, or 4 and 5, the #NUM!
> error value is returned."

Read together, the documentation says the function is defined only for lives in [3, 4], [5, 6]
and (6, ∞), and errors everywhere else — including the whole band between 4 and 5 years, which
is a hole in the middle of the domain rather than an edge case.

**The reference engine implements a different function.** Its rule is a total step function on
`1/rate`, with a single rejection band at the short end:

| Life `1/rate` (reference engine) | Coefficient |
|---|---|
| ≤ 2 (that is, `rate` ≥ 0.5) | `#NUM!` |
| more than 2, less than 3 | 1 |
| at least 3, less than 5 | 1.5 |
| at least 5, at most 6 | 2 |
| more than 6 | 2.5 |

The two disagree on two named bands, and the disagreement is not subtle:

- **Life strictly between 2 and 3.** Documentation: `#NUM!`. Reference engine: a coefficient of
  1, i.e. straight declining balance with no acceleration at all — a coefficient that appears
  nowhere in Microsoft's table.
- **Life in [4, 5).** Documentation: `#NUM!`. Reference engine: 1.5, extending the "between 3
  and 4" row upward.

The Handbook records this as an open documentation-versus-reference-engine divergence. It is
also the cheapest thing on this page to settle empirically: one cell with `rate` = 0.22 (life
≈ 4.55) returns either a number or `#NUM!`, and that single observation decides it.

## Arguments

`AMORDEGRC(cost, date_purchased, first_period, salvage, period, rate, [basis])`

| Argument | Meaning | Required? |
|---|---|---|
| `cost` | The cost of the asset. | Required |
| `date_purchased` | The date of purchase. | Required |
| `first_period` | The date of the **end of the first period**. | Required |
| `salvage` | The salvage value at the end of the asset's life. | Required |
| `period` | The period. **0-based**: 0 is the prorated first period. | Required |
| `rate` | The rate of depreciation — the *un*-accelerated annual rate, whose reciprocal is the life. | Required |
| `basis` | The year basis. | Optional, defaults to 0 |

`rate` carries two jobs at once: it is the base depreciation rate *and*, through `1/rate`, the
asset's life in periods. Changing `rate` therefore moves the schedule length and the coefficient
band simultaneously, which is why small changes to it can produce large, discontinuous changes
to the answer.

**The basis table for this function is not the usual one.** Microsoft's Learn page gives:

| `basis` | Date system |
|---|---|
| 0 or omitted | 360 days (NASD method) |
| 1 | Actual |
| 3 | 365 days in a year |
| 4 | 360 days in a year (European method) |

**There is no `basis` = 2.** Every other function in the financial category — see
[ACCRINTM](FUNC.ACCRINTM.md), [COUPDAYS](FUNC.COUPDAYS.md), [YEARFRAC](FUNC.YEARFRAC.md) —
documents a five-row table with 2 = actual/360. This pair of French-accounting functions
documents four rows and skips 2. The reference engine agrees with the documentation here: it
rejects `basis` = 2 with `#NUM!`. So this is a genuine, documented irregularity in the shared
`basis` axis rather than a documentation error, and any code that validates `basis` centrally as
"0 to 4" will accept an input these two functions reject.

Date arguments are Excel date serials in numeric slots; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns a `Number`: the depreciation charge for the requested period, in the units of `cost`.

- **`period` = 0 is the first period**, and it is the only prorated one. Asking for period 1 on
  an asset bought mid-period returns a full-period charge, not the prorated one.
- **Fractional `period`.** The reference engine floors it, and returns zero for a `period`
  strictly between 0 and 1. The documentation says nothing about fractional periods.
- **The schedule terminates.** Once the book value has fallen to `salvage` (or below zero), the
  reference engine returns 0 for every later period rather than an error — matching the
  documented stopping rule.
- **`salvage` = `cost`** leaves nothing to depreciate; the reference engine still computes a
  first charge from `cost` and the effective rate, then immediately runs out of book value.
- **`date_purchased` after `first_period`** is rejected; the proration fraction would be
  negative.
- **The rounding.** Under the reference engine each period's charge is rounded to a whole unit,
  half away from zero, and the *rounded* value is what reduces the book value. Two
  implementations that agree on the mathematics and differ on this will diverge by growing
  amounts across the schedule, not by a last bit.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md).

## Errors

Documented on Microsoft's Learn page — one condition only, and it is the coefficient-band
sentence quoted above: a life between 0 and 1, 1 and 2, 2 and 3, or 4 and 5 returns `#NUM!`.
No other error condition is documented there.

The reference engine's conditions, recorded as the reference engine's:

| Error | Condition (reference engine) |
|---|---|
| `#NUM!` | `cost` ≤ 0, `salvage` < 0, or `salvage` > `cost` |
| `#NUM!` | `cost`, `salvage` or `rate` is not finite |
| `#NUM!` | `rate` ≤ 0 or `rate` ≥ 0.5 (a life of two periods or less) |
| `#NUM!` | `basis` truncates to anything other than 0, 1, 3 or 4 — **including 2** |
| `#NUM!` | `period` is negative or not finite |
| `#NUM!` | `date_purchased` is later than `first_period` |
| `#VALUE!` | A date argument is not finite, or truncates outside the representable date range |
| propagated | An error value in any argument surfaces as that error |

Note that the reference engine's `#NUM!` band (`rate` ≥ 0.5) and the documented `#NUM!` bands
are different sets, as set out above.

## Relationships

- **[AMORLINC](FUNC.AMORLINC.md)** — the same function without the acceleration. Microsoft
  describes the pair explicitly: `AMORDEGRC` "is similar to AmorLinc, except that a depreciation
  coefficient is applied in the calculation depending on the life of the assets." Same
  arguments, same proration, same four-row basis table; different charge per period.
- **[DB](FUNC.DB.md)** — the Anglo-American fixed-declining-balance function. It also prorates a
  partial first year, but through a `month` argument rather than a pair of dates, and its rate
  is derived from the salvage ratio rather than supplied. `DB` is period-1-based;
  `AMORDEGRC` is period-0-based.
- **[DDB](FUNC.DDB.md)** and `VDB` — declining balance with an explicit factor. `VDB` is the
  closest in spirit, since it also handles partial periods, but it prorates by period number
  rather than by calendar date.
- **`SLN`, `SYD`** — the straight-line and sum-of-years-digits schedules, with no proration at
  all.
- **Confused with**: `AMORLINC` (above), and with `DDB` — the coefficient here is *not* the
  `factor` argument of `DDB`; it is derived from `rate` and cannot be supplied.

## Numerical notes

1. **The coefficient is a step function, so the result is discontinuous in `rate`.** Crossing
   a band boundary changes the answer by a finite amount, not by an epsilon. Any implementation
   that compares `1/rate` against 3, 5 or 6 in floating point is deciding those boundaries with
   a comparison that a rate like `0.2` (life exactly 5) will exercise — and `1.0/0.2` is exactly
   5 in binary64, while `1.0/0.15` is not exactly 6.667. Pin the comparison, and state whether
   the boundary is inclusive.
2. **The per-period rounding compounds.** Rounding half away from zero at each step and carrying
   the rounded book value forward is a different function from rounding only the reported
   answer. The reference engine does the former.
3. **Two 30/360 routines live in this reference engine.** The proration fraction under `basis` 0
   uses this family's own US 30/360 routine, which applies the end-of-month collapse *in place*
   when the start day is already 30 or later. The bond and coupon families use a shared routine
   that instead rolls the end date into the following month when the end day is 31 and the start
   day is below 30. Those two routines return different day counts on that input class. The
   Handbook's research record documents exactly this failure mode elsewhere in the same engine.
   For a reader, the practical consequence is that `AMORDEGRC`'s basis-0 proration and
   `COUPDAYBS`'s basis-0 accrual are **not** guaranteed to be the same day count.
4. **The stopping rule needs the whole schedule.** There is no closed form for period *n*: the
   book value depends on every rounded charge before it, so the implementation must iterate.
   That makes the function `O(period)` and makes early-period errors permanent.
5. `1/rate` is used both as a coefficient selector and, ceilinged, as the schedule length. Those
   two uses can disagree for a `rate` whose reciprocal is barely above an integer.

## What has not been checked

No Handbook vector suite exists for `AMORDEGRC`, and **no evidence record lists this surface in
its subjects** — nor does any record name its family module. Nobody has checked this function
against Excel within the Handbook's record. The battery on this page is the reference engine
answering its own probes; no Excel was involved.

Two divergences are published above rather than resolved: the coefficient bands, and the
per-period rounding. Both are decidable with a handful of cells.

Inputs worth probing first:

1. **`rate` = 0.22 (life ≈ 4.55) and `rate` = 0.4 (life = 2.5).** The documentation says
   `#NUM!` for both; the reference engine returns numbers, with coefficients 1.5 and 1
   respectively. Two cells settle the largest divergence on this page.
2. **`rate` = 0.2 and `rate` = 1/6 exactly** — the band boundaries at life 5 and life 6, where
   inclusive and exclusive readings differ.
3. **A non-integer `cost`** such as 2400.37, over the whole schedule. If Excel rounds each
   charge as the reference engine does, the charges will be whole numbers; if it does not, they
   will not. One column of output answers the question.
4. **`period` = 0 versus `period` = 1** with `date_purchased` well inside the first period,
   confirming the 0-based indexing and locating the prorated charge.
5. **`basis` = 2.** Documentation omits it and the reference engine rejects it; Excel's answer
   is the whole question, and it is a one-cell probe.
6. **Basis 0 with `date_purchased` on the 31st and `first_period` on a month end**, compared
   against `COUPDAYBS` on the same dates — the probe that would expose the two-30/360-routines
   issue described above.
7. **The last two periods of a full schedule**, checking the documented 50 per cent and 100 per
   cent steps and where exactly they land relative to `1/rate`.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| amortissement dégressif | The French declining-balance depreciation regime this function implements |
| depreciation coefficient | The statutory multiplier applied to `rate`, selected by the asset's life |
| life | `1 / rate`, the number of periods over which the asset depreciates |
| proration | Scaling the first period's charge by the fraction of that period the asset was held |
| book value | Cost less the depreciation already taken, the base for the next charge |

## Sources

- Microsoft Learn, **WorksheetFunction.AmorDegrc method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.amordegrc>. Source
  of the parameter table, the four-row basis table (with no `basis` = 2), the depreciation
  coefficient table, the "50 percent … 100 percent" sentence, the stopping-rule sentence, the
  "provided for the French accounting system" statement, and the `#NUM!` life-range sentence
  quoted above.
- Microsoft Learn, **WorksheetFunction.AmorLinc method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.amorlinc>. Cited for
  the identical parameter list and basis table.
- Microsoft 365 support, **AMORDEGRC function** —
  <https://support.microsoft.com/en-us/office/amordegrc-function-a14d0ca1-64a4-42eb-9b3d-b0dededf9e51>.
  The worksheet-surface page; **not retrieved** for this entry (the host returned HTTP 403), so
  nothing here is quoted from it.
- Handbook, [YEARFRAC](FUNC.YEARFRAC.md) — the day-count basis axis and the 30/360 rules.
- Handbook, `content/lastbit/MATERIAL_W109_PRIMITIVES.md` §21 — the recorded case of two
  30/360 routines in one engine disagreeing on month-end pairs.
- Handbook chapters [01 the value universe](../model/01-value-universe.md),
  [02 coercion and lifting](../model/02-coercion-and-lifting.md),
  [06 claim language](../model/06-claim-language.md).
- OxFunc `crates/oxfunc_core/src/functions/amor_depreciation_family.rs` and
  `crates/oxfunc_core/src/functions/day_count_common.rs` at commit `473efa3` — the reference
  engine's coefficient bands, validation, rounding and the two distinct US 30/360 routines.
- `data/functions/FUNC.AMORDEGRC.json`, `data/presence/FUNC.AMORDEGRC.json`,
  `data/battery/FUNC.AMORDEGRC.json`.
