---
schema: efh.function-page/v1
function_id: FUNC.SKEW
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — SKEW function"
    locator: "https://support.microsoft.com/en-us/office/skew-function-bdf49d86-b1ef-4804-a046-28eaea69c9fa"
    role: "documented signature, the stated equation, and the documented #DIV/0! condition"
  - work: "Abramowitz, M. and Stegun, I. A., Handbook of Mathematical Functions"
    locator: "chapter 26, section 26.1 (moments, standardised moments and the shape coefficients)"
    role: "the standard-moment vocabulary the skewness coefficients are defined in"
  - work: "Joanes, D. N. and Gill, C. A., \"Comparing measures of sample skewness and kurtosis\""
    locator: "Journal of the Royal Statistical Society Series D 47(1), 1998, pp. 183-189"
    role: "the three competing sample skewness estimators and the exact algebraic relations between them"
  - work: "OxFunc — moment_stats_family.rs"
    locator: "crates/oxfunc_core/src/functions/moment_stats_family.rs"
    role: "reference-engine kernel: the two-pass centred moments and the per-element standardisation"
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
family: moment_stats_family
role_in_family: >-
  The sample-corrected third standardised moment: the family's asymmetry measure under the
  bias-adjusted convention, requiring at least three points.
---

# SKEW

## What it computes

`SKEW(number1, [number2], …)` returns the **sample skewness** of the values it is given: a
dimensionless measure of the asymmetry of their distribution about its mean, under the
bias-corrected convention.

For values `x_1, …, x_n` with mean `xbar` and *sample* standard deviation

    s  =  sqrt( sum (x_i - xbar)^2 / (n - 1) )

the returned quantity is

    SKEW  =  G1  =  ( n / ((n-1)(n-2)) )  *  sum_{i=1..n} ( (x_i - xbar) / s )^3

This is the **adjusted Fisher–Pearson standardised moment coefficient**, conventionally written
`G1`. It is the estimator obtained by correcting the raw third standardised moment so that it is
unbiased for the population skewness when the underlying data is normal.

Its meaning is the classical one from the moment vocabulary of Abramowitz & Stegun chapter 26:
the third standardised moment `E[((X - mu)/sigma)^3]` measures which tail is longer. `SKEW`
returns zero for any symmetric sample, positive when the right tail is longer (mass bunched at
the low end, a few large values), and negative for the mirror image. It is invariant under any
positive affine change of units — shift the data, scale it by a positive factor, and `SKEW` does
not move — which is what makes it comparable across variables measured in different units.

**Skewness has more than one sample definition, and Excel publishes two of them.** The
uncorrected form, the plain third standardised moment about the sample mean using the
*population* standard deviation, is

    g1  =  m3 / m2^(3/2)  ,   where  m_k = (1/n) sum (x_i - xbar)^k

and that is what [SKEW.P](FUNC.SKEW.P.md) returns. The two are related by an exact algebraic
identity:

    G1  =  g1  *  sqrt( n (n-1) ) / (n - 2)

so `SKEW` is always the larger in magnitude, and the inflation factor tends to `1` as `n` grows —
it is above `1.1` only for small samples. Joanes and Gill (1998) catalogue this estimator and its
competitors and give the relations among them; `G1` is the one used by SAS and SPSS, and `g1` is
the one used by many textbooks. Neither is wrong. A reader comparing Excel against another
package should establish which convention that package used before calling anything a
discrepancy.

The estimator is **bounded**, a fact that is easy to miss and useful to know. For the uncorrected
form the classical bound is `|g1| <= (n-2)/sqrt(n-1)`, attained when one observation sits apart
from `n-1` identical ones. Substituting into the identity above gives

    |SKEW|  <=  sqrt(n)

