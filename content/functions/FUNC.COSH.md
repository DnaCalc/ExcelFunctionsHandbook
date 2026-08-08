---
schema: efh.function-page/v1
function_id: FUNC.COSH
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0008
open_problems: []
references:
  - work: "Microsoft Support — COSH function"
    locator: "https://support.microsoft.com/en-us/office/cosh-function-e460d426-c471-43e8-9540-a57ff3b70555"
    role: "documented signature, the (e^x + e^-x)/2 formula, and worked examples; no error conditions are stated there"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 4.5 (Hyperbolic Functions), 4.5.1-4.5.20"
    role: "definition, identities, series, and the relations to the circular functions"
  - work: "fdlibm (Sun Microsystems freely distributable libm)"
    locator: "e_cosh.c"
    role: "the branch structure that avoids premature overflow and the expm1 form near zero"
  - work: "Cephes mathematical library"
    locator: "cosh.c"
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
family: cosh
role_in_family: >-
  The hyperbolic cosine — the even half of the exponential, and the member of the hyperbolic
  set whose only hard problem is overflow rather than cancellation.
---

# COSH

## What it computes

`COSH(number)` returns the hyperbolic cosine. Microsoft's page gives the defining formula:

    cosh x = (e^x + e^-x) / 2

It is the even part of the exponential function; `sinh` is the odd part, and `e^x = cosh x +
sinh x` is the decomposition.

| Property | Statement |
|---|---|
| Domain | all real `x` |
| Range | `[1, ∞)`, with the minimum `cosh 0 = 1` attained only at 0 |
| Parity | even: `cosh(-x) = cosh(x)` |
| Series | `1 + x²/2! + x⁴/4! + …`, convergent for all `x` |
| Fundamental identity | `cosh²x - sinh²x = 1` |
| Relation to circular | `cosh x = cos(ix)` and `cos x = cosh(ix)` |
| Derivative | `d/dx cosh x = sinh x` |
| Addition | `cosh(x+y) = cosh x cosh y + sinh x sinh y` |
| Asymptotics | `cosh x ~ e^abs(x) / 2` as `abs(x) → ∞` |
| Entire | no poles, no branch cuts; periodic with period `2πi` in the complex plane |

The catenary is the physical reading: a uniform chain hanging under gravity takes the shape
`y = a·cosh(x/a)`. That is also why `COSH` shows up in engineering spreadsheets far more often
than its siblings.

Two numbers fix the practical boundary. `cosh x` exceeds the largest finite binary64 when
`|x|` passes roughly `709.78 + ln 2 ≈ 710.4758`; and `cosh x` is indistinguishable from
`e^{|x|}/2` in binary64 once `|x|` is above about 22, because the `e^{-|x|}` term has fallen
below half an ulp of the result.

## Arguments

| Argument | Meaning | Notes |
|---|---|---|
| `number` | Any real number. Required. | Microsoft documents no magnitude constraint |

Ordinary to-number coercion applies, and the reference engine declares the surface a scalar
kernel that lifts elementwise over arrays — see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

Note the contrast with the newer hyperbolic surfaces: [COTH](FUNC.COTH.md) and
[CSCH](FUNC.CSCH.md) carry a documented `|number| < 2^27` constraint on their pages, and
`COSH` — a much older function — carries none.

## Result and edge cases

Returns `Number` in `[1, ∞)`.

- **Zero** — `COSH(0) = 1`, exactly, and this is the function's minimum.
- **Sign** — the result never depends on the sign of the argument.
- **Never below 1.** An implementation that returns a value slightly under 1 for a tiny
  argument has a cancellation bug; the correct evaluation cannot do it. This is the cheapest
  self-check on the whole page.
- **Overflow** — beyond roughly `|x| = 710.4758` the true value is not representable. The
  reference engine records `non_finite=num` for this surface, meaning the non-finite result is
  converted to an error rather than published, and `EV-MATH-0008` records that convention with
  `COSH` among its named surfaces: a large-magnitude `COSH` argument is one of that record's
  witnesses. The record publishes **no count** — it establishes where an error code appears,
  with witnesses and no denominator.
- **The gap between `e^709` and `cosh(710)`.** A naive implementation that computes `EXP(x)`
  first overflows about `ln 2` earlier than the true function does. There is a narrow band of
  arguments where the mathematically representable answer is lost to the algorithm rather than
  to the format. Whether Excel's answer covers that band is unchecked.

## Errors

Microsoft's `COSH` page documents no error conditions.

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | Non-numeric argument | Shared call model, not this page |
| `#NUM!` | The true result is not finite | `EV-MATH-0008`, which names `COSH` — kind and error-code convention, no count |

That second row is the honest statement of what is known: the FINITE convention is on record
with witnesses, and the boundary between the last `#NUM!`-free argument and the first `#NUM!`
argument has not been located.

## Relationships

- **`SINH`** — the odd partner. `SINH` has a cancellation problem near zero that `COSH` does
  not have, and the same overflow problem at large arguments. `EV-MATH-0008` names both.
- **`TANH`** — `sinh/cosh`, bounded by 1, and the numerically well-behaved member.
- **`SECH`** — the reciprocal `1/COSH`, which *underflows* smoothly to zero where `COSH`
  overflows: the reciprocal pair is not symmetric in difficulty.
- **[COTH](FUNC.COTH.md), [CSCH](FUNC.CSCH.md)** — the 2013-generation hyperbolic reciprocals,
  with documented magnitude constraints that `COSH` does not carry.
- **`ACOSH`** — the inverse on `[1, ∞)`, principal branch. `ACOSH(COSH(x)) = |x|`, which is
  the useful metamorphic identity here precisely because it exposes the evenness.
