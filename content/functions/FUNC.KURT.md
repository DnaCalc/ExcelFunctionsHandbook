---
schema: efh.function-page/v1
function_id: FUNC.KURT
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
  The fourth-moment member: the bias-corrected sample excess kurtosis, and the family's most
  cancellation-prone estimator.
---

# KURT

## What it computes

`KURT(number1, [number2], …)` returns the **bias-corrected sample excess kurtosis** of the
numeric values it admits. The estimator Microsoft documents is

    KURT = [ n(n+1) / ((n−1)(n−2)(n−3)) ] · Σᵢ ( (xᵢ − x̄) / s )⁴
           −  3(n−1)² / ((n−2)(n−3))

where `x̄` is the sample mean and `s` is the **sample** standard deviation with the `n − 1`
denominator, the same `s` that [STDEV.S](FUNC.STDEV.S.md) returns.

Three separate conventions are baked into that one line, and every disagreement between
statistical packages about "the kurtosis" is a disagreement about which of them applies:

1. **Excess, not raw.** The trailing term subtracts the kurtosis of the normal distribution,
   so a Gaussian sample has expectation zero rather than three. The population quantity being
   estimated is `β₂ − 3 = μ₄/μ₂² − 3`, where `μₖ` is the `k`-th central moment.
2. **Bias-corrected, not the plug-in moment ratio.** The `n(n+1)/((n−1)(n−2)(n−3))` factor is
   the standard `G₂` correction, which makes the estimator unbiased for the excess kurtosis
   when the parent distribution is normal. In the taxonomy of Joanes and Gill (1998) this is
   `G₂`, not the plug-in `g₂` and not the `b₂` variant; SAS and SPSS use `G₂`, several other
   systems do not.
3. **Sample, not population, scaling of `s`.** Using `n − 1` inside the standardisation and
   then applying the `G₂` factor on top is what makes the algebra come out.

**Domain.** The formula requires `n ≥ 4` — the factor `(n−3)` in both denominators is the
binding constraint — and `s > 0`. **Range:** the sample excess kurtosis of `n` points is
bounded below by `−2·(n−1)/(n−2) + 3(n−1)²/((n−2)(n−3))`-type expressions and above by a
finite value determined by `n`; unlike the population quantity it cannot be arbitrarily
large for fixed `n`. The population excess kurtosis is bounded below by `−2` (attained by the
two-point distribution) and unbounded above.

Useful reference values, all exact and all cheap to check: four equally spaced points give
`−1.2`, which is the excess kurtosis of the continuous uniform distribution; a sample that is
exactly normal in its order statistics tends to `0`; heavy-tailed samples give large positive
values. Kurtosis is **not** a measure of "peakedness" — that reading is a persistent
textbook error. It is a standardised fourth moment, and the fourth power means it is driven
almost entirely by the observations farthest from the mean. A single outlier moves `KURT`
more than the entire rest of the sample.

## Arguments

| Argument | Meaning | Admissible values |
|---|---|---|
| `number1` | The first value or range of the data set | Required |
| `number2, …` | Further values or ranges | Optional; the reference engine declares up to 255 argument slots |

All slots contribute to one pooled sample; the argument boundaries carry no meaning. The
reference engine classifies this surface as `AggregateDirectAndRangeDualPolicy`, which is the
projection's name for the split described in
[Coercion and lifting](../model/02-coercion-and-lifting.md): values typed directly into the
argument list and values reached by scanning a range are governed by different coercion
rules. For the moment-statistics family the documented reading is the usual one — text,
logicals and empty cells inside a scanned range are ignored — but the Handbook has not
verified which policy row this surface actually takes.

## Result and edge cases

Returns `Number`.

- **Fewer than four data points** → the `(n−3)` denominators are zero or negative. The
  reference engine's own battery — OxFunc's answers, no Excel involved — returns `#DIV/0!`
  for every single-value row, which is what the formula predicts and what Microsoft's page
  documents.
- **All values identical** → `s = 0`, the standardisation divides by zero, `#DIV/0!`.
- **Exactly four points** is the boundary case and the one worth testing: the factors are
  `20/6` and `27/2`, both exactly representable, so a correct implementation returns the
  uniform-distribution value for equally spaced points with only the fourth-power sum
  contributing rounding.
- **Very large or very small values.** Standardising *before* raising to the fourth power is
  what keeps the sum in range; forming `Σ(xᵢ − x̄)⁴` first overflows for `|xᵢ − x̄| >` about
  `1.3·10⁷⁷` and underflows below about `10⁻⁷⁷`. See Numerical notes.
