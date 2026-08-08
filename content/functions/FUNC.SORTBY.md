---
schema: efh.function-page/v1
function_id: FUNC.SORTBY
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
  Sorts an array by one or more parallel key arrays that need not be part of the data, with an
  independent direction per key — the multi-level member of the sorting pair.
---

## What it computes

`SORTBY` reorders an array using keys held **outside** it.

Given data `A` and key vectors `K₁, K₂, …` all of the same length as `A`'s sorted axis, and a
direction `d₁, d₂, …` for each, `SORTBY` computes the permutation `π` that orders the tuple
sequence `(K₁[i], K₂[i], …)` lexicographically — `K₁` first, `K₂` breaking `K₁`'s ties, and so
on — each component ordered in its own direction. It then applies `π` to `A`.

Two properties distinguish it from `SORT`:

1. **The keys need not be in the data.** You can sort a visible table by a hidden column, by a
   computed expression (`SORTBY(names, LEN(names))`), or by a lookup result. Nothing of the key
   appears in the output.
2. **It is multi-level.** `SORT` sorts by one key. `SORTBY` composes an arbitrary number of
   keys with independent directions, which is what "sort by region ascending, then revenue
   descending" requires.

As with `SORT`, the result is a **permutation of the same shape**: nothing is deduplicated,
nothing is filtered, and every element of `A` appears exactly once.

## Arguments

Microsoft documents
`SORTBY(array, by_array1, [sort_order1], [by_array2, sort_order2], …)`.

| Argument | Required | Default | Meaning |
|---|---|---|---|
| `array` | yes | — | The range or array to sort |
| `by_array1` | yes | — | The first key vector |
| `sort_order1` | no | `1` | `1` ascending, `-1` descending |
| `by_array2`, `sort_order2`, … | no | `1` | Further key/direction pairs, applied as tie-breakers |

The documented shape rules are strict and worth stating exactly, because they are the usual
source of `#VALUE!`:

- each `by_array` must be **one row high or one column wide** — a key vector, never a
  rectangle;
- all the arguments must be the same size as each other, and `array`'s sorted dimension must
  match the key vectors' length.

The argument positions most often misread are the pairs themselves. Because `sort_order` is
optional *within* each pair, the arguments after the second are position-sensitive in a way
that is easy to get wrong: `SORTBY(A, K1, K2)` does not mean "two keys" — it means "key `K1`,
sort order `K2`", and unless `K2` happens to be `1` or `-1` that is a `#VALUE!`. Two keys
require the explicit order for the first: `SORTBY(A, K1, 1, K2, 1)`.

Omitted-slot forms (`SORTBY(A, K1,)`) must default exactly as an absent argument does. The
reference engine's `BUG-FUNC-010` records the `SORT`/`SORTBY` omitted-optional defaulting path
as a defect class that existed and was repaired — a reminder that the empty-slot spelling is a
distinct call shape.

## Result and edge cases

The return kind is an array of the same shape as `array`, spilled.

- **Orientation.** Whether `SORTBY` reorders rows or columns follows from the orientation of
  the key vectors — a column key sorts rows, a row key sorts columns. There is no `by_col`
  flag; the shape carries the intent.
- **Ties beyond the last key.** When every supplied key ties, the resulting order is whatever
  the underlying sort does with equal elements. Stability is not documented, and the Handbook
  has not pinned it.
- **Mixed value kinds in a key vector** require the same cross-kind total order that `SORT`
  needs, and it is equally unverified here.
- **Text keys are collated**, and collation is locale-dependent.
- Dynamic-array publication (`#SPILL!` when the result cannot be placed) is host-side
  adaptation, described in [the call pipeline](../model/03-call-pipeline.md), not a `SORTBY`
  outcome.

## Errors

Documented by Microsoft:

| Error | Documented condition |
|---|---|
| `#VALUE!` | A `sort_order` argument that is not `1` or `-1` |
| `#REF!` | A linked source workbook is closed when the dynamic array refreshes |

