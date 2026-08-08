---
schema: efh.function-page/v1
function_id: FUNC.POISSON.DIST
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0030
  - EV-DIST-0033
open_problems: []
references:
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 26, 26.4.21 and 26.1.22 (Poisson sums and the incomplete gamma / chi-square identity)"
    role: "The identity turning the Poisson CDF into a regularized incomplete gamma function"
  - work: "C. Loader, Fast and Accurate Computation of Binomial Probabilities"
    locator: "2000; the saddle-point dpois/dbinom used by R"
    role: "The modern accurate route to discrete densities, avoiding overflow and cancellation"
  - work: "A. R. DiDonato and A. H. Morris, Algorithm 654: computation of the incomplete gamma function ratios"
    locator: "ACM TOMS 12 (1986) 377-393 (GRATIO)"
    role: "The reference implementation of the incomplete gamma ratios behind the Poisson CDF"
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
family: discrete_dist_family
role_in_family: >-
  The Poisson member of the discrete distribution family; the surface whose k=0 window is the
  best-evidenced point anywhere in the family and whose behaviour away from that window is
  openly worse.
---

# POISSON.DIST

## What it computes

`POISSON.DIST(x, mean, cumulative)` evaluates the Poisson distribution with expectation `λ` at
the count `x`, returning either the probability of exactly `x` events or the probability of at
most `x` events.

Writing `k` for the truncated `x` and `λ` for `mean`:

**Probability mass function** (`cumulative = FALSE`) — the probability of exactly `k` events:

    p(k; λ) = e^(−λ) · λ^k / k!,        k = 0, 1, 2, …

**Cumulative distribution function** (`cumulative = TRUE`) — Microsoft's page describes this as
"the cumulative Poisson probability that the number of random events occurring will be between
zero and x inclusive":

    F(k; λ) = Σ_{j=0}^{k} e^(−λ) · λ^j / j!

**Domain and range.** The documented domain is `k ≥ 0` (integer, after truncation) and `λ ≥ 0`.
Note that `λ = 0` is admissible — the documented `#NUM!` condition is `mean < 0`, not
`mean ≤ 0` — and at `λ = 0` the distribution is degenerate at zero, so `p(0; 0) = 1` and
`p(k; 0) = 0` for `k > 0`. The pmf takes values in `(0, 1]` and the CDF in `(0, 1]`, with
`F(k; λ) → 1` as `k → ∞`.

**The identity that makes this function tractable.** The Poisson tail sum is a regularized
incomplete gamma function, and A&S 26.4.21 states the equivalent relation through the
chi-square distribution:

    F(k; λ) = Σ_{j=0}^{k} e^(−λ)λ^j/j!  =  Q(k+1, λ)  =  Γ(k+1, λ) / Γ(k+1)

where `Q` is the **upper** regularized incomplete gamma function. Equivalently, in Excel's own
vocabulary:

    POISSON.DIST(k, λ, TRUE)  =  CHISQ.DIST.RT(2λ, 2(k+1))

That is not a curiosity. It is the reason a Poisson CDF need not be a loop, it is what any
serious implementation actually computes, and it is a cross-surface identity that makes an
excellent oracle-free consistency test between two Excel functions that have no obvious reason
to share code.

**Structural identities and limits.**

    p(0; λ) = e^(−λ)                     the k = 0 window
    p(k; λ) = p(k−1; λ) · λ / k          the recurrence
    mean = variance = λ
    mode = ⌊λ⌋   (and both ⌊λ⌋ and λ−1 when λ is an integer)
    Poisson(λ) → Normal(λ, λ)            as λ → ∞ (with a continuity correction)
    Binomial(n, λ/n) → Poisson(λ)        as n → ∞

The `k = 0` row deserves separate notice because it is where the evidence on this surface
concentrates: at `k = 0`, `λ^0 = 1` and `0! = 1`, so the pmf collapses to a bare `e^(−λ)` and
the cumulative equals it. **Both branches reduce to the exponential.** That makes `k = 0`
simultaneously the easiest input to get right and the least informative about how the function
is built — a point the attached evidence record makes about itself, in its own words, and one
this page will not soften.

