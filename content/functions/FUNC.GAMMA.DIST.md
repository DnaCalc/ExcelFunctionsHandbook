---
schema: efh.function-page/v1
function_id: FUNC.GAMMA.DIST
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0002
  - EV-DIST-0011
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Gamma_Dist method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.gamma_dist"
    role: "documented parameters, the cumulative flag, the three error conditions, the alpha=1 exponential case, the chi-square identity, and the Erlang remark"
  - work: "Microsoft Support — GAMMA.DIST function"
    locator: "https://support.microsoft.com/en-us/office/gamma-dist-function-9b6f1538-d11c-4d5f-8966-21f6a2201def"
    role: "the worksheet-surface documentation page; not retrievable at curation time (the host refused the request)"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapters 6 and 26"
    locator: "6.5 — incomplete gamma function; 26.4 — the chi-square distribution and its relation to the incomplete gamma"
    role: "the mathematics of the regularized incomplete gamma function and the chi-square link"
  - work: "W. Gautschi, 'A computational procedure for incomplete gamma functions' (TOMS, 1979)"
    locator: null
    role: "the standard series/continued-fraction split for the incomplete gamma"
  - work: "NSWC Library / DCDFLIB GRATIO"
    locator: null
    role: "the incomplete-gamma-ratio routine family named in the upstream identification work"
  - work: "L. Knüsel and B. D. McCullough, published assessments of Excel's statistical distribution functions"
    locator: null
    role: "the standing external record on Excel's distribution accuracy"
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
family: beta_gamma_stats_family
role_in_family: >-
  The forward gamma distribution, both density and cumulative, and the surface the module's
  gamma inverse roots; its cumulative mode routes through the shared regularized incomplete
  gamma while its density mode does not.
---

## What it computes

`GAMMA.DIST(x, alpha, beta, cumulative)` evaluates the two-parameter gamma distribution — its
probability density when `cumulative` is FALSE, its cumulative distribution function when TRUE.

With shape α > 0 and scale β > 0, for x ≥ 0:

    density      f(x; α, β)  =  x^{α−1} · e^{−x/β}  /  ( β^α · Γ(α) )

    cumulative   F(x; α, β)  =  P(α, x/β)  =  γ(α, x/β) / Γ(α)
                             =  (1/Γ(α)) ∫₀^{x/β} t^{α−1} e^{−t} dt

`P(a, z)` is the **regularized lower incomplete gamma function**, and it is the whole
computational story: the cumulative mode of this function *is* P, and every accuracy question
about it is a question about P. Abramowitz & Stegun treat the incomplete gamma in §6.5.

**Parameters and their admissible ranges.** α > 0 (shape) and β > 0 (scale) — Excel uses the
scale parametrisation, not the rate parametrisation used by R's `dgamma(shape, rate)` and by
much of the statistics literature. β and 1/rate are the same number; confusing them is the most
common way to get a plausible wrong answer out of this function. Microsoft names β "a parameter
to the distribution" and notes only that β = 1 gives "the standard gamma distribution".

**Moments.** Mean αβ, variance αβ², skewness 2/√α. The distribution is right-skewed for every
finite α and tends to normality as α → ∞.

**Shape behaviour, which changes qualitatively at α = 1.**

| α | Density at x → 0⁺ | Shape |
|---|---|---|
| α < 1 | → +∞ | strictly decreasing, integrable singularity at the origin |
| α = 1 | → 1/β | exponential; the density is finite and non-zero at the origin |
| α > 1 | → 0 | unimodal with mode at (α − 1)β |

That table is worth internalising because the origin is a genuine boundary case with three
different answers, and implementations disagree there — see **Result and edge cases**.

**Documented special cases**, all from Microsoft's page:

- **α = 1** gives the exponential distribution with mean β.
- **α = n/2, β = 2, cumulative TRUE** gives `1 − CHIDIST(x)` with n degrees of freedom — the
  chi-square distribution is the gamma with those parameters, and A&S §26.4 gives the same
  identity from the other direction.
- **α a positive integer** makes this the Erlang distribution.

Each of those is a metamorphic identity a reader can check without an oracle, and each is on
the probe list.

## Arguments

Four arguments, all required at the call site; the reference engine declares an arity of
exactly 4.