`#SPILL!` arises, as for every dynamic-array function, when the result has nowhere to land.
Mismatched sizes between `array` and a `by_array`, or a rectangular `by_array`, violate the
documented shape rules; the documentation states the requirement without naming the error, and
the Handbook has not verified which error results.

## Relationships

- **`SORT`** is the single-key, internal-key sibling. `SORT(A, k, d)` is
  `SORTBY(A, INDEX(A, 0, k), d)` in spirit; `SORTBY` is the more general of the two and is the
  one to reach for whenever the key is computed or the sort is multi-level.
- **`UNIQUE`** deduplicates; `SORTBY` never does.
- **`FILTER`** selects; `SORTBY` reorders. `SORTBY(FILTER(...), FILTER(...))` requires the two
  filters to agree, which is a common source of misalignment.
- **`RANK` / `LARGE` / `SMALL`** answer order questions without materializing a permutation.
- Readers confuse `SORTBY` with `SORT`'s `sort_index`, and expect a `by_col` argument that does
  not exist.

## Notes for implementers

- The multi-key comparison is lexicographic with per-component direction. Implementing it as
  repeated single-key sorts only works if the underlying sort is stable *and* the keys are
  applied in reverse order; if stability is undecided, that trick is unsound.
- The pairing structure means argument parsing is positional and irregular: an odd trailing
  argument is a key, an even one is a direction. Validate `sort_order` values eagerly —
  Microsoft documents `#VALUE!` for anything other than `1` or `-1`, so silently accepting `0`
  or `2` is a divergence.
- Shape validation belongs before sorting: rectangular key arrays and length mismatches are
  documented as invalid, not as something to broadcast.
- The omitted-slot default path is a recorded defect class in this family.
- The cross-kind comparison order and the collation used for text should be the same ones
  `SORT` uses; two orderings in one engine is a bug waiting for a witness.

## What has not been checked

No Handbook vector suite exists for `SORTBY`, and no Handbook evidence record is attached to
this page. Nothing here asserts that any implementation agrees with Excel.

First probes:

1. **The error for shape violations** — a rectangular `by_array`, and a `by_array` shorter and
   longer than `array`'s sorted axis. Documented as invalid, error unnamed.
2. **Multi-key tie-breaking**, with three keys and deliberately colliding prefixes, in every
   direction combination — this is the behaviour that has no single-key analogue and therefore
   no borrowed evidence.
3. **Stability** past the last key.
4. **Orientation inference**: a row-shaped `by_array` against a rectangular `array`, and the
   degenerate 1×1 case where row and column shapes coincide.
5. **`sort_order` values outside `{1, -1}`**, including `0`, `TRUE`/`FALSE`, text `"1"`, and an
   array-valued order.
6. **Mixed-kind and locale-sensitive keys**, shared with the `SORT` probe list.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| key vector | A `by_array`: one row high or one column wide, parallel to the sorted axis |
| multi-level sort | Lexicographic ordering by several keys, each with its own direction |
| orientation inference | Row-versus-column sorting decided by the key vector's shape, not a flag |
| omitted slot | An empty argument position between commas; delivers `Missing`, not `Empty` |

## Sources

- Microsoft, *SORTBY function* —
  <https://support.microsoft.com/en-us/office/sortby-function-cd2d7a62-1b93-435c-b561-d6a35134f28f>
  (syntax, the key/order pair structure, the one-row-or-one-column and equal-size requirements,
  the `#VALUE!` condition for a `sort_order` other than `1` or `-1`, and the cross-workbook
  `#REF!` note). Retrieved for this page.
- Handbook `content/model/01-value-universe.md` and `content/model/03-call-pipeline.md`.
- OxFunc bug stream
  `docs/bugs/streams/BUG-FUNC-010_dynamic_array_sort_family_omitted_optional_argument_defaulting_gap.md`
  — the omitted-optional defaulting defect covering the `SORTBY(…, by_array,)` lane.
- Handbook `data/functions/FUNC.SORTBY.json` (signature, arity, classification axes).
