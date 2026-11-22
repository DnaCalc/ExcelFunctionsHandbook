---
schema: efh.function-page/v1
function_id: FUNC.CUBEKPIMEMBER
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
family: null
role_in_family: null
---

A KPI in an OLAP cube is not a number. It is a small bundle of related expressions — an actual
value, a target, a status, a trend, a weight, and a time context — defined once by whoever
built the cube. `CUBEKPIMEMBER` selects **one component of that bundle** and hands back a member
expression naming it.

The reference engine (OxFunc) carries the function in its catalog and defers it, with the
recorded reason **"Deferred cube-context function."** There is no implementation module and no
live registry entry — catalog-only. The deferral is structural: KPI definitions live in the
cube.

## What it computes

`CUBEKPIMEMBER(connection, kpi_name, kpi_property, [caption])` resolves the named KPI against
the cube and returns the member corresponding to the requested component.

The crucial point — and the one that makes the function make sense — is that **the result is a
coordinate, not a value**. Microsoft states the usage rule directly: "to use the KPI in a
calculation, specify the CUBEKPIMEMBER function as a member_expression argument in the CUBEVALUE
function." A bare `CUBEKPIMEMBER` cell displays the KPI's name; it does not display the KPI's
number. Getting the number takes two functions.

That is the design the whole family shares (`CUBEMEMBER` addresses, `CUBEVALUE` fetches), but
it surprises people here more than anywhere else, because a "KPI" sounds like a value by name.

The six components Microsoft documents:

| Integer | Constant | What it names |
|---|---|---|
| 1 | `KPIValue` | the actual value |
| 2 | `KPIGoal` | a target value |
| 3 | `KPIStatus` | the state of the KPI at a specific moment in time |
| 4 | `KPITrend` | a measure of the value over time |
| 5 | `KPIWeight` | a relative importance assigned to the KPI |
| 6 | `KPICurrentTimeMember` | a temporal context for the KPI |

`KPIStatus` and `KPITrend` are the two whose *scale* is a cube convention rather than an Excel
one: they are typically normalized indicator values, and a workbook that treats them as
currency or as raw counts will render nonsense. `KPICurrentTimeMember` is different in kind
from the other five — it names a time member rather than a numeric component, which is what
makes "compare this KPI against the same period last year" expressible.

## Arguments

- **`connection`** (required) — text; the name of a connection stored in the workbook, used as
  a lookup key rather than as a connection string.
- **`kpi_name`** (required) — text; the KPI's name as defined in the cube. Cube metadata, not a
  label of the workbook's choosing.
- **`kpi_property`** (required) — the component selector from the table above. Microsoft
  documents both integers and enumerated-constant names. Note that this argument is *required*,
  which means there is no "just give me the KPI" call — you must always choose a component.
- **`caption`** (optional) — alternative text displayed instead of `kpi_name` and
  `kpi_property`.

One display detail Microsoft records and which is easy to trip over: if `kpi_property` is
`KPIValue`, **only `kpi_name` is displayed** in the cell. So the `KPIValue` cell and a
`KPIGoal` cell for the same KPI look different by default, and the difference is a display
convention rather than a difference in what the cells are.

## Result and edge cases

The cell displays text — the KPI name, or the name plus component, or the supplied `caption` —
and carries the underlying member expression as its identity for consumption by `CUBEVALUE`.
The identity-versus-display split is the same one described on [`CUBEMEMBER`](FUNC.CUBEMEMBER.md).

While the request is outstanding the cell displays `#GETTING_DATA…` — a transient
extended-family placeholder catalogued in [the value universe](../model/01-value-universe.md),
not a failure.

The Handbook cannot say from documentation whether `kpi_property` accepts the enumerated
constant *names* as text in a formula or only the integers; Microsoft's table lists both
columns without saying which is accepted where. That is a genuine ambiguity, not a stylistic
one, and it is listed as a probe below.

## Errors

As documented by Microsoft on the page linked below:

- **`#NAME?`** — the connection name is not a valid workbook connection; or the OLAP server is
  not running, not available, or returns an error message.
