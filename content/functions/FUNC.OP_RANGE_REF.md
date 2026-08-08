---
schema: efh.function-page/v1
function_id: FUNC.OP_RANGE_REF
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
family: operator_reference_family
role_in_family: "The colon: the reference algebra's constructor, and the operator every range in every workbook is built from."
---

## What it computes

`A : B` takes two references and returns the smallest rectangular reference that contains
both — the *bounding box*, not the pair.

This is the most-used function in Excel and one of the least described. `A1:B10` is not
notation; it is a call. Chapter 03 says so explicitly: every evaluable operator is a function
with its own identity in the same catalog, and `:` is `FUNC.OP_RANGE_REF`.

Three properties follow from "smallest containing rectangle":

1. **Order does not matter.** OxFunc's provisional reference-family contract records
   `SUM((A1:B2))` and `SUM((B2:A1))` producing the same result: the operator normalizes
   corner order rather than requiring top-left first. `B2:A1` is the same area as `A1:B2`.
2. **The operands are corners, not endpoints of a path.** `A1:B10` covers 20 cells, not the
   cells "between" A1 and B10 in reading order.
3. **The result is a reference, not values.** `:` is one of the reference-visible operators
   (`ArgPreparationProfile::RefsVisibleInAdapter`): its operands must survive as references
   into the operator, and its result is a `Reference` value in the sense of chapter 01 — an
   address, not the contents. Whatever consumes it decides whether to dereference.

The contract records the admitted operand shapes as A1-style cells, areas, whole rows and
whole columns that parse through the local A1 reference substrate. Whole-row and
whole-column forms (`1:1`, `A:A`) are this same operator with row-only or column-only
operands.

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | First corner reference. Required. |
| 1 | `B` | Second corner reference. Required. |

Arity is exactly 2; no optional arguments, no defaults. Both operands must be
reference-shaped: a number, text or logical operand is not a reference and has no bounding
box with anything.

The subtlety readers miss is that the operands need not be literal addresses. Reference-
returning expressions — `INDEX` in its reference form, `OFFSET`, `INDIRECT`, a defined name —
can appear on either side, which is how idioms like `A1:INDEX(A:A,n)` build a growing range
without a volatile function. That works only because `:` receives references rather than
values.

## Result and edge cases

Returns a `Reference` (`KernelSignatureClass::Custom`).

- **Non-reference operands.** No bounding box exists; the result is an error, not a
  coercion.
- **Cross-sheet operands.** `Sheet1:Sheet3!A1` is the three-dimensional reference shape of
  chapter 01. Whether that is this operator applied to sheet names, or a separate grammar
  form, is not settled in this Handbook's sources, and the contract's admitted slice does not
  cover mixed-prefix composition.
- **Result shape.** Always a single rectangular area. `:` cannot produce a multi-area
  reference; that is [`FUNC.OP_UNION_REF`](FUNC.OP_UNION_REF.md).
- **Deleted operands.** If either corner is invalidated (its rows or columns deleted), the
  reference is no longer valid and `#REF!` is the documented error for that state
  (chapter 01).
- **Coercion and lifting do not apply.** This operator sits outside the scalar coercion
  story; there is nothing to convert and nothing to broadcast.

## Errors

| Error | Condition |
|---|---|
| `#REF!` | An operand is not a usable reference, or the constructed reference is no longer valid. |
| any incoming error | An error operand propagates. |

`data/functions/FUNC.OP_RANGE_REF.json` records no Microsoft documentation URL (`docs` is
`null`), so these conditions come from chapter 01's error table and OxFunc's provisional
reference-family contract rather than from a cited Microsoft page for `:`.

## Relationships

- [`FUNC.OP_UNION_REF`](FUNC.OP_UNION_REF.md) — the comma; combines areas without merging
  them into a bounding box.
- [`FUNC.OP_INTERSECTION_REF`](FUNC.OP_INTERSECTION_REF.md) — the space; the meet operation
  to this operator's join.
- [`FUNC.OP_SPILL_REF`](FUNC.OP_SPILL_REF.md) — a different way to name a region, by anchor
  rather than by corners.
