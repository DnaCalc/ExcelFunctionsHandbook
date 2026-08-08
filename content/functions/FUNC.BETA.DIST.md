---
schema: efh.function-page/v1
function_id: FUNC.BETA.DIST
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0005
  - EV-DIST-0011
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - Arguments
  - The cumulative flag, and why the two forms are different functions
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: beta_gamma_stats_family
role_in_family: >-
  The family's forward beta CDF and density, and the substrate the whole module rests on: the
  regularized incomplete beta function that F, t and the binomial identities are all expressed
  through.
---

# BETA.DIST

## What it computes

`BETA.DIST(x, alpha, beta, cumulative, [A], [B])` evaluates the beta distribution with shape
parameters \(\alpha,\beta > 0\), rescaled from its natural support \([0,1]\) to \([A, B]\).

Write the standardised variable

    z = (x − A) / (B − A),        z ∈ [0, 1]

Then:

**Density** (`cumulative` = FALSE)

    f(x; α, β, A, B) = z^{α−1} (1 − z)^{β−1} / ( B(α, β) · (B − A) )

**Cumulative distribution** (`cumulative` = TRUE)

    F(x; α, β, A, B) = I_z(α, β)
                     = (1 / B(α, β)) ∫₀^z t^{α−1} (1 − t)^{β−1} dt

where \(B(\alpha,\beta) = \Gamma(\alpha)\Gamma(\beta)/\Gamma(\alpha+\beta)\) is the beta
function and \(I_z(\alpha,\beta)\) is the **regularized incomplete beta function**. The
\(1/(B-A)\) factor on the density is the Jacobian of the rescaling; the CDF needs no such
factor, which is why the two forms scale differently and why a reader checking one against the
other by numerical differentiation must remember it.

**Domain and range.** \(\alpha > 0\), \(\beta > 0\), \(A < B\), \(x \in [A, B]\). The CDF maps
\([A,B]\) onto \([0,1]\), is non-decreasing, and satisfies \(F(A) = 0\), \(F(B) = 1\). The
density is non-negative and integrates to 1 over \([A,B]\); it is **not** bounded by 1 and can
be arbitrarily large.

**Poles and shape.** As \(z \to 0^+\) the density behaves like \(z^{\alpha-1}\): it diverges for
\(\alpha < 1\), equals \(\beta/(B-A)\cdot 1\) in the limit for \(\alpha = 1\), and vanishes for
\(\alpha > 1\). Symmetrically at \(z \to 1^-\) with \(\beta\). So \(\alpha,\beta < 1\) gives a
U-shaped density with an integrable singularity at each end — a real, reachable configuration,
not a pathology, and the one where any implementation's density is hardest to evaluate.

**The symmetry.** \(I_z(\alpha,\beta) = 1 - I_{1-z}(\beta,\alpha)\). This identity is the
backbone of every serious implementation: it lets an algorithm always evaluate on the side
where the series or continued fraction converges quickly, and it is why an implementation's
error profile is usually discontinuous across \(z \approx \alpha/(\alpha+\beta)\).

**Moments.** Mean \(A + (B-A)\,\alpha/(\alpha+\beta)\); variance
\((B-A)^2 \alpha\beta / \big((\alpha+\beta)^2(\alpha+\beta+1)\big)\).

**Why this function matters beyond itself.** \(I_z\) is the common substrate of a large part of
the statistical lane. The F distribution, Student's t, and the binomial tail are all expressible
through it:

    binomial tail:  Σ_{j=k}^{n} C(n,j) p^j (1−p)^{n−j} = I_p(k, n−k+1)          [A&S 26.5.24]
    F distribution: P(F ≤ f; d₁, d₂) = I_{d₁f/(d₁f+d₂)}(d₁/2, d₂/2)
    Student's t:    P(|T| ≥ t; ν)    = I_{ν/(ν+t²)}(ν/2, 1/2)

An error in the incomplete beta is therefore not one function's error. This is exactly why the
reference engine implements F and t through the same routine and why the Handbook's evidence
records for this area group several surfaces together.

## Arguments

