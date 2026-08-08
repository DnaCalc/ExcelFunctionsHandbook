---
schema: efh.function-page/v1
function_id: FUNC.SUMSQ
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — SUMSQ function"
    locator: "https://support.microsoft.com/en-us/office/sumsq-function-e3313c02-51cc-4963-aae6-31442d9ec307"
    role: "documented signature, the 255-argument limit, and the array/reference scan remarks"
  - work: "Higham, Accuracy and Stability of Numerical Algorithms"
    locator: "chapters on summation and on the computation of norms"
    role: "error bounds for summation orders, and the scaled two-pass Euclidean norm"
  - work: "Blue, A Portable Fortran Program to Find the Euclidean Norm of a Vector (TOMS, 1978)"
    locator: null
    role: "the canonical three-accumulator scaled norm that avoids overflow and underflow in the squares"
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
family: sumsq
role_in_family: >-
  The sole member: the sum of squares, and the aggregate whose overflow behaviour differs most
  sharply from the magnitude of its own answer.
---

## What it computes

`SUMSQ(number1, [number2], ...)` returns

    SUM over i of x_i^2

the sum of the squares of its arguments.

- **Domain**: any finite list of real numbers.
- **Range**: `[0, +infinity)`. The result is zero only when every contributing value is zero;
  it is otherwise strictly positive. `SUMSQ` of an empty selection is zero.
- **Structure**: this is the squared Euclidean norm, `||x||^2`. It is the quantity that sits
  under least squares, variance, the chi-square statistic, and every energy or power
  calculation. `SQRT(SUMSQ(...))` is the Euclidean norm itself, and Excel has no separate
  function for it.
- **Identities**: `SUMSQ(x) = SUMPRODUCT(x, x)`; `SUMSQ(x) - n*mean^2 = DEVSQ(x)` is the
  computational (and numerically unwise — see below) form of the sum of squared deviations;
  `SUMX2PY2(x, y) = SUMSQ(x) + SUMSQ(y)`.
- **Degenerate cases**: a single argument gives `x^2`; `SUMSQ(3, 4)` gives 25, the documented
  example, and is the Pythagorean case that explains why the function is useful.

Microsoft's page gives the signature and describes the arguments as numbers, names, arrays or
references containing numbers, up to 255 arguments.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number1` | The first value or range. Required. | — |
| `number2, ...` | Further values or ranges, up to 255 arguments in total. Optional. | — |

The reference engine records an arity of one to 255, matching the documented limit.

The commonly misunderstood aspect is not any single position but the **direct-argument versus
range-scan asymmetry**, which `SUMSQ` shares with the whole aggregate family and which the
reference engine names explicitly on this surface as
`coercion_lift_profile: AggregateDirectAndRangeDualPolicy`. The two policies are:

- A value passed **directly** — `SUMSQ("3", TRUE)` — undergoes ordinary to-number coercion.
- A value reached by **scanning a range** — `SUMSQ(A1:A3)` where a cell holds the text `"3"` —
  is governed by the family's scan policy, which is not the same rule.

Microsoft's article states both sides in its own vocabulary: values "typed into the list" are
included, while "if an argument is an array or reference, only numbers in that array or
reference are counted", and the function "will ignore empty cells, logical values, text, or
error values in the array or reference". The Handbook's shared treatment of this asymmetry is in
[Coercion and lifting](../model/02-coercion-and-lifting.md#direct-arguments-versus-range-scans);
this page does not restate it.

**A divergence worth recording.** Microsoft's remark places *error values* in the ignored set
for arrays and references, while the same article separately says errors arise from error values
in arguments. The Handbook's call model takes the opposite position from the first clause: "An
error in a scanned range does surface from `SUM` — skipping errors is not part of the
ignore-text scan policy." So the documented remark and the Handbook's model of the aggregate
family disagree about what an error value inside a scanned range does. Neither has been checked
against Excel for `SUMSQ` in this record, and the disagreement is published rather than
silently resolved. It is the first probe listed below.

## Result and edge cases

Returns `Number`, always non-negative.

- **No numeric values found**: zero. An aggregate over an empty or all-text range returns zero,
  not an error, which is the family convention.
- **Squares overflow before sums do.** This is the characteristic edge and it is worth stating
  plainly: a value near the square root of the largest double squares to the top of the range,
  and anything above that overflows *in a single term* even though the mathematical answer would
  be perfectly representable if the computation were scaled. Conversely, small values square to
  subnormals and then to zero, so a vector of small entries can produce a sum of squares of zero
  when the true value is a small positive number.
- **Logicals and text** follow the dual policy above.
- **Arrays**: `SUMSQ` is an aggregate, not a lift kernel — an array argument is consumed by
  scanning, not applied elementwise.

The reference engine declares `error_collapse_profile: ReductionFold` with
`numerical_reduction_policy=SequentialLeftFold` and `error_algebra=CanonicalExcelLegacy`. The
left-fold declaration matters numerically and is discussed below. Its projected
`real_result_policy` reads `arg_domain_guard=none` with `non_finite=allow`.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | A **direct** argument is text that cannot be converted to a number | Documented in substance by Microsoft; shared coercion rules |
| propagated | An error value among the arguments surfaces as that error | Shared coercion rules and the reference engine's `ReductionFold` |
| `#NUM!` | Not documented; but see the overflow discussion above and the general non-finite convention | Undetermined here |

