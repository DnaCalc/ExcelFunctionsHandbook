---
schema: efh.function-page/v1
function_id: FUNC.RSQ
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — RSQ function"
    locator: "https://support.microsoft.com/en-us/office/rsq-function-d7161715-250d-4a01-b80d-a8364f2be08f"
    role: "documented signature, the stated equation, and the documented #N/A and #DIV/0! conditions"
  - work: "Chan, T. F., Golub, G. H. and LeVeque, R. J., \"Algorithms for Computing the Sample Variance: Analysis and Recommendations\""
    locator: "The American Statistician 37(3), 1983, pp. 242-247"
    role: "the error analysis separating the two-pass centred form from the raw sums-of-squares form"
  - work: "OxFunc — paired_stats_common.rs"
    locator: "crates/oxfunc_core/src/functions/paired_stats_common.rs"
    role: "reference-engine kernel: RSQ is computed as the square of the correlation coefficient"
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
family: rsq_fn
role_in_family: >-
  The coefficient of determination of the simple linear fit: the squared Pearson correlation,
  the only member of the regression set that is symmetric in its two arguments.
---

# RSQ

## What it computes

`RSQ(known_y's, known_x's)` returns `r^2`, the **coefficient of determination** of the simple
linear regression of `y` on `x` — equivalently, the square of the Pearson product-moment
correlation coefficient.

For pairs `(x_i, y_i)` with means `xbar`, `ybar` and centred sums

    Sxx = sum (x_i - xbar)^2 ,   Syy = sum (y_i - ybar)^2 ,   Sxy = sum (x_i - xbar)(y_i - ybar)

the correlation and its square are

    r     =  Sxy / sqrt( Sxx * Syy )
    RSQ   =  r^2  =  Sxy^2 / ( Sxx * Syy )

Its range is `[0, 1]`, and the two ends have plain meanings: `1` exactly when the points are
collinear (with nonzero spread in both coordinates), `0` exactly when `Sxy = 0`, meaning the
least-squares line is flat and knowing `x` tells you nothing about `y` in a linear sense.

Three readings are worth carrying:

1. **Variance explained.** `RSQ = 1 - SSE/SST`, where `SST = Syy` is the total sum of squares of
   `y` about its mean and `SSE = Syy - Sxy^2/Sxx` is what the fitted line leaves. `RSQ` is the
   fraction of the variation in `y` that the line accounts for. This is the reading that makes
   it a *goodness of fit* rather than a strength of association.
2. **A product of two slopes.** `RSQ(y,x) = SLOPE(y,x) * SLOPE(x,y)`. The regression of `y` on
   `x` and the regression of `x` on `y` are different lines; `r^2` is exactly the amount by which
   their slopes fail to be reciprocals. This identity is why `RSQ` is **symmetric in its
   arguments** while [SLOPE](FUNC.SLOPE.md) is not — swapping `known_y's` and `known_x's` changes
   nothing about the answer.
3. **A squared cosine.** `r` is the cosine of the angle between the centred `x` and `y` vectors,
   so `RSQ` is that cosine squared. This reading explains why the value is dimensionless and why
   it is invariant under any positive affine rescaling of either variable.

Two things `RSQ` is emphatically not. It is not a measure of whether a linear model is
*appropriate* — Anscombe's quartet is four data sets with the same `RSQ` and four completely
different pictures. And it is not a measure of slope magnitude: a tiny slope measured precisely
gives `RSQ` near `1`, and a steep slope measured noisily gives `RSQ` near `0`. Read `RSQ` next to
[SLOPE](FUNC.SLOPE.md) and [STEYX](FUNC.STEYX.md), never alone.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `known_y's` | The dependent values. First. | yes |
| `known_x's` | The independent values. Second. | yes |

