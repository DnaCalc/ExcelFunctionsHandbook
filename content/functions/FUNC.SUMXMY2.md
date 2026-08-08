---
schema: efh.function-page/v1
function_id: FUNC.SUMXMY2
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — SUMXMY2 function"
    locator: "https://support.microsoft.com/en-us/office/sumxmy2-function-9d144ac1-4d79-43de-b524-e2ecee23b299"
    role: "documented signature, the defining sum, the ignored-values remark, the #N/A length-mismatch condition, and the worked example"
  - work: "Higham, Accuracy and Stability of Numerical Algorithms"
    locator: "chapters on summation and on cancellation; Sterbenz's lemma"
    role: "why the subtraction here is benign and why the summation order is the remaining question"
  - work: "Blue, A Portable Fortran Program to Find the Euclidean Norm of a Vector (TOMS, 1978)"
    locator: null
    role: "scaled accumulation for a sum of squares whose terms span a wide range"
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
family: sumproduct_family
role_in_family: >-
  The squared-distance member: the paired-array aggregate that computes the residual sum of
  squares, and the one whose subtraction happens before the squaring rather than after.
---

## What it computes

`SUMXMY2(array_x, array_y)` returns

    SUM over i of (x_i - y_i)^2

- **Domain**: two equal-length lists of real numbers. Microsoft documents that arrays with
  different numbers of values give `#N/A`.
- **Range**: `[0, +infinity)`. Zero exactly when the two arrays agree elementwise.
- **Interpretation**: the **squared Euclidean distance** between the two vectors,
  `||x - y||^2`. This is the residual sum of squares of regression, the objective function of
  least squares, the numerator of the mean squared error, and the quantity minimized by every
  curve fit in the statistics category. Of the three paired-array functions it is by far the most
  used, and it is the one with a genuine claim to being a primitive rather than a convenience.
- **Identities**: `SUMXMY2(x, y) = SUMX2PY2(x, y) - 2*SUMPRODUCT(x, y)` (the polarization
  identity); `SUMXMY2(x, y) = SUMSQ(x-y)` if the difference is formed first;
  `SQRT(SUMXMY2(x, y))` is the Euclidean distance.
- **Metric properties**: `SUMXMY2` is symmetric in its arguments and vanishes only on equality,
  but it is *not* a metric — the triangle inequality holds for its square root, not for it.
  Nothing in the function enforces or checks this; it matters when the result is fed into
  clustering or nearest-neighbour logic that assumes a metric.

Microsoft's page states the equation as the sum of squared differences and gives a worked
example over two seven-element arrays.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `array_x` | The first array or range of values. Required. | — |
| `array_y` | The second array or range of values. Required. | — |

Exactly two arguments; the reference engine records an arity of exactly two. Not variadic.

The documented scan rule is the trio's: "If an array or reference argument contains text,
logical values, or empty cells, those values are ignored. However, cells with the value zero are
included."

**The pairing gap matters more here than anywhere else in the trio.** For `SUMX2PY2` the pairing
is mathematically irrelevant; for `SUMX2MY2` it changes the answer; for `SUMXMY2` a
mis-alignment is *catastrophic*, because the whole point of the function is that `x_i` and `y_i`
are the same observation measured two ways. If ignoring a text cell in `array_x` compacts that
array independently, every subsequent pair is shifted by one and the result is the distance
between two vectors that were never meant to be compared — a large, plausible-looking number
with no diagnostic. The documentation does not say which reading applies. This is the first
probe below and it is the most consequential unstated behaviour on the page.

The second misunderstood point is the order of operations, encoded in the name: the subtraction
happens **before** the squaring. `SUMXMY2` is `sum of (x - y) squared`, not
`sum of x - (y squared)` and not `sum of (x squared) - (y squared)` — the last of which is
`SUMX2MY2`, a different function with a nearly identical name.

## Result and edge cases

Returns `Number`, always non-negative.

- **Length mismatch**: `#N/A`, as documented — not `#VALUE!`. See the family divergence below.
- **Identical arrays**: exactly zero, and exactly zero is achievable here because each term is
  the square of an exactly-computed difference (see *Numerical notes*).
