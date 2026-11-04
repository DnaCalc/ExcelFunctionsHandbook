---
schema: efh.function-page/v1
function_id: FUNC.PRICEDISC
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
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
  DISC run forwards: given the bank-discount rate, the price that rate implies. The member that
  shows most plainly that simple discount has no floor, because the price it returns can go
  below zero.
---

# PRICEDISC

## What it computes

`PRICEDISC(settlement, maturity, discount, redemption, [basis])` returns the **price per 100 of
face value** of a security sold at a discount: one that pays nothing until maturity, when it pays
`redemption`.

The model is *simple bank discount*. With `T` the year fraction from settlement to maturity under
the chosen day-count `basis`, and `d` the discount rate:

    price = redemption × (1 − d × T)

Written with `B` the number of days in the year implied by the basis and `DSM` the number of days
from settlement to maturity, so that `T = DSM/B`:

    PRICEDISC = redemption − discount × redemption × DSM / B

Microsoft's page prints its equation as an image that could not be read, and defines `B` and `DSM`
in words. The expression above is the one the notes below record from Excel's own answers.

The discount is charged on the *redemption* value, not on the amount paid, and it is charged once.
Nothing compounds. That makes the price a straight line in time, falling from `redemption` at
`T = 0` and reaching zero at `T = 1/d`. A 10% discount rate reaches zero at ten years.

Microsoft's example: settlement 16 February 2008, maturity 1 March 2008, discount 5.25%, redemption
100, basis 2 (Actual/360). That is 14 days, and 2008 is a leap year, so

    100 × (1 − 0.0525 × 14/360) = 99.795833…

which Microsoft shows as $99.80.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `settlement` | The date the buyer purchases the security, after its issue date. Required. | — |
| `maturity` | The date the security expires. Required. | — |
| `discount` | The security's discount rate, as a decimal fraction. Required. | — |
| `redemption` | The redemption value per $100 of face value. Required. | — |
| `basis` | The day-count basis. Optional. | 0 |

The documented basis values are the standard five, as on the [DISC](FUNC.DISC.md) page: `0` or
omitted = US (NASD) 30/360, `1` = Actual/actual, `2` = Actual/360, `3` = Actual/365, `4` = European
30/360. Dates and `basis` are documented as truncated to integers, and Microsoft advises entering
dates with `DATE` rather than as text.

`redemption` is documented as a per-100 quantity. The result is proportional to it, so passing an
amount returns the price of that amount; the function has no way to tell the two apart.

Numeric coercion of the argument slots follows the shared call model; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`, a price per 100 of face value, or an amount in the units of `redemption` when
`redemption` is an amount.

- **A negative price is an answer, not an error.** There is no check on the result. In the 2,400
  rows behind the notes below (Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H), 284
  return a negative price and none returns an error. The documented conditions are on the inputs;
  a long span at a high discount passes all of them. The sibling [RECEIVED](FUNC.RECEIVED.md) does
  refuse the equivalent case, and the two functions differ here.
- **`settlement ≥ maturity`** is documented as `#NUM!`, not a reversed reading.
- **The fractional part of a date is dropped.** 195 of those rows carry a fractional date, and each
  fits the integer day count.
- **The 1900 calendar.** The rows include a settlement of serial 60, the fictitious 29 February
  1900, and on basis 4 it counts as day 29 of February, not as 1 March. The basis 1 treatment of
  the same date is the subject of the second note below.
- Empty, missing and error arguments are expected to follow the shared call model; see
  [The value universe](../model/01-value-universe.md). No reading records this for `PRICEDISC`.

## Errors

As documented on Microsoft's `PRICEDISC` page:

| Error | Condition |
|---|---|
| `#VALUE!` | `settlement` or `maturity` is not a valid date |
| `#NUM!` | `discount ≤ 0` or `redemption ≤ 0` |
| `#NUM!` | `basis < 0` or `basis > 4` |
| `#NUM!` | `settlement ≥ maturity` |

None of these rows is measured. All 2,400 recorded rows return a number, so the table is what
Microsoft states and nothing more.

The reference engine at commit `473efa3` raises the same four conditions with the same codes. It
also refuses a date serial below 1 or past the end of year 9999 with `#VALUE!`. That lower limit
is not Excel's rule elsewhere in this family: the notes on [YEARFRAC](FUNC.YEARFRAC.md) record
serial 0 as an accepted date, and [TBILLEQ](FUNC.TBILLEQ.md) and
[TBILLYIELD](FUNC.TBILLYIELD.md) accept it too. Whether `PRICEDISC` does has not been read.

## Relationships

- **[DISC](FUNC.DISC.md)** — the inverse: given the price, recover the rate.
  `DISC(s, m, PRICEDISC(s, m, d, r, b), r, b)` returns `d` up to floating-point rounding, for any
  positive price. A negative price from `PRICEDISC` has no inverse, because `DISC` documents
  `pr ≤ 0` as `#NUM!`.
