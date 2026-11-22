---
schema: efh.function-page/v1
function_id: FUNC.NORM.INV
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0018
open_problems: []
references:
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 26, 26.2.22-26.2.23 (rational approximations to the normal quantile)"
    role: "The classical rational approximations to the inverse normal probability integral"
  - work: "M. J. Wichura, Algorithm AS 241: The percentage points of the normal distribution"
    locator: "Applied Statistics 37 (1988) 477-484 (PPND7 / PPND16)"
    role: "The standard double-precision normal quantile algorithm"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - The documented algorithm
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Notes
  - Page vocabulary
  - Sources
family: normal_log_family
role_in_family: >-
  The general two-parameter normal quantile; the member whose own documentation makes its
  accuracy explicitly derivative of NORM.DIST's, by naming an iterative search over it.
---

# NORM.INV

## What it computes

`NORM.INV(probability, mean, standard_dev)` returns the quantile of the normal distribution
with mean `μ` and standard deviation `σ` at probability `p` — the value `x` such that a normal
random variable falls below `x` with probability exactly `p`:

    NORM.INV(p, μ, σ) = x   where   F(x; μ, σ) = p

Because the normal CDF standardizes, the whole function reduces to the standard normal
quantile — the **probit** function — composed with an affine map:

    x = μ + σ · Φ⁻¹(p)
      = μ + σ √2 · erf⁻¹(2p − 1)

**Domain and range.** `p` must lie strictly inside `(0, 1)`; `p ≤ 0` and `p ≥ 1` are documented
`#NUM!` conditions, and `σ ≤ 0` is another. On `(0, 1)` the function is a strictly increasing
bijection onto the whole real line — so the *range is unbounded*, and it is unbounded in a very
sharp way: as `p → 0⁺` and `p → 1⁻` the result runs to `∓∞`. There are no poles in the interior
and no branch cuts; `Φ⁻¹` is analytic on `(0, 1)`.

**The endpoints are the entire difficulty.** Differentiating the inverse relation,

    d/dp Φ⁻¹(p) = 1 / φ(Φ⁻¹(p))

and `φ` vanishes faster than any power at `±∞`, so the derivative **blows up superpolynomially**
at both ends. Concretely, near `p = 0`,

    Φ⁻¹(p)  ~  −√( 2 ln(1/p) − ln(2 ln(1/p)) − ln(2π) )      as p → 0⁺

so an absolute error `δ` in `p` produces an absolute error of roughly `δ / φ(z)` in the answer,
which for `z = −6` is a factor of about `10^8`. **The inverse normal is ill-conditioned at the
tails by nature, not by implementation.** No algorithm can do better than the conditioning
allows, and the practical consequence is that `NORM.INV` at extreme `p` is only as good as the
representation of `p` itself.

**Special values and identities.**

    Φ⁻¹(1/2) = 0        exactly, so NORM.INV(0.5, μ, σ) = μ
    Φ⁻¹(p) = −Φ⁻¹(1 − p)                the reflection identity
    NORM.INV(Φ(x), μ, σ) = x            round-trip in x
    Φ(NORM.INV(p, 0, 1)) = p            round-trip in p

`NORM.INV(0.5, μ, σ) = μ` is the one input whose answer is exactly nameable and it is the
cheapest possible test of this surface.

## Arguments

Microsoft's page gives three required arguments:

| Argument | Meaning | Required |
|---|---|---|
| `probability` | "A probability corresponding to the normal distribution" | yes |
| `mean` | "The arithmetic mean of the distribution" | yes |
| `standard_dev` | "The standard deviation of the distribution" | yes |

Microsoft further notes that with `mean = 0` and `standard_dev = 1`, `NORM.INV` "uses the
standard normal distribution".

The pinned catalogue supplies `NORM.INV(probability, mean, standard_dev)` with three required arguments.

## The documented algorithm

Microsoft's `NORM.INV` page states something that documentation of a mathematical function very
rarely states: **how it is computed.**

> "Given a value for probability, NORM.INV seeks that value x such that
> NORM.DIST(x, mean, standard_dev, TRUE) = probability."
>
> "Thus, precision of NORM.INV depends on precision of NORM.DIST."

The companion `NORM.S.INV` page says the same thing about its own pair and adds the phrase
"uses an iterative search technique".

Three consequences follow, and the Handbook states them as consequences of the *documentation*,
not as observations of Excel:

