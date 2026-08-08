---
schema: efh.function-page/v1
function_id: FUNC.MINVERSE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MISC-0004
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
  The array-valued solve: it drives the family's LU factorisation all the way to an inverse,
  and it is the only member of the four carrying a substantial, still-open numeric
  investigation against live Excel.
---

## What it computes

`MINVERSE(array)` returns the **matrix inverse** of a square matrix: the unique matrix A⁻¹ with

    A · A⁻¹ = A⁻¹ · A = I

**Existence.** A⁻¹ exists exactly when *A* is nonsingular, equivalently when det A ≠ 0,
equivalently when the columns are linearly independent, equivalently when the only solution of
Ax = 0 is x = 0. Microsoft's page states the criterion in the determinant form: "Some square
matrices cannot be inverted and will return the #NUM! error value with MINVERSE. The determinant
for a noninvertable matrix is 0."

**The closed form** — and the reason it must not be used as an algorithm — is Cramer's rule:

    A⁻¹ = adj(A) / det(A)

where adj(A) is the transpose of the cofactor matrix. For a 2×2 matrix this is the familiar

    [a b; c d]⁻¹ = 1/(ad − bc) · [d −b; −c a]

**Properties** worth carrying: (AB)⁻¹ = B⁻¹A⁻¹ (note the order reversal); (Aᵀ)⁻¹ = (A⁻¹)ᵀ;
det(A⁻¹) = 1/det(A); and the inverse of a diagonal matrix is the elementwise reciprocal. The map
A ↦ A⁻¹ is smooth on the open set of nonsingular matrices, with derivative −A⁻¹ (dA) A⁻¹ — which
is the analytic statement of the fact that inversion is badly behaved near singularity: the
derivative blows up as A⁻¹ does.

**Domain and range.** Domain: nonsingular square numeric matrices. Range: nonsingular square
matrices. There is no pole in the analytic sense, but the set of singular matrices is a
measure-zero hypersurface (the zero set of the determinant polynomial) across which the function
is unbounded — the closest thing to a singularity a matrix function has, and the origin of every
difficulty below.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `array` | "A numeric array with an equal number of rows and columns." | Yes |

Exactly one argument; the declared arity is one to one. Microsoft's page notes the array may be
given as a cell range such as `A1:C3`, an array constant such as `{1,2,3;4,5,6;7,8,9}`, or a name.

**A projection gap worth naming.** The Handbook's mechanical projection carries no signature for
this entry — the field is a placeholder — so the generator has nothing to render. The signature
above comes from Microsoft's page.

**Result shape.** `MINVERSE` returns an array the same size as its input. Microsoft's page states
the entry mechanics explicitly: in current Microsoft 365 the formula is entered in the top-left
cell and confirmed with Enter, spilling; in earlier versions the output range must be selected
first and the formula confirmed with Ctrl+Shift+Enter as a legacy array formula. That difference
is a property of the host, not of the function.

## Result and edge cases

Returns `Array` of `Number`.

The reference engine's published battery is rendered beside this page. Qualitatively:

- **A 1×1 zero matrix** produces `#NUM!` — singular, correctly.
- **A 1×1 matrix holding −1** returns a 1×1 array holding −1, its own reciprocal.
- **A logical argument** and **numeric text** both produce `#VALUE!`. As with
  [MDETERM](FUNC.MDETERM.md), the matrix family refuses conversions that the scalar math functions
  accept. The text refusal is documented; the logical refusal is not.
- **The largest finite double** as a 1×1 matrix returns its reciprocal, a subnormal-range value.
- **The smallest positive subnormal** as a 1×1 matrix produces **`#NUM!`** — not a reciprocal.
  Mathematically the matrix is nonsingular; its inverse simply is not representable, since the
  reciprocal overflows. The engine reports it as the singular case. **The documented criterion is
  "the determinant is 0", and this matrix's determinant is not zero**, so the observable behaviour
  and the documented rule part company here. Whatever the right design is, the honest statement is
  that `#NUM!` from `MINVERSE` covers at least two distinct conditions — genuinely singular, and
  representably impossible — and only one of them is documented.
