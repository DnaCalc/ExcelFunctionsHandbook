---
schema: efh.function-page/v1
function_id: FUNC.COUPDAYSNC
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn: WorksheetFunction.CoupDaysNc method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.coupdaysnc"
    role: "the parameter list, the basis table, the truncation rule and the four documented error conditions"
  - work: "Microsoft 365 support: COUPDAYSNC function"
    locator: "https://support.microsoft.com/en-us/office/coupdaysnc-function-5ab3f0b2-029f-4a8b-bb65-47d525eea547"
    role: "the worksheet-surface page; cited but not retrieved — the host returned HTTP 403 to the Handbook's fetch"
episodes: []
body_sections:
  - What it computes
  - The function Excel disagrees with itself about
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: coupon_family
role_in_family: >-
  The forward day count: settlement to the next coupon, and the member with a recorded case of
  Excel's own bond pricing publishing a different number for the same quantity.
---

## What it computes

`COUPDAYSNC(settlement, maturity, frequency, [basis])` returns **the number of days from the
settlement date to the next coupon date**.

It is the forward complement of [COUPDAYBS](FUNC.COUPDAYBS.md) and the numerator of the
first discounting exponent in bond pricing: the first coupon is discounted over
`COUPDAYSNC / COUPDAYS` of a period, and every later one over a whole period more.

