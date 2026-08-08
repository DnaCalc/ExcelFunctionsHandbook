---
schema: efh.function-page/v1
function_id: FUNC.XOR
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
family: xor_fn
role_in_family: "The parity reducer: returns TRUE when an odd number of scanned truth values are true."
---

# XOR

## What it computes

`XOR` shares the truth scan of `AND` and `OR` but performs a different reduction: it returns the
**parity** of the true items, not a threshold test.

Formally, let `T` be the number of items in the scan that classified as true. Then `XOR` returns
`TRUE` when `T` is odd and `FALSE` when `T` is even — including `T = 0`, provided at least one
item classified at all.

This is worth stating carefully because the name misleads. In two-argument use, parity and
"exactly one is true" coincide, so `XOR(a, b)` reads like exclusive disjunction. With three or
more arguments they diverge: `XOR(TRUE, TRUE, TRUE)` is `TRUE` (three is odd), not `FALSE`. If you
want "exactly one of these is true" over three conditions, `XOR` is the wrong function — the
counting idiom `SUM(--(conditions)) = 1` is what you mean.

The scan and its classification are identical to `AND`'s:

| Item | Classification |
|---|---|
| `Logical` | its own truth value |
| `Number` | false if exactly 0, true otherwise |
| Text or empty inside an array or reference argument | skipped |
| `Error` | the scan stops and that error is the result |

And, as with `AND` and `OR`, a scan in which **nothing** classified returns `#VALUE!` rather than
the algebraic identity `FALSE`.

## Arguments

`XOR(logical1, [logical2], ...)`

- `logical1` is required; the registry records an accepted arity of 1 to 255 arguments.
- Each argument is a condition or a container of conditions, scanned in row-major order.
- An omitted slot delivers `Missing` and contributes nothing.

The misunderstood position here is not any one slot but the **count**: readers reach for `XOR`
with more than two arguments expecting uniqueness and get parity.

## Result and edge cases

Returns a `Logical`, or an `Error`.

- **Single argument.** `XOR(x)` is just the truthiness of `x` — parity of one item.
- **Two arguments.** Behaves as exclusive disjunction; this is the only arity at which the
  intuitive reading and the implemented rule agree for all inputs.
- **Three or more.** Parity. `XOR(TRUE,TRUE,TRUE)` is `TRUE`.
- **Zero versus nonzero.** Nonzero numbers are true; only exact zero is false.
- **Text and empty in ranges.** Skipped, so they do not disturb the parity count.
- **All items skipped.** `#VALUE!`.
- **Arrays.** Scanned whole, not lifted elementwise.

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | Nothing in the argument list classified as a truth value. |
| `#VALUE!` | Text supplied directly as an argument (reference-engine coercion outcome). |
| any incoming error | Propagates unchanged. |

Microsoft's `XOR` page is the documented source for the argument rules; it was not re-fetched at
this revision. The direct-text outcome is reported as a reference-engine behaviour.

## Relationships

- `OR` and `XOR` agree whenever at most one item is true and diverge as soon as two are.
- `AND` is the conjunction over the same scan.
- `XOR` was added in Excel 2013; workbooks that must open in earlier versions use
  `MOD(SUMPRODUCT(--(conditions)), 2) = 1` or nested `IF` chains instead. The Handbook does not
  treat `XOR` as a Compatibility-category replacement pair — it supersedes an idiom, not a
  function.
- `BITXOR` is a different function entirely: bitwise exclusive-or on integers, not a logical
  reduction. Readers searching for "XOR" frequently want that one.

## Notes for implementers

1. Accumulate parity as a single boolean toggled on each true item. Counting and then testing the
   low bit is equivalent but invites overflow thinking that does not apply here.
2. The "saw at least one classified item" flag is separate from the parity accumulator, exactly as
   in `AND` and `OR`; the empty scan is not `FALSE`.
3. `XOR` is associative and commutative over the classified items, so scan order does not affect
   the value — but it *does* affect which error surfaces first if more than one error is present.
   Order is therefore still observable.

## What has not been checked

There is no Handbook vector suite for `XOR`; `vectors/` publishes nothing at this revision. No
Excel-comparison evidence record names `XOR`.

Probes worth running first:

1. `=XOR(TRUE,TRUE,TRUE)` — the parity-versus-uniqueness question, directly. Everything else on
   this page is downstream of it.
2. `=XOR("TRUE")` — direct logical-looking text.
3. `=XOR(A1:A3)` where the range holds text, a blank, and one `TRUE` — confirms that skipped items
   do not flip parity.
4. `=XOR(A1:A3)` holding only text — the empty-scan `#VALUE!`.
5. Two errors of different codes in one scan, in each order — establishes whether error precedence
   or positional order decides which surfaces.

## Page vocabulary

| Term | Meaning |
|---|---|
| parity reduction | Result determined by whether the count of true items is odd |
| truth scan | The left-to-right walk that classifies expanded arguments |
| empty scan | A call in which every item was skipped; the `#VALUE!` case |

## Sources

- Microsoft, XOR function —
  <https://support.microsoft.com/en-us/office/xor-function-1548d4c2-5e47-4f77-9a92-0533bba14f37>
  (documented source for syntax and argument rules; not re-fetched at this revision).
- Handbook call-model chapters
  [02 Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [03 The call pipeline](../model/03-call-pipeline.md).
- `data/functions/FUNC.XOR.json`, `data/presence/FUNC.XOR.json`.
- OxFunc `crates/oxfunc_core/src/functions/xor_fn.rs` at commit 473efa3 — the parity accumulator
  and shared classifier, read as implementation facts about the reference engine.
