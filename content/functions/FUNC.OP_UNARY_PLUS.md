---
schema: efh.function-page/v1
function_id: FUNC.OP_UNARY_PLUS
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
role_in_family: "Prefix plus: the family's identity operator, and the row where the Handbook's projected metadata and an upstream Excel observation currently describe two different functions."
---

## What it computes

`+A` returns its operand. The whole question on this page is *how much* of the operand it
returns — the value alone, or the value together with its type.

There are two candidate readings, and they are not compatible:

**Reading 1 — coercing identity.** `+A` converts its operand to a number by the shared
to-number rules and returns that number. Under this reading `+"2"` is the number 2 and
`+TRUE` is the number 1, and `+A` is `-(-A)` with one negation elided. This is what the
Handbook's projected metadata for the entry currently describes:
`data/functions/FUNC.OP_UNARY_PLUS.json` at OxFunc `473efa3` records
`kernel_signature_class: NumToNum` and
`coercion_lift_profile: UnaryNumericScalarOrArrayElementwise` — a numeric-in, numeric-out
kernel.

**Reading 2 — type-preserving identity.** `+A` returns the operand unchanged, including its
kind: text stays text, a logical stays a logical, a number stays that number. Under this
reading `+"2"` is the *text* `2` and `+TRUE` is the *logical* `TRUE`.

OxFunc's defect stream `BUG-FUNC-029` records reading 2 as the observed Excel behaviour. Its
reproduction table, taken against live Excel 16.0 build 20026 with workbook Compatibility
Version 2, lists `=+"2"` yielding text and `=+TRUE` yielding a logical, against a local
implementation that produced numbers for both. The stream's root-cause line states the
principle directly: unary plus is "a type-preserving identity — it returns the operand
unchanged, including text and logical operands", while unary minus "does coerce-and-negate".

**This page does not assert which reading is currently in force.** The stream carries its own
status field and its own account of what was changed; the Handbook has not re-measured `+A`
against Excel, and the data projection it publishes still describes reading 1. What the
Handbook can say honestly is: the two readings are both on the record, they disagree on
observable results for text and logical operands, and the disagreement is exactly the kind of
thing a vector suite exists to settle.

For number operands the two readings agree, which is why the discrepancy survived: almost
every real use of `+A` in a workbook has a numeric operand.

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | The operand. Required. |

Arity is exactly 1; no optional arguments, no defaults.

The same `+` character in infix position is [`FUNC.OP_ADD`](FUNC.OP_ADD.md), a two-argument
function with a different operand doctrine. A leading `+` typed at the start of a formula
(`+A1*2`, the habit inherited from Lotus 1-2-3 keyboards) is this operator applied to the
rest of the expression — which is why the distinction is not academic: millions of formulas
begin with it.

## Result and edge cases

Return kind depends on which reading holds — `Number` under reading 1, the operand's own kind
under reading 2. The projected `KernelSignatureClass::NumToNum` reflects reading 1.

- **Number operand.** Both readings return the same number. The `BUG-FUNC-029` fix note
  additionally describes an underflow normalization on the number path; the Handbook has not
  characterized that behaviour and does not restate it as a rule here.
- **Text operand.** The pivot case. Reading 1 coerces (and fails with `#VALUE!` on
  non-numeric text); reading 2 returns the text unchanged, so `+"abc"` would be `"abc"`
  rather than an error. These differ not only in value but in whether an error occurs at all.
- **Logical operand.** Reading 1 gives 1 or 0; reading 2 gives `TRUE` or `FALSE`.
- **Blank operand.** The `BUG-FUNC-029` record describes blank/empty mapping to 0 among the
  operand semantics it measured before coding. Cited as an upstream observation, not as a
  Handbook claim.
- **Error operand.** Propagates under both readings.
- **Arrays.** Elementwise under both readings; under reading 2 the mapping preserves each
  element's own kind.

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | Under reading 1 only: the operand is text that does not read as a number. Under reading 2 this case does not arise. |
| any incoming error | Propagates unchanged; `ErrorCollapseProfile::None`. |

`data/functions/FUNC.OP_UNARY_PLUS.json` records no Microsoft documentation URL (`docs` is
`null`). No Microsoft page is cited here for the type-preservation question, and the
Handbook has not found one that settles it.

## Relationships

- [`FUNC.OP_NEGATE`](FUNC.OP_NEGATE.md) — the coercing unary sibling. The pair is the clean
  experiment: run identical operands through `-A` and `+A` and the coercion doctrine of each
  becomes visible in one screenshot.
