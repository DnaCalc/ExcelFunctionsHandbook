---
schema: efh.function-page/v1
function_id: FUNC.ACOSH
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Acosh method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.acosh"
    role: "documented description, the 'greater than or equal to 1' constraint, and the Acosh(Cosh(x)) = x round-trip statement"
  - work: "Microsoft Support — ACOSH function"
    locator: "https://support.microsoft.com/en-us/office/acosh-function-e3992cc1-103f-4e72-9f04-624b9ef5ebfe"
    role: "the canonical worksheet article; not retrieved for this pass (fetch returned HTTP 403)"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 4 (Elementary Transcendental Functions), section 4.6"
    locator: "4.6.20-4.6.22, inverse hyperbolic functions"
    role: "the logarithmic closed forms, principal branches and the relations among the inverse hyperbolic functions"
  - work: "fdlibm, e_acosh.c"
    locator: null
    role: "the reference three-branch implementation: near-one log1p form, mid-range closed form, and large-argument ln(2x)"
  - work: "Higham, Accuracy and Stability of Numerical Algorithms"
    locator: "chapter 1, the log1p discussion"
    role: "why ln(1 + t) must not be evaluated by forming 1 + t for small t"
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
family: acosh
role_in_family: >-
  The inverse hyperbolic cosine on [1, infinity): the half-domain member of the inverse hyperbolic
  set, whose square-root singularity at 1 and logarithmic growth at infinity make it hard at both
  ends for different reasons.
---

## What it computes

`ACOSH(number)` is the inverse hyperbolic cosine: the non-negative value whose hyperbolic cosine
is *number*.

    acosh x  =  ln( x + sqrt(x^2 - 1) ),    x >= 1

- **Domain**: `[1, +infinity)`. `cosh` maps `R` onto `[1, +infinity)` and is two-to-one, so the
  inverse requires a branch choice; the principal branch takes the non-negative one. Below `1`
  there is no real value at all — `cosh` never goes there.
- **Range**: `[0, +infinity)`. `acosh(1) = 0` is the branch point.
- **Not odd, not even**: the function is defined only on a half-line, so parity is not available.
  This is the structural difference from `ASINH` and `ATANH`, both of which are odd on symmetric
  domains, and it is why `ACOSH` is the awkward member of the trio.
- **Derivative**: `d/dx acosh x = 1 / sqrt(x^2 - 1)`, which is infinite at `x = 1` and decays like
  `1/x`. Vertical tangent at the left endpoint; logarithmic flattening at the right.
- **Endpoint behaviour**: with `x = 1 + t`,
  `acosh(1 + t) = sqrt(2t) * (1 - t/12 + 3t^2/160 - ...)`. The same square-root singularity that
  governs `ACOS` at its endpoints governs `ACOSH` at `1`, and for the same reason — the inverse
  of a function with a quadratic minimum.
- **Asymptotic behaviour**: `acosh x = ln(2x) - 1/(4x^2) - ... ` as `x -> +infinity`. Growth is
  logarithmic, so the whole upper half of the double range maps into a narrow band of results: at
  the top of binary64 the answer is a little over 710. Every argument from about `1e100` to the
  largest finite double produces an answer between roughly 230 and 711.
- **Relations**: `acosh x = |asinh(sqrt(x^2 - 1))|` and `acosh x = atanh(sqrt(x^2-1)/x)`; also
  `acosh(cosh t) = |t|`, which is the round trip Microsoft's Learn reference states (as
  `Acosh(Cosh(number))` equalling the number — true for non-negative arguments, and the absolute
  value is the part the documentation leaves out).
- **Complex continuation**: `acosh` extends to the plane with a branch cut along
  `(-infinity, 1]`. The real domain is exactly the complement of that cut on the real axis.

Abramowitz & Stegun give the logarithmic closed forms for the inverse hyperbolic functions in
chapter 4 section 4.6.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | Any real number equal to or greater than 1. Required. | — |

That wording is Microsoft's: the Learn reference describes the argument as "any real number equal
to or greater than 1", and the description repeats that *number* must be greater than or equal
to 1.

One argument; the reference engine records an arity of exactly one and classifies the surface as a
unary numeric scalar-or-array kernel, so arrays lift elementwise. Ordinary numeric slot under the
shared coercion rules — see [Coercion and lifting](../model/02-coercion-and-lifting.md).

The misunderstood position is the *lower* bound. Readers who arrive from `ASINH`, which accepts
every real, or from `ACOS`, whose domain is `[-1, 1]`, routinely pass a value in `(0, 1)` and are
surprised by a domain failure. `[1, infinity)` is the complement of `ACOS`'s domain on the positive
axis, and the two functions share exactly one point.

## Result and edge cases

Returns `Number`.

- **`x = 1`** is in the domain and returns zero. The interval is closed on the left; an
  implementation testing `x > 1` gets the branch point wrong.
- **Just below 1** — one ulp under — is a domain failure. This is a real hazard because the
  natural producer of `ACOSH` arguments is a ratio that should be at least one and, after
  rounding, sometimes is not.
