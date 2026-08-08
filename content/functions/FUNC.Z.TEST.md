---
schema: efh.function-page/v1
function_id: FUNC.Z.TEST
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0012
  - EV-DIST-0017
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
family: confidence_test_family
role_in_family: >-
  The one-sample normal-tail test of the confidence module: the only hypothesis-test surface in
  Excel whose defining formula Microsoft publishes as a composition of other worksheet functions —
  and therefore the only one whose accuracy ceiling is documented in plain sight.
---

# Z.TEST

## What it computes

`Z.TEST(array, x, [sigma])` returns the **one-tailed probability value of a z-test**: the
probability of observing a sample mean at least as large as the one in `array`, if the underlying
population mean were `x`.

Microsoft gives the definition as an explicit composition of other worksheet functions, which is
unusual and useful:

    Z.TEST(array, x, sigma) = 1 − NORM.S.DIST( (AVERAGE(array) − x) / (sigma/sqrt(n)), TRUE )

    Z.TEST(array, x)        = 1 − NORM.S.DIST( (AVERAGE(array) − x) / (STDEV(array)/sqrt(n)), TRUE )

with `n = COUNT(array)`. Writing the standardized statistic as

    z = ( x̄ − x ) / ( σ / sqrt(n) )

the function returns `1 − Φ(z) = P(Z > z)`, the upper tail of the standard normal.

Range: `(0, 1)`, strictly. The value is exactly `½` when `x̄ = x`, decreases toward 0 as the
sample mean rises above the hypothesized mean, and rises toward 1 as it falls below. Microsoft
states the direction explicitly: "if `AVERAGE(array) < x`, `Z.TEST` will return a value greater
than 0.5."

Two structural facts distinguish this from every other test surface in Excel:

1. **It is directional.** Unlike [T.TEST](FUNC.T.TEST.md), which takes the absolute value of its
   statistic and is blind to which mean is larger, `Z.TEST` returns an upper-tail probability that
   depends on the sign. Swapping the roles of the sample and the hypothesized mean changes the
   answer to its complement, not to itself. A two-tailed value therefore has to be built by the
   caller, and Microsoft supplies the recipe:
   `=2 * MIN(Z.TEST(array, x, sigma), 1 − Z.TEST(array, x, sigma))`.
2. **The default is not the test the name implies.** When `sigma` is omitted, the documented
   formula substitutes the **sample** standard deviation while keeping the **normal** distribution
   for the tail. That combination is not a z-test and not a t-test; it is a z-test using an
   estimated σ, which is only asymptotically correct. For small `n` it understates the tail.
   Microsoft documents the substitution plainly; the statistical consequence is not commented on
   there, and a reader who wants the exact small-sample answer wants a one-sample t-test, which
   Excel does not offer directly.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `array` | "The array or range of data against which to test `x`." | Yes |
| `x` | "The value to test" — the hypothesized population mean. | Yes |
| `sigma` | "The population (known) standard deviation." | No — defaults to the sample standard deviation |

The projection records an arity of two to three.

The projection also records a **by-index scalar array lift on the second and third positions** —
`x` and `sigma` — and marks that axis `excel-claimed` rather than defaulted, which is a stronger
provenance than most axes on the statistical pages carry. In plain terms: `array` is consumed
whole, while `x` and `sigma` lift elementwise, so passing an array of hypothesized means spills a
`p`-value for each. The reference engine's battery beside this page does exactly that. The
surface's module also carries the open upstream defect stream `BUG-FUNC-018`, whose title names a
scalar-parameter array-lift gap, so the lifting behaviour is an area with known open work; the
Handbook cites the stream by name and does not restate its contents.

Note a small documentation staleness: the defining formula names `STDEV`, the legacy spelling,
inside the page for a modern dotted-name function. `STDEV` and `STDEV.S` are documented
equivalents, but the modern page's formula reaches for the compatibility name.

Note a larger one: the remark defining what the function represents introduces a symbol `μ0`
("when the underlying population mean is μ0") that appears nowhere else on the page and is never
tied to the argument `x`. Read strictly, the sentence defines the function in terms of an
undefined quantity. The intended reading is obvious and this page states it — `μ0` is `x` — but
the Handbook records the defect rather than silently repairing it.

