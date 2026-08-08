---
schema: efh.function-page/v1
function_id: FUNC.NEGBINOM.DIST
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0029
open_problems: []
references:
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 26, 26.5.24-26.5.27 (negative binomial sums and the incomplete beta)"
    role: "The identity turning the negative binomial CDF into a regularized incomplete beta function"
  - work: "C. Loader, Fast and Accurate Computation of Binomial Probabilities"
    locator: "2000; the saddle-point dbinom / dnbinom used by R"
    role: "The modern accurate route to discrete densities; named as ruled out for this surface"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - The rename that added an argument
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: discrete_dist_family
role_in_family: >-
  The negative binomial member of the discrete distribution family; the surface whose upstream
  route hunt established that it does not inherit the binomial's identified route, and whose
  own route remains unidentified.
---

# NEGBINOM.DIST

## What it computes

`NEGBINOM.DIST(number_f, number_s, probability_s, cumulative)` evaluates the negative binomial
distribution: the number of **failures** observed before a fixed number of **successes** is
reached, in independent Bernoulli trials.

Writing `x` for the truncated `number_f`, `r` for the truncated `number_s`, `p` for
`probability_s` and `q = 1 − p`:

**Probability mass function** (`cumulative = FALSE`) — the probability of exactly `x` failures
before the `r`-th success:

    f(x; r, p) = C(x + r − 1, x) · p^r · q^x
               = Γ(x + r) / ( Γ(r) · x! ) · p^r · q^x,      x = 0, 1, 2, …

The combinatorial factor counts the arrangements of `x` failures among the first `x + r − 1`
trials; the final trial is by construction the `r`-th success, which is why the binomial
coefficient uses `x + r − 1` and not `x + r`. Getting that off by one wrong is the classic
error in implementing this distribution.

**Cumulative distribution function** (`cumulative = TRUE`) — the probability of at most `x`
failures:

    F(x; r, p) = Σ_{j=0}^{x} f(j; r, p)  =  I_p(r, x + 1)

where `I_p(a, b)` is the **regularized incomplete beta function**. That closed form (A&S
26.5.24 and the neighbouring relations) is what makes the CDF computable in constant time
rather than as a sum, and it is the same machinery the binomial and `F` distributions bottom out
in.

**Domain and range.** The documented domain is `x ≥ 0` and `r ≥ 1`, both after truncation to
integers, with `0 ≤ p ≤ 1`. The mass is in `[0, 1]` and sums to `1` over `x` whenever `p > 0`.

**Moments and limits.**

    mean     = r·q/p
    variance = r·q/p²  =  mean / p          always ≥ mean: the distribution is overdispersed
    f(0; r, p) = p^r
    r = 1                    gives the geometric distribution on {0, 1, 2, …}
    r → ∞, p → 1 with r·q/p → λ fixed   gives Poisson(λ)

The overdispersion is why the negative binomial is used at all: it is the count distribution
for data whose variance exceeds its mean, where Poisson fails. It is also the Gamma–Poisson
mixture — a Poisson whose rate is itself Gamma-distributed — which is the identity that gives
it a continuous `r` in most statistical software.

**The `r` question.** Mathematically the density is written with `Γ(r)` and is perfectly
well defined for any real `r > 0`; that generalization is what makes the Gamma–Poisson mixture
work and is what R's `dnbinom` supports. Excel does not: `number_s` is documented as truncated
to an integer and required to be at least `1`. So Excel's `NEGBINOM.DIST` is the strictly
integer-`r` special case, and a user fitting an overdispersed count model with a fractional
dispersion parameter cannot express it here.

**Degenerate corners.** At `p = 1` every trial succeeds, so `f(0; r, 1) = 1` and `f(x; r, 1) = 0`
for `x > 0`. At `p = 0` no success ever occurs and the distribution does not exist — the mass
is zero for every finite `x` and the total is zero, not one. Microsoft's documented boundary is
`probability_s < 0`, which admits `p = 0`; what the function returns there is a real question
and is on the probe list.

## Arguments