So a sample of five can never produce a skewness beyond about `2.24`, however extreme it looks.
Small-sample skewness estimates are not merely noisy; they are structurally incapable of
reporting the asymmetry that a heavy-tailed population actually has. Any workflow that flags
"skewness above 2" as a data-quality signal is measuring the sample size as much as the shape.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `number1` | The first value, range or array. | yes |
| `number2`, … | Further values, ranges or arrays. | optional, up to the declared maximum |

The arguments are a flat bag: everything is flattened and concatenated into one data set, and the
argument boundaries have no effect on the answer.

`SKEW` is an **aggregate**, not a lift kernel, and it carries the dual coercion policy described
in [coercion and lifting](../model/02-coercion-and-lifting.md):

- **Values reached by scanning a range or array** are used only if they are numbers. Text,
  logical values and blank cells are skipped — they do not become zeros and they do not enlarge
  `n`.
- **Values passed directly as scalar arguments** go through ordinary to-number coercion, so
  `SKEW(1,2,"3")` admits the text `"3"` as `3` where the same text in a scanned cell would be
  ignored. Direct text that does not parse is `#VALUE!`.

That asymmetry is engine-wide policy, not something specific to `SKEW`, and it is the single most
common source of "the same data gives two answers" reports across the whole statistical category.

## Result and edge cases

Returns `Number`, dimensionless.

- **Fewer than three numeric values.** `#DIV/0!` — the `(n-2)` factor in the denominator is
  undefined at `n = 2` and the estimator has no meaning below that. This matches Microsoft's
  documented condition.
- **All values equal.** The centred sum of squares is zero, `s = 0`, and the standardisation is
  `0/0`. `#DIV/0!`, documented.
- **A symmetric sample.** `SKEW` is zero, and for data symmetric about its mean it is exactly
  zero only when the positive and negative cubes cancel bit for bit — which for a two-sided
  symmetric set in binary64 they generally do, since each `+d^3` has an exact `-d^3` partner. It
  is worth knowing that this exactness is a property of the summation order and the data, not a
  guarantee.
- **Non-numeric cells in a scanned range** are skipped and do not enlarge `n`.
- **Errors anywhere** propagate as themselves.
- The result is not bounded by any fixed constant, but is bounded by `sqrt(n)` as above.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#DIV/0!` | fewer than three data points | documented and implemented |
| `#DIV/0!` | the sample standard deviation is zero | documented and implemented |
| `#VALUE!` | direct text that does not parse as a number | shared coercion rule |
| propagated | An error value in any argument surfaces as that error | shared coercion rule |

## Relationships

- **[SKEW.P](FUNC.SKEW.P.md)** is the population-convention sibling, `g1`, related by
  `SKEW = SKEW.P * sqrt(n(n-1))/(n-2)`. It is not a legacy/modern pair and neither supersedes the
  other: they are two different estimators of the same population quantity, and Excel names both.
  Note the domain difference — `SKEW` requires `n >= 3` by construction, and the reference
  engine's [SKEW.P](FUNC.SKEW.P.md) does not.
- **[KURT](FUNC.KURT.md)** is the fourth-moment companion, with the same shape of bias correction
  and the same family kernel. `KURT` requires `n >= 4` for the same structural reason `SKEW`
  requires `n >= 3`.
- **[STDEV.S](FUNC.STDEV.S.md)** supplies the `s` in the denominator; `SKEW` is undefined exactly
  where `STDEV.S` is zero. **[AVERAGE](FUNC.AVERAGE.md)** supplies `xbar`.
- **[STANDARDIZE](FUNC.STANDARDIZE.md)** performs the same `(x - mu)/sigma` map that `SKEW`
  applies internally, one value at a time.
- **[SKEW](FUNC.SKEW.md) has no Compatibility-category predecessor.** It is one of the older
  statistical functions that was left undotted when the `.P`/`.S` reorganisation happened, so
  `SKEW` is both the historical name and the current one, and `SKEW.P` is the newcomer. This is
  the reverse of the usual pattern and is worth stating because migration guides that say "prefer
  the dotted name" do not apply here.
