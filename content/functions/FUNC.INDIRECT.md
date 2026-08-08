---
schema: efh.function-page/v1
function_id: FUNC.INDIRECT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Volatility and the dependency graph
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
family: indirect
role_in_family: >-
  Converts reference text into a live reference at evaluation time; the catalog's text-to-
  reference bridge, and the reason a formula can point somewhere the dependency graph cannot
  see.
---

## What it computes

`INDIRECT(ref_text, [a1])` parses a string as a reference and returns that reference. It is the
inverse of writing a reference literally: where `=A1` names a cell at authoring time,
`=INDIRECT("A1")` names it at evaluation time, from data.

Everything interesting about this function follows from *when* the reference comes into
existence. A literal reference is part of the formula's structure: the engine can see it before
evaluation, adjust it when rows are inserted, and use it to order the calculation chain. A
reference built by `INDIRECT` exists only while the formula is being evaluated, and only as the
value of an expression the engine cannot inspect in advance.

The projection records the consequences as classification axes:
`fec_dependency_profile: CallerContext`, `host_interaction_class: WorkbookState`,
`thread_safety_class: HostSerialized`, and `volatility_class: VolatileContextual`. Each of those
is a cost, and together they are the reason `INDIRECT` should be a last resort rather than a
first tool.

The text is resolved against the calling context: a bare `"A1"` means `A1` on the calling
formula's sheet.

## Arguments

| Argument | Meaning |
|---|---|
| `ref_text` | Text naming a reference: a cell address, a range, a defined name, a structured reference, or a sheet- or workbook-qualified address. Required. |
| `a1` | Logical selecting the notation of `ref_text`. Microsoft documents `TRUE` or omitted as A1 style, `FALSE` as R1C1 style. |

`ref_text` is where every subtlety lives:

- **It is a reference expression in text, not just an address.** A range (`"A1:B10"`), a defined
  name (`"SalesData"`), and a sheet-qualified address (`"Sheet2!A1"`) are all admissible in
  principle. Which grammar `INDIRECT` accepts in full is broader than the documentation
  enumerates and is on the unchecked list below.
- **Sheet names needing quotes need them in your text.** `"'Q1 Sales'!A1"` — the apostrophes are
  part of the string you must build, not something `INDIRECT` adds.
- **External workbook references require the other workbook to be open.** Microsoft documents
  that if the source workbook is not open, `INDIRECT` returns `#REF!`. This is the single most
  common production failure of the function: a formula that worked while the source was open
  breaks silently once it is closed, and no amount of recalculation fixes it.
- **Microsoft further documents that external references are not supported in Excel for the
  web**, which makes `INDIRECT`-based cross-workbook formulas a platform-dependent construct.

The `a1` argument is the one people forget exists. `INDIRECT("R1C1")` with `a1` omitted asks
Excel to parse `R1C1` as an A1-style address — which is a *valid* A1-style address (column `R1C`
does not exist, but the parse attempt is the point). Notation mismatches here produce `#REF!`
rather than obviously wrong answers, which is at least loud.

## Volatility and the dependency graph

This section exists because these are the properties that decide whether you should use the
function at all.

**Volatility.** The projection records `volatility_class: VolatileContextual`. A volatile
function is re-evaluated on every recalculation, whether or not anything it depends on changed —
and, more expensively, everything downstream of it is re-evaluated too. A handful of `INDIRECT`
calls in a summary sheet is nothing; a column of them over ten thousand rows changes the
workbook's recalculation cost from proportional-to-what-changed to proportional-to-everything.

**Invisibility to the dependency graph.** The engine builds its calculation order from
references it can see in formulas. It cannot see through text. Two consequences:

1. **Restructuring does not update the text.** Insert a row above `A10` and every literal
   reference to `A10` becomes `A11`; `INDIRECT("A10")` still says `A10` and now points at
   different data. Nothing errors. This is the failure mode that makes `INDIRECT` genuinely
   dangerous in maintained workbooks — it is silent, and it is discovered by reconciliation
   rather than by a `#REF!`.
2. **Calculation order cannot be derived.** Because the engine does not know in advance what an
   `INDIRECT` call will read, it cannot order that call after its true precedents by inspection.
   Volatility is, in part, how the engine copes: recalculating always is a blunt substitute for
   knowing when to recalculate.

