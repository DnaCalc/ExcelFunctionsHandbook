---
schema: efh.function-page/v1
function_id: FUNC.LOGEST
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MISC-0010
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - The returned array
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
  The exponential member: LINEST run on the logarithm of the response and reported
  multiplicatively; every statistic it returns describes the log-scale fit.
---

# LOGEST

## What it computes

`LOGEST(known_y's, [known_x's], [const], [stats])` fits the **exponential** model

    y = b · m₁^x₁ · m₂^x₂ · … · m_k^x_k

and returns the bases `m₁ … m_k` together with the multiplier `b`.

The fit is not done in that form. Taking logarithms turns the model linear:

    ln y = ln b + x₁·ln m₁ + … + x_k·ln m_k

so `LOGEST` runs an ordinary least-squares fit of `ln y` on the predictors — exactly the
computation described on the [LINEST](FUNC.LINEST.md) page — and exponentiates the
coefficients back:

    m_j = exp( β̂_j ),    b = exp( β̂₀ )

**This is the whole function, and it is also the whole caveat.** The quantity minimised is

    Σᵢ ( ln yᵢ − ln ŷᵢ )²

not `Σ(yᵢ − ŷᵢ)²`. Least squares in log space weights **relative** error equally at every
scale; least squares in the original space weights **absolute** error equally. On data
spanning several orders of magnitude these two criteria give visibly different curves, and
neither is "the" exponential fit — they answer different questions. If the noise in the data
is multiplicative (a constant percentage), the log-space fit is the maximum-likelihood one and
`LOGEST` is right. If the noise is additive, `LOGEST` systematically over-weights the small
observations and a nonlinear least-squares fit is the correct tool. Excel has no worksheet
function for that.

**Domain.** Every `y` value must be strictly positive — `ln y` is undefined otherwise — and
the usual least-squares requirements apply to the design matrix: at least as many observations
as parameters, full column rank. **Range:** the bases `m_j` and the multiplier `b` are
strictly positive by construction, since they are exponentials. `LOGEST` can never return a
negative or zero coefficient, which is a real modelling restriction: it cannot fit a decaying
process to zero and cannot fit data that changes sign.

Interpretation of the bases: `m_j` is the multiplicative factor per unit increase in `x_j`.
`m = 1.05` means five percent growth per unit; `m = 1` means no effect; `m < 1` means decay.

## The returned array

Same shape and same reversal as [LINEST](FUNC.LINEST.md):

    { m_k , m_{k−1} , … , m₁ , b }

with the multiplier last and the predictor bases in reverse column order. With `stats = TRUE`
the five-row block is returned, laid out identically to `LINEST`'s.

**The statistics describe the log fit, not the exponential fit.** The `r²`, the standard
errors, the `F` statistic and both sums of squares are computed on `ln y`. Consequences that
matter in practice:

- `se(m_j)` in row 2 is the standard error of `β̂_j = ln m_j`, on the log scale. It is **not**
  the standard error of `m_j`. Converting requires the delta method: `se(m_j) ≈ m_j · se(β̂_j)`
  to first order, and that approximation degrades as `se(β̂_j)` grows.
- An `r²` from `LOGEST` and an `r²` from `LINEST` on the same raw data are not comparable. A
  higher `r²` from `LOGEST` does not mean the exponential model fits better in the original
  units; it means it fits better *in log units*, which is a different comparison.