- Readers confuse `SKEW` with a normality test. It is not one: a skewness near zero is consistent
  with many non-normal distributions, and the sampling distribution of `G1` under normality has
  a standard error near `sqrt(6/n)`, which is large for the sample sizes spreadsheets usually
  hold.

## Numerical notes

`SKEW` is a third moment, and third moments amplify every error the mean carries. The reference
engine's kernel is worth reading closely because it makes two good choices and one that is merely
conventional.

**Two-pass centring, not raw power sums.** The kernel computes `xbar` first, then accumulates
`sum (x_i - xbar)^2` and the standardised cubes in a second pass over stored deviations. The
alternative — accumulating `sum x`, `sum x^2`, `sum x^3` and assembling the central moments
algebraically — is the form that appears in textbooks and in one-pass streaming code, and it is
much worse. The third central moment obtained that way is

    m3_raw  =  (1/n) sum x^3  -  3*xbar*(1/n) sum x^2  +  2*xbar^3

which is a difference of three large, nearly-cancelling quantities when `xbar` is large relative
to the spread. Digits lost go as the *cube* of the mean-to-spread ratio, so where a variance
computed this way loses half its digits, a third moment loses all of them. Chan, Golub and
LeVeque's analysis for the variance case is the standard reference; the third-moment case is
strictly worse and the same recommendation applies. The reference engine does the right thing
here.

**Per-element standardisation rather than a final scaling.** The kernel divides each deviation by
`s` and then cubes, rather than cubing the deviations and dividing the sum by `s^3`. This costs
`n` divisions instead of one, and buys range: `((x-xbar)/s)^3` stays near unit scale for typical
data, whereas `(x - xbar)^3` overflows for deviations beyond about `10^102` and underflows below
about `10^{-102}`. Data at those scales is rare but reachable, and the chosen form is the robust
one. It is the same argument that makes hypot-style scaling standard practice.

**What remains.**

- The mean and both accumulations are naive left-to-right sums, so their error grows with `n`
  rather than being bounded by a constant. Compensated summation would fix this and is not used.
- The `s = 0` guard is an exact comparison of the centred sum of squares against zero, not a
  tolerance. Data with a real but vanishing spread divides by a denormal and produces an enormous
  standardised cube rather than a diagnostic. There is no correct answer here, only a choice.
- Cancellation still bites when the data has a large mean relative to its spread, because
  `x_i - xbar` is a subtraction of nearby numbers even in the two-pass form. The two-pass form
  reduces the damage from catastrophic to proportionate; it does not eliminate it. A classic
  test case is the same values shifted by a large constant: the true skewness is invariant, so
  any change in the answer is pure floating-point residue.
- Summation order is part of the answer. Two correct implementations traversing the data
  differently will disagree in the last bits, and nothing in the mathematics fixes those bits.

**Where the identity helps.** `SKEW = SKEW.P * sqrt(n(n-1))/(n-2)` is exact in real arithmetic
and is a strong cross-check: computing both and comparing the ratio against the closed form
exercises the two kernels against each other with no external oracle. Because the reference
engine computes them by parallel code paths with different denominators, agreement is
informative and disagreement localises immediately.

## What has not been checked

No Handbook vector suite exists for `SKEW`, and no Handbook evidence record names `SKEW` as a
subject. Nobody has checked this function against Excel within the Handbook's record.

The reference engine's implementing module — shared with [KURT](FUNC.KURT.md),
[SKEW.P](FUNC.SKEW.P.md), [STEYX](FUNC.STEYX.md) and [TRIMMEAN](FUNC.TRIMMEAN.md) — is named in
an upstream defect stream on statistical numeric exactness drift. That stream is an upstream
register of open work, not a Handbook measurement of this surface, and no figure from it attaches
to this page. What it does tell a reader is that the module has known open numeric questions,
which is a reason to treat `SKEW`'s last bits as unsettled rather than assumed.

