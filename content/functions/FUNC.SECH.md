---
schema: efh.function-page/v1
function_id: FUNC.SECH
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft — WorksheetFunction.Sech method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.sech"
    role: "the complete documented content: one-line description, one argument, no remarks and no error conditions"
  - work: "Microsoft Support — SECH function"
    locator: "https://support.microsoft.com/en-us/office/sech-function-e05a789f-5ff7-4d7f-984a-5edb9b09556f"
    role: "the worksheet-surface documentation page named in the projection; not retrievable for this pass"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 4, sections 4.5.1-4.5.90 (hyperbolic functions); 4.5.65 for the series"
    role: "the defining identities, the Euler-number series, and the relation to the Gudermannian"
  - work: "OxFunc — EXCEL_MATH_DEVIATION_CATALOG.md"
    locator: "docs/EXCEL_MATH_DEVIATION_CATALOG.md entry XMD-008"
    role: "the no-infinities convention that COSH is declared under and SECH is not"
  - work: "Cody & Waite, Software Manual for the Elementary Functions"
    locator: "chapters on EXP and the hyperbolic functions"
    role: "the standard large-argument formulation that avoids overflow of the intermediate"
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
family: sech
role_in_family: "The reciprocal of COSH; the bounded, pole-free member of the reciprocal hyperbolic pair."
---

# SECH

## What it computes

`SECH(number)` returns the hyperbolic secant.

    sech x = 1 / cosh x = 2 / (eˣ + e⁻ˣ)

**Domain.** All real `x` — there are no poles, because `cosh x ≥ 1` everywhere. **Range.**
`(0, 1]`, attaining 1 only at `x = 0` and approaching 0 as `|x| → ∞`. The function is even,
smooth, strictly decreasing in `|x|`, and asymptotically

    sech x ~ 2e^{−|x|}   as |x| → ∞

Its derivative is `−sech x · tanh x`, its integral is the Gudermannian function
`gd(x) = arctan(sinh x)`, and its Maclaurin series is again the Euler-number generating function,
this time with alternating signs:

    sech x = Σ_{n≥0} E_{2n} x^{2n} / (2n)!  =  1 − x²/2 + 5x⁴/24 − 61x⁶/720 + …

convergent for `|x| < π/2` — the radius set by the nearest *complex* pole at `iπ/2`, which is the
only trace the poles of [SEC](FUNC.SEC.md) leave on this function. (A&S 4.5.65.)

`sech²` is also, up to normalisation, the density of the hyperbolic secant distribution and the
derivative of `tanh`, which is why this function turns up in logistic modelling and in neural
network derivatives far more often than its trigonometric namesake turns up anywhere.

**The contrast with `SEC` is the point of the page.** `SEC` is unbounded, has infinitely many
poles, is periodic, and is refused above a large-argument threshold. `SECH` is bounded by 1, has no
poles, no periodicity, and no threshold. They differ by an `i`.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The value whose hyperbolic secant is wanted. Required. | — |

Microsoft's page describes it as "Angle in radians", which is the standard documentation phrasing
inherited from the circular functions; the hyperbolic argument is not an angle and is not in
radians — it is a dimensionless real number, or, if one insists on a geometric reading, twice the
area of a hyperbolic sector. The wording is harmless and worth not taking literally.

One numeric slot under ordinary to-number coercion; the projection classifies the surface
`UnaryNumericScalarOrArrayElementwise`, so array arguments lift elementwise — see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

Microsoft's VBA page states no remarks and no error conditions.

## Result and edge cases

Returns `Number` in `(0, 1]`.

- **`SECH(0)` is exactly 1**, and it is the maximum.
- **`SECH(−x) = SECH(x)`** exactly, for every representable `x`: the function is even and any
  reasonable implementation is symmetric by construction.
- **Underflow to zero happens sooner than the mathematics requires.** The naive `1/cosh(x)`
  overflows its intermediate at `|x| ≈ 710.4758` — the point where `cosh` exceeds the double range
  — and thereafter returns `1/∞ = 0`. But the true `sech x ≈ 2e^{−|x|}` remains representable as a
  subnormal until `|x| ≈ 745.13`. There is therefore a band roughly 35 units wide in which the
  correct answer exists and the naive formula returns zero. The reference engine computes
  `1.0 / n.cosh()` and so has this behaviour; whether Excel does is unknown here and is the first
  probe below.
