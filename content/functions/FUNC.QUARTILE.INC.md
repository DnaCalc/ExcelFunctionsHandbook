---
schema: efh.function-page/v1
function_id: FUNC.QUARTILE.INC
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — QUARTILE.INC function"
    locator: "https://support.microsoft.com/en-us/office/quartile-inc-function-1bbacc80-5075-42f1-aed6-47d735c4819d"
    role: "documented signature, the admissible quart values, and the documented #NUM! and #VALUE! conditions"
  - work: "Hyndman, R. J. and Fan, Y., \"Sample Quantiles in Statistical Packages\""
    locator: "The American Statistician 50(4), 1996, pp. 361-365"
    role: "the nine-type taxonomy of sample quantile definitions; QUARTILE.INC is their type 7"
  - work: "OxFunc — percentile_common.rs"
    locator: "crates/oxfunc_core/src/functions/percentile_common.rs"
    role: "reference-engine kernel: the inclusive rank formula, the interpolation step, and the quart truncation"
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
family: quartile_inc_fn
role_in_family: >-
  The inclusive quartile: the type-7 sample quantile evaluated at 0, 1/4, 1/2, 3/4 and 1, whose
  extreme quarts return the minimum and the maximum of the data.
---

# QUARTILE.INC

## What it computes

`QUARTILE.INC(array, quart)` returns a **sample quantile** of the data in `array`, at one of
five fixed probabilities selected by `quart`.

Sort the numeric values of `array` ascending and call them `x_(1) <= x_(2) <= … <= x_(n)`. Let
`k = quart / 4`, so `quart` in `{0, 1, 2, 3, 4}` selects `k` in `{0, 1/4, 1/2, 3/4, 1}`. Define
the **rank**

    h  =  1 + k * (n - 1)

and let `j = floor(h)`, `g = h - j`. Then

    QUARTILE.INC  =  x_(j)  +  g * ( x_(j+1) - x_(j) )

which is the value at position `h` in the sorted data, obtained by linear interpolation between
the two adjacent order statistics when `h` is not an integer.

This is exactly **type 7** in the Hyndman–Fan taxonomy of sample quantile definitions — the same
definition that is R's default `quantile()` and NumPy's default `percentile()`. Two properties
identify it:

1. **The rank runs from `1` to `n`.** At `k = 0` the rank is `1` and at `k = 1` it is `n`, so
   `QUARTILE.INC(A, 0)` is the minimum and `QUARTILE.INC(A, 4)` is the maximum, exactly. The
   whole probability interval `[0, 1]` maps onto the data, which is what the *inclusive* in the
   name means and what distinguishes it from [QUARTILE.EXC](FUNC.QUARTILE.EXC.md).
2. **It is a continuous, piecewise-linear, non-decreasing function of `k`** on `[0, 1]`, and a
   non-decreasing function of every data value. It is the linear interpolation of the inverse
   empirical distribution function evaluated at the plotting positions `(i-1)/(n-1)`.

Because it interpolates, the result need not be one of the data values. `QUARTILE.INC` of an
integer data set is routinely a fraction.

At `quart = 2` the rank is `1 + (n-1)/2 = (n+1)/2`, which is the median rank, so

    QUARTILE.INC(A, 2)  =  MEDIAN(A)

identically, for every `n`. This is one of the two quartile identities that both the inclusive
and exclusive definitions share; see [QUARTILE.EXC](FUNC.QUARTILE.EXC.md).

**There is no single correct definition of a sample quartile.** Hyndman and Fan catalogue nine
in use across statistical packages, differing in the plotting positions they assign to the order
statistics and therefore in the rank formula. The type-7 answer that `QUARTILE.INC` returns is a
defensible convention, not a theorem, and a reader comparing Excel against another package should
expect a different number rather than a bug.

## Arguments

| Argument | Meaning | Admissible |
|---|---|---|
| `array` | The data set. Required. | Any range or array; numeric cells are used |
| `quart` | Which quartile. Required. | `0, 1, 2, 3, 4` |

`quart` is a numeric slot. The reference engine coerces it to a number and then **truncates
toward zero** before testing the range, so `2.9` behaves as `2`. Microsoft documents truncation
of a non-integer `quart` for the quartile functions; the direction of that truncation, and its
behaviour for a negative fractional value such as `-0.5`, is worth confirming rather than
assuming.

`array` is consumed by scanning. Text, logical values and blank cells in a scanned range are
skipped rather than converted — they do not become zeros and they do not enlarge `n`. This is the
ordinary aggregate scan policy described in
[coercion and lifting](../model/02-coercion-and-lifting.md), and it is the policy the reference
engine's percentile collector implements.

## Result and edge cases

Returns `Number`.

