---
schema: efh.function-page/v1
function_id: FUNC.STDEVA
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — STDEVA function"
    locator: "https://support.microsoft.com/en-us/office/stdeva-function-5ff38888-7ea5-48de-9a6d-11ed73b29e9d"
    role: "documented signature, the n-1 divisor, the text-as-zero and TRUE-as-one inclusion rules, and the #DIV/0! condition"
  - work: "Chan, T. F., Golub, G. H. and LeVeque, R. J., \"Algorithms for Computing the Sample Variance: Analysis and Recommendations\""
    locator: "The American Statistician 37(3), 1983, pp. 242-247"
    role: "the error analysis of the textbook one-pass formula against the two-pass and updating forms"
  - work: "OxFunc — aggregate_common.rs and variance_common.rs"
    locator: "crates/oxfunc_core/src/functions/aggregate_common.rs, crates/oxfunc_core/src/functions/variance_common.rs"
    role: "reference-engine kernel: the AVERAGEA-like inclusion policy and the shared two-pass sample variance"
episodes: []
body_sections:
  - "What it computes"
  - "Arguments"
  - "The inclusion rule"
  - "Result and edge cases"
  - "Errors"
  - "Relationships"
  - "Numerical notes"
  - "What has not been checked"
  - "Page vocabulary"
  - "Sources"
family: stdeva_fn
role_in_family: >-
  STDEV.S with the A-variant inclusion rule: the same n-1 sample estimator over a data set that
  additionally admits text as zero and logicals as one and zero.
---

# STDEVA

## What it computes

`STDEVA(value1, [value2], …)` returns the **sample standard deviation** — the same estimator as
[STDEV.S](FUNC.STDEV.S.md), with Bessel's `n - 1` divisor — computed over a **larger data set**,
because `STDEVA` converts values that `STDEV.S` skips.

For the admitted values `x_1, …, x_n` with mean `xbar`,

    s^2  =  ( 1 / (n - 1) )  *  sum_{i=1..n} (x_i - xbar)^2
    s    =  sqrt( s^2 )

The estimator is identical. **Everything that distinguishes `STDEVA` happens before the
arithmetic starts**, in deciding which cells become values and what those values are. That makes
this page unusual: the mathematics is entirely [STDEV.S](FUNC.STDEV.S.md)'s, and the content is
the admission rule.

Bessel's correction is what it always is: `E[s^2] = sigma^2` for an independent sample, because
one degree of freedom was spent estimating the centre. And as on [STDEV.S](FUNC.STDEV.S.md), `s`
itself remains a biased estimator of `sigma` by Jensen's inequality — the square root of an
unbiased estimator is not an unbiased estimator. Neither of those facts is disturbed by the
inclusion rule; what the inclusion rule disturbs is whether the resulting number means anything.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `value1` | The first value, range or array. | yes |
| `value2`, … | Further values, ranges or arrays. | optional, up to the declared maximum |

Note the parameter name: `value`, not `number`. Across Excel the `A` variants — `AVERAGEA`,
`COUNTA`, `MAXA`, `MINA`, `VARA`, `STDEVA` — signal by that name that they take *values* rather
than numbers, and convert rather than skip.

The arguments form a flat bag: everything is flattened and concatenated, and argument boundaries
have no effect.

## The inclusion rule

This is the section that earns the page. `STDEVA` carries the `AVERAGEA`-style inclusion policy,
and it interacts with the direct-versus-scan split described in
[coercion and lifting](../model/02-coercion-and-lifting.md). The reference engine's behaviour, in
full:

| Value | Reached by scanning a range | Passed directly as a scalar |
|---|---|---|
| Number | used as itself | used as itself |
| `TRUE` | counted as `1` | counted as `1` |
| `FALSE` | counted as `0` | counted as `0` |
| Text (any, including `""`) | counted as **`0`** | coerced by the ordinary rules — `"3"` becomes `3`, unparseable text is `#VALUE!` |
| Blank cell | **skipped** — not counted at all | — |
| Omitted argument | — | skipped |
| Error value | propagates | propagates |

