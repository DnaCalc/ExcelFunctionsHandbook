---
schema: efh.function-page/v1
function_id: FUNC.T.DIST
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
  - Relationships
  - Numerical notes
  - What has not been checked
  - Notes
  - Page vocabulary
  - Sources
family: chi_f_t_family
role_in_family: >-
  The left-tail member of the Student-t quartet, and the only one that can also return the
  density: it is the surface that gives the t-distribution its ordinary CDF, a view the legacy
  TDIST spelling never offered.
---

# T.DIST

## What it computes

`T.DIST(x, deg_freedom, cumulative)` evaluates Student's *t*-distribution with ν degrees of
freedom — the density when `cumulative` is FALSE, the **left-tail** cumulative distribution
function when it is TRUE.

The density is

    f(x; ν) = Γ((ν+1)/2) / ( sqrt(νπ) · Γ(ν/2) ) · (1 + x²/ν)^(−(ν+1)/2)

on the whole real line, for ν > 0. It is symmetric about zero, unimodal, and has tails that
decay like `|x|^(−(ν+1))` — polynomially, not exponentially, which is the entire reason the
distribution exists as a separate object from the normal.

The cumulative form has no elementary closed form for general ν, but it has an exact expression
in the regularized incomplete beta function `I_z(a, b)`:

    F(x; ν) = 1 − ½ · I_{ν/(ν+x²)}(ν/2, ½)     for x ≥ 0
    F(x; ν) =     ½ · I_{ν/(ν+x²)}(ν/2, ½)     for x ≤ 0

Equivalently, if `T ~ t_ν` then `T²/(T² + ν) ~ Beta(½, ν/2)`; the beta identity above is that
statement rearranged. Abramowitz & Stegun give the distribution and its integrals in section
26.7.

Domain and range: `x` ranges over all reals and `F` over `(0, 1)`, strictly — the CDF attains
neither endpoint at any finite `x`. There are no poles and no branch cuts; `f` is real-analytic
in `x` for every ν > 0.

The limiting and special cases are worth keeping in view, because they are the cheapest
correctness checks anyone has:

| ν | Distribution | Closed form for `F` |
|---|---|---|
| 1 | Standard Cauchy | `½ + arctan(x)/π` |
| 2 | — | `½ · (1 + x / sqrt(x² + 2))` |
| ν → ∞ | Standard normal | `Φ(x)` |

Moments exist only up to order ν: the mean is 0 for ν > 1 and undefined for ν = 1, and the
variance is `ν/(ν−2)` for ν > 2 and infinite for 1 < ν ≤ 2. A t-distribution with one degree of
freedom has no mean at all, which is why the ν = 1 corner is a genuinely different object rather
than an extreme setting of the same one.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `x` | The value at which the distribution is evaluated. | Yes |
| `deg_freedom` | ν, "an integer indicating the number of degrees of freedom" (Microsoft). | Yes |
| `cumulative` | TRUE selects the cumulative distribution function; FALSE selects the density. | Yes |

All three positions are required — the reference engine's projection records an arity of exactly
three, and Microsoft marks every argument Required. `cumulative` is not defaulted the way the
logical flag is on some other Excel distribution surfaces; there is no two-argument `T.DIST`.

`x` may be negative. That is the whole point of this surface: the legacy `TDIST` spelling
documented a non-negative `x` and returned a tail area, while `T.DIST` is the ordinary
left-tail CDF over the full real line.

Microsoft describes `deg_freedom` as an integer but the `T.DIST` page — unlike the `T.INV` and
`T.INV.2T` pages — does **not** say what happens to a non-integer value. See "What has not been
checked".

