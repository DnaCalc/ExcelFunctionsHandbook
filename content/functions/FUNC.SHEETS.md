---
schema: efh.function-page/v1
function_id: FUNC.SHEETS
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
family: reference_metadata_family
role_in_family: The sheet-count member — the cardinality answer to SHEET's index answer.
---

## What it computes

`SHEETS([reference])` returns **how many sheets** a reference spans, or — with the argument
omitted — how many sheets the containing workbook has.

Microsoft's description is one line: "Returns the number of sheets in a reference." The argument
description supplies the omitted case: "If Reference is omitted, SHEETS returns the number of
sheets in the workbook that contains the function."

*The quoted one-line description is Microsoft's text, from its support article <https://support.microsoft.com/en-us/office/sheets-function-770515eb-e1e8-45ce-8066-b557e5e4b80b>.*

So the function has two quite different jobs depending on whether you give it an argument, and
the two are worth separating:

- **`SHEETS()`** — a workbook census. How many sheets exist.
- **`SHEETS(ref)`** — a span measurement. How many sheets this reference reaches across, which is
  `1` for an ordinary reference and larger only for a **three-dimensional** reference such as
  `Sheet1:Sheet3!A1` ([the value universe](../model/01-value-universe.md) calls that a
  three-dimensional reference shape). `SHEETS(Sheet1:Sheet3!A1)` is `3`.

That second job is the one the function was really added for. Before dynamic arrays, a 3-D
reference was the only way a single argument could reach across sheets, and `SHEETS` is how a
formula finds out that it did.

The counting rule is documented and is the same as `SHEET`'s: "SHEETS includes all worksheets
(visible, hidden, or very hidden) in addition to all other sheet types (macro, chart, or dialog
sheets)." A workbook showing three tabs can report `5`.

`SHEETS` is a host query (`HostInteractionClass::WorkbookState`) and is reference-aware
(`RefsVisibleInAdapter`): a workbook must exist to answer it. The Handbook's reference engine
refuses every one of its battery inputs with `cannot-call: requires-host-facility: composite` —
the correct answer for an engine with no workbook, not a coverage gap.

## Arguments

`reference` — **optional**, at most one. The published signature is `SHEETS([reference])`.

Unlike `SHEET`, this function takes **only a reference**. It does not accept a sheet name as
text: there is no "how many sheets does the sheet called X span" question to ask. That asymmetry
between the two siblings is easy to miss and is the source of most `SHEETS` errors.

The argument is a reference position, not a value position — the live reference must survive
argument preparation, because the answer depends on the reference's *shape*, not on the values
it designates.

The omitted form is a workbook-level query, not a caller-aware one in the coordinate sense: it
does not depend on which cell the formula is in, only on which workbook. That is why the declared
volatility is `NonVolatile` — the answer changes only when a sheet is added or removed, which is
a structural edit.

## Result and edge cases

Returns a `Number` — a count, minimum `1` for any valid reference.

- **An ordinary reference is `1`**, whether it is a single cell, an area, or a structured
  reference; none of those spans sheets.
- **A 3-D reference returns the span size**, counting every sheet between the endpoints in tab
  order — which means hidden and chart sheets caught inside the span are counted too, by the
  documented counting rule. A `Sheet1:Sheet3` span in a workbook with a hidden sheet between them
  is `3`, not `2`, and the hidden sheet's cells participate in whatever the 3-D reference is
  doing. That is a real workbook hazard, and `SHEETS` is how you detect it.
- **A multi-area reference** spanning two sheets (`(Sheet1!A1, Sheet2!A1)`) — not addressed by
  the documentation, not established here. Whether it counts 2, counts 1, or errors is open.
- **An external-workbook reference** — whose sheet count is being returned, and does it differ
  when the source workbook is closed? Not addressed, not established.
- **`SHEETS` is not available in the object model.** Microsoft says so explicitly: "SHEETS is not
  available in the Object Model (OM) because the Object Model already includes similar
  functionality." Automation code should read `Workbook.Sheets.Count`.

## Errors

- **`#REF!`** — documented: "If reference is not a valid value, SHEETS returns the #REF! error
  value."

Note the difference from `SHEET`, which has two documented errors because it has two argument
forms. `SHEETS` takes only references, so it has only the reference error. There is no `#N/A`
path, because there is no name lookup to fail.

`#NAME?` would arise before `SHEETS` runs, from an unresolvable defined name, in reference
resolution ([coercion and lifting](../model/02-coercion-and-lifting.md)).

Arity failure (two or more arguments) is expected to be refused at formula entry rather than
evaluated ([the call pipeline](../model/03-call-pipeline.md)).

## Relationships

- **`SHEET([reference])`** is the index to `SHEETS`' count. Same module, same 2013 vintage, same
  documented counting rule, routinely confused. Mnemonic: singular returns a position, plural
  returns a quantity. Note also that `SHEET` accepts a text sheet name and `SHEETS` does not.
- **`INFO("numfile")`** counts worksheets across **all open workbooks**; `SHEETS()` counts within
  the containing workbook. Two different numbers that are equal only when one file is open.
- **`AREAS(reference)`** counts the areas in a multi-area reference the way `SHEETS` counts the
  sheets in a 3-D one. They are the two cardinality functions over reference shape, they share an
  implementation module, and together they characterise a reference's geometry.
- **`ROWS` and `COLUMNS`** complete the set: `ROWS`, `COLUMNS`, `SHEETS` and `AREAS` are the four
  functions that measure a reference instead of reading it.
- **`CELL`, `INFO`, `ISFORMULA`, `FORMULATEXT`** are the other host-backed Information functions.

