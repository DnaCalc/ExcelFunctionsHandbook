---
schema: efh.function-page/v1
function_id: FUNC.SQRTPI
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0011
  - EV-MISC-0014
open_problems: []
references:
  - work: "Microsoft Support — SQRTPI function"
    locator: "https://support.microsoft.com/en-us/office/sqrtpi-function-1fb4e63f-9b51-46d6-ad68-b3e7a8b519b4"
    role: "documented signature, the definition sqrt(number * pi), and the #NUM! condition for number < 0"
  - work: "IEEE 754-2019, Standard for Floating-Point Arithmetic"
    locator: "clause 5.4.1 (squareRoot)"
    role: "the correct-rounding requirement that a general power routine does not carry"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 6 and chapter 7"
    locator: "6.1.8 (Gamma(1/2)), 7.1.1 (erf)"
    role: "why sqrt(pi) is a named constant: it is Gamma(1/2) and the normalizing factor of the error function"
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
family: sqrtpi
role_in_family: >-
  The sole member: a two-operation convenience wrapper whose only interesting property is which
  routine performs the root.
---

## What it computes

`SQRTPI(number)` returns

    sqrt(number * pi)

- **Domain**: `[0, +infinity)`. Microsoft's page states: "If number < 0, SQRTPI returns the
  #NUM! error value."
- **Range**: `[0, +infinity)`, with `SQRTPI(0) = 0` and `SQRTPI(1) = sqrt(pi)`.
- **Shape**: a scaling of `SQRT` by `sqrt(pi)`, so it inherits every structural property of the
  square root — strictly increasing, concave, principal branch, relative condition number
  exactly `1/2`, branch point at the origin.

**Why the constant is `pi` and not something else.** `sqrt(pi)` is not an arbitrary convenience.
It is `Gamma(1/2)` (A&S 6.1.8), and it is the normalizing constant of the Gaussian: the error
function is defined as `erf(x) = (2/sqrt(pi)) * integral from 0 to x of e^(-t^2) dt` (A&S
7.1.1), and the normal density carries `1/sqrt(2 pi)`. Expressions of the form
`sqrt(2 pi sigma^2)` and `sqrt(n pi)` (Stirling's approximation:
`n! ~ sqrt(2 pi n) (n/e)^n`) occur constantly in statistics and in asymptotic analysis. `SQRTPI`
exists so that a worksheet can write one call instead of `SQRT(n * PI())`.

That is the whole of the mathematics. The function is a composition of a multiplication and a
square root, and everything interesting about it is *which* square root.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The number by which pi is multiplied. Required, must be non-negative. | — |

One argument; the reference engine records an arity of exactly one. Ordinary numeric slot under
the shared coercion rules — see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

The commonly misunderstood position: `number` is the *argument of the multiplication*, not of
the root. `SQRTPI(2)` is `sqrt(2 pi)`, not `2 sqrt(pi)`. Readers who want `2 sqrt(pi)` want
`SQRTPI(4)`.

## Result and edge cases

Returns `Number`.

- **Zero** gives zero.
- **Negative** gives `#NUM!`, as documented.
- **Very large arguments.** This is where the function stops being boring. `number * pi`
  overflows binary64 for arguments above roughly the top of the range divided by `pi`, and the
  overflow happens in the *intermediate product*, before the root is taken — even though
  `sqrt(number * pi)` would be an entirely ordinary number if computed with any headroom. A
  two-step implementation therefore has a band of arguments it cannot answer that the
  mathematical function has no trouble with. The remedy is standard and cheap: scale by a power
  of four before the multiplication and by the corresponding power of two after the root.
  Whether Excel does this is unverified.

The reference engine's projected `real_result_policy` for `SQRTPI` reads
`arg_domain_guard=none` with `non_finite=allow`. Two things follow, and both are recorded here
as findings rather than resolved:

1. **The documented `#NUM!` for negative arguments is not expressed by the declared axis.**
   `arg_domain_guard=none` says there is no argument-domain guard; Microsoft documents one. The
   likely reading is that the check lives inside the `Custom` kernel rather than in the axis
   vocabulary, making this a classification gap rather than a behavioural claim — but the
   Handbook records the mismatch, because a reader consulting the axis alone would conclude the
   function is total on the reals.
