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