- **A single data point.** `n = 1` makes the rank `1` for every `quart`, so every quartile is
  that one value. The reference engine short-circuits to the same answer.
- **`quart = 0` and `quart = 4`** return the minimum and maximum, so `QUARTILE.INC(A, 0)` is
  `MIN(A)` and `QUARTILE.INC(A, 4)` is `MAX(A)` over the numeric cells — a fact worth knowing
  mostly because it means those two quarts can never fail for a non-empty array.
- **Repeated values** are ordinary: the sort is not distinct-value, and a data set that is
  entirely one value returns that value at every `quart`.
- **Non-numeric cells in `array`** are skipped, so `n` is the count of numeric cells, not the
  size of the range. A range of text returns `#NUM!`, because `n = 0`.
- **Errors in `array`** propagate as themselves.
- **`array` given as a rectangular grid** is flattened; the quartile is over all its numeric
  cells regardless of shape.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | `array` is empty (no numeric values) | documented |
| `#NUM!` | `quart < 0` or `quart > 4` after truncation | documented |
| `#VALUE!` | `quart` is non-numeric text | shared coercion rule |
| propagated | An error value in `array` or in `quart` surfaces as that error | shared coercion rule |

The reference engine's guard is `0 <= trunc(quart) <= 4`, which agrees with the documented
condition on integers. Where the two are not obviously the same is at fractional `quart`:
`4.5` truncates to `4` and is admitted by the reference engine, while the documented condition
reads on the untruncated value. Which order Excel applies — truncate then range-check, or
range-check then truncate — is not settled by the documentation and has not been observed here.

## Relationships

- **[QUARTILE.EXC](FUNC.QUARTILE.EXC.md)** is the exclusive sibling, Hyndman–Fan type 6. It uses
  the rank `k*(n+1)` instead of `1 + k*(n-1)`, admits only `quart` in `{1, 2, 3}`, and is
  undefined on small samples. The two agree at `quart = 2` and disagree at `quart = 1` and `3`
  for essentially every data set. Neither supersedes the other; Excel publishes both because the
  statistical community uses both.
- **[QUARTILE](FUNC.QUARTILE.md)** is the legacy name, retained in the Compatibility category.
  Microsoft's replacement guidance points `QUARTILE` at `QUARTILE.INC`, and the two are
  documented as computing the same quantity. **The Handbook does not treat that as an identity.**
  A legacy alias and its modern name are two entry points that Excel is free to route to two
  different code paths; the observed agreement of a dotted name and its predecessor is a claim
  requiring evidence, not a definition. No such evidence is in the Handbook's record.
- **[PERCENTILE.INC](FUNC.PERCENTILE.INC.md)** is the general form. Mathematically
  `QUARTILE.INC(A, q)` is `PERCENTILE.INC(A, q/4)`, and the reference engine implements it as
  exactly that call. Anything true of the type-7 quantile is true of both.
- **[MEDIAN](FUNC.MEDIAN.md)** is the `quart = 2` case.
- **[SMALL](FUNC.SMALL.md) and [LARGE](FUNC.LARGE.md)** return order statistics without
  interpolation. When you want an actual data value rather than a quantile, they are the correct
  functions — `QUARTILE.INC` will happily return a number that is in no cell.
- Readers confuse `QUARTILE.INC` with `PERCENTRANK.INC`, which runs the map the other way: data
  value in, probability out.

## Numerical notes

The mathematics is exact selection plus one linear interpolation, so the error budget is small
and entirely locatable.

**The rank computation.** `h = 1 + k*(n-1)` with `k = quart/4` is computed in binary64. For
`quart` in `{0, 2, 4}` the value of `k` is exactly representable (`0`, `0.5`, `1`) and `h` is
exact whenever `n-1` is. For `quart` in `{1, 3}`, `k` is `0.25` or `0.75` — also exactly
representable, both being dyadic rationals. This is a small piece of luck specific to quartiles:
the four-way split lands on binary fractions, so the rank arithmetic for `QUARTILE.INC` is exact
for every `n` up to the point where `n-1` exceeds `2^53`. The general
[PERCENTILE.INC](FUNC.PERCENTILE.INC.md) has no such luck, because `k = 0.1` is not dyadic, and
its `floor(h)` can therefore land on the wrong side of an integer boundary. `QUARTILE.INC` does
not inherit that hazard.

**The interpolation.** The reference engine evaluates

    lower + g * (upper - lower)

