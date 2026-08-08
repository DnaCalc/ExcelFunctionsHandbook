---
schema: efh.function-page/v1
function_id: FUNC.COUPNCD
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn: WorksheetFunction.CoupNcd method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.coupncd"
    role: "the parameter list, the basis table, the truncation rule and the four documented error conditions"
  - work: "Microsoft 365 support: COUPNCD function"
    locator: "https://support.microsoft.com/en-us/office/coupncd-function-fd962fef-506b-4d9d-8590-16df5393691f"
    role: "the worksheet-surface page; cited but not retrieved — the host returned HTTP 403 to the Handbook's fetch"
episodes: []
body_sections:
  - What it computes
  - The coupon schedule
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
  The forward end of the schedule, and the family's canonical construction: every other member
  is a measurement taken against the coupon dates this function returns.
---

## What it computes

`COUPNCD(settlement, maturity, frequency, [basis])` returns **the next coupon date strictly
after the settlement date** — as a date serial, which Excel's date formatting renders as a date.

Microsoft's Learn page describes it as returning "a number that represents the next coupon date
after the settlement date", and declares the return type as `Double`. It is a date-valued
function returning a plain number.

The whole content of the function is the schedule it derives, so this page hosts the
description that the other five coupon functions refer to.

## The coupon schedule

None of the six coupon functions is given a coupon schedule. They are given a **maturity date**
and a **frequency**, and they reconstruct the schedule from those two facts:

>     the coupon dates are  maturity,  maturity − p,  maturity − 2p,  maturity − 3p, …
>     where p = 12 / frequency  months

Coupon dates are therefore anchored at *maturity*, counted **backwards**, and the schedule is
infinite in the backward direction — there is no issue date to stop it. `settlement` selects a
period from that infinite ladder: the one whose start is at or before `settlement` and whose end
is after it.

Three consequences do most of the work on all six pages:

1. **The day of the month comes from maturity, not from settlement.** A bond maturing on the
   15th has coupons on the 15th. Everything else about the trade is irrelevant to the schedule.
2. **Month arithmetic needs a month-end rule.** Stepping back six months from 31 August must land
   somewhere in February, and "the 31st" does not exist there. The reference engine's rule is
   *month-end sticks to month-end*: if the anchor date is the last day of its month, every
   generated date is the last day of its month; otherwise the day number is clamped down to the
   length of the target month. That rule makes the schedule from a 30 June maturity differ from
   the schedule from a 30 September maturity — one is a month end, the other is not.
3. **Settlement exactly on a coupon date rolls forward.** Under the reference engine, if
   `settlement` lands on a generated coupon date, that date is treated as the *start* of the
   current period rather than its end. The practical effect is that `COUPPCD` returns
   `settlement` itself, `COUPDAYBS` returns zero, `COUPNCD` returns the following coupon date,
   and `COUPNUM` counts one fewer remaining coupon. Microsoft's pages do not state this
   convention; it is the reference engine's, and it is the single most testable unknown in the
   family.

The schedule is *quasi*-periodic, not periodic: because the month-end rule clamps, stepping back
`n` periods is not always the same as stepping back one period `n` times from an intermediate
date. Implementations that step iteratively and implementations that compute the offset in one
jump can disagree.

**None of this is documented.** Microsoft's pages for all six functions document the arguments,
the basis table, the truncation rule and four error conditions, and say nothing about how the
schedule is generated. The construction above is the reference engine's, read from its source,
and it is the natural reading of the market convention — but "natural" is not "verified".

## Arguments

`COUPNCD(settlement, maturity, frequency, [basis])` — the same four arguments as every other
member of the family.

| Argument | Meaning | Required? |
|---|---|---|
| `settlement` | The security's settlement date — "the date after the issue date when the security is traded to the buyer". | Required |
| `maturity` | The security's maturity date — "the date when the security expires". | Required |
| `frequency` | Coupon payments per year. Microsoft: "For annual payments, frequency = 1; for semiannual, frequency = 2; for quarterly, frequency = 4." | Required |
| `basis` | The day-count basis, 0–4. | Optional, defaults to 0 |

The argument descriptions are quoted from Microsoft's Learn page.

