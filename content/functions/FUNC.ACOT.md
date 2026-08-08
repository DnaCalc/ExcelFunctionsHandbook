---
schema: efh.function-page/v1
function_id: FUNC.ACOT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Acot method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.acot"
    role: "documented description and the statement that the result is in radians in the range 0 to pi"
  - work: "Microsoft Support — ACOT function"
    locator: "https://support.microsoft.com/en-us/office/acot-function-dc7e5008-fe6b-402e-bdd6-2eea8383d905"
    role: "the canonical worksheet article; not retrieved for this pass (fetch returned HTTP 403)"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 4 (Elementary Transcendental Functions), section 4.4"
    locator: "4.4.1-4.4.10 and the arccotangent branch conventions"
    role: "the two competing principal-branch conventions for arccotangent and the identities relating it to arctangent"
  - work: "Kahan, Branch Cuts for Complex Elementary Functions"
    locator: null
    role: "the standard treatment of why the arccotangent branch choice is a genuine decision rather than a convention"
  - work: "Cody & Waite, Software Manual for the Elementary Functions"
    locator: "chapter on ATAN/ACOT"
    role: "the reciprocal reduction that keeps large arguments accurate"
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
family: acot
role_in_family: >-
  The continuous-branch arccotangent on all of R with range (0, pi); the member of the inverse
  circular set whose branch convention differs from the arctangent-of-reciprocal reading that most
  readers assume.
---

## What it computes

`ACOT(number)` is the inverse cotangent: the angle whose cotangent is *number*.

    acot: R -> (0, pi),    cot(acot x) = x

- **Domain**: all real numbers. Unlike `ACOS` and `ASIN`, there is no restriction — `cot` maps
  each period onto the whole real line.
- **Range**: the open interval `(0, pi)`. Microsoft's Learn reference states the range as
  0 to pi. Mathematically the endpoints are approached and never reached: `acot(x) -> 0+` as
  `x -> +infinity` and `acot(x) -> pi-` as `x -> -infinity`.
- **Monotonicity**: strictly decreasing on the whole line. `acot(-1) = 3pi/4`, `acot(0) = pi/2`,
  `acot(1) = pi/4`.
- **Reflection**: `acot(-x) = pi - acot(x)`. Antisymmetric about `(0, pi/2)`, like `ACOS`.
- **Derivative**: `d/dx acot x = -1/(1 + x^2)` — bounded, never zero, and the negative of
  `ATAN`'s. `ACOT` is smooth and well conditioned everywhere; the difficulties on this page are
  about representation, not about conditioning.
- **Asymptotics**: `acot(x) = 1/x - 1/(3x^3) + 1/(5x^5) - ...` for large `|x| > 1`; near zero,
  `acot(x) = pi/2 - x + x^3/3 - ...`.

### The branch convention, which is the whole point

There are two incompatible conventions for arccotangent in the literature, and Excel picks one:

| Convention | Range | Behaviour at `x -> 0-` |
|---|---|---|
| **Continuous** (Excel, per the Learn reference) | `(0, pi)` | continuous through zero: `acot(0) = pi/2` |
| **Odd**, i.e. `atan(1/x)` | `[-pi/2, 0) ∪ (0, pi/2]` | jumps by `pi` across zero |

On the **continuous** convention the identity is

    acot(x) = pi/2 - atan(x)      for every real x

which is smooth everywhere. On the **odd** convention `acot(x) = atan(1/x)`, which is undefined at
zero and discontinuous across it. The two agree for `x > 0` and differ by exactly `pi` for
`x < 0`.

This is not a pedantic distinction. `ACOT(-1)` is `3pi/4` on Excel's stated convention and
`-pi/4` on the other, and a formula ported from a language whose `acot` is `atan(1/x)` will
silently be wrong by `pi` on every negative input. Abramowitz & Stegun record both conventions;
Kahan's branch-cut paper is the standard argument for treating the choice as a real decision.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The cotangent of the angle you want. Required. | — |

That description is Microsoft's, from the Learn reference: "the cotangent of the angle that you
want". No constraint on the argument is documented, and none is needed.

One argument; the reference engine records an arity of exactly one, a `NumToNum` kernel signature,
and a unary numeric scalar-or-array lift profile, so arrays lift elementwise. Ordinary numeric slot
under the shared coercion rules — see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

The misunderstood position is, again, not the slot: it is that `number` is a *cotangent*, not an
angle, and that a reader who wants "the angle whose tangent is x" wants `ATAN`.

## Result and edge cases

Returns `Number` — an angle in radians.

