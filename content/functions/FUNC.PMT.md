---
schema: efh.function-page/v1
function_id: FUNC.PMT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-FIN-0001
  - EV-FIN-0002
  - EV-FIN-0003
  - EV-FIN-0004
  - EV-FIN-0019
open_problems: []
references:
  - work: "Microsoft Support — PMT function"
    locator: "https://support.microsoft.com/en-us/office/pmt-function-0214da64-9a63-4996-bc20-214433fa6441"
    role: "syntax, argument meanings, the remarks on what the payment includes and on units, and the three worked examples"
  - work: "Microsoft Learn — WorksheetFunction.Pmt method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.pmt"
    role: "the type table and the unit example"
  - work: "Abramowitz and Stegun, Handbook of Mathematical Functions"
    locator: "formula 4.1.27"
    role: "the series for the logarithm that the notes identify in Excel's PMT"
  - work: "Higham, Accuracy and Stability of Numerical Algorithms, 2nd ed., SIAM 2002"
    locator: "section 1.14.1"
    role: "the device of forming expm1 from an exponential and a logarithm, which the notes attribute to Kahan"
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
  The annuity equation solved for the payment, and the one the schedule functions start from:
  IPMT, PPMT and the cumulative pair all begin with PMT's quotient or its equivalent.
---

# PMT

## What it computes

`PMT(rate, nper, pv, [fv], [type])` returns the **constant payment per period** that takes a
balance of `pv` to a balance of `fv` in `nper` periods at a constant periodic `rate`.

It is the annuity equation of this family solved for the payment. The equation is set out on the
[FV](FUNC.FV.md) page:

    pv·(1 + rate)^nper  +  pmt·(1 + rate·type)·((1 + rate)^nper − 1)/rate  +  fv  =  0

Solving for `pmt` gives

    PMT = −rate · (pv·(1 + rate)^nper + fv) / ( (1 + rate·type) · ((1 + rate)^nper − 1) )

and at `rate = 0`, where the equation is linear,

    PMT = −(pv + fv) / nper

Dividing top and bottom by `(1 + rate)^nper` gives the discounted form, which is how the function
is best thought about in floating point, because for a positive rate `(1 + rate)^−nper` cannot
overflow:

    PMT = rate · (pv + fv·(1 + rate)^−nper) / ( (1 + rate·type) · ((1 + rate)^−nper − 1) )

With `fv = 0` and `type = 0` this reduces to `pv / em × rate`, with `em = (1 + rate)^−nper − 1`. That
small expression is what Excel evaluates, in that order, as the notes below record.

Three worked examples from Microsoft's page, all reproduced by the formula above:

| Formula | Meaning | Result |
|---|---|---|
| `=PMT(8%/12, 10, 10000)` | Monthly payment on a loan of 10,000 over 10 months | `-1037.03` |
| `=PMT(8%/12, 10, 10000, , 1)` | The same, paid at the start of each month | `-1030.16` |
| `=PMT(6%/12, 18*12, 0, 50000)` | Monthly saving to reach 50,000 in 18 years | `-129.08` |

### The sign convention

The family's convention holds: **money you pay out is negative.** A loan entered as a positive
`pv` is cash received, so the payment is negative. A savings target entered as a positive `fv` is
cash to be received later, so the deposits are negative. A positive result is not an error; it
answers a different question, the same one with the cash flows reversed.

### What the payment contains

Microsoft's page says the result covers principal and interest and excludes taxes, reserve payments
and fees. Multiplying it by `nper` gives the total paid over the life of the loan.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `rate` | The interest rate **per period**. Required. | none |
| `nper` | The total number of payments. Required. | none |
| `pv` | The present value: the total that the future payments are worth now, the principal of a loan. Required. | none |
| `fv` | The balance wanted after the last payment. Optional. | 0 |
| `type` | When payments fall due: `0` at the end of each period, `1` at the beginning. Optional. | 0 |

- **`rate` and `nper` must use the same period.** Microsoft's example: monthly payments on a
  four-year loan at 12% a year are `12%/12` and `4*12`. Annual payments on the same loan are `12%`
  and `4`.
