---
schema: efh.function-page/v1
function_id: FUNC.NORM.S.DIST
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0018
open_problems: []
references:
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 26, 26.2 (normal probability function P(x), density Z(x))"
    role: "The defining integral, its symmetry, the tail asymptotics and the rational approximations"
  - work: "W. J. Cody, Rational Chebyshev approximation for the error function"
    locator: "Mathematics of Computation 23 (1969); SPECFUN ANORM"
    role: "The canonical accurate double-precision route to the normal integral"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - A documentation defect worth naming
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: normal_log_family
role_in_family: >-
  The standardized normal surface with a cumulative switch; the modern spelling that added an
  argument its legacy predecessor NORMSDIST does not have, and therefore the family's clearest
  case that a dotted rename is not always a rename.
---

# NORM.S.DIST

## What it computes

`NORM.S.DIST(z, cumulative)` evaluates the **standard** normal distribution — mean `0`,
standard deviation `1` — at `z`, returning either its density or its cumulative distribution
function according to the second argument.

    cumulative = FALSE :   φ(z) = (1 / √(2π)) · e^(−z²/2)
    cumulative = TRUE  :   Φ(z) = ∫₋∞ᶻ φ(t) dt
                                = ½ · [ 1 + erf(z/√2) ]
                                = ½ · erfc(−z/√2)

In Abramowitz & Stegun's notation these are `Z(x)` and `P(x)` respectively (chapter 26 §26.2).

**Domain and range.** Defined for every real `z`; no poles, no branch cuts, both forms entire.

    φ:  strictly positive, even, range (0, 1/√(2π)],  maximum φ(0) = 1/√(2π)
    Φ:  strictly increasing, range (0, 1),  Φ(−∞) = 0,  Φ(+∞) = 1

**The identities that carry the function.**

    Φ(0) = 1/2                          exactly
    Φ(z) + Φ(−z) = 1                    the reflection identity
    Φ'(z) = φ(z)
    φ'(z) = −z φ(z)
    1 − Φ(z) ~ φ(z)/z · (1 − 1/z² + 3/z⁴ − …)     as z → +∞    (A&S 26.2.12)

`Φ(0) = 1/2` is exactly representable and is the single most useful test input this function
has: it removes the special function from the picture entirely and tests only whether the
implementation knows where the centre is. `φ(0) = 1/√(2π)` plays the same role for the density,
testing only the stored constant.

**Where the values stop being informative.** In binary64, `Φ(z)` rounds to exactly `1` once
`1 − Φ(z)` falls below half an ULP of `1`, which happens around `z ≈ 8.3`. Beyond that point
the upper tail is unrecoverable from this surface, no matter how the implementation is written —
the loss is in the representation of the answer. The reflection `Φ(−z)` is the route to the
upper tail, and Excel's normal family provides no right-tailed surface of its own (contrast
`CHISQ.DIST.RT`, `F.DIST.RT`, `T.DIST.RT`). At the other end, `Φ(z)` underflows to `0` around
`z ≈ −37.5`, with a subnormal band before it.

## Arguments

Microsoft's page gives two required arguments:

| Argument | Meaning | Required |
|---|---|---|
| `z` | "the value for which you want the distribution" | yes |
| `cumulative` | "can be either TRUE or FALSE… If cumulative is TRUE then NORM.S.DIST returns the cumulative distribution function. If it is FALSE, it returns the probability mass function." | yes |

`cumulative` has no default: the switch must be written. The reference engine records an arity
of exactly 2, matching.

The reference engine's projection marks this surface's signature as a **placeholder**
(`signature_placeholder: true`), so no parameter list is published from that source; the table
above is Microsoft's.

The recorded coercion/lift profile is `Custom`, meaning the admission of logicals, text and
empty cells in each slot is this function's own decision rather than a family default. See
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## A documentation defect worth naming

Microsoft's `NORM.S.DIST` page describes the `cumulative = FALSE` branch as returning "the
**probability mass function**".

That is wrong, and not merely as pedantry. A probability mass function assigns probability to
individual outcomes and is defined only for discrete distributions; the normal distribution is
continuous, every individual outcome has probability zero, and what `cumulative = FALSE`
returns is a probability **density** — a quantity with units of probability per unit of `z`,
which can and does exceed... well, which is not a probability at all, and whose value at the
origin is about `0.3989`.

