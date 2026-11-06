---
schema: efh.function-page/v1
function_id: FUNC.IMSUM
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

# IMSUM

## What it does

`IMSUM(inumber1, [inumber2], ...)` adds complex numbers component by component. The variadic form accepts individual complex values and collections of them.

## Try it

`=IMSUM("2+3i","4-i")` returns `"6+2i"`. Work with coefficients to explain the value; the displayed result is assembled as complex text.

## Quirks

Complex arguments may be real numbers or text such as `"2-3i"`. Use lowercase `i` or `j`; operands in one call must agree on the suffix. The current parser rejects malformed complex text with `#NUM!` and logical values with `#VALUE!`. A complex result is text, so ordinary worksheet arithmetic does not automatically treat it as a pair of numeric components. Floating-point reduction order can affect a long calculation even when the corresponding mathematical operation is associative.

## How to implement

Parse the two coefficients and suffix before doing arithmetic; use the common complex formatter afterward. Retain suffix consistency, suppress a zero component and omit the coefficient one before the imaginary unit. Formatting and parsing have their own fifteen-significant-digit rules, so testing only the algebra misses observable behaviour.

Start at `(0,0)` and fold `(a,b)+(c,d)=(a+c,b+d)` in argument order. The example adds real coefficients two and four and imaginary coefficients three and minus one. The current implementation uses ordinary binary64 component arithmetic. Keep error propagation and traversal of range arguments consistent with the worksheet call model; flattening values without their error policy can change which result is returned.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 3,260 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IMSUM.json).

## Alternatives

[COMPLEX](FUNC.COMPLEX.md) constructs each input; [IMDIV](https://handbook.dnacalc.com/functions/IMDIV.html) divides; [IMPOWER](https://handbook.dnacalc.com/functions/IMPOWER.html) raises one input to a real power.

## Sources

- [Microsoft: IMSUM function](https://support.microsoft.com/en-us/office/imsum-function-81542999-5f1c-4da6-9ffe-f1d7aaa9457f), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
