---
schema: efh.function-page/v1
function_id: FUNC.OP_MULTIPLY
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
family: operator_arithmetic_family
role_in_family: "Binary multiplication: the family member most often used as a logical AND over coerced booleans, which is why its coercion rules get exercised harder than its arithmetic."
---

## What it computes

`A * B` converts each operand to a number and returns the IEEE-754 binary64 product, rounded
once to nearest with ties to even.

Three properties that matter in practice:

1. **One rounding, not two.** The exact product of two doubles generally needs about twice
   the significand; the operation rounds it once. That single rounding is what makes
   `a*b` reproducible across conforming platforms, and what makes `(a*b)*c ≠ a*(b*c)` in
   general — multiplication is commutative but not associative in binary64.
2. **Underflow is gradual, then total.** A product whose magnitude falls below the smallest
   normal double becomes subnormal and loses significand bits; below the smallest subnormal
   it becomes zero. The zero is signed, and the sign is the XOR of the operand signs.
3. **The coercion is doing most of the work in real workbooks.** `=(A1>5)*(B1<3)` multiplies
   two logicals — the classic array-formula spelling of AND — and depends entirely on the
   shared to-logical-to-number rule (TRUE→1, FALSE→0) of
   [coercion and lifting](../model/02-coercion-and-lifting.md), not on anything specific to
   `*`.

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | Left factor. Required. |
| 1 | `B` | Right factor. Required. |

Arity is exactly 2; no optional arguments, no defaults. Multiplication is commutative on
finite numbers, so operand order does not change the value — but it does change the
broadcast orientation when the operands are arrays of different shapes, so the positions are
not interchangeable in the array case.

## Result and edge cases

Returns a `Number` (`KernelSignatureClass::NumsToNum`).

- **Text and logical operands.** Shared to-number rules; `*` adds nothing of its own.
- **Zero times anything finite** is zero, with the sign given by the operand signs. The
  Handbook has not recorded whether the sign of that zero is observable in a published Excel
  result.
- **Empty and omitted operands.** Per-family policy under chapter 02; unrecorded here.
- **Arrays.** `LiftBroadcastProfile::SurfaceNative`. OxFunc's provisional arithmetic-family
  contract records scalar/array, array/scalar, same-shape and row-vs-column outer-product
  grids as the admitted shapes, with unsupplied coordinates returning `#N/A`. The
  row-vs-column case is the useful one: a row operand times a column operand is an outer
  product, which is how multiplication tables and weighted grids get built without a helper
  function.
- **Overflow and underflow.** The recorded real-result policy is `non_finite=allow`, and the
  entry marks that axis `default-unexamined`. Overflow of a product is easy to reach
  (`1E300 * 1E300`), so this is not a hypothetical lane; it is simply unrecorded.

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | An operand is text that does not read as a number. |
| any incoming error | Propagates unchanged; `ErrorCollapseProfile::None`. |

`data/functions/FUNC.OP_MULTIPLY.json` records no Microsoft documentation URL (`docs` is
`null`), so these conditions rest on the shared call-model chapters and OxFunc's provisional
contract, not on a cited Microsoft page for `*`.

## Relationships

- [`FUNC.OP_DIVIDE`](FUNC.OP_DIVIDE.md) — the inverse operation, and the one that adds an
  error lane (`#DIV/0!`) `*` does not have.
- [`FUNC.OP_POWER`](FUNC.OP_POWER.md) — repeated multiplication when the exponent is an exact
  integer; see that page for the publication subtlety.
- `PRODUCT` — the aggregate. As with `SUM` versus `+`, `PRODUCT` scans ranges under an
  aggregate policy and is not `*` folded over a list.
- `SUMPRODUCT`, `MMULT` — elementwise-then-sum and matrix product. `MMULT` is emphatically
  not `*`: `*` on two arrays broadcasts elementwise, `MMULT` contracts an index.
- `IMPRODUCT` — complex multiplication over text-encoded complex numbers.

## Notes for implementers

- The elementwise-versus-matrix distinction is the single most common user-facing confusion
  on this row. An implementation that "helpfully" contracts conformable arrays is wrong;
  `*` broadcasts.
- Do not fuse. A fused multiply-add (`a*b+c` computed with one rounding) gives a different,
  usually more accurate, result than Excel's two-rounding sequence. For the
  `excel-bitexact` flavour, fusion must be disabled explicitly — many compilers will
  introduce it at higher optimization levels without asking.
- Guard the array orientation before the kernel. Broadcast shape selection belongs in the
  shared lift layer so that `*`, `+`, `-`, `/` and `^` cannot drift apart.

## What has not been checked

No Handbook vector suite covers `*`, and no Excel-comparison evidence record is attached to
this page. Nothing here is a measurement.

Probes worth running first:

