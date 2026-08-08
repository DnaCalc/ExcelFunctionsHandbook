---
schema: efh.function-page/v1
function_id: FUNC.FACTDOUBLE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0018
  - EV-MATH-0008
open_problems: []
references:
  - work: "Microsoft Support — FACTDOUBLE function"
    locator: "https://support.microsoft.com/en-us/office/factdouble-function-e67697ac-d214-48eb-b7b7-cce2589ecac8"
    role: "documented signature, the truncation rule, the even and odd product forms, and the #NUM! and #VALUE! conditions"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 6 (Gamma Function), 6.1.12; and the Legendre/Bessel series in chapters 8 and 9"
    role: "the double factorial as a gamma value at half-integers, and where it arises in series"
  - work: "Press, Teukolsky, Vetterling & Flannery, Numerical Recipes"
    locator: "the `factrl`/`gammln` discussion"
    role: "the tabulate-then-log-gamma strategy that applies here even more strongly"
  - work: "Muller et al., Handbook of Floating-Point Arithmetic"
    locator: "the chapter on exact operations and integer-valued floating-point numbers"
    role: "why scaling by a power of two is free, which is the key to this function's exactness range"
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
family: factdouble
role_in_family: >-
  The double factorial — the skip-one product, whose even branch is a scaled factorial and
  whose odd branch is a half-integer gamma value, with the two branches behaving quite
  differently in binary64.
---

# FACTDOUBLE

## What it computes

`FACTDOUBLE(number)` returns the double factorial: the product of the argument with every
second integer below it, down to 2 or to 1. Microsoft's page gives both branches:

    n!! = n·(n-2)·(n-4)·…·4·2    for even n
    n!! = n·(n-2)·(n-4)·…·3·1    for odd n

The name is misleading and worth clearing up once: `n!!` is **not** `(n!)!`, and it is not
`2·n!`. It is a product with every other factor removed.

| Property | Statement |
|---|---|
| Domain (documented) | non-negative; non-integers truncated |
| Range | positive integers |
| Recurrence | `n!! = n · (n-2)!!` |
| Base cases | `0!! = 1` and `1!! = 1` (empty and singleton products) |
| Even closed form | `(2k)!! = 2^k · k!` |
| Odd closed form | `(2k+1)!! = (2k+1)! / (2^k · k!)` |
| Bridge to factorial | `n! = n!! · (n-1)!!` |
| Gamma relation | `(2k-1)!! = 2^k · Γ(k + ½) / √π` (A&S 6.1.12) |
| Exactly representable | even `n ≤ 44`; odd `n ≤ 29` |
| Largest finite | `300!!` for even, `299!!` for odd |

The two branches are genuinely different objects. The even branch is a factorial scaled by a
power of two — which in binary64 costs *nothing*, since scaling by `2^k` only shifts the
exponent field. The odd branch is a ratio, and it is the one that appears in the coefficients of
Legendre polynomials, in the Wallis product, in Bessel-function series and in the volume of the
`n`-sphere. Half of the reason this function exists on a worksheet is the odd branch.

## Arguments

| Argument | Meaning | Constraint (as documented) |
|---|---|---|
| `number` | The value whose double factorial is wanted. Required. | non-negative; non-integers truncated |

Microsoft documents that non-integer values are truncated and that negative numbers give
`#NUM!`. Ordinary to-number coercion applies; the reference engine declares the surface a scalar
kernel that lifts elementwise over arrays. See
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`, mathematically always a positive integer.

- **`FACTDOUBLE(0)` and `FACTDOUBLE(1)`** — both 1, by the empty and singleton product
  conventions. Microsoft's page states the negative rule but does not state these two values
  explicitly; they follow from the printed product forms read at their boundaries.
- **`FACTDOUBLE(-1)`** — **a documented divergence from the standard convention.** In the
  mathematical literature `(-1)!! = 1`, and that convention is not decorative: it is what makes
  the recurrence `n!! = n·(n-2)!!` and the gamma relation extend correctly, and it is used in the
  series expansions the odd branch appears in. Microsoft's page documents negative arguments as
  `#NUM!`, which excludes `-1`. Either Excel refuses a value the literature defines, or the
  documentation is broader than the implementation. The Handbook has not checked, and this is
  the first probe below.
- **Fractional arguments** — truncate. As on [FACT](FUNC.FACT.md), whether the domain check
  sees the raw or the truncated argument decides the interval `(-1, 0)`, and the documentation
  does not say.
