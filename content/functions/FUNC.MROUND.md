---
schema: efh.function-page/v1
function_id: FUNC.MROUND
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
family: mround
role_in_family: >-
  Sole member of its module; rounds to the nearest multiple rather than up or down, and is the
  one rounding surface whose own documentation admits that its midpoint direction is undefined
  for decimal multiples.
---

## What it computes

`MROUND(number, multiple)` rounds *number* to the **nearest** multiple of *multiple*.

Where [ISO.CEILING](FUNC.ISO.CEILING.md) always rounds up and `FLOOR.MATH` always rounds down,
`MROUND` rounds to whichever multiple is closer, with ties broken **away from zero**. Microsoft
states the tie rule as a condition on the remainder:

> "MROUND rounds up, away from zero, if the remainder of dividing number by multiple is greater
> than or equal to half the value of multiple."

So the mathematical definition is round-half-away-from-zero on the scaled variable:

    MROUND(x, m) = sign(x) · ⌊ |x/m| + 1/2 ⌋ · |m|

Equivalently, with the remainder *r* = MOD(|x|, |m|): the result is the lower multiple when
*r* < |m|/2 and the upper multiple when *r* ≥ |m|/2. The documented examples fit: 10 with a
multiple of 3 gives 9 (the remainder 1 is below 1.5), and −10 with −3 gives −9 by symmetry.

**Domain.** Both arguments must share a sign — Microsoft states this as a hard requirement, not a
convention: "The Number and Multiple arguments must have the same sign. If not, a #NUM error is
returned", with `MROUND(5, -2)` documented as `#NUM!`. This is the property that most sharply
separates `MROUND` from the modern `CEILING.MATH`/`FLOOR.MATH` pair, which take the absolute value
of the significance and never reject a sign combination.

**Range.** The multiples of |*m*|. The function is a staircase: piecewise constant, with jumps of
|*m*| at the midpoints between consecutive multiples, and it is *not* monotone-continuous — it is
non-decreasing but has a jump at every half-multiple.

**Idempotence.** `MROUND(MROUND(x, m), m) = MROUND(x, m)`. Rounding an already-rounded value
changes nothing, provided the multiple is unchanged — a property that fails in practice exactly
when the multiple is not exactly representable, which is the subject of "Numerical notes".

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `number` | "The value to round." | Yes |
| `multiple` | "The multiple to which you want to round number." | Yes |

Both required; the declared arity is exactly two. Neither has a default — unlike
[ISO.CEILING](FUNC.ISO.CEILING.md), whose significance defaults to 1, `MROUND` has no
single-argument form.

Both slots are numeric and subject to ordinary to-number coercion
([coercion and lifting](../model/02-coercion-and-lifting.md)). The declared coercion profile is
the scalar-only unary numeric one.

## Result and edge cases

Returns `Number`.

The reference engine's published battery is rendered beside this page. Qualitatively:

- **Both arguments zero** returns zero. This is **not documented**: the rule as written requires
  dividing by the multiple, which is undefined at zero, and the same-sign test is vacuous when
  both are zero. Returning zero is a defensible reading — the only multiple of zero is zero — but
  it is a choice, and Microsoft's page does not make it.
- **Both arguments −1** returns −1: a value already on a multiple is unchanged, and the negative
  branch is reached with matching signs as required.
- **A logical argument** converts; **numeric text** converts and is rounded to a multiple of
  itself, returning the value unchanged.
- **The smallest positive subnormal against itself** returns itself — the staircase has steps
  everywhere, including at the bottom of the range.
- **An inline array** in both slots produces an array.
- **An empty range** produces `#VALUE!`.

The battery contains no row with **mismatched signs**, no row with a **decimal multiple**, and no
row at a **midpoint** — which is to say, it does not exercise any of the three things that make
this function interesting. That is worth knowing before drawing conclusions from it.

## Errors

| Error | Condition | Basis |
|---|---|---|
| `#NUM!` | *number* and *multiple* have different signs | **Documented**: "The Number and Multiple arguments must have the same sign. If not, a #NUM error is returned", with `MROUND(5,-2)` shown as `#NUM!` |
| — | *multiple* is 0 | **Not documented.** The reference engine returns 0 when both arguments are 0; the mixed case (nonzero number, zero multiple) is unprobed |
| `#VALUE!` | An argument does not convert to a number | Shared call model |
| propagated | An error value in either argument | Shared call model |