The `y` argument comes first, matching [SLOPE](FUNC.SLOPE.md), [INTERCEPT](FUNC.INTERCEPT.md) and
[STEYX](FUNC.STEYX.md). Unlike those three, getting the order wrong here is harmless: `RSQ` is
symmetric, so the answer is the same either way. (It is not harmless in
[CORREL](FUNC.CORREL.md)'s cousin functions when the sign of `r` matters, and `RSQ` discards the
sign entirely — which is its principal weakness as a reported statistic.)

Both arguments are consumed by scanning, and pairing is **positional**, row-major over each
argument; only the element counts must match, not the shapes.

**Non-numeric cells are handled by pairwise deletion.** If either member of a pair is text, a
logical or a blank cell, the reference engine drops the whole pair. This is stronger than the
documentation's "such values are ignored", and it is observable: a blank in `known_x's` removes
its `y` partner and therefore changes `ybar` and `Syy`.

## Result and edge cases

Returns `Number` in `[0, 1]`.

- **Fewer than two complete pairs.** `#DIV/0!`.
- **`Sxx = 0` or `Syy = 0`** — no spread in one of the variables. `#DIV/0!`. There is no
  defensible value: with `Syy = 0` there is nothing to explain, and with `Sxx = 0` there is
  nothing to explain it with. Note the guard is an exact comparison against zero, so data with a
  real but vanishing spread produces a number rather than an error.
- **Perfect collinearity.** Mathematically `1`. In floating point the returned value can fall a
  few units in the last place *short* of `1`; see Numerical notes. Code that tests
  `RSQ(...) = 1` will fail on data that is exactly collinear.
- **Unequal element counts.** `#N/A`, documented and implemented.
- **Both arguments empty.** The length check passes (both zero) and the fit fails, so the
  reference engine returns `#DIV/0!`. **Microsoft's documented condition for empty arguments is
  `#N/A`.** This is a documentation-versus-reference-engine divergence, recorded here as a
  finding; see Errors.
- **Errors in either argument** propagate as themselves.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#N/A` | `known_y's` and `known_x's` have different numbers of data points | documented and implemented |
| `#N/A` | the arguments are empty | **documented**; the reference engine returns `#DIV/0!` here instead |
| `#DIV/0!` | fewer than two complete pairs | documented and implemented |
| `#DIV/0!` | `Sxx = 0` or `Syy = 0` | implemented; follows from the definition |
| propagated | An error value in either argument surfaces as that error | shared coercion rule |

The empty-argument row is the divergence to carry away, and it is shared with
[SLOPE](FUNC.SLOPE.md), [INTERCEPT](FUNC.INTERCEPT.md) and the rest of the paired-statistics set:
Microsoft's page groups "empty" with "different number of data points" under one `#N/A`, while
the reference engine's length check passes on two empty vectors and the failure falls through to
the fit as `#DIV/0!`. A formula guarded with `IFNA` rather than `IFERROR` behaves differently
under the two readings. The Handbook has not observed which one Excel returns.

## Relationships

- **[CORREL](FUNC.CORREL.md) and [PEARSON](FUNC.PEARSON.md)** return `r` itself; `RSQ` is `r^2`.
  `RSQ` therefore discards the *sign* of the relationship, which is usually the first thing a
  reader wants. `CORREL` and `PEARSON` are documented as the same quantity and are two separate
  published entries; whether they are one computation is a question for their own pages.
- **[SLOPE](FUNC.SLOPE.md)** satisfies `RSQ(y,x) = SLOPE(y,x) * SLOPE(x,y)`. The two functions
  answer different questions about the same line — how steep, and how tight.
- **[STEYX](FUNC.STEYX.md)** is the standard error of the regression, `sqrt(SSE/(n-2))`, and is
  the absolute-scale companion to `RSQ`'s relative one. `RSQ` near `1` with a large `STEYX` means
  the data spans a large range, not that the fit is tight in absolute terms.
- **[LINEST](FUNC.LINEST.md)** returns `RSQ` as one cell of its extended statistics block, along
  with the standard errors and the `F` statistic that `RSQ` alone cannot supply.
