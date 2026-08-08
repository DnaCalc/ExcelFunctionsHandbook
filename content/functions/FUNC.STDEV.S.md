---
schema: efh.function-page/v1
function_id: FUNC.STDEV.S
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — STDEV.S function"
    locator: "https://support.microsoft.com/en-us/office/stdev-s-function-7d69cf97-0c1f-4acf-be27-f3e83904cc23"
    role: "documented signature, the stated equation with the n-1 divisor, the scan rules, and the #DIV/0! condition"
  - work: "Chan, T. F., Golub, G. H. and LeVeque, R. J., \"Algorithms for Computing the Sample Variance: Analysis and Recommendations\""
    locator: "The American Statistician 37(3), 1983, pp. 242-247"
    role: "the error analysis of the textbook one-pass formula against the two-pass and updating forms"
  - work: "Welford, B. P., \"Note on a Method for Calculating Corrected Sums of Squares and Products\""
    locator: "Technometrics 4(3), 1962, pp. 419-420"
    role: "the single-pass updating recurrence that is stable without storing the data"
  - work: "McCullough, B. D. and Wilson, B., \"On the accuracy of statistical procedures in Microsoft Excel\""
    locator: "Computational Statistics & Data Analysis, 1999 and the follow-up assessments through 2005"
    role: "the published record of Excel's variance accuracy and of the algorithm change at Excel 2003"
  - work: "OxFunc — variance_common.rs"
    locator: "crates/oxfunc_core/src/functions/variance_common.rs"
    role: "reference-engine kernel: the two-pass centred variance and the sample/population divisor split"
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
family: stdev_s_fn
role_in_family: >-
  The sample standard deviation: Bessel's n-1 divisor, numeric cells only, and the default
  choice when the data at hand is a sample from something larger.
---

# STDEV.S

## What it computes

`STDEV.S(number1, [number2], …)` returns the **sample standard deviation** of the numeric values
it is given: the square root of the unbiased sample variance.

For values `x_1, …, x_n` with mean `xbar`,

    s^2  =  ( 1 / (n - 1) )  *  sum_{i=1..n} (x_i - xbar)^2
    s    =  sqrt( s^2 )

and `STDEV.S` returns `s`. The `n - 1` is **Bessel's correction**, and it is the whole difference
between this function and [STDEV.P](FUNC.STDEV.P.md).

The correction has a precise justification and a precise limit, both worth stating because the
second is routinely forgotten:

- **`s^2` is an unbiased estimator of the population variance.** If the `x_i` are independent
  draws from a distribution with variance `sigma^2`, then `E[s^2] = sigma^2` exactly. Dividing by
  `n` instead would give `E[m_2] = ((n-1)/n) * sigma^2`, an underestimate, because the deviations
  are taken about the *sample* mean, which is itself fitted to the data and therefore sits closer
  to the points than the true mean does. One degree of freedom has been spent estimating the
  centre, and `n - 1` is what remains.
- **`s` itself is not unbiased for `sigma`.** The square root is a strictly concave function, so
  by Jensen's inequality `E[s] = E[sqrt(s^2)] < sqrt(E[s^2]) = sigma`. The sample standard
  deviation systematically *underestimates* the population standard deviation, and no choice of
  divisor fixes this — the unbiasing correction for `s` depends on the distribution (for a normal
  population it is the ratio of two gamma functions, the `c4` factor of quality-control practice).
  Excel has no function for it. `STDEV.S` is the square root of an unbiased estimator, which is
  not the same as an unbiased estimator, and every textbook that says otherwise is wrong.

The quantity is in the **units of the data**, unlike the variance, which is in units squared.
That is the only reason the standard deviation is preferred to the variance in reporting, and it
is why `STDEV.S` needs the square root at all.

Two invariances fix its behaviour: `STDEV.S` is unchanged by adding a constant to every value,
and is multiplied by `|c|` when every value is scaled by `c`. The first of those — shift
invariance — is the property the numerical notes below turn into a test.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `number1` | The first value, range or array. | yes |
| `number2`, … | Further values, ranges or arrays. | optional, up to the declared maximum |

