---
schema: efh.function-page/v1
function_id: FUNC.PERMUT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0019
open_problems: []
references:
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 24 (combinatorial analysis), 24.1.2"
    role: "The falling factorial and the permutation count as a combinatorial primitive"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - A documented boundary the reference engine does not honour
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: permut_fn
role_in_family: >-
  The ordered-selection-without-replacement counter; the falling factorial, and the member whose
  Excel op-graph has been identified as an ascending extended-precision product rather than a
  factorial ratio.
---

# PERMUT

## What it computes

`PERMUT(number, number_chosen)` returns the number of ordered arrangements of `number_chosen`
distinct objects drawn without replacement from `number` objects.

Writing `n` for `number` and `k` for `number_chosen`, this is the **falling factorial**:

    P(n, k) = n! / (n - k)!
            = n · (n-1) · (n-2) · … · (n-k+1)        (k factors)

The two right-hand sides are the same integer and are *not* the same computation; the whole of
the Numerical notes section below is about that difference.

**Domain and range.** After truncation to integers the documented domain is `n ≥ 1`, `k ≥ 0`,
`k ≤ n`. On that domain `P(n, k)` is a positive integer, non-decreasing in `n` and, for fixed
`n`, non-decreasing in `k` up to `k = n`. Boundary identities:

    P(n, 0) = 1          the empty arrangement
    P(n, 1) = n
    P(n, n) = n!         = FACT(n)
    P(n, n-1) = n!       the last factor is 1, so k = n-1 and k = n agree

The last of those is the cheapest self-consistency probe there is: two different calls that
must return the same integer.

`P(n, k)` is exact in binary64 precisely while it is below `2^53`. Beyond that the true value
is an integer that a double cannot name, and every implementation is returning a rounded
answer — which is why the *order* of the multiplications becomes observable.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The size of the pool. Required. | — |
| `number_chosen` | How many are arranged. Required. | — |

Microsoft describes both as integers and states that **both arguments are truncated to
integers**, so `PERMUT(5.9, 2.9)` is the same call as `PERMUT(5, 2)`. Truncation is toward
zero, which matters only for negative inputs, and negative inputs are a documented `#NUM!`
condition anyway.

The reference engine records an arity of exactly 2, a `NumsToNum` kernel signature, and a
`UnaryNumericScalarOnly` coercion/lift profile — that last value is the projection's word for
a scalar-only numeric contract, and it is worth noting that it does **not** advertise
elementwise lifting over arrays. Whether Excel lifts `PERMUT` over an array argument is
therefore an open question here, not an assumption.

## Result and edge cases

Returns `Number` — an integer value where the integer is representable, and a rounded double
where it is not.

- **Non-integer arguments** are truncated before any validation, as documented.
- **`k = 0`** yields the empty arrangement, `1`, for every admissible `n`.
- **`k = n`** yields `n!` and therefore inherits the factorial's overflow wall: the largest
  representable factorial in binary64 is `170!`, so `PERMUT(n, n)` cannot succeed for
  `n ≥ 171`. But `PERMUT` is *not* limited to `n ≤ 170` in general — `PERMUT(1000, 3)` is a
  small number — and an implementation that computes `n!/(n-k)!` inherits a limit the function
  does not have. That is not hypothetical: it is one of the stagings the evidence record named
  below explicitly rules out.
- **Overflow.** Where the true result exceeds the largest finite double, Excel's convention
  across the mathematical surface is to publish `#NUM!` rather than an infinity. The Handbook
  records that convention as a general finding for a named group of functions in
  `EV-MATH-0008`; `PERMUT` is *not* among that record's subjects, and the reference engine's
  `real_result_policy` for `PERMUT` reads `non_finite=allow` rather than the `non_finite=num`
  its sibling `PERMUTATIONA` carries. The two surfaces are declared differently. Whether Excel
  distinguishes them is unchecked, and it is on the probe list.

## Errors

As documented by Microsoft on the `PERMUT` page (retrieved for this page):

| Error | Documented condition |
|---|---|
| `#VALUE!` | `number` or `number_chosen` is nonnumeric |
| `#NUM!` | `number ≤ 0`, or `number_chosen < 0` |
| `#NUM!` | `number < number_chosen` |

