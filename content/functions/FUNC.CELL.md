---
schema: efh.function-page/v1
function_id: FUNC.CELL
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
family: cell
role_in_family: Sole member of its module; the function that asks the host about a cell, and one
  of the two in this category that cannot be evaluated without one.
---

## What it computes

`CELL(info_type, [reference])` returns one property of a cell, selected by name.

It is not a computation. It is a **query against the host**: the address, the row and column
number, the number format, the protection state, the column width and the containing file are
all facts held by Excel, not derivable from any value. `CELL` is declared
`HostInteractionClass::WorkbookState` and `ArgPreparationProfile::RefsVisibleInAdapter` for
exactly that reason — it needs the live reference and it needs the workbook behind it.

The consequence, stated plainly because it governs everything else on this page: **a function
engine with no host cannot answer a `CELL` call at all.** The Handbook's own reference engine
declines every one of its battery inputs with `cannot-call: requires-host-facility:
caller-context`. That is not a gap in coverage or a defect; it is the honest and correct
response, and it means `CELL` can only ever be characterized against a running Excel.

Microsoft's published `info_type` table, verbatim:

| `info_type` | Returns |
|---|---|
| `"address"` | Reference of the first cell in reference, as text. |
| `"col"` | Column number of the cell in reference. |
| `"color"` | The value 1 if the cell is formatted in color for negative values; otherwise returns 0 (zero). |
| `"contents"` | Value of the upper-left cell in reference; not a formula. |
| `"filename"` | Filename (including full path) of the file that contains reference, as text. |
| `"format"` | Text value corresponding to the number format of the cell. |
| `"parentheses"` | The value 1 if the cell is formatted with parentheses for positive or all values; otherwise returns 0. |
| `"prefix"` | Text value corresponding to the "label prefix" of the cell. |
| `"protect"` | The value 0 if the cell is not locked; otherwise returns 1 if the cell is locked. |
| `"row"` | Row number of the cell in reference. |
| `"type"` | Text value corresponding to the type of data in the cell. |
| `"width"` | Returns an array with 2 items. |

Twelve names. The reference engine's declared parser accepts exactly these twelve, case- and
whitespace-insensitively.

Several of these — `"color"`, `"parentheses"`, `"prefix"`, and much of `"format"` — are
survivals from Lotus 1-2-3 compatibility and from Excel's macro-sheet era. They still answer,
but they describe a formatting world that modern workbooks mostly do not use.

## Arguments

**`info_type`** — required. Text naming the property. It is coerced to text and matched
case-insensitively after trimming, so `"ROW"`, `"row"` and `" Row "` are the same request. A
name outside the twelve is a failure, not a null result.

**`reference`** — optional, and this is the argument position most commonly misunderstood.
Microsoft's own caveat is worth quoting because it is unusually blunt for a reference page:
"Although technically reference is optional, including it in your formula is encouraged, unless
you understand the effect its absence has on your formula result."

What its absence does: `CELL` falls back to the **last cell that was changed**, which is a piece
of application state, not a property of the formula. The documented consequences are that
calculation timing varies by platform, that co-authoring can report the wrong active cell, and
that a plain recalculation (F9) can produce a new result with no edit anywhere. That is why the
function's declared volatility is `VolatileContextual` and its thread-safety class is
`HostSerialized`: an argument-free `CELL` is a formula whose answer depends on what the user did
last.

Supply the reference. There is no case in which omitting it makes a formula more correct.

Note also that `reference` is a **reference position, not a value position**: the live reference
survives argument preparation, which is what lets `"address"` and `"row"` mean anything. Handing
`CELL` a computed value where a reference is expected is a failure, not a coercion.

## Result and edge cases

The return kind depends on `info_type`: `Text` for `"address"`, `"filename"`, `"format"`,
`"prefix"` and `"type"`; `Number` for `"col"`, `"row"`, `"color"`, `"parentheses"` and
`"protect"`; whatever the cell holds for `"contents"`; and an **array of two items** for
`"width"` — the only `info_type` whose result is not a scalar, and a shape that will spill or be
intersected depending on context.

- **A multi-cell reference is reduced.** `"address"` reports the first cell, `"contents"` the
  upper-left. `CELL` does not aggregate.
- **`"contents"` returns the value, never the formula.** `FORMULATEXT` is the function for the
  formula text.
- **`"filename"` on an unsaved workbook** has a documented-empty answer in the folklore and no
  observation in this Handbook.
- **`"type"` is coarse.** It reports a one-letter classification of the cell's content, not the
  fine-grained kind that `TYPE` returns. The two functions have similar names and different
  jobs; see "Relationships".
- **`"format"` is locale- and version-sensitive.** The text codes describe number formats, and
  which code a given format maps to is a per-build fact this Handbook has not recorded.
- **Volatility.** With `reference` supplied the function is still declared
  `VolatileContextual` — the properties it reads (format, width, protection) can change without
  any formula input changing, and Excel's dependency graph does not track formatting edits the
  way it tracks values. A `CELL("format", A1)` result can therefore be stale after a formatting
  change until a recalculation happens.

## Errors

Microsoft's page is the documentation of record; the Handbook has read its `info_type` table and
its reference-argument caveat, and does not have a documented error table for this function to
reproduce.

What the reference engine's declared contract produces, stated as the reference engine's
behaviour and not as Excel's:

- an unrecognised `info_type` — `#VALUE!`
- a `reference` argument that is not a reference — `#VALUE!`
- an arity failure (zero arguments, or three) — `#VALUE!`
- no host available — the call is refused rather than answered

`#REF!` from a deleted target and `#NAME?` from an unresolvable name would arise before `CELL`
runs, in reference resolution ([coercion and lifting](../model/02-coercion-and-lifting.md)).

## Relationships

- **`INFO`** is `CELL`'s sibling: same category, same host dependence, same "cannot be evaluated
  without Excel" property, but asking about the environment rather than about a cell.
- **`TYPE`** and **`CELL("type", …)`** are not the same function and do not return the same
  alphabet. `TYPE` returns a numeric kind code for a *value*; `CELL("type", …)` returns a
  one-letter classification of a *cell*. If you want to know what kind of value you have, use
  `TYPE`.
- **`ROW` and `COLUMN`** give the same numbers as `CELL("row", …)` and `CELL("col", …)`, without
  the host-query machinery, and are the better choice.
- **`ADDRESS`** builds an address from coordinates; `CELL("address", …)` reads one from a
  reference. They meet in the middle.
- **`SHEET` and `SHEETS`** answer the sheet-level questions `CELL` does not.
- **`ISFORMULA` and `FORMULATEXT`** answer the formula questions `CELL("contents")` and
  `CELL("type")` cannot.
- **`CELL("filename", …)`** is the traditional route to the workbook or sheet name, usually
  wrapped in `MID`/`FIND`. It is fragile — it depends on the file having been saved and on the
  path's punctuation — and it is the reason many workbooks break when moved.

## Notes for implementers

1. **`CELL` cannot be implemented in the function layer.** It must be a declared host query with
   a defined failure mode when no host is present. Faking `"filename"` or `"format"` from local
   state produces answers that look right and are not.
2. **The reference must survive preparation unresolved.** If the dispatch layer dereferences,
   `"address"`, `"row"`, `"col"` and `"width"` all lose their subject.
3. **The omitted-`reference` path is a different query**, against application state (last changed
   cell), not against an argument. It should be modelled as such, and its non-determinism
   documented rather than smoothed over.
4. **`"width"` returns an array of two.** A scalar-shaped return path will silently drop half the
   answer.
5. **`"format"` and `"prefix"` are locale- and version-scoped.** Any claim about their outputs
   must name the build and locale, per the Handbook's claim rules.

## What has not been checked

No Handbook vector suite exists for `CELL`; `vectors/` publishes nothing for this function. No
Excel-comparison evidence record names `CELL` as a subject — **nobody has checked this
function's answers against Excel inside the Handbook's record.**

And there is a stronger statement to make here than on most pages: **`CELL` has no
implementation-side answers to compare against either.** The reference engine refuses every
battery input because it has no host facility. So for this function the Handbook holds
documentation and declared structure, and nothing else. That is the honest state, and it will
stay that way until a host-backed harness exists.

What such a harness would have to probe, in the order that matters:

1. **The twelve `info_type` names against a live Excel**, each with an explicit `reference`,
   establishing the return kind and value for each — the baseline that does not exist yet.
2. **`"format"`'s code alphabet** across a set of standard number formats, in at least two
   locales, since the codes are the most build- and locale-sensitive thing the function returns.
   Any result here must carry its build and locale axes.
3. **The omitted-`reference` behaviour**, deliberately provoked: change a cell, recalculate,
   observe the answer move. Microsoft warns about it; nobody here has watched it happen.
4. **`"width"`'s two-item array** — what the second item is, and how it spills.
5. **`"filename"` on an unsaved workbook, on a workbook in a cloud location, and on a
   co-authored one**, which are three different host states that folklore treats as one.
6. **Staleness after a formatting change** — set `CELL("format", A1)`, change A1's format, and
   observe how many recalculations it takes for the answer to move. This is the volatility axis
   made visible and it is genuinely unknown here.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| host query | A call answered by Excel's workbook state rather than by computation |
| `info_type` | The text name selecting which property `CELL` returns |
| reference position | An argument that arrives as a live reference, not as a resolved value |
| `VolatileContextual` | Declared volatility: the answer can change without any input changing |
| last changed cell | The implicit subject when `reference` is omitted; application state |

## Sources

- Microsoft, "CELL function" —
  <https://support.microsoft.com/en-us/office/cell-function-51bd39a5-f338-4dbe-a33f-955d67c2b2cf>.
  Read for this page: the full `info_type` table verbatim, and the reference-argument caveat
  ("Although technically reference is optional, including it in your formula is encouraged…")
  together with the documented consequences of omitting it.
- Handbook, [the call pipeline](../model/03-call-pipeline.md) — `RefsVisibleInAdapter`,
  caller-aware functions, and host-side adaptation as an engine obligation.
- Handbook, [the execution context](../model/04-execution-context.md) — the calling-cell and
  workbook state a host query reads.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — reference resolution
  as an explicit step with its own failure modes.
- Handbook, [claim language and honesty](../model/06-claim-language.md) — why a build- and
  locale-scoped result cannot be published without its axes.
- `data/functions/FUNC.CELL.json` — identity (`xlfCell`, code 125), the published signature
  `CELL(info_type, [reference])`, arity 1–2, and the declared host-interaction, volatility and
  thread-safety axes, as projected at OxFunc `473efa3`.
