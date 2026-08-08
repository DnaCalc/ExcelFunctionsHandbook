---
schema: efh.function-page/v1
function_id: FUNC.NORM.DIST
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0018
open_problems: []
references:
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 26, 26.2 (normal probability function and related functions)"
    role: "The defining density and probability integral, the tail asymptotics, and the rational approximations"
  - work: "W. J. Cody, Rational Chebyshev approximation for the error function"
    locator: "Mathematics of Computation 23 (1969) 631-637; SPECFUN ANORM/CALERF"
    role: "The standard accurate route to the normal integral through erf and erfc"
  - work: "B. D. McCullough and B. Wilson, On the accuracy of statistical procedures in Microsoft Excel"
    locator: "Computational Statistics & Data Analysis, 1999 / 2002 / 2005 series"
    role: "The published assessment record of Excel's statistical lane"
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
family: normal_log_family
role_in_family: >-
  The general two-parameter normal surface, cumulative or density by switch; the member the whole
  normal-and-lognormal family reduces to, and the one Microsoft's own page identifies with
  NORM.S.DIST at the standard parameters.
---

# NORM.DIST

## What it computes

`NORM.DIST(x, mean, standard_dev, cumulative)` evaluates the normal distribution with mean `μ`
and standard deviation `σ` at `x`, returning either its density or its cumulative distribution
function according to the fourth argument.

**Density** (`cumulative = FALSE`) — Microsoft's page carries this equation as an image; it is
the standard one:

    f(x; μ, σ) = 1 / (σ √(2π)) · exp( −(x − μ)² / (2σ²) )

**Cumulative** (`cumulative = TRUE`) — Microsoft's page states that this is the integral from
negative infinity to `x` of the density:

    F(x; μ, σ) = ∫₋∞ˣ f(t; μ, σ) dt
               = Φ( (x − μ) / σ )
               = ½ · [ 1 + erf( (x − μ) / (σ √2) ) ]
               = ½ · erfc( −(x − μ) / (σ √2) )

The three closed forms for `F` are algebraically identical and numerically are not: the
`erfc` form is the one that survives in the left tail, and the choice among them is the whole
subject of the Numerical notes below.

**Domain and range.** `x` and `μ` range over the reals; `σ > 0` is required — `σ ≤ 0` is a
documented `#NUM!`. The density is strictly positive with range `(0, 1/(σ√(2π))]`, attaining
its maximum at `x = μ`. The cumulative is strictly increasing with range `(0, 1)`, never
reaching either endpoint mathematically, though in binary64 it rounds to `0` for
`(x−μ)/σ ≲ −37.5` and to `1` for `(x−μ)/σ ≳ 8.3`. There are no poles and no branch cuts; both
forms are entire in `x`.

**Standardization is the whole structure of this family.** Writing `z = (x − μ)/σ`:

    F(x; μ, σ) = Φ(z)                and    f(x; μ, σ) = φ(z) / σ

so the two-parameter surface is a one-parameter surface composed with an affine map. Every
member of the normal group is `Φ` or `φ` wearing a change of variable — which is why
Microsoft's page states the identity directly: with `mean = 0`, `standard_dev = 1` and
`cumulative = TRUE`, `NORM.DIST` "returns the standard normal distribution, `NORM.S.DIST`".

That sentence is a documented *mathematical* identity. It is not, and must not be read as, a
statement that the two Excel surfaces return the same bits. See Relationships.

**Limiting cases and special values.**

    F(μ; μ, σ) = 1/2                        exactly, for every σ > 0
    F(x; μ, σ) + F(2μ − x; μ, σ) = 1        the reflection identity
    f(μ; μ, σ) = 1 / (σ √(2π))
    1 − F(z) ~ φ(z)/z · (1 − 1/z² + 3/z⁴ − …)   as z → ∞   (A&S 26.2.12)

The last of these is the asymptotic expansion that governs the right tail and explains why the
tail is where implementations diverge. The value at the mean is the cheapest exactness probe
this function has: `F(μ; μ, σ)` must be `0.5` with no rounding at all.

## Arguments

Microsoft's page gives four required arguments:

| Argument | Meaning | Required |
|---|---|---|
| `x` | The value for which you want the distribution | yes |
| `mean` | The arithmetic mean of the distribution | yes |
| `standard_dev` | The standard deviation of the distribution | yes |
| `cumulative` | A logical: `TRUE` gives the cumulative distribution function, `FALSE` the probability density function | yes |

`cumulative` has **no default**. This is a real difference from the shape readers expect: many
distribution functions in other systems default to the CDF, and here the switch must be
written. Its argument descriptions above are Microsoft's own wording.

