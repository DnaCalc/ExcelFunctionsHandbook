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

Failure and success counts are truncated toward zero; the first is failures, not total trials. Negative admitted failures or fewer than one target success gives `#NUM!`. Nonnumeric arguments give `#VALUE!`.

Microsoft’s written probability guard excludes values outside zero through one. The current helper uses the stricter open interval `0<p<1`, rejecting both endpoints with `#NUM!`. That is a distinction between the written baseline and the current implementation; no new Excel endpoint reading is asserted here. Counts and endpoint guards must be tested separately from the mass formula.

## How to implement

The mathematical mass for `f` failures before success number `s` is

```text
choose(f+s-1,f) * p^s * (1-p)^f
```

The last trial is fixed as a success. Choose the positions of the other `s-1` successes among the preceding `f+s-1` trials. Thus for `f=2,s=2,p=0.5`, the possible success/failure patterns are `SFFS`, `FSFS`, `FFSS`; each has probability `1/16`, giving `3/16` in total.

The current code composes binomial machinery instead of constructing factorials:

1. Truncate and validate `f,s,p`; retain the endpoint guard above.
2. For `f=0`, return the public exponential of the ordinary binary64 product `s*LN(p)`. Do not infer the stored product stage of BINOM.DIST’s all-success route for this separate call site.
3. Otherwise form `n=s+f`, `q=1-p` and the shared binomial mass `b(s;n,p,q)`. Multiply it by `s/n` with the current binary64 division and multiplication.
4. Apply the finite-result policy. The helper still has recorded numerical and domain discrepancies in the problem report.

For mathematical summation, adjacent masses satisfy `mass(f+1)=mass(f)*(f+s)/(f+1)*(1-p)`. That recurrence can avoid huge factorials, but its rounding and underflow differ from the current binomial composition. The mass-only legacy function has no cumulative argument; use NEGBINOM.DIST when the probability of at most a failure count is required.

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
- [Remaining numerical and domain rows](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/docs/generated/problem-functions.md).
