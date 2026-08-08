---
schema: efh.function-page/v1
function_id: FUNC.TRIMMEAN
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
family: moment_stats_family
role_in_family: >-
  The robust location estimator of the moment-statistics module: an order-statistic function
  among moment functions, and the family's only member whose answer depends on a sort.
---

# TRIMMEAN

## What it computes

`TRIMMEAN(array, percent)` returns the **trimmed mean** — the arithmetic mean of the data after
discarding an equal number of the largest and smallest values.

The trimming rule is the part that needs stating precisely, because it is not "discard
`percent` of the data". Microsoft documents it in two sentences: `percent` is "the fractional
number of data points to exclude from the calculation", and "`TRIMMEAN` rounds the number of
excluded data points down to the nearest multiple of 2". So with `n` numeric values:

    total excluded  =  2 · floor( n · percent / 2 )
    excluded per end,  k  =  floor( n · percent / 2 )
    result = mean of the n − 2k values remaining after sorting

The documented example fixes the reading: with `percent = 0.2` and 20 points, four points are
trimmed — two from the top and two from the bottom.

The rounding-down-to-an-even-count rule exists to keep the trim **symmetric**. An odd excluded
count would have to remove one more value from one end than the other, which would bias the
estimator; Excel would rather trim slightly less than trim asymmetrically. This is the right
choice, and it means the effective trimming fraction is generally *below* the requested
`percent` — a fact worth knowing before comparing `TRIMMEAN` results against a statistics
package that rounds differently. There is no universal convention here: implementations
variously round the per-end count down, round it to nearest, or interpolate between order
statistics, and they disagree on the same data by construction rather than by accident.

As an estimator, `TRIMMEAN` interpolates between two familiar ones:

| `percent` | Result |
|---|---|
| 0 | `AVERAGE(array)` — no trimming |
| small | A robust mean: bounded influence from the tails |
| large, `n` odd | approaches `MEDIAN(array)` |

Its **breakdown point** is the fraction trimmed from each end, `k/n ≈ percent/2`: that is the
proportion of arbitrarily corrupted observations the estimator tolerates before its value can be
driven anywhere. The mean has breakdown point 0; the median has ½. `TRIMMEAN` lets the caller
choose a point on that line, which is the whole reason robust location estimators exist (Tukey;
Huber, *Robust Statistics*; Hampel, Ronchetti, Rousseeuw & Stahel).

Note also what it is **not**: it is not a Winsorized mean, which replaces the extreme values with
the nearest retained value rather than discarding them, and which therefore has a different
variance and a different influence function. Excel offers no Winsorized mean.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `array` | "The array or range of values to trim and average." | Yes |
| `percent` | "The fractional number of data points to exclude from the calculation." | Yes |

Exactly two arguments; the projection records an arity of exactly two.

The projection records a **by-index scalar array lift on the second position** — `percent` lifts
elementwise while `array` is consumed whole — and marks that axis `excel-claimed` rather than
defaulted. So an array of trimming fractions spills one trimmed mean per fraction, which is
genuinely useful for sensitivity analysis. The reference engine's battery beside this page does
lift that way. The surface's module also carries the open upstream defect stream `BUG-FUNC-018`,
whose title names a scalar-parameter array-lift gap; the Handbook cites it by name only.

Microsoft's page says **nothing** about how text, logical values and empty cells inside `array`
are treated, and nothing about the direct-argument-versus-range-scan split that the neighbouring
statistical pages document at length. The reference engine classifies this surface under the
aggregate dual policy — direct arguments coerced, scanned ranges filtered — but that is the
engine's classification, not a documented Excel fact. See
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`.

- **`percent = 0`** trims nothing and equals `AVERAGE(array)`. This should be an exact identity
  between two Excel surfaces, including in the last bits if both sum the same way — and it is
  therefore a good probe of whether `TRIMMEAN` reuses `AVERAGE`'s accumulation.
- **`percent` small enough that `n · percent < 2`** also trims nothing, because the floor of a
  quantity below 2, halved, is zero. So on a ten-point data set every `percent` below `0.2`
  behaves identically to `percent = 0`. That is a large flat region that surprises people who
  expect `percent = 0.1` to do something.
- **`percent = 1`** is documented as admissible — the error condition is `percent < 0` or
  `percent > 1` — and it is the interesting corner. It asks for `n` points to be excluded, which
  rounds down to the largest even number not exceeding `n`. For **odd** `n` that leaves exactly
  one value, the median. For **even** `n` it leaves **nothing at all**, and the mean of an empty
  set is undefined. The documentation admits the input and does not say what comes out. The
  reference-engine battery beside this page happens to exercise only the single-value odd case,
  where it returns that value. The even case is the probe.
- **Ties at the trimming boundary.** Trimming is by position in the sorted order, not by value,
  so when several observations tie at the cut point it does not matter which copies are removed —
  the retained multiset, and therefore the mean, is the same. This is a real robustness of the
  definition and it is worth knowing that no tie-breaking rule is needed.
- **`n = 1`** with any admissible `percent` retains the single value.
- **Sort order and non-finite values.** The definition depends on an ordering of the data. What
  happens with values that do not order conventionally, and how blanks and text interact with the
  sort, is undocumented.
- **`array` empty** is undocumented; the engine's battery reports a value error for an empty
  range.

## Errors

As documented on Microsoft's `TRIMMEAN` page:

| Error | Documented condition |
|---|---|
| `#NUM!` | `percent < 0` or `percent > 1`. |

