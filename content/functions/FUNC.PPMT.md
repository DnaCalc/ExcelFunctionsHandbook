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
  - What it does
  - Try it
  - Quirks
  - How to implement
  - Version differences
  - Alternatives
  - Notes
  - Excel versions, platforms and CPUs
  - Sources
family: financial_time_value_family
role_in_family: "The principal part; its end-timing route uses a stored quotient, discount exponential and rate instead of subtracting interest."
---

# PPMT

## What it does

`PPMT(rate,per,nper,pv,[fv],[type])` returns the principal part of one payment in a fixed-rate, equal-payment annuity. Period per starts at one and precedes nper in the argument list. Omitted fv and type are zero; type one pays at the beginning.

## Try it

`=PPMT(0.25,2,2,1000)` is about `-555.5555556`, the final principal repayment on a two-period loan. The first principal repayment is about `-444.4444444`, so the mathematical pair repays the original thousand.

`=PPMT(0,2,4,1000)` returns `-250`; at zero rate all four equal payments are principal.

## Quirks

Use a periodic rate and a matching period count. Positive principal and negative repayments describe a loan. Per must lie between one and nper; the current helper checks it through IPMT after obtaining PMT.

PPMT’s current first operation is a PMT call. It can therefore return a PMT domain error where IPMT’s beginning-first-period guard returns zero. At a valid beginning first period the whole payment is principal.

The mathematical identity `PPMT=PMT-IPMT` does not reproduce all recorded Excel bits. FN-PPMT-001 retains twelve loan periods where only two such binary64 subtractions equal the recorded principal. Summing principal over a schedule and reproducing CUMPRINC are separate numerical tasks.

## How to implement

For the measured **zero-fv, end-timing** route, reuse L, total-period em_n and the stored q from the [IPMT recipe](FUNC.IPMT.md#how-to-implement). Let R denote its stored arithmetic operation.

1. Obtain PMT and perform the IPMT guard path, including period validation. Exactly zero rate uses the linear principal result.
2. For the eligible nonzero-rate helper, form the binary64 remaining count `m=(nper-per)+1` and argument `t=-(m*L)`, with each named operation stored.
3. Obtain the stored public exponential `u=EXP(t)`. Do not replace it by `1+em_m`: the note’s 74 principal controls distinguish those values.
4. At fv=0 return `R(R(q*u)*r)`. With a nonzero fv, the current candidate is `R(R(R(q-fv)*u)*r)`, but q’s nonzero-fv numerator retains the limitation described on IPMT.

For the second-period example, the reconstruction has q `-2777.7777777777774`, u `0.8` and principal `-555.5555555555555`. Its PMT is `-694.4444444444443` and interest `-138.88888888888886`; ordinary binary64 subtraction of those two produces `-555.5555555555554` instead. These are worked reconstruction values, not a new Excel run.

The source retains 138 of 138 zero-fv principal controls and 25 of 27 nonzero-fv controls for its end route. Beginning timing and helper-declined cases use a separate backward-discount candidate, rather than simply subtracting interest. Its current form discounts `payment-carried_future_interest` over the periods remaining, with a beginning-timing adjustment to that carried interest.

The logarithm’s exceptions, nonzero-fv numerator, beginning-timing body and other recorded residuals remain open. Reusing PMT’s newer balance fold without separately validating these period helpers would transfer a result outside its established scope. The separately pinned calculator can predate the cited source changes.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 396 |
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 7 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.PPMT.json).

## Alternatives

[IPMT](FUNC.IPMT.md) returns interest and [PMT](FUNC.PMT.md) the entire payment. [CUMPRINC](FUNC.CUMPRINC.md) totals principal over an interval, with a separately recorded route; [ISPMT](FUNC.ISPMT.md) belongs to a level-principal schedule.

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

## Sources

- [Microsoft: PPMT function](https://support.microsoft.com/en-us/excel/functions/ppmt-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [R31 financial implementation](https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/crates/calc-func/src/ported/financial_time_value_family.rs); `end_payment_quotient`, `end_payment_parts`, `ipmt` and `ppmt`, including distinct guard and fallback paths.
- [Recorded stage controls](https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/crates/calc-func/tests/f3_financial.rs); payment-part and beginning-period controls. The notes below retain captured examples and their environments.
