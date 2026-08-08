---
schema: efh.function-page/v1
function_id: FUNC.OFFSET
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
  - Volatility
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
  - Page vocabulary
  - Sources
family: offset
role_in_family: >-
  Reference arithmetic: takes a base reference and returns a displaced, optionally resized
  reference — a value in the Reference domain, not the contents of any cell.
---

## What it computes

`OFFSET` is one of the few worksheet functions whose **result is a reference**, not a value.
It performs arithmetic in the address space of the grid.

Given a base reference `B` whose top-left cell is at row `r₀` and column `c₀`, and whose
extent is `h₀ × w₀`, together with displacements `Δr` and `Δc` and optional dimensions `h` and
`w`, `OFFSET` returns the rectangular area

- whose top-left cell is at row `r₀ + Δr` and column `c₀ + Δc`,
- with height `h` (default `h₀`) and width `w` (default `w₀`).

Note precisely what the defaults mean: **the returned area inherits the base reference's
shape, not a single cell.** `OFFSET(A1:C3, 1, 1)` is `B2:D4`, a 3×3 area — not `B2`. This is
the detail that most surprises readers who think of `OFFSET` as "the cell `n` down from here".

Microsoft states the other half explicitly: `OFFSET` does not move any cells or change the
selection; it computes an address. What happens to that address afterwards is the caller's
business — a consuming function resolves it, an aggregate scans it, a cell publishes its
contents (or spills them).

The value of `OFFSET` is that its displacement and its size are *computed*. `OFFSET(A1, 0, 0,
COUNTA(A:A), 1)` is a range whose length tracks the data — the classic dynamic-range idiom,
and the reason `OFFSET` outlived several attempts to replace it.

## Arguments

**`reference`** — the base. Must be a single cell or a contiguous range; a multi-area union is
rejected. Its size supplies the defaults for `height` and `width`.

**`rows`** — vertical displacement, positive downward, negative upward.

**`cols`** — horizontal displacement, positive rightward, negative leftward.

**`height`** — optional row count of the result, defaulting to the base reference's height.

**`width`** — optional column count of the result, defaulting to the base reference's width.

The commonly misunderstood positions are `height` and `width`, for two reasons. First, they
default to the *base's* shape rather than to `1`, as above. Second, they are counts, not
coordinates: `OFFSET(A1, 0, 0, 3, 1)` is `A1:A3`, three rows tall. The reference engine
declares that all four numeric arguments are truncated toward zero before use, and that
`height` and `width` must be positive when supplied; the Handbook has not verified either
against Excel, and Excel's treatment of negative `height`/`width` in particular is a known
folklore area rather than a documented one.

## Result and edge cases

The result kind is `ReferenceLike` — an `A1` shape when the computed extent is a single cell,
an `Area` shape otherwise. This matters because a reference can be consumed in ways a value
cannot: it can be handed to `SUM`, to `ROWS`, to `CELL("address", …)`, or to the `:` range
operator, and it can be resized again.

- **Published into a cell**, an area-shaped result is materialized: on a modern Excel it
  spills; on the legacy grid model an unrequested area produced a `#VALUE!` unless entered as
  an array formula. Which of those a reader sees is a property of the host and the workbook's
  calculation surface, not of `OFFSET`.
- **Displacement off the grid** is an error, not a clamp. Moving before row 1 or column A, or
  past the last row or column, yields `#REF!`.
- **A base reference on another sheet** displaces within that sheet; `OFFSET` carries the
  sheet identity of its base.
- Reference resolution and the values-versus-references distinction are described once in
  [the call pipeline](../model/03-call-pipeline.md), stage 1; `OFFSET` is one of the named
  `RefsVisibleInAdapter` functions there.

## Volatility

`OFFSET` is **volatile**. It is recomputed on every recalculation of the workbook, whether or
not any cell it depends on has changed, and everything downstream of it is recomputed too.

This is not an implementation choice that could be optimized away. `OFFSET`'s arguments name a
*displacement*, so the engine cannot know which cells the result will designate until it has
evaluated the function — the dependency graph cannot be built statically from the formula
text. Volatility is the price of that late binding.

The practical consequence is that `OFFSET` used at scale is a recalculation cost multiplier: a
few thousand volatile cells, each with a dependent subtree, is a workbook that pauses on every
edit. The usual modern replacements are:

- **structured table references** (`Table1[Amount]`), which grow with the table and are not
  volatile;
- **`INDEX`**, which returns a reference from an explicit range and is not volatile — the
  `A1:INDEX(A:A, n)` idiom builds a dynamic range without volatility;
- **spill anchors** (`B1#`), which reference a dynamic array's current extent directly;
- **`TRIMRANGE`** and the trim-reference operators, for the specific case of "the used part of
  this column".

`OFFSET`'s volatility class is recorded in the projected metadata as `VolatileContextual`, and
its dependency profile as caller-context-bearing. The Handbook takes the plain fact —
volatile — from Microsoft's own documentation, quoted in Sources.

## Errors

Documented by Microsoft:

