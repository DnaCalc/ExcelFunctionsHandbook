---
schema: efh.function-page/v1
function_id: FUNC.IPMT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-FIN-0001
  - EV-FIN-0002
  - EV-FIN-0005
  - EV-FIN-0017
open_problems: []
references: []
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
role_in_family: "The interest part of a payment; its zero-fv end-timing route reuses the stored annuity quotient and corrected exponential difference."
---

# IPMT

## What it does

`IPMT(rate,per,nper,pv,[fv],[type])` returns the interest part of a constant annuity payment in one chosen period. The period comes before the total count and starts at one. Future value and timing default to zero; type one means payment at the beginning.

Use a rate per payment period. A positive loan principal normally produces negative interest payments under Excel’s cash-flow sign convention.

## Try it

`=IPMT(0.25,2,2,1000)` is about `-138.8888889`. On this two-payment loan, the interest in the second period is charged on the balance remaining after the first payment. `=IPMT(0.25,1,2,1000,0,1)` returns zero because a beginning payment at the initial instant has no elapsed-period interest.

## Quirks

The documented period range is one through nper. The current helper returns `#NUM!` outside that range and distinguishes exactly zero rate from a tiny nonzero rate. Type is truncated by the wrapper before its zero/nonzero timing choice.

The newer source checks a valid beginning first period before its PMT rate guard, so it can return zero even at a rate PMT itself rejects. PPMT calls PMT first and can therefore have a different error path on that same otherwise invalid annuity.

Mathematically interest plus principal is the payment. Recorded Excel results can differ in their last bits from either subtraction identity: the notes retain a twelve-period example. Money amounts are not automatically rounded to cents by these functions.

## How to implement

Use [D and R](../model/09-staged-arithmetic.md) for ordinary binary64 storage and a stored 64-bit-significand operation. The current route keeps argument validation, timing boundaries and period arithmetic separate.

1. Validate finite numeric arguments and require `1<=per<=nper`. For beginning timing at per=1, return zero before PMT’s rate-domain and reciprocal-pole checks. At exactly zero rate, other valid periods also return zero.
2. For remaining calls reject a nonpositive growth base and obtain PMT under its own guards. A tiny nonzero rate retains the computation; do not use an epsilon band.
3. For **end timing, fv=0 and a valid rate in the series range**, obtain L and the total-period corrected exponential difference `em_n` from the [PMT recipe](FUNC.PMT.md#how-to-implement). Store `q=R(pv/em_n)`; do not recover q by dividing PMT’s already rounded answer by r.
4. Store `m=D(D(nper-per)+1)`, then `t=-D(m*L)`. Obtain the corrected difference `em_m` by the same spilled Kahan rule used for PMT.
5. Return `-R(R(q*em_m)*r)`. Principal instead uses the separately stored EXP(t) result; `1+em_m` is not a substitute for that exponential.

Worked second period of a two-period loan at r=0.25, pv=1000: the stated reconstruction has L `0.22314355131420974`, total em_n `-0.36000000000000004` and q `-2777.7777777777774`. One period remains, with em_m `-0.19999999999999998`; the stored products give interest `-138.88888888888886`. This trace evaluates the reconstruction, rather than adding an Excel capture.

For nonzero fv, the current end helper still forms `v_n=D(1+em_n)`, `N=D(pv+D(fv*v_n))` and `q=R(N/em_n)`. Its interest expression is `-R(R(R(q*em_m)-R(fv*EXP(t)))*r)`. This helper does **not** yet use the later nonzero-fv fold recovered for PMT itself. The note retains only 18 of 27 matching nonzero-fv interest controls; that numerator remains open here.

The helper declines rates outside its series range, opposite nonzero balances and zero/nonfinite em and nonfinite quotients. Beginning timing and other declined cases retain a separate backward-balance candidate with internal log1p/exponential leaves. They are not established by the zero-fv end-timing recipe. The source, first-period guard and unresolved fallback scope supersede the older page’s blanket recurrence description. The interactive calculator remains pinned separately.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 357 |
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 31 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IPMT.json).

## Alternatives

[PPMT](FUNC.PPMT.md) returns principal and [PMT](FUNC.PMT.md) the full payment. [CUMIPMT](FUNC.CUMIPMT.md) totals interest over a period interval. [ISPMT](FUNC.ISPMT.md) uses a different level-principal model and a different period convention.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `IPMT` that an implementer matching Excel, or a reader explaining an
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

<!-- efh:notes end -->

## Excel versions, platforms and CPUs

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.IPMT.json; edits here are overwritten -->

Readings of IPMT with a recorded Excel build: 388 input(s) over 2 recorded environment(s),
Excel 16.0 build(s) 19929, 20131, all on Windows; recorded CPUs: AMD Zen 2. 31 input(s) were
also read in captures that record no build (some may repeat inputs counted above); they are not
counted. No input has been read in two environments, so no difference is known and none is
ruled out. A CPU dependence is expected and untested (see below). No reading from a Mac, Excel
for the web, 32-bit Excel or an Intel CPU is recorded.

Readings of `IPMT` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 357 | date not recorded |
| 16.0 build 20131 | Windows, bitness not recorded | AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | 31 | 2026-07-23 |

No check has read the same inputs in two environments, so the absence of a listed difference says nothing about other builds, platforms or CPUs.

Differences observed: none.

Expected, not yet tested (XS-IPMT-001, cpu): Handbook evidence record EV-FIN-0005 marks 7 of
its counts for this function as CPU-scoped: the counted result depends on how the host CPU
executes x87 instructions. Every reading recorded here was taken on an AMD Zen 2 processor or
on a host whose CPU was not recorded; no reading on an Intel CPU, a Mac or Excel for the web
exists, so a CPU difference is neither observed nor ruled out.

Evidence for every row: [`content/evidence/environments/FUNC.IPMT.json`](../../content/evidence/environments/FUNC.IPMT.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Sources

- [Microsoft: IPMT function](https://support.microsoft.com/en-us/excel/functions/ipmt-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [R31 financial implementation](https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/crates/calc-func/src/ported/financial_time_value_family.rs); `end_payment_quotient`, `end_payment_parts`, `ipmt` and `ppmt`, including distinct guard and fallback paths.
- [Recorded stage controls](https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/crates/calc-func/tests/f3_financial.rs); payment-part and beginning-period controls. The notes below retain captured examples and their environments.
