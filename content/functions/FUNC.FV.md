---
schema: efh.function-page/v1
function_id: FUNC.FV
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-FIN-0001
  - EV-FIN-0002
  - EV-FIN-0005
  - EV-FIN-0012
  - EV-FIN-0014
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
  - Excel versions, platforms and CPUs
  - Sources
family: financial_time_value_family
role_in_family: >-
  The annuity equation solved for the terminal balance; the family's forward direction, and the
  member whose evidence record shows it does not share PMT's kernel.
---

# FV

## What it does

`FV(rate,nper,pmt,[pv],[type])` returns the final balance of a constant-rate investment or loan with equal periodic payments. Present value pv and type default to zero. Type zero pays at period end; type one at the beginning.

Use a rate per payment period and a matching number of periods. Cash paid out is negative and cash received positive. For nonzero rate the mathematical balance equation is `pv*(1+r)^n+pmt*(1+r*type)*((1+r)^n-1)/r+fv=0`.

## Try it

`=FV(0.25,2,-100,-1000,0)` returns about `1787.5`. The starting thousand grows to 1562.5. The first hundred earns one period of interest and the second earns none, adding 225.

`=FV(0.25,2,-100,-1000,1)` returns about `1843.75`: moving both payments one period earlier changes their combined accumulation to 281.25. At zero rate, `=FV(0,4,-250)` returns `1000`.

## Quirks

A zero payment and an omitted payment position are useful for lump-sum growth; supply pv in that case. For example, `FV(0.25,2,,-1000)` describes the starting balance alone.

The current wrapper truncates type, choosing ending timing for zero and beginning timing for other truncated values. Microsoft defines only zero and one. Treat the broader code rule as an implementation fact rather than a documented promise.

The newer cited helper distinguishes exactly zero rate from a tiny nonzero rate. A stored growth factor can round to one and cancel the nonzero-rate annuity numerator. FV therefore does not inherit PMT’s cancellation-corrected discount body. The current POWER route also permits some nonpositive-base cases with integer periods, whereas PMT has a separate domain guard.

The recipe below cites a later source revision than the separately pinned interactive calculator. A preview result can still follow an earlier route at the changed boundaries.

## How to implement

Let `D` mean ordinary nearest/even binary64 storage and [R](../model/09-staged-arithmetic.md) mean a 64-bit-significand operation followed by a binary64 store. The current R31 composition is:

1. Validate finite r, n, payment and pv. Read timing and apply the optional defaults.
2. Form `base=R(1+r)`, then obtain `g=POWER(base,n)` through the public worksheet POWER kernel. Its integer and fractional paths remain separate; this is not an unconditional EXP/LN replacement.
3. If r is exactly zero, set `term=n`. Otherwise form `difference=D(g-1)`, store `end_term=R(difference/r)`, then store `term=R(end_term*timing_factor)`. The current timing factor is one for ending payments and `D(1+r)` for beginning payments. Division comes before timing multiplication.
4. Separately store `present=R(pv*g)` and `payments=R(pmt*term)`.
5. Store `combined=R(present+payments)` and negate it. Apply the finite-result and worksheet number-publication rules.

For the ending-payment example, base is 1.25, g is 1.5625, difference 0.5625 and term 2.25. The two stored components are minus 1562.5 and minus 225, giving positive 1787.5 after negation. Beginning timing multiplies term by 1.25 to obtain 2.8125.

A boundary reconstruction is `r=2^-53,n=1,pmt=-1,pv=0,type=0`: the stored base and growth are one, difference is zero and the reconstructed result is zero. At exactly r=0 the linear term is one instead. The cited test source retains this cancellation control; a stable `expm1(n*log1p(r))/r` implementation would have a different result. Do not smooth away the measured branch merely because the mathematical one-period value is one.

The source retains separate unit controls for the growth, annuity quotient, timing product, two balance products and final addition. Those constrain stages independently of their producers. Complete POWER behaviour, other input routes and environments, and remaining reported FV mismatches still need their own evidence; the stage recipe is not a whole-function equality claim.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 789 |
| 20228 | Windows, 64-bit, CPU not recorded | 512 |
| 20430 | Windows, 64-bit, CPU not recorded | 1,230 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.FV.json).

## Alternatives

[PV](FUNC.PV.md), [PMT](FUNC.PMT.md), [NPER](FUNC.NPER.md) and [RATE](FUNC.RATE.md) solve other variables of the annuity equation. [FVSCHEDULE](FUNC.FVSCHEDULE.md) applies varying rates to a principal without this payment stream; [NPV](FUNC.NPV.md) discounts varying equally spaced payments.

## Excel versions, platforms and CPUs

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.FV.json; edits here are overwritten -->

Readings of FV with a recorded Excel build: 2,531 input(s) over 3 recorded environment(s),
Excel 16.0 build(s) 20131, 20228, 20430, all on Windows; recorded CPUs: AMD Zen 2. 789 input(s)
were also read in captures that record no build (some may repeat inputs counted above); they
are not counted. No input has been read in two environments, so no difference is known and none
is ruled out. A CPU dependence is expected and untested (see below). No reading from a Mac,
Excel for the web, 32-bit Excel or an Intel CPU is recorded.

Readings of `FV` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 20131 | Windows, bitness not recorded | AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | 789 | 2026-07-11 to 2026-07-23 |
| 16.0 build 20228 | Windows, 64-bit | CPU not recorded | 512 | 2026-08-09 |
| 16.0 build 20430 | Windows, 64-bit | CPU not recorded | 1,230 | 2026-09-29 |

No check has read the same inputs in two environments, so the absence of a listed difference says nothing about other builds, platforms or CPUs.

Differences observed: none.

Expected, not yet tested (XS-FV-001, cpu): Handbook evidence record EV-FIN-0005 marks 7 of its
counts for this function as CPU-scoped: the counted result depends on how the host CPU executes
x87 instructions. Every reading recorded here was taken on an AMD Zen 2 processor or on a host
whose CPU was not recorded; no reading on an Intel CPU, a Mac or Excel for the web exists, so a
CPU difference is neither observed nor ruled out.

Evidence for every row: [`content/evidence/environments/FUNC.FV.json`](../../content/evidence/environments/FUNC.FV.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Sources

- [Microsoft: FV function](https://support.microsoft.com/en-us/excel/functions/fv-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [R31 financial composition](https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/crates/calc-func/src/ported/financial_time_value_family.rs); `fv`, stored growth base, quotient, timing and balance operations.
- [Retained public stage controls](https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/crates/calc-func/tests/f3_financial.rs); FV unit controls, exact-zero distinction and base-store ties. These are retained source controls, not fresh Handbook Excel runs.
- [Current wrapper timing and defaults](https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/crates/calc-func/src/catalogue/financial.rs).
- [Remaining numerical failures](https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/docs/generated/problem-functions.md); counts retain their source scopes and do not certify this guide.
