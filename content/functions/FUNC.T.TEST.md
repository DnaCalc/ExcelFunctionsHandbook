---
schema: efh.function-page/v1
function_id: FUNC.T.TEST
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
family: statistical_tests_family
role_in_family: >-
  The t-test member of the statistical-tests module: one surface hiding three different tests
  behind an integer switch, each with its own statistic, its own degrees of freedom, and its own
  admissibility rules.
---

# T.TEST

## What it computes

`T.TEST(array1, array2, tails, type)` returns the **probability associated with a Student's
t-test** — the `p`-value for the null hypothesis that the two samples come from populations with
equal means.

It is not one function. The `type` argument selects among three genuinely different tests, which
differ in their statistic, in their degrees of freedom, and in what they assume:

**Type 1 — paired.** The samples are matched observations; the test is a one-sample t-test on the
differences `dᵢ = x₁ᵢ − x₂ᵢ`:

    t = d̄ / (s_d / sqrt(n)),        ν = n − 1

where `s_d` is the sample standard deviation of the differences. This requires equal lengths, and
Microsoft documents `#N/A` when they differ.

**Type 2 — two-sample, equal variance (homoscedastic).** The two samples are independent and
assumed to share a variance, which is estimated by pooling:

    s_p² = ( (n₁−1)s₁² + (n₂−1)s₂² ) / (n₁ + n₂ − 2)

    t = (x̄₁ − x̄₂) / ( s_p · sqrt(1/n₁ + 1/n₂) ),     ν = n₁ + n₂ − 2

**Type 3 — two-sample, unequal variance (heteroscedastic).** Welch's test. The variances are not
pooled, and the degrees of freedom are approximated by the Welch–Satterthwaite formula:

    t = (x̄₁ − x̄₂) / sqrt( s₁²/n₁ + s₂²/n₂ )

    ν = ( s₁²/n₁ + s₂²/n₂ )² / ( (s₁²/n₁)²/(n₁−1) + (s₂²/n₂)²/(n₂−1) )

That ν is **not an integer**, and it is not intended to be one. It is a moment-matching
approximation: the distribution of the Welch statistic is not exactly a t-distribution for any ν,
and the formula chooses the ν whose t-distribution matches its first two moments. `ν` lies
between `min(n₁, n₂) − 1` and `n₁ + n₂ − 2`.

The `p`-value is then a tail area of the t-distribution with those degrees of freedom:

    tails = 1  →  p = P(T_ν > |t|)          =  T.DIST.RT(|t|, ν)
    tails = 2  →  p = P(|T_ν| > |t|)        =  T.DIST.2T(|t|, ν)  =  2 · T.DIST.RT(|t|, ν)

Range: `p ∈ (0, 1)`, approaching 1 as the sample means coincide and 0 as they separate.

The three types do not nest and do not agree. On the same data, type 2 and type 3 give different
`p`-values whenever the sample variances differ, and type 1 gives a different answer again — often
a much smaller one, because pairing removes between-subject variation. Choosing the wrong `type`
is not a rounding-level mistake; it is a different experiment.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `array1` | "The first data set." | Yes |
| `array2` | "The second data set." | Yes |
| `tails` | 1 for the one-tailed distribution, 2 for the two-tailed. | Yes |
| `type` | 1 = paired, 2 = two-sample equal variance, 3 = two-sample unequal variance. | Yes |

All four are required; the projection records an arity of exactly four. There is no default
`type` and no default `tails` — a deliberate design choice, and the right one, since neither has
a defensible default.

Microsoft documents that **`tails` and `type` are truncated to integers**. So `type = 2.9` is a
type-2 test, and `tails = 1.9` is one-tailed. That is worth knowing, because a `type` computed
from a cell can silently land on a different test than intended.

This surface's argument-preparation profile in the projection is `RefsVisibleInAdapter` — the
kernel sees live references rather than pre-resolved values, which is what a paired test needs in
order to know the shapes it was given.

## Result and edge cases

Returns `Number`, a probability in `(0, 1)`.