The reference engine records an arity of exactly 4 and a `Custom` coercion/lift profile — the
admission of logicals, text and empty cells in each slot is decided by this function, not
inherited from a family default. The signature in the projection is a **placeholder**: the
projection's `signature_placeholder` flag is set, so no parameter list is published from that
source, and the table above comes from Microsoft's page rather than from the projection.

## Result and edge cases

Returns `Number`.

- **`σ ≤ 0`** is a documented `#NUM!`, including `σ = 0`. The degenerate "distribution" that
  puts all its mass at `μ` is therefore not reachable, and neither the step-function CDF nor
  the unbounded density has to be represented.
- **`cumulative` coerced from a number.** Excel's logical slots accept numbers, with zero
  reading as `FALSE`. That means `NORM.DIST(x, μ, σ, 0)` is the density and
  `NORM.DIST(x, μ, σ, 1)` is the CDF, and any nonzero value selects the CDF. The reference
  engine's battery renders the all-`TRUE` and all-zero rows, and their outcomes show beside
  this page.
- **Far left tail.** `F` underflows to `0` for `z` below roughly `−37.5`. The last few orders
  of magnitude before that are subnormal, and the relative accuracy there is a genuine
  implementation difference rather than a formality.
- **Far right tail.** `F` rounds to exactly `1` for `z` above roughly `8.3`, because
  `1 − F(z)` falls below half an ULP of `1`. **All information about the upper tail is lost at
  that point**, and no rearrangement of `NORM.DIST` recovers it — the loss is in the
  representation of the answer, not in the algorithm. A user who needs upper-tail probabilities
  must compute `NORM.DIST(2μ − x, μ, σ, TRUE)` by reflection, or use `1 − Φ` written as `Φ(−z)`.
  This is the single most consequential practical fact on this page.
- **Density at large `|z|`** underflows around `|z| ≈ 38.6`, with a subnormal band before it —
  the same band discussed on [PHI](FUNC.PHI.md).
- **Arrays.** The recorded profile is `Custom`; whether `NORM.DIST` lifts elementwise over
  array arguments is not settled here.

Empty, missing and error arguments follow the shared call model; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

As documented by Microsoft on the `NORM.DIST` page (retrieved for this page):

| Error | Documented condition |
|---|---|
| `#VALUE!` | `mean` or `standard_dev` is nonnumeric |
| `#NUM!` | `standard_dev ≤ 0` |

Two observations about what that table does *not* say, both worth recording rather than
papering over:

1. **The `#VALUE!` row names only `mean` and `standard_dev`.** It does not name `x`. Read
   literally, the documentation says nothing about a nonnumeric `x` — which surely also
   produces `#VALUE!` under the ordinary coercion rules, but is not documented as such on this
   page. This is a documentation gap, not a behavioural claim.
2. **No `#NUM!` for extreme `x`.** The documentation states no domain restriction on `x` at
   all, consistent with the mathematics.

Error values in any argument propagate under the ordinary coercion rules.

## Relationships

- **`NORMDIST`** is the legacy spelling, carrying the same argument count in the reference
  engine's registry — four, including `cumulative`. So this modernization is a **pure rename
  at the signature level**, unlike `NORMSDIST → NORM.S.DIST`, which gained an argument.
  **A shared signature is not a shared computation.** Excel dispatches the compatibility
  functions as their own surfaces, and whether `NORMDIST` and `NORM.DIST` produce identical
  bits is an empirical question that requires evidence to answer. `EV-DIST-0018` names both
  `NORM.DIST` and the two `NORMS*` legacy spellings among its subjects, and its own status note
  records that `NORMDIST` and `NORMINV` are **not** among the surfaces it measured. So the
  legacy-modern pairing for this specific function is, on the Handbook's record, unmeasured.
- **[NORM.S.DIST](FUNC.NORM.S.DIST.md)** is the standardized case. Microsoft's page states the
  identity `NORM.DIST(x, 0, 1, TRUE) = NORM.S.DIST(x, TRUE)` as documented behaviour. The
  Handbook publishes that as a documented identity, and separately notes that no evidence in
  its record demonstrates the two surfaces agree in their last bits.
- **[NORM.INV](FUNC.NORM.INV.md)** is the functional inverse in the first argument:
  `NORM.INV(NORM.DIST(x, μ, σ, TRUE), μ, σ) = x` mathematically. Microsoft's `NORM.INV` page
  states that `NORM.INV` *searches* for the `x` at which `NORM.DIST` equals the given
  probability, and that "precision of `NORM.INV` depends on precision of `NORM.DIST`" — a
  documented dependency between the two surfaces, and an unusually explicit one.
