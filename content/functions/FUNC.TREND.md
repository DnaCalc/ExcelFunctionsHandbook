---
schema: efh.function-page/v1
function_id: FUNC.TREND
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MISC-0009
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
family: regression_forecast_family
role_in_family: >-
  The vectorized, multi-predictor member of the regression family: it fits the same least-squares
  model LINEST reports and then evaluates it, and it is the surface in this module that the
  Handbook holds an open-discrepancy record on.
---

# TREND

## What it computes

`TREND(known_y's, [known_x's], [new_x's], [const])` fits a linear model to the known data by the
**method of least squares** — Microsoft's page says so in those words — and returns the fitted
values of that model at the new `x` positions.

With one predictor the model is the familiar `y = m·x + b` that the documentation names. With
several predictors it is

    y = m₁x₁ + m₂x₂ + … + m_kx_k + b

and `TREND` returns `ŷ* = X* β̂` where `β̂` minimizes `‖y − Xβ‖₂` over the known data. In matrix
terms, with `X` the `n × (k+1)` design matrix whose first column is all ones (when `const` is
TRUE or omitted):

    β̂ = argmin ‖y − Xβ‖₂     ⟺     XᵀX β̂ = Xᵀy       (the normal equations)

The normal-equations characterization is *mathematically* correct and *numerically* the worst way
to obtain `β̂`; that distinction is the entire subject of the Numerical notes section, and it is
the subject of the evidence record attached to this page.

For the single-predictor case the solution is explicit:

    b_slope = Sxy / Sxx,     b_intercept = ȳ − b_slope·x̄

with `Sxx = Σ(x − x̄)²` and `Sxy = Σ(x − x̄)(y − ȳ)`. Two things follow that matter more than
they look:

1. **The centred form is a different computation from the uncentred one.** `Sxx` written as
   `Σ(x − x̄)²` and `Sxx` written as `Σx² − (Σx)²/n` are the same real number and wildly
   different floating-point numbers when the data has a large mean relative to its spread.
2. **`const = FALSE` removes the option to centre.** With the intercept forced to zero the model
   is `y = m·x` and the estimator is `Σxy / Σx²` — an uncentred quantity by necessity. The two
   settings of `const` are therefore not one algorithm with a flag; they are two.

`TREND` is a *dynamic array* surface: it returns one fitted value per row of `new_x's`, so its
result is generally an array rather than a scalar.

## Arguments

| Argument | Meaning | Default when omitted |
|---|---|---|
| `known_y's` | "The set of y-values you already know in the relationship y = mx + b". Required. | — |
| `known_x's` | "An optional set of x-values you may already know". | `{1, 2, 3, …}` the same size as `known_y's` |
| `new_x's` | "New x-values for which you want TREND to return corresponding y-values". | the same as `known_x's` |
| `const` | Logical: whether to force the constant `b` to zero. | TRUE — `b` is calculated normally |

The projection records an arity of one to four.

Documented details worth stating precisely, because each one is a place readers get surprised:

- **Omitting `known_x's` substitutes the index sequence** `{1,2,3,…}`. `TREND(A1:A12)` is a
  regression against time-as-row-number, and it is a real model, not a degenerate one.
- **Omitting `new_x's` re-uses `known_x's`**, which makes `TREND(y, x)` the vector of fitted
  values — the in-sample predictions — rather than a forecast.
- **`const = FALSE` sets `b = 0` and adjusts the `m` values so that `y = mx`.** It does not
  simply drop the intercept from a fit computed with one; the whole fit changes.
- **Several predictors are supported**, and when there is more than one, `known_y's` must be a
  vector.
- **Polynomial fitting** is documented as a use: regress against the same variable raised to
  different powers, supplying `x`, `x²`, `x³` as separate predictor columns. This is worth
  flagging as the numerically hardest thing `TREND` is documented to do — the Vandermonde design
  matrix it produces is the standard example of an ill-conditioned least-squares problem.
- **Array entry**: results that are arrays must be entered as array formulas with
  Ctrl+Shift+Enter unless the build supports dynamic arrays, in which case Enter suffices.

## Result and edge cases

Returns `Number` values, generally as an array shaped by `new_x's`.

