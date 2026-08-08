---
schema: efh.function-page/v1
function_id: FUNC.SEC
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0004
open_problems: []
references:
  - work: "Microsoft — WorksheetFunction.Sec method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.sec"
    role: "the complete documented content: one-line description, one argument, no remarks, no domain limit and no error conditions"
  - work: "Microsoft Support — SEC function"
    locator: "https://support.microsoft.com/en-us/office/sec-function-ff224717-9c87-4170-9b58-d069ced6d5f7"
    role: "the worksheet-surface documentation page named in the projection; not retrievable for this pass"
  - work: "OxFunc — EXCEL_MATH_DEVIATION_CATALOG.md"
    locator: "docs/EXCEL_MATH_DEVIATION_CATALOG.md entry XMD-005"
    role: "the observed, undocumented circular-trigonometric argument limit at |x| >= 2^27"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 4, sections 4.3.1-4.3.70 (circular functions); 4.3.68 for the Euler-number series"
    role: "the defining identities, the series with Euler numbers, and the pole structure"
  - work: "Payne & Hanek, Radian reduction for trigonometric functions"
    locator: "SIGNUM Newsletter 18 (1983)"
    role: "the standard exact argument reduction that a 2^27 cutoff exists to avoid"
  - work: "Muller, Elementary Functions: Algorithms and Implementation"
    locator: "chapters on argument reduction and on the condition number of trigonometric evaluation"
    role: "the error amplification near a pole and the cost of correct reduction"
episodes: []
body_sections:
  - "What it computes"
  - "Arguments"
  - "Result and edge cases"
  - "Errors"
  - "Relationships"
  - "Numerical notes"
  - "What has not been checked"
  - "Page vocabulary"
  - "Sources"
family: sec
role_in_family: "The reciprocal of COS; one of the three reciprocal circular surfaces built on the two published primaries."
---

# SEC

## What it computes

`SEC(number)` returns the secant of an angle given in radians.

    sec x = 1 / cos x

**Domain.** All real `x` except the zeros of cosine, `x = π/2 + kπ` for integer `k`, where secant
has simple poles. **Range.** `|sec x| ≥ 1`: the values `(−1, 1)` are never attained. The function
is even, `2π`-periodic, and satisfies the Pythagorean identity

    sec²x − tan²x = 1

Its derivative is `sec x · tan x`, and its Maclaurin series is the Euler-number generating function

    sec x = Σ_{n≥0} |E_{2n}| x^{2n} / (2n)!  =  1 + x²/2 + 5x⁴/24 + 61x⁶/720 + …

convergent for `|x| < π/2` — the radius is exactly the distance to the nearest pole, which is the
compact statement of why this function is hard near `π/2`. (A&S 4.3.68.)

**A fact worth stating before the numerics.** No binary64 value is exactly `π/2 + kπ`, because
those points are irrational. So `SEC` never actually divides by zero on a machine: it divides by a
very small number and returns a very large one. The pole is unreachable, and everything difficult
about the function happens in its neighbourhood rather than at it.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The angle, **in radians**, whose secant is wanted. Required. | — |

One numeric slot under ordinary to-number coercion; the projection classifies the surface
`UnaryNumericScalarOrArrayElementwise`, so an array argument lifts elementwise with element-local
failures — see [Coercion and lifting](../model/02-coercion-and-lifting.md).

Degrees must be converted first: `SEC(RADIANS(d))`. See [RADIANS](FUNC.RADIANS.md) for what that
conversion costs, which matters more here than for most consumers because of the amplification
described below.

Microsoft's VBA page for this method is three lines long: a one-sentence description, the single
argument, the return type. **No remarks, no domain restriction, no error condition.**

## Result and edge cases

Returns `Number`.

- **`SEC(0)` is exactly 1**, and it is one of the few inputs where an exact answer is available.
- **Near a pole the result is enormous and its accuracy is not.** As `x` approaches `π/2` the
  returned value grows without bound; the *relative* accuracy of that value degrades in proportion
  to how well the argument's last bits locate `x` relative to the pole. See the numerical notes.
