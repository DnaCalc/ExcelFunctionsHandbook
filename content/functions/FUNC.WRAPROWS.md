---
schema: efh.function-page/v1
function_id: FUNC.WRAPROWS
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
  - Notes
  - Page vocabulary
  - Sources
family: dynamic_array_reshape_family
role_in_family: >-
  Reshapes a one-dimensional vector into a rectangle row by row, padding the last row to a full
  width — WRAPCOLS's row-major twin.
---

## What it computes

`WRAPROWS` turns a vector into a rectangle, filling it **row by row**.

Given a vector of `N` elements and a wrap count `k`, the result has `k` columns and `⌈N / k⌉`
rows: the first `k` elements become row 1 (left to right), the next `k` become row 2, and so
on. When `N` is not a multiple of `k`, the final row is short and its remaining cells take the
`pad_with` value.

`WRAPROWS({1,2,3,4,5}, 3)` gives

    1   2   3
    4   5   #N/A

This is the arrangement people usually have in mind when they say "wrap": reading order, left
to right then down, like text in a paragraph. `WRAPCOLS` does the same job in the other
direction and is the one that surprises.

Two points about the argument's meaning:

- **`wrap_count` is the row *width*** — how many values fit on a row — and the number of rows
  is derived. It is not "make me `k` rows".
- **The result's height grows with the input's length**, so the output shape is not determined
  by the input's shape alone.

When `wrap_count` is greater than or equal to `N`, everything fits on one row and the result is
a single-row array with no padding.

## Arguments

Microsoft documents `WRAPROWS(vector, wrap_count, [pad_with])`.

**`vector`** — required, and genuinely one-dimensional: a single row or a single column. A
rectangular input is documented as `#VALUE!` on the twin function's page, not silently
flattened.

**`wrap_count`** — required. The maximum number of values per row, and therefore the result's
column count. Documented as `#NUM!` when less than 1.

**`pad_with`** — optional, **default `#N/A`**. Placed in the trailing cells of the short final
row.

As on `WRAPCOLS` and `VSTACK`, the error-valued default is deliberate: padding that announces
itself cannot be mistaken for data. Supplying `pad_with` explicitly is preferable to wrapping
the whole call in `IFERROR`, which would also erase genuine `#N/A` values coming from the
source.

## Result and edge cases

The return kind is a two-dimensional array of `wrap_count` columns.

- **Exact multiples** produce no padding.
- **`wrap_count ≥ N`** gives a single row.
- **`wrap_count = 1`** gives a single column — one element per row.
- **A one-element vector** gives a 1×1 array, which must stay array-shaped.
- **`pad_with` is placed verbatim**, without coercion to the vector's type.
- **Errors and blanks inside `vector`** are ordinary elements; a genuine `#N/A` in the data is
  indistinguishable in the result from default padding.
- Dynamic-array publication and `#SPILL!` are host-side adaptation, described in
  [the call pipeline](../model/03-call-pipeline.md).

## Errors

The documented conditions for this pair, taken from the twin function's page:

| Error | Documented condition |
|---|---|
| `#VALUE!` | `vector` is not a one-dimensional array |
| `#NUM!` | `wrap_count` is less than 1 |
| `#N/A` | Placed in each cell of the return array that has no result — the default padding |

The Handbook has not separately confirmed that the `WRAPROWS` page states these identically;
they are recorded here as the documented conditions for the pair.

## Relationships

- **`WRAPCOLS`** is the twin, filling column by column with `wrap_count` as the column height.
  The pair are *not* transposes of one another in the simple sense: `WRAPROWS(v, k)` and
  `TRANSPOSE(WRAPCOLS(v, k))` both have `k` columns and put the same elements in the same
  reading order, so for the same `k` they agree — but that identity is worth verifying rather
  than assuming, and it fails to hold in the obvious way once padding enters.
- **`TOROW` / `TOCOL`** are the inverse direction: rectangle to vector.
- **`EXPAND`** pads an array to a target size with a chosen filler, which is the explicit form
  of `WRAPROWS`'s incidental padding.
- **`SEQUENCE(rows, cols)`** builds a numbered rectangle directly.
- **`MAKEARRAY`** builds a rectangle from a `LAMBDA`, which is the general form of everything
  in this corner of the family.
- Readers confuse `WRAPROWS` with `TRANSPOSE` (which does not change the element count per
  line) and with `HSTACK` (which concatenates rather than reshapes).

## Notes for implementers

- Output shape is `⌈N / wrap_count⌉ × wrap_count`. The ceiling and the final-row remainder are
  the two off-by-one sites.