- **Error values** anywhere in the data propagate.
- Arrays are consumed by scanning, not lifted elementwise; `KURT` is not a lift kernel.

## Errors

| Error | Condition | Basis |
|---|---|---|
| `#DIV/0!` | Fewer than four data points, or the sample standard deviation is zero | Documented by Microsoft on the `KURT` page; also what the defining formula requires |
| `#VALUE!` | A directly-passed argument that cannot be converted to a number | Shared coercion rule, chapter 02 |
| propagated | An error value among the data | Shared coercion rule, chapter 02 |

Retrieval of Microsoft's `KURT` page was blocked for this curation pass; the `#DIV/0!` row is
stated as documented behaviour with its source named, and independently as a consequence of
the formula. Nobody has checked any of it against Excel within the Handbook's record.

## Relationships

- **[SKEW](FUNC.SKEW.md)** — the third-moment sibling, with its own bias-correction factor
  `n/((n−1)(n−2))`. `SKEW` needs `n ≥ 3`; `KURT` needs `n ≥ 4`. They share an implementing
  module in the reference engine.
- **[SKEW.P](FUNC.SKEW.P.md)** — the population (plug-in) skewness. Note the asymmetry in the
  published surface: Excel offers both a sample and a population *skewness*, but only the
  bias-corrected sample *kurtosis*. There is no `KURT.P`. A reader who wants the plug-in
  `g₂ = μ₄/μ₂² − 3` must compute it from [DEVSQ](FUNC.DEVSQ.md) and a fourth-power sum by
  hand, or invert the `G₂` correction algebraically.
- **[STDEV.S](FUNC.STDEV.S.md)** and **[VAR.S](FUNC.VAR.S.md)** — supply the `s` in the
  denominator; any disagreement about `s` propagates to the fourth power.
- **[DEVSQ](FUNC.DEVSQ.md)** — the second-moment building block, and the natural surface to
  compare against when checking whether the centred sums agree.
- **Confused with:** "peakedness". See above. Also confused with `SKEW`, since both are called
  "shape statistics" and both fail on small samples, but they fail at different `n`.

## Numerical notes

`KURT` is the hardest of the elementary descriptive statistics to compute well, for a reason
that is entirely structural: it is a **ratio of a fourth central moment to the square of a
second central moment**, and both are differences of large quantities.

**Cancellation.** The one-pass "computational" route accumulates `Σx`, `Σx²`, `Σx³`, `Σx⁴`
and reconstructs the central moments by the binomial expansion

    μ₄ = (Σx⁴ − 4x̄Σx³ + 6x̄²Σx² − 4x̄³Σx + n x̄⁴)/n

For data with a small coefficient of variation this is catastrophic: the terms are of order
`n·x̄⁴` while the result is of order `n·σ⁴`, so the relative error is inflated by
`(x̄/σ)⁴`. A sample with mean 10⁶ and spread 1 loses roughly twenty-four decimal digits — far
more than a double has. The estimator is then not merely inaccurate; it is arbitrary, and can
come out negative where the true value is positive. The canonical treatment is Chan, Golub and
LeVeque (1983) for the variance, extended to arbitrary order by Pébay, *Formulas for Robust,
One-Pass Parallel Computation of Covariances and Arbitrary-Order Statistical Moments* (Sandia
report SAND2008-6212), whose update relations are the standard modern reference for a stable
streaming fourth moment.

**What a careful implementation does.** Two passes: compute `x̄` first (with a compensated or
pairwise sum), then accumulate the centred powers `Σ(xᵢ − x̄)²` and `Σ(xᵢ − x̄)⁴` directly.
Optionally refine `x̄` with a second-pass correction term `Σ(xᵢ − x̄)/n`, which is zero in
exact arithmetic and is a cheap error-cancelling trick. Standardise before the fourth power —
that is, accumulate `Σz⁴` with `z = (xᵢ − x̄)/s` — to keep the summands near unity and avoid
the overflow and underflow bands mentioned above. Apply the two exactly-representable rational
factors last, and do the final subtraction in the order the formula is written, since the two
terms are of comparable size for near-normal data and their difference is the answer.

**The subtraction at the end is itself a cancellation.** For a near-normal sample the first
term is close to `3(n−1)²/((n−2)(n−3))` and the excess is a small difference of two moderate
numbers. Even with a perfect fourth-moment sum, a `KURT` near zero has few correct significant
digits in the relative sense — which is fine, because the quantity of interest is the absolute
excess, but it means a relative-error residual plate for `KURT` will look alarming near zero
and should be read as an absolute-error plate instead.

