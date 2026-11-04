---
schema: efh.function-page/v1
function_id: FUNC.PRICEMAT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-FIN-0032
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Notes
  - Page vocabulary
  - Sources
family: bond_core_family
role_in_family: >-
  The at-maturity price: one interest payment, one simple discount, no coupon schedule. The bond
  family's closed-form member, and the one whose three day counts are not independent of each
  other.
---

# PRICEMAT

## What it computes

`PRICEMAT(settlement, maturity, issue, rate, yld, [basis])` returns the **clean price per 100 of
face value** of a security that pays all of its interest at maturity. Interest accrues at `rate`
from `issue`, the buyer settles part-way through, and the whole of it arrives with the principal.

With `B` the year length under the chosen `basis`, and three day counts

- `A`, from issue to settlement,
- `DIM`, from issue to maturity,
- `DSM`, from settlement to maturity,

the price is

    PRICEMAT = (100 + (DIM/B) × rate × 100) / (1 + (DSM/B) × yld) − (A/B) × rate × 100

Microsoft's page prints its equation as an image that could not be read, and defines the four
terms in words. The expression above is the one the notes below record from Excel's own answers.

Read from left to right, it has three parts:

- `100 + (DIM/B) × rate × 100` is the single payment at maturity: principal plus all interest since
  issue.
- Dividing by `1 + (DSM/B) × yld` discounts that payment back to settlement at simple interest, not
  compound. The yield applies only to the span the buyer actually holds.
- `(A/B) × rate × 100` removes the interest already accrued, which is paid to the seller separately.
  What is left is the price quoted without accrued interest.

The interest is simple throughout. A security with `rate = yld` does not price at exactly 100, even
though the two rates agree, because the numerator earns from issue and the denominator discounts
only from settlement. Microsoft's example shows this: settlement 15 February 2008, maturity 13
April 2008, issue 11 November 2007, rate 6.1%, yield 6.1%, basis 0. Here `A = 94`, `DIM = 152`,
`DSM = 58`, and

    (100 + 152/360 × 6.1) / (1 + 58/360 × 0.061) − 94/360 × 6.1 = 99.98449…

which Microsoft shows as $99.98.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `settlement` | The date the buyer purchases the security, after its issue date. Required. | — |
| `maturity` | The date the security expires. Required. | — |
| `issue` | The security's issue date. Required. | — |
| `rate` | The security's interest rate at the issue date, as a decimal fraction. Required. | — |
| `yld` | The security's annual yield, as a decimal fraction. Required. | — |
| `basis` | The day-count basis. Optional. | 0 |

Note the order: `issue` is the **third** argument, after `maturity`, although it comes first in time.
A call that passes the dates chronologically puts `issue` in the settlement slot, and the
`#NUM!` it then produces says nothing about which argument is out of place.

The documented basis values are the standard five, as on the [DISC](FUNC.DISC.md) page: `0` or
omitted = US (NASD) 30/360, `1` = Actual/actual, `2` = Actual/360, `3` = Actual/365, `4` = European
30/360. All three dates and `basis` are documented as truncated to integers.

Microsoft's example labels `rate` "Percent semiannual coupon". Nothing in the function is
semiannual: there is one interest payment, at maturity. The label is best ignored.

There is no redemption argument. The security redeems at 100 plus interest, and the page's result is
per 100 of face value.

