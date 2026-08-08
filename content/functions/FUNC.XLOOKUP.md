---
schema: efh.function-page/v1
function_id: FUNC.XLOOKUP
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
family: xlookup
role_in_family: >-
  The modern general lookup: an independent lookup vector and return vector, an explicit match
  mode defaulting to exact, an explicit search direction, and a caller-supplied not-found value.
---

## What it computes

`XLOOKUP` finds a position in one vector and returns the corresponding slice of another.
Given a lookup value `v`, a lookup array `L` of `n` entries, and a return array `R`, it
determines an index `i ∈ 1..n` by the match rule below and returns `R`'s `i`-th slice — a
single value when `R` is a vector, and a whole row or column when `R` is a two-dimensional
area aligned to `L`.

The separation is the design point. In `VLOOKUP` the returned column is named by a positional
integer counted inside the searched table, so the search and the return are coupled through a
shared geometry. In `XLOOKUP` they are two independent arguments: `R` may sit to the left of
`L`, may be wider or taller, and may live in a different part of the sheet, as long as its
length along the matching axis equals `L`'s.

The match rule is a two-dimensional choice, and both dimensions are explicit:

- **`match_mode`** decides *what counts as a match*: exact only (default), exact-or-next-
  smaller, exact-or-next-larger, or wildcard.
- **`search_mode`** decides *how the vector is traversed*: forward from the first item,
  backward from the last, or by binary search assuming `L` is sorted ascending or descending.

Making both explicit is the substantive improvement over the older lookups, where the two
were entangled in one flag and the default silently chose the risky combination. `XLOOKUP`'s
default is exact match, forward search — the combination whose failure mode is a visible
`#N/A` rather than a plausible wrong number.

## Arguments

Microsoft documents the signature as
`XLOOKUP(lookup_value, lookup_array, return_array, [if_not_found], [match_mode], [search_mode])`.

**`lookup_value`** — the key. Microsoft notes that if it is omitted, `XLOOKUP` returns blank
cells found in `lookup_array`; that is an unusual admission (the argument is otherwise
required) and is worth treating as an edge to probe rather than a routine idiom.

**`lookup_array`** — the array or range to search. Its length along the matching axis fixes
the correspondence with `return_array`.

**`return_array`** — the array or range to return from. Aligned to `lookup_array` by position,
not by address.

**`if_not_found`** — optional. The value to return when no match is found. When it is absent,
a miss is `#N/A`. This argument is the reason `IFNA(XLOOKUP(...), ...)` is usually redundant.

**`match_mode`** — optional, default `0`:

| Value | Documented meaning |
|---|---|
| `0` | Exact match; if none found, return `#N/A` (default) |
| `-1` | Exact match; if none found, return the next smaller item |
| `1` | Exact match; if none found, return the next larger item |
| `2` | A wildcard match where `*`, `?` and `~` have special meaning |

**`search_mode`** — optional, default `1`:

| Value | Documented meaning |
|---|---|
| `1` | Search starting at the first item (default) |
| `-1` | Reverse search starting at the last item |
| `2` | Binary search; relies on `lookup_array` being sorted ascending |
| `-2` | Binary search; relies on `lookup_array` being sorted descending |

The two argument positions most commonly misread are `match_mode` and `search_mode`
themselves — they are easy to swap, and both accept small signed integers, so a swap is not a
type error. `XLOOKUP(v, L, R, , -1)` is "next smaller item, forward search"; `XLOOKUP(v, L, R,
, , -1)` is "exact match, searched backwards". They are different functions, and both are
spelled with a `-1`.

Note also the fourth position: `if_not_found` sits *between* the return array and the mode
flags. Writing `XLOOKUP(v, L, R, 0)` intending exact match sets the not-found value to `0` and
leaves the mode at its default — a mistake that produces `0` instead of `#N/A` on a miss.

## Result and edge cases

`XLOOKUP` returns whatever `return_array` holds at the matched position: a scalar for a vector
return array, or an entire row or column for a two-dimensional one. The row-or-column return
is what makes `XLOOKUP` usable as a record fetch — one call, whole record — where `VLOOKUP`
needs one call per field.

- **Array-valued `lookup_value`.** `XLOOKUP` lifts over an array of needles and spills. Live
  Excel replay on 2026-04-29, recorded in OxFunc's `BUG-FUNC-006`, pinned the multi-needle
  return shape; that record's sibling rows show the adjacent lookup functions returning one
  result per needle with element-local `#N/A`. The exact spill geometry when both the needle
  array and the return array are two-dimensional is the interesting case, and the Handbook has
  not characterized it.
- **Binary search modes on unsorted data.** Microsoft states only that binary search "relies
  on" the array being sorted. As with `VLOOKUP`'s approximate mode, that is a precondition
  rather than a checked constraint: unsorted input under `search_mode` `±2` yields a
  deterministic but undocumented answer, not an error.
- **Blanks in the lookup array.** Excel's approximate and binary search paths skip blank cells
  rather than aborting; see the `MATCH`/`XMATCH` evidence in OxFunc's `BUG-FUNC-040`. Whether
  `XLOOKUP` inherits exactly the same skipping in every mode is not something the Handbook has
  pinned.
