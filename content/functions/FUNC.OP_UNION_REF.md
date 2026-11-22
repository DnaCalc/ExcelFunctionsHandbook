---
schema: efh.function-page/v1
function_id: FUNC.OP_UNION_REF
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
role_in_family: "The comma: the only operator whose typed form collides with the formula grammar's argument separator, and the family's multi-area constructor."
---

## What it computes

`A , B` takes two references and returns a reference designating **both** areas — a
multi-area reference, not a merged rectangle.

The contrast with [`FUNC.OP_RANGE_REF`](FUNC.OP_RANGE_REF.md) is the point: `A1:B2` is the
rectangle spanning two corners, while `(A1,B2)` is two separate cells travelling together as
one reference value. Chapter 01 lists multi-area as one of the reference shapes, and OxFunc's
provisional reference-family contract records same-sheet multi-area formation as a
first-class `MultiArea` reference kind rather than a string convention.

Two consequences that a reader can observe:

- **Areas keep their identity.** The contract records `INDEX((A1:A2,B1:B2),2,1,2)` selecting
  from the *second* area — `INDEX`'s fourth argument is an area number, which only means
  something because the union preserved the areas separately. Any function taking an area
  index (`INDEX`, `AREAS`) is consuming exactly this structure.
- **Overlaps are not de-duplicated.** A union is a list of areas, not a set of cells. The
  contract's `SUM((A1,B1))` finding is a plain two-cell sum, but a union whose areas overlap
  presents the overlapping cells more than once to a consumer that scans areas in order.
  Whether every consumer double-counts is a per-function question, and the Handbook has not
  recorded it.

**The parenthesis requirement is not decoration.** Inside a function call, a comma is the
argument separator — grammar, not an operator (chapter 03: parse-only delimiters carry no
semantics and get no function identity). `SUM(A1,B1)` is a two-argument call; `SUM((A1,B1))`
is a one-argument call whose argument is a union. Both happen to total the same cells here,
which is exactly why the distinction is so easy to miss, and why it bites when the consuming
function counts its arguments (`AREAS`, `INDEX`, `SUBTOTAL`).

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | First reference. Required. |
| 1 | `B` | Second reference. Required. |

Arity is exactly 2; no optional arguments, no defaults. A chain `(A1,B1,C1)` is repeated
application of the two-argument operator, and area order is preserved — so the operator is
not commutative in the way a set union would be, because area *order* is observable through
area-indexed consumers.

Both operands must be reference-shaped.

## Result and edge cases

Returns a `Reference` with the multi-area shape (`KernelSignatureClass::Custom`).

- **Materialization.** When a consumer needs values rather than a reference, the areas must
  be combined. The contract records same-sheet rect-style multi-area materialization as
  happening through resolver-driven row-vector combination in member order — that is, the
  member order of the union decides the resulting value order.
- **Cross-sheet unions.** The contract's admitted slice is same-sheet. Unions across sheets
  are not covered by it, and are not settled here.
- **Non-reference operands.** No union exists; the result is an error.
- **Overlapping areas.** Preserved as written; see above.
- **A known seam.** OxFunc's `BUG-FUNC-003` records a state in which union formation emitted
  a plain area rather than a first-class multi-area value, with downstream consumers reading
  a string convention instead of the structured reference. The Handbook cites this as
  evidence that multi-area identity is a real, separable behaviour that implementations can
  and do get wrong — not as a statement about any current state.

## Errors

| Error | Condition |
|---|---|
| `#REF!` | An operand is not a usable reference, or a member area is invalid. |
| any incoming error | An error operand propagates. |

`data/functions/FUNC.OP_UNION_REF.json` records no Microsoft documentation URL (`docs` is
`null`).

## Relationships

- [`FUNC.OP_RANGE_REF`](FUNC.OP_RANGE_REF.md) — the bounding-box constructor; the join to
  this operator's list.
- [`FUNC.OP_INTERSECTION_REF`](FUNC.OP_INTERSECTION_REF.md) — the meet.
- `AREAS` — the function that counts the members of a multi-area reference. It exists
  essentially to inspect the output of this operator.
