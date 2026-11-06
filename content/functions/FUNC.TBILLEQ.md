---
schema: efh.function-page/v1
function_id: FUNC.TBILLEQ
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-FIN-0034
  - EV-STRUCT-0007
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
  The Treasury bill's bond-equivalent yield, restating a discount quotation on the footing of a
  semiannual bond. The family's one two-branch function, and the one whose published formula
  stops being true at 182 days.
---

# TBILLEQ

## What it computes

`TBILLEQ(settlement, maturity, discount)` returns the **bond-equivalent yield** of a Treasury bill:
the yield that lets a bill quoted at a bank-discount rate be compared with a coupon bond.

A bill's price per 100 follows from its discount rate `d` and the actual day count `DSM`:

    price = 100 × (1 − d × DSM/360)

Write `p = price/100`. What happens next depends on how long the bill runs, and **Microsoft's page
describes only one of the two cases.**

| | Microsoft's page | What Excel does |
|---|---|---|
| `DSM` up to 182 | `(365 × rate) / (360 − rate × DSM)` | The same formula |
| `DSM` above 182 | The same formula | The quadratic below |
| `DSM` as a count | "computed according to the 360 days per year basis" | Actual days |

For the bill of 182 days or fewer, the two forms below are the same number:

    TBILLEQ = (365 × d) / (360 − d × DSM)  =  (1/p − 1) × 365/DSM

That is a simple yield on the price, over a 365-day year.

For a bill of more than 182 days, Excel returns

    TBILLEQ = (−t + √(t² − (2t − 1)(1 − 1/p))) / (t − 1/2)

with `t = DSM/365`, as recorded in the notes below. Microsoft's page does not mention 182 days or
this expression. Both accounts are shown here because they differ, and on the note's own row the
difference is large: 203 days at a discount of about 21.2%, where the documented formula gives
0.244038 and Excel gives 0.241103.

### Where the quadratic comes from

A coupon bond pays every half year, so a bill longer than about half a year has to be compared with
a bond that has already paid once. The standard construction lets the first half year earn `y/2`,
with the interest reinvested, and the rest of the span earn simple interest at `y`:

    p × (1 + y/2) × (1 + y × (t − 1/2)) = 1

Expanding gives a quadratic in `y`, and its positive root is the expression above. That is algebra
from the construction; Microsoft does not state it.

The table shows how the two compare at a 5% discount. These are formula values, not Excel readings.

| `DSM` | Documented formula | Quadratic | Excel's branch |
|---|---|---|---|
| 91 | 0.051343 | not applied | documented |
| 182 | 0.052009 | 0.052013 | documented |
| 183 | 0.052017 | 0.052013 | quadratic |
| 270 | 0.052670 | 0.052228 | quadratic |
| 364 | 0.053394 | 0.052701 | quadratic |

### The 366-day bill

The note writes `t = DSM/365`, and that holds with one exception. All 33 numeric rows with
`DSM = 366` (for example 13 July 2047 to 13 July 2048, a span that crosses a 29 February) fit
`t = 1`, that is `366/366`, and do not fit `366/365`. The 95 rows of shorter long bills across a 29
February keep 365. The one reading at this length, from serial 0 to serial 366, agrees.

### The documented DSM wording

Microsoft says `DSM` is computed on a 360-day-year basis. Its own example does not bear that out:
31 March 2008 to 1 June 2008 at 9.14% gives `0.0941515` with 62 actual days, matching the 9.42%
Microsoft shows, whereas a 30/360 count of 61 gives `0.0941272`, or 9.41%. The note's row is 203
actual days where 30/360 would give 200. The 360 in the formula is the year length of the discount
quotation; the day count is actual.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `settlement` | The bill's settlement date, the date after issue when it trades to the buyer. Required. | — |
| `maturity` | The date the bill expires. Required. | — |
| `discount` | The bill's discount rate, as a decimal fraction. Required. | — |

There is no `basis`. The day count is always actual, and the conventions in the formulas are fixed.
Dates are documented as truncated to integers, and 205 of the 2,400 rows behind the notes carry a
fractional date that fits this.

