---
schema: efh.function-page/v1
function_id: FUNC.ADDRESS
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
family: reference_metadata_family
role_in_family: >-
  Builds a cell address as text from numeric coordinates; the family's only member that
  manufactures reference syntax rather than reading a reference's metadata.
---

## What it computes

`ADDRESS` is a formatter, not a lookup. Given a row number and a column number it renders the
**text** of a cell address in the style you ask for. It does not read the cell, it does not
produce a reference value, and it never touches the grid — the row and column numbers may point
far outside any sheet's bounds and `ADDRESS` will still render an address for them, subject to
the admissible ranges below.

The rendered string is assembled from four independent decisions:

1. whether the row part carries a `$` (absolute) or not (relative),
2. whether the column part carries a `$`,
3. whether the address is written in A1 notation (`$D$5`) or R1C1 notation (`R5C4`), and
4. whether a sheet qualifier is prefixed (`Sheet2!$D$5`).

Decisions 1 and 2 are jointly encoded by `abs_num`; decision 3 by `a1`; decision 4 by
`sheet_text`. The result is `Text` under the value-universe chapter's kinds — the same kind you
would get from `"$D$5"` typed as a literal. Nothing downstream treats it as a reference unless
you pass it through `INDIRECT`.

## Arguments

| Argument | Meaning |
|---|---|
| `row_num` | The row coordinate. Required. |
| `column_num` | The column coordinate. Required. |
| `abs_num` | Which parts are absolute. Microsoft documents `1` (or omitted) = absolute row and column, `2` = absolute row, relative column, `3` = relative row, absolute column, `4` = both relative. |
| `a1` | Reference style. Microsoft documents a logical: `TRUE` or omitted = A1 style, `FALSE` = R1C1 style. |
| `sheet_text` | Text naming the worksheet (or external workbook and worksheet) to prefix. Omitted means no qualifier at all — not "the current sheet spelled out". |

The argument that is most often misread is `abs_num`. It is not a boolean and it is not a
bitmask you can reason your way into: the mapping from 1–4 to the four `$` combinations is an
arbitrary enumeration you have to look up, and the "natural" reading (that 1 means relative)
is backwards. The second most-misread is `sheet_text`: it is *your* text, inserted into the
result, so quoting for sheet names containing spaces or punctuation is a question about how
Excel renders the qualifier, not something you can assume `ADDRESS` handles for you.

## Result and edge cases

Return kind: `Text`. The general argument-preparation and coercion rules are the shared ones —
see [the call pipeline](../model/03-call-pipeline.md) and
[coercion and lifting](../model/02-coercion-and-lifting.md); `ADDRESS` is prepared under
`ValuesOnlyPreAdapter`, so it never sees a live reference, only numbers, logicals and text that
coerce to numbers.

Specific to this function:

- **`ADDRESS` lifts over arrays on every one of its five argument positions.** The projected
  axis is `by_index_scalar_array_lift(positions=0|1|2|3|4)`. Practically: hand it a column of
  row numbers and a row of column numbers and you get a spilled grid of address strings. This
  is a real, load-bearing property for anyone building address tables, and it is why `ADDRESS`
  appears in the pipeline chapter's list of by-index lifted functions.
- **Out-of-sheet coordinates.** Whether a row number beyond the sheet's row count renders,
  errors, or wraps is a boundary the Handbook has not established for Excel.
- **Fractional coordinates.** Whether `row_num` is truncated, rounded, or rejected is likewise
  not established here.

## Errors

Microsoft's documentation for `ADDRESS` describes the admissible values of `abs_num` but does
not enumerate an error table on the page as a whole. The conditions the Handbook expects to be
error-producing — non-positive coordinates, an `abs_num` outside 1–4 — are stated here as
*expected* and are on the unchecked list below rather than asserted. `#VALUE!` is the
conventional code for a coercion or domain failure in this family
([value universe](../model/01-value-universe.md)).

An error value arriving in any argument propagates by the universal rule in
[coercion and lifting](../model/02-coercion-and-lifting.md): coercion never silently discards a
worksheet error.

## Relationships

- **`INDIRECT`** is the other half of the classic pair: `ADDRESS` makes address text,
  `INDIRECT` turns address text back into a live reference. That round trip is a common idiom
  and also a common source of fragility — see the `INDIRECT` page for why the text hop is not
  free.