1. **`NORM.INV` is documented as a root-finder over `NORM.DIST`, not as a direct quantile
   approximation.** That is architecturally different from every standard library
   implementation, which evaluates a rational approximation to `Φ⁻¹` directly.
2. **Its accuracy is documented as bounded above by `NORM.DIST`'s.** Whatever error the forward
   CDF carries is inherited, and then divided by `φ(z)` through the conditioning above — so the
   inherited error is *amplified* in the tails, not merely passed along.
3. **Round-trip consistency is expected to be unusually good, and monotonicity is not
   guaranteed.** A search that terminates on a tolerance will reproduce `p` well by
   construction, while adjacent `p` values can land on the same or non-monotone `x` if the
   termination rule is coarse. Those are opposite predictions from the ones a direct rational
   approximation makes, which is what makes them worth testing.

Whether Excel today actually performs an iterative search is **not** something this page
asserts. The documentation says so; the Handbook has not observed it, and no evidence record in
its possession identifies this surface's op-graph.

## Result and edge cases

Returns `Number`.

- **`p = 0.5`** returns `μ` exactly, for every admissible `σ`.
- **`p` at the ends.** `p ≤ 0` and `p ≥ 1` are documented `#NUM!`. The smallest positive
  double is about `4.9 × 10⁻³²⁴`, whose quantile is about `−38.4`; the largest `p` strictly
  below `1` is `1 − 2⁻⁵³`, whose quantile is about `8.13`. **The reachable range of
  `NORM.INV(p, 0, 1)` is therefore roughly `[−38.4, +8.13]` and is wildly asymmetric** —
  because probabilities near `1` are represented with absolute spacing `2⁻⁵³` while
  probabilities near `0` are represented with relative precision all the way into the
  subnormals. This asymmetry is a fact about binary64, not about the normal distribution, and
  it is the reason serious tail work is done with `Φ(−z)` rather than `1 − Φ(z)`.
- **`σ ≤ 0`** is a documented `#NUM!`, `σ = 0` included.
- **`standard_dev` very large or `mean` very large**: the result is `μ + σ z` and can overflow;
  what Excel publishes at overflow is not established here.
- **Arrays.** The recorded profile is `Custom`; elementwise lifting is not settled.

Empty, missing and error arguments follow the shared call model; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

As documented by Microsoft on the `NORM.INV` page (retrieved for this page):

| Error | Documented condition |
|---|---|
| `#VALUE!` | Any argument is nonnumeric |
| `#NUM!` | `probability ≤ 0` or `probability ≥ 1` |
| `#NUM!` | `standard_dev ≤ 0` |

Note that this page's `#VALUE!` row says "any argument", where the sibling `NORM.DIST` page
names only `mean` and `standard_dev`. The two pages describe the same coercion behaviour with
different completeness; `NORM.INV`'s wording is the better one.

Error values in any argument propagate under the ordinary coercion rules.

## Relationships

- **`NORMINV`** is the legacy spelling, with the same argument count in the reference engine's
  registry — three. This modernization is a pure rename at the signature level. **That is not
  a claim that the two compute the same bits**: Excel dispatches the compatibility functions as
  their own surfaces, and identity requires evidence. Here the Handbook can be unusually
  specific about the absence: `EV-DIST-0018`'s own status note records that `NORMDIST` and
  `NORMINV` are **not** among the surfaces it measured. The legacy-modern pairing for this
  function is, on the Handbook's record, unmeasured on both sides.
- **[NORM.DIST](FUNC.NORM.DIST.md)** is the forward function and, per Microsoft's own text, the
  thing `NORM.INV` searches over. The round trip in both directions is the natural test pair.
- **[NORM.S.INV](FUNC.NORM.S.INV.md)** is the standardized case, `NORM.INV(p, 0, 1)`.
- **`LOGNORM.INV`** is the same quantile after an exponential change of variable, and shares
  this function's implementing module in the reference engine along with the rest of the normal
  and lognormal group.
- **`CONFIDENCE.NORM`** is a direct consumer: a confidence half-width is `NORM.S.INV` of a
  tail probability, scaled.
- **`STANDARDIZE`** is the affine map this function composes with, run the other way.
- **Confused with**: `NORM.DIST` itself, by readers who want "the normal function" and pick the
  wrong direction; and `NORM.INV(p, μ, σ)` used with `p` expressed as a percentage, where `95`
  is a `#NUM!` and `0.95` is what was meant.

