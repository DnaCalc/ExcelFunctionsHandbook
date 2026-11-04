---
schema: efh.function-page/v1
function_id: FUNC.STDEVPA
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — STDEVPA function"
    locator: "https://support.microsoft.com/en-us/office/stdevpa-function-5578d4d6-455a-4308-9991-d405afe2c28c"
    role: "documented signature, the n divisor, the text-as-zero and TRUE-as-one inclusion rules, and the #DIV/0! condition"
  - work: "Chan, T. F., Golub, G. H. and LeVeque, R. J., \"Algorithms for Computing the Sample Variance: Analysis and Recommendations\""
    locator: "The American Statistician 37(3), 1983, pp. 242-247"
    role: "the error analysis of the textbook one-pass formula against the two-pass and updating forms"
  - work: "OxFunc — aggregate_common.rs and variance_common.rs"
    locator: "crates/oxfunc_core/src/functions/aggregate_common.rs, crates/oxfunc_core/src/functions/variance_common.rs"
    role: "reference-engine kernel: the AVERAGEA-like inclusion policy and the shared two-pass population variance"
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
  - "Notes"
  - "Page vocabulary"
  - "Sources"
family: stdevpa_fn
role_in_family: >-
  STDEV.P with the A-variant inclusion rule: the same n-divisor population estimator over a data
  set that additionally admits text as zero and logicals as one and zero.
---

# STDEVPA

## What it computes

`STDEVPA(value1, [value2], …)` returns the **population standard deviation** — the same estimator
as [STDEV.P](FUNC.STDEV.P.md), with the plain `n` divisor and no Bessel correction — computed over
a **larger data set**, because `STDEVPA` converts values that `STDEV.P` skips.

For the admitted values `x_1, …, x_n` with mean `xbar`,

    sigma^2  =  ( 1 / n )  *  sum_{i=1..n} (x_i - xbar)^2
    sigma    =  sqrt( sigma^2 )

The estimator is identical to [STDEV.P](FUNC.STDEV.P.md)'s. **Everything that distinguishes
`STDEVPA` happens before the arithmetic starts**, in deciding which cells become values and what
those values are. This page is therefore mostly about the admission rule; the mathematics is on
[STDEV.P](FUNC.STDEV.P.md)'s page and is unchanged here.

`STDEVPA` sits at the intersection of two independent choices, and the four-way grid is the
clearest way to hold the family in mind:

| | skip non-numeric (`number` args) | convert non-numeric (`value` args) |
|---|---|---|
| **sample, `n-1`** | [STDEV.S](FUNC.STDEV.S.md) | [STDEVA](FUNC.STDEVA.md) |
| **population, `n`** | [STDEV.P](FUNC.STDEV.P.md) | **`STDEVPA`** |

The two axes are orthogonal, and the two questions they answer are different in kind. The divisor
is a **statistical** decision — is this the whole population, or a sample of something larger. The
inclusion rule is a **data-cleaning** decision — do non-numeric cells mean zero, or mean missing.
Excel makes you answer both by choosing a name, and the names give no hint that two independent
decisions are being made.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `value1` | The first value, range or array. | yes |
| `value2`, … | Further values, ranges or arrays. | optional, up to the declared maximum |

The parameter name is `value`, not `number` — the signal that this is an `A` variant and converts
rather than skips. The arguments form a flat bag: everything is flattened and concatenated, and
argument boundaries have no effect.

## The inclusion rule

`STDEVPA` carries the `AVERAGEA`-style inclusion policy, interacting with the direct-versus-scan
split of [coercion and lifting](../model/02-coercion-and-lifting.md). The reference engine's
behaviour in full:

| Value | Reached by scanning a range | Passed directly as a scalar |
|---|---|---|
| Number | used as itself | used as itself |
| `TRUE` | counted as `1` | counted as `1` |
| `FALSE` | counted as `0` | counted as `0` |
| Text (any, including `""`) | counted as **`0`** | coerced by the ordinary rules — `"3"` becomes `3`, unparseable text is `#VALUE!` |
| Blank cell | **skipped** — not counted at all | — |
| Omitted argument | — | skipped |
| Error value | propagates | propagates |

