---
schema: efh.function-page/v1
function_id: FUNC.IMPORTCSV
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "IMPORTCSV function"
    locator: "https://support.microsoft.com/en-us/excel/functions/importcsv-function"
    role: "public source read for this page"
  - work: "IMPORTTEXT function"
    locator: "https://support.microsoft.com/en-us/excel/functions/importtext-function"
    role: "public source read for this page"
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

# IMPORTCSV

## What it does

`IMPORTCSV` loads a CSV file into a dynamic array with one formula. It is a shorter form of `IMPORTTEXT` for this one format.

`IMPORTCSV(path, [skip_rows], [take_rows], [locale])`

`path` and the optional arguments mean what they mean in [IMPORTTEXT](FUNC.IMPORTTEXT.md).

## Try it

## Quirks

It does not update by itself: use Refresh All on the Data tab. The default locale belongs to the machine. Microsoft's page does not list errors, and none are recorded here.

## How to implement

DNA Calc does not know this name yet. An implementation needs file and URL access from the host, a parser with the
page's delimiter, encoding and locale rules, an array result, and a refresh step separate from recalculation. Web sources
also need the host's sign-in handling.

## Version differences

Microsoft's page lists it for Microsoft 365 Insiders in the Beta channel on Windows, version 2502 (build 18604.20002) or
later. No Excel answers for this function are recorded here, so no comparison between Excel builds exists.

## Alternatives

- `IMPORTTEXT` for other delimiters, fixed-width files and encodings.
- Power Query, for transformations and repeatable loads.

## Sources

- [IMPORTCSV function](https://support.microsoft.com/en-us/excel/functions/importcsv-function)
- [IMPORTTEXT function](https://support.microsoft.com/en-us/excel/functions/importtext-function)
