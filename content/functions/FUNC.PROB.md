---
schema: efh.function-page/v1
function_id: FUNC.PROB
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — PROB function"
    locator: "https://support.microsoft.com/en-us/office/prob-function-9ac30561-c81c-4259-8253-34f0a238fc49"
    role: "documented signature, the omitted-upper_limit rule, and the documented #NUM! and #N/A conditions"
  - work: "OxFunc — lookup_prob_frequency_family.rs"
    locator: "crates/oxfunc_core/src/functions/lookup_prob_frequency_family.rs"
    role: "reference-engine kernel: admissibility guards, the sum-to-one test, and the closed-interval accumulation"
episodes: []
body_sections:
  - "What it computes"
  - "Arguments"
  - "Result and edge cases"
  - "Errors"
  - "Relationships"
  - "Numerical notes"
  - "What has not been checked"
  - "Page vocabulary"
  - "Sources"
family: lookup_prob_frequency_family
role_in_family: >-
  The family's probability-measure member: it reads a finite discrete distribution out of two
  parallel vectors and returns the mass of a closed interval.
---

# PROB

## What it computes

`PROB(x_range, prob_range, lower_limit, [upper_limit])` treats its two vector arguments as a
**finite discrete probability distribution** and returns the mass that distribution assigns to a
closed interval.

Write the paired vectors as `(x_1, p_1), …, (x_n, p_n)`. They define the discrete measure

    mu  =  sum_{i=1..n}  p_i * delta_{x_i}

on the real line, where `delta_x` is the unit point mass at `x`. `PROB` returns

    PROB(x, p, a, b)  =  mu([a, b])  =  sum over { i : a <= x_i <= b } of p_i

Two features of that definition carry the page:

1. **The interval is closed at both ends.** The accumulation admits `x_i` when
   `a <= x_i` and `x_i <= b`. So `PROB` computes `P(a <= X <= b)`, not `P(a < X <= b)` and not
   `P(a <= X < b)`. On a discrete distribution the difference is a whole atom, not a
   measure-zero technicality: it is exactly `p` at the endpoint.
2. **Omitting `upper_limit` collapses the interval to a point.** The documented behaviour is
   that `PROB` then returns the probability of being *equal to* `lower_limit` — the point mass
   `mu({a}) = P(X = a)`. In the reference engine this is implemented by setting `b := a`, which
   makes the point form a special case of the interval form rather than a separate branch.

The range of the function is `[0, 1]`, and it is monotone in the interval: widening `[a, b]`
never decreases the result. `PROB` is a probability *lookup*, not a distribution fit — it does
no smoothing, no interpolation between the `x_i`, and no normalization. A value of `lower_limit`
that lies strictly between two atoms contributes nothing.

`x_range` is not required to be sorted, and the atoms are not required to be distinct. If the
same `x` appears twice with different probabilities, both are admitted and both are counted;
`PROB` sums over index positions, not over distinct support points.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `x_range` | The support: the numeric values `X` can take. Required. | — |
| `prob_range` | The probabilities attached to those values, positionally. Required. | — |
| `lower_limit` | The lower bound of the interval, inclusive. Required. | — |
| `upper_limit` | The upper bound, inclusive. Optional. | `lower_limit` |

The pairing is **positional**, element by element in row-major order over each argument. That is
the whole of the correspondence: `x_range` and `prob_range` are two columns of one table, and
nothing checks that they were meant to line up beyond their having the same number of elements.

`prob_range` is documented to be a genuine probability vector: each entry in `(0, 1]`, and the
entries summing to 1. Both conditions are stated by Microsoft as `#NUM!` conditions rather than
as advice, which makes `PROB` unusually strict for a worksheet function — most of Excel will
compute happily on nonsense.

