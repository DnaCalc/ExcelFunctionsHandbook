---
schema: efh.function-page/v1
function_id: FUNC.OP_IMPLICIT_INTERSECTION
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
family: op_implicit_intersection
role_in_family: "The sole member: `@` is the only operator whose result depends on where the formula is, which is why it cannot share a module with anything else."
---

## What it computes

`@A` reduces its operand to a single value, using the position of the *calling cell* to
decide which one.

It is the only operator in this Handbook whose result depends on where the formula lives.
Every other operator is a function of its operands alone; `@` is a function of its operand
*and* the calling cell's row and column. That is why its declared axes are unlike anything
else in the catalog: `ArgPreparationProfile::RefsVisibleInAdapter`,
`HostInteractionClass::WorkbookState`, `ThreadSafetyClass::HostSerialized`, and a composite
dependency on the formula-evaluation context.

OxFunc's provisional slice contract states the outcome model in four cases:

1. **Already a single item** — returned unchanged.
2. **A reference or range** — scalarized relative to the caller: the same-row or same-column
   cell of that range, chosen against the formula's anchor. A two-dimensional range yields
   `#VALUE!` on the admitted native baseline.
3. **An array payload** rather than a reference identity — scalarized to the top-left item.
4. **A spill-capable expression** — removing `@` changes the publication surface from
   scalarization to spilling, subject to host spill rules.

Cases 2 and 3 differ in *which* item survives, and the difference is the reason the operator
needs references visible: a range is scalarized positionally, an array is scalarized to its
first element. An implementation that dereferences the operand before `@` sees it cannot tell
the two apart.

### Why the character exists at all

Before dynamic arrays, a formula that handed a range to a scalar-shaped argument got this
scalarization silently — "implicit intersection" is the historical name for the silent
behaviour. When Excel gained dynamic arrays, the same expression started spilling instead.
`@` is the explicit marker that preserves the old meaning: it makes the intersection visible
in the formula text so that legacy workbooks keep computing what they always computed. The
contract puts it plainly: explicit `@` "is a modern surface marker for implicit-intersection
behavior that older Excel performed silently".

That is also why `@` appears unbidden in formulas that were written years earlier: opening an
old workbook in a dynamic-array build can surface the marker that was previously implicit.
The character is a compatibility artifact, not a new feature.

### One operator, several spellings

Chapter 05 records `SINGLE(...)` and `_xlfn.SINGLE(...)` as compatibility and serialization
representations of this same operator identity — not separate semantics. The contract adds
that the host baseline normalizes `_xlfn.SINGLE(...)` back onto explicit `@` in the modern
formula property. So a reader may encounter three spellings of one function.

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | The expression to scalarize. Required. |

Arity is exactly 1; no optional arguments, no defaults. The operator is prefix.

`data/functions/FUNC.OP_IMPLICIT_INTERSECTION.json` carries `signature: null` — the entry has
no displayable signature, and its metadata status is `doc_modeled` rather than extracted from
the implementation registry. It is the only operator entry in that state, and it is why the
typed form here (`@A`) is supplied by the Handbook rather than read from the projection.

## Result and edge cases

Returns whatever single value the scalarization selects — a scalar of any kind, or an error.
The kernel class is `Custom`; there is no meaningful signature in the numeric sense.

- **Scalar operand.** Passthrough.
- **One-row or one-column range.** The cell in the caller's column or row respectively.
- **A range that does not intersect the caller's row or column.** The historical behaviour of
  silent implicit intersection was `#VALUE!` in this case; the contract's admitted lane
  records `#VALUE!` for the two-dimensional case, and this page does not extend that to every
  non-intersecting shape without evidence.
- **Two-dimensional range.** `#VALUE!` on the admitted native baseline.
- **Array payload.** Top-left element.
- **Empty array.** Chapter 01 lists `#CALC!` as the error for an engine that cannot produce a
  value, including the empty-array case. Whether `@` on an empty array reaches it is not
  recorded here.
- **Inside a lambda or a function argument.** The "calling cell" for a nested evaluation is
  an execution-context question (chapter 04), not an operator question, and this page does
  not settle it.

Note one structural consequence: because the operator needs a workbook and a calling cell, it
cannot be exercised by a value-only dispatch harness at all. The Handbook's mechanical
battery records it as not dispatchable for exactly that reason — a fact about the harness,
not about the operator.

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | The operand cannot be scalarized against the caller's position — recorded for the two-dimensional range case on the admitted baseline. |
| any incoming error | An error operand propagates. |

`data/functions/FUNC.OP_IMPLICIT_INTERSECTION.json` records no Microsoft documentation URL
(`docs` is `null`).

## Relationships

- [`FUNC.OP_SPILL_REF`](FUNC.OP_SPILL_REF.md) — the other dynamic-array operator, and the
  opposite motion: `#` names a whole spilled region, `@` refuses to spill.
