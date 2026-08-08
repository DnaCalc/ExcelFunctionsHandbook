---
schema: efh.function-page/v1
function_id: FUNC.LOGNORM.DIST
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0020
  - EV-DIST-0021
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
family: normal_log_family
role_in_family: >-
  The lognormal forward distribution: a change of variable on the normal CDF, and the
  family member whose accuracy is inherited entirely from ln and Phi.
---

# LOGNORM.DIST

## What it computes

`LOGNORM.DIST(x, mean, standard_dev, cumulative)` evaluates the **lognormal** distribution.

A positive random variable `X` is lognormal with parameters `μ` and `σ` when `ln X` is normal
with mean `μ` and standard deviation `σ`. The two functions follow by change of variable:

**Density** (`cumulative = FALSE`):

    f(x; μ, σ) = 1 / ( x·σ·√(2π) ) · exp( −(ln x − μ)² / (2σ²) ),    x > 0

**Distribution function** (`cumulative = TRUE`):

    F(x; μ, σ) = Φ( (ln x − μ) / σ ),    x > 0

where `Φ` is the standard normal CDF. That second identity is the entire mathematical content
of the function: **the lognormal CDF is the normal CDF of the standardised logarithm.** The
`1/x` factor in the density is the Jacobian of the substitution `u = ln x`, and it is the only
place the two forms differ structurally.

**The parameters are those of `ln X`, not of `X`.** This is the single most common misreading
of this function, and it is worth setting the record out explicitly:

| Quantity of `X` | Expression |
|---|---|
| median | `e^μ` |
| mean | `exp(μ + σ²/2)` |
| mode | `exp(μ − σ²)` |
| variance | `(e^{σ²} − 1)·e^{2μ + σ²}` |

So `mean = 0, standard_dev = 1` describes a distribution whose median is 1 and whose mean is
`√e ≈ 1.6487`. Passing the sample mean and standard deviation of the *data* into this function
gives an answer to a different question. The correct parameter estimates are the mean and
standard deviation of the logged data.

**Domain.** `x > 0`, `σ > 0`, `μ` any real. **Range:** the density is positive and unbounded
above as `σ → 0`; the CDF is confined to `(0, 1)`, is strictly increasing on `(0, ∞)`, and
satisfies `F(0⁺) = 0`, `F(∞) = 1`. The distribution is right-skewed for every `σ > 0`, with
skewness `(e^{σ²} + 2)·√(e^{σ²} − 1)`, and all its moments are finite even though its
moment-generating function does not exist — a standard textbook curiosity, and the reason the
lognormal is not determined by its moments.

**The two forms are related by differentiation**, `f = F′`, and a correct implementation
should satisfy that relation numerically to within the accuracy of `Φ` and `ln`; it is a
usable self-check.

## Arguments

| Argument | Meaning | Admissible values |
|---|---|---|
| `x` | The value at which the distribution is evaluated | `x > 0`; required |
| `mean` | `μ`, the mean of `ln X` | Any finite real; required |
| `standard_dev` | `σ`, the standard deviation of `ln X` | `σ > 0`; required |
| `cumulative` | `TRUE` for the CDF, `FALSE` for the density | Logical; **required** |

The reference engine declares an arity of exactly 4: unlike several distribution surfaces
where `cumulative` is documented as optional, here every argument must be supplied. That is
also the visible difference from the legacy [LOGNORMDIST](FUNC.LOGNORMDIST.md), which takes
three arguments and computes only the cumulative form.

All four are scalar slots governed by ordinary to-number and to-logical coercion — see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`.

- **`x` at or below zero, or `σ` at or below zero** → outside the domain. Microsoft's page
  documents `#NUM!` for these; the reference engine's own battery (OxFunc's answers, no Excel
  involved) returns `#NUM!` for its zero and negative rows, consistent with that.
- **`x → 0⁺`** — the CDF tends to zero and the density tends to zero as well, because
  `exp(−(ln x)²/2σ²)` beats the `1/x` blow-up. The density therefore has a removable-looking
  but genuinely zero limit at the origin, which is why the lognormal density is smooth at 0
  even though the formula has a pole there.
- **Very large `x`** — the CDF approaches 1 and, in double precision, *reaches* it: once
  `(ln x − μ)/σ` exceeds roughly `8.3`, `Φ` rounds to exactly `1.0` and every distinction in
  the upper tail is lost. See Numerical notes; this is a representational limit, not an
  implementation defect, and it is the reason there is no way to compute a small upper-tail
  probability from this function.
