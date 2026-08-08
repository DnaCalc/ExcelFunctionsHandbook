---
schema: efh.function-page/v1
function_id: FUNC.COMBINA
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0017
  - EV-MATH-0016
open_problems: []
references:
  - work: "Microsoft Support — COMBINA function"
    locator: "https://support.microsoft.com/en-us/office/combina-function-efb49eaa-4f4c-4cd2-8179-0ddfcf9d035d"
    role: "documented signature, the argument constraints, the truncation rule, the two error conditions, and the printed equation"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 24 (Combinatorial Analysis)"
    role: "the binomial coefficient the multiset count reduces to"
  - work: "Feller, An Introduction to Probability Theory and Its Applications, volume I"
    locator: "chapter II (occupancy and the stars-and-bars argument)"
    role: "the combinatorial derivation of the with-repetition count"
  - work: "Press, Teukolsky, Vetterling & Flannery, Numerical Recipes"
    locator: "the `bico` / `factln` routines"
    role: "the exp(log-gamma) evaluation route identified as this surface's substrate"
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
family: combina
role_in_family: >-
  The with-repetition combination count — the multiset coefficient — and the member whose
  evaluation substrate has been identified as an exp(log-gamma) composition rather than a
  product.
---

# COMBINA

## What it computes

`COMBINA(number, number_chosen)` counts **combinations with repetition**: the number of ways to
choose `k` items from `n` types when the same type may be chosen more than once and order does
not matter. This is the multiset coefficient, written `((n choose k))` in Stanley's notation.

Microsoft's page prints the equation directly, with `N` for *number* and `M` for
*number_chosen*:

    COMBINA(N, M) = (N + M - 1)! / ( M! (N - 1)! )

which is the same thing as the binomial coefficient

    COMBINA(n, k) = C(n + k - 1, k) = C(n + k - 1, n - 1)

The identity is the *stars and bars* argument: a multiset of size `k` drawn from `n` types is
determined by where you place `n - 1` dividers among `k + n - 1` positions.

| Property | Statement |
|---|---|
| Domain (documented) | `n ≥ 0`, `k ≥ 0`, and `n ≥ k` |
| Range | positive integers |
| Reduction | `COMBINA(n, k) = C(n+k-1, k)` — a binomial coefficient, always |
| Boundary | `COMBINA(n, 0) = 1`; `COMBINA(1, k) = 1`; `COMBINA(n, 1) = n` |
| Generating function | `1/(1-x)^n = Σ_k COMBINA(n, k) x^k` |
| Symmetry | none in `(n, k)`; the underlying binomial's symmetry is `k ↔ n-1` |

One documentation observation belongs here rather than buried below, because it is a genuine
divergence between the mathematics and the published constraint. **The multiset coefficient is
defined for every `n ≥ 1` and every `k ≥ 0`, with no requirement that `n ≥ k`** — choosing five
scoops from three flavours is a perfectly ordinary question, and the answer is `C(7, 5) = 21`.
Microsoft's page nevertheless documents *Number* as having to be "greater than or equal to
Number_chosen", and documents `#NUM!` when an argument is outside its constraints. Either Excel
refuses a well-posed combinatorial question, or the documented constraint is stricter than the
implementation. The Handbook has not checked which, and it is the first probe listed below.

## Arguments

| Argument | Meaning | Constraint (as documented) |
|---|---|---|
| `number` (`N`) | The number of item **types** available. Required. | `≥ 0`, and `≥ number_chosen` |
| `number_chosen` (`M`) | The number of items chosen. Required. | `≥ 0` |

Microsoft states that **non-integer values are truncated** in both positions, and that a
non-numeric value in either position gives `#VALUE!`.

The reference engine records `UnaryNumericScalarOnly` as the coercion-and-lift profile for this
two-argument surface — the same projection state noted on [COMBIN](FUNC.COMBIN.md). Read
literally it declares no elementwise array lift; the projection does not say whether that is a
description of Excel or an artefact of classification, and neither does this page.

## Result and edge cases

Returns `Number`, mathematically always a positive integer.

