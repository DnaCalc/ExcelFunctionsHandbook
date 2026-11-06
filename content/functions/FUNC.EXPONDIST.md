---
schema: efh.function-page/v1
function_id: FUNC.EXPONDIST
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

# EXPONDIST

## What it does

`EXPONDIST(x, lambda, cumulative)` models a nonnegative waiting time with rate lambda. TRUE returns the probability of a wait up to x; FALSE returns density. EXPONDIST is the compatibility name.

## Try it

`=EXPONDIST(2,0.5,TRUE)` is about `0.632120558828558`. The rate-times-time product is one. With FALSE the density is about `0.183939720585721`.

## Quirks

Negative x or a nonpositive rate gives `#NUM!`; nonnumeric arguments give `#VALUE!`. Rate has inverse-time units; it is not a scale parameter. A naive `1-exp(-lambda*x)` loses small probabilities when the exponential rounds close to one.

## How to implement

Store `t=R(lambda*x)` using the 64-bit-significand then binary64 operation. The current kernel rejects a nonfinite t and replaces a subnormal t with zero. Density is `R(lambda*EXP(-t))`. The CDF is `-expm1_spilled(-t)`, using the stored Kahan correction described in the PMT recipe: form EXP, then correct `(u-1)*t/LN(u)` when needed. Flush final subnormal results. The example uses t=1; this central identity alone does not prove global compatibility.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 9,740 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.EXPONDIST.json).

## Alternatives

[GAMMA.DIST](FUNC.GAMMA.DIST.md) with shape one is related, but its scale transformation has its own stores. [WEIBULL.DIST](FUNC.WEIBULL.DIST.md) generalises the waiting-time shape.

## Sources

- [Microsoft: EXPONDIST function](https://support.microsoft.com/en-us/excel/functions/expondist-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/discrete_dist.rs); implementation steps and the limits identified in the text.