- **Very small `x`** — the CDF underflows to zero once the standardised value falls below
  roughly `−38`, and the density underflows sooner because of the extra factor.
- **`σ` very small** — the distribution concentrates at `e^μ`; the density there behaves like
  `1/(e^μ σ√(2π))` and overflows for `σ` small enough. A density that overflows to `#NUM!` on
  a legitimate parameter is a real edge, and where the boundary falls is unverified.
- **`x`, `μ`, `σ` all at the largest finite double** — the standardised value tends to `−1`
  and the CDF to `Φ(−1)`; the reference engine's battery is consistent with that limit. It is
  a good sanity check that no intermediate overflows.
- **Arrays** in any slot: the reference engine's battery returns `#VALUE!` for a
  two-dimensional inline array literal rather than lifting the function elementwise. Modern
  Excel lifts scalar-slot arguments, so this is a **divergence candidate between the
  documented dynamic-array behaviour and the reference engine's current answer**, recorded as
  a finding. Nobody has checked it against Excel.

## Errors

| Error | Condition | Basis |
|---|---|---|
| `#NUM!` | `x ≤ 0`, or `standard_dev ≤ 0` | Documented by Microsoft on the `LOGNORM.DIST` page |
| `#VALUE!` | Any argument is non-numeric (or `cumulative` is not interpretable as a logical) | Documented; also the shared coercion rule |
| propagated | An error value in any argument | Shared coercion rule, chapter 02 |

Retrieval of Microsoft's `LOGNORM.DIST` page was blocked for this curation pass (the host
returned a refusal), so the documented rows above are stated as documented behaviour with the
source named rather than transcribed. Nobody has checked any of it against Excel within the
Handbook's record.

## Relationships

- **[LOGNORMDIST](FUNC.LOGNORMDIST.md)** — the legacy three-argument, cumulative-only
  spelling, retained in the Compatibility category. The documented relationship is
  `LOGNORMDIST(x, μ, σ) = LOGNORM.DIST(x, μ, σ, TRUE)`. **That is an equality of documented
  meaning, not a demonstrated equality of computation.** Two surfaces that document the same
  value may be reached by different code with different rounding; proving they return the same
  bits requires evidence, and the Handbook has none. Both of the evidence records attached to
  this page treat `LOGNORM.DIST` and `LOGNORMDIST` as an unresolved pair for exactly this
  reason — one of them notes that the upstream sentence carrying the only relevant figure does
  not say which of the two spellings was probed.
- **[LOGNORM.INV](FUNC.LOGNORM.INV.md)** — the quantile function, and the exact inverse in the
  mathematical sense: `LOGNORM.INV(LOGNORM.DIST(x, μ, σ, TRUE), μ, σ) = x`. The round trip is
  the natural metamorphic probe for both surfaces.
- **[NORM.DIST](FUNC.NORM.DIST.md)** and **[NORM.S.DIST](FUNC.NORM.S.DIST.md)** — the identity
  `LOGNORM.DIST(x, μ, σ, TRUE) = NORM.S.DIST((LN(x) − μ)/σ, TRUE)` holds exactly in
  mathematics. Whether it holds in bits is a first-class question, and the two surfaces share
  an implementing module in the reference engine, which makes agreement plausible and
  unproven.
- **[WEIBULL.DIST](FUNC.WEIBULL.DIST.md)**, **[GAMMA.DIST](FUNC.GAMMA.DIST.md)** — the other
  right-skewed positive-support distributions readers choose between. The lognormal is the
  one with the multiplicative central-limit justification: a product of many independent
  positive factors tends to lognormal, as a sum tends to normal.
- **Confused with:** the normal distribution of the data itself, and — persistently — with the
  meaning of the `mean` argument. See above.

## Numerical notes

`LOGNORM.DIST` performs almost no arithmetic of its own. Its accuracy is the accuracy of two
imported primitives, and the interesting engineering is entirely in how they are used.

**The composition.** The cumulative form is `Φ(z)` with `z = (ln x − μ)/σ`. Error enters at
three points:

1. **`ln x`** — a correctly rounded logarithm gives `z` a relative error of about one ulp in
   the logarithm, which becomes an *absolute* error in `z` of about `|ln x|·ε/σ`. For large
   `x` and small `σ` this is amplification: `x = 10¹⁰⁰` gives `ln x ≈ 230`, so an ulp of the
   logarithm is around `3·10⁻¹⁴`, divided by `σ`.
