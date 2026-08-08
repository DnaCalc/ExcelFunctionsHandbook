---
schema: efh.function-page/v1
function_id: FUNC.COLUMNS
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
family: columns_fn
role_in_family: >-
  Measures the width of an array or reference; the column-axis counterpart of ROWS and the
  usual source of the width argument for reshaping and lookup formulas.
---

## What it computes

`COLUMNS(array)` returns the number of columns in an array or reference. It is a measurement of
shape, not of content: no cell is read, no value is coerced, and empty cells inside the range
count exactly as much as filled ones.

The distinction worth stating at the top, because the names invite the mistake: `COLUMNS` counts
columns, `COLUMN` locates one. `COLUMNS(C1:E1)` is 3; `COLUMN(C1:E1)` is the array 3, 4, 5.

`COLUMNS` is reference-aware — the projection records
`arg_preparation_profile: RefsVisibleInAdapter` — which lets it measure a reference's declared
extent rather than the shape of some materialized copy of its values. For a whole-column
reference or a large range that matters: the answer is available from the reference's geometry
without touching the grid.

## Arguments

| Argument | Meaning |
|---|---|
| `array` | An array, an array formula's result, or a reference to a range of cells. Required, exactly one. |

Microsoft's page names this argument `array` and describes it as an array, an array formula, or
a reference to a range of cells. Two consequences readers trip over:

- **A single cell is a 1×1 shape**, so `COLUMNS(A1)` is 1 rather than an error.
- **A multi-area reference** — `COLUMNS((A1:B2,D1:E2))` — has no single width. What `COLUMNS`
  reports for a union is not established here, and it is not obvious: the plausible answers
  (the first area's width, the bounding box's width, an error) are all defensible.

What `COLUMNS` does with a bare scalar that is neither array nor reference is likewise not
established here.

## Result and edge cases

Return kind: `Number` — a positive integer.

- **No coercion of contents.** Element kinds are irrelevant. An array of errors has a width.
- **Nothing is scanned.** Unlike `COUNT`-style functions, `COLUMNS` does not visit elements, so
  an error inside the range does not become the result. This is the practical difference between
  a shape query and an aggregate: see the scan-policy discussion in
  [coercion and lifting](../model/02-coercion-and-lifting.md).
- **A spill anchor** (`B1#`) has a width that changes when the spill changes; `COLUMNS(B1#)` is
  the idiomatic way to ask how wide a dynamic result currently is, and its dependency behaviour
  is a property of the spill-anchor reference shape rather than of this function.
- **A structured reference** to several table columns has a width that tracks the table.

## Errors

Microsoft's page does not publish an error table for `COLUMNS`. The expected failure is an
argument that is neither an array nor a reference, for which `#VALUE!` is this family's
conventional code — recorded here as expected, not verified. An error value passed as the
argument propagates under the universal rule in
[coercion and lifting](../model/02-coercion-and-lifting.md); note that this is different from an
error *inside* the array, which does not affect the count.

## Relationships

- **`ROWS`** is the same measurement on the other axis. Together they give an array's shape.
- **`COLUMN`** locates rather than counts, and the name similarity is the standing trap.
- **`AREAS`** counts the rectangles in a reference where `COLUMNS` measures extent. For a
  multi-area reference the two questions come apart, which is why `AREAS` exists.
- **`INDEX`, `OFFSET`, `TAKE`, `DROP`, `WRAPROWS`** all take width or height arguments that are
  frequently computed with `COLUMNS`.
- **`HLOOKUP`'s `row_index_num`** and **`VLOOKUP`'s `col_index_num`** are commonly written as
  `COLUMNS($A$1:B$1)` or similar so that the index advances as the formula is copied — one of
  the oldest idioms in spreadsheet practice, and one that `XLOOKUP` removes the need for.
- **`SEQUENCE(1, COLUMNS(a))`** is the modern way to build an index vector matching an array's
  width.

## Notes for implementers

- **Answer from the reference, not from the values.** Materializing a whole-column reference to
  count its columns is correct and catastrophic. The `RefsVisibleInAdapter` preparation profile
  exists so this function can avoid exactly that.
- **The multi-area case needs a declared policy**, because there is no single right answer. An
  implementation must choose and record its choice rather than fall out of whatever its
  reference representation happens to make easy.
- **1×1 arrays.** The pipeline's argument preparation collapses a single-cell reference to a
  scalar for values-only functions, but `COLUMNS` is not values-only, and the unit-array
  preservation caveat described in [the call pipeline](../model/03-call-pipeline.md) is exactly
  the sort of normalization that can silently change a shape query's answer.
- The projection records `host_interaction_class: WorkbookState`: the function needs to know
  about the workbook's reference geometry even though it reads no cell contents.

## What has not been checked

No Handbook vector suite exists for `COLUMNS`, and no Excel-comparison evidence record is
recorded for it. Nobody has checked this function against Excel here.

Probes, in order of how much they would change this page:

1. **A multi-area reference** — `COLUMNS((A1:B2,D1:F3))`. There is no obvious answer, and the
   answer determines whether `COLUMNS` is a geometric or a first-area measurement.
2. **A bare scalar** — `COLUMNS(5)`, `COLUMNS("x")`, `COLUMNS(TRUE)`. Whether these are 1 or
   `#VALUE!` decides whether the argument is genuinely typed as array-or-reference.
3. **A whole-column reference** and a **three-dimensional reference**, to confirm the count is
   taken from geometry rather than from used range.
4. **A spill anchor before and after the spill's width changes**, which also exercises the
   dependency chain rather than only the value.
5. **An array containing errors**, confirming that shape queries do not propagate element
   errors.

Items 1 and 2 are the two genuine unknowns; the rest are confirmations.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| shape query | A function that reports an array's dimensions without reading its elements |
| extent | The declared width or height of a reference, independent of its contents |
| multi-area reference | A reference built by the union operator from several rectangles |
| spill anchor | The `B1#` reference shape designating a dynamic array's current extent |

## Sources

- Microsoft, COLUMNS function —
  <https://support.microsoft.com/en-us/office/columns-function-4e8e7b4e-e603-43e8-b177-956088fa48ca>
  (argument definition: an array, an array formula, or a reference to a range of cells).
- Handbook `content/model/01-value-universe.md` (array kind; reference shapes).
- Handbook `content/model/02-coercion-and-lifting.md` (shape queries versus scans; error
  propagation).
- Handbook `content/model/03-call-pipeline.md` (`RefsVisibleInAdapter`; unit-array
  preservation).
- Handbook `data/functions/FUNC.COLUMNS.json` and `data/presence/FUNC.COLUMNS.json` (arity,
  classification axes, implementing module).
