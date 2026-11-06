---
schema: efh.function-page/v1
function_id: FUNC.JIS
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

# JIS

## What it does

`JIS(text)` is the language-dependent width-conversion name documented for changing half-width English letters or katakana to full-width forms. The function name and supported character transformations depend on Excel’s language settings.

## Try it

Under the documented Japanese mapping, `=JIS("abc")` produces full-width `ａｂｃ`. A string containing only characters outside the half-width conversion set is retained. The preview returns `#NAME?` because its engine does not resolve this spelling; that support limit is not a reading from Excel.

## Quirks

“Single-byte” and “double-byte” describe legacy character-width conventions here, not UTF-8 byte lengths. This is not a generic Unicode normalization operation. The current Windows-oriented catalogue exposes DBCS for the width conversion and does not supply an executable JIS entry; locale/name resolution is part of the missing context.

## How to implement

Choose the language-specific mapping first, then scan text with a table of supported half-width letters and kana. Handle voiced kana sequences as the mapping requires and leave unrelated characters intact. Preserve UTF-16 text and invalid-text policy rather than counting encoded bytes. A whole-string NFKC call is not a substitute: it changes a broader class of characters and converts compatibility forms in another direction. The exact mapping outside the documented Japanese examples needs locale-specific evidence.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[DBCS](FUNC.DBCS.md) is the other documented name for conversion to full-width forms; [ASC](FUNC.ASC.md) performs the opposite width direction.

## Sources

- [Microsoft: JIS function](https://support.microsoft.com/en-us/excel/functions/jis-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/text_format.rs); implementation steps and the limits identified in the text.
