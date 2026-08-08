---
schema: efh.function-page/v1
function_id: FUNC.SIGN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — SIGN function"
    locator: "https://support.microsoft.com/en-us/office/sign-function-109c932d-fcdc-4023-91f1-2dd0e916a1d8"
    role: "documented signature, the three-valued result, and the worked examples"
  - work: "IEEE 754-2019, Standard for Floating-Point Arithmetic"
    locator: "clauses on signed zero and on the copySign/isSignMinus operations"
    role: "why signum and sign-bit extraction are different functions, and where they differ"
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
family: sign_fn
role_in_family: >-
  The sole member: the three-valued signum, and the category's cleanest example of a function
  whose only difficulty is the boundary case at zero.
---

## What it computes

`SIGN(number)` is the **signum** function:

    sgn(x) =  1  for x > 0
              0  for x = 0
             -1  for x < 0

Microsoft's page states exactly that: it "Returns 1 if the number is positive, zero (0) if the
number is 0, and -1 if the number is negative", with `=SIGN(10)` giving 1, `=SIGN(4-4)` giving
0, and `=SIGN(-0.00001)` giving -1.

- **Domain**: all real numbers; the documentation says "any real number".
- **Range**: the three-element set `{-1, 0, 1}`.
- **Parity**: odd, `sgn(-x) = -sgn(x)`.
- **Structure**: `sgn` is the derivative of `|x|` away from the origin, and `x = sgn(x) * |x|`
  is the polar decomposition of a real number. `sgn` is discontinuous at `0` — the only
  discontinuity, a jump of size 2 — and that single point is where every implementation question
  lives.
- **Identities worth having**: `SIGN(x) * ABS(x) = x`; `SIGN(x)^2` is `1` except at zero;
  `ABS(SIGN(a) - SIGN(b))` is a sign-change detector, which is the classic bisection guard.

The three-valued convention is worth naming explicitly because it is not universal. Some
languages define `sign` as two-valued (`±1`, with zero going to `+1`), and hardware sign-bit
extraction is two-valued by construction. Excel's documented function is the three-valued
mathematical signum.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | Any real number. Required. | — |

One argument; the reference engine records an arity of exactly one, an ordinary numeric slot
under the shared coercion rules — see
[Coercion and lifting](../model/02-coercion-and-lifting.md). Logicals convert (`TRUE` is 1) and
text that reads as a number converts.

There is no tolerance argument and no epsilon. `SIGN` compares against exact zero, which is the
source of essentially every surprise it produces (see *Numerical notes*).

## Result and edge cases

Returns `Number`, one of `-1`, `0`, `1`.

- **Zero.** The documented result is `0`. The interesting question is **negative zero**: IEEE
  754 has two zeros, `+0` and `-0`, which compare equal to each other and to nothing else. Under
  a comparison-based signum, `SIGN(-0)` is `0`; under a sign-bit-based one it would be `-1`.
  Microsoft's page does not address the case. The Handbook does not assert an answer, and this
  is the first probe listed below. Producing a `-0` argument in a worksheet is easy enough —
  `-1 * 0`, or `0 * -1`, or a formula that underflows from below.
- **Subnormals.** A subnormal is a nonzero number, so `SIGN` of one is `±1`. The documented
  example `SIGN(-0.00001)` makes the general point: magnitude is irrelevant, only the sign
  matters.
- **Non-finite inputs.** The reference engine's projected `real_result_policy` for `SIGN` reads
  `arg_domain_guard=none` with `non_finite=allow`. Since `SIGN`'s output is always one of three
  small integers, `non_finite=allow` cannot describe the output; it describes the policy slot
  being unused. An infinite *input* is not reachable from a worksheet cell, so the case is
  theoretical on this surface.

The reference engine classifies `SIGN` as a unary numeric scalar-or-array kernel
(`UnaryNumericScalarOrArrayElementwise`) with `kernel_signature_class: NumToNum` and
`error_collapse_profile: None`, so arrays lift elementwise and element failures stay
element-local.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | The argument does not convert to a number | Shared coercion rules |
| propagated | An error value in the argument is returned unchanged | Shared coercion rules |

Microsoft's page lists no error conditions for `SIGN`, and no domain restriction exists — the
function is total on the reals.

## Relationships

- **`ABS`** — the other half of the decomposition `x = SIGN(x) * ABS(x)`. The two are almost
  always used together.
- **`IF(x>0, 1, IF(x<0, -1, 0))`** — the hand-written equivalent, and worth mentioning because
  it is the form readers write when they have forgotten `SIGN` exists. It also makes the
  comparison semantics explicit, which `SIGN` hides.
- **`GESTEP` and `DELTA`** (engineering category) — the other threshold functions. `DELTA(a, b)`
  is an equality indicator and `GESTEP(x, step)` a step function; between them and `SIGN` there
  are three different conventions for what happens exactly at the boundary, and they do not
  agree with each other.
