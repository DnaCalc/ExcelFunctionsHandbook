---
schema: efh.function-page/v1
function_id: FUNC.PERMUTATIONA
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0008
open_problems: []
references:
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 24 (combinatorial analysis)"
    role: "Ordered selection with replacement as a combinatorial primitive"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - The suffix that means something else here
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: permutationa_fn
role_in_family: >-
  Ordered selection with replacement: the power n^k, and the member of the counting group whose
  reference-engine declaration maps a non-finite result to #NUM! rather than allowing it.
---

# PERMUTATIONA

## What it computes

`PERMUTATIONA(number, number_chosen)` counts the ordered arrangements of `number_chosen`
objects drawn from `number` objects **with repetition allowed**.

Writing `n` for `number` and `k` for `number_chosen`, Microsoft states the equation directly:

    PERMUTATIONA(n, k) = n^k

The derivation is the one-line one: each of the `k` positions is filled independently from all
`n` objects, so the arrangements are the functions from a `k`-element set into an `n`-element
set, and there are `n^k` of them. Combinatorially this is the size of `Sⁿ` raised to the
sequence length — the count of words of length `k` over an alphabet of size `n`.

**Domain and range.** After truncation to integers the documented domain is `n ≥ 0`, `k ≥ 0`,
with the single excluded corner `n = 0, k > 0` (there are no words of positive length over an
empty alphabet). On that domain the value is a positive integer. Boundary identities:

    PERMUTATIONA(n, 0) = 1        for every n ≥ 0, including n = 0
    PERMUTATIONA(n, 1) = n
    PERMUTATIONA(1, k) = 1
    PERMUTATIONA(0, 0) = 1        the empty word; documented as admissible

Unlike `PERMUT`, this function has **no upper constraint linking `k` to `n`**: `k` may exceed
`n` freely, and does in the natural applications — how many four-digit PINs (`n = 10`,
`k = 4`), how many length-`k` passwords over a character set.

Growth is exponential in `k`, so the representable range is narrow: the largest finite double
is reached when `k log n` passes about `709.78` in natural logarithm, or equivalently when
`n^k` exceeds roughly `1.798 × 10^308`. In practice the overflow boundary arrives at modest
inputs, and how it is reported is a real part of this function's behaviour rather than an
afterthought.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The size of the pool the objects are drawn from. Required. | — |
| `number_chosen` | The length of the arrangement. Required. | — |

Microsoft describes both as integers and states that **both arguments are truncated to
integers**. Truncation happens before validation, so `PERMUTATIONA(2.9, 3.9)` is the same call
as `PERMUTATIONA(2, 3)`.

The reference engine records an arity of exactly 2, a `NumsToNum` kernel signature, and a
`UnaryNumericScalarOnly` coercion/lift profile — a scalar-only numeric contract that does not
advertise elementwise lifting over array arguments.

## Result and edge cases

Returns `Number`.

- **`k = 0`** returns `1` for every admissible `n`, including `n = 0`. Microsoft's own remark
  makes this explicit by scoping the error to the case where "the total number is zero (0) and
  the chosen number is larger than zero (0)" — the corner is excluded, not the whole `n = 0`
  column.
- **`k > n` is ordinary**, not an error. This is the sharpest behavioural difference from
  `PERMUT`, where `number < number_chosen` is a documented `#NUM!`.
- **Overflow.** `n^k` leaves the representable range quickly. The Handbook's evidence record
  `EV-MATH-0008` records the Excel convention that a non-finite real result is published as
  `#NUM!` rather than as an infinity, and it names `PERMUTATIONA` among its subjects with a
  named witness. That record publishes no count and its own reader warning says so; it is a
  statement about where an error code is placed, with witnesses and no denominator. Consistent
  with it, the reference engine's `real_result_policy` for this surface reads `non_finite=num`
  — the finite-publication policy — where its sibling `PERMUT` reads `non_finite=allow`.
- **Negative arguments.** Microsoft's remarks do not spell out a negative-input rule for this
  function; the reference engine's battery renders the both-negative case and its outcome shows
  beside this page. Whether Excel rejects a negative `n` with a positive `k`, and whether it
  rejects a negative `k`, is not settled here.

## Errors

