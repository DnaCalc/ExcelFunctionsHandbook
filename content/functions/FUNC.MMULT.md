---
schema: efh.function-page/v1
function_id: FUNC.MMULT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-STRUCT-0012
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
family: matrix_family
role_in_family: >-
  The family's binary operation and its only non-elimination member: a sum of products with no
  pivoting, no division and no singularity — and therefore the one whose last bits are decided
  entirely by accumulation order.
---

## What it computes

`MMULT(array1, array2)` returns the **matrix product** of two matrices.

    (AB)ᵢⱼ = Σₖ₌₁ⁿ aᵢₖ · bₖⱼ

Each entry of the result is a **dot product**: one row of the left matrix against one column of
the right. Microsoft's page states the shape rule that makes this well defined:

> "The number of columns in array1 must be the same as the number of rows in array2, and both
> arrays must contain only numbers."

So an m×n matrix times an n×p matrix gives an m×p result, and the shared dimension n — the length
of every dot product — vanishes from the answer's shape.

**Algebraic properties**, all of which readers use and some of which surprise them:

- **Associative**: (AB)C = A(BC). *Not commutative*: AB ≠ BA in general, and the two products may
  not even have the same shape.
- **Distributive** over addition, and compatible with scalar multiplication.
- **[MUNIT](FUNC.MUNIT.md) is the identity**: AI = IA = A at the matching size.
- **det(AB) = det A · det B** — the bridge to [MDETERM](FUNC.MDETERM.md), and the cheapest
  metamorphic test available for this family.
- **(AB)ᵀ = BᵀAᵀ** and **(AB)⁻¹ = B⁻¹A⁻¹** — both reverse the order.
- Matrix multiplication is exactly function composition for the corresponding linear maps, which
  is why the order reverses and why it fails to commute.

**Domain and range.** Domain: any conformable pair of numeric matrices. Range: numeric matrices.
The product is a polynomial in the entries — continuous, smooth, no poles, no branch cuts. As with
[MDETERM](FUNC.MDETERM.md), every difficulty here is numerical rather than analytic; unlike
[MINVERSE](FUNC.MINVERSE.md), there is no singularity to approach.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `array1` | The left factor. | Yes |
| `array2` | The right factor. | Yes |

Both required; the declared arity is exactly two. Microsoft's page notes that both "can be given
as cell ranges, array constants, or references".

**A projection gap worth naming.** The Handbook's mechanical projection carries no signature for
this entry — the field is a placeholder — so the generator has nothing to render there. The
signature above comes from Microsoft's page.

**Result shape and entry.** `MMULT` returns an array with the row count of `array1` and the column
count of `array2`. Microsoft's page gives the entry mechanics: in current Microsoft 365 the formula
spills from a single cell, and in earlier versions the output range must be selected first and the
formula confirmed with Ctrl+Shift+Enter.

## Result and edge cases

Returns `Array` of `Number`.

The reference engine's published battery is rendered beside this page. Qualitatively:

- **Two scalars** produce a **1×1 array**, not a scalar. This is not a rounding detail; it is a
  documented structural fact in the reference engine's own record. This page's evidence record
  notes that the scalar appearance of `MMULT(5,2)` in a worksheet cell belongs to the **worksheet
  publication seam** — the value the *function* produces stays a 1×1 array, and nested type
  inspection shows it. The collapse to a scalar happens when the result is published into a cell,
  not inside the function. A formula that nests `MMULT` inside something shape-sensitive is dealing
  with an array.
- **A logical argument** and **numeric text** both produce `#VALUE!`, consistent with the
  documented "must contain only numbers" rule for text and undocumented for logicals — the same
  family-wide refusal noted on [MDETERM](FUNC.MDETERM.md).
- **An empty range** produces `#VALUE!`, matching the documented empty-cell rule.
- **Two inline 2×2 arrays** produce the expected 2×2 product.
- **The largest finite double against itself** produces an array whose element is **infinite**, not
  `#NUM!`. The declared axis for this surface permits non-finite results, so this is deliberate in
  the reference engine. Excel's usual habit for arithmetic overflow is `#NUM!`, and the Handbook
  has **not** observed which Excel produces here. It is a probe below, and it is a real candidate
  divergence rather than a curiosity — an overflowing dot product is reachable from ordinary
  financial data at modest matrix sizes.

## Errors

| Error | Documented condition |
|---|---|
| `#VALUE!` | "Any cells are empty or contain text" |
| `#VALUE!` | "The number of columns in array1 is different from the number of rows in array2" |

Error values inside either array propagate; the family declares a reduction error-collapse profile,
so competing errors are folded rather than surfacing independently
([coercion and lifting](../model/02-coercion-and-lifting.md)).