Error values arriving in either argument propagate under the ordinary coercion rules; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## A documented boundary the reference engine does not honour

Microsoft's stated condition is `number ≤ 0`, not `number < 0`. Read literally, `PERMUT(0, 0)`
is a `#NUM!` — even though the mathematics is unambiguous that there is exactly one
arrangement of nothing chosen from nothing, and even though `PERMUT(n, 0) = 1` holds for every
other `n`.

The reference engine's battery row for `PERMUT(0, 0)` records a **numeric** outcome, not an
error. The value itself renders in the battery panel beside this page; what matters here is
the kind. Documentation says error; the reference engine says number.

The Handbook publishes this as a divergence rather than picking a side, because both readings
are defensible and neither has been checked against Excel here:

- Microsoft's inequality may be a documentation infelicity — `n ≤ 0` written where `n < 0` was
  meant — inherited from a `COMBIN`-style page where the same wording appears.
- Or Excel may genuinely reject `n = 0`, in which case the reference engine has an off-by-one
  at the boundary and `PERMUT(0, 0)` is a one-cell reproduction.

`PERMUT(0, 0)` is therefore the single highest-value probe on this page: one cell decides
which of two published descriptions of Excel is wrong.

## Relationships

- **[PERMUTATIONA](FUNC.PERMUTATIONA.md)** is ordered selection **with** replacement: `n^k`
  rather than the falling factorial. Note the naming trap — the `A` suffix here does not mean
  what it means in `AVERAGEA`, `COUNTA`, `MAXA` and `STDEVA`, where it marks a variant that
  admits text and logicals. On `PERMUTATIONA` and `COMBINA` the suffix marks *repetition
  allowed*. Two unrelated conventions share one letter.
- **`COMBIN`** is the unordered counterpart: `COMBIN(n, k) = PERMUT(n, k) / FACT(k)`. That
  identity is exact in the integers and only approximately reproducible in floating point,
  which makes it a useful residual probe rather than a substitute computation.
- **`COMBINA`** completes the two-by-two grid: ordered/unordered against
  with/without replacement. `PERMUT` (ordered, without), `PERMUTATIONA` (ordered, with),
  `COMBIN` (unordered, without), `COMBINA` (unordered, with).
- **`FACT`** is the `k = n` diagonal, and `FACTDOUBLE` is unrelated despite the neighbouring
  name.
- **Confused with**: `COMBIN`, constantly. If order matters — podium finishes, PINs,
  arrangements — it is `PERMUT`; if it does not — hands of cards, committees — it is `COMBIN`.

## Numerical notes

`PERMUT` looks like a function with no numerical content and is in fact a small case study in
how staging changes answers.

**The factorial-ratio staging is wrong twice over.** Computing `n!` and `(n-k)!` and dividing
loses in two independent ways. First, it manufactures an overflow the function does not have:
`n!` is not representable for `n ≥ 171`, so a perfectly small `PERMUT(200, 2)` fails.
Second, each factorial is itself a rounded double once past `2^53`, and the quotient of two
rounded values is not the rounded quotient — the two errors do not cancel. The evidence record
attached to this page names exactly this staging as ruled out, reporting it as off by a
recorded residual on the catalogue's own witness and as spuriously overflowing beyond `n = 170`.

**The product staging is the right shape, and its details are still observable.** Multiplying
the `k` factors accumulates one rounding per multiply, so the answer depends on:

1. **Direction.** Ascending (`n-k+1` upward to `n`) and descending (`n` downward to `n-k+1`)
   are different sequences of roundings and give different last bits once the accumulator
   passes `2^53`. Neither is universally better; the accurate-summation intuition (combine
   small terms first) does not transfer cleanly to products.
2. **Working precision.** A product accumulated in 80-bit extended precision and rounded once
   per step to 64 bits is a third answer again, distinct from a pure-double accumulation and
   from a fully extended accumulation rounded once at the end. On x86 this is not an exotic
   choice — it is what a legacy x87 code path does by default.

The evidence record `EV-MATH-0019` reports OxFunc's identification of Excel's `PERMUT` as an
**ascending extended-precision spill-loop product** — each step forming the product in the
wider format and storing it back to a double — and records that five of six raced stagings
were eliminated. That is an upstream identification of Excel's op-graph, on a corpus described
in the record; the Handbook has not re-run it, and the record's own reader warning about how
its figures may be read travels with it.

