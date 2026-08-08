---
schema: efh.function-page/v1
function_id: FUNC.FACT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0008
open_problems: []
references:
  - work: "Microsoft Support — FACT function"
    locator: "https://support.microsoft.com/en-us/office/fact-function-ca8588c2-15f2-41c0-8e8c-c11bd471a4f3"
    role: "documented signature, the truncation rule, FACT(0) = 1, and the #NUM! condition for negative arguments"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 6 (Gamma Function), 6.1.1-6.1.5 and the factorial tables in chapter 24"
    role: "the factorial as a special case of the gamma function, and its identity set"
  - work: "Press, Teukolsky, Vetterling & Flannery, Numerical Recipes"
    locator: "the `factrl` and `gammln` routines"
    role: "the table-then-log-gamma strategy and its stated accuracy boundary"
  - work: "Muller et al., Handbook of Floating-Point Arithmetic"
    locator: "the chapter on integer-valued floating-point numbers"
    role: "why n! is exactly representable in binary64 only for small n"
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
family: fact
role_in_family: >-
  The factorial — the integer special case of the gamma function, and the surface whose entire
  useful domain is small enough to tabulate exactly.
---

# FACT

## What it computes

`FACT(number)` returns the factorial. Microsoft's page states the definition as the product
`1·2·3·…·number` and states that a non-integer argument is truncated.

    n! = ∏_{i=1}^{n} i,    with 0! = 1

| Property | Statement |
|---|---|
| Domain (documented) | non-negative; non-integers truncated |
| Range | positive integers |
| Recurrence | `n! = n · (n-1)!` |
| Base case | `0! = 1` — the empty product |
| Gamma relation | `n! = Γ(n+1)`, and `Γ` extends the factorial to all complex arguments except the non-positive integers |
| Growth | Stirling: `n! ~ √(2πn) · (n/e)^n` |
| Log form | `ln(n!) = lnΓ(n+1)`, the quantity `GAMMALN` supplies |
| Exactly representable | for `n ≤ 22` only |
| Largest finite | `170!`; `171!` exceeds the binary64 range |

Two boundaries govern this function and they are far apart. Above `n = 22` the true factorial is
no longer a binary64 value, so every larger result is rounded. Above `n = 170` it is not even
close — the value exceeds the format's range entirely. The whole interesting domain of `FACT` is
therefore the 148 integers between them, which is small enough that an implementation can simply
**tabulate every correctly-rounded answer**. Few functions in this Handbook have that property.

The reason `22` is the exactness limit is worth stating, because it is not obvious: `n!` has
`n - s₂(n)` trailing zero bits, where `s₂(n)` is the population count of `n`. Exact
representability needs the bit-length of `n!` minus those trailing zeros to fit in 53 bits, and
`23!` is the first factorial for which it does not.

## Arguments

| Argument | Meaning | Constraint (as documented) |
|---|---|---|
| `number` | The value whose factorial is wanted. Required. | non-negative; non-integers are truncated |

Truncation, not rounding: Microsoft's page gives the worked example that a value between 1 and 2
behaves as 1. Ordinary to-number coercion applies to the slot, and the reference engine declares
the surface a scalar kernel that lifts elementwise over arrays. See
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`, mathematically always a positive integer.

- **`FACT(0)`** — 1, documented. The empty product convention, and the value that makes the
  binomial coefficient formula work at its boundaries.
- **Fractional arguments between 0 and 1** — truncate to 0, so the answer is 1.
- **Negative arguments** — documented `#NUM!`. This is a genuine mathematical statement rather
  than an implementation limit: `Γ` has poles at every non-positive integer, so `(-1)!` is
  infinite, and the values between the poles alternate in sign with no combinatorial meaning.
- **Fractional negative arguments** — truncate toward zero, so an argument in `(-1, 0)` becomes
  `0` and would give 1 rather than an error. Whether Excel truncates before or after the
  domain check is not documented, and it is the sharpest edge case on this page.
- **Above the representable-exactly boundary** — the result is rounded, and *which* neighbour it
  lands on is implementation-dependent. This is where a product loop and a table diverge.
