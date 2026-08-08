---
schema: efh.function-page/v1
function_id: FUNC.OP_ADD
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
family: op_add
role_in_family: "The sole member: `+` is the seed operator slice, and the only operator with a module of its own rather than a place in one of the operator family modules."
---

## What it computes

`A + B` converts each operand to a number and returns their IEEE-754 binary64 sum, rounded
once to nearest with ties to even.

The arithmetic is the easy half. The interesting half is everything the sentence assumes:

1. **`+` never sees a cell.** Argument preparation resolves references to plain values before
   the operator runs (`ArgPreparationProfile::ValuesOnlyPreAdapter`), so `A1+A2` and `3+4`
   reach the same kernel by the same route. See
   [the call pipeline](../model/03-call-pipeline.md).
2. **The conversion is per-operand and can fail.** Each operand goes through the shared
   to-number rules of [coercion and lifting](../model/02-coercion-and-lifting.md): a number is
   itself; a logical is 1 or 0; text that reads as a number parses; text that does not is a
   named coercion failure; an error value propagates rather than converting.
3. **The result is one rounded double, not an exact sum.** `+` is therefore not associative:
   `(a+b)+c` and `a+(b+c)` are different functions of the same three numbers, and a workbook
   that reassociates a chain of `+` can change its last bits.

`+` is a two-argument function in this model, not a variadic one. `SUM` is a different
function with a different coercion policy, not `+` folded over a list — the direct-argument
versus range-scan asymmetry described in chapter 02 is exactly where the two part company.

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | Left addend. Required. |
| 1 | `B` | Right addend. Required. |

Arity is exactly 2 (`Arity { min: 2, max: 2 }`); there are no optional arguments and no
defaults. `+` is commutative on numbers up to the sign of a zero result, so operand order is
not semantically load-bearing the way it is for `-`, `/` and `^` — but it is still two
positions, and an array-shaped left operand and an array-shaped right operand broadcast
against each other by the rules below.

The typed surface has a second `+` that is not this function: prefix `+x` is
[`FUNC.OP_UNARY_PLUS`](FUNC.OP_UNARY_PLUS.md), a one-argument operator with materially
different operand handling. Confusing the two is the most common misreading of this row.

## Result and edge cases

Returns a `Number`. The declared kernel class is `KernelSignatureClass::NumsToNum`.

- **Text operands.** Governed by the shared to-number rule, not by anything specific to `+`.
  The locale-dependent part of Excel's text-to-number recognizer (thousands separators,
  currency symbols, date text) is flagged as an open area in chapter 02 and is not settled
  here.
- **Logical operands.** Convert to 1 and 0 by the shared rule.
- **Empty and omitted.** Chapter 02 keeps `Empty` (a blank cell) and `Missing` (an omitted
  argument slot) distinct at the call boundary, and states that what a function *does* with
  each is per-family policy. This Handbook has not recorded `+`'s policy for a blank operand
  against Excel. It is the first thing to probe.
- **Arrays.** The declared lift axis is `LiftBroadcastProfile::SurfaceNative`: the operator's
  own evaluation does the lifting. OxFunc's provisional arithmetic-family contract records
  scalar/array, array/scalar, same-shape array/array and row-vs-column outer-product grids as
  the admitted broadcast shapes, with coordinates neither operand can supply returning `#N/A`
  — the ordinary dynamic-array broadcast rule of chapter 02, applied per cell so that one
  failing coordinate does not collapse the whole result.
- **Overflow.** A binary64 sum can overflow to infinity, and chapter 01 states that a cell
  never publishes an infinity. The entry's real-result policy is recorded as
  `non_finite=allow`, meaning "the kernel cannot produce a non-finite result" — but
  `data/functions/FUNC.OP_ADD.json` marks that axis `default-unexamined`, so it is a
  projection default rather than an examined fact. Treat the overflow lane as unresolved.

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | An operand is text that does not read as a number (a to-number failure). |
| any incoming error | An error operand propagates; `ErrorCollapseProfile::None` means `+` does no folding or precedence selection of its own. |

Microsoft's operator documentation for this row is not linked from
`data/functions/FUNC.OP_ADD.json` (its `docs` field is `null`), so the conditions above are
stated from the shared call-model chapters and OxFunc's provisional slice contract, not from
a cited Microsoft page for `+` itself.

## Relationships

- [`FUNC.OP_SUBTRACT`](FUNC.OP_SUBTRACT.md), [`FUNC.OP_MULTIPLY`](FUNC.OP_MULTIPLY.md),
  [`FUNC.OP_DIVIDE`](FUNC.OP_DIVIDE.md), [`FUNC.OP_POWER`](FUNC.OP_POWER.md) — the other
  binary arithmetic operators. They share the coercion story and differ in domain and error
  set.
