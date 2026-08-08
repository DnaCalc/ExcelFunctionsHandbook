---
schema: efh.function-page/v1
function_id: FUNC.GAMMA
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0015
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Gamma method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.gamma"
    role: "the entire documented surface: a one-line description, one parameter, a Double return, and no Remarks section"
  - work: "Microsoft Support — GAMMA function"
    locator: "https://support.microsoft.com/en-us/office/gamma-function-ce1702b1-cf55-471d-8307-f83be0fc5297"
    role: "the worksheet-surface documentation page; not retrievable at curation time (the host refused the request)"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 6"
    locator: "6.1 — the gamma function: integral definition, recurrence, reflection, duplication, Stirling's series"
    role: "the mathematics this page states"
  - work: "C. Lanczos, 'A precision approximation of the gamma function' (1964)"
    locator: null
    role: "the approximation family the reference engine uses"
  - work: "W. J. Cody, Algorithm 715 / the SPECFUN gamma and lgamma routines"
    locator: null
    role: "the minimax rational approach that is the usual alternative to Lanczos"
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
  The bare gamma function in a module otherwise built around error functions, log-gamma and
  Weibull; the member the landed GAMMALN kernel deliberately does not touch.
---

## What it computes

`GAMMA(x)` returns Euler's gamma function, the analytic continuation of the factorial.

For Re z > 0 it is the Euler integral of the second kind:

    Γ(z)  =  ∫₀^∞  t^{z−1} e^{−t}  dt

and everywhere else it is defined by analytic continuation through the functional equation.

**The identities that matter for a spreadsheet.**

    Γ(z + 1)  =  z · Γ(z)                     (recurrence — the whole function in one line)
    Γ(n)      =  (n − 1)!                     for positive integers n
    Γ(z)·Γ(1 − z)  =  π / sin(π z)            (reflection — how negatives are reached)
    Γ(½)      =  √π
    Γ(2z)     =  2^{2z−1} π^{−½} Γ(z) Γ(z + ½)   (Legendre duplication)

**Domain and range on the real line.** Γ is defined for every real x except x = 0 and the
negative integers, where it has **simple poles**. Between consecutive poles on the negative axis
the function alternates sign: Γ is negative on (−1, 0), positive on (−2, −1), negative on
(−3, −2), and so on, with |Γ| → ∞ at each pole while the local extrema between poles shrink
toward zero as x → −∞.
On the positive axis Γ is smooth, strictly positive, and **not monotone**: it falls
from +∞ at 0⁺ to a single minimum near x ≈ 1.4616 (value ≈ 0.8856) and then rises without
bound. The two exact fixed points Γ(1) = Γ(2) = 1 straddle that minimum.

**Growth.** Stirling's series gives ln Γ(x) ~ (x − ½)ln x − x + ½ln(2π) + 1/(12x) − …, so Γ
grows faster than any exponential. In double precision Γ(x) overflows a little above x ≈ 171.6;
past that the true value exists and is not representable. That single fact — that the *useful
domain* of `GAMMA` in binary64 is roughly (−∞, 171.6] minus the poles — is why `GAMMALN` exists
as a separate function and why almost all serious use goes through the logarithm instead.

**No branch cuts on the real line.** Γ is real-analytic on each interval between poles; the
branch-cut question belongs to the complex function, which Excel does not expose.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `Number` | The value at which to evaluate Γ. Required. Documented only as "the value for which you want to calculate gamma". | — |

One argument; the reference engine declares an arity of exactly 1. The slot is numeric under
ordinary to-number coercion. The reference engine's source records that the GAMMA/GAMMALN
family **accepts logical arguments** (so `TRUE` converts to 1) while the ERF/ERFC family in the
same module rejects them with `#VALUE!` — a per-family split that the documentation does not
mention, attributed upstream to an empirical sweep against Excel 16.0.

## Result and edge cases

Returns `Number`.