The numeric slots are subject to ordinary to-number coercion; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`, a probability in `(0, 1)` when `cumulative` is TRUE and a density value in
`(0, f(0; ν)]` when it is FALSE.

- **Large positive `x`, cumulative.** `F(x; ν)` approaches 1 and, in double precision, becomes
  indistinguishable from 1 long before the true value reaches it. Everything informative about
  the upper tail has been rounded away. `T.DIST.RT` exists precisely to keep it, and is the
  surface to use for right-tail work.
- **Large negative `x`, cumulative.** The left tail underflows gracefully toward 0 and stays
  informative far longer than the upper tail does, because the value itself is small rather than
  being a small perturbation of 1.
- **`x = 0`.** `F(0; ν) = ½` exactly, for every ν, by symmetry. The density at zero is
  `Γ((ν+1)/2) / (sqrt(νπ) Γ(ν/2))`, which is `1/π` at ν = 1 and tends to `1/sqrt(2π)` as
  ν → ∞. Both are exact-value probes that cost nothing to run.
- **Symmetry.** `T.DIST(−x, ν, TRUE) = 1 − T.DIST(x, ν, TRUE)` and
  `T.DIST(−x, ν, FALSE) = T.DIST(x, ν, FALSE)` are exact mathematical identities. Whether
  Excel's published bits satisfy them is an empirical question nobody has answered here.
- **Very large ν.** The distribution converges to the standard normal, so `T.DIST(x, ν, TRUE)`
  should approach `NORM.S.DIST(x, TRUE)`. How large ν has to be before the two agree to the last
  bit, and whether the approach is monotone in the published bits, is unprobed.
- **Arrays.** The projection records a `surface_native` lift-broadcast profile for this surface,
  which is the "not separately characterized" default rather than a determined fact; the axis
  provenance marks it `default-unexamined`. The reference engine's own battery, rendered beside
  this page, refuses an inline array in the `x` position. Whether Excel spills over array
  arguments here is not settled by anything the Handbook holds.

Empty, missing and error arguments follow the shared call model; see
[The value universe](../model/01-value-universe.md).

## Errors

As documented on Microsoft's `T.DIST` page:

| Error | Documented condition |
|---|---|
| `#VALUE!` | Any argument is nonnumeric. |
| unnamed error value | `deg_freedom < 1`. The page says only that the function "returns an error value" and that `deg_freedom` "needs to be at least 1". |

That second row is a documentation defect, not a summary of one. The sibling pages `T.DIST.2T`
and `T.DIST.RT` both name `#NUM!` for exactly the same condition; the `T.DIST` page declines to
name it. The Handbook records the gap rather than filling it in from the siblings — a reader who
needs the code should test for it, and a reader writing a compatibility implementation should
treat the code as unpinned by documentation.

Errors arriving in any argument propagate under the universal coercion rule. The Handbook has
not verified any of this against Excel.

## Relationships

- **[T.DIST.RT](FUNC.T.DIST.RT.md)** is the right-tail complement: `1 − F(x; ν)`, computed
  directly rather than by subtraction, and therefore the accurate surface for small upper-tail
  probabilities.
- **[T.DIST.2T](FUNC.T.DIST.2T.md)** is the two-tail area `P(|T| > x)`, defined only for
  `x ≥ 0`.
- **[T.INV](FUNC.T.INV.md)** is this function's inverse in the cumulative sense: `T.INV(p, ν)`
  is the `x` with `T.DIST(x, ν, TRUE) = p`. **[T.INV.2T](FUNC.T.INV.2T.md)** inverts `T.DIST.2T`
  instead.
- **`TDIST`** is the legacy compatibility spelling, and it is *not* this function.
  `TDIST(x, ν, tails)` takes `x ≥ 0` and returns a tail area — one-tailed for `tails = 1`,
  two-tailed for `tails = 2` — so its modern counterparts are `T.DIST.RT` and `T.DIST.2T`,
  not `T.DIST`. `T.DIST` has no legacy predecessor at all; the left-tail CDF and the density
  were both new surfaces when the dotted names arrived. Anyone mechanically rewriting `TDIST`
  to `T.DIST` is changing the mathematics, not modernizing a name.
- **`NORM.S.DIST`** is the ν → ∞ limit; **`F.DIST`** is a relative, since `T² ~ F(1, ν)`, which
  gives `F.DIST.RT(x², 1, ν) = T.DIST.2T(x, ν)` as an identity worth probing across the pair.
- **`CHISQ.DIST`** and **`BETA.DIST`** share the incomplete-gamma/incomplete-beta machinery that
  any t-distribution implementation is built on, and in the reference engine `T.DIST` shares its
  implementing module with the whole chi-square/F/t cluster.

