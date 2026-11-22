---
schema: efh.function-page/v1
function_id: FUNC.OP_SPILL_REF
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
family: op_spill_ref
role_in_family: "The sole member: the spill-anchor operator has its own module because it forms a reference kind no other operator produces."
---

## What it computes

`A#` takes a reference to a cell that anchors a spilled dynamic-array result and returns a
reference to the whole spilled region — however large it currently is.

The essential idea is *deferred extent*. `B1#` does not mean "B1 through B10"; it means "the
region B1 spilled", and that region is resolved when the reference is used. A formula written
against `B1#` follows the spill as it grows and shrinks, which is what makes dynamic arrays
composable without helper columns or volatile functions.

OxFunc's provisional slice contract for this operator is unusually clear about the division
of labour, and it is worth stating because it explains every error on this page:

- The operator **forms spill-anchor reference identity**. Successful evaluation returns a
  `Reference` whose shape is `SpillAnchor` (chapter 01's reference-shape table).
- The operator does **not** resolve spill size or even spill existence. "Spill existence and
  spill-range materialization are downstream resolver concerns, not operator-formation
  concerns."

So `A#` is a constructor, not a lookup. Asking whether anything actually spilled at `A1`
happens later, in whatever consumes the reference.

The contract also records that an already-spill-anchor target remains stable under a second
application, so `A1##` does not compound into anything new.

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | The anchor reference: the top-left cell of a spilled result. Required. |

Arity is exactly 1; no optional arguments, no defaults. The operator is postfix.

The operand must be an anchor-shaped reference. The contract records the admitted runtime
slice as A1-style *single-cell* anchors, already-tagged spill anchors, and non-A1 symbolic
anchor names passed through as spill-anchor text — and records that multi-cell A1 areas are
rejected as invalid anchors. A range is not an anchor: `A1:B2#` has no meaning under that
slice.

Because the operand must survive as a reference, this operator is declared
`ArgPreparationProfile::RefsVisibleInAdapter`, like the rest of the reference algebra.

## Result and edge cases

Returns a `Reference` with the `SpillAnchor` shape (`KernelSignatureClass::Custom`).

- **Nothing spilled at the anchor.** The reference still forms; the failure surfaces
  downstream when the region is resolved. The contract records downstream dereference
  failures as resolver-level `#REF!` behaviour.
- **Anchor cell holds a scalar.** A single-cell result is a spill of size 1 in the dynamic-
  array model; whether `A#` on such a cell is valid is not settled here.
- **Invalid anchor shape.** A multi-cell operand is rejected in the admitted slice, surfacing
  as `#REF!`.
- **The reference is live.** The extent is whatever the spill currently is, so a formula
  consuming `A#` recalculates when the spill's size changes. That is the operator's purpose,
  and also its main performance consideration.
- **No coercion, no lifting.** This operator sits outside the scalar coercion story
  entirely.

## Errors

| Error | Condition |
|---|---|
| `#REF!` | The operand is not a usable anchor (including a multi-cell operand in the admitted slice), or a downstream dereference of the formed reference fails. |
| `#SPILL!` | Not produced by this operator. It is the error of a spill that *could not happen* — something blocking the target region — and belongs to the formula that spills, not to the operator that names the result. |
| any incoming error | An error operand propagates. |

`data/functions/FUNC.OP_SPILL_REF.json` records no Microsoft documentation URL (`docs` is
`null`), so these conditions come from chapter 01's error table and OxFunc's provisional
slice contract.

## Relationships

- [`FUNC.OP_RANGE_REF`](FUNC.OP_RANGE_REF.md) — the static way to name a region, by corners.
  `A1#` and `A1:C3` may designate the same cells today and different cells tomorrow.
- [`FUNC.OP_IMPLICIT_INTERSECTION`](FUNC.OP_IMPLICIT_INTERSECTION.md) — the operator that
  goes the other way, collapsing an array-or-reference result to a single value. `@` and `#`
  are the two halves of the dynamic-array compatibility story: one refuses spilling, the
  other consumes it.
- `SINGLE`/`_xlfn.SINGLE` is recorded in chapter 05 as the compatibility/serialization
  representation of `@`. This Handbook has **not** recorded the corresponding
  compatibility-serialization name for `#`, and does not assert one here.
- `ANCHORARRAY` — commonly reported as the serialized form of `#` in older builds. Not
  recorded in this Handbook's data, and stated here only as a pointer for someone probing the
  file format, not as a claim.
- `ROWS`, `COLUMNS`, `COUNTA`, `SUMPRODUCT` — typical consumers that resolve the region.
- `TAKE`, `DROP`, `CHOOSEROWS`, `SORT`, `FILTER` — the modern array functions whose results
  are usually what `#` is pointed at.

## Notes for implementers

- Keep formation and resolution separate, exactly as the contract does. An implementation
  that resolves the spill region inside the operator cannot represent "the anchor exists but
  the spill does not yet", and will report the wrong error at the wrong time.
- Validate anchor shape at formation: single cell, or an already-tagged anchor. Reject
  multi-cell operands rather than silently taking their top-left.
- Make the reference re-resolve on recalculation. A cached extent is a correctness bug that
  looks like a stale-value bug.
- Record the dependency on the anchor's spill extent, not merely on the anchor cell, or the
  calculation chain will miss updates when the spill grows.
- The operator interacts with workbook state (`HostInteractionClass::WorkbookState`); it is
  not a pure value function even though it takes one operand.

## What has not been checked

No Handbook vector suite covers `#`, and no Excel-comparison evidence record is attached to
this page. The formation-versus-resolution split above is reported from OxFunc's provisional
contract, not measured by the Handbook.

Probes worth running first:

1. **Anchor with no spill.** `=SUM(A1#)` where `A1` holds an ordinary scalar, and where `A1`
   is empty — to see which error appears and at what point.
2. **Growth and shrink.** Point a formula at `A1#`, then change the spilling formula's size,
   confirming the consumer follows the new extent and recalculates.
3. **Invalid anchors.** `=A1:B2#`, `=#`, and `#` applied to a defined name and to a
   structured reference.
4. **Serialization round-trip.** Save a workbook containing `A1#`, inspect the stored formula
   text, and reopen in an older build — this is the experiment that would settle the
   compatibility-name question this page declines to assert.
5. **Interaction with `@`.** `=@A1#`, which combines the two dynamic-array operators and is
   exactly the shape where a compatibility layer is most likely to differ from a modern one.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `OP_SPILL_REF` that an implementer matching Excel, or a reader explaining
an unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's
answer and where it was recorded; "measured" means the statement is read off recorded Excel
answers, "inferred" marks what goes beyond them. Where older text on this page says a behaviour
was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

### FN-OP_SPILL_REF-001: Spill reference (A1#) tracks dynamic-array extent including 1x1 errors and returns #REF! on non-spills

*Quirk; measured; recorded 2026-11-22.*

In Excel 16.0.20527, the postfix spill operator A1# resolves to the spilled region anchored at
A1 (ROWS(fn2_q009!J20#) = 2 and COLUMNS(fn2_q009!J20#) = 2 for a 2x2 spill; ROWS(J20#) = 1 and
COLUMNS(J20#) = 4 in fn7_format_context, with INDEX(J20#,2,1) = #REF!). A dynamic-array formula
evaluating to a single error cell still acts as a 1x1 spill anchor (ROWS(fn2_q001!J20#) = 1
where J20 holds =EXPAND(#N/A,0,3,"pad")), whereas referencing a cell with no spilling formula
returns #REF! (ROWS(fn2_q123!J20#) = #REF!). In .xlsx files, Excel 16.0 build 20430 saves A1#
as _xlfn.ANCHORARRAY(A1), but typing =ANCHORARRAY(B1) or =_xlfn.ANCHORARRAY(B1) at worksheet
entry is refused (0x800A03EC).

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=ROWS(fn2_q009!J20#)` | as typed | `2.0` (`0x4000000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=ROWS(fn2_q001!J20#)` | as typed | `1.0` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=ROWS(fn2_q123!J20#)` | as typed | `#REF!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=COLUMNS(J20#)` | as typed | `4.0` (`0x4010000000000000`) | Excel 16.0.20527.20150 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=INDEX(J20#,2,1)` | as typed | `#REF!` | Excel 16.0.20527.20150 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Measured across Excel 16.0.20527 in fn2_questions (478 observer # formulas
across fn2_o001..fn2_o197), fn7_format_context (144 J20# formulas), excel-entry.tsv (B1# and
ANCHORARRAY entry probes), and Excel 16.0 build 20430 in d1/excel-file-form.json (lines
291-311).

Evidence:

- DnaCalc `crates/calc-fml/src/eval/run.rs`, lines 2507-2530 at `494f964897` — Spill reference (#) evaluation and #REF! on non-spill/multi-cell anchors
- DnaCalc `crates/calc-fml/tests/acceptance/l1.rs`, lines 984-1014 at `494f964897` — Spill reference acceptance checks and ANCHORARRAY grammar refusal
- DnaCalc `crates/calc-doc/src/save/file_form.rs`, lines 12-13 at `494f964897` — _xlfn.ANCHORARRAY file-form serialization rule
- DnaCalc `crates/calc-doc/tests/fixtures/d1/excel-file-form.json`, lines 291-311 at `494f964897` — Excel 16.0 build 20430 saved _xlfn.ANCHORARRAY XML and cm="1" flags
- DnaCalc `crates/calc-fml/tests/fixtures/entry/excel-entry.tsv`, lines 135, 236, 618, and 3806-3808 at `494f964897` — Excel 16.0.20527 entry acceptance of B1# vs refusal of ANCHORARRAY
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_o009 row 2 column 4 at `494f964897` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_o001 row 2 column 4 at `494f964897` — fn2_q001!J20 holds =EXPAND(#N/A,0,3,"pad") evaluating to #N/A (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_o123 row 2 column 4 at `494f964897` — fn2_q123!J20 holds no spilling formula (readings)
- DnaCalc `crates/calc-func/tests/captures/fn7_format_context/oracle.capture.json`, sheet q001 row 3 column 4 at `494f964897` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn7_format_context/oracle.capture.json`, sheet q001 row 8 column 4 at `494f964897` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Machine name | Meaning |
|---|---|
| `Arity { min: 1, max: 1 }` | Exactly one operand |
| `ArgPreparationProfile::RefsVisibleInAdapter` | The operator sees live references and controls dereference itself |
| `KernelSignatureClass::Custom` | Kernel shape is function-specific |
| `HostInteractionClass::WorkbookState` | The operator's meaning depends on workbook state |
| `SpillAnchor` | The reference shape produced: the region spilled from an anchor |
| `#SPILL!` | The error of a blocked spill — produced by the spilling formula, not by this operator |

## Sources

- `data/functions/FUNC.OP_SPILL_REF.json` at OxFunc `473efa3` — identity, arity, signature
  `A#`, `RefsVisibleInAdapter` preparation, `WorkbookState` host interaction. `docs` is
  `null`: **no Microsoft documentation URL is recorded for this entry.**
- `data/presence/FUNC.OP_SPILL_REF.json` — implementing module
  `crates/oxfunc_core/src/functions/op_spill_ref.rs`, the family slug for this page.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OP_SPILL_REF_CONTRACT_PRELIM.md` — the admitted
  anchor shapes, the `SpillAnchor` result identity, the formation-versus-resolution split,
  the stability of an already-anchor target, the multi-cell rejection, and the resolver-level
  `#REF!` behaviour. Marked provisional by its own header.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OPERATOR_REFERENCE_FAMILY_CONTRACT_PRELIM.md` —
  reconciles this operator into the wider reference-operator packet.
- Handbook `content/model/01-value-universe.md` (reference shapes including spill anchor;
  `#SPILL!` and `#REF!`), `03-call-pipeline.md`, `05-version-axes.md` (the `@` /
  `_xlfn.SINGLE` compatibility-representation record, cited for contrast).
