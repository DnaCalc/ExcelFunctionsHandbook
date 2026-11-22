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
  - Notes
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

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `OP_LESS_EQUAL` that an implementer matching Excel, or a reader
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
