---
schema: efh.function-page/v1
function_id: FUNC.TBILLYIELD
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-FIN-0025
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
family: discount_bill_yearfrac_family
role_in_family: >-
  The Treasury bill's money-market yield from its price: gain over price, scaled to a 360-day
  year. The plainest of the T-bill functions, with one formula at every span, where TBILLEQ has
  two.
---

# TBILLYIELD

## What it computes

`TBILLYIELD(settlement, maturity, pr)` returns the **yield of a Treasury bill** bought at price `pr`
per 100 of face value and redeemed at 100.

With `DSM` the actual number of days from settlement to maturity:

    TBILLYIELD = (100 − pr) / pr × 360 / DSM

The first factor is the gain as a fraction of the price paid. The second scales it to a year of
360 days. Microsoft's page prints its equation as an image that could not be read and defines `DSM`
in words. The expression above is the one the first note below records from Excel's own answers,
including the grouping `((100 − pr)/pr) × (360/DSM)`.

This is a yield on the **investment**, simple and uncompounded, on a 360-day year. It is not a
discount rate, which would divide by 100 and not by `pr`, and it is not the bond-equivalent yield
that [TBILLEQ](FUNC.TBILLEQ.md) returns, which uses a 365-day year.

The relation to the discount rate `d` of the same bill is exact:

    TBILLYIELD = 360 × d / (360 − d × DSM)

Microsoft's example: settlement 31 March 2008, maturity 1 June 2008, price 98.45. That is 62 actual
days, so

    (100 − 98.45) / 98.45 × 360 / 62 = 0.0914169…

which Microsoft shows as 9.14%. A 30/360 count would be 61 and would give 0.0929, so the example
confirms actual days.

**One formula at every span.** Unlike [TBILLEQ](FUNC.TBILLEQ.md), this function has no long-bill
branch. In the rows behind the notes below (Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7
4800H), all 1,586 numeric rows fit the formula above, including the 738 with `DSM` above 182.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `settlement` | The bill's settlement date, the date after issue when it trades to the buyer. Required. | — |
| `maturity` | The date the bill expires. Required. | — |
| `pr` | The bill's price per $100 of face value. Required. | — |

There is no `basis`, and there is no redemption argument: a bill redeems at 100. An amount passed as
`pr` returns a nonsense answer, not an error, because the function subtracts it from 100.

Dates are documented as truncated to integers, and 196 of the 2,400 rows behind the notes carry a
fractional date that fits this.

**Date arguments do not follow the ordinary to-number rules.** Logicals are refused with `#VALUE!`
in all three slots, including `FALSE`. An empty cell as `settlement` reads as serial 0 and gives a
normal answer, as the notes below record for this family: `TBILLYIELD(B1,5,90)` is 8. A direct `0`
gives the same answer. See [Coercion and lifting](../model/02-coercion-and-lifting.md) for the
shared model these depart from.

## Result and edge cases

Returns `Number`, an annualized yield as a decimal fraction.

Figures in this section and the next are from the 2,400 rows behind the notes below unless marked
as a reading. Readings are from Excel 16.0.20527.20088 on Windows, with bitness and CPU not recorded
in the reading file.

- **`pr > 100` is admissible and yields a negative rate.** There is no check against a price above
  redemption. All 27 such rows inside the date limits return a negative number.
- **A low price and a short span amplify.** The first note's example is a one-day bill at a price of
  about 2.28, with a yield of about 15,431. The formula is working, and the number means nothing
  commercially.
- **The one-year limit is a calendar rule.** The bound is the same month and day a year on, not a
  count of days. A 366-day bill across a 29 February is allowed. From 29 February, maturity on 1
  March a year later is allowed (3 rows), and 2 March is `#NUM!`. From 28 February of the year before
  a leap year, maturity on 29 February a year later is `#NUM!` (2 rows), although it is the last day
  of the month.
- **Serial 0 works as a settlement date.** Readings give numbers for maturities of serial 1, 5, 365
  and 366, and `#NUM!` for serial 367.
- **The order of arguments decides which error appears.** Readings show the first failing argument
  wins: `TBILLYIELD(#REF!,TRUE,90)` is `#REF!` and `TBILLYIELD(TRUE,#REF!,90)` is `#VALUE!`.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | `settlement` or `maturity` is not a valid date | Documented; see below |
