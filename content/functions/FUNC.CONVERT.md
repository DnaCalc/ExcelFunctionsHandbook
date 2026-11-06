---
schema: efh.function-page/v1
function_id: FUNC.CONVERT
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

# CONVERT

## What it does

`CONVERT(number, from_unit, to_unit)` converts a measured quantity between supported units of the same dimension. Unit names are text codes, not the labels shown by cell formatting.

## Try it

`=CONVERT(3,"ft","in")` returns `36`: each foot contributes twelve inches. `=CONVERT(32,"F","C")` returns zero; a temperature conversion needs an offset as well as a scale.

Prefix case changes the quantity. `=CONVERT(1,"Mm","m")` returns `1000000`, whereas `=CONVERT(1,"mm","m")` returns `0.001`. The first unit is a megametre and the second a millimetre.

## Quirks

Unit codes and prefixes are case-sensitive. Microsoft documents `#N/A` for an unknown unit, a binary prefix not supported by its unit, or units from incompatible measurement groups; incorrect input types give `#VALUE!`. A mass-to-length conversion has no factor in this function.

The opened Microsoft unit list is broader than the current code table. It includes information units `bit` and `byte` and binary prefixes; the current helper implements a smaller linear-unit table and decimal prefixes. A preview `#N/A` for a documented unit can therefore be an implementation coverage gap. Do not infer an Excel rejection from that result alone.

Temperature offsets also prevent a single scale-only implementation. The current helper handles C, F and K through explicit pair formulas, and trims outer whitespace from unit strings before its case-sensitive lookup.

## How to implement

Keep unit validation, linear arithmetic and temperature arithmetic separate.

For the current linear route, resolve each code to a measurement category, stored base factor and decimal-prefix exponent. Resolve a direct unit name before trying prefixes. Check that categories agree, then use the [stored operation R](../model/09-staged-arithmetic.md):

```text
product = R(number * from_factor)
core = R(product / to_factor)
result = R(core * stored_power_of_ten(from_prefix - to_prefix))
```

Prefix scaling is a separate final multiplication. Replacing the three steps by one precomputed ratio changes the rounding history. The source retains a disjoint 10,418-row check for its selected linear graph; that scope does not cover every documented unit or prove universal agreement.

For three feet to inches, the retained length factors are `3048000000` and `254000000`. The product is `9144000000`, the division gives 36, and prefix scaling multiplies by one. For metres to millimetres the base factors cancel, then the final prefix factor is 1000.

The current temperature route uses ordinary binary64 pair formulas. C to F is `x*1.8+32`; F to C is `(x-32)/1.8`; C to K adds `273.15`. Do not replace division by the stored `1.8` with multiplication by a separately stored `5/9` when reproducing this code. An explicit reviewed unit table and the stated operation order are both needed; extending the table still requires its own Excel comparisons.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20228 | Windows, 64-bit, CPU not recorded | 33,972 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.CONVERT.json).

## Alternatives

[EUROCONVERT](FUNC.EUROCONVERT.md) handles its specified currency table. Direct multiplication is clear for a single known factor, but does not validate units or handle offsets.

## Sources

- [Microsoft: CONVERT function](https://support.microsoft.com/en-us/excel/functions/convert-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/misc_conversion_family.rs); implementation steps and the limits identified in the text.
