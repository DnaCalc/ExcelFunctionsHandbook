---
schema: efh.function-page/v1
function_id: FUNC.CHOOSECOLS
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
family: dynamic_array_reshape_family
role_in_family: >-
  Projects an array onto a chosen list of columns, in the order given; the family's
  column-axis selector and the only reshaper that can duplicate and reorder in one step.
---

## What it computes

`CHOOSECOLS(array, col_num1, [col_num2], …)` builds a new array whose columns are the columns of
`array` named by the index arguments, **in the order the indices are given**. It is a projection
in the relational sense: choose columns, keep every row.

Three properties follow from "in the order given" and are the reason the function exists at all:

1. **It reorders.** `CHOOSECOLS(A, 3, 1, 2)` is a column permutation.
2. **It duplicates.** An index may repeat; `CHOOSECOLS(A, 1, 1)` yields a two-column array whose
   columns are equal. The output column count is the number of indices, not the number of
   distinct indices.
3. **It projects to fewer or more columns than the input has.** Fewer by omission, more by
   repetition.

Row count is invariant: the result has exactly as many rows as `array`. Only the column axis is
touched.

Indices are one-based, and Microsoft documents that a **negative index counts from the end** of
the array — `-1` is the last column, `-2` the second to last. There is no zero index; zero is an
error, which is the natural consequence of one-based counting from both ends.

## Arguments

| Argument | Meaning |
|---|---|
| `array` | The array or range whose columns are to be selected. Required. |
| `col_num1` | The first column to return. One-based; negative counts from the last column. Required. |
| `col_num2`, … | Further columns, in output order. Optional, up to the arity ceiling of 255 arguments. |

An index argument may itself be an array — `CHOOSECOLS(A, {1,3,5})` — which is how you select a
computed set of columns without writing them out. The interaction between an array-valued index
argument and several index arguments (`CHOOSECOLS(A, {1,2}, 4)`) is a shape question the
Handbook has not settled.

The position readers misjudge is the *first*: `array` is a single argument, so a multi-area
reference or a comma-separated list of ranges does not work here the way it does for `HSTACK`.
If you want to combine several sources and then select, stack first, select second.

## Result and edge cases

Return kind: `Array`, of shape (rows of `array`) × (number of index values). The result spills
in dynamic-array Excel; the shared spill and adaptation rules are the host-side concern
described in [the call pipeline](../model/03-call-pipeline.md), not function semantics.

- **A scalar or 1×1 `array`.** One column exists, so `CHOOSECOLS(x, 1)` is defined; anything
  else is out of range.
- **A reference argument.** `array` is prepared under `ValuesOnlyPreAdapter`: the reference is
  resolved to values before the function runs, so the result is an array of values and carries
  no reference structure. Consequently `CHOOSECOLS` cannot be used where a reference is
  required.
- **Errors inside the array.** Element errors are values; a selected column carrying `#N/A`
  yields a result column carrying `#N/A`. Nothing collapses — the projection records
  `error_collapse_profile: None`.
- **Mixed types.** No coercion is applied to the selected elements; text, logicals and errors
  pass through as themselves.

## Errors

Microsoft's page documents the error condition directly: `CHOOSECOLS` returns `#VALUE!` if any
`col_num` argument is zero or exceeds the number of columns in `array`. By the symmetry of the
documented negative indexing, an index more negative than the column count is out of range on
the same rule.

Beyond that, an error value supplied as an *argument* (rather than sitting inside the array)
propagates under the universal rule in
[coercion and lifting](../model/02-coercion-and-lifting.md).

Whether a fractional index is truncated or rejected is not established here.

## Relationships

- **`CHOOSEROWS`** is the same function on the other axis, with the same negative-index rule and
  the same error condition. The pair is symmetric by construction.
- **`TAKE` and `DROP`** select a *contiguous* block from an edge. `CHOOSECOLS` selects an
  arbitrary list. If your selection is "the first three columns", `TAKE` says it more directly
  and does not require you to enumerate.
