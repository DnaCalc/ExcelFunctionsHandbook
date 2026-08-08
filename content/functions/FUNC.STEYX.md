---
schema: efh.function-page/v1
function_id: FUNC.STEYX
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
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
family: moment_stats_family
role_in_family: >-
  The regression member of the moment-statistics module: the only one of its siblings that
  consumes two paired vectors rather than one, and the one whose documented formula contains a
  subtraction that can cancel to nothing.
---

# STEYX

## What it computes

`STEYX(known_y's, known_x's)` returns the **standard error of the estimate** for a simple linear
regression of `y` on `x` — the residual standard deviation of the fitted line, and the quantity a
textbook writes `s_{y·x}` or `s_e`.

Microsoft gives the defining formula on the function's page. Written out, with `n` the number of
paired points and bars denoting means:

    STEYX = sqrt(  ( Σ(y − ȳ)²  −  ( Σ(x − x̄)(y − ȳ) )² / Σ(x − x̄)² )  /  (n − 2)  )

The bracketed quantity is the **residual sum of squares** of the least-squares line. Introducing
the usual centred sums

    Sxx = Σ(x − x̄)²,    Syy = Σ(y − ȳ)²,    Sxy = Σ(x − x̄)(y − ȳ)

the formula is `sqrt( (Syy − Sxy²/Sxx) / (n − 2) )`, and every equivalent reading is worth having
in view because each one exposes a different implementation route:

    Syy − Sxy²/Sxx  =  Syy · (1 − r²)  =  Σ(yᵢ − ŷᵢ)²  =  Syy − b · Sxy

where `r` is the Pearson correlation (`RSQ` is `r²`), `b = Sxy/Sxx` is the slope (`SLOPE`), and
`ŷᵢ = a + b·xᵢ` are the fitted values. The last of the four is a sum of squares and therefore
non-negative by construction; the first is a difference of two positive numbers and is not. That
distinction is the entire numerical content of this function, and it is taken up below.

Domain and range: defined for `n ≥ 3` paired finite points with `Sxx > 0`; the result is a
non-negative real in the same units as `y`. It is exactly 0 when the points are collinear, and
it grows without bound as the scatter about the line grows.

The divisor `n − 2` is the residual degrees of freedom: two parameters — slope and intercept —
have been estimated from the data. This makes `STEYX²` the unbiased estimator of the error
variance σ² in the model `y = α + βx + ε`, `ε ~ N(0, σ²)`, which is why it is `n − 2` and not
`n` or `n − 1`.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `known_y's` | "An array or range of dependent data points." | Yes |
| `known_x's` | "An array or range of independent data points." | Yes |

Exactly two arguments; the projection records an arity of exactly two. The two arguments are
**paired positionally** — the `i`-th `y` goes with the `i`-th `x` — so their order matters and
`STEYX(x, y)` is a different, usually smaller or larger, number than `STEYX(y, x)`. The function
is not symmetric in its arguments, and the name does not warn you.

The documented coercion rule is the familiar direct-versus-scan split, stated on the function's
page in the same words the aggregates use:

- **Typed directly into the argument list**: logical values and text representations of numbers
  are counted.
- **Inside an array or reference**: text, logical values and empty cells are ignored — but cells
  containing zero *are* included.

See [Coercion and lifting](../model/02-coercion-and-lifting.md) for why those two rules are
different rules rather than one rule with an exception.

What the documentation does not say, and this page will not invent: **when a value is ignored in
one vector, what happens to its partner in the other?** A regression over paired data has to
answer this — either the pair is dropped together, or the two vectors are compacted
independently and the pairing silently shifts. The two readings give different numbers on the
same input, and the second one gives an answer that is not a regression of anything. Microsoft's
page is silent. It is the first probe below.

## Result and edge cases

Returns `Number`, a non-negative standard error.

- **Perfectly collinear data** gives exactly 0 mathematically, and this is the corner where the
  documented formula is most dangerous: `Syy − Sxy²/Sxx` is a subtraction of two quantities that
  are equal in exact arithmetic. In floating point the difference can come out as a tiny positive
  number, as zero, or as a tiny **negative** number — and the last of those makes the square root
  fail. The reference-engine battery beside this page does return an exact zero on a collinear
  inline array; that is the engine's own answer, on one input, with no Excel involved, and it
  demonstrates nothing about the general case.
- **Near-collinear data** — real calibration data, for instance — is worse than perfectly
  collinear data, because the true answer is small and non-zero, so relative accuracy is
  observable and the cancellation eats it.
- **`n = 3`** is the smallest admissible case (one residual degree of freedom) and is the point
  where the estimator has the least information; `n < 3` is a documented `#DIV/0!`.
- **Constant `x`** makes `Sxx = 0` and the regression undefined. The documented error list does
  not cover it. What Excel does — `#DIV/0!`, `#NUM!`, or a number — is unchecked.
- **Constant `y` with varying `x`** is fine: the fitted line is flat and the standard error is 0.
- **Very large or very small magnitudes.** Because the formula is written in centred deviations
  it does not have the classic uncentred sum-of-squares overflow problem, but `Sxy²` squares an
  already-squared quantity and can overflow for data that `Syy` and `Sxx` handle comfortably. A
  vector of values around `1e160` is enough.
