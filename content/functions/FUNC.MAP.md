---
schema: efh.function-page/v1
function_id: FUNC.MAP
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
role_in_family: "Elementwise application: invokes the callable once per cell position and returns an array of the results."
---

# MAP

## What it computes

`MAP` applies a callable to every cell position of one or more arrays, in parallel across the
arrays, and returns an array of the results.

The rule:

1. The **last** argument is the callable. Everything before it is an input array.
2. Positions are enumerated in row-major order over the arrays' common extent.
3. For each position, the callable is invoked with one argument per input array — the value at that
   position in each.
4. The results are assembled into an array whose shape is inferred from the inputs.

`MAP` is the higher-order counterpart of the engine's built-in array lifting. The engine already
lifts scalar kernels over arrays automatically ([chapter 02](../model/02-coercion-and-lifting.md),
"Array lifting"); `MAP` lets a formula author do the same thing with an expression of their own
instead of a built-in kernel. Everything `MAP` does could in principle be written as an array
formula — the value of `MAP` is that the per-element computation can be arbitrarily complicated and
still be written once.

The callable's declared parameter count must match the number of input arrays. A one-array `MAP`
takes a one-parameter lambda; a two-array `MAP` takes a two-parameter lambda. The reference engine
checks this arity before each invocation and reports a mismatch rather than padding.

## Arguments

`MAP(array1, [array2, ...], lambda)`

- **`array1`** — required. The first input array. A scalar is treated as a one-cell array.
- **`array2`, …** — optional additional input arrays, each contributing one argument to the
  callable.
- **`lambda`** — required, and always the **last** argument. The registry records an accepted arity
  of 2 to 255 arguments.

The commonly misunderstood position is the last one. The projected signature displays the callable
in trailing position after a repeating group, which reads as though the lambda were optional; it is
not. `MAP` with a single array and no callable has nothing to apply.

The second misunderstanding is shape: `MAP` pairs inputs **positionally**, not combinatorially.
Two arrays do not produce a cross product; they produce one result per position. If you want a
cross product, you want `MAKEARRAY` or a broadcast expression.

## Result and edge cases

Returns an array.

- **Result shape.** Inferred from the inputs. The reference engine sizes the enumeration by the
  largest input extent and then validates that a well-formed output shape exists. What Excel does
  when two inputs have *different* shapes — broadcast, `#N/A` padding, or `#VALUE!` — is not
  settled here and is the first probe listed below.
- **Scalar input.** Treated as a one-cell array, so `MAP(5, LAMBDA(x, x*2))` is a one-cell result
  rather than a bare scalar.
- **Non-scalar callable result.** A callable that returns an array for a single position has no
  cell to occupy; the reference engine reports a non-scalar helper result rather than nesting.
- **Errors inside the array.** An error value at a position is passed to the callable as an
  ordinary argument. What the callable does with it decides the outcome for that cell — `MAP` does
  not pre-filter errors.
- **Callables inside the input array.** Preserved: the reference engine deliberately carries a
  callable stored in an array cell through to invocation instead of reconstructing it from its
  `#CALC!` core projection, so `MAP(array_of_lambdas, LAMBDA(f, f(10)))` is meaningful.
- **Spilling.** The result is an array value; whether it lands in the grid is a host-side
  adaptation question ([chapter 03](../model/03-call-pipeline.md), "Host-side adaptation"), and a
  blocked spill publishes `#SPILL!`.

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | Callable arity mismatch, a missing callable in the last position, or a non-scalar result from one invocation, per the reference engine's error mapping. |
| any error the callable returns | Placed in the corresponding result cell. |
| `#SPILL!` | The result array could not spill into the grid — a host-side condition, not a `MAP` condition. |

The classification records `ErrorCollapseProfile::ReductionFold` with
`ErrorAlgebra::CanonicalExcelLegacy`. Microsoft's `MAP` page is the documented source for the
syntax; it was not re-fetched at this revision.

## Relationships

- **`LAMBDA`** supplies the callable and is export-only; see its page for why.
- **`BYROW` and `BYCOL`** apply a callable to whole rows or columns instead of to single cells, and
  reduce each to one value. `MAP` preserves shape; they collapse one dimension.
- **`REDUCE`** collapses the whole array to a single accumulated value; `SCAN` is `REDUCE` that
  keeps every intermediate. `MAP` is the shape-preserving member of the set.
