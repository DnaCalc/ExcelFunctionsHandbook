---
schema: efh.function-page/v1
function_id: FUNC.INTERCEPT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MISC-0008
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
family: intercept_fn
role_in_family: >-
  The constant term of the ordinary least-squares line through a paired sample; SLOPE's
  companion, and the more ill-conditioned of the two.
---

# INTERCEPT

## What it computes

`INTERCEPT(known_y's, known_x's)` returns `b` in the ordinary least-squares line

    y = a·x + b

fitted to the paired sample `(x₁, y₁) … (xₙ, yₙ)`.

The least-squares problem is to minimise the sum of squared vertical residuals

    S(a, b) = Σᵢ ( yᵢ − a·xᵢ − b )²

Setting `∂S/∂a = ∂S/∂b = 0` gives the normal equations, whose solution is

    a = Σᵢ (xᵢ − x̄)(yᵢ − ȳ) / Σᵢ (xᵢ − x̄)²
    b = ȳ − a·x̄

with `x̄` and `ȳ` the arithmetic means. `INTERCEPT` returns `b`; [SLOPE](FUNC.SLOPE.md)
returns `a`.

Two structural facts follow directly from that pair of formulas and are worth stating,
because both are load-bearing for the rest of the page:

1. **The fitted line passes through the centroid `(x̄, ȳ)`.** The second formula *is* that
   statement rearranged. `b` is therefore not an independent quantity: it is `ȳ` shifted by
   the slope times the horizontal distance from the origin to the centroid.
2. **`b` is an extrapolation to `x = 0`.** When the `x` data sit far from zero — years like
   2024, temperatures in kelvin, prices in cents — the intercept is the value of the fitted
   line at a point that is nowhere near the data. That is a statement about the estimand,
   not about any implementation, and it is the root of the conditioning problem discussed
   under Numerical notes.

**Domain.** Defined for `n ≥ 2` paired points with `Σ(xᵢ − x̄)² > 0`, i.e. the `x` values
are not all equal. The vertical-line case has no least-squares line of the form `y = ax + b`
and the denominator above is exactly zero. **Range:** any real number.

The regression is asymmetric in its arguments: `INTERCEPT(y, x)` and `INTERCEPT(x, y)` fit
different lines, because the criterion minimises vertical, not perpendicular, distance.
The first argument is the response.

## Arguments

| Argument | Meaning | Admissible values |
|---|---|---|
| `known_y's` | The dependent values `y₁ … yₙ` | Array or reference; required |
| `known_x's` | The independent values `x₁ … xₙ` | Array or reference; required |

The reference engine declares an arity of exactly 2 — there is no optional argument and no
`const` switch, so unlike [LINEST](FUNC.LINEST.md) there is no way to ask for a
through-the-origin fit. A through-the-origin fit has no intercept to return, which is the
consistent reading.

The two arguments must contain the same number of values; pairing is positional. Both slots
are array/reference slots rather than scalar slots, so the shared range-scan policy applies
to values reached inside them — see [Coercion and lifting](../model/02-coercion-and-lifting.md).
Whether a value that is skipped in one argument causes the *corresponding* value in the other
to be skipped as well — pairwise deletion — is a real question for any regression surface and
is on the probe list below.

## Result and edge cases

Returns `Number`.

- **Two points.** With `n = 2` and distinct `x`, the fit is exact: the line through the two
  points, and the residual sum of squares is zero.
- **All `x` equal.** The denominator `Σ(xᵢ − x̄)²` is zero. The reference engine's own
  battery — OxFunc's answers, no Excel involved — returns `#DIV/0!` for the degenerate rows,
  which is what the algebra predicts.
- **A single point.** Degenerate for the same reason: one point has zero `x`-variance.
- **Text that looks numeric, logicals, blanks.** These are the general scan-policy questions
  and the Handbook has not pinned this family's policy. Note that the reference engine's
  battery gives `#DIV/0!` rather than `#VALUE!` for a directly-passed numeric string, which
  is consistent with the string being coerced and then failing the variance test — but that
  is an inference about the engine, not an observation of Excel.
- **Error values** in either argument propagate.
- **Perfectly collinear data** is not an edge case at all; it is the easy case.

## Errors

The error conditions for `INTERCEPT` are documented on Microsoft's `INTERCEPT` page.
Retrieval of that page was blocked for this curation pass, so this page does not transcribe
its error table. What can be said without it:

| Error | Condition | Basis |
|---|---|---|
| `#DIV/0!` | The `x` values have zero variance (all equal), or there are too few points | Follows from the defining formula; matches the reference engine's own battery |
| propagated | An error value in either argument | Shared coercion rule, chapter 02 |
| mismatched lengths | Undefined here | Microsoft's page documents an error for unequal counts; not restated from memory |

Anyone relying on the exact error code for the mismatched-length case should read Microsoft's
page directly. Nobody has checked any of these against Excel within the Handbook's record.

## Relationships

- **[SLOPE](FUNC.SLOPE.md)** — the other half of the same fit. The two are computed from the
  same three centred sums, and evidence record `EV-MISC-0008` names them together for exactly
  that reason.
- **[FORECAST.LINEAR](FUNC.FORECAST.LINEAR.md)** and the legacy
  **[FORECAST](FUNC.FORECAST.md)** — the fitted line evaluated at a chosen `x`. The identity
  `INTERCEPT(y, x) = FORECAST(0, y, x)` holds mathematically; whether the two surfaces return
  identical bits is a separate question that requires evidence, and the Handbook has none.
- **[LINEST](FUNC.LINEST.md)** — the general form. For a single predictor, `LINEST` returns
  the slope and the intercept together as a two-cell array, with the intercept in the
  *rightmost* cell. `INTERCEPT` is the scalar convenience wrapper.
- **[RSQ](FUNC.RSQ.md)**, **[PEARSON](FUNC.PEARSON.md)**, **[STEYX](FUNC.STEYX.md)** — the
  goodness-of-fit companions built from the same centred sums.
- **Confused with:** [TREND](FUNC.TREND.md), which returns fitted values rather than
  coefficients, and with the intercept of a *chart* trendline, which Excel computes on the
  chart's own plotted series and can therefore disagree with a worksheet `INTERCEPT` over a
  different range.

## Numerical notes

`INTERCEPT` is the arithmetically trivial member of a numerically delicate family. There are
two distinct hazards and they are usually confused with each other.

**1. Cancellation in the centred sums.** The textbook "computational" formulas

    Sxy = Σ xᵢyᵢ − n·x̄·ȳ        Sxx = Σ xᵢ² − n·x̄²

are one-pass and catastrophically unstable: when the data are far from the origin, both terms
are large and nearly equal, and the difference loses most of its significant digits. The
classical remedy is the two-pass centred form `Σ(xᵢ − x̄)(yᵢ − ȳ)`, or a numerically stable
updating formula. The reference analysis is Chan, Golub and LeVeque, *Algorithms for
Computing the Sample Variance: Analysis and Recommendations* (1983), and the error bounds are
worked in Higham, *Accuracy and Stability of Numerical Algorithms*, chapter 1. The relevant
data-dependent quantity is the coefficient of variation of the `x` sample: the naive formula
loses roughly `log₁₀(1 + x̄²/Sxx)` decimal digits.

**2. Leverage in the intercept itself.** Even with perfectly computed centred sums, `b` is
recovered as `ȳ − a·x̄`. If `x̄` is large, an error `δ` in the slope becomes an error `x̄·δ`
in the intercept, and the subtraction of two large nearly-equal quantities cancels again.
This is why the intercept of a regression on calendar years is a far worse-conditioned number
than the slope of the same regression, and why standard practice is to centre the predictor
before fitting and add the shift back afterwards. `INTERCEPT` offers no way to do that; the
caller must subtract a reference year from the `x` column themselves.

A careful implementation therefore: accumulates the centred sums in one stable pass or two
passes rather than the one-pass difference form; forms the slope first and the intercept from
the centroid identity; and — if it wants the last bits — accumulates the centred products in
extended precision or with a compensated (Kahan/Neumaier) sum, since the sums are the only
place error can enter. Beyond that there is nothing to get wrong: two divisions and a
multiply-subtract.

The published critique literature on Excel's regression surfaces is worth naming here even
though it targets [LINEST](FUNC.LINEST.md) rather than `INTERCEPT`: McCullough and Wilson's
series *On the accuracy of statistical procedures in Microsoft Excel* used the NIST
Statistical Reference Datasets (Longley, Filip, Wampler) precisely to expose the
normal-equations conditioning problem, and Morten Welinder's work on Gnumeric's statistical
functions documents the same class of defect from the reimplementer's side. Neither says
anything about `INTERCEPT` specifically, and neither is a Handbook measurement.