| `#NUM!` | `pr ≤ 0` | Documented; 37 rows inside the limit |
| `#NUM!` | `settlement ≥ maturity` | Documented; readings at serial 1 and 0 |
| `#NUM!` | Maturity more than one year after settlement | Documented; 745 rows with no other fault |
| `#NUM!` | A negative serial | Differs from documentation; one reading |
| `#VALUE!` | A logical in any slot | Not documented; readings |

Microsoft says an invalid date is `#VALUE!`. The one reading with a negative settlement,
`TBILLYIELD(−1,TRUE,90)`, returns `#NUM!`, ahead of the logical in the next slot, so an out-of-range
number is not treated that way. The `#VALUE!` measured for a date argument is the logical one.

The reference engine at commit `473efa3` raises the documented conditions with the same codes, and
differs from the records above in at least two places. It returns `#VALUE!` for a date serial
below 1 or past
the end of year 9999. And its one-year limit holds to the end of the month, which allows 28 February
to 29 February a year later and refuses 29 February to 1 March a year later; the rows show Excel
doing the opposite in both cases.

## Relationships

- **[TBILLEQ](FUNC.TBILLEQ.md)** — the bond-equivalent yield of the same bill. Up to 182 days,
  `TBILLEQ = TBILLYIELD × 365/360` for a bill at the same price. Above 182 days `TBILLEQ` changes
  formula and this relation fails.
- **`TBILLPRICE`** — the price from the discount rate. It is not the inverse of this function, which
  takes a yield on the investment, so feeding a `TBILLYIELD` result to `TBILLPRICE` as `discount`
  does not recover `pr`.
- **[DISC](FUNC.DISC.md)** — the discount rate from the price, with a `basis`. It divides by the
  redemption where `TBILLYIELD` divides by the price.
- **[INTRATE](FUNC.INTRATE.md)** — the same investment-side rate. `INTRATE(s, m, pr, 100, 2)` is the
  same quantity on Actual/360, apart from the one-year limit; FN-DISC-001 gives both the same order
  of arithmetic.
- **[YEARFRAC](FUNC.YEARFRAC.md)** — not used here. The day count is a plain difference of serials,
  and there is no year-length question at all.

## Numerical notes

**The grouping matters.** The first note below gives the form Excel evaluates, with `360/DSM`
formed first. The alternative `(100 − pr)/pr × 360 / DSM`, evaluated left to right, is the same
number algebraically and a different one in binary64. The reference engine at commit `473efa3` uses
Excel's grouping in ordinary binary64 arithmetic. The first note records what its count shows about
that difference, and the record named below covers an earlier repair of the same grouping.

**Cancellation.** When `pr` is close to 100, the subtraction `100 − pr` cancels: the difference
keeps the absolute accuracy of `pr` and loses relative accuracy. `DSM` is small for short bills, so
`360/DSM` then scales up that error. This is intrinsic to a yield quoted from a price, and no
rearrangement recovers digits the input does not have.

**No calendar arithmetic.** There is no basis and no year length, so none of the family's 30/360
and Actual/actual questions arise. The only calendar logic is the one-year limit.

## What has not been checked

One evidence record names this surface in its subjects: **EV-FIN-0025**, a count for the reference
engine against Excel on the same sweep that its association repair was fitted to. Its counts and
reader warning render mechanically beside this page and are not restated here. It is a statement
about that engine and that sweep. The notes below are the later record of Excel's own arithmetic,
on a different build and a different set of rows.

No Handbook vector suite exists for `TBILLYIELD`; `vectors/` publishes nothing at this revision. The
following is still unmeasured.

1. **`settlement = maturity` at an ordinary date.** The only equal-dates reading is at serial 0.
2. **Serials past the end of year 9999.** `TBILLEQ` has readings there that return `#NUM!`, and this
   function has none.
3. **Text dates**, in every slot. The rows and readings hold numbers, logicals, errors and empty
   cells only. The `#VALUE!` Microsoft documents for an invalid date has not been seen on text.
