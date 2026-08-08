---
schema: efh.function-page/v1
function_id: FUNC.DROP
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
  Removes a contiguous band of rows or columns from an array's edge; the family's
  complement to TAKE, and the usual way to shed a header row.
---

## What it computes

`DROP(array, rows, [columns])` removes a contiguous band from the **edge** of an array and
returns what is left. The sign of each count chooses the edge:

- `rows` positive — remove that many rows from the **top**;
- `rows` negative — remove that many rows from the **bottom**;
- `columns` positive — remove that many columns from the **left**;
- `columns` negative — remove that many columns from the **right**.

Both axes are trimmed in the same call when both arguments are given; the result is the
rectangular remainder. Zero removes nothing on that axis, and omitting `columns` leaves the
column axis untouched.

`DROP` is the exact complement of `TAKE`: for a non-negative `n`, `TAKE` keeps the band `DROP`
removes. Because both use the sign convention for the edge, the pair covers all four
"first/last n rows/columns" operations between them without a mode argument.

## Arguments

| Argument | Meaning |
|---|---|
| `array` | The array or range to trim. Required. |
| `rows` | Number of rows to remove; positive from the top, negative from the bottom. Required. |
| `columns` | Number of columns to remove; positive from the left, negative from the right. Optional. |

**A projection discrepancy worth naming.** The Handbook's projected signature for `DROP` (in
`data/functions/FUNC.DROP.json`) displays `DROP(array, rows)` while the projected arity is 2 to
3. Microsoft's page documents the third argument, `[columns]`. The projected signature display
is therefore incomplete rather than authoritative, and the arity is the field that agrees with
the documentation. This is recorded here rather than silently corrected, because the projection
is a mechanically synced organ and its disagreements are data.

The argument most often misused is `rows` with the wrong sign: "drop the last row" is `-1`, not
`1`, and the mistake is silent — it returns a plausible array of the right shape from the wrong
end.

## Result and edge cases

Return kind: `Array`. The result spills; spilling is host-side adaptation, not function
semantics ([the call pipeline](../model/03-call-pipeline.md)).

- **References are resolved first.** `array` is prepared under `ValuesOnlyPreAdapter`, so the
  result carries values, not reference structure.
- **`rows` and `columns` lift.** The projection records
  `by_index_scalar_array_lift(positions=1|2)` — argument positions 1 and 2, that is `rows` and
  `columns`, are broadcast positions. Handing `DROP` an array of counts is therefore meaningful
  in the pipeline's terms, though the resulting shape composition is not established here.
- **Dropping everything.** If `rows` is at least the array's height (or `columns` at least its
  width), nothing remains. There is no empty array in the published value model
  ([value universe](../model/01-value-universe.md)), so this case has to become an error rather
  than an empty result — see below.
- **Element errors** inside the retained region pass through unchanged; the projection records
  `error_collapse_profile: None`.
- **Fractional counts.** Whether `2.7` truncates or errors is not established here.

## Errors

Microsoft's page for `DROP` documents `#VALUE!` for an invalid argument and `#CALC!` for the
case where the result would be an empty array. `#CALC!` is the calculation engine's "cannot
produce a value" code, and the empty-array case is one of the two canonical producers of it
listed in [the value universe](../model/01-value-universe.md) — the other being the uncalled
lambda.

That pairing is the thing to remember about `DROP`: over-dropping does not give you nothing, it
gives you `#CALC!`. Formulas that compute a drop count from data must either guarantee the count
is smaller than the extent or wrap the call in `IFERROR`.

An error value supplied as an argument propagates under the universal rule in
[coercion and lifting](../model/02-coercion-and-lifting.md).

## Relationships

- **`TAKE`** is the complement: `DROP(a, n)` and `TAKE(a, n)` partition the array's rows. Which
  one reads better depends on whether you are naming what you keep or what you discard.