- **Zero** returns `pi/2`. This is the case that separates the two branch conventions, and it is
  the one worth checking first in any implementation.
- **Negative arguments** return values in `(pi/2, pi)` on the documented convention — never
  negative. A negative result from `ACOT` would be evidence of the other convention.
- **Very large positive arguments** produce results approaching zero from above. Here
  representation becomes the issue: see the numerical notes. The reference engine's own battery
  shows the largest finite double producing exactly zero on this surface, which is *outside* the
  open range `(0, pi)` that the function's mathematics guarantees. Whether Excel does the same is
  not known here; the Handbook records the reference engine's behaviour as a finding and not as a
  statement about Excel.
- **Very large negative arguments** produce results approaching `pi` from below, and the
  corresponding question is whether they saturate at exactly `pi`.
- **Subnormal arguments** return `pi/2`, indistinguishably from zero.
- **Arrays** lift elementwise.

The reference engine's projected `real_result_policy` for `ACOT` records `arg_domain_guard=none`
and `non_finite=allow`; there is no domain to guard.

### Version

`ACOT` is one of the trigonometric surfaces added in Excel 2013, alongside `ACOTH`, `SEC`, `CSC`,
`COT` and their hyperbolic partners. Workbooks that must open in earlier versions substitute
`PI()/2 - ATAN(x)`, which is the same function on the documented convention.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | The argument does not convert to a number | Shared coercion rules |
| propagated | An error value in the argument is returned unchanged | Shared coercion rules |

There is no domain error, because there is no domain restriction. Microsoft's Learn reference lists
no error condition for `ACOT`; the worksheet article was not retrieved for this pass.

## Relationships

- **`ATAN`** — the identity partner. `ACOT(x) = PI()/2 - ATAN(x)` on the documented convention,
  and that is exactly how the reference engine computes it. See the numerical notes for why the
  identity is a better piece of mathematics than it is a program.
- **`COT`** — the forward function. `COT(ACOT(x)) = x` to rounding; `ACOT(COT(t))` recovers `t`
  only for `t` in `(0, pi)`.
- **`ACOTH`** — the hyperbolic namesake, and a genuinely different shape: its domain excludes
  `[-1, 1]` entirely, where `ACOT` accepts everything.
- **`ACOS`** — shares the stated range `0` to `pi`, and shares nothing else. The discriminator is
  the domain: `ACOS` takes `[-1, 1]`, `ACOT` takes all of `R`. Two surfaces with the same stated
  range are a live confusion, and this is the pair.
- **`ATAN2`** — the four-quadrant route. `ACOT(x)` is `ATAN2(x, 1)` in Excel's `x`-first argument
  order, and that formulation is better behaved at large `|x|` than the subtraction is.
- **Confused with**: `1/ATAN(x)`, which is not any inverse cotangent; and `ATAN(1/x)`, which is
  the *other* branch convention and differs by `pi` for negative arguments.

## Numerical notes

`ACOT` is analytically the tamest function on this batch of pages — bounded derivative, no
singularities, no domain edge — and it has one sharp numerical problem, which comes entirely from
how it is usually computed.

**Catastrophic cancellation in `pi/2 - atan(x)`.** The identity is exact in mathematics. In binary64
it is not: as `x` grows, `atan(x)` approaches `pi/2`, and the subtraction of two nearly equal
quantities destroys the relative accuracy of a result that is itself near zero. The failure is
progressive and total:

- The correctly rounded `atan(x)` becomes exactly the double nearest `pi/2` once `x` exceeds
  roughly `2^53` — beyond which the subtraction returns a fixed, tiny residue and then, past
  another threshold, exactly zero.
- The true `acot(x)` at those arguments is approximately `1/x`, a perfectly representable positive
  number all the way down to the subnormal floor.
- So there is a very wide band of arguments — everything above about `1e16`, which is more than
  290 decades of the double range — on which `pi/2 - atan(x)` has no correct significant digits
  at all, and eventually returns a value outside the function's own open range.

This is the mirror image of the `ATAN` situation: `ATAN` is accurate for large `x` because its
answer is large; `ACOT`'s answer is small there, and small answers reached by subtraction are
where relative accuracy goes to die.

**The standard remedy** is the reciprocal reduction, and it is one line:

    acot(x) = atan(1/x)              for x >=  1        (result in (0, pi/4])
    acot(x) = pi + atan(1/x)         for x <= -1        (result in [3pi/4, pi))
    acot(x) = pi/2 - atan(x)         for |x| < 1

