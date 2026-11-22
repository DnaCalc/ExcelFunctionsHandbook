---
schema: efh.function-page/v1
function_id: FUNC.OP_GREATER_EQUAL
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
family: operator_compare_concat_family
role_in_family: "Non-strict greater-than: the threshold operator of real workbooks, and the mirror of `<=` on the family's normalized-equality band."
---

## What it computes

`A >= B` returns `TRUE` when `A` follows `B` in Excel's comparison order **or** is equal to
it under that order, and `FALSE` otherwise.

The order and the equality are the family's, stated in full on
[`FUNC.OP_EQUAL`](FUNC.OP_EQUAL.md): kinds are ordered rather than coerced (number < text <
logical), text comparison is case-insensitive, and numeric comparison passes through a
normalization lane rather than exact IEEE comparison.

The normalized equality is what distinguishes this operator from `>`. OxFunc's provisional
contract records the pair: `=0.1+0.2>0.3` yields `FALSE` while `=0.1+0.2>=0.3` yields
`TRUE`. The operands are identical; the difference is the equality half firing on a pair that
exact arithmetic would call unequal.

That matters because `>=` is the operator business logic is actually written with — minimum
balances, qualifying thresholds, tier boundaries. A rule spelled `>=` includes values that
differ from the threshold by less than the comparison layer's normalization width, whether or
not the author intended a tolerance. The author cannot see this width in the formula; it
comes from the comparison layer.

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | Left operand. Required. |
| 1 | `B` | Right operand, typically the threshold. Required. |

Arity is exactly 2; no optional arguments, no defaults. Order is load-bearing.

## Result and edge cases

Returns a `Logical` (`KernelSignatureClass::Custom`).

- **Equal operands** give `TRUE`, under the family's normalized, case-insensitive equality.
- **Different kinds** give the kind ordering; the equality half never fires across kinds. In
  particular `="10">=2` is `TRUE` because the text outranks the number, not because ten is
  at least two — a threshold test over a column of text-formatted numbers passes
  unconditionally.
- **Blank operands.** Context-sensitive: blank behaves as `0` against a number, `""` against
  text, `FALSE` against a logical. So `=A1>=0` is `TRUE` for an empty `A1`. A "did they meet
  the minimum?" test written with `>=0` counts every empty row as a pass.
- **Arrays.** Ordinary broadcast with `#N/A` for unsupplied coordinates.

## Errors

| Error | Condition |
|---|---|
| any incoming error | Propagates unchanged; `ErrorCollapseProfile::None`. |
| `#N/A` | Inside an array result only: a coordinate neither operand supplies. |

`data/functions/FUNC.OP_GREATER_EQUAL.json` records no Microsoft documentation URL (`docs` is
`null`).

## Relationships

- [`FUNC.OP_GREATER_THAN`](FUNC.OP_GREATER_THAN.md) — the strict form, and the operator with
  which `>=` disagrees exactly on the normalized-equality band.
- [`FUNC.OP_LESS_EQUAL`](FUNC.OP_LESS_EQUAL.md) — the mirror.
- [`FUNC.OP_EQUAL`](FUNC.OP_EQUAL.md) — the equality half.
- `MATCH` with match type `1`, `VLOOKUP`/`HLOOKUP` in approximate mode, `XLOOKUP` with a
  next-smaller match mode — the lookup functions whose semantics are "largest value less than
  or equal to". Their internal comparison is their own; whether it is this operator's
  relation is exactly the kind of thing a vector suite would have to establish.
- `COUNTIFS(range,">=5")` — criteria strings, parsed by the consuming function.
- `IFS`, `SWITCH`, nested `IF` tier ladders — where `>=` chains are written, and where an
  off-by-one-normalization-width can put a row in the wrong tier.

## Notes for implementers

- Derive `>=` from the same three-way comparison core as the other five. In particular its
  equality must be the family's equality, normalization and case-insensitivity included.
- Do not implement `A>=B` as `NOT(A<B)` while errors are in play; the error must propagate.
- If your comparison core normalizes, document the normalization width and direction
  (truncation versus rounding) as part of the implementation's contract — it is user-visible
  behaviour at every threshold in every workbook that uses this operator.

## What has not been checked

No Handbook vector suite covers `>=`, and no Excel-comparison evidence record is attached to
this page. Nothing here is a measurement.

Probes worth running first:

1. **The band, from the greater side.** Sweep the normalization boundary recording `>` and
   `>=` together; the disagreement band should mirror the one measured with `<`/`<=`. If it
   does not mirror, the comparison core is not symmetric and that is a significant finding.
2. **Direction of the normalization.** Pairs whose 16th digit rounds up across the boundary,
   to separate truncation from rounding — the contract names truncation-style as the current
   local model and marks it as such.
3. **Blank as a passing value.** `=A1>=0` with `A1` empty, and the same test inside
   `COUNTIF`/`COUNTIFS`, since the aggregate scan policy and the operator may differ.
4. **Text thresholds.** `="10">=2` and `="abc">=2`, confirming that the kind order makes
   every text pass a numeric threshold.
5. **Lookup agreement.** Compare `MATCH(x, sorted_range, 1)` against an explicit
   `>=`-based scan over the same data containing mixed kinds. Agreement or divergence maps
   whether the lookup functions share this operator's ordering.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `OP_GREATER_EQUAL` that an implementer matching Excel, or a reader
