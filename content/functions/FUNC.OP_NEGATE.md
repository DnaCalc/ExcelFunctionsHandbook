---
schema: efh.function-page/v1
function_id: FUNC.OP_NEGATE
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
role_in_family: "Prefix minus: the family's coercing unary operator, and the deliberate contrast case for unary plus."
---

## What it computes

`-A` converts its single operand to a number and returns that number's negation.

The important word is *converts*. Unary minus is a coercing operator: it takes whatever it is
given, runs it through the shared to-number rules of
[coercion and lifting](../model/02-coercion-and-lifting.md), and returns a number. Text that
reads as a number becomes a negative number; a logical becomes `-1` or `0`. The result kind
is always `Number` (`KernelSignatureClass::NumToNum`).

This is exactly where `-A` and `+A` part company. OxFunc's defect record `BUG-FUNC-029`
states the contrast in one line: unary minus "does coerce-and-negate; unary plus must not
coerce". See [`FUNC.OP_UNARY_PLUS`](FUNC.OP_UNARY_PLUS.md) for that side of the story. The
asymmetry looks like an accident of history and is a genuine, load-bearing difference in
behaviour.

On the arithmetic itself: negation of a binary64 value is exact. It flips the sign bit and
touches nothing else — there is no rounding, no overflow, and no underflow. Negation is one
of the few operations in this Handbook about which nothing numerical can go wrong. The one
consequence worth remembering is that `-0` is a real, distinct binary64 value, and negating
zero produces it.

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | The operand to negate. Required. |

Arity is exactly 1 (`Arity { min: 1, max: 1 }`); no optional arguments, no defaults.

The same `-` character in infix position is [`FUNC.OP_SUBTRACT`](FUNC.OP_SUBTRACT.md), a
two-argument function. Which one a given `-` denotes is decided by the formula grammar, and
formula grammar is out of this Handbook's scope (`CHARTER.md` section 4). The one place this
bites readers is the double-unary idiom `--A`, which is two applications of this operator and
is used precisely for its coercing effect: it turns logicals into 1/0 inside array
expressions.

## Result and edge cases

Returns a `Number`.

- **Text operands.** Numeric-looking text coerces and is negated; other text is a to-number
  failure. Contrast with unary plus, where the recorded Excel observation is that text
  survives as text.
- **Logical operands.** `TRUE` becomes `-1`, `FALSE` becomes `0` (or `-0`) under the shared
  rule.
- **Empty and omitted.** Per-family policy under chapter 02; not recorded here for this
  operator.
- **Arrays.** The declared coercion/lift profile is
  `CoercionLiftProfile::UnaryNumericScalarOrArrayElementwise` — one numeric argument, mapped
  elementwise over arrays. A mixed array yields an array with element-local failures rather
  than a collapsed error, per chapter 02.
- **Zero.** Negating a zero yields a zero of the opposite sign. Whether that sign is
  observable in a published Excel result is unrecorded here; the usual detector is dividing
  into the result, but on the worksheet division by zero is an error either way, so a
  detector has to be constructed carefully (`SIGN` will not see it, and neither will cell
  display).

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | The operand is text that does not read as a number. |
| any incoming error | Propagates unchanged; `ErrorCollapseProfile::None`. |

`data/functions/FUNC.OP_NEGATE.json` records no Microsoft documentation URL (`docs` is
`null`), so these conditions rest on the shared call-model chapters and OxFunc's provisional
arithmetic-family contract.

## Relationships

- [`FUNC.OP_UNARY_PLUS`](FUNC.OP_UNARY_PLUS.md) — the deliberate contrast: same shape,
  opposite coercion doctrine.
- [`FUNC.OP_SUBTRACT`](FUNC.OP_SUBTRACT.md) — same character, two operands.
- `ABS`, `SIGN` — the other sign-manipulating primitives. `-ABS(x)` is the idiomatic "force
  negative"; `SIGN` does not distinguish `0` from `-0`.
- `N`, `VALUE` — the explicit coercion functions. Where `--A` is used only for its coercion,
  `N(A)` or `VALUE(A)` states the intent, though with different behaviour on text.
- `IMSUB` — no relation, but readers looking for complex negation end up there.

## Notes for implementers

- Negate by flipping the sign bit, not by computing `0 - A`. The two differ on zero:
  `0 - 0` is `+0` under round-to-nearest, while `-(0)` is `-0`.
- The coercion must run *before* the negation and must be the shared to-number primitive, not
  a local reimplementation — the whole point of the family module is that `-`, `%` and the
  binary operators cannot drift apart on what counts as numeric text.
- The elementwise array path must preserve element-local failures; do not short-circuit the
  whole array on the first bad element.
- Do not implement `+A` by delegating to this operator with the sign flip omitted. That is
  precisely the shape `BUG-FUNC-029` records as a defect.

## What has not been checked

No Handbook vector suite covers `-A`, and no Excel-comparison evidence record is attached to
this page. Nothing here is a measurement.

Probes worth running first:

1. **Signed zero.** `=-0`, `=-(0)`, `=-A1` with `A1` holding zero, and whether any published
   expression can distinguish the result from `+0`.
2. **Blank operand.** `=-A1` with `A1` empty, to pin the Empty policy.
3. **Text and logical operands.** `=-"2"`, `=-TRUE`, `=-"2%"`, `=-" 2 "`, to confirm the
   coercing behaviour and to map the recogniser edges — and to establish the contrast with
   unary plus on identical inputs, which is the experiment that gives both pages their
   value.
4. **Arrays.** `=-{1,"a",TRUE}`, to confirm element-local failure.
5. **Double unary.** `=--A1` across every operand kind, since the idiom is widespread enough
   that a divergence there would be visible in real workbooks.

## Page vocabulary

| Machine name | Meaning |
|---|---|
| `Arity { min: 1, max: 1 }` | Exactly one operand |
| `ArgPreparationProfile::ValuesOnlyPreAdapter` | References resolved to values before the operator runs |
| `CoercionLiftProfile::UnaryNumericScalarOrArrayElementwise` | One numeric argument, mapped elementwise over arrays |
| `KernelSignatureClass::NumToNum` | Kernel maps one number to one number |
| `ErrorCollapseProfile::None` | Error operands propagate unchanged |
| `default-unexamined` | Axis provenance: a projection default, not an examined fact |

## Sources

- `data/functions/FUNC.OP_NEGATE.json` at OxFunc `473efa3` — identity, arity, signature `-A`,
  classification, axis provenance. `docs` is `null`: **no Microsoft documentation URL is
  recorded for this entry.** Microsoft's account of `-` lives in the support article
  *Calculation operators and precedence in Excel*, not yet linked from the data projection.
- `data/presence/FUNC.OP_NEGATE.json` — implementing module
  `crates/oxfunc_core/src/functions/operator_arithmetic_family.rs`.
- Handbook `content/model/01-value-universe.md`, `02-coercion-and-lifting.md`,
  `03-call-pipeline.md`.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OPERATOR_ARITHMETIC_FAMILY_CONTRACT_PRELIM.md` —
  values-only numeric coercion and elementwise array lift for this row; provisional.
- OxFunc `docs/bugs/streams/BUG-FUNC-029_unary_plus_over_coerces_text_and_logical.md` — the
  recorded statement that unary minus coerces where unary plus does not. Cited as an upstream
  record.