- **[RECEIVED](FUNC.RECEIVED.md)** — the same relation solved for the other end of the trade: price
  known, redemption wanted. `PRICEDISC` multiplies by `(1 − d × T)` and `RECEIVED` divides by it.
- **[INTRATE](FUNC.INTRATE.md)** — the same trade quoted on the investment. If the rate you hold is
  an `INTRATE`-style rate, it is not a valid `discount` here. Convert it first with
  `d = r / (1 + r × T)`.
- **`TBILLPRICE`** — this function fixed to redemption 100 and Actual/360, with a one-year limit.
- **[PRICEMAT](FUNC.PRICEMAT.md)** — the price of a security that does pay interest, all at
  maturity. `PRICEDISC` models no interest at all.
- **[YEARFRAC](FUNC.YEARFRAC.md)** — the year fraction on its own, and the quickest way to separate a
  calendar question from a price question.

## Numerical notes

The arithmetic is one subtraction, two multiplications and one division. Two things matter: how `T`
is obtained, and the order of the operations.

**The order of operations.** The first note below gives the order Excel uses, and it is not the
order in which the formula is usually written. The reference engine at commit `473efa3` forms the
year fraction first and computes `redemption × (1 − discount × T)`. That is the same number
algebraically and a different number in binary64, so results from the two forms can differ in the
last places.

**Near-zero prices.** Where `d × T` is close to 1, the subtraction cancels: the result is small
and its relative error is large, though its absolute error stays of the order of rounding at the
size of `redemption`.

**The year length at basis 1.** Microsoft says only that `B` is the number of days in the year.
Reading `B` back from the 815 basis 1 rows gives one rule, and all 815 fit it to at least ten
significant figures:

- for a span of at most one year, if both dates are in one calendar year, `B` is that year's
  length; otherwise `B` is 366 if a 29 February lies in the span and 365 if not;
- for a span of more than one year, `B` is the average length of the calendar years from the
  settlement year to the maturity year, both included.

So 31 May 2012 to 19 February 2013 is 264 days with `B = 365`, although 2012 is a leap year,
because 29 February 2012 lies before the span. And 30 April 1923 to 12 June 1924 is 409 days with
`B = 365.5`, the average of 365 and 366.

The reference engine at commit `473efa3` does something else here: it accumulates a fraction of
each calendar year against that year's own length. On the first span it gets `T = 0.72168` against
`264/365 = 0.72329`, a difference in the third significant figure that no arrangement of the
arithmetic would explain. That is a defect of the engine at that commit against these rows, and it
answers, for this function on this build, the question the [DISC](FUNC.DISC.md) page raises about
Actual/actual.

**The 30/360 bases.** All rows at bases 0 and 4 fit the standard US and European 30/360 counts,
with the one 1900 date noted above. `PRICEMAT` is the family member that departs from the standard
rule at basis 0; see [PRICEMAT](FUNC.PRICEMAT.md).

## What has not been checked

No Handbook vector suite exists for `PRICEDISC`; `vectors/` publishes nothing at this revision. No
evidence record names this function in its subjects. What is measured is what the notes below
record, plus the row readings stated on this page, all on one Excel build unless a note says
otherwise. The following is still unmeasured.

1. **Every documented error.** No recorded row or reading returns one. The first probes are
   `settlement = maturity`, `discount = 0`, `redemption = 0` and `basis = 5`.
2. **Date serials outside the ordinary range.** Serial 0, a negative serial and a serial past the
   end of year 9999. The lowest settlement in the rows is serial 60, and the reference engine's
   `#VALUE!` below serial 1 is not a statement about Excel.
3. **Fractional `basis`.** Every row has an integer basis, so the documented truncation is measured
   for dates and not for `basis`.
4. **Non-numeric arguments.** Logicals, text dates and empty cells have no reading for `PRICEDISC`.
   The T-bill functions refuse a logical date with `#VALUE!` and read an empty cell as serial 0, but
   that is recorded for them and not for this function.
5. **An argument left empty in the call**, as in `PRICEDISC(s, m, d, r, )`, and an array in any
   slot.
6. **Other Excel builds, platforms and bitness.** The rows are from one build on one 64-bit Windows
   machine.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `PRICEDISC` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

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

### FN-YEARFRAC-001: On basis 1 the fictitious 1900-02-29 is not a leap day, and serial 0 is a valid date

*Quirk; measured; recorded 2026-11-04.* Applies to ACCRINTM, DISC, INTRATE, PRICEDISC, RECEIVED, YEARFRAC.