Microsoft's own `NORM.DIST` page, describing the identical branch of the identical function,
says "probability density function". The two pages disagree with each other, and the
`NORM.S.DIST` page is the one that is wrong.

The Handbook records this because the confusion it invites is a real one: a reader who believes
`NORM.S.DIST(z, FALSE)` is a probability will interpret its value as one, and densities are not
probabilities. The behaviour is not in dispute — both pages describe the same computation — but
the published description of it is defective on one page.

## Result and edge cases

Returns `Number`.

- **`cumulative` coerced from a number.** Excel's logical slots accept numbers with zero
  reading as `FALSE`, so `NORM.S.DIST(z, 0)` selects the density and any nonzero value selects
  the CDF. The reference engine's battery renders the zero-and-zero row and the all-`TRUE` row;
  their outcomes show beside this page.
- **`z = 0`.** The cumulative branch is exactly `0.5`; the density branch is the stored
  constant `1/√(2π)`. Both are diagnostic single cells, for different reasons.
- **Right-tail saturation** around `z ≈ 8.3` and **left-tail underflow** around `z ≈ −37.5`,
  as described above. The density branch underflows around `|z| ≈ 38.6`, with a subnormal band
  before it — see [PHI](FUNC.PHI.md).
- **Symmetry.** `Φ(z) + Φ(−z) = 1` and `φ(z) = φ(−z)` are exact mathematically. Whether the
  implementation reproduces them in the last bit is a metamorphic property that can be tested
  without any external oracle, and is on the probe list.
- **Arrays.** The recorded profile is `Custom`; elementwise lifting is not settled here.

## Errors

As documented by Microsoft on the `NORM.S.DIST` page (retrieved for this page):

| Error | Documented condition |
|---|---|
| `#VALUE!` | `z` is nonnumeric |

That is the complete documented error surface — one row. Note what it omits: **nothing is
documented about a nonnumeric `cumulative`**, and no `#NUM!` condition is stated at all, which
is consistent with the mathematics since `Φ` and `φ` are defined on the whole real line.

Error values in either argument propagate under the ordinary coercion rules.

## Relationships

- **`NORMSDIST`** is the legacy spelling, and this is the pair that most rewards precision.
  The reference engine's registry records `NORMSDIST` with an arity of exactly **1** and
  `NORM.S.DIST` with an arity of exactly **2**. The modernization **added the `cumulative`
  argument**; `NORMSDIST` computes only the cumulative form. So:

      NORMSDIST(z)  corresponds to  NORM.S.DIST(z, TRUE)

  and there is no legacy spelling of the density branch at all. This is not a rename. Anyone
  migrating formulas mechanically by inserting dots will produce calls that do not compile, and
  anyone reasoning about the pair as "the same function under two names" is reasoning about
  only half of the modern one.

  And even for the branch that does correspond: **the correspondence is a signature and
  mathematical correspondence, not a demonstrated identity of bits.** Excel dispatches
  compatibility functions as their own surfaces. `EV-DIST-0018` names both `NORM.S.DIST` and
  `NORMSDIST` among its subjects, but its figures are group totals with a reader warning
  against per-surface attribution, so nothing in the Handbook's record establishes that the two
  agree.
- **[NORM.DIST](FUNC.NORM.DIST.md)** is the two-parameter surface. Microsoft's `NORM.DIST` page
  states the documented identity `NORM.DIST(x, 0, 1, TRUE) = NORM.S.DIST(x, TRUE)`. Documented
  identity, unmeasured bit agreement.
- **[NORM.S.INV](FUNC.NORM.S.INV.md)** is the functional inverse of the cumulative branch.
- **[PHI](FUNC.PHI.md)** computes the same value as `NORM.S.DIST(z, FALSE)`. The pointed fact
  here: `EV-DIST-0022` identifies Excel's `PHI` as a squaring, an exponential and a multiply,
  with **no erf and no CDF in the chain** — a route that a cumulative-capable surface cannot
  be taking for its cumulative branch. Whether Excel's `NORM.S.DIST(z, FALSE)` shares `PHI`'s
  chain or has its own is precisely the kind of question two surfaces for one function raise,
  and it is unanswered.