Microsoft's page gives four required arguments:

| Argument | Meaning | Required |
|---|---|---|
| `number_f` | "The number of failures" | yes |
| `number_s` | "The threshold number of successes" | yes |
| `probability_s` | "The probability of a success" | yes |
| `cumulative` | A logical: `TRUE` gives the cumulative distribution function, `FALSE` the mass | yes |

**`number_f` and `number_s` are truncated to integers**, per Microsoft's remark.
`probability_s` is not truncated. `cumulative` has no default.

The reference engine records an arity of exactly 4 and a `Custom` coercion/lift profile, and
marks the projected signature as a **placeholder** (`signature_placeholder: true`) — the table
above is Microsoft's.

## The rename that added an argument

`NEGBINOMDIST → NEGBINOM.DIST` is **not** a rename. The reference engine's registry records
`NEGBINOMDIST` with an arity of exactly **3** and `NEGBINOM.DIST` with an arity of exactly
**4**. The modernization added the `cumulative` switch; the legacy function computes only the
mass.

    NEGBINOMDIST(x, r, p)   corresponds to   NEGBINOM.DIST(x, r, p, FALSE)

and there is no legacy spelling of the cumulative branch at all. Two practical consequences:

- A formula migrated by mechanically inserting dots produces a call with the wrong number of
  arguments, which fails at entry rather than silently.
- More insidiously, a reader who has internalized "the dotted name is the same function" will
  read `NEGBINOM.DIST(x, r, p, TRUE)` as if a legacy equivalent existed and reason about
  legacy behaviour that was never there.

And for the branch that *does* correspond, correspondence is a statement about signatures and
mathematics, not about bits. Excel dispatches compatibility functions as their own surfaces;
proving that `NEGBINOMDIST(x, r, p)` and `NEGBINOM.DIST(x, r, p, FALSE)` publish identical
doubles requires evidence, and the evidence record attached to this page is explicit that the
question of *which* negative binomial surface was even measured upstream is unresolved. See
What has not been checked.

## Result and edge cases

Returns `Number`.

- **`x = 0`** gives `p^r` in both branches — the probability that the first `r` trials are all
  successes. Like the Poisson's `k = 0` window, this is a **route-blind** input: the
  combinatorial factor is `1` and the `q^x` factor is `1`, so every staging reduces to a power.
  It is the easiest input to get right and says the least about how the function is built.
- **`p = 1`** collapses the distribution to a point mass at `x = 0`.
- **`p = 0`** is admitted by the documented boundary and is mathematically degenerate; see
  above.
- **Truncation before validation.** `number_f` and `number_s` are truncated first, so
  `NEGBINOM.DIST(0.9, 1.9, p, c)` is the `(0, 1)` case, and `number_s = 1.5` truncates to the
  admissible `1` rather than being rejected as non-integer.
- **Large `x` and `r`.** The binomial coefficient `C(x+r−1, x)` overflows binary64 long before
  the probability underflows; the two factors move in opposite directions and the naive product
  is unevaluable across a wide, ordinary band of inputs. See Numerical notes.
- **CDF saturation.** For `x` well above the mean the CDF rounds to exactly `1` and the upper
  tail is unrecoverable by subtraction. The incomplete-beta identity gives the complement
  directly: `1 − F(x; r, p) = I_q(x+1, r)`.
- **Arrays.** The recorded profile is `Custom`; elementwise lifting is not settled here.

Empty, missing and error arguments follow the shared call model; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

As documented by Microsoft on the `NEGBINOM.DIST` page (retrieved for this page):

| Error | Documented condition |
|---|---|
| `#VALUE!` | Any argument is nonnumeric |
| `#NUM!` | `probability_s < 0` or `probability > 1` |
| `#NUM!` | `number_f < 0` or `number_s < 1` |

Three defects in that published text are worth recording rather than tidying away:

1. **The probability row names an argument that does not exist.** It reads "if
   `probability_s < 0` or if **`probability` > 1**". There is no `probability` argument; the
   upper-bound condition is written against a name the syntax line never introduces. The
   intended reading is obvious and the published text is wrong.