- **An inline 2×2 integer array** returns an inverse whose entries **are not the exactly
  representable values the true inverse has**. The exact inverse of that particular matrix has
  entries every one of which is a binary64 value; the returned entries differ from them in the last
  bits. This is not a defect report against anyone — it is the expected signature of a
  factorisation-based solve, and it is exactly why this function has a live numeric investigation
  attached. See "Numerical notes".

## Errors

| Error | Documented condition |
|---|---|
| `#VALUE!` | "If any cells in array are empty or contain text, MINVERSE returns a #VALUE! error" |
| `#VALUE!` | "MINVERSE also returns a #VALUE! error if array does not have an equal number of rows and columns" |
| `#NUM!` | The matrix cannot be inverted; "The determinant for a noninvertable matrix is 0" |

Error values inside the array propagate; the family declares a reduction error-collapse profile,
so competing errors are folded rather than surfacing independently
([coercion and lifting](../model/02-coercion-and-lifting.md)).

**On the documented `#NUM!` criterion.** "The determinant is 0" is the mathematical criterion and
is not a computational test — see the singularity discussion on
[MDETERM](FUNC.MDETERM.md). No implementation can determine exact singularity from rounded inputs;
what it can do is detect a pivot that has become negligible relative to the matrix's scale. So the
documented rule describes the *intent*, and the observed threshold behaviour is a different, and
undocumented, thing. The subnormal case above shows the gap is real.

**On the accuracy remark.** Microsoft's page states:

> "MINVERSE is calculated with an accuracy of approximately 16 digits, which may lead to a small
> numeric error when the cancellation is not complete."

As on [MDETERM](FUNC.MDETERM.md), this is not an error bound: "approximately 16 digits" describes
binary64 generally, and "when the cancellation is not complete" names no determinable condition.
The real bound is conditioning-dependent and is stated below.

## Relationships

- **[MMULT](FUNC.MMULT.md)** — the verifier. `MMULT(A, MINVERSE(A))` against
  [MUNIT](FUNC.MUNIT.md) is the residual test, and it is the single most useful thing a reader can
  do with these four functions together.
- **[MDETERM](FUNC.MDETERM.md)** — shares the family's LU forward elimination in the reference
  engine, and is related mathematically by det(A⁻¹) = 1/det A. **Sharing an elimination is not
  sharing a measurement**: `MDETERM`'s own evidence record forbids it from borrowing this page's
  figures, and this page does not lend them.
- **[MUNIT](FUNC.MUNIT.md)** — the identity the inverse is defined against.
- **`LINEST` / `TREND` / `LOGEST`** — regression surfaces that solve linear systems internally.
  They are the reason most worksheets *think* they need `MINVERSE`, and they are usually the better
  tool, because they solve rather than invert.
- **`SUMPRODUCT` with an explicit 2×2 formula** — for the smallest cases, writing out
  `1/(ad-bc)·[d,-b;-c,a]` is a legitimate alternative and is often more accurate than a general
  factorisation, because it performs fewer operations.

**The most important relationship is a negative one: do not invert to solve.** To compute A⁻¹b,
solve Ax = b. Forming the inverse and multiplying is slower, less accurate, and needs a
nonsingular matrix where a solve needs only a consistent system. Higham devotes a section to this
(*Accuracy and Stability of Numerical Algorithms*, chapter 14, on the "fallacy" of matrix
inversion); Excel's worksheet surface unfortunately makes the wrong route the easy one, since
`MINVERSE` exists and a general solve does not.

## Numerical notes

### The algorithm space, and what has been ruled out of it

Inverting a matrix admits many algorithms that agree in exact arithmetic and disagree in the last
bits. This function's evidence record contains an unusually complete **ruled-out ledger** — the
list of algorithms tested against live-Excel results and eliminated. Stated qualitatively, and
without borrowing the record's figures:

- **Adjugate / cofactor (Cramer's rule)** — ruled out at both sizes tested. Cramer's rule is the
  textbook formula and is numerically the worst of the candidates; it is also O(n!) if implemented
  literally.
- **Gauss–Jordan elimination on the augmented `[A|I]`** — ruled out, and notable because it was the
  *shipping* kernel in the reference engine until the record's own investigation replaced it.
- **An x87 80-bit extended-precision body with double stores** — ruled out, and strictly worse than
  plain binary64. Note the contrast with [LN](FUNC.LN.md) and [LOG](FUNC.LOG.md), where the x87
  substrate is exactly what *was* identified. Different Excel surfaces sit on different substrates,
  and that is a finding in itself.
- **A systematic sweep of solve orderings** — forward against back substitution, streamed against
  sum-then-subtract accumulation, ascending against descending index order, division against
  reciprocal-multiply — all eliminated. This is the shape of the last-bit problem: the *op graph*,
  not the algorithm name, is what determines the final bits.
- **One candidate that fit its corpus perfectly and then failed on fresh data** — a
  reciprocal-multiply variant of back substitution — is recorded explicitly as an **overfit**. It
  is the most instructive entry in the ledger, and the reason the record's held-out corpora matter.

What survived is **Doolittle LU with partial pivoting**: forward substitution through the unit
lower factor (which needs no division), then division-form back substitution through the upper
factor, solved column by column against permuted unit vectors, in plain binary64 with no
fused-multiply-add contraction. That is a description of the reference engine's kernel as the
record identifies it. It is **not** a claim about what Excel does internally.

### Why the last bits are hard at all

An inverse is n separate linear solves. Each solve accumulates rounding through O(n²) operations,
and the *order* in which the accumulations happen changes the last bits — floating-point addition
is not associative. Two implementations of "the same" LU inverse can therefore differ in every
entry while both being backward-stable and both being right. Reproducing another implementation's
exact bits requires reproducing its op graph, not merely its algorithm, which is why the ruled-out
ledger above is a list of *orderings* as much as a list of *methods*.

### Conditioning governs everything

The relative error in a computed inverse is bounded by roughly κ(A)·u, with κ the condition number
and u the unit roundoff. For an ill-conditioned matrix the computed inverse can have **no correct
digits at all** while still satisfying a small backward error. Two consequences a reader should
carry away:

1. **A small residual `MMULT(A, MINVERSE(A)) − I` does not certify the inverse.** Backward
   stability gives you a small residual almost regardless of conditioning; it does not give you a
   small forward error.
2. **Near-singular is a continuum, not a boundary.** The `#NUM!` from `MINVERSE` fires at some
   threshold; just inside it, the function returns numbers that are formally defined and
   practically meaningless. The dangerous inputs are the ones that do *not* error.

Golub and Van Loan, *Matrix Computations*, section 3.5, and Higham chapter 14, are the standard
treatments; the residual-versus-error distinction is Higham's chapter 7.

## What has not been checked

No Handbook vector suite exists for `MINVERSE`; `vectors/` publishes nothing for this function.

Two evidence records list `MINVERSE` among their subjects, and this is the best-evidenced surface
in this batch.

**EV-MISC-0004** records a substantial numeric investigation against cached live-Excel results at a
named build. Its figures, corpora and held-out split are rendered from the record and are not
restated here. What governs how much you may conclude:

1. **The stream is still open**, with its closure checklist entirely unchecked. This is not a
   settled result.
2. **A residual set of cells on ill-conditioned small matrices remains unexplained**, and the
   record notes that the direction of the residual **flips** between matrix families — which is why
   the source treats it as a genuinely different op graph rather than a rounding-count difference.
   That is a strong, specific, and unresolved finding.
3. **Most of the corpus is held out** and the record says so in the source's own words — a rare and
   genuinely valuable property in this evidence base, and the reason the overfit above was caught
   at all.
4. **The corpora are not in version control.** The counts are quoted from committed documents; the
   witness files themselves are local-only.
5. **The record carries a retraction**: an earlier conclusion, drawn from an emulation scoring
   perfectly on held-out data, was withdrawn when the shipping kernel turned out not to be the
   algorithm the conclusion assumed. The lesson is on the page because it is exactly the mistake a
   reader of an evidence base is most likely to repeat — a model matching Excel does not establish
   that the shipped code is that model.

**EV-STRUCT-0012** is a provisional structural COM probe over the family with pinned values and no
count at all; its reader warning says a provisional spot probe is close to nothing but not nothing,
and that it is no evidence about numeric results.

So: `MINVERSE` has been measured against Excel, substantially and with held-out data, and it still
does not match everywhere; the disagreement is characterised but unexplained. Nothing here is a
Handbook vector suite and none of it was re-verified by the Handbook.

Inputs I would probe first:

1. **The remaining ill-conditioned small matrices**, with the residual direction recorded per
   entry. The flip in residual direction is the live lead, and reproducing it is the shortest path
   to identifying the op graph.
2. **A 1×1 subnormal matrix and a 1×1 matrix whose reciprocal overflows.** The reference engine
   reports `#NUM!` where the documented criterion (determinant zero) does not apply. Whether Excel
   agrees would settle whether `#NUM!` genuinely covers two conditions.
3. **`MMULT(A, MINVERSE(A))` against `MUNIT(n)`**, entry by entry, over a graded family from
   well-conditioned to nearly singular. This maps the usable domain in a way no single corpus does,
   and it needs no oracle.
4. **Integer matrices with exactly representable inverses** — the 2×2 case in the battery is one,
   and a family of them can be constructed at every size. Any deviation from the exact answer is
   then unambiguous, and the *pattern* of deviations discriminates op graphs.
5. **A matrix that is exactly singular by construction** (a repeated row) against one perturbed by
   a single ULP, to locate the `#NUM!` threshold and to test whether it is scale-invariant.
6. **The same corpus at 5×5 and above.** The existing investigation is at small sizes; whether the
   identified kernel continues to match as n grows is unmeasured, and pivot-order effects grow with
   n.
7. **Non-square input and an array containing a logical**, against the documented and undocumented
   `#VALUE!` rows.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| nonsingular | Invertible; equivalently, determinant nonzero, columns independent |
| Cramer's rule | The adjugate-over-determinant closed form; a definition, not an algorithm |
| Doolittle LU | LU factorisation with a unit lower triangular factor |
| partial pivoting | Row interchange to put the largest available element on the pivot |
| op graph | The exact sequence and grouping of floating-point operations; what fixes the last bits |
| backward stable | The computed answer is exact for a slightly perturbed input |
| condition number κ(A) | The amplification factor from input perturbation to output error |
| overfit | A candidate that matched its fitting corpus and failed on fresh data |
| held out | Data not used to choose the model, and therefore able to disconfirm it |

## Sources

- Microsoft, "MINVERSE function" —
  <https://support.microsoft.com/en-us/office/minverse-function-11f55086-adde-4c9f-8eb9-59da2d72efc6>.
  Retrieved for this page: the syntax, the array argument description and its permitted forms, the
  two `#VALUE!` conditions, the `#NUM!` singular condition with its determinant criterion, the
  accuracy remark quoted above, and the dynamic-array versus legacy-array entry mechanics.
- Handbook evidence record `EV-MISC-0004` — the open numeric investigation: the identified Doolittle
  LU substrate, the ruled-out ledger including the recorded overfit, the held-out corpora, the
  unexplained residual cells with flipping direction, the local-only evidence locality, and the
  retraction described above.
- Handbook evidence record `EV-STRUCT-0012` — provisional structural COM probe over the matrix
  family; pinned values, no count, no named build.
- N. J. Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd edition, chapters 7 and 14 —
  residual versus forward error, LU error analysis, and the case against forming inverses.
- G. H. Golub and C. F. Van Loan, *Matrix Computations*, 4th edition, sections 3.2 and 3.5 —
  pivoting, conditioning, and solving in preference to inverting.
- Handbook, [MDETERM](FUNC.MDETERM.md), [MMULT](FUNC.MMULT.md), [MUNIT](FUNC.MUNIT.md) — the rest
  of the family; [coercion and lifting](../model/02-coercion-and-lifting.md) — error propagation
  and the reduction fold.
- `data/functions/FUNC.MINVERSE.json` and `data/presence/FUNC.MINVERSE.json` — identity, arity 1–1,
  the reduction error-collapse profile, the empty signature placeholder, the shared `matrix_family`
  module, and the `BUG-FUNC-023`, `BUG-FUNC-025` and `BUG-FUNC-026` defect streams, as projected at
  OxFunc `473efa3`.
