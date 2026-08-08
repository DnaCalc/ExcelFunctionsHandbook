---
schema: efh.function-page/v1
function_id: FUNC.GEOMEAN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.GeoMean method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.geomean"
    role: "documented description, the direct-argument versus array scan rules, the #NUM! condition for any data point ≤ 0, and the equation reference"
  - work: "Microsoft Support — GEOMEAN function"
    locator: "https://support.microsoft.com/en-us/office/geomean-function-db1ac48d-25a5-40a0-ab83-0b38980e40d5"
    role: "the worksheet-surface documentation page; not retrievable at curation time (the host refused the request)"
  - work: "Hardy, Littlewood and Pólya, Inequalities"
    locator: "chapter 2 — the means and the AM–GM inequality"
    role: "the inequality that orders this function against HARMEAN and AVERAGE"
  - work: "N. J. Higham, Accuracy and Stability of Numerical Algorithms"
    locator: "chapters on summation and on products"
    role: "the error analysis behind the log-sum versus scaled-product choice"
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
family: geomean_fn
role_in_family: >-
  Sole member of its module; the multiplicative mean, and the one of the three Pythagorean means
  whose reference implementation runs entirely in log space.
---

## What it computes

`GEOMEAN(number1, [number2], …)` returns the geometric mean of a set of positive numbers:

    GEOMEAN(x₁ … xₙ)  =  ( x₁ · x₂ · … · xₙ )^{1/n}
                      =  exp( (1/n) · Σ ln xᵢ )

- **Domain:** every admitted value must be **strictly positive**. Microsoft states the guard as
  "If any data point ≤ 0, GeoMean returns the #NUM! error value", which rules out zero as well as
  negatives. Zero is excluded because the product would be zero regardless of the other values;
  negatives because the n-th root of a negative product is not real for even n and is not the
  mean anyone wants for odd n.
- **Range:** strictly positive, and always between the smallest and largest input.
- **The defining property.** The geometric mean is the value g such that replacing every xᵢ by g
  leaves the *product* unchanged — exactly as the arithmetic mean is the value that leaves the
  *sum* unchanged. That is what makes it the correct average for anything that compounds:
  growth rates, index numbers, ratios, returns. Microsoft's own framing is "average growth rate
  given compound interest with variable rates".
- **The log identity.** ln(GEOMEAN) = AVERAGE(ln xᵢ). The geometric mean is the arithmetic mean
  on a logarithmic scale, and every property of it follows from that: it is scale-equivariant
  (GEOMEAN(c·x) = c·GEOMEAN(x)), it is the exponential of a linear functional of the logs, and
  it is invariant under taking reciprocals in the sense that GEOMEAN(1/xᵢ) = 1/GEOMEAN(xᵢ).
- **Position among the means.** For positive data,

      HARMEAN  ≤  GEOMEAN  ≤  AVERAGE

  with equality throughout **if and only if all the values are equal**. This is the classical
  AM–GM–HM chain (Hardy, Littlewood & Pólya, chapter 2). See the note under **Relationships**
  about how Microsoft's own documentation states this inequality.

**A worked distinction worth keeping.** For returns of +50% then −50%, the arithmetic mean of the
factors 1.5 and 0.5 is 1.0 — suggesting break-even — while the geometric mean is √0.75 ≈ 0.866,
correctly reporting a compounded loss. The geometric mean is the only one of the three that
answers the question "what constant rate would have produced the same final value".

## Arguments

| Argument | Meaning |
|---|---|
| `number1` | Required. The first value, array or reference. |
| `number2, …` | Optional, repeating. Further values, arrays or references. |

The reference engine declares an arity of 1 to 255, matching the modern worksheet limit;
Microsoft's VBA reference lists 30 argument slots, which is the older method-surface cap and not
the worksheet's.

**The scan policy is documented, and it is the two-regime policy the call model calls out as the
flagship asymmetry** — see [Coercion and lifting](../model/02-coercion-and-lifting.md). Microsoft
states it in three sentences:

1. "Logical values and text representations of numbers that you type directly into the list of
   arguments are counted."
2. "If an array or reference argument contains text, logical values, or empty cells, those values
   are ignored; **however, cells with the value zero are included**."
3. "Arguments that are error values or text that cannot be translated into numbers cause errors."

Sentence 2 carries a trap that is specific to this function and to `HARMEAN`: a **zero in a
scanned range is included**, and an included zero triggers the `#NUM!` guard. So a range with one
empty cell is fine and a range with one cell containing `0` is an error — and on a worksheet those
two look identical unless zero display is turned on. This is the single most likely cause of an
unexplained `#NUM!` from this function.

Sentence 1 has the mirror-image consequence: `GEOMEAN(TRUE, 4)` counts the logical as 1 and gives
2, while the same two values in a range give 4, because the logical is skipped there.

## Result and edge cases

Returns `Number`.