1. **Overflow and underflow.** `1E300*1E300`, `1E-300*1E-300`, and products landing exactly
   on the subnormal boundary, to decide what Excel publishes where IEEE would give infinity
   or a subnormal.
2. **Fused-multiply-add detection.** Expressions of the form `a*b+c` chosen so that the fused
   and unfused results differ in the last bit, to establish which one Excel produces.
3. **Signed zero.** `-1*0` and `0*-1` fed into a sign-detecting expression.
4. **Outer product padding.** A row operand against a column operand, and a
   non-conformable pair, to confirm the grid shape and the `#N/A` padding rule.
5. **Logical coercion in array context.** `={TRUE,FALSE}*{1,2}`, to confirm that the
   logical-to-number rule applies elementwise under lifting rather than at the whole-array
   level.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `OP_MULTIPLY` that an implementer matching Excel, or a reader explaining
an unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's
answer and where it was recorded; "measured" means the statement is read off recorded Excel
answers, "inferred" marks what goes beyond them. Where older text on this page says a behaviour
was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

### FN-OP_DIVIDE-001: Multiplication and division double-round through x87 64-bit significand precision and flush subnormal results to +0

*Detail for matching Excel's bits; measured; recorded 2026-11-22.* Applies to OP_DIVIDE, OP_MULTIPLY.

In Excel 16.0.20527.20088, * and / coerce operands left-to-right to numbers, evaluate at x87
64-bit significand precision (PC64) before rounding to binary64 (differing by 1 ULP from
binary64 single rounding on dc6qki_arith rows 1-24, e.g. 0.0989043*6004.60993 =
0x40828f0dceb275c2 vs nearest ...75c3), and flush any subnormal product or quotient to +0
(2.3E-308/2 = 0 and 1E-307*0.01 = 0, while normal 5E-308/2 stays 2.5E-308).

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `=0.0989043*6004.60993` | as typed | `593.8817418996989` (`0x40828f0dceb275c2`) | `593.881741899699` (`0x40828f0dceb275c3`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=0.002577/0.752039485` | as typed | `0.0034266817785504975` (`0x3f6c1245c5868a94`) | `0.003426681778550497` (`0x3f6c1245c5868a93`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=2.3E-308/2` | as typed | `0.0` (`0x0000000000000000`) | — | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=5E-308/2` | as typed | `2.5e-308` (`0x0011fa182c40c60d`) | — | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Correct value: Exact rational product of both binary64 operands and decimal literals, rounded
to nearest binary64
Correct value: Exact rational quotient of both binary64 operands and decimal literals, rounded
to nearest binary64

Basis (measured): Measured across Excel 16.0.20527.20088 in dc6qki_arith (rows 1-24) and
jnt_operators (rows 14-15, 17, 22-24).

Evidence:

- DnaCalc `crates/calc-func/src/catalogue/arithmetic.rs`, lines 52-75 at `2e9be2b5b7` — * and / x87 double-rounding with subnormal flush to +0
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dc6qki_arith.capture.json`, rows 1 to 24 at `2e9be2b5b7` — 24 double-rounding * and / readings in Excel 16.0.20527
- DnaCalc `crates/calc-func/tests/captures/jnt/sections/jnt_operators.capture.json`, rows 22 to 24 at `2e9be2b5b7` — Subnormal vs normal flush boundary on * and /
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dc6qki_arith.capture.json`, sheet dc6qki_arith row 2 column 4 at `2e9be2b5b7` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dc6qki_arith.capture.json`, sheet dc6qki_arith row 14 column 4 at `2e9be2b5b7` (readings)
- DnaCalc `crates/calc-func/tests/captures/jnt/sections/jnt_operators.capture.json`, sheet jnt_operators row 23 column 4 at `2e9be2b5b7` (readings)
- DnaCalc `crates/calc-func/tests/captures/jnt/sections/jnt_operators.capture.json`, sheet jnt_operators row 24 column 4 at `2e9be2b5b7` (readings)

<!-- efh:notes end -->

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

- `data/functions/FUNC.OP_MULTIPLY.json` at OxFunc `473efa3` — identity, arity, signature
  `A * B`, classification, axis provenance. `docs` is `null`: **no Microsoft documentation
  URL is recorded for this entry.** Microsoft's account of `*` lives in the support article
  *Calculation operators and precedence in Excel*, not yet linked from the data projection.
- `data/presence/FUNC.OP_MULTIPLY.json` — implementing module
  `crates/oxfunc_core/src/functions/operator_arithmetic_family.rs`.
- Handbook `content/model/01-value-universe.md`, `02-coercion-and-lifting.md`,
  `03-call-pipeline.md`.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OPERATOR_ARITHMETIC_FAMILY_CONTRACT_PRELIM.md` —
  admitted operand and broadcast lanes; provisional by its own header.