explaining an unexpected Excel result, needs to know. Each note gives exact binary64 inputs,
Excel's answer and where it was recorded; "measured" means the statement is read off recorded
Excel answers, "inferred" marks what goes beyond them. Where older text on this page says a
behaviour was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

### FN-OP_GREATER_EQUAL-001: Comparison operators >=, <=, and <> compare kind-ordered values without coercion under en-ZA collation

*Quirk; measured; recorded 2026-11-22.* Applies to OP_GREATER_EQUAL, OP_LESS_EQUAL, OP_NOT_EQUAL.

In Excel 16.0.20527.20088, >=, <=, and <> share Excel's non-coercive comparison rules: numbers,
text, and logicals compare in distinct ordered kinds (so =A1<>2 returns TRUE when cell A1 holds
text "2"); errors propagate left-to-right without numeric coercion (#N/A<>"a" and "a"<>#N/A
both return #N/A, while #DIV/0!<=#N/A returns #DIV/0!); and text compares case-insensitively
under en-ZA collation ("a"<="A" and "B">="b" are TRUE, "æ"<="ae" is TRUE, and "é"<>"e" is
TRUE).

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=A1<>2` | as typed | `TRUE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `="a"<="A"` | as typed | `TRUE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=1>=1` | as typed | `TRUE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=1<=0` | as typed | `FALSE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `="é"<>"e"` | as typed | `TRUE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `="æ"<="ae"` | as typed | `TRUE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=#DIV/0!<=#N/A` | as typed | `#DIV/0!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `="a">=#N/A` | as typed | `#N/A` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Measured across Excel 16.0.20527.20088 in cfunc_m1_2 (mixed_operands rows 61,
76-78; error_order rows 31-35, 41-45, 51-55) and cdoc_jpha (text_and_number_comparison rows 7,
940, 941, 943).

- Open: 15-significant-digit numeric normalization near 16th-digit ties and binary64 top-binade comparisons share the comparison kernel tracked under dc-j9k.

Evidence:

- DnaCalc `crates/calc-func/src/catalogue/comparison.rs`, lines 46-54, 100-151, and 195-240 at `494f964897` — Shared comparison kernel for =, <>, <, <=, >, >=
- DnaCalc `crates/calc-func/tests/captures/cfunc_m1_2/oracle.capture.json`, mixed_operands rows 61, 76-78; error_order rows 31-35, 41-45, 51-55 at `494f964897` — Kind ordering, non-coercion, and left-to-right error precedence
- DnaCalc `crates/calc-func/tests/captures/cdoc_jpha/oracle.capture.json`, text_and_number_comparison rows 7, 940, 941, 943 at `494f964897` — en-ZA case-insensitive collation under <=, >=, and <>
- DnaCalc `crates/calc-func/tests/captures/cfunc_m1_2/oracle.capture.json`, sheet mixed_operands row 61 column 4 at `494f964897` — mixed_operands!A1 holds text "2" (readings)
- DnaCalc `crates/calc-func/tests/captures/cfunc_m1_2/oracle.capture.json`, sheet mixed_operands row 76 column 4 at `494f964897` (readings)
- DnaCalc `crates/calc-func/tests/captures/cfunc_m1_2/oracle.capture.json`, sheet mixed_operands row 77 column 4 at `494f964897` (readings)
- DnaCalc `crates/calc-func/tests/captures/cfunc_m1_2/oracle.capture.json`, sheet mixed_operands row 78 column 4 at `494f964897` (readings)
- DnaCalc `crates/calc-func/tests/captures/cdoc_jpha/oracle.capture.json`, sheet text_and_number_comparison row 940 column 4 at `494f964897` (readings)
- DnaCalc `crates/calc-func/tests/captures/cdoc_jpha/oracle.capture.json`, sheet text_and_number_comparison row 943 column 4 at `494f964897` (readings)
- DnaCalc `crates/calc-func/tests/captures/cfunc_m1_2/oracle.capture.json`, sheet error_order row 42 column 4 at `494f964897` (readings)
- DnaCalc `crates/calc-func/tests/captures/cfunc_m1_2/oracle.capture.json`, sheet error_order row 55 column 4 at `494f964897` (readings)

<!-- efh:notes end -->

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

- `data/functions/FUNC.OP_GREATER_EQUAL.json` at OxFunc `473efa3` — identity, arity,
  signature `A >= B`, classification. `docs` is `null`: **no Microsoft documentation URL is
  recorded for this entry.** Microsoft's account of the comparison operators lives in the
  support article *Calculation operators and precedence in Excel*, not yet linked from the
  data projection.
- `data/presence/FUNC.OP_GREATER_EQUAL.json` — implementing module
  `crates/oxfunc_core/src/functions/operator_compare_concat_family.rs`.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OPERATOR_COMPARE_CONCAT_FAMILY_CONTRACT_PRELIM.md`
  — the normalization model, blank lanes and the `=0.1+0.2>0.3` / `=0.1+0.2>=0.3` finding
  pair; provisional by its own header.
- Handbook `content/model/01-value-universe.md`, `02-coercion-and-lifting.md`,
  `03-call-pipeline.md`, and [`FUNC.OP_EQUAL`](FUNC.OP_EQUAL.md) for the three rules.