Inside the unit interval the subtraction is harmless — both terms are of order one and the answer
is of order one. Outside it, `1/x` is small and `atan` of a small argument is accurate to the last
bit, so `acot(x)` inherits full relative accuracy right down to the subnormals. Cody & Waite give
this reduction for the `ATAN`/`ACOT` pair; it is the same trick, run the other way, that makes
`ATAN` accurate for large arguments.

**Cost of the remedy**: one division and one comparison. There is no accuracy trade-off, no extra
polynomial, and no branch subtlety beyond getting the `x <= -1` offset right.

**What the reference engine does.** Its `ACOT` kernel is the single expression
`pi/2 - atan(x)`, with no reciprocal branch. That is the naive form described above, and its
consequence — exactly zero at the top of the double range, where the mathematics gives a positive
subnormal — is visible in the battery rendered beside this page. The Handbook states this as an
observation about the reference engine at commit `473efa3`. It says nothing about Excel: it is
entirely possible that Excel does the same thing, and if so the reference engine is right to, but
nobody has checked.

**A self-test without an oracle.** `TAN(ACOT(x)) * x` should be `1`, and `ACOT(x) + ACOT(-x)`
should be `PI()`. Sweeping the first across many decades of `x` shows the exact point where the
subtraction stops carrying information, without needing any high-precision reference.

## What has not been checked

**No evidence record in the Handbook names `FUNC.ACOT`**, and no Handbook vector suite exists for
it. Nobody has checked this function against Excel within the Handbook's record. The presence
projection records no entries for this surface in the discrepancy catalogue, the math-deviation
catalogue, or any defect stream — the cleanest possible sheet, which here means "unexamined"
rather than "sound".

The documented statements above — the description and the `0` to `pi` range — come from
Microsoft's Learn `WorksheetFunction.Acot` reference, which was retrieved. Microsoft's worksheet
article was not retrieved for this pass (HTTP 403).

Probes worth running first:

1. **`ACOT(-1)`** — one probe that settles the branch convention. `3pi/4` confirms the documented
   continuous branch; `-pi/4` would mean the `atan(1/x)` convention and would contradict the
   documentation.
2. **`ACOT(0)`** — the point where the two conventions differ most sharply (one is `pi/2`, the
   other undefined).
3. **A logarithmic sweep of large positive `x`**, from `1e10` to the largest finite double, against
   `1/x`. This is the decisive probe for the cancellation hazard and it needs no high-precision
   oracle at the top end, because `acot(x)` and `1/x` agree to well within a double there. If Excel
   returns exactly zero anywhere in that sweep, the naive form is identified.
4. **The mirror sweep at large negative `x`**, testing whether the result saturates at exactly
   `PI()` — the other end of the same defect.
5. **`ACOT(x) + ACOT(-x) - PI()`** across the range, as the oracle-free symmetry probe.
6. **Array arguments**, to confirm elementwise lifting.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| continuous branch | The `(0, pi)` convention, on which `acot` is smooth through zero |
| odd branch | The `atan(1/x)` convention, discontinuous at zero and differing by `pi` for `x < 0` |
| reciprocal reduction | Computing `acot(x)` as `atan(1/x)` (plus `pi` if `x < 0`) for `|x| >= 1` |
| cancellation | Loss of significant bits when two nearly equal numbers are subtracted |

## Sources

- Microsoft Learn, "WorksheetFunction.Acot method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.acot> (retrieved: the
  description and the statement that the result is in radians in the range 0 to pi).
- Microsoft, "ACOT function" —
  <https://support.microsoft.com/en-us/office/acot-function-dc7e5008-fe6b-402e-bdd6-2eea8383d905>
  — the canonical worksheet article. **Not retrieved for this pass** (HTTP 403).
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 4 section 4.4 — inverse
  circular functions and the arccotangent branch conventions.
- Kahan, *Branch Cuts for Complex Elementary Functions* — why the arccotangent branch is a decision
  with consequences rather than a notational preference.
- Cody & Waite, *Software Manual for the Elementary Functions*, chapter on `ATAN`/`ACOT` — the
  reciprocal reduction.
- Handbook projections `data/functions/FUNC.ACOT.json` (arity, `NumToNum` kernel signature,
  `real_result_policy` with `arg_domain_guard=none;non_finite=allow`) and
  `data/presence/FUNC.ACOT.json` (implementing module; no discrepancy, math-deviation or defect
  entries).
- OxFunc `crates/oxfunc_core/src/functions/acot.rs` at commit `473efa3` — the kernel expression
  `pi/2 - atan(x)`, read for the substrate note above.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md).
