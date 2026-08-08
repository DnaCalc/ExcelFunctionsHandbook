---
schema: efh.function-page/v1
function_id: FUNC.VLOOKUP
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - Arguments
  - The omitted fourth argument
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
  - Page vocabulary
  - Sources
family: vhlookup_family
role_in_family: >-
  The vertical member of the classic two-function lookup pair: searches down the first column
  of a table and returns a value from a fixed column offset in the matched row.
---

## What it computes

`VLOOKUP` is a *keyed row selection* over a rectangular table, followed by a fixed column
projection. Given

- a lookup value `v`,
- a table `T` with rows `1..r` and columns `1..c`,
- a column index `k`,
- and a mode flag,

`VLOOKUP` searches **only the first column of `T`** — `T[1,1] … T[r,1]` — for a row index `i`,
then returns `T[i,k]`. Two things about that sentence carry most of the function's real-world
failure surface, and both are structural rather than numerical:

1. **The search column is fixed at column 1 of `T`.** It is not a separate argument. The
   caller chooses it implicitly by where the range starts, which is why inserting a column to
   the left of a table changes what `VLOOKUP` searches.
2. **`k` is an absolute index within `T`, not an offset from the search column.** `k = 1`
   returns the search column itself; `k = 2` returns the column immediately to its right. `k`
   is counted from `T`'s own left edge, so it is invalidated by any column inserted inside
   `T` — the formula keeps working and silently returns a different column.

The choice of `i` depends on the mode flag, and the two modes are genuinely different
algorithms rather than two settings of one algorithm:

- **Exact mode** (`range_lookup` = `FALSE`). `i` is the smallest row index whose first-column
  value equals `v`. Equality is Excel's worksheet equality — case-insensitive for text, and,
  for numbers, the tolerant near-equality Excel applies in comparison contexts rather than raw
  IEEE-754 bit equality. If no row matches, the result is `#N/A`. The scan direction (top to
  bottom, first match wins) is what makes the result deterministic when the key repeats.
- **Approximate mode** (`range_lookup` = `TRUE`, or omitted). `i` is the row whose
  first-column value is the **largest value not exceeding `v`**, under Excel's ordering of
  values — and this answer is only meaningful if the first column is sorted in ascending
  order. Microsoft documents the ascending-sort requirement as a caller obligation. It is a
  precondition, not a checked constraint: Excel does not validate it and raises no error when
  it is violated.

Approximate mode is the older and, historically, the primary mode: it is what makes `VLOOKUP`
a *bracket lookup*, the tool for tax bands, shipping tiers, grade boundaries, and any table
whose keys are the lower bounds of intervals. Used that way it is exactly the right function.
Used as a substitute for exact lookup, it is the single most expensive default in the product.

## Arguments

**`lookup_value`** — the key to search for. Numbers, text and logicals are all admissible. In
exact mode, `*` and `?` in a text lookup value are documented to act as wildcards (any run of
characters, and any single character respectively), with `~` as the escape; that is a real
behavioural difference from a plain equality test, and it means a lookup key that legitimately
contains `*` or `?` must be escaped.

**`table_array`** — the table. May be a reference or an array. Only its first column is
searched; the remaining columns exist solely as return material. A `table_array` whose left
edge is not where the caller thinks it is produces wrong answers with no diagnostic.

**`col_index_num`** — the 1-based column position **within `table_array`**. The expected
handling of a non-integer index is truncation toward zero before validation, which is how the
reference engine declares it; the Handbook has not confirmed that against Excel.

**`range_lookup`** — the mode flag. Optional. See below.

## The omitted fourth argument

This is the part of `VLOOKUP` worth stating at length, because the cost is asymmetric and
therefore invisible.

**Omitting `range_lookup` selects approximate match.** `VLOOKUP(v, T, k)` and
`VLOOKUP(v, T, k, TRUE)` are the same call. A reader who writes the three-argument form
intending "find `v` in the table" has instead written "find the largest key not exceeding `v`,
trusting that the first column is sorted ascending".

Why this default is expensive is not that it is wrong — it is the right default for the
bracket-lookup use it was designed for — but that **its failure mode is silent**:

| Situation | Exact mode (`FALSE`) | Approximate mode (omitted / `TRUE`) |
|---|---|---|
| Key present | the matching row | the matching row |
| Key absent | `#N/A` — visible | some other row's value — invisible |
| Column not sorted | irrelevant | an arbitrary row's value — invisible |

An exact-match miss announces itself as `#N/A` and stops the spreadsheet. An approximate-match
miss returns a plausible number of the right type, in the right format, in the right cell. It
propagates. It is discovered, if ever, by someone reconciling totals months later.

