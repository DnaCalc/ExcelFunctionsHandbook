---
schema: efh.function-page/v1
function_id: FUNC.SHEET
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
family: reference_metadata_family
role_in_family: The sheet-index member — turns a sheet or reference into its position in the
  workbook's tab order.
---

## What it computes

`SHEET([value])` returns the **position number** of a sheet within the workbook.

Microsoft's description: "The SHEET function returns the sheet number of the specified sheet or
other reference." Called with no argument, it returns the number of the sheet containing the
formula.

The number is a **tab-order index**, not an identity. It counts sheets from the left of the tab
strip, and Microsoft states the counting rule with unusual completeness: "SHEET
locating/identification capability includes all worksheets (visible, hidden, or very hidden) in
addition to all other sheet types (macro, chart, or dialog sheets)."

That sentence is the function. Two consequences follow, and both are the reason `SHEET` results
should be treated with care:

1. **Hidden and very-hidden sheets are counted.** A workbook that shows three tabs may return `5`
   for the third one. The number does not match what a user can see.
2. **Chart sheets, macro sheets and dialog sheets are counted.** These are not worksheets and do
   not appear in most sheet enumerations, but they occupy positions.

And because it is positional, **the number is not stable**: dragging a tab, inserting a sheet, or
deleting one renumbers everything to its right. `SHEET` answers "where is this sheet right now",
never "which sheet is this".

`SHEET` is a host query (`HostInteractionClass::WorkbookState`) and is reference-aware
(`RefsVisibleInAdapter`): a workbook must exist to answer it, and the reference must survive
argument preparation. The Handbook's reference engine refuses every one of its battery inputs
with `cannot-call: requires-host-facility: composite` — the correct answer for an engine with no
workbook, not a coverage gap.

## Arguments

`value` — **optional**, at most one. The published signature is `SHEET([reference])`; Microsoft's
page writes it as `SHEET(value)` and describes the argument as "the name of a sheet or a
reference for which you want to obtain the sheet number".

The argument therefore has two admissible forms, and they take different paths:

| Form | Example | What it means |
|---|---|---|
| A reference | `SHEET(Sheet3!A1)` | The sheet the reference lives on |
| A sheet name as text | `SHEET("Sheet3")` | The sheet with that name |
| Omitted | `SHEET()` | The sheet containing the formula |

The text form is the one that surprises people: `SHEET` accepts a *name*, which most
reference-aware functions do not. That is why the two failure modes are different errors — an
invalid reference and an unknown name are distinct conditions with distinct results, documented
separately.

The omitted form is a **caller-aware** call ([the call pipeline](../model/03-call-pipeline.md)):
the answer comes from the execution context, not from any argument. Unlike `CELL`'s omitted
reference, which falls back to volatile application state, `SHEET()`'s fallback is a stable
property of where the formula lives — which is why the declared volatility here is
`NonVolatile`.

## Result and edge cases

Returns a `Number` — a 1-based tab position.

- **Hidden, very hidden, chart, macro and dialog sheets all count**, per the documented remark.
  This is the most consequential edge case and it is documented rather than inferred.
- **The number is positional and unstable** under tab reordering, insertion and deletion.
- **A 3-D reference** (`Sheet1:Sheet3!A1`) spans several sheets. What `SHEET` returns for one is
  not addressed by the documentation and is not established here — the first sheet of the span
  is the plausible answer, and plausible is not measured.
- **An external-workbook reference** — a reference into another file — raises the question of
  *whose* tab order is being counted. Not addressed by the documentation, not established here.
- **A multi-area reference** (`(Sheet1!A1, Sheet2!A1)`) spans two sheets. Not addressed, not
  established.
- **`SHEET` is not available in the object model.** Microsoft says so explicitly: "SHEET is not
  available in the Object Model (OM) because the Object Model already includes similar
  functionality." Automation code should read `Worksheet.Index` rather than evaluating this
  function.

## Errors

Both documented, and the pair is worth memorising because they encode the two argument forms:

- **`#REF!`** — "If the value argument is not a valid reference, SHEET returns the #REF! error."
- **`#N/A`** — "If the value argument is not a valid sheet name, SHEET returns the #NA error
  value." (Microsoft's page writes it `#NA`; the worksheet value is `#N/A`.)

So a broken *reference* gives `#REF!` while an unknown *name* gives `#N/A` — the lookup-style
error for the lookup-style argument. An implementation that collapses the two loses real
information.

