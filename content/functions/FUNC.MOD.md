---
schema: efh.function-page/v1
function_id: FUNC.MOD
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MISC-0012
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
family: mod_fn
role_in_family: >-
  Sole member of its module; the floored remainder, whose result takes the sign of the divisor —
  and the math surface carrying the most sharply characterised Excel departure from mathematics
  in this category.
---

## What it computes

`MOD(number, divisor)` returns the remainder after *number* is divided by *divisor*.

Microsoft's page gives the definition outright, in terms of another published function:

> `MOD(n, d) = n - d*INT(n/d)`

and states the rule that distinguishes it from every other remainder you may have met:

> "The result has the same sign as divisor."

Because [INT](FUNC.INT.md) is the **floor**, this is the *floored* remainder, sometimes called
the mathematician's or Knuth's mod:

    MOD(n, d) = n − d·⌊n/d⌋

**There are three inequivalent remainder conventions in common use**, and confusing them is the
main source of off-by-a-divisor bugs:

| Convention | Definition | Sign of result | Where you meet it |
|---|---|---|---|
| **Floored** | n − d·⌊n/d⌋ | follows the **divisor** | Excel `MOD`, Python `%`, Ruby |
| Truncated | n − d·trunc(n/d) | follows the **dividend** | C/C++/Java/C# `%`, IEEE `fmod` |
| Euclidean | remainder always ≥ 0 | always non-negative | Pascal `mod`, some proof systems |

Microsoft's four documented examples pin the floored convention exactly: with dividend and
divisor of opposite signs the answer takes the divisor's sign, so `MOD(-3, 2)` is 1 while
`MOD(3, -2)` is −1. A C programmer's intuition gives the opposite answers for both.

**Domain and range.** *n* any finite real; *d* any nonzero real. Mathematically the result lies
in the half-open interval [0, *d*) for *d* > 0 and in (*d*, 0] for *d* < 0 — the sign rule and
the interval are the same statement. The function is periodic in *n* with period *d*, and
discontinuous at every multiple of *d*, with a jump of |*d*|.

Note that Excel's `MOD` is defined for **non-integer** arguments: `MOD(5.5, 2)` is 1.5. It is a
real remainder, not an integer operation, which is what makes the numerical section below
interesting.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `number` | "The number for which you want to find the remainder." | Yes |
| `divisor` | "The number by which you want to divide number." | Yes |

Both required; the declared arity is exactly two. Both slots are numeric and subject to ordinary
to-number coercion ([coercion and lifting](../model/02-coercion-and-lifting.md)).

The declared coercion profile is the **scalar-only** unary numeric one rather than the elementwise
variant used by [INT](FUNC.INT.md) — a difference worth noting when array arguments are in play.

## Result and edge cases

Returns `Number`.

The reference engine's published battery is rendered beside this page. Qualitatively:

- **Both arguments zero** produces `#DIV/0!`, as documented.
- **`MOD(-1, -1)`** returns a **negative zero**. The remainder is zero and the sign rule says the
  result carries the divisor's sign, so this is the sign rule applied consistently rather than an
  artefact — but a negative zero is a value most worksheets never see, and whether it survives to
  the published result is a boundary question ([the value universe](../model/01-value-universe.md)).
- **Logical arguments** convert; **numeric text** converts.
- **An inline array** in both slots produces an array.
- **An empty range** produces `#VALUE!`.
- **The largest finite double against itself**, and the smallest subnormal against itself, both
  give zero — a value is always congruent to itself.

## Errors

| Error | Condition | Basis |
|---|---|---|
| `#DIV/0!` | *divisor* is 0 | **Documented**: "If divisor is 0, MOD returns the #DIV/0! error value." |
| `#NUM!` | The **quotient** magnitude \|n/d\| reaches a fixed threshold | **Not documented by Microsoft.** Characterised upstream against live Excel; see below |
| `#VALUE!` | An argument does not convert to a number | Shared call model |
| propagated | An error value in either argument | Shared call model |

### The undocumented quotient ceiling

This is the most sharply characterised documentation-versus-behaviour divergence on any page in
this batch, and it deserves to be stated plainly.

Microsoft's page gives `MOD(n, d) = n - d*INT(n/d)` with no domain restriction. That expression is
finite for **every** finite *n* and nonzero *d*, however large the quotient. Yet the upstream
evidence record for this function reports that live Excel returns `#NUM!` once the *quotient*
magnitude |n/d| reaches a precise, **divisor-independent** threshold — bisected against a named
live Excel build down to the exact double at which the behaviour flips, with the adjacent double
below it still returning a number.

Three features of that finding are worth separating:

1. **The limit is on the quotient, not on the dividend.** A very large *n* with a proportionally
   large *d* is fine; a moderate *n* with a small *d* is not. The record's ruled-out list
   explicitly disposes of the "limit on |n|" reading with a counterexample pair.