- **[GAUSS](FUNC.GAUSS.md)** is `NORM.S.DIST(z, TRUE) − 0.5`, i.e. `Φ(z) − ½`. `GAUSS` exists
  because that subtraction loses precision near the origin when done by the user, and it is
  one of the two surfaces `EV-DIST-0018` reports as having drifted at its sweep.
- **`ERF` / `ERFC`** are the underlying special functions; `Φ(z) = ½ erfc(−z/√2)`.
- **`Z.TEST`**, **`CONFIDENCE.NORM`** and **`STANDARDIZE`** are the usual consumers.

## Numerical notes

**The two branches are different numerical problems sharing a name.** The density branch is a
squaring and an exponential — see [PHI](FUNC.PHI.md) for why the squaring, not the exponential,
sets the accuracy ceiling, and why the error grows quadratically in `z`. The cumulative branch
is a special function requiring region-switched approximation. An implementation that treats
them as two branches of one routine is doing the right thing structurally, but it should not
be expected that accuracy claims transfer between them.

**The cumulative branch, properly.** The route that survives is `erfc`:

    Φ(z) = ½ · erfc(−z/√2)

written that way and never as `½[1 + erf(z/√2)]`, because the latter subtracts nearly equal
quantities in the left tail. Then `erfc` itself is region-switched: a rational approximation
near the origin, a different one for moderate argument, and for large argument the scaled form
`erfc(u) = e^(−u²) · R(u) / u` with `R` from a continued fraction or the asymptotic series
(A&S 7.1.13/7.1.14, 26.2.12), which keeps the exponential factored out so nothing underflows
prematurely. Cody's rational Chebyshev approximations are the standard source; Cephes's `ndtr`
is a widely used independent implementation; Boost's `erfc` carries stated bounds.

**The `√2` in the argument is not free.** `−z/√2` is computed in floating point, and the
rounding of that division is amplified into the result by the local logarithmic derivative,
which grows like `|z|` in the tail. Carrying `z/√2` as a double-double — or equivalently
multiplying by a two-part representation of `1/√2` — is the standard fix and matters only
where it matters, which is exactly the tail where users are least likely to check.

**A note on what the identities buy you.** `Φ(0) = ½` and `Φ(z) + Φ(−z) = 1` are exact and
oracle-free. An implementation that is region-switched will generally *not* satisfy the
reflection identity in the last bit unless it is written to compute one side and reflect,
because the two sides go through different approximations. Whether that is a defect depends on
what was promised. Recording it is not optional.

**The published assessment record.** Excel's statistical lane, and the normal distribution
functions in particular, have been examined in print for two decades — McCullough and Wilson in
*Computational Statistics & Data Analysis*, Knüsel on the distribution functions specifically —
with revisions reported around Excel 2003 and again when the dotted names arrived. Morten
Welinder's Gnumeric work is the standard practical account of getting this family right in a
spreadsheet. The Handbook names these as literature and restates no figure from them.

## What has not been checked

`EV-DIST-0018` names `NORM.S.DIST` among its subjects. It is a re-sweep of **pinned witnesses,
one row per surface**, not a corpus sweep, and its own reader warning states that its group
total is not any surface's pass rate. The correct reading for this page: the normal group was
measured at pinned points, and **this surface was not measured separately** by that figure. The
record's own note observes that one of its pinned rows is a special value — the kind of input
that any implementation gets right — which is a limit on what the sweep can tell anyone.

No Handbook vector suite exists for `NORM.S.DIST`, and no Handbook measurement of it exists;
`EV-DIST-0018` is upstream OxFunc work the Handbook has not re-verified. The density branch of
this surface is not, on the Handbook's record, measured at all.

Microsoft's documented behaviour above was retrieved from the `NORM.S.DIST` page, including the
"probability mass function" defect recorded above.

Inputs worth probing first:

1. **`NORM.S.DIST(0, TRUE)`** — must be exactly `0.5`, with no rounding anywhere. And
   **`NORM.S.DIST(0, FALSE)`** against **`PHI(0)`**, which tests only whether the two surfaces
   hold the same constant. Two cells, two independent facts.
