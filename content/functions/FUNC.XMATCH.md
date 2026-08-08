---
schema: efh.function-page/v1
function_id: FUNC.XMATCH
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
family: xmatch
role_in_family: >-
  The modern position-finder: the search half of XLOOKUP, with match rule and traversal
  direction as independent arguments and an exact-match default.
---

## What it computes

`XMATCH` returns the 1-based position of a value within a range or array. Given a lookup value
`v` and a lookup array `L` of `n` entries, it selects an index `i ∈ 1..n` and returns `i`, or
`#N/A`.

What distinguishes it from `MATCH` is that the single ternary `match_type` flag is split into
two orthogonal arguments:

- **`match_mode`** — the predicate. Exact (default), exact-or-next-smallest,
  exact-or-next-largest, or wildcard.
- **`search_mode`** — the traversal. First-to-last (default), last-to-first, binary ascending,
  or binary descending.

That split is not cosmetic. In `MATCH`, choosing "nearest below" also chose "assume ascending
sort" and (in practice) a particular traversal; the three were welded together. In `XMATCH`
you can ask for an exact match searched backwards — which is how you find the *last*
occurrence of a repeated key, something `MATCH` cannot express at all. You can also ask for a
nearest-below match over a linearly scanned array, without committing to the sorted-array
premise that binary search requires.

The default combination is exact match, first-to-last. Its failure mode is `#N/A`: visible.
That is a deliberate reversal of `MATCH`'s default, whose failure mode is a plausible wrong
index.

## Arguments

Microsoft documents `XMATCH(lookup_value, lookup_array, [match_mode], [search_mode])`.

**`lookup_value`** — the value to locate.

**`lookup_array`** — the array or range to search.

**`match_mode`** — optional, default `0`:

| Value | Documented meaning |
|---|---|
| `0` | Exact match (default) |
| `-1` | Exact match or next smallest item |
| `1` | Exact match or next largest item |
| `2` | A wildcard match where `*`, `?` and `~` have special meaning |

**`search_mode`** — optional, default `1`:

| Value | Documented meaning |
|---|---|
| `1` | Search first-to-last (default) |
| `-1` | Search last-to-first |
| `2` | Binary search, `lookup_array` sorted ascending |
| `-2` | Binary search, `lookup_array` sorted descending |

The position most often misused is the third. `XMATCH(v, L, -1)` is *not* a reverse search — it
is "exact or next smallest, searched forwards". A reverse search is `XMATCH(v, L, 0, -1)`.
Both spellings contain a `-1` and neither is a type error, so the mistake survives entry and
shows up as a subtly wrong index.

Note also that `match_mode` `±1` does **not** by itself require a sorted array — only
`search_mode` `±2` does. Nearest-below over an unsorted array is a legal, meaningful request
under linear traversal. This is a genuine expressive gain over `MATCH` and is easy to overlook.

## Result and edge cases

The return kind is a number: an index relative to `lookup_array`, not to the worksheet.

- **Array-valued `lookup_value`.** Excel lifts `XMATCH` over an array of needles and spills
  one index per needle, with element-local `#N/A`. Live Excel observed on 2026-04-08 and
  recorded in OxFunc's `BUG-FUNC-006`: `=XMATCH({1,2,3},{2,4,6,8})` → `{#N/A,1,#N/A}`, and the
  composed row `=SUM(FILTER({1,2,3,4,5},ISNUMBER(XMATCH({1,2,3,4,5},{2,4,6,8}))))` → `6`.
- **Blanks under binary search are skipped.** Live Excel on 2026-06-18, recorded in OxFunc's
  `BUG-FUNC-040`, gives `=XMATCH(2.5,{1,<blank>,2,3},1,2)` → `4` (the entry `3`): the blank is
  passed over and the returned position is the *original* index, not an index into the
  compacted subsequence.
- **Exact numeric comparison is strict.** `BUG-FUNC-004`, signed off against live Excel 16.0
  build 20026 on 2026-06-20, records `=XMATCH(0.1+0.2,{0.3},0)` → `#N/A`, deliberately
  contrasted with the tolerant comparison families where `=0.1+0.2=0.3` is `TRUE`.
- **Binary search on unsorted input** returns something rather than erroring. Microsoft states
  only that results are invalid; the behaviour is therefore undefined by documentation, not
  defined-and-bad.