- **Parity is decided after truncation**, necessarily, since a fractional argument has no
  parity. That means `FACTDOUBLE(7.9)` follows the odd branch.
- **Above the exactness boundaries** — the result is rounded. Note the boundaries are *far*
  apart for the two branches: the even branch stays exact to `44` and the odd branch only to
  `29`. See the numerical notes for why.
- **Above the range boundary** — the true value is not finite. The reference engine records
  `non_finite=num`, and `EV-MATH-0008` records the convention that a non-finite real result
  surfaces as an error rather than as an infinity, with `FACTDOUBLE` among its named surfaces.
  That record publishes **no count**.

## Errors

As documented on Microsoft's `FACTDOUBLE` page:

| Error | Condition |
|---|---|
| `#NUM!` | `number` is negative |
| `#VALUE!` | `number` is non-numeric |

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | The true result is not finite | `EV-MATH-0008`, which names `FACTDOUBLE` — a kind and error-code convention, no count |

## Relationships

- **[FACT](FUNC.FACT.md)** — joined by `n! = n!! · (n-1)!!`, a free cross-surface identity that
  neither page's evidence covers and that requires no oracle to test.
- **`GAMMA`, `GAMMALN`** — the odd branch is a half-integer gamma value: `(2k-1)!! = 2^k
  Γ(k+½)/√π`. Excel exposes `GAMMA`, so `FACTDOUBLE(2k-1)` and
  `2^k · GAMMA(k+0.5) / SQRT(PI())` are two routes to one number, across two surfaces with
  different substrates. That is a strong probe and it is unused.
- **`SQRTPI`** — appears in the same identity, and exists on the worksheet surface precisely
  because `√π` shows up wherever half-integer gammas do.
- **[COMBIN](FUNC.COMBIN.md), `MULTINOMIAL`** — the other combinatorial surfaces with factorial
  overflow hazards. The central binomial coefficient is
  `C(2k, k) = 2^k · (2k-1)!! / k!`, another cross-surface bridge.
- **`BESSELJ`, `BESSELY`, `BESSELI`, `BESSELK`** — their series coefficients are double
  factorials; the odd branch is where it appears.
- **`SERIESSUM`** — the natural consumer, for the same reason.

## Numerical notes

The two branches deserve different implementations, and the reason is a fact about binary64
rather than about combinatorics.

**The even branch is free.** `(2k)!! = 2^k · k!`. Multiplication by a power of two is exact in
binary64 — it adjusts the exponent field and touches no significand bit — so the even branch is
exactly as accurate as `FACT` is, and exactly representable for the same `k`. That is why the
exactness boundary sits at `n = 44`: it is `2 × 22`, and 22 is the largest `k` with `k!`
exactly representable. An implementation that computes the even branch as a product loop throws
this away for nothing; computing it as `2^k · FACT(k)` inherits `FACT`'s accuracy and the
scaling costs no error at all.

**The odd branch is not free.** `(2k+1)!! = (2k+1)!/(2^k k!)` is a genuine ratio, and its
exactness boundary is much lower — `n = 29` — because the odd product accumulates factors with
no trailing zero bits at all. Odd numbers are odd; the product of `k` of them has no factor of
two to give it trailing zeros, so it consumes significand bits at full rate. This asymmetry is
the single most useful implementation fact on this page and it is invisible from the
documentation.

**The domain is small enough to tabulate — twice.** There are 151 even arguments up to `300`
and 150 odd ones up to `299`, so 301 correctly rounded doubles cover the entire function. As on
[FACT](FUNC.FACT.md), a table is faster than a loop, exact where exactness is possible, and
correctly rounded everywhere else. There is no reason to ship anything else.

**What a product loop costs.** Rounding at every step, accumulating roughly as the square root
of the number of factors in the best case. For the odd branch at `n = 299` that is 150
multiplications, and a few ulp of drift by the end is unremarkable. Ascending and descending
loops give different answers, which is the same op-graph question
[COMBIN](FUNC.COMBIN.md)'s evidence turns on — and unlike `COMBIN`, here the correct answer is
known independently, so the question is decidable without an oracle for every argument.

