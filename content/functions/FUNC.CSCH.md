---
schema: efh.function-page/v1
function_id: FUNC.CSCH
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — CSCH function"
    locator: "https://support.microsoft.com/en-us/office/csch-function-f58f2c22-eb75-4dd6-84f4-a503527f8eeb"
    role: "documented signature, the magnitude constraint, and the #NUM! and #VALUE! conditions"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 4.5 (Hyperbolic Functions), 4.5.66 for the csch expansion"
    role: "definition, the Laurent expansion at the origin, and the identity set"
  - work: "fdlibm (Sun Microsystems freely distributable libm)"
    locator: "s_sinh.c and s_expm1.c"
    role: "the expm1-based sinh whose reciprocal is the stable route to csch"
  - work: "Cephes mathematical library"
    locator: "sinh.c"
    role: "an independent branch structure for the same problem"
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
family: csch
role_in_family: >-
  The hyperbolic cosecant — the reciprocal of sinh, unbounded at the origin and vanishing
  exponentially at infinity, and the member of this batch with no evidence record naming it at
  all.
---

# CSCH

## What it computes

`CSCH(number)` returns the hyperbolic cosecant.

    csch x = 1 / sinh x = 2 / (e^x - e^{-x})

Microsoft's page describes it as the hyperbolic cosecant of an angle specified in radians. That
phrasing is worth a note: the argument of a hyperbolic function is not an angle in the circular
sense and there are no radians involved — it is a hyperbolic angle, twice the area of the
corresponding hyperbolic sector. The wording appears to be inherited from the circular
functions' documentation template. It changes nothing about the mathematics.

| Property | Statement |
|---|---|
| Domain | all real `x` except `x = 0` |
| Range | `(-∞, 0) ∪ (0, ∞)` — every nonzero real is attained exactly once |
| Parity | odd: `csch(-x) = -csch(x)` |
| Pole | one, simple, at `x = 0`, with residue 1 |
| Laurent series at 0 | `csch x = 1/x - x/6 + 7x³/360 - …` (A&S 4.5.66) |
| Asymptotics | `csch x ~ 2e^{-x}` as `x → +∞`: decays to zero, never reaching it |
| Fundamental identity | `coth²x - csch²x = 1` |
| Derivative | `d/dx csch x = -csch x · coth x` |
| Relation to circular | `csch x = -i·csc(ix)` |
| Monotone | strictly decreasing on `(-∞, 0)` and on `(0, ∞)` |

The shape is a single hyperbola-like curve on each side of the origin: unbounded at `0`,
decaying exponentially to `0` at `±∞`. Unlike its sibling [COTH](FUNC.COTH.md), which flattens
onto `±1`, `CSCH` flattens onto `0` — so its large-argument behaviour is **underflow**, not
overflow, and that is the whole difference in their numerical stories.

The practical thresholds: `csch x` falls below the smallest normal double once `x` is above
about 710, and below the smallest subnormal at about 745. The correct answer there is `0`, and
`0` is representable, so nothing goes wrong.

## Arguments

| Argument | Meaning | Constraint (as documented) |
|---|---|---|
| `number` | The hyperbolic angle. Required. | `ABS(number) < 2^27` |

**A recorded divergence, identical in shape to [COTH](FUNC.COTH.md)'s.** Microsoft's page
documents the `2^27` magnitude constraint and `#NUM!` beyond it. The reference engine's
real-result policy for this surface records `arg_domain_guard=none` — no argument guard — and
`non_finite=allow`. Those cannot both describe `ABS(x) ≥ 2^27`. The Handbook publishes the
divergence and has not resolved it.

As on `COTH`, the constraint has no numerical motivation: `csch` needs no argument reduction,
and its correct value at `2^27` is `0`. The bound reads like a documentation template applied
across the 2013 trigonometric batch. That reading is a hypothesis and is on the probe list.

The `non_finite=allow` value is, by contrast, entirely consistent with the mathematics here:
`csch` of a large argument underflows to zero, which is finite, so no non-finite result arises
from the value — only, potentially, from the pole.

## Result and edge cases

Returns `Number`, never zero mathematically but zero by underflow for large arguments.