- Empty cells, omitted arguments, and error inputs follow the shared rules in
  [coercion and lifting](../model/02-coercion-and-lifting.md); the reference-aware argument
  preparation `XLOOKUP` carries is described in
  [the call pipeline](../model/03-call-pipeline.md).

## Errors

Documented by Microsoft: `#N/A` when no match exists and no `if_not_found` was supplied.
Microsoft additionally warns that binary search modes over an unsorted array produce invalid
results — note that this is a warning about *wrong answers*, not about an error value.

`XLOOKUP` is also commonly reported to return `#VALUE!` when `lookup_array` and `return_array`
have incompatible dimensions, and `#REF!` for arrays in different workbooks in some
configurations. The Handbook has not verified either against the current documentation page or
against a live build, and does not assert them.

## Relationships

- **`XLOOKUP` supersedes `VLOOKUP`, `HLOOKUP` and `LOOKUP`.** It covers all three, in both
  orientations, with an exact-match default, an explicit not-found value, backward search, and
  a return array that need not lie to the right of (or below) the search vector. Microsoft has
  not deprecated the older functions: they remain supported, and existing workbooks keep
  working. For new work `XLOOKUP` is the better default.
- **`XMATCH`** is `XLOOKUP` without the return step: same `match_mode` and `search_mode`
  vocabulary, returning the position instead of the value. `INDEX(R, XMATCH(v, L, ...))` is
  `XLOOKUP` decomposed.
- **`FILTER`** answers a different question: all rows satisfying a predicate, not the one row
  matching a key.
- Readers coming from `INDEX`/`MATCH` should note that `XLOOKUP`'s alignment is positional
  along the matching axis, exactly as `INDEX`/`MATCH`'s is — the relationship is a repackaging,
  not a new mechanism.

## Notes for implementers

- The four `match_mode` values are not four settings of one comparison; wildcard mode changes
  the predicate from equality to pattern matching, and the two "next item" modes require an
  order over values that exact mode does not need.
- `search_mode` and `match_mode` interact. Binary search is only coherent with an ordered
  predicate; what binary search does under wildcard mode is a real question, not a
  don't-care.
- Blank-skipping belongs to the ordered search paths. An implementation that treats a blank
  as an incomparable value and aborts will fail on ordinary trailing-blank ranges.
- `lookup_value` must admit arrays, and `return_array` must be able to yield a whole row or
  column per match. Those two together mean the natural result shape is not always a vector.
- `if_not_found` is returned verbatim; it is not coerced to the return array's type, and it
  can itself be an array.

## What has not been checked

No Handbook vector suite exists for `XLOOKUP`, and no Handbook evidence record is attached to
this page. Nothing here claims that any implementation agrees with Excel. The one live-Excel
datum cited above comes from a named upstream OxFunc bug stream and is quoted so it can be
re-run, not offered as a Handbook measurement.

What the Handbook would probe first, and why:

1. **The `match_mode` × `search_mode` grid.** Sixteen combinations, over sorted, reverse-
   sorted and unsorted vectors, with and without duplicate keys. This is the cheapest
   experiment with the highest information content, because the two arguments are documented
   independently and nothing states how they compose.
2. **Wildcard mode under binary search** (`match_mode` `2`, `search_mode` `±2`) — the
   combination whose meaning the documentation does not define.
3. **Two-dimensional return arrays** with an array-valued `lookup_value`: what shape spills,
   and what happens when it cannot.
4. **Mismatched lengths** between `lookup_array` and `return_array`, in both orientations, to
   pin the error rather than repeat folklore.
5. **The omitted `lookup_value` behaviour** Microsoft mentions — returning blanks found in the
   lookup array — which is documented in one sentence and characterized nowhere.
6. **Blanks and errors inside `lookup_array`** in each of the four search modes.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| lookup vector | `lookup_array`; the searched sequence |
| return array | `return_array`; the aligned sequence values are drawn from |
| match mode | What counts as a match: exact, next-smaller, next-larger, wildcard |
| search mode | How the vector is traversed: forward, reverse, binary ascending, binary descending |
| supersedes | Covers the older function's use cases; the older name remains supported |

## Sources

- Microsoft, *XLOOKUP function* —
  <https://support.microsoft.com/en-us/office/xlookup-function-b7fd680e-6d10-43e6-84f9-88eae8bf5929>
  (syntax, argument table, the full `match_mode` and `search_mode` value tables and their
  defaults, and the `#N/A` and unsorted-binary-search notes). Retrieved for this page.
- Handbook `content/model/02-coercion-and-lifting.md` and `content/model/03-call-pipeline.md`.
- OxFunc bug stream `docs/bugs/streams/BUG-FUNC-006_lookup_selection_array_lookup_value_lifting_gap.md`
  — live Excel replay of multi-needle lookup shapes, 2026-04-08 and 2026-04-29.
- OxFunc bug stream `docs/bugs/streams/BUG-FUNC-040_approximate_match_blank_skipping_gap.md`
  — blank-skipping in ordered search paths.
- Handbook `data/functions/FUNC.XLOOKUP.json` (signature, arity, classification axes).