| Argument | Meaning |
|---|---|
| `x` | The value at which to evaluate the distribution. |
| `alpha` | The shape parameter α. |
| `beta` | The scale parameter β. Documented: "If beta = 1, GAMMA.DIST returns the standard gamma distribution." |
| `cumulative` | Logical. TRUE → cumulative distribution function; FALSE → probability density function. |

All four are values-only slots under ordinary to-number coercion; see
[Coercion and lifting](../model/02-coercion-and-lifting.md). The reference engine treats
`cumulative` as a numeric flag — non-zero is TRUE — rather than requiring a logical.

A note on the modern-versus-legacy pair that belongs with the arguments: the reference engine's
source records that the legacy `GAMMADIST` surface is scalar-shaped and broadcasts only its
**first three** argument positions over an array, while the modern `GAMMA.DIST` lifts natively.
Upstream attributes that distinction to a live Excel observation on a named build. If it holds,
the two surfaces differ in *array behaviour* even where they agree on scalars — which is
exactly the kind of difference an alias-identity check on scalar probes cannot see.

## Result and edge cases

Returns `Number`.

- **x = 0** is inside the documented domain (`#NUM!` is documented only for x < 0) and the three
  α regimes above give three different correct answers for the density: +∞ for α < 1, 1/β for
  α = 1, and 0 for α > 1. The cumulative value is 0 for every α. **Reading the reference
  engine's density branch, x = 0 does not produce any of those**: the log-form expression
  computes (α − 1)·ln(x), which for x = 0 and α = 1 is 0 · (−∞) — a NaN — and the kernel maps a
  non-finite result to `#NUM!`. So where the density is defined and finite, the reference engine
  reaches an error. This is a reading of the reference kernel, not an observation of Excel; it
  is the first item on the probe list below precisely because it is cheap to settle and would be
  a real defect if confirmed.
- **Very large x** drives the cumulative value to 1 and the density to 0 (underflow). Neither is
  an error.
- **Very large α** makes both modes hard: Γ(α) overflows for α ≳ 171, so any implementation that
  forms Γ(α) explicitly rather than lnΓ(α) fails well before the distribution becomes
  uninteresting.
- **Non-integer α** is entirely ordinary — the gamma distribution is defined for all positive
  real α, and only the *Erlang* special case requires an integer.
- **The cumulative mode is monotone non-decreasing in x** and bounded in [0, 1]; any
  implementation that returns a value outside that interval, or that is non-monotone, is
  observably wrong without needing an oracle. That is a strong self-check and it is under-used.
- **Array arguments**: the modern surface lifts; the projected battery beside this page records
  the reference engine's answers to a fixed probe list, including array-shaped inputs. Those are
  OxFunc's answers; no Excel was involved.

## Errors

As documented by Microsoft:

| Error | Condition |
|---|---|
| `#VALUE!` | `x`, `alpha` or `beta` is nonnumeric |
| `#NUM!` | `x` < 0 |
| `#NUM!` | `alpha` ≤ 0, or `beta` ≤ 0 |

Two observations about the boundary of that table:

1. **The documentation says nothing about the `cumulative` argument's error behaviour.** The
   `#VALUE!` row names the three numeric arguments only.
2. **The documented table admits x = 0**, and the density there is finite for α ≥ 1. See the
   divergence recorded under **Result and edge cases**.

The reference engine additionally maps non-finite `alpha`, `beta` or `x` to `#VALUE!` rather
than `#NUM!` — a choice with no documented counterpart.

## Relationships

- **`GAMMADIST`** — the legacy spelling, classified under **Compatibility** in the catalogue
  while this surface is classified under **Statistical functions**. `EV-DIST-0011` is an
  alias-pairing record that names both: it reports an upstream identity check across one probe
  battery on one named Excel build in which five legacy forward-CDF surfaces published the same
  results as their modern counterparts, this pair among them. Read the limit the record states
  in as many words: **the collapse covers the five forward CDFs only.** It licenses treating a
  figure measured on one spelling as relevant to the other *for the forward direction*, and it
  licenses nothing about inverses. It is also one battery on one build, and the Handbook has no
  suite of its own with which to restate it as a Handbook claim.
- **[GAMMA.INV](FUNC.GAMMA.INV.md)** — the inverse of the cumulative mode. Microsoft states the
  round trip: if p = GAMMA_DIST(x, …) then GAMMA_INV(p, …) = x. Its precision is documented to
  depend on this function's precision, which makes this page the upstream dependency of that one.
