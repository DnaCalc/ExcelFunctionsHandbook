---
schema: efh.function-page/v1
function_id: FUNC.ABS
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — ABS function"
    locator: "https://support.microsoft.com/en-us/office/abs-function-3420200f-5628-4e8c-99da-c99d7c87713c"
    role: "the canonical article; not retrieved for this pass (fetch returned HTTP 403)"
  - work: "IEEE Std 754-2019, Standard for Floating-Point Arithmetic"
    locator: "clause 5.5.1, sign-bit operations"
    role: "the statement that absolute value is a sign-bit operation, exact and never signalling"
  - work: "Higham, Accuracy and Stability of Numerical Algorithms"
    locator: "chapter 2, Sterbenz's lemma"
    role: "the band in which a floating-point subtraction is exact, which is what governs the ABS(a-b) idiom"
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
family: abs
role_in_family: >-
  The magnitude projection: the one arithmetic surface in the math category that is exact by
  construction, because it changes a sign bit and nothing else.
---

## What it computes

`ABS(number)` is the absolute value — the distance of *number* from zero on the real line:

    |x|  =   x   if x >= 0
            -x   if x <  0

- **Domain**: all real numbers.
- **Range**: the non-negative reals, `[0, +infinity)`.
- **Parity**: even. `|-x| = |x|`, and this is the defining symmetry.
- **Smoothness**: continuous everywhere, differentiable everywhere *except* at zero, where the
  left and right derivatives are `-1` and `+1`. The corner at the origin is the whole reason
  `ABS` shows up in optimisation problems as the awkward term.
- **Algebra**: `|xy| = |x||y|`, `|x/y| = |x|/|y|`, and the triangle inequality
  `|x + y| <= |x| + |y|` with equality exactly when `x` and `y` have the same sign. Absolute
  value is the standard norm on `R`; that is what makes `ABS(a - b)` the natural distance.

The identity that ties it to its sibling is `x = SIGN(x) * ABS(x)`, exact for every `x` except
that `SIGN(0)` is `0` and so contributes nothing at the origin.

In binary64 terms `ABS` is not an approximation of anything. The magnitude of a representable
number is representable, so the correctly rounded result *is* the exact result: `ABS` clears the
sign bit and leaves every other bit alone. Under IEEE 754 that makes it one of a very small set
of operations — sign-bit operations — that are exact, cannot overflow, cannot underflow, and
raise no exception on any input.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The real number whose magnitude you want. Required. | — |

One argument; the reference engine records an arity of exactly one, and classifies the surface as
a unary numeric scalar-or-array kernel, so an array in the single slot lifts elementwise. Ordinary
numeric slot under the shared coercion rules — see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

There is no commonly misunderstood position here, because there is only one. The misunderstanding
that does occur is upstream of the call: readers reach for `ABS` to make a *result* positive when
what they wanted was to make an *argument* positive, and the two differ whenever anything
non-monotonic sits in between.

## Result and edge cases

Returns `Number`.

- **Zero.** `|0| = 0`. The interesting sub-case is negative zero: IEEE 754 distinguishes `-0`
  from `+0`, and absolute value maps `-0` to `+0`. Whether a negative zero can reach an Excel
  worksheet function at all, and what it displays as if it does, is not settled on this page.
- **Subnormals.** Pass through with their magnitude unchanged; there is no underflow, because no
  arithmetic happens.
- **The largest finite double.** Passes through unchanged; there is no overflow, for the same
  reason.
- **Logicals.** Convert by the shared rule (`TRUE` to 1, `FALSE` to 0) before the kernel sees
  them.
- **Text that reads as a number.** Converts as a direct argument under the shared to-number rule;
  text in a *scanned range* is a different question and does not arise for a unary kernel.
- **Arrays.** Lift elementwise. The implementing module carries an open upstream defect stream on
  exactly this point (`BUG-FUNC-022`, named in the presence projection as an array-lift gap on the
  unary `ABS` surface), so array-shaped arguments are the unsettled part of this surface, not the
  arithmetic.

The reference engine's projected `real_result_policy` for `ABS` records `non_finite=allow` — the
kernel is permitted to hand back a non-finite value. For `ABS` specifically this is inert, since
no finite input can produce one; it matters on the pages of functions that can.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | The argument does not convert to a number | Shared coercion rules, not an `ABS`-specific condition |
| propagated | An error value in the argument is returned unchanged | Shared coercion rules |

There is no domain error and no overflow error, because `ABS` has neither a restricted domain nor
an arithmetic step. Microsoft's article for `ABS` was not retrieved for this pass, so no error
condition on this page is stated on its authority.

## Relationships

- **`SIGN`** — the other half of the polar decomposition of a real number. `SIGN(x) * ABS(x) = x`
  away from the origin.
- **`IMABS`** — the complex modulus, which is the same idea one dimension up and *not* the same
  computation: it is `hypot(a, b)`, an operation with real overflow and cancellation questions,
  where the real `ABS` has none.
- **Unary minus** — `ABS` is not "negate if negative" implemented as a comparison plus a
  negation; it is a single sign-bit clear. The distinction is invisible in results and visible in
  performance.
- **`SUMSQ`, `SUMXMY2`, `DEVSQ`** — squaring is the other standard way to discard a sign, and it
  is the one that overflows. Where a formula can use either, `ABS` has the wider range.
