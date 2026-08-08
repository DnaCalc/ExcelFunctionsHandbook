---
schema: efh.function-page/v1
function_id: FUNC.REDUCE
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
role_in_family: "Sequential left fold: threads an accumulator through the array one cell at a time and returns the final value."
---

# REDUCE

## What it computes

`REDUCE` folds an array into a single value by threading an **accumulator** through it, one cell at
a time.

The rule:

1. The accumulator starts at `initial_value`.
2. The array is enumerated in row-major order.
3. For each cell, the callable is invoked with two arguments — the current accumulator and the
   cell's value — and its result becomes the new accumulator.
4. When the enumeration ends, the accumulator is the result.

This is a **left fold**, and the reference engine declares it as such: the numerical reduction
policy recorded in the axes is `SequentialLeftFold`, and the batch mode used to drive the
invocations is `SequentialStateful`, described in the source as an ordering obligation rather than
a hint — implementers must preserve call order and feed each accepted result back before preparing
the next argument.

Order matters here in a way it does not for `MAP`, `BYROW`, or `BYCOL`, and for two distinct
reasons:

- **Semantically**, because the callable need not be associative or commutative. Building a
  delimited string, or subtracting each element in turn, gives different answers under a different
  order.
- **Numerically**, because floating-point addition is not associative. A left fold over an array of
  doubles is a specific summation algorithm with a specific error behaviour, and it is not the same
  algorithm `SUM` uses. `REDUCE(0, range, LAMBDA(a,v,a+v))` and `SUM(range)` are not required to
  agree in the last bits, and expecting them to is a mistake worth naming explicitly.

## Arguments

`REDUCE(initial_value, array, lambda)`

- **`initial_value`** — required. The accumulator's starting value. It is the *first* argument, not
  the last, which is the position most easily misremembered.
- **`array`** — required. The values to fold over.
- **`lambda`** — required, and always third. It takes exactly two parameters, in the order
  `(accumulator, value)`.

The registry records arity exactly 3 — minimum and maximum both 3. Nothing is optional and there is
no repeating group, which distinguishes `REDUCE` from `MAP` at a glance.

The commonly misunderstood positions are the first argument and the callable's parameter order.
Swapping `(accumulator, value)` for `(value, accumulator)` produces a lambda that runs without
complaint and computes something else.

## Result and edge cases

Returns a single value of whatever kind the last invocation produced. Unlike the other helpers on
this shelf, `REDUCE` does not return an array.

- **Empty array.** With nothing to fold, `initial_value` should be the answer. This is the
  degenerate case most likely to differ between implementations and is unverified here.
- **Single-cell array.** One invocation.
- **Accumulator kind changes mid-fold.** Nothing forbids it: a fold that starts with a number and
  concatenates text will have a text accumulator after the first step. The callable is responsible
  for coherence.
- **Errors in the array.** Delivered to the callable as ordinary argument values. Whether the fold
  continues past an error depends entirely on what the callable returns.
- **Accumulator becomes an error.** It is threaded onward like any other value unless the callable
  handles it, so one bad cell typically poisons the whole result — usually the desired behaviour,
  occasionally a surprise.
- **Non-scalar accumulator.** The reference engine's general helper path requires scalar results
  per invocation; whether an array-valued accumulator is admissible in Excel is unverified here and
  is one of the more interesting open questions, because an affirmative answer makes `REDUCE`
  strictly more expressive.
- **All-numeric arrays** take a specialised path in the reference engine that avoids the generic
  iterable machinery. It is an optimisation with the same declared semantics; the Handbook records
  it because a divergence between the two paths would be an implementation bug rather than an Excel
  behaviour.

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | Callable arity mismatch, a missing callable, or an unsupported result kind from an invocation, per the reference engine's error mapping. |
| any error the callable returns | Becomes the accumulator and is normally the final result. |

The classification records `ErrorCollapseProfile::ReductionFold` with
`ErrorAlgebra::CanonicalExcelLegacy`. Microsoft's `REDUCE` page is the documented source for the
syntax; it was not re-fetched at this revision.

## Relationships

- **`SCAN`** is `REDUCE` that keeps every intermediate accumulator instead of only the last, and
  returns an array shaped like the input. If you want to see the running total rather than the
  total, `SCAN` is the function.
- **`MAP`** transforms without accumulating; **`BYROW`** and **`BYCOL`** reduce along one axis.
  `REDUCE` is the only member of the set that collapses everything to one value.