- **Shape.** One fitted value per new observation. The orientation of the result follows the
  orientation of `new_x's`, which is one of the two most common sources of `#REF!` and `#VALUE!`
  from this function; the other is a design matrix whose predictor columns run the wrong way.
- **Rank deficiency.** Collinear predictors — including the exactly-duplicated column that a
  careless polynomial expansion produces — make `XᵀX` singular. What Excel does then is not
  documented on this page.
- **Fewer observations than parameters** is the same problem in a different dress and is equally
  undocumented.
- **Large offsets.** Data whose `x` values sit far from zero relative to their spread (times as
  serial dates, sensor readings around a large baseline) is the regime where the uncentred and
  centred arrangements of the same formula give visibly different answers. It is also the regime
  the evidence record attached to this page found most diagnostic.
- **Text, logicals and empty cells** in the data ranges follow the family's coercion policy. This
  surface's module carries the open upstream defect stream `BUG-FUNC-028`, whose title names an
  unswept conversion, text/date, array-lift and coercion gap; the Handbook cites it by name and
  does not restate its contents.
- The reference-engine battery beside this page refuses an inline array in the sole-argument
  form. That is the engine's own answer, and it says nothing about Excel.

## Errors

Microsoft's `TREND` page carries no error table. That is itself a finding: a function with a
documented rank-deficiency exposure, a documented orientation requirement, and a documented
optional-argument default chain publishes **no** documented error conditions at all.

What can be said honestly:

| Error | Basis |
|---|---|
| `#REF!`, `#VALUE!` | Observed shapes in the reference engine's own battery for malformed arguments; not documented, and not verified against Excel. |
| `#N/A` | Propagated from error values in the data under the universal coercion rule. |
| `#SPILL!` | The ordinary dynamic-array condition when the result cannot spill; see [The value universe](../model/01-value-universe.md). |

Nothing in this table is a documented Microsoft statement, and the Handbook has not verified any
of it against Excel.

## Relationships

- **`LINEST`** returns the *coefficients* (and regression statistics) that `TREND` uses; `TREND`
  is `LINEST` composed with evaluation. In the reference engine the two share an implementing
  module, and the evidence record attached to this page keeps them open together, identifying
  normal-equations staging as the shared question.
- **`FORECAST` / `FORECAST.LINEAR`** are the single-predictor, single-point special case. This
  relationship is not decorative: the record attached to this page found that every natural
  centred arrangement it raced reproduced `FORECAST`'s slope and never `TREND`'s, on the datasets
  where the two disagree. Two Excel surfaces that ought to compute the same slope apparently do
  not compute it the same way — and that is a finding about Excel's internals, arrived at by
  elimination.
- **`GROWTH`** is `TREND` on `ln y`: the exponential model `y = b·m^x`, fitted by least squares in
  the log domain. Same module, same machinery, different link.
- **`SLOPE`, `INTERCEPT`, `RSQ`, `STEYX`** are the scalar readouts of the same single-predictor
  fit; [STEYX](FUNC.STEYX.md) tabulates the correspondence.
- **Confused with**: `FORECAST.ETS` (exponential smoothing, an entirely different model),
  charting trendlines (which fit the same model but display coefficients rounded for the label),
  and `SLOPE`×`x` + `INTERCEPT` written out by hand, which is a *third* route to the same number
  and need not agree with either of the first two in the last bits.

## Numerical notes

Least squares is the best-studied problem in numerical linear algebra, and the whole literature
converges on one sentence: **do not form `XᵀX`.**

**Why.** The normal equations square the condition number: `κ₂(XᵀX) = κ₂(X)²`. A design matrix
with a modest condition number of `10⁶` — easily produced by a quadratic fit on data that does not
straddle zero — yields a normal-equations system with condition number `10¹²`, and in double
precision that consumes most of the available accuracy before the solve begins. A Vandermonde
matrix from a cubic or higher polynomial fit is far worse; this is why the NIST StRD `Filip`
dataset (a tenth-degree polynomial fit) is the standard executioner of regression
implementations.