2. **`non_finite=allow` would permit publishing an infinity.** For an argument large enough that
   `number * pi` overflows, a naive two-step evaluation produces an infinite intermediate and an
   infinite root, and `non_finite=allow` does not reject it. That sits in tension with the
   Handbook's evidence record `EV-MATH-0008`, whose finding — for a different set of surfaces,
   not including `SQRTPI` — is that Excel never publishes an infinity and maps non-finite real
   results to `#NUM!`. Whether `SQRTPI` is an exception to that convention, or whether the axis
   value is simply not load-bearing here, is an open question and the sharpest probe on this
   page.

The reference engine classifies `SQRTPI` as a unary numeric scalar-or-array kernel
(`UnaryNumericScalarOrArrayElementwise`) with `kernel_signature_class: Custom`, so arrays lift
elementwise.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | `number < 0` | Documented by Microsoft |
| `#VALUE!` | The argument does not convert to a number | Shared coercion rules |
| propagated | An error value in the argument is returned unchanged | Shared coercion rules |

What happens at the overflow boundary is not documented and is not settled by the reference
engine's declared axes; see above.

## Relationships

- **`SQRT`** — the underlying operation, and the function `SQRTPI` is *not* implemented on top
  of, according to the evidence attached to this page. That is the whole story of this surface.
- **`PI`** — the constant. `SQRTPI(n)` and `SQRT(n * PI())` are the same mathematical value and
  are not guaranteed to be the same double, because they take different routes.
- **`POWER` and the `^` operator** — the general power routine, which is what the evidence
  records identify as the substrate here.
- **`GAMMA`** — `SQRTPI(1)` is `GAMMA(0.5)`, exactly. That identity is a free cross-check
  between two unrelated implementations in the same workbook.
- **`NORM.DIST`, `NORM.S.DIST`, `ERF`** — the consumers. Every one of them carries a
  `sqrt(pi)`-family constant internally, which is why this function exists.
- **Confused with**: `SQRT(PI() * number)` written by hand, which is the same thing and may not
  produce the same bits; and with `PI()` alone.

## Numerical notes

**The finding on this surface is about the routine, not the mathematics.** Both evidence records
attached to this page make the same point from different angles: the square root here is
performed by a general power routine — `(n*pi)^0.5` — rather than by a dedicated square root.

That distinction is not pedantry. IEEE 754 requires `squareRoot` to be correctly rounded; it
requires nothing of the sort from `pow`. A correctly rounded `sqrt` returns the true value
rounded once; a general `pow` typically computes `exp(0.5 * log(x))` with extended-precision
bookkeeping and lands on the correctly rounded value almost always, but not always. The two
therefore agree at ordinary magnitudes and can part company near the extremes of the exponent
range. `EV-MISC-0014` records the witness: at the input where `n*pi` reaches the largest finite
double, the correctly rounded square root and the power routine give different results, and
**the record's reading is that the power routine is the less accurate party while the reference
engine reproduces it deliberately.** `EV-MISC-0014` also rules out x87 extended precision as the
cause — both sides of that comparison are SSE2 — so this is a routine-choice effect, not a
register-width effect.

Both records carry a reader warning about attribution, and the warning is the interesting part
of the epistemology: the count that exists in the upstream source has a power routine as its
grammatical subject, not `SQRTPI` worksheet calls, and neither record is willing to republish it
as a pass rate for this function. **This page states none of those figures; the evidence layer
renders them beside it with their warnings attached.**

**What a careful implementation does.** Three things, in order of importance:

1. **Use `sqrt`, not `pow`,** if the goal is the mathematically best answer. This costs nothing
   and buys correct rounding of the root. It is also, per the records above, precisely the choice
   that would *disagree* with Excel at the extremes — which is the four-flavour distinction this
   Handbook exists to make explicit: an Excel-compatibility implementation and a
   mathematically-correct implementation genuinely differ here, and both are right about
   different questions.
2. **Guard the intermediate.** `n * pi` is one multiplication and one rounding; the product both
   overflows for large `n` and loses a rounding step that the true `sqrt(n*pi)` does not have.
   Scaling `n` down by a power of four and the result up by the matching power of two removes
   the overflow band exactly, because powers of four are exact under square root.
