---
schema: efh.function-page/v1
function_id: FUNC.PEARSON
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "B. D. McCullough and B. Wilson, On the accuracy of statistical procedures in Microsoft Excel"
    locator: "Computational Statistics & Data Analysis, 1999 / 2002 / 2005 series"
    role: "The published record of Excel's statistical accuracy, including the CORREL/PEARSON discrepancy"
  - work: "T. F. Chan, G. H. Golub and R. J. LeVeque, Algorithms for computing the sample variance"
    locator: "The American Statistician 37 (1983) 242-247"
    role: "The comparative analysis of one-pass, two-pass and updating formulas for variance and covariance"
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
family: pearson_fn
role_in_family: >-
  The product-moment correlation coefficient under its statistician's name; mathematically
  identical to CORREL and, in the published literature, historically not identical to it in
  Excel.
---

# PEARSON

## What it computes

`PEARSON(array1, array2)` returns the **Pearson product-moment correlation coefficient** `r`
between two paired samples — the standardized measure of linear association.

For paired observations `(x₁,y₁) … (xₙ,yₙ)` with means `x̄` and `ȳ`:

    r  =  Σ (xᵢ − x̄)(yᵢ − ȳ)
          ────────────────────────────────────────
          √( Σ (xᵢ − x̄)²  ·  Σ (yᵢ − ȳ)² )

Microsoft's page gives this equation as an image and states that `x̄` and `ȳ` are
`AVERAGE(array1)` and `AVERAGE(array2)`.

Equivalently, in the language of moments,

    r = Cov(x, y) / ( σ_x · σ_y )

with the same denominator convention (population or sample) used top and bottom, so the choice
cancels and `r` is unambiguous where variance and covariance are not.

**Domain and range.** Defined for `n ≥ 2` paired numeric observations with **both** samples
non-constant. The range is the closed interval `[−1, +1]`:

    r = +1   exactly    iff  y is an increasing affine function of x
    r = −1   exactly    iff  y is a decreasing affine function of x
    r =  0              no linear association (which is not the same as independence)

The bound is the Cauchy–Schwarz inequality applied to the centred vectors, and equality holds
exactly when they are parallel. Geometrically `r` is the cosine of the angle between the two
centred data vectors in `ℝⁿ` — which is the single most useful way to think about it, because it
explains at once why `r` is invariant under any positive affine rescaling of either variable
and why it is undefined when either centred vector is zero.

**Invariances.** For `a, c > 0`:

    r(ax + b, cy + d) = r(x, y)
    r(x, y) = r(y, x)                    symmetric
    r(−x, y) = −r(x, y)                  sign flips with orientation

The affine invariance is why `r` is dimensionless and comparable across units, and it is also
the source of the numerical hazard described below: a computation that is not itself
shift-invariant will return different answers for data that differ only by a large offset.

**Degeneracy.** If either sample is constant, the corresponding centred vector is zero, the
denominator vanishes, and `r` is undefined — genuinely undefined, not zero. This is not an edge
case to be papered over: constant columns arise constantly in real spreadsheets.

## Arguments

Microsoft's page gives two required arguments:

| Argument | Meaning | Required |
|---|---|---|
| `array1` | "A set of independent values" | yes |
| `array2` | "A set of dependent values" | yes |

The independent/dependent labelling is a convention only — `r` is symmetric in its arguments,
unlike `SLOPE`, `INTERCEPT` and `FORECAST`, which are not. Swapping the arguments of `PEARSON`
changes nothing; swapping the arguments of `SLOPE` changes the answer.

Microsoft states the admission rule explicitly, and it is the standard statistical-family rule:

> "The arguments must be either numbers or names, array constants, or references that contain
> numbers."
>
> "If an array or reference argument contains text, logical values, or empty cells, those values
> are ignored; however, cells with the value zero are included."

Note what that rule does **not** say: it does not say how the pairing survives the omission. If
`array1` has text in row 7 and `array2` has a number there, is the pair dropped, or is the text
dropped and the numbers re-aligned? The published wording is compatible with both readings and
they give different answers. See the probe list.

