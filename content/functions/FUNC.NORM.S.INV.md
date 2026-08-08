---
schema: efh.function-page/v1
function_id: FUNC.NORM.S.INV
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0018
open_problems: []
references:
  - work: "M. J. Wichura, Algorithm AS 241: The percentage points of the normal distribution"
    locator: "Applied Statistics 37 (1988) 477-484 (PPND7 / PPND16)"
    role: "The standard double-precision normal quantile algorithm"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 26, 26.2.22-26.2.23"
    role: "The classical rational approximations to the normal quantile"
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
  - Page vocabulary
  - Sources
family: normal_log_family
role_in_family: >-
  The bare probit: the standard normal quantile with no location or scale, and the family's
  single-argument kernel that CONFIDENCE.NORM and the normal interval builders consume directly.
---

# NORM.S.INV

## What it computes

`NORM.S.INV(probability)` returns the quantile of the **standard** normal distribution — mean
`0`, standard deviation `1` — at probability `p`. It is the **probit** function:

    NORM.S.INV(p) = Φ⁻¹(p) = z    where    Φ(z) = p
                  = √2 · erf⁻¹(2p − 1)

**Domain and range.** `p` must lie strictly inside `(0, 1)`; `p ≤ 0` and `p ≥ 1` are documented
`#NUM!`. On that open interval `Φ⁻¹` is a strictly increasing, analytic bijection onto the whole
real line. No poles in the interior, no branch cuts; the singularities sit at the two endpoints,
and they are essential in the practical sense — the function is unbounded there.

**The endpoints are the whole story.** From the inverse-function rule,

    d/dp Φ⁻¹(p) = 1 / φ(Φ⁻¹(p))

and `φ` decays like `e^(−z²/2)`, so the derivative grows superpolynomially at both ends. The
asymptotic form of the function itself is

    Φ⁻¹(p) ~ −√( 2L − ln L − ln(2π) ),   L = ln(1/p),   as p → 0⁺

— a square root of a logarithm, which is why the function is so flat in the middle and so steep
at the edges, and why every good implementation reparameterizes the tails in
`r = √(−ln min(p, 1−p))` rather than in `p`.

**Identities and special values.**

    Φ⁻¹(1/2) = 0                      exactly
    Φ⁻¹(p) = −Φ⁻¹(1 − p)              reflection; exact
    Φ(Φ⁻¹(p)) = p,   Φ⁻¹(Φ(z)) = z    round trips
    Φ⁻¹(0.975) ≈ 1.959963984540054    the two-sided 95% constant

`Φ⁻¹(1/2) = 0` is the only exactly nameable value, and it is this function's cheapest test.
The `0.975` quantile is worth listing because it is the number every confidence interval in
every textbook is built from, and because "1.96" is the rounding of it that people memorize.

**The reachable range is asymmetric, and that is a binary64 fact.** The smallest positive
double is about `4.9 × 10⁻³²⁴`, whose probit is about `−38.5`; the largest double strictly below
`1` is `1 − 2⁻⁵³`, whose probit is about `+8.13`. So `NORM.S.INV` can reach far into the left
tail and barely into the right one — not because the distribution is asymmetric (it is not) but
because probabilities near `1` are stored with absolute spacing `2⁻⁵³` while probabilities near
`0` keep relative precision into the subnormals. Anyone wanting a large positive quantile must
compute `−NORM.S.INV(q)` with `q` the small upper-tail probability.

## Arguments

Microsoft's page gives one required argument:

| Argument | Meaning | Required |
|---|---|---|
| `probability` | "A probability corresponding to the normal distribution" | yes |

The reference engine records an arity of exactly 1 and a `Custom` coercion/lift profile, and
marks the projected signature as a **placeholder** (`signature_placeholder: true`) — the table
above is Microsoft's.

## The documented algorithm

Microsoft's page states, unusually explicitly, how the function is computed:

> "Given a value for probability, NORM.S.INV seeks that value z such that
> NORM.S.DIST(z,TRUE) = probability. Thus, precision of NORM.S.INV depends on precision of
> NORM.S.DIST. NORM.S.INV uses an iterative search technique."

