---
schema: efh.function-page/v1
function_id: FUNC.RATE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
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
family: null
role_in_family: null
---

# RATE

## What it does

`RATE(nper,pmt,pv,[fv],[type],[guess])` solves the periodic rate of a fixed-payment annuity. Future value defaults to zero, type zero pays at period end, and the documented default guess is 0.1.

## Try it

`=RATE(1,-1250,1000)` is about `0.25`. With one end-period payment the equation is `1000*(1+r)-1250=0`, so r is one quarter. The output is per payment period, not automatically annual.

## Quirks

The equation may have no acceptable root or more than one root; guess and search behaviour can affect the answer. Nonconvergence gives `#NUM!` under the documented policy. A returned monthly rate times twelve is a nominal annual calculation, not a change of the function’s period units.

Microsoft describes convergence within `0.0000001` after at most twenty iterations. That is the public stopping description, not a specification of every step in the current alternative solver. Comparing the returned residual alone does not establish Excel's root-selection or stopping behavior.

## How to implement

The following is the current candidate’s algorithm, rather than an identified Excel iteration schedule. Its residual and stopping stages are distinct from PMT’s measured discount route.

### Residual

With ordinary binary64 arithmetic, at `abs(r)<1e-12` use `F(r)=fv+pv+pmt*n`. Otherwise obtain `g=POWER(1+r,n)` through the current growth helper, set `timing=1` for end payments or `1+r` for beginning payments, form `term=timing*(g-1)/r`, and evaluate `F(r)=fv+pv*g+pmt*term` in that association. Reject invalid growth or nonfinite results. This small-rate band belongs to the RATE residual; PMT’s exact-zero branch does not replace it.

### Search

1. Start with the pair `(0,F(0))` and `(guess,F(guess))`, defaulting guess to 0.1. The current domain bound is `-0.999999999`; calls at or below it need the source’s explicit boundary/error handling.
2. Accept a search residual with magnitude at most `1e-7`. If two successive residuals have opposite signs, bisect that endpoint pair for at most 64 steps, accepting the same residual threshold. After those steps the helper also accepts a bracket width at most `1e-12`.
3. Otherwise take a secant step `next=r-F(r)*(r-prev)/(F(r)-F(prev))`. If the residual difference’s magnitude is at most `1e-12`, use a finite-difference Newton slope instead: `h=max(abs(r),1)*1e-6`, sample at `max(r-h,domain_bound)` and `r+h`, and divide the residual difference by the actual sample separation. A slope of magnitude at most `1e-12` fails.
4. Permit twenty search iterations. A nonfinite next rate or one at/below the domain bound leaves that loop. If no acceptable root was found, the source searches from zero: positive rates start at `max(abs(guess),0.1)` and double; negative probes move halfway from the current rate toward minus one. Each side has a limit of 64 probes, and a found sign change uses the bisection helper. Evaluation errors still propagate.

### Refinement and remaining limits

A successful seed then gets up to sixty central-difference Newton refinements. Here the sampling width is `h=max(abs(r),1e-4)*1e-6`, distinct from the fallback width above. Stop at a stored-rate fixpoint, invalid next rate, failed residual or noncontracting step. At the latter boundary the source keeps whichever candidate has the smaller absolute residual.

For the one-period example, F(0) is minus 250 and F(0.1) minus 150. Their secant line reaches r=0.25, whose residual is zero. Longer or nearly flat problems need the remaining phases; a small currency residual alone does not guarantee a similarly small rate error.

The source retains a counterexample at `RATE(360,-1073.64,200000)`: its polished result bits `0x3f71110b20485718` remain 55 representable steps below the recorded Excel target `0x3f71110b2048574f`. Root selection, the residual’s arithmetic and Excel’s full stopping rules remain open. These candidate steps provide something concrete to implement and compare without claiming whole-function agreement.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 105 |
| 20228 | Windows, 64-bit, CPU not recorded | 256 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.RATE.json).

## Alternatives

[IRR](FUNC.IRR.md) accepts varying periodic flows; [XIRR](FUNC.XIRR.md) uses dates; [PMT](FUNC.PMT.md) computes a payment for a supplied rate.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `RATE` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-RATE-001: RATE returns finite numbers near DBL_MAX on 46 inputs where no real annuity root exists above -1

*Quirk; measured; recorded 2026-11-22.*

