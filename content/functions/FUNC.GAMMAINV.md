---
schema: efh.function-page/v1
function_id: FUNC.GAMMAINV
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

# GAMMAINV

## What it does

`GAMMAINV(probability, alpha, beta)` inverts the gamma cumulative distribution. Alpha is a positive shape and beta a positive **scale**. This is the compatibility name.

## Try it

`=GAMMAINV(0.5,1,2)` is about `1.38629436111989`: the median of the shape-one distribution with scale two is `-2*ln(0.5)`.

## Quirks

Nonpositive shapes/scales give `#NUM!`; nonnumeric numeric arguments give `#VALUE!`. Invalid probabilities and a finite inverse at the upper endpoint need explicit error policy; the current inverse rejects probability one.

## How to implement

Use the shared gamma inverse with shape alpha and scale beta, preserving its endpoint rules and bracket/stopping policy. A mathematical design can invert `P(alpha,z)=p` in a nonnegative bracket and multiply z by beta, but a different scale order may change last bits. The numerical outline is an implementation route, not an identification of every Excel approximation or return-rounding stage. The current problem-functions report retains unresolved distribution mismatches.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 82 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.GAMMAINV.json).

## Alternatives

[GAMMA.INV](FUNC.GAMMA.INV.md) is the current spelling; [EXPON.DIST](FUNC.EXPON.DIST.md) handles the shape-one waiting time; [CHISQ.DIST](FUNC.CHISQ.DIST.md) is a shape/scale specialization.

## Sources

- [Microsoft: GAMMAINV function](https://support.microsoft.com/en-us/excel/functions/gammainv-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/beta_gamma_stats_family.rs); implementation steps and the limits identified in the text.