That last sentence is the strongest algorithmic statement Microsoft makes anywhere in the
normal family. Three consequences follow from the *documentation* — the Handbook has not
observed Excel and does not assert that the description is current:

1. **`NORM.S.INV` is documented as a root-finder over `NORM.S.DIST`**, not as a direct rational
   approximation to `Φ⁻¹`. That is a different architecture from every standard statistical
   library.
2. **Its accuracy is documented as derivative of the forward CDF's**, and the conditioning
   factor `1/φ(z)` then amplifies whatever error the CDF carries — most severely at exactly the
   inputs where users care.
3. **Round-trip in `p` should be near-perfect and monotonicity in `z` is not guaranteed.** A
   search terminating on `|Φ(z) − p| < tol` reproduces `p` by construction while permitting
   adjacent `p` values to map to the same or out-of-order `z`. A direct approximation behaves
   the other way round. Testing both directions discriminates the two designs, which is the
   highest-value experiment on this page.

## Result and edge cases

Returns `Number`.

- **`p = 0.5`** returns `0` exactly.
- **`p ≤ 0` and `p ≥ 1`** are documented `#NUM!`. The reference engine's battery renders the
  zero, negative and largest-finite-double rows; their outcomes show beside this page.
- **Logical arguments.** `TRUE` coerces to `1`, which is a documented `#NUM!` boundary, and
  `FALSE` coerces to `0`, likewise. So both logicals are rejected by the domain rule rather
  than by a type rule — an example of coercion succeeding and validation then failing.
- **The two reachable extremes** described above, around `−38.5` and `+8.13`.
- **Arrays.** The recorded profile is `Custom`; elementwise lifting is not settled here.

Empty, missing and error arguments follow the shared call model; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

As documented by Microsoft on the `NORM.S.INV` page (retrieved for this page):

| Error | Documented condition |
|---|---|
| `#VALUE!` | `probability` is nonnumeric |
| `#NUM!` | `probability ≤ 0` or `probability ≥ 1` |

A small documentation defect worth recording rather than silently correcting: **both error
sentences on that page spell the function `NORMS.INV`** — a third spelling that names neither
the modern surface (`NORM.S.INV`) nor the legacy one (`NORMSINV`). No such function exists. It
is a typographical error in the published page, and it is the kind of thing that sends a reader
searching for a function that was never shipped.

Error values in the argument propagate under the ordinary coercion rules.

## Relationships

- **`NORMSINV`** is the legacy spelling, with the same argument count in the reference engine's
  registry — one. Here the modernization **is** a pure rename at the signature level, in
  contrast to its CDF partner: `NORMSDIST` (arity 1) became `NORM.S.DIST` (arity 2) by
  *gaining* the `cumulative` switch, while `NORMSINV` became `NORM.S.INV` unchanged in shape.
  So within one family, one of the two renames added an argument and the other did not, and a
  reader who generalizes from either to the other will be wrong half the time. See
  [NORM.S.DIST](FUNC.NORM.S.DIST.md).

  Same signature is still not same computation. Excel dispatches compatibility functions as
  their own surfaces; `EV-DIST-0018` names both `NORM.S.INV` and `NORMSINV` among its subjects,
  but its figures are group totals carrying a reader warning against per-surface attribution,
  so nothing in the Handbook's record establishes that the two agree.
- **[NORM.S.DIST](FUNC.NORM.S.DIST.md)** is the forward function and, per Microsoft's text, the
  thing this function searches over. The round trip is the natural test pair.
- **[NORM.INV](FUNC.NORM.INV.md)** is the general case: `NORM.INV(p, μ, σ) = μ + σ·NORM.S.INV(p)`
  mathematically. Whether Excel's two surfaces agree at `μ = 0, σ = 1` is unmeasured.
- **`CONFIDENCE.NORM`** is this function's most common consumer: a normal confidence half-width
  is `NORM.S.INV(1 − α/2) · σ / √n`. `CONFIDENCE.NORM` shares this function's implementing
  module in the reference engine.
- **`LOGNORM.INV`**, **`STANDARDIZE`** and **`Z.TEST`** are the other members of the normal
  cluster.
