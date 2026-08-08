---
schema: efh.function-page/v1
function_id: FUNC.HSTACK
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
family: hstack
role_in_family: >-
  Concatenates arrays side by side into one wider array, padding short columns; the
  horizontal half of the stacking pair with VSTACK.
---

## What it computes

`HSTACK(array1, [array2], …)` places its arguments side by side, left to right in the order
given, and returns the combined array.

The result's **width** is the sum of the arguments' widths. Its **height** is the height of the
tallest argument — and every argument shorter than that is padded at the bottom. Microsoft
documents the pad value as `#N/A`, which is the same convention `VSTACK` uses and the same
default `EXPAND` uses: synthesized cells announce themselves rather than passing as zeros.

That padding rule is what makes `HSTACK` a *stacking* operation rather than a join. It does not
align rows by any key; it aligns them by position, starting at row 1. Two lists of different
lengths stacked horizontally give you a two-column array whose shorter column trails off into
`#N/A`, which is correct and is very often not what the author meant.

## Arguments

| Argument | Meaning |
|---|---|
| `array1` | The first array or range. Required. |
| `array2`, … | Further arrays, in left-to-right order. Optional, to the arity ceiling of 255 arguments. |

Scalars are admissible: a scalar is a 1×1 array, so `HSTACK(A1:A10, "total")` produces a
two-column result whose second column is one value and nine `#N/A`s.

The misunderstanding to head off: **`HSTACK` takes several arguments, not a multi-area
reference.** `HSTACK((A1:A5,C1:C5))` passes one union-shaped argument, which is a different
thing from `HSTACK(A1:A5, C1:C5)`. What the union form does is not established here.

## Result and edge cases

Return kind: `Array`, spilled. Spilling is host-side adaptation
([the call pipeline](../model/03-call-pipeline.md)).

- **References are resolved first** (`ValuesOnlyPreAdapter`), so the result carries values, not
  reference structure. `HSTACK` cannot be used where a reference is required.
- **A single argument** returns that argument as an array — the identity case, and a useful way
  to force a reference into array form.
- **Element kinds are preserved**: no coercion is applied to the stacked elements, and errors
  inside the inputs pass through as values (`error_collapse_profile: None`).
- **Ragged inputs are the normal case, not an error.** Padding is the documented behaviour;
  there is no strict mode.
- **Whether a whole-column reference is admissible** as an argument, and what height it
  contributes, is not established here — it is the input most likely to interact badly with the
  "height of the tallest argument" rule.

## Errors

Microsoft's page for `HSTACK` documents `#N/A` as the *pad value*, which is a value in the
result rather than a failure of the call. The page does not publish a general error table.
`#VALUE!` for an inadmissible argument is the family's conventional code and is recorded here as
expected, not verified.

An error value supplied as an argument propagates under the universal rule in
[coercion and lifting](../model/02-coercion-and-lifting.md) — as distinct from an error sitting
*inside* an argument array, which is stacked like any other value.

A result exceeding the grid's dimensions is a host limit rather than a function semantic; what
Excel returns at that limit is not established here.

## Relationships

- **`VSTACK`** stacks vertically: widths are padded instead of heights, and the result's height
  is the sum. The pair is symmetric, and `HSTACK(VSTACK(…), …)` compositions are how
  dynamic-array formulas assemble tables.
- **`EXPAND`** is how you control the padding rather than accept `#N/A`: expand each piece to the
  common height with your own pad value first, then stack.
- **`CHOOSECOLS`** is the inverse in spirit — it takes columns out of one array where `HSTACK`
  puts columns from several together.
- **`TOROW` / `TOCOL`** flatten rather than stack, and `WRAPROWS` / `WRAPCOLS` reshape a vector
  into a rectangle; all four answer the neighbouring question of "how do I get this data into
  that shape".
- **The `&` operator and `TEXTJOIN`** concatenate *values*; `HSTACK` concatenates *arrays*. The
  word "concatenate" pulls readers toward the wrong one.
- **`INDEX` / `OFFSET` gymnastics** were how workbooks assembled multi-source arrays before the
  stacking functions existed; `HSTACK` replaces most of that with a call whose intent is legible.

## Notes for implementers

- **The pad is per-argument, not per-result.** Each argument is padded to the common height
  independently; there is no single rectangle being filled. Implementations that build the
  output row-major must know each source's height to decide where padding begins in that row.
- **Width is a sum and can overflow the grid.** Validate against the host's column limit before
  allocating, and decide what the failure looks like — the Handbook has not established Excel's
  behaviour at the boundary.
- **Preserve element kinds exactly.** Stacking is a copy, not a conversion; text, logicals and
  errors must arrive unchanged. This is the property that lets `HSTACK` build heterogeneous
  tables, and the easiest one to lose in an implementation that types its buffers numerically.
- **The identity case matters.** `HSTACK(x)` with a single argument is a legitimate call — the
  arity minimum is 1 — and it is used deliberately to materialize a reference as an array. It
  must not be optimized into a pass-through that preserves reference-ness.
- This function has its own module in the reference implementation rather than sharing the
  reshape family's, which is a code-organization fact, not a behavioural one.

## What has not been checked

No Handbook vector suite exists for `HSTACK`, and no Excel-comparison evidence record is
recorded for it. Nobody has checked this function against Excel here.

Probes worth running first:

1. **Ragged stacking**, verified rather than assumed: a 5-row and a 3-row argument, confirming
   the pad value is `#N/A` and that padding is at the bottom.
2. **Mixed shapes including scalars** — `HSTACK(A1:A5, 1, B1:C2)` — which exercises the
   per-argument padding rule with three different heights in one call.
3. **A whole-column reference** as one argument, which tests whether the height rule is taken
   from the reference's declared extent or from its used range. The two answers differ by
   roughly a million rows.
4. **A multi-area reference** as a single argument, against the multi-argument form.
5. **Element kinds**: an argument containing text, a logical, an error and a blank, confirming
   each arrives unchanged and that blanks normalize as the value model describes.
6. **The width limit**: enough arguments to exceed the grid's column count.

Item 3 is the one most likely to produce a surprise, and item 1 the one that must be right for
any of the rest to matter.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| stacking | Positional concatenation of arrays without key alignment |
| pad value | The value filling cells below a short argument; documented as `#N/A` |
| ragged input | Arguments of differing heights, the normal case for `HSTACK` |
| identity case | A single-argument call, used to materialize a reference as an array |

## Sources

- Microsoft, HSTACK function —
  <https://support.microsoft.com/en-us/office/hstack-function-98c4ab76-10fe-4b4f-8d5f-af1c125fe8c2>
  (horizontal appending in sequence, and `#N/A` padding for arguments shorter than the tallest).
- Handbook `content/model/01-value-universe.md` (array kind; errors as values; blank
  normalization at the published boundary).
- Handbook `content/model/02-coercion-and-lifting.md` (error propagation at the argument
  boundary).
- Handbook `content/model/03-call-pipeline.md` (`ValuesOnlyPreAdapter`; host-side spill
  adaptation).
- Handbook `data/functions/FUNC.HSTACK.json` and `data/presence/FUNC.HSTACK.json` (arity,
  classification axes, implementing module).
