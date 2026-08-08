---
schema: efh.function-page/v1
function_id: FUNC.STDEV.P
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — STDEV.P function"
    locator: "https://support.microsoft.com/en-us/office/stdev-p-function-6e917c05-31a0-496f-ade7-4f4e7462f285"
    role: "documented signature, the stated equation with the n divisor, the scan rules, and the #DIV/0! condition"
  - work: "Chan, T. F., Golub, G. H. and LeVeque, R. J., \"Algorithms for Computing the Sample Variance: Analysis and Recommendations\""
    locator: "The American Statistician 37(3), 1983, pp. 242-247"
    role: "the error analysis of the textbook one-pass formula against the two-pass and updating forms"
  - work: "Welford, B. P., \"Note on a Method for Calculating Corrected Sums of Squares and Products\""
    locator: "Technometrics 4(3), 1962, pp. 419-420"
    role: "the single-pass updating recurrence that is stable without storing the data"
  - work: "OxFunc — variance_common.rs"
    locator: "crates/oxfunc_core/src/functions/variance_common.rs"
    role: "reference-engine kernel: the two-pass centred variance and the population divisor"
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
family: stdev_p_fn
role_in_family: >-
  The population standard deviation: the plain n divisor, numeric cells only, defined for a
  single data point where the sample form is not.
---

# STDEV.P

## What it computes

`STDEV.P(number1, [number2], …)` returns the **population standard deviation** of the numeric
values it is given: the root mean squared deviation about their own mean.

For values `x_1, …, x_n` with mean `xbar`,

    sigma^2  =  ( 1 / n )  *  sum_{i=1..n} (x_i - xbar)^2
    sigma    =  sqrt( sigma^2 )

The divisor is `n`, with no Bessel correction. That is the entire difference from
[STDEV.S](FUNC.STDEV.S.md), and it encodes a statement about what the data *is*: `STDEV.P` treats
the supplied values as the **whole population** — every member, nothing outside — rather than as
a sample drawn from something larger.

Two ways to see what the `n` divisor means:

1. **It is the exact second central moment of the empirical distribution.** Put mass `1/n` on
   each value; `sigma^2` is the variance of that distribution, full stop. Under this reading
   `STDEV.P` is not an *estimator* of anything and there is nothing to correct for. It is a
   descriptive summary of the numbers in front of you, computed exactly.
2. **As an estimator of a larger population's variance it is biased downward.** If the `x_i` were
   a sample, `E[sigma^2] = ((n-1)/n) * sigma_true^2`, an underestimate, because the deviations are
   taken about the fitted sample mean rather than the true one. That is the shortfall
   [STDEV.S](FUNC.STDEV.S.md)'s `n - 1` repairs.

The relationship between the two is exact:

    STDEV.P  =  STDEV.S * sqrt( (n-1) / n )

so `STDEV.P` is always the smaller, by a factor that approaches `1` as `n` grows and that matters
most on the small data sets spreadsheets usually hold. On five values the gap is about eleven
per cent — large enough to change a reported figure, small enough to go unnoticed.

**`STDEV.P` is defined for a single data point**, where `STDEV.S` is not: `n = 1` gives a
deviation of exactly zero and a standard deviation of exactly `0`. That is mathematically correct
(a one-member population has no spread) and statistically vacuous, and it is a real difference in
the error surface of the two functions rather than a curiosity.

The result is in the **units of the data**. Its invariances are the same as
[STDEV.S](FUNC.STDEV.S.md)'s: unchanged by adding a constant to every value, multiplied by `|c|`
when every value is scaled by `c`. Shift invariance is what the numerical test below exploits.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `number1` | The first value, range or array. | yes |
| `number2`, … | Further values, ranges or arrays. | optional, up to the declared maximum |

The arguments form a flat bag: everything is flattened and concatenated, and argument boundaries
have no effect.

`STDEV.P` is an **aggregate**, not a lift kernel, and carries the dual coercion policy of
[coercion and lifting](../model/02-coercion-and-lifting.md):

- **Reached by scanning a range or array**: only numbers count. Text, logical values and blank
  cells are **skipped** — not converted to zero, and not counted toward `n`.
- **Passed directly as a scalar argument**: ordinary to-number coercion applies, so
  `STDEV.P(1,2,"3")` admits `"3"` as `3` and `STDEV.P(1,2,TRUE)` admits `TRUE` as `1`, where the
  same values in scanned cells would be ignored.

[STDEVPA](FUNC.STDEVPA.md) is the same estimator with the opposite range policy: text in a range
counts as `0` there rather than being skipped. The two functions therefore compute over different
data sets and can differ by a great deal.

## Result and edge cases

Returns `Number`, non-negative, in the units of the data.

- **No numeric values at all.** `#DIV/0!`.
- **Exactly one numeric value.** Exactly `0`. This is where `STDEV.P` and
  [STDEV.S](FUNC.STDEV.S.md) part company: the same single-cell range gives `0` here and
  `#DIV/0!` there.