`discount` is the bank-discount quotation, not a price and not a yield. The function that takes a
price is [TBILLYIELD](FUNC.TBILLYIELD.md).

**Date arguments do not follow the ordinary to-number rules.** Logicals are refused with `#VALUE!`
in all three slots, including `FALSE`. An empty cell as `settlement` reads as serial 0 and gives a
normal answer, as the notes below record: `TBILLEQ(B1,5,0.1)` is `36.5/359.5`. A direct `0` gives
the same answer. See [Coercion and lifting](../model/02-coercion-and-lifting.md) for the shared
model these depart from.

## Result and edge cases

Returns `Number`, an annualized yield as a decimal fraction.

Figures in this section and the next are from the 2,400 rows behind the notes below (Excel 16.0
build 20430, Windows 64-bit, AMD Ryzen 7 4800H) unless marked as a reading. Readings are from Excel
16.0.20527.20088 on Windows, with bitness and CPU not recorded in the reading file.

- **The branch point is exact in the rows.** The 992 numeric rows with `DSM ≤ 182` fit the documented
  formula, including 48 at 182. The 712 numeric rows above it fit the quadratic, including 91 at 183.
  No row above 182 fits the documented formula.
- **A non-positive price is refused.** Where `d × DSM/360 ≥ 1` the bill would have no positive price,
  and all 25 such rows inside the one-year limit return `#NUM!`.
- **The one-year limit is a calendar rule.** The bound is the same month and day a year on, not a
  count of days. A 366-day bill across a 29 February is allowed. From 28 February of the year before
  a leap year, maturity on 29 February a year later is `#NUM!` (2 rows), although that is the last
  day of the month. From 29 February, the 28 February a year later is allowed, and 2 March is `#NUM!`.
- **Serial 0 works as a settlement date.** Readings give numbers for maturities of serial 1, 5, 365
  and 366, and `#NUM!` for serial 367.
- **The order of arguments decides which error appears.** Readings show the first failing argument
  wins: `TBILLEQ(#REF!,TRUE,0.1)` is `#REF!` and `TBILLEQ(TRUE,#REF!,0.1)` is `#VALUE!`.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | `settlement` or `maturity` is not a valid date | Documented; see below |
| `#NUM!` | `discount ≤ 0` | Documented; 60 rows inside the limit |
| `#NUM!` | Maturity more than one year after settlement | Documented; 541 rows with no other fault |
| `#NUM!` | `settlement > maturity` | Documented; reading at serial 1 and 0 |
| `#NUM!` | `settlement = maturity` | Not documented; one reading |
| `#NUM!` | `d × DSM/360 ≥ 1` | Not documented; 25 rows |
| `#NUM!` | A negative or past-year-9999 serial | Differs from documentation; readings |
| `#VALUE!` | A logical in any slot | Not documented; readings |

Three of these depart from Microsoft's page, and both accounts are shown:

- Microsoft's `TBILLEQ` page says `settlement > maturity`, where its `TBILLYIELD` page says
  `settlement ≥ maturity`. The formula has a value at `DSM = 0`, so the difference is not vacuous. A
  reading of `TBILLEQ(0,0,0.1)` returns `#NUM!`. That is the only equal-dates reading.
- Microsoft says an invalid date is `#VALUE!`. Readings of settlement `−1`, `−0.5` and 2958466 (the
  first serial past 31 December 9999) all return `#NUM!`, so an out-of-range number is not treated
  that way. The `#VALUE!` measured for a date argument is the logical one.
- Microsoft does not mention a price that reaches zero.

The reference engine at commit `473efa3` differs from these in at least three ways. It applies the
documented
formula at every span, with no branch at 182 days, so it does not agree with Excel on long bills. It
returns `#VALUE!` for a date serial below 1 or past the end of year 9999. And its one-year limit
holds to the end of the month, which allows 28 February to 29 February a year later and refuses 29
February to 1 March a year later. It raises the documented conditions with the same codes, and it
also refuses equal dates and a price at or below zero with `#NUM!`.

## Relationships

- **[TBILLYIELD](FUNC.TBILLYIELD.md)** — the yield of the same bill from its price, on a 360-day
  year and with no long-bill branch. Up to 182 days, on the same bill,
  `TBILLEQ = TBILLYIELD × 365/360`. Above that the relation fails.