The zero-multiple row is a genuine gap. It is the only input for which the documented rule cannot
be evaluated at all, and neither Microsoft's page nor the Handbook's record says what Excel
returns for `MROUND(5, 0)`.

## Relationships

- **`ROUND`** — rounds to a number of decimal places rather than to a multiple. `ROUND(x, 2)` and
  `MROUND(x, 0.01)` are the same idea expressed two ways, and they can disagree, because the first
  scales by a power of ten and the second divides by an inexact binary constant. Which of the two
  a reader should use is decided by that difference, not by taste.
- **[ISO.CEILING](FUNC.ISO.CEILING.md)**, **[CEILING.MATH](FUNC.CEILING.MATH.md)**,
  **[FLOOR.MATH](FUNC.FLOOR.MATH.md)** — the directed round-to-multiple family. `MROUND` is the
  nearest-multiple member and the only one that can move a value in either direction.
- **[CEILING](FUNC.CEILING.md)** — the legacy directed form, and `MROUND`'s closest relative in
  temperament: both **reject** mismatched signs where the modern `.MATH` surfaces accept them.
  The sign policy, not the rounding direction, is what dates a function in this neighbourhood.
- **[MOD](FUNC.MOD.md)** — the primitive `MROUND`'s documented rule is phrased in terms of ("the
  remainder of dividing number by multiple"). The two pages should be read together.
- **[INT](FUNC.INT.md)** — the floor underneath the scaled formulation.
- **`EVEN` and [ODD](FUNC.ODD.md)** — round to a multiple of 2 with a parity offset, always away
  from zero. `MROUND(x, 2)` and `EVEN(x)` differ whenever *x* is not past the midpoint.

## Numerical notes

`MROUND` carries a hazard that its own documentation admits, which is rare enough to quote:

> "MROUND(6.05,0.1) returns 6.0 while MROUND(7.05,0.1) returns 7.1."

Two calls, the same multiple, the same apparent midpoint, opposite directions. Microsoft describes
the rounding direction for midpoint values with decimal multiples as undefined. **This is not a
bug and it is not fixable inside the function.** It is what happens when a round-to-nearest rule is
evaluated on values that are not what they appear to be.

**Why.** Neither 0.1 nor 6.05 nor 7.05 is representable in binary64. The stored double for 6.05 is
slightly *below* six-and-a-twentieth; the stored double for 7.05 is slightly *above* seven-and-a-
twentieth. The remainder test — "is the remainder at least half the multiple?" — is therefore being
asked about a value that sits a hair on one side of the true midpoint, and which side it sits on
is determined by the binary expansion, effectively at random with respect to the decimal the user
typed. Every decimal step size (0.1, 0.05, 0.25 is fine, 0.2 is not, 0.01 is not) inherits this.

**What a careful implementation does about it**, in rough order of increasing effort:

1. **Scale to integers when the multiple is a decimal fraction.** If the multiple is *k*·10⁻ⁿ with
   small *k* and *n*, multiply both arguments by 10ⁿ, round in a domain where the midpoint is
   exact, and scale back. This is what `ROUND`-style functions do internally, and it changes the
   answers at midpoints — which is precisely why `ROUND(x,2)` and `MROUND(x,0.01)` need not agree.
2. **Compute the quotient with a correction term.** The quantity |x/m| must be compared against a
   half-integer boundary; a fused multiply-add residual (x − q·m computed exactly) tells you which
   side of the boundary the *exact* quotient lies on, even when the rounded quotient does not.
   This is the same technique that makes correctly-rounded division-based predicates possible, and
   it is discussed in Muller et al., *Handbook of Floating-Point Arithmetic*, in the chapters on
   exact arithmetic operations and on rounding to a multiple.
3. **Do nothing, and document it** — which is the route Microsoft's page takes, and which is
   honest. An implementation that silently "fixes" the midpoints has made itself incompatible with
   the documented behaviour of the function it claims to implement.

**The multiplication at the end is also inexact.** After choosing the integer multiplier *n*, the
returned value is *n*·|m|, which rounds. So the result of `MROUND` is not guaranteed to be an
exact multiple of *m* — it is the nearest double to one. A downstream test of the form
`MOD(MROUND(x, m), m) = 0` can therefore fail, and readers who chain rounding into a divisibility
check should expect it to.

