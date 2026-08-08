---
schema: efh.function-page/v1
function_id: FUNC.BETA.INV
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
  - Documentation divergences
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: beta_gamma_stats_family
role_in_family: >-
  The family's beta quantile: defined by inversion of BETA.DIST rather than by a formula of its
  own, and therefore the surface whose accuracy is bounded by the forward function's.
---

# BETA.INV

## What it computes

`BETA.INV(probability, alpha, beta, [A], [B])` returns the **quantile** of the beta
distribution: the value \(x\) at which the cumulative distribution reaches the given
probability.

    BETA.INV(p, α, β, A, B) = x   such that   I_{(x−A)/(B−A)}(α, β) = p

Equivalently, with \(z\) the standardised variable and \(I^{-1}\) the inverse of the
regularized incomplete beta function in its first argument,

    x = A + (B − A) · I⁻¹(p; α, β)

Microsoft defines the function by exactly this inversion rather than by a formula: "Given a
value for probability, BETA.INV seeks that value x such that BETA.DIST(x, alpha, beta, TRUE, A,
B) = probability. Thus, precision of BETA.INV depends on precision of BETA.DIST."

That sentence is the most important thing on this page. `BETA.INV` has no independent
mathematical content — the inverse of the regularized incomplete beta has no closed form except
in degenerate cases — and no independent accuracy. Whatever the forward function gets wrong,
the inverse inherits, amplified by the reciprocal of the density at the answer:

    δx ≈ δp / f(x)

So a quantile in a flat tail, where \(f(x)\) is tiny, is enormously more sensitive to forward
error than a quantile near the mode. This is not an implementation weakness; it is the
conditioning of the problem, and it is why quantile functions in every library are the least
accurate members of their families.

**Domain and range.** \(\alpha,\beta > 0\); \(A < B\); \(p\) a probability. The result lies in
\([A, B]\), is non-decreasing in \(p\), and satisfies the two boundary identities
\(x(0) = A\) and \(x(1) = B\).

**Closed forms that do exist.** \(\alpha = \beta = 1\) (uniform) gives
\(x = A + (B-A)p\) exactly. \(\beta = 1\) gives \(z = p^{1/\alpha}\); \(\alpha = 1\) gives
\(z = 1 - (1-p)^{1/\beta}\). \(\alpha = \beta = 1/2\) (arcsine) gives
\(z = \sin^2(\pi p/2)\). These four are the exactness probes an implementation can actually be
held to, and they are listed again at the end of this page for that reason.

## Arguments

| Argument | Meaning | Admissible | Default |
|---|---|---|---|
| `probability` | The cumulative probability to invert | documented as \(0 < p \le 1\) | — |
| `alpha` | First shape parameter | \(> 0\) | — |
| `beta` | Second shape parameter | \(> 0\) | — |
| `A` | Lower bound of the support | \(< B\) | `0` |
| `B` | Upper bound of the support | \(> A\) | `1` |

Microsoft describes `probability` as "A probability associated with the beta distribution",
`alpha` and `beta` each as "A parameter of the distribution", and `A` and `B` as lower and
upper bounds to the interval of `x`, with "If you omit values for A and B, BETA.INV uses the
standard cumulative beta distribution, so that A = 0 and B = 1."

There is no `cumulative` argument: an inverse of a density would not be a function, so the
question does not arise.

The reference engine declares an arity of 3 to 5.

## Result and edge cases

Returns `Number`.

- **`p = 1`.** The true answer is `B`. Documented as admissible — Microsoft's error rule
  excludes `probability > 1`, not `probability = 1`.
- **`p = 0`.** The true answer is `A`. Microsoft documents this as an **error**; the reference
  engine returns `A`. See the divergences section.
- **`alpha = beta = 1`.** Linear in `p`; the exactness probe.
- **Extreme tails.** For \(p\) near 0 with \(\alpha\) large, or \(p\) near 1 with \(\beta\)
  large, the answer is compressed into a region where the density is small and the inversion is
  badly conditioned. Results there are correct in *probability* far more than in *value*, and
  quoting a `BETA.INV` result to full precision in that regime overstates what any
  implementation can deliver.