The reference engine records an arity of exactly 2, a `Custom` kernel signature class and a
`Custom` coercion/lift profile — the admission decision belongs to this function rather than to
a family default. See [Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number` in `[−1, +1]`.

- **`n = 1`, or arrays of length 1.** One point defines no line; both centred vectors are zero.
  The reference engine's battery renders several such degenerate rows and their outcomes show
  beside this page.
- **A constant sample.** Undefined mathematically. Microsoft's `PEARSON` page documents no error
  for this case at all — see Errors below, where the Handbook records that as a gap.
- **Mismatched lengths** are a documented `#N/A`.
- **Perfect correlation.** `r = ±1` should be returned exactly for exactly-collinear data, and
  whether it is depends entirely on the formula used — the classical failure is a computed `r`
  slightly exceeding `1` in magnitude, which then produces a `#NUM!` in any downstream
  `ATANH`, `FISHER` or confidence-interval calculation. Clamping to `[−1, 1]` is a defensible
  and commonly omitted implementation step.
- **Arrays.** `PEARSON` is an aggregate over two grids, not a lift kernel; it returns a scalar.

## Errors

As documented by Microsoft on the `PEARSON` page (retrieved for this page):

| Error | Documented condition |
|---|---|
| `#N/A` | `array1` and `array2` are empty, or have a different number of data points |

**That is the whole documented error surface — one row — and it does not cover the case that
matters most.** The `PEARSON` page states nothing about what happens when a sample is constant
and the denominator is zero. That is the natural `#DIV/0!` condition, it is documented on the
`CORREL` page for the mathematically identical function, and it is absent here. The reference
engine's battery renders degenerate inputs and returns a division error for them; its outcomes
show beside this page.

The Handbook records this as a **documentation gap** rather than as a behavioural claim: the
divide-by-zero behaviour is not in dispute among implementations, but Microsoft's `PEARSON`
page does not document it, while its `CORREL` page documents the analogous condition for the
same computation. Two pages describing one function, with different error coverage.

Error values arriving in either array propagate under the ordinary coercion rules.

## Relationships

- **`CORREL`** computes the same mathematical quantity. Two Excel surfaces, one statistic —
  and this pair carries a specific place in the published literature. **McCullough and Wilson**,
  in their series on Excel's statistical accuracy in *Computational Statistics & Data Analysis*,
  report that in Excel versions through the early 2000s `CORREL` and `PEARSON` — algebraically
  identical — returned **different** answers on ill-conditioned data, with `PEARSON` implemented
  through the numerically inferior formula, and that this was among the defects addressed in
  later versions. The Handbook names that finding as published literature. It has not re-run
  those tests, restates none of their figures, and makes no claim about any current Excel build.
  What the reader should take from it is the shape of the risk: **two names for one statistic in
  one product is exactly the configuration in which two different formulas hide**, and it is why
  comparing `PEARSON` against `CORREL` is the first probe on this page.
- **`RSQ`** is `r²`: `RSQ(y, x) = PEARSON(x, y)^2` mathematically. Squaring destroys the sign
  and, numerically, halves the significant digits of a near-`1` correlation — so `RSQ` computed
  as `PEARSON^2` and `RSQ` computed directly need not agree.
- **`SLOPE`**, **`INTERCEPT`**, **`STEYX`** and **`LINEST`** are the regression family built on
  the same three sums of squares and cross-products. `SLOPE(y, x) = r · σ_y/σ_x`. Unlike
  `PEARSON`, these are **not** symmetric in their arguments.
- **`COVARIANCE.P`** and **`COVARIANCE.S`** are the unstandardized numerator.
- **`FISHER`** is the usual consumer: `FISHER(r) = atanh(r)` is the variance-stabilizing
  transform used to build confidence intervals for `r`, and it is `#NUM!` at `r = ±1` — which is
  precisely why an implementation that returns `1.0000000000000002` for perfectly collinear data
  breaks a downstream formula that has nothing wrong with it.
- **Confused with**: Spearman's rank correlation (which Excel has no direct function for —
  `PEARSON` over `RANK` values is the usual construction), and with the regression slope, which
  `r` is not unless both samples are standardized.

## Numerical notes