- **Arrays.** The projection records `surface_native` lift with `default-unexamined` provenance.
  Whether Excel spills over higher-dimensional arguments here is unsettled.

## Errors

As documented on Microsoft's `STEYX` page:

| Error | Documented condition |
|---|---|
| `#N/A` | `known_y's` and `known_x's` have a different number of data points. |
| `#DIV/0!` | `known_y's` and `known_x's` are empty, or have fewer than three data points. |
| propagated | "Arguments that are error values or text that cannot be translated into numbers cause errors." |

Not documented: the constant-`x` case (`Sxx = 0`), and what happens if the cancellation in the
documented formula produces a negative radicand. Both would have to resolve to something, and
neither is named. The Handbook has not verified any of this against Excel.

## Relationships

`STEYX` is one output of a single least-squares fit, and Excel spreads that one fit across half a
dozen surfaces. Holding them together is the useful thing this page can do:

| Surface | Which part of the same fit |
|---|---|
| `SLOPE` | `b = Sxy / Sxx` |
| `INTERCEPT` | `a = ȳ − b·x̄` |
| `RSQ` | `r² = Sxy² / (Sxx·Syy)` |
| `PEARSON`, `CORREL` | `r = Sxy / sqrt(Sxx·Syy)` |
| `FORECAST`, `FORECAST.LINEAR` | `a + b·x*` at a new point |
| `TREND` | the same, vectorized and generalized to several predictors |
| `LINEST` | all of the above at once — and its statistics block reports the same standard error of the estimate that `STEYX` returns |
| `STEYX` | `sqrt( Syy(1 − r²) / (n − 2) )` |

The identity worth remembering, because it is exact and testable:
`STEYX² · (n − 2) = Syy · (1 − RSQ)`, with `Syy = DEVSQ(known_y's)`. That gives a route to check
Excel against Excel with no external oracle.

- **`LINEST`** is the closest relative and the one that matters most: it computes this quantity
  as part of its statistics block, so Excel has at least two code paths to the same number. Are
  they the same path? Nobody has published an answer.
- **`STDEV.S`** is the corresponding one-variable object: `STEYX` is to a fitted line what
  `STDEV.S` is to a fitted constant, right down to the degrees-of-freedom bookkeeping
  (`n − 1` for one estimated parameter, `n − 2` for two).
- **Confused with**: the standard error *of the slope* (which `LINEST` also reports, and which is
  `STEYX / sqrt(Sxx)`), and with the standard error *of a prediction* at a new point (which is
  larger, and which no Excel surface returns). `STEYX`'s own English description — "the standard
  error of the predicted y-value for each x in the regression" — invites exactly this confusion,
  because the standard error of a predicted value is not what the documented formula computes.

## Numerical notes

**The subtraction is the problem.** `Syy − Sxy²/Sxx` is mathematically `Syy(1 − r²)`, and when
the fit is good `r²` is close to 1, so the two terms are nearly equal and their difference loses
digits proportionally. For `r² = 1 − δ`, roughly `−log₁₀ δ` decimal digits are gone. Real
regression data with `r² = 0.9999` — an ordinary result in a calibration lab — has already lost
four digits before the square root is taken, and the square root then halves the *relative* error,
which is the one mercy in the arrangement. The failure mode at the end of the road is a negative
radicand and a domain error on `sqrt` for data that is merely collinear to within rounding.

**The fix is to compute a sum of squares, not a difference.** `Σ(yᵢ − ŷᵢ)²` is non-negative by
construction and has no cancellation between terms — only within each residual, where it is
benign because the residual is the quantity of interest. It costs a second pass over the data.
For the same reason a QR factorization of the design matrix by Householder reflections yields the
residual norm directly as the norm of the transformed tail, with no subtraction anywhere; this is
the standard recommendation of the least-squares literature (Lawson & Hanson; Björck; Golub & Van
Loan; Higham, *Accuracy and Stability of Numerical Algorithms*, chapter 20). The one-variable
analogue of the same argument — never compute a variance as `Σx² − (Σx)²/n` — is Higham's §1.9
and Chan, Golub & Van Loan's classic note, and it is the same lesson one dimension down.

**Centring is already correct here.** Microsoft's documented formula is written in deviations
from the means, so it is a two-pass centred form and avoids the worst of the uncentred disease.
That is worth crediting: the documented formula's weakness is the `r² → 1` cancellation, not the
large-offset catastrophe that afflicts uncentred computational formulas.

**Accumulation order matters.** `Sxx`, `Syy` and `Sxy` are sums of products; their accuracy
depends on summation order and on whether a compensated (Kahan/Neumaier) or pairwise sum is used.
For data spanning many magnitudes the difference is visible in the last digits of the answer.

The accuracy of Excel's own statistical procedures — and specifically its regression outputs — is
the subject of a substantial published literature: Morten Welinder's work on Gnumeric's
statistical functions documents the historical failure modes concretely, and the assessments by
Knüsel and by McCullough & Wilson evaluate Excel's regression against the NIST StRD reference
datasets (`Norris`, `Longley`, `Filip`, the `Wampler` series). The Handbook names those sources
because they are the right reading; it does **not** assert from them what any current Excel build
does with `STEYX`, which is a separate question nobody in this record has measured.

