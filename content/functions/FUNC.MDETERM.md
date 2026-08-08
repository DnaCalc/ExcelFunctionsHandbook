---
schema: efh.function-page/v1
function_id: FUNC.MDETERM
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MISC-0005
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
  The scalar-valued member: it shares the family's LU forward elimination with MINVERSE but
  publishes a single number from it, and it is the member with no numeric evidence against
  Excel of any kind.
---

## What it computes

`MDETERM(array)` returns the **determinant** of a square matrix.

The determinant is the unique function of a square matrix that is multilinear in the rows,
alternating (it changes sign when two rows are swapped, and vanishes when two rows are equal), and
equal to 1 on the identity. That characterisation determines it completely, and it yields the
Leibniz formula

    det A = Σ_{σ ∈ Sₙ} sgn(σ) · Π_{i=1..n} a_{i,σ(i)}

a sum over all n! permutations. Microsoft's page gives the 3×3 instance of the same thing by
cofactor expansion:

> `MDETERM(A1:C3) equals A1*(B2*C3-B3*C2) + A2*(B3*C1-B1*C3) + A3*(B1*C2-B2*C1)`

**What it means.** det A is the signed volume scaling factor of the linear map *A*: it multiplies
oriented n-volumes, and its sign records whether orientation is preserved. Hence the two facts
every reader needs:

- **det A = 0 exactly when *A* is singular** — the map collapses space, the columns are linearly
  dependent, no inverse exists.
- **det(AB) = det A · det B**, and det(Aᵀ) = det A, and det(cA) = cⁿ · det A for an n×n matrix.

Further properties worth having: det of a triangular matrix is the product of its diagonal; det of
a permutation matrix is its sign; det(A⁻¹) = 1/det A. The last connects this page directly to
[MINVERSE](FUNC.MINVERSE.md), and the family's shared implementation exploits the same connection
from the other side — the LU factorisation that inverts a matrix also hands you its determinant as
the product of the pivots.

**Domain and range.** Domain: square numeric matrices. Range: all reals (any real is the
determinant of some matrix). The function is a polynomial in the entries, hence continuous and
infinitely differentiable — there are no poles and no branch cuts. Every difficulty on this page is
numerical, not analytic.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `array` | "A numeric array with an equal number of rows and columns." | Yes |

Exactly one argument; the declared arity is one to one. The array may be given as a cell range, an
array constant, or a name.

**A projection gap worth naming.** The Handbook's mechanical projection of this entry carries no
signature — the field is marked as a placeholder rather than filled — so the generator has nothing
to render there. The signature above comes from Microsoft's page, not from the projection.

The argument preparation profile declared for this function makes references visible to the
adapter rather than resolving them to values first, which is the shape one expects of a function
that consumes a whole grid rather than a scalar.

## Result and edge cases

Returns `Number`.

The reference engine's published battery is rendered beside this page. Qualitatively:

- **A 1×1 matrix** — a bare scalar — returns that scalar. The determinant of a 1×1 matrix is its
  single entry, so this is right, and it is also the degenerate case where the matrix machinery
  disappears entirely.
- **A logical argument** produces `#VALUE!`, and **numeric text** produces `#VALUE!`. This is the
  family's distinguishing coercion behaviour: unlike the scalar math functions in this category,
  which convert logicals and numeric text happily, the matrix family **refuses** them. Microsoft's
  page documents the refusal for text ("Any cells in array are empty or contain text") but says
  nothing about logicals.
- **An inline 2×2 array** returns the determinant, and that specific value is one of the four
  pinned by the structural COM probe recorded in this page's second evidence record.
- **An empty range** produces `#VALUE!`, consistent with the documented empty-cell rule.
- **The largest finite double** as a 1×1 matrix is returned unchanged.

The battery contains **no non-square case**, **no singular case beyond the trivial**, and **no
matrix above 2×2**, so it does not exercise the elimination at all.

## Errors

| Error | Documented condition |
|---|---|
| `#VALUE!` | "Any cells in array are empty or contain text." |
| `#VALUE!` | "Array does not have an equal number of rows and columns." |

Error values in the array propagate under the shared discipline
([coercion and lifting](../model/02-coercion-and-lifting.md)); the family declares a reduction
error-collapse profile, meaning competing errors inside the grid are folded rather than each
surfacing independently.

Microsoft's page documents **no `#NUM!`** for `MDETERM`. That is worth contrasting with
[MINVERSE](FUNC.MINVERSE.md), which does document `#NUM!` for a singular matrix — a determinant of
zero is a perfectly good *answer*, whereas an inverse of a singular matrix does not exist. The
asymmetry is mathematically correct and is a useful check on any implementation: `MDETERM` should
never refuse a well-formed square numeric matrix.

