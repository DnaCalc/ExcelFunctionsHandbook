---
schema: efh.function-page/v1
function_id: FUNC.OP_NOT_EQUAL
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
role_in_family: "Inequality: the complement of `=` on values, but not on errors — and the operator whose spelling readers most often get wrong."
---

## What it computes

`A <> B` returns `TRUE` when the operands are *not* equal under Excel's comparison rules, and
`FALSE` when they are.

Those rules are stated in full on [`FUNC.OP_EQUAL`](FUNC.OP_EQUAL.md) and are inherited
unchanged: operands of different kinds are ordered rather than coerced (so `1 <> "1"` is
`TRUE`), text comparison is case-insensitive (so `"a" <> "A"` is `FALSE`), and numeric
comparison goes through a normalization lane rather than exact IEEE equality (so
`0.1+0.2 <> 0.3` is recorded as `FALSE` in OxFunc's contract findings). Everything on this
page is about the *negation*, which is the only part that is not `=`.

**`<>` is the complement of `=` on values, and not on errors.** For two ordinary values,
exactly one of `A=B` and `A<>B` is `TRUE`. But an error operand propagates through both:
`NA()=1` and `NA()<>1` are both `#N/A`, so neither is `TRUE` and the law of excluded middle
does not hold on the error lane. A formula that assumes `IF(A<>B, x, y)` covers every case
covers every case *except* the one where an operand is an error — and there the whole `IF`
becomes the error. This is the practical content of `ErrorCollapseProfile::None`.

The spelling is `<>`, not `!=` and not `≠`. Excel's grammar admits only the two-character
form.

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | Left operand. Required. |
| 1 | `B` | Right operand. Required. |

Arity is exactly 2; no optional arguments, no defaults. Symmetric in value, but the operand
positions still set broadcast orientation for array operands.

## Result and edge cases

Returns a `Logical` (`KernelSignatureClass::Custom`).

- **Different kinds.** Always `TRUE`, by the kind-ordering rule. A number is never equal to
  text that looks like it, so it is always unequal to it.
- **Blank operands.** The context-sensitive blank rule of the family applies: blank behaves
  as `0` against a number, `""` against text, `FALSE` against a logical. A blank cell is
  therefore *not* unequal to `0` and *not* unequal to `""`, though `0 <> ""` is `TRUE`.
  Testing "is this cell empty?" with `<>""` is consequently not a test for emptiness — it is
  a test that is also satisfied by numeric zero in the blank case, and `ISBLANK` is the
  function that means what the reader intended.
- **Arrays.** Ordinary broadcast, `#N/A` for coordinates neither operand supplies.
- **Errors.** Propagate; see above.

## Errors

| Error | Condition |
|---|---|
| any incoming error | Propagates unchanged, so `A<>B` is *not* guaranteed to be the logical negation of `A=B`. |
| `#N/A` | Inside an array result only: a coordinate neither operand supplies. |

`data/functions/FUNC.OP_NOT_EQUAL.json` records no Microsoft documentation URL (`docs` is
`null`).

## Relationships

- [`FUNC.OP_EQUAL`](FUNC.OP_EQUAL.md) — the rules this page inherits.
- `NOT` — `NOT(A=B)` and `A<>B` agree on values and differ in error handling only in that
  `NOT` will itself propagate the error it receives; both are error-transparent, but the
  rewrite is worth stating deliberately rather than assuming.
- `COUNTIF(range,"<>")`, `COUNTIFS`, `SUMIF` — the criteria mini-language, where the bare
  string `"<>"` conventionally means "not empty". That is a criteria-parser convention
  belonging to those functions, not a use of this operator, and its rules differ.
- `EXACT` — for case-sensitive difference, `NOT(EXACT(a,b))` rather than `<>`.

## Notes for implementers

- Implement `<>` as the negation of the *same* equality routine `=` uses, not as an
  independently written inequality. Two routines drift; one cannot.
- Negate after the error check, never before. The error lane must short-circuit to the error
  value, or the operator will report `TRUE` for comparisons involving errors.
- In the array case, negate per coordinate, leaving `#N/A` padding as `#N/A` rather than
  turning it into `TRUE`.

## What has not been checked

No Handbook vector suite covers `<>`, and no Excel-comparison evidence record is attached to
this page. Nothing here is a measurement.

Probes worth running first:

1. **Complementarity sweep.** For a corpus spanning all kind pairs, blanks and errors, check
   `A=B` and `A<>B` in adjacent cells and confirm exactly one is `TRUE` except on the error
   lane. Any other exception would be a genuine discovery.
2. **The blank trap.** `=A1<>""` and `=A1<>0` with `A1` empty, alongside `ISBLANK(A1)`, to
   pin the blank rule from the inequality side.
3. **Case folding.** `="a"<>"A"` and non-ASCII case pairs, under at least two locales.
4. **Numeric normalization from the negative side.** The same near-boundary pairs used for
   `=` — a normalization rule must give consistent answers to both operators, and a
   disagreement would show the two operators do not share a comparison core.
5. **Arrays with padding.** `={"a","b"}<>{"x","y","z"}`, to confirm the padded coordinate is
   `#N/A` and not `TRUE`.

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

- `data/functions/FUNC.OP_NOT_EQUAL.json` at OxFunc `473efa3` — identity, arity, signature
  `A <> B`, classification. `docs` is `null`: **no Microsoft documentation URL is recorded
  for this entry.** Microsoft's account of the comparison operators lives in the support
  article *Calculation operators and precedence in Excel*, not yet linked from the data
  projection.
- `data/presence/FUNC.OP_NOT_EQUAL.json` — implementing module
  `crates/oxfunc_core/src/functions/operator_compare_concat_family.rs`.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OPERATOR_COMPARE_CONCAT_FAMILY_CONTRACT_PRELIM.md`
  — the shared comparison lanes and the `=0.1+0.2<>0.3` current-baseline finding; provisional
  by its own header.
- Handbook `content/model/01-value-universe.md`, `02-coercion-and-lifting.md`,
  `03-call-pipeline.md`, and this Handbook's [`FUNC.OP_EQUAL`](FUNC.OP_EQUAL.md) page for the
  three comparison rules.
