---
schema: efh.function-page/v1
function_id: FUNC.T.DIST.RT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0011
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
family: chi_f_t_family
role_in_family: >-
  The right-tail Student-t surface, and the one member of the t quartet that the Handbook can
  pair with its legacy spelling on recorded evidence: the alias-pairing record places TDIST and
  T.DIST.RT in its five-pair forward-CDF collapse.
---

# T.DIST.RT

## What it computes

`T.DIST.RT(x, deg_freedom)` returns the **right-tail** area of Student's *t*-distribution with
ν degrees of freedom:

    T.DIST.RT(x, ν) = P(T > x) = 1 − F(x; ν)

where `F` is the cumulative distribution function described on
[T.DIST](FUNC.T.DIST.md#what-it-computes). In terms of the regularized incomplete beta function
the identity is exact and, importantly, *direct* — the quantity is computed as itself rather
than as one minus something:

    T.DIST.RT(x, ν) = ½ · I_{ν/(ν+x²)}(ν/2, ½)          for x ≥ 0
    T.DIST.RT(x, ν) = 1 − ½ · I_{ν/(ν+x²)}(ν/2, ½)      for x ≤ 0

Domain and range: `x` over all reals, ν > 0; the value decreases strictly from 1 (as
`x → −∞`) to 0 (as `x → +∞`), passing through exactly `½` at `x = 0`. No poles, no branch cuts.

The asymptotics are the reason this surface is separate from `T.DIST` rather than a convenience
wrapper. As `x → ∞`,

    P(T > x) ~ ( Γ((ν+1)/2) · ν^(ν/2) ) / ( sqrt(νπ) · Γ(ν/2) · ν/2 ) · x^(−ν)

— a power law. The probability decays polynomially, so it stays representable as a double far
out into the tail, and it stays *informative* only if it is never formed by subtracting a
near-one quantity from one. A right-tail surface that internally computes `1 − T.DIST(...)`
throws away the entire reason it exists.

Special cases with elementary closed forms, useful as exactness probes:

| ν | `T.DIST.RT(x, ν)` |
|---|---|
| 1 | `½ − arctan(x)/π` |
| 2 | `½ · (1 − x / sqrt(x² + 2))` |
| ν → ∞ | `1 − Φ(x)` |

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `x` | The numeric value at which the distribution is evaluated. | Yes |
| `deg_freedom` | ν, "an integer indicating the number of degrees of freedom" (Microsoft). | Yes |

Two arguments only — the projection records an arity of exactly two. There is no `cumulative`
flag: this surface is a tail area, never a density.

Microsoft's `T.DIST.RT` page documents **no** restriction on the sign of `x`, which is a real
difference from the legacy `TDIST` spelling. `TDIST` documents `x ≥ 0` and returns `#NUM!`
below it. If Excel honours the modern page as written, `T.DIST.RT(−1, 5)` is a number greater
than a half. That is a specific, cheap probe, and it appears on the list below.

The page describes `deg_freedom` as an integer but does not say what a non-integer value does.
The `T.INV` and `T.INV.2T` pages document truncation for the same parameter; these three
`T.DIST*` pages do not.

The numeric slots take ordinary to-number coercion; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`, a probability in `(0, 1)`.

- **`x = 0`** gives exactly `½` for every ν, by symmetry.
- **Large positive `x`** gives a small probability that remains meaningful — the power-law tail
  means `T.DIST.RT` keeps returning distinguishable values in a range where
  `1 − T.DIST(x, ν, TRUE)` has already collapsed to zero. This is the intended use.
- **Large negative `x`** gives a value approaching 1 from below, where the *other* member of the
  pair (`T.DIST`, cumulative) carries the accuracy instead. The two surfaces are accurate in
  opposite halves of the line; neither is a superset of the other.
- **The complement identity.** `T.DIST.RT(x, ν) + T.DIST(x, ν, TRUE) = 1` and
  `T.DIST.RT(−x, ν) = T.DIST(x, ν, TRUE)` are exact mathematically. Whether Excel's published
  bits satisfy them is exactly the kind of metamorphic question a suite should settle, and no
  suite exists.
- **Arrays.** The projection carries the `surface_native` lift profile with
  `default-unexamined` provenance, and the reference-engine battery beside this page refuses an
  inline array argument. Whether Excel spills here is unsettled.

Note also that this surface is one of the two on this page's module carrying an open upstream
defect stream, `BUG-FUNC-021`, whose title names a statistical numeric exactness drift. The
Handbook cites the stream by name; it does not restate its contents or its figures.

## Errors

As documented on Microsoft's `T.DIST.RT` page:

| Error | Documented condition |
|---|---|
| `#VALUE!` | Any argument is nonnumeric. |
| `#NUM!` | `deg_freedom < 1`. |

No documented `#NUM!` for negative `x` — again, unlike the legacy `TDIST`. Errors in either
argument propagate under the universal coercion rule. The Handbook has not verified any of this
against Excel.

## Relationships

- **`TDIST`** is the legacy spelling, and this is the one place in the t family where the
  Handbook holds recorded evidence about the pairing. The Handbook's alias-pairing record
  `EV-DIST-0011` places `TDIST` and `T.DIST.RT` among five legacy/modern forward-CDF pairs found
  to publish the same results everywhere the recorded battery probed them. Two cautions travel
  with that, and the record states both: it is an Excel-versus-Excel identity, so **it transfers
  no pass rate onto either surface**, and it covers forward CDFs only — no inverse pair has ever
  been probed for identity. Note also that `TDIST(x, ν, 1)` is the corresponding call: `TDIST`
  carries a `tails` argument, and `tails = 2` corresponds to `T.DIST.2T` instead.
- **[T.DIST](FUNC.T.DIST.md)** is the left-tail complement and the density surface.
- **[T.DIST.2T](FUNC.T.DIST.2T.md)** is the two-tail area; for `x ≥ 0`,
  `T.DIST.2T(x, ν) = 2 · T.DIST.RT(x, ν)` exactly. Whether the two Excel surfaces satisfy that
  in published bits is unprobed and is the sharpest cheap probe on the pair.
- **[T.INV](FUNC.T.INV.md)** inverts the left tail; **[T.INV.2T](FUNC.T.INV.2T.md)** inverts the
  two-tail area. There is no `T.INV.RT` — the right-tail inverse is spelled
  `T.INV(1 − p, ν)`, which itself throws away precision for small `p` and is a real gap in the
  surface set rather than an omission this page can paper over.
- **`F.DIST.RT`**: `T² ~ F(1, ν)`, so `F.DIST.RT(x², 1, ν) = T.DIST.2T(x, ν) = 2·T.DIST.RT(x, ν)`
  for `x ≥ 0`. Both surfaces live in the same reference-engine module, which makes the identity
  a useful internal-consistency probe.
- **`T.TEST`** and the regression `p`-values are the main consumers of this surface in practice.

## Numerical notes

The whole engineering content of a right-tail distribution function is: *never form it by
subtraction*. Written as `1 − F(x; ν)` in double precision, every value below about `2^-53`
relative to one is gone, so the surface would saturate at zero for moderate `x` and stay there.
Computed directly as `½ I_{ν/(ν+x²)}(ν/2, ½)`, it stays accurate to relative precision as far out
as the power-law tail goes.

The reduction to the incomplete beta is the standard one; the continued fraction of Numerical
Recipes' `betacf`, with the reflection `I_z(a,b) = 1 − I_{1−z}(b,a)` applied so that the fraction
is always evaluated on its fast-converging side, is the canonical route. Cephes' `stdtr` and
Boost.Math's `students_t_distribution` (built on `ibeta`/`ibetac`, where the `ibetac`
complement entry point exists for exactly this reason) are the two implementations to read. The
existence of a separate complement entry point in a serious library is the same design decision
Excel made by publishing `T.DIST.RT` as its own function.