## Numerical notes

**1. The conditioning is the headline and it is not fixable.** As derived above, the derivative
of `Φ⁻¹` is `1/φ(Φ⁻¹(p))`, which grows without bound at both ends. Any error in `p` — including
the rounding that put `p` into a double in the first place — is multiplied by that factor. At
`p = 10⁻¹⁰` the amplification is enormous. A function that is *correctly rounded* at every
input is still delivering an answer whose meaning is limited by the input's representation.
The right way to state accuracy for this function is in terms of the round trip, not the
forward error.

**2. Direct approximation is the standard route, and it is not what the documentation
describes.** The reference algorithm is Wichura's **AS 241** (*Applied Statistics* 37, 1988),
whose `PPND16` gives about 16 significant digits over the whole representable range using three
rational approximations selected by region: `|p − ½| ≤ 0.425` on the central branch, then two
tail branches parameterized in `r = √(−ln min(p, 1−p))`. This is what R's `qnorm`, Boost, and
most statistical libraries use. Earlier and weaker: A&S 26.2.22 (about `3 × 10⁻³` absolute) and
26.2.23 (about `4.5 × 10⁻⁴`), the Beasley–Springer algorithm AS 111 with Moro's tail
modification, and Acklam's widely copied approximation — all fine for graphics and inadequate
for a reference. Anyone implementing this function today should start from AS 241.

**3. A root-finder over the CDF is a defensible design with distinctive failure modes.** If the
documented description is accurate, the properties to expect are:

- **Excellent round-trip in `p`** — the search terminates on that criterion, so
  `NORM.DIST(NORM.INV(p, …), …, TRUE) ≈ p` by construction;
- **Possible non-monotonicity** in `x` across adjacent `p`, if the termination tolerance is
  loose relative to the local `dx/dp`;
- **Degrading accuracy in the tails** at exactly the rate the conditioning predicts, because
  each evaluation of the forward CDF carries its own error and the search cannot beat it;
- **Sensitivity to the bracketing strategy** at extreme `p`, where the initial bracket must
  already reach `−38` on one side.

Each of those is testable, and together they discriminate a search from a direct rational
approximation. That is the highest-value experiment on this page.

**4. Newton refinement is the bridge between the two designs.** A standard high-accuracy
implementation evaluates a rational approximation and then applies one or two Halley or Newton
steps using the exact derivative `φ(z)`:

    z ← z − (Φ(z) − p) / φ(z)                    Newton
    z ← z − u / (1 + z·u/2),  u = (Φ(z) − p)/φ(z)   Halley

Each step roughly squares (Newton) or cubes (Halley) the accuracy, and the derivative is
available in closed form — which is why the inverse normal is one of the pleasanter quantiles to
refine. An implementation that does this *is* using the forward CDF, which means its accuracy
is also derivative of the CDF's, exactly as Microsoft's page says. The two descriptions are not
as far apart as they first look.

**5. Argument reduction.** `μ + σ z` is a fused-multiply-add opportunity and a cancellation
opportunity: when `μ` is large and `σ z` is small, the result loses the information in `z`
entirely. This is the same ceiling described on [NORM.DIST](FUNC.NORM.DIST.md), and it is
invisible to any test that uses `μ = 0`.

**6. The published assessment record.** McCullough and Wilson's series in *Computational
Statistics & Data Analysis*, and Knüsel's assessments of Excel's distribution functions, are
the standing published examinations of this lane, with revisions reported around Excel 2003 and
again when the dotted names arrived. Morten Welinder's Gnumeric work is the standard practical
account of implementing this family in a spreadsheet, including the inverses. Named as
literature; no figure from them is restated here.

## What has not been checked

`EV-DIST-0018` names `NORM.INV` among its subjects. It is a re-sweep of **pinned witnesses, one
row per surface**, and its own reader warning states that its group total is not any surface's
pass rate — so the normal group was measured at pinned points and **this surface was not
measured separately** by that figure. The record additionally records that `NORMINV` is not in
that set at all, so the legacy counterpart is unmeasured.

No Handbook vector suite exists for `NORM.INV`. No Handbook measurement of this surface exists;
`EV-DIST-0018` is upstream OxFunc work the Handbook has not re-verified. No evidence record in
the Handbook's possession identifies this surface's op-graph, so the documented iterative-search
description is unconfirmed as well as unrefuted.