## Result and edge cases

Returns `Number`, a probability in `(0, 1)`.

- **`x̄ = x`** gives exactly `½`. Free and exact, and the cheapest diagnostic on the surface.
- **Large positive `z`** is where the documented formula fails, and it fails hard. See Numerical
  notes: `1 − Φ(z)` computed by subtraction saturates at zero once `Φ(z)` rounds to 1, which
  happens beyond roughly eight standard errors. Every `p`-value smaller than that is published as
  zero, whatever the data says. This is a property of the documented composition, not a defect
  discovered by measurement.
- **Large negative `z`** is the well-conditioned direction: the answer approaches 1 from below and
  `Φ(z)` carries it accurately.
- **`sigma = 0`** makes the statistic infinite. Undocumented.
- **`n = 1` with `sigma` omitted** leaves the sample standard deviation undefined; the
  reference-engine battery beside this page reports a division error for single-value input. That
  is the engine's own answer, not Excel's.
- **Zero-variance `array` with `sigma` omitted** is the same situation with more data.
- **Text, logicals and empty cells** in `array` follow the family's coercion policy; the engine's
  battery rejects a logical in the `array` position. Undocumented on Microsoft's page, which says
  nothing about the contents of `array` beyond that it must not be empty.
- **`COUNT` semantics.** The documented `n` is `COUNT(array)`, which counts numbers only. So a
  range mixing numbers and text has an `n` that differs from its cell count, and the documented
  formula's `AVERAGE` and `STDEV` use the same convention. Consistent, but worth stating.

## Errors

As documented on Microsoft's `Z.TEST` page:

| Error | Documented condition |
|---|---|
| `#N/A` | `array` is empty. |

That is the entire documented error table — one row, for a function with a divisor that can be
zero, an optional argument that can be negative, and a data argument whose admissible contents
are never described. Undocumented and therefore open: `sigma ≤ 0`, `n = 1`, zero-variance data
with `sigma` omitted, and non-numeric contents of `array`.

Errors in any argument propagate under the universal coercion rule. The Handbook has not verified
any of this against Excel.

## Relationships

- **`ZTEST`** is the legacy compatibility spelling. Neither spelling appears in the Handbook's
  legacy/modern alias-pairing record, so **nothing in the Handbook's record bears on whether they
  publish the same bits.**
- **`NORM.S.DIST`** is the function the documented formula calls, and therefore the surface whose
  accuracy `Z.TEST` inherits. This surface's module carries the open defect stream
  `BUG-FUNC-013`, whose title names a normal-distribution exact-value accuracy gap, and
  `BUG-FUNC-021`, whose title names a statistical numeric exactness drift. Both are cited by name
  only.
- **`ERFC`** is the escape hatch. `1 − Φ(z) = ½·ERFC(z/sqrt 2)` exactly, and `ERFC` is a
  complementary surface computed directly rather than by subtraction — so
  `=0.5*ERFC((AVERAGE(a)-x)/(s/SQRT(n))/SQRT(2))` reproduces `Z.TEST` in the well-conditioned
  region and keeps working where `Z.TEST`'s documented composition has saturated to zero. Whether
  Excel's `ERFC` is itself accurate in the far tail is a separate question, on a separate page.
- **[T.TEST](FUNC.T.TEST.md)** is the small-sample counterpart — and note the shape mismatch: it
  takes two samples, not a sample and a hypothesized mean, and it is direction-blind. There is no
  one-sample `T.TEST`. A one-sample t-test must be assembled from `AVERAGE`, `STDEV.S`, `COUNT`
  and [T.DIST.RT](FUNC.T.DIST.RT.md).
- **`CONFIDENCE.NORM` / `CONFIDENCE.T`** are the interval-estimation counterparts; `CONFIDENCE.T`
  shares this surface's implementing module in the reference engine.
- **`STANDARDIZE`** computes the `z` this function's formula forms internally.
- **Confused with**: the Data Analysis ToolPak's "z-Test: Two Sample for Means", which is a
  different (two-sample) test on a different code path.

## Numerical notes