- **All values equal.** Exactly `0`; each deviation is exactly zero and `sqrt(0) = +0`.
- **Two values.** `sigma = |x_1 - x_2| / 2`, exactly representable when the difference is.
- **A blank cell inside a scanned range** is skipped, so `n` is smaller than the column height.
- **Errors anywhere** propagate as themselves.
- The result can overflow to an infinity when the squared deviations do — see Numerical notes.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#DIV/0!` | no numeric data points | documented and implemented |
| `#VALUE!` | direct text that does not parse as a number | shared coercion rule |
| propagated | An error value in any argument surfaces as that error | shared coercion rule |

Note the shorter table compared with [STDEV.S](FUNC.STDEV.S.md): with an `n` divisor there is no
`n = 1` failure, so the only way to reach `#DIV/0!` is to supply nothing numeric at all.

## Relationships

- **[STDEV.S](FUNC.STDEV.S.md)** is the sample form with the `n - 1` divisor, related exactly by
  `STDEV.P = STDEV.S * sqrt((n-1)/n)`. Choosing between them is a statistical decision about what
  the data represents, not a stylistic one. The common workbook error is using `STDEV.P` because
  it gives the smaller number.
- **[VAR.P](FUNC.VAR.P.md)** is the square: `STDEV.P = SQRT(VAR.P)`, and that is literally how
  the reference engine computes it.
- **[STDEVPA](FUNC.STDEVPA.md)** uses the same `n` divisor with a different inclusion rule — text
  in a range counts as `0`, logicals as `1`/`0`. Same estimator, different data set.
- **[STDEVP](FUNC.STDEVP.md)** is the legacy Compatibility-category name. Microsoft's replacement
  guidance points `STDEVP` at `STDEV.P`, and the two are documented as computing the same
  quantity. **The Handbook does not treat that as an identity.** A legacy alias and its modern
  name are two registered entry points, and Excel is free to route them to different code; that
  they agree is a claim requiring evidence, and none is in the Handbook's record. The variance
  algorithms were changed at Excel 2003 with both names still in the product, which is exactly
  the situation in which an unverified alias assumption is worth least.
- **[AVERAGE](FUNC.AVERAGE.md)** supplies `xbar` and shares the scan policy exactly.
- **[DEVSQ](FUNC.DEVSQ.md)** returns the numerator alone, and is the right diagnostic when a
  discrepancy might be a divisor question rather than an algorithm question.
- **[SKEW.P](FUNC.SKEW.P.md)** uses this same `sigma` as its standardising scale, which is why
  the population skewness and the population standard deviation fail on the same data.
- **[Z.TEST](FUNC.Z.TEST.md)** and the normal-theory functions consume a population `sigma` when
  one is genuinely known — the case `STDEV.P` is actually for.

## Numerical notes

Everything on [STDEV.S](FUNC.STDEV.S.md)'s numerical notes applies here, because the two functions
share one kernel and differ only in the final divisor. The summary, with the points that are
specific to the population form.

**Two-pass centred, not raw power sums.** The reference engine computes `xbar` first and then
accumulates `sum (x_i - xbar)^2` in a second pass. The textbook one-pass identity

    sum (x - xbar)^2  =  sum x^2  -  (sum x)^2 / n

is algebraically equal and catastrophically unstable: when the mean is large relative to the
spread it subtracts two nearly-equal large numbers, losing roughly `2*log10(xbar/sigma)`
significant digits, and can return a **negative** sum of squares — the signature by which this
bug is identified. Chan, Golub and LeVeque (1983) give the analysis; Welford (1962) gives the
stable single-pass alternative for cases where the data cannot be stored.

**The published record on Excel.** McCullough and Wilson's assessments in *Computational
Statistics & Data Analysis* from 1999 onward, Knüsel's parallel work, and Welinder's writing on
Excel's statistical functions arising from the Gnumeric project are the standing published record
on Excel's variance accuracy, and the algorithms were changed at Excel 2003 in response. **This
page asserts nothing about what Excel's `STDEV.P` does internally today.** It asserts that the
two families of algorithm are distinguishable by a measurement, that the measurement is the shift
test below, and that the Handbook has not made it.

**Specific to the population divisor.** `sumsq / n` is a division by an exactly representable
integer, and for `n` a power of two it is exact. `STDEV.S`'s `sumsq / (n-1)` has no such property
in general. This makes `STDEV.P` the marginally cleaner of the two, and it also means the exact
identity `STDEV.P = STDEV.S * sqrt((n-1)/n)` will not hold bit for bit even when both functions
are computed correctly: the two divisions round differently and the correcting square root adds
its own rounding. A suite asserting that identity must assert it to a tolerance, and the size of
that tolerance is itself worth measuring.

