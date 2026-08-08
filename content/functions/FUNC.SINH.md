---
schema: efh.function-page/v1
function_id: FUNC.SINH
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0008
open_problems: []
references:
  - work: "Microsoft Support — SINH function"
    locator: "https://support.microsoft.com/en-us/office/sinh-function-1e4e8b9f-2b65-43fc-ab8a-0a37f4081fa7"
    role: "documented signature and the statement that the defining formula is the hyperbolic sine"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 4 (Elementary Transcendental Functions), section 4.5"
    locator: "4.5.1-4.5.x"
    role: "definitions, series, identities and relations to the circular functions"
  - work: "Cody & Waite, Software Manual for the Elementary Functions"
    locator: "chapter on SINH/COSH"
    role: "the small-argument series/expm1 branch and the large-argument scaled-exponential branch"
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
family: sinh
role_in_family: >-
  The odd hyperbolic primary: unbounded, cancellation-prone near zero, and one of the surfaces
  named in the record that Excel publishes an error rather than an infinity.
---

## What it computes

`SINH(number)` is the hyperbolic sine:

    sinh x  =  (e^x - e^(-x)) / 2  =  x + x^3/3! + x^5/5! + x^7/7! + ...

- **Domain**: all real numbers. `sinh` is entire; no poles, no branch cuts.
- **Range**: all real numbers. Unlike its circular sibling, `sinh` is unbounded and strictly
  increasing, hence a bijection from `R` onto `R` — which is why `ASINH` is defined everywhere
  with no branch selection.
- **Parity**: odd. `sinh(-x) = -sinh(x)`.
- **Not periodic** over the reals (it has period `2*pi*i` in the complex plane, which is the
  same statement as `sin(ix) = i sinh(x)`).
- **Limiting behaviour**: `sinh x / x -> 1` as `x -> 0`, with leading correction `x^3/6`. As
  `x -> +infinity`, `sinh x ~ e^x / 2`; the term `e^(-x)/2` becomes negligible well before it
  underflows.
- **Fundamental identity**: `cosh^2 x - sinh^2 x = 1` — the hyperbola, in the same role the
  Pythagorean identity plays for the circle. Addition formula:
  `sinh(x + y) = sinh x cosh y + cosh x sinh y`.

Abramowitz & Stegun treat the hyperbolic functions in chapter 4 section 4.5; the relations
`sinh x = -i sin(ix)` and `sin x = -i sinh(ix)` are the bridge to section 4.3.

Microsoft's page gives the signature, describes the argument as "any real number", and states
that the formula for the hyperbolic sine is the one shown — the formula itself appears on that
page only as an image, so it is not quotable from the article text. The definition above is the
standard one and is stated here from the mathematical literature, not from the article.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | Any real number. Required. | — |

One argument; the reference engine records an arity of exactly one. Ordinary numeric slot under
the shared coercion rules — see [Coercion and lifting](../model/02-coercion-and-lifting.md).

There are no units here. `SINH` takes a number, not an angle, and the `RADIANS` apparatus that
attaches to `SIN` is meaningless for it. That is the position most often misunderstood: readers
arriving from `SIN` sometimes convert degrees first, which is simply wrong.

## Result and edge cases

Returns `Number`.

The reference engine classifies `SINH` as a unary numeric scalar-or-array kernel
(`UnaryNumericScalarOrArrayElementwise`), so arrays lift elementwise.

The characteristic edge is **overflow**. Because `sinh x ~ e^x/2`, the result exceeds the
binary64 range at a moderate argument — somewhere just past the point where `e^x` itself
overflows, shifted by the halving. Beyond that point the mathematical value exists and is
finite; it is only the representation that fails.

The reference engine's projected `real_result_policy` for `SINH` carries `non_finite=num`: a
non-finite result is published as `#NUM!`. This is the subject of the evidence record attached
to this page. `EV-MATH-0008` is an `excel-math-deviation` record whose statement is a kind and
error-code convention — that Excel never publishes an infinity or a NaN, and that any
non-finite real result becomes `#NUM!` — carried with named witnesses and, in the record's own
words, no row count anywhere. `FUNC.SINH` is one of its subjects. **The record's own reader
warning is that it publishes no count, so nothing about numeric agreement follows from it; what
is on record is where an error code is placed.**

**Microsoft's `SINH` page lists no error conditions.** The overflow-to-`#NUM!` behaviour is
therefore documented nowhere in the article and is on record only through the evidence record
and the reference engine's classification. The Handbook records that as a
documentation-versus-record divergence: the article describes a function of "any real number"
with no failure mode, and the record says a failure mode exists.

Small arguments round-trip: below the point where `x^3/6` is negligible against `x`, the
correctly rounded `sinh x` is `x`, so subnormals pass through unchanged.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | The mathematical result overflows binary64 | Evidence record `EV-MATH-0008` and the reference engine's `non_finite=num` — **not on Microsoft's page** |
| `#VALUE!` | The argument does not convert to a number | Shared coercion rules |
| propagated | An error value in the argument is returned unchanged | Shared coercion rules |

## Relationships

- **`COSH`** — the even primary; `COSH` and `SINH` differ by the sign in the numerator, and
  `COSH` never approaches zero, which makes it the numerically easy one.
- **`TANH`** — `SINH/COSH`, bounded in `(-1, 1)`. Computing `TANH` as that quotient is a
  mistake for large arguments (see the `TANH` page).
- **`ASINH`** — the inverse, defined on all of `R` as `ln(x + sqrt(x^2 + 1))`. That closed form
  is where the interesting cancellation in this family lives; see the `ASINH` page.