Microsoft's page documents **no `#NUM!`** for `MMULT`, and no overflow condition at all. Given that
a dot product of large entries overflows readily, that is a gap rather than a guarantee.

## Relationships

- **[MINVERSE](FUNC.MINVERSE.md)** — `MMULT(A, MINVERSE(A))` against [MUNIT](FUNC.MUNIT.md) is the
  residual test, and the most useful single thing to do with this family. Read the caution on the
  `MINVERSE` page first: a small residual is *not* a certificate of an accurate inverse.
- **[MDETERM](FUNC.MDETERM.md)** — det(AB) = det A · det B is a metamorphic identity requiring no
  oracle and exercising two surfaces at once.
- **[MUNIT](FUNC.MUNIT.md)** — the multiplicative identity; `MMULT(A, MUNIT(n))` should return *A*
  unchanged, entry for entry, which is a strong exactness test because every dot product is a sum
  of exact zeros and one exact copy.
- **`SUMPRODUCT`** — the single-dot-product function. `MMULT` of a row by a column is
  `SUMPRODUCT`, and the two are worth comparing: if they accumulate in different orders they will
  differ in the last bits for the same data, which is a clean, oracle-free way to see accumulation
  order made visible.
- **`TRANSPOSE`** — the other shape operator, and the partner in (AB)ᵀ = BᵀAᵀ.
- **`LINEST` and the regression family** — they form and solve normal equations, which are
  `MMULT(TRANSPOSE(X), X)` in disguise. That product is famously the step that squares the
  condition number, which is why forming normal equations explicitly is discouraged.

## Numerical notes

`MMULT` has no pivoting, no division, and no singularity. Every last bit it produces is decided by
one thing: **the order in which the dot products accumulate.**

### The error bound

For a single dot product of length n computed by sequential accumulation, the standard bound
(Higham, *Accuracy and Stability of Numerical Algorithms*, chapter 3) is

    |fl(xᵀy) − xᵀy| ≤ γₙ · |x|ᵀ|y|,      γₙ = n·u / (1 − n·u)

with u the unit roundoff. Two things follow immediately, and they are the whole practical story:

1. **The bound grows with n**, the shared dimension — not with the size of the result. A product of
   two 2×500 and 500×2 matrices has four entries and five hundred terms each; it is a *harder*
   computation than a 20×20 by 20×20 product with four hundred entries.
2. **The bound is in terms of |x|ᵀ|y|, not |xᵀy|.** When the terms of the dot product have mixed
   signs and largely cancel, the relative error in the answer is unbounded. This is the matrix
   version of catastrophic cancellation, and it is why differencing data before multiplying is
   dangerous.

### Accumulation order changes the answer, and every implementation chooses one

Floating-point addition is not associative, so:

- Sequential left-to-right, pairwise, and blocked accumulation give different bits.
- A **fused multiply-add** contracts `a*b + c` into one rounding instead of two, changing the
  result — usually improving it, always changing it.
- Cache-blocked and vectorised kernels reorder the sum by construction; a BLAS `dgemm` and a naive
  triple loop will not agree bit for bit.
- Strassen-style fast algorithms change the arithmetic entirely and have a **different, weaker**
  error bound — they are componentwise less stable, which is why they are rare in general-purpose
  libraries.

For a compatibility implementation the consequence is the same one recorded on
[MINVERSE](FUNC.MINVERSE.md): matching another implementation's bits means matching its **op
graph**, not merely its algorithm. `MMULT` is the cleanest place in the family to see that,
because there is nothing else going on.

### If you need accuracy rather than compatibility

The remedies are well established and none of them is reachable from the worksheet:

- **Compensated (Kahan/Neumaier) summation**, or the Dot2 algorithm of Ogita, Rump and Oishi
  ("Accurate sum and dot product", *SIAM Journal on Scientific Computing* 26(6), 2005), which
  delivers a result as accurate as one computed in twice the working precision, at a small constant
  multiple of the cost.
- **Extended-precision accumulation** of the dot product before rounding once at the end.
- **Sorting or grouping terms by magnitude** before summing, which helps in the mixed-sign case.

A `natural-best` implementation of `MMULT` should use a compensated dot product; an
`excel-bitexact` one must not, unless Excel does.
See [implementation options](../model/07-implementation-options.md) for why those are different
deliverables.

### Overflow

A dot product overflows when the *partial sums* overflow, which can happen even when the true
result is representable — a sum of large positive terms followed by large negative ones is the
standard example. Scaling the matrices before multiplying and unscaling afterwards is the classical
remedy, and it is exactly what LAPACK-style codes do. The reference engine's declared policy is to
allow non-finite results through; what Excel does is unobserved.

## What has not been checked

No Handbook vector suite exists for `MMULT`; `vectors/` publishes nothing for this function.

