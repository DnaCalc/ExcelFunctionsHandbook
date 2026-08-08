---
schema: efh.function-page/v1
function_id: FUNC.BINOM.DIST
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0027
  - EV-DIST-0028
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
family: discrete_dist_family
role_in_family: >-
  The family's binomial mass and tail, and the surface whose underlying probability-mass
  algorithm the rest of the discrete module's binomial surfaces are built on.
---

# BINOM.DIST

## What it computes

`BINOM.DIST(number_s, trials, probability_s, cumulative)` evaluates the binomial distribution:
the law of the number of successes in \(n\) independent Bernoulli trials each succeeding with
probability \(p\).

Write \(k =\) `number_s`, \(n =\) `trials`, \(p =\) `probability_s`, \(q = 1 - p\).

**Probability mass** (`cumulative` = FALSE)

    b(k; n, p) = C(n, k) · p^k · q^{n−k},      C(n, k) = n! / (k! (n−k)!)

**Cumulative distribution** (`cumulative` = TRUE)

    B(k; n, p) = Σ_{j=0}^{k} C(n, j) · p^j · q^{n−j}

Microsoft's page gives both formulae, with \(C(n,x)\) written as `COMBIN(n,x)`, and describes
the cumulative form as the probability of "at most number_s successes" and the mass form as
"exactly number_s successes".

**Domain.** \(n \ge 0\) integer, \(0 \le k \le n\) integer, \(0 \le p \le 1\). Non-integer `k`
and `n` are truncated toward zero before use; Microsoft documents "Number_s and trials are
truncated to integers."

**Range.** Each mass lies in \([0,1]\) and the masses sum to exactly 1 over
\(k = 0,\dots,n\). The CDF is non-decreasing in \(k\), with \(B(n;n,p) = 1\).

**Moments.** Mean \(np\); variance \(npq\); mode \(\lfloor (n+1)p \rfloor\) (two modes when
\((n+1)p\) is an integer).

**Structural identities** worth having, because they are what makes this function checkable:

    b(k+1; n, p) = b(k; n, p) · ((n − k)/(k + 1)) · (p/q)      forward recurrence
    Σ_{k=0}^{n} b(k; n, p) = 1                                  normalisation
    b(k; n, p) = b(n − k; n, q)                                 reflection
    B(k; n, p) = I_q(n − k, k + 1) = 1 − I_p(k + 1, n − k)      incomplete beta  [A&S 26.5.24]

The last identity is the bridge to [BETA.DIST](FUNC.BETA.DIST.md), and it is the reason a
binomial tail and a beta CDF are, mathematically, the same object. Excel implements them as
separate functions in separate places, so the identity is a metamorphic test rather than a
guarantee.

**Degenerate parameters.** \(p = 0\) puts all mass on \(k = 0\); \(p = 1\) puts all mass on
\(k = n\); \(n = 0\) puts all mass on \(k = 0\). All three are in the documented domain and all
three are handled as closed forms rather than through the general algorithm.

## Arguments

| Argument | Meaning | Admissible |
|---|---|---|
| `number_s` | Number of successes | \(0 \le k \le n\), truncated to an integer |
| `trials` | Number of independent trials | \(\ge 0\), truncated to an integer |
| `probability_s` | Success probability per trial | \(0 \le p \le 1\) |
| `cumulative` | TRUE for the CDF, FALSE for the mass | logical / numeric |

Microsoft describes them as "The number of successes in trials", "The number of independent
trials", "The probability of success on each trial", and "A logical value that determines
function form".

All four are required — the reference engine declares an arity of exactly 4. `cumulative` is
converted by the ordinary nonzero-is-true rule, so any nonzero number selects the CDF.

The truncation rule is a genuine semantic decision, not a convenience: `BINOM.DIST(2.9, 10,
0.5, FALSE)` is the mass at 2, not an interpolation and not an error.

## Result and edge cases

Returns `Number`.

- **`p = 0` or `p = 1`.** The reference engine short-circuits to the degenerate answer (1 at the
  supported point, 0 elsewhere) without entering the general algorithm. These are the two
  configurations where the exact answer is a representable integer and any implementation can
  be held to it exactly.
