---
schema: efh.function-page/v1
function_id: FUNC.WEIBULL.DIST
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0031
  - EV-DIST-0033
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
family: special_dist_family
role_in_family: >-
  The reliability distribution of the special-function module, and the surface in this family
  whose numerical substrate has been identified in detail and then gated on fresh held-out rows
  against a live Excel oracle.
---

# WEIBULL.DIST

## What it computes

`WEIBULL.DIST(x, alpha, beta, cumulative)` evaluates the two-parameter Weibull distribution with
shape α and scale β — the cumulative distribution function when `cumulative` is TRUE, the
probability density when it is FALSE.

Microsoft states both forms on the function's page. Writing them out:

    F(x; α, β) = 1 − exp( −(x/β)^α )                            cumulative
    f(x; α, β) = (α / β^α) · x^(α−1) · exp( −(x/β)^α )          density

for `x ≥ 0`, `α > 0`, `β > 0`. `F` rises strictly from 0 at `x = 0` to 1 as `x → ∞`; `f` is
non-negative and integrates to 1 over `[0, ∞)`.

The single structural fact that organizes everything else is the **hazard rate**:

    h(x) = f(x) / (1 − F(x)) = (α/β) · (x/β)^(α−1)

a pure power of `x`. That is what the Weibull *is*: the distribution whose instantaneous failure
rate is a power law. The shape parameter chooses which regime:

| α | Hazard | Interpretation |
|---|---|---|
| α < 1 | decreasing | Infant mortality — early failures dominate |
| α = 1 | constant | Memoryless; the **exponential** distribution with mean β |
| α = 2 | linearly increasing | The **Rayleigh** distribution, with `σ = β/sqrt 2` |
| α > 1 | increasing | Wear-out |

Microsoft documents the α = 1 case explicitly: `WEIBULL.DIST` returns the exponential
distribution there.

Other facts worth having on the page:

- **Moments.** `E[X^k] = β^k · Γ(1 + k/α)`, so the mean is `β·Γ(1 + 1/α)` and the variance is
  `β²·[Γ(1 + 2/α) − Γ(1 + 1/α)²]`. Every moment exists for every α > 0 — the Weibull's tail is
  stretched-exponential, not power-law, which is exactly the contrast with the t-distribution.
- **Quantile.** `F` inverts in closed form: `x = β · (−ln(1 − p))^(1/α)`. This is elementary
  and exactly computable, which matters below, because **Excel publishes no `WEIBULL.INV`
  surface** — the inverse must be written out by the caller.
- **Scale is a pure scaling.** `F(x; α, β) = F(x/β; α, 1)`, so β never affects shape. The whole
  family is one function of the reduced variable `u = x/β`.
- **Min-stability.** The minimum of independent Weibulls with common α is Weibull with the same
  α; the Weibull is the type III extreme-value distribution for minima. That is why it dominates
  reliability engineering: a system that fails when its weakest component fails inherits the
  shape.

There are no poles in `F`. The density has one, at `x = 0` when `α < 1`, where
`x^(α−1) → +∞`. That corner is the sharpest edge in the whole surface and is unaddressed by the
documentation.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `x` | "The value at which to evaluate the function." Documented non-negative. | Yes |
| `alpha` | "A parameter to the distribution" — the shape parameter. Documented positive. | Yes |
| `beta` | "A parameter to the distribution" — the scale parameter. Documented positive. | Yes |
| `cumulative` | "Determines the form of the function." TRUE gives the CDF, FALSE the density. | Yes |

All four positions are required; the projection records an arity of exactly four. Microsoft's
argument descriptions for `alpha` and `beta` are the least informative in the statistical
category — neither is named as shape or scale, and neither is given a role. A reader who does
not already know the distribution learns nothing about which parameter does what from the
documentation, which is why this page states the hazard-rate reading above.

Numeric slots take ordinary to-number coercion; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`: a probability in `[0, 1)` for the cumulative form, a density in `[0, ∞)` for
the density form.

- **`x = 0`, cumulative** is exactly 0, for every α and β.
- **`x = 0`, density** is where the three shape regimes visibly separate: the value is 0 for
  α > 1, exactly `1/β` for α = 1, and mathematically infinite for α < 1. The documentation says
  nothing about this. Excel has no infinity in its value model
  ([The value universe](../model/01-value-universe.md)), so the α < 1 corner must resolve to a
  finite number, `#NUM!`, or `#DIV/0!` — and which one is unknown. It is the first probe below.
