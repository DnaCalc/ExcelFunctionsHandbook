---
schema: efh.function-page/v1
function_id: FUNC.SMALL
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — SMALL function"
    locator: "https://support.microsoft.com/en-us/office/small-function-17da8222-7c82-42b2-961b-14c45384df07"
    role: "documented signature and the documented #NUM! conditions for an empty array and an out-of-range k"
  - work: "Blum, M., Floyd, R. W., Pratt, V., Rivest, R. L. and Tarjan, R. E., \"Time Bounds for Selection\""
    locator: "Journal of Computer and System Sciences 7(4), 1973, pp. 448-461"
    role: "linear-time selection, the algorithmic alternative to sorting for an order statistic"
  - work: "OxFunc — small_fn.rs"
    locator: "crates/oxfunc_core/src/functions/small_fn.rs"
    role: "reference-engine kernel: the k truncation, the count guard, and the full sort"
episodes: []
body_sections:
  - "What it computes"
  - "Arguments"
  - "Result and edge cases"
  - "Errors"
  - "Relationships"
  - "Numerical notes"
  - "What has not been checked"
  - "Page vocabulary"
  - "Sources"
family: small_fn
role_in_family: >-
  The ascending order statistic: returns the k-th smallest numeric value in a data set, the
  ascending counterpart of LARGE and the general form of MIN.
---

# SMALL

## What it computes

`SMALL(array, k)` returns the **`k`-th order statistic** of the numeric values in `array`,
counting from the bottom.

Let the numeric values of `array`, sorted ascending, be `x_(1) <= x_(2) <= … <= x_(n)`. Then

    SMALL(array, k)  =  x_(k)

for `1 <= k <= n`, and `#NUM!` otherwise.

This is selection, not estimation. Unlike [PERCENTILE.INC](FUNC.PERCENTILE.INC.md) or
[QUARTILE.INC](FUNC.QUARTILE.INC.md), `SMALL` performs no interpolation: the result is always a
value that actually occurs in the data, returned unchanged. That property is the reason to reach
for it — when a report must cite a real observation rather than a computed quantile, `SMALL` is
the function that guarantees it.

The two ends are the familiar aggregates:

    SMALL(A, 1)  =  MIN(A)          SMALL(A, n)  =  MAX(A)

and the ascending/descending duality with [LARGE](FUNC.LARGE.md) is exact:

    SMALL(A, k)  =  LARGE(A, n - k + 1)

where `n` is the count of *numeric* values, which is `COUNT(A)` and not the size of the range.
That last qualification is where the identity is usually got wrong in practice: a range with text
labels in it has an `n` smaller than its cell count, and a formula written as
`LARGE(A, ROWS(A)-k+1)` silently indexes the wrong statistic.

**Ties are not deduplicated.** `SMALL` indexes the sorted multiset, not the sorted set of distinct
values. On `{5, 5, 5}`, `SMALL(A, 2)` is `5`, not an error and not the second distinct value.
Readers wanting the second *distinct* smallest need a different construction entirely.

`SMALL` is monotone non-decreasing in `k`, and monotone non-decreasing in each data value — it is
an order-preserving selector, which is what makes it safe to compose with itself.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `array` | The data set. | yes |
| `k` | The position from the bottom, 1-based. | yes |

`array` is consumed by scanning. Text, logical values and blank cells reached through a range or
array are skipped rather than converted — they do not become zeros and they do not enlarge `n`.
Values passed directly as scalars go through ordinary to-number coercion under the dual policy of
[coercion and lifting](../model/02-coercion-and-lifting.md).

`k` is a numeric slot. The reference engine coerces it and then **truncates toward zero** before
validating, so `k = 2.9` selects the second smallest and `k = 0.5` truncates to `0` and errors.
Microsoft's page states the `#NUM!` condition on `k <= 0` and on `k` exceeding the number of data
points; it does not state the truncation rule for a fractional `k`, so the truncation is recorded
here as reference-engine behaviour that has not been compared against Excel.

## Result and edge cases

Returns `Number` — one of the values from `array`, returned unchanged.

- **`array` contains no numeric values.** `n = 0`, so every `k` fails: `#NUM!`. A range holding
  only text produces `#NUM!`, not `#VALUE!`.
- **`k` greater than the count of numeric values.** `#NUM!`. Note again that the count is of
  numeric cells, so adding a header row to a range does not increase the admissible `k`.