- **`k = 0` and `k = n`.** The mass reduces to \(q^n\) and \(p^n\). The reference engine treats
  both as special cases with their own evaluation paths, chosen on the size of the relevant
  probability — a detail that matters because it means the function's accuracy is not uniform
  across \(k\), and a residual plate will show seams at the ends.
- **`n = 0`.** \(k\) must be 0, and the mass is 1 (the empty product).
- **Large `n` with small `p`.** The Poisson regime. Masses become tiny and the algorithm's
  relative accuracy is what matters, not its absolute accuracy — see the numerical notes.
- **`cumulative` = TRUE with large `k`.** The reference engine sums the masses
  \(j = 0,\dots,k\) in ascending order, recomputing each from scratch. That is \(O(k)\)
  evaluations of the mass function and an accumulation whose error grows with \(k\); it is also
  the reason the CDF and the mass have different accuracy profiles.
- **Text that looks numeric** in any slot is coerced by the ordinary rules.
- **Arrays.** The modern dotted surface lifts natively. The legacy `BINOMDIST` alias is recorded
  by the reference engine as scalar-shaped, broadcasting over **all four** of its argument
  positions, on the basis of observation against a named live Excel build — a different lift
  profile from the modern name.

## Errors

As documented by Microsoft on the `BINOM.DIST` page:

| Error | Condition |
|---|---|
| `#VALUE!` | `number_s`, `trials` or `probability_s` is nonnumeric |
| `#NUM!` | `number_s` < 0 or `number_s` > `trials` |
| `#NUM!` | `probability_s` < 0 or `probability_s` > 1 |

The reference engine at `473efa3` produces those and adds:

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | `trials` < 0 | reference engine — Microsoft's page does not list it |
| `#NUM!` | Any of the numeric arguments is non-finite | reference engine |

Microsoft's documented list is notably narrower than the domain the mathematics requires:
negative `trials` is excluded by the constraint \(0 \le k \le n\) only indirectly, and the page
does not name it. Recorded here as a documentation gap rather than a contradiction.

## Relationships

- **[BINOM.DIST.RANGE](FUNC.BINOM.DIST.RANGE.md)** — the interval probability
  \(P(s \le X \le s_2)\). Mathematically a difference of two `BINOM.DIST` cumulative values, but
  **not** the same computation: the reference engine sums masses directly, and for moderate
  `trials` uses a different mass evaluation altogether. The two routes can disagree in the last
  bits; see that page.
- **[BINOM.INV](FUNC.BINOM.INV.md)** — the quantile: the smallest \(k\) whose cumulative
  probability reaches a criterion. It is built directly on this function's cumulative form, so
  this page's accuracy bounds it.
- **`BINOMDIST`** — the legacy compatibility name with the same four arguments. The reference
  engine routes both through one kernel, but that is an implementation choice and **not**
  evidence that Excel computes them identically; the array-lift profiles already differ.
  Evidence record `EV-DIST-0028` concerns exactly the question of what is on record for the
  legacy name and what is not.
- **[BETA.DIST](FUNC.BETA.DIST.md)** — the continuous counterpart through the incomplete-beta
  identity above. `1 - BETA.DIST(p, k+1, n-k, TRUE)` and `BINOM.DIST(k, n, p, TRUE)` are the
  same number and are computed by entirely different code.
- **`POISSON.DIST`** — the \(n \to \infty\), \(np \to \lambda\) limit; the right function when
  \(n\) is large and \(p\) small.
- **`NORM.DIST`** — the normal approximation, \(\text{Binomial}(n,p) \approx
  \mathcal{N}(np, npq)\) with a continuity correction. Accurate in the body, useless in the
  tails, and the source of a large fraction of wrong answers in practice.
- **`HYPGEOM.DIST`** — sampling without replacement; the binomial is its large-population limit.
- **`NEGBINOM.DIST`** — the complementary question (trials until a fixed number of successes).
- **`COMBIN`** — the \(C(n,k)\) factor, exposed as its own function. Note that
  `COMBIN(n,k)*p^k*q^(n-k)` written out on the worksheet is precisely the naive evaluation the
  numerical notes warn against.

## Numerical notes

The binomial mass function is the standard worked example of a formula that is trivial to write
and hard to evaluate, and its literature is unusually well settled.