**A note on the accuracy remark.** Microsoft's page states:

> "MDETERM is calculated with an accuracy of approximately 16 digits, which may lead to a small
> numeric error when the calculation is not complete."

Read carefully, this is not an error bound. "Approximately 16 digits" describes binary64 precision
generally, not this function's accuracy on any particular matrix, and "when the calculation is not
complete" names no determinable condition. The genuine statement — that the relative error in a
computed determinant is governed by the conditioning of the matrix and can be arbitrarily large —
is in "Numerical notes" below, and is considerably less reassuring than the documentation.

## Relationships

- **[MINVERSE](FUNC.MINVERSE.md)** — the family's other elimination consumer. Both are built on
  the same LU forward elimination in the reference engine; `MDETERM` publishes the product of the
  pivots while `MINVERSE` goes on to solve. det(A⁻¹) = 1/det A ties the two mathematically.
  **Sharing an algorithm is not sharing a measurement** — this page's first evidence record exists
  specifically to say so.
- **[MMULT](FUNC.MMULT.md)** — det(AB) = det A · det B gives the cheapest available metamorphic
  test for all three functions at once, needing no oracle table.
- **[MUNIT](FUNC.MUNIT.md)** — det(I) = 1, the normalisation in the determinant's defining
  characterisation.
- **`SUMPRODUCT`** — for a 2×2 or 3×3 matrix the determinant is a small explicit expression, and
  writing it out is often *more* accurate than calling `MDETERM`, because it avoids the pivoting
  and the divisions. Microsoft's own 3×3 formula is the expression to use.
- **`LINEST`**, **`TREND`**, and the regression family — they solve normal equations whose
  singularity `MDETERM` is sometimes (wrongly) used to test. See the warning below.

## Numerical notes

### Never compute the determinant by its definition

The Leibniz sum has n! terms; cofactor expansion is the same cost. For a 10×10 matrix that is over
three million products, for 15×15 it is over a trillion. Every practical implementation instead
factors the matrix — LU with partial pivoting, O(n³) — and takes

    det A = (−1)^(number of row interchanges) · Π pivots

The reference engine's determinant kernel is identified in this page's first evidence record as
exactly that: LU forward elimination with partial pivoting, determinant as the product of the
pivots. That identification is a statement about the reference engine's internals, **not** a
statement about Excel.

### The determinant overflows and underflows long before the matrix is remarkable

Because det scales as cⁿ, a 20×20 matrix with entries of order 100 has a determinant of order
10⁴⁰, and one with entries of order 0.01 has a determinant of order 10⁻⁴⁰. Overflow and underflow
are reached at modest sizes with entirely ordinary data. The standard remedy in numerical libraries
is to return **log|det| and a sign** rather than the determinant itself (LAPACK-based codes and R's
`determinant()` do this), which stays in range for any matrix the factorisation can handle. Excel
publishes no such surface, so `MDETERM` inherits the full dynamic-range problem. For a large matrix
`MDETERM` may return zero, or an overflow condition, for a matrix that is perfectly well
conditioned.

### The determinant is a bad test for singularity

This is the single most important practical fact on the page, and it is not in any documentation.

- A matrix can have a **tiny determinant and be perfectly well conditioned**: 0.01·I on 20×20 has
  a determinant of 10⁻⁴⁰ and a condition number of exactly 1. Nothing is wrong with it.
- A matrix can have a **determinant of 1 and be catastrophically ill conditioned**: any unimodular
  matrix with wildly different singular values.

The determinant is not scale-invariant and does not measure distance to singularity; the condition
number does. The classic statements of this are in Forsythe and Moler, *Computer Solutions of
Linear Algebraic Systems*, and in Higham, *Accuracy and Stability of Numerical Algorithms*, chapter
14, both of which say plainly that `det` should not be used to detect near-singularity. A worksheet
that tests `IF(MDETERM(A)=0, …)` before calling `MINVERSE` is doing something that looks careful
and is not.

### Error growth

The computed determinant from LU with partial pivoting satisfies a backward-error result: it is the
exact determinant of a nearby matrix. The *forward* error — how far the number is from the true
determinant — is bounded by roughly the condition number times the unit roundoff times n, so it
grows with both size and conditioning. For an ill-conditioned matrix the leading digits of
`MDETERM`'s answer can be wrong, and the documentation's "approximately 16 digits" does not warn
about it. Higham chapter 14 gives the analysis; Golub and Van Loan, *Matrix Computations*,
section 3.2, gives the pivoting theory.

## What has not been checked

No Handbook vector suite exists for `MDETERM`; `vectors/` publishes nothing for this function.