- **Confused with**: `NORM.S.DIST`, by readers who want "the z-table" and pick the wrong
  direction — a z-table is read both ways and only one of these functions reads each way; and
  with `NORM.S.INV(95)` where `NORM.S.INV(0.95)` was meant, which is a `#NUM!` rather than a
  quiet wrong answer, mercifully.

## Numerical notes

**1. Conditioning first.** The amplification factor is `1/φ(z)`, which is `1` at the centre,
about `8` at `z = 2`, about `10⁴` at `z = 4`, and grows without bound. An error in the *input*
`p` — including the rounding that put it into a double — is multiplied by that. A correctly
rounded `NORM.S.INV` still returns an answer whose meaning is limited by the representation of
`p`. Accuracy for this function is best stated as a round-trip property.

**2. AS 241 is the reference algorithm.** Wichura's `PPND16` (*Applied Statistics* 37, 1988)
delivers about 16 significant digits across the whole representable domain using three rational
approximations chosen by region:

    |p − ½| ≤ 0.425                  central branch, in q = p − ½ and q²
    otherwise, r = √(−ln min(p,1−p)) tail branches, split again at r ≈ 5

That reparameterization in `r` is the essential trick: it converts the square-root-of-a-log
behaviour into something a rational function can approximate uniformly. R's `qnorm`, Boost, and
most modern libraries descend from it. Weaker predecessors, worth naming because they are still
in circulation: A&S 26.2.22 (about `3 × 10⁻³` absolute), A&S 26.2.23 (about `4.5 × 10⁻⁴`),
Beasley–Springer AS 111 with Moro's tail patch, and Acklam's approximation. None of those is
adequate as a reference; all of them are fine for a plot.

**3. Newton and Halley refinement close the gap, and reconcile the two designs.** With the
derivative available in closed form, a single Halley step from a good starting point reaches
full double precision:

    u = (Φ(z) − p) / φ(z)
    z ← z − u / (1 + z·u/2)

An implementation doing this *is* consuming the forward CDF, so its accuracy is derivative of
the CDF's — exactly what Microsoft's page says about `NORM.S.INV`. "Iterative search" and
"rational approximation plus refinement" are not as far apart as the words suggest; what
separates them observationally is monotonicity and the forward-versus-reverse round-trip
asymmetry described above.

**4. The tails must be computed in the tail parameter, never by symmetry from a central
formula.** Evaluating `Φ⁻¹(p)` for tiny `p` by reflecting a computation of `Φ⁻¹(1 − p)` throws
away everything: `1 − p` rounds to `1` and the reflection returns `+∞` where a finite answer
exists. This is the same complementary-probability trap that runs through the whole
distribution family, and it is why `p` and `1 − p` must be kept as separate inputs wherever a
surface can offer both.

**5. The published assessment record.** McCullough and Wilson's series in *Computational
Statistics & Data Analysis*, and Knüsel's assessments of Excel's distribution functions, report
on this lane across several Excel versions, with revisions around Excel 2003 and again when the
dotted names arrived. Morten Welinder's Gnumeric work is the standard practical account of
implementing the normal quantile in a spreadsheet. Named as literature; no figure restated.

## What has not been checked

`EV-DIST-0018` names `NORM.S.INV` among its subjects. It is a re-sweep of **pinned witnesses,
one row per surface** — not a corpus sweep — and its own reader warning states that the group
total is not any surface's pass rate. The normal group was measured at pinned points; **this
surface was not measured separately** by that figure.

No Handbook vector suite exists for `NORM.S.INV`, and no Handbook measurement of it exists;
`EV-DIST-0018` is upstream OxFunc work the Handbook has not re-verified. No evidence record in
the Handbook's possession identifies this surface's op-graph, so the documented iterative-search
description stands unconfirmed and unrefuted.

Microsoft's documented behaviour above, including the "iterative search technique" statement and
the `NORMS.INV` misspelling, was retrieved from the `NORM.S.INV` page.

Inputs worth probing first:

1. **`NORM.S.INV(0.5)`.** Must be exactly `0`. One cell, nameable answer, no oracle.
2. **The reflection identity** `NORM.S.INV(p) = −NORM.S.INV(1−p)` across a spread of `p`,
   including `p` near the region boundaries AS 241 would use (`|p − ½| ≈ 0.425`, and
   `r ≈ 5`, i.e. `p ≈ 1.4 × 10⁻¹¹`). Oracle-free, and a region seam shows here first.
3. **Both round trips**: `NORM.S.DIST(NORM.S.INV(p), TRUE)` against `p`, and
   `NORM.S.INV(NORM.S.DIST(z, TRUE))` against `z`. **This is the discriminating experiment**
   between a documented search and a direct approximation, as set out above.
4. **ULP-step monotonicity**: increment `p` by one ULP through the central and tail regions and
   confirm the result never decreases. A search with a loose tolerance is visible only here.
5. **The reachable extremes**: `p` at the smallest positive double, at `2⁻¹⁰⁰⁰`, at
   `1 − 2⁻⁵³`, and at the first value where the documented `#NUM!` fires. This maps the
   asymmetric range and pins the boundary exactly.
6. **`NORM.S.INV(0.975)`** against the published constant — the single most-consumed value of
   this function in practice, and one where a discrepancy would be noticed by every user of the
   spreadsheet rather than by a numerical analyst.
7. **`NORM.S.INV(p)` against `NORM.INV(p, 0, 1)` and against `NORMSINV(p)`** over the same
   spread — two pairings, neither established on the Handbook's record.
8. **`TRUE` and `FALSE` as the argument**, confirming that coercion succeeds and the domain
   rule then rejects both.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| probit | The standard normal quantile `Φ⁻¹`; this function |
| conditioning | The factor `1/φ(z)` amplifying an error in `p` into the answer |
| tail parameter | `r = √(−ln min(p, 1−p))`, the reparameterization AS 241 uses in the tails |
| reachable range | The results attainable given `p` must be a double strictly inside `(0, 1)` |
| documented search | Microsoft's stated description of this function as an iterative search |

## Sources

- Microsoft, "NORM.S.INV function" —
  <https://support.microsoft.com/en-us/office/norm-s-inv-function-d6d556b4-ab7f-49cd-b526-5a20918452b1>
  (syntax; the `probability` description; the `#VALUE!` and `#NUM!` conditions, both of which
  the page spells `NORMS.INV`; and the statement that the function seeks the `z` at which
  `NORM.S.DIST(z,TRUE)` equals the probability, that its precision depends on `NORM.S.DIST`'s,
  and that it "uses an iterative search technique"). Retrieved for this page.
- Handbook evidence record `EV-DIST-0018` — the ten-witness normal-group re-sweep; names
  `NORM.S.INV` and `NORMSINV` as subjects and carries a reader warning against per-surface
  attribution of its group figure.
- M. J. Wichura, "Algorithm AS 241: The percentage points of the normal distribution",
  *Applied Statistics* 37 (1988) 477–484 — `PPND16` and the `r = √(−ln min(p,1−p))`
  reparameterization.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 26 §26.2.22–26.2.23 (the
  classical rational approximations) and §26.2.12 (the tail asymptotics behind the
  `√(2 ln(1/p))` leading term).
- J. D. Beasley and S. G. Springer, "Algorithm AS 111", *Applied Statistics* 26 (1977), with
  Moro's tail modification — named for contrast.
- B. D. McCullough and B. Wilson, *Computational Statistics & Data Analysis* (1999, 2002,
  2005); L. Knüsel on Excel's distribution functions; M. Welinder's Gnumeric statistical work.
  Named as literature; no figure restated.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [About implementation options](../model/07-implementation-options.md).
- `data/functions/FUNC.NORM.S.INV.json` (arity 1, `Custom` profile,
  `signature_placeholder: true`, XLL symbol `xlfNorm_s_inv`) and
  `data/functions/FUNC.NORMSINV.json` (arity 1) — the source of the unchanged-signature
  observation, contrasted with the `NORMSDIST`/`NORM.S.DIST` pair.
- `data/presence/FUNC.NORM.S.INV.json` — implementing module
  `crates/oxfunc_core/src/functions/normal_log_family.rs`, shared with twelve other normal and
  lognormal surfaces.