- `INDIRECT`, `OFFSET`, `INDEX` (reference form) — the reference-producing functions that can
  supply either operand.
- `ROW`, `COLUMN`, `ROWS`, `COLUMNS`, `AREAS`, `CELL` — the reference-inspecting functions
  that consume the result without dereferencing it.
- Structured references (`Table1[Amount]`) and defined names denote areas without using this
  operator; they are separate reference shapes in chapter 01's table.

## Notes for implementers

- Normalize the corners. Store an area canonically (top-left, bottom-right) so that `B2:A1`
  and `A1:B2` are the same value, and so that downstream equality of references works.
- Keep the operands as references all the way in. An implementation that dereferences before
  applying `:` cannot support `A1:INDEX(...)` at all, and that idiom is common enough to be a
  correctness requirement rather than a nicety.
- Absolute/relative markers (`$`) and sheet qualification belong to the reference value, not
  to this operator. `:` composes whatever it is given; it must not normalize away anchoring.
- This operator is declared `WorkbookState`-interacting in the data projection, because a
  reference means nothing without a workbook. Treat it as host-coupled even though its logic
  is pure geometry.

## What has not been checked

No Handbook vector suite covers `:`, and no Excel-comparison evidence record is attached to
this page. The order-independence and admitted-operand facts above are reported from OxFunc's
provisional contract, not measured by the Handbook.

Probes worth running first:

1. **Reversed and partial corners.** `B2:A1`, `B1:A2`, `A1:A1`, and whole-row/whole-column
   combinations, confirming the bounding-box normalization in every orientation.
2. **Reference-returning operands.** `A1:INDEX(A:A,3)`, `OFFSET(A1,0,0):B2`,
   `INDIRECT("A1"):B2`, to map which reference producers survive as operands — and whether
   any of them force the range to be recomputed volatilely.
3. **Cross-sheet forms.** `Sheet1:Sheet3!A1` and `Sheet1!A1:Sheet2!B2`, to determine which
   are this operator and which are grammar, since the contract puts mixed-prefix composition
   out of slice.
4. **Invalidation.** Build a range, delete a corner's row, and record whether the result is
   `#REF!` immediately or on recalculation.
5. **Full-column ranges under different grid sizes.** `A:A` in a modern grid versus a
   legacy-compatibility workbook, since the bounding box depends on the sheet's dimensions.

## Page vocabulary

| Machine name | Meaning |
|---|---|
| `Arity { min: 2, max: 2 }` | Exactly two operands |
| `ArgPreparationProfile::RefsVisibleInAdapter` | The operator sees live references and controls dereference itself |
| `KernelSignatureClass::Custom` | Kernel shape is function-specific |
| `HostInteractionClass::WorkbookState` | The operator's meaning depends on workbook state |
| `ReferenceLike` | The return kind: a cell-range designator |
| `Area` | The reference shape produced: one rectangle |

## Sources

- `data/functions/FUNC.OP_RANGE_REF.json` at OxFunc `473efa3` — identity, arity, signature
  `A:B`, `RefsVisibleInAdapter` preparation and `WorkbookState` host interaction. `docs` is
  `null`: **no Microsoft documentation URL is recorded for this entry.** Microsoft's account
  of the reference operators lives in the support article *Calculation operators and
  precedence in Excel*, not yet linked from the data projection.
- `data/presence/FUNC.OP_RANGE_REF.json` — implementing module
  `crates/oxfunc_core/src/functions/operator_reference_family.rs`, the family slug for this
  page.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OPERATOR_REFERENCE_FAMILY_CONTRACT_PRELIM.md` —
  the admitted operand shapes, the `SUM((A1:B2))` / `SUM((B2:A1))` order-independence
  finding, and the explicit out-of-slice list including mixed-prefix composition. Provisional
  by its own header.
- Handbook `content/model/01-value-universe.md` (reference shapes, `#REF!`),
  `03-call-pipeline.md` (operators are functions; reference-visible preparation).