4. **An argument left empty in the call**, which the note on empty cells leaves open.
5. **Arrays in any slot.**
6. **Dates before 1990, beyond a few readings.** The rows run from 1990 to 2052, and the readings
   stop at serial 367.
7. **Other Excel builds, platforms and bitness.** The rows are from one build on one 64-bit Windows
   machine.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `TBILLYIELD` that an implementer matching Excel, or a reader explaining
an unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's
answer and where it was recorded; "measured" means the statement is read off recorded Excel
answers, "inferred" marks what goes beyond them. Where older text on this page says a behaviour
was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

### FN-DISC-001: The discount-security functions run a fixed formula order on the x87 unit

*Detail for matching Excel's bits; measured; recorded 2026-11-04.* Applies to DISC, INTRATE, PRICEDISC, RECEIVED, TBILLYIELD.

With B the year basis and D the day count, Excel evaluates DISC = (1 - pr/red) * (B/D); INTRATE
= ((red - inv)/inv) * (B/D); RECEIVED = inv / (1 - (disc*D)/B); PRICEDISC = red -
((disc*red)*D)/B; TBILLYIELD = ((100 - pr)/pr) * (360/D), each + - * / taken from the x87 unit
at precision 64 and stored to binary64. Results can sit several units from the exact value: the
DISC row below is 5 units below it.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `DISC(37655, 38261, 94.87631295104353, 102.73579504052532, 3)` | settlement `37655` (`0x40e262e000000000`); maturity `38261` (`0x40e2aea000000000`); pr `94.87631295104353` (`0x4057b81582ea72c4`); redemption `102.73579504052532` (`0x4059af174414e760`); basis `3` (`0x4008000000000000`) (cells) | `0.046077868684136686` (`0x3fa79784b622b014`) | `0.04607786868413672` (`0x3fa79784b622b019`) | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |
| `TBILLYIELD(47701, 47702, 2.279784366464575)` | settlement `47701` (`0x40e74aa000000000`); maturity `47702` (`0x40e74ac000000000`); pr `2.279784366464575` (`0x40023cff95ff2def`) (cells) | `15430.96713248709` (`0x40ce237bcaff517a`) | `15430.967132487087` (`0x40ce237bcaff5178`) | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |
| `INTRATE(58548, 58599, 428532.13421818457, 6.805737049418593, 0)` | settlement `58548` (`0x40ec968000000000`); maturity `58599` (`0x40ec9ce000000000`); investment `428532.13421818457` (`0x411a27d089707de5`); redemption `6.805737049418593` (`0x401b39132211b6ad`); basis `0` (`0x0000000000000000`) (cells) | `-7.199885653133469` (`0xc01cccaed31c9766`) | — | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |

Correct value: Exact rational value of the same formula from the binary64 inputs, correctly
rounded.

Basis (measured): 2,400 of 2,400 rows for each function with stored precision-64 arithmetic.
Plain binary64 matches 2,398 to 2,399 of the same rows, so few rows separate the two; keeping
intermediates in 80-bit registers without stores loses 475 to 1,335.

Evidence:

- DnaCalc `crates/calc-func/tests/f3_financial.rs`, lines 435-466 at `26bb1800d3` — recorded answers pinned
- DnaCalc-Research `evidence/round23/security-1900-hard/summary.json`, counts at `f177408286` — stored-PC64 against other precisions
- DnaCalc-Research `round23-result.json`, recorded_completed_groups at `f177408286` — 18,040 of 18,040 outcomes
- OxFunc-Evidence `cache/oracle-20430/build-20430/DISC.jsonl.gz`, line 1458, key DISC|0x40e262e000000000|0x40e2aea000000000|0x4057b81582ea72c4|0x4059af174414e760|0x4008000000000000 at `e26b7bddb8` (readings)
- OxFunc-Evidence `cache/oracle-20430/build-20430/TBILLYIELD.jsonl.gz`, line 413, key TBILLYIELD|0x40e74aa000000000|0x40e74ac000000000|0x40023cff95ff2def at `e26b7bddb8` (readings)
- OxFunc-Evidence `cache/oracle-20430/build-20430/INTRATE.jsonl.gz`, line 2326, key INTRATE|0x40ec968000000000|0x40ec9ce000000000|0x411a27d089707de5|0x401b39132211b6ad|0x0000000000000000 at `e26b7bddb8` (readings)

