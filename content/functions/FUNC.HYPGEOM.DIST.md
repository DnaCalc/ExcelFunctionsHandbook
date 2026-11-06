---
schema: efh.function-page/v1
function_id: FUNC.HYPGEOM.DIST
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

# HYPGEOM.DIST

## What it does

`HYPGEOM.DIST(sample_s, number_sample, population_s, number_pop, cumulative)` models sampling **without replacement**. It counts successes in a fixed sample drawn from a finite population. FALSE returns exactly that many successes; TRUE returns at most that many.

## Try it

`=HYPGEOM.DIST(1,2,3,5,FALSE)` is about `0.6`. Drawing two objects from five, three marked successful, yields one success in `choose(3,1)*choose(2,1)=6` of `choose(5,2)=10` possible samples.

## Quirks

Count arguments are truncated. Impossible success counts and invalid sample/population sizes give `#NUM!`; nonnumeric arguments give `#VALUE!`. The feasible success interval is `max(0,n-(N-K))..min(n,K)`, not merely zero through the sample size.

## How to implement

Validate all four truncated counts and compute the support bounds. Mathematically a mass is `choose(K,k)*choose(N-K,n-k)/choose(N,n)`; a stable implementation evaluates ratios or log terms instead of separately overflowing combinations. A cumulative form sums supported masses through k with an explicit tail choice. The current mass routine uses a ratio of binomial-mass constructions, so it inherits their numerical leaves and remaining differences. The six-out-of-ten example explains the probability without identifying that whole floating-point program.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 3,653 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.HYPGEOM.DIST.json).

## Alternatives

[BINOM.DIST](FUNC.BINOM.DIST.md) models independent trials with replacement; [COMBIN](FUNC.COMBIN.md) counts combinations; [HYPGEOM.DIST](FUNC.HYPGEOM.DIST.md) exposes the cumulative switch.

## Sources

- [Microsoft: HYPGEOM.DIST function](https://support.microsoft.com/en-us/excel/functions/hypgeom-dist-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/discrete_dist.rs); implementation steps and the limits identified in the text.
