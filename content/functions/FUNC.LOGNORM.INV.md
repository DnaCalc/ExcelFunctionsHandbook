---
schema: efh.function-page/v1
function_id: FUNC.LOGNORM.INV
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
family: normal_log_family
role_in_family: >-
  The lognormal quantile function: the probit composed with an exponential, and the family
  member where a small error in the inverse normal becomes a relative error in the answer.
---

# LOGNORM.INV

## What it computes

`LOGNORM.INV(probability, mean, standard_dev)` returns the **quantile** of the lognormal
distribution: the value `x` for which `P(X ≤ x) = p`.

Because the lognormal CDF is `F(x) = Φ((ln x − μ)/σ)`, inverting is a one-line exercise:

    F(x) = p
    ⇔  (ln x − μ)/σ = Φ⁻¹(p)
    ⇔  x = exp( μ + σ·Φ⁻¹(p) )

so

    LOGNORM.INV(p, μ, σ) = exp( μ + σ·z_p ),   z_p = Φ⁻¹(p)

where `Φ⁻¹` is the standard normal quantile function, universally called the **probit**. The
function is therefore an exponential of an affine transform of the probit, and everything
interesting about it — its accuracy, its failure modes, its extreme-value behaviour — is
inherited from `Φ⁻¹`.

**Domain.** `0 < p < 1` strictly, `σ > 0`, `μ` any finite real. **Range:** `(0, ∞)`. The
function is strictly increasing in `p`, with

    p → 0⁺  ⇒  x → 0        p → 1⁻  ⇒  x → ∞
    p = ½   ⇒  x = e^μ      (the median, independent of σ)

The last of those is worth remembering: at the median the probit is exactly zero, so the answer
is `exp(μ)` regardless of `σ`. It is the cleanest single test case the function has.

**The exact inverse relationship** with [LOGNORM.DIST](FUNC.LOGNORM.DIST.md) holds in
mathematics:

    LOGNORM.INV( LOGNORM.DIST(x, μ, σ, TRUE), μ, σ ) = x
    LOGNORM.DIST( LOGNORM.INV(p, μ, σ), μ, σ, TRUE ) = p

In floating point neither round trip can be exact, and how much they lose is a measurable
property of the pair. See Numerical notes.

**Quantiles of a lognormal are multiplicative.** Since `x_p = e^μ · e^{σ z_p}`, the ratio of
two quantiles depends only on `σ`: `x_{p₂}/x_{p₁} = exp(σ(z_{p₂} − z_{p₁}))`. That is the
structural reason the lognormal is the natural model for quantities that vary by percentages —
incomes, particle sizes, failure times, asset prices under geometric Brownian motion, where
`LOGNORM.INV` is precisely the inverse-transform sampler.

## Arguments

| Argument | Meaning | Admissible values |
|---|---|---|
| `probability` | `p`, the cumulative probability | `0 < p < 1`; required |
| `mean` | `μ`, the mean of `ln X` | Any finite real; required |
| `standard_dev` | `σ`, the standard deviation of `ln X` | `σ > 0`; required |

The reference engine declares an arity of exactly 3. All three are scalar numeric slots
governed by ordinary to-number coercion — see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

As on [LOGNORM.DIST](FUNC.LOGNORM.DIST.md), `mean` and `standard_dev` describe **`ln X`**, not
`X`. Feeding this function the sample mean and sample standard deviation of untransformed data
is the standard misuse and produces a plausible-looking wrong answer with no diagnostic.

The endpoints are genuinely excluded, not merely awkward. `p = 0` would require `Φ⁻¹(0) = −∞`
and `p = 1` would require `+∞`; neither has a finite answer, so both are domain errors rather
than infinities.

## Result and edge cases

Returns `Number`, always strictly positive when it returns at all.

- **`p` at or outside the open unit interval** → `#NUM!`. The reference engine's own battery —
  OxFunc's answers, no Excel involved — returns `#NUM!` for its `p = 0`, `p = 1` and negative
  rows, consistent with the documented domain.