The three consequences that reach real workbooks:

1. **Text counts as zero, and a zero is a data point.** Substituted zeros pull the mean toward
   zero and sit far from the genuine observations, inflating the deviation twice over. `STDEVPA`
   over a column with a few "n/a" labels can be several times [STDEV.P](FUNC.STDEV.P.md) over the
   same column.
2. **Blank is not text and is not zero — it is skipped.** An empty cell contributes nothing; a
   cell containing `""` contributes an observation at `0`. The two are indistinguishable on screen
   and decisive here, so a formula that emits `""` as its "nothing" result is *not* neutral to
   `STDEVPA` while a genuinely blank cell is. This is the most surprising row in the table and the
   one worth probing first.
3. **Text behaves differently as a direct argument.** `STDEVPA(1, 2, "3")` admits `3` under
   ordinary to-number coercion, where `"3"` in a scanned cell would count as `0`. The same three
   characters mean two different numbers depending on how they arrived.

**When `STDEVPA` is the right function.** When the data genuinely *is* the whole population *and*
non-numeric entries genuinely mean zero. Indicator columns of `TRUE`/`FALSE` are the clean case:
they are exhaustive, and the logical-to-one-or-zero mapping is exactly the intended reading. For
measurement data with missing entries typed as text, `STDEVPA` computes an accurate standard
deviation of the wrong data set.

## Result and edge cases

Returns `Number`, non-negative, in the units of the data.

- **No admitted values at all.** `#DIV/0!`. Reaching this requires every argument to be blank or
  omitted — a range of pure text does *not* reach it, because text is admitted as `0`.
- **Exactly one admitted value.** Exactly `0`, since the population divisor is defined at `n = 1`.
  This is where `STDEVPA` and [STDEVA](FUNC.STDEVA.md) part company on identical input.
- **A range of only text.** Every cell counts as `0`, all admitted values are equal, and the
  answer is exactly `0` — not `#DIV/0!`, which is what [STDEV.P](FUNC.STDEV.P.md) returns on the
  same range. A zero standard deviation and a `#DIV/0!` are very different things to see in a
  report, and the only difference between the two formulas is one letter.
- **All admitted values equal.** Exactly `0`. An all-`FALSE` range returns `0`; a range mixing
  `TRUE` and `FALSE` returns the population standard deviation of a zero-one vector, which for a
  proportion `p` of `TRUE` values is exactly `sqrt(p(1-p))` up to rounding — a small exact
  identity worth knowing, since it makes indicator columns easy to check by hand.
- **Errors anywhere** propagate as themselves.
- Overflow and underflow of the squared deviations behave as on [STDEV.P](FUNC.STDEV.P.md).

## Errors

| Error | Condition | Source |
|---|---|---|
| `#DIV/0!` | no admitted values | documented and implemented |
| `#VALUE!` | direct scalar text that does not parse as a number | shared coercion rule; note that the same text in a range would count as `0` instead |
| propagated | An error value in any argument surfaces as that error | shared coercion rule |

The `#VALUE!` row is the split worth reading twice: `STDEVPA("abc", 1, 2)` is an error while
`STDEVPA(A1:A3)` with `"abc"` in `A1` computes over `{0, 1, 2}`. Microsoft's documented sentence
that text evaluates as `0` is a statement about the range-scan lane; the direct lane is governed
by ordinary coercion. The Handbook can cite no documentation that states the split explicitly, so
it is recorded here as reference-engine behaviour that has not been compared against Excel.

Note also how hard `#DIV/0!` is to reach: because text admits as `0`, almost any non-empty range
produces a number. `STDEVPA` is the least likely of the four to error and the most likely to
return a confidently meaningless value.

## Relationships

- **[STDEV.P](FUNC.STDEV.P.md)** is the same estimator with the skip-non-numeric range policy.
  Comparing the two over the same range is the standard diagnostic when a population standard
  deviation looks wrong.
- **[STDEVA](FUNC.STDEVA.md)** is the sample form of this function — same inclusion rule, `n - 1`
  divisor — related exactly by `STDEVPA = STDEVA * sqrt((n-1)/n)` with `n` counting the admitted
  values.