- **[INTERCEPT](FUNC.INTERCEPT.md), [TREND](FUNC.TREND.md),
  [FORECAST.LINEAR](FUNC.FORECAST.LINEAR.md)** complete the simple-regression set.
- Readers confuse `RSQ` with the adjusted `R^2` reported by statistics packages, which penalises
  for the number of predictors. With one predictor the two differ by a factor in `n`; Excel
  publishes no adjusted form.

## Numerical notes

The most consequential fact about `RSQ` in the reference engine is an implementation choice that
the mathematics does not force: **`RSQ` is computed by squaring the correlation coefficient**,
not from the closed form `Sxy^2 / (Sxx * Syy)`.

Written out, the implemented route is

    r    =  ( Sxy/(n-1) )  /  ( sqrt(Sxx/(n-1)) * sqrt(Syy/(n-1)) )
    RSQ  =  r * r

against the direct route

    RSQ  =  (Sxy * Sxy) / (Sxx * Syy)

They are algebraically identical and numerically are not. The implemented route performs two
square roots, a multiplication of the roots, a division and finally a squaring — five rounded
operations where the direct route needs three, and two of them are the square roots that the
direct route does not need at all. The direct route is both faster and shorter in its error
chain.

The visible consequence is at the top of the range. For exactly collinear data the true value is
`1`, and the implemented route reaches it through `r`, which is itself a rounded quotient
slightly below `1`; squaring a value slightly below `1` moves it further below. The reference
engine's own witness assertions for this kernel record a returned value strictly less than `1` on
perfectly collinear input, with the correlation and its square each landing a small number of
units in the last place short. The Handbook does not restate those figures — they belong to the
evidence layer — but the *shape* of the result is a mathematical consequence and worth stating:
**`RSQ` on collinear data is not exactly `1`, and should not be tested for equality with `1`.**
The direct closed form would not automatically fix this either; a genuinely exact answer at the
top of the range requires either a fused multiply-add formulation or an explicit clamp, and both
are decisions to record rather than defaults.

The same argument applies to the bottom of the range with the opposite sign: `Sxy` near zero is a
cancelling sum of mixed-sign terms, so `RSQ` near zero has poor *relative* accuracy. Its absolute
accuracy stays good, which is usually what a reader of an `RSQ` near zero cares about.

**The centred versus raw forms.** Everything said on [SLOPE](FUNC.SLOPE.md) about the two-pass
centred accumulation applies here and applies twice, because `RSQ` needs `Sxx` and `Syy` and
`Sxy`. The one-pass form `n*sum(x^2) - sum(x)^2` loses digits in proportion to the square of the
mean-to-spread ratio and can return a negative sum of squares; the centred form the reference
engine uses does not. Chan, Golub and LeVeque (1983) is the analysis. **This page does not assert
what Excel does internally** — only that the two are distinguishable by measurement on
ill-conditioned data, and that the measurement has not been made here.

**Residual hazards in the good form.** The means and all three centred sums are accumulated by
naive left-to-right summation, so the error grows with `n` where compensated or pairwise
summation would bound it. `Sxx` and `Syy` have non-negative terms and are well-conditioned;
`Sxy` does not and is not. The `Sxx == 0` and `Syy == 0` guards are exact comparisons, not
tolerances, so data with real but tiny spread yields a number rather than a diagnostic.

**Argument-slot note.** In the reference engine, `RSQ` binds its first argument into the kernel
slot named for `x` and its second into the slot named for `y` — the opposite of the binding
[SLOPE](FUNC.SLOPE.md) uses on the same kernel. Because the pair collector, the covariance and
both variances are all symmetric under exchanging the two vectors, the returned value is
provably unaffected. It is worth recording only so that a reader comparing the two modules does
not conclude that one of them has its arguments backwards in a way that matters.

## What has not been checked

