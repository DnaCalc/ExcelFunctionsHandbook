---
schema: efh.function-page/v1
function_id: FUNC.SUMX2PY2
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — SUMX2PY2 function"
    locator: "https://support.microsoft.com/en-us/office/sumx2py2-function-826b60b4-0aa2-4e5e-81d2-be704d3d786f"
    role: "documented signature, the defining sum, the ignored-values remark, and the #N/A length-mismatch condition"
  - work: "Blue, A Portable Fortran Program to Find the Euclidean Norm of a Vector (TOMS, 1978)"
    locator: null
    role: "the scaled accumulation that removes the overflow and underflow bands in a sum of squares"
  - work: "Higham, Accuracy and Stability of Numerical Algorithms"
    locator: "chapter on summation"
    role: "error bounds for summation orders over non-negative terms"
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
  The all-positive member: the paired-array aggregate that cannot cancel, whose only real hazard
  is the dynamic range of the squares.
---

## What it computes

`SUMX2PY2(array_x, array_y)` returns

    SUM over i of (x_i^2 + y_i^2)

- **Domain**: two equal-length lists of real numbers. Microsoft documents that unequal lengths
  give `#N/A`.
- **Range**: `[0, +infinity)`. Every term is non-negative, so the sum is monotone in every
  input's magnitude and is zero only when all contributing values are zero.
- **Interpretation**: `SUMX2PY2(x, y) = ||x||^2 + ||y||^2` — the total squared Euclidean norm of
  the two vectors taken together. Equivalently it is `SUMSQ` applied to the concatenation of the
  two arrays, which is the cleanest way to think about it: **the pairing does not actually
  matter to the answer.** Addition is commutative and every term enters positively, so any
  re-pairing of the same multiset of values gives the same mathematical result. That property is
  unique to this member of the trio and it has a practical consequence noted below.
- **Identities**: `SUMX2PY2(x, y) = SUMSQ(x) + SUMSQ(y)`;
  `SUMX2PY2(x, y) = SUMXMY2(x, y) + 2*SUMPRODUCT(x, y)`, which is the polarization identity and
  the reason this function turns up in regression bookkeeping.

Microsoft's page states the equation as the sum of `x^2 + y^2` over corresponding values.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `array_x` | The first array or range of values. Required. | — |
| `array_y` | The second array or range of values. Required. | — |

Exactly two arguments; the reference engine records an arity of exactly two. Not variadic.

The documented scan rule: "If an array or reference argument contains text, logical values, or
empty cells, those values are ignored; however, cells with the value zero are included."

Because the pairing is mathematically irrelevant here, the ambiguity that matters for
`SUMX2MY2` and `SUMXMY2` — whether ignoring a value in one array also drops its partner — has a
*visible* consequence for this function too, but of a different kind. If ignoring is pairwise,
a text cell in `array_x` silently removes a perfectly good number from `array_y` as well, and
the total drops by that number's square. If ignoring is independent, the total keeps it. The
documentation does not say which, and the two readings give different numbers. It is the first
probe below.

The other commonly misunderstood point is the length check: the function requires equal counts
even though the answer would be well defined for unequal ones. The requirement is structural,
not mathematical.

## Result and edge cases

Returns `Number`, always non-negative.

- **Length mismatch**: `#N/A`, as documented — not `#VALUE!`. See the family divergence below.
- **Zero values are included**, per the documented remark.
- **Overflow.** The characteristic edge. The squares carry twice the exponent range of the
  inputs, so a single term overflows once `|x|` passes roughly the square root of the largest
  double — even though a scaled computation would return a finite answer. Because all terms are
  positive, overflow is monotone and irreversible: once the accumulator saturates there is no
  later term that can bring it back.
- **Underflow.** Symmetrically, values below about `1e-160` square to subnormals and then to
  zero, so a vector of small entries can return exactly zero when the true sum of squares is a
  representable positive number.
- **Empty or all-ignored input**: not documented; zero is the family-consistent answer and the
  Handbook has not checked.