## Arguments

Microsoft's page gives three required arguments:

| Argument | Meaning | Required |
|---|---|---|
| `x` | "The number of events" | yes |
| `mean` | "The expected numeric value" | yes |
| `cumulative` | A logical: `TRUE` gives the probability of between zero and `x` inclusive; `FALSE` gives "the Poisson probability mass function that the number of events occurring will be exactly x" | yes |

**`x` is truncated**: Microsoft states "If x is not an integer, it is truncated." So
`POISSON.DIST(2.9, λ, c)` is the same call as `POISSON.DIST(2, λ, c)`. `mean` is *not*
truncated — `λ` is a genuine real parameter.

`cumulative` has no default; the switch must be written.

The reference engine records an arity of exactly 3 and a `Custom` coercion/lift profile, and
marks the projected signature as a **placeholder** (`signature_placeholder: true`) — the table
above is Microsoft's.

## Result and edge cases

Returns `Number`.

- **`λ = 0`** is admissible per the documented `mean < 0` boundary and yields the degenerate
  distribution at zero. The reference engine's battery renders the all-zero row and its outcome
  shows beside this page.
- **Truncation before validation.** `x` is truncated first, so `POISSON.DIST(0.9, λ, c)` is the
  `k = 0` case and `POISSON.DIST(−0.5, λ, c)` truncates toward zero to `0` rather than becoming
  a negative count — a boundary worth probing rather than assuming.
- **Large `λ`.** `e^(−λ)` underflows below the smallest normal double once `λ` passes about
  `708`, and to zero once `λ` passes about `745`. But `p(k; λ)` for `k` near `λ` is not small at
  all — it is about `1/√(2πλ)`. **A naive `e^(−λ)·λ^k/k!` therefore computes a perfectly
  ordinary probability as the product of an underflowed zero and an overflowed infinity.** This
  is the central implementation hazard of the whole discrete-distribution family and it is why
  the Numerical notes below are the substance of this page.
- **Large `k`.** `k!` overflows in binary64 at `k = 171`, so any staging that forms the
  factorial explicitly fails there regardless of `λ`.
- **CDF saturation.** For `k` well above `λ` the CDF rounds to exactly `1` and all upper-tail
  information is lost. Excel offers no right-tailed Poisson surface, but the incomplete-gamma
  identity above gives one: the upper tail is `1 − F(k; λ) = P(k+1, λ)`, reachable through
  `CHISQ.DIST` rather than by subtraction.
- **Arrays.** The recorded profile is `Custom`; elementwise lifting is not settled here.

Empty, missing and error arguments follow the shared call model; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

As documented by Microsoft on the `POISSON.DIST` page (retrieved for this page):

| Error | Documented condition |
|---|---|
| `#VALUE!` | `x` or `mean` is nonnumeric |
| `#NUM!` | `x < 0` |
| `#NUM!` | `mean < 0` |

Note that neither `#NUM!` row excludes zero: `x = 0` and `mean = 0` are both documented as
admissible, and this is a real difference from several neighbouring distribution pages where
the analogous condition is written with `≤`. Note also that the `#VALUE!` row does not mention
`cumulative`.

Error values in any argument propagate under the ordinary coercion rules.

## Relationships

- **`POISSON`** is the legacy spelling, with the same argument count in the reference engine's
  registry — three, including `cumulative`. So this modernization is a **pure rename at the
  signature level**, unlike `NEGBINOMDIST → NEGBINOM.DIST` (which gained an argument) or
  `NORMSDIST → NORM.S.DIST` (likewise). **Same signature is not same computation.**

  The Handbook can be precise about the gap here, because one of the evidence records attached
  to this page exists for exactly that purpose. `EV-DIST-0033` is an alias-pairing record
  covering `WEIBULL`/`WEIBULL.DIST`, `EXPONDIST`/`EXPON.DIST` and `POISSON`/`POISSON.DIST`, and
  its finding is that **the sign-off measurements in this cluster were all taken on the modern
  surfaces by name, and nothing measures the legacy spellings separately.** Its reader warning
  states that no figure may be inherited onto the legacy surface, and it records the pointed
  contrast that a legacy-modern collapse *was* measured on the gamma side for five other pairs —
  so the distinction is a real one, not pedantry. Whether Excel's `POISSON` and `POISSON.DIST`
  publish the same bits has, on this record, never been probed.