- **`x = β`, cumulative** is `1 − 1/e` for every α — an exact, shape-independent value, and
  therefore a free correctness probe on any implementation.
- **Small `(x/β)^α`.** `F` behaves like `(x/β)^α` itself. Computed as a literal
  `1 − exp(−t)` in double precision, everything below `t ≈ 2⁻⁵³` is lost to zero. Computed as
  `−expm1(−t)` it survives to the underflow floor. This is the defining accuracy question for
  the surface, and it is discussed under Numerical notes.
- **Large `(x/β)^α`.** `F` saturates at 1 and the density underflows to 0. The saturation point
  moves with α, so a large α turns a modest `x/β` into an overflowed exponent.
- **`cumulative` coercion.** The flag is a logical slot; how a number, text or blank in that
  position is interpreted follows the shared call model, and the Handbook has not checked it for
  this surface.
- **Arrays.** The lift axis is projected as `surface_native` with `default-unexamined`
  provenance. The reference-engine battery beside this page does return an array for array
  arguments in this surface — unlike its t-family neighbours — which is a difference in the
  engine's behaviour worth knowing about, and says nothing about Excel.

## Errors

As documented on Microsoft's `WEIBULL.DIST` page:

| Error | Documented condition |
|---|---|
| `#VALUE!` | `x`, `alpha`, or `beta` is nonnumeric. |
| `#NUM!` | `x < 0`. |
| `#NUM!` | `alpha ≤ 0`. |
| `#NUM!` | `beta ≤ 0`. |

Note what is *not* covered: nothing is said about a nonnumeric `cumulative`, and nothing is said
about the `x = 0`, `α < 1` density singularity. Errors in any argument propagate under the
universal coercion rule.

The projection records `arg_domain_guard=none; non_finite=allow` for this surface, which reads
oddly beside three documented domain refusals — but that axis carries `default-unexamined`
provenance, so it is almost certainly a projection default rather than a statement about the
implementation. It is recorded so a later pass examines it instead of inheriting it.

## Relationships

- **`WEIBULL`** is the legacy compatibility spelling, documented to compute the same thing.
  Whether it computes it the *same way* is a separate question, and the Handbook holds an
  evidence record on exactly that point: `EV-DIST-0033` records that the sign-off measurement in
  this cluster was made on the **modern** surface and on the modern kernel by name, and that
  nothing measures the legacy spelling separately. The record's reader warning is explicit that
  the modern figure may not be inherited onto `WEIBULL`. The contrast the record draws is the
  useful part: the gamma-side legacy/modern collapse *was* measured, and this pair is not among
  the pairs it covers.
- **`EXPON.DIST`** is the α = 1 special case, and the two surfaces overlap exactly there.
  `WEIBULL.DIST(x, 1, β, TRUE)` and `EXPON.DIST(x, 1/β, TRUE)` are the same number — note the
  reciprocal, since `EXPON.DIST` is parameterized by rate λ and `WEIBULL.DIST` by scale β. That
  reciprocal is a standing trap in both directions.
- **There is no `WEIBULL.INV`.** The quantile has an elementary closed form,
  `beta * (-LN(1-p))^(1/alpha)`, and a caller who needs it writes it out. That expression forms
  `1 − p`, so it loses precision for `p` near 1; `beta * (-LN(q))^(1/alpha)` with `q = 1 − p`
  supplied directly is the accurate form when the caller has `q`.
- **`GAMMA.DIST`** is the other main lifetime distribution and the usual alternative; the two
  agree only at α = 1, where both reduce to the exponential.
- **`NORM.DIST`** relates through the log: `ln X` for Weibull `X` is a Gumbel (smallest-extreme)
  variate, not a normal one — which is the technical reason Weibull probability paper is not
  lognormal probability paper.
- In the reference engine this surface shares its implementing module with `ERF`, `ERFC`,
  `GAMMA`, `GAMMALN` and the legacy `WEIBULL`.

