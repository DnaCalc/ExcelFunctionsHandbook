---
schema: efh.function-page/v1
function_id: FUNC.BINOM.DIST.RANGE
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
family: discrete_dist_family
role_in_family: >-
  The interval form of the binomial: the only member that answers a two-sided question directly
  instead of leaving the caller to difference two cumulative values.
---

# BINOM.DIST.RANGE

## What it computes

`BINOM.DIST.RANGE(trials, probability_s, number_s, [number_s2])` returns the probability that
the number of successes in \(n\) independent Bernoulli trials falls in a closed interval:

    P(s ≤ X ≤ s₂) = Σ_{k=s}^{s₂} C(n, k) · p^k · (1 − p)^{n−k}

with \(n =\) `trials`, \(p =\) `probability_s`, \(s =\) `number_s`, \(s_2 =\) `number_s2`. When
`number_s2` is omitted the interval collapses to a point and the function returns the single
mass \(b(s; n, p)\) — the same quantity as `BINOM.DIST(s, n, p, FALSE)`.

**Note the argument order.** `trials` comes *first* here, and `number_s` third. Every other
binomial surface in Excel puts the success count first. Transposing the first two arguments by
habit produces a call that is usually still valid and always wrong, which makes this the single
most error-prone signature in the discrete family.

**Domain.** \(n \ge 0\); \(0 \le p \le 1\); \(0 \le s \le s_2 \le n\); all numeric arguments
truncated to integers. Microsoft states each constraint in the argument descriptions themselves
rather than in the Remarks: `trials` "Must be greater than or equal to 0"; `probability_s`
"Must be greater than or equal to 0 and less than or equal to 1"; `number_s` "Must be greater
than or equal to 0 and less than or equal to Trials"; `number_s2` "Must be greater than or equal
to Number_s and less than or equal to Trials".

**Range.** \([0, 1]\), with the value 1 attained exactly when the interval covers the whole
support.

**Identities.**

    BINOM.DIST.RANGE(n, p, s, s₂) = B(s₂; n, p) − B(s−1; n, p)         difference of CDFs
    BINOM.DIST.RANGE(n, p, s)     = b(s; n, p)                          single mass
    BINOM.DIST.RANGE(n, p, 0, n)  = 1                                   normalisation
    BINOM.DIST.RANGE(n, p, s, s₂) = BINOM.DIST.RANGE(n, 1−p, n−s₂, n−s) reflection

The first identity is the one to be careful with. It is true in exact arithmetic and it is
**not** how the function is computed — not by Microsoft's documented description (which gives a
summation), and not by the reference engine. Differencing two cumulative probabilities is
exactly the cancellation this function exists to avoid: when both cumulative values are close to
1, their difference loses every significant digit, whereas the direct sum of the interval's
masses loses none. The existence of `BINOM.DIST.RANGE` as a separate function is a numerical
decision, not a convenience.

## Arguments

| Argument | Meaning | Admissible | Default |
|---|---|---|---|
| `trials` | Number of independent trials | \(\ge 0\), truncated | — |
| `probability_s` | Success probability per trial | \(0 \le p \le 1\) | — |
| `number_s` | Lower end of the success interval | \(0 \le s \le n\), truncated | — |
| `number_s2` | Upper end of the interval | \(s \le s_2 \le n\), truncated | `number_s` |

Microsoft describes `number_s2` as: "If provided, returns the probability that the number of
successful trials will fall between Number_s and number_s2." The reference engine declares an
arity of 3 to 4 and defaults `number_s2` to `number_s`, giving the single-mass reading above.

There is no `cumulative` flag — the interval form subsumes it.

Microsoft documents "Numeric arguments are truncated to integers", which on this page covers
`trials`, `number_s` and `number_s2`. `probability_s` is of course not truncated; the sentence
is a family-wide phrasing rather than a literal claim about every argument.

## Result and edge cases

Returns `Number`.

- **`number_s2` omitted.** A single mass. This is the reading most callers do not expect from
  the function's name, and it is the one that makes `BINOM.DIST.RANGE(n, p, k)` a synonym for
  `BINOM.DIST(k, n, p, FALSE)` — mathematically. Whether the two are the *same computation* is
  a separate question; see the numerical notes.
- **`number_s = 0`, `number_s2 = trials`.** The whole support; the true answer is exactly 1.
  The best available exactness probe.
- **`number_s = number_s2`.** A single mass, stated explicitly rather than by omission.
- **`p = 0` or `p = 1`.** Degenerate: all mass on \(k=0\) or \(k=n\). The reference engine
  routes these away from its direct combinatoric path.
- **`trials = 0`.** The only admissible interval is \([0,0]\), and the answer is 1.
- **Wide intervals with large `trials`.** The cost is \(O(s_2 - s)\) mass evaluations. There is
  no closed form and no early exit; a range over a million trials is a million evaluations.
