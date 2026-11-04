---
schema: efh.function-page/v1
function_id: FUNC.LINEST
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
  - Notes
  - Page vocabulary
  - Sources
family: regression_forecast_family
role_in_family: >-
  The family's general engine: multiple linear least squares with an optional regression
  statistics block; every other member is a special case or a transform of it.
---

# LINEST

## What it computes

`LINEST(known_y's, [known_x's], [const], [stats])` fits the multiple linear model

    y = m₁x₁ + m₂x₂ + … + m_k x_k + b

to the data by **ordinary least squares**, and returns the fitted coefficients — optionally
together with a block of regression statistics.

In matrix form, with `y` the `n`-vector of responses and `X` the `n × p` design matrix (its
columns the predictors, plus a column of ones when `const` is TRUE), least squares chooses

    β̂ = argmin_β ‖ y − Xβ ‖₂²

whose stationarity condition is the **normal equations**

    XᵀX β̂ = Xᵀy

with the unique solution `β̂ = (XᵀX)⁻¹Xᵀy` whenever `X` has full column rank. When `X` is rank
deficient — two identical predictors, or fewer observations than parameters — the minimiser is
not unique and the problem as posed has no answer.

The residual vector `e = y − Xβ̂` is orthogonal to the column space of `X`; that orthogonality
is the geometric content of least squares and the source of the sums-of-squares decomposition
reported in the statistics block:

    SST = SSR + SSE,   SST = Σ(yᵢ − ȳ)²,  SSR = Σ(ŷᵢ − ȳ)²,  SSE = Σ(yᵢ − ŷᵢ)²

(with `SST = Σyᵢ²` when `const` is FALSE, because the model is then not required to reproduce
the mean).

**Domain.** Needs `n ≥ p` observations, `X` of full column rank, and — for the statistics
block — `n > p` so the residual degrees of freedom `df = n − p` are positive. **Range:** the
coefficients are real numbers of unbounded magnitude.

## The returned array

This is where most `LINEST` confusion lives, and it has two distinct parts.

**Coefficient order is reversed.** With `k` predictors the first row is

    { m_k , m_{k−1} , … , m₂ , m₁ , b }

The slope for the **last** predictor column comes first, and the constant is last. A reader
who lays the predictors out left to right and reads the result left to right gets them
backwards. For simple regression (`k = 1`) the row is `{m, b}` — slope then intercept — which
is easy to remember and gives no warning about the general rule.

**`stats = TRUE` adds four rows.** The full block is a `5 × (k+1)` array:

| Row | Leftmost `k` columns | Rightmost column |
|---|---|---|
| 1 | `m_k … m₁` — the coefficients | `b` — the constant |
| 2 | `se(m_k) … se(m₁)` — coefficient standard errors | `se(b)` |
| 3 | `r²` — coefficient of determination | `se(y)` — standard error of the estimate |
| 4 | `F` — the F statistic | `df` — residual degrees of freedom |
| 5 | `SSR` — regression sum of squares | `SSE` — residual sum of squares |

Rows 3, 4 and 5 have only two meaningful entries each; the remaining cells of those rows are
documented as `#N/A`. The layout is documented by Microsoft on the `LINEST` page.

Note the arithmetic relations that must hold internally, and which make excellent metamorphic
probes: `r² = SSR/(SSR+SSE)`, `se(y) = √(SSE/df)`, `F = (SSR/(p−1))/(SSE/df)` when `const` is
TRUE, and `df = n − p`.

Evidence record `EV-MISC-0010` records one structural comparison against Excel that belongs
here rather than in a statistics table: with `const` FALSE the returned row still carries a
trailing constant cell set to zero rather than being shortened by one column, and multivariate
coefficients come back in reverse predictor order. That record is explicit that these
observations are structural and uncounted.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `known_y's` | The response values. A single column, a single row, or — with multiple predictors — a vector whose orientation determines how `known_x's` is read. Required | — |
| `known_x's` | The predictor columns (or rows). Optional | `{1, 2, …, n}` |
| `const` | `TRUE` fits the constant `b`; `FALSE` forces `b = 0`. Optional | `TRUE` |
| `stats` | `TRUE` returns the five-row block; `FALSE` returns the coefficient row only. Optional | `FALSE` |

The reference engine declares an arity of 1 to 4 and classifies the surface as
`RefsVisibleInAdapter` — it sees live references rather than only resolved values, which is
what the orientation logic needs.