The arguments form a flat bag: everything is flattened and concatenated into one data set, and
the argument boundaries have no effect on the answer.

`STDEV.S` is an **aggregate**, not a lift kernel, and it carries the dual coercion policy of
[coercion and lifting](../model/02-coercion-and-lifting.md):

- **Reached by scanning a range or array**: only numbers count. Text, logical values and blank
  cells are **skipped** — they are not converted to zero and they do not enlarge `n`.
- **Passed directly as a scalar argument**: ordinary to-number coercion applies, so
  `STDEV.S(1,2,"3")` admits the text `"3"` as `3`, and `STDEV.S(1,2,TRUE)` admits `TRUE` as `1`,
  where the same values in scanned cells would be ignored.

This is the split that separates `STDEV.S` from [STDEVA](FUNC.STDEVA.md), which counts text in a
range as zero rather than skipping it. The two functions can give very different answers on the
same range, and the difference is not a rounding matter — it is a different data set.

## Result and edge cases

Returns `Number`, non-negative, in the units of the data.

- **Fewer than two numeric values.** `#DIV/0!` — the `n - 1` divisor is zero at `n = 1` and the
  sample variance is genuinely undefined for a single observation. This is documented.
- **No numeric values at all.** `#DIV/0!`.
- **All values equal.** Exactly `0`. Each deviation is exactly zero, their squares are exactly
  zero, and `sqrt(0) = +0`.
- **Two values.** `s = |x_1 - x_2| / sqrt(2)`, which is exact up to the two roundings of the
  subtraction (exact by Sterbenz when the operands are close) and the square root.
- **A blank cell inside a scanned range** is skipped, so `STDEV.S` over a column with gaps uses
  a smaller `n` than the column height. Compare [STDEVA](FUNC.STDEVA.md), which also skips blanks
  but not text.
- **Errors anywhere** propagate as themselves.
- The result is never negative and never `NaN` for finite input; it can overflow to an infinity
  if the deviations are large enough that their squares overflow — see Numerical notes.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#DIV/0!` | fewer than two numeric data points | documented and implemented |
| `#VALUE!` | direct text that does not parse as a number | shared coercion rule |
| propagated | An error value in any argument surfaces as that error | shared coercion rule |

## Relationships

- **[STDEV.P](FUNC.STDEV.P.md)** is the population form, dividing by `n`. The two are related
  exactly: `STDEV.P = STDEV.S * sqrt((n-1)/n)`, so the population value is always the smaller.
  Choosing between them is a statistical question — is this data the whole population, or a
  sample from a larger one — and not a matter of preference.
- **[VAR.S](FUNC.VAR.S.md)** is the square of this function: `STDEV.S = SQRT(VAR.S)`, and that
  is literally how the reference engine computes it. Anything true of `VAR.S`'s accuracy is true
  of `STDEV.S`'s, halved in relative terms by the square root.
- **[STDEVA](FUNC.STDEVA.md)** uses the same `n - 1` divisor with a different inclusion rule:
  text in a range counts as `0` and logicals count as `1`/`0`. It is the same estimator over a
  different data set.
- **[STDEV](FUNC.STDEV.md)** is the legacy Compatibility-category name. Microsoft's replacement
  guidance points `STDEV` at `STDEV.S`, and the two are documented as computing the same
  quantity. **The Handbook does not treat that as an identity.** A legacy alias and its modern
  name are two registered entry points, and Excel is free to route them to two different code
  paths; that they agree — let alone that they agree in the last bits — is a claim requiring
  evidence, and no such evidence is in the Handbook's record. This pair is a particularly live
  question because the variance algorithms were changed at Excel 2003 while both names remained
  in the product.
- **[AVERAGE](FUNC.AVERAGE.md)** supplies `xbar` and shares the scan policy exactly, so
  `COUNT` and `AVERAGE` over the same range see the same `n` and the same values `STDEV.S` does.