**The published record on Excel's statistical accuracy** is relevant background here and is
worth naming precisely: McCullough and Wilson's papers on the accuracy of Excel's statistical
procedures, and Morten Welinder's work on Gnumeric's statistical functions, both document
one-pass moment accumulation as a recurring defect class in spreadsheet implementations. The
Handbook does not assert what Excel does internally, and neither of those sources is a
measurement of this surface.

## What has not been checked

No evidence record lists `FUNC.KURT` among its subjects, and the surface does not appear in
any count in the Handbook's evidence layer. Nobody has checked `KURT` against Excel within the
Handbook's record.

The presence projection at commit `473efa3` places this surface in a module shared with
`SKEW`, `SKEW.P`, `STEYX` and `TRIMMEAN`, and records that the module is named by the upstream
defect stream `BUG-FUNC-021` on statistical numeric exactness drift. Being named in a
module-level defect stream is not a per-surface finding: it says the neighbourhood has known
drift, not that `KURT` has been measured. No Handbook vector suite exists.

Inputs I would probe first, and why:

1. **A shifted sample.** The same spread at mean `0`, `10³`, `10⁶` and `10⁹` — for example
   `{1,2,3,4}` versus `{1000001,1000002,1000003,1000004}`. In exact arithmetic all four give
   `−1.2`. This is the single decisive probe for one-pass accumulation, and it costs four
   cells.
2. **`n = 3` and `n = 4`** on the same data prefix, to pin the boundary at which the error
   turns into a value.
3. **All-equal data, and data equal to within one ulp**, to separate the `s = 0` guard from
   an `s ≈ 0` catastrophe. The second case is the dangerous one: no error, and a value with
   no correct digits.
4. **A single extreme outlier** — `{1,2,3,4,10¹⁵⁰}` — which tests the overflow behaviour of
   whatever power is accumulated first. Standardising first survives; raising the raw
   deviation to the fourth power does not.
5. **Subnormal spread** — four values differing by a few ulps near `10⁻³⁰⁵` — the underflow
   mirror of the previous probe.
6. **`KURT` against a hand-computed `G₂` built from [DEVSQ](FUNC.DEVSQ.md) and
   [STDEV.S](FUNC.STDEV.S.md)** on the same data, as a metamorphic check that the
   bias-correction factors are the documented ones rather than a plug-in variant.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| excess kurtosis | `μ₄/μ₂² − 3`; zero for the normal distribution |
| `G₂` | The bias-corrected sample estimator of excess kurtosis, in the Joanes–Gill taxonomy |
| plug-in `g₂` | The uncorrected moment-ratio estimator; not what this surface returns |
| central moment | `μₖ = Σ(xᵢ − x̄)ᵏ / n` |
| standardise-first | Dividing by `s` before raising to the fourth power, to control range |
| one-pass form | Reconstructing central moments from raw power sums; algebraically equal, numerically far worse |

## Sources

- Microsoft, "KURT function" —
  <https://support.microsoft.com/en-us/office/kurt-function-bc3a265c-5da4-4dcb-b7fd-c237789095ab>
  (the defining equation and the `#DIV/0!` conditions). Retrieval was blocked for this pass;
  the equation above is stated as the documented estimator with its source named.
- Joanes and Gill, *Comparing measures of sample skewness and kurtosis*, Journal of the Royal
  Statistical Society Series D 47 (1998) — the `g₂` / `G₂` / `b₂` taxonomy that identifies
  which estimator a package returns.
- Chan, Golub and LeVeque, *Algorithms for Computing the Sample Variance* (1983) — the
  cancellation analysis that generalises to higher moments.
- Pébay, *Formulas for Robust, One-Pass Parallel Computation of Covariances and
  Arbitrary-Order Statistical Moments*, Sandia SAND2008-6212 — stable update relations for
  third and fourth moments.
- McCullough and Wilson, *On the accuracy of statistical procedures in Microsoft Excel*; and
  Morten Welinder's work on Gnumeric's statistical functions — the published record on
  spreadsheet statistical accuracy. Named as literature, not as evidence about this surface.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md) — the direct-argument
  versus range-scan split.
- Handbook projections `data/functions/FUNC.KURT.json` (arity 1–255, `xlfKurt`,
  `AggregateDirectAndRangeDualPolicy`) and `data/presence/FUNC.KURT.json` (module
  `moment_stats_family.rs`, shared with `SKEW`, `SKEW.P`, `STEYX`, `TRIMMEAN`; upstream defect
  stream `BUG-FUNC-021` named on the module).