- **Above the range boundary** — the true value is not finite. The reference engine records
  `non_finite=num`, and `EV-MATH-0008` records the convention that a non-finite real result
  surfaces as an error rather than as an infinity, naming a `FACT` witness pair straddling the
  boundary. That record publishes **no count**; it establishes where an error code appears, with
  witnesses and no denominator.

## Errors

As documented on Microsoft's `FACT` page:

| Error | Condition |
|---|---|
| `#NUM!` | `number` is negative |

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | Non-numeric argument | Shared call model; the page's worked examples imply it, and the sibling `FACTDOUBLE` page states it explicitly |
| `#NUM!` | The true result is not finite | `EV-MATH-0008`, which names `FACT` — a kind and error-code convention, no count |

The split above is deliberate: only the first row is documented on the `FACT` page itself.

## Relationships

- **[FACTDOUBLE](FUNC.FACTDOUBLE.md)** — the skip-one variant `n·(n-2)·(n-4)·…`, related by
  `n! = n!! · (n-1)!!`. That identity is a free cross-surface test and neither page's evidence
  covers it.
- **`GAMMA`** — the continuous extension: `GAMMA(n+1) = FACT(n)`. Excel exposes both, and they
  must agree at the integers; whether they do bitwise is unchecked and is a strong probe.
- **`GAMMALN`, `GAMMALN.PRECISE`** — the logarithm, which is what any implementation must use
  above `n = 170` if it wants an answer at all. `LN(FACT(n))` and `GAMMALN(n+1)` are the two
  routes to the same quantity and differ.
- **[COMBIN](FUNC.COMBIN.md), [COMBINA](FUNC.COMBINA.md), `PERMUT`** — the combinatorial
  surfaces whose documented formulas are written in factorials and whose implementations must
  not be. See the numerical notes on [COMBIN](FUNC.COMBIN.md) for why.
- **`MULTINOMIAL`** — the multinomial coefficient, a ratio of factorials with the same overflow
  hazard.
- **`SERIESSUM`, `EXP`** — Taylor-series machinery where factorials appear as denominators; in
  that role the reciprocal `1/n!` is what is wanted, and it underflows gracefully where `n!`
  overflows abruptly.
- **`POISSON.DIST`, `BINOM.DIST`** — distributions with factorial kernels, which is why they
  are implemented through log-gamma rather than through `FACT`.

## Numerical notes

`FACT` is the clearest case in this batch of a function where the **best implementation is a
table**, and where any cleverness is a mistake.

**The argument.** The whole domain is `0 … 170`. That is 171 values. Storing 171 correctly
rounded doubles costs about 1.4 kilobytes, gives a correctly-rounded answer for every input, is
faster than any loop, and removes the question of accumulated error entirely. Numerical Recipes'
`factrl` does exactly this (with a smaller table and a log-gamma fallback), and there is no
argument for anything else on a domain this size.

**What a product loop costs.** Computing `n!` as a running product in binary64 rounds at every
step above the exactness boundary. The errors accumulate roughly as `√n` ulp in the best case
and linearly in the worst, so by `n = 170` a naive loop can be several ulp from the correctly
rounded value. It is never catastrophic and it is entirely avoidable. Note also that the loop's
*order* matters: multiplying ascending and descending give different results, which is the same
op-graph question that [COMBIN](FUNC.COMBIN.md)'s evidence turns on.

**What the log-gamma route costs.** `exp(lnΓ(n+1))` extends past 170 in the sense that it does
not overflow internally, but the answer still does, and below 170 it is markedly less accurate
than either a table or a loop — the same amplification described on
[COMBINA](FUNC.COMBINA.md), where a small relative error in a large logarithm becomes a large
relative error in the result. For `FACT` specifically there is no reason to pay it.

**The one place the mathematics is genuinely subtle** is the interaction of truncation with the
domain check. `FACT(-0.5)` truncates to `0` and is therefore in the documented domain, while
`-0.5` itself is not; `FACT(-1.5)` truncates to `-1`, which is not. An implementation that
checks the sign of the raw argument and one that checks the sign of the truncated argument
disagree on exactly the interval `(-1, 0)`. The documentation does not say which order applies.

