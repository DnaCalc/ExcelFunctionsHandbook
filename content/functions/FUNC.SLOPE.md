---
schema: efh.function-page/v1
function_id: FUNC.SLOPE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MISC-0008
open_problems: []
references:
  - work: "Microsoft Support — SLOPE function"
    locator: "https://support.microsoft.com/en-us/office/slope-function-11fb8f97-3117-4813-98aa-61d7e01276b9"
    role: "documented signature, the argument order, the stated equation, and the documented #N/A and #DIV/0! conditions"
  - work: "Chan, T. F., Golub, G. H. and LeVeque, R. J., \"Algorithms for Computing the Sample Variance: Analysis and Recommendations\""
    locator: "The American Statistician 37(3), 1983, pp. 242-247"
    role: "the error analysis separating the two-pass centred form from the raw sums-of-squares form"
  - work: "McCullough, B. D. and Wilson, B., \"On the accuracy of statistical procedures in Microsoft Excel\""
    locator: "Computational Statistics & Data Analysis, 1999 and the follow-up assessments through 2005"
    role: "the published record of Excel's regression and variance accuracy and of the algorithm change at Excel 2003"
  - work: "OxFunc — paired_stats_common.rs"
    locator: "crates/oxfunc_core/src/functions/paired_stats_common.rs"
    role: "reference-engine kernel: pairwise deletion, the centred slope, and the error split"
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
family: slope_fn
role_in_family: >-
  The ordinary-least-squares slope of y on x, the first of the two coefficients of the simple
  regression line and the one that carries the units of the relationship.
---

# SLOPE

## What it computes

`SLOPE(known_y's, known_x's)` returns the slope `b` of the **ordinary least squares** line fitted
through the paired data — the unique `b` minimising the sum of squared vertical residuals.

For pairs `(x_1, y_1), …, (x_n, y_n)`, write the means `xbar` and `ybar` and the centred sums

    Sxx  =  sum (x_i - xbar)^2
    Sxy  =  sum (x_i - xbar)(y_i - ybar)

Then

    SLOPE  =  b  =  Sxy / Sxx

This is the stationary point of `sum (y_i - a - b*x_i)^2` in `b`, and it is the entire content of
the function. Three equivalent readings are worth carrying, because each explains a different
edge case:

1. **`b = Sxy / Sxx`** — a ratio of second moments. Undefined precisely when `Sxx = 0`, that is,
   when every `x_i` is the same. A vertical scatter has no least-squares slope, which is why the
   documented failure there is `#DIV/0!` rather than a large number.
2. **`b = r * (s_y / s_x)`** — the correlation scaled by the ratio of standard deviations. This
   is the reading that shows `SLOPE` carries **units**: `b` is in units of `y` per unit of `x`,
   unlike `r` and [RSQ](FUNC.RSQ.md), which are dimensionless.
3. **`b = COVARIANCE.P(y, x) / VAR.P(x)`** — and equally with the sample forms, since the `n`
   and `n-1` factors cancel between numerator and denominator. `SLOPE` is a ratio in which the
   population/sample distinction is invisible.

`SLOPE` is **not symmetric in its arguments**. The least-squares line of `y` on `x` minimises
vertical distances; the line of `x` on `y` minimises horizontal ones, and they are different
lines unless the fit is perfect. The relation between them is
`SLOPE(y,x) * SLOPE(x,y) = r^2`, which is [RSQ](FUNC.RSQ.md) — so swapping the arguments does not
give the reciprocal, it gives the reciprocal *shrunk by* `r^2`. This is regression to the mean,
stated as an identity.

Together with [INTERCEPT](FUNC.INTERCEPT.md), which returns `a = ybar - b*xbar`, the pair
determines the fitted line. The line always passes through `(xbar, ybar)`.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `known_y's` | The dependent values — the ones being explained. First. | yes |
| `known_x's` | The independent values. Second. | yes |