- **`CELL("address", ref)`** answers the inverse question: given a reference, what is its
  address text. `ADDRESS` goes from coordinates to text; `CELL` goes from reference to text.
- **`ROW` / `COLUMN`** produce the coordinates that `ADDRESS` consumes.
- **`OFFSET` and `INDEX`** are what you actually want when your goal is a *reference* computed
  from coordinates. Reaching for `ADDRESS` + `INDIRECT` to do that job trades a direct
  reference for a text round trip and, through `INDIRECT`, for volatility.

## Notes for implementers

- **The function is pure text assembly with no workbook access.** The projection records
  `host_interaction_class: None` and `thread_safety_class: SafePure` — unlike its family
  siblings `SHEET`, `SHEETS` and `FORMULATEXT`, which do need the workbook. Treating `ADDRESS`
  as a string builder is correct.
- **Column-number-to-letters is the only interesting algorithm here**, and it is a bijective
  base-26 encoding (`A`…`Z`, `AA`…), not ordinary base 26 — there is no zero digit. Off-by-one
  bugs at `Z`/`AA` and at `ZZ`/`AAA` are the classic failure, and any implementation should be
  probed exactly there.
- **R1C1 output is a different grammar, not a different formatting of the same string.** In
  R1C1, "relative" is expressed with bracketed offsets relative to the calling cell in ordinary
  formula text — but `ADDRESS` is given absolute coordinates and no calling cell. What
  `ADDRESS` emits for the relative R1C1 cases is therefore a genuine question, not a detail,
  and it is first on the probe list below.
- **Locale.** The rendered separator between sheet qualifier and address, and the quoting of
  sheet names, are presentation decisions that may vary by locale and by sheet-name content.

## What has not been checked

No Handbook vector suite exists for `ADDRESS`, and no Excel-comparison evidence record is
recorded for it. Nobody has checked this function's output against Excel here.

The inputs worth probing first, in order, because each one changes the shape of the answer
rather than a digit of it:

1. **`a1 = FALSE` with each of `abs_num` 1–4.** This is where the two notations stop being
   interchangeable: A1 style has four distinct renderings, and R1C1's relative form is defined
   against a calling cell that `ADDRESS` does not have. Establish all four strings.
2. **Column-letter boundaries**: `column_num` = 26, 27, 702, 703, and the sheet's maximum
   column. These pin the bijective-base-26 encoder.
3. **Domain edges**: `row_num` or `column_num` of 0, negative, fractional, and beyond the
   sheet's limits; `abs_num` of 0, 5, and fractional. Each should be recorded as a value or an
   error code, not assumed.
4. **`sheet_text` containing a space, an apostrophe, and a `!`.** This establishes whether
   `ADDRESS` quotes the qualifier or emits your text verbatim — a difference that decides
   whether `INDIRECT(ADDRESS(...))` round-trips.
5. **Array arguments in each of the five positions**, to confirm the projected lift axis
   against Excel's actual spilling.

Settling items 1 and 4 would close most of the practical uncertainty on this page.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `ADDRESS` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-ADDRESS-001: ADDRESS quotes sheet names resembling R1C1 axis prefixes (even R1048576C16385), omits quotes after [book], and allows signed relative R1C1 offsets

*Quirk; measured; recorded 2026-11-22.*

