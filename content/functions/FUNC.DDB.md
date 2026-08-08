---
schema: efh.function-page/v1
function_id: FUNC.DDB
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn: WorksheetFunction.Ddb method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.ddb"
    role: "the parameter list, the factor default, the Min(...) depreciation formula, the all-arguments-positive remark, and the pointer to VDB for the straight-line switch"
  - work: "Microsoft Learn: WorksheetFunction.Db method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.db"
    role: "the sibling's derived-and-rounded rate, cited for the comparison that defines this function's role"
  - work: "Microsoft 365 support: DDB function"
    locator: "https://support.microsoft.com/en-us/office/ddb-function-519a7a37-8772-4c96-85c0-ed2c209717a5"
    role: "the worksheet-surface page; cited but not retrieved — the host returned HTTP 403 to the Handbook's fetch"
episodes: []
body_sections:
  - What it computes
  - The clamp, and why DDB never switches
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
  The explicit-rate declining-balance member: the factor is given rather than derived, the charge
  is clamped at the salvage floor, and the schedule never switches to straight line — that being
  VDB's job.
---

## What it computes

`DDB(cost, salvage, life, period, [factor])` returns the depreciation charge for **one period**
under the double-declining-balance method, or any other declining rate you name.

Microsoft's Learn page gives the rule in one line:

>     Min( (cost − total depreciation from prior periods) × (factor / life),
>          (cost − salvage − total depreciation from prior periods) )

Writing `book` for `cost` minus the depreciation already taken, that is:

>     charge = min( book × factor / life,  book − salvage )

The first term is the declining-balance charge: a constant fraction `factor/life` of the
remaining book value. With the default `factor` = 2 that fraction is twice the straight-line
rate, which is where "double declining" comes from — the method writes off twice as fast at the
start and tapers.

The second term is a **clamp**: the charge may never take the book value below `salvage`.

Two properties define the method, and both matter:

1. **The rate is given, not derived.** Unlike [DB](FUNC.DB.md), which solves for the rate that
   lands exactly on `salvage`, `DDB` applies whatever `factor/life` you specify and lets the
   clamp deal with the consequences.
2. **`salvage` enters only through the clamp.** It does not affect the declining-balance term at
   all, so for early periods `DDB` returns the same number whatever `salvage` is.

## The clamp, and why DDB never switches

Declining balance has a well-known defect: `book × factor/life` shrinks geometrically and never
reaches zero, so the asset is never fully written off within its life. Standard accounting
practice is to **switch to straight line** for the remaining periods once straight line would
give the larger charge.

**`DDB` does not do that.** Microsoft is explicit: "Use the VDB function if you want to switch to
the straight-line depreciation method when depreciation is greater than the declining balance
calculation." `DDB` runs pure declining balance, clamped at `salvage`, for the whole life.

So a `DDB` schedule summed over `1 … life` does **not** generally reach `cost − salvage`. It
approaches it geometrically and stops short — unless the clamp cuts in near the end, which
happens only when `salvage` is large enough for the geometric tail to reach it. This is the
single most common surprise in the depreciation category, and it is documented behaviour, not a
defect.

The comparison that makes the family legible:

| | rate | `salvage` used for | switches to straight line | first-period proration |
|---|---|---|---|---|
| `SLN` | implicit, flat | the whole charge | n/a | none |
| [DB](FUNC.DB.md) | derived from the salvage ratio, rounded to 3 dp | deriving the rate | no | `month` |
| `DDB` | `factor / life`, supplied | a clamp only | **no** | none |
| `VDB` | `factor / life`, supplied | a clamp only | **optional** | period range |
| `SYD` | implicit, declining | the whole charge | n/a | none |

## Arguments

`DDB(cost, salvage, life, period, [factor])`

