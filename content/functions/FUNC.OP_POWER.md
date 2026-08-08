---
schema: efh.function-page/v1
function_id: FUNC.OP_POWER
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
role_in_family: "Exponentiation: the family's only member with a genuine domain restriction, and the one whose publication path is documented to differ from a plain library call."
---

## What it computes

`A ^ B` converts both operands to numbers and returns `A` raised to the power `B`, as a real
value.

The word "real" is the entire specification. Unlike IEEE-754's `pow`, which is defined over
extended reals and returns infinities and NaN, the worksheet's `^` is a *real-valued* power
with three consequences:

1. **Negative base with a non-integer exponent has no real value.** `(-8)^(1/3)` is `-2` over
   the reals but the principal complex cube root is not real, and Excel's power path does not
   attempt a real branch selection. OxFunc's provisional arithmetic-family contract records
   this lane as "real-domain NaN cases as `#NUM!`".
2. **Zero to a negative power diverges.** OxFunc's defect stream `BUG-FUNC-005` records a
   live Excel replay observing `=0^-1` as `#DIV/0!`.
3. **Zero to the zero is refused.** The same record observes `=0^0` and `=POWER(0,0)` as
   `#NUM!` on the replayed baseline (Excel 16.0 build 19929, replay dated 2026-04-29 in that
   record). This is the single most surprising fact on this page: mathematics conventionally
   defines `0^0 = 1`, C's `pow(0,0)` returns `1`, and Excel — on that observation — does not.
   The Handbook reports the record; it has not re-measured it.

The second specification-level fact is *how* the power is computed. Chapter 03 describes a
declared publication axis, `PrecisionRoundingProfile::IntegerExponentPublication`: when the
exponent is an exact integer, Excel computes the power by repeated multiplication (binary
exponentiation) rather than by the transcendental `exp(b·ln a)` path, and the two disagree in
the last bits. Chapter 03 names `POWER` and the `^` operator as the carriers of that axis and
gives a worked `POWER(1.05, 10)` example.

**There is a live inconsistency here, and this page states it rather than smoothing it.**
`data/functions/FUNC.POWER.json` records `precision_rounding_profile:
integer_exponent_publication`, while `data/functions/FUNC.OP_POWER.json` records
`precision_rounding_profile: default` — and the entry marks that axis with provenance
`default-unexamined`. So the chapter says `^` carries the integer-exponent publication rule,
and the operator's own projected axis says it does not. One of the two is wrong. Which one is
an open question that only a measurement can settle.

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | Base. Required. |
| 1 | `B` | Exponent. Required. |

Arity is exactly 2; no optional arguments, no defaults. Order is emphatically load-bearing.

The exponent's *exactness as an integer* is a semantic input, not a formatting detail: on the
integer-exponent publication rule, `x^3` and `x^3.0000000000000004` may take different
computation paths, and a spreadsheet that computes its exponent rather than typing it can
land on either side of that boundary without the author noticing.

## Result and edge cases

Returns a `Number`, `#NUM!` or `#DIV/0!` (`KernelSignatureClass::NumsToNum`).

- **Negative base, integer exponent.** Real and well defined; the sign alternates with the
  parity of the exponent.
- **Negative base, non-integer exponent.** Real-domain failure; recorded as `#NUM!` in
  OxFunc's contract.
- **Zero base.** Positive exponent gives zero; negative exponent is recorded as `#DIV/0!`;
  zero exponent is recorded as `#NUM!` (see above).
- **Base 1 and exponent 0** are the usual identities, with the `0^0` exception above the only
  interesting corner.
- **Huge results.** `1E300^2` overflows the finite range. The recorded real-result policy for
  this entry is `non_finite=allow` with provenance `default-unexamined`, which cannot be
  right as stated — the kernel demonstrably *can* produce non-finite values — so the overflow
  publication lane is unresolved on this page.
- **Text and logical operands.** Shared to-number rules from
  [coercion and lifting](../model/02-coercion-and-lifting.md).
- **Arrays.** `LiftBroadcastProfile::SurfaceNative`, with the family's admitted broadcast
  shapes and `#N/A` for coordinates neither operand supplies.

## Errors

| Error | Condition |
|---|---|
| `#NUM!` | The real power is undefined for the operand pair — negative base with non-integer exponent; and, on the replayed observation, `0^0`. |
| `#DIV/0!` | Zero base with a negative exponent. |
| `#VALUE!` | An operand is text that does not read as a number. |
| any incoming error | Propagates unchanged; `ErrorCollapseProfile::None`. |

`data/functions/FUNC.OP_POWER.json` records no Microsoft documentation URL (`docs` is
`null`). The `#NUM!` and `#DIV/0!` conditions above come from OxFunc's provisional
arithmetic-family contract and the `BUG-FUNC-005` record, not from a cited Microsoft page for
`^`.

## Relationships

- `POWER` — the named function form. The two are meant to be the same semantics; the
  projected precision axis currently disagrees between them, which is precisely the open
  problem above. Anyone comparing `x^y` against `POWER(x,y)` in a real workbook is running
  the most valuable available experiment on this row.