- **[EXP](FUNC.EXP.md)** — the substrate in every plausible implementation, and the surface
  whose evaluation route has actually been identified. Anything learned about `EXP` transfers
  here.
- **[COS](FUNC.COS.md)** — `cosh x = cos(ix)`. Related in name and in theory; opposite in every
  computational respect.
- **`IMCOSH`** — the complex version.

## Numerical notes

Hyperbolic cosine has no cancellation problem — the two terms of `(e^x + e^-x)/2` are both
positive — and no reduction problem. Its whole difficulty is **range**.

The standard branch structure, essentially as in fdlibm's `e_cosh.c` and Cephes' `cosh.c`:

1. **Tiny `|x|`** (below about `2^-27`): return 1. The correction term is below half an ulp.
2. **Small `|x|`** (up to about `ln 2 / 2`): use `t = expm1(|x|)` and evaluate
   `cosh x = 1 + t²/(2(1+t))`. The point is accuracy in the *correction*, not in the result:
   computing `(e^x + e^-x)/2` directly here is stable in the result but computes the deviation
   from 1 poorly, and the deviation is what a caller who then subtracts 1 actually wants.
   (Excel has no `COSHM1`, so a spreadsheet user computing `COSH(x)-1` for small `x` gets the
   cancellation regardless of how good `COSH` is. The remedy in a worksheet is the series
   `x²/2 + x⁴/24`, not a better `COSH`.)
3. **Moderate `|x|`** (up to about 22): `t = e^{|x|}`; return `(t + 1/t)/2`.
4. **Large `|x|`** (up to the overflow threshold of `e`): return `e^{|x|}/2`; the reciprocal
   term is below the rounding of the result.
5. **Near the top** (`|x|` between `ln(DBL_MAX)` and `ln(2·DBL_MAX)`): `e^{|x|}` itself
   overflows even though `cosh x` does not. The standard trick is `w = e^{|x|/2}`, return
   `(w/2)·w` — two representable halves multiplied at the end. Skipping this step costs the
   function its top `ln 2` of range, which is the band named above.

A&S chapter 4.5 gives the identity set; 4.5.64 onwards gives the classical polynomial
approximations. The accuracy of any of these branches is inherited from `EXP`, which is why
`EV-MATH-0001`'s identification of the exponential's substrate (on the [EXP](FUNC.EXP.md) page)
is the most relevant external fact about this surface — even though `EV-MATH-0001` does not
name `COSH` and nothing from it may be attributed here.

## What has not been checked

`EV-MATH-0008` names this surface. It is a kind-and-error-code convention record with named
witnesses and, in its own words, **no row count anywhere**; it establishes where Excel places
an error, not how accurate the values are. No numeric-bits comparison count exists for `COSH`
from that entry, and no other evidence record in the Handbook names it.

No Handbook vector suite exists for `COSH`. Microsoft's page documents the formula and two
worked examples and no error conditions. Nobody has checked this function's values against
Excel within the Handbook's record.

Inputs I would probe first:

1. **The overflow boundary, bisected**: `COSH(710)`, `COSH(710.4)`, `COSH(710.5)`,
   `COSH(711)`. The mathematical threshold is near `710.4758`; an implementation that computes
   `EXP(x)/2` without the split-and-multiply trick fails from about `709.79` instead. The gap
   between where Excel starts returning `#NUM!` and the true threshold measures precisely which
   branch structure is in use, and the answer is an error code rather than a number.
2. **The evenness identity**: `COSH(x) = COSH(-x)` bitwise across the range. Any failure means
   the sign is entering the computation, which it must not.
3. **Small arguments**: `COSH(2^-20)`, `COSH(2^-27)`, `COSH(2^-30)` — the branch boundary where
   an `expm1` form and a direct form separate, and where a value below 1 would be a defect.
4. **`COSH(x)^2 - SINH(x)^2 - 1`** across the range — a metamorphic probe needing no oracle,
   which localises drift to one of the pair.
5. **`ACOSH(COSH(x)) - ABS(x)`** — the round trip, which is well conditioned for moderate `x`.
6. **`COSH(x)` against `(EXP(x) + EXP(-x))/2` computed in the worksheet** — if the two agree
   bitwise across the whole range, that is itself a substrate identification, and it would be a
   surprising one at the top of the range.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| even part | `(f(x) + f(-x))/2`; for `f = exp` this is exactly `cosh` |
| overflow threshold | The largest `|x|` for which `cosh x` is a finite binary64 value |
| split-and-multiply | Computing `e^{x/2}` twice to reach results `EXP` alone cannot represent |
| `expm1` | `e^x - 1` computed without cancellation; the accurate route near zero |
| FINITE convention | The recorded convention that a non-finite real result surfaces as an error |
| catenary | The hanging-chain curve `a·cosh(x/a)`, the function's classic application |

## Sources

- Microsoft, "COSH function" —
  <https://support.microsoft.com/en-us/office/cosh-function-e460d426-c471-43e8-9540-a57ff3b70555>
  (fetched at curation: signature, the `(e^x + e^-x)/2` formula, worked examples; no error
  conditions are documented there).
- Handbook evidence record `EV-MATH-0008` — the FINITE error-code convention with named
  witnesses across a nine-surface family including `COSH`, and its explicit statement that it
  publishes no count. Read its reader warning.
- Abramowitz & Stegun, chapter 4.5 — hyperbolic functions, identities, series and
  approximations.
- fdlibm `e_cosh.c` and Cephes `cosh.c` — the standard branch structures.
- Handbook, [EXP](FUNC.EXP.md) — the substrate every branch above depends on.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.COSH.json` (the `arg_domain_guard=none` and
  `non_finite=num` axis values) and `data/presence/FUNC.COSH.json`.