- **Siblings in the reference engine's implementing module**: `BINOM.DIST`, `BINOM.DIST.RANGE`,
  `BINOM.INV`, `EXPON.DIST`, `HYPGEOM.DIST`, `NEGBINOM.DIST` and their legacy spellings. That
  is a statement about the reference engine's structure and none at all about Excel's.
- **`CHISQ.DIST.RT`** is the cross-surface identity partner:
  `POISSON.DIST(k, λ, TRUE) = CHISQ.DIST.RT(2λ, 2(k+1))` (A&S 26.4.21). Two Excel functions
  computing the same number by different routes is a free consistency oracle.
- **`GAMMA.DIST`** carries the same incomplete gamma from the other side:
  `POISSON.DIST(k, λ, TRUE) = 1 − GAMMA.DIST(λ, k+1, 1, TRUE)`.
- **`EXPON.DIST`** is the waiting-time dual: inter-arrival times of a Poisson process with rate
  `λ` are `Exponential(λ)`, and `POISSON.DIST(0, λt, FALSE) = 1 − EXPON.DIST(t, λ, TRUE)`.
- **`BINOM.DIST`** converges to this function in the rare-event limit, and
  **[NEGBINOM.DIST](FUNC.NEGBINOM.DIST.md)** is its overdispersed generalization — the Poisson
  is the `r → ∞` limit of the negative binomial at fixed mean.
- **`NORM.DIST`** approximates it for large `λ`, with a continuity correction.
- **Confused with**: the exponential distribution (times, not counts), and `POISSON.DIST` called
  with `x` as a rate rather than a count.

## Numerical notes

This is one of the genuinely hard functions in the statistical category, and the difficulty is
concentrated in the pmf rather than in the sum.

**1. The naive formula fails for ordinary inputs, not exotic ones.** `e^(−λ)λ^k/k!` computes a
probability of order `1/√(2πλ)` — a perfectly representable number near `0.04` at `λ = 100` —
as the product of a factor that underflows, a factor that overflows, and a factor that
overflows. At `λ = 1000, k = 1000` every one of the three intermediates is out of range while
the answer is about `0.0126`. **The formula is not wrong; it is unevaluable as written.**

**2. Log-space is the obvious fix and is not good enough.** Writing

    p(k; λ) = exp( k·ln λ − λ − lnΓ(k+1) )

fixes the range problem and creates an accuracy problem: for `k ≈ λ` the three terms in the
exponent are individually large and nearly cancel, so the exponent is a small difference of
large quantities. An absolute error `ε` in the exponent becomes a relative error `ε` in the
answer; with `k ≈ λ ≈ 10⁶` the terms are of order `10⁷` and double rounding alone gives an
exponent error near `10⁻⁹`. The result loses roughly as many digits as the exponent has.

**3. The saddle-point form is the modern answer.** Loader's method (2000) rewrites the pmf so
that the cancellation is performed analytically rather than numerically:

    p(k; λ) = exp( −stirlerr(k) − D₀(k, λ) ) / √(2πk)

where `stirlerr(k) = lnΓ(k+1) − [k ln k − k + ½ln(2πk)]` is the *Stirling correction* (small,
tabulated for small `k`, expanded asymptotically for large `k`) and

    D₀(k, λ) = k·ln(k/λ) + λ − k

is the *deviance term*, evaluated with a series in `(λ−k)/(λ+k)` precisely so that no
cancellation occurs when `k ≈ λ`. This is what R's `dpois` and `dbinom` use, and it is the
standard against which any new implementation should be measured. The evidence record attached
to this page reports that Loader's `dpois` reproduces Excel above a stated `λ` threshold and
does **not** at `k = 1`, which is itself a finding about Excel's staging.

