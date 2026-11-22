---
schema: efh.function-page/v1
function_id: FUNC.MATCH
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
family: match_fn
role_in_family: >-
  The classic position-finder: returns the 1-based index of a value within a one-dimensional
  range or array, under one of three ordered or exact match rules.
---

## What it computes

`MATCH` returns a **position, not a value**. Given a lookup value `v` and a one-dimensional
lookup array `L` with entries `L[1] … L[n]`, it returns an index `i ∈ 1..n` chosen by the
`match_type` rule, or `#N/A`.

The three rules, as Microsoft documents them:

| `match_type` | Rule | Sort requirement |
|---|---|---|
| `1` (default) | the largest value that is less than or equal to `v` | `L` ascending |
| `0` | the first value exactly equal to `v` | none |
| `-1` | the smallest value that is greater than or equal to `v` | `L` descending |

Two of the three rules therefore carry a caller obligation that Excel does not check. The
`1` and `-1` rules are *ordered* searches: they are only meaningful over a sorted array, and
Excel raises no error when the array is not sorted — it returns a deterministic index computed
under a false premise.

Microsoft's documentation also states the sort order it means, which is unusually explicit and
worth quoting structurally rather than paraphrasing: ascending order is given as
`-2, -1, 0, 1, 2, …, A-Z, FALSE, TRUE`. That is a **cross-type total order**: all numbers
precede all text, text precedes `FALSE`, and `FALSE` precedes `TRUE`. This is the ordering an
ordered `MATCH` compares against when the array mixes types, and it is the same ordering that
governs the older approximate lookups.

The index returned is relative to `L`, not to the worksheet. `MATCH(v, A5:A9, 0)` returning
`2` means `A6` — the caller adds the offset. That is why `MATCH` is almost always seen inside
`INDEX`, which does the arithmetic.

## Arguments

**`lookup_value`** — the value to find. May be a number, text, a logical, or a reference to
one. In exact mode (`match_type` `0`) a text lookup value may contain wildcards: `?` matches
any single character, `*` matches any run of characters, and `~` escapes a literal `?`, `*` or
`~`. Wildcards are documented only for `match_type` `0`.

**`lookup_array`** — the sequence to search. Conventionally a single row or single column.
What `MATCH` does with a genuinely two-dimensional array is not stated in the documentation
and is not asserted here.

**`match_type`** — optional, **defaults to `1`**. This default deserves the same suspicion as
`VLOOKUP`'s omitted fourth argument, for the same reason: omitting it selects an *ordered*
search whose failure on unsorted data is a plausible wrong index rather than an error. A
reader who writes `MATCH(v, L)` meaning "where is `v`" has written "where is the last entry not
exceeding `v`, assuming `L` is sorted ascending". Pass `0` explicitly when you mean exact.

## Result and edge cases

The return kind is a number: a 1-based index into `lookup_array`.

- **Case-insensitivity.** Microsoft states plainly that `MATCH` does not distinguish uppercase
  from lowercase. Exact mode is therefore *not* string identity.
- **Array-valued `lookup_value`.** Excel lifts `MATCH` over an array of needles and spills one
  index per needle. Live Excel observed on 2026-04-08 and recorded in OxFunc's
  `BUG-FUNC-006`: `=MATCH({1,2,3},{2,4,6,8},0)` → `{#N/A,1,#N/A}`. Element failures stay
  element-local.
- **Blank cells in the lookup array are skipped by the ordered searches**, not treated as
  match failures. This is what makes the canonical last-value idiom `=MATCH(9.99E307,A:A,1)`
  work over a column with gaps. Live Excel on 2026-06-18, recorded in OxFunc's
  `BUG-FUNC-040`, over `A1:A5` = `[10, 20, blank, 40, 50]`: `MATCH(9.99E307,A1:A5,1)` = `5`
  and `MATCH(35,A1:A5,1)` = `2`.
