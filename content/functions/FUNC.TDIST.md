---
schema: efh.function-page/v1
function_id: FUNC.TDIST
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

# TDIST

## What it does

`TDIST(x,deg_freedom,tails)` returns a Student-t upper-tail probability. One tail means `P(T>x)`; two means `P(abs(T)>x)`. The compatibility function takes nonnegative x, not a signed cumulative-probability argument.

## Try it

`=TDIST(1,1,2)` is about `0.5`; with one tail it is about `0.25`. At one degree of freedom the distribution is Cauchy, giving this useful control.

## Quirks

Degrees of freedom and tails are truncated. Degrees below one, negative x and tails other than one or two give `#NUM!`; nonnumeric inputs give `#VALUE!`. Halving or doubling a tail is a different operation from changing a CDF into an upper tail.

## How to implement

For nonnegative x and degree v, form `t2=x*x`, `den=v+t2`, and the paired beta arguments `z=v/den`, `zc=t2/den`. The current two-tail route evaluates the beta ratio with shape `v/2` and `1/2` using both supplied arguments; the one-tail route halves that result. Computing `zc=1-z` can lose an observed rounding. At x=v=1 both beta arguments are one half and the two-tail result is one half. Inner beta-ratio last bits remain open.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 17 |
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 92 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.TDIST.json).

## Alternatives

[T.DIST.RT](FUNC.T.DIST.RT.md) and [T.DIST.2T](FUNC.T.DIST.2T.md) make the tail explicit; [T.DIST](FUNC.T.DIST.md) provides a signed CDF/density; [TINV](https://handbook.dnacalc.com/functions/TINV.html) finds a two-tail critical value.

## Sources

- [Microsoft: TDIST function](https://support.microsoft.com/en-us/excel/functions/tdist-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/chi_f_t_family.rs); implementation steps and the limits identified in the text.
