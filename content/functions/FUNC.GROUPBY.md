---
schema: efh.function-page/v1
function_id: FUNC.GROUPBY
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

# GROUPBY

## What it does

`GROUPBY(row_fields,values,function,[field_headers],[total_depth],[sort_order],[filter_array],[field_relationship])` aggregates source rows by one or more key columns and spills a grouped table. The callback can be SUM, another supported function value, a LAMBDA or a vector of callbacks.

## Try it

Put `east,west,east,west` in B2:B5 and `3,5,7,11` in C2:C5. `=GROUPBY(B2:B5,C2:C5,LAMBDA(v,SUM(v)),0,0)` returns the two group rows `east,10` and `west,16`. Explicit zero options mean no input headers and no totals.

## Quirks

Automatic header inference can remove a first row when values begin with text then a number; specify field_headers when that ambiguity matters. Header modes zero through three distinguish no headers, hidden existing headers, generated headers and shown existing headers. Totals, hierarchy/table relationships, sorting and filtering affect output shape. Current recorded text keys are case-insensitive and retain the first spelling.

## How to implement

Validate matching row counts, callable arguments and any Boolean filter. Resolve headers and remove filtered rows before grouping. Group rows by typed key tuples, preserve the represented key label, sort under the requested relationship and sort keys, then pass each group’s value column to the callback. Build requested subtotals/grand totals by aggregating their underlying rows, not by summing already aggregated cells. A two-argument callback also receives its comparison set. Recorded array callback results remain boxed within a result cell; do not silently flatten them. In the example east receives three and seven, west five and eleven.

## Version differences

The current source retains Excel 16.0 build 20527 COM observations from 30 October 2026: sixty grouping calls, plus separately described depth and multi-value controls. Those are one-build observations. No same-input difference against an earlier build is established here. Microsoft’s availability list is separate from these measurements.

## Alternatives

[PIVOTBY](FUNC.PIVOTBY.md) adds column groups; [SUMIFS](FUNC.SUMIFS.md) computes a chosen scalar group; [PERCENTOF](FUNC.PERCENTOF.md) uses the second callback argument.

## Sources

- [Microsoft: GROUPBY function](https://support.microsoft.com/en-us/excel/functions/groupby-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/grouping.rs); implementation steps and the limits identified in the text.