Numeric coercion of the argument slots follows the shared call model; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`, a price per 100 of face value, excluding accrued interest.

- **`yld = 0` is admissible.** The documented refusal is `yld < 0`, and a recorded row with `yld = 0`
  returns a number: 105.472… for a 5% rate with 394 days to run on basis 0. With zero yield there is
  no discounting, and the result is 100 plus the interest still to come.
- **`rate = 0`** is admissible under Microsoft's wording, which refuses only `rate < 0`. It reduces
  the formula to a plain simple-yield discount of 100. No recorded row exercises it.
- **`issue` on or after `settlement`** is not on Microsoft's list, but see Errors below.
- **The fractional part of a date is dropped.** 208 of the 2,404 rows behind the notes carry a
  fractional date, and each fits the integer day counts.
- Empty, missing and error arguments are expected to follow the shared call model; see
  [The value universe](../model/01-value-universe.md). No reading records this for `PRICEMAT`.

## Errors

As documented on Microsoft's `PRICEMAT` page, with what the recorded rows add (Excel 16.0 build
20430, Windows 64-bit, AMD Ryzen 7 4800H):

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | `settlement`, `maturity` or `issue` is not a valid date | Documented; not measured |
| `#NUM!` | `rate < 0` or `yld < 0` | Documented; one row with `rate < 0` |
| `#NUM!` | `basis < 0` or `basis > 4` | Documented; not measured |
| `#NUM!` | `settlement ≥ maturity` | Documented; one row with equal dates |
| `#NUM!` | `issue` after `settlement` | Not documented; one row |

Microsoft's list says nothing about the order of `issue` and `settlement`, and the page checks
`issue` only for validity. One recorded row, with issue 22 January 2010 and settlement 14 October
2009, returns `#NUM!`. Both statements are shown because they differ. That row does not settle
`issue = settlement`.

The reference engine at commit `473efa3` requires `issue < settlement < maturity` strictly and
returns `#NUM!` otherwise, so it also refuses `issue = settlement`. It refuses a date serial below 1
or past the end of year 9999 with `#VALUE!`, a lower limit that does not match what the notes on
[YEARFRAC](FUNC.YEARFRAC.md) record for serial 0 in Excel. Whether `PRICEMAT` accepts serial 0 has
not been read.

## Relationships

- **`YIELDMAT`** — the inverse: given the price, recover `yld`. The formula is linear in
  `1 + (DSM/B) × yld`, so it can be solved in closed form.
- **[ACCRINTM](FUNC.ACCRINTM.md)** — the accrued-interest term in the same form, over the same span:
  `ACCRINTM(issue, settlement, rate, 100, basis)`. Whether the two always take the same year length
  at basis 1 has not been compared.
- **`PRICE`** — the same question for a security with periodic coupons, with the yield compounded
  over a schedule.
- **[PRICEDISC](FUNC.PRICEDISC.md)** — a security that pays no interest and is quoted at a discount.
  `PRICEMAT` is interest-bearing and quoted at a yield.
- **[RECEIVED](FUNC.RECEIVED.md)** and [INTRATE](FUNC.INTRATE.md) — the discount-family functions
  for fully invested securities, which have no accrual term and no issue date.
- **[YEARFRAC](FUNC.YEARFRAC.md)** and [DAYS360](FUNC.DAYS360.md) — the day counts exposed directly.

## Numerical notes

**The three day counts are not three counts.** Microsoft defines `DSM` as the days from settlement
to maturity, but Excel does not count them separately. In the recorded rows, `DSM` is `DIM − A`.
Actual-day bases cannot tell the difference, and at basis 0 they can: six rows have `DIM − A`
different from a direct 30/360 count from settlement to maturity, and all six fit `DIM − A`. The
reference engine at commit `473efa3` also subtracts.

**The 30/360 February rule.** The second note below records that at basis 0 `PRICEMAT` tests for an
end on the 31st before it tests for a start on the last day of February, the reverse of the usual
US (NASD) order. The size of the effect is in that note. Only one row separates the two rules, and
the note marks the order of the tests as inferred. The reference engine at commit `473efa3` uses
the standard order for this count, so it does not agree with that row. All basis 4 rows fit the
standard European count.

**The year length at basis 1.** Excel's `B` comes from the span from *issue to settlement*, not
from settlement to maturity. All 526 basis 1 rows fit this rule, to at least ten significant
figures:

- for a span of at most one year, if both dates are in one calendar year, `B` is that year's
  length; otherwise `B` is 366 if a 29 February lies in the span and 365 if not;
- for a span of more than one year, `B` is the average length of the calendar years from the issue
  year to the settlement year, both included.

