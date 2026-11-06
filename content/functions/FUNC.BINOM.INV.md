---
schema: efh.function-page/v1
function_id: FUNC.BINOM.INV
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

# BINOM.INV

## What it does

`BINOM.INV(trials, probability_s, alpha)` returns the smallest success count whose cumulative binomial probability is at least alpha. Trials are independent and share a success probability.

## Try it

`=BINOM.INV(4,0.5,0.6)` returns `2`. The cumulative probabilities through zero, one and two successes are `1/16`, `5/16` and `11/16`; two is the first count to reach 0.6.

## Quirks

The trial count is truncated toward zero. Negative trials, or success probability and alpha outside their documented open interval zero to one, give `#NUM!`. Nonnumeric arguments give `#VALUE!`. At an exact CDF boundary, the “at least” comparison matters.

## How to implement

Validate arguments, then accumulate binomial masses from k=0 upward until the cumulative value is `>=alpha`. Return that k; return n if no earlier count qualifies. For the four-trial example the masses are `1,4,6,4,1` divided by sixteen. A faster large-n route can bracket or search counts using a stable CDF, but needs integer endpoint and rounding controls. The current sequential routine still has recorded inverse mismatches; this is not a solved whole-domain quantile recipe.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit-bit, CPU not recorded | 2,152 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.BINOM.INV.json).

## Alternatives

[BINOM.DIST](FUNC.BINOM.DIST.md) maps a count to a probability; [CRITBINOM](https://handbook.dnacalc.com/functions/CRITBINOM.html) supplies the other spelling.

## Sources

- [Microsoft: BINOM.INV function](https://support.microsoft.com/en-us/excel/functions/binom-inv-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/discrete_dist.rs); implementation steps and the limits identified in the text.