Three things follow, and each one is a trap that reaches real workbooks:

1. **Text counts as zero, and zero is a data point.** A column of measurements with `"n/a"` typed
   into three cells has three extra observations at `0`. Those zeros pull the mean toward zero
   *and* sit far from the rest of the data, so they inflate the deviation twice over. `STDEVA`
   over such a column can be several times [STDEV.S](FUNC.STDEV.S.md) over the same column, and
   the difference is not subtle.
2. **Blank is not text and is not zero — it is skipped.** The distinction between an empty cell
   and a cell containing an empty string is invisible on screen and decisive here: `""` counts as
   an observation at `0`, and a genuinely empty cell does not count at all. A formula that
   produces `""` as its "nothing here" output is therefore *not* neutral to `STDEVA`, while a
   truly blank cell is. This is the single most surprising row in the table.
3. **Text behaves differently as a direct argument.** `STDEVA(1, 2, "3")` admits `3`, because
   direct scalar text goes through ordinary to-number coercion, while `"3"` in a scanned cell
   would be counted as `0`. The same three characters mean two different numbers depending on how
   they reached the function. That is engine-wide dual policy, not a quirk of `STDEVA`, but
   `STDEVA` is where it does the most damage because the scanned reading is `0` rather than
   "skip".

**When to use `STDEVA` at all.** Almost never for measurement data. It exists for data sets where
non-numeric entries genuinely denote zero — a column of TRUE/FALSE indicators, or a legacy sheet
in which "no" was typed instead of `0`. If the text means "missing", `STDEVA` is the wrong
function and [STDEV.S](FUNC.STDEV.S.md) is the right one. If the text means "zero", `STDEVA` is
correct and saves a cleanup pass. The choice is a statement about what the sheet means, and
nothing in Excel will check it.

## Result and edge cases

Returns `Number`, non-negative, in the units of the data.

- **Fewer than two admitted values.** `#DIV/0!` — the `n - 1` divisor. Note that `n` here counts
  the *admitted* values, so a range with one number and one text cell has `n = 2` and computes,
  where the same range would give `#DIV/0!` under [STDEV.S](FUNC.STDEV.S.md). The A-variant can
  succeed exactly where the plain variant fails, on identical input.
- **All admitted values equal.** Exactly `0`. A range of all-`FALSE` cells has every value `0` and
  returns `0`; a range mixing `TRUE` and `FALSE` returns the standard deviation of a zero-one
  vector.
- **A range of only text.** Every cell counts as `0`, so the answer is `0` rather than `#DIV/0!`
  once there are two or more such cells — a striking difference from
  [STDEV.S](FUNC.STDEV.S.md), which returns `#DIV/0!` on the same range.
- **Errors anywhere** propagate as themselves.
- Overflow and underflow of the squared deviations behave as on [STDEV.S](FUNC.STDEV.S.md).

## Errors

| Error | Condition | Source |
|---|---|---|
| `#DIV/0!` | fewer than two admitted values | documented and implemented |
| `#VALUE!` | direct scalar text that does not parse as a number | shared coercion rule; note that the same text in a range would count as `0` instead |
| propagated | An error value in any argument surfaces as that error | shared coercion rule |

The `#VALUE!` row is worth reading twice. `STDEVA("abc", 1, 2)` and `STDEVA(A1:A3)` with `"abc"`
in `A1` are not the same call: the first is an error, the second is a computation over `{0,1,2}`.
Microsoft's documented sentence that text "evaluates as 0" is a statement about the range-scan
lane; the direct lane is governed by ordinary coercion. Neither this page nor the documentation
the Handbook can cite states that split explicitly, so it is recorded here as reference-engine
behaviour that has not been compared against Excel.

## Relationships

- **[STDEV.S](FUNC.STDEV.S.md)** is the same estimator with the skip-non-numeric range policy.
  The pair is the cleanest illustration in Excel of the difference between "ignore" and "convert",
  and comparing the two over the same range is the standard diagnostic when a standard deviation
  looks wrong.
