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
  - Notes
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
  the selected value becomes numeric `0`. In Excel a selected reference stays a reference:
  `ISBLANK(IFS(TRUE,Data!Z1))` is `TRUE` over an empty cell (FN-IFS-001).
- **Selected value is an error.** Returned verbatim. `IFS` selects; it does not inspect.
- **Arrays.** `IFS` is not surface-native for lifting: the reference engine declares
  `LiftBroadcastProfile::ByIndexScalarArrayLift` over argument positions 0, 1 and 2 — the first
  condition, the first value, and the second condition — and carries a note in the source that
  this position list was verified against live Excel 16.0 build 20026. An irregular list like that
  is exactly the kind of structure that cannot be derived from first principles, which is why the
  Handbook records it as a declared axis rather than a rule. Once a scalar test has selected a
  result, later arrays do not lift the call: `IFS(TRUE,1,{FALSE,TRUE},{2,3})` returns one `1`
  (FN-IFS-001).

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
3. **The lift positions are irreducible structure.** The reference engine declares positions 0, 1
   and 2 as lifting and the rest not, and in Excel an array in position 2 does not lift the call
   once a scalar test has selected a result (FN-IFS-001). This cannot be inferred, only recorded,
   and an implementation that lifts uniformly over all positions will diverge on array inputs to
   later pairs.
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
2. `=IFS({TRUE;FALSE}, 1, 2, 3)` and array arguments in the *fourth* position with no earlier test
   true — tests the declared lift-position list directly. If a later position lifts then, the
   declared axis is wrong; after a scalar test has selected a result, later arrays do not lift
   (FN-IFS-001).
3. `=IFS("", 1)` — whether empty text gives the same `#VALUE!` as other text, which takes the shape
   of its result: `=IFS("2",{"hit","hit"})` is `{#VALUE!,#VALUE!}` (FN-IFS-001).
4. `=IFS(A1, 1)` with `A1` empty — confirms blank reads as false rather than erroring.
5. Entering `=IFS(TRUE)` — establishes whether the unpaired condition is refused at entry (as
   Microsoft describes) or evaluates to `#VALUE!`.
6. `=IFS(1/0, 1, TRUE, 2)` — confirms the error in an early condition wins over a later match.
7. A selected `LAMBDA` that is then called, or passed through `LET` or `IFERROR`, which FN-IFS-001
   leaves open.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `IFS` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-IFS-001: IFS and SWITCH pass on the selected value whole, and later arrays do not lift the call

*Quirk; measured; recorded 2026-11-04.* Applies to IFS, SWITCH.

Once a scalar test selects a result, array arguments after it have no effect on the shape:
IFS(TRUE,1,{FALSE,TRUE},{2,3}) and SWITCH(2,2,1,{1,2},3) each return one 1, not an array, and
SWITCH("A","a",1,{"A","A"},2) returns 1 because the match ignores case. The selected value
itself is passed on as it is. A selected reference stays a reference:
ISBLANK(IFS(TRUE,Data!Z1)) is TRUE over an empty cell. A selected LAMBDA shows as #CALC! in the
cell, IFS(TRUE,LAMBDA(xx,xx)), and a selected array of LAMBDAs as #CALC! in each cell. A test
that is not TRUE or FALSE gives #VALUE! in the shape of its result: IFS("2",{"hit","hit"}) is
{#VALUE!,#VALUE!}. A direct error as the SWITCH expression stays one error:
SWITCH(#VALUE!,1,{1,2},0) is #VALUE!.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=IFS(TRUE,1,{FALSE,TRUE},{2,3})` | as typed | `1` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=SWITCH(2,2,1,{1,2},3)` | as typed | `1` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=SWITCH("A","a",1,{"A","A"},2)` | as typed | `1` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=ISBLANK(IFS(TRUE,Data!Z1))` | Data!Z1 `(empty cell)` (cells) | `TRUE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=ISBLANK(SWITCH(1,1,Data!Z1))` | Data!Z1 `(empty cell)` (cells) | `TRUE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=IFS(TRUE,LAMBDA(xx,xx))` | as typed | `#CALC!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=SWITCH(1,1,LAMBDA(xx,xx))` | as typed | `#CALC!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=IFS(TRUE,HSTACK(LAMBDA(xx,xx),LAMBDA(xx,xx)))` | as typed | `{#CALC!,#CALC!}` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=IFS("2",{"hit","hit"})` | as typed | `{#VALUE!,#VALUE!}` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=SWITCH(#VALUE!,1,{1,2},0)` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): 11 recorded selection readings in dynamic-array Excel, each with size
observers.

- Open: A selected LAMBDA that is then called, or passed through LET or IFERROR, has not been read in a published capture.

Evidence:

- DnaCalc `apps/dna-onecalc/tests/acceptance.rs` at `2d0eddcac8` — selection and callable readings pinned
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheets fn2_q016 to fn2_q022, fn2_q194, fn2_q195, fn2_q208, fn2_q209 at `2d0eddcac8` — recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q208, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q209, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q016, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q194, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q195, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q017, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q018, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q019, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q021, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q022, row 20, column 10 at `2d0eddcac8` (readings)

<!-- efh:notes end -->

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