**The `y` argument comes first.** This is the single most common error at the call site, because
it reads backwards against `(x, y)` habit and against the way charts are labelled. Getting it
wrong does not produce an error — it produces `1/b` shrunk by `r^2`, a plausible number in the
wrong units.

Both arguments are consumed by scanning. Pairing is **positional**, in row-major order over each
argument; the two need not have the same shape, only the same number of elements.

**Non-numeric cells are handled by pairwise deletion.** If either member of a pair is text, a
logical or a blank cell, the reference engine drops **the whole pair**, not just the offending
value. This is the only coherent reading — dropping one side would shift every subsequent pairing
— but it is stronger than what the documentation says, which is that such values "are ignored".
The distinction is observable: a blank in `known_x's` removes its `y` partner from the fit, and
therefore changes `ybar`.

## Result and edge cases

Returns `Number`.

- **Fewer than two complete pairs.** `#DIV/0!` from the reference engine. Microsoft documents
  `#DIV/0!` for a single data point.
- **All `x` equal.** `Sxx = 0` and the answer is `#DIV/0!`. Note this is checked on the computed
  `Sxx`, not on a distinctness test of the `x` values, so it also triggers when the `x` values
  differ but their centred squares underflow to zero.
- **All `y` equal.** `Sxy = 0` and the slope is exactly `0` — a valid answer, not an error.
- **Unequal element counts.** `#N/A`, documented and implemented.
- **Both arguments empty.** The counts are equal (both zero), so the length check passes and the
  fit fails: the reference engine returns `#DIV/0!`. **Microsoft's documented condition for empty
  arguments is `#N/A`.** This is a documentation-versus-reference-engine divergence and it is
  recorded here as a finding rather than resolved; see Errors.
- **Errors in either argument** propagate as themselves.
- **Two-dimensional arguments** are flattened row-major before pairing.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#N/A` | `known_y's` and `known_x's` have different numbers of data points | documented and implemented |
| `#N/A` | the arguments are empty | **documented**; the reference engine returns `#DIV/0!` here instead |
| `#DIV/0!` | only one data point | documented and implemented |
| `#DIV/0!` | `Sxx = 0` — the `x` values have no spread | implemented; follows from the definition |
| propagated | An error value in either argument surfaces as that error | shared coercion rule |

The empty-argument row is the divergence worth carrying away. Microsoft's page groups "empty" and
"different number of data points" into one `#N/A` sentence. The reference engine's length check
passes trivially on two empty vectors and the failure falls through to the fit, which reports
`#DIV/0!`. Both are defensible readings of "there is nothing to fit"; they are not the same error
value, and a formula wrapped in `IFNA` rather than `IFERROR` behaves differently under the two.
The Handbook has not observed which one Excel returns.

## Relationships

- **[INTERCEPT](FUNC.INTERCEPT.md)** is the other coefficient of the same line,
  `a = ybar - b*xbar`. The two share one implementing kernel in the reference engine, and
  `INTERCEPT` is computed by calling the slope and then correcting — so anything true of `SLOPE`'s
  failure modes is true of `INTERCEPT`'s.
- **[RSQ](FUNC.RSQ.md)** is the goodness of fit of the same line, and satisfies
  `RSQ(y,x) = SLOPE(y,x) * SLOPE(x,y)`. A large slope with a small `RSQ` is a strong-looking
  relationship that explains nothing; the two functions are meant to be read together.
- **[CORREL](FUNC.CORREL.md) and [PEARSON](FUNC.PEARSON.md)** give `r`, from which
  `b = r * s_y / s_x`.
- **[STEYX](FUNC.STEYX.md)** is the standard error of the same regression — the scale of the
  residuals `SLOPE` leaves behind.
- **[LINEST](FUNC.LINEST.md)** returns the same slope as part of a full regression, with standard
  errors and diagnostics, and generalises to multiple predictors. `SLOPE` is the one-predictor
  special case with everything else discarded. **[TREND](FUNC.TREND.md)** and
  **[FORECAST.LINEAR](FUNC.FORECAST.LINEAR.md)** evaluate the same line at new `x` values.
