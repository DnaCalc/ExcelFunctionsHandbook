---
schema: efh.function-page/v1
function_id: FUNC.IMSUB
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

# IMSUB

## What it does

`IMSUB(inumber1, inumber2)` subtracts its second complex argument from its first. Both arguments are required.

## Try it

`=IMSUB("5+2i","1+5i")` returns `"4-3i"`. Work with coefficients to explain the value; the displayed result is assembled as complex text.

## Quirks

Complex arguments may be real numbers or text such as `"2-3i"`. Use lowercase `i` or `j`; operands in one call must agree on the suffix. The current parser rejects malformed complex text with `#NUM!` and logical values with `#VALUE!`. A complex result is text, so ordinary worksheet arithmetic does not automatically treat it as a pair of numeric components. Floating-point reduction order can affect a long calculation even when the corresponding mathematical operation is associative.

## How to implement

Parse the two coefficients and suffix before doing arithmetic; use the common complex formatter afterward. Retain suffix consistency, suppress a zero component and omit the coefficient one before the imaginary unit. Formatting and parsing have their own fifteen-significant-digit rules, so testing only the algebra misses observable behaviour.

Compute `(a,b)-(c,d)=(a-c,b-d)`. The example gives `5-1=4` and `2-5=-3`. Preserve operand order; subtraction is not commutative. The current implementation uses ordinary binary64 component arithmetic. Keep error propagation and traversal of range arguments consistent with the worksheet call model; flattening values without their error policy can change which result is returned.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit-bit, CPU not recorded | 1,216 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IMSUB.json).

## Alternatives

[COMPLEX](FUNC.COMPLEX.md) constructs each input; [IMDIV](https://handbook.dnacalc.com/functions/IMDIV.html) divides; [IMPOWER](https://handbook.dnacalc.com/functions/IMPOWER.html) raises one input to a real power.

## Sources

- [Microsoft: IMSUB function](https://support.microsoft.com/en-us/office/imsub-function-2e404b4d-4935-4e85-9f52-cb08b9a45054), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
