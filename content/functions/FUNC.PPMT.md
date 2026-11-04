---
schema: efh.function-page/v1
function_id: FUNC.PPMT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-FIN-0001
  - EV-FIN-0002
  - EV-FIN-0005
  - EV-FIN-0017
  - EV-FIN-0019
open_problems: []
references:
  - work: "Microsoft Support — PPMT function"
    locator: "https://support.microsoft.com/en-us/office/ppmt-function-c370d9e3-7749-4ca4-beea-b06c6ac95e1b"
    role: "syntax, argument meanings including the range of per, the unit remark and the two worked examples"
  - work: "Microsoft Learn — WorksheetFunction.Ppmt method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.ppmt"
    role: "the type table and the unit example"
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
  - Excel versions, platforms and CPUs
  - Page vocabulary
  - Sources
family: financial_time_value_family
role_in_family: >-
  The principal half of one scheduled payment: mathematically the payment less the interest, and in
  Excel a product of PMT's quotient, a discount factor and the rate, not a subtraction.
---

# PPMT

## What it computes

`PPMT(rate, per, nper, pv, [fv], [type])` returns the **part of the payment in period `per` that
repays principal**, for an annuity with constant periodic `rate` and constant payment.

The definition is a split of the payment into two parts:

    PPMT(per) = PMT − IPMT(per)

Interest is charged on the balance entering the period, and whatever is left of the payment reduces
the balance. In exact arithmetic that identity holds for every period. In Excel's stored answers it
does not hold in the last bits, which is the main thing to know about this function as a piece of
arithmetic; see *Numerical notes*.

With end-of-period payments the definition has a simple closed form. Each period the interest falls
by `rate` times the previous principal repayment, so the principal repayment grows by the factor
`1 + rate`:

    PPMT(per) = (PMT + pv·rate) · (1 + rate)^(per − 1)              type = 0

When `fv = 0` this collapses further, because the last repayment clears the balance:

    PPMT(per) = PMT · (1 + rate)^−(nper − per + 1)                  type = 0, fv = 0

So the first repayment is the payment discounted over the whole term, and the last is the payment
discounted by one period. That second form is close to what Excel evaluates.

Two checks follow, both exact in the mathematics:

1. **The repayments add up.** The sum of `PPMT` over all periods is `−(pv + fv)`.
2. **With `rate = 0`** interest vanishes and every period repays `−(pv + fv)/nper`.

Worked examples from Microsoft's page, both reproduced by the first closed form:

| Formula | Meaning | Result |
|---|---|---|
| `=PPMT(10%/12, 1, 2*12, 2000)` | Principal in month 1 of a two-year loan of 2,000 | `-75.62` |
| `=PPMT(8%, 10, 10, 200000)` | Principal in year 10 of a ten-year loan of 200,000 | `-27598.05` |

### The `type = 1` schedule

With `type = 1` the first payment falls at time zero, before any interest has accrued, so all of it
is principal:

    PPMT(1) = PMT(rate, nper, pv, fv, 1)        type = 1

Later periods follow the definition from a balance already reduced by that payment. Getting this
shift wrong gives answers that are right in the middle of a schedule and wrong at its ends, and
`EV-FIN-0017` records an open type-1 defect in this area; see *What has not been checked*.

### The sign convention

As for the rest of the family, money paid out is negative. For a loan entered as a positive `pv`,
`PMT`, `IPMT` and `PPMT` are all negative. Repayments grow in magnitude through the life of a loan
while interest shrinks.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `rate` | The interest rate **per period**. Required. | none |
| `per` | The period wanted, in the range `1` to `nper`. Required. | none |
| `nper` | The total number of payment periods. Required. | none |
| `pv` | The present value: the total that the future payments are worth now. Required. | none |
| `fv` | The balance wanted after the last payment. Optional. | 0 |
| `type` | `0` for payments at the **end** of each period, `1` for the **beginning**. Optional. | 0 |

- **`per` comes before `nper`.** `PMT` has no `per`, so a formula edited from `PMT` to `PPMT` by
  inserting an argument in the wrong slot gives a plausible wrong answer, or `#NUM!`.
- **`per` starts at 1.** Contrast [ISPMT](FUNC.ISPMT.md), which counts periods from 0.
- **`rate` and `nper` must use the same period.** Monthly payments on a four-year loan at 12% a year
  are `12%/12` and `4*12`.