- **Large arguments are refused.** The projection declares
  `real_result_policy … arg_domain_guard=circular_trig_overflow; non_finite=num` for this surface.
  That axis is the projection of OxFunc math-deviation entry **XMD-005**, which records live Excel
  returning `#NUM!` for the whole circular family — `SIN`, `COS`, `TAN`, `COT`, `SEC`, `CSC` — once
  the argument magnitude reaches `2²⁷ = 134217728`, with the boundary bisected: the value just
  below is finite and the value at the boundary errors, on both signs. The catalogue's evidence
  line names live Excel 16.0 build 20026.

  **This is a documentation-versus-behaviour divergence, and the Handbook records it as a finding.**
  Microsoft's reference for `Sec` states no domain limit at all. The limit is not a mathematical
  property — secant is defined and finite for every representable argument that is not near a pole
  — and it is not an overflow: it is a hard refusal of large arguments, which the catalogue
  attributes to Excel's argument-reduction routine capping at `|x| < 2²⁷`. A function that is
  mathematically total on the reals is, on this surface, undefined above a threshold no
  documentation mentions.
- **The reciprocal is not a plain division.** The reference engine computes `SEC` as a
  *double-rounded* reciprocal of its own `COS`: the quotient is formed in extended precision and
  then rounded to binary64, which is not the same as the single-rounded `1/COS(x)` a worksheet
  formula would compute. So `SEC(x)` and `1/COS(x)` are not guaranteed to be the same value, and
  neither is guaranteed to be the correctly rounded secant.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | `\|number\| ≥ 2²⁷` | observed live Excel, catalogue `XMD-005`; **not documented** |
| `#VALUE!` | The argument does not convert to a number | shared coercion model |
| propagated | The argument is an error value | shared coercion model |

Microsoft's documentation states none of these.

## Relationships

- **`COS`** — the primary. `SEC` is its reciprocal by definition, and in the reference engine
  literally so: the same cosine kernel feeds both. That is exactly why `EV-MATH-0004` is careful to
  record that the reciprocal surfaces were nevertheless scored *individually* rather than having a
  primary's result inherited.
- **`CSC` and `COT`** — the other two reciprocals, of `SIN` and `TAN`. All three share the
  `2²⁷` guard, all three are built on two kernels, and all three carry the same host-CPU caveat.
- **`TAN`** — related by `sec²x = 1 + tan²x`, which is the numerically preferable route to `sec`
  when `tan` is already available and `x` is near a pole: computing `sqrt(1 + tan²x)` avoids the
  division by a tiny cosine, at the cost of losing the sign, which must be restored from the
  quadrant.
- **[SECH](FUNC.SECH.md)** — the hyperbolic namesake, and a completely different function: bounded,
  pole-free, and with no argument limit. The two are related only through the complex identity
  `sech(x) = sec(ix)`.
- **`ACOS`** — there is no `ASEC`. The inverse secant must be written `ACOS(1/x)`, which is defined
  for `|x| ≥ 1` and inherits a division by the argument.
- **[RADIANS](FUNC.RADIANS.md)** — the usual upstream converter, and, given the amplification
  below, a genuine contributor to the error of the result.

## Numerical notes

**The condition number is the whole story.** Differentiating, the relative error in `sec x`
produced by a relative error `ε` in `x` is

    |x · d(sec x)/dx / sec x| · ε  =  |x · tan x| · ε

so the amplification factor is `|x tan x|`, which is unbounded as `x` approaches a pole. Near
`π/2`, a last-bit perturbation of the argument moves the result by an arbitrarily large relative
amount. This is not an implementation weakness; it is a property of the function, and it means that
**no implementation can return an accurate secant near a pole from an inexact argument**. The
honest response is to say so, which most libraries do not.

**Argument reduction is where the accuracy is won or lost.** Evaluating `cos x` requires `x mod
2π`, and the modulus must be taken against the real π rather than against `fl(π)` — every bit by
which `|x|` exceeds π costs a bit of the reduced argument if the reduction uses a rounded constant.
The standard remedy is Payne–Hanek reduction against a many-word 2/π, as implemented in fdlibm's
`__ieee754_rem_pio2`; it is exact for every finite double and costs a bounded amount of work.

Excel's recorded behaviour is the alternative policy: **refuse**. A cutoff at `2²⁷` is
approximately where a reduction using a double-precision π begins to lose the low half of the
reduced argument, so the threshold reads as the point at which the implementer stopped trusting
their own reduction. The Handbook's position is that both are defensible engineering, that only one
of them is documented (neither), and that a `natural-best` implementation should reduce correctly
and return a value where Excel returns an error — which is a deliberate, declared divergence, not a
compatibility bug. See [About implementation options](../model/07-implementation-options.md).

**Reciprocal double rounding.** Computing `1/cos x` by forming the quotient in a wider format and
rounding twice — to 64 significant bits, then to 53 — can differ from the single-rounded quotient
by one unit in the last place on a small fraction of inputs. That is the classic double-rounding
error, it is the shape of computation an x87 backend performs naturally, and the reference engine
reproduces it deliberately for this surface. Any implementation aiming at the same values has to
reproduce the *staging*, not merely the arithmetic.