- **[FORECAST](FUNC.FORECAST.md)** is the legacy name of `FORECAST.LINEAR` and uses the same
  fit.
- Readers confuse `SLOPE` with the chart trendline coefficient. They are the same quantity
  mathematically; whether Excel computes them by the same code path is not something this page
  claims.

## Numerical notes

`SLOPE` is the function where Excel's statistical accuracy was fought over in public, and the
argument is entirely about **which algebraically-equal formula you evaluate**.

**Two forms, one answer, very different error.** The textbook one-pass form is

    b  =  ( n*sum(x*y) - sum(x)*sum(y) )  /  ( n*sum(x^2) - sum(x)^2 )

It needs one pass and no stored data, which is why every statistics textbook of the punched-card
era printed it. It is also catastrophically unstable: when the `x` values are large relative to
their spread, `n*sum(x^2)` and `sum(x)^2` are two nearly-equal large numbers whose difference is
the small quantity `Sxx`. Subtracting them destroys leading digits in proportion to
`(xbar/s_x)^2`. For `x` values around a million with a spread of one, the denominator can lose
every significant digit it had, and the computed `Sxx` can even come out **negative** — which is
impossible for a sum of squares and is the signature by which this bug is recognised.

The two-pass centred form the reference engine uses,

    xbar  =  sum(x)/n ;   Sxx = sum (x - xbar)^2 ;   Sxy = sum (x - xbar)(y - ybar)

computes each deviation before squaring, so no large cancellation ever occurs. Its error grows
with the condition number of the problem rather than with its square. Chan, Golub and LeVeque
(1983) give the error analysis for exactly this comparison, and their recommendation — use the
two-pass form, or Welford-style updating if one pass is required — is the settled answer.

**The published record on Excel.** McCullough and Wilson's assessments of Excel's statistical
procedures, running from 1999 through the 2000s in *Computational Statistics & Data Analysis*,
and Knüsel's work on the same subject, documented failures of exactly this kind in Excel's
regression and variance functions, and recorded that the algorithms were changed at Excel 2003.
Welinder's work on Gnumeric's statistical functions is the other standing reference on Excel's
statistical numerics and is legitimate to consult. **What none of that licenses is a statement on
this page about what Excel's `SLOPE` does internally today.** The Handbook does not assert
Excel's algorithm. What it asserts is that the two forms are distinguishable by measurement, and
that the measurement has not been made here.

**Residual hazards in the good form.** The two-pass form is not immune, only far better:

- The mean is accumulated by naive left-to-right summation, so `xbar` itself carries an error
  growing with `n`. Compensated or pairwise summation would bound it by a constant.
- `Sxx` and `Sxy` are accumulated naively too. `Sxx` has non-negative terms and is therefore
  well-conditioned; `Sxy` has mixed signs and can cancel badly when the relationship is weak —
  which is exactly when the slope is near zero and its relative error is worst. A near-zero slope
  from a large noisy sample is the least trustworthy answer this function gives.
- The final division is a single correctly-rounded operation and contributes at most half an ulp.
- The `Sxx == 0` test is an exact comparison against zero, not a tolerance. Data whose `x` spread
  is real but tiny passes the test and yields an enormous slope rather than an error. There is no
  right answer here; there is a choice, and the choice is exactness.

**Reproducibility.** Summation order is part of the answer. Two implementations that both use the
centred form, and both are correct, will disagree in the last bits if they traverse the data in
different orders or vectorise differently. Nothing about the mathematics fixes those bits.

## What has not been checked

One Handbook evidence record, `EV-MISC-0008`, names `FUNC.SLOPE` among its subjects. It is a
live-verification record from an upstream sweep, and its figures, scope and reader warning render
mechanically beside this page — read them there rather than taking a summary from this prose. Its
scope is small and its own warning says so. No Handbook vector suite exists for `SLOPE`, and
nothing on this page is a statement that any implementation agrees with Excel over any domain.

The reference engine's implementing module also appears in the upstream discrepancy catalogue and
in two discrepancy ledgers. Those are upstream registers, not Handbook measurements.

