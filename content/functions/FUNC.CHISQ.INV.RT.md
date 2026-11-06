---
schema: efh.function-page/v1
function_id: FUNC.CHISQ.INV.RT
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

# CHISQ.INV.RT

## What it does

`CHISQ.INV.RT(probability, deg_freedom)` finds the chi-square value whose right-tail probability is the requested value.

## Try it

`=CHISQ.INV.RT(0.5,2)` is about `1.38629436111989`. At two degrees of freedom, a half right tail solves `exp(-x/2)=0.5`; the same point also has a half left CDF.

## Quirks

Degrees of freedom are truncated. Invalid probabilities or degree counts give `#NUM!`; nonnumeric arguments give `#VALUE!`. The current right-tail inverse requires `0<p<1`, even though the old documentation’s range wording can appear to include endpoints. The current left inverse admits zero as zero and rejects probability one as `#NUM!`.

## How to implement

Solve the decreasing upper gamma ratio directly, rather than converting p to `1-p`. Expand an upper x bound until the target tail is bracketed; use a bracketed search with the inequality reversed for a decreasing function. The current helper implements this through the negated forward function. Worked at df=2, `x=-2*ln(0.5)` gives about `1.38629436111989`. Inverse accuracy depends on the forward routine and on its stopping rule; the family still has recorded last-bit differences.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[CHISQ.DIST.RT](FUNC.CHISQ.DIST.RT.md) supplies the forward probability; [CHISQ.TEST](https://handbook.dnacalc.com/functions/CHISQ.TEST.html) returns a test p-value directly.

## Sources

- [Microsoft: CHISQ.INV.RT function](https://support.microsoft.com/en-us/excel/functions/chisq-inv-rt-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/chi_f_t_family.rs); implementation steps and the limits identified in the text.
