---
schema: efh.function-page/v1
function_id: FUNC.IMLN
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

# IMLN

## What it does

`IMLN(inumber)` returns the principal natural logarithm of a nonzero complex value. The real component describes magnitude and the imaginary component describes angle.

## Try it

`=IMLN("i")` gives about `"1.5707963267949i"`. The modulus is one, so the real logarithm component is zero; the positive imaginary axis supplies phase `pi/2`. `=IMLN("1")` returns text zero.

## Quirks

Complex arguments may be real numbers or text such as `"2-3i"`. Use lowercase `i` or `j`; operands in one call must agree on the suffix. The current parser rejects malformed complex text with `#NUM!` and logical values with `#VALUE!`. A complex result is text, so ordinary worksheet arithmetic does not automatically treat it as a pair of numeric components. Zero has no finite logarithm and the current kernel gives `#NUM!`. Principal angles create a branch cut on the negative real axis; a real-valued LOG call cannot replace this function.

## How to implement

Parse the two coefficients and suffix before doing arithmetic; use the common complex formatter afterward. Retain suffix consistency, suppress a zero component and omit the coefficient one before the imaginary unit. Formatting and parsing have their own fifteen-significant-digit rules, so testing only the algebra misses observable behaviour.

Compute a scaled modulus `h=hypot(a,b)` and principal angle `theta=atan2(b,a)`. Reject `h=0`. Form the natural-log pair `(ln(h),theta)`. Return those two components. The current code uses runtime log and atan2 leaves. This mathematical composition does not identify all their rounding stages in Excel. In particular, preserve negative-real and signed-zero boundary tests rather than assuming that every logarithm wrapper shares IMARGUMENT’s ratio route.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,209 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IMLN.json).

## Alternatives

[IMEXP](FUNC.IMEXP.md) is the exponential companion; [IMABS](FUNC.IMABS.md) and [IMARGUMENT](FUNC.IMARGUMENT.md) expose the polar ingredients.

## Sources

- [Microsoft: IMLN function](https://support.microsoft.com/en-us/office/imln-function-32b98bcf-8b81-437c-a636-6fb3aad509d8), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
