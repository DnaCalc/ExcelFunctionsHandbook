---
schema: efh.function-page/v1
function_id: FUNC.NORMDIST
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

# NORMDIST

## What it does

`NORMDIST(x,mean,standard_dev,cumulative)` is the compatibility normal distribution. TRUE returns the probability up to x; FALSE returns a density. Standard deviation must be positive.

## Try it

`=NORMDIST(10,10,2,TRUE)` returns `0.5`. At the mean, half the distribution lies below the input. With FALSE the density is about `0.1994711402`.

## Quirks

Nonnumeric numeric arguments give `#VALUE!`. A nonpositive standard deviation gives `#NUM!`. A density is not a point probability and can exceed one when the scale is small. The compatibility name does not imply that its numerical stages are identical in every Excel build.

## How to implement

For a general CDF first standardize using binary64 `u=(x-mean)/sigma`, then apply the standard wrapper. For the standard-normal CDF, the current measured wrapper stores `z=R(abs(x)*c)`, with binary64 `c` given by bits `0x3fe6a09e667f3bcd`. Evaluate the public ERFC body at z, halve it, and select that half for a negative argument or `1-half` for a nonnegative argument. Flush the final subnormal under the current publication rule. The ERFC upper factor remains open; identifying this wrapper does not solve that inner approximation.

For density, the current kernel instead stores `u=R(R(x-mean)/sigma)`, `sq=R(u*u)`, then evaluates `E=EXP(-sq/2)`, divides E by sigma at a stored site and finally multiplies by binary64 `0x3fd9884533d43651`. Calling a previously flushed PHI and dividing later changes the route. `R` means 64-bit-significand rounding followed by a binary64 store.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, AMD Ryzen 7 4800H with Radeon Graphics | 3,005 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.NORMDIST.json).

## Alternatives

[NORM.DIST](FUNC.NORM.DIST.md) is the modern spelling. [NORM.INV](FUNC.NORM.INV.md) and [NORM.S.INV](FUNC.NORM.S.INV.md) map probabilities back to values.

## Sources

- [Microsoft: NORMDIST function](https://support.microsoft.com/en-us/excel/functions/normdist-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/normal_log_family.rs); implementation steps and the limits identified in the text.