The correlation coefficient is the standard classroom example of a formula that is correct in
exact arithmetic and disastrous in floating point, and it deserves to be spelled out.

**1. The textbook "computational formula" is the trap.** Algebra gives

    Σ(xᵢ − x̄)(yᵢ − ȳ)  =  Σxᵢyᵢ − n·x̄·ȳ

and the right-hand side is attractive because it needs one pass and no stored means. It is also
**catastrophically cancelling**: both terms are of order `n·x̄·ȳ`, their difference is of order
`n·σ_xσ_y`, and the relative error is amplified by roughly

    (x̄ · ȳ) / (σ_x · σ_y)

which is the product of the two coefficients of variation, inverted. For data with a large mean
and a small spread — measurements around 1000 varying by 0.1, dates, prices, sensor readings —
that factor is `10⁸` or more and **the answer loses eight digits before anything else happens**.
The same one-pass form applied to `Σxᵢ²  − n x̄²` can even produce a negative sum of squares,
after which the square root in the denominator is a domain error on data that is perfectly fine.

This is the exact defect the McCullough–Wilson series reports having found in Excel's
statistical procedures, and the exact defect that the recommended alternatives fix.

**2. The two-pass centred formula is the honest default.** Compute `x̄` and `ȳ` first, then
accumulate `Σ(xᵢ−x̄)(yᵢ−ȳ)`, `Σ(xᵢ−x̄)²` and `Σ(yᵢ−ȳ)²` in a second pass. Cost: one extra pass
and one array traversal. Benefit: the cancellation is gone, because the quantities being summed
are the quantities the answer is made of. Chan, Golub and LeVeque (*The American Statistician*,
1983) analyse this comparison for the variance and give the error bounds; the covariance case is
identical in structure. A **corrected two-pass** variant, which adds back the (theoretically
zero) residual `(Σ(xᵢ−x̄))²/n`, recovers another digit or two and costs nothing.

**3. Updating formulas when one pass is required.** Welford's recurrence for the variance and
its bivariate extension (the Youngs–Cramer form) maintain the centred sums incrementally:

    dx = xᵢ − x̄ₙ₋₁
    x̄ₙ = x̄ₙ₋₁ + dx/n
    C  ← C + dx · (yᵢ − ȳₙ)

These are one-pass, streaming, and numerically comparable to the two-pass method. There is no
accuracy argument left for the naive computational formula; only an ignorance-of-the-literature
argument.

**4. The denominator and the bound.** `√(Sxx · Syy)` should be formed as
`√Sxx · √Syy` rather than `√(Sxx·Syy)` when the two sums differ wildly in magnitude, to avoid
overflow or underflow of the product. And the result should be clamped to `[−1, 1]`: exact
collinearity can produce a computed `|r|` a few ULP above `1`, which is mathematically
impossible and downstream-fatal.

**5. Summation order.** For long arrays the three sums are themselves a summation-accuracy
problem. Pairwise or Kahan summation costs little and removes the `O(n)` error growth of naive
accumulation. Where a function is expected to reproduce another system's bits, the summation
order becomes part of the specification rather than an implementation detail.

**What a careful independent implementation does**: two passes (or Welford), corrected means,
pairwise summation, separated square roots, and an explicit clamp — with a stated bound. See
[About implementation options](../model/07-implementation-options.md).

## What has not been checked

No Handbook vector suite exists for `PEARSON`, and **no Handbook evidence record names this
surface**. Nobody has compared this function against Excel within the Handbook's record.

What exists instead is published literature — the McCullough–Wilson series — reporting a
`CORREL`/`PEARSON` discrepancy in Excel versions of the early 2000s. The Handbook names that
finding, does not restate its figures, and explicitly does **not** claim it describes any
current build. It is a reason to run the experiment, not a substitute for running it.

Microsoft's documented behaviour above was retrieved from the `PEARSON` page, including the
value-admission rule and the single `#N/A` condition; the equation on that page is an image and
was not read, and the formula in this page's first section is stated as mathematics.

Inputs worth probing first:

