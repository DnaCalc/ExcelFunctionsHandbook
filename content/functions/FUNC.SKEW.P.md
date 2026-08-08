---
schema: efh.function-page/v1
function_id: FUNC.SKEW.P
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — SKEW.P function"
    locator: "https://support.microsoft.com/en-us/office/skew-p-function-76530a5c-99b9-48a1-8392-26632d542fcb"
    role: "documented signature, the stated equation, and the documented error conditions"
  - work: "Abramowitz, M. and Stegun, I. A., Handbook of Mathematical Functions"
    locator: "chapter 26, section 26.1 (moments, standardised moments and the shape coefficients)"
    role: "the standard-moment vocabulary the skewness coefficients are defined in"
  - work: "Joanes, D. N. and Gill, C. A., \"Comparing measures of sample skewness and kurtosis\""
    locator: "Journal of the Royal Statistical Society Series D 47(1), 1998, pp. 183-189"
    role: "the three competing sample skewness estimators and the exact algebraic relations between them"
  - work: "OxFunc — moment_stats_family.rs"
    locator: "crates/oxfunc_core/src/functions/moment_stats_family.rs"
    role: "reference-engine kernel: the population divisor and the admissibility guard"
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
family: moment_stats_family
role_in_family: >-
  The uncorrected third standardised moment: the family's asymmetry measure under the population
  convention, with no bias adjustment and no (n-2) factor.
---

# SKEW.P

## What it computes

`SKEW.P(number1, [number2], …)` returns the **population skewness** of the values it is given:
the third standardised moment computed with the population standard deviation and no bias
correction.

For values `x_1, …, x_n` with mean `xbar`, define the central moments

    m_k  =  (1/n) * sum_{i=1..n} (x_i - xbar)^k

and the population standard deviation `sigma = sqrt(m_2)`. Then

    SKEW.P  =  g1  =  m_3 / m_2^(3/2)  =  (1/n) * sum ( (x_i - xbar) / sigma )^3

This is the plain third standardised moment about the sample mean — the estimator textbooks
usually write as `b1` or `g1`. It treats the supplied values as the *whole population* rather
than as a sample drawn from a larger one, which is what the `.P` names throughout Excel's
statistical category mean.

The interpretation is the classical one from the moment vocabulary of Abramowitz & Stegun chapter
26: zero for a symmetric distribution, positive when the right tail is longer, negative when the
left one is. It is invariant under any positive affine change of units, and it is dimensionless.

**The relationship to [SKEW](FUNC.SKEW.md) is an exact algebraic identity**, not an
approximation:

    SKEW  =  SKEW.P  *  sqrt( n (n-1) ) / (n - 2)

for `n >= 3`. `SKEW` applies the bias correction that makes the estimator unbiased for a normal
population; `SKEW.P` does not. The correction factor exceeds `1` always and decays toward `1` as
`n` grows, so the two agree asymptotically and can differ noticeably on small samples. Joanes and
Gill (1998) catalogue both, together with a third variant, and give the relations among them.
Neither convention is wrong; a reader comparing Excel against another package must first
establish which convention that package uses.

`g1` is **bounded**, and by a tighter bound than [SKEW](FUNC.SKEW.md)'s:

    |SKEW.P|  <=  (n - 2) / sqrt(n - 1)

with equality when one observation stands apart from `n-1` identical ones. A population of five
cannot report a skewness beyond `1.5`, however lopsided it looks. Small-sample skewness is
structurally limited, not merely noisy.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `number1` | The first value, range or array. | yes |
| `number2`, … | Further values, ranges or arrays. | optional, up to the declared maximum |

The arguments form a flat bag: everything is flattened and concatenated, and the argument
boundaries have no effect on the answer.

`SKEW.P` is an **aggregate**, not a lift kernel, and it carries the dual coercion policy of
[coercion and lifting](../model/02-coercion-and-lifting.md): values reached by scanning a range
are used only if they are numbers, with text, logicals and blank cells skipped rather than
converted, while values passed directly as scalar arguments go through ordinary to-number
coercion. `SKEW.P(1,2,"3")` therefore admits `"3"` where the same text in a cell would be ignored.

## Result and edge cases

Returns `Number`, dimensionless.

- **All values equal.** `m_2 = 0`, the standardisation is `0/0`, and the result is `#DIV/0!`.
- **A symmetric sample.** Zero, and exactly zero when the positive and negative cubes cancel bit
  for bit — which for a symmetric set in binary64 they usually do, each `+d^3` having an exact
  `-d^3` partner. The exactness follows from the data and the summation order, not from a
  guarantee.
- **Exactly two distinct values.** The two deviations are `+d/2` and `-d/2`, their cubes cancel
  exactly, and the reference engine returns `0`. **This is the page's principal open question:**
  the reference engine's guard admits any non-empty data set with nonzero spread, so `n = 2` is
  computed rather than refused. Microsoft's documented error condition for `SKEW.P` should be
  read carefully here — if the documentation states a minimum of three data points, as the
  [SKEW](FUNC.SKEW.md) page does, then the documented behaviour and the reference engine's
  behaviour diverge at `n = 2`, and the divergence is between an error value and the number
  zero. The Handbook has not been able to retrieve the page on this pass and has not observed
  Excel; the probe that settles it is named below.
