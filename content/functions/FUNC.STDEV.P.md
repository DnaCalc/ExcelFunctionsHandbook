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
  - "Notes"
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
- **Two values.** `sigma = |x_1 - x_2| / 2`, exactly representable when the difference is. Excel
  reaches it through the raw moments or the centered sum of FN-VAR.S-001, and their steps round.
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
- **`VAR.P`** is the square: `STDEV.P = SQRT(VAR.P)`, and that is literally how
  the reference engine computes it.
- **[STDEVPA](FUNC.STDEVPA.md)** uses the same `n` divisor with a different inclusion rule — text
  in a range counts as `0`, logicals as `1`/`0`. Same estimator, different data set.
- **`STDEVP`** is the legacy Compatibility-category name. Microsoft's replacement
  guidance points `STDEVP` at `STDEV.P`, and the two are documented as computing the same
  quantity. **The Handbook does not treat that as an identity.** A legacy alias and its modern
  name are two registered entry points, and Excel is free to route them to different code; that
  they agree is a claim requiring evidence, and the Handbook's record holds only indirect evidence:
  all recorded rows of both names match the rule of FN-VAR.S-001. The variance
  algorithms were changed at Excel 2003 with both names still in the product, which is exactly
  the situation in which an unverified alias assumption is worth least.
- **[AVERAGE](FUNC.AVERAGE.md)** supplies `xbar` and shares the scan policy exactly.
- **`DEVSQ`** returns the numerator alone, and is the right diagnostic when a
  discrepancy might be a divisor question rather than an algorithm question.
- **[SKEW.P](FUNC.SKEW.P.md)** uses this same `sigma` as its standardising scale, which is why
  the population skewness and the population standard deviation fail on the same data.
- **[Z.TEST](FUNC.Z.TEST.md)** and the normal-theory functions consume a population `sigma` when
  one is genuinely known — the case `STDEV.P` is actually for.

## Numerical notes

Everything on [STDEV.S](FUNC.STDEV.S.md)'s numerical notes applies here, because in the reference
engine the two functions share one kernel and differ only in the final divisor; Excel's raw-moment
forms differ by more (FN-VAR.S-001). The summary, with the points that are
specific to the population form.

**Two-pass centred, not raw power sums.** The reference engine computes `xbar` first and then
accumulates `sum (x_i - xbar)^2` in a second pass. The textbook one-pass identity

    sum (x - xbar)^2  =  sum x^2  -  (sum x)^2 / n

is algebraically equal and catastrophically unstable: when the mean is large relative to the
spread it subtracts two nearly-equal large numbers, losing roughly `2*log10(xbar/sigma)`
significant digits, and can return a **negative** sum of squares — the signature by which this
bug is identified. Chan, Golub and LeVeque (1983) give the analysis; Welford (1962) gives the
stable single-pass alternative for cases where the data cannot be stored. Excel uses raw moments
with stored steps, and two stored tests switch it to a centered sum (FN-VAR.S-001).

**The published record on Excel.** McCullough and Wilson's assessments in *Computational
Statistics & Data Analysis* from 1999 onward, Knüsel's parallel work, and Welinder's writing on
Excel's statistical functions arising from the Gnumeric project are the standing published record
on Excel's variance accuracy, and the algorithms were changed at Excel 2003 in response. **What
Excel's `STDEV.P` does is recorded in FN-VAR.S-001**: raw moments with stored steps, a centered sum
when either of two stored tests holds, and a root rounded to 64 bits and then stored. So Excel uses
both families of algorithm and switches between them. The shift test below remains a measurement
anyone can make.

**Specific to the population divisor.** `sumsq / n` is a division by an exactly representable
integer, and for `n` a power of two it is exact. `STDEV.S`'s `sumsq / (n-1)` has no such property
in general. On Excel's raw-moment path the division is by `n*n` instead (FN-VAR.S-001). This makes
`STDEV.P` the marginally cleaner of the two, and it also means the exact
identity `STDEV.P = STDEV.S * sqrt((n-1)/n)` will not hold bit for bit even when both functions
are computed correctly: the two divisions round differently and the correcting square root adds
its own rounding. A suite asserting that identity must assert it to a tolerance, and the size of
that tolerance is itself worth measuring.

**Residual hazards.** The mean and the sum of squares are both accumulated by naive
left-to-right summation, so their error grows with `n` where compensated or pairwise summation
would bound it; the reference engine uses neither. The sum of squares has non-negative terms and
is therefore well-conditioned. The final `sqrt` *halves* the relative error of the variance, and
in Excel is rounded to 64 bits and then stored (FN-VAR.S-001), so the whole accuracy question lives
in the variance. Squared deviations
overflow above roughly `1.3 * 10^154` and underflow to zero below roughly `10^{-162}`, so
`STDEV.P` can report an infinity or an exact zero for finite, nonzero-spread data; a scale-aware
formulation that factors out the largest deviation first would avoid both, and is not used.

**Reproducibility.** Summation order is part of the answer, so two correct implementations that
traverse or vectorise differently will disagree in the last bits.

## What has not been checked

No Handbook vector suite exists for `STDEV.P`, and no Handbook evidence record names `STDEV.P` as
a subject. FN-VAR.S-001 checks this function against Excel: with the rest of the variance family,
all 14,679
recorded rows match. Still open there: whether its two tests keep the same edges for ranges and
arrays. The implementing module carries no tests of its own in the reference engine; the shared
variance
kernel it calls carries a small number, which speaks to that kernel rather than to this surface.

Everything above marked as documented comes from Microsoft's `STDEV.P` page. **Retrieval of that
page was blocked by the upstream host on this pass**, so those statements are recorded as
documented behaviour with the source named and should be re-read against the live page.

Inputs worth probing first:

1. **The shift test.** A small data set with a known exact answer, evaluated as given and then
   with `10^6`, `10^9` and `10^{12}` added to every value. The true answer is invariant.
   FN-VAR.S-001 has since identified the algorithm, so this is now a four-cell check of it.
2. **A single numeric cell.** `STDEV.P` should return `0` where [STDEV.S](FUNC.STDEV.S.md)
   returns `#DIV/0!`. This is the cheapest probe that the two functions are genuinely different
   surfaces rather than one with a parameter.
3. **`STDEV.P` against `SQRT(VAR.P)`**, bitwise. Answered: `STDEV.P` is the root of the variance,
   rounded to 64 bits and then stored (FN-VAR.S-001).
4. **`STDEV.P` against `STDEV.S * SQRT((n-1)/n)`**, measuring the gap the exact identity leaves
   after three roundings. The size of that gap is the tolerance a suite would have to allow.
5. **`STDEV.P` against the legacy `STDEVP`** on the same data, bitwise — the probe that would
   turn the documented replacement relationship into an evidenced one.
6. **An empty range and a range of only text**, confirming `#DIV/0!` and not `#VALUE!`.
7. **The scan/direct split**: `STDEV.P(1,2,"3",TRUE)` against the same four values in cells,
   which should give different answers and different `n`.
8. **Overflow and underflow**: deviations near `10^{154}` and near `10^{-162}`.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `STDEV.P` that an implementer matching Excel, or a reader explaining an
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
and then stored. So the answer depends on which path a call takes: the two VAR.S calls below
differ by one input step in two arguments, the first passes the 0.99 test and is correctly
rounded, and the second takes the raw moments and lands 76 units high. The second pair shows
the size test: scaling the inputs by 4 leaves the first path.

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