- **Non-integer shapes** are ordinary.
- **`A` and `B`.** The result is affine in the bounds: \(x = A + (B-A)z\). The reference engine
  computes \(z\) on \([0,1]\) and rescales at the end, which means the rescaling contributes one
  multiplication and one addition of rounding on top of the inversion error.
- **Arrays.** The modern dotted surface lifts natively; the legacy `BETAINV` alias is recorded
  by the reference engine as scalar-shaped, broadcasting over its first three argument positions
  only, on the basis of observation against a named live Excel build.

## Errors

As documented by Microsoft on the `BETA.INV` page:

| Error | Condition |
|---|---|
| `#VALUE!` | Any argument is nonnumeric |
| `#NUM!` | `alpha` ≤ 0 or `beta` ≤ 0 |
| `#NUM!` | `probability` ≤ 0 or `probability` > 1 |

The reference engine at `473efa3` produces the first two, and differs on the third:

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | `probability` < 0 or `probability` > 1 | reference engine |
| `#NUM!` | `B` ≤ `A` | reference engine |
| `#VALUE!` | A shape, bound or probability argument is non-finite | reference engine |

## Documentation divergences

**`probability = 0`.** Microsoft's page says `probability ≤ 0` returns `#NUM!`, which makes zero
an error. The reference engine admits zero and returns the support's lower bound `A`, which is
the mathematically correct quantile: \(I_0(\alpha,\beta) = 0\) for every admissible shape, so
`A` is the unique point satisfying the defining equation.

This is a clean divergence between the documentation and the reference implementation, and it
is not academic — `BETA.INV(0, 2, 3)` returns a number under one reading and an error under the
other. Nobody has checked which Excel does. It is the first probe below.

The reference engine's sibling `GAMMA.INV` in the same module carries an inline note recording
the analogous boundary pair as *observed* against a named live Excel build: probability 0
returning the support's lower bound, probability 1 returning `#NUM!`. That is an observation
about a different function, and it is recorded here only as the reason to expect the beta
boundary to be worth probing — it transfers no result to `BETA.INV`.

## Relationships

- **[BETA.DIST](FUNC.BETA.DIST.md)** — the forward function, and the definition of this one.
  Everything in that page's numerical notes bounds this page's accuracy.
- **`BETAINV`** — the legacy compatibility name with the same signature. The reference engine
  implements both through one kernel, but **that is an implementation choice and not evidence
  that Excel computes them identically.** The Handbook's alias-pairing record for this area
  covers a group of legacy/modern *forward CDF* pairs and states its own limit explicitly: the
  pairing does not extend to inverses, and no inverse pair is established anywhere in the
  upstream record. Proving `BETA.INV` and `BETAINV` are the same computation requires evidence
  that does not exist. Treat them as two functions until it does.
- **`GAMMA.INV`** — the module sibling, with the same inversion structure and a boundary rule
  that has been observed where this one has not.
- **`F.INV`, `T.INV`, `CHISQ.INV`, `BINOM.INV`** — the other quantile surfaces. All of them
  share this page's conditioning story: quantile accuracy is forward accuracy divided by the
  density.
- **Metamorphic partner**: `BETA.DIST(BETA.INV(p, α, β), α, β, TRUE)` should return `p`. Round
  trips like this are the most productive way to test an inverse, because they need no
  high-precision oracle — but note they test the *pair*, and a compensating error in both
  survives the test.

## Numerical notes

**The problem has no closed form**, so every implementation is a root-finder wrapped around the
forward function. The engineering choices are the starting approximation, the iteration, and
the stopping rule, and they are what separate implementations.

**Starting approximations.** The classical route is the Wilson–Hilferty-style normal
approximation to the beta quantile, refined by a Taylor correction; Abramowitz & Stegun 26.5.22
gives the standard relation used for this. Applied Statistics **AS 64** (Cran, Martin and
Thomas, "Remark AS R19 and Algorithm AS 109: Inverse of the incomplete beta function ratio")
and its successor **AS 109** are the canonical published algorithms, and they are what most
statistical libraries either use or descend from.