- **Numeric comparison in exact mode is exact.** OxFunc's `BUG-FUNC-004`, signed off against
  live Excel 16.0 build 20026 on 2026-06-20, records `=MATCH(0.1+0.2,{0.3},0)` → `#N/A` as a
  deliberate contrast against the *tolerant* comparison families (`=0.1+0.2=0.3` is `TRUE`,
  and `COUNTIF` agrees with the tolerant side). `MATCH`'s exact lane sits on the strict side of
  that split. Whether the ordered lanes sit on the same side is not recorded.
- Errors, empty cells and omitted arguments follow the shared rules in
  [coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

Documented: `#N/A` when no match is found. Under `match_type` `1` that includes the case where
`v` is smaller than every entry; under `-1`, where `v` is larger than every entry.

Error values arriving in `lookup_value` propagate. What an error value *inside* the lookup
array does — propagate, skip, or abort the search — is not documented and the Handbook has not
verified it; OxFunc's `BUG-FUNC-040` explicitly separates the error-cell lane from the
blank-skipping lane it did verify, which is a fair signal that the error case is genuinely
open.

## Relationships

- **`XMATCH` supersedes `MATCH`.** `XMATCH` keeps the position-returning shape but splits the
  single `match_type` flag into an explicit `match_mode` and `search_mode`, defaults to exact
  match rather than ordered match, and adds reverse and binary search. For new work `XMATCH`
  is the better choice. Microsoft has not deprecated `MATCH`; it remains supported and remains
  what existing workbooks contain.
- **`INDEX`** is `MATCH`'s usual partner: `INDEX(R, MATCH(v, L, 0))` is the classic
  exact-lookup composition, and it is what `XLOOKUP` packages into one call.
- **`VLOOKUP` / `HLOOKUP`** contain `MATCH`'s search as their first half; they differ only in
  performing the column (or row) projection themselves.
- **`LOOKUP`** shares the ordered-search rule with `match_type` `1` and has no exact mode.
- Readers confuse `MATCH` with `FIND` and `SEARCH` (which locate a substring inside one text
  value, not a value inside a list).

## Notes for implementers

- Three `match_type` values are two different algorithms: an unordered scan for `0`, and an
  ordered search for `±1`. Sharing one code path between them is how implementations acquire
  the wrong behaviour on unsorted input.
- Blanks are skippable candidates in the ordered paths. OxFunc's `BUG-FUNC-040` records that
  aborting on the first blank turns `MATCH(x, range, 1)` over any range with one gap into
  `#N/A` — and that the same abort propagates into `VLOOKUP`/`HLOOKUP` approximate and
  `XMATCH` binary modes, because they share the candidate-collection helper. When the ordered
  search runs over a filtered subset, the chosen position must be mapped back to the original
  1-based index.
- Exact mode is case-insensitive but numerically strict. Those two facts pull in opposite
  directions and are easy to implement inconsistently.
- Wildcard handling belongs to exact mode only, and `~` escaping has to be handled before
  pattern compilation, not after.
- `lookup_value` must admit arrays and spill elementwise.

## What has not been checked

No Handbook vector suite exists for `MATCH`, and no Handbook evidence record is attached to
this page. The live-Excel observations above are quoted from named upstream OxFunc bug streams
so a reader can re-run them; they are not Handbook measurements, and nothing here says any
implementation agrees with Excel.

The probes that would settle the most:

1. **Ordered search on unsorted arrays.** Every permutation of a short array, probed at every
   key and both midpoints between keys, under `match_type` `1` and `-1`. This characterizes
   the search procedure, which the documentation deliberately leaves undefined.
2. **The documented cross-type order under test.** Arrays mixing numbers, text, `FALSE` and
   `TRUE`, probed with keys of each type, to confirm that `-2 … A-Z, FALSE, TRUE` is what the
   ordered comparison actually implements — including where blanks sit in that order.
3. **Errors inside the lookup array**, in all three modes.
4. **Two-dimensional lookup arrays**, which the documentation does not describe.
5. **Wildcards**: whether they are active under `match_type` `±1`; `~` escaping; a lookup
   value that is only wildcards.
6. **Near-equality in the ordered lanes**, against the strict/tolerant split that
   `BUG-FUNC-004` pins for exact mode.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `MATCH` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-MATCH-001: MATCH and VLOOKUP require equal UTF-16 length in non-wildcard exact mode and count wildcard ? in UTF-16 units

*Quirk; measured; recorded 2026-11-22.* Applies to MATCH, VLOOKUP.

In Excel 16.0.20527.20088, MATCH(..., 0) and VLOOKUP(..., FALSE) without wildcards match only
text of the same UTF-16 code-unit length: "SS" does not match "ß" and "ae" does not match "æ",
whereas "e"&UNICHAR(769) (length 2) matches "e"+U+0301 in D2 rather than precomposed "é" in D1.
In wildcard mode, "?" matches one UTF-16 code unit, so surrogate-pair emoji "😀" matches "??"
rather than "?". Both functions read a blank or omitted lookup_value as 0 and reject lookup
strings longer than 255 UTF-16 code units with #VALUE!.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=MATCH("SS",D5:D16,0)` | as typed | `#N/A` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VLOOKUP("ae",D7:D16,1,FALSE)` | as typed | `#N/A` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=MATCH("??",{"😀";"a";"ab"},0)` | as typed | `1.0` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=MATCH("?",{"😀";"a";"ab"},0)` | as typed | `2.0` (`0x4000000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Measured in Excel 16.0.20527.20088 in dczsn1_collation (rows 107-116) and
fn1_collation (rows 217, 219), and matched across 3,373/3,373 MATCH and 3,680/3,680 VLOOKUP
full-bank rows plus 13/13 MATCH pinned rows.

Evidence:

- DnaCalc `crates/calc-func/src/catalogue/lookup.rs`, lines 17-37 at `95eb0cb2ef` — Exact length rule vs wildcard UTF-16 matching and bisection semantics
- DnaCalc `crates/calc-func/witness-status.tsv`, MATCH; VLOOKUP at `95eb0cb2ef` — 3,373/3,373 MATCH and 3,680/3,680 VLOOKUP full-bank rows matched
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dczsn1_collation.capture.json`, sheet dczsn1_collation row 109 column 4 at `95eb0cb2ef` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dczsn1_collation.capture.json`, sheet dczsn1_collation row 116 column 4 at `95eb0cb2ef` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/fn1_collation.capture.json`, sheet fn1_collation row 217 column 4 at `95eb0cb2ef` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/fn1_collation.capture.json`, sheet fn1_collation row 219 column 4 at `95eb0cb2ef` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| ordered search | `match_type` `1` or `-1`; requires a sorted array, unchecked |
| exact mode | `match_type` `0`; first equal entry, wildcards active, case-insensitive |
| cross-type order | Microsoft's documented ascending order: numbers, then text, then `FALSE`, then `TRUE` |
| last-value idiom | `=MATCH(9.99E307,A:A,1)`, which depends on ordered search skipping blanks |

## Sources

- Microsoft, *MATCH function* —
  <https://support.microsoft.com/en-us/office/match-function-e8dffd45-c762-47d6-bf89-533f4a37673a>
  (syntax, the three `match_type` rules and their sort requirements, the explicit ascending
  order including text and logicals, case-insensitivity, wildcard support, and `#N/A`).
  Retrieved for this page.
- Handbook `content/model/02-coercion-and-lifting.md` and `content/model/03-call-pipeline.md`.
- OxFunc bug stream `docs/bugs/streams/BUG-FUNC-006_lookup_selection_array_lookup_value_lifting_gap.md`
  — live Excel array-valued lookup values, 2026-04-08.
- OxFunc bug stream `docs/bugs/streams/BUG-FUNC-040_approximate_match_blank_skipping_gap.md`
  — blank skipping in ordered search; live Excel 16.0 build 20026, 2026-06-18.
- OxFunc bug stream `docs/bugs/streams/BUG-FUNC-004_numeric_comparison_tolerance_family_split.md`
  — the strict/tolerant comparison split; live Excel 16.0 build 20026, 2026-06-20.
- Handbook `data/functions/FUNC.MATCH.json` (signature, arity, classification axes).