The reference engine implements this function in the shared `sumproduct_family` module with
`error_collapse_profile: ReductionFold`, `numerical_reduction_policy=SequentialLeftFold`,
`error_algebra=CanonicalExcelLegacy`, and `arg_preparation_profile: RefsVisibleInAdapter`. Its
projected `real_result_policy` reads `arg_domain_guard=none` with `non_finite=allow`.

**A divergence worth recording, at family level.** This function and its two siblings are
documented to return **`#N/A`** on a length mismatch; `SUMPRODUCT`, implemented in the *same*
reference-engine module, is documented to return **`#VALUE!`** on a dimension mismatch. One
module, two documented codes for the same class of caller error. The Handbook publishes the
inconsistency rather than harmonizing it.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#N/A` | `array_x` and `array_y` have a different number of values | Documented by Microsoft |
| `#VALUE!` | An argument does not resolve to an array or reference of values | Shared call model; not stated on the page |
| propagated | An error value among the scanned data surfaces as that error | Shared coercion rules and the module's `ReductionFold` |

The documented ignore-list names text, logicals and empty cells; error values are not mentioned,
so their treatment is unstated by the documentation.

## Relationships

- **`SUMSQ`** — `SUMX2PY2(x, y) = SUMSQ(x) + SUMSQ(y)`, exactly. Since `SUMSQ` is variadic and
  accepts both ranges in one call, `SUMSQ(x, y)` computes the same mathematical quantity with a
  different (and looser) length requirement. The two differ only in the structural check and in
  the summation order.
- **`SUMX2MY2`** — the same construction with a minus. That one cancels; this one cannot. If you
  only read one paragraph of the trio's numerical notes, read `SUMX2MY2`'s.
- **`SUMXMY2`** — the sum of squared differences; related to this function by
  `SUMXMY2 = SUMX2PY2 - 2*SUMPRODUCT(x, y)`.
- **`SUMPRODUCT`** — the general member, with the differing documented mismatch code.
- **`SQRT(SUMX2PY2(x, y))`** — the Euclidean norm of the concatenated vectors, and the idiom to
  reach for given that Excel has no norm function.
- **Confused with**: `SUMX2MY2` and `SUMXMY2`, whose names differ only in the placement of a
  letter. A useful reading rule: everything before the final `2` describes the expression; the
  trailing `2` means "squared". `SUMX2PY2` is `x²+y²`; `SUMXMY2` is `(x−y)²`.

## Numerical notes

This is the placid member of the trio. There is **no cancellation** — every term is
non-negative, so the running sum is monotone and the relative error of the result is bounded by
the accumulated rounding of the additions. Everything difficult about `SUMX2PY2` is about
*range*, not about *accuracy*.

**1. Dynamic range is the whole problem.** Squaring doubles the exponent, so a set of inputs
spanning the ordinary working range produces squares spanning twice that, and the two ends of
that span behave badly:

- Above the square root of the largest double, a term overflows on its own.
- Below roughly the fourth root of the smallest normal, a term underflows to zero, silently
  discarding a contribution.

The remedy is the standard scaled accumulation from Blue's TOMS algorithm and the LAPACK
`dnrm2` line: find the largest magnitude `m` over both arrays, accumulate `SUM (v/m)^2`, and
multiply by `m^2` at the end. Blue's three-accumulator refinement handles data that spans the
whole range in a single pass. This removes both bands exactly, and it costs one extra pass or a
running maximum. Nothing in the interface tells you whether it is done, and probe 3 below is the
way to find out.

**2. Summation order, with a non-negative simplification.** The reference engine declares a
sequential left fold. For non-negative terms the error bound is the benign case of Higham's
analysis: the worst case still grows linearly with the count, but there is no cancellation to
amplify it, so the *relative* error of the result stays proportional to the number of terms
times the unit roundoff. The failure mode that remains is **swallowing**: after a few large
terms the accumulator is big, and subsequent small terms can contribute nothing at all. A vector
containing one dominant value and a long tail of small ones is the case where a left fold and a
pairwise sum give visibly different answers.