| Error | Documented condition |
|---|---|
| `#VALUE!` | `reference` is not a cell or a contiguous range (for example, a multi-area union) |
| `#REF!` | `rows` and `cols` displace the reference over the edge of the worksheet |

Non-numeric text in a numeric argument is a `#VALUE!` under the ordinary coercion rules rather
than anything specific to `OFFSET`. The reference engine's contract additionally maps invalid
dimensions to `#REF!`; whether Excel agrees on the `#REF!`-versus-`#VALUE!` split for a
zero or negative `height` is exactly the kind of question the Handbook has not settled.

## Relationships

- **`INDEX`** is the closest sibling and the recommended replacement for most dynamic-range
  work: it also returns a reference, but from an explicit range and without volatility.
- **`INDIRECT`** is the other reference-producing function, and is also volatile; it builds a
  reference from *text*, which is strictly more dangerous and strictly less analyzable.
- **`ROW`, `ROWS`, `COLUMN`, `COLUMNS`, `CELL`, `ADDRESS`** are the rest of the
  reference-inspection family; `OFFSET` is the reference-*producing* member.
- **`TRIMRANGE`** and the trim-reference operators occupy some of the ground `OFFSET` was used
  for — trimming a whole-column reference down to its populated extent — without volatility.
- Readers confuse `OFFSET` with `INDEX`'s reference form (which selects within a range rather
  than displacing from it) and with `INDIRECT`.

## Notes for implementers

- The first argument must arrive reference-bearing. If argument preparation dereferences it,
  the function cannot be written: there is no address left to displace.
- The result must be a reference, not the values at that address. A tempting shortcut is to
  materialize the area and return an array; it produces the right numbers in `SUM(OFFSET(...))`
  and the wrong answer everywhere the reference identity is observed — `CELL("address",
  OFFSET(...))` is the cheap test.
- Default `height` and `width` come from the base reference's extent, which means the
  function needs the base's *shape*, not just its top-left cell.
- Volatility must be declared, not inferred. An engine that constant-folds `OFFSET` because
  all its arguments are literals will produce a stale reference after a row insertion.
- The reference engine's contract records that `OFFSET`'s empirical baseline was taken on a
  specific Excel build with both default and `.xls`-compatibility workbook lanes; the
  compatibility axis is real here, because the grid's edge — and therefore the `#REF!`
  boundary — differs between the two.

## What has not been checked

No Handbook vector suite exists for `OFFSET`, and no Handbook evidence record is attached to
this page. Nothing here claims that any implementation agrees with Excel.

`OFFSET` is unusually hard to test with value-comparison harnesses, because its result is an
address rather than a number — which is itself a reason the evidence is thin. A suite has to
observe the reference indirectly, through `CELL("address", …)`, `ROWS`, `COLUMNS`, and
composition with an aggregate.

First probes:

1. **Shape inheritance.** `OFFSET(A1:C3, 0, 0)` and friends, read back through `ROWS` and
   `COLUMNS`, to pin that the default extent is the base's and not `1×1`.
2. **The `#REF!` boundary**, at all four edges, on both the default grid and an `.xls`
   compatibility workbook — the boundary moves with the grid size.
3. **Zero and negative `height`/`width`**, and non-integer values, to pin truncation direction
   and the error each produces.
4. **Reference identity**, via `CELL("address", OFFSET(...))` and `OFFSET(OFFSET(A1,1,1),1,1)`,
   to confirm the result is a live reference rather than a materialized array.
5. **Multi-area and 3-D bases**, which the documentation covers only by exclusion.
6. **Volatility itself** — observable as recalculation behaviour rather than as a returned
   value, and therefore a host-level probe rather than a function-level one.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| reference-producing | The function returns a `ReferenceLike` value, not the cells' contents |
| shape inheritance | Omitted `height`/`width` default to the base reference's extent |
| volatile | Recomputed on every recalculation regardless of dependency changes |
| `RefsVisibleInAdapter` | Argument-preparation profile: the function receives live references |
| spill anchor | `B1#`, the reference shape naming a dynamic array's current extent |

## Sources

- Microsoft, *OFFSET function* —
  <https://support.microsoft.com/en-us/office/offset-function-c8de19ae-dd79-4b9b-a14e-b4d906d11b66>
  (syntax, argument meanings, the `height`/`width` defaults, the `#VALUE!` and `#REF!`
  conditions, the statement that `OFFSET` returns a reference without moving cells, and
  volatility). Retrieved for this page.
- Handbook `content/model/01-value-universe.md` (reference shapes and the reference domain)
  and `content/model/03-call-pipeline.md` (reference-aware argument preparation).
- OxFunc `docs/function-lane/FUNCTION_SLICE_OFFSET_CONTRACT_PRELIM.md` — the reference
  engine's declared contract: truncation toward zero, positive `height`/`width`, `#REF!` for
  invalid reference text and dimensions, and the bounded empirical baseline (Excel 16.0 build
  19725, en-US, default and `.xls`-compatibility workbook lanes).
- Handbook `data/functions/FUNC.OFFSET.json` (signature, arity, volatility and dependency
  axes).
