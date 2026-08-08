---
schema: efh.function-page/v1
function_id: FUNC.GAMMA.INV
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0003
  - EV-DIST-0016
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Gamma_Inv method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.gamma_inv"
    role: "documented parameters, the round-trip statement, the error conditions, and the iterative-search / 100-iteration / #N/A remark"
  - work: "Microsoft Support — GAMMA.INV function"
    locator: "https://support.microsoft.com/en-us/office/gamma-inv-function-74991443-c2b0-4be5-aaab-1aa4d71fbb18"
    role: "the worksheet-surface documentation page; not retrievable at curation time (the host refused the request)"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapters 6 and 26"
    locator: "6.5 — incomplete gamma; 26.4 — chi-square percentage points"
    role: "the forward function this one inverts, and the classical quantile relations"
  - work: "D. J. Best and D. E. Roberts, Algorithm AS 91: the percentage points of the chi-squared distribution (1975)"
    locator: null
    role: "the classical seeded Newton scheme for gamma/chi-square quantiles"
  - work: "DCDFLIB / CDFLIB (Brown, Lovato, Russell)"
    locator: null
    role: "the standard inverse-by-bracketed-search design for distribution quantiles"
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
  The gamma quantile: a root-finder over the module's own forward cumulative surface, and the
  member whose accuracy is documented to be bounded by that forward's accuracy.
---

## What it computes

`GAMMA.INV(probability, alpha, beta)` returns the value x at which the gamma cumulative
distribution attains the given probability. Microsoft states it as an inverse relation:
"If p = GAMMA_DIST(x, …), then GAMMA_INV(p, …) = x", and the same page adds the operational
definition — the function "seeks that value x such that GAMMA_DIST(x, alpha, beta, TRUE) =
probability".

In the notation of the [GAMMA.DIST](FUNC.GAMMA.DIST.md) page, with P the regularized lower
incomplete gamma:

    GAMMA.INV(p, α, β)  =  β · P⁻¹(α, p)      i.e. the unique x ≥ 0 with P(α, x/β) = p

**Why the inverse is well defined.** For fixed α > 0, P(α, ·) is continuous and strictly
increasing from 0 to 1 on [0, ∞). A strictly monotone continuous surjection onto (0, 1) has a
unique continuous inverse there, so the quantile exists and is unique for every 0 < p < 1. The
scale enters linearly: the quantile for scale β is β times the quantile for β = 1, which is why
a competent implementation solves the standard problem and multiplies.

**The endpoints are the interesting part of the domain.** At p = 0 the answer is the support's
lower bound, 0. At p = 1 there is **no finite answer** — P reaches 1 only in the limit — so the
inverse is +∞. The documented admissible range and the mathematics therefore disagree at one
endpoint; see **Errors**.

**Behaviour of the quantile function.** It is increasing in p, increasing in β (linearly),
and increasing in α. Its derivative is 1/f(x), the reciprocal of the density, so the quantile
is *steep wherever the density is small* — which is to say in both tails. That is the
conditioning statement that governs everything numerical about this function: near p = 0 and
near p = 1, a small perturbation in p moves x a great deal, and no algorithm can do better than
the conditioning allows.

**Relation to chi-square.** With α = n/2 and β = 2 this is the chi-square quantile with n
degrees of freedom — the same identity that makes `GAMMA.DIST` a chi-square CDF, read
backwards. A&S §26.4 tabulates those percentage points, and Best & Roberts' AS 91 is the
classical algorithm for them.

## Arguments

Three arguments, all required; the reference engine declares an arity of exactly 3.

| Argument | Meaning |
|---|---|
| `probability` | The probability associated with the gamma distribution. |
| `alpha` | The shape parameter α. |
| `beta` | The scale parameter β. Documented: "If beta = 1, GAMMA_INV returns the standard gamma distribution." |

Values-only numeric slots under ordinary to-number coercion; see
[Coercion and lifting](../model/02-coercion-and-lifting.md). As with the forward function,
β is a **scale**, not a rate.

## Result and edge cases

Returns `Number`.