- **A single value.** `m_2 = 0` for one point, so the zero-variance guard fires and the answer is
  `#DIV/0!` regardless of the minimum-count question.
- **Non-numeric cells in a scanned range** are skipped and do not enlarge `n`.
- **Errors anywhere** propagate as themselves.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#DIV/0!` | the standard deviation is zero (including the single-value case) | documented and implemented |
| `#DIV/0!` | fewer than three data points | **possibly documented** — see the open question above; the reference engine does not enforce this and returns `0` at `n = 2` |
| `#VALUE!` | direct text that does not parse as a number | shared coercion rule |
| propagated | An error value in any argument surfaces as that error | shared coercion rule |

The second row is recorded as a candidate documentation-versus-reference-engine divergence rather
than as a settled one, because the Handbook could not retrieve the documentation on this pass.
The engineering point stands either way: `SKEW.P` has no structural reason to require `n >= 3`.
The `(n-2)` denominator that forces the restriction on [SKEW](FUNC.SKEW.md) does not appear in
`g1` at all, so a minimum of three, if Excel enforces one, is inherited convention rather than
mathematics.

## Relationships

- **[SKEW](FUNC.SKEW.md)** is the bias-corrected sibling, related by the exact identity above.
  They are two estimators of the same population quantity, not a legacy/modern pair; neither
  supersedes the other. Note the asymmetry of the naming: `SKEW` is the *older* function and
  `SKEW.P` the newer one, so the usual "prefer the dotted name" migration advice does not apply
  to this pair.
- **[KURT](FUNC.KURT.md)** is the fourth-moment companion. Excel publishes only the
  bias-corrected kurtosis; there is no `KURT.P`, which makes the `SKEW`/`SKEW.P` pair the only
  place in the category where both moment conventions are exposed.
- **[STDEV.P](FUNC.STDEV.P.md)** supplies the `sigma` in the denominator; `SKEW.P` is undefined
  exactly where `STDEV.P` is zero. **[AVERAGE](FUNC.AVERAGE.md)** supplies `xbar`.
- **[STANDARDIZE](FUNC.STANDARDIZE.md)** performs the same `(x - mu)/sigma` map one value at a
  time.
- **`SKEW.P` has no Compatibility-category predecessor.** It was added with the dotted-name
  reorganisation as a genuinely new function rather than as a rename.
- Readers confuse `SKEW.P` with a normality test. It is not one, and the sampling variability of
  `g1` is large at the sample sizes spreadsheets usually hold.

## Numerical notes

The kernel is the same two-pass centred construction as [SKEW](FUNC.SKEW.md)'s, with `n` in place
of `n-1` in the standard deviation and `1/n` in place of `n/((n-1)(n-2))` in the scaling.
Everything said there about algorithm choice applies here; the points specific to `SKEW.P` are
these.

**Two-pass centring, not raw power sums.** The kernel computes `xbar`, then accumulates the
centred sum of squares and the standardised cubes from stored deviations. The alternative — build
`m_3` from `sum x`, `sum x^2`, `sum x^3` algebraically — cancels catastrophically when the mean
is large relative to the spread, and loses digits in proportion to the *cube* of that ratio. A
third moment computed that way on data with a large offset can come out with the wrong sign.
Chan, Golub and LeVeque (1983) analyse the variance case; the third moment is strictly worse.

**Per-element standardisation.** Each deviation is divided by `sigma` before cubing rather than
cubing first and scaling the total by `sigma^3`. This keeps the cubed quantities near unit scale
and avoids the overflow that `(x - xbar)^3` reaches at deviations beyond roughly `10^102`, at the
cost of `n` divisions.

**The population divisor is the numerically friendlier one.** `sigma^2 = sumsq/n` needs no
`n-1` and no guard against `n = 1` beyond the zero-variance test, and `g1 = standardised_sum/n`
is one division by an exact integer. There is no `(n-1)(n-2)` product to overflow or to lose
precision in, and no compound rational factor. `SKEW.P` is, arithmetic for arithmetic, the
cleaner of the two skewness functions — which is worth noting because the bias correction that
makes [SKEW](FUNC.SKEW.md) statistically preferable also makes it numerically busier.

**What remains.** The mean and both accumulations are naive left-to-right sums, so error grows
with `n` rather than being bounded; compensated summation is not used. The zero-variance guard is
an exact comparison, not a tolerance, so data with a real but vanishing spread divides by a
denormal and produces an enormous result rather than a diagnostic. Summation order is part of the
answer, so two correct implementations traversing differently will disagree in the last bits.

