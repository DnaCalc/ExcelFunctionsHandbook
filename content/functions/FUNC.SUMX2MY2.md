---
schema: efh.function-page/v1
function_id: FUNC.SUMX2MY2
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — SUMX2MY2 function"
    locator: "https://support.microsoft.com/en-us/office/sumx2my2-function-9e599cc5-5399-48e9-a5e0-e37812dfa3e9"
    role: "documented signature, the defining sum, the ignored-values remark, and the #N/A length-mismatch condition"
  - work: "Higham, Accuracy and Stability of Numerical Algorithms"
    locator: "chapters on summation and on cancellation"
    role: "the error analysis of the difference-of-squares rewrite and of summation order"
  - work: "Kahan, On the Cost of Floating-Point Computation Without Extra-Precise Arithmetic"
    locator: null
    role: "the canonical treatment of accurate difference-of-products and difference-of-squares evaluation"
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
  The difference-of-squares member: the one paired-array aggregate whose defining formula
  cancels, and the family's best argument for a rewritten kernel.
---

## What it computes

`SUMX2MY2(array_x, array_y)` returns

    SUM over i of (x_i^2 - y_i^2)

- **Domain**: two equal-length lists of real numbers. Microsoft documents that unequal lengths
  give `#N/A`.
- **Range**: all reals. Unlike `SUMSQ` and `SUMX2PY2`, this aggregate is signed and can be zero
  for non-zero data — indeed it is zero whenever the two vectors have the same Euclidean norm,
  which is exactly the case a user is most likely to be testing.
- **The identity that defines the page**: `x^2 - y^2 = (x - y)(x + y)`. Algebraically trivial,
  numerically decisive; see *Numerical notes*.
- **Interpretation**: `SUMX2MY2(x, y) = ||x||^2 - ||y||^2`, the difference of squared Euclidean
  norms. It appears in energy-difference calculations, in the numerator of certain variance-ratio
  statistics, and in physics worksheets computing `E = m(c^2 - v^2)`-shaped quantities.

Microsoft's page states the equation as the sum of `x^2 - y^2` over corresponding pairs.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `array_x` | The first array or range of values. Required. | — |
| `array_y` | The second array or range of values. Required. | — |

Exactly two arguments; the reference engine records an arity of exactly two, so unlike
`SUMPRODUCT` this function is not variadic.

The commonly misunderstood aspect is **pairing**. The two arrays are matched element by element
in their natural order, so a row range and a column range of the same length pair up
positionally, not geometrically. The documentation speaks of "a different number of values"
rather than of shape, which suggests count rather than dimension is what is compared — but it
does not say so explicitly, and the Handbook does not assert it.

The scan rule is documented: "If an array or reference argument contains text, logical values,
or empty cells, those values are ignored. However, cells with the value zero are included."

**That remark hides a real question the documentation does not answer.** If a value in
`array_x` is ignored, is its partner in `array_y` also dropped, or does the pairing shift? Two
readings are possible — pairwise exclusion (drop both, keep the alignment) and independent
compaction (each array compacts separately, re-pairing surviving elements) — and they give
different answers on the same data. Nothing on the page settles it. It is the first probe below.

## Result and edge cases

Returns `Number`.

- **Length mismatch**: `#N/A`, as documented. Note the code: `#N/A`, not `#VALUE!`.
- **Empty pairs / all values ignored**: not documented. Zero is the family-consistent answer;
  the Handbook has not checked.
- **Zero values are included**, per the documented remark, which matters because a zero
  contributes nothing to the sum but does occupy a pairing slot.
- **Overflow**: the squares carry twice the exponent range of the inputs, so a term can overflow
  even where the mathematical result is small. This is `SUMSQ`'s hazard, inherited.
- **Cancellation**: the defining hazard of this surface. Discussed below.

The reference engine implements this function in the shared `sumproduct_family` module, with
`error_collapse_profile: ReductionFold`, `numerical_reduction_policy=SequentialLeftFold` and
`error_algebra=CanonicalExcelLegacy`. Its projected `real_result_policy` reads
`arg_domain_guard=none` with `non_finite=allow`, and its argument-preparation profile is
`RefsVisibleInAdapter` — the function sees live references rather than pre-resolved values.

