---
schema: efh.function-page/v1
function_id: FUNC.IF
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
family: if_fn
role_in_family: "The branch selector: evaluates a condition and returns one of two branch values verbatim."
---

# IF

## What it computes

`IF` selects between two values according to the truthiness of a condition, and returns the
selected value **verbatim** — it does not coerce, reshape, or inspect what it returns.

The rule:

1. Prepare `logical_test` and reduce it to a truth value. A `Logical` is itself; a `Number` is
   false when exactly 0 and true otherwise; an `Empty` or omitted condition reads as false; text
   that does not read as a number fails; an error propagates.
2. If the condition is true, return `value_if_true`. Otherwise return `value_if_false`.
3. Only the selected branch's value is delivered. `IF` is classified as a **lazy branch selector**
   in the call model: the unselected branch does not contribute to the result, which is why
   `IF(TRUE, lambda)` can pass a callable value through unchanged
   ([chapter 03](../model/03-call-pipeline.md), "Functions that step outside the plain pipeline").

Two structural properties follow from "verbatim", and they are what make `IF` more than sugar:

- **`IF` is kind-preserving.** If the selected branch is text, `IF` returns text; if it is an
  error, `IF` returns that error. `IF` is not an error handler — `IFERROR` is.
- **`IF` is reference-aware.** Its argument-preparation profile is
  `ArgPreparationProfile::RefsVisibleInAdapter`: it receives live references rather than
  pre-resolved values and controls dereference timing itself.

## Arguments

`IF(logical_test, value_if_true, [value_if_false])`

- **`logical_test`** — required. A condition, or anything that can be reduced to one. The most
  common misunderstanding lives here: `logical_test` is *not* required to be a `Logical`. Numbers
  work, and the zero test is exact.
- **`value_if_true`** — required by the registry's arity (minimum 2 arguments). Whatever value the
  formula should produce when the condition holds.
- **`value_if_false`** — optional; arity maximum is 3. What an *omitted* third argument produces is
  the sharpest documented gap on this page: Microsoft's `IF` page marks the argument optional but
  does not state the value returned when it is left off. The reference engine returns the
  `Logical` FALSE.

There is a third case that is neither "supplied" nor "omitted": a **present but empty slot**, as in
`IF(cond, 1, )`. Microsoft's page addresses this obliquely under common problems, noting that a
missing argument for either branch shows as `0` in the cell. So the documented reading is that an
empty slot yields `0` while a genuinely absent third argument is undocumented — and the reference
engine distinguishes `Missing` from `Empty` at exactly this seam
([chapter 01](../model/01-value-universe.md), "Missing versus Empty"). Whether Excel's two cases
really differ is listed below as unchecked, and it is the most useful probe on the page.

## Result and edge cases

Returns whatever kind the selected branch carries: `Number`, `Text`, `Logical`, `Error`, or an
array.

- **Condition is text.** `IF("", "")` surfaces `#VALUE!` in the reference engine — empty text is
  not zero and is not treated as false.
- **Condition is an empty cell.** Reads as false; the false branch is taken.
- **Condition is an error.** Propagates; neither branch is selected.
- **Branch is an error.** Returned as-is. `IF(TRUE, 1/0, 0)` is `#DIV/0!`.
- **Branch is an array.** Delivered as an array; with an array-shaped *condition*, `IF` produces a
  result of the condition's shape, materialising each branch against that shape. A branch array
  whose shape disagrees with the condition's is a shape failure rather than a silent broadcast in
  the reference engine.
- **Error folding.** The classification records `ErrorCollapseProfile::SelectorBranch` with
  `ErrorAlgebra::CanonicalExcelLegacy` — the branch-selector family collapses competing branch
  errors by Excel's classic precedence order rather than by position
  ([chapter 03](../model/03-call-pipeline.md), "Error folding").

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | `logical_test` is text that does not read as a number. |
| any incoming error | An error in `logical_test` propagates; an error in the *selected* branch is returned unchanged. |
| `#NAME?` | Documented by Microsoft as the symptom of a misspelled formula — a parse-level, not an `IF`-level, failure. |

Microsoft's `IF` page documents the `#NAME?` symptom and the "0 in cell" symptom for missing branch
arguments; it does not enumerate a runtime error table for `IF`. The `#VALUE!` on text conditions
is reported here as a reference-engine behaviour.

## Relationships

- **`IFS`** replaces long `IF(a, x, IF(b, y, IF(c, z, ...)))` chains with a flat condition/value
  list. It is not a superseding pair — Microsoft retains both and they are not equivalent at the
  edges: `IFS` returns `#N/A` when nothing matches, where a nested `IF` chain returns its innermost
  false branch.
