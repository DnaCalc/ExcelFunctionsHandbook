---
schema: efh.function-page/v1
function_id: FUNC.GETPIVOTDATA
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Why this entry does not calculate yet
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
  - Notes
  - Page vocabulary
  - Sources
family: null
role_in_family: null
---

## What it computes

`GETPIVOTDATA` queries a PivotTable. It does not read a cell; it names a *cell of a report* by
its coordinates in the report's own dimensional language, and returns whatever aggregate sits
there.

The query has two parts:

1. **Which report, and which measure.** `data_field` names a value field of the PivotTable —
   the aggregate being read, as a text name. `pivot_table` is a reference to any cell inside
   the PivotTable, used purely to identify the table; its address is not the address of the
   answer.
2. **Which cell of that report.** Zero or more `field`/`item` pairs each fix one dimension:
   "the `Region` field at item `North`", "the `Month` field at item `Mar`". Microsoft documents
   up to 126 pairs. The pairs may appear in any order, because they are named coordinates
   rather than positional ones. Multiple items for one field are given in braces
   (`{"Mar","Apr"}`), and OLAP sources may use hierarchical item names such as
   `"[Product].[All Products].[Foods].[Baked Goods]"`.

With no pairs, the result is the grand total for `data_field`. Each pair narrows the query one
dimension further.

The design intent is worth stating because it is routinely resented: `GETPIVOTDATA` exists so
that a formula referring to a PivotTable keeps meaning the same thing when the PivotTable is
refreshed, re-sorted, or re-laid-out. A plain `=D7` breaks the moment a row is added upstream.
`GETPIVOTDATA("Sales", $A$3, "Region", "North")` does not, because it names the data rather
than its position. That is also why Excel *generates* these formulas automatically when you
type `=` and click a PivotTable cell — behaviour that can be turned off under **PivotTable
Analyze ▸ Options ▸ Generate GetPivotData**, and which is the single most common reason readers
encounter this function without having chosen it.

## Arguments

Microsoft documents
`GETPIVOTDATA(data_field, pivot_table, [field1, item1, field2, item2], …)`.

**`data_field`** — required, text, quoted. The name of the PivotTable's value field.

**`pivot_table`** — required. A reference to any cell, range, or named range **within** the
PivotTable. It identifies which table to query, nothing more. Passing a reference that is not
inside a PivotTable is an error.

**`field`/`item` pairs** — optional and repeating, up to 126 pairs. Field names and non-numeric,
non-date items are quoted. Two points that catch people:

- **Dates and times must be expressed as serial numbers or built with `DATE`/`TIME`.**
  Microsoft is explicit about this, and the reason is locale: a quoted date string means
  different things on different machines, so a workbook written in one locale would query the
  wrong item in another. `DATE(1999,3,5)` and `36224` are portable; `"3/5/1999"` is not.
- **The pairs are unordered**, which means there is no positional discipline to lean on and a
  misspelled field name is a runtime failure rather than a shifted argument.

## Why this entry does not calculate yet

The catalogue has a row for `GETPIVOTDATA`, and the row declines every call. PivotTables are being
implemented in full, and this function with them; the state at the top of this page comes from the
pinned commit and changes when that code lands.

The reason is layering rather than difficulty. Every other function
in this Handbook's lookup-and-reference category can be described as a computation over values
and addresses. `GETPIVOTDATA` cannot: its arguments are coordinates in a *PivotTable's* model —
fields, items, hierarchies, filters, visibility — and that model is a workbook object with its
own cache, its own refresh lifecycle, and its own source (a range, an external query, or an
OLAP cube). A function evaluator that owns none of that has nothing to query.

The consequences are visible in the projected metadata, and the Handbook shows them rather than
hiding them:

- the entry carries **no signature and no arity** in the projection, because the reference
  engine has no registered definition to project;
- the Handbook's uniform behavioural battery records every probe row as **not dispatchable** —
  `cannot-call:not-in-reference-catalog`;
- the entry has **no implementing module**, and therefore no family.

This is the honest state, and the Handbook publishes it as such: a real entry, with real
documented semantics, and no implementation behind it.

## Result and edge cases

The return kind is whatever the PivotTable's value field holds at the queried coordinate —
almost always a number, since value fields are aggregates, but a text or error value is
possible depending on the source and the aggregation.

- **A query with no pairs** returns the grand total.
- **Multiple items for one field** narrow to the union of those items, subject to how the
  PivotTable aggregates them.
- **Calculated fields and calculated items** are part of the PivotTable's model and are
  queryable by name like any other field, but what they return depends on the PivotTable's own
  calculation semantics rather than on `GETPIVOTDATA`.
