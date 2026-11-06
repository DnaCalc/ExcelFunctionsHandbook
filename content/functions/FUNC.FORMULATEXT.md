---
schema: efh.function-page/v1
function_id: FUNC.FORMULATEXT
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

# FORMULATEXT

## What it does

`FORMULATEXT(reference)` returns the stored formula text of the referenced cell, including the leading equals sign. It inspects the workbook’s formula rather than converting the cell’s calculated value back into an expression.

## Try it

In Excel put `=7*6` in B2, then enter `=FORMULATEXT(B2)` elsewhere. It returns the text `"=7*6"`, while B2 displays 42. The preview input grid currently supplies cell values, so its B2=1 control returns a worksheet error instead of an authored formula.

## Quirks

Microsoft documents `#N/A` when the target has no formula, the formula exceeds 8192 characters, worksheet protection hides it, or an external source workbook is closed. Invalid argument types give `#VALUE!`. A multi-cell reference selects its upper-left cell. A self-reference can return its own text without normal circular calculation.

## How to implement

Resolve the reference to its first cell and ask the workbook host for its stored formula text. Do not evaluate that formula to obtain the text and do not reconstruct spelling from the result. The current host interface distinguishes “known cell without formula” (`#N/A`) from “formula information unavailable” (a typed refusal); it also rejects a closed external workbook as `#N/A`. The host must supply formula length/protection policy and formula-bar spelling. This is a host-backed function, not a pure numeric kernel.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[ISFORMULA](FUNC.ISFORMULA.md) asks whether a formula is present; [FORMULATEXT](https://handbook.dnacalc.com/functions/FORMULATEXT.html) retrieves it; [TEXT](FUNC.TEXT.md) formats a calculated value.

## Sources

- [Microsoft: FORMULATEXT function](https://support.microsoft.com/en-us/excel/functions/formulatext-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/host_facts.rs); implementation steps and the limits identified in the text.