- **[DEVSQ](FUNC.DEVSQ.md)** returns the numerator `sum (x - xbar)^2` on its own, and is the
  right function to inspect when diagnosing which of the two divisors a discrepancy came from.
- **[SKEW](FUNC.SKEW.md) and [KURT](FUNC.KURT.md)** use this same `s` as their standardising
  scale.
- **[CONFIDENCE.T](FUNC.CONFIDENCE.T.md)** and the `t`-tests consume `s` as the estimated scale;
  the `n - 1` here and the `n - 1` degrees of freedom there are the same degree of freedom.

## Numerical notes

This is the function on which Excel's statistical numerics were publicly litigated, and the
argument is entirely about **which algebraically-equal formula gets evaluated**.

**The textbook one-pass formula is dangerous.** Expanding the square gives

    sum (x - xbar)^2  =  sum x^2  -  ( sum x )^2 / n

which needs a single pass and two running totals. Every statistics text of the mechanical-calculator
era printed it, and it is catastrophically unstable. When the mean is large relative to the
spread, `sum x^2` and `(sum x)^2/n` are two nearly-equal large numbers whose difference is the
small quantity you actually want. The number of significant digits lost is roughly
`2*log10(xbar/s)`: for data around `10^9` with a spread of `1`, all of them. Worse, the computed
difference can come out **negative** — an impossibility for a sum of squares — and the square root
of a negative number is where the failure becomes visible instead of merely wrong. That negative
sum of squares is the classic signature of this bug, and it is how it was identified in
spreadsheet products.

**The two-pass centred form is the standard fix**, and it is what the reference engine uses:
compute `xbar` first, then accumulate `sum (x_i - xbar)^2` in a second pass. Each deviation is
formed before squaring, so no large cancellation occurs; the error grows with the condition
number of the problem rather than with its square. Chan, Golub and LeVeque (1983) give the
analysis and the recommendation, and their paper remains the reference. Where a single pass is
required — streaming data, no storage — **Welford's (1962) updating recurrence** achieves
comparable stability by maintaining the running mean and the running corrected sum of squares
together, and is what a well-written online implementation uses.

**The published record on Excel.** McCullough and Wilson's assessments of Excel's statistical
procedures, running from 1999 through the 2000s in *Computational Statistics & Data Analysis*,
Knüsel's parallel work, and Welinder's writing on Excel's statistical functions in the course of
the Gnumeric project, together form the standing published record on exactly this question; the
variance and regression algorithms were changed at Excel 2003 in response. **This page does not
assert what Excel's `STDEV.S` does internally today.** What it asserts is that the two families
of algorithm are distinguishable by a measurement anyone can make, that the measurement is the
shift test described below, and that the Handbook has not made it.

**Residual hazards in the good form.** The two-pass form is far better, not perfect:

- The mean is accumulated by naive left-to-right summation, so `xbar` carries an error growing
  with `n`; compensated (Kahan) or pairwise summation would bound it by a constant. The reference
  engine uses neither. Since every deviation is taken against this `xbar`, an error in it biases
  the whole sum of squares — though only in second order, because the sum of squares is
  stationary at the true mean.
- The sum of squares is also accumulated naively, but its terms are all non-negative, so it is
  well-conditioned and the error is benign.
- `sqrt` is correctly rounded under IEEE-754, contributing at most half an ulp, and it *halves*
  the relative error of the variance. The whole accuracy question therefore lives in the variance,
  and the square root is the cheapest and most accurate step in the function.
- Overflow: `(x - xbar)^2` overflows when a deviation exceeds about `1.3 * 10^154`, so
  `STDEV.S` can report an infinity for data that is itself perfectly finite. A scale-aware
  implementation would factor out the largest deviation first, as `hypot` does. The reference
  engine does not.
- Underflow: deviations below about `10^{-162}` square to zero, so a genuinely tiny spread is
  reported as exactly `0` rather than as a small number.