| Argument | Meaning | Admissible | Default |
|---|---|---|---|
| `x` | The point of evaluation | \(A \le x \le B\) | — |
| `alpha` | First shape parameter | \(> 0\) | — |
| `beta` | Second shape parameter | \(> 0\) | — |
| `cumulative` | CDF if TRUE, density if FALSE | logical / numeric | — required, not optional |
| `A` | Lower bound of the support | \(< B\) | `0` |
| `B` | Upper bound of the support | \(> A\) | `1` |

Microsoft documents `x` as "The value between A and B at which to evaluate the function",
`alpha` and `beta` each as "A parameter of the distribution", and `cumulative` as "A logical
value that determines the form of the function. If cumulative is TRUE, BETA.DIST returns the
cumulative distribution function; if FALSE, it returns the probability density function."
`A` and `B` are "A lower bound to the interval of x" and "An upper bound to the interval of x",
and: "If you omit values for A and B, BETA.DIST uses the standard cumulative beta distribution,
so that A = 0 and B = 1".

Note that `cumulative` is **required** here, unlike some other distribution surfaces where it is
optional. The reference engine declares an arity of 4 to 6 accordingly. The flag is converted by
the ordinary nonzero-is-true rule, so any nonzero number selects the CDF.

`alpha` and `beta` are *not* required to be integers. Non-integer shapes are the normal case for
a Bayesian posterior and are fully in the domain; only the algorithm's difficulty changes.

## The cumulative flag, and why the two forms are different functions

`BETA.DIST(x, α, β, TRUE, …)` and `BETA.DIST(x, α, β, FALSE, …)` do not merely differ by an
integration. In the reference engine they take entirely different code paths with different
failure modes:

- The **CDF** path evaluates the regularized incomplete beta directly. The reference engine
  routes *every* shape through one identified algorithm; its source records that a
  binomial-sum shortcut for integer shapes was measured against a live Excel oracle and
  **removed** because it lost accuracy rather than gaining it. That is an implementation
  decision recorded in the reference engine, not a statement about Excel's internals.
- The **density** path is evaluated in logarithms:
  \(\exp\!\big((\alpha-1)\ln z + (\beta-1)\ln(1-z) - \ln B(\alpha,\beta) - \ln(B-A)\big)\),
  and returns `#NUM!` if the exponential is not finite. So the density can error where the CDF
  cannot, at exactly the singular configurations described above.

A reader who checks one against the other should expect them to disagree at the level a
numerical derivative disagrees with an analytic one — that is not evidence of a defect in
either.

## Result and edge cases

Returns `Number`.

- **`x = A` or `x = B`.** The CDF returns exactly `0` and exactly `1`. The density at those
  points is the interesting case: finite and generally nonzero when the corresponding shape
  parameter is 1, zero when it exceeds 1, and infinite when it is below 1 — where the reference
  engine returns `#NUM!` rather than an infinity.
- **`alpha = beta = 1`.** The distribution is uniform on \([A,B]\): the density is
  \(1/(B-A)\) everywhere and the CDF is \(z\). This is the cheapest exactness probe the
  function has, because the true answer is a simple rational number.
- **`alpha = beta`.** The distribution is symmetric about the midpoint, so
  \(F(\text{mid}) = 1/2\) exactly. A second cheap exactness probe.
- **Very small or very large shapes.** \(\alpha, \beta \to 0^+\) concentrates mass at the two
  endpoints; \(\alpha,\beta \to \infty\) concentrates it at the mean and makes the density
  narrow and tall. Both directions stress the algorithm; neither is out of domain.
- **`A` and `B` reversed or equal.** `#NUM!` — see errors.
- **Text that looks numeric** in any slot is coerced by the ordinary rules; the reference
  engine's own tests exercise `"0.5"` in the `x` slot and accept it.
- **Arrays.** The modern dotted surface lifts natively
  (`lift_broadcast_profile: surface_native`). The legacy `BETADIST` alias does not: the
  reference engine records it as scalar-shaped, broadcasting only over its first three argument
  positions, and attributes that distinction to observation against a named live Excel build.
  This is a real behavioural difference between the alias and the modern name.

## Errors

As documented by Microsoft on the `BETA.DIST` page:

| Error | Condition |
|---|---|
| `#VALUE!` | Any argument is nonnumeric |
| `#NUM!` | `alpha` ≤ 0 or `beta` ≤ 0 |
| `#NUM!` | `x` < `A`, `x` > `B`, or `A` = `B` |

The reference engine at `473efa3` produces those three and adds two the documentation does not
mention:

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | `B` < `A` (not merely equal) | reference engine |
| `#VALUE!` | A shape or bound argument is non-finite | reference engine |
| `#NUM!` | The density form overflows to a non-finite value | reference engine |

The last row is the one to know: at a singular endpoint with a shape below 1 the true density is
\(+\infty\), and the reference engine reports `#NUM!` rather than an infinity. Microsoft's page
does not address it.

## Relationships

- **[BETA.INV](FUNC.BETA.INV.md)** — the inverse. Microsoft's `BETA.INV` page states the
  dependency explicitly: "Given a value for probability, BETA.INV seeks that value x such that
  BETA.DIST(x, alpha, beta, TRUE, A, B) = probability. Thus, precision of BETA.INV depends on
  precision of BETA.DIST." That sentence is the single most useful documented fact about the
  pair, and it means every finding on this page is also a finding about the inverse.
- **`BETADIST`** — the legacy compatibility name, `BETADIST(x, alpha, beta, [A], [B])`, with no
  `cumulative` argument: it is the CDF only. The reference engine implements it by calling the
  same kernel with `cumulative` forced true, but that is an implementation choice, **not a
  proof that Excel computes the two identically**. The Handbook's evidence record
  `EV-DIST-0011` is precisely about this question for a group of legacy/modern forward-CDF
  pairs; read it before assuming the alias is the same computation. Note also the lift
  difference recorded above — the two names already differ in array behaviour, whatever they do
  numerically.
- **`GAMMA.DIST`** — the other half of this module, and the beta distribution's limit: if
  \(X \sim \text{Beta}(\alpha,\beta)\) then \(\beta X \to \text{Gamma}(\alpha)\) as
  \(\beta \to \infty\).
- **`F.DIST`, `T.DIST`, `BINOM.DIST`, `CHISQ.DIST`** — all expressible through \(I_z\), as shown
  above. They are implemented in different modules in the reference engine, which means the
  identities are *not* enforced by construction and can be checked as metamorphic tests.
- **Confused with**: `BETA.DIST` with `cumulative` omitted. It cannot be omitted; the
  `BETADIST`-shaped habit of leaving it out produces an arity error, and reaching for the
  legacy name to avoid typing it silently changes the array behaviour.

## Numerical notes

The regularized incomplete beta function is one of the genuinely hard special functions, and
the difficulty is not in the formula but in the fact that no single expansion works over the
whole \((z, \alpha, \beta)\) space.

**The classical toolkit.**

- **Series.** \(I_z(\alpha,\beta)\) has a hypergeometric series expansion
  (Abramowitz & Stegun 26.5.4 and 6.6) that converges rapidly only when \(z\) is small relative
  to \(\alpha/(\alpha+\beta)\).
- **Continued fraction.** A&S 26.5.8 gives the standard continued fraction; *Numerical Recipes*
  §6.4 implements it as `betacf` with the Lentz evaluation, and wraps it in `betai` with the
  standard symmetry switch: use \(I_z(\alpha,\beta)\) when
  \(z < (\alpha+1)/(\alpha+\beta+2)\), otherwise compute \(1 - I_{1-z}(\beta,\alpha)\). The
  switch is not an optimisation — the continued fraction simply does not converge usefully on
  the wrong side.
- **Asymptotic expansions.** For large \(\alpha+\beta\) neither the series nor the fraction is
  practical, and one needs asymptotics in terms of the incomplete gamma or the error function.