**The sign policy costs nothing numerically and something practically.** Rejecting mismatched
signs means a formula whose inputs can change sign — a variance, a delta, a balance — will
intermittently produce `#NUM!` rather than a rounded value. `CEILING.MATH` and `FLOOR.MATH` exist
partly to remove that failure mode. `MROUND` has no such escape, so the guard belongs in the
formula: `SIGN(x)*MROUND(ABS(x), ABS(m))` is the usual reconstruction, and it is arithmetic, not a
Handbook claim about Excel.

## What has not been checked

No Handbook vector suite exists for `MROUND`; `vectors/` publishes nothing for this function, and
**no evidence record names `MROUND` among its subjects**.

`MROUND` does appear inside the group membership of one structural array-lift record without being
one of that record's subjects. Those group counts carry an explicit warning against per-surface
attribution, so the honest statement is: **the family was measured on an array-shape axis and this
surface was not measured separately.** Nobody has checked `MROUND`'s values against Excel within
the Handbook's record.

The implementing module carries three open upstream defect streams touching this function
(`BUG-FUNC-017`, `BUG-FUNC-027`, `BUG-FUNC-039`), covering math scalar/array lifting, broad
scalar-invocation findings, and a statistical-and-boundary edge batch. Array-shaped and
boundary-valued arguments are therefore areas with recorded historical trouble in the reference
engine, not settled ground.

Everything above marked as documented comes from Microsoft's `MROUND` page: the syntax, both
argument descriptions, the remainder-based tie rule, the same-sign requirement and its `#NUM!`,
the four worked examples, and the admitted undefined midpoint direction for decimal multiples.

Inputs I would probe first:

1. **`MROUND(6.05, 0.1)` and `MROUND(7.05, 0.1)`.** Microsoft's own stated pair. Reproducing it is
   the cheapest way to confirm the documentation describes the shipping behaviour, and it
   immediately calibrates how much of the rest of the page can be trusted.
2. **`MROUND(5, 0)` and `MROUND(0, 0)`.** The undocumented zero-multiple case, where the
   reference engine has an answer for one input and nothing is known about the other.
3. **A midpoint sweep at an exactly representable multiple** — `MROUND(x, 0.5)` and
   `MROUND(x, 0.25)` at exact midpoints. Here the tie rule *is* well defined, so this isolates
   "does it round half away from zero?" from "is the midpoint even detectable?".
4. **`MROUND(x, 0.01)` against `ROUND(x, 2)`** across a few hundred values. Any disagreement
   locates the scaling difference described above, and this is the comparison a reader is most
   likely to care about in practice.
5. **The four sign combinations**, including `MROUND(-5, 2)` and `MROUND(5, -2)`, to confirm the
   documented `#NUM!` is symmetric.
6. **`MOD(MROUND(x, m), m)` for a decimal *m***, testing whether the result is exactly a multiple.
7. **Array arguments in each slot**, given the open lifting defect stream on this module.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| round half away from zero | The documented tie rule: a remainder at or above half the multiple rounds outward |
| multiple | The step size of the staircase; must share the sign of `number` |
| midpoint | A value exactly halfway between two multiples, where the tie rule is invoked |
| inexact multiple | A decimal step such as 0.1 that has no exact binary64 representation |
| group figure | A count measured across several surfaces jointly; never a per-function rate |

## Sources

- Microsoft, "MROUND function" —
  <https://support.microsoft.com/en-us/office/mround-function-c299c3b0-15a5-426d-aa4b-d2d5b3baf427>.
  Retrieved for this page: the syntax, both argument descriptions, the remainder-based tie rule,
  the same-sign requirement and its `#NUM!`, the four worked examples, and the documented
  statement that the rounding direction for midpoint values with decimal multiples is undefined,
  with its two-call illustration.
- J.-M. Muller et al., *Handbook of Floating-Point Arithmetic*, 2nd edition — exact operations,
  residual-based comparison against a boundary, and rounding to a multiple.
- Handbook, [MOD](FUNC.MOD.md) — the remainder the tie rule is phrased in terms of;
  [ISO.CEILING](FUNC.ISO.CEILING.md) — the directed sibling and the same inexact-significance
  hazard; [coercion and lifting](../model/02-coercion-and-lifting.md) — to-number outcomes and
  propagation.
- `data/functions/FUNC.MROUND.json` and `data/presence/FUNC.MROUND.json` — identity, signature
  `MROUND(number, multiple)`, arity 2–2, the scalar-only coercion profile, the implementing module
  and its Lean companion, and the `BUG-FUNC-017`, `BUG-FUNC-027` and `BUG-FUNC-039` defect
  streams, as projected at OxFunc `473efa3`.
