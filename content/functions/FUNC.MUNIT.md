---
schema: efh.function-page/v1
function_id: FUNC.MUNIT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
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
  The generator: it takes no matrix and produces one, it is the only member with no arithmetic
  in it at all, and it is therefore the exact reference object the other three are tested
  against.
---

## What it computes

`MUNIT(dimension)` returns the **identity matrix** of the given size: the square matrix with ones
on the main diagonal and zeros everywhere else.

    Iₙ = (δᵢⱼ),   δᵢⱼ = 1 if i = j, else 0

δ is the Kronecker delta, and `MUNIT` is simply its tabulation over an n×n grid.

The identity is the multiplicative unit of the matrix algebra, and every property a reader needs
follows from that:

- **AI = IA = A** for every conformable *A*. This is the defining property, and it is the reason
  `MUNIT` is useful in a worksheet at all.
- **I⁻¹ = I**, **Iᵀ = I**, **I² = I** — the identity is its own inverse, its own transpose, and
  idempotent.
- **det I = 1** (this is the normalisation in the determinant's defining characterisation — see
  [MDETERM](FUNC.MDETERM.md)), **trace I = n**, and every eigenvalue is 1 with the whole space as
  eigenspace.
- Iₙ is the unique matrix with these properties at each size, so `MUNIT` has exactly one correct
  answer per argument and no implementation freedom whatever.

**Domain and range.** Domain: the positive integers. Range: the identity matrices. This is the
only member of the matrix family that is a *generator* rather than an operator — it consumes a
scalar and produces a grid — which puts it closer in spirit to `SEQUENCE` and `RANDARRAY` than to
its three siblings.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `dimension` | "An integer specifying the dimension of the unit matrix that you want to return." | Yes |

Microsoft's page adds two facts in the same sentence: "It returns an array" and "The dimension has
to be greater than zero."

Exactly one argument; the declared arity is one to one.

**A projection gap worth naming.** The Handbook's mechanical projection carries no signature for
this entry — the field is a placeholder — so the generator has nothing to render there. The
signature above comes from Microsoft's page.

**Result shape and entry.** The result is a *dimension* × *dimension* array. Microsoft's page gives
the mechanics: in current Microsoft 365 the formula spills from a single cell; in earlier versions
the output range must be selected first and the formula confirmed with Ctrl+Shift+Enter.

## Result and edge cases

Returns `Array` of `Number`.

The reference engine's published battery is rendered beside this page, and this function's rows
are the most surprising in the batch.

- **Zero and negative dimensions** produce `#VALUE!`, exactly as documented.
- **A subnormal positive dimension** produces `#VALUE!` — it truncates to zero, and zero is
  rejected. Consistent.
- **The largest finite double** produces `#VALUE!`. A dimension that large cannot be materialised;
  the rejection is sensible and is not documented.
- **A logical argument** is **accepted**, producing a 1×1 identity, and **numeric text** is
  **accepted and truncated**, producing the identity at the truncated size. This is a striking
  contrast with the rest of the family: [MDETERM](FUNC.MDETERM.md),
  [MINVERSE](FUNC.MINVERSE.md) and [MMULT](FUNC.MMULT.md) all refuse logicals and numeric text,
  while `MUNIT` converts them like an ordinary scalar math function. **The four surfaces share one
  implementation module and do not share one coercion policy.** That is a real finding about the
  family's internal consistency, and it is worth checking against Excel because there is no
  documented reason for the split.
- **An array argument** produces the strangest row on the page. `MUNIT` declares a **by-index
  scalar-array lift** on its single argument position, so an array of dimensions is lifted
  elementwise — and since each elementwise result is itself an array, each one is reduced to a
  single value in its cell. The published result for a 2×2 array of dimensions is a 2×2 grid of
  **ones**, which is not an identity matrix of any size. Nothing in Microsoft's documentation
  describes this, no reading of "the identity matrix of dimension n" produces it, and the
  implementing module carries an open upstream defect stream specifically about scalar-parameter
  array lifting (`BUG-FUNC-018`) plus a second about 1×1 scalar publication (`BUG-FUNC-026`).
  Treat this row as a **behaviour of the reference engine under an unsettled lifting policy**, not
  as a statement about Excel — and see the probe list.