2. **It is a departure from mathematics, not from precision.** The true remainder exists and is
   representable; Excel declines to produce it. The upstream catalogue records this as a
   deliberate Excel deviation, and the stated cause is that Excel recovers the remainder from
   `n − d·INT(n/d)` and cannot once `INT(n/d)` exceeds its internal magnitude limit — that is,
   the documented *formula* is the cause of the undocumented *error*.
3. **"Matching Excel" here means reproducing the refusal.** The evidence record's own reader
   warning says so. An implementation that returns the mathematically correct remainder above the
   threshold is more correct and less compatible, and the four-flavour framework
   ([implementation options](../model/07-implementation-options.md)) exists precisely so that this
   choice is made explicitly rather than by accident.

The Handbook has not itself observed the threshold in Excel; it publishes the upstream
characterisation, with its build named in the record, as the current best account.

## Relationships

- **[INT](FUNC.INT.md)** — the function `MOD` is documented *in terms of*. The sign rule is not an
  independent fact about `MOD`; it is what you get when the division is floored. Anyone who
  understands `INT`'s behaviour on negatives has already understood `MOD`'s.
- **[QUOTIENT](FUNC.QUOTIENT.md)** — the companion integer division. Note the trap: `QUOTIENT`
  truncates toward zero while `MOD` floors, so **`QUOTIENT(n,d)*d + MOD(n,d)` does not reconstruct
  *n* when the signs differ.** The pair does not satisfy the division identity, which is
  surprising and is worth checking before relying on it.
- **[GCD](FUNC.GCD.md)** — built on repeated remainder; `MOD` is the Euclidean algorithm's step.
- **[MROUND](FUNC.MROUND.md)** — rounds to a multiple, and its documented rule is stated in terms
  of "the remainder of dividing number by multiple", making `MOD` its conceptual primitive too.
- **`FLOOR.MATH(n, d)`** — the other half of the same decomposition: n = FLOOR.MATH(n,d) + MOD(n,d)
  when *d* > 0.
- **`ISEVEN` / `ISODD`** — often written as `MOD(n,2)=0`, which is *not* equivalent, because the
  IS-functions truncate toward zero while `MOD` floors. On negative non-integers the two disagree.

## Numerical notes

`MOD` looks like a trivial function and is not. Two distinct problems live in it.

### Catastrophic cancellation in the naive formula

Evaluate `n − d·⌊n/d⌋` in floating point with a large quotient. The two terms `n` and
`d·⌊n/d⌋` are nearly equal and both large; their difference is O(*d*), which is small. That is the
textbook cancellation setup, and the relative error in the small result is the absolute error of
the large operands divided by the small result — which grows without bound as the quotient grows.
The upstream evidence record for this function reports exactly this: the shipping kernel used the
naive formula, the error reached an enormous multiple of an ULP for large quotients, and the fix
was to stop using it.

**The remedy is to not subtract.** IEEE 754 specifies a remainder operation that is computed
*exactly* — C99's `fmod`, the `%` operator on floating-point in Rust — returning the truncated
remainder with **no rounding whatever**, for any finite operands. It is exact because the true
remainder of two doubles is always itself representable: it is smaller in magnitude than the
divisor and shares its exponent range. From the exact truncated remainder, the floored one is one
conditional away:

    r = fmod(n, d)
    if r ≠ 0 and sign(r) ≠ sign(d):  r = r + d

That is the whole algorithm, it is exact except for the single final addition (which is itself
exact whenever it is needed, since *r* and *d* have opposite signs and |r| < |d|), and it has no
large intermediate to cancel. The evidence record reports this as the shape the reference engine
moved to.

The general lesson generalises past this function: **when a quantity is defined as a small
difference of large numbers, look for a primitive that computes it directly.** Compare
[LCM](FUNC.LCM.md)'s divide-before-multiply and the `log1p` discussion on [LN](FUNC.LN.md) — the
same instinct, three different functions.

### The interval postcondition is not free

The mathematical postcondition is 0 ≤ MOD(n,d) < d for *d* > 0. A naive implementation can violate
it: if the computed remainder is a tiny negative value that should have been zero, adding *d*
produces exactly *d*, which is outside the half-open interval. Careful implementations test the
postcondition explicitly rather than trusting the arithmetic. The exact-`fmod` route above cannot
produce this failure, which is a second reason to prefer it.

### Non-integer divisors

Because Excel's `MOD` accepts real arguments, `MOD(x, 0.1)` is a legal and common call — and 0.1
is not representable in binary64, so the divisor the user meant is not the divisor the function
receives. The result is exact for the divisor that arrived and surprising for the divisor that was
typed. This is not repairable inside the function; it is the same
inexact-decimal-constant problem as on [ISO.CEILING](FUNC.ISO.CEILING.md), and it is worth
warning readers about because worksheets are full of decimal step sizes.

