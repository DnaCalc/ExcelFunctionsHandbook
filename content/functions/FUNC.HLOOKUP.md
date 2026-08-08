---
schema: efh.function-page/v1
function_id: FUNC.HLOOKUP
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
family: vhlookup_family
role_in_family: >-
  Searches the top row of a table and returns a value from a row below it; the horizontal twin
  of VLOOKUP, sharing one search kernel with the axes exchanged.
---

## What it computes

`HLOOKUP(lookup_value, table_array, row_index_num, [range_lookup])` searches the **top row** of
`table_array` for `lookup_value`, and returns the value in the same column from the row given by
`row_index_num`, counting the top row as row 1.

"Searches" means one of two genuinely different algorithms, chosen by `range_lookup`:

**Exact match (`range_lookup` = FALSE).** A linear scan of the top row for a value equal to
`lookup_value`; the first match wins. Microsoft documents that in this mode wildcards are
honoured in a text `lookup_value`: `?` matches any single character, `*` any run of characters,
and `~` escapes a literal `?` or `*`. If nothing matches, the result is `#N/A`.

**Approximate match (`range_lookup` = TRUE, or omitted).** The top row is assumed **sorted in
ascending order**, and the function finds the largest value that is less than or equal to
`lookup_value`. This is a range lookup in the literal sense — it is how tax bands, grade
boundaries and shipping tiers are expressed. If `lookup_value` is smaller than every value in
the top row, the result is `#N/A`.

The assumption in the second mode is not checked, and this is the function's defining hazard.
Microsoft documents that the values must be sorted ascending; on unsorted data the function does
not error, it simply returns something. What it returns depends on the search strategy the
implementation uses, which is why "approximate match on unsorted data" is a bug that survives
review — the answer looks like an answer.

## Arguments

| Argument | Meaning |
|---|---|
| `lookup_value` | The value sought in the top row. May be a value, a reference, or text — and, on the exact-match lane, wildcard text. |
| `table_array` | The table to search. The top row is the search axis; the returned value comes from a row of the same block. |
| `row_index_num` | Which row of `table_array` supplies the result, counting the top row as 1. |
| `range_lookup` | Logical. `TRUE` or omitted selects approximate match; `FALSE` selects exact match. |

The two positions that are routinely misunderstood:

**`row_index_num` is relative to `table_array`, not to the worksheet.** Row 1 is the top row of
the table — the one being searched — so a lookup that returns the search key itself uses
`row_index_num` 1. The Handbook's projection carries a curated note on this argument recording
that the value is truncated toward zero before validation.

**Omitting `range_lookup` selects approximate match.** The default is the dangerous mode. Almost
every accidental wrong answer from this function comes from an omitted fourth argument on
unsorted data. The projection's curated note on the argument states the same thing: omitted
means approximate-match mode.

## Result and edge cases

Return kind: whatever the located cell holds — number, text, logical, error, or a blank
normalized under the publication rules in
[the value universe](../model/01-value-universe.md).

`HLOOKUP` is reference-aware: the projection records
`arg_preparation_profile: RefsVisibleInAdapter`, so `table_array` arrives as a live reference
rather than as a materialized copy. That is what makes lookups over large ranges tractable —
the search can probe a sub-range instead of pulling every cell across the boundary. Note that
[the call pipeline](../model/03-call-pipeline.md) records selective dereference as a
deliberately function-local capability, not a general pipeline feature.

Edge cases specific to this function:

- **Duplicate keys.** On the exact lane the first match wins. On the approximate lane, which of
  several equal keys is found depends on the search strategy, and the Handbook has not
  established Excel's choice.
- **Mixed types in the search row.** Excel's ordering across numbers, text and logicals is a
  total order, but where the type boundaries fall governs what "largest value less than or
  equal" means on a mixed row. Not established here.
- **Text comparison** is case-insensitive in Excel's usual convention, and the collation used
  for the approximate lane's ordering may be locale-dependent. Not established here.
- **Empty cells in the search row** — skipped, matched by an empty lookup value, or treated as
  a zero — is a real question for approximate match over a partially filled table.
- **`lookup_value` as an array.** The projection records `lift_broadcast_profile: surface_native`
  — the function does its own lifting, if any — so whether a vector of lookup values spills a
  vector of results is a property of the function rather than of the dispatch layer.

## Errors

Microsoft's page documents the error surface:

| Condition | Result |
|---|---|
| No match found (either lane) | `#N/A` |
| `lookup_value` smaller than every value in the top row, approximate lane | `#N/A` |
| `row_index_num` less than 1 | `#VALUE!` |
| `row_index_num` greater than the number of rows in `table_array` | `#REF!` |

The `#VALUE!` / `#REF!` split on `row_index_num` is worth keeping straight: too small is a bad
argument, too large is a bad reference. That asymmetry is inherited from the C-API era and is
shared with `VLOOKUP`.

An error value in `lookup_value` propagates under the universal rule in
[coercion and lifting](../model/02-coercion-and-lifting.md). An error value sitting in the
*located* cell is simply the result — it is a value like any other.

