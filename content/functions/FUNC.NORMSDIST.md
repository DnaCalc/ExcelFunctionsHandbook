---
schema: efh.function-page/v1
function_id: FUNC.NORMSDIST
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

# NORMSDIST

## What it does

`NORMSDIST(z)` is the compatibility standard-normal **cumulative** probability up to z. There is no density switch; mean is zero and standard deviation one.

## Try it

`=NORMSDIST(0)` returns `0.5`, half the symmetric distribution. `=NORMSDIST(1)` is about `0.8413447461`.

## Quirks

Nonnumeric numeric arguments give `#VALUE!`. NORMSDIST is not the standard-normal density; use the modern function’s FALSE switch or PHI for that. The compatibility name does not imply that its numerical stages are identical in every Excel build.

## How to implement

For the standard-normal CDF, the current measured wrapper stores `z=R(abs(x)*c)`, with binary64 `c` given by bits `0x3fe6a09e667f3bcd`. Evaluate the public ERFC body at z, halve it, and select that half for a negative argument or `1-half` for a nonnegative argument. Flush the final subnormal under the current publication rule. The ERFC upper factor remains open; identifying this wrapper does not solve that inner approximation. At zero the wrapper returns a half directly, avoiding an unnecessary transcendental call. `R` means 64-bit-significand rounding followed by a binary64 store.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 3 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.NORMSDIST.json).

## Alternatives

[NORM.S.DIST](FUNC.NORM.S.DIST.md) is the modern spelling. [NORM.INV](FUNC.NORM.INV.md) and [NORM.S.INV](FUNC.NORM.S.INV.md) map probabilities back to values.

## Sources

- [Microsoft: NORMSDIST function](https://support.microsoft.com/en-us/excel/functions/normsdist-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/normal_log_family.rs); implementation steps and the limits identified in the text.