- **p = 0** returns 0, the lower bound of the support. Documented as admissible (the `#NUM!`
  condition is `probability < 0`, strictly), and the reference engine returns 0 with an upstream
  note recording a live-Excel check of the same behaviour.
- **p = 1** has no finite answer. See the divergence recorded under **Errors**.
- **Both tails are ill-conditioned**, as described above. A quantile requested at p = 10⁻¹⁵ or
  p = 1 − 10⁻¹⁵ is a question about the forward function's behaviour in a region where the
  forward function has no resolution left, and the answer inherits that.
- **Small α** pushes the quantile toward 0 for most p and makes the density singular at the
  origin, so the root-finder's lower bracket needs care.
- **Large α** makes the distribution nearly normal, and a normal-approximation seed (the
  Wilson–Hilferty cube-root transformation) becomes an excellent starting point.
- **Monotonicity in p is a free self-check**: evaluating the function on an increasing grid of
  p and observing a non-monotone result proves an implementation defect without any oracle.
- **The round trip** `GAMMA.DIST(GAMMA.INV(p, α, β), α, β, TRUE)` should return p to within the
  conditioning; it is the natural metamorphic test and it is discussed under **Numerical notes**.

The projected battery beside this page records the reference engine's own answers on a fixed
probe list; no Excel was involved in producing them.

## Errors

As documented by Microsoft:

| Error | Condition |
|---|---|
| `#VALUE!` | any argument is text |
| `#NUM!` | `probability` < 0, or `probability` > 1 |
| `#NUM!` | `alpha` ≤ 0, or `beta` ≤ 0 |
| `#N/A` | the iterative search has not converged after 100 iterations |

**Two divergences worth recording.**

*First, the p = 1 endpoint.* The documented `#NUM!` condition is `probability > 1`, strictly, so
**p = 1 is inside the documented admissible range** — yet no finite value exists there, because
P(α, z) approaches 1 only asymptotically. The reference engine rejects `probability >= 1` with
`#NUM!`, and its source records that behaviour as verified against a live Excel build under the
upstream defect id `BUG-FUNC-039`. So the documented domain and the observed behaviour disagree
at exactly one point, and the observation is upstream's rather than the Handbook's. This is a
documentation defect, not an implementation one — but it is the documentation a reader consults.

*Second, the `#N/A` convergence clause.* The documented failure mode presumes an iterative search
that can run out of iterations. The reference engine's inverse does not: it bisects on the
**float lattice** until the bracket is two adjacent doubles, which terminates in a bounded number
of steps by construction, so its `#N/A` is unreachable. An implementation and a documentation
that disagree about whether a failure mode exists at all is a divergence, and it matters to
anyone writing an `IFNA` guard around this function.

Errors arriving in the arguments propagate under the shared coercion discipline. The reference
engine additionally maps non-finite arguments to `#VALUE!`, which has no documented counterpart.

## Relationships

- **[GAMMA.DIST](FUNC.GAMMA.DIST.md)** — the forward function this one inverts. Microsoft is
  explicit that "precision of GAMMA_INV depends on precision of GAMMA_DIST", which makes the
  forward page a hard dependency of this one: any inaccuracy in P is inherited here, amplified
  by the reciprocal of the density.
- **`GAMMAINV`** — the legacy spelling, classified under **Compatibility**. Here the Handbook
  must be blunt, and `EV-DIST-0016` says it in as many words: **no inverse pair is proven
  identical anywhere in the upstream record**, so `GAMMA.INV`'s figure may not be placed on
  `GAMMAINV`. The alias-collapse result that licenses cross-attribution for forward CDFs
  (`EV-DIST-0011`) explicitly does not extend to inverses. Two functions with the same
  documented definition, one measured and one not, and no warrant to transfer the measurement:
  that is the honest position and it is worth stating loudly, because the tempting inference is
  wrong.
- **`CHISQ.INV` / `CHIINV`** — the α = n/2, β = 2 case. Historically the chi-square quantile is
  the better-studied problem (AS 91), and an implementation of one is usually reachable from
  the other.