**3. Squaring is exact in the significand, inexact in the result.** `x*x` carries one rounding,
which is the minimum possible. There is no accuracy to be gained by a cleverer squaring; the
gains are all in the scaling and the summation order.

**4. What a careful implementation does**: scale by the global maximum magnitude, accumulate
pairwise or with compensation, unscale once. Where the data is well inside the normal range the
result is indistinguishable from the naive one, which is why this function looks easy — the
difference only appears at the ends, and it appears as the difference between an answer and an
error.

## What has not been checked

No Handbook evidence record lists `FUNC.SUMX2PY2` in its subjects, and no Handbook vector suite
exists for it. **Nobody has checked this function against Excel within the Handbook's record.**
The documented remarks are Microsoft's; the summation policy is the reference engine's declared
axis and not an observation of Excel.

Probes worth running first:

1. **The pairing question.** A text cell at one position of `array_x`, with a large number at
   the matching position of `array_y`. Pairwise exclusion drops that number's square from the
   total; independent compaction keeps it. Construct the data so the two answers are far apart.
   Documented nowhere, decided by one probe.
2. **Length mismatch versus shape mismatch.** Unequal counts (expect `#N/A`), and equal counts
   in different shapes — a 2x3 against a 3x2, and a row against a column — which distinguishes
   "different number of values" from "different dimensions" in the documented wording. The
   contrast with `SUMPRODUCT`'s `#VALUE!` should be run in the same session.
3. **The overflow band.** A single pair near the square root of the largest double. If Excel
   returns a number the implementation scales; if it errors or saturates it does not. Decisive,
   one probe.
4. **The underflow band.** Values around `1e-180`, whose squares are subnormal or zero, compared
   against `SUMSQ` on the same data.
5. **`SUMX2PY2(x, y)` against `SUMSQ(x) + SUMSQ(y)` and against `SUMSQ(x, y)`** — three routes
   to the same number, in one workbook, no oracle required. Disagreement localizes the summation
   order.
6. **Summation order**, using one dominant pair followed by a long tail of tiny ones, in both
   orders.
7. **An error value inside one array**, which the documented ignore-list does not cover.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| dynamic range | The span of exponents the squares occupy — twice that of the inputs |
| scaled accumulation | Dividing by the largest magnitude before squaring, and unscaling once at the end |
| swallowing | A small term contributing nothing because the accumulator is already large |
| pairwise exclusion | Dropping both partners when either is ignored, preserving alignment |
| left fold | Adding terms in order into one accumulator; the module's declared reduction policy |

## Sources

- Microsoft, "SUMX2PY2 function" —
  <https://support.microsoft.com/en-us/office/sumx2py2-function-826b60b4-0aa2-4e5e-81d2-be704d3d786f>
  (signature, the defining equation, the ignored-values remark including "cells with the value
  zero are included", and the `#N/A` length-mismatch condition).
- Microsoft, "SUMPRODUCT function" —
  <https://support.microsoft.com/en-us/office/sumproduct-function-16753e75-9f68-4874-94ac-4d2145a2fd2e>
  (cited for the contrasting `#VALUE!` dimension-mismatch code recorded as a divergence).
- Blue, "A Portable Fortran Program to Find the Euclidean Norm of a Vector", *ACM TOMS*, 1978;
  the LAPACK `dnrm2` lineage — scaled accumulation.
- Higham, *Accuracy and Stability of Numerical Algorithms* — summation error bounds for
  non-negative terms.
- Handbook projections `data/functions/FUNC.SUMX2PY2.json` (arity exactly 2,
  `error_collapse_profile: ReductionFold`, `numerical_reduction_policy=SequentialLeftFold`,
  `arg_preparation_profile: RefsVisibleInAdapter`) and `data/presence/FUNC.SUMX2PY2.json`
  (shared `sumproduct_family` module).
- Handbook [SUMSQ](FUNC.SUMSQ.md), [SUMX2MY2](FUNC.SUMX2MY2.md), and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
