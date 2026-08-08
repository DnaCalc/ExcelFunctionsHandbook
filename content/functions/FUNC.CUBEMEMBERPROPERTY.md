---
schema: efh.function-page/v1
function_id: FUNC.CUBEMEMBERPROPERTY
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

`CUBEMEMBERPROPERTY` reads an *attribute* of a cube member — not a measure, not an aggregate,
but a piece of descriptive data hanging off the member itself: a product's colour, a store's
city, an employee's hire date.

The reference engine (OxFunc) carries it in its catalog and defers it, with the recorded reason
**"Deferred cube-context function."** There is no implementation module and no live registry
entry — catalog-only. As with the rest of the family, the deferral is structural: member
properties are defined and stored inside the cube.

## What it computes

Given a connection, an MDX expression naming a member, and a property name, the function
returns the value of that property for that member. Microsoft's description pairs it with a
validation role, as it does for `CUBEMEMBER`: "use to validate that a member name exists within
the cube and to return the specified property for this member."

The distinction that matters is **property versus measure**, and it is the reason this function
exists alongside `CUBEVALUE`:

- A **measure** is a number the cube aggregates over the dimensions you did not pin. Retrieved
  with `CUBEVALUE`. Adding a second product to the slice changes the answer.
- A **member property** is a fixed attribute of one member. Retrieved with
  `CUBEMEMBERPROPERTY`. It does not aggregate, does not depend on any other dimension, and has
  no meaning for a set — only for a single member.

If you find yourself wanting the property "of a set", the model is telling you to use
`CUBERANKEDMEMBER` (or `CUBESET` plus iteration) to get down to one member first.

Which properties exist is a fact about the cube's schema, not about Excel. There is no closed
list, and a property name that works against one cube is meaningless against another.

## Arguments

`CUBEMEMBERPROPERTY(connection, member_expression, property)`

All three are required; there is no optional `caption` here, unlike `CUBEMEMBER` and
`CUBERANKEDMEMBER` — the result is a property value, not a member, so there is no caption to
override.

- **`connection`** (required) — text; the name of a connection stored in the workbook, used as
  a lookup key rather than as a connection string.
- **`member_expression`** (required) — text holding an MDX expression naming *a member* within
  the cube. Note the narrowing relative to `CUBEMEMBER`, whose corresponding argument also
  accepts a tuple: a property belongs to one member.
- **`property`** (required) — text naming the property, or a reference to a cell containing
  that name. This is the argument readers misjudge: it is a schema-level identifier from the
  cube's own metadata, not a display label and not something Excel validates in advance.

## Result and edge cases

The result is whatever the property holds — commonly text, but a property may equally be a date
or a number, so a workbook should not assume a return kind. There is no documented coercion or
formatting step; the value arrives as the cube supplies it.

While the request is outstanding the cell displays `#GETTING_DATA…` — a transient
extended-family placeholder catalogued in [the value universe](../model/01-value-universe.md),
not a failure.

**Power Pivot data models are out of scope, and the documentation says so.** Microsoft states
that this function does not work with Excel Data Models edited in Power Pivot, because those
are not multidimensional cubes. This is worth flagging loudly: much of what users call "the
cube" in a modern workbook is a tabular Data Model, and other CUBE functions do work against
it. `CUBEMEMBERPROPERTY` is the family member that draws the line.

## Errors

As documented by Microsoft on the page linked below:

- **`#NAME?`** — the connection name is not a valid workbook connection; or the OLAP server is
  not running, not available, or returns an error message.
- **`#N/A`** — the `member_expression` syntax is incorrect, or the member it names does not
  exist in the cube; or the expression refers to a session-scoped calculated member or named
  set whose PivotTable has been deleted or converted to formulas.

The documentation does not state what happens when the *member* is valid but the *property*
name is not. That is the obvious asymmetry in the error list — two of the three arguments have
documented failure modes and the third does not — and it is listed as a probe below.

## Relationships

- **`CUBEMEMBER`** — the sibling that returns the member itself rather than one of its
  attributes. `CUBEMEMBERPROPERTY` is what you reach for once `CUBEMEMBER` has confirmed the
  member exists.