**The omitted `known_x's` default is a real behaviour, not a convenience.** Leaving it out
regresses `y` against the sequence `1, 2, …, n`, which is a time index. That is often what a
user wants and is occasionally a silent disaster when the observations are not equally spaced.

**Orientation matters.** `known_y's` and `known_x's` must have the same shape when there is
one predictor. With several predictors, each predictor occupies one column if `known_y's` is a
column, and one row if `known_y's` is a row. There is no argument that says which; the shapes
decide.

**`const = FALSE` changes the statistics, not just the coefficient.** Forcing the line through
the origin changes the definition of `SST` and of `df`, so `r²` computed under `const = FALSE`
is not comparable with the ordinary one and can behave in ways that surprise — it is not
bounded the same way. This is a genuine and much-litigated statistical point, not an
implementation quirk.

## Result and edge cases

Returns an `Array` — a spilled dynamic array in modern Excel, and historically a
Ctrl+Shift+Enter formula whose selection size decided how much of the block you saw. Selecting
too small a region truncated the block silently, which is the reason so many workbooks contain
a `LINEST` that reports only its first cell.

- **Exactly determined system** (`n = p`) — the fit is exact, `SSE = 0`, and `df = 0`. The
  statistics block then divides by zero for `se(y)` and `F`; the documented result is `#N/A`
  in those cells.
- **Rank-deficient `X`** — duplicated or linearly dependent predictor columns. The
  least-squares solution is not unique. Microsoft's page documents that `LINEST` may drop
  collinear columns and return zero coefficients for them; the Handbook has not verified what
  the rank tolerance is, and the tolerance is the whole question, because "collinear" in
  floating point is a threshold decision rather than a fact.
