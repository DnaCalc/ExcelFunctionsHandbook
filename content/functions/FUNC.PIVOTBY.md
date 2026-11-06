---
schema: efh.function-page/v1
function_id: FUNC.PIVOTBY
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

# PIVOTBY

## What it does

`PIVOTBY(row_fields,col_fields,values,function,[field_headers],[row_total_depth],[row_sort_order],[col_total_depth],[col_sort_order],[filter_array],[relative_to])` groups records along row and column keys, applies an aggregation callback and spills the cross-tabulation. Its optional arguments control headers, row/column totals and sorting, filtering, and the comparison set for a two-argument callback.

## Try it

Put regions `east,east,west,west` in B2:B5, periods `Q1,Q2,Q1,Q2` in C2:C5 and amounts `3,7,5,11` in D2:D5. `=PIVOTBY(B2:B5,C2:C5,D2:D5,LAMBDA(v,SUM(v)),0,0,,0)` returns east with Q1=3, Q2=7, and west with Q1=5, Q2=11, without totals.

A two-argument share callback can use the whole table as its denominator:

```excel
=PIVOTBY(B2:B5,C2:C5,D2:D5,LAMBDA(s,t,SUM(s)/SUM(t)),0,0,,0,,,2)
```

The final `2` selects the grand-total comparison set. East/Q1 is `3/26`, about `0.1153846154`. With relative_to zero its column denominator is `3+5=8`, giving `0.375`; with one its row denominator is `3+7=10`, giving `0.3`. Percentage formatting changes their display, not those fractions.

## Quirks

Header inference and spill space matter. Totals have separate row and column depth controls, including negative depths for totals before details. Relative_to applies only to two-argument callbacks: the public page lists column, row, grand, parent-column and parent-row comparison sets. Empty intersections are not automatically numeric zeros in the current recorded route.

| Optional argument | What it controls |
|---|---|
| `field_headers` | Whether the first source row is data or headings, and which headings appear. Modes are shared with GROUPBY. |
| `row_total_depth`, `col_total_depth` | Independent grand totals and hierarchy subtotals; `0` omits that axis's totals. |
| `row_sort_order`, `col_sort_order` | Signed sort positions; negative means descending. |
| `filter_array` | One Boolean for each source record. |
| `relative_to` | `0` column, `1` row, `2` grand, `3` parent-column, `4` parent-row comparison set. |

The row and column field arguments both describe source **columns**. They are not already shaped row/column headers to cross-multiply. This formula returns a spill array; it does not create an Excel PivotTable object.

## How to implement

Validate row alignment of both key tables, value columns and filter flags. Resolve header policy, group typed row and column tuples, sort each axis, and invoke the callback on values at each intersection. Generate subtotal/grand-total cells from their source-row sets. For a two-argument callable choose the second set using relative_to, preserving the grouping level. The current source retains separate scope limits for parent totals, multiple value columns and boxed array results; a basic hash-map sum does not implement these branches. The example has one row in each intersection, so it isolates layout from reduction.

## Version differences

The implementation comments retain Excel 16.0 build 20527 COM readings on 30 October 2026, including fifteen depth controls and ten multi-value/subtotal controls. They identify behaviour on that build and input scope. No same-input earlier-build comparison is recorded here.

## Alternatives

[GROUPBY](FUNC.GROUPBY.md) has only row groups; [PERCENTOF](FUNC.PERCENTOF.md) calculates a share within a supplied comparison set; [SUMIFS](FUNC.SUMIFS.md) can build a fixed cross-tab manually.

## Sources

- [Microsoft: PIVOTBY function](https://support.microsoft.com/en-us/excel/functions/pivotby-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/grouping.rs); implementation steps and the limits identified in the text.
