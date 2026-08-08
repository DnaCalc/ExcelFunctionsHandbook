---
schema: efh.function-page/v1
function_id: FUNC.COUPDAYBS
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn: WorksheetFunction.CoupDayBs method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.coupdaybs"
    role: "the parameter list with argument descriptions, the basis table, the truncation rule and the four documented error conditions"
  - work: "Microsoft 365 support: COUPDAYBS function"
    locator: "https://support.microsoft.com/en-us/office/coupdaybs-function-eb9a8dfb-2fb2-4c61-8e5d-690b320cf872"
    role: "the worksheet-surface page; cited but not retrieved — the host returned HTTP 403 to the Handbook's fetch"
episodes: []
body_sections:
  - What it computes
  - How the days are counted
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
  The accrual numerator: days already elapsed in the current coupon period, and the input that
  turns a clean price into an invoice price.
---

## What it computes

`COUPDAYBS(settlement, maturity, frequency, [basis])` returns **the number of days from the
beginning of the current coupon period to the settlement date** — the days the seller has
already earned.

Its whole economic purpose is one quotient:

>     accrued interest  =  coupon  ×  COUPDAYBS / COUPDAYS

which is why the denominator's page, [COUPDAYS](FUNC.COUPDAYS.md), is required reading alongside
this one: the two functions do not measure time the same way on every basis.