- **`AVEDEV`** — the mean absolute deviation, which is `ABS` under an aggregate, and the reason
  the function exists in the statistical category at all.
- **Confused with**: `INT` and `TRUNC`, which also "simplify" a number but change its magnitude;
  `ABS` never does.

## Numerical notes

`ABS` is the rare function whose numerical note is that it has no numerical error. Everything
interesting happens on either side of it.

**The comparison idiom.** `ABS(a - b) < tol` is the standard floating-point equality test, and the
error in it belongs entirely to the subtraction. Sterbenz's lemma gives the good case: if `a` and
`b` are within a factor of two of each other, `a - b` is computed exactly, so `ABS(a - b)` is the
exact distance. Outside that band the subtraction rounds, and for widely separated magnitudes it
can round to something with no correct digits at all. `ABS` faithfully reports whatever the
subtraction produced; readers who see a surprising result from this idiom are looking at the wrong
function.

**Absolute versus relative tolerance.** A fixed `tol` in the idiom above is an *absolute*
tolerance, and it is meaningful only over a narrow range of magnitudes. The relative form
`ABS(a - b) <= tol * ABS(b)` scales correctly but fails when `b` is zero. The mixed form
`ABS(a - b) <= tol * MAX(ABS(a), ABS(b), 1)` is the usual compromise, and it is worth writing out
rather than reaching for, because the choice is a modelling decision and not a numerical one.

**Why implementations should not branch.** The obvious implementation — compare against zero,
negate if negative — is correct but has two defects: it is a branch on data, and it maps `-0` to
`-0` rather than to `+0` unless the comparison is written carefully. Clearing the sign bit has
neither problem. Any implementation claiming `ABS` should be checked at `-0` for exactly this
reason.

**`ABS` as a guard.** A common pattern wraps a domain-restricted function in `ABS` to keep its
argument admissible — `SQRT(ABS(x))`, `LN(ABS(x))`. This silently changes the function being
computed. The Handbook notes it because it turns a `#NUM!` into a plausible wrong answer, which is
the worse of the two failures.

## What has not been checked

**No evidence record in the Handbook names `FUNC.ABS`**, and no Handbook vector suite exists for
it. Nobody has checked this function against Excel within the Handbook's record. The battery
rendered beside this page is the reference engine's own answers with no Excel involved, and it is
labelled as such.

Microsoft's `ABS` support article was not retrieved for this pass — the fetch returned HTTP 403 —
so no statement on this page rests on it. The mathematics above is stated from the standard
literature; the classification facts are read from the Handbook's own projections.

Inputs worth probing first, in order of how much each would settle:

1. **`ABS(-0)` and its round trip** — enter a negative zero (for example as `-1 * 0` or via a
   formula that underflows to it) and test whether the result is distinguishable from `+0` by
   `1/ABS(x)`. This is the single probe that tells you whether Excel's value model preserves the
   IEEE sign of zero at all, and the answer is load-bearing for several other pages.
2. **Array arguments in the single slot**, given the open `BUG-FUNC-022` array-lift stream on this
   module. A `2x2` inline array and a range reference are different paths and both need a probe.
3. **The largest finite double and the smallest subnormal**, to confirm that `ABS` is a pure
   pass-through at both ends rather than routing through any arithmetic.
4. **`ABS("2.5")` versus `ABS(A1)` with `"2.5"` in `A1`** — the direct-argument versus range-scan
   asymmetry described in the coercion chapter. `ABS` is a clean instrument for it, precisely
   because the arithmetic cannot confound the result.
5. **`ABS(TRUE)` and `ABS("")`** — the logical and empty-text conversions, which are shared-model
   behaviour but have never been observed in Excel for this surface.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| sign-bit operation | An operation that changes only the sign bit; exact, and never overflows or underflows |
| Sterbenz's lemma | If two positive numbers are within a factor of two, their difference is computed exactly |
| absolute tolerance | A fixed comparison threshold, meaningful only over a narrow magnitude range |
| negative zero | The IEEE 754 value `-0`, distinct in bits from `+0` and equal to it under `=` |

## Sources

- Microsoft, "ABS function" —
  <https://support.microsoft.com/en-us/office/abs-function-3420200f-5628-4e8c-99da-c99d7c87713c>
  — the canonical article. **Not retrieved for this pass** (HTTP 403), so nothing here is stated
  on its authority. The projection `data/functions/FUNC.ABS.json` carries Microsoft's English
  one-line description verbatim, and that is the only documented text this page relies on.
- IEEE Std 754-2019, clause 5.5.1 — absolute value as a sign-bit operation: exact, quiet, and
  total.
- Higham, *Accuracy and Stability of Numerical Algorithms*, chapter 2 — Sterbenz's lemma and the
  conditioning of subtraction, which is where the error in the `ABS(a - b)` idiom actually lives.
- Handbook projections `data/functions/FUNC.ABS.json` (arity, `NumToNum` kernel signature,
  `UnaryNumericScalarOrArrayElementwise` lift profile, `real_result_policy` with
  `non_finite=allow`) and `data/presence/FUNC.ABS.json` (implementing modules and the
  `BUG-FUNC-022` array-lift defect stream).
- Handbook [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
