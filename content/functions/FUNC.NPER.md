---
schema: efh.function-page/v1
function_id: FUNC.NPER
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
  - Sources
family: null
role_in_family: null
---

# NPER

## What it does

`NPER(rate,pmt,pv,[fv],[type])` solves for the number of equal payment periods at a fixed periodic rate. Future value defaults to zero; type zero pays at period end and type one at the beginning. The result may be fractional.

## Try it

`=NPER(0,-250,1000)` returns `4`. At zero interest, four outflows of 250 repay the positive principal. A nonzero rate can require more periods for the same payment.

## Quirks

Cash-flow signs matter: a positive principal and negative payments describe repayment. Exactly zero rate is a separate branch. A tiny nonzero rate that makes the stored `1+rate` round to one reaches a logarithmic denominator of zero, giving `#DIV/0!` in the recorded route; it must not be replaced by a small-rate approximation.

## How to implement

Use `R(op)` for 64-bit-significand rounding followed by a binary64 store. At zero rate, return `R(-R(fv+pv)/pmt)` and reject a zero payment. Otherwise set `tf=1` for type zero or `R(1+r)` for type one, then `a=R(tf*pmt)`, `num=R(a-R(fv*r))`, `den=R(a+R(pv*r))`, and `ratio=R(num/den)`. Reject a nonpositive or nonfinite ratio and a nonpositive stored `base=R(1+r)`. Return `R(LN(ratio)/LN(base))` using the measured worksheet logarithm; a zero denominator log gives `#DIV/0!`.

The zero-rate example computes `-1000/-250=4`. This recipe preserves the recorded stores; it does not round the period count to an integer or establish all workbook input routes.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 1,291 |
| 20430 | Windows, 64-bit, CPU not recorded | 1,210 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.NPER.json).

## Alternatives

[PMT](FUNC.PMT.md) solves the payment; [RATE](https://handbook.dnacalc.com/functions/RATE.html) solves the rate; [PDURATION](https://handbook.dnacalc.com/functions/PDURATION.html) describes growth without intervening payments.

## Sources

- [Microsoft: NPER function](https://support.microsoft.com/en-us/excel/functions/nper-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/financial_time_value_family.rs); implementation steps and the limits identified in the text.