2. **`p = 0` is admitted by the stated inequality** — the row says `< 0`, not `≤ 0` — even
   though the distribution does not exist at `p = 0`. Either the boundary is deliberate and the
   function returns something at `p = 0`, or the documentation is loose. Nobody has checked.
3. **The `cumulative` argument is described as returning "the probability density function"**
   when `FALSE`. The negative binomial is a *discrete* distribution; what that branch returns is
   a probability **mass** function. This is the mirror image of a defect the Handbook records on
   [NORM.S.DIST](FUNC.NORM.S.DIST.md), where a *continuous* density is called a mass function.
   The two pages have the terminology exactly inverted, each in the wrong direction. The
   computations are not in dispute; the published descriptions of them are.

Error values in any argument propagate under the ordinary coercion rules.

## Relationships

- **`NEGBINOMDIST`** is the legacy spelling with three arguments — see
  [the rename that added an argument](#the-rename-that-added-an-argument) above. This is one of
  the clearest cases in the statistical category that a dotted name is not always a rename.
- **`BINOM.DIST`** is the conjugate question: the binomial fixes the number of *trials* and
  counts successes; the negative binomial fixes the number of *successes* and counts failures.
  The two CDFs are two faces of the same incomplete beta, which is why one might expect them to
  share a route — and why the upstream finding that this surface **does not** inherit the
  binomial's identified route is a genuine result rather than an absence of one.
- **`GEOMETRIC`** does not exist in Excel as its own function; the geometric distribution is
  `NEGBINOM.DIST(x, 1, p, cumulative)`.
- **[POISSON.DIST](FUNC.POISSON.DIST.md)** is the limit at large `r`, and the negative binomial
  is the Gamma–Poisson mixture that generalizes it to overdispersed counts.
- **`BETA.DIST`** carries the same regularized incomplete beta from the continuous side:
  `NEGBINOM.DIST(x, r, p, TRUE) = BETA.DIST(p, r, x+1, TRUE)`. Two Excel surfaces, one number —
  a free consistency oracle, and the analogue of the Poisson/chi-square identity.
- **Siblings in the reference engine's implementing module**: `BINOM.DIST`,
  `BINOM.DIST.RANGE`, `BINOM.INV`, `EXPON.DIST`, `HYPGEOM.DIST`, `POISSON.DIST` and their legacy
  spellings. A statement about the reference engine's structure, not Excel's.
- **Confused with**: the "number of trials" parameterization used by some textbooks and by
  several other statistical packages, in which the variable counts *trials* rather than
  *failures* and everything shifts by `r`. Excel counts failures. A formula ported from another
  system without checking this is off by `r` and will still look plausible.

## Numerical notes

**1. The naive product is unevaluable over an ordinary band.** `C(x+r−1, x) · p^r · q^x` pairs
a combinatorial factor that overflows with probability factors that underflow. At `r = 10`,
`p = 0.01`, `x = 2000` the coefficient is astronomically large and `p^r q^x` astronomically
small, while the answer is an unremarkable small probability. This is the same structural
hazard as the Poisson's, and it is why every serious discrete-distribution implementation works
in log space or better.

**2. Log space fixes the range and creates the cancellation.**

    ln f = lnΓ(x+r) − lnΓ(r) − lnΓ(x+1) + r·ln p + x·ln q

is representable everywhere, but for `x` near the mean the terms are individually large and
nearly cancel. An absolute error `ε` in `ln f` is a relative error `ε` in `f`, so the accuracy
is set by how well the log-gamma differences are computed, not by the exponential. This is the
`lgamma` composition the upstream record refers to when it describes the negative binomial as
presumably sharing the binomial's route family "with different integer arguments".

**3. The saddle-point route is the modern standard — and it was ruled out here.** Loader's
method (2000) writes the mass as an exponential of a Stirling correction and a
cancellation-free deviance term, and it is what R's `dnbinom` uses. The evidence record attached
to this page reports that **Loader's `dnbinom` was refuted** as a model of Excel's surface, and
separately that this surface **does not inherit** the identified binomial route. Both are
recorded upstream as candidate-model scores; the record's own reader warning states that neither
figure may be attributed to `NEGBINOM.DIST` specifically. What survives as a Handbook-safe
statement is qualitative and still useful: **two strong hypotheses about how Excel computes this
distribution have been eliminated, and no replacement has been identified.**

**4. The CDF should be an incomplete beta, not a sum.** `F(x; r, p) = I_p(r, x+1)` costs
`O(1)` through a continued fraction and accumulates one rounding rather than `x`. The standard
implementation is the Lentz-evaluated continued fraction with the `I_p(a,b) = 1 − I_q(b,a)`
symmetry used to keep the argument on the fast-converging side of `(a)/(a+b)` — the treatment in
*Numerical Recipes* §6.4 and, at reference quality, DiDonato and Morris's TOMS algorithms.
Summing the mass terms instead is both slower and less accurate, and it inherits every problem
of section 1.

**5. The recurrence.** `f(x) = f(x−1) · q · (x + r − 1)/x` is exact in structure and cheap, and
running it upward from `f(0) = p^r` is stable while the terms increase. It is the natural way to
produce a whole table of masses, and it is a good cross-check on a saddle-point implementation —
though it cannot start where `p^r` underflows.

**6. `p^r` deserves its own attention.** At `x = 0` the entire answer is `p^r` with integer `r`.
Whether that is computed by `pow`, by repeated multiplication, or by `exp(r ln p)` is directly
observable at that one input, and the three disagree in the last bits. It is the same
one-cell-diagnoses-the-constant situation that `PHI(0)` provides for the normal density.

**What a careful independent implementation does**: saddle-point mass with a tabulated Stirling
correction; incomplete-beta CDF by continued fraction with the symmetry switch; explicit
handling of `x = 0`, `p = 1` and the `p = 0` degeneracy; and a stated bound. See
[About implementation options](../model/07-implementation-options.md).

## What has not been checked

`EV-DIST-0029` names `NEGBINOM.DIST` among its subjects, and it is an **open-discrepancy**
record whose central finding is about the *limits* of what is known. In the record's own terms:
two candidate-model scores exist — one establishing that the negative binomial does not inherit
the identified binomial route, one refuting Loader's `dnbinom` — and **neither sentence in the
upstream source names which negative binomial surface was measured.** The record states that the
available evidence points at the *legacy* spelling, because the cached oracle shard for that
lane is the legacy one, and that the source does not resolve it. Its reader warning is explicit
that neither figure may be attributed to `NEGBINOM.DIST` specifically. The record further notes
that this modern surface's own evidence is a single witness with no count.

So the honest summary is unusually stark: **the route by which Excel computes this distribution
is unidentified, two strong hypotheses have been eliminated, and it is not even established
which of the two Excel surfaces the elimination was performed against.**

No Handbook vector suite exists for `NEGBINOM.DIST`. No Handbook measurement of this surface
exists; `EV-DIST-0029` is upstream OxFunc work the Handbook has not re-verified.

Microsoft's documented behaviour above, including the three published defects recorded under
Errors, was retrieved from the `NEGBINOM.DIST` page. The page's equation is an image and was not
read; the formulas in this page's first section are stated as mathematics rather than
transcribed from Microsoft.

Inputs worth probing first:

1. **`NEGBINOM.DIST(x, r, p, TRUE)` against `BETA.DIST(p, r, x+1, TRUE)`** across a grid.
   Two Excel surfaces, one number, no external oracle. If the incomplete beta underneath is
   shared, they will agree closely; if they diverge, one of them is doing something else and
   that is a finding either way. This is the single best experiment on this page.
2. **`NEGBINOM.DIST(x, 1, p, c)` against the geometric distribution**, and against
   `NEGBINOM.DIST(x, 1, p, TRUE) = 1 − q^(x+1)` in closed form — an exact identity with no
   special function in it at all, which isolates the combinatorial staging completely.
3. **`x = 0` at a spread of `r` and `p`**, where the answer is exactly `p^r`. One column that
   distinguishes `pow` from repeated multiplication from `exp(r ln p)`.
4. **`x` near the mean `r·q/p` at increasing scale** — the cancellation regime. The rate at
   which accuracy degrades separates a log-composed staging from a saddle-point one.
5. **`p = 0` and `p = 1`**, and `number_s` at `0`, `0.9`, `1` and `1.9`, confirming the
   documented inclusive boundaries and the truncation direction. The `p = 0` cell in particular
   tests whether the documented `< 0` boundary is deliberate.
6. **`NEGBINOM.DIST(x, r, p, FALSE)` against `NEGBINOMDIST(x, r, p)`** over every probe above.
   `EV-DIST-0029` says the upstream work may have been measuring the legacy surface all along;
   one extra column would settle which surface anything is known about.
7. **A row-by-row sum**: `SUM` of `NEGBINOM.DIST(j, r, p, FALSE)` for `j = 0…x` against
   `NEGBINOM.DIST(x, r, p, TRUE)`. The two branches must agree, and the discrepancy between them
   is a direct read of whether the CDF is a sum or an incomplete beta.
8. **`cumulative` as `0`, `1`, `2`, text `"TRUE"`, and empty**, given the `Custom` profile.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| failures-before-successes | Excel's parameterization: the variable counts failures, not trials |
| overdispersion | Variance exceeding the mean; the reason this distribution is used |
| route-blind | An input whose value is common to competing stagings, so it cannot tell them apart |
| regularized incomplete beta | `I_p(a, b)`; the closed form of the negative binomial CDF |
| Gamma–Poisson mixture | The representation giving the distribution a continuous `r`; not available in Excel |

## Sources

- Microsoft, "NEGBINOM.DIST function" —
  <https://support.microsoft.com/en-us/office/negbinom-dist-function-c8239f89-c2d0-45bd-b6af-172e570f8599>
  (syntax; the four required arguments and their descriptions; "Number_f and number_s are
  truncated to integers"; the `#VALUE!` condition; the `#NUM!` conditions on `probability_s`
  and on `number_f`/`number_s`; and the `cumulative` description that calls the discrete mass a
  "probability density function"). Retrieved for this page; the page's equation is an image and
  was not read.
- Handbook evidence record `EV-DIST-0029` — the open-discrepancy record: the non-inheritance of
  the binomial route, the refutation of Loader's `dnbinom`, and the record's own statement that
  neither figure names a surface and that its reader warning forbids attributing them here.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 26 §26.5 — the incomplete
  beta function and its relations to the binomial and negative binomial sums; chapter 6 §6.6.
- C. Loader, "Fast and Accurate Computation of Binomial Probabilities" (2000) — the
  saddle-point `dbinom`/`dnbinom`, named here as the model the upstream record reports refuted.
- W. H. Press et al., *Numerical Recipes*, §6.4 — the incomplete beta by continued fraction with
  the symmetry switch.
- A. R. DiDonato and A. H. Morris, ACM TOMS algorithms for the incomplete beta and gamma ratios
  — the reference-quality route.
- B. D. McCullough and B. Wilson, *Computational Statistics & Data Analysis* (1999, 2002,
  2005); L. Knüsel on Excel's distribution functions; M. Welinder's Gnumeric statistical work.
  Named as literature; no figure restated.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [About implementation options](../model/07-implementation-options.md).
- `data/functions/FUNC.NEGBINOM.DIST.json` (arity 4, `Custom` profile,
  `signature_placeholder: true`, XLL symbol `xlfNegbinom_dist`) and
  `data/functions/FUNC.NEGBINOMDIST.json` (arity 3) — the source of the added-argument finding.
- `data/presence/FUNC.NEGBINOM.DIST.json` — implementing module
  `crates/oxfunc_core/src/functions/discrete_dist_family.rs`, shared with twelve other discrete
  distribution surfaces.
