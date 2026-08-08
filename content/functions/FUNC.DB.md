---
schema: efh.function-page/v1
function_id: FUNC.DB
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn: WorksheetFunction.Db method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.db"
    role: "the parameter list, the month default, the rate formula with its three-decimal rounding, and the first-period and last-period formulas"
  - work: "Microsoft Learn: WorksheetFunction.Ddb method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.ddb"
    role: "the sibling's declining-balance formula and factor default, cited for the comparison"
  - work: "Microsoft 365 support: DB function"
    locator: "https://support.microsoft.com/en-us/office/db-function-354e7d28-5f93-4ff1-8a52-eb4ee549d9d7"
    role: "the worksheet-surface page; cited but not retrieved — the host returned HTTP 403 to the Handbook's fetch"
episodes: []
body_sections:
  - What it computes
  - The rounded rate
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: depreciation_family
role_in_family: >-
  The fixed-declining-balance member: the only one that derives its rate from the salvage ratio,
  rounds that rate to three decimals, and prorates a partial first year by a month count.
---

## What it computes

`DB(cost, salvage, life, period, [month])` returns the depreciation charge for **one period**
under the fixed-declining-balance method.

"Fixed" means the *rate* is fixed, not the charge: a constant fraction of the remaining book
value is written off each period, so charges fall geometrically. Microsoft's Learn page states
the whole schedule in three formulas.

The ordinary period:

>     charge = (cost − total depreciation from prior periods) × rate
>     where rate = 1 − (salvage / cost) ^ (1 / life),  rounded to three decimal places

The first period, prorated by the number of months the asset was held in the first year:

>     charge = cost × rate × month / 12

And the last period — the extra stub period that exists only when `month` < 12:

>     charge = ((cost − total depreciation from prior periods) × rate × (12 − month)) / 12

Those three formulas are quoted from Microsoft. Together they define a schedule of `life`
periods when `month` is 12, and `life + 1` periods otherwise: the first and last are partial
years that sum to one full year of depreciation.

The **derivation of `rate`** is the idea worth pausing on. Solving
`salvage = cost × (1 − rate)^life` for `rate` gives exactly the formula above: it is the constant
rate that would depreciate the asset from `cost` down to precisely `salvage` over `life` periods.
`DB` is therefore the declining-balance method that *targets* the salvage value, rather than one
that stops when it reaches it.

Except that it does not quite reach it, because of the rounding.

## The rounded rate

`rate` is **rounded to three decimal places** before any charge is computed. Microsoft documents
this explicitly, and it is not a display convention: the rounded rate is what the schedule uses,
so it changes every number the function returns.

Two consequences follow, and both are visible in ordinary use:

1. **The schedule does not land on `salvage`.** The exact rate would; a rate rounded to three
   decimals will not. A `DB` schedule summed over its whole life leaves a residual book value
   that is above or below `salvage` depending on which way the rounding went. Anyone reconciling
   a `DB` column against `cost − salvage` will find a discrepancy, and it is by construction.
2. **The function is a step function of its inputs.** Nudge `salvage`, `cost` or `life` slightly
   and `rate` may not change at all — or may jump by 0.001, moving every subsequent charge. `DB`
   is therefore piecewise-constant in a way none of its siblings are, and small input errors
   produce either no error or a discontinuous one.

Microsoft does not say which rounding rule applies at a tie. The reference engine rounds half
away from zero. Half-to-even would give a different rate on exactly the inputs where the third
decimal is a tie, and those inputs are reachable — a `salvage/cost` ratio engineered to put the
fourth decimal of `1 − ratio^(1/life)` at 5 is not exotic. This is an open question, and it is
the cheapest one on the page to settle.

## Arguments

`DB(cost, salvage, life, period, [month])`