**The alternatives.** A Householder QR factorization `X = QR` reduces the problem to the
triangular system `Rβ̂ = Qᵀy` and works with `κ₂(X)`, not its square; a singular value
decomposition additionally handles rank deficiency gracefully and yields the minimum-norm
solution. These are the recommendations of Lawson & Hanson, Björck, Golub & Van Loan, and
Higham's *Accuracy and Stability of Numerical Algorithms* chapter 20, and they are what LAPACK's
`gels`/`gelsd` do. Solving normal equations with a Gauss–Jordan inverse — forming `(XᵀX)⁻¹`
explicitly and multiplying — is the worst option available on both accuracy and stability
grounds, and it is the one textbooks use to state the model.

**Centring.** For a model with an intercept, subtracting the column means before the solve and
recovering `b = ȳ − Σ mⱼx̄ⱼ` afterwards is mathematically equivalent and numerically decisive: it
removes the offset that otherwise dominates every entry of `XᵀX`. With `const = FALSE` this route
is closed, because there is no intercept to absorb the shift — which is exactly why the two
`const` settings deserve separate treatment and separate testing.

**What the attached evidence record establishes about the reference engine and about Excel.**
`EV-MISC-0009` is an open-discrepancy record on `TREND`, and its content is unusually
informative for a page like this one:

- The **shipping** reference-engine kernel solves uncentred normal equations with a Gauss–Jordan
  inverse. On the record's live-witness corpus it additionally produces spurious `#NUM!` results
  on offset data, because a small epsilon guard trips on a near-singular `XᵀX`. That is the
  predicted failure mode of the predicted algorithm, observed.
- A **centred candidate** kernel — the arrangement promoted for `FORECAST`, generalized to the
  design matrix — does substantially better on the same witnesses. Its score is a *candidate*
  score, not the shipping surface's; the record's reader warning says so, and this page repeats
  it.
- The record's `ruled_out` list is the interesting part, because each entry is a conclusion about
  **Excel**, reached by elimination: any one-pass computational sum-of-squares formula is
  refuted, because it explodes on the huge-offset dataset where Excel does not — so **Excel
  centres**. Extended-precision-throughout arrangements are refuted as strictly worse. And every
  natural centred variant raced — forward and reverse accumulation, extended precision, centred
  QR by Householder, centred normal equations, fused-multiply-add dot products, symmetric versus
  asymmetric centring, one- versus two-pass — reproduces `FORECAST`'s slope and never `TREND`'s.
- The record's open question states the limit honestly: the exact operation graph producing
  `TREND`'s slope on the residual datasets is **under-determined** by the available witnesses,
  and the multivariate regime is entirely unprobed.

The figures behind all of that belong to the evidence layer rendered beside this page and are not
restated in this prose. The record also carries a warning this page must repeat in its own voice:
the catalogue's observation that the *prediction* step reproduces Excel given the coefficients is
a decomposition of the problem, not a pass rate, and **it must never be read as "TREND matches
Excel"**.

The accuracy of Excel's regression outputs is also the subject of a published literature — Morten
Welinder's work on Gnumeric's statistical functions, and the NIST StRD assessments by Knüsel and
by McCullough & Wilson. The Handbook names those as the right reading and does not assert from
them what any current Excel build does.

## What has not been checked

One Handbook evidence record lists this surface among its subjects: `EV-MISC-0009`, an open
discrepancy. What it establishes is that **`TREND` has been compared against live Excel on a
counted corpus and diverges** — the reference engine's shipping kernel and Excel do not agree
across those witnesses, and a better candidate exists but has not been promoted. That is a
genuine measurement, and its figures render mechanically beside this page.

What does not exist: **no Handbook vector suite for `TREND`**, so nothing here is independently
re-runnable by a reader; no Handbook re-verification of the upstream record (it carries
`handbook_reverified: false`); and the record's own corpus lives outside version control, so its
evidence locality is `local-only`. Nothing on this page is a statement that any implementation
agrees with Excel.

Unprobed by everything above: **the entire multivariate regime**. Every witness behind the
attached record is single-predictor. Excel's behaviour with several predictors, with collinear
predictors, with `const = FALSE`, and with the polynomial expansion Microsoft's own page
recommends is unmeasured by anybody in this record.

Inputs worth probing first, in the order the Handbook would run them:

1. **The multivariate case at all.** Two and three predictors, well conditioned, against a
   correctly rounded reference. This is the largest unexplored region of a documented surface in
   the regression family, and no measurement of any kind exists for it.
