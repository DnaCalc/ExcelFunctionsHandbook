---
schema: efh.function-page/v1
function_id: FUNC.CUBEVALUE
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
family: null
role_in_family: null
---

`CUBEVALUE` is the only member of the cube family that returns a number, and the only one whose
answer is computed by the server rather than merely looked up. The reference engine (OxFunc)
carries it in its catalog and defers it, with the recorded reason **"Deferred cube-context
function."** There is no implementation module and no live registry entry: the projection is
catalog-only. The deferral is structural — the aggregation happens inside an OLAP engine at the
other end of a workbook connection, so a host-free reference implementation has nothing to
compute.

## What it computes

`CUBEVALUE` names a point (or a region) of a cube by intersecting a list of member expressions,
and asks the cube for the aggregated measure value at that intersection.

The mental model that makes the argument list intelligible: each `member_expression` is a
**slicer**. Every argument you supply constrains one hierarchy; every hierarchy you leave
unconstrained is aggregated over at its default (usually the "All" member). So
`CUBEVALUE(conn, "[Measures].[Sales]", "[Time].[2024]")` means *sales, in 2024, summed over
everything else* — the aggregation is not something `CUBEVALUE` performs, it is what the cube
already holds along the axes you did not pin.

Two consequences follow directly:

1. **Order does not carry meaning; hierarchy membership does.** The arguments are a set of
   constraints, not a positional coordinate vector. Two expressions drawn from the *same*
   hierarchy do not narrow — they contradict, and produce an empty intersection.
2. **A missing measure is not an error.** Microsoft documents that if no measure appears in any
   `member_expression`, the cube's default measure is used. A formula can therefore be
   perfectly valid, return a number, and be measuring something the author never named.

An argument may also be a `CUBESET` result rather than a single member. In that case the slice
is the whole set and the cube aggregates across it — which is how `CUBEVALUE` produces subtotals
over arbitrary member collections.

## Arguments

`CUBEVALUE(connection, [member_expression1], [member_expression2], …)`

- **`connection`** (required) — text; the name of a connection *stored in the workbook*. Not a
  connection string, not a server name. Because it is a lookup key, a wrong name fails as
  `#NAME?`.
- **`member_expression1`, `member_expression2`, …** (optional, repeating) — each is text
  holding an MDX expression evaluating to a member or a tuple, or a reference to a cell
  containing a `CUBESET` or other CUBE function.

The argument position most commonly misunderstood is any `member_expression` given as a **cell
reference**. Microsoft states the rule plainly: if the referenced cell contains a CUBE function,
`CUBEVALUE` uses that cell's MDX expression, *not* the value displayed in it. So a grid of
`CUBEMEMBER` headers works as an addressing layer, while a cell containing the plain text
`"Sales"` — which looks identical to a reader — does not.

Omitting *all* member expressions is legal and asks for the cube's default measure at its
default coordinates.

## Result and edge cases

The published result is normally a number. Two edges deserve naming:

- **Nulls arrive as empty text.** Microsoft documents that database null values are converted
  to zero-length strings, and explicitly recommends guarding dependent formulas with `IF` and
  `ISTEXT` so that arithmetic downstream does not fail. This is the single most consequential
  edge case on the page: a cell that *looks* blank is text, not a number, and it will turn
  `A1*2` into `#VALUE!`. The Handbook's coercion chapter explains why — text that does not read
  as a number is a named conversion failure, not a silent zero. See
  [coercion and lifting](../model/02-coercion-and-lifting.md).
- **`#GETTING_DATA` while the request is outstanding.** The cell temporarily displays
  `#GETTING_DATA…` before the data arrives. It is one of the extended error codes catalogued in
  [the value universe](../model/01-value-universe.md) — a transient placeholder, not a failure.

## Errors

As documented by Microsoft on the page linked below:

- **`#NAME?`** — the connection name is not a valid workbook connection; or the OLAP server is
  not running, not available, or returns an error message. As with the rest of the family, a
  naming failure and a connectivity failure share one code.
- **`#VALUE!`** — at least one element within the tuple is invalid.
- **`#N/A`** — the `member_expression` syntax is incorrect; the named member does not exist in
  the cube; the tuple is invalid because there is no intersection for the specified values
  (which Microsoft notes "can occur with multiple elements from the same hierarchy"); the set
  contains at least one member with a different dimension than the others; or the expression
  refers to a session-scoped calculated member or named set whose PivotTable has been deleted
  or converted to formulas.

None of these has been observed by the Handbook against a live Excel build. They are reported
as documented behaviour, with the source named.

## Relationships

`CUBEVALUE` is the consumer at the end of the family's pipeline; the other six functions
produce the coordinates it reads:

- `CUBEMEMBER` and `CUBERANKEDMEMBER` produce single members or tuples.
- `CUBESET` produces a set, which `CUBEVALUE` will aggregate across.
- `CUBEKPIMEMBER` produces a KPI component; Microsoft's documented way to use a KPI in a
  calculation is to pass `CUBEKPIMEMBER` as a `member_expression` argument to `CUBEVALUE`.
- `CUBESETCOUNT` and `CUBEMEMBERPROPERTY` are terminal — they return counts and properties
  and are not fed back into `CUBEVALUE`.

The function readers confuse with this one is `GETPIVOTDATA`, which extracts a value from a
PivotTable already rendered on a sheet. `CUBEVALUE` talks to the cube and needs no PivotTable.

## Notes for implementers

- There is no numeric kernel here at all. The interesting work is MDX construction, connection
  resolution, and asynchronous retrieval; the arithmetic belongs to the server.
- The null-to-empty-text conversion is a deliberate representational choice with a large blast
  radius. Any implementation that maps SQL/MDX null to numeric zero will silently produce
  different totals from Excel, and any implementation that maps it to a genuine blank will
  behave differently under `ISTEXT`. The documented behaviour is neither: it is a zero-length
  *string*.
- Argument-position identity (MDX expression versus displayed value for referenced cells) must
  be resolved before the request is built, which means the evaluator has to be able to see the
  formula behind a referenced cell, not only its value. That is unusual for a worksheet
  function and does not fit the ordinary values-only argument-preparation profile described in
  [the call pipeline](../model/03-call-pipeline.md).
- Retrieval is asynchronous with a visible intermediate state. Recalculation semantics — when
  a cell re-requests, what happens to dependents while `#GETTING_DATA` is showing — are host
  behaviour, not function semantics.

## What has not been checked

No Handbook vector suite exists for `CUBEVALUE`, and no Excel-comparison evidence is recorded.
Nobody has checked this function against Excel in the Handbook's record. Every behavioural
statement above is Microsoft's documented behaviour, cited as such.

A meaningful suite requires an oracle that includes a live cube, which the Handbook does not
currently pin. The probes that would settle the most, in order:

1. **The null representation.** A measure known to be null at a coordinate, then
   `ISTEXT`, `ISNUMBER`, `LEN`, `ISBLANK`, and `+0` applied to the result cell. This
   determines the exact kind of the "empty" result and is the behaviour most likely to bite
   real workbooks.
2. **Default-measure selection.** `CUBEVALUE(conn)` with no member expressions, on a cube with
   a known default measure, to confirm what is actually returned when nothing is named.
3. **Reference-versus-literal argument identity.** The same `member_expression` supplied (a) as
   a literal string, (b) as a cell containing that literal string, and (c) as a cell containing
   a `CUBEMEMBER` formula — three formulas that should not all agree.
4. **Same-hierarchy contradiction.** Two members of one hierarchy passed together, to confirm
   `#N/A` rather than a silent aggregation.
5. **Set-valued slicing.** A `CUBESET` result passed as a `member_expression`, checked against
   the sum of the individual members, to see whether the cube's aggregation and a naive sum
   agree — they will not, for non-additive measures such as distinct counts.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| reference engine: deferred | OxFunc intentionally defers the function; the recorded reason is "Deferred cube-context function." |
| catalog-only | The projection carries identity and documentation metadata but no implementation module or live registry entry |
| slicer | A member expression that constrains one hierarchy, leaving others aggregated |
| measure | The numeric quantity a cube stores and aggregates |
| default measure | The measure the cube uses when none is named in any argument |
| `#GETTING_DATA` | Extended-family transient placeholder while external data is retrieved |

## Sources

- Microsoft, "CUBEVALUE function" —
  <https://support.microsoft.com/en-us/office/cubevalue-function-8733da24-26d1-4e34-9b3a-84a8f00dcbe0>
  (syntax, the slicer framing, default-measure rule, the MDX-versus-displayed-value rule for
  cell references, the null-to-zero-length-string conversion and its `IF`/`ISTEXT` guard, and
  every error condition listed above).
- Microsoft, "CUBEKPIMEMBER function" —
  <https://support.microsoft.com/en-us/office/cubekpimember-function-744608bf-2c62-42cd-b67a-a56109f4b03b>
  (the documented route for using a KPI in a calculation via `CUBEVALUE`).
- Handbook `data/functions/FUNC.CUBEVALUE.json` — admission label and reason, category, the XLL
  identity `xlfCubevalue` (380), and the catalog-only metadata status.
- Handbook `data/presence/FUNC.CUBEVALUE.json` — no implementation module, no dispatch entry.
- Handbook `content/model/01-value-universe.md` and `content/model/02-coercion-and-lifting.md`
  — the extended error-code family and the text-that-is-not-a-number conversion rule.
