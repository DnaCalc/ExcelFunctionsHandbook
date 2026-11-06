---
schema: efh.function-page/v1
function_id: FUNC.FLATTEN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "FLATTEN function"
    locator: "https://support.microsoft.com/en-us/excel/functions/flatten-function"
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

# FLATTEN

## What it does

`FLATTEN` removes nesting from an array, so inner arrays are replaced by their contents.

`FLATTEN(array, [pad_value], [levels])`

- **`array`** is a nested array, or a reference to one.
- **`pad_value`** fills gaps when rows come out uneven; without it they show `#N/A`.
- **`levels`** is how many layers to remove, one by default. 0 removes all of them, and a number above the depth also does.

## Try it

## Quirks

Excel arrays are rectangular, so uneven rows are padded. Microsoft's page does not say what a flat input does, and no
reading is recorded here.

## How to implement

No implementation exists yet. It needs arrays held in a single cell and nested arrays, which DNA Calc's value model does not
have, then the search or flattening rule on top. Exact matching rules should be measured in Excel first.

## Version differences

Microsoft announced it in September 2026 for Insiders on Windows and Mac, with lists and arrays in cells. Its page lists
Excel for Microsoft 365 on Windows and Mac. No Excel answers for this function are recorded here, so no comparison between Excel builds exists.

## Alternatives

- `TOCOL` and `TOROW` for ordinary two-dimensional arrays.
- `VSTACK` and `HSTACK` to combine arrays.

## Sources

- [FLATTEN function](https://support.microsoft.com/en-us/excel/functions/flatten-function)
- [What's New in Excel (September 2026)](https://techcommunity.microsoft.com/blog/excelblog/whats-new-in-excel-september-2026/4549755)
