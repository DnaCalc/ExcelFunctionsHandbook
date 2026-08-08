---
schema: efh.function-page/v1
function_id: FUNC.BYCOL
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
family: callable_helpers
role_in_family: "Column-wise reduction: invokes the callable once per column and returns one row of results."
---

# BYCOL

## What it computes

`BYCOL` invokes a callable once for each **column** of an array, passing that entire column as a
single argument, and assembles the results into a single row.

The shape rule:

> An `r × c` input produces a `1 × c` output.

Each invocation receives an `r × 1` array — one complete column — and must return a scalar, which
lands in the corresponding cell of the output row. `BYCOL` collapses the row dimension and
preserves the column dimension. It is the exact transpose of `BYROW`, and in the reference engine
the two share one implementation, differing only in the enumeration axis and the output shape.

The everyday use is a totals row: `BYCOL(range, LAMBDA(c, SUM(c)))` produces one aggregate per
column with an arbitrary expression, which before `LAMBDA` required either a literal row of
formulas or an aggregate that happened to support the shape.

The callable takes exactly one parameter — the column.

## Arguments

`BYCOL(array, lambda)`

- **`array`** — required. The array or range whose columns are visited. The registry records arity
  exactly 2: both minimum and maximum.
- **`lambda`** — required, second argument, one parameter, scalar result.

The commonly misunderstood point, as with `BYROW`, is what the callable receives: **the whole
column as an array**, not a single cell and not a column index. A lambda written for scalars will
be handed an array.

The second is orientation. `BYCOL` produces a *row* of results — one per column. The output is
oriented across the direction being collapsed, which reads backwards the first time.

## Result and edge cases

Returns a `1 × c` array, where `c` is the input's column count.

- **Scalar input.** Treated as a one-cell array; one-cell result.
- **Single-column input.** One invocation, one result cell.
- **Callable returns a non-scalar.** There is one output cell per column and no room for an array;
  the reference engine reports a non-scalar helper result.
- **Errors inside a column.** Delivered to the callable inside the column array; the callable
  decides. `BYCOL` does not pre-filter.
- **Text and blanks in a column.** Carried through as-is; the callable's own scan rules govern —
  `SUM`'s ignore-text rule, `COUNT`'s counting rule, and so on
  ([chapter 02](../model/02-coercion-and-lifting.md)).
- **Spilling.** A blocked spill publishes `#SPILL!` at the host boundary.

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | Callable arity mismatch, a missing callable, or a non-scalar result from one invocation, per the reference engine's error mapping. |
| any error the callable returns | Placed in that column's result cell. |
| `#SPILL!` | The result row could not spill — a host-side condition. |

The classification records `ErrorCollapseProfile::ReductionFold` with
`ErrorAlgebra::CanonicalExcelLegacy`. Microsoft's `BYCOL` page is the documented source for the
syntax; it was not re-fetched at this revision.

## Relationships

- **`BYROW`** is the transpose. `TRANSPOSE(BYROW(TRANSPOSE(a), f))` and `BYCOL(a, f)` should agree,
  and disagreement between them would be a genuine finding — it is one of the cheapest metamorphic
  checks available on this shelf.
- **`MAP`** preserves both dimensions and works cell by cell.
- **`REDUCE`** collapses everything to one value; `SCAN` keeps the intermediates.
- **`MAKEARRAY`** generates rather than reduces.
- Readers confuse `BYCOL` with `BYROW` by orientation, and both with `MAP` by granularity.

## Notes for implementers

1. **Materialise the column as an `r × 1` array.** A callable that aggregates its argument needs a
   real array, and the orientation matters to anything shape-sensitive inside the body.
2. **Fix the output shape from the input**, not from the results; require a scalar per invocation.
3. **Share one enumerator with `BYROW`, parameterised by axis.** Duplicating the loop is how the
   two drift apart in error handling and degenerate-shape behaviour.
4. **Lift a scalar input to a one-cell array** before enumerating.
5. **Commit to left-to-right invocation order** and state it, since an impure callable makes order
   observable.

## What has not been checked

There is no Handbook vector suite for `BYCOL`; `vectors/` publishes nothing at this revision. No
Excel-comparison evidence record names `BYCOL`.

There is also **no reference-engine battery outcome**, for a reason that is a real fact about the
function rather than an omission: the battery drives functions with fixed scalar inputs, and
`BYCOL` requires a callable argument that a value-driven harness cannot synthesise. Every battery
row for `BYCOL` records that it cannot be called for exactly that reason.

Probes worth running, in priority order:

1. `=BYCOL({1,2;3,4}, LAMBDA(c, SUM(c)))` — the base case, confirming the `1 × c` shape and that
   the callable receives a whole column.
2. `=BYCOL(a, f)` against `=TRANSPOSE(BYROW(TRANSPOSE(a), f))` for several shapes — the
   metamorphic check. Any disagreement is a finding about one of the two functions.
3. `=BYCOL({1,2;3,4}, LAMBDA(c, c))` — a non-scalar result per column.
4. `=BYCOL(5, LAMBDA(c, c*2))` — scalar input.
5. `=BYCOL(A1:C3, LAMBDA(c, SUM(c)))` with text and blanks — confirms the column array carries
   them and the callable's scan rules apply.
6. A single-row input, where every column is one cell — confirms the degenerate case follows the
   general rule.

## Page vocabulary

| Term | Meaning |
|---|---|
| column-wise reduction | One callable invocation per column, each producing one scalar |
| metamorphic check | Comparing two routes to the same answer (here, `BYCOL` versus transposed `BYROW`) |
| callable | A lambda value; its core projection is `#CALC!` |
| `ErrorCollapseProfile::ReductionFold` | Family that folds inputs into a result and collapses errors by precedence |

## Sources

- Microsoft, BYCOL function —
  <https://support.microsoft.com/en-us/office/bycol-function-58463999-7de5-49ce-8f38-b7f7a2192bfb>
  (documented source for syntax; not re-fetched at this revision).
- Handbook call-model chapters
  [02 Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [03 The call pipeline](../model/03-call-pipeline.md).
- `data/functions/FUNC.BYCOL.json`, `data/presence/FUNC.BYCOL.json`.
- `data/battery/FUNC.BYCOL.json` — every row records that the function cannot be called by the
  battery runner because it requires a callable argument.
- OxFunc `crates/oxfunc_core/src/functions/callable_helpers.rs` at commit 473efa3 — the column
  enumeration, the fixed `1 × cols` output shape, and the scalar-result requirement.