- Errors, blanks and omitted arguments otherwise follow
  [coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

Documented: no error is named beyond the ordinary `#N/A` for a value not found, and a warning
that binary search over an unsorted array yields invalid results — wrong answers, not an error
value.

Error values arriving in `lookup_value` propagate. What an error *inside* `lookup_array` does
is not documented; `BUG-FUNC-040` names the error-cell lane as an explicitly separate, open
probe, which is a fair signal that nobody has pinned it.

## Relationships

- **`XMATCH` supersedes `MATCH`.** Same job, orthogonal controls, exact-match default, plus
  reverse and binary search. Microsoft has not deprecated `MATCH`, which remains supported.
- **`XLOOKUP`** is `XMATCH` plus the return step, sharing the `match_mode` and `search_mode`
  vocabulary exactly. `INDEX(R, XMATCH(v, L, m, s))` and `XLOOKUP(v, L, R, , m, s)` are the
  same query written two ways; the difference is that `XLOOKUP` also carries `if_not_found`.
- **`INDEX`** is the usual consumer of `XMATCH`'s result.
- **`VLOOKUP`/`HLOOKUP`** contain a `MATCH`-shaped search internally; `XMATCH` is the modern
  standalone form of that search.

## Notes for implementers

- `match_mode` and `search_mode` are independent inputs to one search, and the implementation
  has to decide what each of the sixteen combinations means. Wildcard matching under binary
  search is the combination with no documented meaning; it should be a recorded decision, not
  an accident of code structure.
- Reverse exact search must return the *last* matching index, which means it cannot be
  implemented as forward search over a reversed array without mapping the index back.
- The same index-mapping obligation applies to blank skipping under binary search: the search
  runs over the comparable subsequence, and the chosen position must be translated back to the
  original 1-based index. `BUG-FUNC-040` records exactly this repair.
- Exact mode is numerically strict while much of Excel's comparison surface is tolerant; the
  two comparison helpers must not be shared.
- `lookup_value` must admit arrays and spill elementwise; the reference engine records a period
  in which it did not, and rejected array needles with `#VALUE!`.

## What has not been checked

No Handbook vector suite exists for `XMATCH`, and no Handbook evidence record is attached to
this page. The Excel observations quoted above come from named upstream OxFunc bug streams and
are cited so they can be re-run; they are not Handbook measurements, and nothing here asserts
that any implementation agrees with Excel.

First probes:

1. **The full `match_mode` × `search_mode` grid** over sorted, reverse-sorted, unsorted and
   duplicate-bearing arrays. Sixteen combinations, each documented only on its own axis.
2. **Wildcard mode under binary search** — the undefined corner.
3. **Duplicate keys under `search_mode` `-1`** versus `1`, to confirm that reverse search
   really means last-occurrence for every `match_mode`.
4. **The cross-type ordering** used by `match_mode` `±1` when the array mixes numbers, text
   and logicals — and whether it matches the order Microsoft documents for `MATCH`.
5. **Blanks and errors inside `lookup_array`** under each of the four search modes.
6. **Two-dimensional lookup arrays**, which the documentation does not describe.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| match mode | The predicate: exact, next-smallest, next-largest, wildcard |
| search mode | The traversal: forward, reverse, binary ascending, binary descending |
| reverse exact search | `match_mode` `0`, `search_mode` `-1`; finds the last occurrence |
| index mapping | Translating a position in the comparable subsequence back to the original array index |

## Sources

- Microsoft, *XMATCH function* —
  <https://support.microsoft.com/en-us/office/xmatch-function-d966da31-7a6b-4a13-a1c6-5a33ed6a0312>
  (syntax, the `match_mode` and `search_mode` value tables and defaults, and the
  unsorted-binary-search warning). Retrieved for this page.
- Microsoft, *MATCH function* —
  <https://support.microsoft.com/en-us/office/match-function-e8dffd45-c762-47d6-bf89-533f4a37673a>
  (the superseded function and its documented cross-type sort order).
- Handbook `content/model/02-coercion-and-lifting.md` and `content/model/03-call-pipeline.md`.
- OxFunc bug streams `BUG-FUNC-006` (array-valued lookup values, live Excel 2026-04-08),
  `BUG-FUNC-040` (blank skipping and index mapping, live Excel 16.0 build 20026, 2026-06-18),
  and `BUG-FUNC-004` (strict-versus-tolerant numeric comparison, live Excel 16.0 build 20026,
  2026-06-20), under `docs/bugs/streams/` in OxFunc.
- Handbook `data/functions/FUNC.XMATCH.json` (signature, arity, classification axes).
