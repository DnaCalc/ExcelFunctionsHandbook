---
schema: efh.function-page/v1
function_id: FUNC.SIN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0004
open_problems: []
references:
  - work: "Microsoft Support — SIN function"
    locator: "https://support.microsoft.com/en-us/office/sin-function-cf0e3432-8b9e-483c-bc55-a76651c95602"
    role: "documented signature, the radians convention, and the degrees-to-radians remark"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 4 (Elementary Transcendental Functions), section 4.3"
    locator: "4.3.1-4.3.100"
    role: "defining series, identities, periodicity and the circular-function relations used here"
  - work: "Cody & Waite, Software Manual for the Elementary Functions"
    locator: "chapter on SIN/COS"
    role: "the classical two- and three-part argument reduction against a split constant"
  - work: "Payne & Hanek, Radian reduction for trigonometric functions (SIGNUM Newsletter, 1983)"
    locator: null
    role: "exact reduction of huge arguments against a many-bit representation of pi"
  - work: "Muller, Elementary Functions: Algorithms and Implementation"
    locator: "chapters on range reduction and on correctly rounded elementary functions"
    role: "the modern treatment of worst cases and of why large-argument reduction is expensive"
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
family: sin
role_in_family: >-
  The circular sine on the radian convention; one of the three trigonometric primaries from which
  the reference engine derives the reciprocal surfaces.
---

## What it computes

`SIN(number)` is the circular sine of an angle measured in **radians**.

The defining power series, entire over the whole complex plane and convergent for every real
argument, is

    sin x  =  x - x^3/3! + x^5/5! - x^7/7! + ...  =  SUM over k>=0 of (-1)^k x^(2k+1) / (2k+1)!

equivalently, in exponential form,

    sin x  =  (e^(ix) - e^(-ix)) / (2i)

- **Domain**: all real numbers. There is no argument at which the mathematical function is
  undefined, and there are no poles and no branch cuts — sine is entire.
- **Range**: the closed interval `[-1, 1]`.
- **Parity and period**: odd (`sin(-x) = -sin x`), periodic with period `2*pi`, and
  antiperiodic with period `pi` (`sin(x + pi) = -sin x`).
- **Zeros**: exactly the integer multiples of `pi`. Note the consequence for a spreadsheet:
  the only *representable* double at which `SIN` is mathematically zero is `0` itself, because
  no other multiple of `pi` is a binary64 number.
- **Limiting behaviour**: `sin x / x -> 1` as `x -> 0`; the leading error of the approximation
  `sin x ~ x` is `-x^3/6`. There is no limit as `x -> +/-infinity`; sine oscillates forever.

The identities a reader is most likely to want, in Abramowitz & Stegun's numbering region 4.3:
`sin^2 x + cos^2 x = 1`, `sin(x +/- y) = sin x cos y +/- cos x sin y`, `sin 2x = 2 sin x cos x`,
and the half-angle and product-to-sum forms that follow from them.

Microsoft's page states the calling convention in one remark: the argument is in radians, and
degrees must be converted, either by multiplying by `PI()/180` or by passing the argument
through `RADIANS`. That remark is the whole of the documented semantics.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The angle, in radians. Required. | — |

One argument, no options; the reference engine records an arity of exactly one. The slot is an
ordinary numeric slot, so logicals and text that reads as a number convert under the shared
rules — see [Coercion and lifting](../model/02-coercion-and-lifting.md).

The commonly misunderstood position is the only position: readers who type `SIN(30)` expecting
one half are asking for the sine of thirty radians, and get a legitimate answer to a different
question. Nothing in the function can detect the mistake.

## Result and edge cases

Returns `Number`.

The reference engine classifies `SIN` as a unary numeric scalar-or-array kernel
(`UnaryNumericScalarOrArrayElementwise`), so an array argument lifts elementwise and element
failures stay element-local, per the shared lifting rules. It declares no error-collapse
behaviour, which is what one expects of a function with a single scalar slot.

Two edges deserve naming:

- **Small arguments.** For arguments below the point where `x^3/6` underflows relative to `x`,
  the correctly rounded sine of `x` is `x` itself. Subnormal inputs therefore round-trip.