- **`k = 0`** — one way to choose nothing: the value is 1.
- **`n = 0`** — the printed equation reads `(M-1)!/(M!(-1)!)`, which is undefined for `M > 0`;
  the documented constraint `N ≥ M` forces `M = 0` there, and `COMBINA(0, 0)` is conventionally
  1. What Excel actually returns at `(0, 0)` is a documented-constraint boundary the Handbook
  has not checked.
- **Growth** — `COMBINA(n, k)` is `C(n+k-1, k)`, so it crosses the `2^53` exact-integer
  threshold at modest arguments and overflows the double range not far beyond.
- **A returned value that is not an integer is itself a finding.** Every value of this function
  is an integer, so a fractional result cannot come from the mathematics. `EV-MATH-0017` turns
  exactly this observation into a substrate identification (below).

The real-result policy axis records `arg_domain_guard=none; non_finite=allow` for this surface,
which — as on `COMBIN` — sits uneasily beside a documented `#NUM!` domain condition. Recorded
here as a divergence, not resolved.

## Errors

As documented on Microsoft's `COMBINA` page:

| Error | Condition |
|---|---|
| `#NUM!` | The value of either argument is outside its documented constraints |
| `#VALUE!` | Either argument is a non-numeric value |

Note that the documented `#NUM!` condition is stated indirectly — "outside of its constraints"
— so the exact trigger set is only as precise as the constraint list above it, and that list
includes the `N ≥ M` requirement this page flags as mathematically unnecessary.

## Relationships

- **[COMBIN](FUNC.COMBIN.md)** — the without-repetition count. The bridge identity
  `COMBINA(n, k) = COMBIN(n+k-1, k)` means the two surfaces must agree on the same integer when
  their arguments are matched up. They have different identified substrates, so that identity
  is a metamorphic test rather than a tautology.
- **`PERMUTATIONA`** — with-repetition and *ordered*: simply `n^k`. `PERMUT` is the ordered
  without-repetition count. The four functions form the standard 2×2 of ordered/unordered
  against with/without repetition.
- **`GAMMALN`, `GAMMALN.PRECISE`, [EXP](FUNC.EXP.md)** — the identified substrate composition.
  `EV-MATH-0017` reduces this surface's remaining error to the log-gamma surface's, which is
  why the two pages should be read together.
- **`NEGBINOM.DIST`** — the distribution whose mass function is exactly a multiset coefficient
  times probabilities; the same evaluation problem appears there with a stability requirement.

## Numerical notes

The interesting fact about this surface is that its evaluation route has been **identified**,
and identified by a single decisive witness rather than by a sweep.

The reasoning is worth stating because it is a model of how substrate identification works.
`COMBINA(n, k)` is always an integer. A product of integers, evaluated in binary64, can round —
but it can only land on a representable neighbour of the true value, and while the true value
is below `2^53` the exact integer *is* representable, so a product route hits it. `EV-MATH-0017`
records a live witness in which Excel returns a value just below an exact integer that a product
route would have reached exactly. A product cannot produce that. An `exp(lnΓ)` composition can:
the logarithm loses the integrality, and the final exponentiation lands on whichever neighbour
the rounding chain reaches. The record therefore identifies the substrate as `exp(gammaln)` and
**reduces `COMBINA` to the log-gamma wall** — meaning its residual error is not its own problem
to solve but `GAMMALN`'s.

Two honest riders travel with that, both from the record itself: the reference implementation
does *not* implement the identified substrate — it ships a product kernel — and the formal
reduction to `GAMMALN` is a designed recipe that has not been executed.

What a careful implementation does when it wants the *intended* function rather than the
observed bits:

1. **Reduce to a binomial coefficient first**: `C(n+k-1, k)`, then apply the symmetry reduction
   `k ← min(k, n-1)`. Never form the three factorials of the printed equation — `(N+M-1)!`
   overflows at an argument sum of 171 while the answer may still be small.
2. **Use the multiplicative recurrence** `acc ← acc·(n-1+i)/i`, whose every partial product is
   itself a binomial coefficient and therefore an exact integer while it fits.
3. **Return the exact integer below `2^53`.** This is achievable and is the property the
   `exp(lnΓ)` route gives up.