- **`CUBEVALUE`** — the measure-returning counterpart. The property/measure distinction above
  is the whole reason both exist.
- **`CUBEKPIMEMBER`** — also returns a component of a cube object, but a KPI component is a
  *member expression* usable as a coordinate, whereas a member property is a terminal value.
  The two look similar in a formula and behave quite differently downstream.

Readers confuse member properties with PivotTable "Show Details" fields and with
`GETPIVOTDATA`. `GETPIVOTDATA` reads from a PivotTable rendered on a sheet;
`CUBEMEMBERPROPERTY` reads cube metadata with no PivotTable involved.

## Notes for implementers

- There is no kernel. The function is a metadata request; the whole of its behaviour is
  connection resolution, MDX member resolution, and property lookup in the cube schema.
- The return kind is not fixed by the function. An implementation that always returns text will
  diverge on date- and number-valued properties, and the divergence will only show up in
  downstream arithmetic.
- The Power Pivot exclusion is a capability boundary, not a bug: an implementation backed by a
  tabular model has no member properties to return and must decide, explicitly, what to do.
- `property` accepting either a literal or a cell reference is ordinary argument handling, but
  the cell-reference form is how workbooks parameterize property choice from a dropdown; an
  implementation must resolve it as a plain value, unlike the family's identity-bearing
  arguments.

## What has not been checked

No Handbook vector suite exists for `CUBEMEMBERPROPERTY`, and no Excel-comparison evidence is
recorded. Nobody has checked this function against Excel in the Handbook's record. Everything
behavioural above is Microsoft's documented behaviour or explicitly flagged as unknown.

Given an oracle with a live cube, the probes that would settle the most:

1. **An unknown property name on a valid member.** The documentation's silence here is the
   biggest gap. `#N/A`? `#VALUE!`? An empty result?
2. **Return-kind fidelity.** A date-typed and a numeric-typed member property, checked with
   `ISNUMBER`, `ISTEXT`, and `+0`, to see whether the cube's type survives into the cell or is
   flattened to text.
3. **A tuple passed as `member_expression`.** The argument is documented as a member; supplying
   a tuple is the natural user error and its result is unstated.
4. **Null-valued properties.** Whether an unset property behaves like `CUBEVALUE`'s documented
   null handling (a zero-length string) or differently — the two functions are documented
   separately and only observation can connect them.
5. **A Power Pivot Data Model connection.** Confirming the documented non-support, and
   recording *how* it fails, since "does not work" is not an error code.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| reference engine: deferred | OxFunc intentionally defers the function; the recorded reason is "Deferred cube-context function." |
| catalog-only | The projection carries identity and documentation metadata but no implementation module or live registry entry |
| member property | A fixed attribute of a single cube member; does not aggregate |
| measure | A numeric quantity the cube aggregates; retrieved with `CUBEVALUE` |
| Data Model / Power Pivot | Excel's tabular in-workbook model, explicitly not supported by this function |
| `#GETTING_DATA` | Extended-family transient placeholder while external data is retrieved |

## Sources

- Microsoft, "CUBEMEMBERPROPERTY function" —
  <https://support.microsoft.com/en-us/office/cubememberproperty-function-001e57d6-b35a-49e5-abcd-05ff599e8951>
  (syntax, all three arguments, the `#GETTING_DATA` remark, the `#NAME?` and `#N/A` conditions,
  and the statement that the function does not work with Excel Data Models edited in Power
  Pivot).
- Microsoft, "CUBEVALUE function" —
  <https://support.microsoft.com/en-us/office/cubevalue-function-8733da24-26d1-4e34-9b3a-84a8f00dcbe0>
  (the measure semantics contrasted above, and the documented null handling referenced in the
  probe list).
- Handbook `data/functions/FUNC.CUBEMEMBERPROPERTY.json` — admission label and reason,
  category, the XLL identity `xlfCubememberproperty` (382), and the catalog-only metadata
  status.
- Handbook `data/presence/FUNC.CUBEMEMBERPROPERTY.json` — no implementation module, no dispatch
  entry.
- Handbook `content/model/01-value-universe.md` — the extended error-code family.