For **integer** ν, A&S 26.7.3–26.7.4 give finite closed forms in `arctan` and powers of sine and
cosine, and there is a companion recursion for the tail area. They are exact in exact arithmetic
and lose digits for large ν in floating point; a switch on integer ν is defensible but has to
name its crossover.

Argument reduction has one subtlety worth stating: the beta argument `z = ν/(ν + x²)` needs care
when `x²` overflows or when `ν` is huge. Computing it as `1/(1 + x²/ν)` — or, better, keeping
the complement `x²/(ν + x²)` and calling the complementary beta — avoids both the overflow and
the loss of the small quantity.

None of this is a claim about Excel's internals. The Handbook does not know Excel's algorithm.

## What has not been checked

No Handbook vector suite exists for `T.DIST.RT`. One Handbook evidence record lists this surface
among its subjects: `EV-DIST-0011`, the alias-pairing record for the legacy/modern collapse. What
that record establishes is an **Excel-versus-Excel** identity between `TDIST` and `T.DIST.RT` on
its battery — it says nothing about whether either matches a correctly rounded value, and the
record's own reader warning forbids reading a pass rate out of it. So: **nobody has checked
`T.DIST.RT` against a mathematical reference within the Handbook's record.** The figures rendered
beside this page from that record belong to the evidence layer and are not restated here.