- **`VARPA`** is the square: `STDEVPA = SQRT(VARPA)`, and that is how the reference
  engine computes it.
- **[AVERAGEA](FUNC.AVERAGEA.md)** supplies the same `xbar` under the same inclusion rule.
  **[COUNTA](FUNC.COUNTA.md)** is *not* the matching count — it counts every non-empty cell
  including error values, while `STDEVPA`'s `n` counts admitted values and errors propagate rather
  than counting.
- **`STDEVPA` has no Compatibility-category predecessor and no dotted successor.** The `A`
  variants were untouched by the `.P`/`.S` reorganisation, so `STDEVPA` is both the historical and
  the current name. There is no `STDEVPA.P`, and migration guidance about dotted names does not
  apply.
- **[SKEW.P](FUNC.SKEW.P.md)** uses a population `sigma` computed the [STDEV.P](FUNC.STDEV.P.md)
  way, not the `STDEVPA` way — the A-variant inclusion rule does not propagate to the moment
  functions, which have no A variants at all.
- Readers confuse `STDEVPA` with `STDEV.P` on the assumption that the trailing `A` means "array"
  or "all cells". It means *values* rather than *numbers*.

## Numerical notes

The arithmetic is identical to [STDEV.P](FUNC.STDEV.P.md)'s, and everything on that page's
numerical notes applies without change. In the reference engine that means two-pass centred
accumulation rather than the unstable raw sums-of-squares identity `sum x^2 - (sum x)^2/n`; naive
left-to-right summation for both the
mean and the sum of squares, where compensated summation would bound the error by a constant; a
correctly rounded final `sqrt` that halves the variance's relative error; overflow of squared
deviations above roughly `1.3 * 10^154` and underflow below roughly `10^{-162}`; and summation
order as part of the answer. Chan, Golub and LeVeque (1983) is the reference for the algorithm
choice, and the published record on Excel's variance accuracy — McCullough and Wilson, Knüsel,
Welinder's Gnumeric work — belongs to the same discussion. **What Excel does is recorded in
FN-VAR.S-001**: raw moments with stored steps, a centered sum when either of two stored tests holds,
and a root rounded to 64 bits and then stored.

Two points are specific to `STDEVPA`.

**The inclusion rule changes the conditioning, usually for the worse.** Substituted zeros are not
neutral. If genuine observations cluster around a large value `M` and a handful of text cells
admit as `0`, the mean moves between the two clusters, every genuine observation acquires a
deviation of order `M`, and the sum of squares becomes dominated by terms of order `M^2` in which
the information about the real spread is a small correction. In the reference engine the two-pass
form still avoids the *catastrophic* cancellation the one-pass form would suffer, but the computed
answer is now an
accurate standard deviation of a bimodal data set rather than an inaccurate one of the original.
Overflow also becomes reachable earlier, because deviations of order `M` square to order `M^2`.
`STDEVPA`'s accuracy risk is semantic rather than arithmetic, and that is the more common kind of
failure in practice.

**Indicator data is the numerically ideal case.** When every admitted value is `0` or `1` — a
pure logical column, or a column of logicals and text — the data is a vector of exact small
integers. The mean is a ratio of small integers, the deviations are exactly representable for any
`n` up to the limits of the significand, and the sum of squares is exact. `sigma^2` is then
`p(1-p)` computed as a single division of exact quantities, and the only rounding in the whole
function is that division and the square root. `STDEVPA` over indicator data is about as accurate
as a floating-point standard deviation can be, and this is the case the function is genuinely for.

## What has not been checked

No Handbook vector suite exists for `STDEVPA`, and no Handbook evidence record names `STDEVPA` as
a subject. FN-VAR.S-001 checks this function against Excel: with the rest of the variance family,
all 14,679
recorded rows match. Still open there: whether its two tests keep the same edges for ranges and
arrays. The implementing module carries no tests of its own in the reference engine; the shared
variance
kernel it calls carries a small number, which speaks to that kernel rather than to this surface.

Everything above marked as documented comes from Microsoft's `STDEVPA` page. **Retrieval of that
page was blocked by the upstream host on this pass**, so those statements are recorded as
documented behaviour with the source named and should be re-read against the live page. The
direct-versus-scan split for text is stated from the reference engine, not from documentation.

