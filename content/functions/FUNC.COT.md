---
schema: efh.function-page/v1
function_id: FUNC.COT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0004
open_problems: []
references:
  - work: "Microsoft Support — COT function"
    locator: "https://support.microsoft.com/en-us/office/cot-function-c446f34d-6fe4-40dc-84f8-cf59e5f5e31a"
    role: "documented signature, the |number| < 2^27 constraint, the #NUM! and #VALUE! conditions, and COT(0) = #DIV/0!"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 4.3 (Circular Functions), 4.3.70 for the cotangent Laurent expansion"
    role: "definition, identities, the pole structure and the series near zero"
  - work: "Payne & Hanek, Radian reduction for trigonometric functions (SIGNUM Newsletter, 1983)"
    locator: null
    role: "the reduction that survives large arguments"
  - work: "Muller, Elementary Functions: Algorithms and Implementation"
    locator: "chapters on argument reduction and on the tangent"
    role: "why the tangent family is the hardest of the circular set to round correctly"
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
family: cot
role_in_family: >-
  The circular cotangent — a 2013-generation reciprocal surface, documented with an explicit
  magnitude constraint and the only member of its trio whose zero-pole error is documented.
---

# COT

## What it computes

`COT(number)` returns the cotangent of an angle in radians. Microsoft's page describes it as
the cotangent of an angle specified in radians.

    cot x = cos x / sin x = 1 / tan x = tan(π/2 - x)

| Property | Statement |
|---|---|
| Domain | all real `x` except `x = kπ`, `k ∈ ℤ` |
| Range | all of `ℝ`; every real value is attained once per period |
| Parity | odd: `cot(-x) = -cot(x)` |
| Period | `π` — half the period of `sin` and `cos` |
| Poles | simple, at `x = kπ`, each with residue 1 |
| Zeros | `x = π/2 + kπ` |
| Laurent series at 0 | `cot x = 1/x - x/3 - x³/45 - 2x⁵/945 - …` (A&S 4.3.70) |
| Derivative | `d/dx cot x = -csc²x = -(1 + cot²x)` |
| Monotone | strictly decreasing on each interval `(kπ, (k+1)π)` |

The pole structure is where the reading and the implementation part company. Mathematically
there is a pole at every integer multiple of `π`. In binary64 **only one of those poles is
representable**: `x = 0`. Every other `kπ` is irrational, so no double sits on it, and near
those poles `COT` returns a large finite number rather than an error. Microsoft documents
`COT(0)` as `#DIV/0!` and documents nothing about the others, which is consistent: `0` is the
only argument at which the division is exactly by zero.

The consequence for a reader is worth stating plainly. `COT(PI())` is not an error and is not
infinite; it is roughly the reciprocal of the distance between the double `PI()` and the true
`π`, which is a large number whose value says more about the rounding of `PI()` than about the
cotangent.

## Arguments

| Argument | Meaning | Constraint (as documented) |
|---|---|---|
| `number` | The angle in radians. Required. | `ABS(number) < 2^27` |

Microsoft states the magnitude constraint on this page and states `#NUM!` when it is exceeded.
The bound is `2^27 ≈ 1.342 × 10^8`, and it is not arbitrary: it is approximately where a
reduction against a limited-precision `π` stops producing a reduced argument with any correct
bits. Documenting the bound rather than returning noise is, from a numerical-analysis point of
view, the right call, and it is a choice the older surfaces ([COS](FUNC.COS.md), `SIN`, `TAN`)
do not make on their own documentation pages.

The reference engine carries `arg_domain_guard=circular_trig_overflow` for this surface, which
is consistent with the documented bound. The slot takes ordinary to-number coercion and the
surface is declared an elementwise scalar kernel; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`.

- **`COT(0)`** — `#DIV/0!`, documented explicitly.
- **Near other poles** — a large finite value, not an error. The magnitude is bounded by
  roughly `1/ulp(kπ)`, so it grows with `k`.
- **Sign near a pole** — `cot` changes sign across every pole, so two adjacent doubles either
  side of `kπ` give large values of opposite sign. Any formula that uses `COT` near a multiple
  of `π` is sitting on a discontinuity of magnitude `~2/ulp`.
- **Zeros** — at `π/2 + kπ`, again not representable, so `COT` of a double is essentially never
  exactly zero.