**Threading.** `thread_safety_class: HostSerialized` in the projection: the call must be
serialized against the host rather than run freely on a calculation thread. On a large sheet
that removes a source of parallelism.

**The upside, stated fairly.** `INDIRECT` does one thing nothing else does: it makes the
*target* of a reference into data. Sheet names in a column, a lookup table chosen by a
drop-down, a range whose extent is a user setting — these are genuinely dynamic and `INDIRECT`
expresses them directly. It is also the only way to write a reference that survives row
deletion deliberately, which is occasionally exactly what you want. The advice is not "never";
it is "know what you are buying".

## Result and edge cases

Return kind: `Reference`. Because it is a reference and not a value, `INDIRECT` composes with
range-consuming functions — `SUM(INDIRECT("A1:A10"))` scans a range — and with the range
operator.

- **Preparation.** Unlike most reference-producing functions, `INDIRECT` is prepared under
  `ValuesOnlyPreAdapter`: it takes text and produces a reference, rather than receiving one. It
  is a reference *source*, not a reference *inspector*.
- **The text is trimmed and must be non-empty**, per the reference implementation's parse; an
  empty or whitespace-only `ref_text` is an invalid reference.
- **`a1` given as a blank or omitted slot.** Missing and Empty are distinct at the call boundary
  ([value universe](../model/01-value-universe.md)), and the reference implementation treats
  them differently here — omitted selects A1 style, an explicitly empty argument does not. That
  is an implementation detail worth probing against Excel rather than assuming.
- **Defined names, structured references, spill anchors** (`"B1#"`) — each is a distinct
  reference shape and each is a separate question about what `INDIRECT`'s parser accepts.
- **Three-dimensional references** (`"Sheet1:Sheet3!A1"`) — likewise.
- **An array of `ref_text`** — whether `INDIRECT` lifts over an array of address strings is not
  established here. The projection records `lift_broadcast_profile: surface_native`, meaning any
  lifting is the function's own rather than the dispatch layer's.

## Errors

Microsoft's page documents:

- `#REF!` when `ref_text` is not a valid reference.
- `#REF!` when `ref_text` refers to another workbook that is not open.

`#REF!` is therefore the function's characteristic failure, and note what it does *not* cover:
text that parses fine but points at the wrong place because the sheet was restructured. That
case produces a number, not an error.

An error value supplied as `ref_text` propagates under the universal rule in
[coercion and lifting](../model/02-coercion-and-lifting.md) — the reference implementation
converts it to a coercion failure carrying the incoming code, which is the same discipline.

Whether a non-text `ref_text` (a number, a logical) is coerced to text and parsed, or rejected
outright, is not established here.

## Relationships

- **`ADDRESS`** is the natural producer of the text `INDIRECT` consumes. The pair
  `INDIRECT(ADDRESS(r, c))` is the textbook computed-reference idiom — and, in almost every
  case, `INDEX(range, r, c)` does the same job without volatility, without the text round trip,
  and while remaining visible to the dependency graph. That comparison is the practical bottom
  line of this page.
- **`OFFSET`** computes a reference by displacement rather than by text. It is also volatile,
  but it *is* visible to the dependency graph, so it sits between `INDEX` and `INDIRECT` on the
  cost scale.
- **`INDEX`'s reference form** is the non-volatile alternative for nearly every computed
  reference that does not need a *name* to come from data. See the `INDEX` page.
- **Defined names and `LET`** cover cases where the "dynamic" part is really parameterization
  rather than genuinely data-driven addressing.
- **`CHOOSE`** selects among ranges written in the formula. When the candidate ranges are known
  at authoring time — the usual case for a drop-down-driven lookup — `CHOOSE` replaces
  `INDIRECT` and keeps the references visible.
- **`FORMULATEXT`** goes the other way, exposing formula text as data.

## Notes for implementers

- **Parsing is the function.** `INDIRECT` is a reference parser wearing a function's clothes,
  and its accepted grammar is co-extensive with the host's reference syntax — including
  quoting rules, sheet-span syntax, structured-reference syntax, and the R1C1 alternative.
  Implementing it means either reusing the formula parser's reference production or accepting
  that the two will drift.
