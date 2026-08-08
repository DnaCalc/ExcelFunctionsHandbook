---
schema: efh.function-page/v1
function_id: FUNC.COS
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0004
open_problems: []
references:
  - work: "Microsoft Support — COS function"
    locator: "https://support.microsoft.com/en-us/office/cos-function-0fb808a5-95d6-4553-8148-22aebdce5f05"
    role: "documented signature, the radians requirement, and the RADIANS()/PI()/180 conversion note"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 4.3 (Circular Functions)"
    role: "definition, identities, series, and the polynomial approximations for cosine"
  - work: "Cody & Waite, Software Manual for the Elementary Functions"
    locator: "the SIN/COS chapter"
    role: "the classical two- or three-part argument reduction and its stated range of validity"
  - work: "Payne & Hanek, Radian reduction for trigonometric functions (SIGNUM Newsletter, 1983)"
    locator: null
    role: "infinite-precision argument reduction, the only route that is accurate for huge arguments"
  - work: "Muller, Elementary Functions: Algorithms and Implementation"
    locator: "the argument-reduction and worst-case chapters"
    role: "the modern treatment of reduction error and hardest-to-round cases"
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
family: cos
role_in_family: >-
  The circular cosine — one of the three published trigonometric primaries, and the surface the
  reciprocal SEC is built from.
---

# COS

## What it computes

`COS(number)` returns the cosine of an angle **in radians**. Microsoft's page says exactly that,
and adds the conversion note: use `RADIANS(x)` or multiply degrees by `PI()/180`.

The mathematics, as A&S chapter 4.3 states it:

    cos z = (e^{iz} + e^{-iz}) / 2
    cos x = 1 - x²/2! + x⁴/4! - x⁶/6! + …          (convergent for all x)

| Property | Statement |
|---|---|
| Domain | all real `x` (mathematically; see the guard note below) |
| Range | `[-1, 1]`, attained |
| Parity | even: `cos(-x) = cos(x)` |
| Period | `2π`, and `cos(x + π) = -cos(x)` |
| Zeros | `x = π/2 + kπ`, all simple — **none of them is a binary64 number** |
| Pythagorean identity | `cos²x + sin²x = 1` |
| Phase shift | `cos x = sin(x + π/2)` |
| Derivative | `d/dx cos x = -sin x` |
| Entire | no poles, no branch cuts anywhere in the complex plane |

The row that carries the most practical weight is the one about zeros. `π/2` is irrational, so
no double is exactly a zero of the cosine; `COS(PI()/2)` is not zero and cannot honestly be
expected to be. What it *is* equals, to first order, the distance from the double `PI()/2` to
the true `π/2` — which makes it a direct readout of how well the constant is rounded, not a
defect in the cosine.

There is no analytic difficulty in cosine anywhere. It is entire, it is bounded, and its
Taylor series converges everywhere. **The entire numerical problem is argument reduction**, and
that is the subject of the numerical notes.

## Arguments

| Argument | Meaning | Notes |
|---|---|---|
| `number` | The angle in radians. Required. | Microsoft states no magnitude constraint on this page |

The slot takes ordinary to-number coercion, and the reference engine declares this surface a
scalar kernel that lifts elementwise over arrays
(`UnaryNumericScalarOrArrayElementwise`) — see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

**A divergence to record.** The reference engine's real-result policy for `COS` carries
`arg_domain_guard=circular_trig_overflow`, a magnitude guard. Microsoft's `COS` page documents
no magnitude constraint at all; the `|number| < 2^27` constraint appears on the *later*
trigonometric pages — [COT](FUNC.COT.md), [CSC](FUNC.CSC.md), `SEC` — and not on this one. So
either `COS` inherits an undocumented bound, or the guard is applied more widely than Excel
applies it. The projection does not say, and the Handbook has not checked.

## Result and edge cases

Returns `Number` in `[-1, 1]`.

- **Zero** — `COS(0) = 1` exactly, and that is the one value the function returns exactly for
  a reason rather than by luck.
- **Large arguments** — mathematically well defined and, for a double argument, a perfectly
  determinate real number: a double *is* an exact rational, so `cos` of it has one true value.
  The question is only whether an implementation computes it. See below.
