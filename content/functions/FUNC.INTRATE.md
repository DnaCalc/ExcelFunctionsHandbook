---
schema: efh.function-page/v1
function_id: FUNC.INTRATE
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
  The investment-side quotation: the simple annualized rate earned on the amount actually paid.
  DISC's mirror image, and the numerator/denominator swap that separates them is the whole
  distinction.
---

# INTRATE

## What it computes

`INTRATE(settlement, maturity, investment, redemption, [basis])` returns the **simple annualized
interest rate** for a fully invested security — one that pays nothing until maturity, when it pays
`redemption` against an outlay of `investment`.

With `T` the year fraction from settlement to maturity under the chosen day-count `basis`:

    INTRATE = (redemption − investment) / investment × 1/T

In Microsoft's notation, with `B` the basis year length in days and `DIM` the number of days from
settlement to maturity, `T = DIM/B` and

    INTRATE = (redemption − investment) / investment × B / DIM

**"Fully invested" is the definitional constraint**, and it means exactly one thing: there are no
intermediate cash flows. Every coupon-bearing instrument is outside this function's model, and
`INTRATE` will still return a number if you feed it one — a wrong number, computed correctly.

The rate is **simple, not compounded**. Over a span longer than a year, `INTRATE` returns the total
gain divided by the number of years, with no reinvestment of interest at all. It is not comparable
to an `EFFECT`-style annual equivalent and it is not `RRI`.

### The pair with DISC

`INTRATE` and [DISC](FUNC.DISC.md) describe the same trade and disagree about the denominator:

| | Numerator | Denominator |
|---|---|---|
| `INTRATE` | `redemption − investment` | `investment` — what you paid |
| `DISC` | `redemption − pr` | `redemption` — what you get back |

Since `redemption > investment` for any security bought at a discount, `INTRATE > DISC` always, and
the gap widens with `T`. The exact conversions:

    INTRATE = DISC / (1 − DISC·T)
    DISC    = INTRATE / (1 + INTRATE·T)

Confusing the two is the classic money-market quotation error. It is not a small error: on a
one-year instrument at a 10% discount rate the investment-side rate is about 11.1%.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `settlement` | The settlement date — when the security trades to the buyer. Required. | — |
| `maturity` | The maturity date — when the security expires. Required. | — |
| `investment` | The amount invested in the security. Required. | — |
| `redemption` | The amount to be received at maturity. Required. | — |
| `basis` | The day-count basis. Optional. | 0 |

Note the argument-shape asymmetry with `DISC`, and it is easy to miss: `DISC` takes a **price per
$100 face value**, while `INTRATE` takes an **amount invested**. Because both functions use their
two money arguments only as a ratio, either scaling gives the same answer *provided both money
arguments use the same one* — but the documented meanings differ, and a formula that feeds a
per-100 price to `INTRATE` alongside a raw redemption amount is silently wrong by four orders of
magnitude.

The documented day-count basis values are the standard five, identical to the table on the
[DISC](FUNC.DISC.md) page: `0` or omitted = US (NASD) 30/360, `1` = Actual/actual, `2` =
Actual/360, `3` = Actual/365, `4` = European 30/360. Dates and `basis` are documented as truncated
to integers.

Numeric coercion of the argument slots follows the shared call model; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number` — an annualized rate as a decimal fraction.

- **`redemption < investment`** is admissible and yields a negative rate. There is no guard against
  a loss-making trade.
- **`settlement ≥ maturity`** is a domain error rather than a sign flip.
- **Short spans amplify.** As `T → 0` the rate diverges; a one-day instrument at a hundredth of a
  percent gain annualizes to a large number, and that is the arithmetic working, not failing.
- **Non-integer `basis`** is truncated toward zero before the range check.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md).

## Errors

As documented on Microsoft's `INTRATE` page:

| Error | Condition |
|---|---|
| `#VALUE!` | `settlement` or `maturity` is not a valid serial date number |
| `#NUM!` | `investment ≤ 0` or `redemption ≤ 0` |
| `#NUM!` | `basis < 0` or `basis > 4` |
| `#NUM!` | `settlement ≥ maturity` |

The reference engine raises these four with the same codes, and additionally refuses a date serial
below 1 or beyond the end of year 9999 with `#VALUE!` — a boundary the documentation does not
mention. The Handbook has not observed either in Excel.

## Relationships

- **[DISC](FUNC.DISC.md)** — the redemption-side quotation of the same instrument, with the exact
  conversion above. Read the two pages together.
- **`RECEIVED`** — the inverse: given `INTRATE`'s rate and the investment, what comes back at
  maturity. `RECEIVED(…, INTRATE(…))` round-trips up to floating-point rounding.
- **`YIELDMAT`** — for a security that pays interest at maturity but *accrues* from an issue date,
  which `INTRATE` does not model. If your instrument has an issue date distinct from settlement,
  `INTRATE` is the wrong function.
