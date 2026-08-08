---
schema: efh.function-page/v1
function_id: FUNC.MEDIAN
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
family: median_fn
role_in_family: >-
  The central order statistic, with the mid-average convention on even counts — the one place
  this family performs arithmetic, and therefore the one place it can round.
---

# MEDIAN

## What it computes

`MEDIAN(number1, [number2], …)` returns the middle value of the numeric data it admits.

Let `x₍₁₎ ≤ x₍₂₎ ≤ … ≤ x₍ₙ₎` be the order statistics of the admitted values. Then

    n odd :   MEDIAN = x₍₍ₙ₊₁₎/₂₎
    n even:   MEDIAN = ( x₍ₙ/₂₎ + x₍ₙ/₂₊₁₎ ) / 2

Microsoft states the even case directly: "If there is an even number of numbers in the set,
then MEDIAN calculates the average of the two numbers in the middle." The documented worked
example is `MEDIAN({2, 3, 3, 5, 7, 10}) = 4`, the average of `3` and `5`.

Two consequences that are the whole character of the function:

1. **On an odd count `MEDIAN` is a selector** — its result is bit-identical to one of the
   inputs, with no arithmetic performed. **On an even count it is not**: it returns a mean of
   two data points, which need not be in the data and *is* subject to floating-point rounding.
   `MEDIAN` is therefore the one member of the order-statistic group whose answer can differ in
   the last bits between two correct implementations. See Numerical notes.
2. **The median is the minimiser of mean absolute deviation**, `argmin_c Σ|xᵢ − c|`, as the
   mean is the minimiser of mean squared deviation. On an even count the minimiser is not
   unique — every point of the closed interval `[x₍ₙ/₂₎, x₍ₙ/₂₊₁₎]` achieves the same total
   absolute deviation — and the mid-average convention is a *choice* of representative from
   that interval, not a derivation. Other conventions (lower median, upper median,
   interpolated median) are equally defensible and are used elsewhere; this one is Excel's, and
   it agrees with the mainstream statistical convention.

**Domain.** At least one admitted numeric value. **Range:** the closed interval
`[min, max]` of the data; the median is a location statistic and cannot leave the convex hull
of the sample.

The median is the **50th percentile**, the second quartile, and the most robust common measure
of centre: its breakdown point is `50%`, meaning up to half the data can be replaced by
arbitrary values before the median can be forced anywhere. The arithmetic mean's breakdown
point is `0%` — a single value moves it arbitrarily far. That contrast is the reason `MEDIAN`
exists next to [AVERAGE](FUNC.AVERAGE.md), and it is the right basis for choosing between them.

## Arguments

| Argument | Meaning | Notes |
|---|---|---|
| `number1` | The first value or range | Required |
| `number2, …` | Further values or ranges | Optional; up to 254 more |

The reference engine declares the matching 1-to-255 arity. Argument boundaries carry no
meaning; every slot pools into one sample.

The coercion behaviour is the family-standard split documented on Microsoft's page:

| Value | Typed **directly** into the argument list | Reached inside an **array or reference** |
|---|---|---|
| Number | counted | counted |
| Zero | counted | **counted** — Microsoft states this explicitly |
| `TRUE` / `FALSE` | counted (as 1 / 0) | ignored |
| Text that reads as a number | counted | ignored |
| Text that does not read as a number | error | ignored |
| Empty cell | — | ignored |
| Error value | error | error |

The zero row is called out on Microsoft's page for a reason: users routinely expect a "blank
or zero" range to contribute nothing, and blanks and zeros are treated oppositely. A column
where missing readings were entered as `0` has a very different median from one where they were
left blank, and the function gives no signal about which situation it is in.

See [Coercion and lifting](../model/02-coercion-and-lifting.md) for the general rule this
instantiates.

## Result and edge cases

Returns `Number`.

- **`n = 1`** — the single value, returned unchanged.
- **`n = 2`** — the arithmetic mean of the two, which is where the rounding question below
  first bites.
- **An admitted set that is empty** — every value skipped, or an all-blank range. There is no
  middle element. The reference engine's own battery — OxFunc's answers, no Excel involved —
  returns an error for its blank-argument row. Microsoft's page documents no answer for this
  case at all, which is a **documentation gap**: `MAX` and `MAXA` both document an explicit
  `0` for the no-numbers case and `MEDIAN` documents nothing. The Handbook records the gap and
  does not fill it from memory.
- **Ties and repeated values** need no special handling; multiplicity is preserved by the
  ordering and the middle position is well defined regardless.
- **Very large values.** `MEDIAN` over two values near the largest finite double is the
  overflow probe: `(a + b)/2` overflows to infinity where `a + (b − a)/2` does not. Excel has
  no infinity in its published value universe, so an implementation that overflows internally
  must produce *something* — a `#NUM!`, a saturated value, or a correct answer, depending on
  how the mean is formed. Which one is unverified and is on the probe list.