- **`COS(PI())`** — not exactly `-1`, for the same reason `COS(PI()/2)` is not exactly `0`.
- **Overflow is impossible.** The output is bounded, so the `non_finite=num` half of the
  real-result policy can only fire because of the argument guard, never because of the value.
- **Degrees.** The single most common wrong answer from this function is a correct cosine of
  the wrong angle. Microsoft's page pre-empts it; the Handbook repeats it.

## Errors

Microsoft's `COS` page documents no error conditions. The shared call model supplies the rest:
non-numeric text in the numeric slot surfaces `#VALUE!`, and an error value in the argument
propagates. The mechanically rendered battery beside this page shows the boundary probes.

If the `circular_trig_overflow` guard noted above corresponds to Excel behaviour, a large enough
argument would produce `#NUM!` — an outcome Microsoft's page does not document for `COS`. That
is stated here as an open question, not as behaviour.

## Relationships

- **`SIN`, `TAN`** — the other two published primaries. `EV-MATH-0004` treats all six circular
  functions as one identification problem, and it is the right way to read them.
- **`SEC`** — the reciprocal, `1/COS`. `EV-MATH-0004` records that the reciprocal surfaces are
  built from the *published* primaries, which matters: see the numerical notes.
- **`ACOS`** — the inverse, principal branch `[0, π]`. `ACOS(COS(x))` recovers `x` only on
  `[0, π]`; elsewhere it returns the folded representative.
- **[COSH](FUNC.COSH.md)** — the hyperbolic partner, `cosh x = cos(ix)`. The name is the only
  thing they share computationally: `COSH` overflows and has no reduction problem, `COS` cannot
  overflow and has nothing *but* a reduction problem.
- **`RADIANS`, [DEGREES](FUNC.DEGREES.md)** — the unit converters that stand between this
  function and most spreadsheet users.
- **`IMCOS`** — the complex-argument version, which needs `COS`, `SIN`, `COSH` and `SINH`
  together.
- **`PI()`** — the constant whose rounding sets the error at every "should be zero" point.

## Numerical notes

Cosine is the textbook case where the interesting error is committed *before* the approximation
starts.

**The reduction problem.** To evaluate `cos x` you write `x = k·(π/2) + r` with `|r| ≤ π/4`,
then use a polynomial in `r` and a quadrant rule. Since `π/2` is irrational, computing `r`
requires the true `π/2` to enough bits. If the reduction uses a `π/2` stored to `p` bits, the
computed `r` carries an absolute error of roughly `|x| · 2^-p`, and because `cos` has unit
derivative near its zeros, that absolute error passes straight into the result. For `|x|` near
`2^30` and a 64-bit stored constant the reduced argument can be wrong in its leading bits;
by `|x|` near `2^60` it can be entirely noise.

The three standard answers:

1. **Cody & Waite** two- or three-part reduction: split the constant into exactly representable
   pieces and accumulate `x - k·π/2` in stages. Cheap, and correct only up to a stated argument
   magnitude, which the authors state.
2. **Payne–Hanek** infinite-precision reduction: multiply by a long stored expansion of `2/π`
   and keep only the fractional part. Correct for every finite double, at real cost. This is
   what a `math-correct` flavour must do.
3. **Hardware reduction** — the x87 `FPREM1` instruction reduces against a `π` stored in ROM to
   66 bits. Fast, and its accuracy ceiling is that constant.

**What the evidence says.** `EV-MATH-0004` is a substrate identification covering all six
circular surfaces, each scored individually. It names the substrate as the legacy CRT
`fFSIN`/`fFPTAN` chain with `FPREM1` argument reduction against the 64-bit ROM `π`, and it
records that Cody–Waite reduction against extended `π` is *ruled out* — the pre-sign-off
catalogue row recorded large argument-reduction drift in the moderate-to-large range, which is
the fingerprint of a limited-precision reduction constant rather than a bad polynomial. The
figures live in the evidence layer and render beside this page. The record also carries a
host-CPU microcode caveat on `FPREM1`/`FSIN`/`FPTAN`: an answer produced through those
instructions is a property of the CPU as much as of the software, so this surface's last bits
are platform-scoped by construction.

