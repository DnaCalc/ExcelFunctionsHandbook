---
schema: efh.function-page/v1
function_id: FUNC.AND
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
family: and_fn
role_in_family: "The conjunction reducer: scans every argument for truth values and returns FALSE on the first falsy one."
---

# AND

## What it computes

`AND` is not a two-operand boolean connective. It is a **variadic reduction over a truth scan**,
and that difference is the whole content of the function.

The scan walks the argument list left to right. Each argument is expanded: a direct scalar
contributes itself; a reference or array contributes every cell it designates, in row-major order.
Each expanded item is then classified into exactly one of three buckets:

| Item | Classification |
|---|---|
| `Logical` | its own truth value |
| `Number` | false if the number is exactly 0, true otherwise |
| Text or empty inside an array or reference argument | **skipped** — contributes nothing |
| `Error` | the scan stops and that error is the result |

The result is then:

1. **FALSE** if any item classified false;
2. **TRUE** if at least one item classified (true or false) and none classified false;
3. **`#VALUE!`** if no item classified at all — every item was skipped.

Rule 3 is the clause readers forget. `AND` over a range holding nothing but text is not vacuously
TRUE; it is an error. Microsoft's documentation states this directly for the range case: when the
specified range contains no logical values, `AND` returns `#VALUE!`. In logical terms `AND` is
therefore *not* the fold of conjunction over an empty list — it refuses the empty case rather than
returning the identity element.

## Arguments

`AND(logical1, [logical2], ...)`

- `logical1` is required; the registry records an accepted arity of 1 to 255 arguments.
- Every argument is a *condition or a container of conditions*. Microsoft describes them as values
  that must evaluate to logical values, or arrays and references containing logical values.
- There are no optional-argument defaults to learn: every supplied slot is scanned, and an
  omitted slot between commas delivers the `Missing` marker, which the reference engine treats as
  contributing nothing (the same as a skipped item).

The commonly misunderstood position is *all of them at once*: readers expect `AND` to coerce
whatever it is handed, and it does not. Numbers and logicals participate; text and blanks reached
through a range do not. See the shared rule in
[Coercion and lifting](../model/02-coercion-and-lifting.md) — direct arguments and range-scanned
cells are governed by different policies, and `AND` is one of the families that exercises the
difference.

## Result and edge cases

Returns a `Logical`, or an `Error`.

- **Zero versus nonzero.** Any nonzero number is true, including negative numbers and the largest
  finite double. Only exact zero is false. There is no rounding or tolerance in this test.
- **Text.** Text reached through a range is skipped (documented). Text passed *directly* as an
  argument is treated by the reference engine as a `#VALUE!` failure rather than as a skipped
  item — the direct/scan asymmetry described in
  [Coercion and lifting](../model/02-coercion-and-lifting.md). Whether Excel agrees for the
  specific case of the strings `"TRUE"` and `"FALSE"` is not settled here; see below.
- **Empty cells.** Skipped, per the documented ignore rule.
- **All items skipped.** `#VALUE!`, per rule 3 above.
- **Arrays.** `AND` consumes an array whole, by scanning it; it is not a scalar kernel that lifts
  elementwise, so `AND({TRUE,FALSE})` is a single `FALSE`, not a two-element array. The
  classification chip for this is `LiftBroadcastProfile::SurfaceNative`.

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | No argument or scanned cell classified as a truth value at all (documented). |
| `#VALUE!` | Text supplied directly as an argument, per the reference engine's coercion outcome. |
| any incoming error | An error value among the arguments or scanned cells propagates unchanged. |

The `#VALUE!`-on-no-logical-values rule is documented by Microsoft on the `AND` page linked under
Sources. The direct-text rule is a reference-engine behaviour reported here as such, not as a
verified statement about Excel.

## Relationships

- `OR` is the dual: same scan, same skip rules, same empty-scan `#VALUE!`, returning TRUE on the
  first truthy item instead of FALSE on the first falsy one.
