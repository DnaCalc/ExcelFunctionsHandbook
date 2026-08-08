---
schema: efh.function-page/v1
function_id: FUNC.ISFORMULA
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
family: misc_switch_info_family
role_in_family: The host-backed predicate of the module — it asks the workbook a question no
  value can answer.
---

## What it computes

`ISFORMULA(reference)` returns `TRUE` if the referenced cell contains a formula, and `FALSE` if
it does not.

Microsoft's description, verbatim: "Checks whether there is a reference to a cell that contains
a formula, and returns TRUE or FALSE."

The important word is **contains**. `ISFORMULA` asks about the cell's *content* — is there a
formula stored there — not about the cell's value. A cell holding `=1+1` and a cell holding the
literal `2` display the same thing, produce the same value everywhere downstream, and are
distinguished by exactly one worksheet function: this one.

That makes `ISFORMULA` structurally different from every other predicate in the Information
category. `ISNUMBER`, `ISTEXT`, `ISERROR` and the rest classify a *value*, and a value is
self-describing. Whether a cell holds a formula is a fact about the workbook's storage, not
about any value, so `ISFORMULA` must be **reference-aware** (`RefsVisibleInAdapter`) and must
**query the host** (`HostInteractionClass::WorkbookState`). The Handbook's reference engine
refuses every one of its battery inputs with `cannot-call: requires-host-facility: composite` —
the correct answer for an engine with no workbook behind it, not a coverage gap.

Microsoft's `TYPE` page states the complementary limitation from the other side: "You cannot use
TYPE to determine whether a cell contains a formula." `ISFORMULA`, added in Excel 2013, is the
function that filled that hole.

## Arguments

`reference` — required, exactly one. The published signature is `ISFORMULA(reference)`.

Microsoft's argument description, verbatim: "Reference is a reference to the cell you want to
test. Reference can be a cell reference, a formula, or a name that refers to a cell."

Two things that sentence settles, and one it does not:

- **A defined name is admissible** provided it resolves to a reference. A name that refers to a
  constant or to a formula expression is not a reference and is an error — see "Errors".
- **"A formula" in that list means an expression that *produces* a reference**, such as
  `INDIRECT("A1")` or `OFFSET(A1,0,0)`. `ISFORMULA(INDIRECT("A1"))` asks whether `A1` holds a
  formula, not whether `INDIRECT(...)` is one.
- **What it does not settle** is the multi-cell case. `ISFORMULA(A1:A10)` is not addressed by the
  documentation, and whether it spills a column of booleans on a modern build or intersects to a
  single answer is not established here.

The argument is a **reference position, not a value position**. Handing `ISFORMULA` a computed
value is a failure, not a coercion, and that is exactly what the documented `#VALUE!` condition
covers.

## Result and edge cases

Returns a `Logical`.

- **A cell containing a constant is `FALSE`** — including a constant that was typed as the
  result of a formula and then pasted as values, which is the workflow this function exists to
  audit.
- **An empty cell is `FALSE`.** It contains no formula.
- **A cell whose formula evaluates to an error is still `TRUE`.** The formula is there; its
  result is irrelevant. This is the property that makes `ISFORMULA` useful for auditing a broken
  sheet — `ISERROR` finds the symptom, `ISFORMULA` finds whether there is anything to fix.
- **A cell whose formula returns the empty string is `TRUE`**, while `ISBLANK` on it is `FALSE`
  and it *looks* empty. The three-way disagreement between what the eye sees, what `ISBLANK`
  says and what `ISFORMULA` says is the standard way to find `=""` helper columns.
- **A spilled cell.** The anchor of a dynamic array holds the formula; the spilled-into cells do
  not hold formulas of their own. What `ISFORMULA` reports for a spilled-into cell is **not
  established here**, and it is the single most interesting open question about this function on
  a modern build.
- **Volatility.** The function is declared `NonVolatile`, unlike `CELL` and `INFO` — editing a
  cell's *content* is a change Excel's dependency graph tracks, so `ISFORMULA` does not need to
  recalculate on a timer the way a formatting query does.

## Errors

- **`#VALUE!`** — documented, verbatim: "If reference is not a valid data type, such as a
  defined name that is not a reference, ISFORMULA returns the #VALUE! error value." This covers
  a literal, a computed value, and a defined name that resolves to a constant or an expression.
- **`#NAME?`** would arise before `ISFORMULA` runs, from an unresolvable name, in reference
  resolution ([coercion and lifting](../model/02-coercion-and-lifting.md)).
- **`#REF!`** likewise, from a reference whose target has been deleted.
- **Arity** — zero arguments or two. Expected to be refused at formula entry rather than
  evaluated ([the call pipeline](../model/03-call-pipeline.md)).

Note that `ISFORMULA` is one of the few IS-named functions with a *documented* error return.
The classifiers have none; `ISFORMULA` has one because its argument can genuinely be of the
wrong kind.