- **`σ ≤ 0`** → `#NUM!`.
- **`p` extremely small** — the probit falls like `−√(2·ln(1/p))`, so `z_p` reaches about `−38`
  at the smallest positive normal double and about `−39` at the smallest subnormal. The answer
  is then `exp(μ − 38σ)`, which underflows to zero for quite ordinary `σ`. A returned zero is a
  correct underflow, not an error, and it is indistinguishable from a value that genuinely
  rounds to zero.
- **`p` extremely close to 1** — the mirror case. Here the *representation of `p`* is the
  binding limit, not the algorithm: the doubles just below 1 are spaced `2⁻⁵³` apart, so the
  largest `p` distinguishable from 1 is `1 − 2⁻⁵³` and the largest reachable quantile is
  `exp(μ + σ·Φ⁻¹(1 − 2⁻⁵³)) ≈ exp(μ + 8.13σ)`. Anything beyond that is unreachable through
  this interface no matter how good the implementation is. This is the reason survival
  analysis and reliability work want a complementary-probability entry point, and Excel does
  not offer one for the lognormal.
- **`σ` very large** — `exp(μ + σ z_p)` overflows to `#NUM!` for `|z_p|σ` above about 709. The
  boundary is a legitimate parameter region, and where exactly it falls is unverified.
- **Subnormal arguments** — the reference engine's battery returns a finite value when all
  three arguments are the smallest subnormal, which is what the algebra predicts (`σ·z_p`
  underflows to zero and the exponential of a near-zero argument is 1). It is a useful
  no-intermediate-overflow check.
- **Arrays** in any slot: the reference engine's battery returns `#VALUE!` for a
  two-dimensional inline array literal rather than lifting the function elementwise. Modern
  Excel lifts scalar-slot arguments, so this is a **divergence candidate between the documented
  dynamic-array behaviour and the reference engine's current answer**, recorded as a finding.
  Nobody has checked it against Excel.
- **Error values** in any argument propagate.

## Errors

| Error | Condition | Basis |
|---|---|---|
| `#NUM!` | `probability ≤ 0` or `probability ≥ 1`; or `standard_dev ≤ 0` | Documented by Microsoft on the `LOGNORM.INV` page |
| `#NUM!` | Overflow of `exp(μ + σ z_p)` | Follows from the formula; the exact boundary is unverified |
| `#VALUE!` | Any argument is non-numeric | Documented; also the shared coercion rule |
| propagated | An error value in any argument | Shared coercion rule, chapter 02 |

Retrieval of Microsoft's `LOGNORM.INV` page was blocked for this curation pass, so the
documented rows are stated as documented behaviour with the source named rather than
transcribed. Nobody has checked any of it against Excel within the Handbook's record.

## Relationships

- **[LOGNORM.DIST](FUNC.LOGNORM.DIST.md)** — the forward distribution, and the exact inverse.
  Note the argument-count asymmetry: the forward surface takes a `cumulative` flag and the
  inverse does not, because only the CDF can be inverted. A density has no quantile.
- **[LOGINV](FUNC.LOGINV.md)** — the legacy spelling, retained in the Compatibility category
  with the same three arguments. The documented relationship is that `LOGINV(p, μ, σ)` and
  `LOGNORM.INV(p, μ, σ)` compute the same quantity. **Documented equivalence is not
  demonstrated identity of computation.** The two are separate registered surfaces and may
  reach the answer by different code; establishing that they return the same bits requires
  evidence, and the Handbook has none for either surface. This is the cleanest
  legacy/modern pair in the family to test, because the signatures are identical.
- **[NORM.INV](FUNC.NORM.INV.md)** and **[NORM.S.INV](FUNC.NORM.S.INV.md)** — the identities
  `LOGNORM.INV(p, μ, σ) = EXP(NORM.INV(p, μ, σ))` and
  `= EXP(μ + σ·NORM.S.INV(p))` hold exactly in mathematics. All three surfaces share an
  implementing module in the reference engine, which makes agreement plausible and unproven.
- **[NORM.S.INV](FUNC.NORM.S.INV.md)** deserves separate mention as the accuracy bottleneck:
  every digit `LOGNORM.INV` can deliver comes from the probit.