- **Large arguments.** This is the interesting edge and it is discussed under *Numerical notes*.
  The reference engine's projected `real_result_policy` for `SIN` carries
  `arg_domain_guard=circular_trig_overflow` and `non_finite=num` — that is, it declares a guard
  that rejects arguments too large for meaningful reduction, and maps any non-finite result to
  `#NUM!`. **Microsoft's `SIN` page documents no error condition at all.** The Handbook records
  that as a documentation-versus-reference-engine divergence rather than resolving it: the
  documentation describes a total function, the reference engine's classification describes a
  guarded one, and nobody in this record has checked which one live Excel is.

Empty, missing and error arguments follow the shared call model; see
[The value universe](../model/01-value-universe.md).

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | The argument does not convert to a number | Shared coercion rules, not the `SIN` page |
| propagated | An error value in the argument is returned unchanged | Shared coercion rules |
| `#NUM!` | Argument outside the reference engine's declared circular-trig guard | Reference-engine classification only — **not documented by Microsoft** |

Microsoft's page lists no errors for `SIN`. Every row above is either the shared call model or
the reference engine's own declaration, and the third row is the divergence named in the
previous section.

## Relationships

- **`COS` and `TAN`** — the other two primaries. `TAN = SIN/COS`; `COS(x) = SIN(x + pi/2)`.
- **`SEC`, `CSC`, `COT`** — the reciprocals. The evidence record attached to this page
  identifies them, in the reference engine, as `excel_x87_recip` of the published primaries
  rather than as independent kernels.
- **`ASIN`** — the inverse on the principal branch `[-pi/2, pi/2]`. `ASIN(SIN(x))` recovers `x`
  only inside that interval; outside it the composition folds.
- **`SINH`** — the hyperbolic sibling, related by `sin(ix) = i sinh(x)`. Bounded versus
  unbounded is the practical difference, and it is why the two have completely different
  overflow stories.
- **`RADIANS` / `DEGREES` / `PI`** — the conversion apparatus Microsoft's remark points at.
- **Confused with**: nothing in Excel, but frequently confused with *itself in degrees*. There
  is no degree-mode sine.

## Numerical notes

Everything hard about `SIN` is argument reduction. The kernel — a minimax polynomial in `x`
over roughly `[-pi/4, pi/4]` — is a solved problem and has been since Cody & Waite. The
difficulty is getting `x` into that interval.

**Why reduction is hard.** To evaluate `sin x` you need `r = x - n*(pi/2)` accurately, where
`n = round(2x/pi)`. The cancellation in that subtraction is total: when `x` is large, `x` and
`n*(pi/2)` agree in most of their leading bits, and every bit of accuracy in `r` has to come
from bits of `pi` that were not in the working precision to begin with. The number of bits of
`pi` required grows with the exponent of `x`; near the top of the binary64 range it runs into
the low thousands. This is the content of Payne & Hanek's method: keep `2/pi` to enough bits,
multiply, and discard the integer part exactly.

**The three regimes.**

1. Small `x` (below about `pi/4`): no reduction, evaluate the kernel directly.
2. Moderate `x`: Cody-Waite reduction, subtracting a two- or three-part split of `pi/2` whose
   high parts have trailing zero bits so that the products are exact.
3. Huge `x`: Payne-Hanek, or an equivalent exact reduction against a stored many-bit `2/pi`.
   `fdlibm`'s `__rem_pio2` and `__kernel_rem_pio2` are the canonical readable implementation;
   Boost.Math and the Sun-derived libms in wide use follow it.

**The philosophical trap.** For huge `x`, the "right" answer is a matter of definition. The
double `x` is an exact rational; `sin` of *that* rational is a well-defined real number, and an
exact reduction computes it. But the user's `x` was almost certainly a rounded stand-in for
something else, and the sine of the intended value is unrelated. Both positions are defensible:
some libraries reduce exactly and return the mathematically correct value for the argument they
were handed; others refuse. A library that refuses is making a usability argument, not a
numerical one, and a page like this one should say which position an implementation has taken
rather than treat one as correct.

**What the attached evidence record says about substrate.** `EV-MATH-0004` is a
substrate-identification record covering the trig six. It names, for the family, the legacy CRT
`FSIN`/`FPTAN` chain with `FPREM1` argument reduction against the x87 ROM value of `pi`, and it
carries a host-CPU microcode caveat on those instructions. The consequence a reader should take
from it is structural, not numeric: an x87-`FPREM1`-based reduction uses a fixed, finite
approximation to `pi`, so the reduction is not exact, and the error it introduces grows with
the magnitude of the argument. That is a different failure mode from a Payne-Hanek library, and
it is the mechanism by which two implementations that agree perfectly on small angles can
disagree wildly on large ones. The record explicitly rules out Cody-Waite reduction against an
extended `pi` as the substrate. **Read the record itself for what was counted and how; this page
deliberately states none of its figures.**