- **The layout is irrelevant.** Collapsing an outline, moving a field from rows to columns, or
  re-sorting does not change what a given `GETPIVOTDATA` call returns — which is the whole
  point.
- **Refresh does change it**, because the underlying data changed.

## Errors

Microsoft documents `#REF!` for this function, with three stated causes:

| Error | Documented condition |
|---|---|
| `#REF!` | The arguments describe a field that is not visible |
| `#REF!` | A report filter excludes the requested data |
| `#REF!` | The `pivot_table` argument is not an actual PivotTable range |

The first two are the ones that surprise. `GETPIVOTDATA` reads the *report*, not the source
data: a value that exists in the source but is filtered out of the current view is not
retrievable, and a field that has been removed from the layout is not retrievable either. A
formula that worked yesterday can return `#REF!` today because someone collapsed a field —
which is a different fragility from the one `GETPIVOTDATA` was built to remove, and worth
knowing about before adopting it wholesale.

## Relationships

- **`RTD`** is the other entry in this category the reference engine cannot answer, for the
  symmetric reason: `RTD` needs an external process, `GETPIVOTDATA` needs a workbook object.
- **`CUBEVALUE` / `CUBEMEMBER`** and the rest of the cube family are the OLAP-native way of
  asking the same kind of dimensional question, without a PivotTable in between.
- **`SUMIFS` / `COUNTIFS` / `AVERAGEIFS`** query the *source* data with criteria and are the
  usual alternative when the PivotTable is only there as an intermediary. They see filtered-out
  rows, which `GETPIVOTDATA` does not — sometimes a feature, sometimes a bug.
- **`INDEX`/`MATCH` over the PivotTable's cells** is what people do when they turn off
  automatic `GETPIVOTDATA` generation. It is positional and therefore fragile in exactly the
  way `GETPIVOTDATA` avoids.
- The `Generate GetPivotData` toggle is the reason most readers meet this function; it is a UI
  setting, not a function argument.

## Notes for implementers

- This function cannot be implemented in the function layer. It requires a PivotTable object
  model — fields, items, hierarchies, active filters, visibility, and the cache behind them —
  which is workbook state, not evaluation state. Any implementation is really an implementation
  of that model with a thin lookup on top.
- The query is by *name*, not by position, and names are matched against the report's current
  configuration. Visibility is part of the match: an invisible field is not merely empty, it is
  `#REF!`.
- Date and time items must be handled as serial numbers to stay locale-portable. An
  implementation that accepts date strings will produce workbooks that break when opened
  elsewhere.
- The `pivot_table` argument must be resolved to a table identity, not dereferenced to a value.
- The 126-pair limit is a real boundary and belongs in the admission check.

## What has not been checked

No Handbook vector suite exists for `GETPIVOTDATA`, no Handbook evidence record is attached to
this page, and — unlike most entries — there is no reference implementation to compare against
either. The reference engine defers the function, and the Handbook's battery records it as not
dispatchable for every probe. Nobody has checked this function's behaviour against Excel in
this project, and the layering reason above means nobody will until PivotTable structure enters
scope.

What a characterization would need, whenever it happens:

1. **A fixture PivotTable** with known source data, several dimensions, a report filter, a
   calculated field, and a date dimension — because almost every interesting behaviour needs
   one of those to be observable.
2. **The `#REF!` triangle**: a hidden field, a filter-excluded item, and a non-PivotTable
   `pivot_table` reference, to confirm all three produce the same error rather than three
   different ones.
3. **Date and time items** as serial numbers, as `DATE(...)` results, and as locale-formatted
   text, across at least two locales.
4. **Item-name matching**: case sensitivity, leading and trailing spaces, and whether a
   misspelled field is distinguishable from a filtered-out one.
5. **Multiple items in braces**, and whether the result aggregates or errors.
6. **The grand-total form** with no pairs, and behaviour when `data_field` names a field that
   is not a value field.
7. **Layout invariance**: the same call before and after collapsing, pivoting and re-sorting,
   which is the property the function is sold on and which nobody in this project has verified.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `GETPIVOTDATA` that an implementer matching Excel, or a reader explaining
an unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's
answer and where it was recorded; "measured" means the statement is read off recorded Excel
answers, "inferred" marks what goes beyond them. Where older text on this page says a behaviour
was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

### FN-GETPIVOTDATA-001: GETPIVOTDATA resolves visible PivotTable cells without trimming, matches "(blank)" for blank items, and returns #N/A for a reference data_field

*Quirk; measured; recorded 2026-11-22.*