- **Just above 1** is where the square-root singularity lives, and where a naive implementation
  loses half its significant digits. See the numerical notes.
- **Very large arguments** are all admissible: the mathematical result stays finite and small
  right up to the largest finite double. Whether an implementation *delivers* it there is the
  interesting question, because the naive closed form squares the argument first.
- **Arrays** lift elementwise, with out-of-domain elements producing element-local errors.

**A reference-engine classification worth recording.** The projected `real_result_policy` for
`ACOSH` carries `non_finite=allow`: a non-finite kernel result is permitted to leave the kernel.
`ACOSH`'s sibling `ASINH` carries the same axis value but guards the overflow explicitly inside
its kernel; `ACOSH` does not, and it delegates to the platform's `acosh` after a single
`x < 1` test. The two nearest-neighbour surfaces therefore treat the same
`ln(x + sqrt(x^2 ± 1))` overflow question differently. The Handbook records that asymmetry as a
finding rather than smoothing it over: it is visible in the battery rendered beside this page,
where the largest finite double produces a non-finite outcome on this surface and an error on
`ASINH`. Excel's published value universe has no infinity — see
[The value universe](../model/01-value-universe.md) — so a non-finite raw return is a state that
cannot survive to a cell, and what Excel itself returns at that argument is unknown here.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | *number* is less than 1 | Reference engine's kernel; Microsoft's Learn page states the constraint but names no error code |
| `#VALUE!` | The argument does not convert to a number | Shared coercion rules |
| propagated | An error value in the argument is returned unchanged | Shared coercion rules |

**The documentation states a constraint and no consequence.** Microsoft's Learn reference says the
argument must be at least 1; it does not say what happens otherwise. The `#NUM!` above is the
reference engine's choice. Microsoft's worksheet article was not retrieved for this pass, so
whether it names an error code is unknown here.

## Relationships

- **`COSH`** — the forward function. `ACOSH(COSH(t)) = |t|`; the absolute value is the branch
  choice made visible, and it is the part of the round trip the documentation omits.
- **`ASINH`** — the everywhere-defined sibling. The two closed forms differ by one sign under the
  square root, `x^2 + 1` versus `x^2 - 1`, and that sign is the difference between a domain
  restriction and none. It is also the pair where the reference engine's overflow handling
  diverges; see above and the `ASINH` page.
- **`ATANH`** — the third inverse hyperbolic primary, defined on `(-1, 1)`. Between them the three
  cover `(-1,1)`, `R`, and `[1,infinity)`; `ACOSH` and `ATANH` have complementary domains that meet
  only at `±1`, where `ATANH` has poles and `ACOSH` a branch point.
- **`ACOTH`** — defined on `|x| > 1`, so it shares `ACOSH`'s exclusion of the unit interval but on
  both sides.
- **`ACOS`** — the circular namesake, and the complementary domain: `[-1, 1]` versus
  `[1, infinity)`. Both vanish at `x = 1` and both have a square-root approach to it. They are
  genuinely two halves of one complex function restricted to the real axis.
- **`LN`** and **`SQRT`** — the substrate of every closed-form evaluation, and the source of both
  hazards.
- **Confused with**: `ACOS`, by name; and with `1/COSH`, by readers expecting the "inverse" in
  "inverse hyperbolic cosine" to mean a reciprocal. It does not — `SECH` is the reciprocal.

## Numerical notes

The closed form `ln(x + sqrt(x^2 - 1))` is correct mathematics and, evaluated literally, a poor
program. It fails at both ends of the domain, in different ways, and both failures have standard
remedies.

**Failure one: cancellation at the left endpoint.** For `x` just above `1`, `x^2 - 1` is a
subtraction of two nearly equal numbers. Worse, `x^2` has already rounded, so bits are lost before
the subtraction even happens. The correct answer near `1` is `sqrt(2t)` with `t = x - 1`; the naive
form computes a square root of a quantity that has lost roughly half its significant digits, so
the result loses about half of its own. The remedy is to factor:

    x^2 - 1 = (x - 1)(x + 1) = t(t + 2)

and to write the whole thing through `log1p`:

    acosh(1 + t) = log1p( t + sqrt(t*(t + 2)) )

Here `1 + t` is never formed, `t` is exact when `x` is a double near `1` (Sterbenz), and the
logarithm's own near-one cancellation is handled by `log1p` rather than by `ln`. Both remedies —
factoring the difference of squares and using `log1p` — are needed; either alone leaves half the
problem in place. Higham's discussion of `log(1+x)` is the standard statement of the second half.

**Failure two: overflow in the intermediate at the right end.** The literal form squares `x`
first. `x^2` overflows binary64 once `x` exceeds about `1.34e154` — the square root of the largest
finite double — while the true `acosh(x)` at that point is a perfectly ordinary number near 355,
and stays finite and small all the way to the top of the double range. So the naive form has an
entire band of arguments, spanning more than half of the exponent range, on which the mathematics
is representable and the program is not. The remedy is a branch: for `x` large enough that
`x^2 - 1` would round to `x^2` anyway,

    acosh(x) = ln(x) + ln(2)