## What has not been checked

No Handbook vector suite exists for `STEYX`, and no Handbook evidence record lists this surface
among its subjects. **Nobody has checked `STEYX` against Excel within the Handbook's record.**
The reference-engine battery rendered beside this page is the engine answering its own questions;
no Excel was involved in it. The nearest measured neighbour is `TREND`, whose evidence record
concerns the least-squares pipeline in the reference engine — that record's subject is `TREND`,
and nothing in it may be read onto this page.

Documentation gaps this page could not close: the pairing rule when a value is ignored in one
vector, the constant-`x` case, and what happens when the documented formula's radicand comes out
negative.

Inputs worth probing first:

1. **Paired exclusion.** `known_y's` = `{1; 2; "x"; 4}`, `known_x's` = `{1; 2; 3; 4}`. If the
   pair is dropped, the answer equals the three-point regression on the remaining pairs; if the
   vectors are compacted independently, the answer is a regression of `{1,2,4}` against
   `{1,2,3}` — a different, meaningless number. One call distinguishes the two readings, and the
   answer governs how every paired regression surface in Excel must be implemented.
2. **The near-collinear ladder.** `y = 2x + ε` with `ε` shrunk by factors of ten from `1e-3` down
   to `1e-15`, at fixed `x`. This traces the cancellation directly: a correct implementation
   tracks the true `s_e` proportionally all the way down; a `Syy − Sxy²/Sxx` implementation
   flattens out and then produces noise, and the point where it departs measures the arrangement.
3. **Exactly collinear data** at several scales, checking for exact 0, for a tiny positive value,
   and for `#NUM!` from a negative radicand.
4. **`STEYX(y,x)^2 * (n−2)` against `DEVSQ(y) * (1 − RSQ(y,x))`** — an exact identity among four
   Excel surfaces, needing no external oracle. Disagreement localizes to whichever surface
   computes its difference differently.
5. **`STEYX` against `LINEST`'s reported standard error of the estimate** on the same data — two
   Excel routes to one number.
6. **Constant `x`** (`Sxx = 0`) and **constant `y`**, one probe each.
7. **Large-magnitude data**: values around `1e160`, where `Sxy²` overflows although `Sxx` and
   `Syy` do not.
8. **`n = 2` and `n = 3`** on both sides of the documented `#DIV/0!` boundary, and mismatched
   lengths for the documented `#N/A`.
9. **A NIST StRD linear-regression dataset** (`Norris` for an easy case, `Filip` for a hard one)
   run through `STEYX` and compared against the certified residual standard deviation. This is
   the probe that connects the Handbook to the published accuracy literature rather than to its
   own reasoning.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| standard error of the estimate | `s_{y·x}` — the residual standard deviation of the fitted line |
| residual sum of squares | `Σ(yᵢ − ŷᵢ)²`; the bracketed quantity in the documented formula |
| centred sums | `Sxx`, `Syy`, `Sxy` — sums of squares and cross-products about the means |
| residual degrees of freedom | `n − 2`, the divisor; two parameters were estimated |
| paired exclusion | The unsettled question of what happens to a value's partner when it is ignored |
| `r² → 1` cancellation | The digit loss in `Syy − Sxy²/Sxx` when the fit is good |

## Sources

- Microsoft, "STEYX function" —
  <https://support.microsoft.com/en-us/office/steyx-function-6ce74b2c-449d-4a6e-b9ac-f9cef5ba48ab>
  (syntax; the standard-error equation; the direct-versus-reference coercion split, including
  that cells with the value zero are included while text, logicals and empty cells in a reference
  are ignored; `#N/A` for different numbers of data points; `#DIV/0!` for empty arguments or
  fewer than three data points; and that error values and untranslatable text cause errors).
  Retrieved for this page.
- N. J. Higham, *Accuracy and Stability of Numerical Algorithms* — §1.9 on computing the sample
  variance without a cancelling difference, and chapter 20 on least-squares problems and the
  residual norm from a QR factorization.
- C. L. Lawson and R. J. Hanson, *Solving Least Squares Problems*; Å. Björck, *Numerical Methods
  for Least Squares Problems*; G. H. Golub and C. F. Van Loan, *Matrix Computations* — the
  Householder-QR route to a residual sum of squares with no subtraction.
- T. F. Chan, G. H. Golub and R. J. LeVeque, "Algorithms for computing the sample variance:
  analysis and recommendations" — the one-dimensional form of the same argument.
- M. Welinder's documentation of Gnumeric's statistical functions, and the assessments of Excel's
  statistical procedures against the NIST Statistical Reference Datasets by R. Knüsel and by
  B. D. McCullough and B. Wilson — named as the standing literature on Excel's regression
  accuracy, not as evidence about any current build.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.STEYX.json` (arity, classification axes),
  `data/presence/FUNC.STEYX.json` (the shared `moment_stats_family` module) and
  `data/battery/FUNC.STEYX.json`.
