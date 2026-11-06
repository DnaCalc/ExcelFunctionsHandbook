---
schema: efh.function-page/v1
function_id: FUNC.BESSELJ
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

# BESSELJ

## What it does

`BESSELJ(x,n)` evaluates the order-n Bessel function of the first kind. It is a special function used in radial and oscillatory problems, rather than an elementary trigonometric function. The order is truncated to an integer.

## Try it

`=BESSELJ(0,0)` returns `1`, and `=BESSELJ(0,3)` returns `0`. These zero-input identities are useful controls.

## Quirks

Nonnumeric arguments give `#VALUE!`; a negative order gives `#NUM!`. Negative x follows the integer-order parity relation. Recorded code also limits truncated order to 10,000,000 and applies phase-based large-argument admission. Published approximation methods remain distinct from an identification of Excel’s numerical body.

## How to implement

Validate x and the order before selecting a numerical route. Use order-zero and order-one seed approximations. For higher orders the recurrence is `C_(j+1)=(2*j/x)*C_j-C_(j-1)`. Forward recurrence is used when the magnitude of x exceeds the order; otherwise the current code uses an adaptive backward program with normalization to avoid amplifying the wrong solution. For example the recurrence gives `C2=(2/x)*C1-C0`; its numbers depend on both seed values. Seed coefficients, phase grouping and flushing are separate compatibility questions, with residual differences still recorded. This is a published-method implementation outline, not a claim that every Excel stage is solved.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20228 | Windows, 64-bit-bit, CPU not recorded | 794 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.BESSELJ.json).

## Alternatives

[BESSELI](FUNC.BESSELI.md) and [BESSELK](FUNC.BESSELK.md) are modified Bessel functions. [BESSELY](https://handbook.dnacalc.com/functions/BESSELY.html) supplies the other ordinary kind.

## Sources

- [Microsoft: BESSELJ function](https://support.microsoft.com/en-us/excel/functions/besselj-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/bessel.rs); implementation steps and the limits identified in the text.
