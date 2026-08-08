---
schema: efh.function-page/v1
function_id: FUNC.TOCOL
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
  - Page vocabulary
  - Sources
family: dynamic_array_reshape_family
role_in_family: >-
  Flattens a rectangular array into a single column, in row-major or column-major order, with
  optional removal of blanks and errors.
---

## What it computes

`TOCOL` flattens. It takes an `m × n` array and returns a single column containing its
elements in a chosen traversal order, optionally dropping some of them.

The traversal is the substantive parameter. With `scan_by_column` false or omitted, the array
is read **by row** — left to right along row 1, then row 2, and so on, which is row-major
order. With `scan_by_column` true, it is read **by column** — top to bottom down column 1, then
column 2. For the array `{1,2;3,4}` those give `{1;2;3;4}` and `{1;3;2;4}` respectively.

The `ignore` argument then removes categories of element before the column is assembled:

| `ignore` | Documented meaning |
|---|---|
| `0` | Keep all values (default) |
| `1` | Ignore blanks |
| `2` | Ignore errors |
| `3` | Ignore blanks and errors |

The result length is therefore `m · n` minus whatever was dropped — the one place in this
family where the output size is not a pure function of the input's shape.

`TOCOL(range, 1)` is, in practice, the modern answer to "give me the non-empty values of this
rectangle as a list", which is why it turns up in `UNIQUE`, `SORT` and `TEXTJOIN` pipelines far
more often than its documentation suggests.

## Arguments

Microsoft documents `TOCOL(array, [ignore], [scan_by_column])`.

**`array`** — required. The array or reference to flatten.

**`ignore`** — optional, default `0`. Takes the four values above. Note that it is a small
integer code, not a logical: `TOCOL(A1:C9, TRUE)` coerces to `1` and means "ignore blanks",
which happens to be a reasonable request but is not what the writer of `TRUE` was thinking.

**`scan_by_column`** — optional, default `FALSE`. A logical. When omitted or `FALSE`, the array
is scanned by row; when `TRUE`, by column.

The two optional arguments are the ones readers swap, because both are small and both are
"options". `TOCOL(A, 1)` ignores blanks; `TOCOL(A, , 1)` scans by column. The first is a
filter, the second a traversal order, and neither is an error, so a swap produces a
correct-looking column with the wrong contents or the wrong order.

## Result and edge cases

The return kind is a single-column array.

- **The result can be empty.** `TOCOL(range, 3)` over a range of only blanks and errors leaves
  nothing. The Excel convention for an empty array result is `#CALC!`
  ([the value universe](../model/01-value-universe.md)); the Handbook has not verified that
  `TOCOL` uses it.
- **What counts as "blank".** Whether `ignore` `1` drops empty cells only, or also empty text
  `""`, is the question this function turns on in practice — a column produced by a formula
  chain is usually full of `""`, not of empty cells. The documentation says "blanks" and does
  not disambiguate; the Handbook has not verified it.
- **Ignoring errors removes them entirely**; it does not replace them. The result is shorter,
  which means positions no longer correspond to the source.
- **A single-element result stays an array.** See the `TAKE` page for the probes that make that
  distinction observable.
- Dynamic-array publication and `#SPILL!` are host-side adaptation, not `TOCOL` semantics.

## Errors

Documented by Microsoft on the `TOCOL` page:

| Error | Documented condition |
|---|---|
| `#VALUE!` | An array constant containing one or more numbers that are not whole numbers |
| `#NUM!` | `array` is too large |

The first of those is worth flagging as unusual: it is stated on Microsoft's page, and it does
not obviously describe a flattening operation, whose result should not care whether its
elements are integers. The Handbook records it as documented and does not attempt to explain
it; confirming what it actually refers to is one of the open probes below.

## Relationships

- **`TOROW`** is the transpose: the same flattening with the same `ignore` and
  `scan_by_column` vocabulary, producing a single row. `TOCOL(x, i, s)` equals
  `TRANSPOSE(TOROW(x, i, s))`.
- **`WRAPCOLS` / `WRAPROWS`** are the inverse: a vector back to a rectangle.
- **`TRANSPOSE`** exchanges the axes without flattening.
- **`FILTER`** removes whole rows by predicate; `TOCOL`'s `ignore` removes individual elements
  by kind. They are not substitutes.
- **`TRIMRANGE`** removes the blank *margin* of a range; `TOCOL(…, 1)` removes blanks
  everywhere, including interior ones, and destroys the two-dimensional structure while doing
  it.
- **`VSTACK`** concatenates while preserving structure; flattening a stack is a common
  composition (`TOCOL(VSTACK(a, b), 1)`).

## Notes for implementers

- Two orthogonal parameters: what to skip, and in what order to walk. Implementing `ignore` as
  a post-filter on an already-built column is correct only if the traversal order was applied
  first — the two commute here, but only because filtering is order-preserving.
- `ignore` is a code, not a flag. Values outside `0..3` need a decided outcome.
- The blank definition — empty cell versus empty string versus a formula returning `""` — must
  be stated. It is the single most consequential ambiguity in this function.
- An empty result is a real case with a specific outcome, not an edge to leave undefined.
- Scalars and 1×1 arrays flatten to a one-element column, which must stay array-shaped.

## What has not been checked

No Handbook vector suite exists for `TOCOL`, and no Handbook evidence record is attached to
this page. Nothing here claims agreement with Excel for any implementation.

First probes:

1. **The blank definition.** A rectangle containing an empty cell, a cell holding `""`, a
   formula returning `""`, a space, and a zero, under `ignore` `1` and `3`. This decides how
   the function behaves on every real pipeline.
2. **Traversal order** on a non-square array, under both `scan_by_column` settings — the case
   where row-major and column-major visibly differ.
3. **The documented `#VALUE!` condition**, to find out what "an array constant containing
   numbers that are not whole numbers" actually refers to; as written it does not match the
   function's semantics and may be inherited boilerplate.
4. **The empty result** under `ignore` `3`.
5. **`ignore` values outside `0..3`**, including negatives, non-integers and logicals.
6. **Size limits**, against the documented `#NUM!`.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| flattening | Reducing a rectangle to a vector by a chosen traversal |
| row-major / column-major | The two traversal orders selected by `scan_by_column` |
| ignore code | The `0`–`3` value selecting which element kinds are dropped |
| blank | Ambiguous between an empty cell and empty text; unresolved on this page |

## Sources

- Microsoft, *TOCOL function* —
  <https://support.microsoft.com/en-us/office/tocol-function-22839d9b-0b55-4fc1-b4e6-2761f8f122ed>
  (syntax, the four `ignore` values, the `scan_by_column` default and meaning, and the
  documented `#VALUE!` and `#NUM!` conditions). Retrieved for this page.
- Handbook `content/model/01-value-universe.md` (the `Empty` kind and the `#CALC!` convention)
  and `content/model/03-call-pipeline.md`.
- Handbook `data/functions/FUNC.TOCOL.json` (signature, arity, classification axes).