- [`FUNC.OP_ADD`](FUNC.OP_ADD.md) — same character, two operands.
- `N`, `VALUE`, `T` — the explicit coercion functions. If a formula wants a number, one of
  these says so; `+A` does not, under either reading.
- The `--A` double-unary idiom uses [`FUNC.OP_NEGATE`](FUNC.OP_NEGATE.md) twice, not this
  operator, and its coercing effect is well established — which is itself circumstantial
  support for reading 2, since nobody would need `--` if `+` already coerced.

## Notes for implementers

- Decide the reading explicitly and record it. An implementation that silently routes `+A`
  through the shared unary-numeric path has chosen reading 1 by omission — which is exactly
  the shape `BUG-FUNC-029` describes as its root cause (a coercing surface reused for an
  identity operator).
- If you implement reading 2, the operator is not a numeric kernel at all and does not belong
  behind a `NumToNum` signature; its array path must map kinds elementwise, not values.
- Under either reading, error operands propagate and the operator is `SafePure`,
  `Deterministic`, `NonVolatile`.
- Do not let the leading-`+` typing habit reach a different code path from an inner `+A`.
  They are the same operator.

## What has not been checked

No Handbook vector suite covers `+A`, and no Excel-comparison evidence record is attached to
this page. The reading-2 observations above come from an upstream OxFunc defect record; the
Handbook has neither re-measured them nor recorded which reading the current implementation
or the current Excel build presents.

The probes that would settle it, in order:

1. **The two pivot cells.** `=+"2"` and `=+TRUE`, with the result's *type* read through
   `ISTEXT`, `ISNUMBER` and `ISLOGICAL` rather than by looking at the cell, since both
   readings display identically.
2. **Non-numeric text.** `=+"abc"`. Reading 1 predicts `#VALUE!`; reading 2 predicts the text
   back. This single cell discriminates the readings with no ambiguity.
3. **Blank and error operands.** `=+A1` with `A1` empty, and `=+NA()`.
4. **Array operands.** `=+{1,"a",TRUE}` with per-element type inspection.
5. **Build spread.** Whichever answer comes back, repeat it on a second Excel build and
   channel: the upstream observation is pinned to one named build, and a type-preservation
   rule is exactly the kind of thing that could differ across compatibility versions.
6. **The `POWER`-style cross-check.** Compare `+A` against `N(A)` and against `A` alone in
   the same workbook; agreement patterns identify the reading without needing type functions.

## Page vocabulary

| Machine name | Meaning |
|---|---|
| `Arity { min: 1, max: 1 }` | Exactly one operand |
| `ArgPreparationProfile::ValuesOnlyPreAdapter` | References resolved to values before the operator runs |
| `CoercionLiftProfile::UnaryNumericScalarOrArrayElementwise` | Projected axis: one numeric argument, elementwise over arrays (reading 1) |
| `KernelSignatureClass::NumToNum` | Projected axis: kernel maps one number to one number (reading 1) |
| `ErrorCollapseProfile::None` | Error operands propagate unchanged |
| `default-unexamined` | Axis provenance: a projection default, not an examined fact |

## Sources

- `data/functions/FUNC.OP_UNARY_PLUS.json` at OxFunc `473efa3` — identity, arity, signature
  `+A`, and the `NumToNum` / `UnaryNumericScalarOrArrayElementwise` classification that
  encodes reading 1. `docs` is `null`: **no Microsoft documentation URL is recorded for this
  entry.**
- `data/presence/FUNC.OP_UNARY_PLUS.json` — implementing module
  `crates/oxfunc_core/src/functions/operator_arithmetic_family.rs`, and the listing of
  `BUG-FUNC-029` as a behavioural-finding document for this row.
- OxFunc `docs/bugs/streams/BUG-FUNC-029_unary_plus_over_coerces_text_and_logical.md` — the
  type-preserving-identity statement, the `=+"2"` / `=+TRUE` reproduction table, the named
  Excel build and workbook compatibility version, and the blank/empty operand note. Cited as
  an upstream record; its status and remediation are not restated as Handbook claims.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OPERATOR_ARITHMETIC_FAMILY_CONTRACT_PRELIM.md` —
  the contract text that lists this row as "values-only numeric coercion", i.e. reading 1;
  provisional by its own header.
- Handbook `content/model/01-value-universe.md`, `02-coercion-and-lifting.md`,
  `03-call-pipeline.md`, `06-claim-language.md`.
