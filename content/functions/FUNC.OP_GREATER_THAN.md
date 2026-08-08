---
schema: efh.function-page/v1
function_id: FUNC.OP_GREATER_THAN
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
role_in_family: "Strict greater-than: the mirror of `<`, and the operator that carries the family's two published cross-type findings."
---

## What it computes

`A > B` returns `TRUE` when `A` follows `B` in Excel's comparison order, and `FALSE`
otherwise.

The order is the family's, stated in full on [`FUNC.OP_EQUAL`](FUNC.OP_EQUAL.md): kinds are
ordered rather than coerced, text comparison is case-insensitive, numeric comparison is
normalized rather than exact. Two of OxFunc's current-baseline findings for the family are
written in this operator, and they are the two that pin the cross-type order:

- `="10">2` yields `TRUE` — every text outranks every number.
- `=TRUE>0` yields `TRUE` — every logical outranks every number.

Both are stated by the contract as empirically observed type ordering rather than numeric-text
coercion. They are worth memorizing in this direction, because `>` is the operator readers
actually write when they filter: `=A1>100` behaves quite differently on a column of numbers
than on a column of numbers stored as text, and the difference is not a rounding problem, it
is a kind problem. Text beats the threshold unconditionally.

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | Left operand: the candidate greater value. Required. |
| 1 | `B` | Right operand: the candidate lesser value. Required. |

Arity is exactly 2; no optional arguments, no defaults. Order is load-bearing.

## Result and edge cases

Returns a `Logical` (`KernelSignatureClass::Custom`).

- **Equal operands** give `FALSE`; `>` is strict, and its equality is the family's normalized
  one, so a pair that differs in the 16th significant digit is expected to give `FALSE`
  rather than `TRUE`.
- **Different kinds** give the kind ordering: number < text < logical.
- **Blank operands.** Context-sensitive: blank behaves as `0` against a number, `""` against
  text, `FALSE` against a logical. So `=A1>0` is `FALSE` for an empty `A1`, but `=A1>-1` is
  `TRUE` — an empty cell participates in comparisons as a value, not as an absence, and
  `ISBLANK` is the function that asks the other question.
- **Text ordering.** Case-insensitive; collation beyond that is out of the contract's slice.
- **Arrays.** Ordinary broadcast with `#N/A` for unsupplied coordinates.

## Errors

| Error | Condition |
|---|---|
| any incoming error | Propagates unchanged; `ErrorCollapseProfile::None`. |
| `#N/A` | Inside an array result only: a coordinate neither operand supplies. |

`data/functions/FUNC.OP_GREATER_THAN.json` records no Microsoft documentation URL (`docs` is
`null`).

## Relationships

- [`FUNC.OP_LESS_THAN`](FUNC.OP_LESS_THAN.md) — the mirror.
- [`FUNC.OP_GREATER_EQUAL`](FUNC.OP_GREATER_EQUAL.md) — the non-strict form.
- [`FUNC.OP_EQUAL`](FUNC.OP_EQUAL.md) — the rules.
- `MAX`, `LARGE`, `SORT`, `FILTER` — the ordering and selection functions. `MAX` ignores text
  in a scanned range while `>` ranks text above every number; the two do not agree about what
  "largest" means, and that is by design, not by defect.
- `COUNTIF(range,">100")`, `SUMIF`, `AVERAGEIF`, `MAXIFS` — the criteria mini-language. The
  string `">100"` is parsed by those functions; it is not this operator, and its treatment of
  text and blanks in the scanned range follows the aggregate's scan policy, not this page.
- Conditional formatting and data-validation rules use comparison expressions evaluated in
  their own contexts; the operator is the same, the surrounding coercion may not be.

## Notes for implementers

- Derive `>` from the same three-way comparison core as `<`. Writing it separately is how
  mirror inconsistencies get in.
- The cross-type rule must be checked before any attempt to interpret text as a number. An
  implementation that "helpfully" parses `"10"` before comparing gets `="10">2` wrong in the
  most visible possible way.
- Errors propagate; do not fold them, and do not let a `>` inside a criteria evaluation take
  a different path from a `>` in a formula.

## What has not been checked

No Handbook vector suite covers `>`, and no Excel-comparison evidence record is attached to
this page. The two cross-type findings above are reported from OxFunc's provisional contract,
not measured by the Handbook.

Probes worth running first:

1. **The cross-type matrix.** Every ordered kind pair in both directions with `>` and `<`, to
   turn two observations into a complete, total ordering claim.
2. **Text that looks numeric, at scale.** `="10">2`, `="10">20`, `="2">"10"`, `=2>"10"` — the
   four combinations that separate "text outranks numbers" from any residual numeric
   interpretation.
3. **Blank versus threshold.** `=A1>0`, `=A1>-1`, `=A1>""`, `=A1>FALSE` with `A1` empty.
4. **Mirror consistency.** `=A>B` against `=B<A` over the same corpus.
5. **Criteria versus operator.** `=COUNTIF(range,">100")` against
   `=SUMPRODUCT(--(range>100))` over a range containing numbers, numeric text, blanks and
   logicals. Any divergence maps the boundary between the criteria mini-language and this
   operator, which is a genuinely under-documented seam.

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

- `data/functions/FUNC.OP_GREATER_THAN.json` at OxFunc `473efa3` — identity, arity, signature
  `A > B`, classification. `docs` is `null`: **no Microsoft documentation URL is recorded for
  this entry.** Microsoft's account of the comparison operators lives in the support article
  *Calculation operators and precedence in Excel*, not yet linked from the data projection.
- `data/presence/FUNC.OP_GREATER_THAN.json` — implementing module
  `crates/oxfunc_core/src/functions/operator_compare_concat_family.rs`.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OPERATOR_COMPARE_CONCAT_FAMILY_CONTRACT_PRELIM.md`
  — the type-ordering lane and the `="10">2`, `=TRUE>0` current-baseline findings;
  provisional by its own header.
- Handbook `content/model/01-value-universe.md`, `02-coercion-and-lifting.md`,
  `03-call-pipeline.md`, and [`FUNC.OP_EQUAL`](FUNC.OP_EQUAL.md) for the three rules.
