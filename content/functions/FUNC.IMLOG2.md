---
schema: efh.function-page/v1
function_id: FUNC.IMLOG2
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

# IMLOG2

## What it does

`IMLOG2(inumber)` returns the principal base-two logarithm of a nonzero complex value. The real component describes magnitude and the imaginary component describes angle.

## Try it

`=IMLOG2("8")` returns approximately `"3"`. For a non-real example, `a+bi` with modulus one has natural-log real component zero; its phase supplies the imaginary component.

## Quirks

Complex arguments may be real numbers or text such as `"2-3i"`. Use lowercase `i` or `j`; operands in one call must agree on the suffix. The current parser rejects malformed complex text with `#NUM!` and logical values with `#VALUE!`. A complex result is text, so ordinary worksheet arithmetic does not automatically treat it as a pair of numeric components. Zero has no finite logarithm and the current kernel gives `#NUM!`. Principal angles create a branch cut on the negative real axis; a real-valued LOG call cannot replace this function.

## How to implement

Parse the two coefficients and suffix before doing arithmetic; use the common complex formatter afterward. Retain suffix consistency, suppress a zero component and omit the coefficient one before the imaginary unit. Formatting and parsing have their own fifteen-significant-digit rules, so testing only the algebra misses observable behaviour.

Compute a scaled modulus `h=hypot(a,b)` and principal angle `theta=atan2(b,a)`. Reject `h=0`. Form the natural-log pair `(ln(h),theta)`. Divide both components by the runtime natural logarithm of two. For non-real input the current code uses runtime log and atan2 leaves. On the real axis its real component instead uses the measured public LN of the absolute input multiplied with stored x87 arithmetic by the binary64 constant `0x3ff71547652b82ff`. The imaginary coefficient still uses the runtime phase and base logarithm. This mathematical composition does not identify all their rounding stages in Excel. In particular, preserve negative-real and signed-zero boundary tests rather than assuming that every logarithm wrapper shares IMARGUMENT’s ratio route.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,209 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IMLOG2.json).

## Alternatives

[IMEXP](https://handbook.dnacalc.com/functions/IMEXP.html) is the exponential companion; [IMABS](FUNC.IMABS.md) and [IMARGUMENT](FUNC.IMARGUMENT.md) expose the polar ingredients.

## Sources

- [Microsoft: IMLOG2 function](https://support.microsoft.com/en-us/office/imlog2-function-152e13b4-bc79-486c-a243-e6a676878c51), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.

- [Real-axis wrapper](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/engineering.rs).