The unsorted case deserves separate emphasis. Because Microsoft documents only that
approximate match on unsorted data "may not return the correct value", the behaviour on
unsorted input is *undefined by the documentation* rather than defined-and-undesirable. What
is observable is that it is not random: for a fixed table it is stable and repeatable, which is
exactly what makes it survive testing. The shape of those wrong answers is consistent with a
bisection over the search column, but the Handbook has not pinned the search procedure and
does not assert one.

Two practical consequences the Handbook is willing to state plainly:

1. If you mean exact lookup, pass `FALSE` (or `0`) explicitly, every time. There is no
   defensible reason to rely on the default when the intent is exact.
2. If you mean bracket lookup, pass `TRUE` explicitly and say so in a comment, so the next
   reader knows the sorted-column precondition is load-bearing rather than accidental.

## Result and edge cases

The return kind is whatever the selected cell holds — number, text, logical, error, or the
zero that a referenced empty cell publishes. `VLOOKUP` performs no conversion on the value it
returns; it is a selector, not a coercer. The general rules for how empty cells, omitted
arguments and error values reach a function are in
[the call pipeline](../model/03-call-pipeline.md) and
[coercion and lifting](../model/02-coercion-and-lifting.md); only the function-specific parts
are stated here.

- **Array-valued `lookup_value`.** A modern Excel lifts `VLOOKUP` over an array of lookup
  values and spills one result per needle. Live Excel observed on 2026-04-08 and recorded in
  OxFunc's bug stream `BUG-FUNC-006`:
  `=VLOOKUP({1,2,3},{2,20;4,40;6,60;8,80},2,FALSE)` → `{#N/A,20,#N/A}`. Element failures stay
  element-local — a missing needle contributes `#N/A` to its own cell without collapsing the
  spill.
- **Blank cells in the search column.** Excel's approximate/binary path *skips* blanks rather
  than aborting on them; the canonical last-value idiom `=MATCH(9.99E307,A:A,1)` depends on
  it. The same skipping is documented for the `VLOOKUP` approximate lane in OxFunc's
  `BUG-FUNC-040`, whose conceptual table gives Excel
  `=VLOOKUP(2.9,{1,10;2,20;<blank>,99;3,30},2)` → `20`. That record's live-Excel verification
  on 2026-06-18 covered the `MATCH` lanes directly; the `VLOOKUP` row is stated there
  analytically, and the Handbook treats it as expected-but-unverified.
- **Repeated keys.** Exact mode returns the first match scanning downward. Approximate mode's
  choice among equal keys is not something the Handbook has pinned.
- **Mixed types in the search column.** Approximate mode needs a total order over values, so a
  column mixing numbers, text and logicals forces Excel's cross-type ordering into view.
  Microsoft states such an order on the `MATCH` page — ascending means
  `-2, -1, 0, 1, 2, …, A-Z, FALSE, TRUE`, so all numbers precede all text, and text precedes
  the logicals. Whether `VLOOKUP`'s approximate lane uses that same order is expected but not
  documented on `VLOOKUP`'s own page, and the Handbook has not verified it.

## Errors

As documented by Microsoft (see Sources):

| Error | Documented condition |
|---|---|
| `#N/A` | Exact mode and no matching key; or approximate mode where `lookup_value` is smaller than every key in the first column |
| `#REF!` | `col_index_num` exceeds the number of columns in `table_array` |
| `#VALUE!` | `col_index_num` less than 1 |

Error values arriving in `lookup_value` propagate rather than being searched for, which is the
universal coercion rule rather than anything special to `VLOOKUP`. The Handbook has not
verified what an error value *inside the search column* does to either mode.

## Relationships

- **`HLOOKUP`** is the same function transposed: it searches the first *row* and returns from
  a row offset. The two share one implementation module in the reference engine, which is a
  reasonable signal that they are one algorithm with an axis parameter.
- **`XLOOKUP` supersedes `VLOOKUP` and `HLOOKUP`.** `XLOOKUP` separates the lookup vector from
  the return vector (so no fragile column index), defaults to **exact** match, can search
  backwards, offers an `if_not_found` value instead of `#N/A`, and can return to the left of
  the search column. For new work, `XLOOKUP` is the function to reach for. Microsoft has not
  deprecated `VLOOKUP` — it remains fully supported, and it remains what tens of millions of
  existing workbooks contain.
- **`MATCH`** is `VLOOKUP`'s search half without the projection half: it returns the position
  `i` rather than `T[i,k]`. The `INDEX(..., MATCH(...))` composition is the classic way to get
  `VLOOKUP`'s effect with an explicit return range, and it is what `XLOOKUP` packages.
- **`LOOKUP`** is the older, weaker ancestor: vector and array forms, approximate match only.
- Readers confuse `VLOOKUP` with `FILTER` (which returns *all* matching rows rather than one)
  and with `XMATCH` (position, not value).

## Notes for implementers

