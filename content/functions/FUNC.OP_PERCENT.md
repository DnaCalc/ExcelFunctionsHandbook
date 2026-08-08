---
schema: efh.function-page/v1
function_id: FUNC.OP_PERCENT
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
role_in_family: "The postfix operator: the family's only suffix form, and the one whose one-line definition hides a last-bit ambiguity."
---

## What it computes

`A%` converts its operand to a number and returns that number scaled by one hundredth. `50%`
is `0.5`; `A1%` is a hundredth of whatever `A1` holds.

The interesting content is in the word "scaled", because binary64 gives two inequivalent
spellings of it:

- `x / 100` — divide by an exactly representable integer, one rounding.
- `x * 0.01` — multiply by the double nearest to one hundredth, which is *not* one hundredth,
  and then round.

These do not agree for every `x`. The literal `0.01` is already a rounded approximation, so
the multiplication rounds twice in effect; the division rounds once against an exact divisor
and is the more accurate of the two. For most operands both spellings land on the same
double, and for some they do not — which makes this a real, if small, last-bit question about
a function whose definition looks trivial.

OxFunc's provisional arithmetic-family contract describes this row as "postfix numeric
scaling by `1/100`", which does not disambiguate the two spellings. The Handbook has not
recorded which one Excel uses.

The percent *format* is a separate thing entirely. Formatting a cell as a percentage changes
how a number is displayed and how typed input is interpreted by the host; `%` as an operator
changes the value. A cell holding `0.5` formatted as a percentage shows `50%` without this
operator ever running. Chapter 01's rule applies: dates, currency and percentages are numbers
wearing formats.

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | The operand to scale. Required. |

Arity is exactly 1; no optional arguments, no defaults. The operator is postfix — it follows
its operand — which makes it the only member of the arithmetic family with that shape.

Postfix position raises a precedence question (`-2%`, `2^3%`, `2%%`) that this Handbook does
not answer: formula grammar and precedence are explicitly out of scope (`CHARTER.md` section
4), and the operator-inventory decision register in the sources records precedence ownership
as an open item. What the Handbook *can* say is that `%` is a one-argument function of a
value, and that chaining it (`A%%`) applies it twice.

## Result and edge cases

Returns a `Number` (`KernelSignatureClass::NumToNum`).

- **Text and logical operands.** Shared to-number rules from
  [coercion and lifting](../model/02-coercion-and-lifting.md). Note the overlap: chapter 02
  records that Excel's text-to-number recognizer itself accepts percent signs in some
  locale-dependent forms, so `"50%"` as *text* may already convert to `0.5` before this
  operator is reached — and `"50%"%` would then scale it again. That interaction is
  unrecorded here and is a good probe.
- **Empty and omitted.** Per-family policy under chapter 02; unrecorded for this row.
- **Arrays.** `CoercionLiftProfile::UnaryNumericScalarOrArrayElementwise` — mapped
  elementwise, with element-local failures.
- **Underflow.** Scaling a subnormal by a hundredth flushes it toward zero; scaling any tiny
  value repeatedly reaches zero. Nothing about this is specific to `%`, but `%` is a cheap way
  to reach the subnormal range from ordinary-looking data.
- **Overflow.** Not reachable: scaling down cannot overflow.

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | The operand is text that does not read as a number. |
| any incoming error | Propagates unchanged; `ErrorCollapseProfile::None`. |

`data/functions/FUNC.OP_PERCENT.json` records no Microsoft documentation URL (`docs` is
`null`), so these conditions rest on the shared call-model chapters and OxFunc's provisional
contract.

## Relationships

- [`FUNC.OP_DIVIDE`](FUNC.OP_DIVIDE.md) and [`FUNC.OP_MULTIPLY`](FUNC.OP_MULTIPLY.md) — the
  two operators `%` might be implemented in terms of, and the reason the ambiguity above
  matters.
- [`FUNC.OP_NEGATE`](FUNC.OP_NEGATE.md), [`FUNC.OP_UNARY_PLUS`](FUNC.OP_UNARY_PLUS.md) — the
  other unary members of the family.
- `PERCENTILE`, `PERCENTRANK`, `PERCENTOF` — statistical functions whose names share the word
  and share nothing else.
- `TEXT(x, "0%")` — the formatting route. If the goal is display, `%`-the-operator is the
  wrong tool; if the goal is arithmetic, the format is.

## Notes for implementers

- Pick a spelling and record which one: `x / 100` or `x * 0.01`. They are not the same
  function of a double, and a compatibility implementation that guesses will disagree with
  Excel on some operands. The Handbook cannot yet tell you which to pick.
- Do not implement `%` by string manipulation of the formula or by adjusting the cell format.
  It is a value transformation.
- Chained application (`A%%`) must scale twice; there is no idempotence to exploit.
- The elementwise array path must preserve element-local failures.

## What has not been checked

No Handbook vector suite covers `%`, and no Excel-comparison evidence record is attached to
this page. Nothing here is a measurement.

Probes worth running first:

1. **Which spelling.** Sweep operands `x` where `x/100` and `x*0.01` produce different
   doubles, comparing raw result bits from `=x%`. This is a small, decisive experiment and it
   is the reason this page exists.
2. **Text with a percent sign.** `="50%"%`, `=VALUE("50%")%`, and `"50%"` in a referenced
   cell, to map the interaction between the recognizer's percent handling and the operator.
3. **Chaining.** `=5%%`, `=5%%%`, to confirm repeated scaling.
4. **Blank operand.** `=A1%` with `A1` empty.
5. **Subnormal range.** Operands near the subnormal boundary, to see whether the published
   result flushes to zero or retains a subnormal.

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

- `data/functions/FUNC.OP_PERCENT.json` at OxFunc `473efa3` — identity, arity, signature
  `A%`, classification, axis provenance. `docs` is `null`: **no Microsoft documentation URL
  is recorded for this entry.** Microsoft's account of `%` lives in the support article
  *Calculation operators and precedence in Excel*, not yet linked from the data projection.
- `data/presence/FUNC.OP_PERCENT.json` — implementing module
  `crates/oxfunc_core/src/functions/operator_arithmetic_family.rs`.
- Handbook `content/model/01-value-universe.md` (numbers wearing formats),
  `02-coercion-and-lifting.md` (the locale-dependent recognizer note),
  `03-call-pipeline.md`.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OPERATOR_ARITHMETIC_FAMILY_CONTRACT_PRELIM.md` —
  "postfix numeric scaling by `1/100`" and the elementwise lift; provisional by its own
  header.
- OxFunc `docs/function-lane/W45_NON_AT_OPERATOR_INVENTORY.csv` — the `arithmetic_postfix`
  family classification for this row.