Which error wins when several are present is the business of the error-collapse profile; the
reference engine declares a fold with a canonical legacy error algebra for this surface.

## Relationships

- **`SUMPRODUCT(x, x)`** — the same value by a different route, and a useful cross-check.
- **`SUMX2PY2(x, y)`** — `SUMSQ(x) + SUMSQ(y)` in one call, over paired arrays. `SUMX2MY2` and
  `SUMXMY2` complete that trio; all three are documented with a different mismatch error code
  than `SUMPRODUCT`, which is discussed on their pages.
- **`DEVSQ`, `VAR.S`, `VAR.P`, `STDEV.S`** — the statistical consumers. Note the warning below
  about computing them from `SUMSQ`.
- **`SQRT(SUMSQ(...))`** — the Euclidean norm. There is no `HYPOT` or `NORM` function in Excel,
  so this composition is the idiom, and it inherits the overflow problem described below.
- **`SUM`** — the parent aggregate; `SUMSQ` differs only in squaring each term before adding.
- **`POWER` / `^`** — writing `SUM(A1:A10^2)` as an array formula is the hand-rolled equivalent
  and is not obliged to agree in the last bit, because it materializes an intermediate array.
- **Confused with**: `SUMSQ` versus `SUM(...)^2`. The sum of squares and the square of the sum
  are different numbers, and the confusion is common enough to be worth naming.

## Numerical notes

`SUMSQ` combines two well-studied numerical problems: squaring and summing. Each has a standard
treatment and Excel offers no control over either.

**1. Overflow and underflow in the squares.** The dynamic range of the squares is twice that of
the inputs, in the exponent. A vector whose entries are all around `1e200` has a perfectly
representable sum of squares in the mathematical sense — but every individual square overflows.
Symmetrically, entries around `1e-200` square to zero. The standard remedy, from Blue's TOMS
algorithm and the LAPACK `dnrm2` lineage, is **scaling**: find the largest magnitude `m`,
accumulate `SUM (x_i/m)^2`, and multiply back by `m^2` at the end. Blue's refinement uses three
accumulators for small, medium and large entries so that a single pass suffices even when the
data spans the whole range. This is textbook, it costs one extra pass or a running maximum, and
nothing in `SUMSQ`'s interface tells you whether it is done.

**2. Summation order and error growth.** The reference engine declares
`numerical_reduction_policy=SequentialLeftFold` for this surface — the terms are added in order,
left to right. That is the simplest policy and the one with the weakest error bound: the
worst-case error grows linearly with the number of terms, and it grows fastest when a long run
of small terms is added to an already-large accumulator, where each small term is partly or
wholly swallowed. Higham's summation chapter gives the bounds and the alternatives: pairwise
summation (error growing with the logarithm of the count, essentially free), and compensated
summation in the Kahan/Neumaier line (error independent of the count, at the price of a few
extra operations per term).