- **`SWITCH`** compares one expression against a list of candidate values rather than evaluating
  independent conditions.
- **`IFERROR` and `IFNA`** branch on error-ness rather than on truth, and they are what people
  reach for after discovering that `IF` returns errors verbatim.
- **`CHOOSE`** selects by 1-based index rather than by condition.
- **`AND` / `OR`** are the usual condition builders; note that they evaluate all their arguments,
  so `IF(AND(...), ...)` does not short-circuit the way a programmer expects.
- Readers confuse `IF` with `IFS` most often, and with `SUMIF`/`COUNTIF` second most often — the
  latter are aggregates whose `IF` is a criteria filter, not a branch.

## Notes for implementers

1. **Laziness is observable.** Evaluating both branches and then discarding one changes behaviour
   whenever a branch is expensive, volatile, or returns a callable. The call model classifies `IF`
   as a lazy branch selector for that reason; an eager implementation is not merely slower.
2. **Keep `Missing` and `Empty` apart in the branch slots.** The reference engine maps an omitted
   third argument to the `Logical` FALSE and an empty cell to numeric `0`. Collapsing the two
   upstream of the function makes the distinction unrecoverable, and the documented Microsoft
   symptom ("0 in cell") suggests the two really do differ.
3. **Reference awareness is not optional.** Because `IF` sees live references, the branch it does
   not take is never dereferenced. An implementation that resolves arguments before dispatch will
   produce reference-resolution errors that Excel would never raise.
4. **Shape first, then select.** With an array condition, the result shape is the condition's
   shape; branches are materialised against it. Deciding shape after selection gives the wrong
   answer whenever one branch is scalar and the other is not.

## What has not been checked

There is no Handbook vector suite for `IF`; `vectors/` publishes nothing at this revision, so no
suite-scoped claim exists. No Excel-comparison evidence record names `IF`. Nobody has checked this
function's edge behaviour against Excel for this Handbook.

Probes worth running, in priority order:

1. `=IF(FALSE, 1)` versus `=IF(FALSE, 1, )` — the `Missing`-versus-empty-slot question. Microsoft
   documents `0` for the empty-slot symptom and documents nothing for the omitted case; the
   reference engine returns FALSE for the omitted case. If Excel returns `0` for both, the
   reference engine diverges; if it returns FALSE and `0` respectively, the distinction is real
   and load-bearing.
2. `=IF("", 1, 2)` and `=IF("0", 1, 2)` — does Excel coerce numeric-looking text in the condition,
   and does empty text fail?
3. `=IF(A1, 1, 2)` with `A1` empty — confirms blank reads as false.
4. `=IF({TRUE;FALSE}, {1;2}, {3;4})` and mismatched-shape variants — pins the array behaviour and
   whether shape disagreement broadcasts or errors.
5. `=IF(TRUE, LAMBDA(x,x))` entered in a cell — confirms the lazy-selector claim that a callable
   passes through unchanged (expected to publish `#CALC!` as the callable's core projection).
6. Two different error codes in the two branches with an array condition selecting both — tests
   whether `SelectorBranch` folding by canonical precedence is observable.

## Page vocabulary

| Term | Meaning |
|---|---|
| lazy branch selector | Evaluates and returns only the selected branch, verbatim |
| `ArgPreparationProfile::RefsVisibleInAdapter` | The function sees live references and controls dereference itself |
| `ErrorCollapseProfile::SelectorBranch` | Branch-selector family; competing branch errors collapse by precedence |
| `ErrorAlgebra::CanonicalExcelLegacy` | Excel's classic error-precedence order |
| Missing versus Empty | Omitted argument slot versus a referenced blank cell; distinct kinds |

## Sources

- Microsoft, IF function —
  <https://support.microsoft.com/en-us/office/if-function-69aed7c9-4e8a-4755-a9bc-aa8bbff73be2>
  (fetched for this revision; the optional marking of `value_if_false`, the "0 in cell" symptom for
  missing branch arguments, the `#NAME?` symptom, and the nesting note are from that page. It does
  not state what an omitted `value_if_false` returns).
- Handbook call-model chapters
  [01 The value universe](../model/01-value-universe.md),
  [02 Coercion and lifting](../model/02-coercion-and-lifting.md),
  [03 The call pipeline](../model/03-call-pipeline.md).
- `data/functions/FUNC.IF.json` — arity, classification axes, localized names.
- `data/presence/FUNC.IF.json` — the reference-engine module carrying the surface.
- OxFunc `crates/oxfunc_core/src/functions/if_fn.rs` at commit 473efa3 — the condition
  classification, the `Missing`→FALSE and `Empty`→`0` branch mapping, and the shape-first array
  handling, read as implementation facts about the reference engine.