The "beginning of the current coupon period" is the previous coupon date, which is exactly what
[COUPPCD](FUNC.COUPPCD.md) returns. The period is derived from the schedule described on
[COUPNCD](FUNC.COUPNCD.md#the-coupon-schedule) — coupon dates counted backwards from `maturity`
in `12/frequency`-month steps.

## How the days are counted

Under the reference engine the count from the previous coupon date to `settlement` is taken on
the `basis`:

| `basis` | How `COUPDAYBS` counts |
|---|---|
| 0 — US (NASD) 30/360 | 30/360-adjusted day count |
| 1 — Actual/actual | actual calendar days |
| 2 — Actual/360 | actual calendar days |
| 3 — Actual/365 | actual calendar days |
| 4 — European 30/360 | 30/360-adjusted day count |

So `COUPDAYBS` *is* a calendar measurement on three of the five bases, and a 30/360 measurement
on the other two. It is never a declared constant — which is precisely where it parts company
with [COUPDAYS](FUNC.COUPDAYS.md), and why

>     COUPDAYBS + COUPDAYSNC  =  COUPDAYS

holds on bases 0, 1 and 4 and **fails on bases 2 and 3**. The full account of that failure, and
of the upstream case where Excel's own `PRICE` and `COUPDAYSNC` disagreed because of it, is on
[COUPDAYS](FUNC.COUPDAYS.md#the-additivity-that-fails).

Microsoft's page documents the basis names and does not state which of them count real days
here; the table above is the reference engine's.

## Arguments

`COUPDAYBS(settlement, maturity, frequency, [basis])`

| Argument | Meaning (Microsoft's wording) | Required? |
|---|---|---|
| `settlement` | "The security's settlement date. The security settlement date is the date after the issue date when the security is traded to the buyer." | Required |
| `maturity` | "The security's maturity date. The maturity date is the date when the security expires." | Required |
| `frequency` | "The number of coupon payments per year. For annual payments, frequency = 1; for semiannual, frequency = 2; for quarterly, frequency = 4." | Required |
| `basis` | "The type of day count basis to use." | Optional, defaults to 0 |

The five-row basis table is documented on this page and is the standard one — 0 or omitted = US
(NASD) 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360.

**There is no issue date argument.** A security issued mid-period does not shorten the first
`COUPDAYBS`: the function measures from the *schedule's* previous coupon date, real or notional,
not from issuance. Accrued interest on a newly issued bond therefore needs
[ACCRINT](FUNC.ACCRINT.md), which does take an issue date, rather than this function.

Microsoft documents that **"All arguments are truncated to integers."** Dates should be produced
by `DATE` or by other formulas rather than typed as text; ordinary coercion rules are in
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns a `Number`: a day count, integral on every basis.

- **`settlement` exactly on a coupon date** returns 0 under the reference engine, because the
  schedule rolls forward and treats that date as the start of the new period. Microsoft's page
  does not state this. If Excel instead reported a full period, every accrual built on this
  function would jump on coupon dates.
- **`settlement` one day after a coupon date** returns 1 on bases 1, 2 and 3, and 1 on the
  30/360 bases too except where the adjustment rules bite.
- **The 30/360 bases can disagree with the calendar by several days**, by design: a period from
  31 January to 31 July counts as 180 days on basis 0 whatever the calendar says.
- **The result is bounded by the period length only on bases 0, 1 and 4.** On bases 2 and 3 the
  count can exceed the `COUPDAYS` the same call reports, because that denominator is a
  convention.
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

- **[COUPDAYS](FUNC.COUPDAYS.md)** — the denominator. Read its additivity section before
  combining the two.
- **[COUPDAYSNC](FUNC.COUPDAYSNC.md)** — the complement, days from settlement forward to the
  next coupon. The pair is not guaranteed to partition the period.
- **[COUPPCD](FUNC.COUPPCD.md)** — the date this function counts from. `settlement − COUPPCD` is
  the *actual* elapsed days, which equals `COUPDAYBS` on bases 1, 2 and 3 and not on 0 or 4.
- **[ACCRINT](FUNC.ACCRINT.md)** — the function to use when accrual starts at issuance rather
  than at a schedule coupon date, or when the interval spans several periods.
- **`PRICE`, `YIELD`** — consumers. `PRICE` returns a clean price; the invoice price adds the
  accrual this function's quotient defines.
- **[YEARFRAC](FUNC.YEARFRAC.md)** — the same day-count conventions, applied to an arbitrary
  date pair instead of to a coupon period.
- **Confused with**: `settlement − COUPPCD(...)` (above), and with `ACCRINT`, which answers a
  different question with a different domain.

## Numerical notes

1. **This is integer arithmetic pretending to be floating point.** Every answer is a whole
   number of days on every basis. An implementation that computes it in integers and converts
   once at the end has no rounding to discuss; one that routes through a year fraction and
   multiplies back does.
2. **Share one 30/360 routine across the whole engine.** The reference engine's bond and coupon
   families share one; its French depreciation family has a second, differently ordered one, and
   the two disagree on month-end pairs. See [AMORLINC](FUNC.AMORLINC.md) and the Handbook's
   research record on that class of divergence.
3. **The order of the two 30/360 adjustments is observable.** Collapsing the 31st to the 30th
   before or after noticing that the start date was a month end changes the answer by a day on a
   nameable input class — and only on that class, which is why it survives large corpora and
   then fails in production.
4. **The roll-forward convention is a branch, not a formula.** Whether `settlement` on a coupon
   date yields 0 or a full period is decided before any counting happens, and it is the largest
   single behavioural unknown on this page.
5. No cancellation, no overflow, no transcendental. The difficulty here is entirely
   definitional.

## What has not been checked

No Handbook vector suite exists for `COUPDAYBS`, and **no evidence record lists this surface in
its subjects**. The shared `coupon_family` module is named by no record either. Nobody has
checked this function against Excel within the Handbook's record. The battery on this page is
the reference engine answering its own probes; no Excel was involved.

Inputs worth probing first:

1. **`settlement` exactly on a coupon date.** Zero or a full period — the answer changes every
   accrual calculation that crosses a coupon, and it is one cell.
2. **`COUPDAYBS + COUPDAYSNC` against `COUPDAYS` at bases 2 and 3**, on a period whose actual
   length is not `360/frequency`. This publishes the additivity failure with three cells.
3. **A period from 31 January to 31 July at bases 0 and 4**, where the two 30/360 conventions
   diverge from each other and from the calendar.
4. **Month-end and 29 February settlements**, where the 30/360 adjustment order is decidable.
5. **`COUPDAYBS` against `settlement − COUPPCD(...)`** on each basis — a two-cell demonstration
   of which bases count real days.
6. **A settlement one day into a period, at every basis and frequency**, as the simplest
   possible fixed point for a future vector suite.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| coupon period | The schedule period containing `settlement`; see COUPNCD |
| accrual | The seller's earned share of the current coupon, `COUPDAYBS / COUPDAYS` |
| 30/360 adjustment | The rules that map a calendar date pair onto a 30-day-month count |
| roll forward | The reference engine's treatment of a settlement landing exactly on a coupon date |
| clean versus invoice price | A quoted price without and with accrued interest added |

## Sources

- Microsoft Learn, **WorksheetFunction.CoupDayBs method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.coupdaybs>. Source
  of the one-line description, the quoted argument descriptions, the five-row basis table, the
  "All arguments are truncated to integers" rule, and the four error conditions.
- Microsoft 365 support, **COUPDAYBS function** —
  <https://support.microsoft.com/en-us/office/coupdaybs-function-eb9a8dfb-2fb2-4c61-8e5d-690b320cf872>.
  The worksheet-surface page; **not retrieved** for this entry (the host returned HTTP 403), so
  nothing here is quoted from it.
- Handbook, [COUPNCD](FUNC.COUPNCD.md#the-coupon-schedule) — the shared schedule construction;
  [COUPDAYS](FUNC.COUPDAYS.md#the-additivity-that-fails) — the additivity failure.
- Handbook, `content/lastbit/MATERIAL_W109_PRIMITIVES.md` §18 and §21 — the recorded day-count
  divergences in the same engine and in Excel's own bond routines.
- Handbook chapters [01 the value universe](../model/01-value-universe.md),
  [02 coercion and lifting](../model/02-coercion-and-lifting.md),
  [06 claim language](../model/06-claim-language.md).
- OxFunc `crates/oxfunc_core/src/functions/coupon_family.rs` and
  `crates/oxfunc_core/src/functions/day_count_common.rs` at commit `473efa3`.
- `data/functions/FUNC.COUPDAYBS.json`, `data/presence/FUNC.COUPDAYBS.json`,
  `data/battery/FUNC.COUPDAYBS.json`.