**Residual hazards.** The mean and the sum of squares are both accumulated by naive
left-to-right summation, so their error grows with `n` where compensated or pairwise summation
would bound it; the reference engine uses neither. The sum of squares has non-negative terms and
is therefore well-conditioned. The final `sqrt` is correctly rounded and *halves* the relative
error of the variance, so the whole accuracy question lives in the variance. Squared deviations
overflow above roughly `1.3 * 10^154` and underflow to zero below roughly `10^{-162}`, so
`STDEV.P` can report an infinity or an exact zero for finite, nonzero-spread data; a scale-aware
formulation that factors out the largest deviation first would avoid both, and is not used.

**Reproducibility.** Summation order is part of the answer, so two correct implementations that
traverse or vectorise differently will disagree in the last bits.

## What has not been checked

No Handbook vector suite exists for `STDEV.P`, and no Handbook evidence record names `STDEV.P` as
a subject. Nobody has checked this function against Excel within the Handbook's record. The
implementing module carries no tests of its own in the reference engine; the shared variance
kernel it calls carries a small number, which speaks to that kernel rather than to this surface.

Everything above marked as documented comes from Microsoft's `STDEV.P` page. **Retrieval of that
page was blocked by the upstream host on this pass**, so those statements are recorded as
documented behaviour with the source named and should be re-read against the live page.

Inputs worth probing first:

1. **The shift test.** A small data set with a known exact answer, evaluated as given and then
   with `10^6`, `10^9` and `10^{12}` added to every value. The true answer is invariant; how the
   returned values degrade identifies the algorithm. This is the highest-value measurement on the
   page and it takes four cells.
2. **A single numeric cell.** `STDEV.P` should return `0` where [STDEV.S](FUNC.STDEV.S.md)
   returns `#DIV/0!`. This is the cheapest probe that the two functions are genuinely different
   surfaces rather than one with a parameter.
3. **`STDEV.P` against `SQRT(VAR.P)`**, bitwise — a structural fingerprint of whether Excel
   routes through the variance.
4. **`STDEV.P` against `STDEV.S * SQRT((n-1)/n)`**, measuring the gap the exact identity leaves
   after three roundings. The size of that gap is the tolerance a suite would have to allow.
5. **`STDEV.P` against the legacy `STDEVP`** on the same data, bitwise — the probe that would
   turn the documented replacement relationship into an evidenced one.
6. **An empty range and a range of only text**, confirming `#DIV/0!` and not `#VALUE!`.
7. **The scan/direct split**: `STDEV.P(1,2,"3",TRUE)` against the same four values in cells,
   which should give different answers and different `n`.
8. **Overflow and underflow**: deviations near `10^{154}` and near `10^{-162}`.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| population divisor | The plain `n`, with no Bessel correction |
| second central moment | The exact variance of the empirical distribution on the supplied values |
| two-pass centred form | Compute the mean, then accumulate squared deviations; the stable route |
| raw sums-of-squares form | `sum x^2 - (sum x)^2/n`; the unstable one-pass textbook formula |
| shift test | Adding a large constant to every value; the true answer is invariant, the computed one need not be |

## Sources

- Microsoft, *STDEV.P function* —
  <https://support.microsoft.com/en-us/office/stdev-p-function-6e917c05-31a0-496f-ade7-4f4e7462f285>
  (signature, the stated equation with the `n` divisor, the rule that text and logicals in
  scanned ranges are ignored while direct arguments are converted, and the `#DIV/0!` condition).
  Retrieval was blocked by the upstream host for this page; the documented behaviour above is
  stated as documented behaviour and should be re-checked against the page.
- T. F. Chan, G. H. Golub and R. J. LeVeque, "Algorithms for Computing the Sample Variance:
  Analysis and Recommendations", *The American Statistician* 37(3), 1983, pp. 242–247.
- B. P. Welford, "Note on a Method for Calculating Corrected Sums of Squares and Products",
  *Technometrics* 4(3), 1962, pp. 419–420.
- B. D. McCullough and B. Wilson, "On the accuracy of statistical procedures in Microsoft Excel",
  *Computational Statistics & Data Analysis*, 1999, and the follow-up assessments through 2005;
  L. Knüsel's parallel work; M. Welinder's writing on Excel's statistical functions arising from
  the Gnumeric project.
- N. J. Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd ed., chapter 4.
- OxFunc `crates/oxfunc_core/src/functions/stdev_p_fn.rs` and
  `crates/oxfunc_core/src/functions/variance_common.rs` at commit `473efa3` — the two-pass
  centred variance, the population divisor, the empty-input `#DIV/0!` guard, and the final
  `sqrt`.
- Handbook [coercion and lifting](../model/02-coercion-and-lifting.md) — the direct-argument
  versus range-scan dual policy.
- Handbook `data/functions/FUNC.STDEV.P.json`, `data/presence/FUNC.STDEV.P.json`.