- `XOR` uses the same scan and returns the **parity** of the truthy items rather than their
  conjunction, which makes it a genuinely different reduction and not "OR without the ties".
- `NOT` is the unary complement, and is *not* a reducer: it takes exactly one argument.
- `IF` is where `AND` results usually land. `IF(AND(a,b), x, y)` is the idiom; note that `AND`
  evaluates every argument, so it does not give you short-circuit evaluation of side effects — it
  gives you a short-circuited *scan* over already-evaluated values.
- Readers confuse `AND` with the `*` idiom (`(a>1)*(b>2)`), which multiplies coerced logicals and
  therefore has completely different empty/text behaviour and returns a number.

## Notes for implementers

1. **Three buckets, not two.** Modelling the item classification as `Option<bool>` — true, false,
   or skip — is what makes rule 3 expressible. A two-valued model cannot distinguish "no truth
   values present" from "all present values were true".
2. **The skip rule is provenance-sensitive.** You must know whether an item arrived as a direct
   scalar or by scanning a container before you can decide whether text is skipped or is an error.
   The pipeline's origin tagging (see [The call pipeline](../model/03-call-pipeline.md), stage 2)
   exists for exactly this.
3. **Early exit is an optimisation with a visible edge.** Returning FALSE the moment a falsy item
   is seen is only safe if you have already committed to error propagation ordering — an error
   *after* the first FALSE would never be seen. Whether Excel's own scan stops early is a real,
   observable question and is listed below as unchecked.

## What has not been checked

There is no Handbook vector suite for `AND`; `vectors/` publishes nothing at this revision, so no
suite-scoped claim exists for it. No Excel-comparison evidence record names `AND`.

The probes that would settle the open questions, in the order worth running:

1. `=AND("TRUE")` and `=AND("true")` — does Excel accept logical-looking text as a direct
   argument, or return `#VALUE!`? This is the single most load-bearing unknown on the page.
2. `=AND(A1)` with `A1` holding the text `TRUE` — the range-scan counterpart. The documented
   ignore rule predicts a skip and therefore `#VALUE!`, which would be a striking contrast with
   probe 1 if probe 1 returns TRUE.
3. `=AND(1,,2)` — is an omitted middle slot skipped, or does it behave as an empty cell, or as
   zero (which would make the whole call FALSE)?
4. `=AND(A1:A3)` where the range holds an error and a FALSE, in each order — does the error win
   regardless of position, or does an earlier FALSE short-circuit past it?
5. `=AND()` with no argument at all — an admission-boundary question (entry-time refusal) rather
   than a runtime one; see [The call pipeline](../model/03-call-pipeline.md).

## Page vocabulary

| Term | Meaning |
|---|---|
| truth scan | The left-to-right walk over expanded arguments that classifies each item |
| skipped item | An expanded item that contributes to neither the true nor the false bucket |
| `LiftBroadcastProfile::SurfaceNative` | The function consumes arrays itself; no elementwise lift |
| direct argument vs range scan | The two coercion policies distinguished in call-model chapter 02 |

## Sources

- Microsoft, AND function —
  <https://support.microsoft.com/en-us/office/and-function-5f19b2e8-e1df-4408-897a-ce285a19e9d9>
  (fetched for this revision; the argument rule, the ignore-text-and-empty rule for arrays and
  references, and the no-logical-values `#VALUE!` rule are all stated there).
- Handbook call-model chapters
  [01 The value universe](../model/01-value-universe.md),
  [02 Coercion and lifting](../model/02-coercion-and-lifting.md),
  [03 The call pipeline](../model/03-call-pipeline.md).
- `data/functions/FUNC.AND.json` — arity, classification axes, localized names.
- `data/presence/FUNC.AND.json` — the reference-engine module that implements the surface.
- OxFunc `crates/oxfunc_core/src/functions/and_fn.rs` and
  `crates/oxfunc_core/src/functions/aggregate_common.rs` at commit 473efa3 — the three-bucket
  classification and the direct-text coercion outcome described above, read as implementation
  facts about the reference engine.