Two evidence records list `MDETERM` among their subjects, and the honest reading of the pair is
unusually specific:

**EV-MISC-0005** exists to record an **absence**. Its own status is that `MDETERM` has **no numeric
comparison against Excel and no counted comparison of any kind** — no discrepancy row, no corpus,
no sweep, at any size. It also carries an explicit warning that `MDETERM` **must not inherit
[MINVERSE](FUNC.MINVERSE.md)'s figures**: the two share an elimination step by construction, and
sharing an algorithm is not sharing a measurement. That warning is the reason this page states its
numerical claims about the *reference engine's internals* and never about agreement with Excel.

**EV-STRUCT-0012** is a provisional structural COM probe that pinned a handful of values across the
matrix family, including a 2×2 determinant, with **no numerator and no denominator** — no row count
was extracted and, the record says, none can be. Its reader warning is worth repeating in the
Handbook's own voice: a provisional spot probe with pinned values and no count is close to nothing,
but it is not nothing, so "no record names this function" would be false for `MDETERM` — and the
record is no evidence at all about numeric results.

Putting those together: **a few structural values are on record; nobody has ever compared
`MDETERM`'s numbers to Excel's.**

Inputs I would probe first:

1. **Well-conditioned integer matrices at 2×2, 3×3, 4×4 with exact integer determinants.** These
   have exactly representable answers, so any deviation is unambiguous and needs no tolerance
   argument. This is the corpus that does not exist and should.
2. **det(A)·det(B) against det(AB)** using [MMULT](FUNC.MMULT.md), over random integer matrices.
   A metamorphic test needing no oracle, and it exercises all three surfaces at once.
3. **A Hilbert matrix at 5×5 and 8×8** — the standard ill-conditioned example, with a known exact
   rational determinant. This is where the forward-error growth becomes visible and where two
   implementations will most readily disagree.
4. **A matrix with a large scale factor** — 100·I at 20×20, and 0.01·I at 20×20 — to find where
   `MDETERM` overflows and underflows, and to demonstrate the scale-dependence argument to readers
   concretely.
5. **A singular matrix, and a matrix one ULP away from singular.** Whether `MDETERM` returns exact
   zero for an exactly singular integer matrix is a real question about the pivoting.
6. **Non-square input and an array containing a logical**, against the documented `#VALUE!` rows —
   the logical case being undocumented.
7. **A matrix with a row of zeros**, whose determinant is exactly zero by the alternating property,
   as the cheapest exactness check available.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| multilinear alternating | The characterising properties of the determinant: linear in each row, sign-flipping on swaps |
| Leibniz formula | The n!-term permutation sum; the definition, never the algorithm |
| LU with partial pivoting | The O(n³) factorisation from which the determinant is the pivot product |
| condition number | The scale-invariant measure of distance to singularity; not the determinant |
| forward error | How far a computed value is from the true one, as opposed to backward error |
| pinned value | A single input/output pair recorded by a probe, with no count attached |

## Sources

- Microsoft, "MDETERM function" —
  <https://support.microsoft.com/en-us/office/mdeterm-function-e7bfa857-3834-422b-b871-0ffd03717020>.
  Retrieved for this page: the syntax, the array argument description, the two `#VALUE!` conditions,
  the accuracy remark quoted above, the 3×3 cofactor formula, and the worked examples.
- Handbook evidence record `EV-MISC-0005` — the recorded absence of any numeric `MDETERM`-versus-
  Excel comparison, the identification of the shared LU elimination as an internals fact, and the
  prohibition on inheriting `MINVERSE`'s figures.
- Handbook evidence record `EV-STRUCT-0012` — provisional structural COM probe over the matrix
  family, pinned values, no count, no named build.
- G. E. Forsythe and C. B. Moler, *Computer Solutions of Linear Algebraic Systems* — the classic
  statement that the determinant is not a measure of near-singularity.
- N. J. Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd edition, chapter 14 — error
  analysis of LU factorisation and of computed determinants.
- G. H. Golub and C. F. Van Loan, *Matrix Computations*, 4th edition, section 3.2 — partial
  pivoting and its stability.
- Handbook, [MINVERSE](FUNC.MINVERSE.md), [MMULT](FUNC.MMULT.md), [MUNIT](FUNC.MUNIT.md) — the
  rest of the family; [coercion and lifting](../model/02-coercion-and-lifting.md) — error
  propagation and the reduction fold.
- `data/functions/FUNC.MDETERM.json` and `data/presence/FUNC.MDETERM.json` — identity, arity 1–1,
  the reduction error-collapse profile, the empty signature placeholder, and the shared
  `matrix_family` module with its three sibling surfaces, as projected at OxFunc `473efa3`.