Microsoft's documented behaviour above, including the iterative-search statement, was retrieved
from the `NORM.INV` page.

Inputs worth probing first:

1. **`NORM.INV(0.5, μ, σ)` for several `μ` and `σ`.** Must return `μ` exactly. One cell,
   nameable answer, no oracle needed.
2. **The reflection identity** `NORM.INV(p, 0, 1) = −NORM.INV(1−p, 0, 1)` across a spread of
   `p`. Oracle-free, and it fails visibly at a region seam.
3. **The round trip both ways**: `NORM.INV(NORM.DIST(x, 0, 1, TRUE), 0, 1)` against `x`, and
   `NORM.DIST(NORM.INV(p, 0, 1), 0, 1, TRUE)` against `p`, over the whole range. **These two
   are the discriminating experiment.** A documented search should be near-perfect on the
   second and visibly worse on the first; a direct rational approximation behaves the other way
   round. Running both settles what kind of function this is.
4. **Monotonicity across adjacent doubles**: step `p` by one ULP through several regions and
   check that the result never decreases. A search with a loose tolerance shows up here and
   nowhere else.
5. **The extreme ends of the reachable domain**: `p` at the smallest positive double, at
   `2⁻¹⁰⁰⁰`, at `1 − 2⁻⁵³`, and at the first `p` for which the documented `#NUM!` fires. This
   maps the asymmetric reachable range described above and finds the exact boundary.
6. **`NORM.INV(p, 0, 1)` against `NORM.S.INV(p)`**, and `NORM.INV` against `NORMINV`, over the
   same spread — two pairings, both unmeasured on the Handbook's record.
7. **Large `μ` with small `σ`** — say `μ = 1e15`, `σ = 1` — to isolate the final-affine-step
   cancellation from everything else.
8. **`standard_dev` exactly `0` and `probability` exactly `0` and `1`**, confirming the
   documented inclusive boundaries.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `NORM.INV` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-NORM.S.INV-001: NORM.S.INV uses AS241 tails (p < 0.075 and p > 15/16) and three x87-stored rational/Padé pieces for sqrt(2)*erfinv(2p-1) in between

*Detail for matching Excel's bits; inferred; recorded 2026-11-22.* Applies to CONFIDENCE, CONFIDENCE.NORM, LOGINV, LOGNORM.INV, NORM.INV, NORM.S.INV, NORMINV, NORMSINV.

Excel's standard normal inverse (NORM.S.INV and NORMSINV) partitions (0, 1) asymmetrically:
Wichura's AS241 formula runs for p < 0.075 and p > 15/16 using x87 FYL2X + FSQRT stored
sqrt(-ln v), while the middle computes sqrt(2) * erfinv(t) with t = 2*(p - 0.5) across three
pieces: for s = RN(t*t) <= 0.5, a degree-7/7 rational interpolant at the 15 zeros of T15 on [0,
0.73] solved near 100-bit precision; for sqrt(0.5) < |t| <= 0.85, a degree-7/5 rational
interpolant times sqrt(-ln(1 - |t|)); and for 0.85 < t <= 0.875 (p in (0.925, 15/16]), a
degree-9/9 Padé approximant centred at 225/256 solved at 96-bit precision times sqrt(-ln(1 -
t)), storing every Horner product and sum from x87 64-bit precision to binary64.
NORM.INV/NORMINV return mu + sigma*z, LOGNORM.INV/LOGINV return EXP(mu + sigma*z), and
CONFIDENCE/CONFIDENCE.NORM return NORM.S.INV(1 - alpha/2)*sigma/SQRT(size) with stored steps.
See
https://github.com/DnaCalc/DnaCalc/blob/4fb9b728df8a96546ae685589b47e057b7e26771/docs/articles/last-bit-normsinv.md.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=NORM.S.INV(0.9)` | as typed | `1.2815515655446006` (`0x3ff4813c36e26d33`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=NORM.INV(0.9,40,1.5)` | as typed | `41.9223273483169` (`0x4044f60ed2929d1e`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=LOGNORM.INV(0.04,3.5,1.2)` | as typed | `4.051862753430021` (`0x4010351b82777327`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (inferred): Matched across 21,898 of 21,898 full-bank witness rows across all eight
functions in Excel 16.0.20527 (NORMSINV 2,210/2,210, NORM.S.INV 2,206/2,206, NORM.INV
2,915/2,915, NORMINV 2,914/2,914, LOGNORM.INV 2,911/2,911, LOGINV 2,916/2,916, CONFIDENCE
2,910/2,910, CONFIDENCE.NORM 2,916/2,916), plus 32,452 of 32,452 frozen piece-C inputs and
1,230,456 distinct sweep inputs (docs/articles/last-bit-normsinv.md).

