---
schema: efh.function-page/v1
function_id: FUNC.OP_LESS_EQUAL
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
family: operator_compare_concat_family
role_in_family: "Non-strict less-than: the operator that inherits the family's equality normalization at its boundary, which is where a strict and a non-strict comparison can disagree."
---

## What it computes

`A <= B` returns `TRUE` when `A` precedes `B` in Excel's comparison order **or** is equal to
it under that order, and `FALSE` otherwise.

The comparison order and the equality it uses are the family's, stated in full on
[`FUNC.OP_EQUAL`](FUNC.OP_EQUAL.md): kinds are ordered rather than coerced (number < text <
logical), text comparison is case-insensitive, and numeric comparison passes through a
normalization lane instead of exact IEEE comparison.

That last rule is what gives this operator its own character. Because the equality half is
normalized, `<=` is `TRUE` in a band around the boundary where exact arithmetic would say
`FALSE`. OxFunc's provisional contract records the pair directly: `=0.1+0.2<0.3` yields
`FALSE` while `=0.1+0.2<=0.3` yields `TRUE`. The operands are the same; the difference is
entirely the normalized equality. A workbook that guards a threshold with `<=` therefore
admits values a `<` guard would reject, by a margin set not by the author but by the
comparison layer's normalization width.

Everything else follows from `<` and `=`.

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | Left operand. Required. |
| 1 | `B` | Right operand. Required. |

Arity is exactly 2; no optional arguments, no defaults. Order is load-bearing.

## Result and edge cases

Returns a `Logical` (`KernelSignatureClass::Custom`).

- **Equal operands** give `TRUE`, under the family's equality — which includes the
  case-insensitive text rule, so `"a" <= "A"` is `TRUE` in both directions.
- **Different kinds** give the kind ordering; the equality half never fires across kinds,
  because operands of different kinds are never equal.
- **Blank operands.** Context-sensitive: blank behaves as `0` against a number, `""` against
  text, `FALSE` against a logical. A blank cell is therefore `<= 0`, `<= ""` and `<= FALSE`,
  all `TRUE`.
- **Arrays.** Ordinary broadcast with `#N/A` for unsupplied coordinates.

## Errors

| Error | Condition |
|---|---|
| any incoming error | Propagates unchanged; `ErrorCollapseProfile::None`. |
| `#N/A` | Inside an array result only: a coordinate neither operand supplies. |

`data/functions/FUNC.OP_LESS_EQUAL.json` records no Microsoft documentation URL (`docs` is
`null`).

## Relationships

- [`FUNC.OP_LESS_THAN`](FUNC.OP_LESS_THAN.md) — the strict form, and the operator with which
  `<=` disagrees exactly on the normalized-equality band.
- [`FUNC.OP_GREATER_EQUAL`](FUNC.OP_GREATER_EQUAL.md) — the mirror.
- [`FUNC.OP_EQUAL`](FUNC.OP_EQUAL.md) — the equality half.
- `MEDIAN`, `PERCENTILE`, `QUARTILE`, histogram-style `FREQUENCY` — functions whose bucket
  boundaries are defined with a closed side. Their internal boundary convention is their own,
  documented per function; do not assume it matches this operator's normalized equality.
- `COUNTIFS(range,"<=5")` — a criteria string, parsed by the consuming function.

## Notes for implementers

- Derive `<=` from the same three-way comparison core as the other five operators. In
  particular, the equality it uses must be *the* equality, normalization included; a `<=`
  implemented as "exact less-than or normalized-equal", or the reverse, produces a subtly
  different relation.
- `A<=B` must not be implemented as `NOT(A>B)` while errors are in play: the error must
  propagate as an error, not be negated into a logical.
- If the comparison core is a tolerance model rather than a normalization model, `<=` will
  be non-transitive near the boundary. Record which model you implemented.

## What has not been checked

No Handbook vector suite covers `<=`, and no Excel-comparison evidence record is attached to
this page. Nothing here is a measurement.

Probes worth running first:

1. **The disagreement band.** Sweep pairs `(a, b)` from clearly-less to clearly-greater
   through the normalization boundary, recording `<`, `<=`, `=`, `>` and `>=` for each. The
   width of the band where `<` and `<=` disagree *is* the normalization width, measured
   directly — the most informative single experiment available for the whole comparison
   family.
2. **Boundary construction from arithmetic.** Build operands by arithmetic rather than by
   typing literals, since a typed literal is re-parsed and may not land on the intended
   double. The contract's own stronger boundary lane is built that way.
3. **Antisymmetry.** `A<=B` and `B<=A` both `TRUE` should imply the operands compare equal;
   check that on the boundary band, where a tolerance model would break it.
4. **Blank operands.** `=A1<=0`, `=A1<=""`, `=A1<=FALSE` with `A1` empty.
5. **Case pairs.** `="a"<="A"` and `="A"<="a"`, both of which the case-insensitive rule
   predicts `TRUE`.

## Page vocabulary

| Machine name | Meaning |
|---|---|
| `Arity { min: 2, max: 2 }` | Exactly two operands |
| `ArgPreparationProfile::ValuesOnlyPreAdapter` | References resolved to values before the operator runs |
| `CoercionLiftProfile::Custom` | Operator-specific comparison rules |
| `KernelSignatureClass::Custom` | Kernel shape is function-specific |
| `LiftBroadcastProfile::SurfaceNative` | The operator does its own array lifting |
| `ErrorCollapseProfile::None` | Error operands propagate; no precedence folding |
| `Logical` | The return kind |

## Sources

- `data/functions/FUNC.OP_LESS_EQUAL.json` at OxFunc `473efa3` — identity, arity, signature
  `A <= B`, classification. `docs` is `null`: **no Microsoft documentation URL is recorded
  for this entry.** Microsoft's account of the comparison operators lives in the support
  article *Calculation operators and precedence in Excel*, not yet linked from the data
  projection.
- `data/presence/FUNC.OP_LESS_EQUAL.json` — implementing module
  `crates/oxfunc_core/src/functions/operator_compare_concat_family.rs`.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OPERATOR_COMPARE_CONCAT_FAMILY_CONTRACT_PRELIM.md`
  — the normalization model, the blank lanes, and the `=0.1+0.2<0.3` / `=0.1+0.2<=0.3`
  finding pair quoted above; provisional by its own header.
- Handbook `content/model/01-value-universe.md`, `02-coercion-and-lifting.md`,
  `03-call-pipeline.md`, and [`FUNC.OP_EQUAL`](FUNC.OP_EQUAL.md) for the three rules.