- **`MAKEARRAY`** builds an array from row and column indices rather than from existing arrays —
  the generator to `MAP`'s transformer.
- **Implicit array lifting** is what `MAP` makes explicit; for a simple elementwise expression the
  ordinary array formula is shorter and does the same thing.
- Readers confuse `MAP` with `BYROW` when the intent is per-row aggregation, and with `REDUCE` when
  the intent is a running total.

## Notes for implementers

1. **The callable is positional, at the end.** Splitting the argument list as "all but last are
   inputs, last is the callable" is the entire parsing rule, and it must happen before argument
   preparation decides anything else.
2. **Check callable arity per call, not once.** The number of inputs fixes the expected parameter
   count, but a callable value can in principle arrive from an array cell and differ per position.
3. **Preserve callables carried in array cells.** Reconstructing an array cell from its core
   projection turns a callable into `#CALC!` and silently breaks higher-order use. The reference
   engine special-cases this, and the special case is load-bearing.
4. **Decide the mismatched-shape policy explicitly.** Broadcasting, `#N/A` padding, and outright
   refusal are all plausible, and the pipeline's general broadcast rule (`#N/A` where a coordinate
   is missing) may or may not govern a higher-order helper.
5. **Order of invocation is observable if the callable is impure.** Nothing in the worksheet model
   makes lambdas pure in practice — a body referencing a volatile function or a cell can see
   different values per call — so a documented enumeration order is worth committing to.

## What has not been checked

There is no Handbook vector suite for `MAP`; `vectors/` publishes nothing at this revision. No
Excel-comparison evidence record names `MAP`.

There is also **no reference-engine battery outcome** for `MAP`, and the reason is a real,
publishable fact about the function rather than a gap in the harness's diligence: the battery drives
functions with fixed scalar inputs, and `MAP` requires a *callable* argument that a value-driven
harness cannot synthesise. Every battery row for `MAP` records exactly that. Higher-order functions
need a higher-order harness, and none exists here yet.

Probes worth running, in priority order:

1. `=MAP({1,2;3,4}, {10,20}, LAMBDA(a,b,a+b))` — mismatched input shapes. Broadcast, `#N/A`
   padding, or `#VALUE!`? Nothing on this page is settled until this is.
2. `=MAP({1,2,3}, LAMBDA(x, {x,x}))` — a callable returning a non-scalar per position.
3. `=MAP({1,2,3}, LAMBDA(a,b,a+b))` — arity mismatch between input count and parameter count, and
   the reverse case with a two-parameter callable and one array.
4. `=MAP(5, LAMBDA(x,x*2))` — does a scalar input give a one-cell array or a bare scalar?
5. `=MAP({1,#N/A,3}, LAMBDA(x, IFERROR(x, 0)))` — confirms errors reach the callable as ordinary
   arguments rather than short-circuiting the whole call.
6. A callable whose body references a volatile function, over a multi-cell array — establishes
   whether invocation order and count are observable.

## Page vocabulary

| Term | Meaning |
|---|---|
| higher-order helper | A function whose argument is itself a callable |
| callable | A lambda value; its core projection is `#CALC!` |
| positional pairing | Inputs combined index-by-index, not as a cross product |
| `ErrorCollapseProfile::ReductionFold` | Family that folds many inputs into a result and collapses errors by precedence |

## Sources

- Microsoft, MAP function —
  <https://support.microsoft.com/en-us/office/map-function-48006093-f97c-47c1-bfcc-749263bb1f01>
  (documented source for syntax; not re-fetched at this revision).
- Handbook call-model chapters
  [01 The value universe](../model/01-value-universe.md),
  [02 Coercion and lifting](../model/02-coercion-and-lifting.md),
  [03 The call pipeline](../model/03-call-pipeline.md).
- `data/functions/FUNC.MAP.json`, `data/presence/FUNC.MAP.json`.
- `data/battery/FUNC.MAP.json` — every row records that the function cannot be called by the
  battery runner because it requires a callable argument.
- OxFunc `crates/oxfunc_core/src/functions/callable_helpers.rs` at commit 473efa3 — the
  last-argument-is-callable split, the output-shape inference, the per-invocation arity check, and
  the deliberate preservation of callables carried in array cells.