As documented by Microsoft on the `PERMUTATIONA` page (retrieved for this page):

| Error | Documented condition |
|---|---|
| `#NUM!` | The numeric arguments are not valid — the example given is `number = 0` with `number_chosen > 0` |
| `#VALUE!` | An argument uses a nonnumeric data type |

The `#NUM!` row is documented by example rather than by a complete inequality, which is a real
gap: the page states one invalid combination and says "for example". The negative-argument
cases are therefore documented only by implication.

Error values arriving in either argument propagate under the ordinary coercion rules; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## The suffix that means something else here

`PERMUTATIONA` looks like it belongs to the `AVERAGEA` / `COUNTA` / `MAXA` / `MINA` /
`STDEVA` / `VARA` group, where a trailing `A` marks a variant that **admits text and logical
values** into a scan that the base function ignores. It does not. Here — and on `COMBINA` —
the `A` marks **repetition allowed**. Two unrelated naming conventions share one letter, and
this is the pair that catches people, because `PERMUTATIONA` is a statistical-category function
sitting alphabetically beside genuine `A`-variants.

Nothing about `PERMUTATIONA`'s argument admission differs from `PERMUT`'s on account of the
suffix. Both take two numeric scalars.

## Relationships

- **[PERMUT](FUNC.PERMUT.md)** is the same count **without** repetition — the falling
  factorial `n(n-1)…(n-k+1)` — and requires `k ≤ n`. The two agree at `k = 0` and `k = 1` and
  diverge thereafter, with `PERMUTATIONA(n, k) ≥ PERMUT(n, k)` always.
- **`COMBIN`** and **`COMBINA`** are the unordered counterparts, completing the standard
  two-by-two grid: ordered/unordered against with/without replacement. `COMBINA(n, k)` counts
  multisets and equals `COMBIN(n+k-1, k)`.
- **`POWER`** and the `^` operator compute the same mathematical quantity for integer `n` and
  `k`. This is not a decorative observation: `PERMUTATIONA(n, k)` and `POWER(n, k)` are two
  Excel surfaces for `n^k`, and `EV-MATH-0008` names `POWER`, the `^` operator and
  `PERMUTATIONA` together as subjects of the same finite-publication convention — while also
  recording that `POWER` and `^` carry an *extra* rule the others do not, mapping an infinity
  produced by a negative exponent over a sub-unit base to `#DIV/0!` instead of `#NUM!`.
  Whether `PERMUTATIONA` and `POWER` return the same bits for the same integer inputs is not
  established, and is a natural probe.
- **`FACT`**, **`FACTDOUBLE`** and **`MULTINOMIAL`** are the neighbouring counting functions.
- A documentation contrast worth noticing: `PERMUT`'s page states `number ≤ 0` as a `#NUM!`
  condition, while `PERMUTATIONA`'s page explicitly admits `number = 0` when
  `number_chosen = 0`. Two adjacent pages describing the same corner of the same combinatorial
  grid give incompatible boundary conventions. The [PERMUT](FUNC.PERMUT.md) page records that
  divergence in full, because there the reference engine takes the other side.

## Numerical notes

The mathematics is a power of an integer, and the numerics is entirely about *how* that power
is formed.

1. **`pow` and repeated multiplication are different functions in floating point.** For
   integer `k`, `exp(k · log n)` — what a general `pow` does internally, in one form or another
   — accumulates the error of a logarithm and an exponential, amplified by `k`. Repeated
   multiplication accumulates `k-1` roundings but never leaves the integers while the result is
   below `2^53`. Binary exponentiation (square-and-multiply) is a third answer again: it uses
   about `log₂ k` multiplications and rounds at different places. For results below `2^53` all
   three agree, because the exact integer is representable; above it they generally do not.
   An implementation that quietly delegates to the platform `pow` has chosen one of these and
   should say so.
2. **The exact regime is worth detecting.** `n^k < 2^53` covers most real spreadsheet uses, and
   in that regime the correct answer is an exact integer that any careful staging reaches.
   Getting the boundary right — and returning the exactly rounded value just past it — is more
   valuable than shaving a multiply.