- **[STDEVPA](FUNC.STDEVPA.md)** is the population form of this function — same inclusion rule,
  `n` divisor. The exact relation `STDEVPA = STDEVA * sqrt((n-1)/n)` holds with `n` counting the
  admitted values.
- **[VARA](FUNC.VARA.md)** is the square: `STDEVA = SQRT(VARA)`, and that is how the reference
  engine computes it.
- **[AVERAGEA](FUNC.AVERAGEA.md)** supplies the same `xbar` under the same inclusion rule, and
  **[COUNTA](FUNC.COUNTA.md)** is *not* the same count — `COUNTA` counts every non-empty cell
  including error values, while `STDEVA`'s `n` counts admitted values and errors propagate rather
  than counting. Using `COUNTA` to predict `STDEVA`'s `n` is wrong whenever the range holds an
  error.
- **`STDEVA` has no Compatibility-category predecessor and no dotted successor.** The `A` variants
  were never reorganised by the `.P`/`.S` rename, so `STDEVA` is both the historical and the
  current name. There is no `STDEVA.S`, and migration guidance about preferring dotted names does
  not apply.
- Readers confuse `STDEVA` with `STDEV.S` on the assumption that the `A` means "array" or
  "all cells". It means neither: it means *values* rather than *numbers*.

## Numerical notes

The arithmetic is identical to [STDEV.S](FUNC.STDEV.S.md)'s, and everything on that page's
numerical notes applies without change: two-pass centred accumulation rather than the unstable
raw sums-of-squares identity `sum x^2 - (sum x)^2/n`, naive summation for both the mean and the
sum of squares, a correctly rounded final `sqrt` that halves the variance's relative error, exact
overflow and underflow thresholds inherited from squaring the deviations, and summation order as
part of the answer. Chan, Golub and LeVeque (1983) is the reference for the algorithm choice, and
the published record on Excel's variance accuracy — McCullough and Wilson, Knüsel, and Welinder's
Gnumeric work — belongs to the same discussion. **This page does not assert what Excel does
internally.**

What is specific to `STDEVA` is that **the inclusion rule changes the conditioning of the
problem**, and usually for the worse.

Substituted zeros are not neutral data. Suppose a column holds measurements clustered around some
large value `M` with a small spread, plus a handful of text cells that `STDEVA` reads as `0`. The
admitted data now has two clusters, at `M` and at `0`, and:

- The mean moves to a weighted point between them, so every genuine observation acquires a
  deviation of order `M` rather than of order the true spread.
- The sum of squares becomes dominated by terms of order `M^2`, and the information about the
  real spread is a small correction to a large number — which is precisely the cancellation regime
  the two-pass form was chosen to avoid. The two-pass form still avoids the *catastrophic* form of
  it, but the answer is now a well-computed standard deviation of a bimodal data set, not a
  poorly-computed one of the original data.
- Overflow becomes reachable earlier: deviations of order `M` square to order `M^2`, so data that
  would never have overflowed under [STDEV.S](FUNC.STDEV.S.md) can overflow here.

The honest statement is that `STDEVA`'s accuracy risk is not arithmetic but semantic. The
floating-point behaviour is [STDEV.S](FUNC.STDEV.S.md)'s; the number it computes accurately may
not be the number anyone wanted. That is a different failure from the one the variance literature
is about, and it is the more common one in practice.

One arithmetic remark that *is* specific: because logicals map to exactly `0` and `1` and text
maps to exactly `0`, a data set consisting only of A-variant substitutions is a vector of exact
small integers. Its mean is a ratio of small integers and its sum of squares is exact, so
`STDEVA` over a pure indicator column is computed with essentially no rounding at all until the
final division and square root. Indicator data is the case where this function is both
appropriate and numerically ideal.

## What has not been checked

No Handbook vector suite exists for `STDEVA`, and no Handbook evidence record names `STDEVA` as a
subject. Nobody has checked this function against Excel within the Handbook's record. The
implementing module carries no tests of its own in the reference engine; the shared variance
kernel it calls carries a small number, which speaks to that kernel rather than to this surface.