- **`+0` and `−0`.** On an odd count the returned representation is whichever value was
  selected; on an even count `(+0 + −0)/2` is `+0` under round-to-nearest, so the sign
  information is destroyed. Observable through `1/MEDIAN(...)`.
- **Error values** in the data propagate. The reference engine records an `ErrorCollapseProfile`
  of `None` for this surface — unlike [MAX](FUNC.MAX.md), which declares `ReductionFold` — so
  the projection does not describe `MEDIAN` as folding competing errors by a reduction rule.
  What happens when two different errors are present is therefore not settled by the
  classification either.
- **Arrays** are consumed by scanning, not lifted elementwise; `MEDIAN` is not a lift kernel.

## Errors

| Error | Condition | Basis |
|---|---|---|
| `#VALUE!` | A directly-passed argument is text that cannot be translated into a number | Documented on Microsoft's `MEDIAN` page |
| propagated | An argument is an error value | Documented ("error values or untranslatable text cause errors") |
| *(undocumented)* | No admitted numeric values | Microsoft's page states no answer; the reference engine's battery errors |

## Relationships

- **[AVERAGE](FUNC.AVERAGE.md)** — the other location statistic, with the same coercion split
  and a completely different robustness profile. Comparing the two on the same data is the
  standard skewness diagnostic: `MEDIAN < AVERAGE` indicates right skew.
- **[PERCENTILE.INC](FUNC.PERCENTILE.INC.md)** at `p = 0.5` and
  **[QUARTILE.INC](FUNC.QUARTILE.INC.md)** at `quart = 2` compute the same quantity by the same
  interpolation convention, so all three must agree. They are separate registered surfaces and
  may reach the answer by different code; whether they agree in bits is a first-class question
  the Handbook has not settled. **[PERCENTILE.EXC](FUNC.PERCENTILE.EXC.md)** at `p = 0.5` also
  equals the median, because the two percentile conventions coincide exactly at the centre —
  which makes the centre a poor place to test the difference between them, and a good place to
  test that they agree.
- **[LARGE](FUNC.LARGE.md)** / **[SMALL](FUNC.SMALL.md)** — the general order-statistic
  selectors. On an odd count, `MEDIAN` equals `LARGE(array, (n+1)/2)` exactly; on an even count
  it does not equal any `LARGE`.
- **[MODE.SNGL](FUNC.MODE.SNGL.md)** / **[MODE.MULT](FUNC.MODE.MULT.md)** — the third classical
  measure of centre. Mean, median and mode answer different questions and coincide only for
  symmetric unimodal data.
- **[TRIMMEAN](FUNC.TRIMMEAN.md)** — the tunable compromise between mean and median, discarding
  a proportion from each tail.
- **[SUBTOTAL](FUNC.SUBTOTAL.md)** (function numbers 12 and 112) and
  **[AGGREGATE](FUNC.AGGREGATE.md)** (function number 12) — the filter-aware and error-skipping
  medians. `AGGREGATE` is the only route to a median that ignores error values.
- **Confused with:** "the middle cell of the range", which is what a reader sometimes assumes
  when the data are already sorted in the sheet — the two coincide only by accident, and stop
  coinciding the moment a blank appears.

## Numerical notes

`MEDIAN` divides into a **selection** problem and, on even counts, a **one-operation
arithmetic** problem. Both are small and both have a right answer.

**Selection.** Finding the middle order statistic does not require a full sort. The classical
routes are quickselect (expected linear, quadratic worst case), the median-of-medians
algorithm of Blum, Floyd, Pratt, Rivest and Tarjan (worst-case linear), and simply sorting
(`O(n log n)`, and usually fastest at spreadsheet sizes). Knuth's *The Art of Computer
Programming* volume 3 §5.3.3 treats minimum-comparison selection; Numerical Recipes §8.5
presents exactly this problem, including the two-element case that even counts require. An even
count needs **both** middle elements, which a naive single-selection routine does not give —
running quickselect twice is wasteful and a single partition pass can yield both.

**The average of two.** This is the only arithmetic in the function and it has three
formulations that are not equivalent in floating point:

    (a + b) / 2          overflows when |a + b| exceeds the largest double
    a/2 + b/2            never overflows; can underflow both halves to zero for subnormals
    a + (b − a)/2        never overflows; exact for a == b; the usual recommendation

The third is the standard "midpoint without overflow" formula and is what a careful
implementation uses; it also guarantees the result lies in `[min(a,b), max(a,b)]`, which the
first formulation does *not* guarantee under directed rounding. For a median that must be
`a` when `a == b`, only the third form is exact by construction. The discussion in Sterbenz's
*Floating-Point Computation* and in Higham's *Accuracy and Stability of Numerical Algorithms*
(chapter 2, on the perils of naive averaging) covers the general case; the same reasoning is
why `(low + high)/2` is the famous binary-search overflow bug.