## Numerical notes

**The cancellation is the whole story for the CDF.** `1 − exp(−t)` for small `t` is the textbook
catastrophic-cancellation example: the true value is `t − t²/2 + …`, and the computed value has
absorbed the entire result into the rounding of `exp(−t)` against 1. Below `t ≈ 2⁻⁵³` the naive
form returns exactly zero for a quantity that is perfectly representable. The remedy is
`−expm1(−t)`, which is exactly what `expm1` was standardized for, and it costs nothing. Any
implementation of a Weibull CDF that does not use `expm1` (or an equivalent series for small `t`)
is wrong in a region real reliability data occupies — early-life failure probabilities are small
by construction.

**Forming `t = (x/β)^α` is the second problem.** Three separate hazards:

1. *Overflow and underflow of the power.* For large α, a modest `x/β` produces an enormous or
   vanishing `t`. Evaluating as `exp(α · ln(x/β))` moves the problem into the exponent, where it
   is at least detectable, but introduces its own rounding: an error of ε in `ln(x/β)` becomes an
   error of `α·ε` in the exponent, so accuracy degrades linearly in α.
2. *`x/β` near 1.* This is where `ln(x/β)` suffers cancellation of its own; `log1p((x − β)/β)` is
   the accurate form, and it matters because `x ≈ β` is the characteristic-life region where
   engineers actually read the curve.
3. *The division itself.* `x/β` rounds once before the power is taken, and that single rounding
   is then amplified by α. Whether the division is performed first or folded into the exponential
   is a visible implementation decision, not an invisible one.

**The density adds a third factor.** `x^(α−1)` must not be computed as `pow(x, α−1)` when α is
near 1 and `x` is far from 1, and the leading `α/β^α` overflows for large α and small β even when
the product is well scaled; grouping as `(α/β)·(x/β)^(α−1)` keeps every intermediate in range and
is the form the hazard-rate reading suggests anyway.

**On what the reference engine does.** The Handbook's evidence record `EV-DIST-0031` describes
the identified substrate for this surface in the reference engine: a legacy x87
per-operation double-rounded compilation unit, with the cumulative branch forming the reduced
variable, raising it through a power chain, and publishing the negated `expm1` of the negated
result — and the density evaluated left to right, division first, with every operation
double-rounded through a spilled local. That record is a **substrate identification**, gated on
fresh held-out rows against a live Excel oracle; its counts belong to the evidence layer
rendered beside this page and are not restated in this prose. The record also names its own
limits: the build is not restated near its scored line, and its sole residual belongs to a named
open class.

The relevant lesson for an implementer is the one the substrate makes concrete: on this surface
the *association order and the rounding of each intermediate* are load-bearing, not just the
mathematical formula. Two implementations of `1 − exp(−(x/β)^α)` that agree on paper can differ
in the last bits purely through where the compiler spilled a double.

Nothing in this section is a claim that Excel uses any particular algorithm. The identification
record concerns the reference engine's kernel and the evidence it was gated on.

## What has not been checked

Two Handbook evidence records list this surface among their subjects, and they say different
kinds of thing, so the distinction matters:

- `EV-DIST-0031` is a **substrate identification** with a fresh held-out gate against a live
  Excel oracle, measured on the production kernel. This surface has therefore been compared
  against Excel — which is more than almost any other statistical page in this batch can say.
  Its figures, its residual class, and its stated build ambiguity are rendered mechanically
  beside this page.
- `EV-DIST-0033` is an **alias-pairing** record whose content is a negative: nothing measures the
  legacy `WEIBULL` spelling separately, and the modern figure may not be inherited onto it.

What still does not exist: **no Handbook vector suite for `WEIBULL.DIST`**, so nothing here is
independently re-runnable by a reader, and no Handbook re-verification of the upstream record
(it carries `handbook_reverified: false`). The identification concerns the reference engine's
kernel; it is not a statement that the mathematics is right, and a kernel can reproduce Excel's
bits while both are further from the true value than a careful implementation would be.

Also unchecked, and not covered by either record: the density branch at its singular corner, the
`cumulative` flag's coercion, array behaviour in Excel, and the legacy `WEIBULL` spelling.