In Excel 16.0 build 20430, ADDRESS quotes sheet_text not only for A1 references, spaces,
punctuation, and TRUE/FALSE, but also for any unbracketed name starting with R<1..1048576> or
C<1..16384> or RC<1..16384> not followed by a dot (so R1048576C16385 and R1048576C and R1C000
are quoted, whereas R1048577C16384 is not). A workbook-prefixed sheet_text like [a]TRUE is not
quoted when the book and sheet parts use identifier characters, and empty sheet_text produces
an unquoted '!'. When a1 is FALSE and abs_num selects relative coordinates (abs_num 2, 3, or
4), row_num and column_num admit negative and zero offsets in (-limit+1)..limit and format zero
as R or C without brackets.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `ADDRESS(1, 1, 1, TRUE, "R1048576C16385")` | row_num `1.0` (`0x3ff0000000000000`); column_num `1.0` (`0x3ff0000000000000`); abs_num `1.0` (`0x3ff0000000000000`); a1 `TRUE`; sheet_text `R1048576C16385` (cells) | `'R1048576C16385'!$A$1` | Excel 16.0 build 20430, Windows 64-bit (NT 10.00), workbook compatibility 2; CPU not recorded |
| `ADDRESS(1, 1, 1, TRUE, "R1048577C16384")` | row_num `1.0` (`0x3ff0000000000000`); column_num `1.0` (`0x3ff0000000000000`); abs_num `1.0` (`0x3ff0000000000000`); a1 `TRUE`; sheet_text `R1048577C16384` (cells) | `R1048577C16384!$A$1` | Excel 16.0 build 20430, Windows 64-bit (NT 10.00), workbook compatibility 2; CPU not recorded |
| `ADDRESS(1048576, 16384, 1, TRUE, "[a]TRUE")` | row_num `1048576.0` (`0x4130000000000000`); column_num `16384.0` (`0x40d0000000000000`); abs_num `1.0` (`0x3ff0000000000000`); a1 `TRUE`; sheet_text `[a]TRUE` (cells) | `[a]TRUE!$XFD$1048576` | Excel 16.0 build 20430, Windows 64-bit (NT 10.00), workbook compatibility 2; CPU not recorded |
| `ADDRESS(-1, 1, 4, FALSE)` | row_num `-1.0` (`0xbff0000000000000`); column_num `1.0` (`0x3ff0000000000000`); abs_num `4.0` (`0x4010000000000000`); a1 `FALSE` (cells) | `R[-1]C[1]` | Excel 16.0 build 20430, Windows 64-bit, workbook compatibility 2; CPU not recorded |

Basis (measured): Measured in Excel 16.0 build 20430 across w111-broad,
w111-address-sheet-20260929, and w111-address-refinement-20260929 in CalcFunc-Evidence, and
matched on 1,284/1,284 full-bank ADDRESS rows.

Evidence:

- DnaCalc `crates/calc-func/src/ported/address.rs`, quote_sheet_text_if_needed; format_address_body at `95eb0cb2ef` — Sheet quoting grammar and relative R1C1 coordinate validation
- DnaCalc `crates/calc-func/witness-status.tsv`, ADDRESS at `95eb0cb2ef` — 1,284/1,284 full-bank ADDRESS matches
- CalcFunc-Evidence `runs/w111-address-refinement-20260929/outcomes/excel.jsonl.gz`, address-refinement-20260929-00588 at `4ef2c244d7` (readings)
- CalcFunc-Evidence `runs/w111-address-refinement-20260929/outcomes/excel.jsonl.gz`, address-refinement-20260929-00587 at `4ef2c244d7` (readings)
- CalcFunc-Evidence `runs/w111-address-sheet-20260929/outcomes/excel.jsonl.gz`, address-sheet-discovery-20260929-0093 at `4ef2c244d7` (readings)
- CalcFunc-Evidence `runs/w111-broad-20260929/answers/answers-address.json.gz`, w111broad-20260929-ADDRESS-00032 at `4ef2c244d7` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| A1 style | `$D$5` — column letters and row numbers, `$` marking absolute parts |
| R1C1 style | `R5C4` — numeric row and column, offsets bracketed when relative |
| absolute part | A coordinate written with `$`, unchanged when the formula is copied |
| sheet qualifier | The `SheetName!` prefix built from `sheet_text` |
| bijective base 26 | The `A`…`Z`, `AA`… column-letter encoding, which has no zero digit |

## Sources

- Microsoft, ADDRESS function —
  <https://support.microsoft.com/en-us/office/address-function-d0c26c0d-3991-446b-8de4-ab46431d4f89>
  (argument meanings, the `abs_num` 1–4 table, and the `a1` reference-style flag).
- Handbook `content/model/01-value-universe.md` (value kinds, error registry).
- Handbook `content/model/02-coercion-and-lifting.md` (error propagation, direct-argument
  coercion).
- Handbook `content/model/03-call-pipeline.md` (`ValuesOnlyPreAdapter`,
  `ByIndexScalarArrayLift`).
- Handbook `data/functions/FUNC.ADDRESS.json` and `data/presence/FUNC.ADDRESS.json`
  (signature, arity, classification axes, implementing module).
