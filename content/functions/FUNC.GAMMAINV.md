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

Alpha and beta must be positive; nonpositive shape or scale gives `#NUM!`, while text arguments are documented as `#VALUE!`. The opened legacy page rejects probabilities outside `[0,1]`. The current shared helper returns zero at `p=0` and rejects `p=1` with `#NUM!`, since no finite upper support endpoint is available.

Microsoft describes this legacy inverse as an iterative search over GAMMADIST with a 100-iteration limit and `#N/A` on failure. Its current GAMMA.INV page describes 64 iterations and also `#N/A`. This is a difference in published descriptions of the two spellings; it does not establish a measured change between Excel builds or identify either internal search.

## How to implement

The current helper searches the positive binary64 lattice rather than stopping at a decimal tolerance:

1. Validate shape, scale and probability. Return zero at `p=0`; reject `p>=1` or `p<0`.
2. Start the upper bound at `hi=beta*max(alpha+10*sqrt(alpha)+10,1)`. The code allows up to 200 doublings while the generic forward value `P(alpha,hi/beta)` is below `p`.
3. With lower bound zero, encode the nonnegative bounds as unsigned binary64 bit integers. Their ordering agrees with numeric ordering.
4. While the two encodings differ by more than one, bisect their integer encodings and decode the middle value `x`. If `P(alpha,x/beta)>=p`, move the upper encoding to it; otherwise move the lower encoding.
5. Return the upper value once the encodings are adjacent. This chooses the first representable candidate meeting the current forward test under the maintained bracket.

For shape one, scale two and probability one half, the mathematical equation is `1-exp(-x/2)=0.5`; its solution is `-2*ln(0.5)`, about `1.38629436111989`. That explains the scale and the example, not every last bit of the generic forward search.

The inverse callback calls `regularized_gamma_p` directly. It does not use every special-case wrapper of public GAMMADIST. Its forward approximation, bracket failures and return policy remain separate compatibility obligations; the current problem report still lists distribution mismatches. This is the current implementable candidate, not a recovered Excel search program.

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
- [Binary64-lattice bisection](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/special_math_common.rs), `bisect_inverse`.
- [Microsoft: GAMMA.INV](https://support.microsoft.com/en-us/excel/functions/gamma-inv-function), opened 6 November 2026; the modern page’s documented 64-iteration limit and error.