- Row-major fill is the defining behaviour and must not be reachable by flipping a flag on the
  column-major path without also flipping the shape derivation.
- `pad_with` defaults to an error value and is placed without coercion.
- One-dimensionality is validated, not repaired.
- Single-row and 1×1 results must remain array-shaped; the reference engine's `BUG-FUNC-018`
  records this family's scalar-parameter array-admission gap and `BUG-FUNC-026` its 1×1 shape
  gap.

## What has not been checked

No Handbook vector suite exists for `WRAPROWS`, and no Handbook evidence record is attached to
this page. Nothing here claims agreement with Excel for any implementation.

First probes:

1. **Remainder cases** — `N` a multiple of `k`, one more, one less — with the padded cells read
   back individually.
2. **Input orientation**: the same values as a row and as a column, to establish whether
   "one-dimensional" erases the distinction.
3. **`wrap_count` boundaries**: `0`, negative, non-integer, `1`, `N`, `N + 1`.
4. **`pad_with` of every value kind**, including an array-valued `pad_with`, which the family's
   `BUG-FUNC-018` stream shows is a live question for scalar parameter positions here.
5. **Rectangular input**, against the documented `#VALUE!`.
6. **The relationship to `WRAPCOLS`**, stated above as a conjecture and testable without any
   external oracle.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `WRAPROWS` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-WRAPCOLS-001: WRAPCOLS and WRAPROWS ignore an error in pad_with when no padding cell is needed and lift over array wrap_count

*Quirk; measured; recorded 2026-11-22.* Applies to WRAPCOLS, WRAPROWS.

In Excel 16.0.20527.20088, WRAPCOLS and WRAPROWS require wrap_count >= 1 (0 gives #NUM!) and
only use pad_with for trailing incomplete rows or columns: when no padding is needed, an error
passed in pad_with (such as #VALUE!) is ignored. When wrap_count is an array, Excel lifts the
call over wrap_count's shape.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=WRAPCOLS("2",2,#VALUE!)` | as typed | `2` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=WRAPCOLS("2",0,"pad")` | as typed | `#NUM!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=WRAPROWS({1,2,3,4},{2,3})` | as typed | `{1,1}` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=CONCAT(WRAPROWS({1,2,3,4,5},2,0))` | as typed | `123450` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Measured in Excel 16.0.20527.20088 in fn2_questions (fn2_q011, fn2_q012,
fn2_q074, fn2_q075), f3_arrays (rows 29-30), and jnt_aggregate_arrays (rows 101-102), and
matched across 3,000/3,000 WRAPCOLS and 3,000/3,000 WRAPROWS full-bank rows.

Evidence:

- DnaCalc `crates/calc-func/witness-status.tsv`, WRAPCOLS; WRAPROWS at `95eb0cb2ef` — 3,000/3,000 WRAPCOLS and 3,000/3,000 WRAPROWS full-bank rows matched
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q012 row 20 column 10 at `95eb0cb2ef` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q011 row 20 column 10 at `95eb0cb2ef` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q074 row 20 column 10 at `95eb0cb2ef` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/f3_arrays.capture.json`, sheet f3_arrays row 29 column 4 at `95eb0cb2ef` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| wrap count | The number of values per row — the result's column count |
| row-major fill | Elements placed across each row in turn, reading order |
| pad value | `pad_with`, defaulting to `#N/A`; fills the short final row |
| self-announcing padding | Padding with an error value so it cannot be mistaken for data |

## Sources

- Microsoft, *WRAPROWS function* —
  <https://support.microsoft.com/en-us/office/wraprows-function-796825f3-975a-4cee-9c84-1bbddf60ade0>
  (syntax and the row-by-row placement rule). Not retrieved for this page; the behaviour above
  is stated as documented behaviour and should be re-checked against the page.
- Microsoft, *WRAPCOLS function* —
  <https://support.microsoft.com/en-us/office/wrapcols-function-d038b05a-57b7-4ee0-be94-ded0792511e2>
  (retrieved for this page; the twin's argument semantics, the `#N/A` default for `pad_with`,
  and the `#VALUE!` and `#NUM!` conditions recorded above for the pair).
- Handbook `content/model/01-value-universe.md` and `content/model/03-call-pipeline.md`.
- OxFunc bug streams `BUG-FUNC-018` and `BUG-FUNC-026`, under `docs/bugs/streams/`.
- Handbook `data/functions/FUNC.WRAPROWS.json` (signature, arity, classification axes).