For `SUMSQ` specifically, the left fold interacts badly with the squaring: squares are all
non-negative, so there is no cancellation between terms, but the *magnitude spread* is doubled,
which is exactly the condition under which a naive running sum swallows terms. A sum of squares
is therefore one of the better cases for compensated summation and one of the worst for a plain
fold.

**3. Do not compute variance from `SUMSQ`.** The textbook shortcut
`variance = (SUMSQ(x) - n * mean^2) / (n - 1)` subtracts two large nearly equal quantities and
can return a negative variance for data with a large mean and small spread. This is the most
famous cancellation example in applied statistics and it is worth repeating on this page because
`SUMSQ` is what makes the shortcut writable. Use `DEVSQ`, or a two-pass or Welford
formulation, and leave `SUMSQ` for the cases where the sum of squares is itself the answer.

**4. What a careful implementation does**: scale by the largest magnitude, accumulate pairwise
or with compensation, and apply the scale back once at the end. The result differs from a naive
left fold, sometimes substantially — which is precisely the four-flavour split this Handbook
publishes rather than hides. The Handbook makes no claim about what Excel does internally.

## What has not been checked

No Handbook evidence record lists `FUNC.SUMSQ` in its subjects, and no Handbook vector suite
exists for `SUMSQ`. **Nobody has checked this function against Excel within the Handbook's
record.** The documented remarks above are Microsoft's; the summation-policy statements come
from the reference engine's own declared axes, not from observation of Excel.

Probes worth running first:

1. **An error value inside a scanned range.** This is the divergence recorded above: Microsoft's
   remark lists error values among the ignored contents of an array or reference, and the
   Handbook's call model says errors surface. One cell, one probe, and a documented contradiction
   is resolved. Highest value on this page.
2. **Direct versus scanned text and logicals**: `SUMSQ("3")` and `SUMSQ(TRUE)` against
   `SUMSQ(A1)` with the same content in the cell. This pins the dual policy for this surface
   rather than inheriting it from `SUM`.
3. **Overflow scaling.** A two-element range near the square root of the largest double, whose
   true sum of squares is representable. If Excel returns a number the implementation scales; if
   it returns an error or an infinity it does not. One probe, decisive.
4. **Underflow.** A range of values around `1e-180`, whose squares are subnormal or zero.
5. **Summation order.** A long range constructed so that a left fold and a pairwise sum differ:
   one large value followed by many tiny ones, arranged in both orders. Comparing the two
   orderings' results detects a plain fold without needing to know the implementation.
6. **Empty and all-text ranges**, to confirm the zero convention.
7. **The 255-argument boundary** — 255 and 256 arguments — against the documented limit.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| dual policy | The family's separate rules for direct arguments and for range-scanned values |
| left fold | Adding terms in order into a single accumulator; the declared reduction policy here |
| pairwise summation | Recursive halving of the summation tree, with logarithmic error growth |
| compensated summation | Kahan/Neumaier-style correction terms that recover the lost low bits |
| scaled norm | Dividing by the largest magnitude before squaring, to avoid overflow and underflow |

## Sources

- Microsoft, "SUMSQ function" —
  <https://support.microsoft.com/en-us/office/sumsq-function-e3313c02-51cc-4963-aae6-31442d9ec307>
  (signature, up to 255 arguments, the "typed into the list" versus array/reference distinction,
  and the ignored-contents remark whose treatment of error values this page records as a
  divergence).
- Higham, *Accuracy and Stability of Numerical Algorithms* — summation error bounds, pairwise and
  compensated summation, and the two-pass norm.
- Blue, "A Portable Fortran Program to Find the Euclidean Norm of a Vector", *ACM TOMS*, 1978;
  the LAPACK `dnrm2` lineage — the scaled accumulation that removes the overflow band.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md), especially the
  direct-argument versus range-scan section and the statement that errors in scanned ranges
  surface.
- Handbook projections `data/functions/FUNC.SUMSQ.json` (arity 1-255,
  `coercion_lift_profile: AggregateDirectAndRangeDualPolicy`,
  `error_collapse_profile: ReductionFold`, `numerical_reduction_policy=SequentialLeftFold`) and
  `data/presence/FUNC.SUMSQ.json`.