The next coupon date is what [COUPNCD](FUNC.COUPNCD.md) returns, derived from the schedule
described on [COUPNCD](FUNC.COUPNCD.md#the-coupon-schedule).

Under the reference engine the count is taken on the `basis`, in the same pattern as its
backward sibling:

| `basis` | How `COUPDAYSNC` counts |
|---|---|
| 0 — US (NASD) 30/360 | 30/360-adjusted day count |
| 1 — Actual/actual | actual calendar days |
| 2 — Actual/360 | actual calendar days |
| 3 — Actual/365 | actual calendar days |
| 4 — European 30/360 | 30/360-adjusted day count |

Microsoft's page documents the basis names, not this mapping; the table is the reference
engine's.

## The function Excel disagrees with itself about

`COUPDAYSNC` is the one member of this family where the Handbook's research record contains a
concrete, published case of **Excel returning two different answers to the same question in the
same workbook**.

The quantity "days from settlement to the next coupon" can be obtained two ways:

1. **Directly** — count the days from `settlement` to `COUPNCD`, on the `basis`. This is what
   `COUPDAYSNC` does.
2. **By subtraction** — take the period length and remove the days already accrued:
   `COUPDAYS − COUPDAYBS`.

On the 30/360 bases and on actual/actual these agree, because `COUPDAYS` there is either the
same 30/360 arithmetic or the real calendar length. On **actual/360 and actual/365** they do
not, because `COUPDAYS` reports a declared `360/frequency` or `365/frequency` while `COUPDAYBS`
counts real days. The subtraction then yields a number that no calendar produces.

The Handbook's research record documents Excel's `PRICE` taking route 2 — deriving
days-to-next-coupon from the period length and the accrued days for every basis — while Excel's
`COUPDAYSNC`, on the same bond and the same arguments, publishes route 1. The two numbers differ
by whole days at bases 2 and 3, and the resulting prices differ in the first decimal place, not
in the last bit. Both of Excel's answers are deterministic and reproducible; they are simply
different functions of the same inputs.

For a reader the operative consequence is: **`COUPDAYSNC` is not a safe substitute for whatever
`PRICE` is using internally.** If you are reconciling a price, reconcile against the price, not
against this function's day count.

The general form of this fault line — that `COUPDAYBS + COUPDAYSNC ≠ COUPDAYS` on bases 2 and
3 — is set out on [COUPDAYS](FUNC.COUPDAYS.md#the-additivity-that-fails).

## Arguments

`COUPDAYSNC(settlement, maturity, frequency, [basis])` — the family's shared four arguments,
with Microsoft's wording quoted on [COUPDAYBS](FUNC.COUPDAYBS.md#arguments).

| Argument | Meaning | Required? |
|---|---|---|
| `settlement` | The security's settlement date. | Required |
| `maturity` | The security's maturity date. | Required |
| `frequency` | Coupon payments per year: 1, 2 or 4. | Required |
| `basis` | The day-count basis, 0–4. | Optional, defaults to 0 |

The five-row basis table is documented on this page in the standard form — 0 or omitted = US
(NASD) 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360 — and
Microsoft documents that **"All arguments are truncated to integers."**

There is no issue date and no first-coupon date: like every member of this family, the function
knows only `maturity` and `frequency`, so an odd first period is invisible to it. Odd-period
instruments need `ODDFPRICE` and `ODDFYIELD`, which take the extra dates.

## Result and edge cases

Returns a `Number`: a day count, integral on every basis, and strictly positive.

- **`settlement` exactly on a coupon date.** Under the reference engine the schedule rolls
  forward, so the answer is a whole period rather than zero: the coupon date is treated as the
  start of the new period, not as the next coupon. Microsoft's page does not state this. It is
  the mirror image of the [COUPDAYBS](FUNC.COUPDAYBS.md) boundary, and the two must be probed
  together.
- **`settlement` one day before maturity** returns 1 on the actual bases, since `maturity` is
  always a coupon date.
- **On bases 2 and 3 the answer can exceed `COUPDAYS`** — a "days remaining" larger than the
  reported period length. That is not a bug in this function; it is the additivity failure seen
  from this side.
- **The 30/360 bases** can return a number that no calendar interval matches, by construction.
- **`settlement` ≥ `maturity`** is a documented error.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md).

## Errors

Documented on Microsoft's Learn page for this function, in these words (it says "generates an
error" without naming a worksheet error value):

| Condition (documented) | Documented outcome |
|---|---|
| `settlement` or `maturity` is not a valid date | error |
| `frequency` is any number other than 1, 2 or 4 | error |
| `basis` < 0 or `basis` > 4 | error |
| `settlement` ≥ `maturity` | error |

The reference engine maps all four onto `#NUM!`, and also returns `#NUM!` for a non-finite
argument or a date serial outside the representable range. Errors in any argument propagate.

## Relationships

- **[COUPDAYBS](FUNC.COUPDAYBS.md)** — the backward complement. The pair partitions the period
  only on bases 0, 1 and 4.
- **[COUPDAYS](FUNC.COUPDAYS.md)** — the denominator, and the source of the mismatch described
  above.
- **[COUPNCD](FUNC.COUPNCD.md)** — the date this function counts to. `COUPNCD − settlement` is
  the *actual* remaining days, equal to `COUPDAYSNC` on bases 1, 2 and 3 only.
- **`PRICE`, `YIELD`, `DURATION`, `MDURATION`** — the consumers, and, per the research record,
  not necessarily consumers of *this* function's answer.
- **`ODDFPRICE`, `ODDFYIELD`** — the odd-first-period instruments, which need the extra dates
  this family does not take.
- **Confused with**: `COUPDAYS − COUPDAYBS`, which is a different quantity on bases 2 and 3 and
  is the one Excel's `PRICE` was observed to use.

## Numerical notes

1. **Integer arithmetic throughout.** Every answer is a whole number of days; compute it in
   integers and convert once.
2. **Never derive it by subtraction.** The whole content of the section above is that the
   derived and the direct forms are different functions. An implementation that computes
   `COUPDAYS − COUPDAYBS` will match Excel's `PRICE` and contradict Excel's `COUPDAYSNC`, or the
   reverse — it cannot do both.
3. **Share one 30/360 routine**, and pin the order of its two adjustments; see
   [COUPDAYBS](FUNC.COUPDAYBS.md#numerical-notes).
4. **The roll-forward branch dominates the boundary behaviour** and should be decided and
   documented before any counting code is written.
5. There is no rounding, cancellation or transcendental content in this function. Its
   difficulty is definitional and, unusually for this Handbook, it is a difficulty Excel itself
   demonstrably has.

## What has not been checked

No Handbook vector suite exists for `COUPDAYSNC`, and **no evidence record lists this surface in
its subjects**. The shared `coupon_family` module is named by no record either. The
`PRICE`-versus-`COUPDAYSNC` divergence described above comes from the Handbook's own research
narrative, not from an evidence record attached to this surface, and it concerns what `PRICE`
does — it is not a measurement of this function. Nobody has checked `COUPDAYSNC` itself against
Excel within the Handbook's record. The battery on this page is the reference engine answering
its own probes; no Excel was involved.

Inputs worth probing first:

1. **A bond settled mid-period, at bases 2 and 3, comparing `COUPDAYSNC` with
   `COUPDAYS − COUPDAYBS`.** Three cells, and the disagreement is in whole days. This is the
   most instructive experiment in the entire coupon family.
2. **The same bond's `PRICE` against a hand-built price using each of the two day counts**,
   which identifies which one Excel's pricing routine consumed.
3. **`settlement` exactly on a coupon date**, to settle the roll-forward convention — probed
   simultaneously with `COUPDAYBS`, `COUPPCD`, `COUPNCD` and `COUPNUM`, since one observation
   constrains all five.
4. **Month-end schedules at bases 0 and 4**, where the 30/360 adjustment order is decidable.
5. **Basis 1 across a leap day**, where the period length varies and the forward count is a real
   calendar measurement.
6. **`COUPDAYSNC` against `COUPNCD − settlement`** on each basis, distinguishing the calendar
   bases from the 30/360 ones in two cells.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| next coupon date | The forward end of the current schedule period; what COUPNCD returns |
| derived count | `COUPDAYS − COUPDAYBS`, the subtraction form of this quantity |
| direct count | Days actually counted from `settlement` to the next coupon date |
| additivity | The property `COUPDAYBS + COUPDAYSNC = COUPDAYS`, which fails on bases 2 and 3 |
| roll forward | The reference engine's treatment of a settlement landing exactly on a coupon date |

## Sources

- Microsoft Learn, **WorksheetFunction.CoupDaysNc method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.coupdaysnc>. Source
  of the one-line description, the parameter table, the five-row basis table, the "All arguments
  are truncated to integers" rule, the date-entry warning, and the four error conditions.
- Microsoft 365 support, **COUPDAYSNC function** —
  <https://support.microsoft.com/en-us/office/coupdaysnc-function-5ab3f0b2-029f-4a8b-bb65-47d525eea547>.
  The worksheet-surface page; **not retrieved** for this entry (the host returned HTTP 403), so
  nothing here is quoted from it.
- Handbook, `content/lastbit/MATERIAL_W109_PRIMITIVES.md` §18 — the recorded case of Excel's
  `PRICE` deriving days-to-next-coupon by subtraction and disagreeing with this function on the
  actual/360 and actual/365 bases.
- Handbook, [COUPNCD](FUNC.COUPNCD.md#the-coupon-schedule) and
  [COUPDAYS](FUNC.COUPDAYS.md#the-additivity-that-fails).
- Handbook chapters [01 the value universe](../model/01-value-universe.md),
  [02 coercion and lifting](../model/02-coercion-and-lifting.md),
  [06 claim language](../model/06-claim-language.md).
- OxFunc `crates/oxfunc_core/src/functions/coupon_family.rs` at commit `473efa3`.
- `data/functions/FUNC.COUPDAYSNC.json`, `data/presence/FUNC.COUPDAYSNC.json`,
  `data/battery/FUNC.COUPDAYSNC.json`.