- **A single value** must return that value: GEOMEAN(x) = x for every positive x. This is an
  exact mathematical identity, and it is the sharpest available test of an implementation's
  arithmetic — see **Numerical notes**, where it fails.
- **All values equal** must return that common value, again exactly.
- **Any admitted value ≤ 0** is `#NUM!`, including zeros arriving from a scanned range.
- **No admitted values at all** — every scanned cell blank or textual — has no documented
  answer. The reference engine returns `#NUM!`. Note that its sibling `HARMEAN` returns `#N/A`
  in the same situation; see **Relationships**.
- **Very large or very small magnitudes**: the mathematical result is well inside the double
  range whenever the inputs are, because the geometric mean lies between the extremes. Whether an
  implementation gets there without overflowing on the way is the substance of the next section.
- **Arrays and references** are consumed by scanning, not by elementwise lifting. `GEOMEAN` never
  spills.

## Errors

As documented by Microsoft:

| Error | Condition |
|---|---|
| `#NUM!` | any data point is ≤ 0 |
| propagated | an argument is an error value |
| `#VALUE!` | a direct argument is text that cannot be translated into a number |

Undocumented, and recorded here from the reference engine only: an argument set that admits no
values at all yields `#NUM!`.

## Relationships

- **`AVERAGE`** — the arithmetic mean; the upper bound of the chain. Use it for quantities that
  add.
- **[HARMEAN](FUNC.HARMEAN.md)** — the harmonic mean; the lower bound. Use it for rates defined
  per unit of something (speeds over fixed distances, price–earnings ratios).
- **A documented statement that is false in one case.** Microsoft's `HarMean` page asserts: "The
  harmonic mean is always less than the geometric mean, which is always less than the arithmetic
  mean." The classical inequality is *non-strict*: all three coincide when every value is equal,
  and that case is not exotic — a range of identical numbers is the commonest degenerate input
  there is. Recorded as a documentation-versus-mathematics divergence. The strict form holds only
  when the data are not all equal.
- **A divergence between the siblings.** `GEOMEAN` and `HARMEAN` have identical documented scan
  rules and identical documented `#NUM!` guards, and the reference engine nevertheless answers
  the "no admitted values" case differently — `#NUM!` for this one, `#N/A` for `HARMEAN`. Neither
  answer is documented. Two functions written to the same specification disagreeing on an
  undocumented boundary is exactly the sort of thing worth publishing rather than tidying away.
- **`PRODUCT` and `POWER`** — the direct route, `PRODUCT(range)^(1/COUNT(range))`, which is what
  the definition says and what overflows first.
- **`EXP(AVERAGE(LN(range)))`** — the log-space route, computable as an array formula, and the
  one that reveals what the implementation is doing when its answers are compared against
  `GEOMEAN`'s.
- **`RRI`, `IRR`, `XIRR`** — the finance-category functions that answer the same
  "equivalent constant rate" question for cash flows rather than for a list of factors.
- **`TRIMMEAN`, `MEDIAN`** — the robustness alternatives when the reason for reaching past
  `AVERAGE` is outliers rather than compounding.

## Numerical notes

**Two implementations, and neither is obviously right.**

*The direct product.* Multiply everything, take the n-th root. This is the definition and it
overflows or underflows almost immediately: a hundred values of magnitude 10⁴ produce a product of
10⁴⁰⁰, which is not representable, even though the geometric mean is a perfectly ordinary 10⁴.
For any realistic range length the direct product is unusable, and that is why nobody writes it.

*The log sum.* Accumulate Σ ln xᵢ, divide by n, exponentiate. This never overflows and is what the
reference engine does. Its cost is the same **ln-amplification** that afflicts `GAMMA`: a relative
error ε in the accumulated mean-log becomes a relative error of about |mean-log| · ε in the
result, and the mean-log is large whenever the data are far from 1 in magnitude. Concretely, the
identity GEOMEAN(x) = x **fails** under this implementation for large x: the reference engine's
own battery shows a single very large input coming back changed in the low-order digits, because
exp(ln x) is not the identity in binary64. That is a fact about the reference engine, produced
without any Excel involvement, and it is the clearest demonstration available that the log route
is a convenience rather than an accurate method.

*The scaled product.* The technique the error-analysis literature recommends, and which neither of
the above uses: accumulate the product with the exponent held separately. Split each xᵢ into
mantissa and exponent (`frexp`), multiply the mantissas — which stay in [½, 1) and therefore never
overflow — and add the exponents as integers, renormalising when the mantissa product drifts.
At the end take the n-th root of the mantissa product and divide the exponent sum by n. This
touches the logarithm only once, if at all, and it makes GEOMEAN(x) = x exact by construction.
Higham's treatment of products and of summation error is the reference for why this is the right
shape.