- **No domain guard.** The projection declares `arg_domain_guard=none; non_finite=allow` for this
  surface. **This is worth recording as a projection-level inconsistency**: `COSH`, of which this
  is the reciprocal, declares `non_finite=num` and is a named member of the `FINITE` family in
  OxFunc's catalogue entry **XMD-008** — the entry that records Excel never publishing an infinity
  and converting every non-finite real result to `#NUM!`. `SECH` (and its sibling `CSCH`) sit
  outside that declaration. For `SECH` the consequence is benign in the real domain, because a
  bounded function cannot produce an infinity from a finite argument; but the pair
  `COSH`/`SECH` carrying different non-finite policies is an axis-level divergence the Handbook
  records rather than smooths over.
- **Arrays.** Elementwise lift, element-local failures.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | The argument does not convert to a number | shared coercion model |
| propagated | The argument is an error value | shared coercion model |

No documented error condition, and no declared domain guard. Unlike [SEC](FUNC.SEC.md), this
surface carries no large-argument refusal in the projection — which is itself a testable claim,
since `SEC` and `SECH` are documented in identical terms and behave, on this axis, completely
differently.

## Relationships

- **`COSH`** — the reciprocal. `SECH(x)` and `1/COSH(x)` should agree; whether they do, and
  whether both are available where the other is not, is the natural consistency probe. `COSH`
  overflows to `#NUM!` under the no-infinities convention while `SECH` of the same argument is a
  perfectly ordinary small number — so `SECH` is *strictly more capable* than the expression it is
  shorthand for, in exactly the band where it matters.
- **`CSCH`** — the other reciprocal hyperbolic, `1/sinh`. Unlike `SECH` it *does* have a pole, at
  `x = 0`, and it shares `SECH`'s `non_finite=allow` declaration — which for `CSCH` is a
  substantive question rather than a benign one.
- **`TANH`** — related by `sech²x + tanh²x = 1`, the hyperbolic Pythagorean identity, which gives
  the alternative evaluation `sech x = √(1 − tanh²x)`. That route is well conditioned for large
  `|x|` (where `tanh → ±1` and the subtraction cancels catastrophically — so it is in fact the
  *worse* route there) and fine near zero. The identity is more useful in the other direction.
- **[SEC](FUNC.SEC.md)** — the circular namesake, `sech(x) = sec(ix)`. Beyond the name and the
  Euler numbers in their series they have almost nothing in common.
- **There is no `ASECH`.** The inverse must be written `ACOSH(1/x)`, defined for `0 < x ≤ 1`.
- **`2/(EXP(x)+EXP(-x))`** — the hand-written form, which overflows earlier than `SECH` and should
  not be used.

## Numerical notes

**`sech` is a well-conditioned function, and that is unusual enough to say plainly.** Its condition
number is `|x tanh x|`, which grows only linearly in `|x|` — compare the unbounded amplification of
[SEC](FUNC.SEC.md) near its poles. There is no cancellation anywhere: the denominator
`eˣ + e⁻ˣ` is a sum of positive terms, so no digits are lost forming it. Near zero the series is
alternating but the terms decay fast and the value is near 1, so relative accuracy is easy. The
only difficulty in the entire function is *range*, not accuracy.

**The premature-underflow band, and the fix.** For `|x|` beyond about 710 the correct formulation
is not `1/cosh(x)` but

    sech x = 2·e^{−|x|} / (1 + e^{−2|x|})

in which every intermediate is bounded: `e^{−|x|}` underflows gracefully rather than overflowing,
and the denominator tends to 1. This is the standard Cody & Waite treatment of the hyperbolic
functions applied in the reciprocal direction, and it extends the usable domain by the full ~35
units and gives correctly rounded subnormals throughout. An even simpler variant for the far tail
is `exp(ln 2 − |x|)`, which is a single exponential and is accurate to within the error of `exp`
until the result underflows to zero at the true boundary.

The remaining question — one an implementation must answer explicitly — is what to return once even
that underflows: `+0` is the correct rounding of a positive value smaller than the smallest
subnormal, and it is also what the naive route returns 35 units too early. The two zeros are
indistinguishable at the value level, which is why this defect is so easy to ship.