- **`#N/A`** — `kpi_name` or `kpi_property` is invalid; or the expression refers to a
  session-scoped object whose PivotTable has been deleted or converted to formulas.

Note that an invalid *component selector* and an invalid *KPI name* collapse onto the same
code, so `#N/A` here does not tell a workbook author which of the two arguments is wrong.

## Relationships

- **`CUBEVALUE`** — the necessary partner. `CUBEKPIMEMBER` produces the coordinate; `CUBEVALUE`
  turns it into a number. Neither is useful alone for KPI reporting.
- **`CUBEMEMBER`** — the general coordinate producer. `CUBEKPIMEMBER` is best read as a
  specialized `CUBEMEMBER` that knows the MDX shape of KPI components so the author does not
  have to write `KPIGoal("…")` by hand.
- **`CUBEMEMBERPROPERTY`** — also returns "something attached to a member", but terminally: a
  property is a value, a KPI component is a coordinate. The two are easy to confuse and behave
  oppositely downstream.

Outside the family, the confusion is with PivotTable KPI display fields and with Power BI's KPI
visual. Those render KPIs; this function addresses them.

## Notes for implementers

- There is no kernel; the function is an MDX-shape helper over a cube metadata lookup.
- Returning a *value* rather than a coordinate would be the natural, wrong simplification. It
  would make single cells look right and break every `CUBEVALUE` that references them.
- The `KPIValue`-only-shows-`kpi_name` display rule means the displayed text is not a function
  of the arguments in a uniform way. An implementation reproducing display text needs that
  special case explicitly.
- Whether the integer selector and the constant name are interchangeable is exactly the kind of
  admission detail that must be pinned by observation before an implementation can claim
  compatibility.

## What has not been checked

No Handbook vector suite exists for `CUBEKPIMEMBER`, and no Excel-comparison evidence is
recorded. Nobody has checked this function against Excel in the Handbook's record. Everything
behavioural above is Microsoft's documented behaviour or explicitly flagged as unknown.

Given an oracle with a live cube carrying at least one defined KPI, the probes that would
settle the most:

1. **Selector form.** `kpi_property` supplied as `1`, as `"KPIValue"`, as `"kpivalue"`, and as
   `TRUE`. This decides whether the enumerated names are real formula input or documentation
   shorthand, and it is the most basic unanswered question about the function.
2. **Out-of-range selectors.** `0` and `7`, plus a fractional value such as `1.5`, to see
   whether the argument truncates or errors.
3. **The `KPIValue` display rule.** The same KPI at components 1 and 2, with and without
   `caption`, recording the exact displayed text in each case.
4. **Round trip through `CUBEVALUE`.** `CUBEVALUE(conn, kpiCell)` for each of the six
   components, to confirm which components actually yield numbers and what
   `KPICurrentTimeMember` does when asked for a value.
5. **`KPIStatus` and `KPITrend` scales.** Recording the actual returned values for a known KPI,
   since their range is a cube convention the Handbook cannot state a priori.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `CUBEKPIMEMBER` that an implementer matching Excel, or a reader
explaining an unexpected Excel result, needs to know. Each note gives exact binary64 inputs,
Excel's answer and where it was recorded; "measured" means the statement is read off recorded
Excel answers, "inferred" marks what goes beyond them. Where older text on this page says a
behaviour was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

### FN-CUBEKPIMEMBER-001: CUBEKPIMEMBER validates scalar errors and numeric kpi_property text before connection lookup, returning #VALUE! for invalid text and #N/A on models without KPIs

*Quirk; measured; recorded 2026-11-22.*

