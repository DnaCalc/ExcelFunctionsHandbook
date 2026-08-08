---
schema: efh.function-page/v1
function_id: FUNC.TANH
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — TANH function"
    locator: "https://support.microsoft.com/en-us/office/tanh-function-017222f0-a0c3-4f69-9787-b3202295dc6c"
    role: "documented signature and the statement of the defining formula; retrieval for this page was blocked by the upstream host"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 4 (Elementary Transcendental Functions), section 4.5"
    locator: "4.5.x"
    role: "definitions, series, identities, and the relation tanh x = -i tan(ix)"
  - work: "Cody & Waite, Software Manual for the Elementary Functions"
    locator: "chapter on TANH"
    role: "the three-band scheme: series, rational kernel on expm1, and saturation"
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
family: tanh
role_in_family: >-
  The bounded hyperbolic: saturating, pole-free, and the one member of the family whose
  reference-engine policy admits every finite result without a domain guard.
---

## What it computes

`TANH(number)` is the hyperbolic tangent:

    tanh x  =  sinh x / cosh x  =  (e^x - e^(-x)) / (e^x + e^(-x))  =  (e^(2x) - 1) / (e^(2x) + 1)

- **Domain**: all real numbers. No poles, no branch cuts, no excluded points. `tanh` is
  meromorphic with poles only at odd multiples of `i*pi/2` — nowhere on the real line. This is
  the sharpest contrast with `TAN`, which has a pole every `pi`.
- **Range**: the open interval `(-1, 1)`. Strictly increasing, hence a bijection from `R` onto
  `(-1, 1)`, which is why `ATANH` has domain `(-1, 1)` and blows up at both ends.
- **Parity**: odd.
- **Asymptotes**: `tanh x -> +1` as `x -> +infinity` and `-> -1` as `x -> -infinity`, and the
  approach is exponential: `1 - tanh x ~ 2 e^(-2x)`.
- **Series about zero**: `tanh x = x - x^3/3 + 2 x^5/15 - 17 x^7/315 + ...`, radius of
  convergence `pi/2` (the distance to the nearest complex pole). The coefficients are those of
  `tan` with alternating signs, which is the series form of `tanh x = -i tan(ix)`.
- **Identity**: `1 - tanh^2 x = sech^2 x`, the hyperbolic counterpart of the derivative relation
  for `tan`.

Microsoft's article gives the signature and states the defining formula. **Retrieval of that
page for this entry was blocked by the upstream host (HTTP 403), so the documented content is
stated here as documented behaviour and should be re-checked against the live article.** The
mathematics above is from the standard literature, not from the article.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | Any real number. Required. | — |

One argument; the reference engine records an arity of exactly one. An ordinary numeric slot
under the shared coercion rules — see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

No units. `TANH` is not an angle function and nothing in the `RADIANS`/`DEGREES` apparatus
applies to it.

## Result and edge cases

Returns `Number`, always in `(-1, 1)` mathematically — and in floating point, exactly `1` or
`-1` for arguments beyond the saturation point, because `1 - 2e^(-2x)` rounds to `1` once the
correction falls below half an ULP of `1`.

That saturation is the function's defining edge and it is entirely benign: `TANH` cannot
overflow, cannot divide by zero, and has no domain restriction. The reference engine's projected
`real_result_policy` reflects exactly that: `arg_domain_guard=none` and `non_finite=allow`. Set
against `SINH` (`non_finite=num`) and `TAN` (`arg_domain_guard=circular_trig_overflow`), `TANH`
is the family member with no guard at all, and the classification is consistent with the
mathematics — a bounded function has nothing to guard against.

The reference engine classifies `TANH` as a unary numeric scalar-or-array kernel
(`UnaryNumericScalarOrArrayElementwise`), so arrays lift elementwise with element-local failures.

Small arguments round-trip: for `|x|` small enough that `x^3/3` is negligible, the correctly
rounded `tanh x` is `x`, so subnormals pass through.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | The argument does not convert to a number | Shared coercion rules |
| propagated | An error value in the argument is returned unchanged | Shared coercion rules |

No domain error is possible for `TANH` and the reference engine declares no guard. Microsoft's
article, as far as the Handbook's record goes, lists no error conditions either — but see the
retrieval note above; that statement has not been re-verified against the live page.

## Relationships

- **`SINH` and `COSH`** — the numerator and denominator of the defining quotient. Note that
  `TANH` is well behaved precisely where `SINH` overflows: the two exponentials that overflow
  individually cancel in the ratio.
- **`ATANH`** — the inverse, `ATANH(y) = ln((1+y)/(1-y))/2`, with domain `(-1, 1)` and poles at
  both endpoints. The saturation of `TANH` and the singularity of `ATANH` are the same fact seen
  from two sides: once `TANH` has rounded to `1`, the round trip through `ATANH` is lost.
- **`TAN`** — the circular sibling, `tanh x = -i tan(ix)`. Bounded and pole-free versus
  unbounded and pole-ridden.
- **`SECH`** — `1 - TANH^2`, and the reciprocal of `COSH`.
- **Confused with**: `TAN`, by name; and `SIGN`, by shape, since `TANH` of a large argument is
  a smooth saturating stand-in for the sign function. That resemblance is the reason `TANH` is
  the classical neural-network activation.