**Small-argument accuracy.** For `|x| < 2⁻²⁶`, `sech x` rounds to exactly 1, and any implementation
that computes `1/cosh(x)` gets that for free because `cosh` rounds to 1 first. No special case is
needed, which is a pleasant contrast with almost every other function in this batch.

**Evenness should be structural.** Fold the argument to `|x|` at the top of the kernel. An
implementation that does not, and that has any asymmetry in its exponential, will return different
values for `x` and `−x` — a defect that no accuracy test looking only at magnitudes will find.

## What has not been checked

No Handbook vector suite exists for `SECH`, and no evidence record in
`content/evidence/records/` lists this surface among its subjects. The presence projection records
no upstream defect stream touching this module, and no math-deviation entry names it. Nobody has
checked this function against Excel within the Handbook's record.

Note also the shape of what *is* known about its neighbours: `SEC` carries a per-surface upstream
sweep and an observed argument limit; `SECH` carries neither. The hyperbolic reciprocals are the
thinner-evidenced half of the trigonometric surface.

Inputs I would probe first:

1. **`SECH(710)`, `SECH(711)`, `SECH(720)`, `SECH(745)`, `SECH(746)`** — the premature-underflow
   band. If Excel returns zero from around 710 it is computing `1/COSH`; if it returns subnormal
   values out to about 745 it is doing something better; if it returns `#NUM!` anywhere in the band
   it is applying the no-infinities convention to an intermediate, which would be a third and quite
   different answer. This is the single most informative probe on the page.
2. **`SECH(1000)` beside `1/COSH(1000)`** — the direct comparison, where `COSH` is expected under
   XMD-008 to be `#NUM!` while `SECH` returns a number. A function succeeding where its own
   definition fails is worth confirming.
3. **`SECH(1)` against `1/COSH(1)` and `2/(EXP(1)+EXP(-1))`** — three routes to one value in the
   well-behaved region, which characterises the rounding staging.
4. **`SECH(-x)` against `SECH(x)`** for a spread of magnitudes — the evenness check.
5. **`SECH(134217728)`** — the argument at which the *circular* family is recorded to refuse. If
   `SECH` also refuses there, the guard is broader than the catalogue entry states and the
   catalogue is understating its scope; if it returns zero, the projection's `arg_domain_guard=none`
   is confirmed from the worksheet.
6. **`SECH(5E-324)` and `SECH(1E-20)`** — the flat region near zero, where the answer must be
   exactly 1.
7. **An array argument**, given the elementwise-lift declaration.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| pole-free | `cosh` never vanishes, so `sech` is finite everywhere on the reals |
| premature underflow | Returning zero while the true value is still representable as a subnormal |
| Gudermannian | `gd(x) = ∫sech`, the bridge between circular and hyperbolic arguments |
| no-infinities convention | The observed Excel rule that a non-finite real result becomes `#NUM!` |

## Sources

- Microsoft, "WorksheetFunction.Sech method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.sech> (the entire
  documented content, including the "angle in radians" phrasing; no remarks, no errors).
- Microsoft Support, "SECH function" —
  <https://support.microsoft.com/en-us/office/sech-function-e05a789f-5ff7-4d7f-984a-5edb9b09556f>
  (retrieval refused with HTTP 403 during this pass; nothing here is sourced from it).
- OxFunc `docs/EXCEL_MATH_DEVIATION_CATALOG.md` entry XMD-008 — the no-infinities convention and
  its named `FINITE` family, which includes `COSH` and not `SECH`.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, section 4.5 — hyperbolic functions,
  the Euler-number series and the Gudermannian.
- Cody & Waite, *Software Manual for the Elementary Functions* — the bounded-intermediate
  formulation for large arguments.
- Handbook, [SEC](FUNC.SEC.md), [Coercion and lifting](../model/02-coercion-and-lifting.md),
  [About implementation options](../model/07-implementation-options.md).
- Handbook projections `data/functions/FUNC.SECH.json` (`arg_domain_guard=none`,
  `non_finite=allow` — contrast `data/functions/FUNC.COSH.json`, which declares `non_finite=num`)
  and `data/presence/FUNC.SECH.json` (own module, no shared surfaces, no defect streams).