**The residual sensitivity.** Near a zero of sine — that is, near a multiple of `pi` — the
relative error of any implementation is dominated by the reduction, because `sin` is locally
linear through zero and the absolute error in `r` becomes the whole answer. Test vectors that
only sample away from multiples of `pi` will not see the difference between a good reduction
and a bad one.

## What has not been checked

The evidence attached to this page is `EV-MATH-0004`, which lists `FUNC.SIN` among its
subjects. It is a substrate-identification record with a reader warning about how its figures
may be read; the Handbook's rendering of it sits beside this prose, and this page does not
restate any of it.

Beyond that record: no Handbook vector suite exists for `SIN`. The Handbook has not observed
`SIN` in Excel itself, and nothing on this page is a statement that any implementation agrees
with Excel.

The probes that would settle the most, in order:

1. **The large-argument boundary.** Walk the argument upward by powers of two and find the
   exact input at which Excel stops returning a number. If a threshold exists it identifies the
   guard; if none exists, the reference engine's `circular_trig_overflow` classification is a
   divergence from Excel as well as from the documentation. This is the single most valuable
   probe on the page.
2. **Multiples of `pi`.** `SIN(PI())`, `SIN(2*PI())`, `SIN(1e6*PI())` and the neighbouring
   doubles. These expose the reduction directly, because the answer is dominated by the
   reduction error.
3. **The classic argument-reduction witnesses** — the standard large-`x` test points used in
   the libm literature, chosen so that `x mod pi/2` is tiny. If Excel and a Payne-Hanek library
   disagree anywhere, they disagree here first.
4. **`SIN(0)` and negative zero.** Whether `SIN(-0)` publishes a signed zero, and whether the
   sign survives to the sheet at all.
5. **Subnormal and tiny arguments**, to confirm the identity region where `sin x = x`.
6. **Array arguments**, to confirm elementwise lifting and element-local failures.
7. **Cross-check against `COS(PI()/2 - x)` and against `TAN`,** as a metamorphic test that does
   not require an external oracle: any implementation whose reduction is inconsistent between
   the two primaries will fail its own identities.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| argument reduction | Mapping the input into the kernel's small interval by subtracting a multiple of `pi/2` |
| Cody-Waite reduction | Reduction by subtracting a multi-part split of `pi/2` chosen so each product is exact |
| Payne-Hanek reduction | Exact reduction of huge arguments against a stored many-bit `2/pi` |
| circular-trig guard | The reference engine's declared `arg_domain_guard=circular_trig_overflow` |
| primary | One of `SIN`, `COS`, `TAN`, from which the reference engine derives the reciprocals |

## Sources

- Microsoft, "SIN function" —
  <https://support.microsoft.com/en-us/office/sin-function-cf0e3432-8b9e-483c-bc55-a76651c95602>
  (signature, the radian convention, and the degrees-conversion remark; no error conditions are
  listed there).
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 4 section 4.3 — the
  circular functions: series, identities, periodicity, zeros.
- Cody & Waite, *Software Manual for the Elementary Functions* — the classical split-constant
  argument reduction.
- Payne & Hanek, "Radian reduction for trigonometric functions", 1983 — exact reduction for
  huge arguments.
- `fdlibm` `__rem_pio2`, `__kernel_rem_pio2`, `__kernel_sin`; Boost.Math's trigonometric
  documentation — the canonical readable implementations of the three-regime scheme.
- Muller, *Elementary Functions: Algorithms and Implementation* — range reduction and worst
  cases.
- Handbook evidence record `EV-MATH-0004` (subjects include `FUNC.SIN`) — substrate
  identification for the trig six, with its own reader warning.
- Handbook projections `data/functions/FUNC.SIN.json` (arity, classification,
  `real_result_policy` with `arg_domain_guard=circular_trig_overflow` and `non_finite=num`) and
  `data/presence/FUNC.SIN.json` (implementing module).
- Handbook [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