| Argument | Meaning (Microsoft's wording) | Required? |
|---|---|---|
| `cost` | "The initial cost of the asset." | Required |
| `salvage` | "The value at the end of the depreciation (sometimes called the salvage value of the asset)." | Required |
| `life` | "The number of periods over which the asset is being depreciated (sometimes called the useful life of the asset)." | Required |
| `period` | "The period for which you want to calculate the depreciation. Period must use the same units as life." | Required |
| `month` | "The number of months in the first year. If month is omitted, it is assumed to be 12." | Optional, defaults to 12 |

**`period` is 1-based.** Period 1 is the first (possibly prorated) year. This is the opposite of
[AMORLINC](FUNC.AMORLINC.md) and [AMORDEGRC](FUNC.AMORDEGRC.md), which number depreciation
periods from 0, and mixing the two conventions in one model is a reliable way to produce a
plausible wrong schedule.

**`month` is not a date.** It is a count of months in the first *year*, so it prorates by
twelfths regardless of the calendar. `DB` therefore needs no day-count `basis` and has none —
which is exactly why it is not interchangeable with the French depreciation pair, which prorate
by an actual date interval.

**`month` < 12 adds a period.** The schedule then runs to `life + 1`, with the last period
taking the complementary `(12 − month)/12` fraction. Asking for `period` = `life + 1` with
`month` = 12 is out of range; asking for it with `month` = 11 is the stub year.

`salvage` and `cost` enter only through their **ratio**, so scaling both by the same factor
leaves `rate` unchanged and scales every charge linearly.

Numeric slots follow the shared model; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns a `Number`: the charge for the requested period, in the units of `cost`.

- **`salvage` = 0.** The ratio is 0, `0^(1/life)` is 0, and the rate is exactly 1 — so the first
  period writes off the entire asset (prorated by `month`) and later periods charge nothing. The
  formula is doing what it says; the method simply has no meaning when the target salvage is
  zero. Microsoft's page does not flag this. If Excel special-cases it, that is a divergence
  worth recording.
- **`salvage` close to `cost`** gives a rate near 0, which rounds to 0.000 and produces a
  schedule of zeros. Again the rounding, not the mathematics.
- **`period` = `life` + 1 with `month` < 12** is the stub year and uses the last-period formula.
- **`period` > that** is rejected by the reference engine. Microsoft documents no such condition.
- **Fractional `period`** is accepted by the reference engine, which interpolates linearly
  between the charge for the floor period and the charge for the next one. Nothing in the
  documentation covers fractional periods, and linear interpolation inside a geometric schedule
  is a choice, not a derivation.
- **Fractional `life`** is accepted; the "last full regular period" is taken as its floor, which
  moves where the stub-year formula applies.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md).

## Errors

**Microsoft's Learn page documents no error conditions for this function** — it carries the
parameter table and the three formulas, and stops. There is no published statement of what
happens with a negative `cost`, a `month` of 13, or a `period` beyond the schedule.

The reference engine's conditions, recorded as the reference engine's and not as Excel's:

| Error | Condition (reference engine) |
|---|---|
| `#NUM!` | `cost` ≤ 0, `salvage` < 0, or `life` ≤ 0 |
| `#NUM!` | `period` ≤ 0 |
| `#NUM!` | `month` ≤ 0 or `month` > 12 |
| `#NUM!` | `period` > `life` (when `month` = 12) or > `life` + 1 (when `month` < 12) |
| `#NUM!` | Any argument is not finite, or the derived rate is not finite |
| `#VALUE!` | An argument cannot be coerced to a number |
| propagated | An error value in any argument surfaces as that error |

Note that `salvage` > `cost` is **not** rejected: the ratio exceeds 1, the power is above 1, and
the rate goes negative — producing negative depreciation, that is, appreciation. Whether Excel
admits that is unverified, and the documentation is silent.

## Relationships

- **[DDB](FUNC.DDB.md)** — the other declining-balance function. The contrast is sharp and worth
  learning: `DDB` takes the rate as a `factor` and derives nothing; `DB` derives the rate from
  the salvage ratio and rounds it. `DDB` clamps each charge so the book value never falls below
  `salvage`; `DB` has no clamp and relies on the rate to land near it. `DDB` has no
  first-year proration; `DB` has `month`.
- **`VDB`** — the general declining-balance function, with a rate `factor`, a period *range*, and
  an optional switch to straight line. The most flexible member of the family.
- **`SLN`** — straight line, `(cost − salvage)/life`, the flat comparison every declining-balance
  schedule is measured against.
- **`SYD`** — sum-of-years-digits, an accelerated schedule that does exhaust `cost − salvage`
  exactly, unlike `DB`.
- **[AMORDEGRC](FUNC.AMORDEGRC.md)** and **[AMORLINC](FUNC.AMORLINC.md)** — the French
  date-prorated pair. They answer the same accounting question with a date interval instead of a
  `month` count and with 0-based periods.
- **Confused with**: `DDB` (above), and with the assumption that a `DB` schedule sums to
  `cost − salvage`. It does not.

## Numerical notes

1. **The rounding of `rate` dominates everything.** Three decimal places is coarse: on a
   ten-period asset the difference between a rate of 0.2065 and its rounded 0.207 compounds
   through the whole schedule. Any implementation must round at exactly the same point — after
   forming `1 − ratio^(1/life)`, before the first charge — and with the same tie rule.