Everything above marked as documented comes from Microsoft's `STDEVA` page. **Retrieval of that
page was blocked by the upstream host on this pass**, so those statements are recorded as
documented behaviour with the source named and should be re-read against the live page. The
direct-versus-scan split for text is stated from the reference engine, not from documentation.

Inputs worth probing first:

1. **Empty cell against empty string.** A three-cell range holding `1`, `2` and a genuinely blank
   cell, against the same range with `=""` in the third cell. If the two answers differ, the
   skip-versus-zero distinction is confirmed and is the most consequential undocumented behaviour
   on this page.
2. **`STDEVA(A1:A3)` with text in `A1`** against `STDEVA("abc", 1, 2)` — the direct-versus-scan
   split, where the reference engine gives a number and an error respectively.
3. **A range of only text**, at two cells and at one, against `0` and `#DIV/0!` respectively —
   and the same range through [STDEV.S](FUNC.STDEV.S.md), which should give `#DIV/0!` in both
   cases. This pair of probes separates the two functions completely.
4. **`TRUE` and `FALSE` in a range and as direct arguments**, confirming they are admitted as `1`
   and `0` on both routes — the one place where the A variants have no dual policy.
5. **`STDEVA` against `SQRT(VARA)`**, bitwise.
6. **`STDEVA` against `STDEVPA * SQRT(n/(n-1))`**, where `n` counts admitted values, measuring
   the gap the exact identity leaves after rounding.
7. **The shift test** from [STDEV.S](FUNC.STDEV.S.md), applied here on data containing
   substituted zeros — where the substitution has already spoiled the conditioning and the test
   becomes a test of the whole path rather than of the summation alone.
8. **An error value in a range alongside text**, confirming propagation wins over substitution.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| A variant | The `value`-taking sibling of a `number`-taking aggregate; converts where the other skips |
| inclusion rule | Which values become data points, and what number each becomes |
| substituted zero | A `0` contributed by a text cell rather than by a number |
| admitted values | The `n` values that reach the variance kernel |
| Bessel's correction | The `n - 1` divisor, shared with [STDEV.S](FUNC.STDEV.S.md) |
| dual policy | The direct-argument versus range-scan split of [coercion and lifting](../model/02-coercion-and-lifting.md) |

## Sources

- Microsoft, *STDEVA function* —
  <https://support.microsoft.com/en-us/office/stdeva-function-5ff38888-7ea5-48de-9a6d-11ed73b29e9d>
  (signature, the `n-1` divisor, the rules that `TRUE` evaluates as `1` and that text and `FALSE`
  evaluate as `0`, that empty cells are not counted, and the `#DIV/0!` condition). Retrieval was
  blocked by the upstream host for this page; the documented behaviour above is stated as
  documented behaviour and should be re-checked against the page.
- T. F. Chan, G. H. Golub and R. J. LeVeque, "Algorithms for Computing the Sample Variance:
  Analysis and Recommendations", *The American Statistician* 37(3), 1983, pp. 242–247.
- B. D. McCullough and B. Wilson, "On the accuracy of statistical procedures in Microsoft Excel",
  *Computational Statistics & Data Analysis*, 1999, and the follow-up assessments through 2005;
  L. Knüsel's parallel work; M. Welinder's writing on Excel's statistical functions arising from
  the Gnumeric project.
- OxFunc `crates/oxfunc_core/src/functions/stdeva_fn.rs`,
  `crates/oxfunc_core/src/functions/variance_common.rs` and
  `crates/oxfunc_core/src/functions/aggregate_common.rs` at commit `473efa3` — the
  `AVERAGEA`-like inclusion policy (text to `0`, logicals to `1`/`0`, blanks skipped, direct
  scalars coerced), the sample divisor, and the shared two-pass variance.
- Handbook [coercion and lifting](../model/02-coercion-and-lifting.md) — the direct-argument
  versus range-scan dual policy, and the Empty/Missing distinction behind the blank-cell row.
- Handbook [the value universe](../model/01-value-universe.md) — the difference between an empty
  cell and a cell containing an empty string.
- Handbook `data/functions/FUNC.STDEVA.json`, `data/presence/FUNC.STDEVA.json`.