`basis` is documented on this page with the standard five-row table:

| `basis` | Day count basis |
|---|---|
| 0 or omitted | US (NASD) 30/360 |
| 1 | Actual/actual |
| 2 | Actual/360 |
| 3 | Actual/365 |
| 4 | European 30/360 |

**`basis` cannot change this function's answer.** A coupon date is a calendar date; no day-count
convention moves it. `COUPNCD` nevertheless accepts `basis` and validates it, so an out-of-range
`basis` turns a perfectly well-defined date into an error. That is a real, observable dependency
on an argument that does not participate in the computation — worth knowing when a `COUPNCD`
formula starts erroring after someone edits a shared basis cell.

Microsoft documents that **"All arguments are truncated to integers."** Dates should be produced
by `DATE` or by other formulas rather than typed as text; ordinary coercion rules are in
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns a `Number`: an Excel date serial. Whether Excel attaches a date-format presentation hint
to the result — the mechanism described in
[The value universe](../model/01-value-universe.md) for `TODAY` — has not been established here;
the reference engine returns a bare number.

- **`settlement` on a coupon date** returns the *following* coupon date under the reference
  engine, not `settlement` itself. See the schedule section above; this is undocumented.
- **`settlement` one day before maturity** returns `maturity`, which is always a coupon date by
  construction.
- **`settlement` ≥ `maturity`** is a documented error.
- **Month-end maturities** propagate month-end coupon dates: a 31 August maturity at
  `frequency` 2 gives a 28 or 29 February coupon, not a 28th-of-the-month coupon.
- **Very early settlements** are accepted: the schedule extends backwards without limit, so a
  settlement decades before maturity simply selects a period far down the ladder. The reference
  engine walks the schedule one period at a time to find it, so the cost grows with the gap.
- Empty, missing and error arguments follow the shared call model.

## Errors

Documented on Microsoft's Learn page for this function, in these words (it says "generates an
error" without naming a worksheet error value):

| Condition (documented) | Documented outcome |
|---|---|
| `settlement` or `maturity` is not a valid date | error |
| `frequency` is any number other than 1, 2 or 4 | error |
| `basis` < 0 or `basis` > 4 | error |
| `settlement` ≥ `maturity` | error |

The reference engine maps all four onto `#NUM!`, and additionally returns `#NUM!` for a
non-finite argument and for a date serial outside the representable range. Error values in any
argument propagate. The reference engine's choice of `#NUM!` for an invalid date — rather than
the `#VALUE!` that [ACCRINT](FUNC.ACCRINT.md) uses for the same failure in the same engine — is
an internal inconsistency worth noting; the documentation names no code at all, so neither is
contradicted.

## Relationships

- **[COUPPCD](FUNC.COUPPCD.md)** — the backward end of the same period. `COUPPCD` and `COUPNCD`
  bracket `settlement`, and together they define the current coupon period.
- **[COUPDAYBS](FUNC.COUPDAYBS.md)**, **[COUPDAYSNC](FUNC.COUPDAYSNC.md)**,
  **[COUPDAYS](FUNC.COUPDAYS.md)** — the three day counts taken against those two dates. Note
  that the three do not always add up; see the additivity discussion on
  [COUPDAYS](FUNC.COUPDAYS.md).
- **[COUPNUM](FUNC.COUPNUM.md)** — how many coupon dates remain from `settlement` to `maturity`
  on the same ladder.
- **`PRICE`, `YIELD`, `DURATION`, `MDURATION`** — the consumers. They rebuild this same schedule
  internally rather than calling these functions, which is why a schedule disagreement between
  Excel's `COUPNCD` and Excel's `PRICE` is possible in principle and has been observed for the
  *day counts* in this family; see [COUPDAYS](FUNC.COUPDAYS.md).
- **[EDATE](FUNC.EDATE.md)** and **[EOMONTH](FUNC.EOMONTH.md)** — the general-purpose
  month-stepping functions. `EDATE` clamps the day of month; it does **not** implement
  "month-end sticks to month-end". Rebuilding a coupon schedule with `EDATE` therefore
  reproduces `COUPNCD` for ordinary maturities and diverges for month-end ones.