In Excel 16.0.20527, GETPIVOTDATA(data_field, pivot_table, [field1, item1, ...]) returns #REF!
for an odd argument count, a non-PivotTable reference, a hidden/filtered item, or an omitted
inner field lacking a visible subtotal; returns #N/A when data_field is a cell reference (while
numeric/logical/empty data_field gives #REF!) or when an error appears in data_field,
pivot_table, or a field name (whereas an error in an item argument gives #REF!); matches
data_field case-insensitively against caption or base field name without trimming trailing
spaces; and matches blank pivot items via the text "(blank)" rather than "". Matches
1,400/1,400 scalar probe rows (#REF!) and the pivot_getpivotdata oracle workbook.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=GETPIVOTDATA("Sum of Amount",PvOne!$A$3,"Region","East")` | as typed | `2074.5` (`0x40a0350000000000`) | Excel 16.0.20527.20150 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=GETPIVOTDATA("Sum of Amount",PvOne!$A$3,"Region","(blank)")` | as typed | `158.0` (`0x4063c00000000000`) | Excel 16.0.20527.20150 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=GETPIVOTDATA(PvOne!$B$3,PvOne!$A$3)` | as typed | `#N/A` | Excel 16.0.20527.20150 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=GETPIVOTDATA("Amount ",PvOne!$A$3)` | as typed | `#REF!` | Excel 16.0.20527.20150 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Measured across Excel 16.0.20527.20150 in
crates/calc-host/tests/captures/pivot_getpivotdata/oracle.capture.json (123 formulas), Excel
16.0.20527.20088 in prefix_metadata row 286, and 1,400/1,400 scalar probe rows in
CalcFunc-Evidence single-cell-answers-getpivotdata-scalar.json.

- Open: Successful GETPIVOTDATA queries against Data Model (OLAP) pivots still need full model-pivot evaluation beyond error cases (invalid_model_request in index.rs).

Evidence:

- DnaCalc `crates/calc-func/src/catalogue/host_facts.rs`, getpivotdata (lines 526-560) at `c08fee0a47` — arity, reference data_field #N/A, and host pivot lookup
- DnaCalc `crates/calc-engine/src/pivot/index.rs`, data_cell (lines 258-446) at `c08fee0a47` — visible axis lookup, "(blank)" matching, and #REF! rules
- DnaCalc `crates/calc-host/tests/captures/pivot_getpivotdata/oracle.capture.json`, sheet GPD rows 2-127 at `c08fee0a47` — 123 recorded Excel 16.0.20527.20150 PivotTable lookup cases
- DnaCalc `crates/calc-host/tests/captures/pivot_getpivotdata/oracle.capture.json`, sheet GPD row 21 column 2 at `c08fee0a47` (readings)
- DnaCalc `crates/calc-host/tests/captures/pivot_getpivotdata/oracle.capture.json`, sheet GPD row 27 column 2 at `c08fee0a47` — "" returns #REF! (row 28) (readings)
- DnaCalc `crates/calc-host/tests/captures/pivot_getpivotdata/oracle.capture.json`, sheet GPD row 15 column 2 at `c08fee0a47` — reference data_field gives #N/A whereas numeric 1 gives #REF! (row 16) (readings)
- DnaCalc `crates/calc-host/tests/captures/pivot_getpivotdata/oracle.capture.json`, sheet GPD row 6 column 2 at `c08fee0a47` — trailing space is not trimmed (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| data field | The PivotTable value field being read — the measure |
| field/item pair | One named dimensional coordinate narrowing the query |
| not dispatchable | Battery outcome: `cannot-call:not-in-reference-catalog` |
| layout invariance | The property that the result does not depend on the report's arrangement |

## Sources

- Microsoft, *GETPIVOTDATA function* —
  <https://support.microsoft.com/en-us/office/getpivotdata-function-8c083b99-a922-4ca0-af5e-3af55960761f>
  (syntax, `data_field` and `pivot_table` meanings, the up-to-126 field/item pairs and their
  order-independence, brace syntax for multiple items, OLAP hierarchical item names, the
  requirement that dates and times be serial numbers or `DATE`/`TIME` results, the three
  `#REF!` conditions, and the automatic-generation behaviour with its **PivotTable Analyze ▸
  Options ▸ Generate GetPivotData** toggle). Retrieved for this page.
- Handbook `data/battery/FUNC.GETPIVOTDATA.json` — every probe row
  `cannot-call:not-in-reference-catalog`.
- Handbook `data/presence/FUNC.GETPIVOTDATA.json` — no implementing module, hence no family.
- Handbook `content/model/03-call-pipeline.md` (the boundary between function semantics and
  host/workbook obligations).
