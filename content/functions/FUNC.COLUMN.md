---
schema: efh.function-page/v1
function_id: FUNC.COLUMN
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
family: column_fn
role_in_family: >-
  Reports the column coordinate of a reference, or of the calling cell when called with no
  argument; the caller-context member of the coordinate pair with ROW.
---

## What it computes

`COLUMN([reference])` returns the column coordinate of a reference as a number: column `A` is 1,
`B` is 2, and so on. Called with no argument it returns the column coordinate **of the cell
containing the formula**.

That no-argument form is the whole reason the function is interesting. It makes `COLUMN` one of
the small set of *caller-aware* functions described in
[the call pipeline](../model/03-call-pipeline.md): its result depends on where the formula sits,
not only on what it was given. The projection records this precisely —
`fec_dependency_profile: CallerContext`, `host_interaction_class: WorkbookState`,
`thread_safety_class: HostSerialized`. A pure kernel cannot answer this question; the execution
context has to supply the calling cell.

With an argument, `COLUMN` is reference-aware in the other sense: the projection records
`arg_preparation_profile: RefsVisibleInAdapter`, so the live reference survives into the
function. It has to. If the reference were resolved to values first, the coordinate would be
gone.

Microsoft documents that when `reference` is a range of cells, `COLUMN` returns the column
numbers of that range as a horizontal array. In modern Excel that array spills; in
pre-dynamic-array Excel it required array entry. This is the property that makes `COLUMN` a
sequence generator in older workbooks — `COLUMN(A1:E1)` as a stand-in for `SEQUENCE`.

## Arguments

| Argument | Meaning |
|---|---|
| `reference` | Optional. The cell or range whose column number you want. Omitted means the calling cell. |

Two things about this single argument are commonly misunderstood.

**Omitted is not the same as empty.** `COLUMN()` delivers the Missing marker and selects the
caller-context behaviour; `COLUMN(A1)` where `A1` is blank delivers a perfectly good reference
to an empty cell and returns 1. The value model keeps Missing and Empty distinct precisely so
that functions like this one can branch on the difference — see
[the value universe](../model/01-value-universe.md).

**The documentation for the sibling `ROW` states that its reference cannot refer to multiple
areas**, and the same restriction should be expected here; the Handbook has not established what
`COLUMN` does with a union reference.

## Result and edge cases

Return kind: `Number` for a single cell; `Array` (a horizontal array of column numbers) for a
multi-column range, per Microsoft's documentation.

Edge cases specific to this function:

- **A multi-row, single-column range.** The column number is constant down the range. Whether
  the result is a single number or an array of repeated numbers is a shape question the
  Handbook has not settled.
- **A whole-column reference** (`A:A`) — one column, so presumably 1, but the interaction with
  the array-returning rule is untested here.
- **A three-dimensional reference** (`Sheet1:Sheet3!A1`) — the coordinate is well defined but
  the reference shape is distinct in the value model.
- **A spill anchor** (`B1#`) — the referenced range's extent is dynamic, so the result's shape
  is too.
- **Inside a lifted or iterated context** (a `LAMBDA` body, `BYCOL`, a defined name evaluated
  from several cells) the "calling cell" that `COLUMN()` reports is a question about the
  execution context, not about this function's kernel — and it is exactly where caller-aware
  functions get surprising.

## Errors

Microsoft's page does not publish an error table for `COLUMN`. The expected failure is a
non-reference argument — `COLUMN(5)`, `COLUMN("A1")` — for which `#VALUE!` is this family's
conventional code; the Handbook records that as expected, not verified. An error value passed as
the argument propagates under the universal rule in
[coercion and lifting](../model/02-coercion-and-lifting.md).

## Relationships

- **`ROW`** is the same function on the other axis and shares every structural property: the
  optional argument, the caller-context default, the array result over a range.