**The identity as a cross-check.** `SKEW = SKEW.P * sqrt(n(n-1))/(n-2)` is exact in real
arithmetic. Computing both and comparing against the closed form exercises the two kernels
against each other without an external oracle, and any disagreement localises immediately to one
of the two scaling paths — which is exactly the kind of self-consistency check a vector suite for
this family should carry.

## What has not been checked

No Handbook vector suite exists for `SKEW.P`, and no Handbook evidence record names `SKEW.P` as a
subject. Nobody has checked this function against Excel within the Handbook's record.

`SKEW.P` is named in the reference engine's upstream known-exactness-deviations register and in a
defect stream on statistical numeric exactness drift, and it shares its implementing module with
[KURT](FUNC.KURT.md), [SKEW](FUNC.SKEW.md), [STEYX](FUNC.STEYX.md) and
[TRIMMEAN](FUNC.TRIMMEAN.md). Those are upstream registers of open work, not Handbook
measurements of this surface, and no figure from them attaches to this page. What they establish
is that the module's numeric agreement with Excel is a live open question upstream — which is a
reason to treat `SKEW.P`'s last bits as unsettled rather than assumed.

Everything above marked as documented comes from Microsoft's `SKEW.P` page. **Retrieval of that
page was blocked by the upstream host on this pass**, so those statements are recorded as
documented behaviour with the source named and must be re-read against the live page. The
minimum-data-point condition in particular is stated on this page as an open question precisely
because it could not be read.

Inputs worth probing first:

1. **`SKEW.P` on exactly two distinct numbers**, for example `{1, 3}`. This is the single probe
   that resolves the page's main open question: the reference engine returns `0`, and a
   documented three-point minimum would make Excel return `#DIV/0!`. Two values in two cells
   settle it, and no other probe on this list is worth as much.
2. **`SKEW.P` against `SKEW / (SQRT(n*(n-1))/(n-2))`** on the same data, bitwise. The identity is
   exact; the gap is pure implementation residue.
3. **The shift-invariance test.** A fixed data set and the same set offset by a large constant.
   The true skewness is unchanged; any difference in the returned bits measures the centring
   cancellation and fingerprints the algorithm. The raw-power-sum form fails this badly and the
   centred form fails it gently.
4. **A constant sample and a single value**, confirming the zero-variance `#DIV/0!`.
5. **A symmetric sample** such as `{-2,-1,0,1,2}`, checking whether the answer is exactly `0`.
6. **The bound**: a maximally skewed five-point population, checked against `3/2`.
7. **The direct-versus-scan split**: `SKEW.P(1,2,"3",4)` against the same four values in cells.
8. **Data with a vanishing spread**, where the exact zero-variance test does not fire.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| `g1` | The uncorrected third standardised moment; what `SKEW.P` returns |
| `G1` | The adjusted Fisher-Pearson coefficient; what [SKEW](FUNC.SKEW.md) returns |
| central moment `m_k` | `(1/n) sum (x - xbar)^k` |
| population convention | Divide by `n`; treat the supplied values as the whole population |
| two-pass centring | Compute the mean first, then accumulate deviations; the stable route |
| shift invariance | The true skewness is unchanged by adding a constant; a test of the algorithm |

## Sources

- Microsoft, *SKEW.P function* —
  <https://support.microsoft.com/en-us/office/skew-p-function-76530a5c-99b9-48a1-8392-26632d542fcb>
  (signature, the stated equation, and the documented error conditions). Retrieval was blocked by
  the upstream host for this page; the documented behaviour above is stated as documented
  behaviour and must be re-checked against the page. The minimum-data-point condition is left
  open on this page for that reason.
- M. Abramowitz and I. A. Stegun, *Handbook of Mathematical Functions*, chapter 26 section 26.1 —
  the moment and standardised-moment vocabulary.
- D. N. Joanes and C. A. Gill, "Comparing measures of sample skewness and kurtosis", *Journal of
  the Royal Statistical Society, Series D (The Statistician)* 47(1), 1998, pp. 183–189 — the
  competing estimators and the exact relations between them.
- T. F. Chan, G. H. Golub and R. J. LeVeque, "Algorithms for Computing the Sample Variance:
  Analysis and Recommendations", *The American Statistician* 37(3), 1983, pp. 242–247.
- OxFunc `crates/oxfunc_core/src/functions/moment_stats_family.rs` at commit `473efa3` — the
  reference-engine kernel: the non-empty and zero-variance guards (and the absence of any
  three-point minimum), the population divisor, and the per-element standardisation.
- OxFunc `docs/KNOWN_EXACTNESS_DEVIATIONS.md` and defect stream
  `docs/bugs/streams/BUG-FUNC-021_w090_statistical_numeric_exactness_drift.md` — the upstream
  registers naming this surface and its module.
- Handbook [coercion and lifting](../model/02-coercion-and-lifting.md) — the direct-argument
  versus range-scan dual policy.
- Handbook `data/functions/FUNC.SKEW.P.json`, `data/presence/FUNC.SKEW.P.json`.