- **Positive integers** are the case every reader checks first, and they are exactly
  representable up to 22! or so. Whether an implementation *returns* them exactly is a
  different question, and the reference engine does not always: because it evaluates Γ as
  exp(lnΓ) rather than by a direct recurrence, its answers at some small positive integers are
  not the exact integer factorial. This is visible in the projected battery beside this page.
  It is a fact about the reference engine — no Excel was involved — and it is the most
  legible symptom of the design discussed under **Numerical notes**.
- **The poles** (0 and the negative integers) have no value. The reference engine returns
  `#NUM!` there, using an integrality test with a *relative* tolerance so that a tiny
  non-integer such as −10⁻²⁰⁰ is not mistaken for zero; upstream attributes that refinement to
  the `BUG-FUNC-027` defect stream (class A2). The naive absolute-tolerance test collapses
  near-zero non-integers onto the pole and errors where a value exists.
- **Negative non-integers** are in the domain and the values alternate in sign. Reaching them
  requires the reflection formula, and reflection is where accuracy goes to die; see
  **Numerical notes**.
- **Very small positive x** has Γ(x) ≈ 1/x − γ + …, so the function diverges like a reciprocal.
  For x below about 10⁻³⁰⁸ the reciprocal itself overflows, which is why an argument at the
  bottom of the double range produces an overflow rather than a huge finite number.
- **Above the overflow threshold** (x ≳ 171.6) the true value is not representable. The
  reference engine tests the log against ln(MAX) and returns `#NUM!` rather than +∞.
- **Array arguments** are lifted elementwise by the module's broadcast helper.

## Errors

**Microsoft's VBA reference page for this method has no Remarks section and documents no error
condition at all.** It is the shortest page in this batch: a one-sentence description, one
parameter row, and a `Double` return type. The worksheet-surface page at `support.microsoft.com`
is the fuller source and was not retrievable at curation time, so this page does not quote it
and does not assert its contents from memory.

That is a documentation gap, and it is the honest headline for this section. What the reference
engine does, stated as a fact about the reference engine and not about Excel:

| Error | Condition, in the reference engine |
|---|---|
| `#NUM!` | `x` is zero or a negative integer (a pole), under a relative-tolerance integrality test |
| `#NUM!` | the result would overflow the double range |
| `#NUM!` | `x` is non-finite |
| `#VALUE!` | the argument does not coerce to a number |

Whether Excel agrees with any row of that table is unverified here.

## Relationships

- **[GAMMALN](FUNC.GAMMALN.md) and [GAMMALN.PRECISE](FUNC.GAMMALN.PRECISE.md)** — the natural
  logarithm of Γ, restricted to positive arguments. They exist because Γ overflows and lnΓ does
  not, and they are the right tool for every ratio-of-gammas computation. Note a structural
  fact recorded in the evidence: the log-gamma kernel identified and landed upstream is wired
  **only** to those two surfaces, and deliberately does not change `GAMMA`. The three surfaces
  share a module and do not share their numeric path.
- **`FACT`** — the factorial for non-negative integers. `GAMMA(n+1)` and `FACT(n)` are the same
  number mathematically; whether they are the same computation, and agree in the last bit, is
  unestablished. `FACT` has an exact small-table implementation available to it that `GAMMA`
  does not.
- **`FACTDOUBLE`, `COMBIN`, `PERMUT`, `MULTINOMIAL`** — combinatorial functions expressible
  through Γ, and generally better computed without it.
- **[GAMMA.DIST](FUNC.GAMMA.DIST.md) and [GAMMA.INV](FUNC.GAMMA.INV.md)** — the distribution
  named after the function. They use the *incomplete* gamma function, a different and harder
  object; sharing a name is the extent of the relationship, and the reference engine implements
  them in a different module.