## Numerical notes

Three separate difficulties, and they are not the same difficulty.

**The density.** Evaluating `Γ((ν+1)/2)/Γ(ν/2)` directly overflows for moderate ν even though
the ratio is perfectly well scaled — it grows only like `sqrt(ν/2)`. The standard remedy is
`exp(lgamma((ν+1)/2) − lgamma(ν/2))`, or the reciprocal of the beta function `B(½, ν/2)`; Cody's
and the fdlibm-lineage `lgamma` are the usual sources for the log-gamma. The second factor wants
`exp(−((ν+1)/2) · log1p(x²/ν))` rather than a literal `pow(1 + x*x/ν, ...)`: the `log1p` form
keeps relative accuracy when `x²/ν` is small, which is exactly the region where the density is
largest and where a spreadsheet user is most likely to be looking.

**The CDF.** The workhorse is the regularized incomplete beta function. Numerical Recipes'
`betai`/`betacf` pair — the Lentz continued fraction for `I_z(a, b)`, with the reflection
`I_z(a,b) = 1 − I_{1−z}(b,a)` applied when `z > (a+1)/(a+b+2)` so that the fraction is always
evaluated in its rapidly converging half — is the canonical treatment, and Cephes (`incbet`,
`stdtr`) and Boost.Math (`ibeta`, `students_t_distribution`) are the two implementations most
worth reading. For **integer** ν there is an alternative: A&S 26.7.3 and 26.7.4 give finite
closed-form expressions in `arctan` and powers of `sin`/`cos`, separately for odd and even ν.
These are exact in principle and fast, and they are also a trap — the alternating recursions in
them lose digits for large ν, which is why general-purpose libraries use the incomplete beta
instead. An implementation that switches on integer ν has to decide where to switch and be able
to defend the choice.

**Which tail you compute.** `F(x; ν)` for `x > 0` written as `1 − ½ I(...)` is a subtraction from
one, and the upper-tail information dies in it. A careful implementation computes the *small*
quantity directly — `½ I_{ν/(ν+x²)}(ν/2, ½)` — and only then subtracts, and it exposes the
small quantity as its own surface. That is the design reason `T.DIST.RT` exists next to
`T.DIST`, and it is why the two are not interchangeable at the last bits even where they agree
mathematically.

Nothing here is a claim about what Excel does internally. The Handbook does not know Excel's
algorithm for this surface.

## What has not been checked

No Handbook vector suite exists for `T.DIST`, and no Handbook evidence record lists this surface
among its subjects. **Nobody has checked `T.DIST` against Excel within the Handbook's record.**
The reference-engine battery rendered beside this page is the reference engine answering its own
questions; no Excel was involved in it.

The neighbouring evidence is worth naming precisely so that it is not mistaken for evidence
here. The Handbook's alias-pairing record for the legacy/modern distribution collapse covers the
five forward CDF pairs — and `T.DIST` is not one of them, because it has no legacy counterpart
to be paired with. Its module-sharing siblings in the reference engine have been measured;
`T.DIST` has not.

Documentation gaps this page could not close:

1. **Which error code `deg_freedom < 1` produces.** Microsoft's page does not name it.
2. **What happens to a non-integer `deg_freedom`.** `T.INV` and `T.INV.2T` document truncation;
   the three `T.DIST*` pages say nothing.
3. **Whether the projected axis values mean anything here.** The projection records
   `arg_domain_guard=none; non_finite=allow` for this surface, which would sit oddly beside a
   documented domain restriction on `deg_freedom` — but the axis provenance marks that value
   `default-unexamined`, so the tension is very likely an artifact of an unexamined default
   rather than a real disagreement. It is recorded here so that a later pass examines it rather
   than inheriting it.

Inputs worth probing first, and why:

1. **`T.DIST(0, ν, TRUE)` for many ν** — must be exactly `½`. The cheapest possible exactness
   probe, and a published value that is not exactly `½` would be immediately diagnostic of the
   evaluation path.
2. **`T.DIST(x, 1, TRUE)` against `½ + ATAN(x)/PI()`** — the Cauchy case has an elementary
   closed form, so this compares Excel against arithmetic rather than against another
   approximation.
