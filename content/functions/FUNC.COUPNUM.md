---
schema: efh.function-page/v1
function_id: FUNC.COUPNUM
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn: WorksheetFunction.CoupNum method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.coupnum"
    role: "the description including the rounding-up clause, the parameter list, the basis table, the truncation rule and the four documented error conditions"
  - work: "Microsoft 365 support: COUPNUM function"
    locator: "https://support.microsoft.com/en-us/office/coupnum-function-a90af57b-de53-4969-9c99-dd6139db2522"
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
family: coupon_family
role_in_family: >-
  The counter: how many coupon dates remain on the ladder, and the member that fixes the number
  of terms in every price and duration sum built on the family.
---

## What it computes

`COUPNUM(settlement, maturity, frequency, [basis])` returns **the number of coupons payable
between the settlement date and the maturity date**. Microsoft's Learn page adds the decisive
clause: "**rounded up to the nearest whole coupon**".

That clause is the whole definition. A bond settled part-way through a coupon period still has
that period's coupon ahead of it, so a partial period counts as a whole one. Equivalently, on
the schedule described on [COUPNCD](FUNC.COUPNCD.md#the-coupon-schedule) — coupon dates counted
backwards from `maturity` in `12/frequency`-month steps — `COUPNUM` is the number of schedule
dates strictly after `settlement`, up to and including `maturity`.

It is therefore always a positive integer, and it is the number of terms in the cash-flow sums
that `PRICE`, `YIELD`, `DURATION` and `MDURATION` evaluate. An off-by-one here is an off-by-one
coupon in every one of them.

The relationship to the rest of the family:

>     COUPNUM = 1 + (whole coupon periods between COUPNCD and maturity)

so the first of the counted coupons falls on [COUPNCD](FUNC.COUPNCD.md) and the last on
`maturity`.

## Arguments

`COUPNUM(settlement, maturity, frequency, [basis])` — the family's shared four arguments, with
Microsoft's wording quoted on [COUPDAYBS](FUNC.COUPDAYBS.md#arguments).

| Argument | Meaning | Required? |
|---|---|---|
| `settlement` | The security's settlement date. | Required |
| `maturity` | The security's maturity date. | Required |
| `frequency` | Coupon payments per year: 1, 2 or 4. | Required |
| `basis` | The day-count basis, 0–4. | Optional, defaults to 0 |

The standard five-row basis table is documented on this page — 0 or omitted = US (NASD) 30/360,
1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360.

**`basis` cannot change the answer.** Counting coupon dates is calendar arithmetic; no
day-count convention adds or removes a date from the schedule. `basis` is nevertheless validated
and an out-of-range value turns a well-defined count into an error — the same
non-participating-but-fatal argument that appears on [COUPNCD](FUNC.COUPNCD.md) and
[COUPPCD](FUNC.COUPPCD.md).

Microsoft documents that **"All arguments are truncated to integers."**

## Result and edge cases

Returns a `Number`: a positive integer.

- **`settlement` exactly on a coupon date** returns one fewer coupon under the reference engine
  than a settlement one day earlier, because the schedule rolls forward and that coupon is
  treated as already passed. Microsoft's page does not state this. The alternative reading —
  that a coupon dated exactly on the settlement date is still "payable between settlement and
  maturity" — gives a different count, and the difference is a whole coupon in every downstream
  price.
- **`settlement` one day before maturity** returns 1: the maturity coupon.
- **The answer is independent of `basis`** on every input where it is defined.
- **Long-dated bonds** return large counts; the reference engine finds the period by walking the
  schedule one step at a time, so its cost is proportional to the answer.
- **`settlement` ≥ `maturity`** is a documented error, which is why the answer is never 0.
- **Month-end maturities** change *which* dates the coupons fall on but not how many there are,
  except at the boundary where a shifted date crosses `settlement`.
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

- **[COUPNCD](FUNC.COUPNCD.md)** — the date of the first of the counted coupons, and the page
  that hosts the shared schedule description.
- **[COUPPCD](FUNC.COUPPCD.md)** — the date of the coupon *not* counted, immediately before
  `settlement`.
- **[COUPDAYS](FUNC.COUPDAYS.md)**, **[COUPDAYBS](FUNC.COUPDAYBS.md)**,
  **[COUPDAYSNC](FUNC.COUPDAYSNC.md)** — the day counts within the first of the remaining
  periods. `COUPNUM` counts whole periods; those three subdivide the current one.
- **`PRICE`, `YIELD`, `DURATION`, `MDURATION`** — the consumers. Each evaluates a sum with
  `COUPNUM` terms, discounted at exponents built from `COUPDAYSNC / COUPDAYS` plus whole
  periods.
- **`NPER`** — the annuity-count function, which answers a superficially similar question
  (how many periods) from rates and payments rather than from dates. The two are not
  substitutes.
- **Confused with**: `(maturity − settlement) / 365 × frequency` rounded up, which agrees with
  `COUPNUM` most of the time and disagrees exactly where the schedule's month arithmetic
  matters.

## Numerical notes

1. **The answer is an integer and should be computed as one.** Deriving it from a year fraction
   and rounding is the classic way to introduce an off-by-one at period boundaries: a
   floating-point year fraction that lands a hair below an integer rounds the wrong way.
2. **Walk or index, but decide.** Counting schedule steps and computing a month-difference
   quotient give different answers under the clamping month rule when `maturity` is a month end.
   The reference engine walks.
3. **The roll-forward branch is the only boundary that matters**, and it changes the answer by a
   whole unit. It is decided before any counting, and it is shared with the rest of the family —
   one probe fixes all six functions.
4. **Cost is linear in the answer.** For a synthetic instrument maturing centuries out, an
   iterating implementation does real work; a consumer computing `COUPNUM` inside a solver loop
   will feel it.
5. **Validate `basis` and ignore it.** There is nothing for a day-count routine to do here.

## What has not been checked

No Handbook vector suite exists for `COUPNUM`, and **no evidence record lists this surface in
its subjects**. The shared `coupon_family` module is named by no record either. Nobody has
checked this function against Excel within the Handbook's record. The battery on this page is
the reference engine answering its own probes; no Excel was involved.

The documented "rounded up to the nearest whole coupon" clause pins the interior behaviour
firmly. What it does not pin is the boundary — a settlement landing exactly on a coupon date —
and that is where the whole-unit difference lives.

Inputs worth probing first:

1. **`settlement` exactly on a coupon date, and one day either side.** Three cells, and the
   middle one decides whether the reference engine's roll-forward convention is Excel's. The
   same three cells constrain `COUPPCD`, `COUPNCD` and `COUPDAYBS`.
2. **`settlement` one day before `maturity`**, which must return 1 under any reading and is a
   cheap sanity anchor for a future vector suite.
3. **Month-end maturities** — 31 January, 28/29 February, 31 August — at each `frequency`, with
   settlements placed so that the shifted schedule date lands on either side of them. This is
   where walking and indexing diverge.
4. **A 29 February maturity** at `frequency` 1, where the schedule dates in non-leap years are a
   modelling choice.
5. **`COUPNUM` against a hand count of `COUPNCD` iterated forward by `EDATE`**, which isolates
   the month rule from the counting rule.
6. **Very long-dated maturities**, checking that the count matches a direct month-difference
   computation.
7. **An out-of-range `basis`**, confirming that an argument which cannot affect the answer can
   still destroy it.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| remaining coupons | Schedule dates strictly after `settlement`, up to and including `maturity` |
| rounded up | Microsoft's rule that a partial coupon period still counts as a whole coupon |
| coupon schedule | The backward ladder of dates generated from `maturity`; see COUPNCD |
| roll forward | The reference engine's treatment of a settlement landing exactly on a coupon date |
| term count | The number of cash flows a price or duration sum must evaluate |

## Sources

- Microsoft Learn, **WorksheetFunction.CoupNum method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.coupnum>. Source of
  the description including the "rounded up to the nearest whole coupon" clause, the parameter
  table, the five-row basis table, the "All arguments are truncated to integers" rule, the
  date-entry warning, and the four error conditions.
- Microsoft 365 support, **COUPNUM function** —
  <https://support.microsoft.com/en-us/office/coupnum-function-a90af57b-de53-4969-9c99-dd6139db2522>.
  The worksheet-surface page; **not retrieved** for this entry (the host returned HTTP 403), so
  nothing here is quoted from it.
- Handbook, [COUPNCD](FUNC.COUPNCD.md#the-coupon-schedule) — the shared schedule construction.
- Handbook chapters [01 the value universe](../model/01-value-universe.md),
  [02 coercion and lifting](../model/02-coercion-and-lifting.md),
  [06 claim language](../model/06-claim-language.md).
- OxFunc `crates/oxfunc_core/src/functions/coupon_family.rs` at commit `473efa3` — the reference
  engine's schedule walk, remaining-coupon count, roll-forward rule and validation.
- `data/functions/FUNC.COUPNUM.json`, `data/presence/FUNC.COUPNUM.json`,
  `data/battery/FUNC.COUPNUM.json`.