- **`BETA.DIST` / `GAMMALN`** — the beta function B(a,b) = Γ(a)Γ(b)/Γ(a+b) is the standard
  reason people reach for `GAMMA` and the standard place they should have reached for
  `GAMMALN` instead, because each factor can overflow while the ratio is small.
- **Module siblings**: `ERF`, `ERF.PRECISE`, `ERFC`, `ERFC.PRECISE`, `WEIBULL`, `WEIBULL.DIST`
  share the implementing module. Module sharing is an implementation fact and implies nothing
  semantic — and here it comes with an explicit statement that the kernels differ.

## Numerical notes

**The two classical designs.** Serious implementations of Γ come in two families. *Lanczos*
(1964) approximates Γ(z + 1) by a shifted product of a power, an exponential, and a short
rational series in z, with coefficients tuned to a chosen shift; it is elegant, uniform, and
easy to get to roughly working precision. *Cody-style minimax rationals* — the SPECFUN/Algorithm
715 lineage, and what fdlibm and most libm implementations descend from — reduce the argument
onto a small interval by the recurrence and evaluate a minimax rational there, with separate
handling for the large-argument Stirling regime. The minimax approach generally wins on
last-bit accuracy; Lanczos wins on brevity. Boost.Math documents a Lanczos with extended
coefficients as a way to close much of the gap.

**Computing Γ as exp(lnΓ) is the design decision that dominates everything else.** The
reference engine takes that route: it evaluates a Lanczos log-gamma and exponentiates. The cost
is structural and unavoidable. A relative error ε in lnΓ becomes a relative error of about
|lnΓ| · ε in Γ, because d(exp u) / exp u = du. Since lnΓ(x) reaches into the hundreds well
before Γ overflows, an implementation that is accurate to a few ulps in the logarithm can be
wrong by hundreds of ulps in the value. This *ln-amplification* is why exp(lnΓ) is a
convenience implementation and not an accurate one, and it is the reason the exact integer
values are not reproduced: Γ(3) = 2 requires exp(ln 2) to land exactly on 2, which it need not.

A careful implementation therefore does **not** route through the logarithm for moderate
arguments. It uses the recurrence to move x into a small interval around the minimum, evaluates
a minimax rational there, and multiplies the recurrence factors back — all in the value domain,
where no exponential amplification occurs — reserving Stirling for large x.

**Reflection is the second hazard.** For x < ½ the standard route is

    Γ(x)  =  π / ( sin(π x) · Γ(1 − x) )

and it has three separate problems. First, sin(πx) must be evaluated with the argument reduced
exactly, or the answer near a large-magnitude pole is dominated by the reduction error rather
than by anything about Γ. Second, near a pole sin(πx) is tiny and the quotient is
ill-conditioned in exactly the way a simple pole demands: the condition number is unbounded, so
there is a genuine limit to achievable accuracy there, not merely an implementation weakness.
Third, the sign must be recovered separately from the magnitude, since the usual formulation
computes |Γ| through logs and then reattaches the sign of sin(πx) — which is what the reference
engine does.

**The floating-point domain is small.** Roughly (−∞, 171.6] minus the poles, with the useful
positive range ending abruptly. Most computations that reach for Γ (binomial coefficients, beta
functions, distribution normalising constants) are ratios whose *value* is modest even when the
factors are not; those computations belong in log space from the start.

## What has not been checked

`EV-MATH-0015` names `GAMMA` as a subject and it is the only record in the Handbook's collection
that does. What it establishes is unfavourable and current: a fresh live sweep found the
reference engine matching Excel on **none** of the positive-side rows it counted, with large
residuals on the positive side and far larger ones on the negative side, and it records that the
log-gamma kernel landed upstream is wired only to the `GAMMALN` surfaces and therefore did not
change `GAMMA` at all. The record's own figures render mechanically beside this page; this prose
deliberately does not restate them. The correct reading is that `GAMMA` is a **known-open
discrepancy**, not an unmeasured one — which is a different and more useful state than silence.