evaluated without ever forming `x^2`. `fdlibm`'s `e_acosh.c` implements exactly this
three-branch structure — `log1p` form near `1`, the closed form in the middle, `ln(2x)` at the
top — and it is the shape any careful implementation converges on.

**Where the branches go.** The conventional cut points are `x < 2` for the `log1p` branch and
`x > 2^28` (or thereabouts) for the `ln(2x)` branch, with the closed form in between. The exact
choices differ between libraries, and the residual profile of an implementation against a
high-precision reference makes them visible as steps — which is precisely how upstream
identification work has read other surfaces in this family.

**What this page does not claim.** Nothing above says what Excel computes. `ASINH`'s substrate has
been identified upstream as the literal `ln(x + sqrt(x^2+1))` — see that page and its evidence
record — which makes the same question live for `ACOSH`; but no identification for `ACOSH` exists
in the Handbook's record, and the two functions could easily be implemented by different code.

**A self-test that needs no oracle.** `ACOSH(COSH(t))` should return `|t|`. Run it across `t` from
subnormal to about 710. Where it stops round-tripping, and how it fails there, distinguishes the
branch structures described above without any external reference.

## What has not been checked

**No evidence record in the Handbook names `FUNC.ACOSH`**, and no Handbook vector suite exists for
it. Nobody has checked this function against Excel within the Handbook's record. The battery
rendered beside this page is the reference engine's own output, no Excel involved, as its own label
states.

The presence projection records that the implementing module is named in one upstream defect
stream (`BUG-FUNC-027`, a broad scalar-invocation-space finding), and that the module carries no
entries in the discrepancy or math-deviation catalogues. In plain terms: the family around this
surface has been swept, and this surface has not been separately identified.

The documented statements above — the `>= 1` constraint and the `Acosh(Cosh(x))` round trip —
come from Microsoft's Learn `WorksheetFunction.Acosh` reference, which was retrieved. Microsoft's
worksheet article was not (HTTP 403).

Probes worth running first:

1. **`ACOSH(1)`** — the closed branch point, and the cheapest test of `>=` versus `>`.
2. **`ACOSH(1 - 2^-52)` and `ACOSH(1 + 2^-52)`** — the two sides of the boundary, which pin both
   the domain edge and the error code that nothing documents.
3. **A sweep of `1 + 2^-k` for `k` from 1 to 52** against a high-precision reference. This is the
   decisive probe for the left-endpoint hazard: a naive `sqrt(x^2 - 1)` implementation degrades
   visibly and monotonically here, and a `log1p`-based one does not.
4. **The overflow band**: arguments spanning the square root of the largest finite double, in both
   directions. This single boundary distinguishes the naive closed form from the `ln(2x)` branch,
   and — given what is on record for `ASINH` — it is the probe most likely to find a real
   Excel departure on this surface.
5. **The largest finite double**, to settle what Excel publishes where the reference engine's own
   battery shows a non-finite outcome. A worksheet cannot hold an infinity, so *something* must
   happen, and the Handbook does not know what.
6. **`ACOSH(COSH(t)) - ABS(t)`** across the range, as the oracle-free round-trip probe.
7. **Array arguments mixing in-domain and out-of-domain elements**, for the element-local error
   policy.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| branch point | `x = 1`, where the two inverse branches of `cosh` meet and `acosh` is zero |
| `log1p` | The primitive computing `ln(1 + t)` accurately for small `t` |
| difference-of-squares factoring | Replacing `x^2 - 1` by `(x-1)(x+1)` to avoid a rounded square |
| overflow band | Arguments whose `acosh` is representable but whose `x^2` is not |
| non-finite policy | The reference engine's `non_finite=allow`: a non-finite kernel result is permitted to leave the kernel |

## Sources

- Microsoft Learn, "WorksheetFunction.Acosh method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.acosh> (retrieved: the
  description, the "equal to or greater than 1" argument constraint, and the round-trip statement).
- Microsoft, "ACOSH function" —
  <https://support.microsoft.com/en-us/office/acosh-function-e3992cc1-103f-4e72-9f04-624b9ef5ebfe>
  — the canonical worksheet article. **Not retrieved for this pass** (HTTP 403).
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 4 section 4.6 — the
  logarithmic closed forms for the inverse hyperbolic functions and their principal branches.
- `fdlibm` `e_acosh.c` — the published three-branch reference implementation.
- Higham, *Accuracy and Stability of Numerical Algorithms*, chapter 1 — the `log(1+x)` problem and
  its remedy.
- Handbook projections `data/functions/FUNC.ACOSH.json` (arity, lift profile, and the
  `real_result_policy` value `non_finite=allow`) and `data/presence/FUNC.ACOSH.json` (implementing
  module, the `BUG-FUNC-027` sweep mention, and zero entries in the discrepancy and math-deviation
  catalogues).
- Handbook [The value universe](../model/01-value-universe.md) — why a non-finite value cannot be
  a published formula result — and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook page [ASINH](FUNC.ASINH.md) — the sibling whose overflow substrate *is* on record, and
  the reason the same question is live here.