- **The A1/R1C1 flag changes the grammar, not the formatting.** Two parsers, or one
  parameterized parser; not a post-processing step.
- **Locale.** Reference syntax is locale-sensitive in places — the sheet separator, list
  separators inside union expressions, and the spelling of R1C1 in localized builds. Text built
  in one locale may not parse in another, which makes `INDIRECT` formulas a portability hazard
  in a way that literal references are not.
- **Caller context is required.** An unqualified address resolves against the calling sheet, so
  the function cannot be evaluated without knowing where it was called from. This is why the
  reference implementation classifies it `CallerContext` and why a host-free harness cannot
  exercise it at all.
- **Volatility must be declared, not inferred.** An engine that treats `INDIRECT` as pure will
  cache a result that is correct only until the workbook changes shape.

## What has not been checked

No Handbook vector suite exists for `INDIRECT`, and no Excel-comparison evidence record is
recorded for it. Nobody has checked this function's behaviour against Excel here — and note
that the reference implementation's own battery cannot call it without a host facility, so the
caller-context path is untested there too. This is a function whose semantics live almost
entirely in the host boundary, which makes it one of the harder pages in the catalog to move off
"documented only".

What to probe first:

1. **The accepted grammar of `ref_text`**, one probe per reference shape: single cell, range,
   whole column, defined name, sheet-qualified, quoted sheet name with a space, three-
   dimensional sheet span, structured reference, spill anchor, and an external workbook
   reference both open and closed. The documentation enumerates far less than the function
   accepts, and this list is the substance of what the page cannot currently state.
2. **`a1 = FALSE`** with each of an absolute R1C1 address and a relative bracketed one — the
   relative form is defined against the calling cell, so this also probes caller context.
3. **Omitted `a1` versus an explicitly empty `a1` slot** (`INDIRECT("A1",)`). The reference
   implementation distinguishes them; whether Excel does is a clean Missing-versus-Empty test.
4. **Non-text `ref_text`**: a number, a logical, an error, an array of address strings.
5. **The restructuring failure, observed rather than described**: build
   `INDIRECT("A10")` and a literal `A10`, insert a row, and record that the two now disagree.
   This is documentation-by-demonstration for the page's central warning.
6. **Volatility, observed**: whether a workbook containing `INDIRECT` recalculates dependents on
   an unrelated edit. Microsoft's recalculation documentation is the place to check the claim;
   the observation is what settles it.

Item 1 is where nearly all the missing knowledge on this page is concentrated.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| evaluation-time reference | A reference that exists only while the formula runs; `eval_time_deref` in the shared model |
| volatile | Re-evaluated on every recalculation regardless of input changes |
| dependency graph | The engine's map of which formulas depend on which cells, built from visible references |
| caller context | The calling cell and sheet, against which unqualified text resolves |
| R1C1 style | The alternative reference notation selected by `a1` = FALSE |

## Sources

- Microsoft, INDIRECT function —
  <https://support.microsoft.com/en-us/office/indirect-function-474b3a3a-8a26-4f44-b491-92b6306fa261>
  (the `ref_text` and `a1` arguments, the A1/R1C1 selection, the `#REF!` conditions for invalid
  references and closed external workbooks, and the note that external references are not
  supported in Excel for the web).
- Handbook `content/model/01-value-universe.md` (reference kind and shapes; Missing versus
  Empty).
- Handbook `content/model/02-coercion-and-lifting.md` (`eval_time_deref`; error propagation;
  reference resolution as an explicit step).
- Handbook `content/model/03-call-pipeline.md` (caller-aware functions; reference-producing
  functions).
- Handbook `data/functions/FUNC.INDIRECT.json` and `data/presence/FUNC.INDIRECT.json` (arity,
  `VolatileContextual`, `CallerContext`, `HostSerialized`, implementing module).
- OxFunc `crates/oxfunc_core/src/functions/indirect.rs` at commit `473efa3` — read for the
  shape of its `ref_text` trimming and empty-text rejection, its error-propagation path, and its
  distinct handling of an omitted versus empty `a1`. Read as reference-implementation structure,
  not as evidence about Excel.