Documentation gaps this page could not close:

1. **Negative `x`.** Documented without restriction here and forbidden on the legacy page. If
   Excel implements the legacy restriction on the modern surface, the documentation is wrong; if
   it does not, then `TDIST` and `T.DIST.RT` differ on `x < 0` in a way that the alias-pairing
   record's battery may or may not have covered.
2. **Non-integer `deg_freedom`** — silent here, documented as truncated on the inverse pages.

Inputs worth probing first:

1. **`T.DIST.RT(−1, 5)`** — one call, and it settles the sign question that separates this
   surface from its legacy spelling. Nothing else on the list is cheaper or more diagnostic.
2. **`T.DIST.RT(x, 1)` against `½ − ATAN(x)/PI()`** and **`T.DIST.RT(x, 2)` against
   `½(1 − x/SQRT(x²+2))`** — the two elementary cases, comparing Excel against arithmetic rather
   than against another approximation.
3. **`2 * T.DIST.RT(x, ν)` against `T.DIST.2T(x, ν)`** across a grid of `x` and ν — an exact
   identity between two Excel surfaces, so any disagreement is a self-inconsistency in Excel and
   needs no external oracle at all.
4. **`T.DIST.RT(x, ν) + T.DIST(x, ν, TRUE)` against 1** — the same argument, one step further.
5. **Deep tail**: `x` at 10, 30, 100 for ν = 1, 2, 5, 30, where the power-law decay means the
   true value is still far above the underflow floor, and any surface that subtracts internally
   will already have collapsed.
6. **`F.DIST.RT(x², 1, ν)` against `2·T.DIST.RT(x, ν)`** — the cross-family identity, over
   surfaces the reference engine implements in one module.
7. **`deg_freedom` at 0, negative, non-integer, and enormous.**

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| right tail | `P(T > x)`, the quantity this surface returns |
| complement entry point | A library function computing `1 − F` directly rather than by subtraction |
| alias-pairing record | An Excel-versus-Excel identity finding; transfers no pass rate |
| power-law tail | `P(T > x) ∝ x^(−ν)` — the decay that makes deep-tail values representable |
| `default-unexamined` | Axis-provenance marker: the projected value is a default, not a finding |

## Sources

- Microsoft, "T.DIST.RT function" —
  <https://support.microsoft.com/en-us/office/t-dist-rt-function-20a30020-86f9-4b35-af1f-7ef6ae683eda>
  (syntax, both arguments required, the `#VALUE!` and `#NUM!` conditions, and the absence of any
  documented restriction on the sign of `x`). Retrieved for this page.
- Handbook evidence record `EV-DIST-0011` — the legacy/modern forward-CDF alias pairing that
  includes `TDIST` / `T.DIST.RT`, with its reader warning that an Excel-versus-Excel identity
  transfers no pass rate and that no inverse pair has been probed.
- M. Abramowitz and I. A. Stegun, *Handbook of Mathematical Functions*, section 26.7 (Student's
  t-distribution, including the finite closed forms 26.7.3–26.7.4 for integer degrees of
  freedom).
- W. H. Press et al., *Numerical Recipes*, incomplete beta function and its continued fraction.
- Cephes `stdtr` and Boost.Math `ibetac` / `students_t_distribution` — complement entry points as
  a deliberate design pattern.
- OxFunc defect stream
  `docs/bugs/streams/BUG-FUNC-021_w090_statistical_numeric_exactness_drift.md`, named by the
  presence projection for this surface. Cited by name only.
- Handbook projections `data/functions/FUNC.T.DIST.RT.json` and
  `data/presence/FUNC.T.DIST.RT.json`.
