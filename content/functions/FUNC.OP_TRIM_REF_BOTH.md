---
schema: efh.function-page/v1
function_id: FUNC.OP_TRIM_REF_BOTH
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
role_in_family: "Two-sided whitespace normalization: the trim identity that covers padding on both sides at once, and the operator entry whose published signature is provably untypeable."
---

## What it computes

`OP_TRIM_REF_BOTH` normalizes a reference operand carrying whitespace on **both** sides,
yielding the same reference without the surrounding space.

It is the two-sided member of the trim family. OxFunc's provisional reference-family contract
admits the trim rows as "structural reference-target normalization only", and its
current-baseline findings include exactly the two-sided shape: whitespace-trimmed reference
forms such as `SUM(( A1 ))` remain transparent on the seeded slice. On that evidence the
operator changes no computed value; it exists so that surrounding whitespace is modelled
explicitly rather than discarded by a parser with no record of the decision.

Why a separate identity for "both", when leading and trailing already exist? Because the
model catalogues the *shape the formula was written in*, not a minimal generating set. A
padded reference is one syntactic phenomenon with three positional variants, and the
inventory keeps all three so that a stored formula maps onto exactly one identity rather than
onto a composition that a round-trip might reassociate.

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | The reference whose surrounding whitespace is normalized. Required. |

Arity is exactly 1 — and here is the entry's most conspicuous oddity. The signature display
recorded in `data/functions/FUNC.OP_TRIM_REF_BOTH.json` is `A @? B`, which reads as a
*binary* operator with a two-character spelling. It is neither. The arity in the same file is
`{min: 1, max: 1}`, and this Handbook's own operator-syntax schema records the point
explicitly: this row's published display "is not a spelling anyone can type", and its typed
form must be Handbook-supplied rather than taken from the projection.

That is worth stating plainly rather than tidying away: **the projected signature for this
operator is a placeholder that contradicts its own arity.** Anyone building a renderer from
`data/functions/` needs to know it, and anyone reading the rendered page needs to know not to
type `A @? B` into a cell.

## Result and edge cases

Returns a `Reference` (`KernelSignatureClass::Custom`,
`ArgPreparationProfile::RefsVisibleInAdapter`).

- **Ordinary case.** The operand reference, unchanged in extent and identity.
- **Only one side padded.** That is [`FUNC.OP_TRIM_REF_LEADING`](FUNC.OP_TRIM_REF_LEADING.md)
  or [`FUNC.OP_TRIM_REF_TRAILING`](FUNC.OP_TRIM_REF_TRAILING.md); which identity a given
  formula produces is a grammar question the sources leave out of slice.
- **Trailing side followed by another reference.** Then the trailing space is the
  intersection operator, not padding, and this row does not apply.
- **Non-reference operand.** Outside the admitted slice.

## Errors

| Error | Condition |
|---|---|
| `#REF!` | The operand is not a usable reference. |
| any incoming error | An error operand propagates. |

`data/functions/FUNC.OP_TRIM_REF_BOTH.json` records no Microsoft documentation URL (`docs` is
`null`). No cited document states an error surface specific to this row.

## Relationships

- [`FUNC.OP_TRIM_REF_LEADING`](FUNC.OP_TRIM_REF_LEADING.md),
  [`FUNC.OP_TRIM_REF_TRAILING`](FUNC.OP_TRIM_REF_TRAILING.md) — the one-sided variants.
- [`FUNC.OP_INTERSECTION_REF`](FUNC.OP_INTERSECTION_REF.md) — the evaluable use of the same
  character.
- [`FUNC.OP_RANGE_REF`](FUNC.OP_RANGE_REF.md), [`FUNC.OP_UNION_REF`](FUNC.OP_UNION_REF.md) —
  the constructors whose operands carry the padding.
- `TRIM` — the text function; no relation beyond the name.

## Notes for implementers

- Do not build this operator's identity from a composition of the two one-sided ones unless
  you can guarantee the composition round-trips to the same stored form. The inventory keeps
  three identities for a reason.
- Do not render the `A @? B` display to users. It is a placeholder in the data projection and
  will be read as a typeable spelling.
- As with the siblings: preserve the whitespace in the stored form, keep the reference
  unchanged in value, and resolve the padding-versus-intersection question in one place.

## What has not been checked

No Handbook vector suite covers this operator, no Excel-comparison evidence record is
attached to this page, and whether the row has any independently observable behaviour is
unresolved. Two questions are specific to this entry:

1. **Is the projected signature a data defect?** `A @? B` with arity 1 cannot both be right.
   Reconciling the signature against the arity in the upstream registry is a documentation
   fix, not a measurement, and it should be raised as such.
2. **Is "both" a distinct identity in the stored form, or a composition?** Round-tripping
   ` A1 ` through save and reopen, and comparing the stored token stream against the
   one-sided cases, is the experiment that answers it.

Beyond those:

3. **Value transparency.** `=SUM(( A1 ))` against `=SUM((A1))` over several operand shapes.
4. **Whitespace classes.** Tabs, non-breaking spaces and line breaks on both sides.
5. **Both-sided padding next to another operator.** ` A1 :B2 ` and ` A1 # `, where padding
   meets a constructor or the spill operator.

## Page vocabulary

| Machine name | Meaning |
|---|---|
| `Arity { min: 1, max: 1 }` | Exactly one operand — despite the two-operand-looking display |
| `ArgPreparationProfile::RefsVisibleInAdapter` | The operator sees live references and controls dereference itself |
| `KernelSignatureClass::Custom` | Kernel shape is function-specific |
| `HostInteractionClass::WorkbookState` | The operator's meaning depends on workbook state |
| `reference_normalization` | The operator-inventory family this row belongs to |
| `handbook-supplied` | Typed-form source: the Handbook supplies the spelling because the projection has none |

## Sources

- `data/functions/FUNC.OP_TRIM_REF_BOTH.json` at OxFunc `473efa3` — identity, arity 1, the
  `A @? B` signature display, `RefsVisibleInAdapter` preparation. `docs` is `null`: **no
  Microsoft documentation URL is recorded for this entry.**
- `data/presence/FUNC.OP_TRIM_REF_BOTH.json` — implementing module
  `crates/oxfunc_core/src/functions/operator_reference_family.rs`.
- Handbook `tools/schemas/f11-operator-syntax.schema.json` — the recorded requirement that
  this row's typed form be Handbook-supplied because its published display "is not a spelling
  anyone can type".
- OxFunc `docs/function-lane/FUNCTION_SLICE_OPERATOR_REFERENCE_FAMILY_CONTRACT_PRELIM.md` —
  the normalization-only admission and the `SUM(( A1 ))` transparency finding; provisional by
  its own header.
- OxFunc `docs/function-lane/W45_NON_AT_OPERATOR_INVENTORY.csv` — the `<trim-both>`
  surface-syntax placeholder and the `reference_normalization` family.
- Handbook `content/model/01-value-universe.md`, `03-call-pipeline.md`.