- **Zero values are included**, per the documented remark.
- **Overflow**: a difference near the top of the range squares to overflow, though this requires
  inputs that are themselves enormous — less reachable than for `SUMSQ`, because the difference
  is usually much smaller than the operands.
- **Underflow**: a very small difference squares to zero. For nearly-equal arrays every term can
  underflow individually while the true sum is representable, which is the mirror image of the
  overflow case and the more likely one in practice.
- **Empty or all-ignored input**: not documented; zero is the family-consistent answer and the
  Handbook has not checked.

The reference engine implements this function in the shared `sumproduct_family` module with
`error_collapse_profile: ReductionFold`, `numerical_reduction_policy=SequentialLeftFold`,
`error_algebra=CanonicalExcelLegacy`, and `arg_preparation_profile: RefsVisibleInAdapter`. Its
projected `real_result_policy` reads `arg_domain_guard=none` with `non_finite=allow`.

**A divergence worth recording, at family level.** This function and its two siblings are
documented to return **`#N/A`** on a length mismatch; `SUMPRODUCT`, implemented in the *same*
reference-engine module, is documented to return **`#VALUE!`** on a dimension mismatch. One
module, two documented codes. The Handbook publishes the inconsistency.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#N/A` | `array_x` and `array_y` do not have the same number of values | Documented by Microsoft |
| `#VALUE!` | An argument does not resolve to an array or reference of values | Shared call model; not stated on the page |
| propagated | An error value among the scanned data surfaces as that error | Shared coercion rules and the module's `ReductionFold` |

Error values are not named in the documented ignore-list, so their treatment is unstated by the
documentation.

## Relationships

- **`SUMX2MY2`** — the near-homograph, and a genuinely different function: subtraction after
  squaring rather than before. That one cancels catastrophically; this one does not. Confusing
  the two is the most likely user error in this corner of the category.
- **`SUMX2PY2`** — the third member, and the other half of the polarization identity.
- **`SUMPRODUCT`** — the general member, with the differing documented mismatch code. Note that
  `SUMPRODUCT((x-y)^2)` computes the same value and is what a reader writes when they have not
  found `SUMXMY2`.
- **`DEVSQ`** — the one-array analogue: `SUMXMY2` against a constant mean is `DEVSQ`.
- **`STEYX`, `LINEST`, `TREND`, `RSQ`, `FORECAST.LINEAR`** — the regression consumers. Every one
  of them minimizes or reports a quantity of this shape, which is why this function exists.
- **`SQRT(SUMXMY2(x, y))`** — Euclidean distance, the composition to use for nearest-neighbour
  and clustering work.
- **Confused with**: `SUMX2MY2`, as above; and with a *mean* squared error, which requires
  dividing by the count and which this function does not do.

## Numerical notes

`SUMXMY2` is, perhaps surprisingly, the **best conditioned** of the three paired-array
functions, and the reason is a small theorem worth stating.

**The subtraction is usually exact.** Sterbenz's lemma: if `y/2 <= x <= 2y` (same sign, within a
factor of two), then `x - y` is computed *exactly* in binary floating point — no rounding at
all. The regime where a subtraction would normally be feared, near-equal operands, is precisely
the regime where it is free of error. And near-equal operands are the normal case for this
function: `x` and `y` are two measurements of the same thing.

So the term `(x - y)^2` carries, in the common case, exactly one rounding (from the squaring),
and the relative error of each term is at most one unit of roundoff. Compare `SUMX2MY2`, where
the same near-equality destroys the term entirely. **The difference between the two functions is
not a matter of degree; it is the difference between an operation that loses no bits and one
that can lose all of them.** A reader choosing between the two formulations of "how far apart
are these vectors" should choose this one for that reason alone.

**Where the remaining error lives: the summation.** With the terms accurate, the accuracy of the
result is entirely a question of how they are added. The reference engine declares a sequential
left fold for this module. All terms are non-negative, so there is no cancellation in the sum
and the error bound is the benign linear-in-count case (Higham). The residual failure mode is
swallowing: a few large residuals early in the array leave the accumulator too big for the later
small ones to register. In least-squares work this is not a curiosity — the residual vector of a
good fit is exactly "a few large outliers and a long tail of small values", which is the shape
that a left fold handles worst. Pairwise summation costs nothing and fixes it; compensated
summation fixes it completely.