## What has not been checked

One evidence record names this surface. `EV-MISC-0008` is classed as a live verification and
lists `FUNC.INTERCEPT` as a subject with a per-surface count against a live Excel oracle; its
figures, corpus, build and scope render mechanically in the evidence panel beside this page
and are deliberately not transcribed into this prose. What the record itself is careful to
say is that its corpus is very small and that the source does not state whether the rows were
held out. A small corpus that fully matched is a real result and a narrow one.

No Handbook vector suite exists for `INTERCEPT`. There is no residual plate, no domain sweep,
and no characterisation of the coercion or pairing behaviour.

One provenance mismatch is worth recording as a finding rather than smoothing over: the
Handbook's presence projection at commit `473efa3` names `intercept_fn.rs` as the sole
implementing module for this surface, while the upstream sweep note cited by `EV-MISC-0008`
describes the scored code as the "paired_stats_common implementations". The two namings may
describe the same code through different layers, but the Handbook has not reconciled them and
does not assume they refer to the same thing.

Inputs I would probe first, and why:

1. **`x` data offset far from the origin** — the same `y` values regressed against
   `{1,2,3,4,5}` and against `{2001,2002,2003,2004,2005}`. Mathematically the intercept
   changes by a known amount; numerically the second case is the one that exposes a one-pass
   accumulation. This is the single most informative probe on the page.
2. **Zero `x`-variance** with `n = 2`, `n = 3` and `n = 1`, to separate "too few points" from
   "no spread" in the error mapping.
3. **Unequal argument lengths**, including the case where one argument is longer only by
   blanks, to distinguish a length check from a pairwise-deletion policy.
4. **Blanks, numeric text and logicals interleaved in one argument only** — the pairwise
   deletion question. If `x` and `y` are de-selected independently, the pairing silently
   shifts and every downstream number is wrong with no diagnostic.
5. **Near-collinear but not collinear data**, e.g. `y = 3x + 2` perturbed in the last bit,
   which is where the difference between a stable and a naive accumulation becomes visible in
   the returned value rather than in an error.
6. **`INTERCEPT(y, x)` against `FORECAST(0, y, x)` and against `LINEST(y, x)`'s second cell**
   on the same data, as a metamorphic triple. Three surfaces that must agree mathematically
   are a cheap way to find out whether they agree computationally.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| centred sums | `Σ(xᵢ − x̄)(yᵢ − ȳ)` and `Σ(xᵢ − x̄)²`, the stable form of the regression sums |
| centroid identity | `b = ȳ − a·x̄`; the fitted line passes through `(x̄, ȳ)` |
| leverage | The amplification of slope error into intercept error by the factor `x̄` |
| pairwise deletion | Dropping the whole `(x, y)` pair when either element is unusable |
| one-pass form | `Σxᵢyᵢ − n·x̄·ȳ`; algebraically equal to the centred sum, numerically much worse |

## Sources

- Microsoft, "INTERCEPT function" —
  <https://support.microsoft.com/en-us/office/intercept-function-2a9b74e2-9d47-4772-b663-3bca70bf63ef>
  (signature and documented error conditions). Retrieval was blocked for this pass; the error
  table above deliberately does not restate what could not be read.
- Chan, Golub and LeVeque, *Algorithms for Computing the Sample Variance: Analysis and
  Recommendations*, The American Statistician 37 (1983) — the stability analysis of the
  one-pass versus centred forms.
- Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd ed., chapter 1 — error bounds
  for summation and for the sample-variance formulas.
- McCullough and Wilson, *On the accuracy of statistical procedures in Microsoft Excel*
  (Computational Statistics & Data Analysis, 1999 and successors) — the NIST StRD critique of
  Excel's regression lane. Named as literature about the family, not as evidence about this
  surface.
- Morten Welinder's work on Gnumeric's statistical functions — the reimplementer's record of
  Excel statistical accuracy problems.
- Handbook evidence record `EV-MISC-0008` (subjects `FUNC.INTERCEPT`, `FUNC.SLOPE`).
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [The value universe](../model/01-value-universe.md).
- Handbook projections `data/functions/FUNC.INTERCEPT.json` (arity 2, `xlfIntercept`,
  `ValuesOnlyPreAdapter`) and `data/presence/FUNC.INTERCEPT.json` (implementing module
  `intercept_fn.rs`).