**Why the direct formula fails.** \(C(n,k)\) grows like \(2^n\) at the centre, overflowing
double precision at around \(n = 1030\), while \(p^k q^{n-k}\) underflows on the same schedule.
The product is perfectly representable; the factors are not. So the direct evaluation fails not
because the answer is out of range but because the intermediate quantities are.

**Why the logarithmic formula also fails.** The obvious fix,
\(\exp(\ln C(n,k) + k\ln p + (n-k)\ln q)\), removes the overflow and replaces it with a
precision problem. \(\ln C(n,k)\) is of order \(n\); computing it to full *relative* precision
still leaves an absolute error of order \(n \cdot 2^{-53}\), and exponentiating a quantity with
absolute error \(\varepsilon\) multiplies the result by \(e^{\varepsilon}\). The relative error
of the answer is therefore proportional to \(n\), and for \(n\) in the millions the result has
no significant digits left. This failure is quiet — the answer looks plausible — which is what
makes it dangerous.

**The saddle-point fix.** Catherine Loader's "Fast and Accurate Computation of Binomial
Probabilities" (2000) restructures the mass as

    b(k; n, p) = ( 1 / sqrt(2π k (n−k)/n) ) · exp( −D(k; n, p) )

where the deviance term \(D\) is assembled from two carefully evaluated pieces: `stirlerr(m)`,
the error in Stirling's approximation to \(\ln m!\) (a small quantity, tabulated for small
\(m\) and given by an asymptotic series above), and `bd0(x, np)`, the deviance
\(x\ln(x/np) + np - x\), evaluated by a convergent series when its arguments are close so that
the cancellation never happens. The point of the construction is that every quantity actually
computed is \(O(1)\) rather than \(O(n)\), so the relative error does not grow with \(n\). This
is the algorithm in R's `dbinom` (`dbinom_raw`), and it is what modern statistical software
uses.

**What the reference engine does.** It implements Loader's construction — the `stirlerr` table
and asymptotic tiers, the `bd0` series-versus-direct split, and the \(k=0\) and \(k=n\) closed
forms — with its transcendental calls routed through its own Excel-compatibility layer rather
than through the platform's. Its source comments record that this replaced an earlier
log-composed path. For the cumulative form it sums masses from \(j=0\) upward, so the CDF
inherits the mass function's error plus \(O(k)\) accumulation.

**Where the remaining error lives.** In an implementation of this shape the residual error is
concentrated in (a) the transcendental calls, whose last-bit behaviour depends on the runtime
library, (b) the association order of the deviance assembly, which is not associative in
floating point, and (c) the accumulation in the cumulative form. All three are exactly the kinds
of detail that differ between engines while the mathematics agrees.