In Excel 16.0.20527, CUBEKPIMEMBER(connection, kpi_name, kpi_property, [caption]) checks scalar
errors on connection, kpi_name, and kpi_property first, then validates that kpi_property is
numeric (returning #VALUE! for non-numeric text such as "1,234" or "<>" before checking the
connection), then checks a scalar error on caption, and returns #N/A when the connection is
unknown or the workbook data model defines no KPI. All 4,294 full-bank CUBEKPIMEMBER rows
match.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=CUBEKPIMEMBER("ThisWorkbookDataModel","KPI1",1)` | as typed | `#N/A` | Excel 16.0.20527 (64-bit), Windows, workbook data model, en-ZA |
| `CUBEKPIMEMBER(1e100, 17, "1,234", 1e100)` | connection `45000.0` (`0x40e5f90000000000`); kpi_name `17.000000000000004` (`0x4031000000000001`); kpi_property `1,234`; caption `1e+100` (`0x54b249ad2594c37d`) (cells) | `#VALUE!` | Excel 16.0 build 20527, Windows, workbook compatibility 2, en-ZA; bitness and CPU not recorded |
| `CUBEKPIMEMBER(2, #VALUE!, 100000, 1e-15)` | connection `2.0` (`0x4000000000000000`); kpi_name `#VALUE!`; kpi_property `45000.0` (`0x40e5f90000000000`); caption `402.65475791367` (`0x40792a79e36f2028`) (cells) | `#VALUE!` | Excel 16.0 build 20527, Windows, workbook compatibility 2, en-ZA; bitness and CPU not recorded |

Basis (measured): Measured in DnaCalc crates/calc-host/tests/fixtures/pivot_cube/oracle.xlsx
(A120-A121, Excel 16.0.20527) and CalcFunc-Evidence admit2-answers-cubekpimember-enrich.json
and special.json (Excel 16.0 build 20527); 4,294/4,294 full-bank rows match.

Evidence:

- DnaCalc `crates/calc-func/src/catalogue/cube_functions.rs`, cubekpimember; early_numeric_text; early_final_error at `2e9be2b5b7` — pre-connection validation order and KPI refusal
- DnaCalc `crates/calc-func/witness-status.tsv`, CUBEKPIMEMBER at `2e9be2b5b7` — 4,294/4,294 full-bank matches
- DnaCalc `crates/calc-host/tests/fixtures/pivot_cube/oracle.xlsx`, xl/worksheets/sheet1.xml A120 at `2e9be2b5b7` (readings)
- CalcFunc-Evidence `readings/excel-20527/admit2-answers-cubekpimember-enrich.json`, admit2-enrich-CUBEKPIMEMBER-0017 at `4ef2c244d7` (readings)
- CalcFunc-Evidence `readings/excel-20527/admit2-answers-cubekpimember-special.json`, admit2-special-CUBEKPIMEMBER-0003 at `4ef2c244d7` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| reference engine: deferred | OxFunc intentionally defers the function; the recorded reason is "Deferred cube-context function." |
| catalog-only | The projection carries identity and documentation metadata but no implementation module or live registry entry |
| KPI | A cube-defined bundle of related expressions: value, goal, status, trend, weight, time member |
| component / `kpi_property` | The selected element of that bundle |
| coordinate, not value | The result names a place in the cube; `CUBEVALUE` is needed to read a number there |
| `#GETTING_DATA` | Extended-family transient placeholder while external data is retrieved |

## Sources

- Microsoft, "CUBEKPIMEMBER function" —
  <https://support.microsoft.com/en-us/office/cubekpimember-function-744608bf-2c62-42cd-b67a-a56109f4b03b>
  (syntax, all four arguments, the complete six-row `kpi_property` table with its integers and
  enumerated constants, the `KPIValue` display rule, the instruction to use the result as a
  `CUBEVALUE` argument, the `#GETTING_DATA` remark, and the `#NAME?` and `#N/A` conditions).
- Microsoft, "CUBEVALUE function" —
  <https://support.microsoft.com/en-us/office/cubevalue-function-8733da24-26d1-4e34-9b3a-84a8f00dcbe0>
  (the consumer side of the two-function pattern).
- Handbook `data/functions/FUNC.CUBEKPIMEMBER.json` — admission label and reason, category, the
  XLL identity `xlfCubekpimember` (477), and the catalog-only metadata status.
- Handbook `data/presence/FUNC.CUBEKPIMEMBER.json` — no implementation module, no dispatch
  entry.
- Handbook `content/model/01-value-universe.md` — the extended error-code family.