- **`CHOOSEROWS` / `CHOOSECOLS`** handle non-contiguous selections. If the selection is
  "everything except the first row", `DROP` says it in one argument where `CHOOSEROWS` would
  need an index vector.
- **`FILTER`** removes rows by predicate rather than by position, and returns `#CALC!` on an
  empty result for the same structural reason — unless its `if_empty` argument is supplied.
  `DROP` has no `if_empty`.
- **`OFFSET`** was the pre-dynamic-array way to skip a header row, and it is volatile; `DROP`
  is not (`volatility_class: NonVolatile` in the projection). Replacing `OFFSET` with `DROP`
  removes a recalculation dependency as well as an argument.
- **`EXPAND`** is the growing counterpart to this shrinking one.

## Notes for implementers

- **The empty result is the whole design problem.** Every implementation needs a decision point
  where "the remainder has zero rows or zero columns" becomes `#CALC!` rather than an
  ill-formed array. Getting this wrong produces an array with a zero dimension that then
  corrupts whatever consumes it, and the failure surfaces far from the cause.
- **Sign handling before bounds checking.** Convert the signed count into a (start, length) pair
  first, then range-check. Mixing the two produces off-by-ones at exactly `n = height`.
- **Both axes in one pass.** Trimming rows then columns and trimming columns then rows must give
  the same rectangle — they do, for a rectangular remainder — but an implementation that
  materializes an intermediate array between the two steps doubles its memory traffic on large
  ranges for no benefit.
- **The lift positions are declared, not incidental.** `by_index_scalar_array_lift(positions=1|2)`
  means the dispatch layer, not the kernel, broadcasts those arguments. An implementation that
  handles arrays inside its own kernel for those positions will diverge from the declared
  pipeline even where the results agree.
- The module is shared with the rest of the dynamic-array reshapers, so the shape-and-bounds
  discipline is common code.

## What has not been checked

No Handbook vector suite exists for `DROP`, and no Excel-comparison evidence record is recorded
for it. Nobody has checked this function against Excel here.

Probes worth running first:

1. **Exact-boundary drops**: `rows` equal to the array's height, and one more. This pins the
   `#CALC!` boundary, which is the function's most consequential edge.
2. **Negative counts on both axes**, including `DROP(a, -1, -1)`, confirming the sign-to-edge
   mapping in both directions rather than inferring the second from the first.
3. **Zero counts** on each axis and both, confirming that zero is a no-op rather than an error —
   the sign convention leaves zero ambiguous in principle.
4. **Array-valued `rows` or `columns`**, testing the declared lift positions and, more
   importantly, what shape comes back.
5. **Fractional and numeric-text counts**, to pin truncation and coercion on the count lane.
6. **A single-cell `array`** with each of `rows` = 0, 1, and −1.

Item 1 is the one that would change how the function must be used in practice.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| band | The contiguous rows or columns removed from an edge |
| sign convention | Positive counts from the start of an axis, negative from the end |
| empty array | A result with a zero dimension; not representable, surfaces as `#CALC!` |
| lift position | An argument index the dispatch layer broadcasts over arrays |

## Sources

- Microsoft, DROP function —
  <https://support.microsoft.com/en-us/office/drop-function-1cb4e151-9e17-4838-abe5-9ba48d8c6a34>
  (the `rows` and `columns` arguments, the negative-from-the-end convention, and the documented
  `#VALUE!` and `#CALC!` conditions).
- Handbook `content/model/01-value-universe.md` (`#CALC!`; no empty array at the published
  boundary).
- Handbook `content/model/02-coercion-and-lifting.md` (error propagation at the argument
  boundary).
- Handbook `content/model/03-call-pipeline.md` (`ValuesOnlyPreAdapter`;
  `ByIndexScalarArrayLift`; host-side spill adaptation).
- Handbook `data/functions/FUNC.DROP.json` and `data/presence/FUNC.DROP.json` (arity, the
  incomplete projected signature display, classification axes, shared reshape-family module).