3. **Consider the two roundings.** Even ignoring `pow`, `SQRTPI` rounds twice — once forming
   `n*pi`, once taking the root. A correctly rounded `SQRTPI` would need the product in higher
   precision (a two-product/`fma` split is enough) before rooting. No implementation is obliged
   to do this, and the Handbook is not claiming any does; it is the difference between "the
   square root of the rounded product" and "the rounded square root of the product", and those
   are different functions in the last bit.

## What has not been checked

Two Handbook evidence records list `FUNC.SQRTPI` in their subjects: `EV-MATH-0011` and
`EV-MISC-0014`. Both are `excel-math-deviation` records about the routine that performs the
root, and **both carry explicit reader warnings that their count must not be read as a pass rate
for this function**, because the measured object in the upstream sentence is a power routine
rather than `SQRTPI` calls. `EV-MISC-0014` records as an open question whether the sampled
inputs were `SQRTPI` worksheet calls at all.

`FUNC.SQRTPI` additionally appears inside the group membership of `EV-STRUCT-0009`, a structural
sweep. That record's counting is at group scope, so **the family was measured and this surface
was not measured separately**; this page does not claim it.

No Handbook vector suite exists for `SQRTPI`. Beyond the two records above, nobody has checked
this function against Excel within the Handbook's record.

Probes worth running first:

1. **The overflow band.** The arguments just below and just above `largest finite double / pi`.
   This settles simultaneously whether the implementation scales, and whether `non_finite=allow`
   means an infinity actually reaches the sheet — the tension with `EV-MATH-0008` named above.
   It is the highest-value probe on the page.
2. **`SQRTPI(n)` against `SQRT(n * PI())`** across the exponent range. If they ever differ, the
   two routes are different routines, which is exactly what the attached records assert. This is
   a self-contained probe requiring no external oracle.
3. **`SQRTPI(1)` against `GAMMA(0.5)`** — an identity across two unrelated implementations.
4. **Perfect cases**: `SQRTPI(0)`, and arguments of the form `k^2/pi` where the answer would be
   an integer if the product were exact (it is not, which is itself the point).
5. **Negative zero and the negative boundary** — whether `-0` takes the documented `#NUM!`
   branch, and the largest negative subnormal.
6. **The reference engine's `arg_domain_guard=none`** against actual behaviour on a negative
   argument — the first divergence recorded above.
7. **Array arguments**, to confirm elementwise lifting and element-local `#NUM!`.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| power routine | A general `pow`/`powf`, not required by IEEE 754 to be correctly rounded |
| correctly rounded | The true value rounded once to the nearest double; required for `squareRoot` |
| overflow band | Arguments whose product with pi overflows although the root would be representable |
| double rounding | Rounding once at the product and again at the root, versus rounding the exact composite once |

## Sources

- Microsoft, "SQRTPI function" —
  <https://support.microsoft.com/en-us/office/sqrtpi-function-1fb4e63f-9b51-46d6-ad68-b3e7a8b519b4>
  (signature, the definition `sqrt(number * pi)`, and "If number < 0, SQRTPI returns the #NUM!
  error value").
- Handbook evidence records `EV-MATH-0011` and `EV-MISC-0014` (subjects include `FUNC.SQRTPI`)
  — the identification of a power routine rather than a dedicated square root, with their own
  attribution warnings. `EV-STRUCT-0009` includes `FUNC.SQRTPI` only in a group-scope membership
  and is therefore not claimed for this surface.
- Handbook evidence record `EV-MATH-0008` — the non-finite publication convention for a
  different set of surfaces, cited here only for the tension it creates with this surface's
  `non_finite=allow` axis.
- IEEE 754-2019 clause 5.4.1 — `squareRoot` is a correctly rounded operation; `pow` is not.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, 6.1.8 and 7.1.1 — `Gamma(1/2) =
  sqrt(pi)` and the error function's normalizing constant.
- Handbook projections `data/functions/FUNC.SQRTPI.json` (arity,
  `kernel_signature_class: Custom`, `real_result_policy` with `arg_domain_guard=none` and
  `non_finite=allow`) and `data/presence/FUNC.SQRTPI.json`.
- Handbook [SQRT](FUNC.SQRT.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
