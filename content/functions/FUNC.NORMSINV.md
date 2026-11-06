---
schema: efh.function-page/v1
function_id: FUNC.NORMSINV
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

# NORMSINV

## What it does

`NORMSINV(probability)` is the compatibility quantile of the standard normal distribution: it returns z with cumulative probability p.

## Try it

`=NORMSINV(0.5)` returns `0`, the centre of the standard normal distribution. A probability below one half has a negative quantile.

## Quirks

The probability must be strictly between zero and one; endpoints or out-of-range values give `#NUM!`. Nonnumeric arguments give `#VALUE!`. The mean and standard deviation are fixed at zero and one; there is no scale argument on this spelling.

A small probability refers to the far **left** tail. For an upper-tail cutoff use the corresponding cumulative probability, while recognizing that `1-small_tail` can round to one for very small tails. Supply probability as a number, such as `0.5` or a numeric `50%` cell.

Microsoft describes an iterative inverse search over the forward normal CDF, with `#N/A` if it has not converged after 100 iterations on this legacy page. That documented method and limit do not identify the current candidate’s rational coefficients or numerical stopping stages.

## How to implement

The current standard-normal inverse is a published-method candidate:

1. Validate `0<p<1`. Choose its rational approximation branch at `p=0.02425` and `p=1-0.02425`.
2. In the centre, set `q=p-0.5`, `r=q*q` and evaluate the coefficient polynomials in `r`, multiplying the numerator by `q`. In the lower tail use `q=sqrt(-2*crt_log(p))`; in the upper tail use `sqrt(-2*crt_log(1-p))` and reverse the sign. The cited source retains the coefficients and their Horner order.
3. Refine the candidate `z` once. Form a small tail directly as `tail=0.5*erfc(abs(z)/sqrt(2))`, using the current library complementary error function. Set `err=tail-p` for negative `z`, or `err=(1-p)-tail` otherwise.
4. If the normal density at `z` is nonzero, replace `z` by `z-err/density`. If it is zero, keep the rational candidate.

Return that `z`. At `p=0.5`, the central numerator has `q=0`, and the symmetric answer is zero.

This recipe can be coded from the source’s published-approximation coefficients, but it is not an identified Excel inverse. In particular, its Newton tail is a library `erfc` call; substituting the public worksheet ERFC introduces another numerical candidate. The unknown Excel approximation and stopping/rounding stages remain open.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 2 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.NORMSINV.json).

## Alternatives

[NORM.S.INV](FUNC.NORM.S.INV.md) is the modern spelling; [NORMSDIST](FUNC.NORMSDIST.md) gives the forward probability.

## Sources

- [Microsoft: NORMSINV function](https://support.microsoft.com/en-us/excel/functions/normsinv-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/normal_log_family.rs); implementation steps and the limits identified in the text.
