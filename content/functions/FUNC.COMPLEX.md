---
schema: efh.function-page/v1
function_id: FUNC.COMPLEX
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

# COMPLEX

## What it does

`COMPLEX(real_num, i_num, [suffix])` constructs Excel’s text representation of a complex number from its two numeric coefficients. The optional imaginary suffix defaults to `"i"`; `"j"` is also supported.

## Try it

`=COMPLEX(2,-3)` returns `"2-3i"`. `=COMPLEX(0,1,"j")` returns `"j"`, since the real zero and imaginary coefficient one are suppressed.

## Quirks

Only lowercase `i` and `j` are documented suffixes; another suffix gives `#VALUE!`. A nonnumeric coefficient also gives `#VALUE!`. The result is text, not a new numeric cell type. The current implementation also admits an empty suffix as the default i; that is an implementation detail beyond the documented two spellings.

## How to implement

Coerce the two coefficients, validate the suffix and normalise zeros. Format each nonzero coefficient under the complex decimal-format rules. If both are zero, produce text zero. Join the real part and signed imaginary part without spaces, omitting the imaginary coefficient when its magnitude is one. For `2,-3`, emit the real two, then the minus sign, then `3i`. Do not use an arbitrary language debug representation: exponent spelling, precision and sign suppression are reader-visible.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit-bit, CPU not recorded | 10,694 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.COMPLEX.json).

## Alternatives

[IMREAL](https://handbook.dnacalc.com/functions/IMREAL.html) and [IMAGINARY](https://handbook.dnacalc.com/functions/IMAGINARY.html) recover the coefficients; [IMSUM](https://handbook.dnacalc.com/functions/IMSUM.html) and [IMPRODUCT](https://handbook.dnacalc.com/functions/IMPRODUCT.html) perform complex arithmetic.

## Sources

- [Microsoft: COMPLEX function](https://support.microsoft.com/en-us/excel/functions/complex-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/engineering.rs); implementation steps and the limits identified in the text.