- **The one-tail / two-tail relationship.** `tails = 2` is exactly twice `tails = 1`, since the
  statistic's absolute value is used. Multiplication by two is exact in binary floating point, so
  the two settings must agree exactly — a free self-consistency probe.
- **A one-tailed `p`-value from `|t|` is not a directional test.** Because the statistic is taken
  in absolute value, `T.TEST(a, b, 1, type)` and `T.TEST(b, a, 1, type)` return the same number.
  A reader wanting a genuinely one-sided test with a direction must check the sign of
  `AVERAGE(array1) − AVERAGE(array2)` themselves. The documentation does not warn about this, and
  it is the most consequential silent trap in the function.
- **`n = 1` in either group** leaves a variance undefined. For type 2 and 3 the statistic is not
  computable; for type 1 with a single pair the difference has no sample standard deviation.
  Undocumented.
- **Zero variance in both groups** makes the denominator zero. If the means also coincide the
  statistic is `0/0`. Undocumented.
- **Unequal lengths** are documented as `#N/A` only for `type = 1`. For types 2 and 3 unequal
  lengths are the normal case.
- **Non-integer Welch degrees of freedom.** Type 3 produces a real ν, and the tail area must then
  be evaluated at a non-integer ν. Whether Excel evaluates the t-distribution at the real ν, or
  truncates it first, changes the answer — and truncating is the conservative-looking choice that
  is actually wrong, because it discards the approximation's entire point. **Microsoft's page does
  not say which happens.** Note the awkward fit with the neighbouring surfaces: `T.DIST.RT` and
  `T.DIST.2T` document `deg_freedom` as an integer, and `T.INV`/`T.INV.2T` document truncation of
  a non-integer — so if `T.TEST` type 3 calls those surfaces as documented, it cannot pass a real
  ν through them. Either it does not use them, or the documentation is incomplete somewhere. This
  is a documentation-versus-mathematics tension the Handbook records rather than resolves.
- **Text, logicals and empty cells** in the data ranges follow the family's coercion policy. The
  reference-engine battery beside this page rejects logical arguments in the data positions;
  that is the engine's own answer, not Excel's.
- **Arrays.** The lift axis is projected as `surface_native` with `default-unexamined`
  provenance.

## Errors

As documented on Microsoft's `T.TEST` page:

| Error | Documented condition |
|---|---|
| `#N/A` | `array1` and `array2` have different numbers of data points and `type = 1`. |
| `#VALUE!` | `tails` or `type` is nonnumeric. |
| `#NUM!` | `tails` is any value other than 1 or 2. |

Note what is missing. **The page states no condition for an out-of-range `type`.** `tails` gets an
explicit `#NUM!` rule; `type` gets none, even though the argument's documented value set is
exactly `{1, 2, 3}`. Whether `type = 0` or `type = 4` produces `#NUM!`, some default test, or
something else is undocumented, and this is a real gap rather than an oversight in this page's
reading: the two arguments are described in the same table and treated asymmetrically in the
remarks. Also undocumented: single-element groups, zero variance, and the treatment of the Welch
degrees of freedom.

Errors in any argument propagate under the universal coercion rule. The Handbook has not verified
any of this against Excel.

## Relationships

- **`TTEST`** is the legacy compatibility spelling, documented to compute the same thing. The
  Handbook's alias-pairing record for the legacy/modern statistical collapse does not list either
  spelling among its subjects, so **nothing in the Handbook's record bears on whether `TTEST` and
  `T.TEST` publish the same bits.** They are documented equivalents; that is a weaker statement
  than it sounds.
- **[T.DIST.RT](FUNC.T.DIST.RT.md)** and **[T.DIST.2T](FUNC.T.DIST.2T.md)** are the tail areas
  this function's answer is, given the statistic and ν. A reader can reconstruct `T.TEST` from
  them plus `AVERAGE`, `VAR.S` and `COUNT` — and doing so is the best available check on it.
