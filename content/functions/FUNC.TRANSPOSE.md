---
schema: efh.function-page/v1
function_id: FUNC.TRANSPOSE
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
  Exchanges the two axes of an array: the m×n input becomes an n×m output with element (i,j)
  moved to (j,i).
---

## What it computes

`TRANSPOSE` exchanges an array's axes. For an `m × n` input `A`, the result `B` is `n × m`
with

    B[j, i] = A[i, j]     for all i ∈ 1..m, j ∈ 1..n

That is the complete definition, and it is one of the very few worksheet functions whose
mathematics is exactly the mathematics of the same-named operation in linear algebra. A row
becomes a column, a column becomes a row, and a rectangle turns on its side. No values change,
none are added or removed, and the operation is its own inverse:
`TRANSPOSE(TRANSPOSE(A)) = A`.

The historical importance of `TRANSPOSE` is out of proportion to its simplicity. Before dynamic
arrays it was one of the very few functions whose *natural* result was an array, which made it
the standard teaching example for Ctrl+Shift+Enter array entry — you had to select the target
rectangle first, with the dimensions swapped, before committing the formula. On a modern Excel
it simply spills, and that entire ritual is gone.

## Arguments

`TRANSPOSE(array)` — exactly one argument, required, no options.

**`array`** — the range or array to transpose. A reference is admissible and is resolved to its
values; unlike `OFFSET` or `ROW`, `TRANSPOSE` has no interest in the address.

The only thing regularly misunderstood about the argument is what comes back: readers who pass
a single column expect a single column with a different orientation on screen, which is exactly
what they get, and readers who pass a rectangle sometimes expect a rotation rather than a
transposition. Transposition is a reflection across the main diagonal, not a 90-degree
rotation: the top-left element stays put, and the anti-diagonal order is not reversed.

## Result and edge cases

The return kind is an array with the input's dimensions exchanged.

- **Values are moved, not interpreted.** Errors, text, logicals and blanks all survive
  transposition unchanged. `TRANSPOSE` does not coerce.
- **What happens to blanks** is the one genuine question. A blank cell in a range has no value
  of its own, and the general model says `Empty` is admitted at the raw-return boundary but not
  at the published-result boundary, normalizing to zero on the way out
  ([the value universe](../model/01-value-universe.md)). A transposed blank cell that publishes
  as `0` is therefore expected behaviour under the shared model rather than a `TRANSPOSE`
  quirk — but the Handbook has not verified it for this function.
- **A 1×1 input** transposes to a 1×1 array, and must remain array-shaped.
- **A single-row or single-column input** is the common case and exercises none of the
  interesting index arithmetic; a rectangular input does.
- **Very large arrays** hit the grid limits at publication.
- Dynamic-array publication and `#SPILL!` are host-side adaptation, described in
  [the call pipeline](../model/03-call-pipeline.md).

## Errors

Microsoft's page for `TRANSPOSE` does not enumerate error conditions, which is consistent with
a total function on arrays: every rectangular array has a transpose.

The errors a reader meets in practice come from elsewhere — `#SPILL!` when the swapped
dimensions do not fit, `#VALUE!` on a legacy grid when an array result was not array-entered,
and whatever error values were already inside the array and are simply carried across.

## Relationships

- **`TOCOL` / `TOROW`** flatten rather than transpose; the pair are related by
  `TOROW(x, i, s) = TRANSPOSE(TOCOL(x, i, s))`, which makes `TRANSPOSE` the bridge between the
  two flattening directions.
- **`VSTACK` / `HSTACK`** are similarly related through transposition:
  `HSTACK(a, b) = TRANSPOSE(VSTACK(TRANSPOSE(a), TRANSPOSE(b)))`.
- **`MMULT`, `MINVERSE`, `MDETERM`** are the rest of the matrix family; `TRANSPOSE` is the one
  member that is pure index arithmetic with no floating-point content, and therefore the one
  member with no accuracy question at all.
- **Paste Special ▸ Transpose** is the destructive command form: it writes values, does not
  recompute, and is what `TRANSPOSE` replaces for live data.
- **`WRAPROWS` / `WRAPCOLS`** reshape rather than reflect; a reader wanting `m × n` from `n × m`
  data in a *different* element order wants those, not this.

## Notes for implementers

- The operation is index arithmetic and nothing else. Any implementation that copies values
  through a coercion step is doing more than the function asks and can only introduce
  divergence.
- Row-major storage makes transposition a strided copy; for large arrays the cache behaviour
  differs sharply between the naive and the blocked implementation, but the result must not.
- A 1×1 result stays an array; the reference engine's `BUG-FUNC-026` records the family-wide
  version of this trap.
- `TRANSPOSE` is an excellent *test instrument* for other functions precisely because it is
  exact and self-inverse: `TRANSPOSE(TRANSPOSE(f(x))) = f(x)` is a metamorphic check that needs
  no oracle.
- Blank handling should follow the engine's general empty-cell rule rather than acquiring a
  local convention.

## What has not been checked

No Handbook vector suite exists for `TRANSPOSE`, and no Handbook evidence record is attached to
this page. Nothing here claims agreement with Excel for any implementation.

`TRANSPOSE` is the easiest function in this set to characterize, because it admits exact
metamorphic tests: self-inversion, and agreement with the identities relating it to the
stacking and flattening functions. A suite that establishes those needs no numerical oracle at
all.

First probes:

1. **Rectangular inputs** (not square, not vectors) read back element by element — the only
   shape where an index-arithmetic error is visible.
2. **Blank cells**, transposed and published, against the model's `Empty`-normalizes-to-zero
   rule.
3. **Mixed value kinds**, including errors and long text, to confirm nothing is coerced in
   transit.
4. **Self-inversion** on every shape, including 1×1.
5. **The stated identities** against `TOCOL`/`TOROW` and `VSTACK`/`HSTACK`.
6. **Size limits** and the `#SPILL!` boundary when the swapped dimensions exceed the grid.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| transposition | Reflection across the main diagonal; `B[j,i] = A[i,j]` |
| self-inverse | Applying the function twice returns the original |
| metamorphic test | A check based on an identity between results, requiring no external oracle |
| array entry | The legacy Ctrl+Shift+Enter ritual that dynamic arrays replaced |

## Sources

- Microsoft, *TRANSPOSE function* —
  <https://support.microsoft.com/en-us/office/transpose-function-ed039415-ed8a-4a81-93e9-4b6dfac76027>
  (syntax and the axis-exchange definition). Not retrieved for this page; the behaviour above is
  stated as documented behaviour and should be re-checked against the page.
- Handbook `content/model/01-value-universe.md` (the `Empty` kind and the raw-return versus
  published-result boundary) and `content/model/03-call-pipeline.md`.
- OxFunc bug stream `docs/bugs/streams/BUG-FUNC-026_take_1x1_scalar_publication_mismatch.md` —
  the 1×1 array shape distinction, which applies across this family.
- Handbook `data/functions/FUNC.TRANSPOSE.json` (signature, arity, classification axes).