**The iteration.** Newton's method on \(g(z) = I_z(\alpha,\beta) - p\) has derivative
\(g'(z) = z^{\alpha-1}(1-z)^{\beta-1}/B(\alpha,\beta)\) — the density, available in closed
form — so Newton is cheap and quadratically convergent near the answer. It is also fragile:
where the density underflows, the Newton step is unbounded, and where the density is huge the
step is degenerate. Robust implementations (Boost's `ibeta_inv`, R's `qbeta`) use a safeguarded
Newton or Halley iteration bracketed by bisection, work in whichever tail is better
conditioned, and iterate on \(\log z\) or \(\log(1-z)\) in the extremes.

**What the reference engine does.** Plain **bisection** on \([0,1]\) against
`regularized_beta`, with no Newton acceleration and no tail transformation. Bisection is
unconditionally robust and gains one bit per iteration, so its achievable precision is bounded
by the number of iterations it runs and by the forward function's ability to distinguish
neighbouring \(z\) values — which, in a flat tail, it cannot. That is a statement about the
reference engine at `473efa3`, not about Excel.

**The conditioning again, quantitatively.** With \(f\) the density at the answer,
\(\mathrm{d}x/\mathrm{d}p = 1/f(x)\). In the far tail of a beta with large \(\beta\), \(f\) can
be many orders of magnitude below 1, and a forward function accurate to a few ulp in \(p\) still
yields a quantile accurate to only a few significant digits in \(x\). **No implementation can do
better than this**, so a residual plate for a quantile function must be read against the
density, not against a uniform ulp target. This is the most common misreading of published
accuracy figures for inverse distribution functions.

**A note on inverting the published surface.** Where a distribution has both a left-tail and a
right-tail surface, inverting the *published* tail directly is not the same as inverting the
complement at \(1-p\), and the difference is systematic rather than random — the reference
engine's own source records exactly this for the chi-square and F inverses, with the complement
staging carrying a directional bias. `BETA.INV` has only one tail, so the question does not
arise here; it is mentioned because it is the structural reason the `.RT` inverses elsewhere in
the statistical lane are separate functions and not wrappers.

## What has not been checked

No Handbook vector suite exists — `vectors/` publishes none — and **no evidence record lists
`BETA.INV` among its subjects.** The alias-pairing record covering this area concerns forward
CDFs and explicitly excludes inverses, so nothing measured on a sibling may be attributed here.
The documented statements above come from Microsoft's `BETA.INV` page, fetched while this page
was written; the rest is mathematics or the reference engine at commit `473efa3`, named as such.

The reference engine's own **known-exactness-deviation register** carries an open entry for the
statistical distribution family (`KED-STAT-001`, owned by defect stream `BUG-FUNC-021`) whose
first representative witness is the pair `=BETA.INV(0.6,2,3)` / `=BETAINV(0.6,2,3)`. That entry
names an input, not a Handbook measurement, and its own status is open; it is recorded here
because it tells you where the upstream lane believed the residual to be, and because a witness
input is a free probe.

Inputs worth probing first:

1. **`BETA.INV(0, 2, 3)`.** One cell settles the documented-versus-implemented divergence:
   `#NUM!` under Microsoft's rule, `0` under the reference engine's. Highest-value probe on the
   page and the cheapest.
2. **`BETA.INV(1, 2, 3)`.** The other boundary, documented as admissible, where the true answer
   is `B`. Whether an iterative implementation actually reaches the endpoint exactly is a
   separate question from whether it errors.
3. **The four closed forms.** `BETA.INV(p, 1, 1)` = \(p\); `BETA.INV(p, α, 1)` = \(p^{1/α}\);
   `BETA.INV(p, 1, β)` = \(1-(1-p)^{1/β}\); `BETA.INV(p, 0.5, 0.5)` = \(\sin^2(\pi p/2)\).
   These are the only cases where an exact expected value is available without a
   high-precision oracle, and any implementation that misses them is not close.
4. **The round trip** `BETA.DIST(BETA.INV(p, α, β), α, β, TRUE)` across a grid of \(p\) and
   shapes, which localises whether error is entering through the inversion or the forward
   function.
5. **A flat-tail quantile** — `BETA.INV(1e-12, 2, 500)` — where the conditioning argument
   predicts that no implementation can be accurate in \(x\), and where a bisection-only
   implementation and a safeguarded-Newton one will differ visibly.
6. **`A` and `B` supplied**, checking that the result is exactly affine in the bounds: the same
   `p` inverted on \([0,1]\) and on \([10, 20]\) should differ by exactly the affine map, up to
   two roundings.
7. **`BETA.INV` against `BETAINV`** on identical arguments, including with an array in the first
   position, where the reference engine already predicts a lift difference. Any numeric
   difference would be a genuine finding, since the record contains no evidence that the two
   agree.
8. **`alpha = beta` at `p = 0.5`**, where symmetry gives the exact midpoint of \([A,B]\).
9. **`BETA.INV(0.6, 2, 3)`** — the register witness named above, which costs nothing to run and
   is where the upstream lane already looked.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| quantile | The value \(x\) at which the CDF reaches a given probability |
| conditioning | \(\mathrm{d}x/\mathrm{d}p = 1/f(x)\); why flat-tail quantiles cannot be accurate in \(x\) |
| safeguarded Newton | A Newton or Halley iteration bracketed by bisection so it cannot diverge |
| round trip | Composing the inverse with the forward function and checking the identity |
| alias pairing | The question of whether a legacy name and its modern counterpart are the same computation |
| reference engine | OxFunc at commit `473efa3` |

## Sources

- Microsoft, "BETA.INV function" —
  <https://support.microsoft.com/en-us/office/beta-inv-function-e84cb8aa-8df0-4cf6-9892-83a341d252eb>
  (syntax; the five argument descriptions; the nonnumeric `#VALUE!`; the `alpha`/`beta` ≤ 0
  `#NUM!`; the `probability ≤ 0 or > 1` `#NUM!`; the A = 0, B = 1 defaults; and the
  defining-by-inversion sentence with its precision-depends-on-`BETA.DIST` clause).
- OxFunc `crates/oxfunc_core/src/functions/beta_gamma_stats_family.rs` at commit `473efa3` —
  the `bisect_inverse` kernel over `regularized_beta`, the closed-unit-interval probability
  check that admits zero, the bound validation, the affine rescaling, the recorded lift
  difference for the legacy alias, and the `GAMMA.INV` boundary note cited above as context.
- OxFunc `docs/function-lane/FUNCTION_SLICE_STATISTICAL_DISTRIBUTIONS_AND_COMPAT_A_CONTRACT_PRELIM.md`
  section 3 — the family contract's bounds and error lanes.
- OxFunc `docs/KNOWN_EXACTNESS_DEVIATIONS.md`, entry `KED-STAT-001` (status open, owner
  `BUG-FUNC-021`) — cited for its representative witness input only; its counts are upstream
  measurements and are not restated here.
- Handbook, [BETA.DIST](FUNC.BETA.DIST.md) — the forward function, its algorithm literature, and
  the evidence records that do list a beta surface among their subjects.
- M. Abramowitz and I. A. Stegun, *Handbook of Mathematical Functions*, chapter 26.5 —
  the beta distribution, its normal approximations (26.5.22) and the relations used for
  starting approximations.
- G. W. Cran, K. J. Martin and G. E. Thomas, "Remark AS R19 and Algorithm AS 109: Inverse of the
  Incomplete Beta Function Ratio", *Applied Statistics* 26 (1977); and AS 64, its predecessor —
  the canonical published beta-quantile algorithms.
- A. R. DiDonato and A. H. Morris, ACM TOMS Algorithm 708 — the forward routine whose accuracy
  bounds this one.
- M. Welinder's work on the Gnumeric statistical functions and the published critiques of
  spreadsheet statistical accuracy.
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md`.