- **`CHISQ.DIST` / `CHIDIST`** — the α = n/2, β = 2 special case, documented on Microsoft's page
  as `1 − CHIDIST(x)`.
- **`EXPON.DIST`** — the α = 1 special case.
- **`POISSON.DIST`** — the discrete dual. The Erlang cumulative and the Poisson tail are the
  same sum, which is why an implementation of one is often reachable from the other.
- **[GAMMA](FUNC.GAMMA.md) and [GAMMALN](FUNC.GAMMALN.md)** — the *complete* gamma function, a
  different object from the incomplete one this page needs. Sharing the name is the relationship.
- **`BETA.DIST`** — the module sibling; the regularized incomplete beta is the two-parameter
  cousin of the regularized incomplete gamma and is reached by the same style of algorithm.

## Numerical notes

**The cumulative mode is the incomplete gamma ratio, and the incomplete gamma ratio has one
classical algorithm.** For z < a + 1 the series

    P(a, z) = z^a e^{−z} / Γ(a+1) · Σ_{n≥0} z^n / ((a+1)(a+2)…(a+n))

converges quickly; for z ≥ a + 1 the complementary function Q(a, z) = 1 − P(a, z) is evaluated
by a continued fraction, and P is recovered by subtraction. That split — series below the
crossover, continued fraction above, and always compute the *smaller* of P and Q directly — is
Gautschi's procedure, is what Numerical Recipes presents as `gammp`/`gammq`, and is what every
serious library does. Computing the larger one and subtracting is the classic way to lose all
precision in the tail.

**Why the tails are where distributions fail.** In the far right tail P is 1 − (something
tiny), and a double cannot represent 1 − 10⁻²⁰ as distinct from 1. Any question about the
extreme upper tail must be asked of Q, not of P — which is why Excel's right-tail surfaces
(`CHISQ.DIST.RT`, `F.DIST.RT`) exist as separate functions and why `GAMMA.DIST` has no
right-tail sibling. A reader who needs gamma upper-tail probabilities below about 10⁻¹⁶ cannot
get them from this function at all.

**The density mode is a different, and worse, computation.** Written directly, the density needs
x^{α−1}, e^{−x/β}, β^α and Γ(α), each of which can overflow or underflow while the product is
perfectly ordinary. The standard fix is to compute the logarithm — (α−1)ln x − x/β − lnΓ(α) −
α ln β — and exponentiate, which is what the reference engine does. That trades overflow for
**ln-amplification**: an error ε in the log becomes a relative error of roughly |log| · ε in the
value, and the log is large exactly where the density is small. The state of the art for
discrete and continuous densities alike is Loader's *saddle-point* formulation, which rewrites
the density in terms of a deviance term evaluated without cancellation, and which is what R's
`dgamma` and `dpois` use. `EV-DIST-0002` records that the density mode measures far worse than
the cumulative mode on a much larger corpus, and that a triage of candidate density formulations
— the log-composed form, separate powers, ratio forms, and an R-style structure — was refuted at
the exact-value level. In other words the density mode's algorithm is not merely inaccurate, it
is **unidentified**.

**A removed shortcut, worth recording.** The reference engine formerly took an integer-shape fast
path for the cumulative mode, summing the Erlang series 1 − e^{−z}Σ z^k/k!. Upstream removed it
after measuring that Excel does *not* take that special case: the shortcut scored dramatically
worse than routing all shapes through the incomplete gamma ratio, and it produced very large
outliers. The lesson generalises — a mathematically valid special case is a *liability* for a
compatibility implementation unless the target is known to take it too.

**The identified substrate.** Upstream's identification work names the NSWC/DCDFLIB `GRATIO`
routine family as the path Excel's forward gamma CDF resolves to, and the same work identifies
`ERF.PRECISE`/`ERFC.PRECISE` as the a < 1 branches of that same routine. That is a structural
identification with its own record and its own caveats; this page names it and does not restate
its figures.

## What has not been checked

Two records name this surface as a subject, and they say different kinds of thing.

`EV-DIST-0002` publishes three separate measurements that must be read together — a fitted
working corpus for the cumulative mode, a genuinely held-out integer-shape gate, and a much
larger density-mode corpus that scores far worse — and the record's own status note says
publishing the first alone would overstate the surface. `EV-DIST-0011` is the alias-pairing
record described under **Relationships**, whose scope is explicitly the forward CDFs only. All
figures render mechanically beside this page; this prose deliberately does not restate them.

