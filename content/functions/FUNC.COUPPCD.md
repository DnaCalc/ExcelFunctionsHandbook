---
schema: efh.function-page/v1
function_id: FUNC.COUPPCD
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn: WorksheetFunction.CoupPcd method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.couppcd"
    role: "the parameter list, the basis table, the truncation rule and the four documented error conditions; note that this page carries no description sentence"
  - work: "Microsoft 365 support: COUPPCD function"
    locator: "https://support.microsoft.com/en-us/office/couppcd-function-2eb50473-6ee9-4052-a206-77a9a385d5b3"
    role: "the worksheet-surface page, and the source of the one-line description carried in the Handbook's projection; cited but not retrieved — the host returned HTTP 403 to the Handbook's fetch"
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
family: coupon_family
role_in_family: >-
  The backward end of the schedule: the coupon date accrual is measured from, and the member
  whose Microsoft Learn reference page has lost its description sentence.
---

## What it computes

`COUPPCD(settlement, maturity, frequency, [basis])` returns **the coupon date on or before the
settlement date** — the start of the current coupon period — as an Excel date serial.

It is the mirror of [COUPNCD](FUNC.COUPNCD.md), and the two together bracket the period that
[COUPDAYBS](FUNC.COUPDAYBS.md), [COUPDAYSNC](FUNC.COUPDAYSNC.md) and
[COUPDAYS](FUNC.COUPDAYS.md) measure. The schedule it selects from is described once, on
[COUPNCD](FUNC.COUPNCD.md#the-coupon-schedule): coupon dates run backwards from `maturity` in
steps of `12/frequency` months, with month-end anchoring, and there is no issue date to stop
them.

**The previous coupon date is notional, not historical.** The schedule is generated from
`maturity` alone, so `COUPPCD` will happily return a date before the security existed. That is
the intended behaviour for accrual arithmetic on a seasoned bond, and it is a trap for anyone
using the function to ask "when was the last coupon actually paid".

### A documentation gap worth recording

Microsoft's Learn reference page for this function is **missing its description sentence**. Every
sibling page opens with a one-line statement of what the function returns — "Returns the number
of days from the beginning of the coupon period to the settlement date", and so on. The
`CoupPcd` page goes straight from the title to the Syntax heading. The parameter table, the basis
table, the date notes and the four error conditions are all present; only the sentence saying
what the function is for is absent. The description carried in the Handbook's own projection —
"Returns the previous coupon date before the settlement date" — comes from Microsoft's
worksheet-surface support page, which was not retrieved for this entry.

Note also that the projected description says "**before** the settlement date", while the
reference engine returns `settlement` itself when settlement lands exactly on a coupon date. On
that input the description and the reference engine disagree, and Microsoft's Learn page says
nothing either way.

## Arguments

`COUPPCD(settlement, maturity, frequency, [basis])` — the family's shared four arguments, with
Microsoft's wording quoted on [COUPDAYBS](FUNC.COUPDAYBS.md#arguments).

| Argument | Meaning | Required? |
|---|---|---|
| `settlement` | The security's settlement date. | Required |
| `maturity` | The security's maturity date. | Required |
| `frequency` | Coupon payments per year: 1, 2 or 4. | Required |
| `basis` | The day-count basis, 0–4. | Optional, defaults to 0 |

The standard five-row basis table is documented on this page — 0 or omitted = US (NASD) 30/360,
1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360.

**`basis` cannot change the answer.** A coupon date is a calendar date; no day-count convention
moves it. `basis` is nevertheless validated, so an out-of-range value converts a well-defined
date into an error. The same non-participating-but-fatal argument appears on
[COUPNCD](FUNC.COUPNCD.md) and [COUPNUM](FUNC.COUPNUM.md).

Microsoft documents that **"All arguments are truncated to integers."** Dates should be produced
by `DATE` or by other formulas rather than typed as text; ordinary coercion rules are in
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns a `Number`: an Excel date serial. Whether Excel attaches a date-format presentation hint
to the result — the mechanism described in
[The value universe](../model/01-value-universe.md) — has not been established here; the
reference engine returns a bare number.

- **`settlement` exactly on a coupon date** returns `settlement` itself under the reference
  engine, because the schedule rolls forward and treats that date as the start of the new
  period. This contradicts the "previous coupon date **before** the settlement date" wording of
  the projected description, and Microsoft's Learn page does not resolve it. Probing it is the
  first item below.
- **Month-end maturities** produce month-end previous-coupon dates: a 31 August maturity at
  `frequency` 2 gives 28 or 29 February, not the 28th of some other month.
- **Early settlements** are accepted without limit; the schedule extends backwards past any
  plausible issue date.
- **`settlement` ≥ `maturity`** is a documented error, so the function never returns `maturity`
  itself as a previous coupon date.
- The reference engine clamps a computed previous-coupon serial at zero, so extremely early
  settlements cannot produce a negative serial.
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

The reference engine maps all four onto `#NUM!`, and also returns `#NUM!` for a non-finite
argument or a date serial outside the representable range. Errors in any argument propagate.

## Relationships

- **[COUPNCD](FUNC.COUPNCD.md)** — the forward end of the same period, and the page that hosts
  the shared schedule description.
- **[COUPDAYBS](FUNC.COUPDAYBS.md)** — the day count from this date to `settlement`. On bases 1,
  2 and 3 it equals `settlement − COUPPCD`; on the 30/360 bases it does not.
- **[COUPDAYS](FUNC.COUPDAYS.md)** — the period length. `COUPNCD − COUPPCD` is the *actual*
  length and equals `COUPDAYS` only under basis 1.
- **[COUPNUM](FUNC.COUPNUM.md)** — the count of remaining coupons on the same ladder.
- **[EDATE](FUNC.EDATE.md)** — the general month-stepping function. `EDATE` clamps the day of
  month but does not make month-ends sticky, so `EDATE(maturity, −n × 12/frequency)` reproduces
  `COUPPCD` for ordinary maturities and diverges for month-end ones.
- **[ACCRINT](FUNC.ACCRINT.md)** — the function that does know an issue date, for cases where
  the notional previous coupon date is the wrong accrual anchor.
- **Confused with**: "the date of the last coupon actually paid", which this function does not
  answer for a bond issued after the notional date it returns.

## Numerical notes

1. **The answer is an integer-valued date serial**; there is no floating-point content.
2. **Iterating and indexing differ.** Stepping back one period at a time under a clamping month
   rule is not the same as computing the period index and jumping. The reference engine
   iterates, and its cost grows with the settlement-to-maturity gap.
3. **Month-end anchoring is a modelling decision** with no documentary support, and it is where
   two careful implementations will part company. State the rule explicitly rather than
   inheriting it from whatever a date library happens to do.
4. **Validate `basis` and then ignore it.** Computing this date through a day-count routine
   would introduce a dependency the function does not have.
5. **The roll-forward branch** decides the equality case and should be settled before anything
   else, because it also fixes `COUPDAYBS`'s zero and `COUPNUM`'s count on the same input.

## What has not been checked

No Handbook vector suite exists for `COUPPCD`, and **no evidence record lists this surface in
its subjects**. The shared `coupon_family` module is named by no record either. Nobody has
checked this function against Excel within the Handbook's record. The battery on this page is
the reference engine answering its own probes; no Excel was involved.

Two things on this page are documentation findings rather than measurements: the missing
description sentence on Microsoft's Learn page, and the "before the settlement date" wording that
the reference engine contradicts on the equality case.

Inputs worth probing first:

1. **`settlement` exactly on a coupon date.** Does Excel return `settlement` or the coupon date
   before it? This single observation settles the wording question and simultaneously fixes
   `COUPDAYBS`, `COUPDAYSNC`, `COUPNCD` and `COUPNUM` on the same input.
2. **Month-end maturities** — 31 January, 28/29 February, 30 April, 31 August — at each
   `frequency`, several periods before maturity. This decides month-end anchoring against day
   clamping.
3. **`COUPPCD` against `EDATE(COUPNCD(...), −12/frequency)`**, which isolates the month rule.
4. **A settlement decades before maturity**, testing whether an iterating walk and a direct
   index computation agree.
5. **A 29 February maturity**, combining the month-end and leap-year questions.
6. **An out-of-range `basis`** on an otherwise valid call, confirming that an argument which
   cannot affect the answer can still destroy it.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| previous coupon date | The schedule date on or before `settlement`; the start of the current period |
| notional coupon date | A schedule date generated backwards from `maturity`, whether or not a coupon was paid then |
| month-end anchoring | The rule that a month-end anchor generates month-end dates |
| roll forward | The reference engine's treatment of a settlement landing exactly on a coupon date |
| date serial | The number Excel uses to represent a date; see the value-universe chapter |

## Sources

- Microsoft Learn, **WorksheetFunction.CoupPcd method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.couppcd>. Source of
  the parameter table, the five-row basis table, the `Double` return type, the "All arguments
  are truncated to integers" rule, the date-entry warning, and the four error conditions. This
  page carries **no description sentence**, which is recorded above as a documentation finding.
- Microsoft 365 support, **COUPPCD function** —
  <https://support.microsoft.com/en-us/office/couppcd-function-2eb50473-6ee9-4052-a206-77a9a385d5b3>.
  The worksheet-surface page and the origin of the projected description "Returns the previous
  coupon date before the settlement date"; **not retrieved** for this entry (the host returned
  HTTP 403), so nothing here is quoted from it.
- Handbook, [COUPNCD](FUNC.COUPNCD.md#the-coupon-schedule) — the shared schedule construction.
- Handbook chapters [01 the value universe](../model/01-value-universe.md),
  [02 coercion and lifting](../model/02-coercion-and-lifting.md),
  [06 claim language](../model/06-claim-language.md).
- OxFunc `crates/oxfunc_core/src/functions/coupon_family.rs` at commit `473efa3`.
- `data/functions/FUNC.COUPPCD.json` (the projected description quoted above),
  `data/presence/FUNC.COUPPCD.json`, `data/battery/FUNC.COUPPCD.json`.
