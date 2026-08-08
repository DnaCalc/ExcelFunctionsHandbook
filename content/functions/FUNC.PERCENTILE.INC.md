---
schema: efh.function-page/v1
function_id: FUNC.PERCENTILE.INC
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "R. J. Hyndman and Y. Fan, Sample quantiles in statistical packages"
    locator: "The American Statistician 50 (1996) 361-365; definition 7"
    role: "The taxonomy of nine sample-quantile definitions; this function is their type 7"
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
family: percentile_inc_fn
role_in_family: >-
  The inclusive sample quantile: Hyndman and Fan's type 7, admitting k = 0 and k = 1 and
  returning the sample minimum and maximum there.
---

# PERCENTILE.INC

## What it computes

`PERCENTILE.INC(array, k)` returns the `k`-th sample quantile of the data, with `k` expressed
as a fraction in `[0, 1]` and with linear interpolation between order statistics.

Let `x₍₁₎ ≤ x₍₂₎ ≤ … ≤ x₍ₙ₎` be the admitted values sorted ascending. Define the **rank
position**

    h = 1 + k·(n − 1)                    so   h ∈ [1, n]

and split it as `h = j + g` with `j = ⌊h⌋` an integer and `g = h − j ∈ [0, 1)`. Then

    PERCENTILE.INC(array, k) = x₍ⱼ₎ + g · ( x₍ⱼ₊₁₎ − x₍ⱼ₎ )

with the convention that `g = 0` returns `x₍ⱼ₎` outright, which covers `h = n` where `x₍ⱼ₊₁₎`
does not exist.

Microsoft's page states the interpolation rule in terms of the same `n − 1`: "If `k` is not a
multiple of `1/(n − 1)`, `PERCENTILE.INC` interpolates to determine the value at the `k`-th
percentile." Those multiples are exactly the `k` values at which `h` lands on an integer, i.e.
`k = (i−1)/(n−1)` for `i = 1 … n` — so **the `n` data points sit at equally spaced `k` values,
with the minimum at `k = 0` and the maximum at `k = 1`.**

**This is Hyndman and Fan's type 7**, the "inclusive" definition, and it is the default in R,
NumPy, Julia and most modern statistical software. Naming it matters: there are at least nine
defensible definitions of a sample quantile, they disagree on the same data, and every argument
about "the wrong median" or "the wrong quartile" between two systems reduces to which of the
nine each one implements. Hyndman and Fan (1996) is the paper that catalogued them.

**Domain and range.** `n ≥ 1` admitted values; `k ∈ [0, 1]` inclusive at both ends — the
inclusive part of the name. The result lies in `[x₍₁₎, x₍ₙ₎]` and is a non-decreasing, piecewise
linear, continuous function of `k`. Special values:

    k = 0     → x₍₁₎ = MIN(array)
    k = 1     → x₍ₙ₎ = MAX(array)
    k = 0.5   → MEDIAN(array)          for every n, odd or even
    n = 1     → the single value, for every admissible k

The `k = 0.5` identity is exact and general: at `h = 1 + (n−1)/2 = (n+1)/2`, odd `n` lands on
the middle order statistic and even `n` lands exactly halfway between the two middle ones,
which is Excel's `MEDIAN`. That makes `MEDIAN` a redundant special case of this function and a
free cross-check.

**Why an interpolated quantile at all.** The empirical distribution function is a step
function, so its inverse is not unique between order statistics; every sample-quantile
definition is a rule for choosing a point in that gap. Type 7 chooses the linear interpolant of
the order statistics against the plotting positions `p_i = (i−1)/(n−1)`. It is the estimator
that makes the sample quantile function the piecewise-linear interpolant through the data
points themselves — which is why its endpoints are the observed minimum and maximum and why it
cannot extrapolate beyond the observed range.

## Arguments

Microsoft's page gives two required arguments:

| Argument | Meaning | Required |
|---|---|---|
| `array` | "The array or range of data that defines relative standing" | yes |
| `k` | "The percentile value in the range 0 to 1, inclusive" | yes |

`k` is a fraction, not a percentage: the 90th percentile is `k = 0.9`, and `k = 90` is a
`#NUM!`. This is the most common user error with this function and, mercifully, a loud one.

The reference engine records an arity of exactly 2, a `Custom` kernel signature and a `Custom`
coercion/lift profile — the admission of text, logicals and empty cells in the array is this
function's own decision. See [Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`.

- **`n = 1`.** The rank position is `h = 1 + k·0 = 1` for every `k`, so the single value is
  returned for all admissible `k`. This is well defined here, and it is exactly the case where
  the exclusive sibling has nothing to return — see
  [PERCENTILE.EXC](FUNC.PERCENTILE.EXC.md).
- **`k = 0` and `k = 1`** are admissible and give the sample extremes. The reference engine's
  battery renders the both-zero row; its outcome shows beside this page.
- **Repeated values** are handled by the ordering with no special case; the interpolation simply
  has a zero-width gap.
- **The result need not be a data value.** For most `k` it is a weighted average of two
  observations, so `PERCENTILE.INC` of an integer data set is generally not an integer. Users
  expecting "one of my numbers" want a different definition (`SMALL`, or the nearest-rank
  convention, which Excel does not provide).
- **Sorting cost.** The function is a selection problem, not a sort problem; see Numerical
  notes.
- **Arrays.** An aggregate over a grid, not a lift kernel; returns a scalar.

## Errors

As documented by Microsoft on the `PERCENTILE.INC` page (retrieved for this page):

| Error | Documented condition |
|---|---|
| `#NUM!` | `array` is empty |
| `#VALUE!` | `k` is nonnumeric |
| `#NUM!` | `k < 0` or `k > 1` |

Note the boundary: `k = 0` and `k = 1` are **admissible**, which is precisely what distinguishes
this function from `PERCENTILE.EXC`. Note also that the documentation says nothing about an
array containing no *numeric* values (as opposed to being empty), which under the ignore-rule is
a distinct situation, and nothing about error values inside the array.

Error values in either argument propagate under the ordinary coercion rules.

## Relationships

- **`PERCENTILE`** is the legacy spelling, with the same argument count in the reference
  engine's registry — two. This modernization is a **pure rename at the signature level**, and
  `PERCENTILE` is documented as the inclusive definition, so `PERCENTILE` and `PERCENTILE.INC`
  are described as the same computation. **Same description is not demonstrated same bits.**
  Excel dispatches compatibility functions as their own surfaces, the reference engine puts them
  in different implementation modules, and no evidence record in the Handbook names either
  surface. The identity is an expectation, not a finding.
- **[PERCENTILE.EXC](FUNC.PERCENTILE.EXC.md)** is the exclusive definition — Hyndman and Fan's
  type 6 — with rank position `h = k(n+1)` instead of `1 + k(n−1)`, a restricted domain, and no
  `k = 0` or `k = 1`. **The two functions return different numbers for the same data and the
  same `k`**, and neither is wrong; they estimate the population quantile under different
  plotting-position conventions. Choosing between them is a statistical decision, and the
  Handbook's position is that a page which pretends otherwise is not being useful.
- **`QUARTILE.INC`** is this function restricted to `k ∈ {0, 0.25, 0.5, 0.75, 1}`:
  `QUARTILE.INC(array, q) = PERCENTILE.INC(array, q/4)`. A free consistency identity between two
  Excel surfaces.
- **`MEDIAN`** is `PERCENTILE.INC(array, 0.5)`, exactly, for every `n`.
- **`MIN`** and **`MAX`** are the `k = 0` and `k = 1` endpoints.
- **[PERCENTRANK.INC](FUNC.PERCENTRANK.INC.md)** is the inverse direction — value in, fraction
  out — and the two are *not* exact inverses of one another, because `PERCENTRANK.INC` truncates
  its result to a fixed number of significant digits by default. That asymmetry is documented on
  that page and is a real trap.