2. **The reflection identity** `NORM.S.DIST(z, TRUE) + NORM.S.DIST(−z, TRUE) = 1` over a
   spread of `z` including region-switch candidates near `|z| ≈ 0.5`, `1`, `2`, `6`. Oracle-free,
   and the cheapest available detector of a region-switch seam.
3. **The right-tail saturation point**: the exact `z` at which the cumulative branch first
   returns `1`. A single number that characterizes the implementation's tail resolution.
4. **The left tail** from `z = −36` to `z = −39`, into the subnormal range, where an `erfc`
   route and a `1 − erf` route separate unmistakably.
5. **`NORM.S.DIST(z, FALSE)` against `PHI(z)`** over the whole range, and both against
   `NORM.DIST(z, 0, 1, FALSE)`. Three surfaces for one density; agreement everywhere would be
   evidence of a shared path and any disagreement proves distinct ones.
6. **`NORM.S.DIST(z, TRUE)` against `NORMSDIST(z)`** over the same spread — the legacy-modern
   pairing, which nothing in the Handbook's record establishes.
7. **`cumulative` given as `0`, as `1`, as `2`, as the text `"TRUE"`, and as an empty cell** —
   the `Custom` coercion profile means none of this is predictable from another function.
8. **A nonnumeric `cumulative`**, which the documentation does not cover at all.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| standard normal | Mean `0`, standard deviation `1`; A&S's `P(x)` and `Z(x)` |
| cumulative switch | The second argument selecting `Φ` or `φ`; required, no default |
| reflection identity | `Φ(z) + Φ(−z) = 1`; exact, oracle-free, and a region-switch detector |
| saturation point | The `z` beyond which `Φ` rounds to exactly `1` and carries no information |
| region switch | The argument-dependent choice among series, rational and asymptotic forms |

## Sources

- Microsoft, "NORM.S.DIST function" —
  <https://support.microsoft.com/en-us/office/norm-s-dist-function-1e787282-3832-4520-a9ae-bd2a8d99ba88>
  (syntax; both arguments required; the `cumulative` description, which calls the non-cumulative
  branch a "probability mass function"; the single `#VALUE!` condition on a nonnumeric `z`).
  Retrieved for this page.
- Microsoft, "NORM.DIST function" —
  <https://support.microsoft.com/en-us/office/norm-dist-function-edb1cc14-a21c-4e53-839d-8082074c9f8d>
  (the documented identity at `mean = 0`, `standard_dev = 1`, `cumulative = TRUE`; and the
  "probability density function" wording that the `NORM.S.DIST` page contradicts). Retrieved.
- Handbook evidence record `EV-DIST-0018` — the ten-witness normal-group re-sweep, which names
  `NORM.S.DIST` and `NORMSDIST` as subjects and whose group figure carries a reader warning
  against per-surface attribution.
- Handbook evidence record `EV-DIST-0022` — cited here only for the fact that Excel's `PHI` was
  identified on a chain containing no erf and no CDF; `NORM.S.DIST` is not among its subjects
  and no claim about this surface is drawn from it.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 26 §26.2 and chapter 7
  §7.1 — the normal probability function, the density, the tail asymptotics and the `erfc`
  continued fraction.
- W. J. Cody, "Rational Chebyshev approximation for the error function", *Mathematics of
  Computation* 23 (1969); SPECFUN `ANORM`/`CALERF`. S. L. Moshier, Cephes `ndtr.c`.
- B. D. McCullough and B. Wilson, *Computational Statistics & Data Analysis* (1999, 2002,
  2005); L. Knüsel on Excel's distribution functions; M. Welinder's Gnumeric statistical work.
  Named as literature; no figure restated.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [About implementation options](../model/07-implementation-options.md).
- `data/functions/FUNC.NORM.S.DIST.json` (arity 2, `Custom` profile,
  `signature_placeholder: true`, XLL symbol `xlfNorm_s_dist`) and
  `data/functions/FUNC.NORMSDIST.json` (arity 1) — the source of the added-argument finding.
- `data/presence/FUNC.NORM.S.DIST.json` — implementing module
  `crates/oxfunc_core/src/functions/normal_log_family.rs`, shared with twelve other normal and
  lognormal surfaces.