`lower_limit` and `upper_limit` are ordinary numeric slots subject to
[coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number` in `[0, 1]`.

- **No atom falls in the interval.** The sum over an empty index set is `0`. `PROB` returns zero,
  not an error and not `#N/A`. A caller cannot distinguish "the interval genuinely has zero mass"
  from "the interval missed every atom" — they are the same statement about the distribution.
- **`upper_limit` below `lower_limit`.** The reference engine rejects this with `#NUM!` rather
  than returning the empty-interval answer of zero. Microsoft's page does not state a rule for an
  inverted interval, so the two defensible readings — `#NUM!` and `0` — are both live, and the
  Handbook has not observed which one Excel takes.
- **A zero probability in `prob_range`.** The reference engine admits `p_i = 0` and rejects only
  `p_i < 0` and `p_i > 1`. Microsoft's documented condition is stated over `p_i <= 0`. See
  Errors below; this is a divergence the Handbook is recording rather than resolving.
- **Non-numeric values in either vector.** The reference engine's `PROB` path collects both
  vectors in strict mode: a text, logical or blank cell in `x_range` or `prob_range` surfaces
  `#VALUE!` rather than being skipped. That is the opposite of the ignore-text scan policy most
  Excel aggregates use on ranges (see
  [coercion and lifting](../model/02-coercion-and-lifting.md)), and it is not something Microsoft's
  page addresses.
- **A two-dimensional `x_range`.** The reference engine accepts a single row or a single column
  and rejects a genuinely rectangular grid with `#REF!`. Undocumented.
- **Errors in the vectors** propagate as themselves.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#N/A` | `x_range` and `prob_range` contain a different number of data points | documented |
| `#NUM!` | A value in `prob_range` is out of the admissible probability range | documented, with a boundary divergence — see below |
| `#NUM!` | The values in `prob_range` do not sum to 1 | documented, with a tolerance divergence — see below |
| `#NUM!` | `upper_limit` is less than `lower_limit`, or a limit is non-finite | reference engine only; not documented |
| `#VALUE!` | A non-numeric value appears in either vector | reference engine only; not documented |
| `#REF!` | `x_range` is a rectangular grid rather than a vector | reference engine only; not documented |
| `#N/A` | Both vectors are empty | reference engine |

Two documentation-versus-reference-engine divergences are worth stating precisely, because they
are the two places where a workbook that Excel accepts and one that Excel rejects part company:

**The zero-probability boundary.** Microsoft's documented condition covers probabilities that
are less than *or equal to* zero. The reference engine's guard rejects only strictly negative
probabilities, so a distribution containing an impossible outcome written as `0` is admitted
there and — on the documented reading — refused by Excel. A support point with probability zero
is mathematically harmless: it contributes nothing to any interval and does not disturb the
sum-to-one condition. Whichever side is right, the two do not agree, and a `prob_range` built by
a formula that can legitimately produce zeros is exactly where a reader will meet it.

**The sum-to-one test.** Microsoft states the condition as an equality. The reference engine
tests it as an absolute tolerance around 1. A tolerance is not a liberty here — it is a
necessity, because a hand-written probability vector of ten tenths does not sum to exactly `1`
in binary64. Microsoft's page names no tolerance, so the width of Excel's own acceptance window,
if it has one, is undocumented and unmeasured. This is a genuinely reachable difference: it
decides whether a real workbook computes or shows `#NUM!`.

## Relationships

- **[FREQUENCY](FUNC.FREQUENCY.md)** is `PROB`'s empirical twin. `FREQUENCY` counts observations
  falling into bins; `PROB` sums a *given* probability vector over an interval. Feed
  `FREQUENCY`'s counts through a division by the sample size and you have a `prob_range` that
  `PROB` will accept.
- **[BINOM.DIST](FUNC.BINOM.DIST.md), [POISSON.DIST](FUNC.POISSON.DIST.md),
  [HYPGEOM.DIST](FUNC.HYPGEOM.DIST.md)** are the named discrete distributions. Their cumulative
  forms answer the same question `PROB` answers, but from a parameterized family rather than from
  a tabulated one. `PROB` is what you use when the distribution came from data or from a model
  Excel does not name.
- **`SUMIFS`** computes the same sum with the interval expressed as criteria:
  `SUMIFS(prob, x, ">="&a, x, "<="&b)`. What `SUMIFS` will not do is check that `prob` is a
  probability vector — which is `PROB`'s entire added value and the source of both of its
  `#NUM!` conditions.
- **[MODE.MULT](FUNC.MODE.MULT.md) and [LOOKUP](FUNC.LOOKUP.md)** share `PROB`'s implementing
  module in the reference engine. That is a packaging fact about parallel-vector handling, not a
  semantic relationship.
- Readers confuse `PROB` with `PERCENTILE`/`QUARTILE`, which invert an *empirical* distribution
  built from raw observations rather than reading a supplied probability vector.

## Numerical notes

`PROB` looks arithmetically trivial and is not, for one reason: it contains an equality test
against `1` in disguise.

The accumulation itself is a plain left-to-right sum of at most `n` doubles, each in `[0, 1]`.
There is no cancellation — every term is non-negative — so the relative error of the returned
mass is bounded by roughly `n` units in the last place of the running total, which is the best
behaviour a naive sum ever has. Compensated summation (Kahan, or the doubly compensated variants
analysed in Higham, *Accuracy and Stability of Numerical Algorithms*, chapter 4) would tighten
that to a constant, and would be worth having only for very long support vectors.

The validation sum is the delicate part. `sum(p_i)` is computed the same naive way and then
compared against `1`. Three separate things now depend on floating-point detail:

1. **Whether the input is representable at all.** Probability vectors that are exact in decimal
   are usually not exact in binary64. Ten copies of `0.1` do not sum to `1`; neither do three
   copies of `1/3`. A function that tested equality literally would reject most hand-written
   distributions, which is why a tolerance exists.
2. **The summation order.** The naive sum's result depends on the order of the vector. A
   distribution can pass the test in one column ordering and fail in another, with no change to
   any input value. Sorting ascending before summing, or compensating, removes most of that
   sensitivity; the reference engine does neither.
3. **The tolerance width itself.** It is a free parameter with observable consequences at the
   boundary. Widening it admits vectors that are not distributions; narrowing it rejects vectors
   that are. There is no principled value, only a chosen one, and Microsoft's documentation
   supplies none.

The accumulated interval mass and the validation sum are also computed independently, so `PROB`
over the whole support is not guaranteed to return exactly `1` even when the validation passed.
An implementation that wanted that identity would have to return the validation sum itself when
the interval covers every atom, which no source suggests Excel does.

## What has not been checked

No Handbook vector suite exists for `PROB`, and no Handbook evidence record names `PROB` as a
subject. Nobody has checked this function against Excel within the Handbook's record. The
reference engine's implementing module carries a behavioural-finding execution record upstream,
but that record is not a per-surface Excel comparison for `PROB` and no figure attaches here.

Everything above marked as documented comes from Microsoft's `PROB` page. **Retrieval of that
page was blocked by the upstream host on this pass**, so the documented statements are recorded
as documented behaviour, with the source named, and should be re-read against the live page.

Inputs worth probing first, in the order that would settle the most:

1. **A `prob_range` containing an exact `0`**, with the rest summing to `1`. This is the single
   probe that resolves the documented `<= 0` versus implemented `< 0` divergence, and it is the
   cheapest one on the list.
2. **The sum-to-one tolerance boundary.** Take a two-atom vector `{p, 1-p}` and perturb one entry
   by successively larger multiples of an ulp until `#NUM!` appears. That maps Excel's acceptance
   window directly, and it is the experiment that turns an undocumented threshold into a
   characterized one.
3. **Ten copies of `0.1`.** The canonical vector whose exact binary sum is not `1`. If Excel
   accepts it, Excel has a tolerance; if it refuses, Excel is stricter than any real workbook can
   survive.
4. **Inverted limits**: `PROB(x, p, 5, 1)`, against the two readings `#NUM!` and `0`.
5. **Endpoint inclusion**: a distribution with an atom exactly at `lower_limit` and another
   exactly at `upper_limit`, confirming both ends are closed.
6. **Text, a logical, and a blank cell in `x_range`, and separately in `prob_range`** — four
   probes that decide whether `PROB` uses a strict scan or the ignore-text scan the rest of the
   statistical category uses.
7. **A rectangular `x_range`**, against the reference engine's `#REF!`.
8. **Duplicate support points** with distinct probabilities, confirming that `PROB` sums over
   positions rather than over distinct values.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| atom | One support point `x_i` together with its probability mass `p_i` |
| point mass | `P(X = a)`; what `PROB` returns when `upper_limit` is omitted |
| closed interval | Both endpoints are admitted: `a <= x_i <= b` |
| probability vector | A `prob_range` whose entries are admissible probabilities summing to 1 |
| sum-to-one test | The reference engine's tolerance comparison of `sum(p_i)` against `1` |
| positional pairing | The element-by-element correspondence between `x_range` and `prob_range` |

## Sources

- Microsoft, *PROB function* —
  <https://support.microsoft.com/en-us/office/prob-function-9ac30561-c81c-4259-8253-34f0a238fc49>
  (signature, the omitted-`upper_limit` point-probability rule, the `#N/A` unequal-count
  condition, and the two `#NUM!` conditions on `prob_range`). Retrieval was blocked by the
  upstream host for this page; the documented behaviour above is stated as documented behaviour
  and should be re-checked against the page.
- OxFunc `crates/oxfunc_core/src/functions/lookup_prob_frequency_family.rs` at commit `473efa3`
  — the reference-engine kernel: the unequal-length `#N/A`, the `p < 0 || p > 1` guard, the
  tolerance-based sum-to-one test, the inverted-interval `#NUM!`, the strict non-numeric
  rejection, the rectangular-grid `#REF!`, and the closed-interval accumulation.
- Handbook `data/functions/FUNC.PROB.json`, `data/presence/FUNC.PROB.json` (arity, classification
  axes, implementing module and its sibling surfaces).
- Handbook [coercion and lifting](../model/02-coercion-and-lifting.md) — the direct-argument
  versus range-scan split that `PROB`'s strict collection sits outside of.
- N. J. Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd ed., chapter 4 (summation
  error bounds and compensated summation) — the analysis behind the summation-order remark.
