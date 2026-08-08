---
schema: efh.function-page/v1
function_id: FUNC.SORT
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
  Reorders an array's rows (or columns) by the values in one of its own rows or columns,
  returning a rearranged array of the same shape.
---

## What it computes

`SORT` permutes an array along one axis. It returns an array of **exactly the same shape** as
its input — no rows are added, removed or merged — whose rows (or, with `by_col` set, whose
columns) have been reordered so that the values in one designated column (or row) are in
ascending or descending order.

Stated precisely: let `A` be an `m × n` array. With `by_col` false, `SORT` computes a
permutation `π` of `1..m` such that the sequence `A[π(1), k], A[π(2), k], …` is ordered, where
`k` is `sort_index`, and returns the array `B` with `B[i, j] = A[π(i), j]`. The whole row moves
with its key; that is the entire point, and it is what distinguishes `SORT` from sorting a
single column in isolation.

Two facts follow from "permutation of the same shape" that are worth stating because readers
routinely expect otherwise:

- **`SORT` does not deduplicate.** Equal keys keep all their rows. That is `UNIQUE`'s job.
- **`SORT` does not filter.** Blanks are values and get a position in the order; they are not
  dropped. A range with trailing blank rows sorts those blanks somewhere, usually visibly.

The comparison order over mixed value kinds — numbers against text against logicals against
blanks — is the part the documentation does not state, and it is the part the Handbook has not
verified. Everything else about `SORT` is shape arithmetic.

## Arguments

Microsoft documents `SORT(array, [sort_index], [sort_order], [by_col])`.

| Argument | Default | Meaning |
|---|---|---|
| `array` | required | The range or array to sort |
| `sort_index` | `1` | Which row or column supplies the keys |
| `sort_order` | `1` | `1` ascending, `-1` descending |
| `by_col` | `FALSE` | `FALSE` sorts rows by a column key; `TRUE` sorts columns by a row key |

The argument most often misread is `by_col`, because it changes what `sort_index` *indexes*.
With `by_col` false (the default), `sort_index` names a **column** and rows are permuted. With
`by_col` true, `sort_index` names a **row** and columns are permuted. The flag and the index
are coupled; changing one without the other silently sorts by the wrong key.

`sort_index` is relative to `array`, not to the worksheet. If `array` is `C2:F20`, then
`sort_index` `1` means column `C`.

Omitted optional arguments take the defaults above. Note that an omitted argument written as
an empty slot — `SORT(A1:C9,,-1)` — is the `Missing` marker of
[the value universe](../model/01-value-universe.md), and must default exactly as though the
argument were absent. The reference engine's `BUG-FUNC-010` records a period in which it did
not: `SORT({2;3;7;5},,-1)` failed on the OxFunc surface (worksheet outcome `#VALUE!`) while the
explicit `SORT({2;3;7;5},1,-1)` returned `{7;5;3;2}`. That is a good illustration of a bug class
specific to this family: the omitted-slot form is a distinct call shape and needs its own tests.

## Result and edge cases

The return kind is an array with the same dimensions as `array`, published as a dynamic-array
spill.

- **Ties.** Whether the permutation is stable — whether rows with equal keys keep their
  original relative order — is not stated by the documentation. It is observable, it matters
  for reproducibility, and the Handbook has not pinned it.
- **Mixed types in the key column.** `SORT` needs a total order across value kinds. Microsoft
  documents such an order for `MATCH` (numbers, then text, then `FALSE`, then `TRUE`); whether
  `SORT` uses the same one is an open question here.
- **Text ordering is locale-dependent.** Sorting text means collation, and collation depends on
  the locale. This is the one part of `SORT` that cannot be characterized by a locale-free
  vector suite.
- **`sort_index` out of range** is expected to be an error rather than a clamp; the specific
  error is not documented.
- **1×1 and single-row/single-column arrays** are trivially sorted but are exactly where
  array-versus-scalar shape mistakes surface — see the `ROWS` page for why a 1×1 result must
  stay an array.

## Errors

Microsoft's page does not enumerate error conditions for `SORT` itself; it mentions `#REF!`
arising in cross-workbook dynamic-array scenarios when the source workbook is closed and the
formula is refreshed, which is a dynamic-array behaviour rather than a `SORT` behaviour.