## Errors

| Error | Documented condition |
|---|---|
| `#VALUE!` | "If dimension is a value that's equal to or smaller than zero (0), MUNIT returns the #VALUE! error value." |

That is the whole of the documented error surface. Note that it is `#VALUE!` and not `#NUM!`,
which is unusual — a dimension of −1 is a *domain* failure, and `#NUM!` is Excel's conventional
answer for those. `MUNIT` uses the kind-failure error for a domain failure, and the documentation
is explicit about it, so this is one of the rarer cases where the documented behaviour is both
surprising and unambiguous.

Undocumented, and therefore open: what happens for a dimension too large to materialise, for a
non-integer dimension (the reference engine truncates), and for a non-numeric argument.

## Relationships

- **[MMULT](FUNC.MMULT.md)** — the operation `MUNIT` is the unit of. `MMULT(A, MUNIT(n))` should
  return *A* entry for entry, and because every dot product involved is a sum of exact zeros and
  one exact copy, **any deviation is a defect rather than a rounding difference**. This makes
  `MUNIT` the family's calibration standard.
- **[MINVERSE](FUNC.MINVERSE.md)** — `MMULT(A, MINVERSE(A))` against `MUNIT(n)` is the residual
  test. Read the caution on the `MINVERSE` page: a small residual does not certify an accurate
  inverse.
- **[MDETERM](FUNC.MDETERM.md)** — det(MUNIT(n)) = 1 for every n, an exactness check that costs one
  cell.
- **`SEQUENCE`** — the other pure generator in the modern surface. `MUNIT(n)` is expressible as
  `--(SEQUENCE(n)=TRANSPOSE(SEQUENCE(n)))`, which is worth knowing both as a fallback and as an
  independent implementation to compare against.
- **`IDENTITY`** does not exist in Excel; `MUNIT` is the name, and it is easy to miss when
  searching.

## Numerical notes

`MUNIT` has **no numerical content**. Both 0 and 1 are exactly representable in binary64, there is
no arithmetic, and there is exactly one correct answer for each input. It cannot lose precision,
cannot overflow, and admits no algorithmic variation.

That is precisely what makes it valuable, and it is worth saying why explicitly:

**`MUNIT` is the family's exact anchor.** Every other member has an accumulation order, a pivot
choice, or a division that puts its last bits in question. `MUNIT` has none. So in any test of this
family, `MUNIT` is the term that is known to be right, and any discrepancy in a residual
`MMULT(A, MINVERSE(A)) − MUNIT(n)` is attributable entirely to the other two surfaces. A test
design that instead builds its identity by hand, or by `SEQUENCE` comparison, gives up that
certainty for nothing.

The two genuine engineering considerations are about **size**, not accuracy:

1. **The result is n² cells.** A dimension of 1000 asks for a million cells, which is a
   materialisation and spill question rather than a mathematical one. Where the practical ceiling
   sits — and whether it produces `#VALUE!`, `#SPILL!`, or a stall — is unmeasured here, and the
   reference engine's rejection of an enormous dimension suggests a guard exists.
2. **The identity should rarely be materialised at all.** In numerical practice one applies the
   identity implicitly; forming it and multiplying by it costs O(n³) to achieve nothing. That is a
   remark about linear-algebra practice (Golub and Van Loan, *Matrix Computations*, chapter 1)
   rather than about Excel, but a worksheet that computes `MMULT(A, MUNIT(n))` in production is
   doing the same wasteful thing.

## What has not been checked

No Handbook vector suite exists for `MUNIT`; `vectors/` publishes nothing for this function, and
**no evidence record names `MUNIT` among its subjects**.

That absence is worth stating precisely, because it is easy to get wrong. A provisional structural
COM probe over this family *does* exist and *is* attached to `MDETERM`, `MINVERSE` and `MMULT`;
`MUNIT` appears in the upstream batch's title but is **not** one of that record's subjects, and the
Handbook's rule is that a surface may not claim a record that does not list it. So: **the matrix
family was probed, and this surface was not measured separately.** Nobody has checked `MUNIT`
against Excel within the Handbook's record.