- **`EXPON.DIST`** — the α = 1 case has a closed-form inverse, x = −β·ln(1 − p), which needs no
  search at all. Whether an implementation takes that shortcut is a fingerprint.
- **`BETA.INV`** — the module sibling, the same root-finding design over the incomplete beta.
- **`NORM.S.INV`** — the source of the Wilson–Hilferty seed discussed below.

## Numerical notes

**The problem is root-finding over an expensive, monotone, ill-conditioned function.** Three
design axes matter and they are largely independent.

*The seed.* A good starting point turns a search into a couple of refinement steps. The
classical seeds are the **Wilson–Hilferty** cube-root normal approximation — the chi-square
variable raised to the one-third power is nearly normal, which gives a closed-form first guess
that is accurate to a few percent over most of the parameter space — and, for small α, a
series inversion near the origin. Best & Roberts' AS 91 is built on exactly this: a
Wilson–Hilferty seed refined by Newton steps on the incomplete gamma.

*The iteration.* Newton's method converges quadratically and needs the density, which is
available. It also overshoots into x < 0 from a bad seed, so production implementations bracket
it — Newton with a bisection fallback, or a safeguarded secant. Pure bisection is slow but
cannot fail; DCDFLIB's design is essentially a bracketed search with a careful expansion phase
to establish the upper bound.

*The stopping rule, which is where this function's identity actually lives.* A relative-tolerance
stop of a few epsilons *looks* prudent and is the source of the largest errors in practice: the
quantile function's steepness in the tails means that a bracket which is narrow in x can still
straddle a wide range of p, and stopping early in a region where the derivative is large leaves
a result that is wrong by far more than the tolerance suggests. The reference engine's inverse
does the opposite: it bisects on the float lattice, mapping doubles to a monotone integer key
and halving the *index* range until the bracket is two adjacent doubles, then publishing the
upper one. Upstream's note records that an earlier early-stop rule produced errors of order a
million ulps at small roots, and that lattice bisection collapsed them; the publication rule —
publish the high end rather than the closest or the low end — was raced against the alternatives
and the high end was retained. That is an unusually explicit account of a design choice that most
libraries leave implicit, and it makes a strong prediction: **an implementation that publishes
the closest double rather than the high one will disagree with this reference engine on roughly
half of all inputs, by one ulp**, and no amount of accuracy work will remove that disagreement.
It is a *convention*, not an error.

**Inherited error.** Because the inverse is defined against the forward, its accuracy is bounded
by the forward's. Microsoft says so; the arithmetic says so too. If P is wrong by δ at the root,
the returned x is wrong by roughly δ/f(x), and f is small in the tails. A perfectly implemented
inverse over an imperfect forward is still wrong — and, more subtly, an inverse tuned to match a
*particular* imperfect forward will stop matching when that forward is repaired. `EV-DIST-0003`
records exactly that hazard: the forward this inverse roots was replaced twice after the inverse
was last scored, and no re-score exists.

**Cost.** Lattice bisection costs on the order of the double exponent-plus-mantissa width in
forward evaluations — dozens of incomplete-gamma evaluations per call. That is far more than a
seeded Newton needs, and it buys determinism and a well-defined stopping point. For a
compatibility-oriented engine that is a good trade; for a hot loop it is not.

## What has not been checked

Two records name this surface as a subject.

`EV-DIST-0003` publishes a per-surface figure and immediately qualifies it: the corpus is
explicitly the one the inverter was *validated on* — a repair target, not a held-out gate — the
held-out corpus was captured separately, the publication rule was raced on the same corpus, and
**no re-score exists after the forward function it roots was replaced twice**. `EV-DIST-0016` is
a projection-gap record whose subject list includes this surface, and its content is the
non-inheritability statement quoted under **Relationships**. All figures render mechanically
beside this page; this prose deliberately does not restate them.

So the state of knowledge is: measured once, on a fitted corpus, against a forward function that
has since changed twice. That is weaker than a bare number would suggest, and stronger than
nothing.