3. **Overflow must be detected before it happens, not after.** Computing `n^k` and then testing
   the result for infinity works on a platform with IEEE default rounding and no traps, but it
   throws away the information needed to distinguish "genuinely too large" from "intermediate
   overflowed while the final result would have been fine" — a distinction that matters for
   `PERMUT` and does not arise here, since `n^k` is monotone in both arguments. The cheap
   pre-test is `k · log₂ n > 1024`.
4. **Underflow does not arise**: with `n ≥ 1` the result is at least 1, and `n = 0` is either
   the empty-word corner or the documented error.

The Handbook does not assert what Excel does internally for this surface. `EV-MATH-0008` tells
us where Excel places an error code, with named witnesses; it does not identify an op-graph,
and it publishes no numeric comparison for this surface.

## What has not been checked

The evidence that exists for this surface is `EV-MATH-0008`, and it is narrow by its own
statement: it records the finite-publication convention — no infinity, no NaN, `#NUM!` instead
— with named witnesses across nine surfaces, `PERMUTATIONA` among them, and **no count of any
kind**. Its reader warning says explicitly that nothing in it should be read as a numeric-bits
comparison for these surfaces. There is no Handbook vector suite for `PERMUTATIONA`, and no
identification of how Excel forms the power.

Microsoft's documented behaviour above was retrieved from the `PERMUTATIONA` page. The
negative-argument rules are not stated there.

Inputs worth probing first:

1. **`PERMUTATIONA(0, 0)` and `PERMUTATIONA(0, 1)`** — the documented corner, one on each
   side. These confirm that the exclusion is the corner and not the whole `n = 0` column, and
   they are the two cells that make the contrast with `PERMUT`'s `number ≤ 0` wording concrete.
2. **`PERMUTATIONA(n, k)` against `POWER(n, k)` and `n^k`** over a spread of integer inputs
   crossing `2^53` — three Excel surfaces for one mathematical value. Any disagreement is a
   direct read of a staging difference, and it is the cheapest available fingerprint of how the
   power is formed.
3. **The overflow boundary**, walking `k` upward at fixed `n` until the true value passes the
   largest finite double, to confirm the `#NUM!` placement that `EV-MATH-0008` records and to
   find the exact last succeeding `k`.
4. **Negative and non-integer arguments**: `PERMUTATIONA(-2, 3)`, `PERMUTATIONA(2, -3)`,
   `PERMUTATIONA(2.9, 3.9)` and `PERMUTATIONA(-0.5, 0)`, which probe both the undocumented
   sign rules and truncation-before-validation.
5. **`PERMUTATIONA(1, k)` for very large `k`** — a value that is always `1` but whose naive
   `exp(k · log 1)` staging has to get `log 1 = 0` and `exp(0) = 1` exactly to say so. It is a
   one-cell test for whether a logarithmic staging is in use.
6. **An array argument in either position**, given the scalar-only coercion profile recorded.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| with repetition | Each object may be chosen more than once; the `A` suffix's meaning here |
| empty word | The `k = 0` arrangement, counted as one for every `n` |
| finite-publication convention | Excel publishing `#NUM!` where a real result would be non-finite |
| `non_finite=num` | The reference engine's declared policy mapping a non-finite result to `#NUM!` |
| exact regime | Results below `2^53`, where the integer answer is representable exactly |

## Sources

- Microsoft, "PERMUTATIONA function" —
  <https://support.microsoft.com/en-us/office/permutationa-function-6c7d7fdc-d657-44e6-aa19-2857b25cae4e>
  (syntax, the `n^k` equation, the integer-truncation rule, the `#NUM!` example condition, the
  `#VALUE!` condition, and version availability). Retrieved for this page.
- Handbook evidence record `EV-MATH-0008` — the finite-publication convention (`XMD-008`),
  which names `PERMUTATIONA` as a subject, carries named witnesses, and publishes no count.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 24 — combinatorial
  analysis.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md).
- `data/functions/FUNC.PERMUTATIONA.json` — arity 2, `NumsToNum`, `UnaryNumericScalarOnly`
  coercion profile, `real_result_policy … non_finite=num`, XLL symbol `xlfPermutationa`.
- `data/presence/FUNC.PERMUTATIONA.json` — implementing module
  `crates/oxfunc_core/src/functions/permutationa_fn.rs`, shared with no other surface.
