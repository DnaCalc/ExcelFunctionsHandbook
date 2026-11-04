---
schema: efh.function-page/v1
function_id: FUNC.RECEIVED
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
  The redemption side of the discount relation: given what was paid and the bank-discount rate,
  what comes back. It divides where PRICEDISC multiplies, so it has a pole, and Excel refuses
  everything past it without the documentation saying so.
---

# RECEIVED

## What it computes

`RECEIVED(settlement, maturity, investment, discount, [basis])` returns the **amount received at
maturity** for a fully invested security: one bought for `investment`, paying nothing in between,
and quoted at the bank-discount rate `discount`.

Let `T` be the year fraction from settlement to maturity under the chosen day-count `basis`, and
`d` the discount rate. Bank discount says the price is the redemption less a charge on the
redemption:

    investment = received × (1 − d × T)

`RECEIVED` solves that for the redemption:

    received = investment / (1 − d × T)

With `B` the number of days in the year implied by the basis and `D` the day count from settlement
to maturity, `T = D/B` and

    RECEIVED = investment / (1 − discount × D / B)

Microsoft's page prints its equation as an image that could not be read. The expression above is
the one the notes below record from Excel's own answers.

**The word "discount" is the point of this function.** The second input is a discount rate, charged
on the redemption. It is not an interest rate on the investment, and using one here does not give
the right answer. With an interest rate `r`, the amount received is `investment × (1 + r × T)`,
which is an ordinary multiplication and needs no function. Over short spans the two are close
enough to hide the mistake, and over long spans they are not.

Microsoft's example: settlement 15 February 2008, maturity 15 May 2008, investment 1,000,000,
discount 5.75%, basis 2 (Actual/360). That is 90 days, so

    1,000,000 / (1 − 0.0575 × 90/360) = 1,014,584.654…

which matches the $1,014,584.65 Microsoft shows.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `settlement` | The date the buyer purchases the security, after its issue date. Required. | — |
| `maturity` | The date the security expires. Required. | — |
| `investment` | The amount invested in the security. Required. | — |
| `discount` | The security's discount rate, as a decimal fraction. Required. | — |
| `basis` | The day-count basis. Optional. | 0 |

The documented basis values are the standard five, as on the [DISC](FUNC.DISC.md) page: `0` or
omitted = US (NASD) 30/360, `1` = Actual/actual, `2` = Actual/360, `3` = Actual/365, `4` = European
30/360. Dates and `basis` are documented as truncated to integers.

**`investment` is an amount, not a per-100 price.** The result is proportional to it and is in the
same units, so a per-100 price returns a per-100 redemption. There is no redemption argument to
keep consistent, unlike [DISC](FUNC.DISC.md) and [INTRATE](FUNC.INTRATE.md).

**There is no issue date, whatever the page says.** Microsoft's page defines its day count `DIM` as
the days from *issue* to maturity, and its example labels the first date a settlement (issue) date.
The function has no issue argument. Every numeric row behind the notes below fits a count from
`settlement`.

Numeric coercion of the argument slots follows the shared call model; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`, in the units of `investment`.

- **A pole at `d × T = 1`.** As the denominator falls towards zero the result grows without limit,
  and past that point the formula would give a negative redemption. Excel refuses instead. In the
  2,400 rows behind the notes below (Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H), all
  282 rows with `1 − d × D/B ≤ 0` return `#NUM!`, and all 2,118 rows with a positive denominator
  return a number. Microsoft's page does not list this condition.
- **A 10% discount rate reaches the pole at ten years.** The function is usable with long spans
  only while `d × T` stays below 1, and well before that the answer stops being commercially
  meaningful.
- **The result is always greater than `investment`** for any admissible input, because `discount`
  must be positive.
- **`settlement ≥ maturity`** is documented as `#NUM!`, not a reversed reading.
- **The fractional part of a date is dropped.** 195 of the rows carry a fractional date, and each
  fits the integer day count.
- Empty, missing and error arguments are expected to follow the shared call model; see
  [The value universe](../model/01-value-universe.md). No reading records this for `RECEIVED`.

## Errors

As documented on Microsoft's `RECEIVED` page, with what the recorded rows add:

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | `settlement` or `maturity` is not a valid date | Documented; not measured |
| `#NUM!` | `investment ≤ 0` or `discount ≤ 0` | Documented; not measured |
| `#NUM!` | `basis < 0` or `basis > 4` | Documented; not measured |
| `#NUM!` | `settlement ≥ maturity` | Documented; not measured |
| `#NUM!` | `1 − discount × D/B ≤ 0` | Not documented; 282 of 282 rows |

Where documentation and behaviour differ, both are shown: the last row is behaviour Microsoft does
not mention. The rows contain no other error, so the first four are Microsoft's statements only.

The reference engine at commit `473efa3` raises all five with the same codes, testing the
denominator after forming the year fraction. It also refuses a date serial below 1 or past the end
of year 9999 with `#VALUE!`. That lower limit is not Excel's rule elsewhere in this family: the
notes on [YEARFRAC](FUNC.YEARFRAC.md) record serial 0 as an accepted date. Whether `RECEIVED` accepts
it has not been read.

## Relationships

- **[DISC](FUNC.DISC.md)** — the inverse in the rate: `DISC(s, m, investment, RECEIVED(s, m,
  investment, d, b), b)` returns `d` up to floating-point rounding. `DISC` documents its third
  argument as a per-100 price, but it uses the two money arguments only as a ratio.
