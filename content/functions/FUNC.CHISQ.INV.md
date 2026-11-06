---
schema: efh.function-page/v1
function_id: FUNC.CHISQ.INV
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

# CHISQ.INV

## What it does

`CHISQ.INV(probability, deg_freedom)` finds the chi-square value whose left cumulative probability is the requested value.

## Try it

`=CHISQ.INV(0.5,2)` is about `1.38629436111989`. At two degrees of freedom, a half right tail solves `exp(-x/2)=0.5`; the same point also has a half left CDF.

## Quirks

Degrees of freedom are truncated. Invalid probabilities or degree counts give `#NUM!`; nonnumeric arguments give `#VALUE!`. The current right-tail inverse requires `0<p<1`, even though the old documentation’s range wording can appear to include endpoints. The current left inverse admits zero as zero and rejects probability one as `#NUM!`.

## How to implement

The current kernel calls a gamma inverse with shape `df/2` and scale two. Validate probability endpoints before search, expand an upper bracket as needed and use a monotone inverse routine. Worked at df=2, `x=-2*ln(0.5)` gives about `1.38629436111989`. Inverse accuracy depends on the forward routine and on its stopping rule; the family still has recorded last-bit differences.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 40 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.CHISQ.INV.json).

## Alternatives

[CHISQ.DIST](FUNC.CHISQ.DIST.md) supplies the forward probability; [CHISQ.TEST](https://handbook.dnacalc.com/functions/CHISQ.TEST.html) returns a test p-value directly.

## Sources

- [Microsoft: CHISQ.INV function](https://support.microsoft.com/en-us/excel/functions/chisq-inv-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/chi_f_t_family.rs); implementation steps and the limits identified in the text.
