---
schema: efh.function-page/v1
function_id: FUNC.TOROW
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
  Flattens a rectangular array into a single row, in row-major or column-major order, with
  optional removal of blanks and errors — TOCOL's transpose.
---

## What it computes

`TOROW` flattens an `m × n` array into a **single row**.

The elements are visited in one of two traversal orders and, optionally, filtered by kind
before being laid out left to right:

- **`scan_by_column` omitted or `FALSE`** — scan by row: row 1 left to right, then row 2, and
  so on (row-major).
- **`scan_by_column` `TRUE`** — scan by column: column 1 top to bottom, then column 2
  (column-major).

For `{1,2;3,4}` those give `{1,2,3,4}` and `{1,3,2,4}`.

The `ignore` argument drops element kinds before assembly, with the same code vocabulary the
whole flattening pair shares: `0` keep everything (default), `1` ignore blanks, `2` ignore
errors, `3` ignore both.

`TOROW` and `TOCOL` are the same operation with the output axis exchanged;
`TOROW(x, i, s)` equals `TRANSPOSE(TOCOL(x, i, s))`. Which one you want is decided by what
consumes the result, not by anything about the input.

## Arguments

Microsoft documents `TOROW(array, [ignore], [scan_by_column])`.

**`array`** — required. The array or reference to flatten.

**`ignore`** — optional, default `0`. Values `0`–`3` as above. It is an integer code, not a
logical, even though `1` and `TRUE` coerce alike.

**`scan_by_column`** — optional, default `FALSE`. A logical selecting the traversal order.

The common confusion is between the two optional positions: `TOROW(A, 1)` filters blanks;
`TOROW(A, , 1)` changes the traversal. Both are legal and neither errors, so the mistake shows
up as a silently reordered or silently shortened row.

A second confusion is specific to this function's name. `TOROW` does **not** mean "read the
array by rows" — that is `scan_by_column` `FALSE`. It means "produce a row". The name describes
the output shape; the traversal is the third argument's business.

## Result and edge cases

The return kind is a single-row array.

- **The result can be empty** when `ignore` removes everything; Excel's convention for an empty
  array result is `#CALC!` ([the value universe](../model/01-value-universe.md)), and the
  Handbook has not verified that `TOROW` uses it.
- **"Blank" is ambiguous** between an empty cell and text of zero length. Which one `ignore`
  `1` drops is the question that decides how the function behaves on formula-generated data,
  and the documentation does not disambiguate it.
- **Ignoring errors shortens the result**; positions no longer correspond to source cells.
- **A one-element result stays an array.**
- Dynamic-array publication and `#SPILL!` are host-side adaptation.

## Errors

Microsoft's `TOCOL` page documents `#VALUE!` for an array constant containing numbers that are
not whole numbers, and `#NUM!` for an array that is too large. `TOROW` is documented as the
twin function and the same conditions are the expected ones; the Handbook has not confirmed
that the `TOROW` page states them identically, and does not assert the first condition's
meaning, which does not obviously describe a flattening operation.

## Relationships

- **`TOCOL`** is the transpose twin, with the identical argument vocabulary.
- **`WRAPROWS` / `WRAPCOLS`** invert the flattening: a vector back to a rectangle. `WRAPROWS`
  is the natural partner of `TOROW` when reshaping `m × n` to `p × q`.
- **`TRANSPOSE`** exchanges axes without flattening.
- **`HSTACK`** builds a row by concatenation rather than by flattening; for single-row inputs
  the two coincide, and for rectangular inputs they do not.
- **`TEXTJOIN(delim, TRUE, TOROW(range, 1))`** is the idiomatic "list these values in a
  sentence" composition.

## Notes for implementers

- The output axis and the traversal axis are independent. Writing `TOROW` as "transpose, then
  `TOCOL`" is correct but changes which traversal flag means what — the composition has to be
  worked out rather than assumed.
- `ignore` is a code with four admitted values; anything else needs a decided outcome.
- The blank definition must be stated explicitly and shared with `TOCOL`.
- Filtering must preserve traversal order.
- One-element results stay array-shaped; see `BUG-FUNC-026` in the reference engine for how
  easily a family-wide scalarization shortcut breaks nested semantics.

## What has not been checked

No Handbook vector suite exists for `TOROW`, and no Handbook evidence record is attached to
this page. Nothing here claims agreement with Excel for any implementation.

First probes:

1. **Traversal order on a non-square array** under both `scan_by_column` settings — the
   distinguishing case, and the one that also confirms the name does not imply the traversal.
2. **The blank definition**: empty cell, `""` literal, formula-produced `""`, a space, and
   zero, under `ignore` `1` and `3`.
3. **`TOROW` against `TRANSPOSE(TOCOL(...))`** on the same inputs, to confirm the twins agree
   — a cheap consistency oracle that needs no external reference.
4. **The empty result** under `ignore` `3`.
5. **`ignore` outside `0..3`**, and array-valued arguments in either optional position.
6. **The documented error conditions**, including whether the `#VALUE!` clause on the sibling
   page applies here at all.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| flattening | Reducing a rectangle to a vector by a chosen traversal |
| output axis | The shape of the result (a row), fixed by the function name |
| traversal axis | The order elements are visited, chosen by `scan_by_column` |
| ignore code | The `0`–`3` value selecting which element kinds are dropped |

## Sources

- Microsoft, *TOROW function* —
  <https://support.microsoft.com/en-us/office/torow-function-b90d0964-a7d9-44b7-816b-ffa5c2fe2289>
  (syntax and argument meanings). Not retrieved for this page; the behaviour above is stated as
  documented behaviour and should be re-checked against the page.
- Microsoft, *TOCOL function* —
  <https://support.microsoft.com/en-us/office/tocol-function-22839d9b-0b55-4fc1-b4e6-2761f8f122ed>
  (the shared `ignore` value table, the `scan_by_column` default, and the documented `#VALUE!`
  and `#NUM!` conditions). Retrieved for this page and used as the twin function's reference.
- Handbook `content/model/01-value-universe.md` and `content/model/03-call-pipeline.md`.
- Handbook `data/functions/FUNC.TOROW.json` (signature, arity, classification axes).