2. **`ratio^(1/life)` is a general power** and is the only transcendental in the function. It
   is evaluated as `exp(ln(ratio)/life)` on most platforms, so `DB` inherits whatever `pow` the
   host provides — the same dependency the Handbook's research record traced through the bond and
   distribution families to a single shared library routine. The rounding to three decimals
   mercifully hides the last-bit differences, but only until the fourth decimal sits near a tie.
3. **The schedule is a recurrence, not a closed form**, because "total depreciation from prior
   periods" is accumulated. In exact arithmetic the book value after *k* full periods is
   `cost × (1 − rate)^k`, and an implementation could use that instead — giving different last
   bits from the accumulating loop. The reference engine accumulates.
4. **The stub-year branch is decided by `floor(life)`**, so a fractional `life` moves it. This is
   an implementation decision with no documentary support.
5. **Fractional-`period` interpolation is a modelling choice.** Linear interpolation between two
   geometric charges is neither the geometric interpolant nor an accounting convention; it is
   what the reference engine does.

## What has not been checked

No Handbook vector suite exists for `DB`, and **no evidence record lists this surface in its
subjects**. The shared `depreciation_family` module — which also implements `DDB`, `SLN`, `SYD`
and `VDB` — is named by no record either. Nobody has checked this function against Excel within
the Handbook's record. The battery on this page is the reference engine answering its own probes;
no Excel was involved.

The documented content of this function is unusually complete — three formulas and a rounding
rule — and its documented *error* content is empty. That shape determines the probe list: the
formulas need spot confirmation, and everything about the domain needs discovery.

Inputs worth probing first:

1. **A tie at the third decimal of `rate`.** Choose `cost`, `salvage` and `life` so that
   `1 − (salvage/cost)^(1/life)` is a hair either side of `x.xxx5`, and read the first period's
   charge. This settles the rounding rule, which is the largest single lever on every answer.
2. **`salvage` = 0**, where the rate is exactly 1 and the first period writes off the asset.
   One cell distinguishes "the formula as documented" from "a special case".
3. **`salvage` > `cost`**, giving a negative rate. Undocumented, unclamped in the reference
   engine, and either an error or negative depreciation in Excel.
4. **`period` = `life` + 1 with `month` = 12 and with `month` = 11**, which pins where the
   schedule ends and whether the stub year exists.
5. **`month` = 0 and `month` = 13**, neither documented.
6. **A fractional `period` such as 2.5**, where the reference engine interpolates linearly and
   nothing is documented.
7. **A whole schedule summed against `cost − salvage`**, which quantifies the rounding residual
   and gives a future vector suite a natural invariant to report.
8. **`DB` against a hand-built `cost × (1 − rate)^(k−1) × rate` closed form**, which distinguishes
   an accumulating loop from a closed-form evaluation in the last bits.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| fixed-declining balance | A constant *rate* applied to a falling book value; charges decline geometrically |
| book value | Cost less depreciation already taken; the base for the next charge |
| derived rate | `1 − (salvage/cost)^(1/life)`, rounded to three decimals |
| stub year | The extra `life + 1`-th period that exists only when `month` < 12 |
| 1-based period | `period` 1 is the first year, unlike the French `AMOR*` pair |

## Sources

- Microsoft Learn, **WorksheetFunction.Db method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.db>. Source of the
  parameter table with its quoted descriptions, the `month` default, the ordinary-period formula
  with the rate derivation and its three-decimal rounding, and the first-period and last-period
  formulas. This page documents no error conditions.
- Microsoft Learn, **WorksheetFunction.Ddb method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.ddb>. Cited for the
  sibling's formula and defaults in the comparison above.
- Microsoft 365 support, **DB function** —
  <https://support.microsoft.com/en-us/office/db-function-354e7d28-5f93-4ff1-8a52-eb4ee549d9d7>.
  The worksheet-surface page; **not retrieved** for this entry (the host returned HTTP 403), so
  nothing here is quoted from it.
- Handbook, `content/lastbit/MATERIAL_W109_PRIMITIVES.md` §19 — the recorded identification of a
  single shared host `pow` routine underneath several unrelated Excel function families.
- Handbook chapters [01 the value universe](../model/01-value-universe.md),
  [02 coercion and lifting](../model/02-coercion-and-lifting.md),
  [06 claim language](../model/06-claim-language.md).
- OxFunc `crates/oxfunc_core/src/functions/depreciation_family.rs` at commit `473efa3` — the
  reference engine's rate derivation and rounding, schedule accumulation, stub-year branch,
  fractional-period interpolation and validation.
- `data/functions/FUNC.DB.json`, `data/presence/FUNC.DB.json`, `data/battery/FUNC.DB.json`.