## Numerical notes

`TANH` is the well-conditioned member of its family, but there is exactly one way to get it
badly wrong and it is the obvious way.

**Do not compute `TANH` as `SINH(x)/COSH(x)`.** Both intermediates overflow at a moderate
argument while their ratio remains a perfectly ordinary number near `1`. A naive quotient
produces `#NUM!`, an infinity, or a NaN — depending on the host's overflow policy — for
arguments where the answer is simply `1`. The same objection applies to
`(exp(x) - exp(-x))/(exp(x) + exp(-x))`.

**The standard three-band scheme** (Cody & Waite; `fdlibm`'s `tanh`):

1. **Tiny `|x|`**: return `x` (with the inexact flag raised where the platform cares), because
   the correction is below rounding.
2. **Small to moderate `|x|`**: use `expm1`. With `t = expm1(2|x|)`, `tanh|x| = t/(t + 2)`; or
   with `t = expm1(-2|x|)`, `tanh|x| = -t/(t + 2)`. The `expm1` form is what keeps relative
   accuracy near zero, where the direct exponential quotient would cancel `1` against `1`.
3. **Large `|x|`**: return `1` (with the sign applied). There is a well-defined threshold above
   which the correctly rounded result is exactly `1`, and computing anything at all beyond it is
   wasted work — and, if done naively, dangerous.

**Near-zero cancellation, stated precisely.** In `(e^x - e^(-x))/(e^x + e^(-x))` the numerator
suffers the same total cancellation as in `SINH`: both exponentials tend to `1` and their
difference is `2x`. The denominator is fine. So the relative error of the naive form grows
without bound as `x -> 0`, exactly where the function is most linear and a user is most likely
to trust it. The `expm1` reformulation is not an optimization; it is the difference between a
correct and an incorrect function on a large part of its useful domain.

**Symmetry.** Odd function: compute on `|x|`, apply the sign. An implementation that does so
gets exact antisymmetry for free.

**Why `TANH` is easy where `TAN` is hard.** There is no argument reduction. There is no `pi`.
There is no cancellation in the reduced argument because there is no reduced argument. Every
difficulty `TAN` has comes from mapping the input into a fundamental domain, and `TANH` has no
fundamental domain to map into. This is worth stating because the two functions are usually
taught as siblings, and numerically they have almost nothing in common.

## What has not been checked

No Handbook evidence record lists `FUNC.TANH` in its subjects, and no Handbook vector suite
exists for `TANH`. **Nobody has checked this function against Excel within the Handbook's
record.** Nothing on this page is a statement that any implementation agrees with Excel.

The documented content is additionally weaker than usual here: retrieval of Microsoft's `TANH`
article was blocked by the upstream host when this page was written, so even the documented
statements above carry a re-check obligation.

Probes worth running first:

1. **The saturation threshold.** Bisect for the smallest argument at which Excel returns
   exactly `1`. Its position is a fingerprint of the algorithm, and it also settles whether the
   result is ever allowed to exceed `1` by a rounding step (it should not).
2. **Large arguments well past saturation** — up to the top of the binary64 range. A naive
   `SINH/COSH` implementation fails here and a three-band implementation does not. One probe
   separates them.
3. **Small arguments across many decades** down to the subnormal floor, against a
   high-precision reference. The `expm1`-versus-naive probe.
4. **`TANH(x) + TANH(-x)`**, which must be exactly zero if the sign is applied by symmetry.
5. **`ATANH(TANH(x))`** for moderate `x` — a round trip that degrades visibly as saturation is
   approached, and a cheap way to locate where the information is lost.
6. **`TANH` against `SINH(x)/COSH(x)`** at an argument where `SINH` alone overflows. If `TANH`
   survives and the quotient does not, the implementation is not naive.
7. **Array arguments**, to confirm elementwise lifting.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| saturation | The point beyond which the correctly rounded result is exactly `1` (or `-1`) |
| `expm1` | The primitive computing `e^x - 1` accurately for small `x` |
| three-band scheme | Tiny / moderate / large branches used by the standard implementations |
| non-finite policy | The reference engine's `non_finite=allow` for this surface — no result is rejected |

## Sources

- Microsoft, "TANH function" —
  <https://support.microsoft.com/en-us/office/tanh-function-017222f0-a0c3-4f69-9787-b3202295dc6c>
  (signature and the defining formula). **Retrieval for this page was blocked by the upstream
  host; the documented behaviour above is stated as documented behaviour and should be
  re-checked against the article.**
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 4 section 4.5 — hyperbolic
  functions, series, identities, and the relation to the circular tangent.
- Cody & Waite, *Software Manual for the Elementary Functions* — the three-band formulation.
- `fdlibm` `tanh` and `expm1`; Boost.Math hyperbolic documentation.
- Handbook projections `data/functions/FUNC.TANH.json` (arity, classification,
  `real_result_policy` with `arg_domain_guard=none` and `non_finite=allow`) and
  `data/presence/FUNC.TANH.json`.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md).
