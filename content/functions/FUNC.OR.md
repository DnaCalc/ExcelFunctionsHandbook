---
schema: efh.function-page/v1
function_id: FUNC.OR
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
family: or_fn
role_in_family: "The disjunction reducer: scans every argument for truth values and returns TRUE on the first truthy one."
---

# OR

## What it computes

`OR` is the dual of `AND`, and shares its whole machinery: a **variadic reduction over a truth
scan**, not a two-operand boolean connective.

The scan walks the argument list left to right, expanding each argument (a direct scalar
contributes itself; a reference or array contributes every designated cell, row-major) and
classifying each expanded item:

| Item | Classification |
|---|---|
| `Logical` | its own truth value |
| `Number` | false if exactly 0, true otherwise |
| Text or empty inside an array or reference argument | **skipped** |
| `Error` | the scan stops and that error is the result |

The result is:

1. **TRUE** if any item classified true;
2. **FALSE** if at least one item classified and none classified true;
3. **`#VALUE!`** if no item classified at all.

Rule 3 again matters more than it looks. `OR` is not the fold of disjunction over an empty list —
an empty scan does not return the identity FALSE, it returns an error. `OR` and `AND` agree
exactly on which inputs are error-shaped and disagree only on the reduction.

## Arguments

`OR(logical1, [logical2], ...)`

- `logical1` is required; the registry records an accepted arity of 1 to 255 arguments.
- Each argument is a condition or a container of conditions.
- An omitted slot between commas delivers `Missing`, which the reference engine treats as
  contributing nothing.

The misunderstood position is the same as for `AND`: readers expect uniform coercion and get a
provenance-sensitive rule instead. See
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns a `Logical`, or an `Error`.

- **Zero versus nonzero.** Any nonzero number is true; only exact zero is false.
- **Text.** Skipped when reached through a range (documented ignore rule for the `AND`/`OR`
  family). Text supplied *directly* is a `#VALUE!` coercion failure in the reference engine.
- **Empty cells.** Skipped.
- **All items skipped.** `#VALUE!`.
- **Arrays.** Consumed whole by scanning, not lifted elementwise; `OR({FALSE,TRUE})` is a single
  `TRUE`.

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | Nothing in the argument list classified as a truth value. |
| `#VALUE!` | Text supplied directly as an argument (reference-engine coercion outcome). |
| any incoming error | Propagates unchanged. |

The no-logical-values rule is documented by Microsoft for this family. The direct-text rule is
reported here as a reference-engine behaviour, not as a verified statement about Excel.

## Relationships

- `AND` is the dual, with an identical scan and skip policy.
- `XOR` shares the scan but reduces by **parity**, so it agrees with `OR` only when at most one
  item is true.
- `NOT` complements a single value; De Morgan (`NOT(OR(a,b)) = AND(NOT(a),NOT(b))`) holds for the
  classified cases but says nothing about the `#VALUE!`-on-empty-scan case, which has no boolean
  algebra counterpart.
- The `+` idiom (`(a>1)+(b>2)`) is the common substitute and behaves differently: it returns a
  number, it counts rather than tests, and it has no skip rule.

## Notes for implementers

1. `OR` and `AND` should share one scan and one item classifier; they differ only in the fold and
   in the early-exit predicate. Implementing them separately is how the empty-scan rule ends up
   inconsistent between the two.
2. The "saw at least one classified item" flag is a required piece of state, distinct from the
   accumulator. Without it, an all-skipped scan is indistinguishable from an all-false scan.
3. Early exit on the first true item is only safe once error ordering is decided; an error later
   in the scan would never be reached.

## What has not been checked

There is no Handbook vector suite for `OR`; `vectors/` publishes nothing at this revision. No
Excel-comparison evidence record names `OR`.

Probes that would settle the open questions:

1. `=OR("TRUE")` and `=OR("false")` — direct logical-looking text: accepted or `#VALUE!`?
2. `=OR(A1)` with `A1` holding the text `FALSE` — the range-scan counterpart; the documented
   ignore rule predicts `#VALUE!`.
3. `=OR(1,,0)` — is the omitted slot skipped, or does it read as an empty cell?
4. An error and a TRUE in the same scanned range, in each order — does the error always win, or
   can an earlier TRUE short-circuit past it? This is the sharpest test of whether Excel's scan
   really exits early.
5. Argument-count behaviour at 255 and 256 arguments — an admission-boundary question.

## Page vocabulary

| Term | Meaning |
|---|---|
| truth scan | The left-to-right walk over expanded arguments that classifies each item |
| skipped item | An expanded item contributing to neither bucket |
| empty scan | A call in which every item was skipped; the `#VALUE!` case |

## Sources

- Microsoft, OR function —
  <https://support.microsoft.com/en-us/office/or-function-7d17ad14-8700-4281-b308-00b131e22af0>
  (the governing document for the argument and ignore rules; not re-fetched at this revision —
  the sibling `AND` page was, and states the family rules in the same words).
- Handbook call-model chapters
  [01 The value universe](../model/01-value-universe.md),
  [02 Coercion and lifting](../model/02-coercion-and-lifting.md),
  [03 The call pipeline](../model/03-call-pipeline.md).
- `data/functions/FUNC.OR.json`, `data/presence/FUNC.OR.json`.
- OxFunc `crates/oxfunc_core/src/functions/or_fn.rs` and `aggregate_common.rs` at commit 473efa3 —
  the shared classifier and the empty-scan `#VALUE!`, read as implementation facts.