2. **The subtraction `ln x − μ`** — cancellation when `ln x ≈ μ`, i.e. near the median. The
   relative error of `z` there is unbounded, though the *absolute* error stays small, and
   since `Φ` is Lipschitz with constant `1/√(2π)` near zero the CDF value is still accurate in
   absolute terms. This is the usual and acceptable trade.
3. **`Φ(z)`** — the normal CDF, whose own accuracy is the subject of a long literature.

**The upper-tail problem.** `Φ(z) → 1` and there is no way back. In double precision
`Φ(z) = 1.0` exactly for `z ≳ 8.3`, and long before that the returned value has no significant
digits *in `1 − Φ(z)`*. Any calculation that needs a small exceedance probability —
value-at-risk, reliability, tail pricing — must not compute `1 − LOGNORM.DIST(...)`. The
mathematically correct route uses the complement directly, `1 − Φ(z) = Φ(−z) = ½·erfc(z/√2)`,
which is accurate to full relative precision for `z` out to about 37 before `erfc` itself
underflows. Excel exposes no complementary lognormal CDF, so the workaround inside the
worksheet is `LOGNORM.DIST(1/x, −μ, σ, TRUE)`, using the reflection identity: if `X` is
lognormal `(μ, σ)` then `1/X` is lognormal `(−μ, σ)`. That identity is exact and gives back
the upper tail as a lower tail. It is the single most useful practical fact on this page.

**The reference literature.** For `Φ` and `erfc`: Abramowitz and Stegun chapter 26, whose
formulas 26.2.16 to 26.2.19 are the classical series and continued-fraction expansions and
whose 26.2.17 rational approximation is the one embedded in an enormous amount of legacy code
at about seven decimal digits; Cody's rational Chebyshev approximations for `erf`/`erfc`
(*Rational Chebyshev approximation for the error function*, Math. Comp. 1969) and his ALGORITHM
715 / SPECFUN implementations, which are the accuracy standard; Hart et al., *Computer
Approximations*; and the treatments in Boost.Math and Cephes. For `ln`: fdlibm's
`__ieee754_log` and its glibc and musl descendants; Muller, *Elementary Functions*, for the
argument-reduction theory.

The published critique record on Excel's statistical distributions — McCullough and Wilson's
accuracy papers, Knüsel's assessments of statistical distribution accuracy in spreadsheets, and
Morten Welinder's work on Gnumeric's statistical functions — is the relevant background for
this family. The Handbook names it as literature and does not assert what Excel computes
internally.

**A note on the density.** `f(x)` should be evaluated as `exp(−z²/2) / (x·σ·√(2π))`, not by
exponentiating a difference of logarithms, and the `√(2π)` constant should be a correctly
rounded literal rather than computed as `sqrt(2*PI())`. For very negative `−z²/2` the
exponential underflows gradually; returning a subnormal is better than returning zero, and
whether that happens is observable.

## What has not been checked

Two evidence records list `FUNC.LOGNORM.DIST` among their subjects, and **neither of them is a
measurement of this surface.** Both are attached to this page so a reader can inspect them,
and both carry reader warnings that this page repeats rather than softens:

- `EV-DIST-0020` is classed as a **projection gap**. Its position is that the only Excel
  evidence any of its subjects has is a nine-decimal rounded text comparison, which is not a
  bit-level comparison at all, and that the surface appears in no bit-level corpus. It also
  records a *retraction*: an earlier numeric shortfall attributed to these surfaces was
  withdrawn, because none of them was among the witnesses that produced it. Its open question
  is stated plainly — whether this surface agrees with Excel at the bit level has never been
  measured.
- `EV-DIST-0021` is classed as a **substrate identification**. Its count is scoped to a
  *group* of the two LOGNORM spellings with attribution "named but not measured": a LOGNORM
  surface was used as an instrument to read Excel's internal logarithm, and the figures score
  that primitive, not the surface. The source sentence does not even say which of the two
  spellings was used. **The family was measured through this window; this surface was not
  measured separately.**

The figures, scopes, builds and warnings belonging to both records render mechanically in the
evidence panel beside this page. This prose deliberately transcribes none of them.

No Handbook vector suite exists for `LOGNORM.DIST`. There is no residual plate and no domain
sweep.

One divergence candidate found during this pass, recorded as a finding: the reference engine's
own battery returns `#VALUE!` for a two-dimensional inline array literal in the argument slots
rather than lifting elementwise, which sits awkwardly with modern Excel's scalar-slot lifting.
Scope and cause unestablished.

