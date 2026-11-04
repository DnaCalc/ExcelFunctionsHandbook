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
  - "Notes"
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
- **Two values.** `s = |x_1 - x_2| / sqrt(2)`. Excel reaches it through the raw moments or the
  centered sum of FN-VAR.S-001, and their steps round.
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
- **`VAR.S`** is the square of this function: `STDEV.S = SQRT(VAR.S)`, and that
  is literally how the reference engine computes it. Anything true of `VAR.S`'s accuracy is true
  of `STDEV.S`'s, halved in relative terms by the square root.
- **[STDEVA](FUNC.STDEVA.md)** uses the same `n - 1` divisor with a different inclusion rule:
  text in a range counts as `0` and logicals count as `1`/`0`. It is the same estimator over a
  different data set.
- **`STDEV`** is the legacy Compatibility-category name. Microsoft's replacement
  guidance points `STDEV` at `STDEV.S`, and the two are documented as computing the same
  quantity. **The Handbook does not treat that as an identity.** A legacy alias and its modern
  name are two registered entry points, and Excel is free to route them to two different code
  paths; that they agree — let alone that they agree in the last bits — is a claim requiring
  evidence, and the Handbook's record holds only indirect evidence: all recorded rows of both names
  match the rule of FN-VAR.S-001. This pair is a particularly live
  question because the variance algorithms were changed at Excel 2003 while both names remained
  in the product.
- **[AVERAGE](FUNC.AVERAGE.md)** supplies `xbar` and shares the scan policy exactly, so
  `COUNT` and `AVERAGE` over the same range see the same `n` and the same values `STDEV.S` does.
- **`DEVSQ`** returns the numerator `sum (x - xbar)^2` on its own, and is the
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
variance and regression algorithms were changed at Excel 2003 in response. **What Excel's
`STDEV.S` does is recorded in FN-VAR.S-001**: raw moments with stored steps, a centered sum when
either of two stored tests holds, and a root rounded to 64 bits and then stored. So Excel uses both
families of algorithm and switches between them. The shift test described below remains a
measurement anyone can make.

**Residual hazards in the good form.** The two-pass form is far better, not perfect:

- The mean is accumulated by naive left-to-right summation, so `xbar` carries an error growing
  with `n`; compensated (Kahan) or pairwise summation would bound it by a constant. The reference
  engine uses neither. Since every deviation is taken against this `xbar`, an error in it biases
  the whole sum of squares — though only in second order, because the sum of squares is
  stationary at the true mean.
- The sum of squares is also accumulated naively, but its terms are all non-negative, so it is
  well-conditioned and the error is benign.
- `sqrt` *halves* the relative error of the variance. In Excel the root is rounded to 64 bits and
  then stored (FN-VAR.S-001), so it is not always correctly rounded. The whole accuracy question
  therefore lives in the variance,
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
a subject. FN-VAR.S-001 checks this function against Excel: with the rest of the variance family,
all 14,679
recorded rows match. Still open there: whether its two tests keep the same edges for ranges and
arrays. The implementing module carries no tests of its own in the reference engine; the shared
variance
kernel it calls carries a small number, which speaks to that kernel rather than to this surface.

Everything above marked as documented comes from Microsoft's `STDEV.S` page. **Retrieval of that
page was blocked by the upstream host on this pass**, so those statements are recorded as
documented behaviour with the source named and should be re-read against the live page.

Inputs worth probing first:

1. **The shift test.** Take a small data set with a known exact standard deviation and evaluate
   `STDEV.S` on it, then on the same values with `10^6`, `10^9` and `10^{12}` added to every one.
   The true answer is identical in all four; the returned values are not, and *how* they degrade
   shows the form in use. FN-VAR.S-001 has since identified the algorithm, so this is now a
   four-cell check of it.
2. **The negative-sum-of-squares signature.** The classic constructed data sets from the
   published accuracy literature — values differing only in their low-order digits at a large
   offset — where the one-pass form produces a negative sum of squares.
3. **`STDEV.S` against `SQRT(VAR.S)`** on the same data, bitwise. The reference engine computes
   exactly that. Answered for Excel: `STDEV.S` is the root of the variance, rounded to 64 bits and
   then stored (FN-VAR.S-001).
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

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `STDEV.S` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-VAR.S-001: The variance family uses raw moments with stored steps, and two tests switch it to a centered sum

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.* Applies to STDEV, STDEV.P, STDEV.S, STDEVA, STDEVP, STDEVPA, VAR, VAR.P, VAR.S, VARA, VARP, VARPA.