That is the whole documented table. Undocumented, and therefore open: an empty `array`; the
`percent = 1` even-`n` case that leaves no data; non-numeric contents of `array`; and a
nonnumeric `percent`.

Errors in either argument propagate under the universal coercion rule. The Handbook has not
verified any of this against Excel.

## Relationships

- **`AVERAGE`** is the `percent = 0` case and the estimator `TRIMMEAN` is a robustification of.
- **`MEDIAN`** is the limit as the trimming approaches everything, for odd `n`. `TRIMMEAN` with a
  large `percent` and `MEDIAN` should agree exactly on odd-length data whenever the trim leaves a
  single value.
- **`AVERAGEIF` / `AVERAGEIFS`** trim by *predicate* rather than by rank; they are the other way
  to exclude outliers and they are not robust in the breakdown-point sense, because a predicate
  chosen from the data is itself corruptible.
- **`QUARTILE.EXC` / `PERCENTILE.INC` / `SMALL` / `LARGE`** are the other order-statistic
  surfaces, and the interpolation conventions they disagree about are the same family of
  convention disagreements that make `TRIMMEAN` non-portable across statistics packages.
- **`STEYX`, `SKEW`, `SKEW.P`, `KURT`** share this surface's implementing module in the reference
  engine — a module of moment statistics with one order statistic sitting inside it.
- **Confused with**: a Winsorized mean (see above), and with "remove outliers then average", which
  is a data-dependent rule with no fixed breakdown point.

## Numerical notes

`TRIMMEAN` is the rare statistical surface whose difficulty is **combinatorial rather than
floating-point**. The value is a mean of a subset, so once the subset is chosen the arithmetic is
ordinary; almost everything that can go wrong goes wrong in choosing it.

