---
schema: efh.function-page/v1
function_id: FUNC.PHI
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0022
  - EV-DIST-0018
open_problems: []
references:
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 26, 26.2.1 (the normal density Z(x))"
    role: "The defining density, its derivatives and its relation to the normal probability integral"
  - work: "Cephes mathematical library (Moshier), ndtr.c"
    locator: "ndtr / erfc"
    role: "A reference implementation of the normal density and integral in double precision"
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
family: phi_fn
role_in_family: >-
  The bare standard normal density; the only surface in the normal group whose Excel op-graph has
  been identified as a squaring, an exponential and a multiply, with no erf and no CDF in the
  chain.
---

# PHI

## What it computes

`PHI(x)` returns the value at `x` of the density function of the standard normal
distribution — Abramowitz & Stegun's `Z(x)`:

    φ(x) = (1 / √(2π)) · e^(−x² / 2)

**Domain and range.** Defined for every real `x`. The function is strictly positive,
even (`φ(−x) = φ(x)`), and unimodal with its maximum at the origin:

    φ(0) = 1 / √(2π)  ≈ 0.3989422804014327
    φ(x) → 0          as |x| → ∞, faster than any exponential in |x|
    range: (0, 1/√(2π)]

There are no poles and no branch cuts; `φ` is entire as a function of a complex argument. Its
inflection points sit at `x = ±1`, which is where the standard deviation shows up in the
picture of the curve.

**Why this function exists at all.** It is the integrand of the normal probability integral,
and the identities that matter are differential rather than algebraic:

    φ'(x)  = −x · φ(x)
    Φ'(x)  = φ(x)                       where Φ is the standard normal CDF
    φ''(x) = (x² − 1) · φ(x)

More generally `φ^(n)(x) = (−1)^n He_n(x) φ(x)`, with `He_n` the probabilists' Hermite
polynomials — A&S 26.2.36 and chapter 22. Every asymptotic expansion of the normal tail is
built by repeated integration by parts on `φ`, which is why a normal-tail routine that is
accurate in the far tail almost always has an accurate `φ` inside it.

**Special values.** `φ(0) = 1/√(2π)` is the one exactly-nameable value, and it is the reason a
single cell is a meaningful test: any implementation that gets `φ(0)` wrong has a rounding
error in the constant, not in the exponential.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `x` | The point at which the density is evaluated. Required. | — |

Microsoft describes `x` as "the number for which you want the density of the standard normal
distribution". There is no mean, no standard deviation, and no cumulative switch: `PHI` is the
standardized density and nothing else. To get a general normal density, scale it —
`NORM.DIST(x, μ, σ, FALSE) = PHI((x − μ) / σ) / σ` mathematically, though whether the two
Excel surfaces agree in their last bits is a separate, unchecked question.

The reference engine records an arity of exactly 1, a `NumToNum` kernel signature, and a
`UnaryNumericScalarOrArrayElementwise` coercion/lift profile — this is a genuine scalar kernel
that lifts elementwise over an array argument, per
[Coercion and lifting](../model/02-coercion-and-lifting.md).

`PHI` is available from Excel 2016 onward per Microsoft's version list; it is a modern addition
with no legacy spelling.

## Result and edge cases

Returns `Number`.

- **Symmetry.** `PHI(x)` and `PHI(−x)` are mathematically equal. Whether they are equal in
  their last bits depends on whether the implementation squares `x` before anything else —
  a squaring is exactly symmetric, so a squaring-first staging gives exact symmetry for free.
  That makes the symmetry test a *weak* probe of the op-graph and a *strong* probe of
  correctness.
- **Underflow.** `e^(−x²/2)` reaches the smallest normal double when `x²/2` passes about `708`,
  that is `|x| ≈ 37.6`, and vanishes entirely when `x²/2` passes about `745`, `|x| ≈ 38.6`.
  Between those two lies a **subnormal band** roughly `37.6 < |x| < 38.6` where the result is a
  subnormal double with progressively fewer significant bits. This band is not a curiosity: it
  is where implementations differ most, and the evidence record attached to this page reports
  that OxFunc's identification of Excel's `PHI` required a *live-pinned subnormal publication
  flush* — a behaviour at exactly this boundary that had to be observed rather than derived.
- **Very large `|x|`.** Beyond the band the true value is zero to within any representable
  precision, and the reference engine's battery renders the largest-finite-double input; its
  outcome shows beside this page.
- **Very small `|x|`.** Near the origin the function is flat (`φ'(0) = 0`), so `PHI` is
  extremely well conditioned there; the relative error of the result is essentially the
  relative error of the constant `1/√(2π)`.
- **Arrays.** Elementwise, per the recorded lift profile.

## Errors

As documented by Microsoft on the `PHI` page (retrieved for this page):

| Error | Documented condition |
|---|---|
| `#NUM!` | `x` is a numeric value that is not valid |
| `#VALUE!` | `x` uses a data type that is not valid, such as a nonnumeric value |