With RN(v) meaning v rounded to 64 bits on the x87 unit and then stored to binary64, S the
running RN sum of the n values and Q the running RN sum of RN(x*x), the sample variance is
RN(RN(Q - RN(RN(S*S)/n))/(n-1)) and the population variance RN(RN(RN(n*Q) - RN(S*S))/RN(n*n)).
Excel then uses a centered sum instead, adding RN(d*d) with d = RN(x - mean) and dividing by
n-1 or n, when either of two stored tests holds: RN(RN(RN(S*S)/n)/Q) > 0.99, or
RN(variance*1000) <= |RN(S/n)|. The STDEV functions are the root of this, rounded to 64 bits
and then stored. So the answer depends on which path a call takes: the first two VAR.S calls
below differ by a few input steps in two arguments, the first passes the 0.99 test and is
correctly rounded, and the second takes the raw moments and lands 76 units high. The next pair
shows the size test: scaling the inputs by 4 leaves the first path.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `VAR.S(0.8769085090206681, 1, 1.123091490979332)` | number1 `0.8769085090206681` (`0x3fec0fa26efa7e14`); number2 `1` (`0x3ff0000000000000`); number3 `1.123091490979332` (`0x3ff1f82ec882c0f6`) (cells) | `0.01515151515151495` (`0x3f8f07c1f07c1e94`) | — | Excel 16.0 build 20527, Windows; bitness and CPU not recorded in this reading |
| `VAR.S(0.8769085090206679, 1, 1.1230914909793321)` | number1 `0.8769085090206679` (`0x3fec0fa26efa7e12`); number2 `1` (`0x3ff0000000000000`); number3 `1.1230914909793321` (`0x3ff1f82ec882c0f7`) (cells) | `0.015151515151515138` (`0x3f8f07c1f07c1f00`) | `0.015151515151515006` (`0x3f8f07c1f07c1eb4`) | Excel 16.0 build 20527, Windows; bitness and CPU not recorded in this reading |
| `VAR.S(3.244042371786199e-10, 0.00034050419142641947)` | number1 `3.244042371786199e-10` (`0x3df64afacd3a935a`); number2 `0.00034050419142641947` (`0x3f3650b65dc7a1d6`) (cells) | `5.797144172853e-08` (`0x3e6f1f88c6bbc51a`) | — | Excel 16.0 build 20527, Windows; bitness and CPU not recorded in this reading |
| `VAR.S(1.2976169487144796e-09, 0.0013620167657056779)` | number1 `1.2976169487144796e-09` (`0x3e164afacd3a935a`); number2 `0.0013620167657056779` (`0x3f5650b65dc7a1d6`) (cells) | `9.275430676564799e-07` (`0x3eaf1f88c6bbc519`) | `9.2754306765648e-07` (`0x3eaf1f88c6bbc51a`) | Excel 16.0 build 20527, Windows; bitness and CPU not recorded in this reading |
| `=VAR.S({157,376.82,189.207,12.38,12.473,483,306.24,324.1852,482.235,78})` | as typed | `31907.480453381795` (`0x40df28debfbf8a84`) | `31907.480453381777` (`0x40df28debfbf8a7f`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `STDEV.S(0, 1, 1.031052478719249)` | number1 `0` (`0x0000000000000000`); number2 `1` (`0x3ff0000000000000`); number3 `1.031052478719249` (`0x3ff07f30e248f14c`) (cells) | `0.5865198874587454` (`0x3fe2c4c55ae2d85e`) | — | Excel 16.0 build 20527, Windows; bitness and CPU not recorded in this reading |

Correct value: Exact rational variance of the binary64 inputs, correctly rounded.

Basis (measured): All 14,679 recorded rows of the twelve functions match: VAR.S, VAR.P, VAR,
VARP, VARA, VARPA, STDEV.S, STDEV.P, STDEV, STDEVP, STDEVA and STDEVPA. Six of those rows
separate the stored steps from plain binary64. 910 fresh scalar variance readings (Excel build
20527) replay exactly in rational arithmetic. Four cancellation edges were bisected to adjacent
inputs; sixteen triples chosen to separate the stored ratio from 0.99*Q and from a division by
0.99 all agree with the ratio, where 0.99*Q misses all 16 and the division 14. Seven of eight
magnitude brackets reach adjacent inputs, and sixteen chosen roots all need the 64-bit root
followed by a store.

Inferred, not measured: These tests were located with scalar arguments in separate cells; that
the same edges hold for ranges and arrays is inferred, and their readings have not been taken.
RN(variance*1000) <= |mean| is not separated from the rearrangement variance <= |mean|/1000.
That a raw moment which overflows also takes the centered sum is inferred from recorded answers
that stay finite.

- Open: Whether the two tests keep the same edges for ranges and arrays.

Evidence:

- DnaCalc `crates/calc-func/src/ported/variance_common.rs`, variance_from_values at `9c23b24e8d` — the rules in the product; the commit message carries the row counts
- DnaCalc `crates/calc-func/tests/f1_stats.rs`, variance_cancellation_guard_stores_the_ratio at `9c23b24e8d` — recorded answers pinned
- DnaCalc `crates/calc-func/witness-status.tsv` at `af74247e1e` — rows matched per function
- DnaCalc-Research `evidence/round25/variance-published-canonical-scope-receipt.json` at `a547d79ed6` — the 14,679 rows before and after
- DnaCalc `crates/calc-func/tests/f1_stats.rs`, variance_cancellation_guard_keeps_the_observed_edge at `9c23b24e8d` (readings)
- DnaCalc `crates/calc-func/tests/f1_stats.rs`, variance_centered_guard_uses_the_absolute_mean at `9c23b24e8d` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcj9k_statistics.capture.json`, sheet dcj9k_statistics, row 3, column 4 at `c9f9d35416` (readings)
- DnaCalc `crates/calc-func/tests/f1_stats.rs`, stdev_stores_the_extended_square_root at `9c23b24e8d` (readings)

<!-- efh:notes end -->

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
