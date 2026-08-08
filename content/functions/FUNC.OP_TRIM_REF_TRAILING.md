---
schema: efh.function-page/v1
function_id: FUNC.OP_TRIM_REF_TRAILING
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
role_in_family: "Trailing whitespace normalization: the trim position that sits directly against the intersection operator, since a space after a reference is exactly where the two readings compete."
---

## What it computes

`OP_TRIM_REF_TRAILING` normalizes a reference operand that carries whitespace **after** it,
yielding the same reference without the trailing space.

Of the three trim identities this is the one with the sharpest reason to exist. A space
*after* a reference is precisely the position in which the space would be the intersection
operator, if another reference followed it. `A1 ` and `A1 B1` begin identically; the first is
a padded reference and the second is a call to
[`FUNC.OP_INTERSECTION_REF`](FUNC.OP_INTERSECTION_REF.md). The disambiguation is
lookahead — grammar, not semantics — and this row is the identity that owns the outcome when
the lookahead finds nothing.

OxFunc's provisional reference-family contract admits the trim rows as "structural
reference-target normalization only" and records whitespace-trimmed reference forms such as
`SUM(( A1 ))` as transparent on the seeded slice. So on the admitted evidence this operator
changes no computed value; it exists so that the whitespace is modelled rather than silently
absorbed.

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | The reference whose trailing whitespace is normalized. Required. |

Arity is exactly 1. The signature display recorded in `data/functions/` for this row is `A@`,
which is not a typed form — no reader types `A@` to invoke it, and the `@` glyph is a
placeholder, not the implicit-intersection operator
([`FUNC.OP_IMPLICIT_INTERSECTION`](FUNC.OP_IMPLICIT_INTERSECTION.md)). The operator-inventory
source records the surface syntax for this row as the placeholder `<trim-trailing>`, which is
the honest statement: there is no spelling.

## Result and edge cases

Returns a `Reference` (`KernelSignatureClass::Custom`,
`ArgPreparationProfile::RefsVisibleInAdapter`).

- **Ordinary case.** The operand reference, unchanged.
- **Followed by another reference.** Then the space is not this operator; it is the
  intersection operator. Which reading applies is decided by grammar, and the contract puts
  parser whitespace ownership beyond the trim slice explicitly out of scope — so the exact
  boundary is not stated by any document this Handbook can cite.
- **Before a closing parenthesis or an argument separator.** The padding reading, on the
  admitted slice.
- **Non-reference operand.** Outside the admitted slice.

## Errors

| Error | Condition |
|---|---|
| `#REF!` | The operand is not a usable reference. |
| any incoming error | An error operand propagates. |

`data/functions/FUNC.OP_TRIM_REF_TRAILING.json` records no Microsoft documentation URL
(`docs` is `null`). No cited document states an error surface specific to this row.

## Relationships

- [`FUNC.OP_TRIM_REF_LEADING`](FUNC.OP_TRIM_REF_LEADING.md) and
  [`FUNC.OP_TRIM_REF_BOTH`](FUNC.OP_TRIM_REF_BOTH.md) — the other two positions.
- [`FUNC.OP_INTERSECTION_REF`](FUNC.OP_INTERSECTION_REF.md) — the competing reading of the
  same character, and the operator this row is most easily confused with in a parser.
- [`FUNC.OP_RANGE_REF`](FUNC.OP_RANGE_REF.md), [`FUNC.OP_UNION_REF`](FUNC.OP_UNION_REF.md) —
  the constructors whose operands may carry padding on either side.
- `TRIM` — the text function. Unrelated: values, not syntax.

## Notes for implementers

- Resolve trailing space by lookahead in one place. If the tokenizer and the binder each
  guess, formulas like `=SUM(A1 )` and `=SUM(A1 B1)` will eventually be handled
  inconsistently, and the failure surfaces as a spurious `#NULL!` — which readers will report
  as an intersection bug, not a whitespace bug.
- Preserve the whitespace in the stored form. Normalizing it away rewrites the user's
  formula.
- Keep the trim identities distinct from the intersection identity in whatever
  representation the evaluator uses, so that a later pass can still tell which reading was
  chosen.

## What has not been checked

No Handbook vector suite covers this operator, no Excel-comparison evidence record is
attached to this page, and whether the row has any user-observable behaviour of its own is
itself unresolved.

Probes worth running first:

1. **The lookahead boundary.** `=SUM(A1 )`, `=SUM(A1 B1)`, `=SUM(A1 ,B1)`, `=SUM((A1 ))` —
   the minimal set that separates padding from intersection, with `#NULL!` as the tell when
   the intersection reading wins.
2. **Value transparency.** The padded forms against their unpadded equivalents.
3. **Stored-form round-trip.** Save and reopen, reading the stored formula text to see
   whether the trailing space survives.
4. **Whitespace classes.** Tab, non-breaking space and line break in the trailing position.
5. **Adjacent operators.** `A1 #`, `A1 :B2` and `A1 ^2`, where a trailing space meets another
   operator's token.

## Page vocabulary

| Machine name | Meaning |
|---|---|
| `Arity { min: 1, max: 1 }` | Exactly one operand |
| `ArgPreparationProfile::RefsVisibleInAdapter` | The operator sees live references and controls dereference itself |
| `KernelSignatureClass::Custom` | Kernel shape is function-specific |
| `HostInteractionClass::WorkbookState` | The operator's meaning depends on workbook state |
| `reference_normalization` | The operator-inventory family this row belongs to |

## Sources

- `data/functions/FUNC.OP_TRIM_REF_TRAILING.json` at OxFunc `473efa3` — identity, arity 1,
  the `A@` signature display, `RefsVisibleInAdapter` preparation. `docs` is `null`: **no
  Microsoft documentation URL is recorded for this entry.**
- `data/presence/FUNC.OP_TRIM_REF_TRAILING.json` — implementing module
  `crates/oxfunc_core/src/functions/operator_reference_family.rs`.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OPERATOR_REFERENCE_FAMILY_CONTRACT_PRELIM.md` —
  the normalization-only admission, the `SUM(( A1 ))` transparency finding, and the
  out-of-slice statement on parser whitespace ownership. Provisional by its own header.
- OxFunc `docs/function-lane/W45_NON_AT_OPERATOR_INVENTORY.csv` — the `<trim-trailing>`
  surface-syntax placeholder and the `reference_normalization` family.
- Handbook `content/model/01-value-universe.md`, `03-call-pipeline.md`, and the
  [`FUNC.OP_INTERSECTION_REF`](FUNC.OP_INTERSECTION_REF.md) page for the competing reading.
