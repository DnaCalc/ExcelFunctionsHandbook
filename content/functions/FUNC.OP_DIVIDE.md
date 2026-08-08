---
schema: efh.function-page/v1
function_id: FUNC.OP_DIVIDE
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
role_in_family: "Binary division: the only arithmetic operator with a domain error of its own, and the reason `#DIV/0!` exists as a worksheet error code."
---

## What it computes

`A / B` converts each operand to a number and returns the IEEE-754 binary64 quotient, rounded
once to nearest with ties to even — except that a zero divisor does not produce an IEEE
infinity but a worksheet error.

That exception is the whole character of this operator. IEEE-754 defines `x/0` as `±∞` for
nonzero `x` and NaN for `0/0`; the worksheet defines both as `#DIV/0!`. OxFunc's provisional
arithmetic-family contract records "divide-by-zero returns `#DIV/0!`" as the admitted lane,
and chapter 01 states that a cell never publishes an infinity or a NaN. `/` is therefore one
of the clearest places where the worksheet value universe deliberately departs from the
underlying floating-point model.

Everything else is the shared story: `/` never sees a cell, and each operand is converted by
the to-number rules of [coercion and lifting](../model/02-coercion-and-lifting.md).

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | Dividend (numerator). Required. |
| 1 | `B` | Divisor (denominator). Required. |

Arity is exactly 2; no optional arguments, no defaults. Order is load-bearing. In the array
case, the divisor position is where the interesting failures live: a single zero anywhere in
a broadcast divisor array produces `#DIV/0!` in exactly the coordinates it reaches, not in
the whole result — per-cell failures stay per-cell under the lifting rule of chapter 02.

## Result and edge cases

Returns a `Number`, or `#DIV/0!` (`KernelSignatureClass::NumsToNum`).

- **Zero divisor.** `#DIV/0!`, including the `0/0` case. Chapter 01 also lists division by an
  *empty cell* as a documented cause of `#DIV/0!`, which is a useful hint about the family's
  Empty policy — but the Handbook has not recorded that behaviour for `/` against Excel and
  does not assert it here.
- **Text and logical operands.** Shared to-number rules.
- **Exactness.** Division is exact only when the quotient is representable — mainly powers of
  two and small integer ratios. `1/3`, `1/10` and most currency ratios are not exact, which
  is the root of the classic "my totals are off by a cent" report. The operator is not at
  fault; binary64 has no exact tenth.
- **Arrays.** `LiftBroadcastProfile::SurfaceNative`, with the family's admitted broadcast
  shapes: scalar/array, array/scalar, same-shape, and row-vs-column outer product, with
  unsupplied coordinates returning `#N/A`.
- **Overflow and underflow.** A quotient can overflow (`1E300/1E-300`) or underflow to zero.
  The entry's real-result policy is recorded as `non_finite=allow` with provenance
  `default-unexamined`, so the overflow lane is a projection default rather than an examined
  fact — and it sits uneasily beside the `#DIV/0!` rule, which shows the operator does police
  at least one non-finite lane.

## Errors

| Error | Condition |
|---|---|
| `#DIV/0!` | The divisor converts to zero, including `0/0`. |
| `#VALUE!` | An operand is text that does not read as a number. |
| any incoming error | Propagates unchanged; `ErrorCollapseProfile::None`. |

`data/functions/FUNC.OP_DIVIDE.json` records no Microsoft documentation URL (`docs` is
`null`). The `#DIV/0!` condition above is taken from OxFunc's provisional arithmetic-family
contract and from the error-code table in chapter 01, not from a cited Microsoft page for
`/`.

## Relationships

- [`FUNC.OP_MULTIPLY`](FUNC.OP_MULTIPLY.md) — the inverse operation. `a/b` and `a*(1/b)` are
  *not* the same double in general: the second rounds twice.
- `QUOTIENT` — integer division, returning the truncated integer part. Different function,
  different error surface.
- `MOD` — the remainder companion. `MOD` has its own sign convention (it follows the divisor)
  which does not match the truncation implied by `QUOTIENT`; do not assume
  `a = b*QUOTIENT(a,b) + MOD(a,b)` without checking.
- `IFERROR`, `IFNA` — the usual wrappers readers reach for when a divisor may be zero.
  `IFERROR` masks every error, not only `#DIV/0!`, which is a common source of hidden bugs.
- `IMDIV` — complex division over text-encoded complex numbers.

## Notes for implementers

- Check the divisor *after* coercion, not before. `"0"` as text coerces to zero and must
  reach the same `#DIV/0!` lane as numeric zero; a pre-coercion check on the raw value misses
  it.
- Do not strength-reduce `a/b` into `a * (1/b)`. The reciprocal introduces a second rounding
  and changes results in the last bit. Compilers do this under fast-math flags; a
  compatibility implementation must forbid them.
- A negative zero divisor is still a zero divisor. `x / -0` must take the `#DIV/0!` lane, not
  produce a negative infinity.
- In the array case, produce the error per coordinate. Collapsing the whole result to
  `#DIV/0!` because one cell divided by zero contradicts the per-cell failure rule of chapter
  02.

## What has not been checked

No Handbook vector suite covers `/`, and no Excel-comparison evidence record is attached to
this page. Nothing here is a measurement.

Probes worth running first:

1. **Every spelling of a zero divisor.** Numeric `0`, text `"0"`, `FALSE`, a blank cell, and
   negative zero, to confirm they all reach `#DIV/0!` and that the blank-cell case behaves as
   chapter 01's error table implies.
2. **Overflow.** `1E300/1E-300` and neighbours, to decide whether the published result is an
   error or something finite.
3. **Underflow.** Quotients landing in the subnormal range and just below it.
4. **Reciprocal drift.** A sweep of `a/b` against `a*(1/b)` chosen so the two differ, to
   confirm which Excel produces.
5. **Per-cell error in broadcast.** `={1,2;3,4}/{1,0;0,2}`, to confirm errors stay local to
   their coordinates.

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

- `data/functions/FUNC.OP_DIVIDE.json` at OxFunc `473efa3` — identity, arity, signature
  `A / B`, classification, axis provenance. `docs` is `null`: **no Microsoft documentation
  URL is recorded for this entry.** Microsoft's account of `/` lives in the support article
  *Calculation operators and precedence in Excel*, not yet linked from the data projection.
- `data/presence/FUNC.OP_DIVIDE.json` — implementing module
  `crates/oxfunc_core/src/functions/operator_arithmetic_family.rs`.
- Handbook `content/model/01-value-universe.md` (the fourteen error codes, including
  `#DIV/0!` and its "or by an empty cell" gloss), `02-coercion-and-lifting.md`,
  `03-call-pipeline.md`.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OPERATOR_ARITHMETIC_FAMILY_CONTRACT_PRELIM.md` —
  the admitted `#DIV/0!` lane and broadcast shapes; provisional by its own header.