Everything above marked as documented comes from Microsoft's `SLOPE` page. **Retrieval of that
page was blocked by the upstream host on this pass**, so those statements are recorded as
documented behaviour with the source named and should be re-read against the live page — the
empty-argument `#N/A` in particular, since it is the divergence this page reports.

Inputs worth probing first:

1. **The empty-argument case.** Two empty ranges of equal size, against `#N/A` (documented) and
   `#DIV/0!` (reference engine). One cell settles it.
2. **The ill-conditioned pair.** `x = {1000000, 1000001, 1000002, 1000003}` with a known exact
   slope, and the same data shifted to `{0,1,2,3}`. If the two agree, the centred form is in use;
   if the large-offset case degrades, the raw-sums form is. This is the experiment that
   characterises Excel's algorithm rather than guessing at it, and it is the highest-value probe
   on the list.
3. **`Sxx` underflow.** `x` values differing by a subnormal amount, where the centred squares
   round to zero and the `#DIV/0!` fires on data that is not constant.
4. **Pairwise deletion.** A blank cell in `known_x's` opposite a nonzero `y`, with the fit
   compared against the same data with that pair physically removed. If they agree, the pair was
   deleted; if not, something else happened.
5. **A logical and a numeric-looking text value** in each argument, separately, against the
   ignore-in-scan rule.
6. **Argument-order confirmation**: `SLOPE(y,x) * SLOPE(x,y)` against `RSQ(y,x)`, compared
   bitwise. The identity is exact in real arithmetic; any gap is pure floating-point residue and
   measures the two functions against each other.
7. **A perfectly collinear data set** where the true slope is exactly representable, checking
   whether it is returned exactly.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| `Sxx`, `Sxy` | The centred sums of squares and cross-products |
| centred (two-pass) form | Subtract the mean, then accumulate; the numerically stable route |
| raw sums-of-squares form | The one-pass textbook formula, unstable under a large mean |
| pairwise deletion | Dropping both members of a pair when either is non-numeric |
| positional pairing | The element-by-element correspondence between the two arguments |
| ill-conditioned | Data whose `x` mean is large relative to its spread, where the two forms separate |

## Sources

- Microsoft, *SLOPE function* —
  <https://support.microsoft.com/en-us/office/slope-function-11fb8f97-3117-4813-98aa-61d7e01276b9>
  (signature and argument order, the stated equation, the ignore-text-and-logicals rule for
  arrays, and the documented `#N/A` and `#DIV/0!` conditions). Retrieval was blocked by the
  upstream host for this page; the documented behaviour above is stated as documented behaviour
  and should be re-checked against the page.
- T. F. Chan, G. H. Golub and R. J. LeVeque, "Algorithms for Computing the Sample Variance:
  Analysis and Recommendations", *The American Statistician* 37(3), 1983, pp. 242–247 — the
  error analysis behind the two-pass recommendation.
- B. D. McCullough and B. Wilson, "On the accuracy of statistical procedures in Microsoft Excel",
  *Computational Statistics & Data Analysis*, 1999, and the follow-up assessments through 2005;
  L. Knüsel's parallel work — the published record of Excel's statistical accuracy and of the
  algorithm change at Excel 2003. M. Welinder's work on Gnumeric's statistical functions is the
  other standing reference on this subject.
- N. J. Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd ed. — summation error
  bounds and the conditioning discussion underlying the residual-hazards list.
- OxFunc `crates/oxfunc_core/src/functions/slope_fn.rs` and
  `crates/oxfunc_core/src/functions/paired_stats_common.rs` at commit `473efa3` — pairwise
  deletion, the unequal-length `#N/A`, the centred accumulation, and the `Sxx == 0` and
  `n < 2` `#DIV/0!` guards.
- Handbook evidence record `EV-MISC-0008` (rendered beside this page).
- Handbook `data/functions/FUNC.SLOPE.json`, `data/presence/FUNC.SLOPE.json`.