**The documented formula has a hard accuracy floor, and it is visible on the documentation page
itself.** `1 − Φ(z)` is a subtraction from one. Once `Φ(z)` rounds to `1.0` in double precision —
which happens beyond roughly eight standard errors — the difference is exactly zero, and every
smaller `p`-value collapses to zero with it. A z-test on a well-powered study routinely produces
statistics past that point; the whole scientific content of such a result is the *size* of the
tiny `p`-value, and this composition discards it.

The fix is standard and old: compute the upper tail directly rather than as a complement. The
identity is exact,

    1 − Φ(z) = ½ · erfc( z / sqrt 2 )

and `erfc` exists precisely so that the small quantity is computed as itself. Cody's rational
approximations for `erf`/`erfc`, the fdlibm implementations, and Boost.Math's `erfc` are the
standard sources, and the far-tail asymptotic
`erfc(t) ~ exp(−t²)/(t·sqrt π) · (1 − 1/(2t²) + …)` is what carries the value once the
approximation regions run out. A normal-tail routine that switches to the asymptotic keeps
returning meaningful values until `exp(−z²/2)` itself underflows, which is dozens of standard
errors out rather than eight.

Excel's surface set does not offer a complementary normal CDF, which is why the composition is
written the way it is. `ERFC` is the workaround, and it is the honest recommendation to a reader
who needs small `p`-values.

**Secondary hazards**, all real but smaller:

1. **The sample standard deviation.** With `sigma` omitted, the default path computes a variance,
   and the usual accumulation argument applies — never `Σx² − (Σx)²/n`; see
   [VAR.S](FUNC.VAR.S.md) and Higham's §1.9. A data set with a large mean relative to its spread
   can lose most of the digits of `s` before the statistic is formed.
2. **The mean difference.** `x̄ − x` is a subtraction of near-equal quantities exactly when the
   null hypothesis is nearly true, which is the interesting case.
3. **Composition ordering.** `(x̄ − x)/(σ/sqrt n)` and `(x̄ − x)·sqrt(n)/σ` are the same real
   number and different floating-point numbers; the second avoids one division and one rounding.
   Which one Excel uses is unknown, and the documented formula shows only the first.

The accuracy of Excel's normal-distribution and hypothesis-test surfaces is a long-standing topic
in the published literature — Morten Welinder's work on Gnumeric's statistical functions is the
most concrete account of the failure modes, and the assessments by Knüsel and by McCullough &
Wilson are the standing evaluations. The Handbook names them as the right reading and does not
assert from them what any current build does.

## What has not been checked

Two Handbook evidence records list this surface among their subjects, and they are very different
in what they support.

- `EV-DIST-0012` carries a **per-surface** count for `Z.TEST` from a sampled cell-reference
  re-sweep. The record itself states that for this surface it is the **only** per-surface count
  that has ever been published — the later campaign explicitly did not probe `Z.TEST`. It also
  carries reader warnings that must travel with it: these are sampled counts drawn from a much
  larger case population, not sweeps; several denominators in the table are very small and
  `Z.TEST`'s is among them, so a percentage on that row means very little; and the table's drifts
  column is internally inconsistent with its match column, so only match-over-total is usable.
  The figures render mechanically beside this page and are not restated here.
- `EV-DIST-0017` names this surface inside a fifteen-function residual family, but publishes only
  **run totals over the family** and no per-surface figures at all. The family was measured;
  **this surface was not measured separately in that record**, and the record's reader warning
  forbids attributing any of its totals to one member. The record also carries a stale claim about
  an unrelated repair, which it discloses itself.

So the honest summary is: `Z.TEST` has been compared against Excel once, on a small sampled row
set, in a record that warns against reading much into it — and never since. **No Handbook vector
suite exists for `Z.TEST`**, nothing here is independently re-runnable by a reader, and neither
record was re-verified by the Handbook.

Documentation gaps this page could not close: `sigma ≤ 0`, `n = 1`, zero-variance data with
`sigma` omitted, the admissible contents of `array`, and the undefined `μ0` in the defining
remark.

Inputs worth probing first:

1. **The tail floor.** `Z.TEST` on data giving `z` at 6, 8, 10, 20 standard errors, against
   `0.5*ERFC(z/SQRT(2))` computed in adjacent cells. This measures the documented composition's
   collapse directly and is the single most informative probe on this page: it needs no external
   oracle, because `ERFC` is an Excel surface.
2. **`x̄ = x`** — must be exactly `½`. One call.
3. **`Z.TEST(array, x, sigma)` against `1 − NORM.S.DIST(z, TRUE)`** written out in cells, on
   well-conditioned data. If they differ, Excel does not implement the formula it documents, and
   that is a finding in itself.
4. **The offset-invariance probe.** Add a large constant to every element of `array` and to `x`
   together: the statistic is invariant, so the result must not move. If it moves, the internal
   standard-deviation accumulation is uncentred, and the size of the move measures it. No
   reference implementation needed.
5. **`sigma` omitted versus `sigma` supplied as `STDEV.S(array)`** — must be identical if the
   documented default is what it says. A difference would show that the default path is not a
   substitution but a different computation.
6. **`sigma = 0`, `sigma < 0`, `n = 1`, constant `array`** — the four undocumented boundaries.
7. **Array-valued `x` and `sigma`**, given the `excel-claimed` lift axis and the open
   `BUG-FUNC-018` stream on this module: the shape of the spill, and whether element failures stay
   element-local.
8. **`ZTEST` against `Z.TEST`** on identical inputs — the legacy/modern pairing that no record in
   the Handbook covers.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| one-tailed probability value | `P(Z > z)` — the upper-tail area this surface returns |
| directional | The result depends on the sign of `x̄ − x`, unlike `T.TEST` |
| tail floor | The point past which `1 − Φ(z)` computed by subtraction is identically zero |
| complementary evaluation | Computing the small tail directly, e.g. via `erfc` |
| per-surface count | An evidence-layer figure measured on this surface alone |
| run total | An evidence-layer figure over a family; never attributable to one member |

## Sources

- Microsoft, "Z.TEST function" —
  <https://support.microsoft.com/en-us/office/z-test-function-d633d5a3-2031-4614-a016-92180ad82bee>
  (syntax; `array`, `x` and the optional `sigma`; `#N/A` when `array` is empty; the two defining
  equations in terms of `NORM.S.DIST`, `AVERAGE`, `STDEV` and `COUNT`; the interpretation remark
  and its statement that a sample mean below `x` gives a result above 0.5; and the two-tailed
  recipe `2*MIN(Z.TEST(...), 1−Z.TEST(...))`). Retrieved for this page.
- Handbook evidence record `EV-DIST-0012` — the sampled per-surface histogram that holds this
  surface's only per-surface count, with its own warnings about sampling, tiny denominators and
  an internally inconsistent drifts column.
- Handbook evidence record `EV-DIST-0017` — the standing residual register publishing run totals
  over a fifteen-function family, with its warning that no total may be attributed to one member.
- W. J. Cody's rational Chebyshev approximations for `erf`/`erfc`; fdlibm and Boost.Math `erfc` —
  the complementary-evaluation route that the documented composition forgoes.
- M. Abramowitz and I. A. Stegun, *Handbook of Mathematical Functions*, chapter 7 (the error
  function and its asymptotic expansion) and chapter 26 (the normal probability function).
- N. J. Higham, *Accuracy and Stability of Numerical Algorithms*, §1.9 — the sample-variance
  accumulation argument behind the offset-invariance probe.
- M. Welinder's documentation of Gnumeric's statistical functions, and the Excel accuracy
  assessments of R. Knüsel and of B. D. McCullough and B. Wilson.
- OxFunc defect streams
  `docs/bugs/streams/BUG-FUNC-013_normal_distribution_exact_value_accuracy_gap.md`,
  `docs/bugs/streams/BUG-FUNC-018_successor_scalar_parameter_array_lift_gap.md` and
  `docs/bugs/streams/BUG-FUNC-021_w090_statistical_numeric_exactness_drift.md`, named by the
  presence projection for this surface. Cited by name only.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.Z.TEST.json` (signature, arity, and the
  `excel-claimed` provenance on the lift axis), `data/presence/FUNC.Z.TEST.json` and
  `data/battery/FUNC.Z.TEST.json`.