2. **`const = FALSE`.** A separate algorithm by the argument above, and unprobed. The
   through-the-origin fit is also the one case where centring is unavailable, so it isolates the
   uncentred path deliberately.
3. **The huge-offset single-predictor set** that the attached record found decisive, extended to
   several predictors — the probe most likely to reproduce the known signature in a new regime.
4. **`TREND` against `FORECAST.LINEAR`** at the same single new point, on data where the record
   found the two slopes to differ. Two Excel surfaces, one model, no external oracle required,
   and a disagreement is a self-inconsistency in Excel.
5. **`TREND` against `LINEST`'s coefficients evaluated by hand** (`SUMPRODUCT` of the reported
   `m` values with the new `x`, plus `b`) — a third Excel route to the same number, and the probe
   that would show whether `TREND` re-derives the fit or reuses `LINEST`'s.
6. **Rank-deficient designs**: a duplicated predictor column, and more parameters than
   observations. Undocumented, and the answer determines whether Excel does a pivoted or a
   minimum-norm solve.
7. **A NIST StRD regression dataset** — `Norris` as a sanity check, `Filip` as the executioner —
   compared against the certified coefficients. This connects the Handbook to the published
   accuracy literature rather than to its own reasoning.
8. **Orientation and shape**: row versus column `new_x's`, a `new_x's` block with a different
   number of predictor columns from `known_x's`, and the omitted-argument default chain, which no
   error table covers.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| design matrix | `X`, the observations-by-predictors matrix (plus an intercept column) |
| normal equations | `XᵀX β = Xᵀy` — mathematically the solution, numerically the trap |
| centring | Subtracting column means before the solve; unavailable when `const = FALSE` |
| condition squaring | `κ₂(XᵀX) = κ₂(X)²` — why the normal equations lose half the digits |
| candidate score | An evidence-layer figure for a kernel that is *not* the shipping surface |
| prediction path | Evaluating `a + Σ bⱼxⱼ` given coefficients — a decomposition, not a pass rate |

## Sources

- Microsoft, "TREND function" —
  <https://support.microsoft.com/en-us/office/trend-function-e2f135f0-8827-4096-9873-9a7cf7b51ef1>
  (syntax; the `y = mx + b` model and the statement that `TREND` fits a straight line using the
  method of least squares; the `{1,2,3,…}` default for `known_x's`; the `new_x's` default; the
  `const = FALSE` behaviour forcing `b = 0` with the `m` values adjusted; the requirement that
  `known_y's` be a vector when there is more than one predictor; polynomial fitting by regressing
  against powers of the same variable; and the array-entry rule). Retrieved for this page. The
  page documents no error conditions.
- Handbook evidence record `EV-MISC-0009` — the open discrepancy on `TREND`: the shipping
  uncentred-normal-equations kernel with its Gauss–Jordan inverse and spurious `#NUM!` guard, the
  centred candidate, the ruled-out arrangements (including the elimination establishing that
  Excel centres), the `FORECAST`-slope coincidence, and the reader warning that the prediction
  path's conditional result is not a statement that `TREND` matches Excel.
- N. J. Higham, *Accuracy and Stability of Numerical Algorithms*, chapter 20 (least squares);
  C. L. Lawson and R. J. Hanson, *Solving Least Squares Problems*; Å. Björck, *Numerical Methods
  for Least Squares Problems*; G. H. Golub and C. F. Van Loan, *Matrix Computations* — the QR and
  SVD alternatives to the normal equations, and the condition-squaring result.
- The NIST Statistical Reference Datasets for linear regression (`Norris`, `Longley`, `Filip`,
  the `Wampler` series) — the standard certified benchmarks.
- M. Welinder's documentation of Gnumeric's statistical functions, and the Excel accuracy
  assessments of R. Knüsel and of B. D. McCullough and B. Wilson — named as the standing
  literature, not as evidence about any current build.
- OxFunc defect stream
  `docs/bugs/streams/BUG-FUNC-028_unswept_conversion_text_date_array_lift_and_coercion_gap.md`,
  named by the presence projection for this surface. Cited by name only.
- Handbook [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.TREND.json`, `data/presence/FUNC.TREND.json` (the
  shared `regression_forecast_family` module) and `data/battery/FUNC.TREND.json`.