Arity failure (two or more arguments) is expected to be refused at formula entry rather than
evaluated ([the call pipeline](../model/03-call-pipeline.md)).

## Relationships

- **`SHEETS([reference])`** is the count to `SHEET`'s index: how many sheets, rather than which
  one. They share an implementation module and a documentation lineage, both arrived in Excel
  2013, and they are routinely confused. `SHEET` returns a position; `SHEETS` returns a
  cardinality.
- **`CELL("filename", ref)`** is the traditional pre-2013 route to a sheet's *name*, extracted
  from the returned path with `MID` and `FIND`. `SHEET` gives a number, not a name, so it does
  not replace that idiom — there is still no clean built-in "what is this sheet called".
- **`INFO("numfile")`** counts worksheets across *all open workbooks*, which is a third and
  different quantity.
- **`ROW` and `COLUMN`** are the within-sheet coordinates; `SHEET` is the third axis.
- **`ISREF` and `INDIRECT`** are the usual companions when a sheet name is being assembled from
  text and needs validating first.
- **`ADDRESS`, `AREAS`, `FORMULATEXT`** share this function's implementation module in the
  reference engine and its reference-metadata character.

## Notes for implementers

1. **The count includes every sheet type.** An implementation that enumerates only visible
   worksheets will be off by the number of hidden and chart sheets to the left of the target —
   a silent, data-dependent error.
2. **Two argument forms, two error results.** Reference-not-valid is `#REF!`; name-not-found is
   `#N/A`. Route them separately.
3. **The omitted form needs the calling cell's sheet from the execution context**, not from an
   argument. It is a caller-aware call and must be declared as one.
4. **The reference must survive preparation unresolved**, or the sheet identity is lost before
   the function runs.
5. **Decide the 3-D, multi-area and external-reference cases explicitly**, and record them as
   decisions awaiting evidence. The documentation covers none of them.

## What has not been checked

No Handbook vector suite exists for `SHEET`; `vectors/` publishes nothing for this function. No
Excel-comparison evidence record names `SHEET` as a subject — **nobody has checked this
function's answers against Excel inside the Handbook's record.**

There are also **no implementation-side answers to compare against**: the reference engine
refuses every battery input for want of a host. This page holds Microsoft's documentation and
the declared structure.

What a host-backed harness would have to establish:

1. **The counting rule, demonstrated.** Build a workbook with a visible sheet, a hidden sheet, a
   very hidden sheet and a chart sheet interleaved, then read `SHEET` for each. The rule is
   documented; watching it produce numbers that disagree with the visible tabs is what makes it
   memorable and is the strongest exhibit this function could have.
2. **3-D references.** `SHEET(Sheet1:Sheet3!A1)` — first sheet of the span, an error, or
   something else.
3. **Multi-area references** spanning two sheets.
4. **External references**, both with the source workbook open and closed. Whose tab order is
   counted, and does the closed state change the answer?
5. **The two error paths**, provoked deliberately: a deleted-sheet reference for `#REF!`, a
   misspelled name for `#N/A`.
6. **`SHEET("")` and `SHEET` of a name that differs only in case or in surrounding spaces**,
   since the name-matching rule is unstated.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| tab-order index | The 1-based position of a sheet in the workbook, counting every sheet type |
| caller-aware | A call whose answer comes from the execution context rather than an argument |
| host query | A call answered by workbook state rather than by computation |
| positional, not identity | The number moves when tabs are reordered; it does not name a sheet |

## Sources

- Microsoft, "SHEET function" —
  <https://support.microsoft.com/en-us/office/sheet-function-44718b6f-8b87-47a1-a9d6-b701c06cff24>.
  Read for this page: the description, the syntax, the `value` argument description verbatim,
  the counting remark naming visible/hidden/very-hidden worksheets and macro/chart/dialog
  sheets, both documented error conditions, and the object-model note.
- Handbook, [the call pipeline](../model/03-call-pipeline.md) — `RefsVisibleInAdapter`,
  caller-aware functions, and the admission boundary.
- Handbook, [the execution context](../model/04-execution-context.md) — the calling cell and the
  workbook state a host query reads.
- Handbook, [the value universe](../model/01-value-universe.md) — three-dimensional and
  multi-area reference shapes.
- `data/functions/FUNC.SHEET.json` — identity (`xlfSheet`, code 586), the published signature
  `SHEET([reference])`, arity 0–1, and the declared host-interaction and argument-preparation
  axes, as projected at OxFunc `473efa3`.