What does not exist: any Handbook vector suite for `GAMMA.INV`; any current measurement against
the present forward; any measurement of the legacy `GAMMAINV` spelling that could be compared
against this one; any characterisation of the tails.

Inputs I would probe first, and why:

1. **`GAMMA.INV(1, α, β)` and `GAMMA.INV(0, α, β)`.** Two probes, and they settle the documented
   domain divergence at p = 1 and confirm the p = 0 boundary. The p = 1 case is the one where the
   documentation is demonstrably at odds with the mathematics.
2. **The round trip in both directions** — `GAMMA.INV(GAMMA.DIST(x, α, β, TRUE), α, β)` for x
   across the support, and its mirror. This needs no oracle, measures the composition, and
   exposes the stopping-rule convention: a systematic one-sided one-ulp bias is the signature of
   a publish-high rule.
3. **The α = 1 closed form**: `GAMMA.INV(p, 1, β)` against `−β·LN(1 − p)`. Exact agreement means
   the implementation short-circuits; disagreement means it searches. Either answer is a
   structural fact about the implementation obtained from one column of a spreadsheet.
4. **The chi-square identity**: `GAMMA.INV(p, n/2, 2)` against `CHISQ.INV(p, n)`, which tests
   whether Excel shares code between the two quantile surfaces.
5. **Both tails**: p = 10⁻¹⁰ and p = 1 − 10⁻¹⁰, at small and large α. This is where conditioning
   bites and where an early-stopping rule shows itself.
6. **Monotonicity sweeps in p** at fixed α, β — a non-monotone result is a defect provable
   without an oracle.
7. **`GAMMA.INV` against `GAMMAINV`** on identical arguments. Because no inverse pair is proven
   identical anywhere, this is genuinely open, and either result is publishable: agreement across
   a wide sweep would be the first evidence for an inverse alias collapse, and disagreement would
   be a finding of the first order.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| quantile | The inverse of a cumulative distribution function |
| conditioning | Here, 1/f(x): how far x moves for a small change in p |
| Wilson–Hilferty | The cube-root normal approximation used to seed gamma/chi-square quantile searches |
| lattice bisection | Bisecting on the integer ordering of doubles until the bracket is two adjacent values |
| publication rule | Which end of the final bracket is returned: high, low, or closest |
| inherited error | Error in the quantile caused by error in the forward CDF it inverts |
| non-inheritable figure | A measurement on one alias that may not be attributed to the other |

## Sources

- Microsoft Learn, "WorksheetFunction.Gamma_Inv method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.gamma_inv>
  (the round-trip statement, the three parameters, the `#VALUE!` and two `#NUM!` conditions, the
  statement that precision depends on `GAMMA_DIST`, and the iterative-search remark with its
  100-iteration `#N/A` clause). The worksheet-surface page at `support.microsoft.com` was not
  retrievable at curation time.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, §6.5 and §26.4.
- D. J. Best & D. E. Roberts, "Algorithm AS 91: The percentage points of the χ² distribution",
  *Applied Statistics* 24 (1975); DCDFLIB/CDFLIB (Brown, Lovato & Russell) — the seeded-Newton
  and bracketed-search designs discussed under **Numerical notes**.
- Handbook evidence records `EV-DIST-0003` (the fitted-corpus figure, the publication-rule race,
  and the missing re-score) and `EV-DIST-0016` (the non-inheritability of this surface's figure
  to the legacy spelling).
- Handbook, [GAMMA.DIST](FUNC.GAMMA.DIST.md) — the forward function and its own record.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.GAMMA.INV.json` and
  `data/presence/FUNC.GAMMA.INV.json` (the `beta_gamma_stats_family` module).
- OxFunc `crates/oxfunc_core/src/functions/beta_gamma_stats_family.rs` and
  `special_math_common.rs` at commit `473efa3` — `gamma_inv_kernel`, its `p >= 1 → #NUM!` guard
  with the `BUG-FUNC-039` live-Excel note, the bracket-expansion loop, and `bisect_inverse`'s
  float-lattice bisection with its publish-high rule.