- `INDEX` with a fourth argument — area selection within a multi-area reference.
- `SUM`, `COUNT` and the other aggregates — the usual consumers, and the ones whose
  double-counting behaviour on overlapping unions is worth checking.
- The locale-dependent list separator is a *grammar* question. In locales where function
  arguments are separated by `;`, whether the union operator is still spelled `,` is not
  recorded in this Handbook's sources, and this page does not assert it.

## Notes for implementers

- Represent a union as a first-class ordered list of areas, from the moment it is formed.
  Flattening it to a bounding box, or to a string, loses the identity that `AREAS` and
  `INDEX` depend on — which is precisely the seam `BUG-FUNC-003` describes.
- Preserve member order, and materialize in member order.
- Do not de-duplicate overlaps at formation time. Whether a consumer double-counts is the
  consumer's semantics, and collapsing early removes the consumer's ability to decide.
- Keep the operator distinct from the argument separator at every layer. If the same token
  handling produces both, a parenthesized union and a two-argument call will eventually be
  confused, and the failure will look like an arity bug rather than a reference bug.

## What has not been checked

No Handbook vector suite covers the union operator, and no Excel-comparison evidence record
is attached to this page. The findings above are reported from OxFunc's provisional contract
and defect record, not measured by the Handbook.

Probes worth running first:

1. **Overlap double-counting.** `=SUM((A1:B2,B2:C3))` against the sum of the distinct cells,
   with a value in the overlapping cell, and the same shape through `COUNT`, `COUNTA` and
   `SUBTOTAL`. This settles whether the union is a list or a set for each consumer.
2. **Area identity.** `=AREAS((A1,B1,C1))` and `=INDEX((A1:A2,B1:B2),1,1,2)`, to confirm
   member count and member order.
3. **Cross-sheet unions.** `=SUM((Sheet1!A1,Sheet2!A1))`, outside the contract's admitted
   slice and therefore genuinely unknown here.
4. **Locale spelling.** The same union formula entered in a locale whose list separator is
   `;`, to settle the grammar question stated above.
5. **Materialization order.** A union of a row area and a column area passed to a function
   that returns its input shape, to observe how members combine.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `OP_UNION_REF` that an implementer matching Excel, or a reader explaining
an unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's
answer and where it was recorded; "measured" means the statement is read off recorded Excel
answers, "inferred" marks what goes beyond them. Where older text on this page says a behaviour
was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

### FN-OP_UNION_REF-001: Union operator (,) preserves ordered areas for AREAS/INDEX/SUM error order and gives #REF! in ROWS/COLUMNS/ROW

*Quirk; measured; recorded 2026-11-22.*

In Excel 16.0.20527, the parenthesized union operator (A,B) constructs an ordered multi-area
reference rather than merging areas: AREAS((A1:B2,C3,A4)) = 3, INDEX((A1:A3,C1:C3),2,1,2)
selects from the second area, ISREF((A1,B2)) = TRUE, and SUM((A2:B2,A1:B1)) returns the first
error in union area order (#DIV/0!) rather than sheet row-major order (#N/A). Functions
requiring a single rectangular reference return #REF! (ROWS((A1:A3,B1:B2)),
COLUMNS((A1:A3,B1:B2)), @ROW((A1,B2)), @COLUMN((A1,B2))), MATCH/XLOOKUP/VLOOKUP/LOOKUP return
#N/A, TYPE((A1,A2)) returns 16, and cross-sheet unions (=SUM((S1!A1,S2!A1))) return #VALUE!.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=AREAS((A1:B2,C3,A4))` | as typed | `3.0` (`0x4008000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=INDEX((A1:A3,C1:C3),2,1,2)` | as typed | `6.0` (`0x4018000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=SUM((A2:B2,A1:B1))` | as typed | `#DIV/0!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=ROWS((A1:A3,B1:B2))` | as typed | `#REF!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=ISREF((A1,B2))` | as typed | `TRUE` | Excel 16.0.20527.20196 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=TYPE((A1,A2))` | as typed | `16.0` (`0x4030000000000000`) | Excel 16.0.20527.20196 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Measured across Excel 16.0.20527 in cfunc_m3_6 (reference_result rows 10, 24,
47; lookup_edges rows 147, 154-155, 436, 442, 447, 454, 462, 472), cfunc_m1_2 (error_order row
72), dc_nzpw (rows 93, 110), and 2026-10-23 Excel 16.0 checks in refops.rs / l1.rs.