**For consumers**: the double factorial is almost always wanted inside a ratio — a Legendre
coefficient, a Bessel term, a sphere volume — and the ratio is better computed by its own
recurrence than by dividing two `FACTDOUBLE` results. `(2k+1)!!/(2k)!!` stays around `√(2k/π)`
for all `k`, while both halves overflow past 300.

## What has not been checked

Two evidence records name this surface. `EV-MATH-0018` records a **spot check** — a small,
fully-matching comparison against Excel — and its reader warning is explicit that a spot check
on a corpus of that size is not a sweep and carries no held-out component. It also flags a scope
trap in the upstream catalogue: `FACTDOUBLE` sits inside a row whose *header* lists it among
open discrepancies while its own clause reports full agreement, so reading the row header as the
status of every member would misrepresent this surface. `EV-MATH-0008` names `FACTDOUBLE` in the
FINITE error-code convention family and publishes no count.

No Handbook vector suite exists for `FACTDOUBLE`. The truncation rule, the two product forms and
the two error conditions are Microsoft's; the exactness boundaries and the branch analysis are
mathematics; the `(-1)!!` question is open.

Inputs I would probe first:

1. **`FACTDOUBLE(-1)`.** The standard mathematical convention gives 1; the documented rule gives
   `#NUM!`. One probe, and either answer is a publishable finding — a documented divergence from
   the literature, or a documentation that over-states its own restriction. Extend with
   `FACTDOUBLE(-0.5)` and `FACTDOUBLE(-1.5)` for the truncation-order question.
2. **`FACTDOUBLE(0)` and `FACTDOUBLE(1)`.** Both should be 1 and neither is documented.
3. **The range boundaries by parity**: `FACTDOUBLE(299)`, `FACTDOUBLE(300)`, `FACTDOUBLE(301)`,
   `FACTDOUBLE(302)`. The even and odd branches overflow at different arguments, and the pattern
   of `#NUM!` across these four tells you whether the implementation knows that.
4. **The full domain against a table.** 301 values, compared against correctly rounded
   references. The domain is finite and small, so this is a complete characterisation rather
   than a sample — the same rare opportunity `FACT` offers.
5. **The exactness frontier by parity**: even arguments 42–48, odd arguments 27–33. If the even
   branch stays exact past 44 or the odd branch past 29, the implementation is not working in
   binary64; if the even branch fails before 44, it is not using the power-of-two form.
6. **The bridge identity** `FACT(n) - FACTDOUBLE(n)·FACTDOUBLE(n-1)` across `n = 2 … 170`. Free,
   oracle-less, and it ties two surfaces together.
7. **The gamma bridge**: `FACTDOUBLE(2k-1)` against `2^k · GAMMA(k+0.5) / SQRT(PI())`. Three
   surfaces, one identity, and a disagreement localises to whichever of them is weakest.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| double factorial | The skip-one product `n·(n-2)·(n-4)·…`; not `(n!)!` |
| even branch | Arguments of even parity, where `n!! = 2^{n/2}·(n/2)!` |
| odd branch | Arguments of odd parity, a half-integer gamma value |
| exact scaling | Multiplication by a power of two, which introduces no rounding error |
| exactness boundary | The largest argument whose result is a binary64 value with no rounding |
| spot check | A small fully-compared corpus; not a sweep and carrying no held-out rows |

## Sources

- Microsoft, "FACTDOUBLE function" —
  <https://support.microsoft.com/en-us/office/factdouble-function-e67697ac-d214-48eb-b7b7-cce2589ecac8>
  (fetched at curation: signature, truncation of non-integers, the even and odd product forms,
  and the `#NUM!` and `#VALUE!` conditions).
- Handbook evidence records `EV-MATH-0018` (the spot check, with its scope-trap warning about
  the upstream catalogue row header) and `EV-MATH-0008` (the FINITE error-code convention, no
  count). Read both reader warnings.
- Abramowitz & Stegun, chapter 6, in particular 6.1.12 — the half-integer gamma relation; and
  the series in chapters 8 and 9 where the odd branch appears.
- Press et al., *Numerical Recipes* — the tabulation strategy.
- Muller et al., *Handbook of Floating-Point Arithmetic* — exactness of power-of-two scaling.
- Handbook, [FACT](FUNC.FACT.md) and [COMBIN](FUNC.COMBIN.md);
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.FACTDOUBLE.json` (the `non_finite=num` axis value)
  and `data/presence/FUNC.FACTDOUBLE.json` (implementing module and the two defect streams that
  mention it).