- **A single observation, or all-equal predictors** — degenerate; the reference engine's own
  battery (OxFunc's answers, no Excel involved) returns `#NUM!` for its single-value rows.
- **A two-dimensional inline array literal in `known_y's`.** The reference engine's battery
  returns `#REF!` for that row. Microsoft documents `known_y's` as an array or range, so this
  is a **divergence between the documentation and the reference engine's current behaviour on
  the direct-call path**, and it is recorded here as a finding rather than resolved. It may be
  an artefact of how the battery constructs a direct call rather than a worksheet semantic;
  either way nobody has checked it against Excel.
- **Text, blanks and logicals in the data.** The presence projection names the upstream defect
  stream `BUG-FUNC-028` on text/date/array-lift conversion gaps against this module, so the
  coercion behaviour of this family is explicitly unsettled upstream. The Handbook states no
  policy here.
- **Error values** propagate.

## Errors

| Error | Condition | Basis |
|---|---|---|
| `#REF!` | `known_y's` and `known_x's` have incompatible shapes | Documented by Microsoft on the `LINEST` page |
| `#VALUE!` | An argument cannot be converted, or `const`/`stats` is not interpretable as a logical | Shared coercion rule, chapter 02 |
| `#N/A` | The unused cells of statistics rows 3–5; also the statistics cells that require `df > 0` when `df = 0` | Documented layout |
| propagated | An error value in the data | Shared coercion rule, chapter 02 |

Retrieval of Microsoft's `LINEST` page was blocked for this curation pass. The rows above are
stated as documented behaviour with the source named, and the reference engine's own battery
outcomes are labelled as such where they are cited. Nobody has checked any of this against
Excel within the Handbook's record.

## Relationships

- **[TREND](FUNC.TREND.md)** — the same fit, returning fitted or predicted `y` values instead
  of coefficients. `TREND(y, x, x_new)` equals `x_new` premultiplied by `LINEST`'s
  coefficients; the two must agree mathematically. They share an implementing module in the
  reference engine.
- **[LOGEST](FUNC.LOGEST.md)** — `LINEST` applied to `ln y`, reported multiplicatively. See
  that page for why its statistics describe the log-scale fit.
- **`GROWTH`** — `LOGEST` as `TREND` is to `LINEST`.
- **[SLOPE](FUNC.SLOPE.md)** and **[INTERCEPT](FUNC.INTERCEPT.md)** — the scalar single-predictor
  special cases. `LINEST(y, x)` returns exactly those two numbers, in that order.
- **[RSQ](FUNC.RSQ.md)**, **[STEYX](FUNC.STEYX.md)**, **[PEARSON](FUNC.PEARSON.md)**,
  **[FORECAST.LINEAR](FUNC.FORECAST.LINEAR.md)** — each reproduces one cell of the `LINEST`
  block for the single-predictor case. Whether the dedicated surfaces and the `LINEST` cell
  return identical bits is exactly the kind of question a metamorphic suite exists to answer,
  and the Handbook has no such suite.
- **The Analysis ToolPak Regression tool** — a separate implementation of the same statistics,
  historically a separate code path. Comparing the two on the same data is a classic
  self-check.
- **Confused with:** chart trendlines, which fit on the plotted series and use their own
  display rounding; and with `LINEST` polynomial fits, which are done by passing
  `x, x², x³, …` as separate predictor columns — the construction that makes the conditioning
  problem below unavoidable.

## Numerical notes

`LINEST` is the function on which the accuracy of a spreadsheet's statistical lane has
historically been judged, and the reason is conditioning.

**The normal equations square the condition number.** Solving `XᵀXβ = Xᵀy` by forming `XᵀX`
and factorising it costs

    κ₂(XᵀX) = κ₂(X)²

so a design matrix with condition number `10⁸` — entirely ordinary for a cubic polynomial fit
on unscaled data — produces a Gram matrix with condition number `10¹⁶`, which is the
reciprocal of double precision. At that point the computed coefficients have no correct
digits, and the failure is silent: the routine returns numbers, not an error. This is the
single most important fact on this page.

**The stable alternatives.** The standard remedy, described in every numerical linear algebra
text, is to avoid forming `XᵀX` at all:

- **Householder QR** on `X` directly: `X = QR`, solve `Rβ = Qᵀy` by back substitution. The
  achieved accuracy scales with `κ₂(X)`, not its square. This is the default in LAPACK's
  `dgels` and in essentially every serious statistics package.
- **Modified Gram–Schmidt** with reorthogonalisation, cheaper and nearly as good.
- **The singular value decomposition**, `X = UΣVᵀ`, which additionally exposes rank deficiency
  as small singular values and lets the caller truncate them deliberately. The SVD is the
  right tool when collinearity is expected rather than exceptional.

The canonical references are Golub and Van Loan, *Matrix Computations*, chapter 5; Björck,
*Numerical Methods for Least Squares Problems*; Lawson and Hanson, *Solving Least Squares
Problems*; and Higham, *Accuracy and Stability of Numerical Algorithms*, chapters 19–20 for
the error analysis. Numerical Recipes chapter 15 covers the applied side including the SVD
route.

**Centring and scaling.** Even with a stable factorisation, an unscaled design matrix wastes
precision. Subtracting the column means (which `const = TRUE` does implicitly for the constant
term, but not for the predictors) and scaling columns to comparable norms can reduce `κ₂(X)`
by many orders of magnitude at no statistical cost, because the fit is equivariant under those
transformations. A polynomial fit on years `1990…2020` is hopeless; the same fit on
`year − 2005` is easy. `LINEST` gives the caller no way to request this, so the caller must do
it in the worksheet.

**The published critique record.** The NIST Statistical Reference Datasets (StRD) include
linear regression problems of graded difficulty — Norris (easy), Longley (the classic
ill-conditioned economic dataset from Longley 1967), Wampler, and Filip, a tenth-degree
polynomial fit that is deliberately near the edge of double precision. McCullough and Wilson's
sequence of papers *On the accuracy of statistical procedures in Microsoft Excel* (1999, and
follow-ups covering later versions) reported `LINEST` failing to return any correct digits on
Filip in the versions they tested, and tracked what changed across releases. Morten Welinder's
work on Gnumeric's statistical functions covers the same territory from the reimplementer's
side. The Handbook names these as the published literature on the family; it does not assert
what Excel does internally in any build, and it has not re-run the StRD datasets itself.

**A note on the statistics block.** `SSE` computed as `SST − SSR` is a cancellation waiting to
happen when `r²` is close to 1 — precisely the case where users care. Accumulating the
residuals directly, `Σ(yᵢ − ŷᵢ)²`, costs one extra pass and is unconditionally better. The
same applies to `r²` itself: computing it as `SSR/SST` when `SSE` is tiny loses the digits
that distinguish `0.999999` from `0.9999999`.

## What has not been checked

One evidence record names this surface. `EV-MISC-0010` is classed as an **open discrepancy**
and lists `FUNC.LINEST` and `FUNC.LOGEST` as its subjects. Its own status is the important
part and the Handbook restates it without softening: no `LINEST` live-witness corpus exists,
the multivariate regime that would separate a QR factorisation from the normal equations is
entirely unprobed, and the slope-bias figure quoted in the upstream catalogue row was read off
[TREND](FUNC.TREND.md) rather than measured on `LINEST`. The record carries an explicit reader
warning that being named in an open divergence row is not the same as having been measured.
The figures and their scope render in the evidence panel beside this page; this prose does not
transcribe them.

So: **nobody has measured `LINEST` against Excel.** No Handbook vector suite exists. There is
no residual plate, no StRD run, and no characterisation of the rank tolerance, the coercion
policy or the orientation rules.

Two divergences found during this curation pass, recorded as findings:

1. **Inline array literal rejected.** Microsoft documents `known_y's` as an array or range;
   the reference engine's own battery returns `#REF!` for a two-dimensional inline array
   literal. Scope and cause unestablished.
2. **`const = FALSE` result shape.** `EV-MISC-0010` records that Excel returns a trailing
   constant cell of zero rather than a shortened row — a structural observation the record
   itself marks as uncounted, and one that a naive implementation returning a `k`-wide row
   would get wrong.

Inputs I would probe first, and why:

1. **The NIST StRD linear regression suite** — Norris, Longley, Wampler and Filip — with the
   certified values. This is the standard instrument for exactly this function, the certified
   values are published, and running it is a day's work that would settle more about `LINEST`
   than everything else on this list combined. Filip in particular separates a QR
   factorisation from the normal equations in a single test.
2. **Two identical predictor columns**, and two columns differing in the last bit, to find the
   rank tolerance and to see whether collinear columns are dropped, zeroed, or silently
   solved.
3. **`n = p` exactly**, checking that `df` is zero and which statistics cells become `#N/A`
   rather than infinity.
4. **`const = FALSE` on data with a large `ȳ`**, comparing `r²` and `SST` against the
   hand-computed uncentred definitions. This is where implementations most often disagree with
   each other.
5. **Three predictors, `stats = TRUE`**, verifying the four internal identities
   (`r² = SSR/(SSR+SSE)`, `se(y) = √(SSE/df)`, the `F` ratio, `df = n − p`) hold in the
   returned bits. Self-consistency failures localise the defect without needing an oracle.
6. **Orientation swap** — the same data as rows and as columns, and a `known_y's` row with
   `known_x's` columns — to pin the shape rules and the `#REF!` boundary.
7. **`known_x's` omitted** on non-equally-spaced data, confirming the `1 … n` default.
8. **`LINEST(y, x)` against `SLOPE` and `INTERCEPT`** on the same data, bit for bit — the
   cheapest cross-surface consistency check in the family.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `LINEST` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-LINEST-001: LINEST's Householder reflector stores each step from the x87 unit

*Detail for matching Excel's bits; measured; recorded 2026-11-04.* Applies to LINEST, LOGEST.

In Excel's QR route the reflector arithmetic behaves as stored precision-64 steps: v0 = x0 -
alpha, v'v, each dot-product term, f = 2t/v'v, and a - f*v. With this change the row below gets
Excel's last bits, where a plain binary64 reflector does not.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=INDEX(LINEST({-17.979;-0.126;8;-5;1.7;37;12.4;14.46;-8;24.2},{20.8,50;34,23.41;38.875,-17.7;4.863,36.9;42.1,6;-14.55,44.41;-4.57,-12.96;-9.909,2.589;47.549,42;54.9,-17.63},TRUE,TRUE),2,1)` | as typed | `0.19054144531344644` (`0x3fc863a97e13b089`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Gains 11 recorded answers and loses none over the 3,944 regression rows of
the corpus; 35 selections from two fits were checked directly.

- Open: Norm, rank, mean, pivot and back-substitution policies are not identified. 48 fixed rank policies and 240 rounded Gram/LDL rank labels are contradicted by the 241 recorded degrees-of-freedom observations.

Evidence:

- DnaCalc `crates/calc-func/tests/f1_stats.rs`, line 680 at `3f3b162200` — recorded selections pinned
- DnaCalc-Research `round22-result.json`, reflector_study at `130ec21b3d` — research round 22 result
- DnaCalc-Research `evidence/round22/householder-all-hard.json` at `130ec21b3d` — reflector study
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcj9k_linest_stats.capture.json`, sheet dcj9k_linest_stats, row 1094, column 4 at `c9f9d35416` (readings)

### FN-LINEST-002: LINEST with no residual degrees of freedom returns zeros and one #NUM!

*Quirk; measured; recorded 2026-11-04.*

With three observations, three x columns and const FALSE, LINEST with stats returns standard
errors of 0 and se_y of 0 rather than #DIV/0!, R^2 of 1, F of #NUM!, degrees of freedom 0,
ss_resid exactly 0 and ss_reg the sum of y^2.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=INDEX(LINEST({31.78;5.61;-9.09},{53.56,30.22,32;-9,-17.977,47.17;9.4,-11.83,39.39},FALSE,TRUE),2,1)` | as typed | `0` (`0x0000000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=INDEX(LINEST({31.78;5.61;-9.09},{53.56,30.22,32;-9,-17.977,47.17;9.4,-11.83,39.39},FALSE,TRUE),3,1)` | as typed | `1` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=INDEX(LINEST({31.78;5.61;-9.09},{53.56,30.22,32;-9,-17.977,47.17;9.4,-11.83,39.39},FALSE,TRUE),4,1)` | as typed | `#NUM!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=INDEX(LINEST({31.78;5.61;-9.09},{53.56,30.22,32;-9,-17.977,47.17;9.4,-11.83,39.39},FALSE,TRUE),5,1)` | as typed | `1124.0686` (`0x409190463f141206`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): One fit, 20 selections read through INDEX.

Evidence:

- DnaCalc `crates/calc-func/tests/f1_stats.rs`, line 725 at `db71314731` — recorded selections pinned
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcj9k_linest_stats.capture.json`, sheet dcj9k_linest_stats, row 1180, column 4 at `c9f9d35416` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcj9k_linest_stats.capture.json`, sheet dcj9k_linest_stats, row 1184, column 4 at `c9f9d35416` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcj9k_linest_stats.capture.json`, sheet dcj9k_linest_stats, row 1188, column 4 at `c9f9d35416` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcj9k_linest_stats.capture.json`, sheet dcj9k_linest_stats, row 1192, column 4 at `c9f9d35416` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| design matrix `X` | The `n × p` matrix of predictor columns, plus a ones column when `const` is TRUE |
| normal equations | `XᵀXβ = Xᵀy`; the algebraically direct but conditioning-squaring route |
| condition number `κ₂(X)` | Ratio of largest to smallest singular value; the accuracy budget of the fit |
| QR route | Solving least squares by orthogonal factorisation of `X` without forming `XᵀX` |
| statistics block | The five-row array returned when `stats` is TRUE |
| reversed coefficient order | `m_k … m₁, b`; the last predictor's slope comes first |
| StRD | The NIST Statistical Reference Datasets, with certified values for regression problems |

## Sources

- Microsoft, "LINEST function" —
  <https://support.microsoft.com/en-us/office/linest-function-84d7d0d9-6e50-4101-977a-fa7abf772b6d>
  (signature, the returned-array layout, the `const` and `stats` switches, and the documented
  error conditions). Retrieval was blocked for this pass; the layout table is stated as
  documented behaviour with the source named.
- Golub and Van Loan, *Matrix Computations*, chapter 5 — least squares by QR and SVD, and the
  `κ₂(XᵀX) = κ₂(X)²` result.
- Björck, *Numerical Methods for Least Squares Problems* (SIAM, 1996).
- Lawson and Hanson, *Solving Least Squares Problems* (SIAM classic).
- Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd ed., chapters 19–20.
- Longley, *An appraisal of least squares programs from the point of view of the user* (JASA,
  1967) — the original ill-conditioned benchmark.
- NIST Statistical Reference Datasets, linear regression collection (Norris, Longley, Wampler,
  Filip) — certified values.
- McCullough and Wilson, *On the accuracy of statistical procedures in Microsoft Excel*
  (Computational Statistics & Data Analysis, 1999 and successors) — the StRD critique of
  Excel's regression lane, named as published literature rather than as Handbook evidence.
- Morten Welinder's work on Gnumeric's statistical functions — the reimplementer's record.
- Handbook evidence record `EV-MISC-0010` (subjects `FUNC.LINEST`, `FUNC.LOGEST`; class
  open-discrepancy; carries a reader warning against per-surface attribution).
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [The value universe](../model/01-value-universe.md).
- Handbook projections `data/functions/FUNC.LINEST.json` (arity 1–4, `xlfLinest`,
  `RefsVisibleInAdapter`) and `data/presence/FUNC.LINEST.json` (module
  `regression_forecast_family.rs`, shared with `FORECAST`, `FORECAST.LINEAR`, `GROWTH`,
  `LOGEST`, `TREND`; upstream defect stream `BUG-FUNC-028` named on the module).