- **`type = 1` makes each payment smaller by the factor `1 + rate`.** The formula divides by
  `1 + rate·type`, because each payment is made one period earlier and carries one period less
  interest. `1037.03 / 1.00667` is `1030.16`, which is Microsoft's pair above.
- **An empty `fv` slot is `0`.** `=PMT(A2/12, A3, A4, , 1)` is the documented way to reach `type`.
- The reference engine reads a blank or missing argument as `0`, and truncates `type` and treats
  any nonzero result as beginning-of-period. Microsoft documents only `0` and `1`. The reference
  engine's reading of other `type` values is an implementation fact; Excel's has not been read
  here.

## Result and edge cases

Returns `Number`, a payment carrying the family sign convention.

Recorded behaviour, from the sample of 47 `PMT` readings on Excel 16.0 build 19929, Windows,
bitness and CPU not recorded:

- **`rate = 0`** gives `-(pv + fv)/nper`, and `type` has no effect, as the formula says. The sample
  holds one call of each type.
- **`pv = 0` and `fv = 0`** give `0`.
- **A negative rate above −1** gives an ordinary number. The larger set of 289,642 readings on build
  20131 holds 39,051 rows with a negative rate, all numeric.

Stated by the reference engine at commit `473efa3`, and not read in Excel for this page:

- **`nper = 0`** is `#NUM!`.
- **`rate ≤ −1`** is `#NUM!`, because `1 + rate` has no real power. Its source says this matches live
  Excel, and the [FV](FUNC.FV.md) page records that the siblings differ in this guard.
- **The zero-rate test is exact.** A merely tiny rate takes the general path and is not collapsed to
  the linear case.

## Errors

Microsoft's `PMT` page lists no error conditions. What the Handbook can state:

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | `nper = 0`, or `rate ≤ −1`, or the result cannot be represented | reference engine at `473efa3` |
| `#VALUE!` | A numeric slot receives non-numeric text or an unsupported value kind | reference engine, under the shared call model |
| propagated | An error value in any argument surfaces as that error | reference engine, under the shared call model |
| `#VALUE!` | Fewer than three or more than five arguments | reference engine, under the shared call model |

None of these rows comes from a reading used for this page. See
[The value universe](../model/01-value-universe.md) and
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Relationships

- **[PPMT](FUNC.PPMT.md)** and **[IPMT](FUNC.IPMT.md)** split the payment into principal and
  interest. In exact arithmetic `PPMT + IPMT = PMT`. In Excel's stored answers it often does not
  hold in the last bits, because the two take different routes from `PMT`'s quotient; the notes
  below give the counts.
- **[CUMPRINC](FUNC.CUMPRINC.md)** and **[CUMIPMT](FUNC.CUMIPMT.md)** sum those pieces over a range.
  On the 30-year loan the [function notes](../notes/NOTES.md) find `CUMIPMT` equal to `12*PMT`
  less `CUMPRINC`, computed from Excel's own answers. Beyond that block it is inferred, not measured.
- **`PV`, [FV](FUNC.FV.md), `NPER`, `RATE`** solve the same equation for the other unknowns.
  `PV(rate, nper, PMT(rate, nper, pv))` should return `pv` up to rounding.
- **[ISPMT](FUNC.ISPMT.md)** is a different loan model, with level principal and not a level payment.
- **Confused with**: a lender's actual payment. Real schedules round to cents and use day counts,
  which `PMT` does not model.

## Numerical notes

The notes below give the route with its counts and its open items. This section explains why the
route has the shape it has.

**Why the formula is not evaluated as written.** Computing `(1 + rate)^nper − 1` directly is a
cancellation at small rates, and it needs a power that can overflow. Working with
`t = −nper·ln(1 + rate)`, then `exp(t)` and `expm1(t)`, avoids both. The reference engine's source
records that an earlier power-based form was wrong by up to about 5.5e8 units in the last place on
its `PMT` test rows. In the region the notes cover, Excel takes this logarithm-and-exponential route
too.

