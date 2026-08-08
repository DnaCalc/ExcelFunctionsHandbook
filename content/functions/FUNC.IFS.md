---
schema: efh.function-page/v1
function_id: FUNC.IFS
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-STRUCT-0006
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
family: choose_ifs_family
role_in_family: "The flat condition ladder: scans condition/value pairs left to right and returns the first match."
---

# IFS

## What it computes

`IFS` evaluates a flat list of condition/value pairs left to right and returns the value belonging
to the **first** condition that is true. It is a ladder, not a table: the conditions are
independent expressions, tested in written order, and order therefore carries meaning.

The rule:

1. Take the arguments in pairs: `(logical_test1, value_if_true1)`, `(logical_test2,
   value_if_true2)`, and so on.
2. For each pair in order, reduce the condition to a truth value — a `Logical` is itself, a
   `Number` is false when exactly 0 and true otherwise, a blank or omitted condition reads as
   false, text fails, an error propagates.
3. On the first true condition, return its paired value and stop. Later conditions are not
   evaluated.
4. If no condition is true, return `#N/A`.

Step 4 is where `IFS` differs sharply from the nested `IF` chain it replaces. A nested chain ends
in an innermost false branch, which returns *something*; `IFS` has no final else, so an unmatched
input is an explicit `#N/A`. Microsoft's documented workaround is to make the final
`logical_test` the literal `TRUE`, whose paired value then acts as the default.

The absence of a default is a design choice worth respecting rather than working around
reflexively: `#N/A` on an unmatched input is a loud failure, and a `TRUE` catch-all converts it
into a silent one.

## Arguments

`IFS(logical_test1, value_if_true1, [logical_test2, value_if_true2], ...)`

- Arguments come strictly in pairs. The registry records an accepted arity of 2 to 254 arguments;
  Microsoft documents support for up to 127 conditions, which is the same bound expressed as
  pairs.
- **`logical_testN`** — a condition, or anything reducible to one. Numbers work; the zero test is
  exact.
- **`value_if_trueN`** — the result for that condition. Microsoft's page notes that values may be
  empty.
- There is no optional trailing default. The final-`TRUE` idiom is a convention, not a parameter.

The commonly misunderstood position is the **pairing itself**. An odd argument count is not a
runtime error in Excel's account of it: Microsoft documents that a `logical_test` supplied without
its `value_if_true` produces an entry-time "too few arguments" message. That places the failure at
the admission boundary rather than in the value plane
([chapter 03](../model/03-call-pipeline.md), "Arity and the admission boundary"). The reference
engine, which describes only the runtime surface, maps an odd argument count to `#VALUE!`.

## Result and edge cases

Returns whatever kind the selected value carries: `Number`, `Text`, `Logical`, `Error`, or array.

- **No condition true.** `#N/A` (documented).
- **Condition resolves to something that is neither TRUE nor FALSE.** Microsoft documents `#VALUE!`
  for this case.
- **Condition is text.** Fails; the reference engine surfaces `#VALUE!`, including for the empty
  string.
- **Condition is blank or an omitted slot.** Reads as false in the reference engine, so the ladder
  simply moves on.
- **Selected value is an omitted slot.** The reference engine returns `#VALUE!`; a blank *cell* as
  the selected value becomes numeric `0`.
- **Selected value is an error.** Returned verbatim. `IFS` selects; it does not inspect.
- **Arrays.** `IFS` is not surface-native for lifting: the reference engine declares
  `LiftBroadcastProfile::ByIndexScalarArrayLift` over argument positions 0, 1 and 2 — the first
  condition, the first value, and the second condition — and carries a note in the source that
  this position list was verified against live Excel 16.0 build 20026. An irregular list like that
  is exactly the kind of structure that cannot be derived from first principles, which is why the
  Handbook records it as a declared axis rather than a rule.

## Errors

| Error | Condition |
|---|---|
| `#N/A` | No `logical_test` evaluated true (documented). |
| `#VALUE!` | A `logical_test` resolved to a value that is neither TRUE nor FALSE (documented). |
| `#VALUE!` | Odd argument count, in the reference engine's runtime surface. Microsoft describes this case as an entry-time message instead. |
| any incoming error | An error in a `logical_test` propagates; an error in the selected value is returned unchanged. |

The classification records `ErrorCollapseProfile::SelectorBranch` with
`ErrorAlgebra::CanonicalExcelLegacy`: competing branch errors collapse by Excel's classic
precedence order ([chapter 03](../model/03-call-pipeline.md), "Error folding").

## Relationships

