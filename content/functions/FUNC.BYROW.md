---
schema: efh.function-page/v1
function_id: FUNC.BYROW
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
role_in_family: "Row-wise reduction: invokes the callable once per row and returns one column of results."
---

# BYROW

## What it computes

`BYROW` invokes a callable once for each **row** of an array, passing that entire row as a single
argument, and assembles the results into a single column.

The shape rule is the function's whole identity:

> An `r × c` input produces an `r × 1` output.

Each invocation receives a `1 × c` array — one complete row — and must return a scalar. The result
lands in the corresponding cell of the output column. `BYROW` therefore collapses the column
dimension and preserves the row dimension.

This is the array-formula idiom that Excel could not previously express. Writing
`SUM` across each row of a range required either a helper column per row or an array formula that
worked only for specific functions; `BYROW(range, LAMBDA(r, SUM(r)))` expresses "reduce each row
with this arbitrary expression" directly.

The callable takes exactly one parameter — the row.

## Arguments

`BYROW(array, lambda)`

- **`array`** — required. The array or range whose rows are visited. The registry records arity
  exactly 2: minimum and maximum both 2, so neither argument is optional and there is no repeating
  group.
- **`lambda`** — required, and always the second argument. It must accept one argument and return a
  scalar.

The commonly misunderstood point is what the callable receives. It is handed **the whole row as an
array**, not a cell and not a row number. A lambda written as `LAMBDA(x, x*2)` will be handed an
array and will behave accordingly; a lambda intended for per-cell work belongs in `MAP`.

The second misunderstanding is directionality: "by row" means *one result per row*, produced by
consuming the row across its columns. Readers occasionally expect the opposite.

## Result and edge cases

Returns an `r × 1` array, where `r` is the input's row count.

- **Scalar input.** Treated as a one-cell array, giving a one-cell result.
- **Single-row input.** One invocation, one result cell.
- **Callable returns a non-scalar.** There is one output cell per row and no room for an array; the
  reference engine reports a non-scalar helper result rather than nesting or spilling sideways.
- **Errors inside a row.** Passed to the callable as ordinary values within the row array; the
  callable decides. `BYROW` does not pre-filter.
- **Row containing text or blanks.** Delivered as-is inside the row array; whatever the callable
  does with them (for example, `SUM`'s ignore-text scan rule) governs, not `BYROW`.
- **Spilling.** The result is an array; a blocked spill publishes `#SPILL!` at the host boundary
  ([chapter 03](../model/03-call-pipeline.md), "Host-side adaptation").

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | Callable arity mismatch, a missing callable, or a non-scalar result from one invocation, per the reference engine's error mapping. |
| any error the callable returns | Placed in that row's result cell. |
| `#SPILL!` | The result column could not spill — a host-side condition. |

The classification records `ErrorCollapseProfile::ReductionFold` with
`ErrorAlgebra::CanonicalExcelLegacy`. Microsoft's `BYROW` page is the documented source for the
syntax; it was not re-fetched at this revision.

## Relationships

- **`BYCOL`** is the exact transpose: an `r × c` input gives a `1 × c` output, with each invocation
  receiving a whole column. The two share one implementation module in the reference engine and
  differ only in the enumeration axis.
- **`MAP`** preserves both dimensions and works cell by cell. If your callable ignores the array
  structure of its argument, you probably wanted `MAP`.
- **`REDUCE`** collapses the entire array to one value rather than one value per row.
- **`SUMPRODUCT` and array-entered aggregates** are the pre-`LAMBDA` idioms this replaces; they
  remain useful and are usually faster for the specific reductions they support.
- **`TRANSPOSE`** plus `BYROW` is a common way to reach `BYCOL` behaviour when it is unavailable,
  and vice versa.
- Readers confuse `BYROW` with `MAP` more than with anything else, and the symptom is a lambda that
  works on a single-column input and misbehaves on a wide one.

## Notes for implementers

1. **Materialise the row as a `1 × c` array, not as a list of scalars.** A callable that calls
   `SUM` on its argument needs a real array; a scalar-per-call design cannot express `BYROW`.
2. **The output shape is fixed by the input, not by the results.** Allocate `r × 1` and require a
   scalar per invocation; do not infer shape from what comes back.
3. **A scalar input must still be lifted to a one-cell array** before enumeration, or the row loop
   has nothing to iterate.
4. **Invocation order is observable if the callable is impure**, exactly as for `MAP`; commit to
   top-to-bottom and say so.
5. `BYROW` and `BYCOL` should be one enumerator parameterised by axis. They are the clearest case
   on this shelf where duplicated code drifts.

## What has not been checked

There is no Handbook vector suite for `BYROW`; `vectors/` publishes nothing at this revision. No
Excel-comparison evidence record names `BYROW`.

There is also **no reference-engine battery outcome**, and the reason is a real fact about the
function: the battery drives functions with fixed scalar inputs, and `BYROW` requires a callable
argument that a value-driven harness cannot synthesise. Every battery row for `BYROW` records that
it cannot be called for exactly that reason. Testing higher-order functions requires a harness that
can construct lambdas, and none exists here yet.

Probes worth running, in priority order:

1. `=BYROW({1,2;3,4}, LAMBDA(r, SUM(r)))` — the base case, confirming the `r × 1` shape and that
   the callable receives a whole row.
2. `=BYROW({1,2;3,4}, LAMBDA(r, r))` — a callable returning the row unchanged, which is
   non-scalar. Does Excel error, take the first element, or something else?
3. `=BYROW(5, LAMBDA(r, r*2))` — scalar input: one-cell array or bare scalar?
4. `=BYROW(A1:C3, LAMBDA(r, COUNT(r)))` with text and blanks in the range — confirms that the row
   array carries them through and that the callable's own scan rules apply.
5. `=BYROW({1,#N/A;3,4}, LAMBDA(r, SUM(r)))` — how an error inside a row surfaces, and whether it
   affects only that row's result cell.
6. A single-row and a single-column input — confirms the degenerate shapes behave as the general
   rule predicts rather than being special-cased.

## Page vocabulary

| Term | Meaning |
|---|---|
| row-wise reduction | One callable invocation per row, each producing one scalar |
| higher-order helper | A function whose argument is itself a callable |
| callable | A lambda value; its core projection is `#CALC!` |
| `ErrorCollapseProfile::ReductionFold` | Family that folds inputs into a result and collapses errors by precedence |

## Sources

- Microsoft, BYROW function —
  <https://support.microsoft.com/en-us/office/byrow-function-2e04c677-78c8-4e6b-8c10-a4602f2602bb>
  (documented source for syntax; not re-fetched at this revision).
- Handbook call-model chapters
  [01 The value universe](../model/01-value-universe.md) and
  [03 The call pipeline](../model/03-call-pipeline.md).
- `data/functions/FUNC.BYROW.json`, `data/presence/FUNC.BYROW.json`.
- `data/battery/FUNC.BYROW.json` — every row records that the function cannot be called by the
  battery runner because it requires a callable argument.
- OxFunc `crates/oxfunc_core/src/functions/callable_helpers.rs` at commit 473efa3 — the row
  enumeration, the fixed `rows × 1` output shape, and the scalar-result requirement.