**Reproducibility.** Summation order is part of the answer. Two implementations that both use the
centred form and are both correct will disagree in the last bits if they traverse the data
differently, vectorise differently, or use extended-precision accumulators. Nothing in the
mathematics fixes those bits, which is why a portable-reproducible flavour of this function is a
distinct engineering artefact and not a tidier version of the same code.

## What has not been checked

No Handbook vector suite exists for `STDEV.S`, and no Handbook evidence record names `STDEV.S` as
a subject. Nobody has checked this function against Excel within the Handbook's record. The
implementing module carries no tests of its own in the reference engine; the shared variance
kernel it calls carries a small number, which speaks to that kernel rather than to this surface.

Everything above marked as documented comes from Microsoft's `STDEV.S` page. **Retrieval of that
page was blocked by the upstream host on this pass**, so those statements are recorded as
documented behaviour with the source named and should be re-read against the live page.

Inputs worth probing first:

1. **The shift test.** Take a small data set with a known exact standard deviation and evaluate
   `STDEV.S` on it, then on the same values with `10^6`, `10^9` and `10^{12}` added to every one.
   The true answer is identical in all four; the returned values are not, and *how* they degrade
   identifies the algorithm. Gentle degradation means a centred form; collapse to zero, to noise,
   or to an error means the raw sums-of-squares form. This is the single highest-value
   measurement on this page, it takes four cells, and nobody in the Handbook's record has run it.
2. **The negative-sum-of-squares signature.** The classic constructed data sets from the
   published accuracy literature — values differing only in their low-order digits at a large
   offset — where the one-pass form produces a negative sum of squares.
3. **`STDEV.S` against `SQRT(VAR.S)`** on the same data, bitwise. The reference engine computes
   exactly that; whether Excel does is a structural fingerprint.
4. **`STDEV.S` against `STDEV.P * SQRT(n/(n-1))`**, bitwise. The identity is exact in real
   arithmetic; the gap measures the two divisor paths against each other with no external oracle.
5. **`STDEV.S` against the legacy `STDEV`** on the same data, bitwise. The probe that would turn
   the documented replacement relationship into an evidenced one — worth running precisely
   because the algorithm change at Excel 2003 makes it non-obvious.
6. **`n = 1` and `n = 0`**, confirming the `#DIV/0!` boundary, and `n = 2`, the smallest defined
   case.
7. **The scan/direct split**: `STDEV.S(1,2,"3",TRUE)` against the same four values placed in
   cells, which should give different answers and different `n`.
8. **Overflow and underflow**: deviations near `10^{154}` and near `10^{-162}`, against the
   squared-deviation limits above.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| Bessel's correction | The `n - 1` divisor, which makes the variance unbiased |
| unbiased for the variance | `E[s^2] = sigma^2`; note this does not make `s` unbiased for `sigma` |
| two-pass centred form | Compute the mean, then accumulate squared deviations; the stable route |
| raw sums-of-squares form | `sum x^2 - (sum x)^2/n`; the unstable one-pass textbook formula |
| Welford updating | The stable single-pass recurrence for mean and corrected sum of squares |
| shift test | Adding a large constant to every value; the true answer is invariant, the computed one need not be |

## Sources

- Microsoft, *STDEV.S function* —
  <https://support.microsoft.com/en-us/office/stdev-s-function-7d69cf97-0c1f-4acf-be27-f3e83904cc23>
  (signature, the stated equation with the `n-1` divisor, the rule that text and logicals in
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
- N. J. Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd ed., chapter 4 — summation
  error bounds and compensated summation.
- OxFunc `crates/oxfunc_core/src/functions/stdev_s_fn.rs` and
  `crates/oxfunc_core/src/functions/variance_common.rs` at commit `473efa3` — the two-pass
  centred variance, the sample divisor, the `n < 2` `#DIV/0!` guard, and the final `sqrt`.
- Handbook [coercion and lifting](../model/02-coercion-and-lifting.md) — the direct-argument
  versus range-scan dual policy this aggregate carries.
- Handbook `data/functions/FUNC.STDEV.S.json`, `data/presence/FUNC.STDEV.S.json`.