Evidence:

- DnaCalc `crates/calc-fml/src/eval/refops.rs`, lines 11-15 and 98-112 at `494f964897` — Union multi-area composition and cross-sheet #VALUE! rule
- DnaCalc `crates/calc-fml/tests/acceptance/l1.rs`, lines 946-956 at `494f964897` — L1 union acceptance checks
- DnaCalc `crates/calc-func/tests/captures/cfunc_m3_6/oracle.capture.json`, reference_result rows 10, 24, 47; lookup_edges rows 147, 154-155, 436-472 at `494f964897` — Excel 16.0.20527 multi-area behavior across lookup and reference functions
- DnaCalc `crates/calc-func/tests/captures/cfunc_m1_2/oracle.capture.json`, sheet error_order row 72 column 4 at `494f964897` — Union area evaluation order for errors
- DnaCalc `crates/calc-func/tests/captures/cfunc_m3_6/oracle.capture.json`, sheet reference_result row 47 column 4 at `494f964897` (readings)
- DnaCalc `crates/calc-func/tests/captures/cfunc_m3_6/oracle.capture.json`, sheet reference_result row 10 column 4 at `494f964897` (readings)
- DnaCalc `crates/calc-func/tests/captures/cfunc_m1_2/oracle.capture.json`, sheet error_order row 72 column 4 at `494f964897` — A2 is #DIV/0! and B1 is #N/A; union visits A2:B2 before A1:B1 (readings)
- DnaCalc `crates/calc-func/tests/captures/cfunc_m3_6/oracle.capture.json`, sheet lookup_edges row 147 column 4 at `494f964897` (readings)
- DnaCalc `crates/calc-func/tests/captures/dc_nzpw/oracle.capture.json`, sheet information_functions row 93 column 4 at `494f964897` (readings)
- DnaCalc `crates/calc-func/tests/captures/dc_nzpw/oracle.capture.json`, sheet information_functions row 110 column 4 at `494f964897` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Machine name | Meaning |
|---|---|
| `Arity { min: 2, max: 2 }` | Exactly two operands |
| `ArgPreparationProfile::RefsVisibleInAdapter` | The operator sees live references and controls dereference itself |
| `KernelSignatureClass::Custom` | Kernel shape is function-specific |
| `HostInteractionClass::WorkbookState` | The operator's meaning depends on workbook state |
| `MultiArea` | The reference shape produced: an ordered union of areas |

## Sources

- `data/functions/FUNC.OP_UNION_REF.json` at OxFunc `473efa3` — identity, arity, signature
  `A, B`, `RefsVisibleInAdapter` preparation, `WorkbookState` host interaction. `docs` is
  `null`: **no Microsoft documentation URL is recorded for this entry.**
- `data/presence/FUNC.OP_UNION_REF.json` — implementing module
  `crates/oxfunc_core/src/functions/operator_reference_family.rs`.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OPERATOR_REFERENCE_FAMILY_CONTRACT_PRELIM.md` —
  first-class `MultiArea` formation, member-order materialization, and the `SUM((A1,B1))` and
  `INDEX((A1:A2,B1:B2),2,1,2)` findings; provisional, same-sheet slice.
- OxFunc `docs/bugs/streams/BUG-FUNC-003_multi_area_reference_seam_collapses_to_area_string.md`
  — the recorded seam in which union formation lost multi-area identity. Cited as an upstream
  record.
- Handbook `content/model/01-value-universe.md` (reference shapes, including multi-area),
  `03-call-pipeline.md` (operators are functions; parse-only delimiters are not).