- Because `E[exp(Z)] > exp(E[Z])` for a non-degenerate `Z` (Jensen's inequality), the fitted
  curve `ŷ = b·∏m_j^{x_j}` is a fit to the **conditional median** of `y`, not its conditional
  mean, when the log-scale residuals are symmetric. Predictions from `LOGEST` are therefore
  systematically low as estimates of the mean. The classical correction multiplies by
  `exp(σ̂²/2)`; whether to apply it is a modelling decision, and no Excel function applies it
  for you.

`EV-MISC-0010` records the corresponding structural observation for this surface: with `const`
FALSE the returned row still carries a trailing base cell set to one rather than being
shortened, and multivariate coefficients come back in reverse predictor order. That record is
explicit that these observations are structural and uncounted.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `known_y's` | The response values; **all must be positive** | Required |
| `known_x's` | The predictor columns or rows | `{1, 2, …, n}` |
| `const` | `TRUE` fits `b`; `FALSE` forces `b = 1` | `TRUE` |
| `stats` | `TRUE` returns the five-row block | `FALSE` |

The reference engine declares an arity of 1 to 4 and classifies the surface as
`RefsVisibleInAdapter`.

**`const = FALSE` forces `b = 1`, not `b = 0`.** In the log-linear model the constant is
`ln b`, so suppressing the constant means `ln b = 0`, i.e. `b = 1`. The curve is then forced
through `(0, 1)` rather than through the origin. This is the exponential analogue of
`LINEST`'s through-the-origin fit and it catches people who reason by analogy with `LINEST`
without redoing the algebra.

The orientation rules — which axis carries observations and which carries predictors — are
identical to `LINEST`'s and are decided by the shapes of the two arguments, not by a flag.

## Result and edge cases

Returns an `Array`; a spilled dynamic array in modern Excel, historically a Ctrl+Shift+Enter
formula.

- **A zero or negative `y`** has no logarithm. Microsoft's page documents `#NUM!` for
  non-positive `known_y's`, and the reference engine's own battery — OxFunc's answers, no
  Excel involved — returns `#NUM!` for its zero and negative rows. This is the constraint that
  most often stops a real workbook: one zero in a column of counts kills the whole fit, and
  the usual workaround (adding a small constant to every `y`) changes the estimates in a way
  that depends on the constant chosen.
- **`y` values very close to 1** are the interesting numerical case; see Numerical notes.
- **`n = p`** — exact fit on the log scale, zero residual sum of squares, `df = 0`, and the
  statistics cells that need `df > 0` become `#N/A`.
- **A two-dimensional inline array literal in `known_y's`.** The reference engine's battery
  returns `#REF!` for that row. Microsoft documents `known_y's` as an array or range, so this
  is a **divergence between the documentation and the reference engine's current behaviour on
  the direct-call path**, recorded here as a finding. It may be an artefact of the battery's
  direct-call construction; nobody has checked it against Excel.
- **Text, blanks and logicals in the data.** The presence projection names the upstream defect
  stream `BUG-FUNC-028` on text/date/array-lift conversion gaps against this module, so the
  coercion behaviour of this family is explicitly unsettled upstream.
- **Error values** propagate.

## Errors

| Error | Condition | Basis |
|---|---|---|
| `#NUM!` | Any `known_y's` value is zero or negative | Documented by Microsoft on the `LOGEST` page; also required by the model |
| `#REF!` | `known_y's` and `known_x's` have incompatible shapes | Documented layout rule shared with `LINEST` |
| `#VALUE!` | An argument cannot be converted | Shared coercion rule, chapter 02 |
| `#N/A` | Unused cells of statistics rows 3–5 | Documented layout |
| propagated | An error value in the data | Shared coercion rule, chapter 02 |

Retrieval of Microsoft's `LOGEST` page was blocked for this curation pass; the rows above are
stated as documented behaviour with the source named, or derived from the model. Nobody has
checked any of it against Excel within the Handbook's record.

## Relationships

- **[LINEST](FUNC.LINEST.md)** — the linear engine `LOGEST` is built on. Mathematically,
  `LOGEST(y, x, …)` is `EXP(LINEST(LN(y), x, …))` applied to the coefficient row. Whether the
  two agree in their last bits is a question requiring evidence; the composition route and the
  direct route can differ by where the logarithm is taken and how the statistics are
  accumulated.
- **[GROWTH](FUNC.GROWTH.md)** — returns fitted `y` values from the same fit, as
  [TREND](FUNC.TREND.md) does for `LINEST`. `GROWTH` is the function to use for prediction;
  `LOGEST` for the parameters.
- **[TREND](FUNC.TREND.md)** — the linear-scale predictor, for the comparison a reader should
  make before choosing an exponential model.
- **[RRI](FUNC.RRI.md)** and **[PDURATION](FUNC.PDURATION.md)** — the financial two-point
  exponential-growth surfaces. `RRI` is exactly `LOGEST` on two observations with the
  constant fitted, expressed as a rate; it uses no least squares because two points determine
  the curve.
- **Confused with:** the exponential chart trendline (fitted on the plotted series, displayed
  with its own rounding), and with `LOGEST`'s own `r²` as a measure of exponential fit
  quality — see above.

## Numerical notes

Everything in [LINEST](FUNC.LINEST.md)'s numerical notes applies to the inner fit —
normal-equations conditioning, the `κ₂(XᵀX) = κ₂(X)²` squaring, centring and scaling, the NIST
StRD record. Three hazards are specific to `LOGEST`.

**1. Cancellation in `ln y` near 1.** For `y` close to 1, `ln y` is a small difference computed
from a value near unity, and the relative error in the logarithm blows up like `1/|ln y|`. This
is the classical reason `log1p` exists: `ln(1 + t)` computed as `log(1 + t)` loses digits once
`|t| ≲ 2⁻²⁶`, because the addition `1 + t` discards the low bits of `t` before the logarithm
ever runs. Data that hovers near 1 — index series normalised to a base year, survival fractions
near unity — is the case where `LOGEST` can lose most of its precision before the regression
starts. A careful implementation cannot fix this from the outside: the `y` values arrive as
doubles and the information is already gone. What it *can* do is compute the logarithm
correctly rounded, so the loss is exactly the unavoidable one.

**2. Exponentiation amplifies absolute error into relative error.** The returned base is
`m = exp(β̂)`, so

    δm / m  ≈  δβ̂

An absolute error of `10⁻¹⁰` in the fitted slope becomes a *relative* error of `10⁻¹⁰` in the
base — acceptable — but the same error compounds when the base is raised to a large `x` in a
prediction: `m^x` carries relative error `x·δβ̂`. Extrapolating an exponential fit fifty steps
multiplies the coefficient error by fifty before you even consider the model risk. This is
inherent to the parameterisation, not a defect.

**3. The statistics block inherits the log-scale accumulation.** `SSE = Σ(ln yᵢ − ln ŷᵢ)²`
computed as `SST − SSR` cancels when the fit is good, and a good exponential fit is exactly
when people look at `r²`. Accumulating residuals directly is unconditionally better here, as
in `LINEST`.

On the mathematical library side, the accuracy of `LOGEST` is bounded by the accuracy of the
platform's `log` and `exp`. The correctly-rounded-or-nearly references are fdlibm's `__ieee754_log`
and `__ieee754_exp` (Sun, and their descendants in glibc and musl), Cody and Waite's
*Software Manual for the Elementary Functions* for the classic argument-reduction schemes, and
Muller's *Elementary Functions: Algorithms and Implementation* for the modern treatment. A
half-ulp `log` and a half-ulp `exp` are achievable; the Handbook does not assert what Excel
uses.

## What has not been checked

One evidence record names this surface. `EV-MISC-0010` is classed as an **open discrepancy**
with `FUNC.LINEST` and `FUNC.LOGEST` as its subjects, and its own status is blunt about
`LOGEST` in particular: the upstream lane survey states that **no `GROWTH`/`LOGEST` live
witness corpus exists at all**. The record carries an explicit reader warning that being named
in an open divergence row is not the same as having been measured, and nothing in it scores
`LOGEST` against Excel. The record's figures and scope render in the evidence panel beside
this page; this prose does not transcribe them.

So: nobody has measured `LOGEST` against Excel. No Handbook vector suite exists, no residual
plate, and no characterisation of the coercion, orientation or rank behaviour.

One divergence found during this curation pass, recorded as a finding: Microsoft documents
`known_y's` as an array or range, while the reference engine's own battery returns `#REF!` for
a two-dimensional inline array literal in that slot. Scope and cause unestablished.

Inputs I would probe first, and why:

1. **`y` values clustered near 1** — for example `{1.0000001, 1.0000002, 1.0000003}` against
   `{1, 2, 3}`. This is the cancellation probe, and it is the one place `LOGEST` can differ
   from a careful implementation by far more than a few ulps.
2. **`LOGEST(y, x)` against `EXP(LINEST(LN(y), x))`** on the same data, bit for bit. If they
   disagree, the difference localises exactly where the logarithm and the accumulation happen.
   This is the highest-information single probe on the page.
3. **A zero and a negative `y`**, separately, confirming `#NUM!` and confirming that one bad
   value poisons the whole fit rather than being skipped.
4. **`const = FALSE`**, checking that the suppressed multiplier comes back as one rather than
   zero and that the row is not shortened — the structural point `EV-MISC-0010` raises.
5. **`stats = TRUE` with two or three predictors**, verifying the internal identities
   (`r² = SSR/(SSR+SSE)`, `se(y) = √(SSE/df)`, `df = n − p`) hold in the returned bits, and
   confirming the statistics are the log-scale ones by recomputing them from `LN(y)` by hand.
6. **The NIST StRD regression datasets transformed to exponential form** — the same instrument
   `LINEST` deserves, applied through the log transform, which additionally exercises the
   `log`/`exp` pair.
7. **Data spanning many orders of magnitude**, comparing the `LOGEST` curve with a nonlinear
   least-squares fit computed elsewhere, to make the relative-versus-absolute criterion
   visible rather than merely stated.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| log-linear fit | Ordinary least squares applied to `ln y`; the computation `LOGEST` performs |
| base `m_j` | `exp(β̂_j)`; the multiplicative factor per unit of predictor `j` |
| multiplier `b` | `exp(β̂₀)`; the fitted value at all-zero predictors, forced to 1 when `const` is FALSE |
| relative-error criterion | The consequence of fitting in log space: percentage errors weighted equally |
| delta method | First-order conversion of a log-scale standard error to the original scale |
| retransformation bias | `exp` of a mean is not the mean of `exp`; `LOGEST` fits the conditional median |

## Sources

- Microsoft, "LOGEST function" —
  <https://support.microsoft.com/en-us/office/logest-function-f27462d8-3657-4030-866b-a272c1d18b4b>
  (signature, the exponential model, the returned-array layout, and the documented error
  conditions). Retrieval was blocked for this pass; the documented rows are stated as
  documented behaviour with the source named.
- Handbook [LINEST](FUNC.LINEST.md) — the least-squares engine and its conditioning analysis,
  including the Golub and Van Loan, Björck, Higham and NIST StRD references that apply here
  unchanged.
- Cody and Waite, *Software Manual for the Elementary Functions* — argument reduction for
  `log` and `exp`.
- Muller, *Elementary Functions: Algorithms and Implementation*, 3rd ed. — the modern
  treatment, including `log1p` and why it exists.
- fdlibm (`__ieee754_log`, `__ieee754_exp`) and its descendants in glibc and musl — the
  reference implementations against which elementary-function accuracy is usually judged.
- McCullough and Wilson, *On the accuracy of statistical procedures in Microsoft Excel*; and
  Morten Welinder's work on Gnumeric's statistical functions — published literature on the
  family, not Handbook evidence about this surface.
- Handbook evidence record `EV-MISC-0010` (subjects `FUNC.LINEST`, `FUNC.LOGEST`; class
  open-discrepancy; carries a reader warning against per-surface attribution).
- Handbook projections `data/functions/FUNC.LOGEST.json` (arity 1–4, `xlfLogest`,
  `RefsVisibleInAdapter`) and `data/presence/FUNC.LOGEST.json` (module
  `regression_forecast_family.rs`, shared with `FORECAST`, `FORECAST.LINEAR`, `GROWTH`,
  `LINEST`, `TREND`; upstream defect stream `BUG-FUNC-028` named on the module).