4. **Above `2^53`, the log-gamma route becomes attractive** for range reasons and immediately
   inherits log-gamma's accuracy: an absolute error `ε` in `lnΓ` becomes a relative error of
   about `ε` in the result, and `lnΓ` of a large argument is large, so a small *relative* error
   in the logarithm is a much larger relative error in the answer. That amplification is the
   wall the record names, and it is the standard reason (A&S chapter 6; Numerical Recipes
   `bico`) that libraries which use `exp(factln)` document reduced accuracy for large binomials.

## What has not been checked

Two evidence records name this surface: `EV-MATH-0017` (the substrate identification, one
decisive witness, no pass count) and `EV-MATH-0016` (the joint `COMBIN`/`COMBINA` discrepancy
record). Both carry reader warnings; the joint record's mismatch figure covers the two surfaces
together and must not be read as a per-surface rate. There is **no per-surface pass count for
`COMBINA` anywhere**, and the record says so in those words.

No Handbook vector suite exists for `COMBINA`. The signature, the constraints, the truncation
rule, the two error conditions and the printed equation are Microsoft's; the substrate finding
is OxFunc's, re-derived into the Handbook's record; nothing else here is established behaviour.

Inputs I would probe first:

1. **`COMBINA(3, 5)`** — the documented-constraint question, and the cheapest experiment on this
   page. Mathematics says 21; the documented constraint says `#NUM!`. Whichever comes back, the
   Handbook has a finding: either the documentation over-constrains a well-posed function, or
   Excel really does refuse it. Extend with `COMBINA(1, 5)` and `COMBINA(0, 0)`.
2. **The integrality scan.** Sweep `(n, k)` with the true value below `2^53` and test
   `result = INT(result)`. Every failure is another witness for the `exp(lnΓ)` substrate, and
   the *set* of failures maps the wall far better than one witness does.
3. **The bridge identity** `COMBINA(n, k)` against `COMBIN(n+k-1, k)` over the same sweep. The
   two surfaces have different identified substrates, so systematic disagreement is expected —
   and the sign and location of the disagreement is the evidence.
4. **`GAMMALN` composition replay**: capture `GAMMALN` and `EXP` at the argument triples the
   identified substrate would use and compare against `COMBINA` directly. This is the recipe
   `EV-MATH-0017` designs and reports as not executed; executing it would close the reduction.
5. **The truncation rule on fractional and negative-fractional inputs**, and array arguments in
   each position, given the `UnaryNumericScalarOnly` profile.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| multiset coefficient | The with-repetition count `((n choose k)) = C(n+k-1, k)` |
| stars and bars | The bijection that proves the reduction to a binomial coefficient |
| substrate | The underlying numeric routine an implementation's answers come from |
| the log-gamma wall | The accuracy limit inherited by anything evaluated as `exp(lnΓ(·))` |
| decisive witness | A single input whose result excludes an entire family of candidate routes |
| bridge identity | `COMBINA(n, k) = COMBIN(n+k-1, k)`, usable as a cross-surface test |

## Sources

- Microsoft, "COMBINA function" —
  <https://support.microsoft.com/en-us/office/combina-function-efb49eaa-4f4c-4cd2-8179-0ddfcf9d035d>
  (fetched at curation: signature, the `N ≥ 0` / `N ≥ M` / `M ≥ 0` constraints, truncation of
  non-integer values, the `#NUM!` and `#VALUE!` conditions, and the printed equation).
- Handbook evidence records `EV-MATH-0017` (substrate identified as `exp(gammaln)`; the product
  kernel excluded; the reduction recipe designed and not executed) and `EV-MATH-0016` (the joint
  discrepancy record). Read their reader warnings before their figures.
- Feller, *An Introduction to Probability Theory and Its Applications*, volume I, chapter II —
  the occupancy/stars-and-bars derivation.
- Abramowitz & Stegun, chapter 24 — combinatorial analysis; chapter 6 for the log-gamma
  accuracy context.
- Press et al., *Numerical Recipes* — `bico`/`factln` and the stated large-argument caveat.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.COMBINA.json` and `data/presence/FUNC.COMBINA.json`.