- **`IF`** is what `IFS` flattens. They are not interchangeable: a nested `IF` chain has a final
  else, `IFS` returns `#N/A` instead. Migration between them changes the unmatched case.
- **`SWITCH`** is the sibling for a different shape of problem: one expression compared against
  many candidate values, rather than many independent conditions. `SWITCH` does take a default.
  If every one of your `IFS` conditions is `x = something`, you want `SWITCH`.
- **`CHOOSE`** selects by 1-based index and shares an implementation module with `IFS` in the
  reference engine.
- `IFS` arrived in Excel 2016 (Office 365 channel first). It is not a Compatibility-category
  function and supersedes nothing; Microsoft retains `IF` in full.
- Readers confuse `IFS` with `SUMIFS`/`COUNTIFS`, which are aggregates with multiple criteria —
  a different family entirely despite the shared suffix.

## Notes for implementers

1. **Short-circuit is required, not an optimisation.** Conditions after the first match must not be
   evaluated; a condition that would error must not error if an earlier condition matched.
2. **The odd-arity split is real.** Whether an unpaired trailing condition is an admission failure
   or a `#VALUE!` depends on which surface you are implementing. State which one you target.
3. **The lift positions are irreducible structure.** Positions 0, 1 and 2 lift; the rest do not.
   This cannot be inferred, only recorded, and an implementation that lifts uniformly over all
   positions will diverge on array inputs to later pairs.
4. **`#N/A` is the result, not a failure.** Do not let an outer `IFERROR` habit in the surrounding
   code convert an honest no-match into a fabricated default without the author noticing.

## What has not been checked

There is no Handbook vector suite for `IFS`; `vectors/` publishes nothing at this revision, so no
suite-scoped claim exists for it.

One Excel-comparison record does name `IFS`: **EV-STRUCT-0006**, a structural-verification record
covering scalar-parameter array lift across a group of functions, in which `IFS` appears as a
body-only subject. That record's figures are group totals over several functions and do not
decompose onto `IFS`, so it establishes that `IFS` was inside a live-Excel structural comparison —
not a per-function result. Read the record itself for its scope and caveats.

Probes worth running, in priority order:

1. `=IFS(FALSE, 1, FALSE, 2)` — confirms the `#N/A` no-match rule.
2. `=IFS({TRUE;FALSE}, 1, 2, 3)` and array arguments in the *fourth* position — tests the declared
   lift-position list directly. If a later position lifts, the declared axis is wrong.
3. `=IFS("x", 1)` and `=IFS("", 1)` — confirms the documented non-boolean `#VALUE!` and whether
   empty text behaves the same.
4. `=IFS(A1, 1)` with `A1` empty — confirms blank reads as false rather than erroring.
5. Entering `=IFS(TRUE)` — establishes whether the unpaired condition is refused at entry (as
   Microsoft describes) or evaluates to `#VALUE!`.
6. `=IFS(1/0, 1, TRUE, 2)` — confirms the error in an early condition wins over a later match.

## Page vocabulary

| Term | Meaning |
|---|---|
| condition ladder | Ordered pairs tested in sequence, first match wins |
| final-`TRUE` idiom | Using a literal `TRUE` as the last condition to supply a default |
| `LiftBroadcastProfile::ByIndexScalarArrayLift` | Scalar kernel broadcast over a named list of argument positions |
| `ErrorCollapseProfile::SelectorBranch` | Branch-selector family; competing branch errors collapse by precedence |

## Sources

- Microsoft, IFS function —
  <https://support.microsoft.com/en-us/office/ifs-function-36329a26-37b2-467c-972b-4a39bd951d45>
  (fetched for this revision; the syntax, the 127-condition bound, the final-`TRUE` default idiom,
  the entry-time message for an unpaired condition, the non-boolean `#VALUE!`, and the `#N/A`
  no-match rule are all from that page).
- Handbook evidence record `EV-STRUCT-0006` (`content/evidence/records/EV-STRUCT-0006.json`) —
  names `IFS` as a body-only subject of a live-Excel structural array-lift comparison; group-scoped
  counts only.
- Handbook call-model chapters
  [01 The value universe](../model/01-value-universe.md),
  [02 Coercion and lifting](../model/02-coercion-and-lifting.md),
  [03 The call pipeline](../model/03-call-pipeline.md).
- `data/functions/FUNC.IFS.json`, `data/presence/FUNC.IFS.json`.
- OxFunc `crates/oxfunc_core/src/functions/choose_ifs_family.rs` at commit 473efa3 — the pair scan,
  the `#N/A` no-match result, the odd-arity `#VALUE!`, and the `lift_at(&[0, 1, 2])` declaration
  with its live-Excel verification note.