- The two modes are different search procedures over the same data, not one procedure with a
  comparison tweak. An implementation that treats approximate match as "exact match, else
  nearest" will diverge on unsorted input, which is precisely the case real workbooks hit.
- Numeric comparison in the tolerant lanes is not raw IEEE-754 equality. OxFunc's
  `BUG-FUNC-004` records a shared truncation-style 15-significant-digit comparison helper for
  Excel's tolerant comparison families, signed off against live Excel 16.0 build 20026 on
  2026-06-20 — and records the deliberate contrast that `MATCH(0.1+0.2,{0.3},0)` stays exact
  and yields `#N/A`. Which side of that split `VLOOKUP`'s two modes fall on is a question the
  Handbook has not settled.
- Blanks in the search column are skippable candidates in the approximate path, not aborts.
  Getting this wrong turns a common trailing-blank range into a whole-formula `#N/A`.
- `lookup_value` must be admitted as an array, with the result spilling elementwise. This is
  easy to miss when the scalar lane is written first.
- `table_array` arrives reference-bearing. A `VLOOKUP` over a full-column reference should not
  need to materialize a million rows to answer; whether selective dereference is available is
  an engine question, and the reference engine records it as an open design decision rather
  than a general capability.

## What has not been checked

No Handbook vector suite exists for `VLOOKUP`, and no Handbook evidence record is attached to
this page. Nothing here is a statement that any implementation agrees with Excel. The
observations quoted above are Excel observations recorded in named upstream OxFunc bug
streams, cited so that a reader can go and re-run them; they are not Handbook measurements.

The probes that would settle the most, in the order the Handbook would run them:

1. **Approximate mode on unsorted first columns.** Fix a small table, permute its first
   column, and record the returned row for every permutation and every probe key. This is the
   experiment that turns "undefined by documentation" into a characterized function, and it is
   the one that would identify the search procedure.
2. **Cross-type ordering in approximate mode.** A first column mixing numbers, text, logicals,
   blanks and errors, probed with keys of each type. Excel needs a total order here and nobody
   has published it.
3. **`col_index_num` boundaries.** `0`, negative, non-integer, exactly `c`, `c + 1`, text that
   parses as a number, and an array-valued index.
4. **Wildcards across modes.** Whether `*`, `?` and `~` are active in approximate mode as well
   as exact, and what a lookup key containing a literal `~` does.
5. **Errors inside the search column** in each mode — propagate, skip, or abort.
6. **Numeric near-equality.** `VLOOKUP(0.1+0.2, {0.3,1}, 2, FALSE)` against the tolerant/exact
   split recorded in `BUG-FUNC-004`.

Until those exist, the honest summary is: `VLOOKUP`'s exact mode is well understood and its
approximate mode is documented only by its precondition. Nobody has published a
characterization of what it does when that precondition fails.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| exact mode | `range_lookup` = `FALSE`; first equal key wins, otherwise `#N/A` |
| approximate mode | `range_lookup` = `TRUE` or omitted; largest key not exceeding the lookup value, sorted column assumed |
| bracket lookup | Using approximate mode deliberately, with keys as interval lower bounds |
| search column | Column 1 of `table_array` — the only column searched |
| `RefsVisibleInAdapter` | Argument-preparation profile: the function sees live references (call-pipeline chapter) |

## Sources

- Microsoft, *VLOOKUP function* —
  <https://support.microsoft.com/en-us/office/vlookup-function-0bbc8083-26fe-4963-8ab8-93a18ad188a1>
  (signature, argument meanings, the approximate-match sort requirement, and the documented
  error conditions). Retrieval for this page was blocked by the upstream host; the documented
  behaviour above is stated as documented behaviour and should be re-checked against the page.
- Microsoft, *XLOOKUP function* —
  <https://support.microsoft.com/en-us/office/xlookup-function-b7fd680e-6d10-43e6-84f9-88eae8bf5929>
  (the supersession relationship and `XLOOKUP`'s exact-match default).
- Handbook `content/model/02-coercion-and-lifting.md` and `content/model/03-call-pipeline.md`
  (error propagation, array lifting, reference-aware argument preparation).
- OxFunc bug stream `docs/bugs/streams/BUG-FUNC-006_lookup_selection_array_lookup_value_lifting_gap.md`
  — live Excel observations of array-valued lookup values, dated 2026-04-08.
- OxFunc bug stream `docs/bugs/streams/BUG-FUNC-040_approximate_match_blank_skipping_gap.md`
  — blank-skipping in the approximate/binary path; live Excel 16.0 build 20026, 2026-06-18.
- OxFunc bug stream `docs/bugs/streams/BUG-FUNC-004_numeric_comparison_tolerance_family_split.md`
  — the tolerant-versus-exact numeric comparison split; live Excel 16.0 build 20026, 2026-06-20.
- Handbook `data/functions/FUNC.VLOOKUP.json` (signature, arity, classification axes).
