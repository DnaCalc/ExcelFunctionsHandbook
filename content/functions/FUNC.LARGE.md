---
schema: efh.function-page/v1
function_id: FUNC.LARGE
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
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: large_fn
role_in_family: >-
  The descending order-statistic selector: returns the k-th largest value without
  interpolating, and is SMALL's exact mirror.
---

# LARGE

## What it computes

`LARGE(array, k)` returns the **k-th largest** value of the numeric data in `array`.

Formally, let `x₍₁₎ ≤ x₍₂₎ ≤ … ≤ x₍ₙ₎` be the admitted values sorted ascending — the *order
statistics* of the sample. Then

    LARGE(array, k) = x₍ₙ₋ₖ₊₁₎

so `LARGE(array, 1)` is the maximum, `LARGE(array, n)` is the minimum, and `k` counts down
from the top. **Ties count with multiplicity**: in `{5, 5, 3}` both `LARGE(·,1)` and
`LARGE(·,2)` are `5`. This is the point most often missed — `LARGE` is not "the k-th largest
*distinct* value", and there is no built-in that is.

**Domain.** `1 ≤ k ≤ n`, where `n` is the count of numeric values actually admitted from
`array`. `k` outside that range has no order statistic to name. **Range:** exactly one of the
input values — `LARGE` is a *selector*, and its result is always bit-identical to some element
of the data. No arithmetic is performed, which is the property that distinguishes it from the
percentile family.

That last point deserves emphasis, because it is the whole reason `LARGE` exists alongside
[PERCENTILE.INC](FUNC.PERCENTILE.INC.md). `PERCENTILE.INC(array, p)` interpolates linearly
between adjacent order statistics and can return a number that appears nowhere in the data.
`LARGE` never does. If you need "the third-highest actual observation", `LARGE` is the correct
function and a percentile is not.

The complementary identity with [SMALL](FUNC.SMALL.md) is exact:

    LARGE(array, k) = SMALL(array, n − k + 1)

## Arguments

| Argument | Meaning | Admissible values |
|---|---|---|
| `array` | The data set to select from | Array or reference; required |
| `k` | The rank, counted from the largest | Number; required; `1 ≤ k ≤ n` |

The reference engine declares an arity of exactly 2 — `k` is not optional, so there is no
"default to the maximum" shorthand.

`k` is a numeric slot subject to ordinary to-number coercion; `array` is a scanned collection
governed by the range-scan policy. The expected handling of a non-integer `k` is truncation
toward zero before the range check, which is the convention the rest of the index-taking
surface uses, but Microsoft's page does not state it on this function and the Handbook has
not confirmed it. See [Coercion and lifting](../model/02-coercion-and-lifting.md).

A subtlety worth stating: `n` is the count of values that survive the scan, not the number of
cells in the range. Blanks and text in a referenced range are ignored by the usual aggregate
scan policy, so `LARGE(A1:A10, 10)` over a column with two blanks is an out-of-range request
even though the range has ten cells. That is a common and silent source of `#NUM!`.

## Result and edge cases

Returns `Number` — one of the input values, unmodified.

- **`k = 1`** coincides with [MAX](FUNC.MAX.md) over the same admitted set, and `k = n` with
  [MIN](FUNC.MIN.md). Whether the three surfaces agree on which *bits* they return when the
  extremum is attained by values that compare equal but differ in representation (`0` and
  `−0`) is unverified.
- **Empty admitted set** — every value in `array` was skipped, or the range is blank. There is
  no order statistic and the documented result is `#NUM!`. The reference engine's own battery,
  which is OxFunc's answers with no Excel involved, returns `#VALUE!` for its blank-argument
  row and `#NUM!` for its out-of-range-`k` rows; the two are different failures and the
  boundary between them has not been characterised.
- **`k` non-integer**, `k ≤ 0`, or `k > n` → `#NUM!` as documented.
- **A direct logical** in either slot is coerced by the direct-argument rules — `TRUE` becomes
  `1` in both positions — while a logical inside a referenced range is ignored. This is the
  standard asymmetry of chapter 02, and it means `LARGE(TRUE, TRUE)` is a well-formed call.
- **An array-valued `k`.** The reference engine's battery rejects a two-dimensional `k` rather
  than lifting `LARGE` elementwise over it. Modern Excel spills many scalar-slot arguments,
  and `LARGE(array, {1;2;3})` returning the top three is an idiom people write. Whether Excel
  lifts here is not stated on Microsoft's page and is unverified; the reference engine's
  current answer is a divergence candidate rather than a settled fact.