- **Beyond `2^27`** — documented `#NUM!`.
- **Small arguments** — `cot x → 1/x`, so `COT` of a tiny argument is a huge finite number and
  overflows only for `|x|` below about `5.6 × 10^-309`, i.e. in the subnormal range.

## Errors

As documented on Microsoft's `COT` page:

| Error | Condition |
|---|---|
| `#DIV/0!` | `number` is 0 |
| `#NUM!` | `ABS(number)` is not less than `2^27` |
| `#VALUE!` | `number` is non-numeric |

`COT` is the only member of its trio whose page states the zero case. [CSC](FUNC.CSC.md)'s page
defines `CSC(n)` as `1/SIN(n)` — which makes `CSC(0)` a division by zero — but does not state
the resulting error; `SEC` has no zero-pole at all. That asymmetry is in Microsoft's
documentation, not in the mathematics.

## Relationships

- **`TAN`** — the reciprocal, and the surface `COT` is built from. `EV-MATH-0004` records that
  the three reciprocal surfaces are formed as `excel_x87_recip` of the *published* primaries;
  for `COT` the primary is the tangent chain (`fFPTAN`). That construction is the single most
  useful implementation fact on this page.
- **[COS](FUNC.COS.md), `SIN`** — the ratio definition. Note that `COS(x)/SIN(x)` and
  `1/TAN(x)` are **different computations** with different error, even though they are the same
  function: the first divides two rounded values, the second inverts one.
- **`ACOT`** — the inverse, principal range `(0, π)`. Note that `ACOT` in Excel returns a value
  in `(0, π)` rather than `(-π/2, π/2)`, so `ACOT(COT(x))` folds into `(0, π)`.
- **[COTH](FUNC.COTH.md)** — the hyperbolic partner; `coth x = i·cot(ix)`. Same pole at the
  origin, same Laurent leading term `1/x`, and the same documented `2^27` constraint on its
  page — although, as that page records, the reference engine's guard axis disagrees there.
- **[CSC](FUNC.CSC.md), `SEC`** — the other two reciprocals introduced alongside `COT`, sharing
  its documentation template and its constraint.
- **`IMCOT`** — the complex-argument version.

## Numerical notes

Cotangent inherits every difficulty of the tangent and adds one of its own.

**The reduction problem, inherited.** As on [COS](FUNC.COS.md), evaluating a circular function
at a large argument requires reducing modulo `π/2` against a sufficiently precise `π`. `COT`'s
documented `2^27` bound is a declaration of exactly where the implementation's reduction gives
up. `EV-MATH-0004` identifies the substrate as the legacy CRT `fFSIN`/`fFPTAN` chain with
`FPREM1` reduction against the 64-bit ROM `π`, with the reciprocal surfaces formed from the
published primaries, and it rules out Cody–Waite reduction against extended `π`. All six
circular surfaces in that record inherit a host-CPU microcode caveat, so the last bits here are
platform-scoped by construction. The record's figures render in the evidence layer beside this
page.

**The conditioning problem, its own.** The relative condition number of `cot` is
`|x / (sin x cos x)| · |cos x / sin x|^{-1}`… stated more usefully: near a pole `kπ`,
`cot x ≈ 1/(x - kπ)`, so a relative error `ε` in the *reduced* argument becomes a relative
error of about `ε · |x| / |x - kπ|` in the result. Close to a pole this amplification is
unbounded. This is not a fixable defect; it is the function. It does mean that "how accurate is
`COT`?" has no single answer — accuracy has to be quoted as a function of distance to the
nearest pole.

**The double-rounding problem, structural.** Building `COT` as `1/TAN(x)` where `TAN(x)` is the
*published* binary64 tangent means the tangent's own rounding error is inverted along with the
value. The result is a correctly-rounded reciprocal of an incorrectly-rounded tangent — which
is a different function from a correctly-rounded cotangent, and differs from it by up to about
one ulp more than the tangent already did. A `natural-best` implementation would instead
evaluate `cos r / sin r` from a single reduction, sharing the reduced argument between
numerator and denominator so the pole locations agree exactly.