**Selection, not sorting.** A full sort is `O(n log n)` and is the obvious implementation, but the
function only needs the `k`-th smallest and `k`-th largest cut points; a two-sided quickselect
(Hoare's algorithm, or `nth_element`) does the job in expected linear time, and introselect
guarantees it. For the array-lifted case — many `percent` values against one `array` — sorting
once and reusing the order is obviously right, and an implementation that re-selects per element
does needless work.

**The count arithmetic is integer arithmetic and should stay that way.** `floor(n · percent / 2)`
computed in floating point can land on the wrong integer when `n · percent` is within rounding of
an even number: `n = 20`, `percent = 0.2` gives `n·percent = 4.000000000000001` or
`3.9999999999999996` depending on how the product is formed, and the second rounds down to a
different trim count. This is a real, reachable discontinuity — one ulp in the product changes how
many observations are discarded, and therefore changes the answer by a visible amount rather than
by a rounding error. Any implementation of this function needs a defensible rule for forming the
count, and the Handbook does not know what Excel's is.

**Summation of the retained values.** The ordinary accumulation question applies: naive
left-to-right summation of a long retained set accumulates error proportional to `n`; pairwise or
compensated (Kahan/Neumaier) summation reduces it. One pleasant side effect of trimming is that
the retained values span a narrower range than the original data, so the sum is generally *better*
conditioned than `AVERAGE`'s on the same input.

**Sorting does not lose accuracy**, but it does fix an order, and summing in sorted order is a
different floating-point sum from summing in sheet order. So `TRIMMEAN(array, 0)` and
`AVERAGE(array)` are mathematically identical and need not be identical in the last bit — which
makes their comparison a probe rather than a tautology.

The accuracy and convention behaviour of Excel's statistical procedures is a documented topic in
the published literature; Morten Welinder's work on Gnumeric's statistical functions is the most
concrete account, and the assessments by Knüsel and by McCullough & Wilson are the standing
evaluations. The Handbook names them as the right reading and does not assert from them what any
current build does.

## What has not been checked

No Handbook vector suite exists for `TRIMMEAN`, and no Handbook evidence record lists this surface
among its subjects. **Nobody has checked `TRIMMEAN` against Excel within the Handbook's record.**
The reference-engine battery rendered beside this page is the engine answering its own questions;
no Excel was involved in it.

Documentation gaps this page could not close: the `percent = 1` even-`n` case; empty `array`;
non-numeric contents of `array`; a nonnumeric `percent`; and how the trim count is formed
arithmetically.

Inputs worth probing first:

1. **`percent = 1` on an even-length array**, and on an odd-length one. The documentation admits
   the input; the even case has no data left to average. One pair of calls decides whether Excel
   returns an error, a zero, or something else — and it is the sharpest undocumented corner on the
   page.
2. **The trim-count boundary.** `n = 20` with `percent` at `0.2`, and at `0.2` perturbed by one
   ulp in each direction; then `n = 3` with `percent = 2/3`, and `n = 7` with `percent = 2/7`.
   These are the inputs where `n · percent` sits on an even integer and the floor can fall either
   way. The answer changes by a visible amount, not by a rounding error, so the probe is easy to
   read.
3. **`TRIMMEAN(array, 0)` against `AVERAGE(array)`** — exact mathematically, and a difference in
   the last bits would reveal that the two surfaces accumulate differently (sorted order versus
   sheet order). Run it on data spanning many magnitudes, where the two orders diverge most.
4. **`TRIMMEAN` against `MEDIAN`** on odd-length data with a `percent` large enough to leave one
   value — should agree exactly.
5. **The flat region.** `percent` swept from 0 to `2/n` on a fixed array: the result must be
   constant across the whole range. A step anywhere inside it means the trim count is not the
   documented floor.
6. **Ties at the cut point** — several equal values straddling the boundary, confirming that the
   answer does not depend on which copies are trimmed.
7. **Text, logicals, blanks and errors inside `array`**, which Microsoft's page does not cover at
   all, and the direct-argument versus range-scan contrast that the neighbouring statistical pages
   document.
8. **Array-valued `percent`**, given the `excel-claimed` lift axis and the open `BUG-FUNC-018`
   stream: the spill shape, and whether an out-of-range element yields an element-local `#NUM!`
   rather than collapsing the whole result.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| trimmed mean | The mean of the data after discarding equal counts from each end |
| trim count | `k = floor(n · percent / 2)`, the number discarded from each end |
| symmetric trimming | The reason the excluded total is rounded down to a multiple of two |
| breakdown point | The fraction of arbitrarily corrupted data the estimator tolerates (`≈ percent/2`) |
| Winsorized mean | The other robust mean — replaces extremes instead of discarding them; not this |
| selection | Finding the `k`-th order statistic without a full sort |

## Sources

- Microsoft, "TRIMMEAN function" —
  <https://support.microsoft.com/en-us/office/trimmean-function-d90c9878-a119-4746-88fa-63d988f511d3>
  (syntax; the description of `percent` as the fractional number of data points to exclude with
  the 20-point / `percent = 0.2` worked example; the rule that the number of excluded data points
  is rounded down to the nearest multiple of 2; and `#NUM!` for `percent < 0` or `percent > 1`).
  Retrieved for this page. The page documents nothing about text, logicals, blanks, an empty
  array, or the `percent = 1` boundary.
- J. W. Tukey on trimmed means; P. J. Huber, *Robust Statistics*; F. R. Hampel, E. M. Ronchetti,
  P. J. Rousseeuw and W. A. Stahel, *Robust Statistics: The Approach Based on Influence
  Functions* — the breakdown-point and influence-function reading of this estimator.
- C. A. R. Hoare's selection algorithm and the introselect refinement — the linear-time route to
  the two cut points.
- N. J. Higham, *Accuracy and Stability of Numerical Algorithms*, chapter 4 (summation) — pairwise
  and compensated accumulation of the retained values.
- M. Welinder's documentation of Gnumeric's statistical functions, and the Excel accuracy
  assessments of R. Knüsel and of B. D. McCullough and B. Wilson — named as the standing
  literature, not as evidence about any current build.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md).
- OxFunc defect stream
  `docs/bugs/streams/BUG-FUNC-018_successor_scalar_parameter_array_lift_gap.md`, named by the
  presence projection for this surface. Cited by name only.
- Handbook projections `data/functions/FUNC.TRIMMEAN.json` (arity, the aggregate dual coercion
  policy, and the `excel-claimed` provenance on the by-index lift axis),
  `data/presence/FUNC.TRIMMEAN.json` (the shared `moment_stats_family` module) and
  `data/battery/FUNC.TRIMMEAN.json`.