- **Arrays.** The surface lifts natively (`lift_broadcast_profile: surface_native`). There is no
  legacy alias for this function — it has no pre-2010 counterpart — so the alias questions that
  attach to the rest of the family do not arise here.

## Errors

As documented by Microsoft on the `BINOM.DIST.RANGE` page:

| Error | Condition |
|---|---|
| `#NUM!` | Any argument is outside its stated constraints |
| `#VALUE!` | Any argument is non-numeric |

That is unusually compact for this family: rather than enumerating conditions, the page defers
to the constraints written into the argument descriptions. Expanded, the `#NUM!` conditions are
\(n < 0\), \(p < 0\), \(p > 1\), \(s < 0\), \(s > s_2\), and \(s_2 > n\).

The reference engine at `473efa3` implements exactly that expansion, and adds `#NUM!` for a
non-finite numeric argument.

## Relationships

- **[BINOM.DIST](FUNC.BINOM.DIST.md)** — the mass and cumulative forms. `BINOM.DIST.RANGE(n, p,
  0, k)` and `BINOM.DIST(k, n, p, TRUE)` are the same number; `BINOM.DIST.RANGE(n, p, k)` and
  `BINOM.DIST(k, n, p, FALSE)` are the same number. In the reference engine they are **not the
  same computation**, which makes the pair a free differential probe — see below.
- **[BINOM.INV](FUNC.BINOM.INV.md)** — the quantile of the same distribution.
- **`BINOMDIST` / `CRITBINOM`** — the legacy names for the mass/cumulative and quantile forms.
  `BINOM.DIST.RANGE` has no legacy counterpart; it was introduced with the dotted-name
  generation, which is why its argument order follows no older convention.
- **[BETA.DIST](FUNC.BETA.DIST.md)** — the incomplete-beta identity gives a closed-form route to
  the same interval probability as a difference of two \(I_p\) values, which for very wide
  intervals is \(O(1)\) rather than \(O(s_2-s)\).
- **`POISSON.DIST`, `NORM.DIST`** — the limiting approximations, and the usual (inaccurate)
  substitutes for a wide-interval binomial probability.
- **Confused with**: `BINOM.DIST` with the arguments in the same order. They are not in the
  same order. This is the relationship most worth remembering.

## Numerical notes

**Why the function exists.** Given the two cumulative values, the interval probability is their
difference. When both are near 1 — an interval in the upper tail — that difference is
catastrophic cancellation: two numbers agreeing in the leading digits are subtracted and the
result is built entirely from the digits that were least reliable. Summing the interval's own
masses instead is a sum of positive quantities, and a sum of positive quantities has no
cancellation at all. Its relative error is bounded by the ordinary summation bound
(\(\gamma_{m-1}\) for \(m\) terms; Higham, *Accuracy and Stability of Numerical Algorithms*,
chapter 4), which for realistic interval widths is negligible.

That is the whole numerical argument for the surface, and it is a good one. It is also why an
implementation that quietly computes this function as a difference of CDFs would be *wrong*
precisely in the regime the function was added for.

**What the reference engine does, and why it matters here.** At `473efa3` the reference engine
takes **two different paths** depending on `trials`:

- For `trials` at or below a fixed small threshold, and with \(p\) strictly between 0 and 1, it
  sums masses computed by a **direct combinatoric** form — building \(C(n,k)\) by a running
  product of ratios and multiplying by integer powers of \(p\) and \(q\).
