---
schema: efh.function-page/v1
function_id: FUNC.SUMPRODUCT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — SUMPRODUCT function"
    locator: "https://support.microsoft.com/en-us/office/sumproduct-function-16753e75-9f68-4874-94ac-4d2145a2fd2e"
    role: "documented signature, the 2-to-255 array range, the #VALUE! dimension-mismatch condition, and the non-numeric-as-zero remark"
  - work: "Higham, Accuracy and Stability of Numerical Algorithms"
    locator: "chapter on the dot product; error bounds and the condition number"
    role: "the standard error analysis of an inner product and the cancellation criterion"
  - work: "Ogita, Rump & Oishi, Accurate Sum and Dot Product (SIAM J. Sci. Comput., 2005)"
    locator: null
    role: "compensated dot product algorithms that deliver a result as if computed in doubled precision"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - The conditional-sum idiom
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: sumproduct_family
role_in_family: >-
  The general member: an n-way elementwise product reduced by summation, from which the three
  fixed-shape paired-array functions are special cases.
---

## What it computes

`SUMPRODUCT(array1, [array2], ...)` multiplies corresponding elements of its array arguments and
sums the products:

    SUM over i of (a1_i * a2_i * ... * ak_i)

- **Domain**: `k` arrays of identical shape. Microsoft documents that arrays of differing
  dimensions give `#VALUE!`.
- **Range**: all reals.
- **With two arrays this is the inner product** `x · y` — the fundamental bilinear form of
  Euclidean geometry. Everything the dot product means is available here: `SUMPRODUCT(x, x)` is
  the squared norm, `SUMPRODUCT(x, y)` divided by the product of the norms is the cosine of the
  angle between the vectors, and `SUMPRODUCT(x, y) = 0` says the vectors are orthogonal.
  Cauchy-Schwarz bounds the result: `|x · y| <= ||x|| ||y||`.
- **With one array** it degenerates to `SUM`. With three or more it is a multilinear contraction
  — a sum over the diagonal of a `k`-fold tensor product — which has no standard name in
  spreadsheet vocabulary but is exactly what the formula says.
- **Identities**: `SUMPRODUCT(x, x) = SUMSQ(x)`;
  `SUMPRODUCT(x-y, x+y) = SUMX2MY2(x, y)`;
  `SUMX2PY2(x,y) - 2*SUMPRODUCT(x,y) = SUMXMY2(x,y)`. The three fixed-shape members of this
  module are all `SUMPRODUCT` in disguise.
- **Weighted averages**: `SUMPRODUCT(values, weights) / SUM(weights)` is the standard idiom, and
  is why this function appears in more finance worksheets than any other in the category.

Microsoft documents an alternative calling style in which the commas are replaced by arithmetic
operators — `SUMPRODUCT((A1:A10)*(B1:B10))` and similar — with the arrays parenthesized to
control evaluation order.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `array1` | The first array argument. Required. | — |
| `array2, array3, ...` | Further arrays, documented as "array arguments 2 to 255". Optional. | — |

**A divergence worth recording.** Microsoft documents the argument list as arrays 2 to 255 — a
bounded list. The reference engine's projection records this surface's arity as a minimum of one
and a maximum of `unbounded`. The minimum agrees with the documented optionality of everything
after the first array; the maximum does not agree with the documented 255. The Handbook records
the mismatch: a caller writing more than the documented number of arrays is in territory the
documentation excludes and the reference engine's classification admits, and nobody in this
record has checked which one Excel enforces.

The commonly misunderstood aspects are two:

1. **Shape, not length.** Unlike the paired-array trio, `SUMPRODUCT` compares *dimensions*. A
   row range and a column range of the same length are not the same shape, and the documented
   consequence is `#VALUE!` — though in modern Excel a row and a column also *broadcast* into a
   rectangle under the dynamic-array evaluation rules, which is a different behaviour reached by
   a different path. Which one applies to a given formula is not settled here.
2. **Non-numeric entries become zero, not skipped.** Microsoft states it plainly: "SUMPRODUCT
   treats non-numeric array entries as if they were zeros." For a *product* that is a much
   stronger statement than for a sum: one text cell zeroes its entire product term across all
   `k` arrays. This is also the exact opposite of the trio's documented rule, where text is
   *ignored* — and the two functions live in the same reference-engine module. Recorded below as
   a family-level divergence.

## The conditional-sum idiom

`SUMPRODUCT` is used far more often as a conditional aggregator than as a dot product, and the
mechanism is worth stating because it depends on the coercion rule above.

A comparison over a range produces an array of logicals: `(A1:A10>5)` is
`{TRUE;FALSE;...}`. Multiplying two such arrays and summing counts the rows where both hold.
But logicals inside an array are *non-numeric entries* under the documented rule, so they must
be forced to numbers first. The two idiomatic forms are:

    =SUMPRODUCT(--(A1:A10>5), --(B1:B10="x"))
    =SUMPRODUCT((A1:A10>5) * (B1:B10="x"))