rather than the algebraically equal `(1-g)*lower + g*upper`. The two forms are not
interchangeable in floating point. The first is exact at `g = 0` and, for `g = 1`, returns
`lower + (upper - lower)`, which need not be exactly `upper` when the two are far apart in
magnitude. The second form is exact at both endpoints but can round outside the interval when
`lower` and `upper` have opposite signs. Since the reference engine returns `x_(j)` directly
whenever `floor(h) == ceil(h)`, the `g = 1` case is not reached and the endpoint concern does not
arise in practice — but the choice is a real one and worth stating rather than assuming away.
The `lower + g*(upper-lower)` form is monotone in `g` and never leaves the bracket when
`lower <= upper`, which is the property that matters here.

**The sort.** Values are sorted with a total-order comparison that falls back to "equal" for
incomparable pairs. Excel's value universe has no NaN, so this is unreachable from a worksheet;
it becomes reachable if a NaN arrives from an add-in boundary, and the result is then an
unspecified ordering rather than a diagnosed error. Signed zeros compare equal, so `-0.0` and
`0.0` may appear in either order; the returned value is `0` either way, but its sign bit is not
determined by the data.

**Cost.** A full sort is `O(n log n)` where two selections would be `O(n)` by quickselect. For a
single quartile of a large column this is the dominant cost, and it is a deliberate simplicity
trade in the reference engine rather than a necessity of the definition.

## What has not been checked

No Handbook vector suite exists for `QUARTILE.INC`, and no Handbook evidence record names it as
a subject. Nobody has checked this function against Excel within the Handbook's record. The
implementing module in the reference engine carries no tests of its own; the shared percentile
kernel it calls carries a small number, which is a statement about that kernel and not about this
surface.

Everything above marked as documented comes from Microsoft's `QUARTILE.INC` page. **Retrieval of
that page was blocked by the upstream host on this pass**, so those statements are recorded as
documented behaviour with the source named and should be re-read against the live page.

Inputs worth probing first:

1. **`QUARTILE.INC({1,2,3,4}, 1)` and `QUARTILE.INC({1,2,3,4}, 3)`** — the smallest data set on
   which type 6 and type 7 give different answers, and therefore the cheapest probe that pins
   which definition Excel implements. Run the same array through
   [QUARTILE.EXC](FUNC.QUARTILE.EXC.md) in the adjacent cell.
2. **Fractional `quart`**: `1.9`, `-0.5`, and `4.5`, which separate truncate-then-check from
   check-then-truncate and pin the truncation direction for negatives.
3. **`n = 1` and `n = 2`**, at every `quart`, where the interpolation degenerates.
4. **A range containing text, a logical, and a blank cell** alongside numbers, confirming that
   `n` counts only the numeric cells.
5. **The legacy identity**: `QUARTILE(A, q)` against `QUARTILE.INC(A, q)` over a data set whose
   answer is a genuine interpolation rather than a data value, compared bitwise. This is the
   probe that would turn the documented replacement relationship into an evidenced one — and it
   is worth running precisely because nobody has.
6. **A data set whose interpolated answer is not representable**, such as thirds, to see whether
   the two argument orderings of the interpolation are distinguishable.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| order statistic | `x_(i)`, the `i`-th smallest numeric value in `array` |
| rank `h` | The real-valued position in the sorted data that the quantile is read at |
| type 7 | The Hyndman-Fan sample quantile definition `QUARTILE.INC` implements |
| inclusive | The rank spans `1` to `n`, so `k = 0` and `k = 1` reach the extremes |
| interpolation fraction `g` | The fractional part of `h`; the weight given to the upper order statistic |
| dyadic | Exactly representable in binary floating point; `1/4`, `1/2`, `3/4` all are |

## Sources

- Microsoft, *QUARTILE.INC function* —
  <https://support.microsoft.com/en-us/office/quartile-inc-function-1bbacc80-5075-42f1-aed6-47d735c4819d>
  (signature, the five admissible `quart` values, truncation of a non-integer `quart`, and the
  `#NUM!` conditions for an empty array and an out-of-range `quart`). Retrieval was blocked by
  the upstream host for this page; the documented behaviour above is stated as documented
  behaviour and should be re-checked against the page.
- R. J. Hyndman and Y. Fan, "Sample Quantiles in Statistical Packages", *The American
  Statistician* 50(4), 1996, pp. 361–365 — the nine-type taxonomy, and the source of the claim
  that `QUARTILE.INC` is type 7 and `QUARTILE.EXC` is type 6.
- OxFunc `crates/oxfunc_core/src/functions/quartile_inc_fn.rs` and
  `crates/oxfunc_core/src/functions/percentile_common.rs` at commit `473efa3` — the
  reference-engine rank formula, the interpolation form, the `quart` truncation, and the
  `0..=4` guard.
- Handbook [coercion and lifting](../model/02-coercion-and-lifting.md) — the range-scan policy
  that decides what counts toward `n`.
- Handbook `data/functions/FUNC.QUARTILE.INC.json`, `data/presence/FUNC.QUARTILE.INC.json`.