No Handbook vector suite exists for `RSQ`, and no Handbook evidence record names `RSQ` as a
subject. Nobody has checked this function against Excel within the Handbook's record. `RSQ`
shares a kernel with [SLOPE](FUNC.SLOPE.md) and [INTERCEPT](FUNC.INTERCEPT.md), for which one
live-verification record exists naming those two surfaces; that record does not name `RSQ`, and
no figure from it attaches here. A shared kernel is a reason to expect correlated behaviour, not
a licence to inherit a measurement.

Everything above marked as documented comes from Microsoft's `RSQ` page. **Retrieval of that page
was blocked by the upstream host on this pass**, so those statements are recorded as documented
behaviour with the source named and should be re-read against the live page — the empty-argument
`#N/A` in particular, since it is the divergence this page reports.

Inputs worth probing first:

1. **Exactly collinear data**: `y = 2x` on `x = {1,2,3,4,5}`. Compare the returned bits against
   `1`. This probe distinguishes squaring-the-correlation from the direct closed form, and it is
   the cheapest structural fingerprint of Excel's route that exists.
2. **`RSQ(y,x)` against `SLOPE(y,x) * SLOPE(x,y)`**, bitwise, on the same data. The identity is
   exact in real arithmetic; the gap measures the three functions against each other and would
   show immediately if `RSQ` were computed from a different intermediate.
3. **`RSQ(y,x)` against `CORREL(y,x)^2`**, bitwise. If Excel computes `RSQ` by squaring its own
   `CORREL`, they agree exactly; if it uses a closed form, they need not.
4. **Argument symmetry**: `RSQ(y,x)` against `RSQ(x,y)`, bitwise. The mathematics says identical;
   an implementation that centres in a fixed order might not deliver identical bits.
5. **The empty-argument case**, against `#N/A` (documented) and `#DIV/0!` (reference engine).
6. **The ill-conditioned pair**: the same relationship with `x` offset by a million, against the
   unshifted version. A degradation identifies the raw-sums form.
7. **Pairwise deletion**: a blank in `known_x's` opposite a nonzero `y`, compared against the same
   data with that pair physically removed.
8. **`Syy = 0` with `Sxx > 0`** and the reverse, confirming both guards fire as `#DIV/0!`.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| coefficient of determination | `r^2`; the fraction of variation in `y` the fitted line accounts for |
| `Sxx`, `Syy`, `Sxy` | The centred sums of squares and the cross-product |
| `SST`, `SSE` | Total and residual sums of squares; `RSQ = 1 - SSE/SST` |
| pairwise deletion | Dropping both members of a pair when either is non-numeric |
| centred (two-pass) form | Subtract the mean, then accumulate; the numerically stable route |
| squaring the correlation | The implemented route: compute `r`, then `r*r` |

## Sources

- Microsoft, *RSQ function* —
  <https://support.microsoft.com/en-us/office/rsq-function-d7161715-250d-4a01-b80d-a8364f2be08f>
  (signature and argument order, the stated equation, the ignore-text-and-logicals rule for
  arrays, and the documented `#N/A` and `#DIV/0!` conditions). Retrieval was blocked by the
  upstream host for this page; the documented behaviour above is stated as documented behaviour
  and should be re-checked against the page.
- T. F. Chan, G. H. Golub and R. J. LeVeque, "Algorithms for Computing the Sample Variance:
  Analysis and Recommendations", *The American Statistician* 37(3), 1983, pp. 242–247.
- F. J. Anscombe, "Graphs in Statistical Analysis", *The American Statistician* 27(1), 1973 —
  the quartet behind the warning that `RSQ` does not test model adequacy.
- N. J. Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd ed., chapter 4 — the
  summation error bounds behind the residual-hazards list.
- OxFunc `crates/oxfunc_core/src/functions/rsq_fn.rs` and
  `crates/oxfunc_core/src/functions/paired_stats_common.rs` at commit `473efa3` — pairwise
  deletion, the unequal-length `#N/A`, the correlation-then-square route, the `Sxx == 0` and
  `Syy == 0` guards, and the argument-slot binding noted above.
- Handbook `data/functions/FUNC.RSQ.json`, `data/presence/FUNC.RSQ.json`.