- **Error values** in `array` propagate rather than being skipped.

## Errors

| Error | Condition | Basis |
|---|---|---|
| `#NUM!` | `array` is empty, or `k < 1`, or `k` exceeds the number of admitted values | Documented by Microsoft on the `LARGE` page |
| `#VALUE!` | An argument cannot be converted — a non-numeric `k`, or a directly-passed non-numeric `array` | Shared coercion rule, chapter 02 |
| propagated | An error value in `array` | Shared coercion rule, chapter 02 |

Retrieval of Microsoft's `LARGE` page was blocked for this curation pass, so the `#NUM!` row
is stated as documented behaviour with its source named rather than transcribed. Nobody has
checked any of this against Excel within the Handbook's record.

## Relationships

- **[SMALL](FUNC.SMALL.md)** — the ascending mirror, with the exact index identity given
  above. The two are the same algorithm with a reversed comparator, and an implementation that
  does not share code between them is duplicating the delicate part twice.
- **[MAX](FUNC.MAX.md)** / **[MIN](FUNC.MIN.md)** — the `k = 1` and `k = n` special cases,
  with different argument shapes: `MAX` takes a variadic argument list, `LARGE` takes exactly
  one array. `MAX` also has a documented "returns 0 if there are no numbers" behaviour where
  `LARGE` errors — the two disagree deliberately on the empty case.
- **[PERCENTILE.INC](FUNC.PERCENTILE.INC.md)** / **[PERCENTILE.EXC](FUNC.PERCENTILE.EXC.md)**
  / **[QUARTILE.INC](FUNC.QUARTILE.INC.md)** — the interpolating relatives. `PERCENTILE.INC`
  at `p = (n−k)/(n−1)` equals `LARGE(array, k)` exactly, because at the knots the linear
  interpolation is degenerate. Away from the knots they differ, and that is the whole point.
- **[RANK.EQ](FUNC.RANK.EQ.md)** / **[RANK.AVG](FUNC.RANK.AVG.md)** — the inverse direction:
  given a value, return its position. `RANK.EQ` and `LARGE` are inverses on data with no ties
  and are *not* inverses when ties are present, because `RANK.EQ` collapses tied ranks while
  `LARGE` does not.
- **[SORT](FUNC.SORT.md)** and **[TAKE](FUNC.TAKE.md)** — the modern dynamic-array route to
  "the top k", which returns all of them at once and makes the tie behaviour visible.
- **Confused with:** `LARGE` and `MAX` when the data contain text — the scan policy decides
  which values count, and it is the same policy for both, so a disagreement between them is a
  signal that something else is wrong.

## Numerical notes

`LARGE` performs no arithmetic, so it has no rounding error and no residual plate worth
drawing. Its entire correctness question is the **comparison predicate** and the **selection
algorithm**.

**The comparison predicate.** Selecting an order statistic requires a total order on the
admitted values. Three questions have to be answered and none of them is answered by
Microsoft's page:

1. Is the comparison raw IEEE-754 `<`, or is it the tolerant, truncation-style
   15-significant-digit comparison that Excel uses in several other families? The reference
   engine's upstream record documents a family split on exactly this point for the criteria
   functions. If `LARGE` were tolerant, `LARGE({0.3, 0.1+0.2}, 1)` and `LARGE({0.3, 0.1+0.2},
   2)` would be free to return either representation; if it is exact, the two are ordered.
2. How are `+0` and `−0` ordered? IEEE comparison calls them equal, so a stable selection
   returns whichever came first in scan order and an unstable one may not.
3. Excel has no NaN in its value universe, so the usual NaN-ordering trap does not arise —
   but an implementation built on a generic float comparator has to decide what it does with
   one anyway, and the safe choice is to make it unreachable.

**The selection algorithm.** Selecting the `k`-th largest of `n` values does not require
sorting. The classical options are:

- **Full sort**, `O(n log n)` — simplest, and for spreadsheet-sized data usually fastest in
  practice. Must be a stable or at least well-defined sort if the `±0` question above matters.