- **`COLUMNS`** counts columns; `COLUMN` locates them. `COLUMNS(A1:E1)` is 5, `COLUMN(A1:E1)` is
  the array 1…5. The near-identical names for genuinely different operations are a standing
  source of confusion, and the same trap exists for `ROW`/`ROWS`.
- **`SEQUENCE`** replaces the historical use of `COLUMN(range)` as a horizontal counter, and
  does so without depending on where the range happens to sit.
- **`ADDRESS`** consumes coordinates; `COLUMN` produces them.
- **`CELL("col", ref)`** answers the same question through the workbook-metadata route.
- **`OFFSET` and `INDEX`** are the functions you feed a computed column number to.

## Notes for implementers

- **`COLUMN()` is not evaluable without a host.** The reference implementation classifies it
  `HostSerialized` and dependent on caller context; an implementation with no notion of a
  calling cell cannot answer the no-argument form at all, and should say so rather than invent
  a default. This is a real fault line between a function library and a calculation engine.
- **The relative/absolute distinction is irrelevant here.** `$A$1` and `A1` have the same column
  number; `COLUMN` reads the resolved coordinate, not the formula text.
- **Copying a formula changes the answer** in the no-argument form and, through relative
  reference adjustment, usually in the argument form as well. Any caching layer keyed on formula
  text rather than on cell position will be wrong for this function.
- **Volatility.** The projection records `volatility_class: NonVolatile`. `COLUMN` is
  caller-aware without being volatile: the answer changes when the formula moves, not on every
  recalculation. That distinction matters when comparing it with `INDIRECT`, which is recorded
  as `VolatileContextual`.

## What has not been checked

No Handbook vector suite exists for `COLUMN`, and no Excel-comparison evidence record is
recorded for it. Nobody has checked this function against Excel here. The reference
implementation's own battery cannot exercise the no-argument form at all without a host
facility, which is itself worth knowing: the caller-context path is the part of this function
that is hardest to test outside Excel.

What to probe first:

1. **`COLUMN()` in a cell, and the same formula copied across and down.** This establishes the
   caller-context rule and confirms it tracks position rather than formula text.
2. **`COLUMN(range)` for a multi-column range, a multi-row single-column range, and a
   rectangular range.** Three different shapes; Microsoft documents only the horizontal-array
   case, so the other two are genuinely open.
3. **A union reference** — `COLUMN((A1,C1))` — against the restriction documented for `ROW`.
4. **Whole-column, three-dimensional, structured, and spill-anchor references**, one probe
   each; these are distinct reference shapes in the value model and none is inferable from the
   others.
5. **`COLUMN()` evaluated inside `LAMBDA`, `BYCOL`, and a defined name**, to establish what the
   calling cell is when the formula's evaluation is nested or iterated. This is the item most
   likely to produce a surprise.
6. **Non-reference arguments**: a number, a text address, an array literal, an error value.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| caller-aware | The result depends on the calling cell's position (`FecDependencyProfile::CallerContext`) |
| reference-aware | The live reference survives into the function (`RefsVisibleInAdapter`) |
| horizontal array | A one-row array; the documented result shape over a multi-column range |
| Missing | The omitted-argument marker, distinct from an empty cell |

## Sources

- Microsoft, COLUMN function —
  <https://support.microsoft.com/en-us/office/column-function-44e8c754-711c-4df3-9da4-47a55042554b>
  (optional argument, calling-cell default, and the horizontal-array result over a range).
- Handbook `content/model/01-value-universe.md` (Missing versus Empty; reference shapes).
- Handbook `content/model/02-coercion-and-lifting.md` (error propagation; reference resolution
  as an explicit step).
- Handbook `content/model/03-call-pipeline.md` (`RefsVisibleInAdapter`; caller-aware functions).
- Handbook `data/functions/FUNC.COLUMN.json` and `data/presence/FUNC.COLUMN.json` (arity,
  classification axes including `CallerContext` and `HostSerialized`, implementing module).
