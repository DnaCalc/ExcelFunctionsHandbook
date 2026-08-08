---
schema: efh.function-page/v1
function_id: FUNC.WRAPCOLS
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
  - Notes for implementers
  - What has not been checked
  - Page vocabulary
  - Sources
family: dynamic_array_reshape_family
role_in_family: >-
  Reshapes a one-dimensional vector into a rectangle column by column, padding the last column
  to a full height.
---

## What it computes

`WRAPCOLS` turns a vector into a rectangle, filling it **column by column**.

Given a vector of `N` elements and a wrap count `k`, the result is an array of `k` rows and
`⌈N / k⌉` columns: the first `k` elements become column 1 (top to bottom), the next `k` become
column 2, and so on. If `N` is not a multiple of `k`, the final column is short and its
remaining cells are filled with `pad_with`.

Concretely, `WRAPCOLS({1,2,3,4,5}, 3)` gives

    1   4
    2   5
    3   #N/A

Two consequences of the definition are worth stating because the argument's name invites the
opposite reading:

- **`wrap_count` is the column *height*, not the number of columns.** It is "how many values
  per column", and the number of columns is derived. Reading it as "make me `k` columns" gives
  a transposed, wrongly sized answer.
- **The result's width grows with the input.** Unlike every other function in this family, the
  output shape depends on the input's *length* rather than on its shape.

When `wrap_count` is greater than or equal to `N`, everything fits in one column and the result
is a single column array — no padding, no second column.

## Arguments

Microsoft documents `WRAPCOLS(vector, wrap_count, [pad_with])`.

**`vector`** — required. Must be genuinely one-dimensional: a single row or a single column. A
rectangular array is documented as `#VALUE!`, not as something to flatten first. (If you want
flattening, that is `TOCOL`.)

**`wrap_count`** — required. The maximum number of values in each column, so the result's row
count. Documented as `#NUM!` when less than 1.

**`pad_with`** — optional, **default `#N/A`**. The value placed in the trailing cells of the
final column.

The default deserves the same note as `VSTACK`'s: padding with an *error* is a deliberate
choice that makes the padding self-announcing. Passing `pad_with` explicitly — `""`, `0`, or a
placeholder — is the usual remedy, and unlike the `IFERROR`-wrapping trick it does not also
swallow genuine `#N/A` values from the data.

## Result and edge cases

The return kind is a two-dimensional array of `wrap_count` rows.

- **Exact multiples** produce no padding at all.
- **`wrap_count ≥ N`** produces a single column, unpadded.
- **`wrap_count = 1`** produces a single row — every element in its own column. This is a
  transposition of the input when the input is a column, and a no-op when it is a row.
- **A one-element vector** produces a 1×1 array, which must stay array-shaped.
- **`pad_with` is not coerced** to the vector's type; it is placed as given, and it may be text
  in an otherwise numeric array.
- **Blanks and errors inside `vector`** are ordinary elements and are placed unchanged, which
  means a genuine `#N/A` in the data is indistinguishable from default padding in the result.
- Dynamic-array publication and `#SPILL!` are host-side adaptation, described in
  [the call pipeline](../model/03-call-pipeline.md).

## Errors

Documented by Microsoft:

| Error | Documented condition |
|---|---|
| `#VALUE!` | `vector` is not a one-dimensional array |
| `#NUM!` | `wrap_count` is less than 1 |
| `#N/A` | Placed in each cell of the return array that has no result — the default padding |

The third row is padding rather than failure, and a test oracle must not treat an `#N/A` in the
result as an error outcome.

## Relationships

- **`WRAPROWS`** is the twin, filling **row by row** instead of column by column, with
  `wrap_count` as the row *width*. The two produce transposed-looking but genuinely different
  arrangements of the same elements, and choosing the wrong one is the most common mistake in
  this pair.
- **`TOCOL` / `TOROW`** are the inverse operation — rectangle to vector — with the same
  self-announcing relationship to structure.
- **`EXPAND`** pads an existing array to a target size with a chosen filler; `WRAPCOLS` pads as
  a side effect of reshaping.
- **`SEQUENCE`** generates a rectangle of consecutive numbers directly, which is often what a
  reader reaching for `WRAPCOLS(SEQUENCE(n), k)` actually wants.
- **`INDEX` with computed row and column arithmetic** is the pre-dynamic-array way of doing
  this, and is still what you will find in older workbooks.

## Notes for implementers

- The output shape is `wrap_count × ⌈N / wrap_count⌉`. Deriving the width by division needs the
  ceiling, and the last column's fill boundary needs the remainder — two places to be off by
  one.
- Column-major fill is the defining behaviour; the twin function's row-major fill must not be
  reachable by a shared code path with a flipped flag unless the shape derivation flips too.
- `pad_with` defaults to `#N/A`, an error value, and must be placed verbatim without coercion.
- One-dimensionality is validated, not repaired: a rectangular input is `#VALUE!`.
- A single-column result and a 1×1 result must both remain array-shaped. The reference engine's
  `BUG-FUNC-018` covers the adjacent question of scalar-parameter positions admitting arrays in
  this family, and `BUG-FUNC-026` covers 1×1 shape preservation.

## What has not been checked

No Handbook vector suite exists for `WRAPCOLS`, and no Handbook evidence record is attached to
this page. Nothing here claims agreement with Excel for any implementation.

First probes:

1. **The remainder cases.** `N` one more, one less, and exactly a multiple of `wrap_count`,
   with the padded region read back cell by cell. This is where the ceiling and remainder
   arithmetic lives.
2. **Orientation of the input.** A row vector and a column vector of the same values — the
   documentation says "one-dimensional" without distinguishing them, and whether they produce
   the same result is not stated.
3. **`wrap_count` boundaries**: `0`, negative, non-integer, `1`, `N`, and `N + 1`, against the
   documented `#NUM!`.
4. **`pad_with` of every kind** — text, number, logical, error, an empty string, and an array —
   to pin whether it is placed verbatim.
5. **Rectangular input**, against the documented `#VALUE!`.
6. **`WRAPCOLS` against `TRANSPOSE(WRAPROWS(...))`** to establish exactly how the twins relate;
   they are not simple transposes of one another, and pinning the relationship is a cheap
   consistency check.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| wrap count | The number of values per column — the result's row count, not its column count |
| column-major fill | Elements placed down each column in turn |
| pad value | `pad_with`, defaulting to `#N/A`; fills the short final column |
| self-announcing padding | Padding with an error value so it cannot be mistaken for data |

## Sources

- Microsoft, *WRAPCOLS function* —
  <https://support.microsoft.com/en-us/office/wrapcols-function-d038b05a-57b7-4ee0-be94-ded0792511e2>
  (syntax, the column-by-column placement rule, `wrap_count` as the per-column element count,
  the `#N/A` default for `pad_with`, the single-column case when `wrap_count ≥ N`, and the
  `#VALUE!` and `#NUM!` conditions). Retrieved for this page.
- Handbook `content/model/01-value-universe.md` and `content/model/03-call-pipeline.md`.
- OxFunc bug streams `BUG-FUNC-018` (scalar-parameter array admission across this family) and
  `BUG-FUNC-026` (1×1 array shape versus worksheet publication), under `docs/bugs/streams/`.
- Handbook `data/functions/FUNC.WRAPCOLS.json` (signature, arity, classification axes).