- **Quickselect** (Hoare's FIND), expected `O(n)`, worst case `O(n²)` on adversarial input.
  See Knuth, *The Art of Computer Programming* volume 3 §5.3.3, and Numerical Recipes §8.5,
  which presents exactly this problem under the name "selection of the M-th largest".
- **Median-of-medians** (Blum, Floyd, Pratt, Rivest, Tarjan), worst-case `O(n)`, with a
  constant large enough that it is rarely the right choice here.
- **A bounded heap of size `k`**, `O(n log k)` — the right choice when `k` is small and `n` is
  a whole column, which is the common spreadsheet shape.

A recomputation-heavy workbook that calls `LARGE(A:A, k)` for `k = 1…10` in ten cells sorts
the same column ten times unless the engine caches; that is an engine question, not a
function-semantics question, and the Handbook does not assert what Excel does.

The one genuinely numerical remark: because the result is a copy of an input value, `LARGE`
can never introduce error, and it can never *remove* it either. `LARGE` over a column of
computed values inherits whatever error those computations had, and selecting the maximum of a
noisy set is a biased operation — the expected maximum of `n` samples of a quantity plus noise
exceeds the maximum of the noiseless quantity. That is a statistics observation, not an
implementation defect, and it is the reason "take the best of `n` runs" is a poor estimator.

## What has not been checked

No evidence record lists `FUNC.LARGE` among its subjects, and no count in the Handbook's
evidence layer touches this surface. Nobody has checked `LARGE` against Excel within the
Handbook's record. No Handbook vector suite exists.

The presence projection at commit `473efa3` records `large_fn.rs` as a small, unshared module
with no upstream defect stream named on it and no module-level tests recorded. A small module
with no defects filed is not evidence of correctness; it is evidence that nobody has looked.

Inputs I would probe first, and why:

1. **`LARGE({0.3, 0.1+0.2}, 1)` and `LARGE({0.3, 0.1+0.2}, 2)`**, examining the returned bits.
   This is the cheapest probe that distinguishes an exact comparator from the tolerant
   15-digit one, and it is the question with the largest downstream consequence.
2. **`LARGE(array, {1;2;3})`** — whether an array-valued `k` lifts and spills. The reference
   engine's battery says no; modern Excel's lifting rules suggest it might. A one-cell
   experiment settles a real divergence candidate.
3. **`k` non-integer**: `1.9`, `0.5`, `−0` and `1 + 2⁻⁵²`, to pin whether truncation, rounding
   or rejection applies, and whether the check happens before or after truncation.
4. **A range with blanks and numeric text**, probed at `k = n_cells` and `k = n_numeric`, to
   confirm that `n` is the admitted count rather than the cell count. This is the failure
   users actually hit.
5. **Ties**: `LARGE({5,5,3}, 2)` — confirming multiplicity, and comparing with
   `RANK.EQ` on the same data to expose the tie-handling asymmetry.
6. **`+0` and `−0` in the same array**, at `k = 1` and `k = n`, reading the sign bit of the
   result. This is the only way `LARGE` can return "the wrong bits" while returning the right
   number.
7. **An array of exactly one value with `k = 1`**, and an all-blank range with `k = 1`, to
   separate the `#NUM!` and `#VALUE!` boundaries the reference engine's battery distinguishes.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| order statistic | `x₍ᵢ₎`, the `i`-th smallest value of the sample |
| selector | A function whose result is bit-identical to one of its inputs |
| admitted count | The number of values in `array` that survive the scan policy; the `n` in `1 ≤ k ≤ n` |
| multiplicity | Repeated values occupy separate ranks; `LARGE` does not deduplicate |
| tolerant comparison | The truncation-style 15-significant-digit equality used by some Excel families; unverified for this one |

## Sources

- Microsoft, "LARGE function" —
  <https://support.microsoft.com/en-us/office/large-function-3af0af19-1190-42bb-bb8b-01672ec00a64>
  (signature and the `#NUM!` conditions). Retrieval was blocked for this pass; the error rows
  are stated as documented behaviour with the source named.
- Knuth, *The Art of Computer Programming*, volume 3, §5.3.3 — minimum-comparison selection.
- Press, Teukolsky, Vetterling and Flannery, *Numerical Recipes*, §8.5 — "Selection of the
  M-th largest", including the heap and quickselect trade-off.
- Blum, Floyd, Pratt, Rivest and Tarjan, *Time bounds for selection* (1973) — worst-case
  linear selection.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md) — the direct-argument
  versus range-scan split, and the tolerant-comparison family split.
- Handbook projections `data/functions/FUNC.LARGE.json` (arity 2, `xlfLarge`) and
  `data/presence/FUNC.LARGE.json` (module `large_fn.rs`, unshared, no defect stream named).