The double unary minus (`--`) is a coercion, not a sign trick: the first negation makes the
logical a number, the second restores the sign. The multiplication form coerces through the
arithmetic operator instead. Both work; the `*` form additionally makes the AND semantics
visible, and `+` gives OR (with the caveat that overlapping conditions then count twice).

Historically this idiom was the only way to write multi-criteria aggregation, and it is the
reason `SUMPRODUCT` appears in workbooks that have no dot product anywhere in them.
`SUMIFS`, `COUNTIFS` and `AVERAGEIFS` supersede it for the common cases and are clearer; but
`SUMPRODUCT` remains strictly more general, because its conditions are arbitrary array
expressions rather than criteria strings, and because it can weight as well as filter.

## Result and edge cases

Returns `Number`.

- **Dimension mismatch**: `#VALUE!`, as documented.
- **A single array**: behaves as `SUM` over it.
- **Non-numeric entries**: zero, per the documented remark — including text, logicals and,
  presumably, empty cells, though the documentation does not enumerate them.
- **Error values in the data**: not addressed by the documented remark. Under the shared
  coercion discipline an error propagates rather than being treated as zero; the reference
  engine declares `error_collapse_profile: ReductionFold` with
  `error_algebra=CanonicalExcelLegacy` for this module, which is a fold rather than a mask.
- **Full-column references**: Microsoft advises against them for performance. The advice is real
  — the function has no way to know that most of a column is empty until it has looked.

The reference engine records `arg_preparation_profile: RefsVisibleInAdapter` (the function sees
live references), `coercion_lift_profile: Custom`, `numerical_reduction_policy=SequentialLeftFold`
and a projected `real_result_policy` of `arg_domain_guard=none` with `non_finite=allow`.

**Two family-level divergences, recorded rather than resolved.** `SUMPRODUCT` shares its
implementing module with `SUMX2MY2`, `SUMX2PY2`, `SUMXMY2` and `SERIESSUM`, and it disagrees
with the first three on two documented points at once:

| | `SUMPRODUCT` | `SUMX2MY2` / `SUMX2PY2` / `SUMXMY2` |
|---|---|---|
| Mismatched inputs | `#VALUE!` (dimensions) | `#N/A` (number of values) |
| Text / logicals in the data | treated as **zero** | **ignored** |

Both rows come from Microsoft's own pages. For a sum, "zero" and "ignored" happen to coincide;
for a product they emphatically do not, since a zeroed factor annihilates its whole term. An
implementation sharing a scan across the module has to keep these apart deliberately.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | The array arguments do not have the same dimensions | Documented by Microsoft |
| `#VALUE!` | An argument does not resolve to an array or reference | Shared call model |
| propagated | An error value among the scanned data surfaces as that error | Shared coercion rules and the module's `ReductionFold` |

## Relationships

- **`SUMX2MY2`, `SUMX2PY2`, `SUMXMY2`** — fixed-shape special cases in the same module, with the
  different documented conventions tabulated above.
- **`SUMSQ`** — `SUMPRODUCT(x, x)`.
- **`SERIESSUM`** — the module's other member: a power series rather than an inner product, but
  the same reduce-a-product-list shape.
- **`SUMIF`, `SUMIFS`, `COUNTIFS`, `AVERAGEIFS`** — the criteria family that supersedes the
  conditional-sum idiom for the common cases. They are clearer and generally faster; they are
  also less general, and they use a criteria-string language rather than array expressions.
- **`MMULT`** — matrix multiplication, which is the inner product generalized in the other
  direction (over pairs of indices rather than over more arrays). `SUMPRODUCT(x, y)` equals
  `MMULT(TRANSPOSE(x), y)` for column vectors, and where a whole matrix of inner products is
  wanted, `MMULT` is the right tool.
- **`SUM` with an array formula** — `SUM((A:A>5)*(B:B))` entered as an array formula does the
  same job; `SUMPRODUCT` was for decades the way to get array evaluation without the
  Ctrl+Shift+Enter ritual, which is a large part of its historical popularity.
- **Confused with**: `SUMIFS` (criteria strings, not array expressions), and `PRODUCT` (which
  multiplies everything together and does not sum).

## Numerical notes

The inner product is one of the most studied kernels in numerical analysis, and its behaviour is
completely characterized.

**Conditioning.** The relative condition number of a dot product is

    2 * SUM |x_i y_i|  /  |SUM x_i y_i|

— the ratio of the sum of absolute term magnitudes to the absolute value of the answer. When the
terms are all the same sign this is about `2` and the computation is perfectly benign. When the
result is small relative to its terms — near-orthogonal vectors, a residual, a balanced
portfolio, a centred correlation — the condition number is unbounded and the computed result can
have **no correct digits at all**. This is not an implementation flaw; it is the problem being
ill-posed, and no algorithm in working precision can fix it.

The practical statement: `SUMPRODUCT` is exact-ish when you use it as a weighted total, and
dangerous when you use it to detect orthogonality or to compute a residual.

**Error bound.** For the naive left-to-right evaluation, the computed dot product satisfies the
classical bound in Higham: the error is at most about `n` times the unit roundoff times the sum
of absolute term magnitudes. Note what the bound is proportional to — the *sum of magnitudes*,
not the answer. That is the same statement as the condition number above, read from the other
side.