The `#NUM!` row is documented without a stated inequality — "a numeric value that is not
valid" names no boundary. Since `φ` is defined on the whole real line, it is not obvious what
input this row is meant to cover; the natural candidate is a magnitude so large that an
intermediate `x²` overflows, but Microsoft's page does not say so and this Handbook will not
invent it. That gap is on the probe list.

Error values in the argument propagate under the ordinary coercion rules.

## Relationships

- **`NORM.S.DIST(x, FALSE)`** computes the same mathematical function. `PHI` is the shorter
  spelling of the non-cumulative standard normal, added later. Two surfaces for one density is
  exactly the situation in which "are they the same computation?" must be asked rather than
  assumed — and here the Handbook has an unusually pointed reason to ask: `EV-DIST-0022`
  identifies `PHI`'s op-graph as a squaring, an exponential and a multiply by a *rounded*
  `1/√(2π)`, with **no erf and no CDF anywhere in the chain**, while the normal CDF surfaces
  necessarily have one. If Excel routed `PHI` through `NORM.S.DIST`, that identification would
  look different. See [NORM.S.DIST](FUNC.NORM.S.DIST.md).
- **`NORM.DIST(x, 0, 1, FALSE)`** and the legacy `NORMDIST(x, 0, 1, FALSE)` are two more
  spellings of the same density. Four surfaces, one function, no published proof that any two
  of them agree bit for bit.
- **[GAUSS](FUNC.GAUSS.md)** is the neighbouring modern addition: `GAUSS(z) = Φ(z) − 0.5`, the
  probability that a standard normal falls between `0` and `z`. `PHI` and `GAUSS` arrived
  together and are constantly confused — `PHI` is the *density*, `GAUSS` is a *probability*.
  They are related by `GAUSS'(z) = PHI(z)`. `EV-DIST-0018` names both, and records that both
  were the ones that drifted in that sweep while the eight CDF-side witnesses did not; `GAUSS`
  remained open there for want of an erf substrate, and `PHI` was identified later — by a route
  that turned out not to be an erf substrate at all.
- **`ERF`** and **`ERFC`** are the underlying special functions of the normal *integral*, not
  of its density. `PHI` needs neither.
- **`EXP`** is `PHI`'s engine. Any account of `PHI`'s last bits is mostly an account of `EXP`'s.
- **Confused with**: `NORM.S.DIST(x)` written with the cumulative argument set to `TRUE`, which
  is a completely different curve.

## Numerical notes

`PHI` is a two-operation function with one genuinely hard property, and it is a good miniature
of why "simple" special functions are not.

**1. The squaring is the accuracy bottleneck, not the exponential.**

Let `x̂ = x(1 + δ)` be the stored argument and consider `e^(−x²/2)`. An absolute error `ε` in
the exponent produces a *relative* error of about `ε` in the result. Forming `x²` in double
precision introduces a relative error of up to `2^−53`, hence an absolute error in `x²/2` of
about `x²/2 · 2^−53`. So:

    relative error of φ(x)  ≳  (x² / 2) · 2^−53

At `x = 10` that is about `5.5 × 10^−15` — tens of ULP. At `x = 30` it is about
`5 × 10^−14` — hundreds of ULP. **The error grows quadratically in `x` and has nothing to do
with the quality of the exponential.** This is the single most important fact about
implementing a normal density, and it is why a `PHI` that is correctly rounded near the origin
can be far off in the tail while every individual operation is faithfully rounded.

The standard remedy is to carry `x²/2` in more than double precision: split `x` into a
high and low part (Dekker–Veltkamp splitting, or a fused multiply-add if one is available),
form `x²= x_hi² + 2x_hi·x_lo + x_lo²` as a double-double, and evaluate
`e^(−h) · e^(−l)` with `l` small enough that `e^(−l) ≈ 1 − l` to full precision. Boost's normal
distribution and the better libm normal routines do a version of this; Cephes's `ndtr`
addresses the same problem in the CDF by working through `erfc`.

**2. The constant matters at the origin and only there.**

`1/√(2π)` is irrational; every implementation multiplies by *some* double near it. Whether the
implementation stores the correctly rounded `1/√(2π)`, or computes `1/sqrt(2*pi)` at runtime
(two roundings), or divides by a stored `√(2π)` (a different rounding again) is directly
visible at `x = 0`, where the exponential is exactly `1` and the answer is nothing but the
constant. This is why `PHI(0)` is a genuinely diagnostic single cell, and why the ruled-out
list on `EV-DIST-0022` includes "PHI as a division by `RN(√(2π))`" as a *distinct hypothesis
from* the multiply — the two stagings differ in the last bit of the answer at the origin.

**3. The subnormal band is a publication question, not an arithmetic one.**

In `37.6 ≲ |x| ≲ 38.6` the result is subnormal. What an implementation does there — compute
faithfully into the subnormal range, flush to zero, or flush at some particular threshold — is
a platform and code-path decision, not a mathematical one, and it is not derivable from the
formula. `EV-DIST-0022` records that this had to be pinned against live observation.

**4. What a careful independent implementation does.**

