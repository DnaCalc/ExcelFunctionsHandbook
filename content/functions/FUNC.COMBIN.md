---
schema: efh.function-page/v1
function_id: FUNC.COMBIN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0016
  - EV-MISC-0015
open_problems: []
references:
  - work: "Microsoft Support — COMBIN function"
    locator: "https://support.microsoft.com/en-us/office/combin-function-12a3f276-0a21-423a-8de6-06990aaf638a"
    role: "documented signature, integer truncation, the #VALUE! and #NUM! conditions, and the n!/(k!(n-k)!) formula"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 24 (Combinatorial Analysis), 24.1.1"
    role: "the binomial coefficient, its recurrences and its symmetry"
  - work: "Knuth, The Art of Computer Programming, volume 1"
    locator: "section 1.2.6"
    role: "the standard identity set for binomial coefficients, including the symmetry reduction"
  - work: "Press, Teukolsky, Vetterling & Flannery, Numerical Recipes"
    locator: "the `bico` / `factln` routines"
    role: "the exp(lgamma) evaluation route and its stated accuracy trade-off"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Notes
  - Excel versions, platforms and CPUs
  - Page vocabulary
  - Sources
family: combin
role_in_family: >-
  The binomial coefficient: the number of unordered selections of k items from n distinct items
  without repetition, and the anchor against which COMBINA's with-repetition variant is read.
---

# COMBIN

## What it computes

`COMBIN(number, number_chosen)` is the binomial coefficient. Writing `n` for *number* and `k`
for *number_chosen*, Microsoft's page states the defining formula directly:

    C(n, k) = n! / ( k! (n-k)! )

It counts the subsets of size `k` of a set of size `n` — sets, so internal order does not
matter. Microsoft's page draws exactly that contrast: combinations are distinct from
permutations, "for which the internal order is significant".

The properties that matter for reading and for implementing:

| Property | Statement |
|---|---|
| Domain | integers with `0 ≤ k ≤ n` (see Arguments for the truncation rule) |
| Range | positive integers; `C(n, k) ≥ 1` on the whole domain |
| Symmetry | `C(n, k) = C(n, n-k)` |
| Pascal recurrence | `C(n, k) = C(n-1, k-1) + C(n-1, k)` |
| Multiplicative recurrence | `C(n, k) = C(n, k-1) · (n-k+1) / k` |
| Row sum | `Σ_k C(n, k) = 2^n` |
| Boundary | `C(n, 0) = C(n, n) = 1`, and `C(0, 0) = 1` |
| Generating function | `(1 + x)^n = Σ_k C(n, k) x^k` |

Two consequences shape everything downstream. First, **the true value is always an integer**,
so any returned value with a fractional part is evidence about the evaluation route rather than
about the mathematics. Second, the values grow fast: the central coefficient `C(n, ⌊n/2⌋)` is
asymptotically `2^n / √(πn/2)`, so it passes `2^53` — the last integer beyond which binary64
cannot represent every integer — in the high fifties, and passes the largest finite double
somewhat past `n = 1000`. Every interesting question about this function lives above the first
threshold and below the second.

## Arguments

| Argument | Meaning | Constraint (as documented) |
|---|---|---|
| `number` (`n`) | The number of items. Required. | `n ≥ 0`, `n ≥ k` |
| `number_chosen` (`k`) | The number of items in each combination. Required. | `k ≥ 0` |

Microsoft states two admission rules on the page: **numeric arguments are truncated to
integers**, and a nonnumeric argument gives `#VALUE!`. Truncation, not rounding — so a
`number` of `5.9` is documented to behave as `5`.

Both positions are numeric slots subject to ordinary to-number coercion; see
[Coercion and lifting](../model/02-coercion-and-lifting.md). Note a projection state worth
recording rather than smoothing over: the reference engine classifies this two-argument
function with the `UnaryNumericScalarOnly` coercion-and-lift profile, where the elementwise
trigonometric surfaces carry `UnaryNumericScalarOrArrayElementwise`. Read literally, that
declares no elementwise array lift for either argument position. Whether Excel lifts `COMBIN`
over array arguments is not settled by the projection and has not been checked here.

## Result and edge cases

Returns `Number`, mathematically always a positive integer.

- **`k = 0` or `k = n`** — the value is 1, including the vacuous `C(0, 0) = 1`.
- **Non-integer arguments** — documented to truncate toward zero before the count is formed.
- **Results above `2^53`** — the exact combinatorial answer is no longer representable as a
  binary64 integer, so *some* rounding is unavoidable in any implementation. What is not
  unavoidable is *which* representable neighbour you land on, and that is the whole content of
  the evidence attached to this page.