3. **`T.DIST(x, 2, TRUE)` against `½(1 + x/SQRT(x²+2))`** — the same trick at the second
   integer, which is where an odd/even closed-form switch would first show itself.
4. **`T.DIST(−x, ν, TRUE) + T.DIST(x, ν, TRUE)`** — should be exactly 1. Any drift measures the
   asymmetry of the tail evaluation directly.
5. **`T.DIST(x, ν, FALSE)` at large ν against `NORM.S.DIST(x, FALSE)`**, and at ν = 1 against
   `1/(π(1+x²))` — the two ends of the density's parameter range.
6. **Non-integer `deg_freedom`** — `T.DIST(1, 2.5, TRUE)` against `T.DIST(1, 2, TRUE)`, which
   distinguishes truncation from genuine real-ν evaluation in one call.
7. **`deg_freedom = 0`, negative, and just below 1** — to pin the error code the documentation
   declines to name.
8. **Array arguments in each position**, to settle the lift question the projection leaves open.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `T.DIST` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-BETA.DIST-002: Incomplete beta has no basym arm, reflects bfrac prefactors above 0.99 using caller y, and unifies bup and bgrat

*Detail for matching Excel's bits; inferred; recorded 2026-11-22.* Applies to BETA.DIST, BETA.INV, BETADIST, BETAINV, BINOMDIST, CONFIDENCE.T, F.DIST, F.DIST.RT, FDIST, FINV, NEGBINOM.DIST, T.DIST, T.DIST.2T, T.DIST.RT, T.INV, T.INV.2T, TDIST, TINV.

In Excel 16.0.20527, the incomplete-beta routine (bratio) removes TOMS 708's bratio_basym arm:
all min(a0, b0) > 1.0, b0 >= 40.0 inputs stay on bratio_bfrac continuously across b = 100 and
lambda = 0.03 * b (checked across 22 live bisection points). Inside bratio_bfrac, when x > 0.99
(0x3fefae147ae147ae, strict >), Excel evaluates the density prefactor as excel_brcomp(b, a, y,
x) using the caller's stored complement y rather than 1.0 - x (eliminating 53-bit cancellation
in F.DIST.RT, FDIST, and F.DIST), while keeping lam = |a - (a + b) * x| on x. bratio_bup runs
Excel's density-based stored-power recurrence for all n >= 2 (including bup_140's n = 20
shift), using ((x * y) / a) * density with stored exp(m * ln(x)) and binary64 summation for b >
1.0, and extended-precision x^m and summation for b <= 1.0 (plus excel_log(y) in
beta_density_at for a < 1 || b < 1, x > 0.99). bratio_bgrat applies the 0.99 lnx switch to all
b, evaluates seeded b == 0.5 via erf/erfc, b == 1.0 via exp(-z), and z >= 1.1 via
excel_gamma_fraction(b, z).

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `BETA.DIST(3.725290298461914e-09, 12.375, 37.62500000000001, TRUE)` | x `3.725290298461914e-09` (`0x3e30000000000000`); alpha `12.375` (`0x4028c00000000000`); beta `37.62500000000001` (`0x4042d00000000001`); cumulative `TRUE` (cells) | `6.820007249574363e-94` (`0x2c96c2fa9478a1b0`) | Excel 16.0.20527.20196, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics |
| `F.DIST.RT(1.0601920728089276, 144, 39536)` | x `1.0601920728089276` (`0x3ff0f68bf68315b0`); deg_freedom1 `137.0` (`0x4061200000000000`); deg_freedom2 `39532.0` (`0x40e34d8000000000`) (cells) | `0.2989163342678236` (`0x3fd32171f9fae994`) | Excel 16.0.20527.20196, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics |

Basis (inferred): Measured across cb5 (1,638 live Excel 16.0.20527 calls;
frozen_cb5_f_gt099_v2.jsonl 59 of 100 vs 0 of 100, 99 of 100 within 5 ULP) and nx4
(frozen_nx4_predictions.jsonl 115 of 360 vs 34 of 360). Landing nx4 moved 29 beta-family
witness banks (168,767 rows) from 166,630 to 167,126 (+496 net, 0 full regressions: FDIST +206,
BETA.DIST +116, BETADIST +24, CONFIDENCE.T 4,282/4,282, FINV pinned 32/32).