Inputs worth probing first:

1. **`WEIBULL.DIST(0, 0.5, 1, FALSE)`** — the density pole at `x = 0` for `α < 1`. Excel has no
   infinity, so this input forces a decision, and the documentation records none. Compare with
   `α = 1` (should be exactly `1/β`) and `α = 2` (should be exactly 0) in the same triple.
2. **`WEIBULL.DIST(beta, alpha, beta, TRUE)` for many α and β** — must be `1 − 1/e` for every
   one of them. A shape-independent exact value is the cheapest diagnostic on the surface, and a
   drift that varies with α localizes the error to the power chain rather than the exponential.
3. **Very small `(x/β)^α`**: `x = 1e-30`, `β = 1`, `α = 1`, cumulative. The true answer is
   `x` to within a relative `1e-30`. An implementation without `expm1` returns zero here, and
   the probe separates the two designs in one call.
4. **`WEIBULL.DIST(x, 1, β, TRUE)` against `EXPON.DIST(x, 1/β, TRUE)`** — an exact identity
   between two Excel surfaces, needing no external oracle, and the probe that would catch a
   rate-versus-scale reciprocal error anywhere in either.
5. **`WEIBULL(x, α, β, c)` against `WEIBULL.DIST(x, α, β, c)`** — the legacy/modern pairing that
   `EV-DIST-0033` records as never having been probed. Cheap, and it converts an assumption into
   a finding either way.
6. **`x ≈ β` with large α** — `α = 100`, `x/β` at `1 ± 1e-9` — where the `log1p` question bites
   and where a naive `ln(x/β)` loses digits that α then multiplies.
7. **Extreme α**: `α = 1e-6` and `α = 1e6`, at fixed `x/β`, to find where the power chain
   overflows and what is published at the boundary.
8. **`alpha = 0`, `beta = 0`, `x < 0`** — confirming the three documented `#NUM!` refusals, and
   a nonnumeric `cumulative`, which the documentation does not cover at all.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| shape (α) | The exponent controlling the hazard-rate power law |
| scale (β) | The characteristic life; `F(β) = 1 − 1/e` regardless of α |
| hazard rate | `f/(1−F)`; a pure power of `x` for the Weibull |
| reduced variable | `u = x/β`; the whole family depends on `x` and `β` only through it |
| `expm1` cancellation | The loss of `1 − exp(−t)` for small `t` unless `expm1` is used |
| substrate identification | An evidence class: what arithmetic a kernel was found to perform |
| alias pairing | An evidence class: whether a legacy spelling was measured separately |

## Sources

- Microsoft, "WEIBULL.DIST function" —
  <https://support.microsoft.com/en-us/office/weibull-dist-function-4e783c39-9325-49be-bbc9-a83ef82b45db>
  (syntax, the four required arguments, the cumulative and density equations, the statement that
  `alpha = 1` returns the exponential distribution, and the `#VALUE!` and three `#NUM!`
  conditions). Retrieved for this page.
- Handbook evidence record `EV-DIST-0031` — substrate identification for this surface plus a
  fresh held-out gate against a live Excel oracle on the production kernel, with its own stated
  build ambiguity and residual class.
- Handbook evidence record `EV-DIST-0033` — the alias-pairing record establishing that the
  legacy `WEIBULL` spelling carries no measurement of its own and may not inherit this surface's
  figure.
- M. Abramowitz and I. A. Stegun, *Handbook of Mathematical Functions*, chapter 26 (probability
  functions) for the surrounding distribution material; the Weibull's extreme-value character is
  the type III minimum law of the classical extreme-value theory in that literature.
- IEEE 754 and the C standard `expm1` / `log1p` rationale — the two primitives this surface's
  accuracy rests on; fdlibm is the readable reference implementation of both.
- Handbook [The value universe](../model/01-value-universe.md) (no infinity kind, which is what
  makes the density pole a decision) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.WEIBULL.DIST.json`,
  `data/presence/FUNC.WEIBULL.DIST.json` (the shared `special_dist_family` module) and
  `data/battery/FUNC.WEIBULL.DIST.json`.