| Argument | Meaning (Microsoft's wording) | Required? |
|---|---|---|
| `cost` | "The initial cost of the asset." | Required |
| `salvage` | "The value at the end of the depreciation … **This value can be 0.**" | Required |
| `life` | "The number of periods over which the asset is being depreciated." | Required |
| `period` | "The period for which you want to calculate the depreciation. Period must use the same units as life." | Required |
| `factor` | "The rate at which the balance declines. If factor is omitted, it is assumed to be 2 (the double-declining balance method)." | Optional, defaults to 2 |

**`period` is 1-based**, like [DB](FUNC.DB.md) and unlike the French pair
[AMORLINC](FUNC.AMORLINC.md) / [AMORDEGRC](FUNC.AMORDEGRC.md).

**`period` and `life` must be in the same units.** Monthly depreciation of a five-year asset
means `life` = 60 and `period` in months — and `factor/life` shrinks accordingly, which is the
point.

**A documented contradiction.** Microsoft's Remarks open with "Important: All five arguments must
be positive numbers", while the parameter table for `salvage` says "This value can be 0". The two
statements are on the same page and cannot both hold. The reference engine follows the parameter
table: it accepts `salvage` = 0, and accepts `cost` = 0 as well, rejecting only negative values —
so it contradicts the Remarks line for two of the five arguments. The Handbook records the
contradiction and adopts neither side as a statement about Excel.

Numeric slots follow the shared model; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns a `Number`: the charge for the requested period, in the units of `cost`.

- **`salvage` is invisible early.** For any period where `book × factor/life` is below
  `book − salvage`, the answer does not depend on `salvage` at all. A spreadsheet in which
  changing `salvage` changes nothing is behaving correctly.
- **The clamp can zero the charge.** Once the book value has fallen to `salvage`, every later
  period returns 0 rather than an error.
- **`factor` > `life`** makes `factor/life` exceed 1, so the first period would write off more
  than the whole asset; the clamp reduces it to `cost − salvage`. Nothing in the documentation
  forbids it.
- **Fractional `period`** is accepted by the reference engine, which prorates within the period:
  it evaluates the whole-period charge and takes the fraction of it covered by the interval from
  `period − 1` to `period`. This is `VDB`'s machinery applied to a unit interval. The
  documentation says nothing about fractional periods.
- **Fractional `life`** is accepted; the final partial period is shorter than 1 and its charge is
  prorated accordingly.
- **`period` > `life`** is rejected by the reference engine. The documentation states no such
  condition.
- **The schedule does not exhaust `cost − salvage`**, per the section above. This is not an edge
  case; it is the normal outcome.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md).

## Errors

**Microsoft's Learn page documents no error values and no error conditions** — only the
"all five arguments must be positive numbers" remark, which is contradicted on the same page.

The reference engine's conditions, recorded as the reference engine's and not as Excel's:

| Error | Condition (reference engine) |
|---|---|
| `#NUM!` | `cost` < 0, `salvage` < 0, `life` ≤ 0, or `factor` ≤ 0 |
| `#NUM!` | `period` ≤ 0 or `period` > `life` |
| `#NUM!` | Any argument is not finite |
| `#VALUE!` | An argument cannot be coerced to a number |
| propagated | An error value in any argument surfaces as that error |

Note that `salvage` > `cost` is **not** rejected: the clamp term `book − salvage` is then
negative from the start, the `min` selects it, and the reference engine's non-negativity guard
turns it into 0. So an over-salvaged asset silently depreciates by nothing rather than erroring.
Undocumented, and worth a probe.

## Relationships

- **`VDB`** — the general form. `VDB(cost, salvage, life, period−1, period, factor, TRUE)` is
  `DDB(cost, salvage, life, period, factor)`: same clamp, same rate, but `VDB` takes a period
  *range* and its final argument controls whether the schedule switches to straight line.
  Microsoft's `DDB` page points at `VDB` explicitly for that switch. In the reference engine both
  functions call one interval kernel, with `DDB` passing the no-switch flag.
- **[DB](FUNC.DB.md)** — the other declining-balance function, which derives and rounds its rate
  and prorates the first year by a `month` count. See the comparison table above.
- **`SLN`** — the straight-line charge `(cost − salvage)/life`. Comparing `DDB` against `SLN`
  period by period is how you find where a switching method would switch, and therefore where
  `DDB` starts under-charging relative to accounting practice.
- **`SYD`** — sum-of-years-digits: also accelerated, but it *does* exhaust `cost − salvage`
  exactly.
- **[AMORDEGRC](FUNC.AMORDEGRC.md)** — the French accelerated method. Its acceleration
  coefficient looks like `factor` but is derived from `rate` and cannot be supplied, and its
  last two periods are forced to 50 and 100 per cent. Not a substitute.
- **Confused with**: `VDB` (above), and with the assumption that a `DDB` column sums to
  `cost − salvage`.

## Numerical notes

1. **This is a recurrence, and the reference engine evaluates it as one.** Computing period *k*
   requires the book value after *k*−1 charges, so the cost is `O(period)` and every earlier
   rounding is inherited. In exact arithmetic the unclamped book value after *k* periods is
   `cost × (1 − factor/life)^k`, and an implementation could jump straight to it — giving
   different last bits, and giving *wrong* answers once the clamp has been active, because the
   closed form does not know about it.