**4. The recurrence is the cheap route and it is conditionally stable.** `p(k) = p(k−1)·λ/k`
running upward from `p(0) = e^(−λ)` is exact-ish while the terms increase (`k < λ`) and
accumulates relative error benignly; running it downward from the mode is the stable direction
for the far right tail. Starting from `e^(−λ)` is only viable while `e^(−λ)` is representable,
so the recurrence and the saddle point are complementary rather than competing.

**5. The CDF should not be a loop.** Summing `k+1` pmf terms costs `O(k)` and accumulates `O(k)`
roundings; for `k` in the millions it is both slow and inaccurate. The right computation is the
regularized incomplete gamma `Q(k+1, λ)`, evaluated by the series for `λ < k+1` and by the
continued fraction otherwise — the classical Temme/DiDonato–Morris switch. **DiDonato and
Morris, ACM TOMS Algorithm 654 (`GRATIO`)** is the reference implementation, and it is not a
coincidence that the upstream identification note behind this page's evidence is named for the
gamma ratio: the whole discrete-distribution family bottoms out in the same incomplete gamma
machinery.

**6. Where the `k = 0` window sits in all this.** At `k = 0` the deviance term, the Stirling
correction and the log-gamma all vanish, and every route — direct product, log-composed,
saddle-point — reduces to `e^(−λ)`. That is why the evidence record attached to this page
insists that its `k = 0` gate is **route-blind**: it validates the exponential subexpression and
can say nothing about which pmf staging is in use. The record states this as a method lesson in
its own words, having withdrawn an earlier claim that read the window the other way. It is a
good lesson and it generalizes: a window that publishes a common subexpression proves the
subexpression, never the route.

**What a careful independent implementation does**: saddle-point pmf with a tabulated Stirling
correction and a cancellation-free deviance; incomplete-gamma CDF with a series/continued-fraction
switch; explicit handling of `λ = 0` and `k = 0`; and a stated bound. See
[About implementation options](../model/07-implementation-options.md).

## What has not been checked

Two evidence records name this surface and they say almost opposite things, which is why both
are attached.

`EV-DIST-0030` is a substrate identification with a **genuinely held-out gate** — and a very
narrow one. Its scope, in the record's own words, is the `POISSON.DIST(0, λ)` input window: the
`k = 0` case, where the mass is `e^(−λ)`. The record states in its own reader warning that the
gate covers that window only, that it cannot discriminate the pmf route (for the reason given
in Numerical notes point 6), and that a larger total appearing in the upstream source belongs to
an internal exponential primitive rather than to this surface. It further records that away from
`k = 0` the surface is **materially worse and openly so**, that a staged model at `k = 1` was
scored well below the `k = 0` window, and that at small `λ` for all `k` neither identified model
matches as staged. The record's figures render mechanically beside this page with their own
scopes; this page transcribes none of them.

`EV-DIST-0033` is the alias-pairing record described under Relationships: `POISSON.DIST` appears
in it as the *contrast* subject, and its finding is that the legacy `POISSON` has no measurement
of its own and may not inherit this surface's.

So: the `k = 0` window is the best-evidenced point in this part of the family, and the rest of
the surface is open. No Handbook vector suite exists for `POISSON.DIST`. Both records are
upstream OxFunc work that the Handbook has not re-verified, and neither is a Handbook
measurement.

Microsoft's documented behaviour above was retrieved from the `POISSON.DIST` page. The equations
on that page are published as images and could not be read as text; the formulas in this page's
first section are the standard mathematical definitions, stated as mathematics, not transcribed
from Microsoft.

Inputs worth probing first:

1. **`POISSON.DIST(k, λ, TRUE)` against `CHISQ.DIST.RT(2λ, 2(k+1))`** across a wide grid.
   Two Excel surfaces, one number, no external oracle, and any disagreement is immediately a
   finding about one of them. This is the best experiment available on this page and it costs
   two columns.
2. **`k = 1` at a spread of `λ`.** The record names `k = 1` as the place where the identified
   `k = 0` route stops explaining the answers. Whatever is happening to this surface is
   happening there, and it is the natural next window.
3. **Small `λ` across all `k`** — the region the record states no staged model matches. If a
   route is to be identified, this is the awkward part.
