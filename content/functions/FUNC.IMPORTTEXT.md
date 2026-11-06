---
schema: efh.function-page/v1
function_id: FUNC.IMPORTTEXT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "IMPORTTEXT function"
    locator: "https://support.microsoft.com/en-us/excel/functions/importtext-function"
    role: "public source read for this page"
  - work: "IMPORTCSV function"
    locator: "https://support.microsoft.com/en-us/excel/functions/importcsv-function"
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

# IMPORTTEXT

## What it does

`IMPORTTEXT` loads a text file into a dynamic array with one formula.

`IMPORTTEXT(path, [delimiter], [skip_rows], [take_rows], [encoding], [locale])`

- **`path`** is a local file path or a URL.
- **`delimiter`** separates columns, with tab as the default. Ascending integers such as `{1,3}` split fixed-width columns.
- **`skip_rows`** and **`take_rows`** count rows. A negative number counts from the end.
- **`encoding`** defaults to UTF-8.
- **`locale`** sets regional formats and defaults to the operating system's.

## Try it

## Quirks

- **It does not update by itself.** Use Refresh All on the Data tab; a change to the file does not recalculate the cell.
- **A URL may ask for sign-in.** Excel shows an authentication dialog, and saved permissions are managed under Data
  Source Settings.
- **The default locale belongs to the machine**, so one file can parse differently on two systems unless `locale` is given.

Microsoft's page does not list errors, and none are recorded here.

## How to implement

DNA Calc does not know this name yet. An implementation needs file and URL access from the host, a parser with the
page's delimiter, encoding and locale rules, an array result, and a refresh step separate from recalculation. Web sources
also need the host's sign-in handling.

## Version differences

Microsoft's page lists it for Microsoft 365 Insiders in the Beta channel on Windows, version 2502 (build 18604.20002) or
later. No Excel answers for this function are recorded here, so no comparison between Excel builds exists.

## Alternatives

- `IMPORTCSV`, its shorter form for CSV files.
- Power Query, for transformations and repeatable loads.

## Sources

- [IMPORTTEXT function](https://support.microsoft.com/en-us/excel/functions/importtext-function)
- [IMPORTCSV function](https://support.microsoft.com/en-us/excel/functions/importcsv-function)