- **Results above the largest finite double** — the mathematics overflows. The reference
  engine's real-result policy for this surface records `non_finite=allow`, meaning a non-finite
  result would be published rather than converted to an error. That is a divergence worth
  naming: `EV-MATH-0008` records the "Excel never publishes an infinity" convention for a
  named list of surfaces, and `COMBIN` is not on that list. Nobody has checked what Excel
  returns when the binomial coefficient overflows.

Logicals, text-that-looks-numeric, blanks and error arguments follow the shared call model; the
mechanically rendered battery beside this page shows the boundary probes for those kinds.

## Errors

As documented on Microsoft's `COMBIN` page:

| Error | Condition |
|---|---|
| `#VALUE!` | Either argument is nonnumeric |
| `#NUM!` | `number < 0`, or `number_chosen < 0`, or `number < number_chosen` |

The reference engine's real-result policy axis for this surface records
`arg_domain_guard=none`. The documented `#NUM!` conditions are argument-domain conditions, so
either the guard lives somewhere other than that axis or the axis and the documentation
disagree. The projection does not say which, and the Handbook has not resolved it.

## Relationships

- **[COMBINA](FUNC.COMBINA.md)** — the with-repetition sibling, `COMBINA(n, k) = C(n+k-1, k)`.
  The two are joined by that single identity and separated by very different evidence: see the
  numerical notes on both pages.
- **`PERMUT`** — the ordered count, `P(n, k) = n! / (n-k)!`, related by
  `P(n, k) = C(n, k) · k!`. `PERMUTATIONA` is its with-repetition partner, `n^k`.
- **[FACT](FUNC.FACT.md)** — the factorial the documented formula is written in. Evaluating
  `COMBIN` *as* that formula is the one implementation choice you should not make; see below.
- **`GAMMALN`, `GAMMALN.PRECISE`** — the log-gamma route, `C(n,k) = exp(lnΓ(n+1) − lnΓ(k+1) −
  lnΓ(n-k+1))`, which is how many libraries evaluate large binomials and which is identified
  as the substrate for `COMBINA` but *not* for `COMBIN`.
- **`HYPGEOM.DIST`, `BINOM.DIST`** — the distribution functions whose mass functions are
  binomial coefficients times probabilities; they usually need better than a `COMBIN` product.

## Numerical notes

The naive route — form `n!`, `k!` and `(n-k)!`, then divide — is the formula Microsoft prints
and the algorithm nobody should ship. It overflows at `n = 171` even when the answer is tiny
(`C(200, 1)` is 200), and it throws away exactness long before that.

The standard remedies, in the order a careful implementation applies them:

1. **Reduce by symmetry**: replace `k` with `min(k, n-k)`. This halves the worst-case work and,
   more importantly, shortens the product that accumulates rounding. `EV-MATH-0016` records
   this reduction as a *confirmed structural finding about Excel* — one of the few things about
   this surface that is settled.
2. **Use the multiplicative recurrence with the division inside the loop**:
   `acc ← acc · (n-k+i) / i` for `i = 1 … k`. Each partial product is itself a binomial
   coefficient, hence an exact integer while it fits, so the intermediate never overflows
   earlier than the result does. The alternative "multiply everything, divide once" ordering
   overflows spuriously.
3. **Return exact integers where they fit.** Below `2^53` the answer is representable and an
   integer-arithmetic path can return it exactly.
4. **Above `2^53`, choose and document a rounding discipline.** This is where implementations
   diverge, and it is where the evidence sits.

The evidence attached to this page — `EV-MATH-0016` and `EV-MISC-0015` — records a capture
built specifically to *discriminate* between candidate op-graphs, and its result is negative in
an unusually informative way. An earlier claim that this surface agreed with Excel below `2^53`
was **withdrawn**, because it rested on a corpus that could not tell the candidates apart. On
the discriminating pairs, the ruled-out list covers every product-loop family in both
directions, strict and per-step double-rounded and fully extended orderings, factorial-ratio
forms, reciprocal-multiply forms, and compositions of the published `GAMMALN`. Plain-double and
x87 extended-precision product orderings score identically. The record states plainly that the
op-graph is unidentified, that a subset of the discriminating rows is matched by no candidate
form at all, and that Excel sits *below* the multiply-first product at larger `n` and `k`. The
figures belong to the evidence layer and render beside this page; the shape of the finding is
the part that belongs in prose.

The literature context: A&S chapter 24 gives the identity set; Knuth volume 1 §1.2.6 is the
standard source for the symmetry-and-multiplicative-recurrence discipline; Numerical Recipes'
`bico` uses the `exp(factln)` route and states its own accuracy caveat, which is exactly the
route ruled out for this surface and identified for its sibling.

## What has not been checked

Two evidence records name this surface: `EV-MATH-0016` (an open discrepancy, with a withdrawn
prior claim) and `EV-MISC-0015` (the same capture scored on the shipping kernel). Both carry
reader warnings; both are about a deliberately adversarial corpus rather than a random sample,
and neither is a general pass rate. `COMBIN` also appears inside group counts elsewhere that
must not be attributed to it.

