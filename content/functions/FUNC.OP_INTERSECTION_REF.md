---
schema: efh.function-page/v1
function_id: FUNC.OP_INTERSECTION_REF
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
role_in_family: "The space operator: the family's meet operation, the only operator spelled with whitespace, and the sole producer of `#NULL!`."
---

## What it computes

`A B` — two references separated by a space — returns the reference designating the cells
belonging to both.

It is the intersection to [`FUNC.OP_RANGE_REF`](FUNC.OP_RANGE_REF.md)'s bounding box, and its
result is again rectangular, because the intersection of two rectangles is a rectangle (or
nothing). OxFunc's provisional reference-family contract records `SUM((A1:C3 B2:D4))` and
`ROWS((A1:C3 B2:D4))` as current-baseline findings — the second confirming that what comes
back is a shaped reference, not a scalar.

**When the operands do not overlap, the result is `#NULL!`.** The contract records this
directly, and chapter 01's error table defines `#NULL!` as exactly that: the intersection of
ranges that do not intersect, produced by the space operator. This is the only place in the
entire function surface where `#NULL!` originates. If a workbook shows `#NULL!`, an
intersection somewhere came up empty — very often an accidental one, because the operator is
spelled with a character that is otherwise invisible.

That invisibility is the operator's defining hazard. `=SUM(A1:A5 B1:B5)` looks like a typo
for a comma and evaluates as an intersection; `=A1 A2` is a valid expression. Whitespace
inside a formula is not always insignificant, and this is why.

The classic *intentional* use is with defined names: if `Revenue` names a column and `Q2`
names a row, then `=Revenue Q2` reads the cell where they cross. That idiom is the reason the
operator exists and is worth knowing even if you never write another one.

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | First reference. Required. |
| 1 | `B` | Second reference. Required. |

Arity is exactly 2; no optional arguments, no defaults. Intersection is commutative and
associative on areas, so chains behave as expected. Both operands must be reference-shaped;
the contract records the admitted operand shapes as A1-style cells, areas, whole rows and
whole columns.

## Result and edge cases

Returns a `Reference` (`KernelSignatureClass::Custom`).

- **No overlap.** `#NULL!`.
- **Partial overlap.** The overlapping rectangle, which may be a single cell, a row segment,
  a column segment, or a block.
- **Whole-row against whole-column.** `1:1 A:A` is the single cell at their crossing — the
  mechanism behind the defined-name idiom above.
- **Multi-area operands.** Intersecting a multi-area reference (from
  [`FUNC.OP_UNION_REF`](FUNC.OP_UNION_REF.md)) with an area is not in the contract's admitted
  slice, and the result shape is not settled here.
- **Non-reference operands.** No intersection exists; the result is an error.
- **Cross-sheet operands.** Not covered by the admitted slice.

## Errors

| Error | Condition |
|---|---|
| `#NULL!` | The operands designate no common cell. |
| `#REF!` | An operand is not a usable reference, or is no longer valid. |
| any incoming error | An error operand propagates. |

`data/functions/FUNC.OP_INTERSECTION_REF.json` records no Microsoft documentation URL
(`docs` is `null`), so these conditions come from chapter 01's error table and OxFunc's
provisional reference-family contract.

## Relationships

- [`FUNC.OP_RANGE_REF`](FUNC.OP_RANGE_REF.md) — the join; this operator is the meet.
- [`FUNC.OP_UNION_REF`](FUNC.OP_UNION_REF.md) — the list constructor.
- [`FUNC.OP_IMPLICIT_INTERSECTION`](FUNC.OP_IMPLICIT_INTERSECTION.md) — a *different*
  operation with a confusingly similar name. `@` scalarizes one operand relative to the
  calling cell; this operator intersects two references with each other. They are not
  variants of one idea.
- [`FUNC.OP_TRIM_REF_LEADING`](FUNC.OP_TRIM_REF_LEADING.md) and its siblings — the
  whitespace-normalization operators, which exist because whitespace around references is
  meaningful enough to need identities of its own.
- `INDEX`, `OFFSET`, `XLOOKUP` — the modern, explicit ways to name a cell at the crossing of
  two coordinates, and what most workbooks should use instead.
- `IFERROR` — because `#NULL!` is an error like any other and can be trapped.

## Notes for implementers

- Intersect as rectangles: take the maximum of the tops and lefts and the minimum of the
  bottoms and rights, then check for emptiness. Emptiness is `#NULL!`, not an empty
  reference — there is no such value in chapter 01's admission matrix.
- The empty check must come before any dereference. Producing an empty area and letting a
  consumer fail on it gives the wrong error code.
- Whitespace handling is a parser responsibility, but the operator identity must not be lost
  in it: a formula's stored form records the spacing, which is exactly why the trim-reference
  operators exist as separate identities.
- Sheet-qualified and multi-area operands are outside the contract's admitted slice; an
  implementation should state what it does rather than assume.

## What has not been checked

No Handbook vector suite covers the space operator, and no Excel-comparison evidence record
is attached to this page. The findings above are reported from OxFunc's provisional contract,
not measured by the Handbook.

Probes worth running first:

1. **The `#NULL!` boundary.** Adjacent-but-disjoint areas (`A1:B2 C3:D4`), touching areas
   (`A1:B2 B2:C3` — a single-cell overlap), and nested areas, to confirm exactly where the
   error begins.
2. **Shape of the result.** `=ROWS(...)`, `=COLUMNS(...)` and `=AREAS(...)` applied to an
   intersection, to confirm it is a reference of the expected shape and area count.
3. **Multi-area operands.** `=SUM((A1:A3,C1:C3) A2:C2)`, outside the admitted slice and
   therefore genuinely unknown.
4. **Cross-sheet intersection.** `=Sheet1!A1:C3 Sheet1!B1:B5` versus operands on different
   sheets.
5. **Whitespace forms.** Multiple spaces, tabs and line breaks between operands, and spacing
   inside a parenthesized operand, to map where the operator is recognized and where the
   trim-reference identities take over instead.

## Page vocabulary

| Machine name | Meaning |
|---|---|
| `Arity { min: 2, max: 2 }` | Exactly two operands |
| `ArgPreparationProfile::RefsVisibleInAdapter` | The operator sees live references and controls dereference itself |
| `KernelSignatureClass::Custom` | Kernel shape is function-specific |
| `HostInteractionClass::WorkbookState` | The operator's meaning depends on workbook state |
| `#NULL!` | The error for a non-intersecting intersection; unique to this operator |

## Sources

- `data/functions/FUNC.OP_INTERSECTION_REF.json` at OxFunc `473efa3` — identity, arity,
  signature `A B`, `RefsVisibleInAdapter` preparation, `WorkbookState` host interaction.
  `docs` is `null`: **no Microsoft documentation URL is recorded for this entry.**
- `data/presence/FUNC.OP_INTERSECTION_REF.json` — implementing module
  `crates/oxfunc_core/src/functions/operator_reference_family.rs`.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OPERATOR_REFERENCE_FAMILY_CONTRACT_PRELIM.md` —
  admitted operand shapes, the `SUM((A1:C3 B2:D4))` and `ROWS((A1:C3 B2:D4))` findings, and
  the recorded rule that no-overlap intersection yields `#NULL!`. Provisional by its own
  header.
- Handbook `content/model/01-value-universe.md` — the `#NULL!` definition ("the intersection
  of ranges that do not intersect (the space operator)") and the reference shapes.
- Handbook `content/model/03-call-pipeline.md` — operators as functions; reference-visible
  argument preparation.