2. **The clamp is a `min` and therefore a branch.** Near the crossover the two terms are close,
   and which one wins is decided by a floating-point comparison. Two implementations that agree
   on both terms to the last bit can still disagree on the answer at exactly one period.
3. **`factor / life` is one division**, not a per-period recomputation. Forming it once and
   multiplying is not the same as multiplying by `factor` and dividing by `life` each period;
   pin the choice.
4. **No transcendentals.** Unlike [DB](FUNC.DB.md), which needs a general power, `DDB` is
   multiplication, subtraction and comparison throughout. It is the most numerically
   straightforward member of the family — which makes it a good calibration surface for a future
   vector suite, since any disagreement is semantic rather than about a library `pow`.
5. **The fractional-period proration is an extension**, not a documented behaviour. It reuses the
   `VDB` interval logic on a unit interval, which is a defensible reading and is not the only
   one.

## What has not been checked

No Handbook vector suite exists for `DDB`, and **no evidence record lists this surface in its
subjects**. The shared `depreciation_family` module — which also implements `DB`, `SLN`, `SYD`
and `VDB` — is named by no record either. Nobody has checked this function against Excel within
the Handbook's record. The battery on this page is the reference engine answering its own probes;
no Excel was involved.

The documented formula is unusually precise for this category, so the open questions here are
almost entirely about the domain: what is rejected, and what happens at the boundaries the
formula does not describe.

Inputs worth probing first:

1. **`salvage` = 0**, which the parameter table permits and the Remarks line forbids. One cell
   decides which half of Microsoft's page Excel implements.
2. **`cost` = 0**, the same question for the other argument the reference engine admits.
3. **`salvage` > `cost`**, where the clamp is negative from the start and the reference engine
   returns 0 rather than erroring.
4. **`factor` > `life`**, where the first period would over-depreciate and only the clamp
   prevents it.
5. **The period where the clamp first binds**, and the period either side of it — the branch
   boundary, and the one place where two correct-looking implementations diverge.
6. **A whole schedule summed against `cost − salvage`**, quantifying the shortfall the
   no-switch rule produces. This is the number that surprises accountants, and it is a good
   invariant for a future suite to report.
7. **A fractional `period` such as 2.5, and a fractional `life`**, neither documented.
8. **`DDB(c, s, l, p, f)` against `VDB(c, s, l, p−1, p, f, TRUE)`**, which should be an identity
   and which localizes any disagreement to one of the two functions.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| declining balance | A constant fraction of the falling book value charged each period |
| book value | Cost less depreciation already taken; the base for the next charge |
| factor | The multiplier on the straight-line rate; `factor/life` is the declining rate |
| clamp | The `min` that stops the charge taking book value below `salvage` |
| switch to straight line | The accounting practice `DDB` deliberately omits; `VDB` provides it |

## Sources

- Microsoft Learn, **WorksheetFunction.Ddb method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.ddb>. Source of the
  parameter table with its quoted descriptions (including "This value can be 0" for `salvage`),
  the `factor` default of 2, the `Min(...)` formula quoted above, the "All five arguments must be
  positive numbers" remark recorded here as contradicting that table, and the pointer to `VDB`
  for the straight-line switch. This page documents no error values.
- Microsoft Learn, **WorksheetFunction.Db method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.db>. Cited for the
  sibling's derived, rounded rate in the comparison above.
- Microsoft 365 support, **DDB function** —
  <https://support.microsoft.com/en-us/office/ddb-function-519a7a37-8772-4c96-85c0-ed2c209717a5>.
  The worksheet-surface page; **not retrieved** for this entry (the host returned HTTP 403), so
  nothing here is quoted from it.
- Handbook chapters [01 the value universe](../model/01-value-universe.md),
  [02 coercion and lifting](../model/02-coercion-and-lifting.md),
  [06 claim language](../model/06-claim-language.md),
  [07 implementation options](../model/07-implementation-options.md).
- OxFunc `crates/oxfunc_core/src/functions/depreciation_family.rs` at commit `473efa3` — the
  shared declining-interval kernel, the no-switch flag, the clamp, the fractional-period
  proration and the validation rules attributed to the reference engine above.
- `data/functions/FUNC.DDB.json`, `data/presence/FUNC.DDB.json`, `data/battery/FUNC.DDB.json`.