- **`TANH`** — the smooth saturating analogue. Where `SIGN` is used to steer a computation,
  `TANH(k*x)` for large `k` is the differentiable stand-in, which is why it turns up in
  optimization work.
- **`TRUNC`, `INT`, `ROUND`** — the rounding family shares `SIGN`'s sensitivity to which side of
  zero a value falls on; `TRUNC` in particular is "round toward zero", i.e. rounding in the
  direction opposite to `SIGN`.
- **Confused with**: `ABS` (magnitude, not direction) and with a two-valued sign, which is what
  most programming languages' bit-level helpers return.

## Numerical notes

`SIGN` performs no arithmetic, so it has no rounding error of its own. Everything that goes
wrong with it goes wrong upstream, and there are three recurring patterns worth naming.

**1. The zero case is almost never the zero you mean.** `SIGN(0.1 + 0.2 - 0.3)` is not `0`,
because `0.1 + 0.2 - 0.3` is not `0` in binary64. Any workflow that uses `SIGN` to test "did
this quantity come out to zero" is testing an exact bit condition and will get `±1` on values
that are zero to any reasonable tolerance. The remedy is not to fix `SIGN`; it is to test
`ABS(x) < tol` explicitly, so that the tolerance is visible in the formula rather than implied.

**2. `SIGN` is a discontinuity amplifier.** Its condition number is infinite at the origin: an
arbitrarily small perturbation of the input changes the output by 1 or 2. That is inherent to
the mathematics, not a defect, but it means `SIGN` should never sit downstream of a long
accumulation whose last bits are noise. The classic failure is a sign-change test on the
residual of an iterative calculation, where the residual's own error exceeds its value near
convergence — the bisection loop then wanders because `SIGN` is reporting the sign of the
rounding error.

**3. Sign-bit extraction and signum are different functions.** They agree everywhere except at
zero, where signum gives `0` for both zeros and sign-bit extraction gives `+1`/`-1`
respectively. An implementation written with `copysign` or a bit mask gets the negative-zero
case wrong relative to the documented three-valued definition; an implementation written with
two comparisons gets it right. Since the difference shows up on exactly one input class, it
survives casual testing indefinitely. This is the reason the negative-zero probe below is worth
running even though the case looks exotic.

**What a careful implementation does**: two comparisons against literal zero, in the order
`x > 0`, then `x < 0`, then `0`. Under IEEE comparison semantics this handles both zeros
correctly and needs no special case. It is three lines and there is no cleverer version worth
having.

## What has not been checked

No Handbook evidence record lists `FUNC.SIGN` in its subjects, and no Handbook vector suite
exists for `SIGN`. **Nobody has checked this function against Excel within the Handbook's
record.** The three-valued definition above is documented by Microsoft; everything else on this
page is mathematics, the shared call model, or the reference engine's own classification.

Probes worth running first:

1. **Negative zero.** `SIGN(-1*0)`, `SIGN(0*-1)`, and a `-0` produced by underflow from below.
   This is the one input that distinguishes a comparison-based implementation from a
   sign-bit-based one, and the documentation is silent on it. Highest value per probe on this
   page by a wide margin.
2. **Subnormal inputs of both signs**, to confirm that magnitude plays no part.
3. **Text and logical arguments** — `SIGN("−5")` with a Unicode minus, `SIGN("1e-400")` (which
   underflows on parse), `SIGN(TRUE)`, `SIGN(FALSE)`. The parse boundary is where the shared
   coercion rules meet this function, and the underflowing-text case is the one that could
   plausibly disagree between implementations.
4. **An empty referenced cell** versus an omitted argument, to confirm the Empty/Missing split
   documented in the call model behaves here as elsewhere.
5. **Array arguments**, to confirm elementwise lifting and element-local `#VALUE!`.
6. **`SIGN(x) * ABS(x) = x`** as a metamorphic identity across a broad sample — cheap, needs no
   oracle, and would catch a two-valued implementation immediately at zero.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| signum | The three-valued sign function returning `-1`, `0` or `1` |
| negative zero | The IEEE 754 value `-0`, which compares equal to `+0` but carries a sign bit |
| sign-bit extraction | A two-valued alternative that reads the sign bit and never returns zero |
| discontinuity amplifier | A function whose output changes by a fixed amount under an arbitrarily small input change |

## Sources

- Microsoft, "SIGN function" —
  <https://support.microsoft.com/en-us/office/sign-function-109c932d-fcdc-4023-91f1-2dd0e916a1d8>
  (signature, "any real number", the three-valued result, and the examples `SIGN(10)`,
  `SIGN(4-4)`, `SIGN(-0.00001)`; no error conditions are listed).
- IEEE 754-2019 — signed zero, comparison semantics, and the distinction between `isSignMinus`
  and a mathematical signum.
- Handbook projections `data/functions/FUNC.SIGN.json` (arity,
  `kernel_signature_class: NumToNum`, `error_collapse_profile: None`, `real_result_policy` with
  `arg_domain_guard=none` and `non_finite=allow`) and `data/presence/FUNC.SIGN.json`.
- Handbook [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