- **`k` less than 1 after truncation.** `#NUM!`.
- **Repeated values** are indexed as they occur; `n` counts them all.
- **The result is the stored value, not a rounded one.** `SMALL` never computes; whatever double
  was in the cell is what comes back, including its exact bit pattern.
- **Errors in `array`** propagate as themselves.
- **`array` given as a rectangular grid** is flattened; shape is irrelevant.
- **`SMALL` returns a scalar, not a spilled array**, even when `k` is array-valued — whether
  Excel lifts over an array-valued `k` in a modern build is a lifting question this page does not
  settle.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | `array` is empty of numeric values | documented |
| `#NUM!` | `k <= 0` | documented |
| `#NUM!` | `k` exceeds the number of numeric data points | documented |
| `#VALUE!` | `k` is non-numeric text | shared coercion rule |
| propagated | An error value in `array` or in `k` surfaces as that error | shared coercion rule |

## Relationships

- **[LARGE](FUNC.LARGE.md)** is the descending counterpart, with the exact identity
  `SMALL(A,k) = LARGE(A, COUNT(A)-k+1)`. The two are the same selection with the comparison
  reversed.
- **[MIN](FUNC.MIN.md)** is `SMALL(A, 1)` and **[MAX](FUNC.MAX.md)** is `LARGE(A, 1)`. `MIN` and
  `MAX` are the ones to use for the extremes — they need no count and no `k`, and they do not
  error on an empty range (they return `0`), which is a behavioural difference worth knowing when
  a range might be empty.
- **[MEDIAN](FUNC.MEDIAN.md)** is `SMALL(A, (n+1)/2)` for odd `n` and the mean of the two middle
  order statistics for even `n`.
- **[QUARTILE.INC](FUNC.QUARTILE.INC.md) and [PERCENTILE.INC](FUNC.PERCENTILE.INC.md)** are the
  interpolating relatives. They return quantiles, which may be values that occur nowhere in the
  data; `SMALL` returns observations. Choosing between them is a choice about what the number is
  supposed to mean, not a choice of formula.
- **[RANK.EQ](FUNC.RANK.EQ.md)** is the inverse map: `LARGE(ref, RANK.EQ(x, ref))` recovers `x`,
  because competition rank and the descending order-statistic index coincide. The ascending
  version composes with `RANK.EQ(x, ref, 1)`.
- **[SORT](FUNC.SORT.md)** and **[SORTBY](FUNC.SORTBY.md)** return the whole ordering as a spilled
  array. Where several order statistics are wanted at once, one `SORT` beats several `SMALL`
  calls by an order of magnitude in work.
- The idiom `SMALL(IF(cond, range), k)` is the classic way to get the `k`-th smallest among rows
  meeting a condition, and depends on the `FALSE` branch producing values `SMALL` skips.
- Readers confuse `SMALL` with `MINIFS`, which returns the minimum under criteria — the `k = 1`
  case of the conditional idiom, with the condition expressed properly rather than through an
  array formula.

## Numerical notes

`SMALL` performs no arithmetic on the data. It sorts and it indexes, so there is **no rounding
error in the result at all** — the returned value is bit-identical to the input value that was
selected. That makes it one of the few statistical functions whose correctness is entirely a
question of ordering and admissibility rather than of accuracy. Three ordering questions
nonetheless deserve attention.

**Signed zeros.** IEEE-754 defines `-0.0 == +0.0` as true and neither as less than the other, so
a data set containing both admits two valid sorted orders and `SMALL` may return either zero. The
values compare equal and print identically, so this is invisible in a spreadsheet — but it is
visible to any comparison of raw bits, and a vector suite comparing bit patterns must decide
whether it cares. The mathematically honest position is that `SMALL` is well defined as a
multiset selection and under-determined as a bit selection.

**NaN.** The reference engine sorts with a comparison that falls back to "equal" for incomparable
pairs, which means a NaN in the data produces an unspecified ordering rather than a diagnosed
error and can displace a genuine value from position `k`. Excel's worksheet value universe has no
NaN, so this is unreachable from a formula; it becomes reachable if a NaN crosses in from an
add-in boundary. It is worth recording because "unreachable today" is not the same as "cannot
happen", and the failure is silent when it does.