- **[T.INV.2T](FUNC.T.INV.2T.md)** is the inverse direction: `T.TEST` gives a `p`-value from
  data, `T.INV.2T` gives a critical value from a `p`. Comparing a `T.TEST` result against a
  chosen α is the same decision as comparing the statistic against `T.INV.2T(α, ν)`.
- **`Z.TEST`** is the large-sample or known-σ counterpart, and it differs in shape as well as in
  distribution: `Z.TEST` takes one sample and a hypothesized mean, and returns a **directional**
  one-tailed probability, not a `|statistic|` tail area. See [Z.TEST](FUNC.Z.TEST.md) — the two
  functions are not parallel and cannot be swapped by changing the distribution.
- **`F.TEST`** tests the equality of variances, which is the assumption `type = 2` makes. Using
  `F.TEST` to choose between `type = 2` and `type = 3` is a well-known statistical mistake (the
  two-stage procedure has the wrong size); the modern advice is to use Welch unconditionally.
- **`CHISQ.TEST`** is the third member of the same reference-engine module.
- **Confused with**: the Data Analysis ToolPak's t-Test dialogs, which report the statistic, both
  critical values and both `p`-values, and which are a different code path from this worksheet
  function.

## Numerical notes

Three separable difficulties, and the third is the one that decides the answer.

**The variances.** Every branch needs sample variances, and the classic hazard applies: computing
`Σx² − (Σx)²/n` loses all significance when the mean is large relative to the spread. Two-pass or
Welford accumulation is the fix, and the argument is Higham's §1.9 and Chan–Golub–LeVeque. This
matters more here than in `VAR.S` itself, because the variance feeds a ratio whose numerator is
*also* a difference of nearly equal means — so the cancellation compounds. See
[VAR.S](FUNC.VAR.S.md) for the one-variable treatment.

**The mean difference.** `x̄₁ − x̄₂` is a subtraction of nearly equal quantities under the null
hypothesis — which is precisely the regime the test is designed for. When the two means agree to
most of their digits, the numerator of `t` is almost entirely rounding error. For a paired test
this is avoidable and important: differencing **first** (`dᵢ = x₁ᵢ − x₂ᵢ`, then average) is far
more accurate than averaging first and differencing after, because each `dᵢ` is a subtraction of
two quantities that are genuinely close, while `x̄₁ − x̄₂` subtracts two quantities that have each
already accumulated rounding from `n` additions. Mathematically identical; numerically not.

**The Welch degrees of freedom.** The Welch–Satterthwaite formula is a ratio of fourth powers of
variances, and it is delicate at the extremes: as one `sᵢ² → 0` the expression tends to
`nᵢ − 1` in a way that is numerically awkward, and when both variances and sample sizes are equal
it reduces exactly to `n₁ + n₂ − 2`. Implementations should form `u = s₁²/n₁` and `v = s₂²/n₂`
once and compute `(u+v)²/(u²/(n₁−1) + v²/(n₂−1))` from them, rather than expanding — the expanded
form squares and cancels needlessly.