Inferred, not measured: The exact polynomial solve precisions (~100 bits for piece A and 96
bits for piece C) and compiled code structure are inferred from output-bit discriminators (such
as p = 0x3fed999bc5f0efd8) and frozen holdout runs rather than source inspection.

Evidence:

- DnaCalc `docs/articles/last-bit-normsinv.md` at `4fb9b728df` — Last-bit investigation of NORMSINV / NORM.S.INV
- DnaCalc `docs/research/normal-inverse-central-2026-11-17.md` at `4fb9b728df` — Piece A, B, and C identification notes and discriminators
- DnaCalc `crates/calc-func/src/ported/normal_log_family.rs`, as241_tail_magnitude; x87_sqrt_of_minus_log at `4fb9b728df` — Production four-piece normal inverse and callers
- DnaCalc `crates/calc-func/witness-status.tsv`, NORM.S.INV; NORM.INV; LOGNORM.INV; CONFIDENCE.NORM at `4fb9b728df` — 21,898 of 21,898 full-bank rows matched
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/prefix_metadata.capture.json`, sheet prefix_metadata row 164 column 1 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/prefix_metadata.capture.json`, sheet prefix_metadata row 162 column 1 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/prefix_metadata.capture.json`, sheet prefix_metadata row 148 column 1 at `4fb9b728df` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| probit | The standard normal quantile `Φ⁻¹`; this function's kernel |
| conditioning | The factor `1/φ(z)` by which an error in `p` is amplified in the answer |
| reachable range | The set of results attainable given that `p` must be a double in `(0, 1)` |
| documented search | Microsoft's stated description of `NORM.INV` as an iterative search over `NORM.DIST` |
| round trip | Composing this function with `NORM.DIST` in either order and comparing to the input |

## Sources

- Microsoft, "NORM.INV function" —
  <https://support.microsoft.com/en-us/office/norm-inv-function-54b30935-fee7-493c-bedb-2278a9db7e13>
  (syntax; the three required arguments and their descriptions; the `#VALUE!` condition on any
  nonnumeric argument; the `#NUM!` conditions on `probability` and `standard_dev`; the statement
  that `NORM.INV` seeks the `x` at which `NORM.DIST` equals the probability; and "precision of
  NORM.INV depends on precision of NORM.DIST"). Retrieved for this page.
- Microsoft, "NORM.S.INV function" —
  <https://support.microsoft.com/en-us/office/norm-s-inv-function-d6d556b4-ab7f-49cd-b526-5a20918452b1>
  (the companion statement, which adds "uses an iterative search technique"). Retrieved.
- Handbook evidence record `EV-DIST-0018` — the ten-witness normal-group re-sweep; names
  `NORM.INV` as a subject, excludes `NORMINV`, and carries a reader warning against per-surface
  attribution of its group figure.
- M. J. Wichura, "Algorithm AS 241: The percentage points of the normal distribution",
  *Applied Statistics* 37 (1988) 477–484 — `PPND16`, the standard double-precision route.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 26 §26.2.22–26.2.23 — the
  classical low-order rational approximations, and §26.2.12 for the tail asymptotics that give
  the `√(2 ln(1/p))` leading behaviour.
- J. D. Beasley and S. G. Springer, "Algorithm AS 111", *Applied Statistics* 26 (1977), with
  Moro's tail modification — the earlier standard, named for contrast.
- B. D. McCullough and B. Wilson, *Computational Statistics & Data Analysis* (1999, 2002,
  2005); L. Knüsel on Excel's distribution functions; M. Welinder's Gnumeric statistical work.
  Named as literature; no figure restated.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [About implementation options](../model/07-implementation-options.md).
- [Pinned catalogue record](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/89d3496b32159c5fee04c2244feabd4e7c4ff648/data/functions/FUNC.NORM.INV.json): current signature and argument count, projected from DnaCalc.
- [Current DnaCalc source](https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/crates/calc-func/src/catalogue/distributions.rs): the code quoted in this page’s implementation panel. A source listing does not constitute admission against a published Handbook suite.