- The reference engine reads a blank or missing argument as `0`, truncates `type` and treats any
  nonzero result as beginning-of-period. Microsoft documents only `0` and `1`.

## Result and edge cases

Returns `Number`, a principal amount carrying the family sign convention.

The cases below are recorded answers from a sample of 396 `PPMT` readings on Excel 16.0 build 19929,
Windows, bitness and CPU not recorded, classified for this page.

- **`per` below 1** is `#NUM!`: 157 readings, all with `per = 0`, including rate zero and both
  `type` values.
- **`per` above `nper`** is `#NUM!`: 164 readings, 16 of them with `type = 1`. So the range in
  Microsoft's argument description is enforced at both ends in everything read here.
- **`rate = 0`** gives `−(pv + fv)/nper` where the answer is nonzero:
  `PPMT(0, 1, 139, 337711.21596695744, -161658.71376712981, 0)` is `-1266.564764027537`.
- **`pv = 0` and `fv = 0`** give `0` at every period read.
- **A high rate over a long term** is the case to watch. `EV-FIN-0019` records two adjacent rows
  where the reference engine returned `#NUM!` and Excel returned a tiny value or zero. The reference
  engine reaches `PPMT` through a balance that grows like `(1 + rate)^per`, which can overflow. The
  route in the notes multiplies by a discount factor that shrinks instead, so a tiny number or zero
  is what it would give. That explanation is inferred; the two rows are what was measured.

## Errors

Microsoft's `PPMT` page publishes no table of error conditions; it states only the range of `per`.

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | `per < 1` or `per > nper` | Microsoft's range; 321 recorded readings |
| `#NUM!` | `rate ≤ −1` | reference engine at `473efa3`, through its `PMT` guard |
| `#VALUE!` | A numeric slot receives non-numeric text or an unsupported value kind | reference engine, under the shared call model |
| propagated | An error value in any argument surfaces as that error | reference engine, under the shared call model |
| `#VALUE!` | Fewer than four or more than six arguments | reference engine, under the shared call model |

Only the first row comes from Excel readings. See
[The value universe](../model/01-value-universe.md) and
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Relationships

- **[PMT](FUNC.PMT.md)** is the payment being split. Excel's `PPMT` starts from `PMT`'s quotient, not
  from `PMT`'s answer.
- **[IPMT](FUNC.IPMT.md)** is the interest half. The IPMT page, which is older, says `IPMT + PPMT =
  PMT` exactly, suggests testing that to the last bit, and says the two share every error because
  one is computed from the other. That describes the reference engine at commit `473efa3`, which
  does compute `PPMT` as `PMT − IPMT`. The notes below show it does not describe Excel, and they are
  the later record.
- **[CUMPRINC](FUNC.CUMPRINC.md)** sums principal over a range of periods. On the 30-year loan its
  answer over periods 1 to 12 equals the left-to-right sum of Excel's own twelve `PPMT` answers, but
  this does not generalise. Over recorded single-period calls with `type = 0`, `CUMPRINC` equals
  `PPMT` on 56 of 135 rows, and its own route is open. See the
  [function notes](../notes/NOTES.md).
- **[CUMIPMT](FUNC.CUMIPMT.md)** is the matching sum of interest.
- **`PV`, [FV](FUNC.FV.md), `NPER`, `RATE`** solve the annuity equation behind all of these.
- **[ISPMT](FUNC.ISPMT.md)** is a different loan model, with level principal and a 0-based period.
  Its principal part is constant, unlike `PPMT`'s.
- **Confused with**: the principal on a lender's statement, which reflects rounding to cents and
  day counts. `PPMT` is the idealized schedule.

## Numerical notes

The notes below give the formula with its counts. This section says what the formula means for
someone using or reproducing the function.

**`PPMT` is not a subtraction.** For `type = 0` Excel takes `PMT`'s quotient `q`, subtracts `fv`,
multiplies by the stored exponential `u = exp(t)` with `t = −m·L` and `m = nper − per + 1`, and
multiplies by `rate`. With `fv = 0` that is the closed form `PMT · (1 + rate)^−m`. So the two
answers on a page, `PMT` and `PPMT`, come from different last steps, and `IPMT` takes a third path
through `expm1` where `PPMT` takes the exponential it starts from.