No Handbook vector suite exists for `COMBIN`. The documented truncation rule, the two error
conditions and the printed formula are Microsoft's; nothing else on this page is documented
behaviour, and the op-graph question is open.

Inputs I would probe first:

1. **The overflow boundary.** `COMBIN(1030, 515)` and neighbours, straddling the largest finite
   double. This is the cheapest test of whether the `non_finite=allow` axis or the
   never-publish-an-infinity convention describes this surface, and the answer is a kind, not a
   number, so it settles cleanly.
2. **The symmetry pair across the `2^53` line**: `COMBIN(n, k)` against `COMBIN(n, n-k)` for
   several `n` well above the exactness threshold. The reduction to `min(k, n-k)` is confirmed,
   so these must agree to the last bit; a disagreement would retract a settled finding.
3. **Rows the discriminating capture could not explain.** The record says a subset of pairs is
   matched by no candidate form; those exact pairs, replayed on a second Excel build and a
   second CPU, separate "an op-graph nobody has guessed" from "a host-dependent last bit".
4. **The truncation rule at negative fractions**: `COMBIN(5.9, 2.9)` and `COMBIN(-0.5, 0)`.
   Truncation toward zero and floor differ on negatives, and `-0.5` truncates to `0`, which is
   inside the documented domain while `-0.5` is outside it.
5. **Array arguments in each position**, given the `UnaryNumericScalarOnly` profile recorded
   for a two-argument function.
6. **`COMBIN(n, k)` against `COMBINA(n-k+1, k)`** — the same integer by two routes with two
   different identified substrates. Any disagreement localises the error to one of them.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `COMBIN` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-COMBIN-001: COMBIN and COMBINA return 83.99999999999999 (1 ULP below 84) for C(9, 3) via cyclic x87-stored factors with * n last

*Excel differs from the mathematical answer; measured; recorded 2026-11-22.* Applies to COMBIN, COMBINA.

In Excel 16.0 build 20228, COMBIN(n, k) truncates n and k (with subnormals treated as signed
zero and n <= 2,147,483,646), reduces k = min(k, n - k), and evaluates product(i=2..=k,
(n-k+i-1)/i) * n, storing every division and accumulator product from x87 PC64 to binary64 and
multiplying n last. Because 7/2 * 8/3 * 9 divides by 3 before multiplying by 9, COMBIN(9, 3)
and COMBINA(7, 3) return 83.99999999999999 (0x4054ffffffffffff) instead of exact 84
(0x4055000000000000). COMBINA truncates its two arguments independently before forming total =
trunc(n) + trunc(k) - 1 and evaluates trunc(n) == 0 && trunc(k) == 0 -> 1 before checking
negative k, so COMBINA(-0.25, 0.75) returns 1.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `COMBIN(9, 3)` | number `9.0` (`0x4022000000000000`); number_chosen `3.0` (`0x4008000000000000`) (cells) | `83.99999999999999` (`0x4054ffffffffffff`) | `84.0` (`0x4055000000000000`) | Excel 16.0 build 20228, Windows 64-bit, workbook compatibility 2; CPU not recorded |
| `COMBINA(7, 3)` | number `7.0` (`0x401c000000000000`); number_chosen `3.0` (`0x4008000000000000`) (cells) | `83.99999999999999` (`0x4054ffffffffffff`) | `84.0` (`0x4055000000000000`) | Excel 16.0 build 20228, Windows 64-bit, workbook compatibility 2; CPU not recorded |
| `COMBINA(-0.25, 0.75)` | number `-0.25` (`0xbfd0000000000000`); number_chosen `0.75` (`0x3fe8000000000000`) (cells) | `1.0` (`0x3ff0000000000000`) | — | Excel 16.0 build 20228, Windows 64-bit, workbook compatibility 2; CPU not recorded |

Correct value: Exact integer combinatorics: C(9, 3) = 9*8*7 / 6 = 84.
Correct value: Exact integer combinatorics: C(7+3-1, 3) = C(9, 3) = 84.

Basis (measured): Measured across 20,713 COMBIN discovery rows, 1,024 cyclic held-out rows, and
35,233 COMBINA identity rows in Excel 16.0 build 20228 (CalcFunc-Evidence
work/w109/G4-04-combin/ and G4-04-combina/), and matched on 25,199/25,199 COMBIN and
40,991/40,991 COMBINA full-bank rows.

Evidence:

- DnaCalc `crates/calc-func/src/ported/combin.rs`, combin_kernel at `95eb0cb2ef` — Cyclic x87-stored factor recurrence with * n last and 2,147,483,646 limit
- DnaCalc `crates/calc-func/src/ported/combina.rs`, combina_kernel at `95eb0cb2ef` — Pre-total truncation and zero/zero guard order
- DnaCalc `crates/calc-func/witness-status.tsv`, COMBIN; COMBINA at `95eb0cb2ef` — 25,199/25,199 COMBIN and 40,991/40,991 COMBINA matches
- CalcFunc-Evidence `work/w109/G4-04-combin/answers-current-discovery-v1-excel.json.gz`, tri-n0009-k0003 at `4ef2c244d7` (readings)
- CalcFunc-Evidence `work/w109/G4-04-combina/answers-combina-identity-discovery-v1-excel.json.gz`, combina-discovery-00048-dense_transformed_triangle-t0009-k0003 at `4ef2c244d7` (readings)
- CalcFunc-Evidence `work/w109/G4-04-combina/answers-combina-admission-boundary-discovery-v1-excel.json.gz`, combina-boundary-fractional-guard-order-n-bfd0000000000000-k-3fe8000000000000 at `4ef2c244d7` (readings)

<!-- efh:notes end -->

## Excel versions, platforms and CPUs

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.COMBIN.json; edits here are overwritten -->

Readings of COMBIN with a recorded Excel build: 25,361 input(s) over 4 recorded environment(s),
Excel 16.0 build(s) 19929, 20131, 20228, 20430, all on Windows; recorded CPUs: AMD Zen 2. 503
input(s) were also read in captures that record no build (some may repeat inputs counted
above); they are not counted. 6 same-input comparison(s) between environments over 626
input-pair(s): no result differs. No reading from a Mac, Excel for the web, 32-bit Excel or an
Intel CPU is recorded.

Readings of `COMBIN` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 148 | date not recorded |
| 16.0 build 20131 | Windows, bitness not recorded | AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | 503 | 2026-07-11 |
| 16.0 build 20228 | Windows, 64-bit | CPU not recorded | 23,468 | 2026-08-09 |
| 16.0 build 20430 | Windows, 64-bit | CPU not recorded | 1,242 | 2026-09-29 |

Compared on the same inputs:

| Check | Differs in | Environments | Inputs compared | Inputs with different results |
|---|---|---|---:|---:|
| XC-COMBIN-001 | excel-build | 16.0 build 19929 vs 16.0 build 20131 | 4 | 0 |
| XC-COMBIN-002 | excel-build | 16.0 build 19929 vs 16.0 build 20228 | 103 | 0 |
| XC-COMBIN-003 | excel-build | 16.0 build 19929 vs 16.0 build 20430 | 2 | 0 |
| XC-COMBIN-004 | excel-build | 16.0 build 20131 vs 16.0 build 20228 | 503 | 0 |
| XC-COMBIN-005 | excel-build | 16.0 build 20131 vs 16.0 build 20228 | 503 | 0 |
| XC-COMBIN-006 | excel-build | 16.0 build 20228 vs 16.0 build 20430 | 14 | 0 |

Differences observed: none.

Evidence for every row: [`content/evidence/environments/FUNC.COMBIN.json`](../../content/evidence/environments/FUNC.COMBIN.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| binomial coefficient | The count `C(n, k)` of `k`-subsets of an `n`-set |
| symmetry reduction | Replacing `k` by `min(k, n-k)` before evaluating |
| multiplicative recurrence | The loop `acc ← acc·(n-k+i)/i`, which keeps every partial exact |
| discriminating corpus | Inputs chosen so that candidate op-graphs give different answers |
| op-graph | The exact sequence of floating-point operations an implementation performs |
| exactness threshold | `2^53`, above which not every integer is a binary64 value |

## Sources

- Microsoft, "COMBIN function" —
  <https://support.microsoft.com/en-us/office/combin-function-12a3f276-0a21-423a-8de6-06990aaf638a>
  (fetched at curation: the signature, integer truncation, the `#VALUE!` and `#NUM!`
  conditions, the combinations-versus-permutations contrast, and the printed formula).
- Handbook evidence records `EV-MATH-0016` and `EV-MISC-0015` — the discriminating capture, the
  withdrawn prior claim, the confirmed `k → min(k, n-k)` reduction, and the ruled-out candidate
  families. Read their reader warnings before reading their counts.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 24 — combinatorial
  analysis and the binomial identity set.
- Knuth, *TAOCP* volume 1 §1.2.6 — binomial coefficient identities and evaluation discipline.
- Press et al., *Numerical Recipes* — `bico`/`factln`, the exp(log-gamma) route.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [The value universe](../model/01-value-universe.md).
- Handbook projections `data/functions/FUNC.COMBIN.json` (arity, classification, the
  `arg_domain_guard=none` and `non_finite=allow` axis values) and
  `data/presence/FUNC.COMBIN.json` (implementing module, defect-stream mentions).
