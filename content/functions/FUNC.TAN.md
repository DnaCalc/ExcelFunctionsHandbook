---
schema: efh.function-page/v1
function_id: FUNC.TAN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0004
open_problems: []
references:
  - work: "Microsoft Support — TAN function"
    locator: "https://support.microsoft.com/en-us/office/tan-function-08851a40-179f-4052-b789-d7f699447401"
    role: "documented signature, the radians convention, and the degrees-to-radians remark"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 4 (Elementary Transcendental Functions), section 4.3"
    locator: "4.3.x, tangent series and identities"
    role: "defining relations, period, poles and the tangent series"
  - work: "Cody & Waite, Software Manual for the Elementary Functions"
    locator: "chapter on TAN/COT"
    role: "the classical reduction and the rational-kernel formulation of tangent"
  - work: "Payne & Hanek, Radian reduction for trigonometric functions (SIGNUM Newsletter, 1983)"
    locator: null
    role: "exact reduction of huge arguments"
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
family: tan
role_in_family: >-
  The circular tangent on the radian convention; the trig primary whose poles make it the
  family's ill-conditioned member.
---

## What it computes

`TAN(number)` is the circular tangent of an angle measured in **radians**:

    tan x  =  sin x / cos x

- **Domain**: all reals except the odd multiples of `pi/2`, where `cos x = 0`.
- **Range**: all of the extended reals — tangent is surjective onto `(-infinity, +infinity)` on
  each of its branches.
- **Poles**: simple poles at `x = (2k+1) * pi/2` for every integer `k`. The function changes
  sign across each pole: `tan(x) -> +infinity` from the left of `pi/2` and `-infinity` from the
  right. These are genuine poles of the real function, not branch cuts; tangent is meromorphic.
- **Period**: `pi`, not `2*pi`. This is the identity most often mis-remembered, and it is why a
  correct implementation reduces modulo `pi/2` and then selects between a tangent and a
  cotangent kernel.
- **Parity**: odd.
- **Series about zero**: `tan x = x + x^3/3 + 2 x^5/15 + 17 x^7/315 + ...`, with radius of
  convergence `pi/2` — the distance to the nearest pole. The coefficients involve the Bernoulli
  numbers; A&S chapter 4 gives the closed form.
- **Zeros**: the integer multiples of `pi`, same as sine.

The identity that matters for implementation is the half-angle relation
`tan(x + pi/2) = -1/tan(x)` — the reason a tangent kernel and its reciprocal are the same
kernel with a branch selector.

Microsoft's page states one remark and it is the calling convention: the argument is in
radians; convert degrees by multiplying by `PI()/180` or by using `RADIANS`.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The angle, in radians. Required. | — |

One argument; the reference engine records an arity of exactly one, an ordinary numeric slot
under the shared coercion rules.

The misunderstood position, again, is the units. `TAN(45)` is not one.

## Result and edge cases

Returns `Number`.

The reference engine classifies `TAN` as a unary numeric scalar-or-array kernel
(`UnaryNumericScalarOrArrayElementwise`), so an array argument lifts elementwise.

The pole is the edge that has no clean answer. **No binary64 number is an odd multiple of
`pi/2`**, so no representable argument sits exactly on a pole: `TAN` always has a finite answer
to give. What it gives near a pole is enormous and almost entirely determined by how accurately
the implementation knows the distance from the argument to the true pole — see *Numerical
notes*. A reader who expects `TAN(PI()/2)` to be an error or an infinity should expect neither;
`PI()/2` is a double that differs from the real `pi/2`, and the tangent of *that* double is a
large finite number.

As with `SIN`, the reference engine's projected `real_result_policy` for `TAN` carries
`arg_domain_guard=circular_trig_overflow` and `non_finite=num`. **Microsoft's `TAN` page
documents no error condition at all.** The Handbook records the mismatch as a
documentation-versus-reference-engine divergence: the documented function is total, the
classified one is guarded, and nobody in this record has checked live Excel against either.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | The argument does not convert to a number | Shared coercion rules, not the `TAN` page |
| propagated | An error value in the argument is returned unchanged | Shared coercion rules |
| `#NUM!` | Argument outside the reference engine's declared circular-trig guard, or a non-finite result | Reference-engine classification only — **not documented by Microsoft** |

Microsoft's page lists no errors for `TAN`.

## Relationships

- **`SIN` and `COS`** — `TAN = SIN/COS` mathematically, though a good implementation does not
  compute it that way (see below).
- **`COT`** — the reciprocal. The reference engine derives it, per the attached evidence
  record, as `excel_x87_recip` of a published primary rather than as an independent kernel.
- **`ATAN` and `ATAN2`** — the inverses. `ATAN` returns the principal branch `(-pi/2, pi/2)`;
  `ATAN2` recovers the quadrant from a coordinate pair and is the function to use when the
  angle of a vector is wanted.
- **`TANH`** — the hyperbolic sibling, related by `tan(ix) = i tanh(x)`. The contrast is
  instructive: `TANH` is bounded and has no real poles, so it is numerically placid where `TAN`
  is not.
- **`RADIANS` / `DEGREES` / `PI`** — the conversion apparatus.