- **`TBILLPRICE`** — the price from the discount, which is the first step of this function.
- **[DISC](FUNC.DISC.md)** — the general discount quotation with a `basis`. The T-bill functions fix
  the convention and add the one-year limit.
- **[INTRATE](FUNC.INTRATE.md)** — the simple rate on the investment. For bills up to 182 days,
  `TBILLEQ` is `INTRATE` on an actual/365 footing.
- **`YIELD`** — the coupon-bond yield that a bond-equivalent yield is built to be compared with.

## Numerical notes

**The first branch is plain.** One multiplication, one subtraction and one division. It has a pole
at `d × DSM = 360`, which is where the price reaches zero, and Excel refuses from there on.

**The second branch has a cancellation site.** The numerator is `−t` plus a square root that is close
to `t`, and it is small when the yield is small or `t` is close to 1/2. The denominator `t − 1/2` is
smallest at `DSM = 183`, where it is `1/730`. The expression cannot be evaluated at `t = 1/2`, and
the integer day count keeps it away, because `DSM = 182.5` does not exist.

**The exact arithmetic.** The note records how the square root is staged, and why Excel's answer on
its row sits 22 units below the correctly rounded value of the quadratic. Reproducing Excel means
following that staging, not computing the quadratic more accurately. The note's count applies to
the same 2,400 rows, and one row separates the staged result from the plain one, so this is a small
effect measured on few rows.

**Discriminant.** The value under the square root was positive on all 712 long-bill rows that
reached it, so no row shows what a negative value would do. For `0 < p < 1` and `t > 1/2` the term
`−(2t − 1)(1 − 1/p)` is positive, so it does not arise on admissible inputs.

## What has not been checked

Two evidence records name this surface in its subjects. **EV-FIN-0034** records that the reference
engine's sources held no numeric comparison count for this function at commit `473efa3`, and
**EV-STRUCT-0007** carries it in an array-lift group on argument shape only, with nothing about
numeric results. Their fields and warnings render mechanically beside this page. Neither is a
statement about Excel's numbers, and the notes below are the later record of those.

No Handbook vector suite exists for `TBILLEQ`; `vectors/` publishes nothing at this revision. The
following is still unmeasured.

1. **29 February settlement with maturity on 1 March a year later.** The `TBILLYIELD` rows allow it,
   but there is no such `TBILLEQ` row.
2. **`settlement = maturity` at an ordinary date.** The only reading is at serial 0.
3. **Text dates**, in every slot. The rows and readings hold numbers, logicals, errors and empty
   cells only. The `#VALUE!` Microsoft documents for an invalid date has not been seen on text.
4. **An argument left empty in the call**, which the note on empty cells leaves open.
5. **Arrays in any slot**, beyond the argument-shape group in the record above.
6. **Dates before 1990, beyond a few readings.** The rows run from 1990 to 2051, and the readings
   stop at serial 367. A span crossing the fictitious 29 February 1900 at serial 60 is reached only
   by readings that start at serial 0.
7. **Other Excel builds, platforms and bitness.** The branch is recorded on one build. Nothing here
   says when it was introduced or whether every platform has it.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `TBILLEQ` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-TBILLEQ-001: For bills longer than 182 days TBILLEQ is a quadratic, not the documented formula

*Documentation differs from behaviour; measured; recorded 2026-11-04.*

When DSM is above 182, TBILLEQ does not use the documented formula. It uses the bond-equivalent
quadratic (-t + sqrt(t^2 - (2t-1)(1 - 1/p))) / (t - 1/2), with t = DSM/365 and p = price/100.
At DSM 366, a bill across a 29 February, t is 1 and not 366/365. The square root is taken in
extended precision and stored, RN53(RN64(sqrt)); the rest is binary64. For the row below, DSM
203, the documented formula gives 0.244038 and Excel 0.241103. Excel's bits are 22 units below
the correctly rounded quadratic because of that staging.

