---
schema: efh.function-page/v1
function_id: FUNC.LOGNORMDIST
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

# LOGNORMDIST

## What it does

`LOGNORMDIST(x,mean,standard_dev)` returns the cumulative lognormal probability up to positive x. Mean and standard deviation describe **ln(X)**, not X. This is the compatibility form.

## Try it

`=LOGNORMDIST(1,0,1)` returns `0.5`: ln(1) is the centre of the specified normal distribution.

## Quirks

Nonpositive standard deviation gives `#NUM!`; nonnumeric inputs give `#VALUE!`. Nonpositive x gives `#NUM!`. This old form always returns a CDF and has no density switch.

## How to implement

Take the internal runtime logarithm of x, then standardize `z=(log(x)-mean)/sigma` and evaluate the standard-normal CDF. The current code distinguishes that internal log from the public worksheet LN on retained controls, even though many public compositions agree. At x=1 and mean zero, z=0 and the CDF is one half. Keep overflow, tail and text coercion tests separate from this mathematical transformation.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 3,299 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.LOGNORMDIST.json).

## Alternatives

[LOGNORM.DIST](FUNC.LOGNORM.DIST.md) is the modern spelling. [NORM.DIST](FUNC.NORM.DIST.md) describes the normal variable in log space.

## Sources

- [Microsoft: LOGNORMDIST function](https://support.microsoft.com/en-us/excel/functions/lognormdist-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/normal_log_family.rs); implementation steps and the limits identified in the text.