**The three improvements, in increasing order of cost:**

1. **Fused multiply-add.** Computing each `x_i*y_i + acc` in one `fma` removes the rounding of
   the product entirely, halving the error term at no cost on hardware that has it. It also makes
   the result depend on whether `fma` was used, which is a reproducibility hazard — a
   portable-reproducible implementation has to choose one and pin it.
2. **Pairwise summation**, which replaces the factor `n` in the bound by its logarithm, at
   essentially no cost.
3. **Compensated dot product** — Ogita, Rump and Oishi's `Dot2` and its relatives — which
   delivers a result as accurate as if the whole computation had been done in doubled precision,
   and which can therefore return correct digits even in the ill-conditioned cases above. This is
   what a mathematically-correct flavour of `SUMPRODUCT` should do.

The reference engine declares a **sequential left fold** for this module, which is option zero:
no `fma`, no pairing, no compensation. That is the ordinary choice for an Excel-compatibility
implementation, since the target is another sequential fold, and it is the weakest choice for
accuracy. The Handbook makes no claim about what Excel does internally; the declaration
described here is the reference engine's own axis.

**Order dependence.** Because floating-point addition is not associative, the answer depends on
the accumulation order. Two implementations that differ only in summation order will disagree in
the last bits on almost any real data set. When comparing a `SUMPRODUCT` against a hand-built
`SUM(x*y)` array formula, expect small differences and do not read them as a defect in either.

## What has not been checked

No Handbook evidence record lists `FUNC.SUMPRODUCT` in its subjects, and no Handbook vector
suite exists for it. **Nobody has checked this function against Excel within the Handbook's
record.** The documented remarks above are Microsoft's; the arity, reduction policy and error
algebra are the reference engine's declared axes rather than observations of Excel.

Probes worth running first:

1. **The argument-count boundary.** Formulas with 255 and 256 array arguments, against the
   documented "2 to 255" and the reference engine's `unbounded`. This is the divergence recorded
   above and it is settled by two formulas.
2. **Text and logicals in the data.** A range containing a text cell, a `TRUE`, and an empty
   cell, in `SUMPRODUCT` and in `SUMXMY2` on the same data. This tests the documented
   zero-versus-ignore split across two functions in one module — the second divergence recorded
   above, and the one with the larger practical consequence.
3. **Row against column.** Equal-length ranges of different orientation: documented `#VALUE!`
   versus modern broadcasting. Run in a dynamic-array-capable build and in a legacy one.
4. **An error value in one array**, which the documented remark does not cover, and which
   distinguishes a fold from a mask.
5. **Ill-conditioned inner products.** Two near-orthogonal vectors whose exact dot product is
   known analytically. Comparing the returned value against the exact one measures the
   implementation's summation strategy directly, and it is the probe that would reveal
   compensation if any is present.
6. **Order dependence.** The same data with the rows reversed. A compensated or pairwise
   implementation is much more nearly order-invariant than a left fold.
7. **`SUMPRODUCT(x, x)` against `SUMSQ(x)`** — same value, two functions, two modules.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| inner product | The two-array case; the dot product of Euclidean geometry |
| conditioning | The ratio of summed term magnitudes to the answer; unbounded near orthogonality |
| double unary | The `--` idiom that coerces an array of logicals to an array of numbers |
| left fold | Adding terms in order into one accumulator; the module's declared reduction policy |
| compensated dot product | An algorithm returning the accuracy of doubled-precision evaluation |

## Sources

- Microsoft, "SUMPRODUCT function" —
  <https://support.microsoft.com/en-us/office/sumproduct-function-16753e75-9f68-4874-94ac-4d2145a2fd2e>
  (signature; "array arguments 2 to 255"; "The array arguments must have the same dimensions. If
  they do not, SUMPRODUCT returns the #VALUE! error value"; "SUMPRODUCT treats non-numeric array
  entries as if they were zeros"; the operator-substitution style; and the full-column
  performance advice).
- Microsoft's `SUMX2MY2`, `SUMX2PY2` and `SUMXMY2` pages — cited for the contrasting `#N/A`
  mismatch code and ignore-rather-than-zero convention recorded as divergences.
- Higham, *Accuracy and Stability of Numerical Algorithms* — dot-product error bounds and the
  condition number.
- Ogita, Rump & Oishi, "Accurate Sum and Dot Product", *SIAM J. Sci. Comput.*, 2005 — compensated
  algorithms.
- Handbook projections `data/functions/FUNC.SUMPRODUCT.json` (arity minimum 1 and maximum
  `unbounded`, `error_collapse_profile: ReductionFold`,
  `numerical_reduction_policy=SequentialLeftFold`, `arg_preparation_profile: RefsVisibleInAdapter`)
  and `data/presence/FUNC.SUMPRODUCT.json` (module shared with `SERIESSUM`, `SUMX2MY2`,
  `SUMX2PY2`, `SUMXMY2`).
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md).