**A divergence worth recording, at family level.** `SUMX2MY2`, `SUMX2PY2` and `SUMXMY2` are all
documented to return **`#N/A`** when the two arrays differ in length. `SUMPRODUCT` — implemented
in the *same* reference-engine module — is documented to return **`#VALUE!`** when its arrays
differ in dimension. One module, two documented error codes for what a reader would call the
same mistake. The Handbook publishes the inconsistency rather than harmonizing it: it is a real
property of the documented surface, and an implementation that shares a shape check across the
module has to keep the two codes apart deliberately.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#N/A` | `array_x` and `array_y` have a different number of values | Documented by Microsoft |
| `#VALUE!` | An argument is not an array/reference or does not resolve to values | Shared call model; not stated on the page |
| propagated | An error value among the scanned data surfaces as that error | Shared coercion rules and the module's `ReductionFold` |

The documented ignore-list covers text, logicals and empty cells; it does not mention error
values, so their treatment here is unstated by the documentation.

## Relationships

- **`SUMX2PY2`** — the same construction with a plus. Numerically it is the easy one, because
  the terms cannot cancel.
- **`SUMXMY2`** — the sum of squared differences. Note the algebra:
  `SUMXMY2 = SUMX2PY2 - 2*SUMPRODUCT(x, y)`, and `SUMX2MY2 = SUMPRODUCT(x-y, x+y)`. The three
  functions plus `SUMPRODUCT` span the same quadratic forms in different arrangements.
- **`SUMPRODUCT`** — the general member, and the one whose documented mismatch error code
  differs (see above). `SUMX2MY2(x, y)` and `SUMPRODUCT((x-y), (x+y))` are the same mathematical
  value by two routes and are the basis for the accuracy comparison below.
- **`SUMSQ`** — `SUMX2MY2(x, y) = SUMSQ(x) - SUMSQ(y)`, which is the *worst* way to compute it
  and is exactly the composition this function should save you from.
- **`F.TEST` and the variance-ratio statistics** — the consumers of a difference of sums of
  squares.
- **Confused with**: `SUMXMY2`, constantly. The names differ by the position of a single letter
  and the functions differ by which operation is inside the square. `SUMX2MY2` is
  "sum of (x-squared minus y-squared)"; `SUMXMY2` is "sum of (x minus y), squared".

## Numerical notes

**This is a cancellation function.** When `x_i` and `y_i` are close, `x_i^2` and `y_i^2` are
closer still in relative terms, and their difference loses roughly as many significant bits as
the two values share. In the extreme — `x` and `y` agreeing to within a few ULP — the computed
difference of squares can be entirely rounding error. Since the natural use of this function is
to compare two vectors of similar magnitude, **the cancellation case is the intended case, not a
pathological one.**

**The remedy is one line of algebra.** Evaluate each term as

    (x_i - y_i) * (x_i + y_i)

rather than as `x_i*x_i - y_i*y_i`. This is not a heuristic; it is provably better:

- `x - y` is computed exactly whenever `y/2 <= x <= 2y` (Sterbenz's lemma), which covers exactly
  the near-cancellation regime that destroys the naive form. The subtraction that loses
  information in the naive formula loses none in the rewritten one, because it happens *before*
  the squaring rather than after.
- `x + y` is a well-conditioned sum for same-sign data and carries at most one rounding.
- The product then carries at most one more rounding. The rewritten term has a small relative
  error where the naive term has an unbounded one.

The rewrite also improves the **overflow** picture: `x^2` overflows for `|x|` above roughly the
square root of the largest double, while `(x-y)(x+y)` overflows only when the answer itself is
near the top of the range. There is a whole band of inputs on which the naive form fails and the
rewritten form returns the right answer.

Kahan's work on difference-of-products (and the `fma`-based two-product technique that goes with
it) is the general treatment; the difference of squares is its simplest instance.

