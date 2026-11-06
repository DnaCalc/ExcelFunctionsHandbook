---
schema: efh.function-page/v1
function_id: FUNC.CHISQ.INV
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

# CHISQ.INV

## What it does

`CHISQ.INV(probability, deg_freedom)` finds the chi-square value whose left cumulative probability is the requested value.

## Try it

`=CHISQ.INV(0.5,2)` is about `1.38629436111989`. At two degrees of freedom, a half right tail solves `exp(-x/2)=0.5`; the same point also has a half left CDF.

## Quirks

Degrees of freedom are truncated. Microsoft documents `#NUM!` below one or above `10^10`, and for probabilities outside zero to one; nonnumeric arguments give `#VALUE!`.

The current left inverse returns zero at probability zero and `#NUM!` at one. Probability one has no finite mathematical quantile, despite being inside the range described by the page’s out-of-range test. The companion right-tail inverse uses a different endpoint policy and is not obtained by relabelling the same probability.

## How to implement

1. Truncate and validate the degree count k, including the current upper limit `10^10`. Validate finite p and handle its endpoints.
2. Call the current [GAMMA.INV](FUNC.GAMMA.INV.md) kernel with probability p, shape `k/2` and scale two. This shared composition is supported by the source’s retained 63 interior pairs; it is not a claim that any gamma library can be substituted unchanged.
3. That inverse uses its own generic lower incomplete-gamma forward callback. It expands an upper x bound and bisects the ordered binary64 encodings until the endpoints are adjacent, then returns the upper one. Preserve its callback and return rule rather than stopping when an arbitrary real-valued residual looks small.

At k=2, the mathematical CDF is `1-exp(-x/2)`. Probability one half therefore gives `x=-2*ln(0.5)`, about `1.38629436111989`. This identity explains the example; the implementation still goes through the shared inverse.

The public GAMMA.DIST wrapper has special-case branches that the inverse callback does not automatically inherit. Complete forward last bits and all inverse edge cases remain open; the 63-pair composition does not identify private Excel source or settle the whole domain.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 40 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.CHISQ.INV.json).

## Alternatives

[CHISQ.DIST](FUNC.CHISQ.DIST.md) supplies the forward probability; [CHISQ.TEST](FUNC.CHISQ.TEST.md) returns a test p-value directly.

## Sources

- [Microsoft: CHISQ.INV function](https://support.microsoft.com/en-us/excel/functions/chisq-inv-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/chi_f_t_family.rs); implementation steps and the limits identified in the text.