Excel's calendar holds a 29 February 1900, serial 60, that never existed. On basis 1
(actual/actual) it does not make 1900 a leap year: YEARFRAC(59,60,1) is 1/365, where the real
1904-02-29 gives YEARFRAC(1520,1521,1) = 1/366. Serial 0 is accepted as a date: YEARFRAC(0,5,0)
is 5/360 and YEARFRAC(0,60,1) is 60/365. Reversed dates give the same answer as in order.
ACCRINTM, DISC, INTRATE, PRICEDISC and RECEIVED take the same 365 and 366 on those dates:
DISC(59,60,99,100,1) is about 3.65 and DISC(1520,1521,99,100,1) about 3.66.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `YEARFRAC(59, 60, 1)` | start_date `59` (`0x404d800000000000`); end_date `60` (`0x404e000000000000`); basis `1` (`0x3ff0000000000000`) (cells) | `0.0027397260273972603` (`0x3f66719f3601671a`) | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `YEARFRAC(1520, 1521, 1)` | start_date `1520` (`0x4097c00000000000`); end_date `1521` (`0x4097c40000000000`); basis `1` (`0x3ff0000000000000`) (cells) | `0.00273224043715847` (`0x3f6661ec6a5122f9`) | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `YEARFRAC(0, 5, 0)` | start_date `0` (`0x0000000000000000`); end_date `5` (`0x4014000000000000`); basis `0` (`0x0000000000000000`) (cells) | `0.013888888888888888` (`0x3f8c71c71c71c71c`) | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `YEARFRAC(0, 60, 1)` | start_date `0` (`0x0000000000000000`); end_date `60` (`0x404e000000000000`); basis `1` (`0x3ff0000000000000`) (cells) | `0.1643835616438356` (`0x3fc50a8542a150a8`) | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `DISC(59, 60, 99, 100, 1)` | settlement `59` (`0x404d800000000000`); maturity `60` (`0x404e000000000000`); pr `99` (`0x4058c00000000000`); redemption `100` (`0x4059000000000000`); basis `1` (`0x3ff0000000000000`) (cells) | `3.650000000000003` (`0x400d33333333333a`) | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `DISC(1520, 1521, 99, 100, 1)` | settlement `1520` (`0x4097c00000000000`); maturity `1521` (`0x4097c40000000000`); pr `99` (`0x4058c00000000000`); redemption `100` (`0x4059000000000000`); basis `1` (`0x3ff0000000000000`) (cells) | `3.6600000000000033` (`0x400d47ae147ae14f`) | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |

Basis (measured): 19 YEARFRAC readings around serial 60 and the 1904, 2000 and 2004 leap days,
35 readings with serial 0 on all five bases, and 3 readings each from five financial functions.

- Open: An argument left empty in the call, as in YEARFRAC(,5), has not been read.

Evidence:

- DnaCalc `crates/calc-func/src/ported/day_count_common.rs` at `c4c5dde7e2` — the leap-day test as implemented
- DnaCalc `apps/dna-onecalc/tests/acceptance.rs`, delivered_scalar_readings_follow_the_application_route at `2d0eddcac8` — recorded answers pinned
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/YEARFRAC.reading.json` at `2d0eddcac8` — 87 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/YEARFRAC.reading.json`, fn7-YEARFRAC-fictitious-leap-day-3 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/YEARFRAC.reading.json`, fn7-YEARFRAC-fictitious-leap-day-9 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/YEARFRAC.reading.json`, fn7-YEARFRAC-calendar-zero-11 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/YEARFRAC.reading.json`, fn7-YEARFRAC-fictitious-leap-day-1 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/DISC.reading.json`, fn7-DISC-fictitious-leap-day-1 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/DISC.reading.json`, fn7-DISC-fictitious-leap-day-2 at `2d0eddcac8` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| bank discount | A rate charged on the redemption value, not on the price paid |
| simple discount | One charge over the whole span, with no compounding |
| day-count basis | The `basis` argument's calendar convention, one of the five documented methods |
| year fraction `T` | The settlement-to-maturity span in years under the chosen basis |
| per-100 quotation | A price or redemption value expressed per 100 units of face value |
| DSM / B | Microsoft's names for the settlement-to-maturity day count and the basis year length |
| recorded rows | Excel's answers to generated inputs, which the notes below cite; a row fits a rule when it agrees to the stated precision |

## Sources

- Microsoft, "PRICEDISC function" —
  <https://support.microsoft.com/en-us/office/pricedisc-function-d06ad7c1-380e-4be7-9fd9-75e3079acfd3>
  (syntax, argument meanings, the day-count basis table, the serial-date and truncation remarks,
  the `#VALUE!` and `#NUM!` conditions and the worked example). The equation on that page is an
  image and was not readable, so no formula here is sourced from it.
- The function notes on this page, and the recorded Excel rows and readings their evidence lists
  cite. The count of negative prices, the fractional-date count, the 1900 date at basis 4 and the
  basis 1 year-length rule were read from those 2,400 rows for this page and go beyond the notes'
  own text; they carry the same Excel build, platform and CPU.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md) — argument kinds, numeric coercion
  and error propagation.
- Handbook, [Claim language and honesty](../model/06-claim-language.md) — the scoping rules this
  page's claims are written under.
- OxFunc `crates/oxfunc_core/src/functions/discount_bill_yearfrac_family.rs` at commit `473efa3`
  (the reference engine's `pricedisc_kernel`, its year-fraction routine, and its date and basis
  guards).
- Handbook projections `data/functions/FUNC.PRICEDISC.json` and
  `data/presence/FUNC.PRICEDISC.json` (arity, classification axes, implementing module and sibling
  set).
