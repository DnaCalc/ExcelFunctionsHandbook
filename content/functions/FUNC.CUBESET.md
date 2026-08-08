---
schema: efh.function-page/v1
function_id: FUNC.CUBESET
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

`CUBESET` is the family's set constructor. It is also the one cube function that asks the
server to *do work and remember it*: Microsoft's description is that it "defines a calculated
set of members or tuples by sending a set expression to the cube on the server, which creates
the set, and then returns that set to Microsoft Excel."

The reference engine (OxFunc) carries `CUBESET` in its catalog and defers it, with the recorded
reason **"Deferred cube-context function."** There is no implementation module and no live
registry entry — catalog-only. The deferral is structural: the set is constructed inside an
OLAP engine reached through a workbook connection.

## What it computes

`CUBESET` evaluates an MDX set expression against the cube and returns a handle to the
resulting set, held in one cell. What lands in the cell is not the members themselves — it does
not spill, and it is not an array. It is a single cell whose display value is a caption and
whose identity is the set, consumable by three other functions:

- `CUBESETCOUNT` asks it how many items it holds,
- `CUBERANKEDMEMBER` pulls out the nth item,
- `CUBEVALUE` slices by the whole set at once, letting the cube aggregate across it.

Optionally the server is also asked to **sort** the set before returning it, and the sort is
part of the set's identity — `CUBERANKEDMEMBER` indexes into the sorted order. This is why
`CUBESET`'s sort arguments matter far more than a presentation setting would: they determine
what "the top performer" means.

The sort orders Microsoft documents:

| Integer | Constant | Behaviour | `sort_by` |
|---|---|---|---|
| 0 | `SortNone` | leaves the set in existing order (the default) | ignored |
| 1 | `SortAscending` | ascending by `sort_by` | required |
| 2 | `SortDescending` | descending by `sort_by` | required |
| 3 | `SortAlphaAscending` | alpha ascending | ignored |
| 4 | `SortAlphaDescending` | alpha descending | ignored |
| 5 | `SortNaturalAscending` | natural ascending | ignored |
| 6 | `SortNaturalDescending` | natural descending | ignored |

Three sort *families* hide in that table, and conflating them is the usual source of surprise.
Orders 1 and 2 sort by a **measure** (`sort_by` names it). Orders 3 and 4 sort by **caption
text**. Orders 5 and 6 sort in the cube's **natural** order — the hierarchy's own declared
member order, which is what makes months come out January-first rather than April-first.

## Arguments

`CUBESET(connection, set_expression, [caption], [sort_order], [sort_by])`

- **`connection`** (required) — text; the name of a connection stored in the workbook. A lookup
  key, not a connection string.
- **`set_expression`** (required) — text holding an MDX set expression, *or* a reference to an
  Excel range containing members, tuples or sets. Microsoft documents a **255-character limit**
  on the expression, and states that a cell reference may instead carry up to 32,767
  characters. Generated MDX crosses 255 characters easily, so this asymmetry is a practical
  constraint on how such workbooks must be built, not a footnote.
- **`caption`** (optional) — text displayed in the cell in place of the cube's caption.
- **`sort_order`** (optional) — one of the integers above; default 0.
- **`sort_by`** (optional) — text naming the value to sort by. **Required** when `sort_order`
  is 1 or 2, and ignored otherwise. This is the argument position most often gotten wrong: it
  is not a column, a member, or a cell reference to sorted data — it is a value expression the
  server evaluates per member.

## Result and edge cases

One cell, one set. Not a spilled array, and not usable directly in ordinary array formulas.
Everything downstream goes through `CUBESETCOUNT`, `CUBERANKEDMEMBER`, or `CUBEVALUE`.

While the request is outstanding the cell displays `#GETTING_DATA…` — an extended-family
transient placeholder, catalogued in [the value universe](../model/01-value-universe.md), not a
failure.

An empty result set is a state worth naming and one the Handbook cannot describe from
documentation: a set expression that legitimately matches nothing is different from a set
expression that is malformed, and the documentation does not say which of the two outcomes an
empty set produces. That question is listed below.

## Errors

As documented by Microsoft on the page linked below:

- **`#NAME?`** — the connection name is not a valid workbook connection; or the OLAP server is
  not running, not available, or returns an error message.