- `EXP`, `LN`, `LOG`, `LOG10` — the transcendental path `^` is documented *not* to take for
  integer exponents.
- `SQRT` — `x^0.5` and `SQRT(x)` are different implementations of the same mathematical
  function. `SQRT` is a correctly-rounded IEEE primitive; a general power path usually is
  not. Prefer `SQRT` where you can.
- `IMPOWER` — complex powers over text-encoded complex numbers, which *does* return a
  negative real base's fractional power as a complex value.
- [`FUNC.OP_MULTIPLY`](FUNC.OP_MULTIPLY.md) — what the integer-exponent path reduces to.

## Notes for implementers

- Implement two paths deliberately: an exact-integer-exponent path by binary exponentiation
  and a general path, and make the integer test exact (the exponent must be an integer *as a
  double*, not merely close to one). This is the difference chapter 03 records as
  observable.
- Binary exponentiation is not associativity-free either. `x^10` computed as
  `((x^2)^2 · x)^2` and as `x·x·…·x` differ in the last bits; a compatibility implementation
  must pin the exact multiplication schedule, not merely "repeated multiplication".
- Route the domain checks before the kernel, in this order: error operands, coercion
  failures, zero-base cases, negative-base-non-integer-exponent, then the arithmetic.
  Reordering these changes which error a doubly-bad operand pair produces.
- Never let a library `pow` return an infinity or NaN to the publication layer. The worksheet
  value universe has no such values (chapter 01); every non-finite must be mapped to an error
  or a documented saturation before it reaches a cell.

## What has not been checked

No Handbook vector suite covers `^`, and no Excel-comparison evidence record is attached to
this page. The `0^0`, `0^-1` and real-domain facts above are reported *from OxFunc's defect
record*, not measured by the Handbook; the record itself is an OxFunc artifact and this page
does not restate its counts or its verification wording.

Probes, in priority order:

1. **The integer-exponent axis disagreement.** Sweep `x^n` against `POWER(x,n)` for exact
   integer `n` over bases where the binary-exponentiation and `exp·ln` paths differ, reading
   raw result bits. This settles whether `FUNC.OP_POWER`'s projected
   `precision_rounding_profile: default` is a data defect or a genuine divergence from
   `POWER`.
2. **The integer boundary.** `x^3` versus `x^(3+2^-50)`, to find where the integer test
   actually sits.
3. **`0^0` re-measurement on a current build.** The `#NUM!` observation is pinned to one
   named build; whether it holds on the current channel is unknown here.
4. **Negative bases.** `(-8)^(1/3)`, `(-2)^2`, `(-2)^2.0`, `(-2)^-3`, to map the real-domain
   boundary precisely.
5. **Overflow.** `1E300^2` and neighbours, to resolve the `non_finite=allow` question.

## Page vocabulary

| Machine name | Meaning |
|---|---|
| `Arity { min: 2, max: 2 }` | Exactly two operands |
| `ArgPreparationProfile::ValuesOnlyPreAdapter` | References resolved to values before the operator runs |
| `CoercionLiftProfile::Custom` | Operator-specific coercion |
| `KernelSignatureClass::NumsToNum` | Kernel maps several numbers to one number |
| `LiftBroadcastProfile::SurfaceNative` | The operator does its own array lifting |
| `PrecisionRoundingProfile::IntegerExponentPublication` | Exact-integer exponents computed by repeated multiplication (carried by `POWER`; disputed for `^`) |
| `PrecisionRoundingProfile::Default` | Publishes the plain IEEE-754 kernel result |
| `default-unexamined` | Axis provenance: a projection default, not an examined fact |

## Sources

- `data/functions/FUNC.OP_POWER.json` at OxFunc `473efa3` — identity, arity, signature
  `A ^ B`, classification, and the `precision_rounding_profile: default` value with
  `default-unexamined` provenance. `docs` is `null`: **no Microsoft documentation URL is
  recorded for this entry.**
- `data/functions/FUNC.POWER.json` — the named function's
  `precision_rounding_profile: integer_exponent_publication`, the other half of the
  disagreement, and the Microsoft URL for the *function* form:
  <https://support.microsoft.com/en-us/office/power-function-d3f2908b-56f4-4c3f-895a-07fb519c362a>.
- `data/presence/FUNC.OP_POWER.json` — implementing module
  `crates/oxfunc_core/src/functions/operator_arithmetic_family.rs`.
- Handbook `content/model/03-call-pipeline.md` — the `IntegerExponentPublication` axis, its
  `POWER(1.05, 10)` worked example, and the statement that `^` carries it.
- Handbook `content/model/01-value-universe.md`, `02-coercion-and-lifting.md`.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OPERATOR_ARITHMETIC_FAMILY_CONTRACT_PRELIM.md` —
  the shared `POWER` kernel domain lanes for `^`; provisional by its own header.
- OxFunc `docs/bugs/streams/BUG-FUNC-005_power_zero_to_zero_diverges_from_excel.md` — the
  `0^0`, `0^-1`, `0^1`, `1^0` replay observations and their named Excel build. Cited as an
  upstream record.