- **[RAND](FUNC.RAND.md)** — `LOGNORM.INV(RAND(), μ, σ)` is the standard inverse-transform
  lognormal sampler, and the reason the extreme-tail behaviour above is not academic: a
  simulation with a million draws will visit `p` values within `10⁻⁶` of both endpoints.
- **Confused with:** the mean and standard deviation of the data rather than of its logarithm;
  and with `LOGNORM.DIST(..., FALSE)`, which is a density and not an inverse of anything.

## Numerical notes

The mathematics is three operations. The accuracy question is entirely about the first.

**The probit is the hard part.** `Φ⁻¹(p)` has no closed form. The implementations that matter,
in increasing order of accuracy:

- **Hastings' rational approximation**, published as Abramowitz and Stegun 26.2.22 and 26.2.23,
  giving about `4.5·10⁻⁴` and `3·10⁻⁴` absolute error respectively. These are trivially cheap
  and were embedded in an enormous amount of mid-century statistical software. Three or four
  correct digits.
- **Beasley and Springer (AS 111, 1977)** with Moro's tail refinement — about seven to nine
  digits, and the version most often found in quantitative finance code.
- **Wichura's AS 241 (1988)**, the `PPND16` routine: two rational approximations in `r =
  √(−ln q)` covering the central and tail regions, delivering close to full double precision
  across the representable range. This is the modern reference and what a careful
  implementation uses.
- **Newton or Halley refinement** of any of the above against a high-quality `Φ`, which
  converges quadratically or cubically and can polish a nine-digit start to full precision in
  one or two steps — at the cost of needing an accurate forward `Φ` and its derivative.

The published critique record on spreadsheet statistics is directly relevant here: Knüsel's
assessments of statistical distribution accuracy, and McCullough and Wilson's *On the accuracy
of statistical procedures in Microsoft Excel*, both singled out the inverse normal as a
low-accuracy routine in the Excel versions they examined, and tracked changes across releases.
Morten Welinder's work on Gnumeric's statistical functions covers the same ground from the
reimplementer's side. **The Handbook does not assert what any Excel build computes
internally**, and it has run none of those tests itself.

**Error propagation through the exponential.** Let `δ` be the absolute error in `z_p`. Then

    x̂ = exp(μ + σ(z_p + δ)) = x·exp(σδ)  ⇒  relative error ≈ σ·δ

So the **absolute** error of the probit becomes a **relative** error of the result, amplified
by `σ`. A probit good to `3·10⁻⁴` — the Hastings level — gives a lognormal quantile good to
about `0.03%` when `σ = 1`, and to about `1.5%` when `σ = 50`. That is the entire argument for
using AS 241 rather than a cheap rational fit, stated quantitatively.

**Where it is worst.** Both the probit's own difficulty and the amplification factor grow in
the tails: `z_p` is steep near the endpoints (`dz/dp = 1/φ(z)`, which blows up), and `σ|z_p|`
is large exactly there. The far tail of `LOGNORM.INV` is therefore doubly disadvantaged, and it
is also where simulation users spend their time.

**What a careful implementation does.** Use AS 241 or an equivalently accurate probit; compute
`μ + σ z_p` in one fused step if available, since the addition can cancel when `μ` and `σz_p`
have opposite signs and comparable size; and use a correctly rounded `exp` (fdlibm's
`__ieee754_exp` and descendants). Do not compute the quantile by bisecting the forward CDF — a
common shortcut that inherits the forward function's tail saturation and cannot reach the
extremes at all.

## What has not been checked

**No evidence record lists `FUNC.LOGNORM.INV` among its subjects.** Records exist for the
forward surface `LOGNORM.DIST`; none of them names the inverse, and this page claims none of
them. Nobody has checked `LOGNORM.INV` against Excel within the Handbook's record.

No Handbook vector suite exists. There is no residual plate, no probit-accuracy
characterisation, and no comparison against `LOGINV`.

One divergence candidate found during this pass, recorded as a finding: the reference engine's
own battery returns `#VALUE!` for a two-dimensional inline array literal in the argument slots
rather than lifting elementwise, which sits awkwardly with modern Excel's scalar-slot lifting.
Scope and cause unestablished.

Inputs I would probe first, and why:

1. **`p = 0.5`** for several `μ` and `σ`. The answer must be exactly `EXP(μ)` — the probit is
   exactly zero there, so any deviation is a direct reading of implementation error at the
   easiest point in the domain. If this fails, nothing else needs testing.
2. **The round trip** `LOGNORM.DIST(LOGNORM.INV(p, μ, σ), μ, σ, TRUE)` against `p`, swept
   across `p`. This needs no external oracle and produces an accuracy plate for the pair. The
   place it degrades tells you which of the two surfaces is weak.
3. **`p` at `10⁻³`, `10⁻⁶`, `10⁻¹⁰`, `10⁻¹⁰⁰`, and at `1 − 2⁻⁵³`** — the probit tail sweep. A
   Hastings-class approximation and an AS 241-class one separate visibly by `p = 10⁻⁶` and
   dramatically by `10⁻¹⁰⁰`, so this probe *identifies the algorithm class* without any source
   access. It is the highest-information experiment on this page.
4. **`LOGNORM.INV(p, μ, σ)` against `EXP(NORM.INV(p, μ, σ))` and against
   `EXP(μ + σ·NORM.S.INV(p))`**, bit for bit. Three routes to one number; disagreement
   localises which surface has its own path.
5. **`LOGNORM.INV` against `LOGINV`** on the same grid, bit for bit — the legacy/modern
   identity question, which is currently documented and unproven.
6. **`σ` large enough to overflow** — `μ = 0`, `σ = 200`, `p = 0.999` — to find the `#NUM!`
   boundary and check that overflow is diagnosed rather than returned as infinity.
7. **Monotonicity sweep**: a dense grid of `p`, checking that the returned sequence is
   non-decreasing. A quantile function that is not monotone is broken in a way no single point
   test reveals, and rational approximations with region switches are exactly where
   non-monotonicity appears — at the join.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| probit | `Φ⁻¹`, the standard normal quantile function |
| quantile | The value `x` with `P(X ≤ x) = p` |
| AS 241 | Wichura's 1988 algorithm for the probit; the near-full-precision reference |
| Hastings approximation | The low-accuracy rational fit of A&S 26.2.22/26.2.23 |
| amplification factor | `σ`, converting absolute probit error into relative quantile error |
| endpoint reachability | The largest `p < 1` representable as a double bounds the reachable upper quantile |
| legacy alias | `LOGINV`; documented as the same quantity, not demonstrated to be the same computation |

## Sources

- Microsoft, "LOGNORM.INV function" —
  <https://support.microsoft.com/en-us/office/lognorm-inv-function-fe79751a-f1f2-4af8-a0a1-e151b2d4f600>
  (signature and the `#NUM!` / `#VALUE!` conditions). Retrieval was blocked for this pass;
  documented rows are stated as documented behaviour with the source named.
- Abramowitz and Stegun, *Handbook of Mathematical Functions*, chapter 26 — the normal
  distribution, and 26.2.22/26.2.23 for the Hastings inverse approximations with their stated
  error bounds.
- Wichura, *Algorithm AS 241: The Percentage Points of the Normal Distribution*, Applied
  Statistics 37 (1988) — the `PPND16` reference implementation of the probit.
- Beasley and Springer, *Algorithm AS 111* (1977), with Moro's tail refinement — the
  intermediate-accuracy route common in finance code.
- Cody's `erf`/`erfc` work (Math. Comp. 1969; SPECFUN) and Boost.Math's normal-distribution
  documentation — the forward function any Newton refinement would need.
- fdlibm `__ieee754_exp` and its glibc/musl descendants; Muller, *Elementary Functions*.
- Knüsel, and McCullough and Wilson, on statistical distribution accuracy in spreadsheets; and
  Morten Welinder's work on Gnumeric's statistical functions. Named as published literature
  about the family, not as evidence about this surface.
- Handbook [LOGNORM.DIST](FUNC.LOGNORM.DIST.md) — the forward distribution and its evidence
  records, none of which name this surface.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.LOGNORM.INV.json` (arity exactly 3,
  `xlfLognorm_inv`) and `data/presence/FUNC.LOGNORM.INV.json` (module `normal_log_family.rs`,
  shared with the whole `NORM`/`LOGNORM`/`CONFIDENCE` group).