**For consumers**: when a formula needs a *ratio* of factorials, never form the factorials.
`n!/(n-k)!` is a product of `k` terms; `n!/(k!(n-k)!)` is [COMBIN](FUNC.COMBIN.md). Both stay
finite far past where `FACT` stops, and both are more accurate. The presence of `FACT` on the
worksheet is not a licence to write the textbook formula.

## What has not been checked

`EV-MATH-0008` names this surface. It is a kind-and-error-code convention record with named
witnesses and, in its own words, **no row count anywhere**; it establishes where Excel places an
error, not how accurate the values are. No numeric-bits comparison count exists for `FACT` from
that entry, and no other evidence record in the Handbook names it. The presence projection does
record several math-deviation-catalogue mentions for this module, which is why the surface is
not simply unexamined upstream — but no per-surface Handbook record follows from them.

No Handbook vector suite exists for `FACT`. The truncation rule, `FACT(0) = 1` and the negative
`#NUM!` are Microsoft's; the exactness and range boundaries are properties of binary64; nobody
has checked this function's values against Excel within the Handbook's record.

Inputs I would probe first:

1. **`FACT(170)` and `FACT(171)`.** The range boundary, answered as a kind. `EV-MATH-0008` names
   this pair as a witness for the FINITE convention, so this probe re-verifies a record rather
   than opening a question — but it is the cheapest possible confirmation.
2. **The exactness sweep `n = 20 … 30`.** Compare against exactly-computed factorials rounded
   once. Below 23 every implementation must be exact; from 23 upward a table and a loop start to
   diverge, and the *first* argument where they differ is a fingerprint of the algorithm.
3. **The full domain against a table.** All 171 values, compared against correctly rounded
   references. This is a complete characterisation of the surface — a rare opportunity, since
   the domain is finite and small — and it would settle the algorithm question outright.
4. **The truncation-versus-domain-check order**: `FACT(-0.5)`, `FACT(-0.9)`, `FACT(-1)`,
   `FACT(-1.5)`. One of these four returns 1 under one reading and `#NUM!` under the other.
   Nothing in the documentation decides it.
5. **`FACT(n)` against `GAMMA(n+1)`** across the domain. Two surfaces, one value; any
   disagreement localises to one of them and is a publishable finding either way.
6. **`FACT(n)` against `FACTDOUBLE(n) · FACTDOUBLE(n-1)`** — the free identity that ties this
   page to its sibling.
7. **`FACT(2^53)` and `FACT(1E300)`** — very large arguments, where the documented truncation
   rule and the range check meet, and where an implementation looping to `n` would not return at
   all.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| empty product | The convention that a product of no factors is 1, giving `0! = 1` |
| exactly representable | The result is a binary64 value with no rounding; here only for `n ≤ 22` |
| range boundary | `170!`, the largest factorial that is a finite double |
| tabulation | Storing every correctly rounded answer, feasible because the domain is finite |
| log-gamma route | Evaluating via `exp(lnΓ(n+1))`, accurate enough for distributions, not for this |
| truncation order | Whether the domain check sees the raw argument or the truncated one |

## Sources

- Microsoft, "FACT function" —
  <https://support.microsoft.com/en-us/office/fact-function-ca8588c2-15f2-41c0-8e8c-c11bd471a4f3>
  (fetched at curation: signature, the `1·2·3·…·number` definition, the truncation of
  non-integers, `FACT(0) = 1`, and the `#NUM!` for a negative argument).
- Handbook evidence record `EV-MATH-0008` — the FINITE error-code convention with named
  witnesses including a `FACT` pair, and its explicit statement that it publishes no count. Read
  its reader warning.
- Abramowitz & Stegun, chapter 6 — the gamma function and the factorial identities; chapter 24
  for the combinatorial context.
- Press et al., *Numerical Recipes* — `factrl` and `gammln`.
- Muller et al., *Handbook of Floating-Point Arithmetic* — exact representability of integers.
- Handbook, [FACTDOUBLE](FUNC.FACTDOUBLE.md), [COMBIN](FUNC.COMBIN.md),
  [COMBINA](FUNC.COMBINA.md);
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.FACT.json` (the `non_finite=num` axis value) and
  `data/presence/FUNC.FACT.json`.