Taking `B` over issue to maturity instead fits 71 of the 526, and over settlement to maturity 59.
The reference engine at commit `473efa3` uses the issue-to-settlement rule.

**The order of operations.** The first note below gives the order Excel uses for the whole
expression. The reference engine at commit `473efa3` has the same grouping of the three parts,
evaluated in ordinary binary64. The first note records how few rows that difference shows on, and
this page does not repeat those counts.

**Conditioning.** Nothing cancels badly here for ordinary inputs. The final subtraction removes a
term much smaller than the first unless the accrual period is long and the rate is high, and the
denominator stays at or above 1 for any admissible yield. The reference engine's guard against a
non-positive denominator cannot be reached with `yld ≥ 0`.

## What has not been checked

One evidence record names this surface in its subjects: **EV-FIN-0032**, which holds a claim about
the reference engine that has no denominator. Its fields and warnings render mechanically beside
this page and are not restated here. It predates the notes below, which carry a count of their own
for Excel's arithmetic.

No Handbook vector suite exists for `PRICEMAT`; `vectors/` publishes nothing at this revision. What
is measured is what the notes below record, plus the row readings stated on this page, all on one
Excel build. The following is still unmeasured.

1. **`issue = settlement`.** The one row on this order has issue strictly after settlement. The
   reference engine refuses equality, and that is not a statement about Excel.
2. **`yld < 0`, `basis` outside 0 to 4, and invalid dates.** The rows reach `rate < 0` and
   `settlement = maturity` once each. The other documented conditions have no row.
3. **`rate = 0`.** No row has it, and the documented wording admits it.
4. **Dates before 1990.** The rows run from 1990 to 2056. The note on the fictitious 29 February
   1900 does not list `PRICEMAT`, and serial 0 has not been read for it.
5. **The basis 0 test order beyond one row.** A second row that separates the two orders would
   settle what the note leaves as inferred.
6. **Fractional `basis`.** Every row has an integer basis.
7. **Non-numeric arguments.** Logicals, text dates and empty cells have no reading for `PRICEMAT`,
   and an argument left empty in the call and an array in any slot have none either.
8. **Other Excel builds, platforms and bitness.** The rows are from one build on one 64-bit Windows
   machine.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `PRICEMAT` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-PRICEMAT-001: PRICEMAT formula order on the x87 unit

*Detail for matching Excel's bits; measured; recorded 2026-11-04.*

PRICEMAT = (100 + (DIM/B)*rate*100) / (1 + (DSM/B)*yld) - (A/B)*rate*100, each operation taken
from the x87 unit at precision 64 and stored to binary64.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `PRICEMAT(38620, 42889, 38168, 0.12180991686638161, 0.019384437069992936, 2)` | settlement `38620` (`0x40e2db8000000000`); maturity `42889` (`0x40e4f12000000000`); issue `38168` (`0x40e2a30000000000`); rate `0.12180991686638161` (`0x3fbf2eef494503b5`); yld `0.019384437069992936` (`0x3f93d9838d1165c1`); basis `2` (`0x4000000000000000`) (cells) | `195.89977581174702` (`0x40687ccaf6a4a5ee`) | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |

Basis (measured): 2,404 of 2,404 recorded rows, with the day counts of FN-PRICEMAT-002. Strict
binary64 with the same day counts matches 2,400.

Evidence:

- DnaCalc `crates/calc-func/tests/f3_financial.rs`, lines 435-466 at `26bb1800d3` — recorded answers pinned
- DnaCalc-Research `evidence/round23/pricemat-hard/summary.json`, counts at `f177408286` — day-count rule against precision
- OxFunc-Evidence `cache/oracle-20430/build-20430/PRICEMAT.jsonl.gz`, line 2282, key PRICEMAT|0x40e2db8000000000|0x40e4f12000000000|0x40e2a30000000000|0x3fbf2eef494503b5|0x3f93d9838d1165c1|0x4000000000000000 at `e26b7bddb8` (readings)