**So the identity fails in the last bits.** On the 30-year loan of 200,000 at 5%, over the first
twelve periods, `PMT − IPMT` reproduces Excel's `PPMT` in only 2 periods and `PMT − PPMT` reproduces
`IPMT` in 7, in binary64 from Excel's own answers. A check column that compares `PPMT` with
`PMT − IPMT` for exact equality will fail on ordinary inputs, and that is not an error in the
workbook. Compare with a tolerance. An implementation that derives one from the other will miss
Excel's last bits, as the reference engine's source records for itself at commit `473efa3`, with an
open one-unit gap on the first period of that loan.

**Why `PPMT` is further from the exact value than `PMT`.** The notes record every answer on that
loan 3 to 5 units from the correctly rounded principal, where `PMT` is 1 unit away. A likely part of
the reason is arithmetic. The stored logarithm `L` is about 8.3e−19 below the true `ln(1 + rate)` at
this rate, and the exponent multiplies that error by `m`. At `m = 360` the exponential is therefore
about 3.0e−16 too large in relative terms, which is well over half of `PPMT`'s relative error of
about 4.7e−16 in the first period. `PMT` feels the same error only through `u − 1`, where it is
diluted because `u` is about 0.22. This paragraph is the Handbook's own arithmetic from the values in
the notes, not a separate measurement.

**No cancellation in the `fv = 0` route.** With `fv = 0` the only subtraction is the one inside
`em`, which the `expm1` form protects, and everything after it is a product of stored values. So,
unlike the balance recurrence described on the IPMT page, the route has no reason to lose digits
near the end of a schedule, and the error carried in from `L` shrinks with `m`. That is what the
arithmetic predicts; the notes count matches, and do not report a spread by period beyond the first
twelve.

**Small rates.** The whole family's discipline applies here: an exact `rate == 0` test in the
reference engine, and a logarithm-based form that keeps tiny nonzero rates on the general path.

## What has not been checked

No Handbook vector suite exists for `PPMT`; `vectors/` publishes nothing at this revision, so no
suite-scoped claim exists for it.

The notes below describe `type = 0`, and match all recorded rows with `fv = 0`, in the rate range
where `PMT`'s logarithm is identified. Still unmeasured or open after them:

1. **`type = 1`.** The route is open, because `PMT`'s stored factor for that case is open. The same
   gap reaches `PPMT`. `EV-FIN-0017` records an open type-1 defect in the reference engine, checked
   on four witnesses at ten decimal places only, and it publishes no count on purpose.
2. **Nonzero `fv`.** `PMT`'s numerator is open and carries through. The formula in the notes fits 25
   of 27 such rows with `type = 0`, so the two others are unexplained.
3. **Rates outside −1/3 to 1/2.** `PPMT` inherits `PMT`'s open logarithm there, and the note on
   `PMT` records a one-unit miss inside that range too, at rate `0.419`.
4. **`rate ≤ −1`, and negative or fractional `nper`.** None appears in the 396-row sample.
5. **Fractional `per`.** Microsoft's text says only that `per` lies between 1 and `nper`. Every `per`
   in the sample is a whole number. The reference engine accepts a fractional `per` and does not
   truncate it. Whether Excel truncates, rounds or interpolates is unread here.
6. **`type` other than `0` or `1`, and non-numeric arguments.** Unread here.
7. **High rates over long terms.** Only the two rows behind `EV-FIN-0019` address this. Where the
   tiny answers turn to zero has not been mapped.
8. **Other environments.** No input has been read in two environments, so no difference between
   builds, CPUs or platforms is known and none is ruled out. Nothing has been read on a Mac, in Excel
   for the web, on 32-bit Excel or on an Intel CPU. A CPU dependence is expected and untested.

Five evidence records name `PPMT`. They measure the **reference engine**, or a research model, and
not the route in the notes. Their counts render beside this page with their reader warnings:

- **`EV-FIN-0001`** is a replay over a **group** of annuity functions. Its total may not be
  attributed to any one of them.
- **`EV-FIN-0002`** is the Handbook's per-function split of that replay, arithmetic over a stored
  file and not a live-Excel measurement.
- **`EV-FIN-0005`** holds scores from a research model, which the source forbids reporting as
  reference-engine pass rates.