- **`SUM`, `PRODUCT`, `CONCAT`, `TEXTJOIN`** are the built-in folds. Prefer them when they fit:
  they are faster and, for `SUM`, their summation behaviour is the one the rest of the workbook
  assumes.
- **`LAMBDA`** supplies the callable; recursion via a named lambda is the alternative route to
  iteration, and `REDUCE` is usually the clearer one.
- Readers confuse `REDUCE` with `MAP` (accumulating versus transforming) and with `SCAN` (final
  versus running).

## Notes for implementers

1. **Sequential order is a contract, not a detail.** The reference engine's batch mode names this
   explicitly and warns that it is a setup-hoisting seam, not permission to parallelise or reorder.
   Any parallel fold changes floating-point results and breaks non-associative callables outright.
2. **Thread the accumulator by value and accept it before preparing the next argument.** Preparing
   several argument slices ahead of time is the natural optimisation and it is incorrect here.
3. **Do not assume the accumulator's kind is stable.** Typing the fold to the initial value's kind
   will fail on the first callable that changes it.
4. **State the empty-array answer.** Returning `initial_value` is the mathematically obvious
   choice; make it explicit rather than falling out of a loop that never ran.
5. **A fast path for all-numeric arrays must produce identical results to the general path**,
   including the order of operations — otherwise the optimisation is observable.

## What has not been checked

There is no Handbook vector suite for `REDUCE`; `vectors/` publishes nothing at this revision. No
Excel-comparison evidence record names `REDUCE`.

There is also **no reference-engine battery outcome**, and the reason is a real fact about the
function: the battery drives functions with fixed scalar inputs, and `REDUCE` requires a callable
argument that a value-driven harness cannot synthesise. Every battery row for `REDUCE` records
that. Higher-order functions need a higher-order harness, and none exists here yet.

Probes worth running, in priority order:

1. `=REDUCE(0, {0.1,0.2,0.3,...}, LAMBDA(a,v,a+v))` against `=SUM(...)` over the same values,
   compared at full precision — establishes whether Excel's `REDUCE` really is a plain left fold or
   whether it borrows a compensated summation. This is the most valuable probe on the page, because
   it is the one place a higher-order helper can carry a numerical fingerprint.
2. `=REDUCE(5, {}, LAMBDA(a,v,a+v))` or the nearest reachable empty-array construction — the
   degenerate case.
3. `=REDUCE(0, {1,2;3,4}, LAMBDA(a,v,a*10+v))` — a deliberately non-commutative callable, which
   reads back the enumeration order exactly. A result of `1234` confirms row-major.
4. `=REDUCE({0,0}, {1,2,3}, LAMBDA(a,v, a+v))` — whether an array-valued accumulator is admissible.
5. `=REDUCE(0, {1,#N/A,3}, LAMBDA(a,v,a+v))` — how an error propagates through the accumulator, and
   whether the fold stops early.
6. `=REDUCE("", {1,2,3}, LAMBDA(a,v,a&v))` — a kind-changing accumulator.

## Page vocabulary

| Term | Meaning |
|---|---|
| left fold | Accumulator threaded through the array in order, each step using the previous result |
| `SequentialLeftFold` | The declared numerical reduction policy for this function |
| `SequentialStateful` | The declared batch mode: call order must be preserved and results fed back |
| callable | A lambda value; its core projection is `#CALC!` |

## Sources

- Microsoft, REDUCE function —
  <https://support.microsoft.com/en-us/office/reduce-function-42e39910-b345-45f3-84b8-0642b568b7cb>
  (documented source for syntax; not re-fetched at this revision).
- Handbook call-model chapters
  [01 The value universe](../model/01-value-universe.md),
  [02 Coercion and lifting](../model/02-coercion-and-lifting.md),
  [03 The call pipeline](../model/03-call-pipeline.md).
- `data/functions/FUNC.REDUCE.json` — arity 3–3, `ReductionFold` error collapse, and the
  `numerical_reduction_policy=SequentialLeftFold` semantic-kernel marker.
- `data/presence/FUNC.REDUCE.json`.
- `data/battery/FUNC.REDUCE.json` — every row records that the function cannot be called by the
  battery runner because it requires a callable argument.
- OxFunc `crates/oxfunc_core/src/functions/callable_helpers.rs` at commit 473efa3 — the
  `CallableBatchMode::SequentialStateful` contract and its explicit warning against reordering or
  parallelising, plus the all-numeric fast path.
