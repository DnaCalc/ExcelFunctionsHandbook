---
schema: efh.function-page/v1
function_id: FUNC.ODD
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
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
family: odd_fn
role_in_family: >-
  Sole member of its module; rounds away from zero to an odd integer, and is one of the few
  Excel functions whose defining postcondition provably fails on part of its own domain.
---

## What it computes

`ODD(number)` rounds *number* **away from zero** to the nearest odd integer.

Microsoft's two remarks give the rule completely:

> "Regardless of the sign of number, a value is rounded up when adjusted away from zero."
>
> "If number is an odd integer, no rounding occurs."

So the direction is outward from zero in both half-planes — 1.5 rises to 3, and −2 falls to −3 —
and an argument already on an odd integer is returned unchanged. The documented examples pin all
five cases: 1.5 → 3, 3 → 3, 2 → 3, −1 → −1, −2 → −3.

Written out:

    ODD(x) = sign(x) · ( 2·⌈ (|x| − 1)/2 ⌉ + 1 )

**Domain and range.** Domain: all reals. Range: the odd integers — with two qualifications, one
at zero and one at the top of the floating-point range, both below.

**Shape.** A staircase with steps of width 2 and jump 2, symmetric under *x* ↦ −*x* (the function
is odd in the algebraic sense as well as in the arithmetic one: ODD(−x) = −ODD(x) away from zero).
It is idempotent — ODD(ODD(x)) = ODD(x) — and it is the away-from-zero counterpart of `EVEN`,
which does the same thing to an even lattice.

**Zero is the interesting point.** Zero is an even integer, so `ODD(0)` must move — but "away from
zero" has no direction at zero. The two candidates are ±1, equidistant and equally defensible.
**Microsoft's page does not state which one is returned**; the documented example list stops at
−1. The reference engine returns +1. That is the first probe below.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `number` | "The value to round." | Yes |

Exactly one argument; the declared arity is one to one. The slot is numeric and subject to
ordinary to-number coercion ([coercion and lifting](../model/02-coercion-and-lifting.md)). The
declared coercion profile is the unary numeric scalar-or-array-elementwise one, so `ODD` is a
scalar kernel that lifts over arrays.

## Result and edge cases

Returns `Number` — an integer value.

The reference engine's published battery is rendered beside this page. Qualitatively:

- **Zero** returns +1, as discussed above. Undocumented.
- **−1** is returned unchanged, matching the documented example.
- **A logical argument** converts and rounds; **numeric text** converts and rounds.
- **The smallest positive subnormal** rounds to 1 — every strictly positive value below 1 does.
- **An inline array** lifts elementwise.
- **An empty range** produces `#VALUE!`.
- **The largest finite double is returned unchanged** — and it is an **even** integer. See below;
  this is not a rounding decision, it is a representability wall.

### The postcondition fails above 2^53

`ODD` promises an odd result. Above 2^53 it cannot deliver one, and the reason is arithmetic
rather than implementation.

Binary64 spaces consecutive representable values 2 apart in [2^53, 2^54), 4 apart in
[2^54, 2^55), and so on. **Every finite double of magnitude at or above 2^53 is an even integer**,
because it is a multiple of at least 2. There is no odd double above that point. So for any
argument in that range:

- returning the argument unchanged gives an **even** answer, violating the function's defining
  property; and
- returning "the next odd integer" is impossible, because that integer is not representable —
  the nearest doubles either side are both even.

The reference engine's battery row for the largest finite double takes the first option. Any
implementation must take it or must error; there is no third choice. The honest statement of the
function's range is therefore: **the odd integers below 2^53, and whatever convention the
implementation adopts above it.** No documentation addresses this, and it is not a defect in
anyone's code — it is the domain running out.

The same wall governs `EVEN` in mirror image, where the story is happier: above 2^53 every double
already *is* even, so `EVEN`'s postcondition survives exactly where `ODD`'s fails.

## Errors

| Error | Condition | Basis |
|---|---|---|
| `#VALUE!` | The argument is nonnumeric | **Documented**: "If number is nonnumeric, ODD returns the #VALUE! error value." |
| propagated | An error value in the argument | Shared call model |

The reference engine additionally reports `#VALUE!` for arity failures; in Excel a missing
required argument is expected to be refused at formula entry
([the call pipeline](../model/03-call-pipeline.md)).

There is no documented domain error, and none is needed: `ODD` is total on the reals.

## Relationships

- **`EVEN`** — the twin, rounding away from zero to an even integer. The pair are not inverses and
  do not compose usefully; they partition nothing. Note the asymmetry recorded above: `EVEN`'s
  postcondition survives the 2^53 wall and `ODD`'s does not.
- **[ISEVEN](FUNC.ISEVEN.md) / `ISODD`** — the parity *tests*, which truncate toward zero. `ODD`
  is a rounding function, not a predicate, and `ISODD(ODD(x))` is true by construction only below
  the wall.
- **[MROUND](FUNC.MROUND.md)** — rounds to the nearest multiple. `ODD` is not `MROUND(x, 2)` with
  an offset: `MROUND` rounds to *nearest* while `ODD` rounds *away*, so they agree only when the
  argument is already past the midpoint.
- **[INT](FUNC.INT.md)** and **`TRUNC`** — the other integer-valued roundings, each with a
  different convention. This category has floor, truncate-toward-zero, away-from-zero and
  half-away-from-zero all in play, and the names do not disclose which is which.