- **`TBILLEQ`** — the bond-equivalent yield of a Treasury bill: the same idea of restating a
  discount quotation on the investment side, wired to the T-bill conventions.
- **`RRI`** — the *compound* rate that takes a present value to a future value over `n` periods.
  `INTRATE` is the simple-interest answer to a superficially similar question, and the two diverge
  as soon as the horizon exceeds a year.
- **`YEARFRAC`** — the day-count machinery, exposed directly; the fastest way to isolate whether a
  surprising `INTRATE` result comes from the calendar or from the amounts.

## Numerical notes

The arithmetic is one subtraction, two divisions and a multiplication. All of the interesting
behaviour is in `T` and in one cancellation site.

**Cancellation in `redemption − investment`.** For a short-dated instrument the two amounts agree to
many significant figures, so the difference loses precision exactly where `1/T` is largest. The
relative error of the result is the relative error of the difference multiplied by nothing at
all — the `1/T` factor scales both the value and its error — but the *absolute* error in the rate
grows with `1/T`, which is what a reader comparing two nearly identical short-dated quotes will
notice. There is no rearrangement that avoids this; the information is genuinely in the last few
digits of the two inputs.

**Day counts are the real algorithm.** As on the `DISC` page: the 30/360 conventions are exact
integer counting rules, not approximations, and the US and European variants differ in their 31st
and end-of-February adjustments. The Actual/actual basis admits more than one defensible year
length for a span crossing a year boundary, and at commit `473efa3` the reference engine's
discount/bill module and its bond module do not use the same rule. That intra-engine divergence is
recorded here as a finding, not resolved.

**Form.** The reference engine computes `((redemption − investment)/investment)/T`, forming the
year fraction first and dividing once; the documented expression forms `B/DIM` and multiplies. The
two are algebraically identical and differ in binary64 by the placement of one rounding.

## What has not been checked

No Handbook vector suite exists for `INTRATE`; `vectors/` publishes nothing at this revision. No
Excel-comparison evidence record names `INTRATE` in its subjects. **Nobody has checked this
function against Excel within the Handbook's record.** The error table is Microsoft's documented
set; the equation is Microsoft's documented equation; the statements about the reference engine are
read from its source at commit `473efa3`.

Inputs worth probing first:

1. **`INTRATE` against `DISC` on identical dates and amounts**, checking the exact conversion
   `INTRATE = DISC/(1 − DISC·T)` numerically. This single probe validates the definitional split
   that the whole page turns on, and any failure localizes immediately to one of the two.
2. **`basis = 1` across a year boundary**, compared with `YEARFRAC` on the same dates and with a
   bond-family function over the same span — the intra-engine Actual/actual divergence described
   above.
3. **`settlement = maturity − 1`** at each of the five bases, the shortest admissible span, where
   both the cancellation and the `1/T` amplification are extreme.
4. **`redemption < investment`**, to confirm a negative rate rather than `#NUM!`.
5. **A per-100 `investment` with a raw-amount `redemption`**, to confirm that the function has no
   scale check at all — a negative result for the Handbook, but a useful one for readers.
6. **Date serial `0`** and **fractional serials**, testing the undocumented lower bound and the
   documented truncation.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `INTRATE` that an implementer matching Excel, or a reader explaining an
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
| fully invested | No cash flows between settlement and maturity; the model `INTRATE` assumes |
| simple rate | Total gain divided by the year fraction, with no compounding or reinvestment |
| investment side | The gain expressed as a fraction of the amount paid, as opposed to the redemption |
| day-count basis | The `basis` argument's calendar convention, one of the five documented methods |
| year fraction `T` | The settlement-to-maturity span in years under the chosen basis |

## Sources

- Microsoft, "INTRATE function" —
  <https://support.microsoft.com/en-us/office/intrate-function-5cb34dde-a221-4cb6-b3eb-0b9e55e1316f>
  (syntax, argument meanings, the day-count basis table, the serial-date and truncation remarks,
  and the documented `#VALUE!` and `#NUM!` conditions).
- Microsoft, "DISC function" —
  <https://support.microsoft.com/en-us/office/disc-function-71fce9f3-3f05-4acf-a5a3-eac6ef4daa53>
  (the basis table verbatim, and the companion quotation convention).
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- OxFunc `crates/oxfunc_core/src/functions/discount_bill_yearfrac_family.rs` at commit `473efa3`
  (`intrate_kernel`, the shared year-fraction routine, the date and basis guards) and
  `crates/oxfunc_core/src/functions/bond_core_family.rs` at the same commit (the bond family's
  differing Actual/actual year length).
- Handbook projections `data/functions/FUNC.INTRATE.json` and `data/presence/FUNC.INTRATE.json`.