Inputs I would probe first, and why:

1. **The far upper tail**: `x` chosen so that `z` is `6`, `8`, `8.3`, `9`, `20`. The question
   is where the returned value becomes exactly `1.0` and whether it does so monotonically.
   This is where a naive `1 − Φ(−z)` implementation and a direct `Φ(z)` implementation part
   company, and it takes five cells.
2. **The far lower tail**: `z = −8, −20, −37, −38, −40`, to find the underflow boundary and
   whether subnormals are returned or flushed.
3. **`x = e^μ` exactly** — the median, where `ln x − μ` cancels to zero and the answer must be
   exactly `0.5`. Any deviation from `0.5` is a direct reading of the cancellation error.
4. **The reflection identity**: `LOGNORM.DIST(x, μ, σ, TRUE)` against
   `1 − LOGNORM.DIST(1/x, −μ, σ, TRUE)` across the whole range. These must sum to one exactly
   in mathematics; the residual is a free accuracy plate that needs no external oracle.
5. **The composition identity**: `LOGNORM.DIST(x, μ, σ, TRUE)` against
   `NORM.S.DIST((LN(x) − μ)/σ, TRUE)`, bit for bit. If they differ, the difference isolates
   whether the lognormal surface has its own path or delegates.
6. **The legacy-alias question**: `LOGNORM.DIST(x, μ, σ, TRUE)` against
   `LOGNORMDIST(x, μ, σ)` on a wide grid, bit for bit. This is the probe that would turn a
   documented equivalence into a demonstrated one, and it is currently unanswered.
7. **The differentiation relation**: `LOGNORM.DIST(x, μ, σ, FALSE)` against a high-accuracy
   numerical derivative of the cumulative form, to check that the two branches are consistent
   with each other rather than independently wrong.
8. **`σ` at `10⁻³⁰⁰` and at `10³⁰⁰`**, and `μ` at `±700`, to find where intermediate overflow
   appears in the density.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| `μ`, `σ` | The mean and standard deviation of `ln X` — not of `X` |
| standardised logarithm | `z = (ln x − μ)/σ`, the argument handed to `Φ` |
| `Φ` | The standard normal cumulative distribution function |
| complementary form | `1 − Φ(z) = Φ(−z) = ½·erfc(z/√2)`; the accurate route to the upper tail |
| reflection identity | `X` lognormal `(μ,σ)` ⟺ `1/X` lognormal `(−μ,σ)`; converts an upper tail to a lower tail |
| legacy alias | `LOGNORMDIST`; documented as the cumulative case, not demonstrated to be the same computation |

## Sources

- Microsoft, "LOGNORM.DIST function" —
  <https://support.microsoft.com/en-us/office/lognorm-dist-function-eb60d00b-48a9-4217-be2b-6074aee6b070>
  (signature, the stated equation, and the `#NUM!` and `#VALUE!` conditions). The host refused
  retrieval for this pass; documented rows are stated as documented behaviour with the source
  named.
- Abramowitz and Stegun, *Handbook of Mathematical Functions*, chapter 26 (normal distribution
  and error function; 26.2.16–26.2.19 for the classical expansions, 26.2.17 for the rational
  approximation embedded in much legacy code).
- Cody, *Rational Chebyshev approximation for the error function*, Mathematics of Computation
  23 (1969), and the SPECFUN/ALGORITHM 715 implementations — the accuracy standard for `erf`
  and `erfc`.
- Hart et al., *Computer Approximations*; the `ndtr`/`erfc` routines in Cephes; and
  Boost.Math's normal distribution and error-function documentation.
- fdlibm `__ieee754_log`, and Muller, *Elementary Functions: Algorithms and Implementation* —
  the accuracy of the logarithm the composition depends on.
- Knüsel, and McCullough and Wilson, on the accuracy of statistical distributions in
  spreadsheets; and Morten Welinder's work on Gnumeric's statistical functions. Named as
  published literature about the family, not as evidence about this surface.
- Handbook evidence records `EV-DIST-0020` (projection gap; subjects include this surface) and
  `EV-DIST-0021` (substrate identification; group-scoped count over the two LOGNORM spellings,
  attribution "named but not measured").
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.LOGNORM.DIST.json` (arity exactly 4,
  `xlfLognorm_dist`) and `data/presence/FUNC.LOGNORM.DIST.json` (module `normal_log_family.rs`,
  shared with the whole `NORM`/`LOGNORM`/`CONFIDENCE` group).