1. **`PEARSON(x, y)` against `CORREL(x, y)` on ill-conditioned data** — for example
   `x = {10⁹+1, 10⁹+2, 10⁹+3, 10⁹+4}` against a correlated `y` of similar scale. **This is the
   experiment.** Two Excel surfaces for one statistic; if they disagree at all, they are running
   different formulas, and the literature says they once did. One column settles it for a given
   build.
2. **The shift-invariance test**: `PEARSON(x, y)` against `PEARSON(x + c, y)` for
   `c = 10³, 10⁶, 10⁹, 10¹²`. Mathematically the answer is unchanged. The rate at which it
   *does* change is a direct, oracle-free measurement of the cancellation described above and
   needs no reference implementation at all. It is the single most informative experiment on
   this page and it is a handful of cells.
3. **Exact collinearity**: `y = 2x + 3` for a spread of `x`. The answer must be exactly `1`.
   Whether it is `1`, `0.9999999999999998`, or a value above `1` tells you about the
   denominator and the clamp, and `FISHER(PEARSON(...))` turns the last of those into a visible
   `#NUM!`.
4. **A constant column** in each position, which is the case Microsoft's page does not document
   at all. Determining what Excel returns there closes the documentation gap recorded above.
5. **Mixed content and the pairing question**: text in `array1` row 7 with a number in
   `array2` row 7, and vice versa. This distinguishes pairwise deletion from independent
   filtering, and Microsoft's wording does not settle it.
6. **Logicals and zeros**, per the documented rule that logicals are ignored in references but
   zeros are included — and the same values passed as direct array constants rather than
   references, which is the direct-versus-scan asymmetry.
7. **Long arrays with a known answer**, to expose summation-order error growth.
8. **`PEARSON(x, y)` against `RSQ(y, x)` and `SLOPE(y, x)·σ_x/σ_y`** — three surfaces over the
   same sums of squares, which must agree and which are computed by three different functions.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| product-moment correlation | `r`, the standardized covariance; this function |
| centred vector | The sample with its mean subtracted; `r` is the cosine of the angle between two |
| computational formula | The one-pass `Σxy − n x̄ȳ` form; algebraically correct, numerically unsound |
| two-pass formula | Means first, centred sums second; the numerically sound default |
| shift invariance | `r(x+c, y) = r(x, y)`; exact in mathematics, a direct accuracy probe in practice |

## Sources

- Microsoft, "PEARSON function" —
  <https://support.microsoft.com/en-us/office/pearson-function-0c3e30fc-e5af-49c4-808a-3ef66e034c18>
  (syntax; the two required arguments; the rule that text, logicals and empty cells in an array
  or reference argument are ignored while zeros are included; and the single `#N/A` condition
  for empty or unequal-length arrays). Retrieved for this page; the equation is an image and was
  not read.
- B. D. McCullough and B. Wilson, "On the accuracy of statistical procedures in Microsoft Excel",
  *Computational Statistics & Data Analysis* (1999, 2002, 2005) — the published record of the
  `CORREL`/`PEARSON` discrepancy and of the one-pass formula's failure on ill-conditioned data.
  Named as literature; no figure restated, and no claim made about any current build.
- T. F. Chan, G. H. Golub and R. J. LeVeque, "Algorithms for computing the sample variance:
  analysis and recommendations", *The American Statistician* 37 (1983) 242–247 — the comparative
  error analysis of the one-pass, two-pass, corrected two-pass and updating formulas.
- B. P. Welford, "Note on a method for calculating corrected sums of squares and products",
  *Technometrics* 4 (1962) 419–420 — the streaming update used for the bivariate case.
- M. Welinder's work on Gnumeric's statistical functions — the standard practical account of
  implementing this family in a spreadsheet.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [About implementation options](../model/07-implementation-options.md).
- `data/functions/FUNC.PEARSON.json` — arity 2, `Custom` kernel signature and coercion profile,
  `ErrorCollapseProfile::None`, XLL symbol `xlfPearson`.
- `data/presence/FUNC.PEARSON.json` — implementing module
  `crates/oxfunc_core/src/functions/pearson_fn.rs`, shared with no other surface. Note that the
  reference engine gives `PEARSON` its own module rather than routing it through `CORREL`'s.
