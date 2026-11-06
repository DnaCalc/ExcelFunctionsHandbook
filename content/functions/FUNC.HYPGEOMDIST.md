---
schema: efh.function-page/v1
function_id: FUNC.HYPGEOMDIST
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

# HYPGEOMDIST

## What it does

`HYPGEOMDIST(sample_s, number_sample, population_s, number_pop)` models sampling **without replacement**. It counts successes in a fixed sample drawn from a finite population.This compatibility form returns a mass only.

## Try it

`=HYPGEOMDIST(1,2,3,5)` is about `0.6`. Drawing two objects from five, three marked successful, yields one success in `choose(3,1)*choose(2,1)=6` of `choose(5,2)=10` possible samples.

## Quirks

The four arguments are counts and are truncated toward zero, with the current code rejecting original negative signs before truncation. The feasible success interval is `max(0,n-(N-K))..min(n,K)`, where `n` is sample size, `K` population successes and `N` population size.

Microsoft’s documented guard gives `#NUM!` for invalid sizes and success counts; nonnumeric arguments give `#VALUE!`. The current shared mass helper instead supplies zero for an unsupported success count and handles an empty sample as a degenerate mass. The modern HYPGEOM.DIST has retained build 20430 controls for such departures, but those calls do not measure this legacy spelling. Keep that remaining scope explicit.

## How to implement

The mathematical probability is `choose(K,k)*choose(N-K,n-k)/choose(N,n)`. For one success in a sample of two from five objects with three successes, the numerator is `3*2=6` and the denominator ten, giving `0.6`.

The current wrapper calls the same mass helper as [HYPGEOM.DIST with cumulative FALSE](FUNC.HYPGEOM.DIST.md#how-to-implement). It rejects original negative signs, truncates the counts, checks `n<=N` and `K<=N`, then applies the mass support and degenerate-sample rules. For nondegenerate supported counts it forms `p=n/N` and `q=(N-n)/N`, obtains three binomial masses, and returns their product/ratio in the stated order. There is no cumulative switch or cumulative endpoint shortcut on this spelling.

Ratios of integer combinations or a recurrence give alternative mathematical algorithms. They avoid separately overflowing factorials, but do not identify Excel’s numerical leaves or error order. The generated fingerprint report still has no adopted complete hypergeometric algorithm; legacy-spelling boundaries and remaining numerical differences stay open.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 46 |
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 3 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.HYPGEOMDIST.json).

## Alternatives

[BINOM.DIST](FUNC.BINOM.DIST.md) models independent trials with replacement; [COMBIN](FUNC.COMBIN.md) counts combinations; [HYPGEOM.DIST](FUNC.HYPGEOM.DIST.md) exposes the cumulative switch.

## Sources

- [Microsoft: HYPGEOMDIST function](https://support.microsoft.com/en-us/excel/functions/hypgeomdist-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/discrete_dist.rs); implementation steps and the limits identified in the text.
