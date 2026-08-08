---
schema: efh.function-page/v1
function_id: FUNC.UNIQUE
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
  Reduces an array to its distinct rows (or columns), or to the rows that occur exactly once —
  two different set operations selected by one flag.
---

## What it computes

`UNIQUE` removes repetition along one axis. It compares **whole rows** (or, with `by_col` set,
whole columns) and returns a subset of them, in their original order of first appearance.

The flag `exactly_once` selects between two genuinely different operations, and Microsoft's
documentation is careful about the distinction:

- **`exactly_once` = `FALSE` (default): distinct.** Every row that appears at least once
  appears once in the output. From `A, B, A, C` you get `A, B, C`.
- **`exactly_once` = `TRUE`: unique in the database sense.** Only rows that appear *exactly*
  once survive. From `A, B, A, C` you get `B, C` — `A` is gone entirely, not collapsed.

These are not two settings of one filter; they are "distinct" and "singletons", and confusing
them silently drops data. The English word "unique" covers both, which is precisely why the
argument exists.

Two structural points:

- **The unit of comparison is a whole row, not a cell.** `UNIQUE(A1:C100)` treats two rows as
  equal only when all three of their cells match. To deduplicate on one column you pass that
  column, or you rebuild the row afterwards.
- **Order is preserved.** The output keeps the input's order of first appearance; `UNIQUE` does
  not sort. `SORT(UNIQUE(x))` is the composition that does both, and the order matters:
  `UNIQUE(SORT(x))` gives the same set with a different provenance for which occurrence was
  kept.

What "equal" means for two rows — case sensitivity for text, numeric near-equality, whether a
blank equals a zero, whether two `#N/A` values are equal — is the substantive open question on
this page. The documentation does not state it.

## Arguments

Microsoft documents `UNIQUE(array, [by_col], [exactly_once])`.

| Argument | Default | Meaning |
|---|---|---|
| `array` | required | The range or array from which to return unique rows or columns |
| `by_col` | `FALSE` | `TRUE` compares columns; `FALSE` or omitted compares rows |
| `exactly_once` | `FALSE` | `TRUE` returns only rows/columns occurring exactly once; `FALSE` returns all distinct ones |

The commonly misunderstood position is the third. Because both optional arguments are logicals
with `FALSE` defaults, `UNIQUE(A1:A9, TRUE)` reads at a glance like "yes, unique" but actually
means "compare columns" — which, for a single-column input, returns the whole column unchanged.
The singleton behaviour needs the *third* argument: `UNIQUE(A1:A9, , TRUE)`.

## Result and edge cases

The return kind is an array — a subset of the input's rows (or columns), full width, spilled.

- **The result can be empty.** `UNIQUE(x, , TRUE)` over data where every row repeats has
  nothing to return. Excel's convention for an empty array result is `#CALC!` (see
  [the value universe](../model/01-value-universe.md)); the Handbook has not verified that
  `UNIQUE` uses it.
- **Blanks are values.** A blank row is a row, and it will appear in the distinct set. This
  surprises readers who apply `UNIQUE` to a whole column and get a blank entry from the unused
  rows below the data — the usual remedy being `TRIMRANGE` or a `FILTER` first.
- **Errors inside the array** are values too; whether two identical error values compare equal
  is unverified.
- **Shape.** Row mode returns full-width rows; column mode returns full-height columns. A
  single-element result must still be array-shaped — the `ROWS`/`TYPE` probe on the `ROWS` page
  explains why that distinction is observable.
- Dynamic-array publication and `#SPILL!` are host-side adaptation, not `UNIQUE` semantics.

## Errors

Microsoft documents one error for `UNIQUE`: `#REF!` in the cross-workbook dynamic-array case,
when a linked source workbook is closed — dynamic arrays between workbooks are supported only
while both are open.

No other error is documented. `#SPILL!` arises from publication as for any dynamic array, and
`#CALC!` is Excel's general marker for an empty array result; neither is stated on the `UNIQUE`
page, and the Handbook does not assert them for this function.