What does not exist: any Handbook vector suite for `GAMMA`; any identification of the algorithm
Excel actually uses for it; any characterisation of where in the domain the disagreement is
worst beyond the positive/negative split the record names.

Inputs I would probe first, and why:

1. **The small positive integers, 1 through 10.** The exact answers are known, they are exactly
   representable, and any implementation that routes through exp(lnΓ) will miss some of them.
   This is the cheapest possible test that distinguishes a value-domain implementation from a
   log-domain one, and the reference engine already fails it. Learning whether Excel passes it
   would immediately narrow the algorithm space.
2. **Γ(½), Γ(3/2), Γ(5/2)** — the half-integer ladder, whose exact values are known multiples
   of √π. These test the recurrence and the argument reduction independently of the integer case.
3. **The minimum near x ≈ 1.4616**, where the derivative vanishes and the function is locally
   flat, so the *value* is well conditioned but any argument reduction error is invisible —
   a control point that isolates the polynomial from the reduction.
4. **A sweep just below the overflow threshold**, x from 170 to 172. This finds the exact
   threshold Excel uses and whether it returns `#NUM!` or something else, which is a strong
   implementation fingerprint.
5. **The negative axis between poles**, for instance −0.5, −1.5, −2.5, and points approaching
   −3 from both sides. The reflection path is where the record reports the worst residuals, and
   the sign alternation is a correctness check that needs no oracle.
6. **The pole neighbourhood at −10⁻²⁰⁰ and similar tiny non-integers**, which is the exact case
   the reference engine's relative-tolerance integrality test exists for. If Excel errors there,
   the two disagree on the *domain*, not merely on the value.
7. **`GAMMA(n+1)` against `FACT(n)`**, a metamorphic probe requiring no oracle: disagreement
   proves the two surfaces are separate computations in Excel.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| Euler integral of the second kind | The defining integral ∫₀^∞ t^{z−1}e^{−t}dt |
| simple pole | A first-order singularity; Γ has one at 0 and at each negative integer |
| reflection formula | Γ(z)Γ(1−z) = π/sin(πz), the route to negative arguments |
| ln-amplification | The error growth incurred by computing Γ as exp(lnΓ) |
| Lanczos approximation | The shifted-series approximation family the reference engine uses |
| minimax rational | The Cody/SPECFUN-style approach that reduces the argument and fits a rational |

## Sources

- Microsoft Learn, "WorksheetFunction.Gamma method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.gamma>
  (description, the single parameter, the `Double` return type; **no Remarks and no documented
  error condition**). The worksheet-surface page at `support.microsoft.com` was not retrievable
  at curation time, so nothing here is quoted from it.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 6 (§6.1) — the integral
  definition, recurrence, reflection, duplication, the location of the positive-axis minimum,
  and Stirling's series.
- C. Lanczos, "A precision approximation of the gamma function", *SIAM J. Numer. Anal.* B1
  (1964); W. J. Cody, SPECFUN / ACM Algorithm 715; Boost.Math's Lanczos notes; fdlibm
  `e_lgamma_r.c` — the implementation families discussed under **Numerical notes**.
- Handbook evidence record `EV-MATH-0015` — the open-discrepancy status, the live sweep, and the
  statement that the landed log-gamma kernel is not wired to this surface.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.GAMMA.json` and `data/presence/FUNC.GAMMA.json`
  (the `special_dist_family` module, shared with the ERF/ERFC, GAMMALN and WEIBULL surfaces).
- OxFunc `crates/oxfunc_core/src/functions/special_dist_family.rs` at commit `473efa3` — the
  `gamma_kernel`, its Lanczos `ln_gamma_positive`, the reflection branch, the overflow test,
  the relative-tolerance pole test attributed to `BUG-FUNC-027` class A2, and the comment
  recording that GAMMA and the shared internal lgamma are unaffected by the landed GAMMALN
  kernel.
