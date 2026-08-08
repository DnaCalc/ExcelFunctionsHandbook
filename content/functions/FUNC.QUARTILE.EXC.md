---
schema: efh.function-page/v1
function_id: FUNC.QUARTILE.EXC
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — QUARTILE.EXC function"
    locator: "https://support.microsoft.com/en-us/office/quartile-exc-function-5a355b7a-840b-4a01-b0f1-f538c2864cad"
    role: "documented signature, the admissible quart values, and the documented #NUM! conditions"
  - work: "Hyndman, R. J. and Fan, Y., \"Sample Quantiles in Statistical Packages\""
    locator: "The American Statistician 50(4), 1996, pp. 361-365"
    role: "the nine-type taxonomy of sample quantile definitions; QUARTILE.EXC is their type 6"
  - work: "OxFunc — percentile_common.rs"
    locator: "crates/oxfunc_core/src/functions/percentile_common.rs"
    role: "reference-engine kernel: the exclusive rank formula, the in-range guard, and the interpolation step"
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
family: quartile_exc_fn
role_in_family: >-
  The exclusive quartile: the type-6 sample quantile, which refuses the extremes, has no
  quart 0 or 4, and is undefined on samples too small to bracket the first quartile.
---

# QUARTILE.EXC

## What it computes

`QUARTILE.EXC(array, quart)` returns a **sample quantile** of the data in `array` under the
*exclusive* convention: the one that treats the sample as an interior slice of an unseen
population, and therefore declines to identify any quantile with the observed extremes.

Sort the numeric values of `array` ascending as `x_(1) <= … <= x_(n)`, and let `k = quart/4` with
`quart` in `{1, 2, 3}`, so `k` is `1/4`, `1/2` or `3/4`. Define the **rank**

    h  =  k * (n + 1)

and, writing `j = floor(h)` and `g = h - j`,

    QUARTILE.EXC  =  x_(j)  +  g * ( x_(j+1) - x_(j) )

The `n + 1` is the whole difference from [QUARTILE.INC](FUNC.QUARTILE.INC.md), and it changes the
function's character rather than just its answer. This is **type 6** in the Hyndman–Fan taxonomy,
the definition used by Minitab and by SPSS, and it comes from assigning the order statistic
`x_(i)` the plotting position `i/(n+1)` — the expected value of the `i`-th uniform order
statistic. Under that assignment the sample never reaches probability `0` or `1`, because no
finite sample can be expected to contain the population extremes.

Three consequences follow directly from the rank formula, and all three are user-visible:

1. **There is no `quart = 0` and no `quart = 4`.** `k = 0` gives `h = 0` and `k = 1` gives
   `h = n+1`, and both lie outside `[1, n]`. The minimum and maximum are simply not quantiles
   under this convention. `QUARTILE.EXC` admits only `1`, `2`, `3`.
2. **The function is undefined on small samples.** `h = (n+1)/4 >= 1` requires `n >= 3`. With
   `n = 1` or `n = 2` the first quartile's rank falls below the first order statistic and the
   result is `#NUM!` — even though the data is perfectly well formed and
   [QUARTILE.INC](FUNC.QUARTILE.INC.md) answers happily. The mirror condition
   `3(n+1)/4 <= n` also requires `n >= 3`.
3. **`QUARTILE.EXC` is more extreme than `QUARTILE.INC` in both tails.** For the same data,
   `QUARTILE.EXC(A,1) <= QUARTILE.INC(A,1)` and `QUARTILE.EXC(A,3) >= QUARTILE.INC(A,3)`, so the
   exclusive interquartile range is the wider of the two. That is the intended behaviour: the
   exclusive convention is making an allowance for the population beyond the sample.

At `quart = 2` the two conventions coincide. The exclusive rank is `(n+1)/2` and the inclusive
rank is `1 + (n-1)/2`, which is the same number, so

    QUARTILE.EXC(A, 2)  =  QUARTILE.INC(A, 2)  =  MEDIAN(A)

for every `n >= 3`. The median is the one quartile everybody agrees on, which is exactly why it
is the one people report.

**There is no single correct definition of a sample quartile.** Hyndman and Fan catalogue nine
conventions in use. Excel publishes two of them under two names and lets the caller choose. A
reader comparing Excel against R, SAS or SPSS should expect a different number, and should check
which type the other tool used before calling anything wrong.

## Arguments

| Argument | Meaning | Admissible |
|---|---|---|
| `array` | The data set. Required. | Any range or array; numeric cells are used |
| `quart` | Which quartile. Required. | `1, 2, 3` |

`quart` is a numeric slot. The reference engine coerces it and **truncates toward zero** before
checking the range, so `1.9` behaves as `1`. Microsoft documents truncation of a non-integer
`quart` for the quartile functions; the interaction between truncation and the range check is not
settled by the documentation.