For ordinary spreadsheet data all three agree to the last bit, and the difference appears only
at the extremes of the exponent range. That makes the extremes the only place a residual plate
for `MEDIAN` would show anything, and it makes them worth probing precisely because nothing
else will reveal which formulation is in use.

**Comparison predicate.** The selection needs a total order, with the same open questions as on
[LARGE](FUNC.LARGE.md): raw IEEE `<` versus a tolerant truncation-style comparison, and the
`±0` tie. For `MEDIAN` a tolerant comparator could reorder near-equal values and thereby change
*which* two values are averaged on an even count — a slightly larger consequence than for a
pure selector.

**Robustness is not stability.** The median's `50%` breakdown point is a statistical property
of the estimator, not a numerical one. A median computed by a numerically careless route is
still a median; a median of badly-computed inputs is still garbage. The two robustness stories
are independent and are frequently conflated.

## What has not been checked

No evidence record lists `FUNC.MEDIAN` among its subjects, and no count in the Handbook's
evidence layer touches this surface. Nobody has checked `MEDIAN` against Excel within the
Handbook's record. No Handbook vector suite exists.

Microsoft's `MEDIAN` page was retrieved for this curation pass, so the even-count rule, the
coercion split, the explicit inclusion of zeros, and the error rule above are quoted
documentation rather than recollection.

One documentation gap recorded as a finding: Microsoft's page documents no result for the case
where no numeric values are admitted, while the neighbouring [MAX](FUNC.MAX.md) and
[MAXA](FUNC.MAXA.md) pages both document an explicit `0` for their equivalent case. The
reference engine's own battery returns an error for that row. The Handbook records the gap and
the engine's answer, and asserts nothing about Excel.

Inputs I would probe first, and why:

1. **`MEDIAN` of two copies of the largest finite double**, and of the largest and second
   largest. The exact answer is representable; `(a+b)/2` overflows on the way. **This single
   probe identifies which averaging formula is in use**, and no other input will.
2. **`MEDIAN` of two adjacent subnormals**, the underflow mirror, which distinguishes
   `a/2 + b/2` from the other two.
3. **`MEDIAN` of `{+0, −0}`**, read through `1/MEDIAN(...)`, and of `{−0}` alone — the sign
   question on both branches.
4. **An even-count set whose two middle values sum to a number needing 54 bits** — for example
   `1` and `1 + 2⁻⁵²` — where the exact mean is not representable and the rounding is
   observable.
5. **`MEDIAN` against `PERCENTILE.INC(·, 0.5)`, `PERCENTILE.EXC(·, 0.5)` and
   `QUARTILE.INC(·, 2)`** on the same data, bit for bit. Four surfaces that must agree; a
   disagreement localises a defect without any external oracle.
6. **`MEDIAN` against `LARGE(·, (n+1)/2)`** on odd counts, which must agree exactly because
   both are selectors.
7. **A blank range, and a range of only text**, to characterise the undocumented empty case.
8. **A range containing zeros and blanks in equal measure**, confirming the documented
   asymmetry between them — the behaviour real users trip over.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| order statistic | `x₍ᵢ₎`, the `i`-th smallest admitted value |
| mid-average convention | Averaging the two central values on an even count; a choice, not a derivation |
| breakdown point | The proportion of arbitrarily corrupted data an estimator tolerates; `50%` for the median |
| midpoint without overflow | `a + (b − a)/2`, the safe formulation of the two-value mean |
| selector branch | The odd-count case, where the result is bit-identical to an input |
| admitted set | The values surviving the scan and coercion policy; `n` counts these, not cells |

## Sources

- Microsoft, "MEDIAN function" —
  <https://support.microsoft.com/en-us/office/median-function-d0916313-4753-414c-8537-ce85bdd967d2>
  — retrieved for this curation pass. Source of the syntax, the even-count averaging rule, the
  worked example, the ignored/counted table including the explicit inclusion of zero cells, the
  direct-argument rule, and the error rule.
- Knuth, *The Art of Computer Programming*, volume 3, §5.3.3 — selection.
- Press, Teukolsky, Vetterling and Flannery, *Numerical Recipes*, §8.5 — selection, including
  simultaneous selection of two adjacent order statistics.
- Blum, Floyd, Pratt, Rivest and Tarjan, *Time bounds for selection* (1973).
- Sterbenz, *Floating-Point Computation*; and Higham, *Accuracy and Stability of Numerical
  Algorithms*, 2nd ed., chapter 2 — the analysis of naive averaging and the overflow-safe
  midpoint.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md) — the direct-argument
  versus range-scan split.
- Handbook projections `data/functions/FUNC.MEDIAN.json` (arity 1–255, `xlfMedian` code 227,
  `AggregateDirectAndRangeDualPolicy`, `ErrorCollapseProfile::None`) and
  `data/presence/FUNC.MEDIAN.json` (module `median_fn.rs`, unshared, no defect stream named).
