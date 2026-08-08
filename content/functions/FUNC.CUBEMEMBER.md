---
schema: efh.function-page/v1
function_id: FUNC.CUBEMEMBER
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

`CUBEMEMBER` is not a computation. It is a *question asked of a server*. Everything on this
page follows from that.

The reference engine (OxFunc) carries `CUBEMEMBER` in its catalog and defers it, with the
recorded reason **"Deferred cube-context function."** There is no implementation module and no
live registry entry behind the name — the projection records it as catalog-only. The reason is
structural rather than incidental: the function's result is defined by an OLAP cube reached
through a named workbook connection, so there is nothing a host-free reference implementation
could compute. A deferred entry here is an honest statement about where the semantics live,
not a gap waiting to be patched by better arithmetic.

## What it computes

Given the name of a workbook connection to an OLAP cube and a multidimensional-expression
(MDX) string, `CUBEMEMBER` resolves that expression against the cube and returns the member it
names — after first *validating* that the member exists. Microsoft's own framing for the whole
family is "use to validate that the member or tuple exists in the cube", and validation is the
operative verb: a `CUBEMEMBER` cell that resolves is a proof of existence, and one that does
not resolve is a diagnostic.

The subtlety that earns this page is that the resulting cell carries **two different things at
once**:

1. a **display value** — the member's caption from the cube, or the `caption` argument if one
   was supplied, and
2. an **identity** — the MDX expression itself.

Which of the two a consumer sees depends on the consumer. A human, a `LEN`, or a text
concatenation sees the caption. Another CUBE function that takes the cell as an argument sees
the MDX expression. Microsoft states this explicitly for `CUBEVALUE`: when a cell reference is
used for `member_expression` and that cell contains a CUBE function, the MDX expression of the
referenced cell is used, "not the value displayed in that referenced cell".

That is why the idiomatic cube report is built in two layers: a grid of `CUBEMEMBER` cells that
pin down coordinates, and a grid of `CUBEVALUE` cells that point at those coordinate cells to
fetch numbers. The `CUBEMEMBER` layer is the addressing layer.

`member_expression` may also denote a **tuple** — an intersection of members from distinct
hierarchies — supplied as a cell range or an array constant. A tuple is a single point in the
cube's coordinate space, not a set; the set-valued sibling is `CUBESET`.

## Arguments

`CUBEMEMBER(connection, member_expression, [caption])`

- **`connection`** (required) — text; the name of a connection stored *in the workbook*, not a
  connection string, not a server name, not a file path. This is the argument position most
  often misunderstood: it is a lookup key into the workbook's connection collection, which is
  why a mistyped or removed connection produces `#NAME?` rather than a connectivity error.
- **`member_expression`** (required) — text holding an MDX expression that evaluates to a
  *unique* member of the cube; or a tuple given as a cell range or array constant. Microsoft
  documents a 255-character limit on the string passed directly in the formula, and notes that
  a cell reference can carry up to the ordinary cell limit of 32,767 characters instead. That
  asymmetry between an inline literal and a referenced cell is a real trap in generated
  workbooks.
- **`caption`** (optional) — text displayed instead of the cube's own caption. When the result
  is a tuple, Microsoft documents that the caption used is the one belonging to the **last**
  member of the tuple. Omitting `caption` means "use whatever the cube says", which makes the
  displayed text a server- and language-pack-dependent value.

## Result and edge cases

The published result is text (the caption), carrying the MDX identity described above; the
Handbook's value-universe chapter would classify the identity-bearing cell as a value with a
payload beyond its core projection rather than as plain text. See
[the value universe](../model/01-value-universe.md) for the two-tier core-projection model.

While the request is outstanding the cell shows `#GETTING_DATA`. That is one of the extended
error codes in the value-universe chapter's registry — a transient placeholder, not a failure.
Whether it survives a boundary crossing into the C API or automation is a per-code,
per-version fact the Handbook does not currently pin.

Nothing in the shared coercion chapter governs this function usefully: there is no numeric
domain, no array lift worth describing, and the interesting behaviour lives entirely in
resolution against a remote server.

## Errors

As documented by Microsoft on the page linked below:

- **`#NAME?`** — the connection name is not a valid workbook connection; or the OLAP server is
  not running, not available, or returns an error. Note that both a *naming* failure and a
  *connectivity* failure land on the same code, which makes `#NAME?` ambiguous here in a way it
  is not elsewhere in Excel.
- **`#N/A`** — the `member_expression` syntax is incorrect; the member the MDX names does not
  exist in the cube; the tuple has no valid intersection (typically two elements drawn from the
  same hierarchy); or the expression refers to a session-scoped object — a calculated member or
  named set belonging to a PivotTable that has since been deleted or converted to formulas.
- **`#VALUE!`** — at least one element within a tuple is invalid; or the expression exceeds the
  documented 255-character inline limit.

The Handbook has not observed any of these against a live Excel build. They are reported here
as documented behaviour with the source named.

## Relationships