- **`#N/A`** — the `set_expression` syntax is incorrect; the set contains at least one member
  with a different dimension from the others; or the expression refers to a session-scoped
  calculated member or named set whose PivotTable has been deleted or converted to formulas.
- **`#VALUE!`** — `set_expression` is longer than 255 characters.

The Handbook has not observed any of these against a live Excel build; they are documented
behaviour with the source named.

## Relationships

`CUBESET` sits between the coordinate producers and the value consumer:

- `CUBEMEMBER` is its scalar counterpart — one member instead of a set. The two are the
  family's constructors.
- `CUBESETCOUNT` and `CUBERANKEDMEMBER` exist *only* to consume `CUBESET` results; neither has
  any other meaningful input.
- `CUBEVALUE` accepts a `CUBESET` result as a `member_expression`, which is how a workbook gets
  a subtotal over an arbitrary member collection.

Outside the family, the natural comparison is a PivotTable filter or a `FILTER` over a table:
both express "the subset I care about". The difference is that `CUBESET` pushes the definition
to the server and keeps the members on the server, which is what makes it viable for
million-member dimensions.

## Notes for implementers

- The result is a handle, not data. An implementation that materializes members into the cell,
  or spills them, is a different function — `CUBERANKEDMEMBER` would then have nothing to do.
- Sort order is semantic, not cosmetic, because `CUBERANKEDMEMBER` indexes the sorted set.
  Getting sort stability wrong changes which member "rank 1" names, silently.
- The three sort families (measure, alphabetic, natural) require three different server
  capabilities. Natural order in particular cannot be reconstructed client-side from captions —
  it is a property of the dimension definition.
- The 255-character limit applies to the expression as passed, with cell references as the
  documented escape hatch. Any code generator emitting MDX must plan for that.

## What has not been checked

No Handbook vector suite exists for `CUBESET`, and no Excel-comparison evidence is recorded.
Nobody has checked this function against Excel in the Handbook's record. Everything
behavioural above is Microsoft's documented behaviour, cited as such.

The probes that would settle the most, given an oracle with a live cube:

1. **The empty set.** A syntactically valid set expression matching no members — does the cell
   hold an empty set (with `CUBESETCOUNT` returning 0) or an error? The documentation does not
   say, and the two answers imply different downstream formula patterns.
2. **`sort_by` when it is ignored.** Supply `sort_by` with `sort_order` 3 or 0. "Ignored" could
   mean silently dropped or could mean validated-then-dropped; only observation distinguishes
   them.
3. **`sort_by` omitted when required.** `sort_order` 1 with no `sort_by` — which error, or
   which fallback?
4. **The 255-character boundary.** Expressions of exactly 255 and 256 characters inline, and
   the same delivered by cell reference.
5. **Sort stability under ties.** Two members with equal `sort_by` values, checked through
   `CUBERANKEDMEMBER` across repeated recalculations, to see whether rank assignment is stable.
6. **What the cell reports to ordinary functions.** `ISTEXT`, `LEN`, and `ISREF` on a `CUBESET`
   cell, to pin the core projection of the handle value.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| reference engine: deferred | OxFunc intentionally defers the function; the recorded reason is "Deferred cube-context function." |
| catalog-only | The projection carries identity and documentation metadata but no implementation module or live registry entry |
| set expression | An MDX expression evaluating to a collection of members or tuples |
| handle | A single-cell value standing for a server-side set, consumed by other CUBE functions |
| natural order | The member order declared by the cube's dimension, as opposed to alphabetic or measure order |
| `#GETTING_DATA` | Extended-family transient placeholder while external data is retrieved |

## Sources

- Microsoft, "CUBESET function" —
  <https://support.microsoft.com/en-us/office/cubeset-function-5b2146bd-62d6-4d04-9d8f-670e993ee1d9>
  (syntax, all five arguments, the complete `sort_order` table with its enumerated constants
  and `sort_by` requirements, the 255-versus-32,767 character rule, and every error condition
  listed above).
- Handbook `data/functions/FUNC.CUBESET.json` — admission label and reason, category, the XLL
  identity `xlfCubeset` (478), and the catalog-only metadata status.
- Handbook `data/presence/FUNC.CUBESET.json` — no implementation module, no dispatch entry.
- Handbook `content/model/01-value-universe.md` — the extended error-code family.