**Summation order.** The reference engine declares a sequential left fold for this module. For a
signed aggregate that can cancel, a left fold is the weakest option: the running sum can grow
large and then be cancelled back down, at which point the accumulated rounding error is measured
against the small final answer rather than against the large intermediate. Pairwise or
compensated summation (Higham) bounds this much better. There is no way to request either from
the worksheet.

**What a careful implementation does**: rewrite each term as `(x-y)(x+y)`, accumulate pairwise
or with compensation, and — if the input range is wide — scale. The result will differ from a
naive implementation, and that difference is largest exactly where users care most. The Handbook
makes no claim about which form Excel uses; probe 2 below settles it.

## What has not been checked

No Handbook evidence record lists `FUNC.SUMX2MY2` in its subjects, and no Handbook vector suite
exists for it. **Nobody has checked this function against Excel within the Handbook's record.**
The `#N/A` mismatch condition and the ignore-list are documented by Microsoft; the summation
policy is the reference engine's declared axis, not an observation of Excel.

Probes worth running first:

1. **The pairing question.** An `array_x` with a text cell at position 3 against a complete
   `array_y`. Pairwise exclusion and independent compaction give different totals; construct the
   data so the two answers are far apart. This is the documented gap named above and the highest
   value probe here.
2. **The cancellation probe.** Two arrays agreeing to within a few ULP — for example `x_i` and
   `x_i * (1 + 2^-40)`. Compare `SUMX2MY2(x, y)` against `SUMPRODUCT(x-y, x+y)` computed in the
   same workbook. If they differ materially, the implementation is naive; if they agree closely,
   it is rewritten. One probe, and it identifies the kernel.
3. **The overflow band.** Values near the square root of the largest double, arranged so the
   true difference is small. A naive implementation errors or saturates; a rewritten one answers.
4. **Length mismatch** in both directions, and shape mismatch with equal counts (a 2x3 against a
   3x2, and a row against a column) — which distinguishes "different number of values" from
   "different dimensions" in the documented wording.
5. **An error value inside one array**, which the documented ignore-list does not cover.
6. **Empty arrays and all-ignored arrays**, to pin the zero-versus-error convention.
7. **Summation order**, using a long array with one large pair and many tiny ones in both
   orders.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| difference of squares | The term `x^2 - y^2`, and the rewrite `(x-y)(x+y)` that computes it accurately |
| Sterbenz's lemma | The result that `x - y` is exact when the two are within a factor of two |
| pairwise exclusion | Dropping both partners when either is ignored, preserving alignment |
| independent compaction | Compacting each array separately, which re-pairs the survivors |
| left fold | Adding terms in order into one accumulator; the module's declared reduction policy |

## Sources

- Microsoft, "SUMX2MY2 function" —
  <https://support.microsoft.com/en-us/office/sumx2my2-function-9e599cc5-5399-48e9-a5e0-e37812dfa3e9>
  (signature, the defining equation, "If an array or reference argument contains text, logical
  values, or empty cells, those values are ignored. However, cells with the value zero are
  included", and the `#N/A` length-mismatch condition).
- Microsoft, "SUMPRODUCT function" —
  <https://support.microsoft.com/en-us/office/sumproduct-function-16753e75-9f68-4874-94ac-4d2145a2fd2e>
  (cited here for the contrasting `#VALUE!` dimension-mismatch code recorded as a divergence).
- Higham, *Accuracy and Stability of Numerical Algorithms* — cancellation, Sterbenz's lemma, and
  summation error bounds.
- Kahan, on accurate difference-of-products evaluation — the general form of the rewrite used
  here.
- Handbook projections `data/functions/FUNC.SUMX2MY2.json` (arity exactly 2,
  `error_collapse_profile: ReductionFold`, `numerical_reduction_policy=SequentialLeftFold`,
  `arg_preparation_profile: RefsVisibleInAdapter`) and `data/presence/FUNC.SUMX2MY2.json`
  (shared `sumproduct_family` module).
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md).
