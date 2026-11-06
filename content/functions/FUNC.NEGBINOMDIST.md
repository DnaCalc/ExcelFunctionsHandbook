---
schema: efh.function-page/v1
function_id: FUNC.NEGBINOMDIST
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

# NEGBINOMDIST

## What it does

`NEGBINOMDIST(number_f,number_s,probability_s)` returns the probability of exactly `number_f` failures before the specified number of successes. The final trial is a success. This compatibility form returns a mass, with no cumulative switch.

## Try it

`=NEGBINOMDIST(2,2,0.5)` is about `0.1875`. The final success occupies the fourth trial; the first three positions contain one success in three possible arrangements, each four-trial sequence having probability `1/16`.

## Quirks

Failure and success counts are truncated. Negative failures, fewer than one target success, or probability outside zero through one give `#NUM!`; nonnumeric inputs give `#VALUE!`. Count failures rather than total trials. Degenerate p=0 or p=1 cases need explicit handling rather than logarithms of zero.

## How to implement

For admitted failures f, target successes s and probability p, the mathematical mass is `choose(f+s-1,f)*p^s*(1-p)^f`. Evaluate a stable combinatorial ratio or log form to avoid overflowing factorials; use a recurrence when accumulating many adjacent masses. The current helper composes shared discrete-distribution numerical routines and still has recorded discrepancies. The small example is `choose(3,2)/16=3/16`; it explains the count convention without establishing all numerical stages.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 138 |
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 1,804 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.NEGBINOMDIST.json).

## Alternatives

[NEGBINOM.DIST](FUNC.NEGBINOM.DIST.md) adds the cumulative option; [BINOM.DIST](FUNC.BINOM.DIST.md) fixes the trial count instead of the success threshold.

## Sources

- [Microsoft: NEGBINOMDIST function](https://support.microsoft.com/en-us/excel/functions/negbinomdist-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/discrete_dist.rs); implementation steps and the limits identified in the text.