Everything above marked as documented comes from Microsoft's `SKEW` page. **Retrieval of that
page was blocked by the upstream host on this pass**, so those statements are recorded as
documented behaviour with the source named and should be re-read against the live page.

Inputs worth probing first:

1. **The shift-invariance test.** A fixed data set, and the same set with a large constant added
   to every value. The true skewness is identical; any difference in the returned bits measures
   the cancellation in the centring step and is the sharpest available fingerprint of which
   algorithm Excel uses. Run it at several offsets — the raw-power-sum form degrades
   catastrophically and the centred form degrades gently.
2. **`SKEW` against `SKEW.P * SQRT(n*(n-1))/(n-2)`** on the same data, bitwise. The identity is
   exact; the gap is pure implementation residue and localises any disagreement to one of the two
   kernels.
3. **Exactly three points**, the smallest admissible sample, and exactly two, confirming the
   `#DIV/0!` boundary.
4. **A constant sample**, confirming the zero-standard-deviation `#DIV/0!`.
5. **A symmetric sample** such as `{-2,-1,0,1,2}`, checking whether the answer is exactly `0` or
   merely near it — which reveals the summation order.
6. **The bound**: a maximally skewed five-point sample, checking the result against `sqrt(5)`.
7. **The direct-versus-scan split**: `SKEW(1,2,"3",4)` against the same four values in cells,
   confirming the dual coercion policy on this surface.
8. **Data with a vanishing spread**, where the exact `s = 0` test does not fire but the
   standardisation is meaningless.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| `G1` | The adjusted Fisher-Pearson coefficient; what `SKEW` returns |
| `g1` | The uncorrected third standardised moment; what [SKEW.P](FUNC.SKEW.P.md) returns |
| standardised moment | A central moment divided by the appropriate power of the standard deviation |
| two-pass centring | Compute the mean first, then accumulate deviations; the stable route |
| raw power sums | Accumulating `sum x`, `sum x^2`, `sum x^3` and assembling centrally; the unstable route |
| shift invariance | The true skewness is unchanged by adding a constant; a test of the algorithm |

## Sources

- Microsoft, *SKEW function* —
  <https://support.microsoft.com/en-us/office/skew-function-bdf49d86-b1ef-4804-a046-28eaea69c9fa>
  (signature, the stated equation, the ignore-text-and-logicals rule for scanned ranges, and the
  `#DIV/0!` conditions for fewer than three points and for zero standard deviation). Retrieval
  was blocked by the upstream host for this page; the documented behaviour above is stated as
  documented behaviour and should be re-checked against the page.
- M. Abramowitz and I. A. Stegun, *Handbook of Mathematical Functions*, chapter 26 section 26.1 —
  the moment and standardised-moment vocabulary used throughout this page.
- D. N. Joanes and C. A. Gill, "Comparing measures of sample skewness and kurtosis", *Journal of
  the Royal Statistical Society, Series D (The Statistician)* 47(1), 1998, pp. 183–189 — the
  three competing sample estimators, their biases, and the exact relations between them.
- T. F. Chan, G. H. Golub and R. J. LeVeque, "Algorithms for Computing the Sample Variance:
  Analysis and Recommendations", *The American Statistician* 37(3), 1983, pp. 242–247 — the
  two-pass versus raw-sums analysis, applied here to a higher moment.
- OxFunc `crates/oxfunc_core/src/functions/moment_stats_family.rs` at commit `473efa3` — the
  reference-engine kernel: the `n >= 3` and zero-variance guards, the two-pass centring, and the
  per-element standardisation.
- OxFunc defect stream `docs/bugs/streams/BUG-FUNC-021_w090_statistical_numeric_exactness_drift.md`
  — the upstream register naming this module.
- Handbook [coercion and lifting](../model/02-coercion-and-lifting.md) — the direct-argument
  versus range-scan dual policy this aggregate carries.
- Handbook `data/functions/FUNC.SKEW.json`, `data/presence/FUNC.SKEW.json`.
