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
  - Notes
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

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `OP_RANGE_REF` that an implementer matching Excel, or a reader explaining
an unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's
answer and where it was recorded; "measured" means the statement is read off recorded Excel
answers, "inferred" marks what goes beyond them. Where older text on this page says a behaviour
was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

### FN-OP_RANGE_REF-001: Range operator (:) forms one normalized area, allows whitespace around :, and spans chained operands

*Quirk; measured; recorded 2026-11-22.*

In Excel 16.0.20527, the range operator (:) takes two references on one sheet and produces a
single rectangular area (AREAS(A1:C7) = 1, ROWS(A:A) = 1048576, ISREF(A1:B2) = TRUE).
Whitespace around the colon still parses as the range operator rather than space intersection
(=SUM(A1 : A3), =SUM(A1 :A3), and =SUM(A1: A3) all return 6; INDIRECT("$A1 : B$2") resolves,
whereas INDIRECT("A1:B2:C3") returns #REF!). Chained ranges in worksheet formulas span their
bounding box (=ROWS(A1:B2:C4) = 4, Excel 16.0 on 2026-10-23).

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=SUM(A1 : A3)` | as typed | `6.0` (`0x4018000000000000`) | Excel 16.0.20527.20196, Windows (bitness and CPU not recorded) |
| `=SUM(INDIRECT("$A1 : B$2"))` | as typed | `100.0` (`0x4059000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=SUM(INDIRECT("A1:B2:C3"))` | as typed | `#REF!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=ROWS(A:A)` | as typed | `1048576.0` (`0x4130000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=AREAS(A1:C7)` | as typed | `1.0` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Measured across Excel 16.0.20527 in hjd (rows 1-3), dc_51ka (rows 44, 46,
51), cfunc_m3_6 (reference_result row 48, lookup_edges row 320), dc_nzpw (row 92), and
2026-10-23 Excel 16.0 checks in refops.rs / l1.rs.

Evidence:

- DnaCalc `crates/calc-fml/src/eval/refops.rs`, lines 9-16 and 55-72 at `494f964897` — Range bounding-box composition and 2026-10-23 =ROWS(A1:B2:C4) = 4 reading
- DnaCalc `crates/calc-fml/tests/acceptance/l1.rs`, lines 908-924 at `494f964897` — L1 range operator acceptance checks
- DnaCalc `crates/calc-func/tests/captures/hjd/oracle.capture.json`, sheet hjd_oxfml rows 1-3 at `494f964897` — Whitespace around colon accepted as range operator
- DnaCalc `crates/calc-fml/tests/captures/dc_51ka/oracle.capture.json`, sheet indirect rows 44, 46, and 51 at `494f964897` — INDIRECT spaced colon vs chained colon behavior
- DnaCalc `crates/calc-func/tests/captures/hjd/oracle.capture.json`, sheet hjd_oxfml row 1 column 4 at `494f964897` (readings)
- DnaCalc `crates/calc-fml/tests/captures/dc_51ka/oracle.capture.json`, sheet indirect row 46 column 4 at `494f964897` (readings)
- DnaCalc `crates/calc-fml/tests/captures/dc_51ka/oracle.capture.json`, sheet indirect row 51 column 4 at `494f964897` (readings)
- DnaCalc `crates/calc-func/tests/captures/cfunc_m3_6/oracle.capture.json`, sheet reference_result row 48 column 4 at `494f964897` (readings)
- DnaCalc `crates/calc-func/tests/captures/cfunc_m3_6/oracle.capture.json`, sheet lookup_edges row 320 column 4 at `494f964897` (readings)

<!-- efh:notes end -->

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