- **`SMALL`** and **`LARGE`** return order statistics without interpolation:
  `SMALL(array, i) = x₍ᵢ₎`. When the answer must be one of the observed values, these are the
  functions.
- **`TRIMMEAN`** and **`AGGREGATE`** are the neighbouring order-statistic consumers.

## Numerical notes

The arithmetic here is a comparison sort and one linear interpolation, so there is no special
function and no series — and the subtleties are correspondingly sharp rather than deep.

**1. The interpolation form is not neutral.** Two algebraically identical expressions:

    A:  x_j + g·(x_{j+1} − x_j)
    B:  (1 − g)·x_j + g·x_{j+1}

Form **A** is exact at `g = 0` and never leaves the interval, but loses precision when
`x_{j+1} − x_j` cancels badly against large `x_j` — the classic case of two nearly equal large
numbers. Form **B** is exact at `g = 1` and is symmetric, but can produce a result **outside**
`[x_j, x_{j+1}]` by an ULP when `g` is not exactly representable, which breaks monotonicity.
The safe construction is form A with a guard, or the `fma`-based
`fma(g, x_{j+1} − x_j, x_j)`. This is not a hypothetical: quantile functions that are
non-monotone in `k` by one ULP are a real and reported class of bug.

**2. `h = 1 + k(n−1)` is a rounding decision in disguise.** For `k` a "nice" decimal like
`0.1`, `k(n−1)` is generally not an integer even when it mathematically should be — `0.1` is not
representable, so `0.1 × 10` need not be exactly `1`. The consequence is that a `k` intended to
land exactly on a data point can instead produce `g = 1e−17`, giving an interpolated answer a
few ULP off the data value rather than the data value itself. **This is the single most likely
source of a "wrong last digit" complaint about this function**, and it is a property of decimal
input in binary arithmetic rather than of the algorithm. An implementation can mitigate it by
computing `j` and `g` from an exactly-rounded product and snapping `g` to `0` or `1` within a
tolerance — which is itself a decision to document, since it changes answers.

**3. Selection, not sorting.** The answer needs only `x₍ⱼ₎` and `x₍ⱼ₊₁₎`, so a full sort is
`O(n log n)` where `O(n)` selection (quickselect, introselect) suffices. For a single `k` on a
large range that is the right algorithm; for many `k` on the same data, sorting once wins. This
matters in a spreadsheet where a column of percentiles over the same range is the common idiom.

**4. Ties and stability are irrelevant here and matter next door.** Because the result depends
only on the *values* at positions `j` and `j+1`, no tie-breaking rule is needed and the sort
need not be stable. That is a genuine simplification relative to `MODE.SNGL` and `RANK.EQ`,
where the tie rule is observable.

**5. Comparison semantics.** Sorting requires a total order on the admitted values. If the
admitted set is all-numeric this is IEEE 754 ordering with the usual caveats about `−0.0`
(compares equal to `0.0`, so either may be reported) and about whether a tolerant comparison is
in play anywhere. Excel's tolerant-versus-exact comparison split is recorded upstream in
`BUG-FUNC-004` for the lookup families; which side an order-statistic function falls on has not
been established here.

## What has not been checked

No Handbook vector suite exists for `PERCENTILE.INC`, and **no Handbook evidence record names
this surface**. Nobody has compared this function against Excel within the Handbook's record.

The definition above — the type 7 rank position `h = 1 + k(n−1)` and linear interpolation — is
grounded in Microsoft's own documented remark about multiples of `1/(n−1)`, which pins the
denominator and therefore the definition. The interpolation *form*, the rounding behaviour of
`h`, and the treatment of a non-empty array containing no numbers are not documented anywhere
the Handbook has found.