**Host-CPU dependence.** `EV-MATH-0004`'s reader warning records that all six circular surfaces
inherit a host-CPU microcode caveat on the x87 `FPREM1`, `FSIN` and `FPTAN` instructions. In plain
terms: the value may depend on which processor evaluates it. That is a `portable-reproducible`
concern, and it is the reason a suite for this function would have to name a CPU as well as an
Excel build.

## What has not been checked

No Handbook vector suite exists for `SEC`. One evidence record, `EV-MATH-0004`, lists this surface
among its subjects — a substrate-identification record covering the six circular functions. Two
things in it deserve to be carried onto this page rather than left in the panel:

- Its status text is explicit that all six surfaces were **scored individually**, so `SEC` is a
  measured surface rather than one inheriting `COS`'s warrant — even though only two kernels exist
  underneath, and a portion of each surface's rows was held out.
- Its reader warning is equally explicit that the catalogue elsewhere restates only a *group*
  figure across all six, that reading that group figure as any one surface's rate is wrong, and
  that all six inherit the host-CPU microcode caveat.

What that record does not establish is anything about the `2²⁷` refusal boundary, which comes from
a separate catalogue entry with its own live-Excel evidence line, and nothing on this page has been
re-observed by the Handbook.

Inputs I would probe first:

1. **`SEC(134217727)` and `SEC(134217728)`, and the same pair negated** — the four cells that pin
   the undocumented refusal boundary. This is the highest-value probe on the page, because the
   behaviour is undocumented, mathematically unnecessary and shared across six functions.
2. **`SEC(2)` against `1/COS(2)`** — whether the surface and the hand-written reciprocal agree.
   If they differ, the double-rounded staging is real and observable from the worksheet.
3. **`SEC(x)` for `x` at the two doubles bracketing `π/2`**, and at the corresponding points near
   `3π/2` and `−π/2`. The results should be very large with opposite signs; how large, and whether
   the magnitudes are consistent with `1/(π/2 − x)`, characterises the reduction.
4. **`SEC(1E15)` and `SEC(1E7)`** — one side of the boundary each, testing whether accuracy
   degrades smoothly below the cutoff or falls off a cliff, which distinguishes an exact reduction
   from a rounded-π one.
5. **`SEC(RADIANS(90))` and `SEC(RADIANS(89.9999999))`** — the practical route by which users reach
   the pole, and where the conversion error of [RADIANS](FUNC.RADIANS.md) is amplified.
6. **`SEC` of an array spanning a pole**, given the elementwise-lift declaration and the
   element-local failure rule.
7. **The same probes on a different CPU vendor**, which is the only way to test the microcode
   caveat the evidence record carries.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| pole | A point where `cos x = 0` and secant is unbounded; never exactly representable |
| condition number | The amplification factor `\|x tan x\|` from argument error to result error |
| argument reduction | Reducing `x` modulo `2π` before evaluating a kernel |
| refusal boundary | The undocumented `\|x\| ≥ 2²⁷` threshold above which the circular family errors |
| double rounding | Rounding a reciprocal first to extended precision and then to binary64 |

## Sources

- Microsoft, "WorksheetFunction.Sec method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.sec> (the entire
  documented content: description, one argument, return type; **no remarks, no domain limit, no
  errors**).
- Microsoft Support, "SEC function" —
  <https://support.microsoft.com/en-us/office/sec-function-ff224717-9c87-4170-9b58-d069ced6d5f7>
  (retrieval refused with HTTP 403 during this pass; nothing here is sourced from it).
- OxFunc `docs/EXCEL_MATH_DEVIATION_CATALOG.md` entry XMD-005 — the bisected `2²⁷` boundary across
  the six circular surfaces, with its own live-Excel evidence line.
- Handbook evidence record `EV-MATH-0004`, rendered beside this page with its own reader warning.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, section 4.3 — circular functions,
  the Euler-number series and the pole structure.
- Payne & Hanek, SIGNUM Newsletter 18 (1983); fdlibm `__ieee754_rem_pio2`; Muller, *Elementary
  Functions: Algorithms and Implementation*.
- Handbook, [RADIANS](FUNC.RADIANS.md), [SECH](FUNC.SECH.md),
  [Coercion and lifting](../model/02-coercion-and-lifting.md),
  [About implementation options](../model/07-implementation-options.md).
- Handbook projections `data/functions/FUNC.SEC.json`
  (`arg_domain_guard=circular_trig_overflow`, `non_finite=num`) and
  `data/presence/FUNC.SEC.json` (own module; one math-deviation mention; no defect streams).