**A structural caution about the cumulative form.** An upper tail computed as
`1 - BINOM.DIST(k, n, p, TRUE)` loses all relative precision once the lower tail approaches 1 —
the same complement-formation loss described on [BETA.DIST](FUNC.BETA.DIST.md#numerical-notes).
For small upper tails, summing the masses from the top is far better than complementing, and the
incomplete-beta identity is better still.

## What has not been checked

No Handbook vector suite exists for `BINOM.DIST`; `vectors/` publishes none for any function, so
no claim here carries the suite axis `CHARTER.md` rule 7.2 requires.

Two evidence records list this surface among their subjects, and the evidence layer renders
their figures, scope and reader warnings mechanically beside this page. In prose:

- **`EV-DIST-0027`** is a substrate-identification record. It reports that the upstream lane
  identified Excel's binomial mass as R's `dbinom_raw` construction, and recovered specific
  implementation details of that identification down to how one logarithm-of-one-plus is
  realized. Its published scores are **percentages with no numerator or denominator**, most of
  them measured on corpora that the implementation was fitted to rather than held out from, and
  the record states plainly that the row remains **open**. A percentage without a base is not a
  count, and this page does not treat it as one.
- **`EV-DIST-0028`** is a projection-gap record. It notes that an upstream defect stream
  records a live-Excel numeric result for this surface — a set of replay rows found to agree —
  **stated without any count**, and scoped to those replay rows only. Its own reader warning is
  that this is different both from having no evidence and from having a measured pass rate.

So: something has been measured here, and what has been measured cannot be stated as a rate.
What remains entirely unchecked by the Handbook: the density-versus-cumulative split, every
error condition, the truncation rule, the degenerate `p` and `n` cases, the coercion behaviour
of each slot, and the array-lift difference against the legacy name.

Inputs worth probing first:

1. **The normalisation identity**: `BINOM.DIST(n, n, p, TRUE)` must be exactly `1`, and
   `SUM` over all `k` of the masses must be 1 to within accumulation error. This is the
   cheapest accuracy probe available and needs no oracle.
2. **`BINOM.DIST(0, 0, 0, TRUE)` and the degenerate `p` cases** — `p = 0` and `p = 1` at every
   `k` — where the true answers are exactly 0 or exactly 1.
3. **`BINOM.DIST(2.9, 10, 0.5, FALSE)` against `BINOM.DIST(2, 10, 0.5, FALSE)`** — the
   documented truncation rule, isolated.
4. **The large-\(n\) relative-accuracy probe**: a mass at \(n = 10^6\), \(k = np\), where the
   log-composed and saddle-point algorithms differ by a factor that grows with \(n\). This is
   the probe that would distinguish which family of algorithm is running, and it is the one
   `EV-DIST-0027`'s identification work is about.
5. **The reflection identity** `BINOM.DIST(k, n, p, FALSE)` = `BINOM.DIST(n−k, n, 1−p, FALSE)`,
   which is exact mathematically and is a pure test of implementation symmetry.
6. **The incomplete-beta bridge**: `BINOM.DIST(k, n, p, TRUE)` against
   `1 - BETA.DIST(p, k+1, n-k, TRUE)`, which crosses two modules and would expose a
   disagreement between the continuous and discrete lanes.
7. **`trials` negative**, undocumented by Microsoft and rejected by the reference engine.
8. **`BINOM.DIST` versus `BINOMDIST`** on identical arguments and on array arguments, where
   the reference engine already predicts a lift difference and where no evidence establishes
   numeric agreement.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| probability mass | The probability of exactly \(k\) successes; the `cumulative` = FALSE form |
| saddle-point form | Loader's restructuring of the mass into a Stirling-error term and a deviance term |
| deviance term | \(x\ln(x/np) + np - x\); evaluated by series when the arguments are close |
| Stirling error | \(\ln m! - (m\ln m - m + \tfrac12\ln 2\pi m)\); small, tabulated or series-evaluated |
| complement-formation loss | The precision destroyed by computing an upper tail as \(1 - \text{lower}\) |
| reference engine | OxFunc at commit `473efa3` |

## Sources

- Microsoft, "BINOM.DIST function" —
  <https://support.microsoft.com/en-us/office/binom-dist-function-c5ae37b6-f39c-4be2-94c2-509a1480770c>
  (syntax; the four argument descriptions; the at-most/exactly reading of `cumulative`; the
  truncation remark; the nonnumeric `#VALUE!`; the `number_s` range `#NUM!`; the
  `probability_s` range `#NUM!`; and the mass and cumulative equations with \(C(n,x)\) given as
  `COMBIN(n,x)`).
- Handbook evidence records `EV-DIST-0027` and `EV-DIST-0028`, which list this surface among
  their subjects; their figures, scope, currency and reader warnings are rendered by the
  evidence layer.
- OxFunc `crates/oxfunc_core/src/functions/discrete_dist_family.rs` at commit `473efa3` — the
  `stirlerr` table and asymptotic tiers, the `bd0` series/direct split, the \(k=0\) and \(k=n\)
  closed forms, the degenerate-`p` short circuits, the ascending cumulative summation, and the
  recorded lift difference for the legacy alias.
- OxFunc `docs/bugs/streams/BUG-FUNC-021_w090_statistical_numeric_exactness_drift.md` — the
  defect stream `EV-DIST-0028` is drawn from.
- C. Loader, "Fast and Accurate Computation of Binomial Probabilities" (2000) — the saddle-point
  construction (`stirlerr`, `bd0`) used by R's `dbinom` and named in the numerical notes.
- M. Abramowitz and I. A. Stegun, *Handbook of Mathematical Functions*, chapter 26.1 (binomial
  distribution) and 26.5.24 (the incomplete-beta identity for the binomial tail).
- W. H. Press et al., *Numerical Recipes*, §6.4 — the incomplete-beta route to binomial tails.
- M. Welinder's work on the Gnumeric statistical functions and the published critiques of
  spreadsheet statistical accuracy.
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md`.