**Summation order matters even in log space.** Σ ln xᵢ is an ordinary floating-point sum and
inherits every ordinary floating-point summation problem: naive left-to-right accumulation has
error growing with n, pairwise summation reduces it to logarithmic growth, and Kahan or
Neumaier compensation makes it effectively exact. The reference engine accumulates naively. For
short spreadsheet ranges the difference is invisible; for long ones it is not, and it is a
per-implementation choice that changes results.

**The self-tests are unusually strong for this function.** Three identities hold exactly in
mathematics and are cheap to evaluate:

- GEOMEAN of one value is that value.
- GEOMEAN of n copies of a value is that value.
- GEOMEAN(c·x₁, …, c·xₙ) = c · GEOMEAN(x₁, …, xₙ) for any positive c.

The third is scale equivariance, and it is the most informative: an implementation that satisfies
it exactly is almost certainly doing exponent bookkeeping; one that satisfies it only
approximately is in log space.

## What has not been checked

No Handbook vector suite exists for `GEOMEAN`, and **no evidence record in the Handbook's
collection names this surface as a subject.** Nobody has checked this function against Excel
within this record — not a pass count, not a witness point, not a single comparison.

The presence projection additionally records that the implementing module carries no unit tests
of its own, so even the reference engine's behaviour here is less exercised than most.

What exists is the projected battery of the reference engine's own answers on a fixed probe list,
with no Excel on either side.

Inputs I would probe first, and why:

1. **`GEOMEAN` of a single very large value, and of a single very small one.** The identity
   GEOMEAN(x) = x is exact, so any deviation is unambiguous evidence that the implementation runs
   through the logarithm — and the reference engine already fails it. One cell settles the
   algorithm question, which is worth more than a broad sweep.
2. **Scale equivariance**: the same data multiplied by 2^k for a range of k, checking that the
   answer scales exactly. This separates exponent-bookkeeping implementations from log-space ones
   even when the single-value test happens to pass.
3. **A range containing a zero versus a range containing a blank** in the same position — the
   documented include/ignore split, and the likeliest real-world cause of a surprising `#NUM!`.
4. **A logical and a numeric-text value, once as direct arguments and once in a range** — the
   documented two-regime scan policy, which no other means-family probe covers as sharply.
5. **A range with no admitted values at all**, to settle the undocumented empty case and to
   compare it directly against `HARMEAN`'s answer on the same input. The sibling divergence
   recorded above makes this a two-function probe with one input.
6. **A long range of values far from 1** — say a thousand values near 10⁶ — comparing `GEOMEAN`
   against `EXP(AVERAGE(LN(range)))`. Agreement identifies the log route; disagreement means
   Excel is doing something else, which would be a genuine finding.
7. **Values whose product would overflow but whose geometric mean does not**, to confirm that no
   direct-product path survives anywhere.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| geometric mean | The n-th root of the product; the mean that preserves products |
| AM–GM–HM chain | HARMEAN ≤ GEOMEAN ≤ AVERAGE, with equality iff all values are equal |
| scale equivariance | GEOMEAN(c·x) = c·GEOMEAN(x); an exact identity and a strong self-test |
| log-sum route | Computing exp(mean of logs); never overflows, but amplifies error |
| scaled product | Accumulating mantissa and exponent separately; the accurate alternative |
| ln-amplification | Relative error growth incurred by exponentiating a computed logarithm |
| include-zero rule | The documented rule that zeros in a scanned range count, and therefore error |

## Sources

- Microsoft Learn, "WorksheetFunction.GeoMean method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.geomean>
  (the description and its compound-growth framing, the direct-argument versus array/reference
  scan rules including the include-zeros clause, the error-value and untranslatable-text clause,
  and the `#NUM!` condition for any data point ≤ 0). The worksheet-surface page at
  `support.microsoft.com` was not retrievable at curation time.
- Microsoft Learn, "WorksheetFunction.HarMean method (Excel)" — the source of the "always less
  than" wording discussed under **Relationships**.
- G. H. Hardy, J. E. Littlewood & G. Pólya, *Inequalities*, chapter 2 — the means and the
  non-strict AM–GM–HM chain with its equality condition.
- N. J. Higham, *Accuracy and Stability of Numerical Algorithms* — summation error and the
  analysis behind the scaled-product technique.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — the direct-argument
  versus range-scan asymmetry this function's documented rules instantiate.
- Handbook projections `data/functions/FUNC.GEOMEAN.json` (arity 1–255, `NumsToNum` kernel class,
  `AggregateDirectAndRangeDualPolicy` coercion profile) and `data/presence/FUNC.GEOMEAN.json`
  (the `geomean_fn` module, no sibling surfaces, no module-level tests).
- OxFunc `crates/oxfunc_core/src/functions/geomean_fn.rs` at commit `473efa3` — the naive
  log-accumulation kernel, its `value ≤ 0 → #NUM!` guard, and its `#NUM!` on an empty admitted set.