**Underflow of the terms.** For two nearly identical arrays, each difference may be around
`1e-170`, whose square is subnormal or zero. The sum then reads zero even though the true value
is a representable positive number. If the caller is using `SUMXMY2` as a convergence test, this
underflow is a *silent success signal* — the worst possible failure mode for that use. Scaled
accumulation (Blue) removes it; so does testing on `SQRT(SUMXMY2(...))`-scale quantities rather
than on the squared value.

**What a careful implementation does**: compute the differences first (exactly, in the common
case), scale if the differences span a wide range, and accumulate pairwise or with compensation.
The Handbook makes no claim about what Excel does.

## What has not been checked

No Handbook evidence record lists `FUNC.SUMXMY2` in its subjects, and no Handbook vector suite
exists for it. **Nobody has checked this function against Excel within the Handbook's record.**
The documented remarks are Microsoft's; the summation policy is the reference engine's declared
axis, not an observation of Excel.

Probes worth running first:

1. **The pairing question**, which for this function is the difference between a right answer
   and a meaningless one. Put a text cell at position 3 of `array_x` and complete numbers in
   `array_y`; arrange the data so that pairwise exclusion and independent compaction give
   obviously different totals. Nothing in the documentation decides it.
2. **Sterbenz confirmation.** Two arrays differing in the last few bits, where the terms are
   exact and the true sum is known analytically. Compare against `SUMPRODUCT((x-y),(x-y))` and
   against `SUMX2MY2` on the same data — the last of which should differ visibly, demonstrating
   the conditioning contrast in one workbook.
3. **The underflow-to-zero case.** Two arrays differing by around `1e-180` elementwise. If the
   result is exactly zero the implementation does not scale; if it is a small positive number it
   does. Decisive, and directly relevant to convergence-test use.
4. **Length mismatch versus shape mismatch**: unequal counts (expect `#N/A`), and equal counts in
   different shapes — a row against a column, a 2x3 against a 3x2 — which distinguishes "number
   of values" from "dimensions" in the documented wording, and should be run alongside
   `SUMPRODUCT`'s `#VALUE!` for the family contrast.
5. **Summation order**: one large residual followed by a long tail of tiny ones, in both orders.
6. **An error value in one array**, which the documented ignore-list does not cover.
7. **The documented worked example** from Microsoft's page, as a cheap sanity anchor before any
   of the above.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| squared Euclidean distance | The squared norm of the difference vector; the quantity this function computes |
| Sterbenz's lemma | The result that `x - y` is exact when the operands are within a factor of two |
| residual sum of squares | The least-squares objective; this function's principal use |
| swallowing | A small term contributing nothing because the accumulator is already large |
| pairwise exclusion | Dropping both partners when either is ignored, preserving alignment |

## Sources

- Microsoft, "SUMXMY2 function" —
  <https://support.microsoft.com/en-us/office/sumxmy2-function-9d144ac1-4d79-43de-b524-e2ecee23b299>
  (signature, the defining equation, the ignored-values remark including "cells with the value
  zero are included", the `#N/A` length-mismatch condition, and the worked example).
- Microsoft, "SUMPRODUCT function" —
  <https://support.microsoft.com/en-us/office/sumproduct-function-16753e75-9f68-4874-94ac-4d2145a2fd2e>
  (cited for the contrasting `#VALUE!` dimension-mismatch code recorded as a divergence).
- Higham, *Accuracy and Stability of Numerical Algorithms* — Sterbenz's lemma, cancellation, and
  summation error bounds.
- Blue, "A Portable Fortran Program to Find the Euclidean Norm of a Vector", *ACM TOMS*, 1978 —
  scaled accumulation.
- Handbook projections `data/functions/FUNC.SUMXMY2.json` (arity exactly 2,
  `error_collapse_profile: ReductionFold`, `numerical_reduction_policy=SequentialLeftFold`,
  `arg_preparation_profile: RefsVisibleInAdapter`) and `data/presence/FUNC.SUMXMY2.json`
  (shared `sumproduct_family` module).
- Handbook [SUMX2MY2](FUNC.SUMX2MY2.md), [SUMX2PY2](FUNC.SUMX2PY2.md), and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