- **`CSCH(0)`** — mathematically a pole. Microsoft's page states the magnitude constraint and
  the non-numeric condition and **says nothing about zero**. `#DIV/0!` follows by analogy with
  [COT](FUNC.COT.md), whose page does state it, but nothing establishes it here.
- **Large `ABS(x)`** — the true value underflows to `0`. Whether Excel returns `0` or something
  else is unchecked; `0` is the mathematically correct rounded answer.
- **Sign** — preserved: `CSCH` of a negative argument is negative, including in the underflow
  region, where the correct answer is negative zero if signed zeros are preserved and `0`
  otherwise. That distinction is invisible in most contexts and visible in `1/CSCH(x)`.
- **Never zero for moderate arguments** — `csch` has no zeros, so a `0` returned for an
  argument below the underflow threshold would be a defect.
- **Near zero** — `csch x ≈ 1/x`, so `CSCH` of a tiny argument is huge, overflowing only in the
  subnormal range.

## Errors

As documented on Microsoft's `CSCH` page:

| Error | Condition |
|---|---|
| `#NUM!` | `ABS(number)` is not less than `2^27` |
| `#VALUE!` | `number` is non-numeric |

The zero case is not documented. Nothing else is documented.

## Relationships

- **`SINH`** — the reciprocal, and the surface whose accuracy `CSCH` inherits. `SINH` has the
  classic cancellation problem near zero; see below.
- **`SECH`** — the other hyperbolic reciprocal that decays to zero, but bounded above by 1 and
  with no pole. `SECH` is the well-behaved one; `CSCH` is not.
- **[COTH](FUNC.COTH.md)** — the sibling from the same 2013 batch, sharing the pole at the
  origin, the documented `2^27` constraint, and the `arg_domain_guard=none` axis value. The two
  pages diverge from their documentation in the same way and should be read together.
- **[COSH](FUNC.COSH.md)** — the identity `coth²x - csch²x = 1` ties this surface to the
  hyperbolic set; `csch x = 1/sinh x` and `sinh` is built from `exp`.
- **`ASINH`** — there is no `ACSCH` on the worksheet surface; `ASINH(1/x)` is the substitute,
  and it is well conditioned, which makes it a good round-trip probe.
- **[CSC](FUNC.CSC.md)** — the circular partner, `csch x = -i·csc(ix)`. Same `1/x` leading
  term; opposite behaviour at infinity.
- **`IMCSCH`** — the complex version.

## Numerical notes

`CSCH` is `1/SINH`, and every hard question about it is really a question about `sinh`.

**The cancellation, and the remedy.** Written as `(e^x - e^{-x})/2`, `sinh x` cancels
catastrophically for small `x`: the two exponentials are both close to 1, and their difference
is close to `2x`, so the leading digits are lost. This is the textbook example of subtractive
cancellation (A&S chapter 4.5; every numerical-analysis course). The standard remedy is
`expm1`: with `t = expm1(x)`,

    sinh x = ( t + t/(t+1) ) / 2

which never forms the difference of two near-equal quantities. fdlibm's `s_sinh.c` and Cephes'
`sinh.c` both branch this way, using a polynomial in `x²` for the smallest arguments and the
`expm1` form up to moderate size. Excel has no `EXPM1` on the worksheet surface, so a user who
writes `(EXP(x)-EXP(-x))/2` by hand will get the cancellation no matter how good `SINH` and
`CSCH` are; the worksheet remedy is the series `x + x³/6 + x⁵/120`.

**For `CSCH` specifically, the reciprocal reverses which end is dangerous.**

- **Near zero**, `sinh` is small and its *relative* error is what propagates, so a stable
  `sinh` gives a stable `csch`. Better still, use the Laurent series `1/x - x/6 + 7x³/360`
  (A&S 4.5.66) directly below about `2^-13`: it avoids the division entirely and gives the
  deviation from `1/x` without cancellation.
- **At large `x`**, `sinh x` overflows around `x ≈ 710.4758` while `csch x` is merely tiny. A
  naive `1/SINH(x)` therefore returns `1/∞` — which is `0`, the right answer, if infinities are
  allowed to appear internally, and an error if they are not. This is the one place where the
  reference engine's `non_finite=allow` axis is doing visible work: the *value* is finite even
  though an intermediate need not be. An implementation that computes `2e^{-x}` directly in
  this regime never sees the overflow at all and gets the extra digits for free.