**Near zero.** For small `|x|`, `1/TAN(x)` is fine — `tan x → x` with no cancellation — but the
Laurent form `1/x - x/3 - x³/45` (A&S 4.3.70) is cheaper and more accurate, and it is the
standard choice below about `2^-13`.

Muller's treatment is the reference for why the tangent family is the hardest of the circular
set to round correctly: the worst cases require more extra precision than sine or cosine do,
because the function is unbounded and its hardest-to-round arguments cluster near the poles.

## What has not been checked

`EV-MATH-0004` names this surface, and names it as a **measured** surface rather than an
inherited one: the record states each of the six was scored individually, even though only
three kernels exist. It carries a reader warning — the figure the upstream catalogue restates
is a group total over all six functions and is not a per-surface rate — and a host-CPU
microcode caveat on the reducing and evaluating instructions.

No Handbook vector suite exists for `COT`. The signature, the `2^27` constraint, the `#NUM!`
and `#VALUE!` conditions and the `COT(0) = #DIV/0!` rule are documented by Microsoft. The
reciprocal construction is on record from OxFunc, not from Excel's documentation, and the
Handbook has not independently observed it.

Inputs I would probe first:

1. **The documented bound, exactly**: `COT(2^27 - 1)`, `COT(2^27)`, `COT(-2^27)`,
   `COT(2^27 + 1)`. Whether the comparison is strict, and whether it is on the magnitude before
   or after coercion, is a two-probe question whose answer is an error code.
2. **`COT(0)` and the signed zero**: `COT(0)` and `COT(-0)`. The documented answer is `#DIV/0!`;
   whether a negative zero takes the same path is not documented anywhere.
3. **`COT(x) · TAN(x) - 1`** across the range. If the reciprocal construction is real, this is
   exactly the rounding of one division and should be very tightly bounded; if `COT` is
   evaluated independently, it will not be.
4. **`COT(x)` against `COS(x)/SIN(x)`** computed in the worksheet, at the same `x`. Systematic
   disagreement distinguishes "reciprocal of the published tangent" from "ratio of the
   published primaries" — two hypotheses that the identity in probe 3 cannot separate on its
   own.
5. **Pole approach**: the doubles nearest `π`, `2π`, `10π`, and the doubles nearest `2^26`
   multiples of `π`. The magnitude returned reads off the reduction's residual precision
   directly, and it is where the amplification analysis above becomes visible.
6. **Small arguments**: `COT(2^-20)` down to `COT(2^-1070)`, testing the `1/x` regime and the
   subnormal overflow boundary.
7. **The same vectors on a second CPU vendor**, given the microcode caveat.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| simple pole | A singularity where the function behaves like `c/(x - a)` near `a` |
| representable pole | A pole that is exactly a binary64 value; for `cot` only `x = 0` |
| reciprocal construction | Forming `COT` as `1/TAN` of the already-rounded published tangent |
| published primary | The binary64 `SIN`/`COS`/`TAN` value Excel returns, before any reciprocal |
| amplification near a pole | The growth of relative error as the argument approaches `kπ` |
| host-scoped | A result whose last bits depend on the CPU executing it |

## Sources

- Microsoft, "COT function" —
  <https://support.microsoft.com/en-us/office/cot-function-c446f34d-6fe4-40dc-84f8-cf59e5f5e31a>
  (fetched at curation: signature, the radians reading, the `|number| < 2^27` constraint, the
  `#NUM!` and `#VALUE!` conditions, and the `COT(0)` `#DIV/0!` rule).
- Handbook evidence record `EV-MATH-0004` — the six-surface circular identification, the
  `FPREM1`-against-ROM-`π` substrate, the reciprocal construction of `COT`/`CSC`/`SEC` from the
  published primaries, the ruled-out Cody–Waite reduction, and the host-CPU microcode caveat.
  Read its reader warning.
- Abramowitz & Stegun, chapter 4.3, in particular 4.3.70 — the cotangent expansion.
- Payne & Hanek (1983) — infinite-precision radian reduction.
- Muller, *Elementary Functions: Algorithms and Implementation* — tangent-family worst cases.
- Handbook, [COS](FUNC.COS.md) — the shared reduction discussion;
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.COT.json` (the
  `arg_domain_guard=circular_trig_overflow` and `non_finite=num` axis values) and
  `data/presence/FUNC.COT.json`.