Microsoft's documented behaviour above was retrieved from the `PERCENTILE.INC` page.

Inputs worth probing first:

1. **`PERCENTILE.INC(array, 0)` and `(array, 1)` against `MIN` and `MAX`**, and
   **`(array, 0.5)` against `MEDIAN`**, for odd and even `n`. Three exact identities between
   Excel surfaces, no oracle needed, and any failure is immediately a finding.
2. **`k` at exact data positions**: with `n = 11`, evaluate at `k = 0.1, 0.2, … 0.9` and compare
   against `SMALL(array, i)`. Every one of these should return an observed value exactly. The
   ones that do not expose the `h`-rounding effect of Numerical notes point 2, and this is the
   cheapest way to see it.
3. **Monotonicity in `k`**: step `k` by one ULP across an interpolation interval and confirm the
   result never decreases. This detects the form-B overshoot of point 1.
4. **Large, close data values** — `{1e16, 1e16+2, 1e16+4}` at `k = 0.25` — where the two
   interpolation forms visibly differ.
5. **`n = 1`**, at `k = 0`, `0.5` and `1`, confirming the constant result and contrasting with
   `PERCENTILE.EXC` on the same input.
6. **`PERCENTILE.INC` against `PERCENTILE`** on every probe above — the legacy pairing, which
   nothing in the Handbook's record establishes.
7. **`PERCENTILE.INC(array, q/4)` against `QUARTILE.INC(array, q)`** for `q = 0…4` — two
   surfaces, one number.
8. **An array of text and logicals with a few numbers**, and the same values as direct array
   constants rather than as a reference — the direct-versus-scan asymmetry, which the `Custom`
   coercion profile means cannot be predicted from another function.
9. **An error value inside the array**, and an array with no numeric values at all — neither
   documented.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| rank position | `h = 1 + k(n−1)`; the fractional index into the sorted data |
| type 7 | Hyndman and Fan's definition 7, the inclusive convention this function implements |
| plotting position | The `k` value assigned to the `i`-th order statistic; here `(i−1)/(n−1)` |
| order statistic | `x₍ᵢ₎`, the `i`-th smallest admitted value |
| interpolation form | The choice between `x_j + g·Δ` and `(1−g)x_j + g·x_{j+1}`; not equivalent in floating point |

## Sources

- Microsoft, "PERCENTILE.INC function" —
  <https://support.microsoft.com/en-us/office/percentile-inc-function-680f9539-45eb-410b-9a5e-c1355e5fe2ed>
  (syntax; both required arguments; `k` in the range 0 to 1 inclusive; the `#NUM!` conditions
  for an empty array and for `k < 0` or `k > 1`; the `#VALUE!` condition for a nonnumeric `k`;
  and the remark that the function interpolates when `k` is not a multiple of `1/(n − 1)`, which
  pins the definition). Retrieved for this page.
- R. J. Hyndman and Y. Fan, "Sample quantiles in statistical packages", *The American
  Statistician* 50 (1996) 361–365 — the taxonomy of nine sample-quantile definitions;
  `PERCENTILE.INC` is their type 7.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [About implementation options](../model/07-implementation-options.md).
- OxFunc bug stream `docs/bugs/streams/BUG-FUNC-004_numeric_comparison_tolerance_family_split.md`
  — the tolerant-versus-exact comparison split, cited as context for the ordering question, not
  as evidence about this function.
- `data/functions/FUNC.PERCENTILE.INC.json` (arity 2, `Custom` kernel signature and coercion
  profile, XLL symbol `xlfPercentile_inc`) and `data/functions/FUNC.PERCENTILE.json` (arity 2) —
  the source of the unchanged-signature observation.
- `data/presence/FUNC.PERCENTILE.INC.json` — implementing module
  `crates/oxfunc_core/src/functions/percentile_inc_fn.rs`, shared with no other surface; note
  that the reference engine gives the legacy `PERCENTILE` a different module.