**Stability and cost.** The kernel performs a **full sort**, `O(n log n)`, to answer a question
that quickselect answers in expected `O(n)` — the classical result of Blum, Floyd, Pratt, Rivest
and Tarjan (1973) gives a worst-case linear bound. For a single `SMALL` over a large column the
sort is the entire cost of the call, and a column of `SMALL` formulas over the same range re-sorts
the same data once per formula. That is a performance property rather than a correctness one, but
it is the reason a `k`-th-smallest column is slow and a `SORT` spill is not, and no amount of
formula rewriting inside `SMALL` fixes it.

**The truncation of `k`.** `trunc(k)` before validation means the admissible interval for `k` is
the half-open `[1, n+1)` rather than the closed `[1, n]`: `k = n + 0.5` truncates to `n` and
succeeds. Nothing documents this, and the alternative — validate first, then truncate — would
reject it. Which one Excel does is a one-cell experiment nobody has run.

## What has not been checked

No Handbook vector suite exists for `SMALL`, and no Handbook evidence record names `SMALL` as a
subject. Nobody has checked this function against Excel within the Handbook's record. The
implementing module carries no tests of its own in the reference engine.

Everything above marked as documented comes from Microsoft's `SMALL` page. **Retrieval of that
page was blocked by the upstream host on this pass**, so those statements are recorded as
documented behaviour with the source named and should be re-read against the live page.

Inputs worth probing first:

1. **Fractional `k`**: `SMALL({10,20,30}, 2.9)` and `SMALL({10,20,30}, 3.5)`. The first pins the
   truncation, the second separates truncate-then-validate from validate-then-truncate. Two cells
   answer both, and both behaviours are undocumented.
2. **`k = 0`, `k = -1`, and `k = n+1`**, the three documented `#NUM!` boundaries.
3. **A range mixing numbers with text, a logical and a blank cell**, probing whether `n` counts
   only the numbers — and then `SMALL(A, n)` and `SMALL(A, n+1)` on that range, which is where a
   miscount shows.
4. **`SMALL(A,k)` against `LARGE(A, COUNT(A)-k+1)`** over the same mixed range, bitwise. The
   identity is exact; a disagreement means the two functions disagree about `n`.
5. **A tied data set** such as `{5,5,5}` at `k = 1, 2, 3`, confirming that ties are indexed rather
   than deduplicated.
6. **Both zeros**: a range containing `-0` and `0`, checking which bit pattern comes back at
   `k = 1` — the only genuinely under-determined case in the function.
7. **An array-valued `k`**, to establish whether a modern Excel lifts `SMALL` and spills.
8. **A range of only text**, confirming `#NUM!` and not `#VALUE!`.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| order statistic | `x_(k)`, the `k`-th smallest numeric value in `array` |
| selection | Returning an observed value by rank, with no interpolation |
| multiset | The data with repeats kept; `SMALL` indexes this, not the distinct values |
| numeric count `n` | `COUNT(array)`, not the number of cells |
| quickselect | Linear-time selection, the algorithmic alternative to sorting |

## Sources

- Microsoft, *SMALL function* —
  <https://support.microsoft.com/en-us/office/small-function-17da8222-7c82-42b2-961b-14c45384df07>
  (signature, and the `#NUM!` conditions for an empty array, for `k <= 0` and for `k` exceeding
  the number of data points). Retrieval was blocked by the upstream host for this page; the
  documented behaviour above is stated as documented behaviour and should be re-checked against
  the page.
- M. Blum, R. W. Floyd, V. Pratt, R. L. Rivest and R. E. Tarjan, "Time Bounds for Selection",
  *Journal of Computer and System Sciences* 7(4), 1973, pp. 448–461 — worst-case linear
  selection, the algorithm the sort-based implementation forgoes.
- IEEE Std 754-2019, clause 5.11 (comparison predicates) — the signed-zero equality and the
  unordered outcome for NaN that make the ordering under-determined in the two cases named above.
- OxFunc `crates/oxfunc_core/src/functions/small_fn.rs` at commit `473efa3` — the reference-engine
  kernel: the `trunc(k)` and `k < 1` guard, the count check, the full sort, and the direct index.
- Handbook [coercion and lifting](../model/02-coercion-and-lifting.md) — the range-scan policy
  that decides what counts toward `n`, and the direct-argument contrast.
- Handbook `data/functions/FUNC.SMALL.json`, `data/presence/FUNC.SMALL.json`.