## Notes for implementers

1. **Count every sheet type**, including hidden, very hidden, chart, macro and dialog sheets. An
   implementation that enumerates visible worksheets will under-report, silently and
   workbook-dependently.
2. **The argument's shape is the subject.** `SHEETS` must inspect the reference's sheet span, not
   dereference it. A values-only path cannot implement this function at all.
3. **Do not accept a text sheet name.** `SHEET` does; `SHEETS` does not. Adding it for symmetry
   would be a divergence.
4. **`SHEETS()` and `SHEETS(ref)` are different queries** — a workbook census and a span
   measurement. Model them separately rather than passing a null reference into one code path.
5. **Decide the multi-area and external-reference cases explicitly** and record them as decisions
   awaiting evidence.

## What has not been checked

No Handbook vector suite exists for `SHEETS`; `vectors/` publishes nothing for this function. No
Excel-comparison evidence record names `SHEETS` as a subject — **nobody has checked this
function's answers against Excel inside the Handbook's record.**

There are also **no implementation-side answers to compare against**: the reference engine
refuses every battery input for want of a host. This page holds Microsoft's documentation and
the declared structure.

What a host-backed harness would have to establish:

1. **The 3-D span count with a hidden sheet inside the span.** This is the exhibit that makes the
   documented counting rule matter: a `Sheet1:Sheet3` reference whose middle sheet is hidden
   should report `3`, and a formula summing over that reference is silently including data the
   user cannot see. Confirming it turns a documentation sentence into a warning worth acting on.
2. **`SHEETS()` versus `INFO("numfile")` with two workbooks open**, to demonstrate that the two
   are different quantities.
3. **A multi-area reference across sheets** — 2, 1, or `#REF!`.
4. **External references**, open and closed source workbook.
5. **The `#REF!` path**, provoked by deleting a sheet inside a 3-D span.
6. **A structured reference and a spill anchor** as arguments, both expected `1`, neither
   measured.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `SHEETS` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-SHEET-001: SHEET and SHEETS return #N/A for invalid non-error arguments and evaluate to #NAME? in compatibility version 1

*Quirk; measured; recorded 2026-11-22.* Applies to SHEET, SHEETS.

In Excel 16.0.20430 and 16.0.20527, SHEET([value]) accepts a reference or sheet-name string and
returns its 1-based workbook sheet index (defaulting to the caller's sheet when omitted),
returning #N/A for numbers, logicals, arrays, or unknown sheet names. SHEETS([reference])
counts distinct sheets spanned by a 3-D reference (or total workbook sheets including hidden
sheets when omitted) and returns #N/A for any non-reference non-error value. Both require
_xlfn. and evaluate to #NAME? in compatibility version 1.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=SHEET()` | as typed | `2.0` (`0x4000000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=@SHEET()` | as typed | `#NAME?` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=SHEETS()` | as typed | `26.0` (`0x403a000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=@SHEETS(Data!A1)` | as typed | `#NAME?` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Measured in Excel 16.0.20527.20088 in prefix_metadata (rows 318-319) and
f2_rows (row 164) plus Excel 16.0.20430 controls, and matched across 3,093/3,093 SHEET and
3,107/3,107 SHEETS full-bank rows.

Evidence:

- DnaCalc `crates/calc-func/src/catalogue/host_facts.rs`, lines 338-392 at `5f45a67148` — SHEET name/reference lookup and SHEETS 3-D sheet counting
- DnaCalc `crates/calc-func/witness-status.tsv`, SHEET; SHEETS at `5f45a67148` — 3,093/3,093 SHEET and 3,107/3,107 SHEETS full-bank rows matched
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/prefix_metadata.capture.json`, sheet prefix_metadata row 318 column 1 at `5f45a67148` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/prefix_metadata.capture.json`, sheet prefix_metadata row 318 column 2 (compat-1) at `5f45a67148` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/prefix_metadata.capture.json`, sheet prefix_metadata row 319 column 1 at `5f45a67148` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/f2_rows.capture.json`, sheet f2_rows row 164 column 4 (compat-1) at `5f45a67148` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| 3-D reference | A reference spanning a range of sheets, e.g. `Sheet1:Sheet3!A1` |
| span measurement | The count of sheets a reference reaches across |
| workbook census | The omitted-argument form: how many sheets the workbook has |
| host query | A call answered by workbook state rather than by computation |

## Sources

- Microsoft, "SHEETS function" —
  <https://support.microsoft.com/en-us/office/sheets-function-770515eb-e1e8-45ce-8066-b557e5e4b80b>.
  Read for this page: the description, the syntax, the `reference` argument description verbatim
  including the omitted case, the counting remark, the `#REF!` condition, and the object-model
  note.
- Microsoft, "SHEET function" —
  <https://support.microsoft.com/en-us/office/sheet-function-44718b6f-8b87-47a1-a9d6-b701c06cff24>.
  Read for the sibling's argument forms and error pair, used in the comparison above.
- Handbook, [the value universe](../model/01-value-universe.md) — the three-dimensional and
  multi-area reference shapes this function measures.
- Handbook, [the call pipeline](../model/03-call-pipeline.md) — `RefsVisibleInAdapter`,
  host-side adaptation, and the admission boundary.
- Handbook, [the execution context](../model/04-execution-context.md) — the workbook state a
  host query reads.
- `data/functions/FUNC.SHEETS.json` — identity (`xlfSheets`, code 587), the published signature
  `SHEETS([reference])`, arity 0–1, and the declared host-interaction and argument-preparation
  axes, as projected at OxFunc `473efa3`.