- **Confused with**: `EDATE(maturity, ±n × 12/frequency)`, for exactly that reason.

## Numerical notes

There is no floating-point content here worth the name: the answer is an integer-valued date
serial. The engineering content is all in the schedule.

1. **Iterate or jump, but be consistent.** Stepping back one period at a time and computing the
   period index directly give different answers under a clamping month rule. The reference
   engine iterates.
2. **The month-end predicate is a decision, not a fact.** "Is this date the last day of its
   month?" is cheap; deciding that the answer should *propagate* to every generated date is a
   modelling choice with no documentary support.
3. **Cost is linear in the settlement-to-maturity gap** under an iterating implementation. A
   30-year quarterly bond settled at issue costs 120 iterations, which is fine; a synthetic test
   with a maturity centuries out is not.
4. **Do not route this through a day-count routine.** `basis` must be *validated* and then
   ignored. An implementation that computes the answer via a year fraction will introduce
   basis-dependence into a function that has none.

## What has not been checked

No Handbook vector suite exists for `COUPNCD`, and **no evidence record lists this surface in
its subjects**. The `coupon_family` module that implements all six coupon functions is named by
no record either. Nobody has checked this function against Excel within the Handbook's record.
The battery on this page is the reference engine answering its own probes; no Excel was involved.

The schedule construction described above is the load-bearing unknown for the whole family. It
is also unusually cheap to probe, because the answer is a date rather than a float: a
disagreement is visible without any tolerance discussion.

Inputs worth probing first:

1. **`settlement` exactly on a coupon date.** The reference engine rolls forward. If Excel
   returns `settlement` itself, four of the six functions in this family change their answers at
   once. This is the highest-value single cell on the page.
2. **Month-end maturities**: 31 January, 28/29 February, 30 April, 31 August, at each
   `frequency`, with settlements several periods earlier. This decides "month-end sticks" versus
   "clamp the day number" — the two rules differ on 31 August at `frequency` 2.
3. **A 29 February maturity**, which forces the month-end question and the leap-year question
   together.
4. **Settlement one day before and one day after a generated coupon date**, bracketing the
   roll-forward boundary.
5. **`COUPNCD` against `EDATE(COUPPCD(...), 12/frequency)`** — if the two disagree, the
   disagreement is in the month rule and nowhere else.
6. **An out-of-range `basis` on an otherwise valid call**, confirming that an argument which
   cannot affect the answer can still destroy it.
7. **A settlement many decades before maturity**, to see whether Excel's schedule walk agrees
   with a direct index computation.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| coupon schedule | The backward ladder of dates generated from `maturity` at `12/frequency`-month steps |
| coupon date | Any date on that ladder |
| current period | The schedule period containing `settlement` |
| month-end anchoring | The rule that a month-end anchor generates month-end dates |
| roll forward | The reference engine's treatment of a settlement landing exactly on a coupon date |

## Sources

- Microsoft Learn, **WorksheetFunction.CoupNcd method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.coupncd>. Source of
  the one-line description, the parameter table with its quoted argument descriptions, the
  five-row basis table, the `Double` return type, the "All arguments are truncated to integers"
  rule, the date-entry warning, and the four error conditions.
- Microsoft 365 support, **COUPNCD function** —
  <https://support.microsoft.com/en-us/office/coupncd-function-fd962fef-506b-4d9d-8590-16df5393691f>.
  The worksheet-surface page; **not retrieved** for this entry (the host returned HTTP 403), so
  nothing here is quoted from it.
- Handbook chapters [01 the value universe](../model/01-value-universe.md),
  [02 coercion and lifting](../model/02-coercion-and-lifting.md),
  [06 claim language](../model/06-claim-language.md).
- OxFunc `crates/oxfunc_core/src/functions/coupon_family.rs` at commit `473efa3` — the reference
  engine's schedule generation, month-end anchoring, roll-forward rule and validation, read for
  every statement attributed to the reference engine above.
- `data/functions/FUNC.COUPNCD.json`, `data/presence/FUNC.COUPNCD.json`,
  `data/battery/FUNC.COUPNCD.json`.