For the `natural-best` flavour: compute `x²/2` in double-double, exponentiate the high part
with a faithful `exp`, correct with the low part, and multiply by the correctly rounded
constant — giving a small stated bound over the whole real line including the subnormal band.
For the `math-correct` flavour: evaluate in higher precision and round once. For
`excel-bitexact`: reproduce the identified op-graph including its rounding sequence and its
flush behaviour. These are three different functions and each is right for its stated purpose;
see [About implementation options](../model/07-implementation-options.md).

The Handbook does not assert what Excel does internally beyond what the attached evidence
records say, in their own scope.

## What has not been checked

Two evidence records name this surface, and neither is a vector suite.

`EV-DIST-0022` is a substrate identification: it records an op-graph for Excel's `PHI` —
squaring, an x87 exponential, a multiply by the rounded `1/√(2π)`, and a live-pinned subnormal
publication flush — together with the rival stagings it ruled out. Its own reader warning
states that its corpus was the identification round's own rather than a held-out one, and that
warning renders mechanically beside this page. `EV-DIST-0018` is a ten-witness re-sweep across
the normal group; `PHI` is one of its named subjects, and it is one of the two surfaces the
record reports as having drifted at that sweep. Its group figure carries a reader warning
forbidding per-surface attribution, so the family was measured and this surface was not
measured separately by that figure.

No Handbook vector suite exists for `PHI`. No Handbook measurement of any kind exists for it —
both records are upstream OxFunc work the Handbook has not re-verified.

The documented behaviour above was retrieved from Microsoft's `PHI` page. The `#NUM!` condition
there names no input.

Inputs worth probing first:

1. **`PHI(0)`.** The constant, alone, with the exponential removed from the picture. One cell
   distinguishes a stored correctly rounded `1/√(2π)` from a runtime `1/sqrt(2*pi)` and from a
   division by a stored `√(2π)` — the exact three-way split the ruled-out ledger records.
2. **`PHI(x)` against `PHI(−x)`** across the range. Exact agreement is evidence for a
   squaring-first staging; any disagreement would be a significant finding, since it would rule
   that staging out.
3. **A dense walk through `37 ≤ |x| ≤ 39`** — the subnormal band, at fine spacing. This is
   where the flush behaviour lives and where any two implementations are most likely to differ.
4. **`PHI(10)`, `PHI(20)`, `PHI(30)`** against a high-precision reference. The predicted
   quadratic-in-`x` error growth from the squaring is either present or it is not, and this
   settles whether Excel carries the exponent in extended precision.
5. **`PHI(x)` against `NORM.S.DIST(x, FALSE)` and `NORM.DIST(x, 0, 1, FALSE)`** at the same
   arguments. Three surfaces for one density; any disagreement proves they are distinct code
   paths, and agreement everywhere would be the beginning of an argument that they are not.
6. **A magnitude large enough to overflow an intermediate `x²`** — around `1.4 × 10^154` — to
   find out what the undocumented `#NUM!` condition actually covers, if anything.
7. **`PHI` over an array argument**, to confirm the recorded elementwise lift.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| standard normal density | `φ(x) = e^(−x²/2)/√(2π)`; A&S's `Z(x)` |
| subnormal band | The `|x|` range in which `φ(x)` is representable only as a subnormal double |
| squaring bottleneck | The quadratic-in-`x` relative error induced by rounding `x²` |
| publication flush | Setting a subnormal result to zero at publication; observed, not derived |
| op-graph | The identified sequence of machine operations and roundings for a surface |

## Sources

- Microsoft, "PHI function" —
  <https://support.microsoft.com/en-us/office/phi-function-23e49bc6-a8e8-402d-98d3-9ded87f6295c>
  (syntax, the argument description, the `#NUM!` and `#VALUE!` conditions, and version
  availability from Excel 2016). Retrieved for this page.
- Handbook evidence record `EV-DIST-0022` — OxFunc's substrate identification for `PHI`, with
  its ruled-out ledger and its reader warning about the corpus.
- Handbook evidence record `EV-DIST-0018` — the ten-witness normal-group re-sweep, which names
  `PHI` as a subject and whose group figure may not be attributed per surface.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 26 §26.2 (`Z(x)`, the
  normal density, and its Hermite-polynomial derivatives) and chapter 22 (Hermite polynomials).
- Cephes (S. L. Moshier), `ndtr.c` — the normal integral and density in double precision;
  cited as a reference implementation of the erfc-based route the identified Excel `PHI`
  op-graph does *not* take.
- T. J. Dekker, "A floating-point technique for extending the available precision",
  *Numerische Mathematik* 18 (1971) — the splitting used to carry `x²/2` in double-double.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [About implementation options](../model/07-implementation-options.md).
- `data/functions/FUNC.PHI.json` — arity 1, `NumToNum`,
  `UnaryNumericScalarOrArrayElementwise` coercion profile, XLL symbol `xlfPhi`.
- `data/presence/FUNC.PHI.json` — implementing module
  `crates/oxfunc_core/src/functions/phi_fn.rs`, shared with no other surface.