- **[ROUNDUP](FUNC.ROUNDUP.md)** — rounds away from zero to a number of decimal places;
  `ROUNDUP(x, 0)` is the away-from-zero integer rounding without the parity constraint.
- The most common real use is column and row banding, and generating alternating series — both of
  which stay in the small-integer range where none of this page's edge cases can arise.

## Numerical notes

`ODD` performs no transcendental work and, below the representability wall, no rounding of its
own. Its numerical character comes from three places.

**Discontinuity amplification.** Like [INT](FUNC.INT.md), `ODD` is a step function, so a sub-ULP
error in its argument becomes a full step — here a jump of **2**, not 1 — whenever the argument
straddles a step boundary. The boundaries are the odd integers themselves: an argument that ought
to be exactly 3 but arrives as the next double above 3 is rounded to 5. Anything computed and then
passed to `ODD` should be rounded to an integer first if the intent was an integer.

**The representability wall**, described in full above. An implementation that computes
`2*ceil((|x|-1)/2)+1` in floating point does not merely return an even number above 2^53 — the
intermediate `|x|-1` is itself a no-op there (subtracting 1 from a double spaced 2 apart returns
the same double), so the formula degenerates to the identity. That the naive formula and the
correct-by-convention answer coincide is a coincidence, not a design, and an implementation that
converts through a fixed-width integer instead will produce something different — or wrap — at the
same inputs.

**The sign of zero.** `ODD(-0.0)` has an argument whose sign bit is set but whose value is zero,
so the "away from zero" direction is even less determined than at +0. Whether an implementation
consults the sign bit is a decision; it should be a recorded one.

The relevant background is not a special-function reference — there is no series to sum here — but
the general treatment of rounding to an integer lattice and the exactness of the operations
involved: IEEE 754-2019's `roundToIntegral*` family, and Muller et al., *Handbook of
Floating-Point Arithmetic*, on the spacing of binary64 and on rounding to a grid.

## What has not been checked

No Handbook vector suite exists for `ODD`; `vectors/` publishes nothing for this function, and
**no evidence record names `ODD` at all** — not as a subject, and not inside any group's
membership. Nobody has checked this function against Excel within the Handbook's record. The
implementing module carries no open upstream defect stream, which means only that nothing has been
filed, not that anything has been confirmed.

Everything above marked as documented comes from Microsoft's `ODD` page: the syntax, the argument
description, the `#VALUE!` remark, the away-from-zero rule, the no-rounding-for-odd-integers rule,
and the five worked examples.

Inputs I would probe first:

1. **`ODD(0)`.** One cell. Microsoft's page does not say, the two candidate answers are equally
   defensible, and the reference engine has picked one. This is the page's cleanest open question.
2. **`ODD(-0.0)`** — reached as `ODD(-1*0)` or `ODD(0*-1)` — to see whether the sign bit steers the
   direction.
3. **The 2^53 boundary**: `ODD(2^53 - 1)`, `ODD(2^53)`, `ODD(2^53 + 2)` and the largest finite
   double. This locates where the postcondition stops holding and reveals whether Excel returns the
   argument, errors, or does something else. It is the most *interesting* probe on the page, and
   nothing in the Handbook's record predicts the answer.
4. **`ODD(0.5)`, `ODD(-0.5)`, `ODD(1)`, `ODD(-1)`** — the smallest-magnitude behaviour either side
   of zero, which is where the away-from-zero rule is easiest to get subtly wrong.
5. **`ODD(TRUE)`** — the reference engine converts the logical, and the documentation's
   "nonnumeric" wording does not obviously cover a logical.
6. **An array argument**, to confirm the elementwise lift and element-local failures.
7. **`ODD(3 + 2^-50)`**, an argument a hair above an odd integer, as the concrete form of the
   amplification hazard.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| away from zero | The rounding direction: outward in both half-planes |
| idempotent | Applying the function twice changes nothing after the first application |
| representability wall | 2^53, above which no odd integer is a binary64 value |
| postcondition | The property the result is supposed to have — here, being odd |
| amplification | A sub-ULP argument error becoming a full step in the result |

## Sources

- Microsoft, "ODD function" —
  <https://support.microsoft.com/en-us/office/odd-function-deae64eb-e08a-4c88-8b40-6d0b42575c98>.
  Retrieved for this page: the syntax, the argument description, the `#VALUE!` nonnumeric remark,
  the away-from-zero rule, the no-rounding-for-odd-integers remark, and the five worked examples.
  The page as retrieved says nothing about zero and nothing about large magnitudes.
- IEEE 754-2019 — binary64 value spacing and the `roundToIntegral*` operations;
  J.-M. Muller et al., *Handbook of Floating-Point Arithmetic*, 2nd edition — spacing of the
  binary64 grid and rounding to a lattice.
- Handbook, [INT](FUNC.INT.md) — the amplification hazard shared by every step function in this
  category; [ISEVEN](FUNC.ISEVEN.md) — the parity predicate and its own 2^53 discussion;
  [coercion and lifting](../model/02-coercion-and-lifting.md) — to-number outcomes, scalar-kernel
  lifting, error propagation.
- `data/functions/FUNC.ODD.json` and `data/presence/FUNC.ODD.json` — identity, signature
  `ODD(number)`, arity 1–1, the `UnaryNumericScalarOrArrayElementwise` coercion profile, the
  implementing module and its Lean companion, as projected at OxFunc `473efa3`.