- **`EV-FIN-0017`** is the type-1 defect stream described above.
- **`EV-FIN-0019`** holds two older joint samples whose counts are group totals and may not be
  distributed to `PPMT`.

Taken together, these records do not support a statement that the reference engine at commit
`473efa3` agrees with Excel on `PPMT`, and the source of that engine says it does not.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `PPMT` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-IPMT-001: IPMT and PPMT reuse PMT's quotient, one through expm1 and the other through exp

*Detail for matching Excel's bits; measured; recorded 2026-11-04.* Applies to IPMT, PPMT.

With RN(v) meaning v rounded to 64 bits on the x87 unit and then stored to binary64, take L and
q from PMT (FN-PMT-001; with nonzero fv, q = RN(N/em) with an fv numerator that is open). For
period k of nper with end-of-period payments, m = nper-k+1, t = -(m*L), em_m is the expm1 of t
by the route in FN-PMT-001, and u = exp(t) stored. Then IPMT = -RN(RN(RN(q*em_m) -
RN(fv*u))*rate) and PPMT = RN(RN(RN(q - fv)*u)*rate). Interest takes the Kahan expm1, and
principal takes the exponential it starts from: 1+em_m fits 36 of 74 principal rows where u
fits all 74. So neither answer is the payment less the other (FN-PPMT-001).

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `=IPMT(0.05/12,12,360,200000)` | as typed | `-822.0867740866778` (`0xc089b0b1b69cc35f`) | `-822.0867740866777` (`0xc089b0b1b69cc35e`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=PPMT(0.05/12,12,360,200000)` | as typed | `-251.5564719376004` (`0xc06f71ce9e3ca456`) | `-251.5564719376003` (`0xc06f71ce9e3ca452`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Correct value: mpmath at 800 bits with rate the binary64 0x3f71111111111111 that 0.05/12
evaluates to; correctly rounded.

Basis (measured): fv 0, type 0: IPMT 95 of 95 and PPMT 74 of 74 recorded rows, plus 64 of 64
fresh PPMT(r,1,1,pv) and PPMT(r,1,2,pv) readings. Nonzero fv, type 0: PPMT 25 of 27, IPMT 18 of
27. Over every recorded row in the evidence repository IPMT matches 48 of 57 and PPMT 26 of 33.
Replaying the formulas in exact rational arithmetic, with exp and ln correctly rounded, gives
all 12 PPMT and all 12 IPMT answers of the loan below.

- Open: Nonzero fv: PMT's open numerator of q carries through.
- Open: type 1: PMT's open factor K carries through.

Evidence:

- [`content/notes/evidence/2026-11-04/annuity-EVIDENCE.txt`](../../content/notes/evidence/2026-11-04/annuity-EVIDENCE.txt), section UPDATE — research write-up
- DnaCalc `crates/calc-func/src/ported/financial_time_value_family.rs`, end_payment_parts at `6c6cbd31c9` — the formulas in the product; the commit message carries the row counts
- DnaCalc `crates/calc-func/tests/f3_financial.rs`, ipmt_and_ppmt_use_the_quotient_of_pmt at `6c6cbd31c9` — eight catalogue readings; failed before the change
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcj9k_annuity.capture.json`, sheet dcj9k_annuity, row 750, column 4 at `c9f9d35416` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcj9k_annuity.capture.json`, sheet dcj9k_annuity, row 749, column 4 at `c9f9d35416` (readings)

### FN-PPMT-001: PPMT is not PMT minus IPMT, and sits further from the exact value than PMT

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.* Applies to PMT, PPMT.

On the 30-year loan PPMT(0.05/12,k,360,200000), for k from 1 to 12 every PPMT answer is 3 to 5
units above the correctly rounded principal payment, while PMT is 1 unit away. Excel's own
answers do not satisfy the identity: in binary64, PMT - IPMT equals PPMT in 2 of the 12 periods
and PMT - PPMT equals IPMT in 7. So reproducing PPMT from PMT and IPMT misses the last bits.
FN-IPMT-001 gives the route Excel takes instead.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `=PPMT(0.05/12,1,360,200000)` | as typed | `-240.30991269094474` (`0xc06e09eace050723`) | `-240.30991269094463` (`0xc06e09eace05071f`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Correct value: mpmath at 800 bits with rate the binary64 0x3f71111111111111 that 0.05/12
evaluates to; correctly rounded.

Basis (measured): Twelve recorded periods of one loan, read from the catalogue capture,
compared with exact values and with Excel's own PMT and IPMT by binary64 subtraction,
recomputed for this note.

Evidence:

- [`content/notes/evidence/2026-11-04/annuity-EVIDENCE.txt`](../../content/notes/evidence/2026-11-04/annuity-EVIDENCE.txt) — research write-up; finding F4 gives 4 and 8 of 12 periods, and the capture gives the 2 and 7 stated here
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcj9k_annuity.capture.json` at `c9f9d35416` — recorded answers of the 30-year loan block
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcj9k_annuity.capture.json`, sheet dcj9k_annuity, row 727, column 4 at `c9f9d35416` (readings)

<!-- efh:notes end -->

## Excel versions, platforms and CPUs

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.PPMT.json; edits here are overwritten -->

Readings of PPMT with a recorded Excel build: 403 input(s) over 2 recorded environment(s),
Excel 16.0 build(s) 19929, 20131, all on Windows; recorded CPUs: AMD Zen 2. 7 input(s) were
also read in captures that record no build (some may repeat inputs counted above); they are not
counted. No input has been read in two environments, so no difference is known and none is
ruled out. A CPU dependence is expected and untested (see below). No reading from a Mac, Excel
for the web, 32-bit Excel or an Intel CPU is recorded.

Readings of `PPMT` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 396 | date not recorded |
| 16.0 build 20131 | Windows, bitness not recorded | AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | 7 | 2026-07-23 |

No check has read the same inputs in two environments, so the absence of a listed difference says nothing about other builds, platforms or CPUs.

Differences observed: none.

Expected, not yet tested (XS-PPMT-001, cpu): Handbook evidence record EV-FIN-0005 marks 7 of
its counts for this function as CPU-scoped: the counted result depends on how the host CPU
executes x87 instructions. Every reading recorded here was taken on an AMD Zen 2 processor or
on a host whose CPU was not recorded; no reading on an Intel CPU, a Mac or Excel for the web
exists, so a CPU difference is neither observed nor ruled out.

Evidence for every row: [`content/evidence/environments/FUNC.PPMT.json`](../../content/evidence/environments/FUNC.PPMT.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| principal repayment | The part of a payment that reduces the balance |
| quotient `q` | `PMT`'s stored `pv / em`, with `fv` entering through an open numerator |
| `m` | `nper − per + 1`, the number of periods left including this one |
| `u` | The stored exponential `exp(−m·L)`, the discount factor over `m` periods |
| `L` | The stored value of `ln(1 + rate)` that Excel uses |
| one unit | One unit in the last place of a binary64 value |

## Sources

- Microsoft Support, "PPMT function" —
  <https://support.microsoft.com/en-us/office/ppmt-function-c370d9e3-7749-4ca4-beea-b06c6ac95e1b>
  (syntax, argument meanings, the unit remark and the two examples). Read through a fetch that
  returned a summary with quoted text, not the raw page.
- Microsoft Learn, "WorksheetFunction.Ppmt method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.ppmt> (the `type` table
  and the unit example).
- Handbook, [PMT](FUNC.PMT.md) and [FV](FUNC.FV.md) — the shared equation, and the logarithm and
  quotient this function reuses.
- Handbook, [Function notes](../notes/NOTES.md), and the research write-up they cite.
- Handbook evidence records `EV-FIN-0001`, `EV-FIN-0002`, `EV-FIN-0005`, `EV-FIN-0017` and
  `EV-FIN-0019`, each with its own reader warning and scope.
- The 396 recorded `PPMT` readings on build 19929 listed in this page's Excel versions section,
  classified for this page by the range of `per`, the rate and `type`.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- OxFunc `crates/oxfunc_core/src/functions/financial_time_value_family.rs` at commit `473efa3` — the
  `ppmt` kernel, its reliance on `pmt` and `ipmt`, the `per` guard, and the test recording its gap on
  the first period of the 30-year loan, read as implementation facts about that engine.
- Handbook projections `data/functions/FUNC.PPMT.json` and `data/presence/FUNC.PPMT.json` (arity,
  classification axes, implementing module and sibling set).