- **`SIN`** — the circular sibling, via `sinh x = -i sin(ix)`. Bounded versus unbounded is the
  practical difference and it is the whole reason their edge cases have nothing in common.
- **`EXP`** — the substrate of every reasonable implementation, and a co-subject of
  `EV-MATH-0008`.
- **Confused with**: `SIN`, by name similarity, and by readers who assume a degrees convention
  applies.

## Numerical notes

`SINH` has two hazards, at opposite ends of its domain, and one identity that fixes each.

**Near zero: cancellation.** The naive formula `(exp(x) - exp(-x))/2` subtracts two numbers that
both approach `1` as `x -> 0`. The difference is `2x` to leading order, so the subtraction
discards approximately as many significant bits as `|x|` has leading zeros in its exponent —
total loss for small `x`. Relative accuracy is destroyed exactly where the function is smoothest.

The two standard remedies:

1. **Use `expm1`.** With `u = expm1(x)` (that is, `e^x - 1` computed without the cancellation),
   `sinh x = (u + u/(1+u)) / 2`. This form is the one Cody & Waite and the modern libms use in
   the middle band, because `expm1` is itself constructed to be accurate near zero.
2. **Use the series** for very small `|x|`: `sinh x = x (1 + x^2/6 + x^4/120 + ...)`, truncated
   as soon as the next term falls below the rounding of the leading `x`.

Any implementation that uses the closed exponential form all the way down to zero has a
relative-error profile that grows without bound as the argument shrinks. This is a
straightforwardly detectable defect and it is worth probing for.

**Near the top: overflow in the intermediate.** The naive form computes `e^x` first, so it
overflows one halving *earlier* than the true result does — there is a band of arguments whose
`sinh` is representable but whose `e^x` is not. The standard remedy is the **scaled
exponential**: compute `e^(x - k*ln2) ` and apply the power of two afterwards by exponent
manipulation, so the intermediate never leaves range. `fdlibm`'s `sinh` and Cody & Waite both
do a version of this. Whether an implementation recovers that band is a clean, single-probe
test.

**Symmetry.** `sinh` is odd, so an implementation should compute on `|x|` and re-apply the sign,
which halves the branch count and guarantees exact antisymmetry. An implementation that does not
can return values for `x` and `-x` that are not exact negatives of each other — a small defect,
but one that breaks any downstream code relying on the parity identity.

**A note on what "correct" means here.** For arguments where the result overflows, an
implementation has three defensible outputs: an infinity, an overflow error, or a saturated
finite value. `EV-MATH-0008` records that Excel's convention is the error. That is a
representation policy, not a numerical claim, and the Handbook keeps the two apart.

## What has not been checked

The evidence attached to this page is `EV-MATH-0008`, which lists `FUNC.SINH` among its
subjects. That record is explicit that it publishes no count: it establishes where an error code
is placed, with named witnesses, and nothing about numeric agreement. There is therefore **no
numeric-comparison evidence for `SINH` in the Handbook's record at all**, and no Handbook vector
suite exists for it.

The Handbook has not observed `SINH` in Excel itself beyond what that record carries.

Probes worth running first:

1. **The overflow boundary.** Bisect for the exact largest argument at which Excel returns a
   number rather than `#NUM!`. Where that boundary sits relative to the overflow point of `e^x`
   tells you immediately whether the implementation uses a scaled exponential or the naive form
   — one probe, two bits of information about the algorithm.
2. **Small arguments across many decades** — `1e-1` down to the subnormal floor — compared
   against a high-precision reference. This is the cancellation probe, and it is where a naive
   implementation fails visibly.
3. **`SINH(x) + SINH(-x)`**, which must be exactly zero for every `x` if the sign is handled by
   symmetry rather than recomputed.
4. **`SINH(x)` against `(EXP(x) - EXP(-x))/2`** at small arguments — a metamorphic probe that
   distinguishes the two algorithms without an external oracle.
5. **The identity `COSH(x)^2 - SINH(x)^2 = 1`** across the range, as an internal consistency
   check on the pair.
6. **Array arguments**, to confirm elementwise lifting.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| cancellation | Loss of significant bits when two nearly equal numbers are subtracted |
| `expm1` | The primitive computing `e^x - 1` accurately for small `x` |
| scaled exponential | Computing `e^(x - k ln2)` and re-applying `2^k` by exponent arithmetic |
| non-finite policy | The reference engine's `non_finite=num`: a non-finite result publishes as `#NUM!` |

## Sources

- Microsoft, "SINH function" —
  <https://support.microsoft.com/en-us/office/sinh-function-1e4e8b9f-2b65-43fc-ab8a-0a37f4081fa7>
  (signature, "any real number", and the statement that a formula is given; the formula appears
  as an image and no error conditions are listed).
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 4 section 4.5 — hyperbolic
  functions: definitions, series, identities, and the relation to the circular functions.
- Cody & Waite, *Software Manual for the Elementary Functions* — the small-argument and
  scaled-exponential branches.
- `fdlibm` `sinh`, `expm1`; Boost.Math hyperbolic documentation — the reference implementations
  of both remedies.
- Handbook evidence record `EV-MATH-0008` (subjects include `FUNC.SINH`) — the non-finite
  publication convention, with witnesses and, in its own words, no row count.
- Handbook projections `data/functions/FUNC.SINH.json` (arity, classification,
  `real_result_policy` with `non_finite=num`) and `data/presence/FUNC.SINH.json`.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md).
