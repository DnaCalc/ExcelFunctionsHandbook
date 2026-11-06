---
schema: efh.function-page/v1
function_id: FUNC.HAS
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "HAS function"
    locator: "https://support.microsoft.com/en-us/excel/functions/has-function"
    role: "public source read for this page"
  - work: "What's New in Excel (September 2026)"
    locator: "https://techcommunity.microsoft.com/blog/excelblog/whats-new-in-excel-september-2026/4549755"
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

# HAS

## What it does

`HAS` tests whether a value is in an array or range, returning TRUE or FALSE.

`HAS(array, value)`

Nested levels are searched too.

## Try it

## Quirks

Microsoft's page does not state its matching rules in detail, and no reading is recorded here.

## How to implement

No implementation exists yet. It needs arrays held in a single cell and nested arrays, which DNA Calc's value model does not
have, then the search or flattening rule on top. Exact matching rules should be measured in Excel first.

## Version differences

Microsoft announced it in September 2026 for Insiders on Windows and Mac, with lists and arrays in cells. Its page lists
Excel for Microsoft 365 on Windows and Mac. No Excel answers for this function are recorded here, so no comparison between Excel builds exists.

## Alternatives

- `HASANY` and `HASALL` for several values.
- `COUNTIF` or `XMATCH` on ordinary ranges.

## Sources

- [HAS function](https://support.microsoft.com/en-us/excel/functions/has-function)
- [What's New in Excel (September 2026)](https://techcommunity.microsoft.com/blog/excelblog/whats-new-in-excel-september-2026/4549755)