Inferred, not measured: That Excel executes these branch rules and intermediate store sites is
inferred from continuity probes, frozen holdouts, and witness-bank improvements; inner-loop
rounding in bfrac and non-half bgrat remain open.

- Open: bfrac inner-loop 1-8 ULP bound (dyadic 2-step (64, 128, 2^-10) is -2 ULP; dc-j9k.55).
- Open: bgrat non-half +-6 ULP ceiling inherited from GAMMA.DIST(z, a < 1, 1, TRUE) (dc-j9k.55 / dc-j9k.59).
- Open: Remaining bup 1-3 ULP residuals and ge8 ln B +-1 ULP (dc-j9k.55).

Evidence:

- DnaCalc `crates/calc-func/src/ported/special_math_common.rs`, bratio; bratio_bfrac; bratio_bup; bratio_bgrat at `5c2054b1b7` — Carbon cb5 beta arms and nx4 refinements
- DnaCalc `crates/calc-func/src/ported/discrete_dist.rs`, beta_density_at at `5c2054b1b7` — x > 0.99 caller complement y in beta_density_at
- CalcFunc-Evidence `readings/excel-20527/studies/cb5.json.gz` at `4ef2c244d7` — 1,635 live Excel cb5 study rows
- CalcFunc-Evidence `readings/excel-20527/studies/nx4.json.gz` at `4ef2c244d7` — 360 frozen nx4 holdout rows
- [`content/notes/evidence/2026-11-22/frozen-holdouts-provenance.tsv`](../../content/notes/evidence/2026-11-22/frozen-holdouts-provenance.tsv), cb5; nx4 — Pre-ask SHA-256 hashes and frozen holdout scores
- CalcFunc-Evidence `readings/excel-20527/studies/cb5.json.gz`, 294 at `4ef2c244d7` (readings)
- CalcFunc-Evidence `readings/excel-20527/studies/nx4.json.gz`, $[219] at `4ef2c244d7` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| ν (`deg_freedom`) | The degrees-of-freedom parameter of the t-distribution |
| left-tail CDF | `F(x; ν) = P(T ≤ x)` — what `cumulative = TRUE` returns |
| density | `f(x; ν)` — what `cumulative = FALSE` returns |
| regularized incomplete beta | `I_z(a,b)`, the function the t CDF reduces to exactly |
| tail cancellation | The loss of upper-tail information when `F` is formed as `1 − (small)` |
| `default-unexamined` | Axis-provenance marker: the projected value is a default, not a finding |

## Sources

- Microsoft, "T.DIST function" —
  <https://support.microsoft.com/en-us/office/t-dist-function-4329459f-ae91-48c2-bba8-1ead1c6c21b2>
  (syntax, the three required arguments, the `cumulative` switch, the `#VALUE!` condition, and
  the unnamed error for `deg_freedom < 1`). Retrieved for this page.
- Microsoft, "T.DIST.2T function" and "T.DIST.RT function" — the sibling pages that *do* name
  `#NUM!` for `deg_freedom < 1`, cited here only to establish that the `T.DIST` page's silence
  is a gap rather than a shared convention.
- M. Abramowitz and I. A. Stegun, *Handbook of Mathematical Functions*, section 26.7 (Student's
  t-distribution: the density, the integral, and the finite closed forms for integer degrees of
  freedom, 26.7.3–26.7.4).
- W. H. Press et al., *Numerical Recipes*, the incomplete beta function `betai`/`betacf` and its
  application to Student's t.
- Cephes (`incbet`, `stdtr`) and Boost.Math (`ibeta`, `students_t_distribution`) as the two
  readable production treatments of the same reduction.
- Handbook [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.T.DIST.json` (arity, classification axes, and the
  `default-unexamined` provenance on the domain-guard and lift axes) and
  `data/presence/FUNC.T.DIST.json` (the shared `chi_f_t_family` implementing module).
