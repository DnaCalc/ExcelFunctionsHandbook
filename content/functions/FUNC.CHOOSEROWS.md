---
schema: efh.function-page/v1
function_id: FUNC.CHOOSEROWS
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
  Selects an ordered list of rows from an array; the row-axis twin of CHOOSECOLS and the
  family's usual tool for reordering, sampling and repeating records.
---

## What it computes

`CHOOSEROWS(array, row_num1, [row_num2], …)` builds a new array from the rows of `array` named
by the index arguments, **in the order the indices are given**. Column count is invariant; only
the row axis is touched.

Because the output is driven by the index list rather than by a range, the function does three
jobs at once:

1. **selection** — keep the rows you name, drop the rest;
2. **reordering** — `CHOOSEROWS(A, 3, 1, 2)` permutes;
3. **repetition** — an index may appear more than once, and the output row count is the number
   of indices, not the number of distinct indices.

Indices are one-based. Microsoft documents that a **negative index counts from the end**: `-1`
is the last row. There is no zero index.

The everyday uses follow directly: `CHOOSEROWS(A, 1)` takes a header row, `CHOOSEROWS(A, -1)`
takes the last record, and `CHOOSEROWS(A, SEQUENCE(…))` takes a computed sample — the last of
which depends on how the function treats an array-valued index argument, which is the open
question flagged below.

## Arguments

| Argument | Meaning |
|---|---|
| `array` | The array or range whose rows are selected. Required. |
| `row_num1` | The first row to return. One-based; negative counts from the last row. Required. |
| `row_num2`, … | Further rows, in output order. Optional, to the arity ceiling of 255 arguments. |

The argument position most often misjudged is the index list's *ordering* semantics: readers
reach for `CHOOSEROWS` expecting a filter (which preserves original order) and get a permutation
(which does not). If you want "the rows where a condition holds, in their original order",
`FILTER` is the function; `CHOOSEROWS` gives you exactly the order you asked for, including an
order you did not mean to ask for.

## Result and edge cases

Return kind: `Array`, of shape (number of index values) × (columns of `array`). It spills;
spilling itself is host-side adaptation, described in
[the call pipeline](../model/03-call-pipeline.md).

- **References are resolved first.** `array` is prepared under `ValuesOnlyPreAdapter`, so the
  result is values, not a reference into the original range. Nothing downstream can treat the
  result as a range.
- **Element errors pass through.** A selected row carrying an error value produces a result row
  carrying it; the projection records `error_collapse_profile: None`, so nothing folds.
- **No coercion of selected elements.** Text stays text, logicals stay logicals.
- **A single-row `array`** admits only index 1 (or `-1`); everything else is out of range.

## Errors

Microsoft's page documents the failure directly: `CHOOSEROWS` returns `#VALUE!` if any `row_num`
argument is zero or exceeds the number of rows in `array`; the same bound applies on the
negative side by the symmetry of the documented indexing.

An error value supplied as an argument — as opposed to sitting inside `array` — propagates under
the universal coercion rule in
[coercion and lifting](../model/02-coercion-and-lifting.md).

Whether a fractional index truncates or errors is not established here.

## Relationships

- **`CHOOSECOLS`** is the identical operation on the column axis. Anything established for one
  should be probed for the other rather than assumed, but the pair is symmetric by
  construction and shares an implementation module.
- **`TAKE` / `DROP`** handle contiguous edge selections — "first 5 rows", "all but the last 2" —
  without enumerating indices, and express intent better when the selection is contiguous.
- **`FILTER`** selects rows by predicate and preserves their original order. Use it when the
  criterion is a condition; use `CHOOSEROWS` when the criterion is a position.
- **`SORT` / `SORTBY`** also produce a permutation of rows, but derive it from the data rather
  than from an explicit index list. `SORTBY` with a computed key and `CHOOSEROWS` with a
  computed index list are two routes to the same reordering.
- **`INDEX` with `column_num` = 0** returns an entire row and is the pre-dynamic-array idiom
  `CHOOSEROWS` supersedes for multi-row selection.
- **`VSTACK`** is the joining counterpart: `CHOOSEROWS` takes rows out, `VSTACK` puts rows
  together.
- **`CHOOSE`** shares only a prefix. It selects among arguments, not among rows.

## Notes for implementers

- **Do not normalize the index list.** Sorting or deduplicating it changes the result. The
  common case (distinct ascending indices) hides the bug completely.
- **Three index branches, not two**: positive, negative, and zero-is-an-error. Negative indices
  must be resolved against the input height before the bounds check.
- **Row selection is a gather, not a slice.** Implementations that assume contiguity — because
  the common calls are contiguous — will pass their tests and fail on `CHOOSEROWS(A, 3, 1)`.
- **The module is shared** with the rest of the dynamic-array reshapers
  (`dynamic_array_reshape_family.rs`), so bounds-and-shape logic is common code; a fix here is a
  fix for the siblings, and a regression here is a regression for them.

## What has not been checked

No Handbook vector suite exists for `CHOOSEROWS`, and no Excel-comparison evidence record is
recorded for it. Nobody has checked this function against Excel here.

Probes worth running first:

1. **An array-valued index argument** — `CHOOSEROWS(A, SEQUENCE(3))` and
   `CHOOSEROWS(A, {1;3})`. This is the difference between "a variadic list of scalars" and "an
   index vector", and it decides whether the computed-sample idiom above is real.
2. **Boundary indices** for an `n`-row array: `1`, `n`, `n+1`, `0`, `-1`, `-n`, `-(n+1)`, and a
   positive/negative mix in one call.
3. **Repeated indices**, confirming that the output row count follows the index count.
4. **Fractional and numeric-text indices**, to pin truncation and coercion on the index lane.
5. **A one-row array and a whole-column reference**, the two degenerate input shapes.

Item 1 is the one whose answer would rewrite the "Arguments" section.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| gather | Selecting rows by an arbitrary index list rather than a contiguous slice |
| permutation | A reordering of rows induced by the index list |
| negative index | An index counted from the last row, `-1` being the last |
| reshaper | A dynamic-array function that changes shape without changing elements |

## Sources

- Microsoft, CHOOSEROWS function —
  <https://support.microsoft.com/en-us/office/chooserows-function-51ace882-9bab-4a44-9625-7274ef7507a3>
  (row selection semantics, negative indexing from the end, and the `#VALUE!` condition for zero
  or out-of-range indices).
- Handbook `content/model/02-coercion-and-lifting.md` (error propagation at the argument
  boundary).
- Handbook `content/model/03-call-pipeline.md` (`ValuesOnlyPreAdapter`; host-side spill
  adaptation).
- Handbook `data/functions/FUNC.CHOOSEROWS.json` and `data/presence/FUNC.CHOOSEROWS.json`
  (arity, classification axes, the shared reshape-family module).