**The logarithm is the surprise.** One would expect a correctly rounded `ln(1 + rate)`. The
reference engine's source says so at commit `473efa3`, and evidence record `EV-FIN-0004` places the
remaining disagreement in Excel's `expm1`. The notes below supersede both. For each rate tested, one
binary64 logarithm explains every `nper`. At nearly every rate that value comes from an `atanh`
series (Abramowitz and Stegun 4.1.27), and at many rates it is not the correctly rounded logarithm.
At `0.05/12` it is one unit low, and the common 30-year loan of 200,000 at 5% comes out one unit
from the correctly rounded payment. Excel is less accurate here than a good `log1p` would make it,
and matching Excel means reproducing that.

**`expm1` is formed from `exp` and `ln`.** For `|t| < 1` the notes record `(u − 1)·t / ln(u)` with
`u = exp(t)`, a device due to Kahan and explained in Higham's text, and plain `u − 1` otherwise.

**Order and precision matter in the last step.** `q = pv / em` is stored, and only then multiplied
by `rate`. Other orders, and plain binary64 at either step, leave groups of recorded answers with no
consistent `em`, which is how the order was established. An implementation that reaches for
`pv·rate/em` is correct mathematically and can differ from Excel in the last bits.

**The zero-rate branch is a discontinuity in the code, not in the mathematics.** The limit of the
general form as `rate → 0` is the linear form, but the two paths are different arithmetic. A
tiny nonzero rate on the general path depends on the logarithm and the exponential, where the
linear path is one addition and one division.

**Cancellation remains in one place.** With `fv` of the opposite sign to `pv` the numerator
`pv + fv·(1 + rate)^−nper` can be small, and its relative error then dominates. No note covers
that case, because the numerator with nonzero `fv` is open.

## What has not been checked

No Handbook vector suite exists for `PMT`; `vectors/` publishes nothing at this revision, so no
suite-scoped claim exists for it.

The notes below describe `fv = 0` and `type = 0` at rates from −1/3 to 1/2. Outside that, the
following is still not known:

1. **`type = 1`.** Excel multiplies the quotient by one stored constant near `rate/(1 + rate)` for
   each rate. That constant has been pinned down rate by rate, and how Excel forms it has not.
   The correctly rounded quotient fits only some of the rates.
2. **Nonzero `fv`.** The numerator of the quotient is open. The best form tried fits about three
   rows in four, and some rows admit no binary64 numerator under the known order of operations.
3. **Rates outside −1/3 to 1/2.** That range is where twelve series terms converge. It is not a
   measured boundary, and the note records a one-unit miss at rate `0.419`, inside it, so a range
   reduction may begin earlier. Negative rates inside the range rest on 20 rates' controls and the
   recorded rows.
4. **The domain edges.** `nper = 0`, negative or fractional `nper`, `rate ≤ −1`, and a `type` other
   than `0` or `1` do not appear in the two sets of readings surveyed for this page, the 289,642
   rows of build 20131 and the 47-row sample. The error table above is the reference engine's.
5. **Non-numeric arguments.** Text, logical values and blank cells are unread here.
6. **The opposite-signs numerator** described at the end of *Numerical notes*.
7. **Other environments.** No input has been read in two environments, so no difference between
   builds, CPUs or platforms is known and none is ruled out. Nothing has been read on a Mac, in
   Excel for the web, on 32-bit Excel or on an Intel CPU. The route in the notes uses the x87 unit,
   so a CPU dependence is expected and untested.

Five evidence records name `PMT`, and they measure the **reference engine against Excel**, not the
route in the notes. Each is narrower than it looks, and their counts render beside this page with
their reader warnings:

- **`EV-FIN-0001`** is a replay of one corpus over a **group** of annuity functions. Its total may
  not be attributed to any one of them.
- **`EV-FIN-0002`** is the Handbook's own per-function split of that replay. It is arithmetic over a
  stored file, not a live-Excel measurement.
- **`EV-FIN-0003`** is a full pass on the corpus a fix was pinned against, restricted to `fv = 0`. It
  is in-sample, and the corpus was the target of the repair it scores, so it must be read with
  `EV-FIN-0001`, `EV-FIN-0002` and `EV-FIN-0004` and never alone.