`array` is consumed by scanning: text, logical values and blank cells in a scanned range are
skipped rather than converted, so `n` is the count of *numeric* cells. See
[coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`.

- **Fewer than three numeric values.** `#NUM!`, for the structural reason above rather than
  because the data is malformed. This is the edge case that surprises people: a two-point sample
  has a perfectly good inclusive first quartile and no exclusive one.
- **`quart` outside `{1, 2, 3}`.** `#NUM!`. In particular `QUARTILE.EXC(A, 0)` and
  `QUARTILE.EXC(A, 4)` are errors, not the minimum and maximum.
- **Repeated values** are ordinary; a constant data set returns that constant at every admissible
  `quart`.
- **Non-numeric cells in `array`** are skipped and do not enlarge `n` — which means adding a text
  label to the top of a two-number column does not rescue it from `#NUM!`.
- **Errors in `array`** propagate as themselves.
- The result is an interpolated value and need not appear in the data.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | `array` is empty (no numeric values) | documented |
| `#NUM!` | `quart <= 0` or `quart >= 4` | documented |
| `#NUM!` | Fewer than three numeric values, so the rank falls outside `[1, n]` | reference engine; the small-sample condition is not stated on Microsoft's page |
| `#VALUE!` | `quart` is non-numeric text | shared coercion rule |
| propagated | An error value in `array` or in `quart` surfaces as that error | shared coercion rule |

The third row is a documentation silence rather than a contradiction. Microsoft's page states
the empty-array and out-of-range-`quart` conditions; the requirement that the sample be large
enough for the requested rank to land inside the data follows from the rank formula and is
enforced by the reference engine, but is not stated in the documentation the Handbook can cite.
It is the condition most likely to reach a real workbook, because small groups are common.

## Relationships

- **[QUARTILE.INC](FUNC.QUARTILE.INC.md)** is the inclusive sibling, Hyndman–Fan type 7 and the
  default of R and NumPy. It admits `quart` `0` through `4`, is defined for every non-empty
  sample, and returns a narrower interquartile range. The two agree only at `quart = 2`. Neither
  supersedes the other.
- **[QUARTILE](FUNC.QUARTILE.md)**, the legacy Compatibility-category name, corresponds to the
  *inclusive* convention, not this one. `QUARTILE.EXC` has no legacy predecessor: it was added
  when the dotted names arrived, because the exclusive convention was not previously available in
  Excel at all. That asymmetry is worth stating, since it means the `.INC`/`.EXC` pair is not a
  renaming of one old function into two — it is one old function plus one genuinely new one.
- **[PERCENTILE.EXC](FUNC.PERCENTILE.EXC.md)** is the general form; the reference engine
  implements `QUARTILE.EXC(A, q)` as `PERCENTILE.EXC(A, q/4)`. `PERCENTILE.EXC` inherits the same
  small-sample restriction in a sharper form: it requires `1/(n+1) <= k <= n/(n+1)`.
- **[MEDIAN](FUNC.MEDIAN.md)** is the `quart = 2` case and the point of agreement with
  `QUARTILE.INC`.
- **[SMALL](FUNC.SMALL.md) and [LARGE](FUNC.LARGE.md)** return order statistics without
  interpolation, and are what you want if the answer must be a value that actually occurs.
- Readers reach for `QUARTILE.EXC` expecting "the quartile excluding the median", which is a
  third, different convention (Tukey's hinges, Hyndman–Fan type 2-ish behaviour on odd `n`). The
  `EXC` does not refer to excluding the median; it refers to the probability interval being open
  at both ends.

## Numerical notes

**The rank arithmetic is exact.** `h = k*(n+1)` with `k` in `{1/4, 1/2, 3/4}` — all dyadic
rationals, exactly representable in binary64 — and `n+1` a small integer. The product is exact
for every `n` within range, so `floor(h)` and `h - floor(h)` are exact and the branch on
`h` integral versus fractional is never a near-miss. This is a real advantage of quartiles over
general percentiles: `PERCENTILE.EXC` with `k = 0.1` computes `0.1*(n+1)` from a `k` that is not
exactly one tenth, and its floor can therefore differ from the mathematically intended one at
values of `n` where the product lands adjacent to an integer. `QUARTILE.EXC` cannot hit that
class of bug.

**The in-range test is done on the rank, before interpolation.** The reference engine checks
`1 <= h <= n` and returns `#NUM!` otherwise. Because `h` is exact, this test has no boundary
fuzz: for `n = 3` and `quart = 1` the rank is exactly `1` and the answer is exactly `x_(1)`,
rather than being a hair below `1` and erroring.

**The interpolation** is evaluated as `lower + g*(upper - lower)` rather than
`(1-g)*lower + g*upper`. The two are algebraically equal and differ in floating point. The form
used is monotone in `g` and stays inside the bracket for `lower <= upper`, which is the property
a quantile needs; it can round short of `upper` at `g` near `1`, but the `g = 0` path returns
`x_(j)` directly so the exact-endpoint case is not routed through the interpolation at all.

**Where the answers actually differ from a competitor.** The gap between type 6 and type 7 at
`quart = 1` is `(x_(j+1) - x_(j))` scaled by a rank difference of `(n+1)/4 - (1 + (n-1)/4)`,
which is `(2 - n)/4 + …` — in words, it shrinks like `1/n`. On a large sample the two conventions
converge; on a sample of four or five they can differ by most of a gap between adjacent
observations. Every disagreement between Excel and another package about quartiles that is worth
investigating is on small `n`, and every one that is not is on large `n`.

**Sorting and cost.** A full `O(n log n)` sort where a linear-time selection would do, and a
total-order comparison that treats incomparable pairs as equal — reachable only if a NaN crosses
in from an add-in boundary, since the worksheet value universe has none.

## What has not been checked

No Handbook vector suite exists for `QUARTILE.EXC`, and no Handbook evidence record names it as a
subject. Nobody has checked this function against Excel within the Handbook's record. The
implementing module carries no tests of its own in the reference engine.

Everything above marked as documented comes from Microsoft's `QUARTILE.EXC` page. **Retrieval of
that page was blocked by the upstream host on this pass**, so those statements are recorded as
documented behaviour with the source named and should be re-read against the live page. The
small-sample `#NUM!` in particular is derived from the rank formula and observed in the reference
engine — it is not quoted from the documentation.

Inputs worth probing first:

1. **`QUARTILE.EXC({1,2}, 1)` and `QUARTILE.EXC({1,2,3}, 1)`** — the two-versus-three boundary,
   which is the cheapest probe of the undocumented small-sample restriction and the edge case a
   real workbook is most likely to hit.
2. **`QUARTILE.EXC({1,2,3,4}, 1)` against `QUARTILE.INC({1,2,3,4}, 1)`** in adjacent cells, the
   minimal experiment that separates type 6 from type 7 and confirms both names do what this page
   says.
3. **`QUARTILE.EXC(A, 0)` and `QUARTILE.EXC(A, 4)`**, confirming they are `#NUM!` and not the
   extremes — the single most common wrong expectation about this function.
4. **Fractional `quart`**: `0.5`, `3.9`, `4.0`, separating truncate-then-check from
   check-then-truncate.
5. **`quart = 2` against `MEDIAN`** over an even-sized and an odd-sized sample, compared bitwise.
   The mathematical identity is exact; whether the two code paths produce identical bits is not
   implied by it, and that is precisely the kind of question a suite exists to answer.
6. **A three-cell range containing one number and two text labels**, confirming that `n` counts
   numeric cells only and that padding a small sample with text does not change the error.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| order statistic | `x_(i)`, the `i`-th smallest numeric value in `array` |
| rank `h` | The real-valued position `k*(n+1)` in the sorted data |
| type 6 | The Hyndman-Fan sample quantile definition `QUARTILE.EXC` implements |
| exclusive | The probability interval is open at both ends; `k = 0` and `k = 1` are unreachable |
| plotting position | The probability `i/(n+1)` assigned to the `i`-th order statistic |
| small-sample restriction | The requirement `n >= 3`, which follows from `h >= 1` at `quart = 1` |

## Sources

- Microsoft, *QUARTILE.EXC function* —
  <https://support.microsoft.com/en-us/office/quartile-exc-function-5a355b7a-840b-4a01-b0f1-f538c2864cad>
  (signature, the three admissible `quart` values, and the documented `#NUM!` conditions for an
  empty array and an out-of-range `quart`). Retrieval was blocked by the upstream host for this
  page; the documented behaviour above is stated as documented behaviour and should be re-checked
  against the page.
- R. J. Hyndman and Y. Fan, "Sample Quantiles in Statistical Packages", *The American
  Statistician* 50(4), 1996, pp. 361–365 — the taxonomy, the plotting-position derivation, and
  the identification of the exclusive convention as type 6.
- OxFunc `crates/oxfunc_core/src/functions/quartile_exc_fn.rs` and
  `crates/oxfunc_core/src/functions/percentile_common.rs` at commit `473efa3` — the exclusive
  rank formula, the `1 <= h <= n` guard, the `1..=3` `quart` range, and the truncation.
- Handbook [coercion and lifting](../model/02-coercion-and-lifting.md) — the range-scan policy
  that decides what counts toward `n`.
- Handbook `data/functions/FUNC.QUARTILE.EXC.json`, `data/presence/FUNC.QUARTILE.EXC.json`.