**And then the tail.** Everything above feeds a t-distribution tail area, so the accuracy
discussion of [T.DIST.RT](FUNC.T.DIST.RT.md#numerical-notes) applies unchanged: the tail must be
computed as itself, not as one minus the CDF, or small `p`-values — the interesting ones — will be
destroyed.

The accuracy of Excel's hypothesis-test functions is part of the published literature on Excel's
statistical procedures; Morten Welinder's Gnumeric work and the assessments by Knüsel and by
McCullough & Wilson are the standing references. The Handbook names them and does not assert from
them what any current build does.

## What has not been checked

No Handbook vector suite exists for `T.TEST`, and no Handbook evidence record lists this surface
among its subjects. **Nobody has checked `T.TEST` against Excel within the Handbook's record.**
The reference-engine battery rendered beside this page is the engine answering its own questions;
no Excel was involved in it.

Documentation gaps this page could not close: the behaviour of an out-of-range `type`; whether the
Welch degrees of freedom are used as a real number or truncated; single-element groups; zero
variance; and the treatment of text, logicals and blanks in the data ranges.

Inputs worth probing first:

1. **`type = 3` against a hand-built reconstruction.** Compute `t` and the Welch–Satterthwaite ν
   in cells, then compare `T.TEST(a, b, 2, 3)` against `T.DIST.2T(ABS(t), nu)` and against
   `T.DIST.2T(ABS(t), TRUNC(nu))`. Whichever it matches settles the truncation question in one
   experiment, and it is the single highest-information probe on this page.
2. **`type = 0`, `type = 4`, `type = 2.9`.** The undocumented corner and the documented
   truncation, in three calls.
3. **`T.TEST(a, b, 2, k)` against `2 * T.TEST(a, b, 1, k)`** for each `k`. Exact in binary
   floating point; needs no oracle.
4. **`T.TEST(a, b, 1, k)` against `T.TEST(b, a, 1, k)`** — confirms (or refutes) that the
   one-tailed value is direction-blind, which is the trap most likely to be producing wrong
   conclusions in real workbooks.
5. **Type 2 against type 3 on data with equal sample variances and equal sizes**, where the two
   are mathematically identical. Any difference in published bits localizes to the degrees of
   freedom path.
6. **A large shared offset.** The same data with `1e9` added to every value in both groups: the
   test statistic is invariant, so the `p`-value must not move. If it moves, the variance
   computation is uncentred, and the size of the move measures it. This is the cheapest possible
   probe of the accumulation arrangement and it needs no reference implementation at all.
7. **Paired test with near-equal pairs**: `x₂ᵢ = x₁ᵢ + δ` with δ shrinking, comparing against
   the difference-first computation. Traces the ordering hazard directly.
8. **Boundaries**: unequal lengths under each `type`, a single-element group, and identical
   constant groups (`0/0`).

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| paired test | `type = 1`: a one-sample test on the differences of matched observations |
| homoscedastic | `type = 2`: two-sample test with a pooled variance estimate |
| Welch's test | `type = 3`: two-sample test with unpooled variances and approximate ν |
| Welch–Satterthwaite | The moment-matching formula giving type 3's non-integer degrees of freedom |
| direction-blind | The property that a one-tailed result here does not depend on which mean is larger |
| `RefsVisibleInAdapter` | Argument-preparation profile: the kernel sees live references |

## Sources

- Microsoft, "T.TEST function" —
  <https://support.microsoft.com/en-us/office/t-test-function-d4e08ec3-c545-485f-962e-276f7cbed055>
  (syntax; the `tails` meaning; the `type` table 1 = paired, 2 = two-sample equal variance
  (homoscedastic), 3 = two-sample unequal variance (heteroscedastic); `#N/A` for different
  numbers of data points with `type = 1`; truncation of `tails` and `type` to integers; `#VALUE!`
  for nonnumeric `tails` or `type`; `#NUM!` for `tails` other than 1 or 2; and the description of
  the returned probability with the two-tailed value as double the one-tailed). Retrieved for
  this page. The page states no condition for an out-of-range `type`.
- B. L. Welch, "The generalization of Student's problem when several different population
  variances are involved" (1947), and F. E. Satterthwaite (1946) — the origin of the type-3
  degrees-of-freedom approximation.
- M. Abramowitz and I. A. Stegun, *Handbook of Mathematical Functions*, section 26.7 (the
  t-distribution whose tail this function returns).
- N. J. Higham, *Accuracy and Stability of Numerical Algorithms*, §1.9, and T. F. Chan,
  G. H. Golub and R. J. LeVeque on computing sample variances — the accumulation argument behind
  the offset-invariance probe above.
- M. Welinder's documentation of Gnumeric's statistical functions, and the Excel accuracy
  assessments of R. Knüsel and of B. D. McCullough and B. Wilson — named as the standing
  literature, not as evidence about any current build.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.T.TEST.json` (arity, `RefsVisibleInAdapter`),
  `data/presence/FUNC.T.TEST.json` (the `statistical_tests_family` and `test_alias_family`
  modules) and `data/battery/FUNC.T.TEST.json`.