## Relationships

- **`VLOOKUP`** is the same function with rows and columns exchanged, and shares its
  implementation module in the reference engine. Every statement on this page has a `VLOOKUP`
  counterpart, and the choice between them is a question about how your data is laid out, not
  about behaviour.
- **`XLOOKUP`** is the modern replacement for both. It searches in either direction, defaults to
  **exact** match, takes an explicit not-found argument instead of `#N/A`, and separates the
  search array from the return array so `row_index_num` disappears entirely. New code should use
  it; Microsoft retains `HLOOKUP` for workbook compatibility.
- **`INDEX` + `MATCH`** is the classic decomposition: `MATCH` finds the position, `INDEX`
  retrieves it. It removes the index-counting fragility and lets the search row sit anywhere in
  the table rather than being forced to the top.
- **`LOOKUP`'s array form** does something similar and is documented by Microsoft as provided
  chiefly for compatibility with other spreadsheet programs; `HLOOKUP` is the better-specified
  choice.
- **`MATCH`** alone carries the same three-way match-type argument and the same sorted-data
  requirement for its approximate modes, so the hazard described above is shared.

## Notes for implementers

- **The two lanes are two algorithms and must be tested as two functions.** Exact match is a
  linear scan with wildcard support; approximate match is an ordered search. Sharing one code
  path between them is where wildcard characters start leaking into the sorted lane, or where
  the sorted lane's early exit silently truncates an exact scan.
- **Do not validate sortedness.** Excel does not, and an implementation that errors on unsorted
  input will reject workbooks that Excel computes. The obligation is to match the observable
  answer, including on unsorted data — which means the search strategy itself is observable and
  cannot be freely chosen.
- **`row_index_num` truncates toward zero before validation**, per the Handbook's projected
  curated note on that argument. Order matters: truncate first, then range-check, or `0.5`
  and `1.5` will be classified differently than Excel classifies them.
- **Wildcard escaping.** `~` before `?` or `*` makes the character literal. A naive
  glob-to-regex translation that does not honour `~` will match the wrong cells and never error.
- **Selective dereference.** The reference-aware preparation exists so that a lookup over a
  large `table_array` need not materialize it. The shared model records this as function-local
  and not generalized, so an implementation should treat it as a permitted optimization within
  this function rather than as a pipeline guarantee.

## What has not been checked

No Handbook vector suite exists for `HLOOKUP`, and no Excel-comparison evidence record is
recorded for it. Nobody has checked this function's behaviour against Excel here.

The probes that would settle the most, in order:

1. **Approximate match on deliberately unsorted data.** This is the observable that pins the
   search strategy — binary, linear, or something else — and it is the single most valuable
   probe on this page, because everything else about the approximate lane follows from it.
   Probe with keys arranged so that a binary search and a linear scan give different answers.
2. **Duplicate keys on both lanes**, establishing which occurrence is returned.
3. **The full wildcard grammar**: `?`, `*`, `~?`, `~*`, `~~`, a wildcard on the approximate
   lane (where Microsoft documents them for the exact lane only), and a wildcard against
   numeric cells.
4. **Mixed-type search rows** — numbers, text, logicals, blanks and errors interleaved — under
   approximate match, to establish the ordering across type boundaries.
5. **`row_index_num` boundaries**: 0, negative, fractional either side of an integer, exactly the
   row count, and one beyond, confirming the documented `#VALUE!` / `#REF!` split and the
   truncation direction.
6. **Case and collation**: keys differing only in case, and keys whose ordering differs by
   locale.

Item 1 is the one whose answer would change this page's description of the function rather than
extend its edge-case list.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| exact lane | The `range_lookup` = FALSE path: equality scan with wildcard support |
| approximate lane | The `range_lookup` = TRUE (or omitted) path: largest value ≤ the key, sorted data assumed |
| search row | The top row of `table_array`, the only row searched |
| selective dereference | Probing part of a reference without materializing all of it |
| sorted-data assumption | Required by the approximate lane, never checked at runtime |

## Sources

- Microsoft, HLOOKUP function —
  <https://support.microsoft.com/en-us/office/hlookup-function-a3034eec-b719-4ba3-bb65-e1ad662ed95f>
  (the two match modes, the ascending-sort requirement for approximate match, wildcard support
  on exact match, and the `#N/A` / `#VALUE!` / `#REF!` conditions).
- Handbook `content/model/01-value-universe.md` (value kinds; error registry).
- Handbook `content/model/02-coercion-and-lifting.md` (error propagation; reference resolution).
- Handbook `content/model/03-call-pipeline.md` (`RefsVisibleInAdapter`;
  `KernelSignatureClass::LookupMatch`; selective dereference as a function-local capability).
- Handbook `data/functions/FUNC.HLOOKUP.json` (arity, curated per-argument notes on
  `row_index_num` truncation and the omitted-`range_lookup` default) and
  `data/presence/FUNC.HLOOKUP.json` (implementing module shared with `VLOOKUP`).