Documentation: TBILLEQ is calculated as TBILLEQ = (365 x rate)/(360-(rate x DSM)); the page
does not mention 182 days.
(https://support.microsoft.com/en-us/office/tbilleq-function-2ab72d90-9b4d-4efe-9fc2-0f81f2c19c8c)

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `TBILLEQ(40633.25, 40836.25, 0.21193100748125787)` | settlement `40633.25` (`0x40e3d72800000000`); maturity `40836.25` (`0x40e3f08800000000`); discount `0.21193100748125787` (`0x3fcb208e2511f676`) (cells) | `0.24110326418450792` (`0x3fcedc78c550cec1`) | `0.24110326418450853` (`0x3fcedc78c550ced7`) | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |
| `TBILLEQ(53886, 54252, 0.4522540814812992)` | settlement `53886` (`0x40ea4fc000000000`); maturity `54252` (`0x40ea7d8000000000`); discount `0.4522540814812992` (`0x3fdcf1bb1a5c75f4`) (cells) | `0.7211303677406913` (`0x3fe7137ffe3328bc`) | — | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |

Correct value: mpmath at 800 bits: the exact quadratic at the binary64 inputs, correctly
rounded.

Basis (measured): 2,400 of 2,400 recorded rows with the stored square root; 2,399 without it.
All 33 recorded rows with a number answer at DSM 366 fit t = 1, and none of the 17 of them in
the run below fits 366/365.

Evidence:

- DnaCalc `crates/calc-func/tests/f3_financial.rs`, lines 435-466 at `26bb1800d3` — recorded answers pinned
- DnaCalc-Research `evidence/round23/tbilleq-hard/summary.json`, counts at `f177408286` — staged square root against alternatives
- DnaCalc `crates/calc-func/src/ported/discount_bill_yearfrac_family.rs`, lines 388-410 at `26bb1800d3` — the arithmetic in the product
- CalcFunc-Evidence `runs/w111-5-20260924/answers/answers-w1115-tbilleq.json.gz`, w1115-TBILLEQ-01109 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `runs/w111-5-20260924/answers/answers-w1115-tbilleq.json.gz`, w1115-TBILLEQ-00179 at `4d5320dfdc` (readings)

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
| bond-equivalent yield | A bill's yield stated so it compares with a semiannual coupon bond |
| bank discount | A rate charged on the redemption value over a 360-day year |
| DSM | The actual number of days from settlement to maturity |
| `p`, `t` | The price as a fraction of face value, and `DSM` in years |
| long bill | On this page, a bill with `DSM` above 182 |
| reading | One Excel answer to one chosen call, as distinct from a generated row |
| recorded rows | Excel's answers to generated inputs, which the notes below cite; a row fits a rule when it agrees to ten significant figures |

## Sources

- Microsoft, "TBILLEQ function" —
  <https://support.microsoft.com/en-us/office/tbilleq-function-2ab72d90-9b4d-4efe-9fc2-0f81f2c19c8c>
  (syntax, argument meanings, the serial-date and truncation remarks, the `#VALUE!` and `#NUM!`
  conditions, the formula and its `DSM` wording, and the worked example). The page does not mention
  182 days.
- Microsoft, "TBILLYIELD function" —
  <https://support.microsoft.com/en-us/office/tbillyield-function-6d381232-f4b0-4cd5-8e97-45b9c03468ba>
  (the `settlement ≥ maturity` wording compared above).
- The function notes on this page, and the recorded Excel rows and readings their evidence lists
  cite. The branch counts, the `DSM = 366` case, the one-year-limit cases, the error rows and the
  serial and logical readings were read from those records for this page and go beyond the notes'
  own text; each carries the Excel build stated in Result and edge cases.
- Handbook evidence records `EV-FIN-0034` and `EV-STRUCT-0007`.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook, [Claim language and honesty](../model/06-claim-language.md) — the scoping rules this
  page's claims are written under.
- OxFunc `crates/oxfunc_core/src/functions/discount_bill_yearfrac_family.rs` at commit `473efa3`
  (the reference engine's `tbilleq_kernel`, `tbill_days` and its date guards).
- Handbook projections `data/functions/FUNC.TBILLEQ.json` and `data/presence/FUNC.TBILLEQ.json`
  (arity, classification axes, implementing module and sibling set).