Inputs worth probing first:

1. **Empty cell against empty string.** A three-cell range holding `1`, `2` and a genuinely blank
   cell, against the same range with `=""` in the third. If the answers differ, the
   skip-versus-zero distinction is confirmed, and it is the most consequential undocumented
   behaviour on this page.
2. **A range of only text**, against `0`, and the same range through
   [STDEV.P](FUNC.STDEV.P.md), against `#DIV/0!`. One pair of cells demonstrates the whole
   difference between the two functions.
3. **`STDEVPA(A1:A3)` with text in `A1`** against `STDEVPA("abc", 1, 2)` — the direct-versus-scan
   split, where the reference engine gives a number and an error respectively.
4. **The indicator identity**: a range of `TRUE` and `FALSE` in a known proportion `p`, against
   `SQRT(p*(1-p))`, bitwise. Exact in real arithmetic and cheap to construct, which makes it a
   strong single-formula check on the whole path.
5. **A single admitted value**, confirming `0` here where [STDEVA](FUNC.STDEVA.md) gives
   `#DIV/0!`.
6. **`STDEVPA` against `SQRT(VARPA)`**, bitwise, is answered: `STDEVPA` is the root of the variance,
   rounded to 64 bits and then stored (FN-VAR.S-001).
   **`STDEVPA` against `STDEVA * SQRT((n-1)/n)`** remains, measuring the gap the exact identity
   leaves after rounding.
7. **The shift test** from [STDEV.P](FUNC.STDEV.P.md), applied on data containing substituted
   zeros, where the substitution has already spoiled the conditioning.
8. **An error value in a range alongside text**, confirming propagation wins over substitution.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `STDEVPA` that an implementer matching Excel, or a reader explaining an
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
| A variant | The `value`-taking sibling of a `number`-taking aggregate; converts where the other skips |
| inclusion rule | Which values become data points, and what number each becomes |
| substituted zero | A `0` contributed by a text cell rather than by a number |
| admitted values | The `n` values that reach the variance kernel |
| population divisor | The plain `n`, with no Bessel correction |
| indicator data | Values that are all `0` or `1`; the case where this function is both apt and exact |

## Sources

- Microsoft, *STDEVPA function* —
  <https://support.microsoft.com/en-us/office/stdevpa-function-5578d4d6-455a-4308-9991-d405afe2c28c>
  (signature, the `n` divisor, the rules that `TRUE` evaluates as `1` and that text and `FALSE`
  evaluate as `0`, that empty cells are not counted, and the `#DIV/0!` condition). Retrieval was
  blocked by the upstream host for this page; the documented behaviour above is stated as
  documented behaviour and should be re-checked against the page.
- T. F. Chan, G. H. Golub and R. J. LeVeque, "Algorithms for Computing the Sample Variance:
  Analysis and Recommendations", *The American Statistician* 37(3), 1983, pp. 242–247.
- B. D. McCullough and B. Wilson, "On the accuracy of statistical procedures in Microsoft Excel",
  *Computational Statistics & Data Analysis*, 1999, and the follow-up assessments through 2005;
  L. Knüsel's parallel work; M. Welinder's writing on Excel's statistical functions arising from
  the Gnumeric project.
- OxFunc `crates/oxfunc_core/src/functions/stdevpa_fn.rs`,
  `crates/oxfunc_core/src/functions/variance_common.rs` and
  `crates/oxfunc_core/src/functions/aggregate_common.rs` at commit `473efa3` — the
  `AVERAGEA`-like inclusion policy (text to `0`, logicals to `1`/`0`, blanks skipped, direct
  scalars coerced), the population divisor, and the shared two-pass variance.
- Handbook [coercion and lifting](../model/02-coercion-and-lifting.md) — the direct-argument
  versus range-scan dual policy, and the Empty/Missing distinction behind the blank-cell row.
- Handbook [the value universe](../model/01-value-universe.md) — the difference between an empty
  cell and a cell containing an empty string.
- Handbook `data/functions/FUNC.STDEVPA.json`, `data/presence/FUNC.STDEVPA.json`.