For the taxonomy of division and remainder conventions and their algebraic properties, Boute's
"The Euclidean definition of the functions div and mod" (*ACM TOPLAS* 14(2), 1992) is the standard
reference; Knuth, *The Art of Computer Programming*, volume 1, section 1.2.4 establishes the
floored convention this function follows.

## What has not been checked

No Handbook vector suite exists for `MOD`; `vectors/` publishes nothing for this function.

One evidence record lists `MOD` among its subjects: **EV-MISC-0012**, carrying two per-surface
counts, both at a single named live Excel build that the record notes is restated on the scored
lines themselves — unusual rigour for this evidence base. Its figures are rendered from the record
and are not repeated here. What the reader needs in order to calibrate them, and the record says
all of this itself:

1. **Both corpora were the probe sets built to pin the two fixes.** They are repair-target scores,
   not held-out ones.
2. **Both are small.** The record says so in its own reader warning.
3. **One of the two is a structural, error-placement measurement**, not a measurement of `MOD`'s
   numbers.
4. **"Matches Excel" includes reproducing an Excel departure from mathematics** — the `#NUM!`
   threshold — rather than computing the true remainder.

So `MOD` is better evidenced than most of this category, and it is still a long way from a
published suite. Nothing outside those probe sets has been checked.

Inputs I would probe first:

1. **The two doubles straddling the quotient threshold**, at several different divisors. This
   both re-confirms the upstream bisection independently and tests the divisor-independence claim,
   which is the part of the characterisation carrying the most weight.
2. **A large-quotient case just below the threshold** — `MOD(2^45, 2^10)` and neighbours — where
   the naive formula cancels catastrophically but the answer is still required to be exact. This
   is the probe that distinguishes an exact-`fmod` implementation from a subtract-and-hope one,
   and it needs no knowledge of the threshold to be informative.
3. **`MOD(-1, -1)` and `MOD(0, -1)`**, through `SIGN` and through `1/result`, to establish whether
   a negative zero reaches the sheet.
4. **The four sign combinations from Microsoft's own examples**, as a documentation-conformance
   check. Cheap, and it anchors the convention.
5. **`MOD(n, 0.1)` for a range of n**, against `MOD(n*10, 1)/10`. The inexact-divisor effect, in
   the form a worksheet author actually encounters.
6. **`QUOTIENT(n,d)*d + MOD(n,d)` against *n*** for mixed signs — the reconstruction that the
   floored/truncated mismatch breaks.
7. **Array arguments**, given that this function declares the scalar-only coercion profile while
   its neighbours declare the elementwise one.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| floored remainder | n − d·⌊n/d⌋; the result takes the divisor's sign |
| truncated remainder | n − d·trunc(n/d); the result takes the dividend's sign. C's `%`, IEEE `fmod` |
| quotient ceiling | The undocumented \|n/d\| magnitude at which Excel is reported to return `#NUM!` |
| catastrophic cancellation | Loss of significance when a small result is formed as a difference of large operands |
| exact remainder | The IEEE property that `fmod` of two finite doubles is computed with no rounding |
| repair-target corpus | A probe set built to pin a fix, then used to score it; not held out |

## Sources

- Microsoft, "MOD function" —
  <https://support.microsoft.com/en-us/office/mod-function-9b6cd169-b6ee-406a-a97b-edf2a9dc24f3>.
  Retrieved for this page: the syntax, both argument descriptions, the sign remark, the identity
  `MOD(n, d) = n - d*INT(n/d)`, the `#DIV/0!` remark, and the four sign-combination examples. The
  page as retrieved states no quotient limit and no `#NUM!` condition.
- Handbook evidence record `EV-MISC-0012` — two per-surface counts at a named live Excel build,
  the bisected quotient threshold, the ruled-out readings, the exact-remainder substrate, and the
  reader warning about repair-target corpora and about reproducing an Excel departure from
  mathematics.
- R. T. Boute, "The Euclidean definition of the functions div and mod", *ACM Transactions on
  Programming Languages and Systems* 14(2), 1992 — the taxonomy of remainder conventions.
- D. E. Knuth, *The Art of Computer Programming*, volume 1, section 1.2.4 — the floored
  convention and its algebraic advantages.
- IEEE 754-2019 and ISO C99 `fmod` — the exactly-computed remainder relied on above.
- Handbook, [INT](FUNC.INT.md) — the floor this function is documented in terms of;
  [implementation options](../model/07-implementation-options.md) — the compatibility-versus-
  correctness choice the quotient ceiling forces;
  [coercion and lifting](../model/02-coercion-and-lifting.md) — to-number outcomes and propagation.
- `data/functions/FUNC.MOD.json` and `data/presence/FUNC.MOD.json` — identity, signature
  `MOD(number, divisor)`, arity 2–2, the scalar-only coercion profile, the implementing module and
  its Lean companion, and the `BUG-FUNC-027` defect stream, as projected at OxFunc `473efa3`.