- **Underflow tail**: below the smallest subnormal the answer is `0`, gradually then abruptly.
  An implementation using `2·EXP(-x)` reaches that boundary smoothly; one using `1/SINH(x)`
  reaches it at whatever point `SINH` stopped being finite.

**The one-line summary an implementer needs**: never compute `csch` as the reciprocal of a
value that can overflow, and never compute `sinh` as a difference of exponentials.

## What has not been checked

**No evidence record in the Handbook names `CSCH`.** It appears in no discrepancy catalogue
entry, no math-deviation entry and no bug stream in the presence projection. Nobody has checked
this function against Excel within the Handbook's record — not its values, not its pole, not
its documented magnitude bound.

No Handbook vector suite exists for `CSCH`. Everything on this page above the numerical notes
is either Microsoft's documentation (the signature, the `2^27` constraint, the two error
conditions) or mathematics; the divergence between the documented constraint and the recorded
`arg_domain_guard=none` axis is stated as a divergence, not resolved.

Inputs I would probe first:

1. **`CSCH(2^27)` and `CSCH(2^27 - 1)`.** The documentation says `#NUM!`; the recorded axis says
   no guard, which would give `0`. One experiment, two different kinds, decisive. This is the
   same probe as on [COTH](FUNC.COTH.md) and the two should be run together — if they answer
   differently, the guard is per-surface rather than per-batch.
2. **`CSCH(0)` and `CSCH(-0)`.** The undocumented pole.
3. **The underflow ladder**: `CSCH(700)`, `CSCH(710)`, `CSCH(711)`, `CSCH(745)`, `CSCH(746)`.
   The mathematical answer is a smooth decay into subnormals and then `0`. An implementation
   built on `1/SINH` loses the tail at about 710, where `SINH` stops being representable; one
   built on `2·EXP(-x)` keeps it to about 745. The first argument returning exactly `0` locates
   the algorithm, and needs no oracle — the mathematics fixes what is correct.
4. **The oddness identity** `CSCH(-x) = -CSCH(x)` bitwise, including at the underflow boundary
   where a signed zero may or may not survive.
5. **Small arguments**: `CSCH(2^-20)` down to `CSCH(2^-1070)`, and `CSCH(x) - 1/x` against the
   series `-x/6 + 7x³/360`, which is the cancellation probe near the pole.
6. **`CSCH(x) · SINH(x) - 1`** across the range, which tests the reciprocal relation, and
   `ASINH(1/CSCH(x)) - x`, which is a well-conditioned round trip.
7. **`CSCH(x)` against `1/SINH(x)` and against `2/(EXP(x)-EXP(-x))`** typed in the worksheet.
   Where the three separate is where the implementation's branch structure becomes visible.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| hyperbolic angle | The argument of a hyperbolic function: twice the hyperbolic sector area |
| subtractive cancellation | Loss of leading digits when two near-equal values are subtracted |
| `expm1` | `e^x - 1` computed without cancellation; the stable route to `sinh` near zero |
| underflow tail | The region where the true value is smaller than the smallest normal double |
| Laurent series | The expansion `1/x - x/6 + …` valid near the pole at the origin |
| domain-guard axis | The recorded `arg_domain_guard` value on the surface's real-result policy |

## Sources

- Microsoft, "CSCH function" —
  <https://support.microsoft.com/en-us/office/csch-function-f58f2c22-eb75-4dd6-84f4-a503527f8eeb>
  (fetched at curation: signature, the `2^27` magnitude constraint, and the `#NUM!` and
  `#VALUE!` conditions. No zero-argument behaviour and no formula are documented there).
- Abramowitz & Stegun, chapter 4.5, in particular 4.5.66 — the `csch` expansion and the
  hyperbolic identity set.
- fdlibm `s_sinh.c`, `s_expm1.c`, and Cephes `sinh.c` — the cancellation-free routes.
- Handbook, [COTH](FUNC.COTH.md) — the sibling carrying the same documentation-versus-axis
  divergence; [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.CSCH.json` (the `arg_domain_guard=none` and
  `non_finite=allow` axis values) and `data/presence/FUNC.CSCH.json` (which records no
  discrepancy-catalogue, math-deviation or bug-stream mention for this surface).
