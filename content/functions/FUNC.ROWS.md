---
schema: efh.function-page/v1
function_id: FUNC.ROWS
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
family: rows_fn
role_in_family: >-
  Measures the vertical extent of a reference or array — the row count, as a single number.
---

## What it computes

`ROWS` returns the number of rows in its argument, as a scalar number.

For a reference, that is the row span of the referenced area: `ROWS(A5:C9)` is `5`. For a
single cell it is `1`. For an array — a literal, a dynamic-array result, or the value returned
by another function — it is the array's first dimension: `ROWS({1,2;3,4})` is `2`.

The whole function is that one sentence, which is why it is worth being exact about the two
things it is *not*:

- **It is not `ROW`.** `ROW` reports *which* rows; `ROWS` reports *how many*. `ROW(A5:A8)` is
  the array `{5;6;7;8}`; `ROWS(A5:A8)` is `4`.
- **It does not count populated rows.** `ROWS` measures the shape of the reference, not its
  contents. A range with three blank rows in the middle still has its full row count. Counting
  data is `COUNTA`'s job, and mistaking one for the other is the usual source of off-by-many
  errors in "dynamic range" formulas.

The function's real importance is structural: it is how a formula asks about the *shape* of
something it was handed. `ROWS` and `COLUMNS` are the shape-introspection primitives that make
generic array formulas possible, and they are what a `LAMBDA` uses to size its own work.

## Arguments

**`array`** — required, exactly one. A reference or an array whose row count is wanted. Both
argument kinds are admissible: the argument name in the signature says `array`, but a reference
is equally valid, and in practice references are the common case.

The reference engine's contract records that a non-reference, non-array argument is refused at
worksheet ingress rather than evaluated to an error value — so `ROWS(5)` is an admission
question, not a runtime one. The Handbook has not verified that split against Excel.

## Result and edge cases

The return kind is always a scalar number — never an array, whatever the argument's shape.

- **A 1×1 array argument** returns `1`, and is one of the places where the distinction between
  a scalar and a 1×1 array becomes observable. OxFunc's `BUG-FUNC-026` uses exactly this:
  `=ROWS(TAKE({1,2;3,4},1,1))` returns `1` in Excel, and `=TYPE(TAKE({1,2;3,4},1,1))` returns
  `64` (array), even though the anchor cell publishes the bare value `1`. `ROWS` is therefore a
  usable probe for whether a nested result is still array-shaped — a fact worth knowing when
  designing tests for other functions.
- **Whole-column references** return the grid's full row count, which differs between the
  modern grid and an `.xls`-compatibility workbook. `ROWS(A:A)` is a version-scoped answer.
- **Multi-area references** are not described by the documentation, and the Handbook has not
  verified what `ROWS` does with a union.
- Reference resolution, and the fact that `ROWS` inspects the reference rather than its values,
  are covered in [the call pipeline](../model/03-call-pipeline.md).

## Errors

Microsoft's documentation for `ROWS` names no error values. Observable failures come from
elsewhere: a `#REF!` reference propagates, and an argument of the wrong kind is refused at
entry.

Whether an *error value* passed directly (`ROWS(#N/A)`) propagates or is treated as a 1×1
shape is not documented, and the Handbook has not verified it.

## Relationships

- **`COLUMNS`** is the transpose: the horizontal extent, same shape of answer.
- **`ROW`** is the coordinate function, not the count function — see above.
- **`COUNTA` / `COUNT` / `COUNTBLANK`** count *contents*; `ROWS` measures *shape*. The
  reference engine's `BUG-FUNC-011` stream concerns a `COUNTBLANK` range-only parity gap and is
  a reminder that the shape/contents distinction is where these functions differ.
- **`SEQUENCE(ROWS(x))`** is the modern idiom for "one index per row of `x`", replacing
  `ROW(INDIRECT(...))`.
- **`TRIMRANGE`** answers the question people often ask `ROWS` by mistake: how big is the
  *populated* part of this range.

## Notes for implementers

- The answer must be a scalar, always. Returning a 1×1 array is wrong in a way that will not
  show up until the result is nested inside something shape-sensitive.
- The argument must not be dereferenced. `ROWS` of a whole column must not require reading a
  million cells.
- The 1×1-array-versus-scalar distinction is load-bearing. Argument preparation normally
  collapses a single-cell reference to a scalar; `ROWS` needs the shape that existed before
  that collapse, or it needs the reference itself.
- Whole-column and whole-row answers are grid-size dependent and therefore workbook-
  compatibility dependent. Hard-coding the modern grid's row count makes the function silently
  wrong on legacy workbooks.

## What has not been checked

No Handbook vector suite exists for `ROWS`, and no Handbook evidence record is attached to this
page. Nothing here claims agreement with Excel for any implementation.

First probes, in order of value:

1. **Shape-preservation probes**: `ROWS` applied to the results of `TAKE`, `INDEX`, `FILTER`,
   `UNIQUE` and `TRANSPOSE`, including their 1×1 cases — this is where `ROWS` is most useful
   as an instrument and most likely to expose an engine's scalarization mistakes.
2. **Whole-column and whole-row references**, on the modern grid and on an `.xls`-compatibility
   workbook, to pin the version scope of the answer.
3. **Multi-area unions and 3-D references.**
4. **Direct scalars, text and error values** as the argument, to establish which failures are
   admission-time and which are runtime.
5. **Empty arrays**, if the host can produce one, against the `#CALC!` convention.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| shape introspection | Asking about an argument's extent rather than its contents |
| 1×1 array | An array with one element; distinct from a bare scalar, and observable through `ROWS`/`TYPE` |
| grid-size dependent | The answer for a whole-column reference depends on the workbook's compatibility version |

## Sources

- Microsoft, *ROWS function* —
  <https://support.microsoft.com/en-us/office/rows-function-b592593e-3fc2-47f2-bec1-bda493811597>
  (syntax and the row-count definition). Not retrieved for this page; the behaviour above is
  stated as documented behaviour and should be re-checked against the page.
- Handbook `content/model/01-value-universe.md` and `content/model/03-call-pipeline.md`.
- OxFunc `docs/function-lane/FUNCTION_SLICE_ROWS_CONTRACT_PRELIM.md` — the reference engine's
  declared contract: `1` for a single cell, the area's row count for a reference, the array's
  row count for an array constant, always a scalar numeric result, and admission rejection for
  other argument kinds.
- OxFunc bug stream `docs/bugs/streams/BUG-FUNC-026_take_1x1_scalar_publication_mismatch.md` —
  the Excel probes `=TYPE(TAKE({1,2;3,4},1,1))` → `64` and `=ROWS(TAKE({1,2;3,4},1,1))` → `1`,
  which separate function-level shape from worksheet publication.
- Handbook `data/functions/FUNC.ROWS.json` (signature, arity, classification axes).