4. **`k ≈ λ` at increasing scale**: `(k, λ) = (100, 100)`, `(10⁴, 10⁴)`, `(10⁶, 10⁶)`. This is
   the cancellation regime of Numerical notes point 2, and the rate at which accuracy degrades
   distinguishes a log-composed staging from a saddle-point one without any need to guess.
5. **The overflow/underflow corners**: `λ` near `708` and `745` with `k = 0`; `k` at `170`,
   `171` and `172` at moderate `λ`. These find any residual factorial or bare-exponential
   staging in a handful of cells.
6. **`λ = 0` with `k = 0` and `k = 1`**, and `x` at `0.9` and `−0.5`, confirming the documented
   inclusive `mean < 0` boundary and the truncation direction.
7. **`POISSON.DIST` against `POISSON`** on every probe above. `EV-DIST-0033` says plainly that
   nobody has done this; it is one extra column and it converts an assumption into a finding.
8. **`cumulative` as `0`, `1`, `2`, text `"TRUE"`, and empty**, given the `Custom` coercion
   profile.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| `k = 0` window | Inputs with `x` truncating to zero, where both branches reduce to `e^(−λ)` |
| route-blind | A test window whose value is common to the competing stagings, so it cannot tell them apart |
| deviance term | `D₀(k, λ) = k ln(k/λ) + λ − k`, evaluated cancellation-free in the saddle-point form |
| Stirling correction | `lnΓ(k+1)` minus its Stirling approximation; small, tabulated or expanded |
| regularized incomplete gamma | `Q(a, x) = Γ(a, x)/Γ(a)`; the closed form of the Poisson CDF |

## Sources

- Microsoft, "POISSON.DIST function" —
  <https://support.microsoft.com/en-us/office/poisson-dist-function-8fe148ff-39a2-46cb-abf3-7772695d9636>
  (syntax; the three required arguments and their descriptions, including the `cumulative`
  wording; "If x is not an integer, it is truncated"; the `#VALUE!` condition on `x` or `mean`;
  and the `#NUM!` conditions `x < 0` and `mean < 0`). Retrieved for this page; the page's
  equations are images and were not read.
- Handbook evidence record `EV-DIST-0030` — the `k = 0` held-out gate and the open state of the
  surface away from it, with the record's own scope statements, retraction and reader warning.
- Handbook evidence record `EV-DIST-0033` — the alias-pairing record establishing that the
  legacy `POISSON` has no measurement of its own and may not inherit this surface's.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 26 §26.4.21 (the Poisson
  sum as a chi-square probability) and chapter 6 §6.5 (incomplete gamma).
- C. Loader, "Fast and Accurate Computation of Binomial Probabilities" (2000) — the
  saddle-point `dpois`/`dbinom` with the Stirling correction and deviance term.
- A. R. DiDonato and A. H. Morris, "Algorithm 654: FORTRAN subroutines for computing the
  incomplete gamma function ratios and their inverses", *ACM TOMS* 12 (1986) 377–393 —
  `GRATIO`, the reference incomplete-gamma implementation.
- N. M. Temme, "A set of algorithms for the incomplete gamma functions", *Probability in the
  Engineering and Informational Sciences* 8 (1994) — the uniform asymptotics behind the
  series/continued-fraction switch.
- B. D. McCullough and B. Wilson, *Computational Statistics & Data Analysis* (1999, 2002,
  2005); L. Knüsel on Excel's distribution functions; M. Welinder's Gnumeric statistical work.
  Named as literature; no figure restated.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [About implementation options](../model/07-implementation-options.md).
- `data/functions/FUNC.POISSON.DIST.json` (arity 3, `Custom` profile,
  `signature_placeholder: true`, XLL symbol `xlfPoisson_dist`) and
  `data/functions/FUNC.POISSON.json` (arity 3) — the source of the unchanged-signature
  observation.
- `data/presence/FUNC.POISSON.DIST.json` — implementing module
  `crates/oxfunc_core/src/functions/discrete_dist_family.rs`, shared with twelve other discrete
  distribution surfaces.