- Otherwise it sums masses computed by the general saddle-point path described on
  [BINOM.DIST](FUNC.BINOM.DIST.md#numerical-notes).

The two paths do not agree in the last bits. The direct combinatoric form has the virtue of
being exact for small \(n\) (every \(C(n,k)\) below \(2^{53}\) is an exact integer, and the
running-product construction keeps it so if the divisions are exact at each step), and the vice
of overflowing for larger \(n\). The reference engine's own known-exactness-deviation register
lists a `BINOM.DIST.RANGE` "finite sum" change among the partial repairs landed under its open
statistical-exactness entry, which is the record of this path having been deliberately chosen.

**The consequence you can act on.** Because `BINOM.DIST.RANGE(n, p, k)` and
`BINOM.DIST(k, n, p, FALSE)` are mathematically identical but take different paths in the
reference engine, comparing them is a **free differential probe of the mass algorithm** — no
oracle needed, no high-precision reference needed. Any disagreement localises immediately to
the path split. This is the cheapest diagnostic anywhere in the discrete family and it is the
first thing to run.

**Summation order.** The reference engine accumulates ascending in \(k\). For an interval in
the upper tail the masses are increasing, so ascending order accumulates small terms first —
which is the favourable direction. For an interval in the lower tail the masses are decreasing
and ascending order is the unfavourable direction. A careful implementation would sort or
accumulate from the smaller end; a reproducible one must fix the order and say so.

**Cost.** \(O(s_2 - s)\) mass evaluations with no shortcut. For a wide interval the
incomplete-beta route is asymptotically better, and for an interval covering most of the support
the complement (1 minus the two excluded tails) is better still — at the cost of reintroducing
exactly the cancellation this function avoids. There is no universally right choice, which is
itself worth stating.

## What has not been checked

**No Handbook vector suite exists for `BINOM.DIST.RANGE`, and no evidence record lists this
surface among its subjects.** It appears inside the reference engine's open defect stream
`BUG-FUNC-021` and inside the open `KED-STAT-001` known-exactness entry — in both cases as a
*repair note* naming a change that landed, not as a measurement of this surface. Those are
records that work was done, not records of a result. The documented statements above come from
Microsoft's `BINOM.DIST.RANGE` page, fetched while this page was written; the rest is
mathematics or the reference engine at commit `473efa3`, named as such.

Inputs worth probing first:

1. **`BINOM.DIST.RANGE(n, p, k)` against `BINOM.DIST(k, n, p, FALSE)`** across a grid of \(n\),
   \(k\) and \(p\), spanning the reference engine's path threshold in `trials`. Mathematically
   identical, differently computed, no oracle required. Highest-value probe on the page.
2. **`BINOM.DIST.RANGE(n, p, 0, n)`**, where the true answer is exactly 1. Any departure is
   pure accumulated rounding and reads off the quality of the summation directly.
3. **`BINOM.DIST.RANGE(n, p, 0, k)` against `BINOM.DIST(k, n, p, TRUE)`** — the cumulative form
   of the same comparison, which additionally exercises the reference engine's two different
   accumulation loops.
4. **An upper-tail interval** with both endpoint CDFs close to 1, compared against the explicit
   difference `BINOM.DIST(s₂,…,TRUE) - BINOM.DIST(s−1,…,TRUE)`. If the two agree to full
   precision, the function is being computed by differencing and the cancellation argument
   applies to it.
5. **The argument-order trap**: `BINOM.DIST.RANGE(10, 0.5, 3)` against
   `BINOM.DIST.RANGE(3, 0.5, 10)` — the second is out of domain and should be `#NUM!`.
   Confirming that the wrong order errors rather than silently answering is worth one cell.
6. **The reflection identity** `BINOM.DIST.RANGE(n, p, s, s₂)` against
   `BINOM.DIST.RANGE(n, 1−p, n−s₂, n−s)`, exact in mathematics and a pure symmetry test.
7. **`p = 0` and `p = 1`** at intervals that do and do not contain the supported point, where
   the answers are exactly 0 and exactly 1.
8. **The truncation rule** on all three integer slots — `BINOM.DIST.RANGE(10.9, 0.5, 2.9, 5.9)`
   against the truncated call — since Microsoft's phrasing "Numeric arguments are truncated to
   integers" is loose enough to be worth confirming.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| interval probability | \(P(s \le X \le s_2)\); the quantity this function returns |
| direct combinatoric form | Building \(C(n,k)\) by a running product and multiplying by integer powers |
| path split | The reference engine's choice between the direct form and the saddle-point form, keyed on `trials` |
| differential probe | Comparing two surfaces that must agree mathematically but are computed differently |
| complement-formation loss | The precision destroyed by differencing two nearly equal cumulative values |
| reference engine | OxFunc at commit `473efa3` |

## Sources

- Microsoft, "BINOM.DIST.RANGE function" —
  <https://support.microsoft.com/en-us/office/binom-dist-range-function-17331329-74c7-4053-bb4c-6653a7421595>
  (syntax and argument order; the four argument descriptions with their inline constraints; the
  out-of-constraint `#NUM!`; the non-numeric `#VALUE!`; the truncation remark; and the summation
  equation over \(k\) from `Number_s` to `Number_s2`).
- OxFunc `crates/oxfunc_core/src/functions/discrete_dist_family.rs` at commit `473efa3` — the
  arity and default for `number_s2`, the domain checks, the `trials`-keyed split between the
  direct combinatoric lane and the general mass path, and the ascending accumulation.
- OxFunc `docs/KNOWN_EXACTNESS_DEVIATIONS.md`, entry `KED-STAT-001` (status open, owner
  `BUG-FUNC-021`), which lists a `BINOM.DIST.RANGE` finite-sum change among landed partial
  repairs — cited as a record that work occurred, not as a measurement; its counts are upstream
  figures and are not restated here.
- Handbook, [BINOM.DIST](FUNC.BINOM.DIST.md) — the mass algorithm this function sums, and the
  evidence records that do list a binomial surface among their subjects.
- M. Abramowitz and I. A. Stegun, *Handbook of Mathematical Functions*, chapter 26.1 (binomial
  distribution) and 26.5.24 (the incomplete-beta route to binomial tails).
- N. J. Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd ed., chapter 4 — the
  summation bound quoted for the positive-terms accumulation.
- M. Welinder's work on the Gnumeric statistical functions and the published critiques of
  spreadsheet statistical accuracy.
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md`.
