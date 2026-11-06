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

The trial count is truncated toward zero. The current helper then requires `n>=0`, `0<p<1` and `0<alpha<1`; probabilities at either endpoint are rejected with `#NUM!`. Nonnumeric arguments give `#VALUE!`. Unlike BINOM.DIST, this inverse does not admit degenerate endpoint probabilities.

At an exact cumulative boundary, “at least” means `>=`, not `>`. A count can change because the accumulated floating-point CDF falls on a different side of `alpha`, even when two mass formulas agree to many decimal places. The returned result is an integer count, not an interpolated fraction of a trial.

## How to implement

The current candidate is a sequential search:

1. Truncate and validate the trial count and the two open-interval probabilities.
2. Form the stored binary64 complement `q=1-p`; start `sum=0` and `k=0`.
3. While `k<n`, obtain the current shared binomial mass at `k`, add it to `sum`, and return `k` if `sum>=alpha`.
4. Otherwise increment `k`. If no earlier count reaches the criterion, return `n`. For admitted `n=0`, this final branch returns zero.

For four trials with `p=0.5`, the mathematical masses are `1,4,6,4,1` divided by sixteen. The cumulative values at counts zero, one and two are `1/16`, `5/16` and `11/16`; `alpha=0.6` first qualifies at two. At criterion `5/16`, the inclusive mathematical comparison first qualifies at one.

A large-`n` implementation can bracket a count and binary-search a stable CDF, but that is a different algorithm. It needs checks around adjacent counts and exact CDF boundaries; stopping when the numerical probability is merely close to `alpha` can return the wrong integer. The current sequential routine retains recorded mismatches, so neither this search outline nor a generic inverse library is a solved whole-domain Excel quantile recipe.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 2,152 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.BINOM.INV.json).

## Alternatives

[BINOM.DIST](FUNC.BINOM.DIST.md) maps a count to a probability; [CRITBINOM](FUNC.CRITBINOM.md) supplies the other spelling.

## Sources

- [Microsoft: BINOM.INV function](https://support.microsoft.com/en-us/excel/functions/binom-inv-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/discrete_dist.rs); implementation steps and the limits identified in the text.
