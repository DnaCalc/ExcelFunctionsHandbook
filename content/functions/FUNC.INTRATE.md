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