**The polynomial half is the easy half.** Once `|r| ≤ π/4`, a minimax polynomial in `r²` of
modest degree delivers well under an ulp; A&S 4.3.98–4.3.99 gives the classical coefficient
sets, fdlibm's `__kernel_cos` is the standard modern reference, and correctly-rounded
implementations (CRlibm, and the CORE-MATH successors) show the achievable target.

**A note for anyone building `SEC` or comparing against it**: computing `1/COS(x)` from the
*published* (already rounded to binary64) cosine double-rounds — the cosine's own error is
inverted along with the value. That is a different function from a directly-evaluated secant,
and `EV-MATH-0004` records the reciprocal construction explicitly.

## What has not been checked

`EV-MATH-0004` names this surface. It is a substrate identification with per-surface live-Excel
comparison and a held-out component, and it carries a reader warning: the group figure quoted
in the upstream catalogue is a group total over all six trigonometric surfaces and must not be
read as a per-surface rate. All six inherit the host-CPU microcode caveat.

No Handbook vector suite exists for `COS`. Microsoft's page documents the radians requirement
and nothing else; the argument-magnitude guard recorded in the projection is **not** documented
for this surface, and the Handbook has not observed Excel refusing a large argument.

Inputs I would probe first:

1. **The guard question, directly**: `COS(2^27)`, `COS(2^27 - 1)`, `COS(2^30)`, `COS(1E300)`.
   Whether any of these is `#NUM!` decides whether the `circular_trig_overflow` axis describes
   Excel or only the reference engine. The answer is a kind, not a number, so it settles cleanly
   — and it is the largest unresolved item on this page.
2. **The reduction ladder**: `COS(x)` at `x = 2^k` for `k` from 20 to 52, compared against a
   Payne–Hanek reference. This is the classical way to *see* the reduction constant's precision:
   the error grows with `|x|` in a way that reads off the number of correct bits in the stored
   `π`.
3. **Near-zero-of-cosine points**: the doubles nearest `π/2 + kπ` for small `k`. Cosine's
   relative error is worst there, and it is where an implementation's reduction and its
   polynomial can be told apart.
4. **`COS(x)² + SIN(x)² - 1`** across the range — a metamorphic probe that needs no oracle and
   localises which of the two primaries drifts.
5. **`SEC(x) · COS(x) - 1`** — tests the reciprocal construction the record describes.
6. **The same vectors on a second CPU vendor**, given the explicit microcode caveat. Any
   difference is a platform axis, not a software bug.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| argument reduction | Replacing `x` by `r` in a small interval plus a quadrant index |
| reduction constant | The stored approximation to `π/2` (or `2/π`) the reduction divides by |
| Payne–Hanek | Infinite-precision reduction, correct for every finite double argument |
| published primary | The binary64 value Excel returns for `SIN`/`COS`/`TAN`, before any reciprocal |
| host-scoped | A result whose last bits depend on the CPU executing it |
| entire | Analytic everywhere in the complex plane: no poles, no branch cuts |

## Sources

- Microsoft, "COS function" —
  <https://support.microsoft.com/en-us/office/cos-function-0fb808a5-95d6-4553-8148-22aebdce5f05>
  (fetched at curation: signature, the radians requirement, the degrees-conversion note; no
  error conditions and no magnitude constraint are documented there).
- Handbook evidence record `EV-MATH-0004` — the six-surface trigonometric identification, the
  `FPREM1`-against-ROM-`π` substrate, the reciprocal construction of `SEC`/`CSC`/`COT`, the
  ruled-out Cody–Waite reduction, and the host-CPU microcode caveat. Read its reader warning.
- Abramowitz & Stegun, chapter 4.3 — circular functions, series, and approximations.
- Cody & Waite, *Software Manual for the Elementary Functions* — classical reduction.
- Payne & Hanek (1983) — infinite-precision radian reduction.
- Muller, *Elementary Functions: Algorithms and Implementation* — reduction error analysis and
  worst cases; fdlibm `__kernel_cos` as the standard polynomial reference.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.COS.json` (the `arg_domain_guard=circular_trig_overflow`
  and `non_finite=num` axis values) and `data/presence/FUNC.COS.json`.