What does not exist: any Handbook vector suite for `GAMMA.DIST`; any identification of the
density-mode algorithm; any measurement of the far tails, where the function is least accurate
and most consequential; any Handbook-side reproduction of the legacy/modern identity check.

Inputs I would probe first, and why:

1. **`GAMMA.DIST(0, 1, 1, FALSE)`, and the same at α = 0.5 and α = 2.** One probe each, and
   together they settle the x = 0 boundary for all three α regimes — including the case where
   reading the reference kernel predicts a `#NUM!` where a finite density exists. This is the
   cheapest probe on the page with the highest chance of exposing a real defect.
2. **The chi-square identity**: `GAMMA.DIST(x, n/2, 2, TRUE)` against `CHISQ.DIST(x, n, TRUE)`
   for several n. Microsoft documents the identity, so any disagreement is a
   documentation-versus-behaviour finding that needs no external oracle.
3. **The exponential identity**: `GAMMA.DIST(x, 1, β, ·)` against `EXPON.DIST(x, 1/β, ·)`,
   in both modes. Same argument, and it also tests the scale-versus-rate convention.
4. **The far right tail**, x/β well beyond α, in cumulative mode — where P saturates at 1 and
   the answer stops carrying information. Establishing *where* that happens is a publishable
   property of the surface, independent of any bit comparison.
5. **The far left tail and small α**, where P is tiny and the series is the only viable route.
6. **Large α** (α = 200, 1000), which forces lnΓ rather than Γ and separates implementations
   that form the normalising constant naively.
7. **Non-integer α near an integer**, e.g. α = 3 ± 10⁻¹², probing whether any integer-shape
   special case survives anywhere — the shortcut upstream removed is exactly the kind of thing
   another implementation may still have.
8. **`GAMMA.DIST` against `GAMMADIST` with an array argument**, which is where the recorded
   broadcast difference would show and where a scalar-only identity check is blind.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| shape α, scale β | The two parameters, in Excel's scale (not rate) parametrisation |
| regularized lower incomplete gamma | P(a, z) = γ(a, z)/Γ(a); the cumulative mode of this function |
| Q(a, z) | The complementary ratio 1 − P; the object the right tail must be computed from |
| Erlang | The gamma distribution with integer shape |
| ln-amplification | Relative error growth incurred by exponentiating a computed logarithm |
| saddle-point / deviance form | Loader's cancellation-free density formulation |
| GRATIO | The NSWC/DCDFLIB incomplete-gamma-ratio routine family named in the upstream identification |

## Sources

- Microsoft Learn, "WorksheetFunction.Gamma_Dist method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.gamma_dist>
  (the four parameters and the meaning of the cumulative flag, the three documented error
  conditions, the β = 1 standard-gamma remark, the α = 1 exponential case, the
  α = n/2, β = 2 chi-square identity, and the Erlang remark). The worksheet-surface page at
  `support.microsoft.com` was not retrievable at curation time.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, §6.5 (incomplete gamma) and §26.4
  (chi-square and its incomplete-gamma representation).
- W. Gautschi, "A computational procedure for incomplete gamma functions", *ACM TOMS* 5 (1979);
  Numerical Recipes, `gammp`/`gammq`; NSWC Library / DCDFLIB `GRATIO`; C. Loader, "Fast and
  accurate computation of binomial probabilities" (the saddle-point density formulation used by
  R's `dgamma`/`dpois`).
- Handbook evidence records `EV-DIST-0002` (the three cumulative/density figures and their
  scopes) and `EV-DIST-0011` (the legacy/modern forward-CDF identity check and its explicit
  forward-only limit).
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.GAMMA.DIST.json` (arity, category) and
  `data/presence/FUNC.GAMMA.DIST.json` (the `beta_gamma_stats_family` module, shared with the
  BETA and legacy GAMMA distribution surfaces).
- OxFunc `crates/oxfunc_core/src/functions/beta_gamma_stats_family.rs` at commit `473efa3` —
  `gamma_dist_kernel`, its routing of all shapes through `regularized_gamma_p`, the log-form
  density branch discussed above, the shape-validation error mapping, and the lane-3 comment
  recording the removal of the integer-shape fast path.