- **The reference implementation of record.** DiDonato and Morris's **ACM TOMS Algorithm 708,
  `BRATIO`** ("Algorithm 708: Significant Digit Computation of the Incomplete Beta Function
  Ratios", *ACM TOMS* 18, 1992) computes \(I_z(\alpha,\beta)\) **and its complement together**,
  selecting among about a dozen expansions by region. Computing both tails simultaneously is
  the whole point: it avoids ever forming \(1 - p\) in floating point, which is where the
  accuracy of a small upper tail is destroyed. The reference engine at `473efa3` routes this
  family through a `bratio` implementation for exactly this reason, and its source comments
  record that the accurate complement pair — not \(1-z\) — is what its F and t wrappers pass in.

**Where naive evaluation loses.**

1. **Forming the complement.** Computing an upper tail as `1 - lower_tail` loses all relative
   precision once the lower tail is near 1. Any tail probability smaller than about \(2^{-53}\)
   computed that way is pure noise. This is the single most important structural point about
   the whole distribution lane and the reason `.RT` surfaces exist as separate functions rather
   than as `1 - ` wrappers.
2. **The log-beta normaliser.** \(\ln B(\alpha,\beta) = \ln\Gamma(\alpha) + \ln\Gamma(\beta) -
   \ln\Gamma(\alpha+\beta)\) is a difference of large, nearly equal quantities when \(\alpha\)
   and \(\beta\) are both large — the classic cancellation that motivates the dedicated
   `lbeta`/`stirlerr` routines in Cephes, Boost and R rather than three calls to `lgamma`.
3. **The density in logarithms.** Evaluating \(z^{\alpha-1}\) directly overflows or underflows
   long before the density does; the logarithmic form the reference engine uses avoids that but
   pays with a `ln`/`exp` round trip whose relative error is roughly the *absolute* error of the
   logarithm times the magnitude of the exponent. For large \(\alpha\) that is a real loss.
4. **Endpoint behaviour.** Near \(z=0\) with \(\alpha\) near 1, and near \(z=1\) with \(\beta\)
   near 1, the answer is small and the algorithm's region selection changes; error profiles
   published for this function characteristically show a seam there.

**Excel's statistical lane specifically.** Morten Welinder's work on Gnumeric's statistical
functions, and the published accuracy critiques of Excel's statistical functions that ran
alongside it, are the standard entry point to the record of where spreadsheet incomplete-beta
implementations have historically failed. Nothing on this page asserts what Excel does
internally.

## What has not been checked

No Handbook vector suite exists for `BETA.DIST` — `vectors/` publishes none for any function —
so no claim on this page carries the suite axis that `CHARTER.md` rule 7.2 requires.

Two evidence records do list this surface among their subjects, and the evidence layer renders
their figures and scope mechanically beside this page. In prose:

- **`EV-DIST-0005`** is an open-discrepancy record. Its own reader-facing summary is at pains
  to separate a per-surface `BETA.DIST` figure from a group total spanning three surfaces —
  the group total is not a `BETA.DIST` figure and must not be read as one. It also carries
  something rarer and more interesting than a pass rate: a designed deep-tail battery on which
  *Excel itself* is measured against the true value, and found to sit further from truth than
  the candidate realizations of the identified algorithm sit from each other. Read plainly,
  that says the departure at that configuration is the algorithm's, not a rounding accident —
  and it is the kind of finding a compatibility implementation must reproduce rather than
  correct.
- **`EV-DIST-0011`** is an alias-pairing record covering a group of legacy/modern forward-CDF
  pairs, on which `BETA.DIST` appears as a *contrast* subject rather than as a measured one.
  Its load-bearing limit is stated in the record itself and repeated here because it governs
  how this page may be read: the pairing covers **forward CDFs only**, and no inverse pair is
  established, so nothing about `BETA.DIST`/`BETADIST` licenses any inference about
  `BETA.INV`/`BETAINV`.

What that leaves unchecked: the density form entirely, every error condition, the `A`/`B`
rescaling, the array-lift difference between the modern and legacy names, and the coercion
behaviour of every argument slot.

Inputs worth probing first:

1. **`BETA.DIST(0.5, 1, 1, TRUE)` and `BETA.DIST(0.5, 1, 1, FALSE)`.** The uniform case, where
   the true answers are `0.5` and `1` exactly. Any departure is a pure implementation artefact
   and is visible without a high-precision oracle.
2. **`BETA.DIST(x, a, a, TRUE)` at the midpoint** for several `a`, where the true answer is
   `0.5` by symmetry. The cheapest available accuracy probe on non-trivial shapes.
3. **The deep upper tail**: `BETA.DIST(x, α, β, TRUE)` at \(z\) close to 1 with large \(\beta\),
   compared against `1 - BETA.DIST` of the mirrored configuration. This is where the
   complement-formation loss described above either appears or does not, and it is the probe
   that distinguishes a `bratio`-style implementation from a naive one.
4. **The singular endpoints**: `BETA.DIST(0, 0.5, 0.5, FALSE)` and `BETA.DIST(1, 0.5, 0.5,
   FALSE)`, where the true density is infinite and the reference engine returns `#NUM!`. Nothing
   documents this case.
5. **`A` and `B` present**: the same distribution evaluated on \([0,1]\) and on \([A,B]\) with
   \(x\) rescaled, to confirm the Jacobian is applied to the density and not to the CDF.
6. **`A = B` and `B < A`**, the documented and undocumented halves of the bound check.
7. **A large-shape pair**, \(\alpha = \beta = 10^4\), where the log-beta cancellation described
   above dominates and where a three-`lgamma` implementation and a dedicated `lbeta` diverge.
8. **The alias pair on array arguments**: the same call to `BETA.DIST` and `BETADIST` with an
   array in the first argument, which the reference engine says behaves differently and which
   is cheaper to check than any numeric question.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| regularized incomplete beta | \(I_z(\alpha,\beta)\); the beta CDF on \([0,1]\) |
| shape parameters | \(\alpha, \beta\); both strictly positive, not required to be integers |
| standardised variable | \(z=(x-A)/(B-A)\), the point mapped back onto \([0,1]\) |
| accurate complement pair | Computing a tail and its complement together rather than as \(1-p\) |
| symmetry switch | Using \(I_z(\alpha,\beta) = 1-I_{1-z}(\beta,\alpha)\) to evaluate on the convergent side |
| reference engine | OxFunc at commit `473efa3` |

## Sources

- Microsoft, "BETA.DIST function" —
  <https://support.microsoft.com/en-us/office/beta-dist-function-11188c9c-780a-42c7-ba43-9ecb5a878d31>
  (syntax; the six argument descriptions; the nonnumeric `#VALUE!`; the `alpha`/`beta` ≤ 0
  `#NUM!`; the `x < A`, `x > B`, `A = B` `#NUM!`; and the A = 0, B = 1 defaults).
- Microsoft, "BETA.INV function" —
  <https://support.microsoft.com/en-us/office/beta-inv-function-e84cb8aa-8df0-4cf6-9892-83a341d252eb>
  (quoted here for the documented dependency of `BETA.INV`'s precision on `BETA.DIST`).
- Handbook evidence records `EV-DIST-0005` and `EV-DIST-0011`, which list this surface among
  their subjects; their figures, scope and reader warnings are rendered by the evidence layer.
- OxFunc `crates/oxfunc_core/src/functions/beta_gamma_stats_family.rs` and
  `crates/oxfunc_core/src/functions/special_math_common.rs` at commit `473efa3` — the
  `regularized_beta` routing decision, the logarithmic density path and its `#NUM!` guard, the
  bound validation, and the recorded lift difference between the modern and legacy names.
- OxFunc `docs/function-lane/FUNCTION_SLICE_STATISTICAL_DISTRIBUTIONS_AND_COMPAT_A_CONTRACT_PRELIM.md`
  sections 3 and 6 — the family contract and its runtime anchors.
- M. Abramowitz and I. A. Stegun, *Handbook of Mathematical Functions*, chapter 6.6 (incomplete
  beta function) and chapter 26.5 (beta distribution; 26.5.4 series, 26.5.8 continued fraction,
  26.5.24 binomial identity).
- A. R. DiDonato and A. H. Morris, "Algorithm 708: Significant Digit Computation of the
  Incomplete Beta Function Ratios", *ACM Transactions on Mathematical Software* 18 (1992) — the
  `BRATIO` algorithm computing both tails together.
- W. H. Press et al., *Numerical Recipes*, §6.4 (`betai`/`betacf` and the symmetry switch).
- M. Welinder's work on the Gnumeric statistical functions and the published critiques of
  spreadsheet statistical accuracy — the standard entry point to this literature.
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md`.