Seven functions form the cube family, and they divide cleanly by what they return:

| Function | Returns |
|---|---|
| `CUBEMEMBER` | one member or tuple — a coordinate |
| `CUBESET` | a set of members or tuples |
| `CUBESETCOUNT` | the cardinality of a `CUBESET` result |
| `CUBERANKEDMEMBER` | the nth member of a set — a coordinate drawn out of a set |
| `CUBEMEMBERPROPERTY` | a property value hanging off a member |
| `CUBEKPIMEMBER` | a KPI component, itself usable as a coordinate |
| `CUBEVALUE` | the aggregated number at a coordinate |

`CUBEMEMBER` and `CUBERANKEDMEMBER` are the two coordinate producers; `CUBEVALUE` is the only
consumer that yields a number.

Readers confuse `CUBEMEMBER` with `GETPIVOTDATA`. They are not siblings: `GETPIVOTDATA` reads a
PivotTable already present on a sheet, while `CUBEMEMBER` addresses the cube directly and needs
no PivotTable at all. They do meet at one edge — a `CUBEMEMBER` expression naming a
session-scoped calculated member created inside a PivotTable breaks when that PivotTable goes
away.

## Notes for implementers

- There is no kernel to port. Any implementation is an MDX parser plus a client for an
  analysis server; the "function" is a thin request wrapper. This is exactly why the reference
  engine defers it.
- The dual display/identity value is the load-bearing design decision. An implementation that
  returns plain caption text will look correct in isolation and then fail the moment a
  `CUBEVALUE` cell references it — the composition breaks, not the individual cell.
- Captions are localized by the server and its language packs. Two workbooks against the same
  cube on differently configured servers can display different text from identical formulas.
  A caption is therefore not a stable identifier and should never be used as a join key.
- The 255-versus-32,767 character asymmetry between inline literals and referenced cells is a
  documented behaviour that a generator emitting long MDX must respect.

## What has not been checked

No Handbook vector suite exists for `CUBEMEMBER`, and no Excel-comparison evidence is recorded
for it. Nobody has checked this function against Excel in the Handbook's record. Everything
above that describes behaviour is Microsoft's documented behaviour, cited as such.

Constructing a suite for this function is not a matter of choosing inputs; it requires pinning
an oracle that includes a cube. What would settle the open questions, roughly in order of
value:

1. **The identity/display split.** Enter `CUBEMEMBER` in `A1`, then `LEN(A1)`, `A1&""`,
   `ISTEXT(A1)`, and `CUBEVALUE(conn, A1)` and record all five. This is the only way to see
   whether the MDX identity is visible to anything other than a CUBE function, and it is the
   behaviour most likely to differ across Excel builds.
2. **The `#NAME?` collision.** Probe a valid connection with a dead server against an invalid
   connection name and confirm that both really produce `#NAME?`, since the documentation
   merges two very different failures onto one code.
3. **The 255-character boundary.** An inline `member_expression` of exactly 255 and of 256
   characters, and the same strings delivered by cell reference.
4. **Transient-state observability.** Whether `#GETTING_DATA` can be captured by a dependent
   formula at all, or whether recalculation ordering makes it unobservable from the grid.
5. **Tuple caption selection.** A three-member tuple with distinct captions, to confirm that
   the last member's caption is the one displayed.

Until an oracle with a live cube connection is pinned, this page states documentation and
structure only.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| reference engine: deferred | OxFunc intentionally defers the function; the recorded reason is "Deferred cube-context function." |
| catalog-only | The projection carries identity and documentation metadata but no implementation module or live registry entry |
| MDX | Multidimensional Expressions, the query language the cube evaluates |
| member | A single element of a cube hierarchy — one coordinate along one axis |
| tuple | An intersection of members from distinct hierarchies — one point in the cube |
| identity vs display | The cell's MDX expression (what CUBE functions consume) versus its caption (what a reader sees) |
| `#GETTING_DATA` | Extended-family transient placeholder while external data is retrieved |

## Sources

- Microsoft, "CUBEMEMBER function" —
  <https://support.microsoft.com/en-us/office/cubemember-function-0f6a15b9-2c18-4819-ae89-e1b5c8b398ad>
  (syntax, argument definitions, the 255-character limit, the caption-of-last-member rule, and
  every error condition listed above).
- Microsoft, "CUBEVALUE function" —
  <https://support.microsoft.com/en-us/office/cubevalue-function-8733da24-26d1-4e34-9b3a-84a8f00dcbe0>
  (the statement that a referenced CUBE-function cell contributes its MDX expression rather
  than its displayed value).
- Handbook `data/functions/FUNC.CUBEMEMBER.json` — admission label and reason, category, the
  XLL identity `xlfCubemember` (381), and the catalog-only metadata status.
- Handbook `data/presence/FUNC.CUBEMEMBER.json` — no implementation module, no dispatch entry,
  registered only in OxFunc's catalog seed.
- Handbook `content/model/01-value-universe.md` — the extended error-code family and the
  core-projection model used above.