- **[PHI](FUNC.PHI.md)** is `NORM.DIST(x, 0, 1, FALSE)` under a shorter name.
- **[GAUSS](FUNC.GAUSS.md)** is `NORM.DIST(z, 0, 1, TRUE) − 0.5`.
- **`ERF` / `ERFC`** are the underlying special functions: `Φ(z) = ½ erfc(−z/√2)`.
- **`LOGNORM.DIST`** is the same integral after a logarithmic change of variable, and shares
  this function's implementing module in the reference engine — the projection puts
  `NORM.DIST`, `NORM.INV`, `NORM.S.DIST`, `NORM.S.INV`, `LOGNORM.DIST`, `LOGNORM.INV`,
  `CONFIDENCE`, `CONFIDENCE.NORM` and their legacy spellings in one module. That is a
  reasonable signal about the reference engine's structure and none at all about Excel's.
- **`CONFIDENCE.NORM`** and **`Z.TEST`** are consumers: both are `NORM.S.INV` or `NORM.S.DIST`
  applied to a standardized statistic.
- **Confused with**: `NORM.DIST(x, μ, σ)` written without the fourth argument, which is not a
  legal call — the argument is required, and omitting it is refused at entry rather than
  defaulting.

## Numerical notes

The normal CDF is the most-studied function in applied statistics and the place where Excel's
statistical lane has historically been assessed hardest. Four things are worth stating.

**1. The tail is the function.** The naive route — compute the density, integrate a series —
is fine near the centre and useless in the tails. Every serious implementation reduces to
`erf`/`erfc` and switches representation by region:

    |z| small          series or rational approximation for erf(z/√2)
    |z| moderate       rational approximation for erfc
    |z| large          erfc via the asymptotic expansion or a continued fraction,
                       scaled by e^(−z²/2) to avoid underflow

Cody's rational Chebyshev approximations (1969, and the `CALERF`/`ANORM` routines in SPECFUN)
are the canonical accurate double-precision route and are what most modern libraries descend
from. Cephes's `ndtr` is a widely used independent implementation. A&S 26.2.17 gives the
Hastings-style rational approximation that older code used, and its accuracy — around seven
decimal digits — is exactly the kind of legacy standard that shows up as a visible defect when
double precision is expected.

**2. Cancellation is where the naive form dies.** Two distinct failures:

- `1 − Φ(z)` for large positive `z` is catastrophic subtraction: the answer is tiny, the
  operands are both near `1`, and the result has no significant digits left once
  `1 − Φ(z) < 2^−53`. The remedy is not a better `Φ` — it is to compute `Φ(−z)` instead, using
  the exact symmetry, or to expose a complementary surface. Excel's normal family has no
  complementary surface (there is no `NORM.DIST.RT` beside `CHISQ.DIST.RT` and `F.DIST.RT`),
  so **the reflection is the user's responsibility**.
- `½[1 + erf(u)]` for `u` very negative is the same failure in the other direction: `erf(u)`
  approaches `−1` and the sum loses digits. Writing it as `½ erfc(−u)` avoids the subtraction
  entirely. This is why the `erfc` form is the one to implement.

**3. Argument reduction has its own error.** `z = (x − μ)/σ` is computed in floating point
before the special function ever runs. When `x` and `μ` are close and large, `x − μ` loses
digits; the resulting relative error in `z` is amplified into the result by a factor of about
`z` for the CDF (since `Φ'(z)/Φ(z) → z` in the left tail) and by about `z²` for the density
(the squaring bottleneck described on [PHI](FUNC.PHI.md)). A two-parameter normal function
therefore has an accuracy ceiling set by the subtraction, before any special-function code
runs. This is invisible in any test that uses `μ = 0`.

**4. The published assessment record.** Excel's statistical functions have been examined in
print for two decades — McCullough and Wilson's series in *Computational Statistics & Data
Analysis*, and Knüsel's assessments of the distribution functions specifically, both report
accuracy shortfalls in Excel's statistical lane across several versions, with revisions in
Excel 2003 and again around Excel 2010 when the dotted names were introduced. Morten Welinder's
work on Gnumeric's statistical functions is the standard practical account of what it takes to
get these functions right in a spreadsheet and of where Excel's answers came from. The Handbook
names these as literature; it has not re-run their tests and does not restate their figures.

**What a careful independent implementation does**: standardize with care (compensated
subtraction if the arguments warrant it), route the CDF through `erfc` with a
region-switched rational approximation, evaluate the density through a double-double `z²/2` as
described on [PHI](FUNC.PHI.md), and state a bound. See
[About implementation options](../model/07-implementation-options.md).

## What has not been checked

`EV-DIST-0018` names `NORM.DIST` among its subjects. It is a **re-sweep of pinned witnesses**,
one row per surface, not a corpus sweep: its figures are group totals across ten surfaces
measured at one point each, and the record's own reader warning states that the total is not
any surface's pass rate. So the correct reading for this page is that the normal group was
measured at pinned points and **this surface was not measured separately by that figure**. The
record additionally states that `NORMDIST` and `NORMINV` are not in that set at all.