## Relationships

- **`FORMULATEXT(reference)`** returns the formula's text where `ISFORMULA` returns a boolean.
  They arrived together in Excel 2013 and are usually used together:
  `IF(ISFORMULA(A1), FORMULATEXT(A1), A1)` renders a sheet's formulas alongside its constants.
  `FORMULATEXT` returns `#N/A` where `ISFORMULA` returns `FALSE`, which is why the guard is
  worth writing.
- **`TYPE`** explicitly cannot answer this question; Microsoft says so on its own page.
- **`CELL("contents", ref)`** returns the value, "not a formula" in Microsoft's own words — the
  same limitation stated a third time.
- **`ISREF`** tests whether an expression *is* a reference; `ISFORMULA` tests what is stored at
  one. Both are reference-aware and they are easy to confuse.
- **`ISBLANK`** is the companion in the `=""` diagnosis described above.
- **`SHEET`, `SHEETS`, `CELL`, `INFO`** are the other host-backed members of the Information
  category.

## Notes for implementers

1. **`ISFORMULA` cannot be answered from values.** It requires a workbook that stores cell
   *content* distinctly from cell *value*. An engine whose model is "cells hold values" cannot
   implement it, and should refuse rather than guess.
2. **The reference must survive preparation unresolved.** Dereferencing loses the subject
   entirely: the value at the cell says nothing about whether a formula produced it.
3. **The documented `#VALUE!` is a real, reachable path**, not a fallback. It fires for a
   defined name that is not a reference, which is a case an implementation must distinguish from
   an unresolvable name (`#NAME?`).
4. **Decide the multi-cell and spill cases deliberately** and record them as decisions awaiting
   evidence; the documentation covers neither.

## What has not been checked

No Handbook vector suite exists for `ISFORMULA`; `vectors/` publishes nothing for this function.
No Excel-comparison evidence record names `ISFORMULA` as a subject — **nobody has checked this
function's answers against Excel inside the Handbook's record.**

And, as with `CELL` and `INFO`, there are **no implementation-side answers to compare against
either**: the reference engine refuses every battery input for want of a host facility. What
this page holds is Microsoft's documentation and the declared structure.

What a host-backed harness would have to establish:

1. **Spilled cells.** Put a dynamic array in `A1`, then read `ISFORMULA(A1)` and `ISFORMULA(A2)`
   where `A2` was spilled into. The anchor holds the formula; the spill target arguably does not.
   Whichever way Excel answers, it is a fact about how dynamic arrays are stored, and nobody has
   written it down here.
2. **A multi-cell reference.** `ISFORMULA(A1:A10)` under dynamic arrays and under implicit
   intersection — does it spill ten booleans, intersect to one, or error?
3. **The three defined-name cases** — a name for a range, a name for a constant, a name for a
   formula expression — to pin the documented `#VALUE!` boundary against `#NAME?`.
4. **Array formulas entered with Ctrl+Shift+Enter**, whose cells all hold the same formula, in
   contrast with the spill case above.
5. **A cell inside a table with a calculated column**, where Excel manages the formula on the
   user's behalf.
6. **A cell containing a defined-name-only formula such as `=MyConstant`** — a formula that does
   nothing, but is one.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| cell content versus cell value | What is stored versus what is displayed; only `ISFORMULA` sees the difference |
| reference position | An argument that arrives as a live reference, not as a resolved value |
| host query | A call answered by workbook state rather than by computation |
| spill anchor | The cell holding a dynamic-array formula, as distinct from the cells spilled into |

## Sources

- Microsoft, "ISFORMULA function" —
  <https://support.microsoft.com/en-us/office/isformula-function-e4d1355f-7121-4ef2-801e-3839bfd6b1e5>.
  Read for this page: the description, the syntax, the `reference` argument description verbatim,
  and the `#VALUE!` remark verbatim.
- Microsoft, "TYPE function" —
  <https://support.microsoft.com/en-us/office/type-function-45b4e688-4bc3-48b3-a105-ffa892995899>.
  Read for the remark that `TYPE` cannot determine whether a cell contains a formula.
- Microsoft, "CELL function" —
  <https://support.microsoft.com/en-us/office/cell-function-51bd39a5-f338-4dbe-a33f-955d67c2b2cf>.
  Read for the `"contents"` row, "Value of the upper-left cell in reference; not a formula."
- Handbook, [the call pipeline](../model/03-call-pipeline.md) — `RefsVisibleInAdapter`,
  host-side adaptation, and the admission boundary.
- Handbook, [the execution context](../model/04-execution-context.md) — the workbook state a
  host query reads.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — reference resolution
  and its failure modes.
- `data/functions/FUNC.ISFORMULA.json` — identity (`xlfIsformula`, code 589), the published
  signature `ISFORMULA(reference)`, arity 1–1, and the declared host-interaction and
  argument-preparation axes, as projected at OxFunc `473efa3`.