### FN-PRICEMAT-002: PRICEMAT basis 0 applies the 31st rule before the February rule

*Quirk; measured; recorded 2026-11-04.*

For basis 0, PRICEMAT tests "end on the 31st gives 30" before "start on the last day of
February gives 30"; the US (NASD) 30/360 rule applies the February adjustment first. An issue
on 28 February 2025 and maturity on 31 March 2043 gives DIM 6511, not 6510, and moves the price
at about 1E-5 relative.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `PRICEMAT(46376, 52321, 45716, 0.04392789506562811, 0.03266239139487676, 0)` | settlement `46376` (`0x40e6a50000000000`); maturity `52321` (`0x40e98c2000000000`); issue `45716` (`0x40e6528000000000`); rate `0.04392789506562811` (`0x3fa67db79160c3a0`); yld `0.03266239139487676` (`0x3fa0b91ffdb34689`); basis `0` (`0x0000000000000000`) (cells) | `109.22025565957247` (`0x405b4e18ab31a7da`) | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |

Basis (measured): One recorded row separates this rule from the standard one: 2,404 of 2,404
against 2,403.

Inferred, not measured: The order of the two tests is taken from the published
ExcelFinancialFunctions getMatFactors rule, which fits; one row cannot pin it alone.

Evidence:

- DnaCalc-Research `evidence/round23/pricemat-hard/summary.json`, counts at `f177408286` — day-count rule against precision
- DnaCalc `crates/calc-func/tests/f3_financial.rs`, lines 435-466 at `26bb1800d3` — recorded answers pinned
- OxFunc-Evidence `cache/oracle-20430/build-20430/PRICEMAT.jsonl.gz`, line 1455, key PRICEMAT|0x40e6a50000000000|0x40e98c2000000000|0x40e6528000000000|0x3fa67db79160c3a0|0x3fa0b91ffdb34689|0x0000000000000000 at `e26b7bddb8` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| clean price | A price quoted without the interest accrued since issue |
| accrued interest | The interest earned from issue to settlement, paid to the seller |
| simple interest | Interest proportional to time, with no compounding |
| day-count basis | The `basis` argument's calendar convention, one of the five documented methods |
| A / DIM / DSM | Days from issue to settlement, issue to maturity, and settlement to maturity |
| B | The year length under the chosen basis |
| recorded rows | Excel's answers to generated inputs, which the notes below cite; a row fits a rule when it agrees to the stated precision |

## Sources

- Microsoft, "PRICEMAT function" —
  <https://support.microsoft.com/en-us/office/pricemat-function-52c3b4da-bc7e-476a-989f-a95f675cae77>
  (syntax, argument meanings, the day-count basis table, the serial-date and truncation remarks,
  the `#VALUE!` and `#NUM!` conditions, the definitions of `B`, `DSM`, `DIM` and `A`, and the worked
  example). The equation on that page is an image and was not readable, so no formula here is
  sourced from it.
- The function notes on this page, and the recorded Excel rows and readings their evidence lists
  cite. The error rows, the `yld = 0` row, the fractional-date count, `DSM = DIM − A` and the basis 1
  year-length rule were read from those 2,404 rows for this page and go beyond the notes' own text;
  they carry the same Excel build, platform and CPU.
- Handbook evidence record `EV-FIN-0032` — the record naming this surface.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook, [Claim language and honesty](../model/06-claim-language.md) — the scoping rules this
  page's claims are written under.
- OxFunc `crates/oxfunc_core/src/functions/bond_core_family.rs` and
  `crates/oxfunc_core/src/functions/day_count_common.rs` at commit `473efa3` (the reference
  engine's `pricemat_kernel`, its `days_in_year_for_mat` year length, its day-count routines and
  its date, rate and basis guards).
- Handbook projections `data/functions/FUNC.PRICEMAT.json` and
  `data/presence/FUNC.PRICEMAT.json` (arity, classification axes, implementing module and sibling
  set).