## Numerical notes

`TAN` inherits every argument-reduction difficulty of `SIN` (see that page for the three-regime
scheme, Cody-Waite and Payne-Hanek) and adds one of its own: **conditioning near the poles.**

**The conditioning statement.** The relative condition number of tangent is
`x / (sin x cos x)`, which blows up as `x` approaches a pole. Concretely: if the reduced
argument `r` — the distance from `x` to the nearest odd multiple of `pi/2` — carries a relative
error `eps`, then `tan x` carries a relative error of roughly `eps` as well, but `r` itself is
the result of a catastrophic cancellation. For an argument near a pole, almost all the
significant digits of `r` came from the bits of `pi` used in the reduction. An implementation
whose stored `pi` is short delivers a `TAN` near the poles whose leading digits are noise.

This is the practical reason the trig family's substrate matters more for `TAN` than for `SIN`.
Sine is bounded and locally linear at its zeros, so a reduction error shows up as a small
absolute error. Tangent multiplies the same reduction error by an unbounded factor.

**Do not compute `TAN` as `SIN/COS`.** Two separately rounded transcendental evaluations,
divided, give you both errors plus the division's; worse, near a pole the `COS` in the
denominator is itself the badly conditioned quantity, so the composition compounds exactly the
error you most wanted to control. The standard formulations (Cody & Waite; `fdlibm`'s
`__kernel_tan`) evaluate a single rational approximation on the reduced argument and select
between `t` and `-1/t` from the parity of the reduction quotient, so the reduction is performed
once and the reciprocal branch reuses it.

**Substrate, as recorded.** `EV-MATH-0004` names, for the trig six in the reference engine, the
legacy CRT `FSIN`/`FPTAN` chain with `FPREM1` argument reduction against the x87 ROM `pi`, with
a host-CPU microcode caveat, and rules out Cody-Waite reduction against an extended `pi`. The
structural consequence for `TAN` is the one stated above: a fixed-length `pi` in the reduction
means the near-pole values are where any two implementations will diverge first and worst.
**The record's counts and their scope are rendered by the evidence layer beside this page; this
prose states none of them.**

**Testing implication.** A vector suite for `TAN` that samples uniformly over a wide interval
is testing the easy part. The informative vectors are: arguments within a few ULP of
`k*pi + pi/2` for a spread of `k`; arguments within a few ULP of `k*pi`; and the large-argument
witnesses from the reduction literature.

## What has not been checked

The evidence attached to this page is `EV-MATH-0004`, which lists `FUNC.TAN` among its
subjects. It is a substrate-identification record and it carries its own reader warning about
how its figures may be read. This page restates none of them.

No Handbook vector suite exists for `TAN`. The Handbook has not observed `TAN` in Excel itself,
and nothing here is a statement that any implementation agrees with Excel.

Probes worth running first:

1. **The neighbourhood of `PI()/2`.** `TAN(PI()/2)` and its neighbouring doubles on both sides.
   Two implementations that agree everywhere else will disagree here, and the sign flip across
   the pole makes the disagreement unmissable.
2. **The large-argument guard.** Walk the argument up by powers of two and find where Excel
   stops returning a number — the same probe as for `SIN`, and the one that decides whether the
   reference engine's `circular_trig_overflow` classification describes Excel.
3. **`TAN` against `SIN/COS`** at the same inputs. Where they differ, the implementation is not
   naive; where they agree exactly across the range, it probably is.
4. **`TAN(x)` against `1/COT(x)`** and `ATAN(TAN(x))` for `x` in the principal branch — cheap
   metamorphic checks that need no external oracle.
5. **Near-zero arguments**, to confirm the region where `tan x = x` to within rounding.
6. **Array arguments**, to confirm elementwise lifting.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| pole | An odd multiple of `pi/2`, where tangent is unbounded; no binary64 number lies on one |
| reduced argument | The distance from the input to the nearest multiple of `pi/2` |
| conditioning | How much a relative error in the reduced argument is amplified in the result |
| circular-trig guard | The reference engine's declared `arg_domain_guard=circular_trig_overflow` |

## Sources

- Microsoft, "TAN function" —
  <https://support.microsoft.com/en-us/office/tan-function-08851a40-179f-4052-b789-d7f699447401>
  (signature, the radian convention, the degrees-conversion remark; no error conditions are
  listed there).
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 4 section 4.3 — tangent's
  series, period, poles and identities.
- Cody & Waite, *Software Manual for the Elementary Functions* — the rational-kernel
  formulation and the reduction that selects between tangent and cotangent branches.
- `fdlibm` `__kernel_tan` and `__rem_pio2`; Payne & Hanek (1983) — the reference treatment of
  reduction and the reciprocal branch.
- Handbook evidence record `EV-MATH-0004` (subjects include `FUNC.TAN`) — substrate
  identification for the trig six, with its own reader warning.
- Handbook projections `data/functions/FUNC.TAN.json` (arity, classification,
  `real_result_policy` with `arg_domain_guard=circular_trig_overflow` and `non_finite=num`) and
  `data/presence/FUNC.TAN.json`.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md).