- [`FUNC.OP_UNARY_PLUS`](FUNC.OP_UNARY_PLUS.md) — same character, different function.
- `SUM` — the aggregate. `A1+A2` and `SUM(A1:A2)` are not interchangeable: `SUM` scans a
  range under the ignore-text-and-empty policy, while `+` coerces its operands directly.
  With `"3"` stored as text in `A1`, that difference is visible in the result.
- `IMSUM` — complex addition over text-encoded complex numbers; unrelated machinery.

## Notes for implementers

- Implement `+` as a two-operand kernel over already-coerced doubles and put every
  interesting decision in the coercion and broadcast layers, where it is shared and testable
  once. That is the shape the reference implementation uses, and it is why the operator's own
  module is small.
- Do not reassociate. A compiler or expression optimizer that rewrites `(a+b)+c` into
  `a+(b+c)` changes results in the last bits and breaks any comparison against Excel.
- Broadcast is not `zip`. Row-vs-column operands produce a two-dimensional grid, not a
  diagonal, and missing coordinates are `#N/A` values inside the result array, not a
  whole-call failure.
- `+` is declared `SafePure`, `Deterministic` and `NonVolatile`, with no host interaction, so
  it is safe to evaluate off the calculation thread.

## What has not been checked

No Handbook vector suite exists for `+` — `vectors/` publishes nothing for this entry — and
no Excel-comparison evidence record is attached to this page (`evidence_records` is empty).
Nothing on this page is a measurement.

What would settle the open questions, in the order worth probing:

1. **Blank operands.** `=A1+1` and `=A1+A2` with the referenced cells genuinely empty, and
   the same expressions with an omitted-argument shape where the grammar allows one. Chapter
   02 says the Empty-versus-Missing treatment is per-family policy; this family's policy is
   unrecorded.
2. **Overflow.** Operands whose exact sum exceeds the finite binary64 range, to decide
   whether the published result is an error or a finite saturation — the recorded
   `non_finite=allow` axis is a default-unexamined projection value and may simply be wrong.
3. **Text recognizer edges.** Operands such as `"1,234"`, `"$5"`, `"1e3"`, `" 7 "` and date
   text, under at least two locales, to pin how much of Excel's text-to-number grammar `+`
   inherits.
4. **Signed zero.** `0 + -0` and `-0 + 0`, to establish whether the sign of a zero result
   survives into anything observable (division by the result is the usual detector).
5. **Broadcast padding.** Row-vs-column and non-conformable operands, to confirm the `#N/A`
   padding rule holds for `+` specifically and not merely for the family as a whole.

## Page vocabulary

| Machine name | Meaning |
|---|---|
| `Arity { min: 2, max: 2 }` | Exactly two operands |
| `ArgPreparationProfile::ValuesOnlyPreAdapter` | References resolved to values before the operator runs |
| `CoercionLiftProfile::Custom` | Operator-specific coercion, not one of the shared categories |
| `KernelSignatureClass::NumsToNum` | Kernel maps several numbers to one number |
| `LiftBroadcastProfile::SurfaceNative` | The operator does its own array lifting |
| `ErrorCollapseProfile::None` | Error operands propagate; no folding or precedence selection |
| `PrecisionRoundingProfile::Default` | Publishes the plain IEEE-754 kernel result |
| `default-unexamined` | Axis provenance: the value is a projection default, not an examined fact |

## Sources

- `data/functions/FUNC.OP_ADD.json` at OxFunc `473efa3` — identity, arity, signature `A + B`,
  classification and axis provenance. `docs` is `null`: **no Microsoft documentation URL is
  recorded for this operator entry.** Microsoft's own account of `+` lives in the support
  article *Calculation operators and precedence in Excel*, which this Handbook has not yet
  linked from the entry.
- `data/presence/FUNC.OP_ADD.json` — implementing module
  `crates/oxfunc_core/src/functions/op_add.rs`, giving this page its family slug.
- Handbook `content/model/01-value-universe.md`, `02-coercion-and-lifting.md`,
  `03-call-pipeline.md` — the value kinds, to-number rules, Empty/Missing distinction,
  broadcast rule, and the "operators are functions" model this page relies on.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OP_ADD_CONTRACT_PRELIM.md` and
  `FUNCTION_SLICE_OPERATOR_ARITHMETIC_FAMILY_CONTRACT_PRELIM.md` — the admitted operand and
  broadcast lanes. Both are marked provisional by their own headers.
- OxFunc `docs/bugs/streams/BUG-FUNC-001_binary_operator_array_lift_value_surface_gap.md` —
  the record that binary-operator array transport was once narrower than the surface implied.
  Cited as the origin of the broadcast question, not as a Handbook verification.