- [`FUNC.OP_INTERSECTION_REF`](FUNC.OP_INTERSECTION_REF.md) — the *explicit* intersection of
  two references, spelled with a space. Despite the shared word, it is a different operation
  with different operands; confusing the two is common and this page names the confusion
  deliberately.
- `SINGLE`, `_xlfn.SINGLE` — the same identity under compatibility and serialization
  spellings (chapter 05).
- `INDEX`, `INDIRECT`, `OFFSET`, `XLOOKUP` — reference-returning functions whose results `@`
  consumes; the contract treats them as supporting substrate that `@` does not redefine.
- `TAKE(x,1,1)`, `INDEX(x,1,1)` — explicit alternatives when the intent is "the first
  element" rather than "the historically intersected element". They mean what they say; `@`
  means something position-dependent.

## Notes for implementers

- The presence of `@` must survive parsing and binding into evaluation metadata even when the
  stored form omits the token. The contract states this as a hard requirement, and it is the
  single thing most likely to be got wrong: an evaluator that reconstructs `@` from the
  stored formula text alone will lose it.
- Keep reference identity distinct from array payload all the way to the operator. The two
  cases scalarize differently.
- The calling cell's position is an input. Any caching keyed only on the operand is wrong.
- Treat the three spellings as one identity with a normalization step, not as three
  functions.
- This operator is `HostSerialized`: it is not safe to evaluate on an arbitrary worker thread
  the way the arithmetic operators are.

## What has not been checked

No Handbook vector suite covers `@`, and no Excel-comparison evidence record is attached to
this page. Everything above is reported from OxFunc's provisional slice contract and the
Handbook's own version-axes chapter; none of it has been re-measured here. This entry is also
the only operator whose projected metadata is `doc_modeled` rather than implementation-
extracted, so even its axis chips rest on a document rather than on a registry.

Probes worth running first, all of which need a real workbook because the operator cannot be
called without one:

1. **Positional scalarization.** Put `=@$A$1:$A$10` in several rows and confirm each picks
   its own row's cell; repeat with a row-shaped operand across several columns.
2. **Non-intersecting position.** The same formula placed in a row outside the operand's
   extent, to pin the error for that case rather than inferring it.
3. **Range versus array.** `=@A1:C3` against `=@{1,2,3;4,5,6}` from the same cell, to confirm
   the range/array split (positional versus top-left).
4. **Serialization round-trip.** Enter `=@A1:A10`, read the stored formula, read the
   compatibility formula property, and reopen in a build without dynamic arrays — the
   experiment that confirms the `SINGLE` aliasing chapter 05 records.
5. **Spill suppression.** The same expression with and without `@`, confirming one
   scalarizes and one spills, and what happens when the spill is blocked.
6. **Nested context.** `@` inside `LAMBDA`, inside a defined name, and inside a function
   argument, to establish what "the calling cell" means when there is not obviously one.

## Page vocabulary

| Machine name | Meaning |
|---|---|
| `Arity { min: 1, max: 1 }` | Exactly one operand |
| `ArgPreparationProfile::RefsVisibleInAdapter` | The operator sees live references and controls dereference itself |
| `KernelSignatureClass::Custom` | Kernel shape is function-specific |
| `HostInteractionClass::WorkbookState` | The operator's meaning depends on workbook state |
| `ThreadSafetyClass::HostSerialized` | Must be evaluated on the host's serialized path |
| `doc_modeled` | Metadata status: the entry's axes come from a document, not from an implementation registry |
| `#CALC!` | The error for a value the engine cannot produce, including empty arrays |

## Sources

- `data/functions/FUNC.OP_IMPLICIT_INTERSECTION.json` at OxFunc `473efa3` — identity, arity
  1, `signature: null`, `metadata_status: doc_modeled`, and the
  `RefsVisibleInAdapter` / `WorkbookState` / `HostSerialized` classification. `docs` is
  `null`: **no Microsoft documentation URL is recorded for this entry.**
- `data/presence/FUNC.OP_IMPLICIT_INTERSECTION.json` — implementing module
  `crates/oxfunc_core/src/functions/op_implicit_intersection.rs`, the family slug for this
  page.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OP_IMPLICIT_INTERSECTION_CONTRACT_PRELIM.md` —
  the four-case outcome model, the caller-context scalarization rule, the two-dimensional
  `#VALUE!` lane, the "modern surface marker for behavior older Excel performed silently"
  statement, the `SINGLE` aliasing note, and the requirement that explicit `@` survive into
  evaluation metadata. Provisional by its own header.
- Handbook `content/model/05-version-axes.md` — the record that `_xlfn.SINGLE(...)` and `@`
  are compatibility/serialization representations of one operator identity.
- Handbook `content/model/01-value-universe.md`, `03-call-pipeline.md`,
  `04-execution-context.md`.