### FN-WORKDAY-001: An empty cell as a first date is serial 0, but a logical in that cell is #VALUE!

*Quirk; measured; recorded 2026-11-04.* Applies to NETWORKDAYS, NETWORKDAYS.INTL, TBILLEQ, TBILLPRICE, TBILLYIELD, WORKDAY, WORKDAY.INTL, YEARFRAC.

A reference to an empty cell as the first date of these functions reads as serial 0 and gives a
normal answer: NETWORKDAYS(B1,5) is 4 and WORKDAY(B1,2) is 3. YEARFRAC(B1,5,0) is 5/360. TRUE
in the same cell gives #VALUE!, as a direct TRUE does, so a blank is not treated as the logical
FALSE either.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=NETWORKDAYS(B1,5)` | B1 `(empty cell)` (cells) | `4` (`0x4010000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=NETWORKDAYS(B1,5)` | B1 `TRUE` (cells) | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=WORKDAY(B1,2)` | B1 `(empty cell)` (cells) | `3` (`0x4008000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=WORKDAY(B1,2)` | B1 `TRUE` (cells) | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=NETWORKDAYS.INTL(B1,5,1)` | B1 `(empty cell)` (cells) | `4` (`0x4010000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=WORKDAY.INTL(B1,2,1)` | B1 `(empty cell)` (cells) | `3` (`0x4008000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=YEARFRAC(B1,5,0)` | B1 `(empty cell)` (cells) | `0.013888888888888888` (`0x3f8c71c71c71c71c`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=TBILLEQ(B1,5,0.1)` | B1 `(empty cell)` (cells) | `0.10152990264255911` (`0x3fb9fddd1b6a757f`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): 8 recorded readings with an empty B1, one per function, each paired with a
reading with TRUE in B1.

- Open: An argument left empty in the call was not read here for YEARFRAC and the T-bill functions.

Evidence:

- DnaCalc `apps/dna-onecalc/tests/acceptance.rs`, delivered_fn4_blank_date_references_read_serial_zero at `c4c5dde7e2` — recorded answers pinned
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheets qa031 and qa032, and the like pairs of each function at `2d0eddcac8` — recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa032, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa031, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa054, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa053, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa040, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa062, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa072, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa082, row 20, column 10 at `2d0eddcac8` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| money-market yield | A simple yield on the price paid, over a 360-day year |
| bond-equivalent yield | A bill's yield stated so it compares with a semiannual coupon bond; see `TBILLEQ` |
| DSM | The actual number of days from settlement to maturity |
| long bill | A bill with `DSM` above 182, the point where `TBILLEQ` changes formula |
| reading | One Excel answer to one chosen call, as distinct from a generated row |
| recorded rows | Excel's answers to generated inputs, which the notes below cite; a row fits a rule when it agrees to ten significant figures |

## Sources

- Microsoft, "TBILLYIELD function" —
  <https://support.microsoft.com/en-us/office/tbillyield-function-6d381232-f4b0-4cd5-8e97-45b9c03468ba>
  (syntax, argument meanings, the serial-date and truncation remarks, the `#VALUE!` and `#NUM!`
  conditions, the wording of `DSM`, and the worked example). The equation on that page is an image
  and was not readable, so no formula here is sourced from it.
- The function notes on this page, and the recorded Excel rows and readings their evidence lists
  cite. The fit at every span, the one-year-limit cases, the error rows, the negative yields and the
  serial and logical readings were read from those records for this page and go beyond the notes'
  own text; each carries the Excel build stated in Result and edge cases.
- Handbook evidence record `EV-FIN-0025` — the record naming this surface.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook, [Claim language and honesty](../model/06-claim-language.md) — the scoping rules this
  page's claims are written under.
- OxFunc `crates/oxfunc_core/src/functions/discount_bill_yearfrac_family.rs` at commit `473efa3`
  (the reference engine's `tbillyield_kernel`, `tbill_days` and its date guards).
- Handbook projections `data/functions/FUNC.TBILLYIELD.json` and
  `data/presence/FUNC.TBILLYIELD.json` (arity, classification axes, implementing module and sibling
  set).