The implementing module carries two open upstream defect streams touching this function —
`BUG-FUNC-018` on scalar-parameter array lifting and `BUG-FUNC-026` on 1×1 scalar publication —
which is precisely the pair of issues the strangest battery row above sits between.

Everything marked as documented comes from Microsoft's `MUNIT` page: the syntax, the dimension
description including "greater than zero" and "It returns an array", the `#VALUE!` remark for
non-positive dimensions, the 3×3 example, and the entry mechanics.

Inputs I would probe first:

1. **`MUNIT({2,3})` and `MUNIT({1,2;3,4})`.** The array-lift row described above. Excel's answer
   here is genuinely unknown and there is no reading of the documentation that predicts one —
   plausible candidates are a `#VALUE!`, a spilled identity at the first element's size, and a
   `#SPILL!`. This is the most interesting unanswered question about the function.
2. **`MUNIT(TRUE)` and `MUNIT("3")`.** The coercion split within the family. Two cells, and they
   would establish whether Excel's matrix functions share a coercion policy or, like the reference
   engine's, do not.
3. **`MUNIT(2.7)`.** Whether a non-integer dimension truncates, rounds, or errors. The
   documentation says "an integer" without saying what happens when it is not one.
4. **`MUNIT(0)` and `MUNIT(-1)`** against the documented `#VALUE!` — cheap conformance, and worth
   doing because `#VALUE!` for a domain failure is unusual enough to be worth confirming.
5. **The size ceiling**: `MUNIT(1000)`, `MUNIT(10000)`, and a dimension beyond the grid. Whether
   the failure is `#VALUE!`, `#SPILL!`, `#NUM!` or a refusal to calculate is unrecorded, and the
   reference engine's guard suggests there is something to find.
6. **`MMULT(A, MUNIT(n))` against `A`** — the exactness anchor, which should be perfect and which
   is the first thing to check before trusting any other result in this family.
7. **`MDETERM(MUNIT(n))` for several n** — should be exactly 1, and is a one-cell test of two
   surfaces.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| identity matrix | The square matrix with ones on the diagonal and zeros elsewhere |
| Kronecker delta | δᵢⱼ, the function `MUNIT` tabulates |
| generator | A function that produces an array from a scalar rather than transforming one |
| exact anchor | A value known to be exactly right, against which others can be measured |
| by-index scalar-array lift | The declared policy under which an array in this slot is applied elementwise |
| materialisation | Actually producing the n² cells, as opposed to using the identity implicitly |

## Sources

- Microsoft, "MUNIT function" —
  <https://support.microsoft.com/en-us/office/munit-function-c9fe916a-dc26-4105-997d-ba22799853a3>.
  Retrieved for this page: the syntax, the dimension argument description including "greater than
  zero" and the statement that it returns an array, the `#VALUE!` remark for non-positive
  dimensions, the 3×3 example, and the dynamic-array versus legacy-array entry mechanics. The page
  as retrieved says nothing about non-integer dimensions, about a size ceiling, or about array
  arguments.
- G. H. Golub and C. F. Van Loan, *Matrix Computations*, 4th edition, chapter 1 — why the identity
  is applied implicitly rather than formed.
- Handbook, [MMULT](FUNC.MMULT.md), [MINVERSE](FUNC.MINVERSE.md), [MDETERM](FUNC.MDETERM.md) — the
  rest of the family and the tests `MUNIT` anchors;
  [coercion and lifting](../model/02-coercion-and-lifting.md) — the lifting policy the strangest
  battery row exercises; [the value universe](../model/01-value-universe.md) — the array and
  publication boundaries.
- `data/functions/FUNC.MUNIT.json` and `data/presence/FUNC.MUNIT.json` — identity, arity 1–1, the
  declared `by_index_scalar_array_lift` broadcast profile, the empty signature placeholder, the
  shared `matrix_family` module, and the `BUG-FUNC-018` and `BUG-FUNC-026` defect streams, as
  projected at OxFunc `473efa3`.