- **`INDEX` with `row_num` = 0** returns an entire column and is the pre-dynamic-array way to do
  a single-column projection. `CHOOSECOLS` generalizes it to many columns in one call and, unlike
  `INDEX`, does not return a reference.
- **`HSTACK`** is the inverse operation in spirit: `CHOOSECOLS` splits columns out of one array,
  `HSTACK` joins columns from several.
- **`FILTER`** selects along an axis too, but by predicate rather than by index, and it filters
  rows or columns according to the shape of its `include` argument.
- **`CHOOSE`** shares a prefix and is unrelated: it selects among *arguments*, not among columns
  of an array.

## Notes for implementers

- **Output width is the index count, not the distinct-index count.** Any implementation that
  deduplicates or sorts the index list silently changes the answer. This is the easiest bug to
  write and the hardest to notice, because the common case has distinct ascending indices.
- **Negative indices must be resolved against the input width before bounds checking**, and the
  bounds check must reject zero. The three cases — positive, negative, zero — are one branch
  each; folding zero into either sign's path produces an off-by-one.
- **The whole family shares one module** in the reference implementation
  (`dynamic_array_reshape_family.rs`, shared with `CHOOSEROWS`, `DROP`, `EXPAND`, `FILTER`,
  `SORT`, `SORTBY`, `TAKE`, `TOCOL`, `TOROW`, `TRANSPOSE`, `UNIQUE`, `VSTACK`, `WRAPCOLS`,
  `WRAPROWS`). Shared code means a shared shape-and-bounds discipline, and it means a test that
  passes here may be exercising machinery the sibling functions also depend on.
- **Empty results are not representable.** There is no zero-column array in the value model, so
  a selection that yields nothing has to become an error rather than an empty result. That is
  the structural reason the zero index is an error rather than a no-op.

## What has not been checked

No Handbook vector suite exists for `CHOOSECOLS`, and no Excel-comparison evidence record is
recorded for it. Nobody has checked this function against Excel here.

First probes, chosen because each answers a question the documentation leaves open:

1. **An array-valued index argument**, alone (`CHOOSECOLS(A, {1,3})`) and mixed with scalar
   index arguments (`CHOOSECOLS(A, {1,2}, 4)`). The documented signature is a variadic list of
   scalars; what an array in one of those slots does — flatten, lift, or error — is the biggest
   open question here.
2. **Negative indices at the boundary**: `-1`, `-n`, and `-(n+1)` for an `n`-column array,
   plus a mix of positive and negative indices in one call.
3. **Fractional and text indices** (`1.7`, `"2"`), to establish truncation and coercion
   behaviour on the index lane.
4. **Repeated indices**, to confirm the duplication described above is Excel's behaviour and
   not merely the obvious reading.
5. **A single-cell `array`** and a **whole-column reference**, which stress the two extreme
   input shapes.

Item 1 is the one that would change the "Arguments" section rather than extend the edge-case
list.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| projection | Selecting a subset of columns while keeping all rows |
| negative index | An index counted from the last column, `-1` being the last |
| index list | The variadic `col_num` arguments, in output order |
| reshaper | A dynamic-array function that changes an array's shape without changing its elements |

## Sources

- Microsoft, CHOOSECOLS function —
  <https://support.microsoft.com/en-us/office/choosecols-function-bf117976-2722-4466-9b9a-1c01ed9aebff>
  (column selection semantics, negative indexing from the end, and the `#VALUE!` condition for
  zero or out-of-range indices).
- Handbook `content/model/01-value-universe.md` (array kind; errors as values).
- Handbook `content/model/02-coercion-and-lifting.md` (error propagation at the argument
  boundary).
- Handbook `content/model/03-call-pipeline.md` (`ValuesOnlyPreAdapter`; host-side spill
  adaptation).
- Handbook `data/functions/FUNC.CHOOSECOLS.json` and `data/presence/FUNC.CHOOSECOLS.json`
  (arity, classification axes, the shared reshape-family module).