**What a careful independent implementation does.** For results below `2^53` the honest answer
is exact and any staging that avoids intermediate overflow will produce it; the correct
strategy is to detect that regime and return the exact integer. Above it, the choice is
between reproducing a target platform's staging (the `excel-bitexact` flavour's job) and
computing the correctly rounded value (the `math-correct` flavour's job) — for which the
standard route is `exp(lgamma(n+1) - lgamma(n-k+1))` with a corrective term, or an
extended-precision product with a final single rounding. Those two answers differ, and the
Handbook's position is that both are right for their stated purpose. See
[About implementation options](../model/07-implementation-options.md).

## What has not been checked

Evidence exists for this surface and it is narrow. `EV-MATH-0019` is a substrate
identification: it names an op-graph for Excel's `PERMUT`, names the rival stagings ruled out,
and carries its own scope and reader warning, which render mechanically beside this page. It
is an upstream measurement that the Handbook has not re-verified, and it is not a vector suite.
No Handbook vector suite exists for `PERMUT`.

The documented error conditions above were retrieved from Microsoft's page. The `PERMUT(0, 0)`
divergence recorded above is unresolved: nobody has checked which behaviour Excel actually has.

Inputs worth probing first:

1. **`PERMUT(0, 0)`.** One cell resolves a published contradiction between Microsoft's
   documentation and the reference engine. Nothing else on this page is as cheap or as
   decisive.
2. **`PERMUT(n, n)` against `PERMUT(n, n-1)` and `FACT(n)`**, at `n` just below and just above
   the point where the exact integer stops being representable. Three expressions that must
   agree mathematically; where they stop agreeing is a direct read of the staging.
3. **`PERMUT(200, 2)` and `PERMUT(1000, 3)`** — small answers from large pools, the inputs
   that separate a product staging from a factorial-ratio staging with no ambiguity.
4. **The overflow boundary**, walking `k` upward at fixed large `n` until the result exceeds
   the largest finite double, to see whether Excel publishes `#NUM!` there. The reference
   engine declares `non_finite=allow` for `PERMUT` and `non_finite=num` for `PERMUTATIONA`;
   if Excel treats them alike, one of those declarations is wrong.
5. **Truncation at the boundary**: `PERMUT(5.9, 5.9)`, `PERMUT(0.9, 0)` and `PERMUT(-0.5, 0)`,
   which probe truncation-before-validation and the sign of the truncation.
6. **An array argument in either position**, given that the recorded coercion profile does not
   advertise elementwise lifting.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| falling factorial | `n(n-1)…(n-k+1)`, the product form of `P(n, k)` |
| factorial-ratio staging | Computing `n!/(n-k)!`; ruled out upstream for this surface |
| spill-loop product | A product accumulated in a wider format and stored back each step |
| exact regime | Results below `2^53`, where the integer answer is representable exactly |
| `non_finite=allow` / `non_finite=num` | The reference engine's declared policy for a non-finite result |

## Sources

- Microsoft, "PERMUT function" —
  <https://support.microsoft.com/en-us/office/permut-function-3bd1cb9a-2880-41ab-a197-f246a7a602d3>
  (syntax, the integer-truncation rule, the `n!/(n-k)!` equation, and the four documented
  error conditions including `number ≤ 0`). Retrieved for this page.
- Handbook evidence record `EV-MATH-0019` — OxFunc's substrate identification for `PERMUT`,
  with its scope, corpus description and reader warning as recorded there.
- Handbook evidence record `EV-MATH-0008` — the "Excel never publishes an infinity" convention,
  cited here for context only; `PERMUT` is not among its subjects and no claim about `PERMUT`
  is drawn from it.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 24 — combinatorial
  analysis; the falling factorial and its identities.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [About implementation options](../model/07-implementation-options.md).
- `data/functions/FUNC.PERMUT.json` — arity 2, `NumsToNum`, `UnaryNumericScalarOnly` coercion
  profile, `real_result_policy … non_finite=allow`, XLL symbol `xlfPermut`.
- `data/presence/FUNC.PERMUT.json` — implementing module
  `crates/oxfunc_core/src/functions/permut_fn.rs`, shared with no other surface.
