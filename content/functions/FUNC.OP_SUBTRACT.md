---
schema: efh.function-page/v1
function_id: FUNC.OP_SUBTRACT
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
family: operator_arithmetic_family
role_in_family: "The binary minus: the only member whose typed form is also the prefix form of another member, and the family's canonical cancellation hazard."
---

## What it computes

`A - B` converts each operand to a number and returns the IEEE-754 binary64 difference,
rounded once to nearest with ties to even.

Subtraction is where floating-point arithmetic stops being invisible. Two properties are
worth stating precisely, because they generate most of the questions readers arrive with:

1. **The subtraction itself is exact when the operands are close.** By Sterbenz's lemma, if
   `B/2 ≤ A ≤ 2B` then `A - B` is representable exactly and the operation introduces no error
   at all. The damage was done earlier: catastrophic cancellation is the *revelation* of
   error already present in `A` and `B`, not error created by `-`.
2. **`A - B` is not `A + (-B)` in every observable respect.** They agree on the finite
   arithmetic, but they are different call shapes with different operand counts, and the
   sign of a zero result differs: `x - x` is a positive zero under round-to-nearest, while
   `x + (-x)` is too, but `0 - 0` and `0 + -0` are not the same expression. If anything in a
   workbook can observe the sign of a zero, the rewrite is not neutral.

As with every operator in this family, the operand conversion is the part that varies: `-`
never sees a cell, only values already resolved and coerced by the shared rules of
[coercion and lifting](../model/02-coercion-and-lifting.md).

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | Minuend. Required. |
| 1 | `B` | Subtrahend. Required. |

Arity is exactly 2; no optional arguments, no defaults. Order is load-bearing —
`A - B` and `B - A` differ in sign — which distinguishes this row from `+` and `*`.

The same `-` character in prefix position is [`FUNC.OP_NEGATE`](FUNC.OP_NEGATE.md), a
one-argument operator. Which one a given `-` is depends on the formula grammar, and formula
grammar is explicitly out of this Handbook's scope (`CHARTER.md` section 4).

## Result and edge cases

Returns a `Number` (`KernelSignatureClass::NumsToNum`).

- **Text and logical operands.** Governed by the shared to-number rules, exactly as for
  [`+`](FUNC.OP_ADD.md). Nothing about `-` changes them.
- **Dates.** Date arithmetic is subtraction: a date is a number wearing a format, so
  `date2 - date1` is an ordinary difference of serial numbers and the "number of days" reading
  is a presentation fact, not a semantic one. The published result carries no date format of
  its own; how the host formats the difference cell is host-side adaptation, outside the
  function's semantics (chapter 03).
- **Empty and omitted operands.** Per-family policy under chapter 02, and unrecorded here.
- **Arrays.** `LiftBroadcastProfile::SurfaceNative`. OxFunc's provisional arithmetic-family
  contract records the same admitted broadcast shapes as the rest of the family —
  scalar/array, array/scalar, same-shape, and row-vs-column outer product — with
  coordinates neither operand can supply returning `#N/A`.
- **Overflow.** The recorded real-result policy is `non_finite=allow`, and
  `data/functions/FUNC.OP_SUBTRACT.json` marks that axis `default-unexamined`. A difference
  of two large opposite-signed operands can overflow; what Excel publishes there is not
  settled on this page.

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | An operand is text that does not read as a number. |
| any incoming error | Propagates unchanged; `ErrorCollapseProfile::None`. |

No Microsoft documentation URL is recorded for this entry (`docs` is `null` in
`data/functions/FUNC.OP_SUBTRACT.json`), so these conditions come from the shared call-model
chapters and OxFunc's provisional contract rather than from a cited Microsoft page.

## Relationships

- [`FUNC.OP_ADD`](FUNC.OP_ADD.md) — the additive sibling; same coercion, same broadcast
  shapes.
- [`FUNC.OP_NEGATE`](FUNC.OP_NEGATE.md) — prefix `-`. Same character, one operand.
- `IMSUB` — complex subtraction over text-encoded complex numbers.
- `DAYS`, `DATEDIF`, `YEARFRAC` — the calendar-aware alternatives to subtracting two date
  serials. They answer different questions; `-` answers only "difference of two numbers".

## Notes for implementers

- Do not "optimize" `A - B` into `A + (-B)` or vice versa in a compatibility implementation.
  The finite results agree, but the call shape, the operand count, and the zero-sign story do
  not, and a compatibility target should preserve the shape it was given.
- Cancellation is a modelling problem, not an operator problem. If a workbook loses
  significance in a subtraction, the fix belongs in the expression that produced the
  operands, not in a "better" subtraction — there is no better subtraction; the IEEE
  difference is already exact when it matters.
- Broadcast and coercion are shared machinery. Implementing `-` as a distinct code path from
  `+` invites the two to drift; the reference implementation keeps them in one family module
  for exactly that reason.

## What has not been checked

No Handbook vector suite covers `-`, and no Excel-comparison evidence record is attached to
this page. Nothing here is a measurement.

The probes that would settle the open questions:

1. **Blank operands.** `=A1-A2` with both cells empty, and `=A1-1` with `A1` empty, to pin
   the family's Empty policy against Excel.
2. **Overflow.** Operands near the finite binary64 boundary with opposite signs, to decide
   the published result.
3. **Signed zero.** `=5-5` fed into `1/result`, to detect whether the zero's sign is
   observable at all.
4. **Cancellation ladders.** Pairs like `(1+2^-52) - 1` across the exponent range, checked
   against a correctly rounded reference, to establish whether the published difference is
   the plain IEEE result or something Excel post-processes. Excel's display rounding is a
   known confounder here and must be defeated by reading raw bits, not cell text.
5. **Date subtraction across the 1900 leap-year artifact.** Differences spanning
   1900-02-28/1900-03-01, where Excel's serial numbering has a documented historical
   discontinuity.

## Page vocabulary

| Machine name | Meaning |
|---|---|
| `Arity { min: 2, max: 2 }` | Exactly two operands |
| `ArgPreparationProfile::ValuesOnlyPreAdapter` | References resolved to values before the operator runs |
| `CoercionLiftProfile::Custom` | Operator-specific coercion |
| `KernelSignatureClass::NumsToNum` | Kernel maps several numbers to one number |
| `LiftBroadcastProfile::SurfaceNative` | The operator does its own array lifting |
| `ErrorCollapseProfile::None` | Error operands propagate unchanged |
| `default-unexamined` | Axis provenance: a projection default, not an examined fact |

## Sources

- `data/functions/FUNC.OP_SUBTRACT.json` at OxFunc `473efa3` — identity, arity, signature
  `A - B`, classification, axis provenance. `docs` is `null`: **no Microsoft documentation
  URL is recorded for this entry.** Microsoft's account of `-` lives in the support article
  *Calculation operators and precedence in Excel*, not yet linked from the data projection.
- `data/presence/FUNC.OP_SUBTRACT.json` — implementing module
  `crates/oxfunc_core/src/functions/operator_arithmetic_family.rs`, the family slug for this
  page.
- Handbook `content/model/01-value-universe.md`, `02-coercion-and-lifting.md`,
  `03-call-pipeline.md`.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OPERATOR_ARITHMETIC_FAMILY_CONTRACT_PRELIM.md` —
  admitted operand and broadcast lanes; marked provisional by its own header.
- Sterbenz's lemma is standard floating-point theory (Sterbenz, *Floating-Point Computation*,
  1974); it is a property of IEEE-754 arithmetic, not a claim about Excel.
