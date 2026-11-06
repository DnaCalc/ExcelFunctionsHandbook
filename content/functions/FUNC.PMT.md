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
role_in_family: >-
  The annuity equation solved for the payment, and the one the schedule functions start from:
  IPMT, PPMT and the cumulative pair all begin with PMT's quotient or its equivalent.
---

# PMT

## What it does

`PMT(rate, nper, pv, [fv], [type])` returns a fixed payment at a constant rate over equal periods. `fv` defaults to zero. Type zero (the default) pays at period end; type one pays at the beginning. Use a rate per payment period.

## Try it

`=PMT(0,4,1000)` returns `-250`: four equal outflows repay a positive principal. `=PMT(0.25,1,1000,0,0)` is about `-1250`, repaying the principal and one period’s interest.

## Quirks

Cash inflows and outflows have opposite signs. A monthly payment needs a monthly rate and a count of months. Taxes and fees are outside this formula. Exactly zero rate has a separate linear branch; a merely small rate must retain the discount computation. Zero periods and invalid growth domains need error handling.

## How to implement

The following is the current **type-zero positive-rate discount route**, with the recovered logarithm series used on `-1/3<=rate<=1/2`. The series reproduces 1,181 of 1,184 inferred logarithm values in the note; it still has exceptions inside that range. Its negative-rate and outer-domain limitations are stated below. Let `D(op)` mean ordinary binary64 round-to-nearest/even, and `R(op)` the [stored 64-bit-significand operation](../model/09-staged-arithmetic.md).

1. Reject zero periods with `#NUM!`. At exactly `r=0`, return `-(pv+fv)/n`. Otherwise reject a growth base `1+r<=0`.
2. Form `s=D(r/D(2+r))`, `s2=D(s*s)`. Start `T=D(1/23)`. For `k=10` down to `1`, assign `T=D(D(T*s2)+D(1/(2*k+1)))`. Form `a=D(2*s)` and `L=D(a+D(a*D(s2*T)))`. This is a twelve-term atanh expansion, **not** a correctly rounded log1p call.
3. Store `t=-D(n*L)`. Obtain the measured public exponential `u=EXP(t)`. If `u==1`, let `em=t`. Otherwise set `d=R(u-1)`; for `abs(t)<1`, use `em=R(R(d*t)/LN(u))`, where LN is the measured public stored result; otherwise use `em=d`. Reject `em=0` with `#NUM!` before division. This Kahan correction avoids cancellation in `exp(t)-1`.
4. On the positive-rate, positive-period discount arm (`t<0`), form `b=R(pv+fv)`, `q=R(R(b/em)+fv)`, and return `R(q*r)` for type zero. Each addition and the division is a separate store. With `fv=0` this reduces to quotient first, rate last.

Worked zero-future-value case, `r=0.25,n=1,pv=1000`: the stated reconstruction gives the following binary64 values. The stored correction moves `em` one representable step from the direct subtraction.

| Stage | Value | Bits |
|---|---:|---|
| Series logarithm `L` | `0.22314355131420974` | `0x3fcc8ff7c79a9a21` |
| `t=-n*L` | `-0.22314355131420974` | `0xbfcc8ff7c79a9a21` |
| Stored EXP `u` | `0.8` | `0x3fe999999999999a` |
| Direct difference `d` | `-0.19999999999999996` | `0xbfc9999999999998` |
| Corrected difference `em` | `-0.19999999999999998` | `0xbfc9999999999999` |
| Balance quotient `q` | `-5000` | `0xc0b3880000000000` |
| Payment | `-1250` | `0xc093880000000000` |

With the same rate, period and principal but `fv=250`, the first stored balance is `1250`. Dividing by em and then adding 250 gives `q=-6000.000000000001`; the final payment is `-1500.0000000000002`. An algebraic one-period calculation rounds differently. These are worked evaluations of the reconstruction, not additional Excel captures.

The nonzero-future-value discount fold is supported by 192 paired calls in the current source; 60 and 99 separately selected readings distinguish its two additions. This later result supersedes that part of the older note's open list. The common loan `PMT(0.05/12,360,200000)` has recorded result `-1073.6432460242781` in FN-PMT-001; replaying the stated series and stored quotient also gives that value.

The code separately handles opposite equal balances as interest-only. Negative-rate composition, rates outside the series’ chosen convergence range and remaining last-bit cases are not solved by this recipe. The note retains an in-range exception at rate `0.419`; a global reduction boundary has not been identified. The series range is an implementation choice, not a measured global Excel boundary. The current type-one coefficient has its own measured stores; it must not be folded into the type-zero recipe by algebra alone. The old annuity write-up’s initial “blocked” section is superseded by its update and later code.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 47 |
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 289,642 |
| 20228 | Windows, 64-bit, CPU not recorded | 2,076 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.PMT.json).

## Alternatives

[IPMT](FUNC.IPMT.md) and [PPMT](FUNC.PPMT.md) split interest and principal; [PV](FUNC.PV.md) and [FV](FUNC.FV.md) solve for a balance; [RATE](FUNC.RATE.md) solves for the periodic rate.

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

## Sources

- [Microsoft: PMT function](https://support.microsoft.com/en-us/excel/functions/pmt-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/financial_time_value_family.rs); implementation steps and the limits identified in the text.
- [Annuity research, including the superseding update](../notes/evidence/2026-11-04/annuity-EVIDENCE.txt).
- [Logarithm and cancellation correction](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/excel_numeric/mod.rs).