- **`EV-FIN-0004`** records the open disagreement and the ceilings of searched families. Its
  counts are not `PMT` pass rates. Its placing of the residual in `expm1` is superseded by the
  notes below, which place it in the logarithm.
- **`EV-FIN-0019`** holds two older joint samples whose counts are group totals.

Taken together: at commit `473efa3` the reference engine and Excel disagree numerically on `PMT`,
and that disagreement is open on those records. The notes below say what later work has identified
about Excel's own route; they are not a statement about that engine.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `PMT` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-PMT-001: PMT takes ln(1+rate) from a series, and stores its quotient before the rate

*Detail for matching Excel's bits; measured; recorded 2026-11-04.*

With RN(v) meaning v rounded to 64 bits on the x87 unit and then stored to binary64, PMT with
fv 0 and type 0 at rates from -1/3 to 1/2 runs: s = rate/(2+rate); L = 2s + 2s*(s^2*T), where T
= 1/3 + s^2/5 + ... + s^20/23 by Horner, all in plain binary64, which is the atanh series for
ln(1+rate) (Abramowitz and Stegun 4.1.27) with the 2s term added last; t = -(nper*L); u =
exp(t) stored; em = RN(RN(RN(u-1)*t)/ln(u)) for |t| < 1, and RN(u-1) otherwise, Kahan's way of
forming expm1; q = RN(pv/em); PMT = RN(q*rate). L is not the correctly rounded logarithm: at
rate 0.05/12 it is 0x3f7107fd59addc8a, one unit below 0x3f7107fd59addc8b, and the common
30-year loan below comes out 1 unit from the correctly rounded payment.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `=PMT(0.05/12,360,200000)` | as typed | `-1073.6432460242781` (`0xc090c692af15f63a`) | `-1073.643246024278` (`0xc090c692af15f639`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Correct value: mpmath at 800 bits with rate the binary64 0x3f71111111111111 that 0.05/12
evaluates to; correctly rounded.

Basis (measured): For each of 1,184 rates one binary64 L reproduces Excel's em at every nper
(1,622 of 1,622 power-of-two and 1,849 of 1,849 other nper, L taken from nper 1 alone). The
series gives that L at 1,181 rates; the correctly rounded log1p and fdlibm's log1p each give it
at 667. Order and precision of the last two steps: in all 4,802 groups of recorded answers
sharing (rate, nper) with 8 or more pv values, these steps admit one em; a binary64 quotient
leaves 59 groups with none, and binary64 for both steps 111. 720 of 720 fresh type 0 rows over
20 rates, both signs, match (Excel build 20527). Over every recorded PMT row, types and fv
included, 279,887 of 291,978 match.

Inferred, not measured: The range -1/3 to 1/2 is where twelve series terms converge, not a
measured Excel boundary. Rate 0.419, near sqrt(2)-1, misses by 1 unit, so a range reduction may
start there. Negative rates rest on 20 rates' controls and the recorded rows.

- Open: type 1: Excel multiplies q by one stored binary64 K near rate/(1+rate) per rate (pinned at all but one of about 458 recorded rates and at 20 of 20 fresh ones), but K's formula is open: the correctly rounded quotient fits 344 of 458.
- Open: Nonzero fv: the numerator of q is open. The best form tried fits 5,600 of 7,491 rows, and 146 rows admit no binary64 numerator under this order.
- Open: Rates outside -1/3 to 1/2.

Evidence:

- [`content/notes/evidence/2026-11-04/annuity-EVIDENCE.txt`](../../content/notes/evidence/2026-11-04/annuity-EVIDENCE.txt), section UPDATE — research write-up; the update supersedes the blocked outcome at its head
- DnaCalc `crates/calc-func/src/ported/financial_time_value_family.rs`, pmt at `08ea3d7ae0` — the logarithm and the stored quotient in the product; the commit message carries the row counts
- DnaCalc `crates/calc-func/src/ported/excel_numeric/mod.rs`, excel_log1p_atanh at `08ea3d7ae0` — the series
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcj9k_annuity.capture.json` at `c9f9d35416` — recorded answers of the 30-year loan block
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcj9k_annuity.capture.json`, sheet dcj9k_annuity, row 726, column 4 at `c9f9d35416` (readings)

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

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.PMT.json; edits here are overwritten -->

Readings of PMT with a recorded Excel build: 291,765 input(s) over 3 recorded environment(s),
Excel 16.0 build(s) 19929, 20131, 20228, all on Windows; recorded CPUs: AMD Zen 2. 289,642
input(s) were also read in captures that record no build (some may repeat inputs counted
above); they are not counted. No input has been read in two environments, so no difference is
known and none is ruled out. A CPU dependence is expected and untested (see below). No reading
from a Mac, Excel for the web, 32-bit Excel or an Intel CPU is recorded.

Readings of `PMT` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 47 | date not recorded |
| 16.0 build 20131 | Windows, bitness not recorded | AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | 289,642 | 2026-07-11 to 2026-07-23 |
| 16.0 build 20228 | Windows, 64-bit | CPU not recorded | 2,076 | 2026-08-09 |

No check has read the same inputs in two environments, so the absence of a listed difference says nothing about other builds, platforms or CPUs.

Differences observed: none.

Expected, not yet tested (XS-PMT-001, cpu): Handbook evidence record EV-FIN-0003 marks 1 of its
counts for this function as CPU-scoped: the counted result depends on how the host CPU executes
x87 instructions. Every reading recorded here was taken on an AMD Zen 2 processor or on a host
whose CPU was not recorded; no reading on an Intel CPU, a Mac or Excel for the web exists, so a
CPU difference is neither observed nor ruled out.

Expected, not yet tested (XS-PMT-002, cpu): Handbook evidence record EV-FIN-0004 marks 8 of its
counts for this function as CPU-scoped: the counted result depends on how the host CPU executes
x87 instructions. Every reading recorded here was taken on an AMD Zen 2 processor or on a host
whose CPU was not recorded; no reading on an Intel CPU, a Mac or Excel for the web exists, so a
CPU difference is neither observed nor ruled out.

Evidence for every row: [`content/evidence/environments/FUNC.PMT.json`](../../content/evidence/environments/FUNC.PMT.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| annuity equation | The balance equation shared by `PV`, `FV`, `PMT`, `NPER` and `RATE` |
| `em` | `(1 + rate)^−nper − 1`, the discounted annuity denominator; negative for a positive rate |
| quotient `q` | `pv / em`, stored before it is multiplied by `rate` |
| `L` | The stored value of `ln(1 + rate)` that Excel uses |
| `type` | Payment timing: `0` end of period, `1` beginning of period |
| one unit | One unit in the last place of a binary64 value |

## Sources

- Microsoft Support, "PMT function" —
  <https://support.microsoft.com/en-us/office/pmt-function-0214da64-9a63-4996-bc20-214433fa6441>
  (syntax, argument meanings, the two remarks and the three examples). Read through a fetch that
  returned a summary with quoted text, not the raw page.
- Microsoft Learn, "WorksheetFunction.Pmt method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.pmt> (the `type` table and
  the unit example).
- Abramowitz and Stegun, *Handbook of Mathematical Functions*, formula 4.1.27 — the series.
- Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd ed., section 1.14.1 — forming
  `expm1` from an exponential and a logarithm.
- Handbook, [FV](FUNC.FV.md) — the shared annuity equation and the sibling domain divergence.
- Handbook, [Function notes](../notes/NOTES.md), and the research write-up they cite.
- Handbook evidence records `EV-FIN-0001`, `EV-FIN-0002`, `EV-FIN-0003`, `EV-FIN-0004` and
  `EV-FIN-0019`, each with its own reader warning and scope.
- The recorded `PMT` readings listed in this page's Excel versions section: the edge cases above
  were read from the 47-row sample on build 19929 and classified across the 289,642 rows on build
  20131 for this page.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- OxFunc `crates/oxfunc_core/src/functions/financial_time_value_family.rs` at commit `473efa3` — the
  `pmt` kernel, its exact zero-rate test, its domain guards and its recorded commentary, read as
  implementation facts about that engine.
- Handbook projections `data/functions/FUNC.PMT.json` and `data/presence/FUNC.PMT.json` (arity,
  classification axes, implementing module and sibling set).