## Relationships

- **`SORT`** orders; `UNIQUE` deduplicates. `SORT(UNIQUE(x))` is the canonical pairing.
- **`FILTER`** selects by predicate; `UNIQUE` selects by novelty. `UNIQUE(FILTER(...))` is the
  common pipeline.
- **`COUNTIF`** is the classic pre-dynamic-array way to compute both behaviours:
  `COUNTIF(range, x) = 1` is the `exactly_once` test, and a running `COUNTIF` over the prefix is
  the distinct test. Anyone maintaining older workbooks will meet these.
- **`TOCOL(…, 1)`** removes blanks, which is often what a reader actually wanted when they
  reached for `UNIQUE` on a padded column.
- **Data ▸ Remove Duplicates** is the destructive command form; `UNIQUE` is the formula form
  and leaves the source intact.

## Notes for implementers

- Row equality, not cell equality, is the primitive. It needs a defined comparison for every
  value kind, including blanks and errors, and it needs to be consistent with whatever equality
  the rest of the engine uses — or to be deliberately different, and recorded as such.
- Order of first appearance must be preserved. A hash-set implementation that returns iteration
  order will be wrong on almost every input and right on the tests.
- `exactly_once` requires counting, not just membership: it is a second pass (or a count-keyed
  map), and it cannot be short-circuited on first sight of a row.
- The empty result is a real case and needs a decided outcome rather than a panic or an empty
  array that later code cannot represent.
- 1×1 and single-row results must stay array-shaped; the reference engine's `BUG-FUNC-026`
  documents how easily a family-wide scalarization shortcut breaks nested semantics.

## What has not been checked

No Handbook vector suite exists for `UNIQUE`, and no Handbook evidence record is attached to
this page. Nothing here claims agreement with Excel for any implementation.

The equality relation is the whole open question, and it is where a suite should start:

1. **Text case.** `{"a";"A"}` — one distinct row or two? This single probe decides whether
   `UNIQUE` uses Excel's case-insensitive comparison or a stricter one, and it governs every
   real deduplication of names and codes.
2. **Numeric near-equality.** `{0.1+0.2; 0.3}` — one row or two? The strict/tolerant split
   recorded for the comparison families in OxFunc's `BUG-FUNC-004` shows Excel has both, and
   which one `UNIQUE` uses is unrecorded.
3. **Blank versus zero versus empty text**, in a single column.
4. **Errors**: two `#N/A` rows, an `#N/A` against a `#VALUE!`.
5. **The empty result** under `exactly_once`, to pin `#CALC!` or whatever else appears.
6. **`by_col` with rectangular input**, and both flags together.
7. **Number formatting irrelevance** — a date and its serial number are the same value, and the
   output should show it.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| distinct | Each repeated row kept once (`exactly_once` = `FALSE`) |
| exactly once | Only rows with no duplicate survive (`exactly_once` = `TRUE`) |
| order of first appearance | The output order; `UNIQUE` does not sort |
| row equality | The comparison primitive: whole rows, not individual cells |

## Sources

- Microsoft, *UNIQUE function* —
  <https://support.microsoft.com/en-us/office/unique-function-c5ab87fd-30a3-4ce9-9d1a-40204fb85e1e>
  (syntax, the `by_col` and `exactly_once` defaults, the explicit distinct-versus-database-unique
  wording, and the cross-workbook `#REF!` condition). Retrieved for this page.
- Handbook `content/model/01-value-universe.md` (the `#CALC!` empty-array convention) and
  `content/model/03-call-pipeline.md`.
- OxFunc bug streams `BUG-FUNC-004` (strict-versus-tolerant numeric comparison) and
  `BUG-FUNC-026` (1×1 array shape versus worksheet publication), under `docs/bugs/streams/`.
- Handbook `data/functions/FUNC.UNIQUE.json` (signature, arity, classification axes).