- **[PRICEDISC](FUNC.PRICEDISC.md)** — the same relation solved for the price. `PRICEDISC` multiplies
  by `(1 − d × T)` where `RECEIVED` divides by it. The two also differ at the far end:
  `PRICEDISC` returns a negative price where `RECEIVED` returns `#NUM!`.
- **[INTRATE](FUNC.INTRATE.md)** — the rate on the investment rather than on the redemption. It is
  not `RECEIVED`'s rate, so `RECEIVED(s, m, investment, INTRATE(...), b)` does not reproduce the
  redemption. Convert first with `d = r / (1 + r × T)`, or compute `investment × (1 + r × T)`.
- **[YEARFRAC](FUNC.YEARFRAC.md)** — the year fraction on its own, and the quickest way to separate a
  calendar question from a rate question.
- **`TBILLPRICE`** — the nearest T-bill relative, with price rather than redemption as the unknown.

## Numerical notes

**The order of operations.** The first note below gives Excel's order: the discount and the day
count are multiplied, that product is divided by the year length, and the investment is divided by
the result. The reference engine at commit `473efa3` forms the year fraction first and computes
`investment / (1 − discount × T)`. The two are algebraically the same and can differ in binary64 in
the last places.

**Near the pole.** When `d × T` is close to 1 the subtraction in the denominator cancels, and the
result inherits a large relative error. This is intrinsic to the quantity: a small change in
`discount` moves the answer by a large amount, in any arrangement of the arithmetic. The
`#NUM!` boundary also depends on rounding at that point, so an input sitting exactly on it can fall
either way between implementations that order the operations differently.

**The year length at basis 1.** Microsoft says only that `B` is the number of days in the year.
Reading `B` back from the 696 numeric basis 1 rows gives one rule, and all 696 fit it to at least
ten significant figures:

- for a span of at most one year, if both dates are in one calendar year, `B` is that year's
  length; otherwise `B` is 366 if a 29 February lies in the span and 365 if not;
- for a span of more than one year, `B` is the average length of the calendar years from the
  settlement year to the maturity year, both included.

The same rule fits [PRICEDISC](FUNC.PRICEDISC.md), whose page gives two worked spans. The reference
engine at commit `473efa3` accumulates a fraction of each calendar year against that year's own
length instead, which does not agree with these rows on spans that cross a year boundary. The notes
below add the 1900 case: the fictitious 29 February does not make 1900 a leap year.

**The 30/360 bases.** All numeric rows at bases 0 and 4 fit the standard US and European 30/360
counts.

## What has not been checked

No Handbook vector suite exists for `RECEIVED`; `vectors/` publishes nothing at this revision. No
evidence record names this function in its subjects. What is measured is what the notes below
record, plus the row readings stated on this page, all on one Excel build unless a note says
otherwise. The following is still unmeasured.

1. **The four documented errors.** The only error in the rows is the undocumented pole. The first
   probes are `settlement = maturity`, `investment = 0`, `discount = 0` and `basis = 5`.
2. **The exact boundary of the pole.** The rows separate positive from non-positive denominators,
   but none is known to sit exactly at zero. Which side an input on the boundary falls depends on
   the order of the arithmetic.
3. **Date serials outside the ordinary range.** Serial 0, a negative serial and a serial past the
   end of year 9999. The lowest settlement in the rows is serial 32, and the reference engine's
   `#VALUE!` below serial 1 is not a statement about Excel.
4. **Fractional `basis`.** Every row has an integer basis.
5. **Non-numeric arguments.** Logicals, text dates and empty cells have no reading for `RECEIVED`,
   and an argument left empty in the call and an array in any slot have none either.
6. **Other Excel builds, platforms and bitness.** The rows are from one build on one 64-bit Windows
   machine.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `RECEIVED` that an implementer matching Excel, or a reader explaining an
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
| fully invested | No cash flows between settlement and maturity |
| bank discount | A rate charged on the redemption value, not on the amount paid |
| pole | The point `d × T = 1`, where the formula's denominator reaches zero |
| day-count basis | The `basis` argument's calendar convention, one of the five documented methods |
| year fraction `T` | The settlement-to-maturity span in years under the chosen basis |
| D / B | The settlement-to-maturity day count and the basis year length; Microsoft writes `DIM` for `D` |
| recorded rows | Excel's answers to generated inputs, which the notes below cite; a row fits a rule when it agrees to the stated precision |

## Sources

- Microsoft, "RECEIVED function" —
  <https://support.microsoft.com/en-us/office/received-function-7a3f8b93-6611-4f81-8576-828312c9b5e5>
  (syntax, argument meanings, the day-count basis table, the serial-date and truncation remarks, the
  `#VALUE!` and `#NUM!` conditions, the wording of `DIM` and the worked example). The equation on
  that page is an image and was not readable, so no formula here is sourced from it.
- The function notes on this page, and the recorded Excel rows and readings their evidence lists
  cite. The pole counts, the fractional-date count and the basis 1 year-length rule were read from
  those 2,400 rows for this page and go beyond the notes' own text; they carry the same Excel build,
  platform and CPU.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook, [Claim language and honesty](../model/06-claim-language.md) — the scoping rules this
  page's claims are written under.
- OxFunc `crates/oxfunc_core/src/functions/discount_bill_yearfrac_family.rs` at commit `473efa3`
  (the reference engine's `received_kernel`, its year-fraction routine, and its date and basis
  guards).
- Handbook projections `data/functions/FUNC.RECEIVED.json` and
  `data/presence/FUNC.RECEIVED.json` (arity, classification axes, implementing module and sibling
  set).