Beyond that, the errors a reader will actually meet are `#SPILL!` when the result cannot be
placed (a publication outcome, not a function outcome) and whatever `SORT` returns for an
out-of-range `sort_index`, which the Handbook has not verified. Error values inside `array`
are carried along as values — `SORT` selects and reorders, it does not evaluate — but where
they sort to is unverified.

## Relationships

- **`SORTBY`** is the sibling that sorts by *external* keys: one or more arrays parallel to the
  data but not part of it, each with its own direction. `SORT` is the special case where the
  key lives inside the array. `SORTBY` also supports multi-level sorts, which `SORT` does not.
- **`UNIQUE`** removes duplicates; `SORT(UNIQUE(x))` is the standard composition and the two
  are frequently confused because both "tidy up a list".
- **`FILTER`** selects rows; `SORT` reorders them. `SORT(FILTER(...))` is the usual pipeline.
- **`LARGE` / `SMALL` / `RANK`** answer order questions about a single vector without
  materializing a sorted copy.
- The legacy equivalent is the **Data ▸ Sort** command, which mutates the sheet. `SORT` is the
  formula form: the source is untouched and the result recomputes.

## Notes for implementers

- Sorting must move whole rows, not just the key column. An implementation that sorts the key
  and reassembles by index is fine; one that sorts each column independently is catastrophic
  and produces plausible-looking output.
- The omitted-slot call shape (`SORT(a,,-1)`) must normalize to the same default path as an
  absent trailing argument. This is a real, recorded defect class in this family.
- Stability is a decision, and an undecided one is a source of nondeterminism across
  implementations. Record whatever choice is made.
- The cross-kind comparison order is shared with every other ordering surface in the engine
  (`SORTBY`, `LARGE`, ordered `MATCH`, `XMATCH`'s nearest modes). It should live in one place.
- Text collation is locale-dependent, which means `SORT` cannot be `portable-reproducible`
  without a declared collation; that declaration belongs in the flavour's contract, not in the
  kernel.

## What has not been checked

No Handbook vector suite exists for `SORT`, and no Handbook evidence record is attached to this
page. Nothing here claims agreement with Excel for any implementation. The one Excel-adjacent
observation quoted above (`BUG-FUNC-010`) is a record of an OxFunc-side defect, not a
measurement of Excel.

First probes:

1. **Tie stability.** An array with repeated keys and a distinguishing second column, sorted
   ascending and descending. This is cheap and settles a question that affects every downstream
   comparison.
2. **Mixed-kind key columns** — numbers, numeric-looking text, text, `TRUE`/`FALSE`, blanks and
   errors in one column — to pin the total order and where blanks and errors land.
3. **Text collation**, across at least two locales, including case and accented characters.
   This is the probe that decides whether `SORT` is locale-scoped.
4. **`sort_index` boundaries**: `0`, negative, non-integer, and one past the last column, in
   both `by_col` settings.
5. **The omitted-slot forms** `SORT(a,,-1)` and `SORT(a,,,TRUE)`, against their explicit
   equivalents.
6. **Shape preservation** for 1×1, single-row and single-column inputs, read back through
   `ROWS`, `COLUMNS` and `TYPE`.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| key column | The column (or row) named by `sort_index` whose values determine the order |
| permutation | The reordering applied to whole rows; the result has the input's shape |
| stability | Whether equal keys preserve their original relative order |
| omitted slot | An empty argument position between commas; delivers `Missing`, not `Empty` |
| collation | The locale-dependent ordering applied to text |

## Sources

- Microsoft, *SORT function* —
  <https://support.microsoft.com/en-us/office/sort-function-22f63bd0-ccc8-492f-953d-c20e8e44b86c>
  (syntax, the four arguments with their defaults `1`, `1`, `FALSE`, and the note about `#REF!`
  in cross-workbook dynamic-array refresh). Retrieved for this page.
- Microsoft, *MATCH function* — for the one cross-type ordering Microsoft does document.
- Handbook `content/model/01-value-universe.md` (`Missing` versus `Empty`) and
  `content/model/03-call-pipeline.md` (dynamic-array publication as host-side adaptation).
- OxFunc bug stream
  `docs/bugs/streams/BUG-FUNC-010_dynamic_array_sort_family_omitted_optional_argument_defaulting_gap.md`
  — the omitted-slot defaulting defect, with the contrasting explicit call.
- Handbook `data/functions/FUNC.SORT.json` (signature, arity, classification axes).