One evidence record lists `MMULT` among its subjects: **EV-STRUCT-0012**, a **provisional**
structural COM probe over the matrix family. It pinned a handful of values — including a scalar
product and a shape-mismatch error — with **no numerator and no denominator**; no row count was
extracted and the record states none can be. Its reader warning is the operative instruction:
a provisional spot probe with pinned values and no count is close to nothing, but it is not
nothing, so "no record names this function" would be false for `MMULT` — and the record is **no
evidence whatever about numeric results**. No Excel build, architecture or CPU is named for it, and
the probe files are not in version control.

So: a few structural values are on record, and **nobody has ever compared `MMULT`'s numbers to
Excel's**. For a function whose entire difficulty is accumulation order, that is the measurement
that would matter most and it does not exist.

Inputs I would probe first:

1. **`MMULT(A, MUNIT(n))` against `A`, entry for entry.** Every dot product here is exact by
   construction, so any deviation is a bug rather than a rounding difference. It is the cheapest
   possible exactness check and it should be the first cell anybody runs.
2. **A long shared dimension with mixed signs** — a 1×n row against an n×1 column whose true dot
   product is small and whose terms are large. This is where accumulation order becomes visible in
   the leading digits, not the last bits, and where two implementations will disagree most.
3. **The same dot product computed by `MMULT` and by `SUMPRODUCT`.** If Excel's two surfaces
   accumulate differently they will differ, and the pattern identifies the orders. No oracle table
   is needed.
4. **Overflow**: two matrices of large entries whose product overflows, and a case whose partial
   sums overflow while the true result does not. This settles the `#NUM!`-versus-infinity question
   that the reference engine's declared policy and Excel's usual habit answer differently.
5. **det(AB) against det(A)·det(B)** over random integer matrices, exercising `MMULT` and
   [MDETERM](FUNC.MDETERM.md) together.
6. **Integer matrices whose product is exactly representable** at several sizes. Exact answers make
   deviations unambiguous, and integer data is where a compatibility corpus should start.
7. **A nested `MMULT(MMULT(5,2), 3)`**, to confirm the 1×1-array result shape survives nesting as
   the evidence record's publication-seam note implies.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| conformable | Shapes that allow the product: columns of the left equal rows of the right |
| shared dimension | The length n of every dot product; it sets the error bound and vanishes from the result |
| dot product | The sum of elementwise products forming one entry of the result |
| unit roundoff u | Half the spacing of binary64 at 1; the basic error unit |
| accumulation order | The sequence and grouping of the additions; what fixes the last bits |
| op graph | The exact arrangement of floating-point operations |
| fused multiply-add | `a*b+c` with a single rounding rather than two |
| publication seam | The worksheet boundary where a 1×1 array is displayed as a scalar |

## Sources

- Microsoft, "MMULT function" —
  <https://support.microsoft.com/en-us/office/mmult-function-40593ed7-a3cd-4b6b-b9a3-e4ad3c7245eb>.
  Retrieved for this page: the syntax, both array argument descriptions and their permitted forms,
  the conformability and numbers-only rule, the two `#VALUE!` conditions, the summation form of the
  product, and the array-formula entry mechanics. The page as retrieved documents no overflow
  condition.
- Handbook evidence record `EV-STRUCT-0012` — provisional structural COM probe over the matrix
  family; pinned values, no count, no named build, and the publication-shape note establishing that
  the scalar case returns a 1×1 array from the function.
- N. J. Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd edition, chapter 3 — the
  dot-product error bound quoted above and the analysis of matrix multiplication; chapter 23 for
  fast (Strassen-type) algorithms and their weaker bounds.
- T. Ogita, S. M. Rump and S. Oishi, "Accurate sum and dot product", *SIAM Journal on Scientific
  Computing* 26(6), 2005 — the compensated dot product recommended above.
- G. H. Golub and C. F. Van Loan, *Matrix Computations*, 4th edition, chapter 1 — blocked
  formulations and their effect on accumulation order.
- Handbook, [MDETERM](FUNC.MDETERM.md), [MINVERSE](FUNC.MINVERSE.md), [MUNIT](FUNC.MUNIT.md) — the
  rest of the family; [implementation options](../model/07-implementation-options.md) — why the
  accurate and the compatible implementations differ here;
  [coercion and lifting](../model/02-coercion-and-lifting.md) — error propagation and the reduction
  fold.
- `data/functions/FUNC.MMULT.json` and `data/presence/FUNC.MMULT.json` — identity, arity 2–2, the
  reduction error-collapse profile, the non-finite-allowed real-result policy, the empty signature
  placeholder, the shared `matrix_family` module, and the `BUG-FUNC-023`, `BUG-FUNC-025` and
  `BUG-FUNC-026` defect streams, as projected at OxFunc `473efa3`.