No Handbook vector suite exists for `NORM.DIST`. No Handbook measurement of this surface exists
— `EV-DIST-0018` is upstream OxFunc work that the Handbook has not re-verified. Nobody has
checked, within the Handbook's record, whether `NORM.DIST` agrees with Excel anywhere other
than at the pinned witness that record describes.

Microsoft's documented behaviour above was retrieved from the `NORM.DIST` page.

Inputs worth probing first:

1. **`NORM.DIST(μ, μ, σ, TRUE)` for several `σ`.** The value must be exactly `0.5`. It is the
   one input where the correct answer is nameable with no rounding, and any deviation is
   immediately a finding.
2. **The reflection identity** `NORM.DIST(μ+d, μ, σ, TRUE) + NORM.DIST(μ−d, μ, σ, TRUE) = 1`
   across a spread of `d`. A metamorphic test that needs no oracle at all, and one that a
   region-switched implementation can fail at a switch boundary.
3. **The right-tail saturation point**: walk `z` upward through `8` to `9` and find the exact
   `z` at which the CDF first returns `1`. That single number characterizes the implementation's
   effective tail resolution, and it is comparable across systems.
4. **The left tail into the subnormal range**, `z` from `−36` to `−39`, which is where the
   `erfc` route and any `1 − erf` route separate visibly.
5. **`NORM.DIST(x, 0, 1, TRUE)` against `NORM.S.DIST(x, TRUE)`**, and `NORM.DIST` against
   `NORMDIST` at the same arguments, over a spread. Microsoft documents the first pair as a
   mathematical identity; whether either pair agrees bit for bit is unmeasured, and the second
   pair is explicitly outside `EV-DIST-0018`'s set.
6. **Large, close `x` and `μ`** — say `x = 1e15 + 1`, `μ = 1e15`, `σ = 1` — where the
   standardizing subtraction loses digits. This isolates the argument-reduction error that
   `μ = 0` testing can never see.
7. **`cumulative` given as a number, as text `"TRUE"`, and as an empty cell**, given the
   `Custom` coercion profile.
8. **`standard_dev` exactly `0`**, to confirm the documented `#NUM!` boundary is inclusive.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| standardization | The change of variable `z = (x − μ)/σ` reducing the surface to `Φ` or `φ` |
| complementary tail | `1 − Φ(z)`; not available as its own Excel surface in the normal family |
| saturation point | The `z` beyond which the CDF rounds to exactly `1` and carries no information |
| region switch | The argument-dependent choice among series, rational and asymptotic forms |
| `signature_placeholder` | The projection flag marking that no real parameter list is published |

## Sources

- Microsoft, "NORM.DIST function" —
  <https://support.microsoft.com/en-us/office/norm-dist-function-edb1cc14-a21c-4e53-839d-8082074c9f8d>
  (syntax; the four required arguments and their descriptions; the `#VALUE!` condition on
  `mean`/`standard_dev`; the `#NUM!` condition on `standard_dev ≤ 0`; the density equation; the
  statement that the cumulative form is the integral from negative infinity to `x`; and the
  identity with `NORM.S.DIST` at `mean = 0`, `standard_dev = 1`, `cumulative = TRUE`).
  Retrieved for this page.
- Handbook evidence record `EV-DIST-0018` — the ten-witness normal-group re-sweep, whose group
  figure carries a reader warning against per-surface attribution and which excludes `NORMDIST`
  and `NORMINV`.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 26 §26.2 — the normal
  probability function `P(x)`, the density `Z(x)`, the tail asymptotic expansion 26.2.12, and
  the rational approximations 26.2.16–26.2.19.
- W. J. Cody, "Rational Chebyshev approximation for the error function", *Mathematics of
  Computation* 23 (1969); and the `CALERF`/`ANORM` routines of SPECFUN.
- S. L. Moshier, Cephes `ndtr.c` — an independent double-precision normal integral.
- B. D. McCullough and B. Wilson, "On the accuracy of statistical procedures in Microsoft
  Excel", *Computational Statistics & Data Analysis* (1999, 2002, 2005); and L. Knüsel's
  assessments of Excel's distribution functions. Named as the published assessment record; no
  figure from them is restated here.
- M. Welinder's work on Gnumeric's statistical functions — the standard practical account of
  implementing this family in a spreadsheet.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [About implementation options](../model/07-implementation-options.md).
- `data/functions/FUNC.NORM.DIST.json` — arity 4, `Custom` coercion/lift profile,
  `signature_placeholder: true`, XLL symbol `xlfNorm_dist`.
- `data/presence/FUNC.NORM.DIST.json` — implementing module
  `crates/oxfunc_core/src/functions/normal_log_family.rs`, shared with twelve other normal and
  lognormal surfaces.