In Excel 16.0.20527, RATE returns large finite numbers (such as 1.765850607151516e+308) on 46
recorded witness-reference rows where future value lies outside the real annuity balance range
(where CalcFunc returns #NUM!), and differs by up to 12.57 ULP from the annuity root on
extreme-rate rows (bank2-more-RATE-1195).

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `RATE(-1.3669552943168466e-08, 0.004130753832019453, -0.9811375421215849, 0, 1, -0.22920042173301014)` | nper `-1.3669552943168466e-08` (`0xbe4d5aea8c545370`); pmt `0.004130753832019453` (`0x3f70eb68c9dc920f`); pv `-0.9811375421215849` (`0xbfef657a8f0947ec`); fv `0.0` (`0x0000000000000000`); type `1.0` (`0x3ff0000000000000`); guess `-0.22920042173301014` (`0xbfcd56707dc94dc8`) (cells) | `1.765850607151516e+308` (`0x7fef6ee528c1d989`) | — | Excel 16.0.20527.20196, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics |
| `RATE(0.1391623113305186, 10, -438.10866310194024, 8205, 0.5, 1.0000000000000002)` | nper `0.1391623113305186` (`0x3fc1d01214000b64`); pmt `10.0` (`0x4024000000000000`); pv `-438.10866310194024` (`0xc07b61bd158551d8`); fv `8205.0` (`0x40c0068000000000`); type `0.5` (`0x3fe0000000000000`); guess `1.0000000000000002` (`0x3ff0000000000001`) (cells) | `1630151639.0138218` (`0x41d84a88f5c0e275`) | `1630151639.0138187` (`0x41d84a88f5c0e268`) | Excel 16.0.20527, Windows (bitness and CPU not recorded), en-ZA |

Correct value: mpmath at 60 decimal digits solving the annuity balance equation

Basis (measured): Recorded across 65 mismatching rows in
crates/calc-func/witness-reference/RATE.tsv (46 structural number vs #NUM!, 9 #NUM! vs number,
2 no-real-root, 4 excel-quirk, 4 large-deviation) out of 4,787 full-bank RATE rows.

- Open: 65 RATE structural and numerical differences remain open (dc-j9k).

Evidence:

- DnaCalc `crates/calc-func/witness-reference/RATE.tsv` at `4fb9b728df` — 65 RATE witness-reference differences
- DnaCalc `crates/calc-func/witness-status.tsv`, RATE at `4fb9b728df` — 4,722 of 4,787 full-bank rows matched
- DnaCalc `crates/calc-func/witness-reference/RATE.tsv`, fx2/evidence/rate-overflow-answers.json $[494] at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/witness-reference/RATE.tsv`, bank2-more-RATE-1195 at `4fb9b728df` (readings)

<!-- efh:notes end -->

## Excel versions, platforms and CPUs

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.RATE.json; edits here are overwritten -->

Readings of RATE with a recorded Excel build: 361 input(s) over 2 recorded environment(s),
Excel 16.0 build(s) 20131, 20228, all on Windows; recorded CPUs: AMD Zen 2. 105 input(s) were
also read in captures that record no build (some may repeat inputs counted above); they are not
counted. No input has been read in two environments, so no difference is known and none is
ruled out. A CPU dependence is expected and untested (see below). No reading from a Mac, Excel
for the web, 32-bit Excel or an Intel CPU is recorded.

Readings of `RATE` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 20131 | Windows, bitness not recorded | AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | 105 | 2026-07-11 |
| 16.0 build 20228 | Windows, 64-bit | CPU not recorded | 256 | 2026-08-09 |

No check has read the same inputs in two environments, so the absence of a listed difference says nothing about other builds, platforms or CPUs.

Differences observed: none.

Expected, not yet tested (XS-RATE-001, cpu): Handbook evidence record EV-FIN-0015 marks 3 of
its counts for this function as CPU-scoped: the counted result depends on how the host CPU
executes x87 instructions. Every reading recorded here was taken on an AMD Zen 2 processor or
on a host whose CPU was not recorded; no reading on an Intel CPU, a Mac or Excel for the web
exists, so a CPU difference is neither observed nor ruled out.

Evidence for every row: [`content/evidence/environments/FUNC.RATE.json`](../../content/evidence/environments/FUNC.RATE.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Sources

- [Microsoft: RATE function](https://support.microsoft.com/en-us/excel/functions/rate-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/financial_time_value_family.rs); implementation steps and the limits identified in the text.
