---
schema: efh.function-page/v1
function_id: FUNC.DCOUNTA
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
family: database_family
role_in_family: >-
  Counts the non-blank cells of the field column over matching records — the presence-counting
  counterpart to DCOUNT's number-counting — and the family's other member whose field argument
  may be omitted.
---

# DCOUNTA

## What it computes

`DCOUNTA(database, field, criteria)` selects the records of *database* that satisfy *criteria*
and returns how many of them have a **non-blank** cell in the column named by *field*.

With R the set of selected record rows and c the resolved field column:

    result = | { r in R : cell (r, c) is not blank } |

Numbers, text, and logicals all count as present. Only genuinely absent content — an empty cell,
or an omitted value — is excluded.

If *field* is omitted, the column stage disappears and the function returns `|R|`, the count of
matching records. In that mode `DCOUNTA` and [DCOUNT](FUNC.DCOUNT.md) return the same thing.

The selection stage — header matching, AND across a criteria row, OR down the rows, blank
criteria cells, and criteria-expression parsing — is identical for all twelve database functions
and is explained once in
[the criteria-range mechanism](FUNC.DAVERAGE.md#the-criteria-range-mechanism).

## Arguments

Three argument slots; the reference implementation declares an arity of exactly 3.

- **`database`** — the header-plus-records range or array.
- **`field`** — the column selector, as header text or as a 1-based column index within the
  database range. Optional in meaning, not in position: the slot must still be present, and an
  empty slot delivers the `Missing` marker described in
  [the value universe](../model/01-value-universe.md#missing-versus-empty). OxFunc also treats an
  empty cell reference and an empty text string here as "omitted".
- **`criteria`** — the header-plus-conditions range or array.

## Result and edge cases

Returns a Number — a count, so always a non-negative integer.

- **No record matches**: `0`. No error on an empty selection.
- **Field omitted**: the count of matching records.
- **Text in the field column counts**, including text that reads as a number. This is the
  cleanest way to see the difference from `DCOUNT`: on a column of `"1"`, `"2"`, `"3"`,
  `DCOUNTA` counts three and `DCOUNT` counts none.
- **Logicals count.**
- **The empty string.** A cell holding the zero-length text `""` — typically produced by a
  formula such as `IF(A1="","",A1)` — is a Text value, not an empty cell, and OxFunc counts it.
  This is a real trap: the column looks blank on screen and counts as full. Whether Excel agrees
  here is unchecked, and it is the first thing worth probing.
- **An error value in a scanned field cell** surfaces rather than being counted; the family
  declares `ErrorCollapseProfile::ReductionFold`.
- **Arrays**: inline array literals resolve to grids and are accepted; `DCOUNTA` is not a lift
  kernel.

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | *database* or *criteria* is not a grid or is empty; a database header cell is blank; a criteria header names no database field; a *supplied* *field* matches no header, or is a non-integer or out-of-range column index |
| propagated | An error value in a header cell, a criteria cell, or a scanned field cell surfaces as that error |

No count-specific domain error exists. Microsoft's page documents argument roles and criteria
conventions; the conditions above are stated from OxFunc's implementation and family contract
and have not been compared against Excel by the Handbook.

## Relationships

- **[DCOUNT](FUNC.DCOUNT.md)** is the number-counting sibling. The pair mirrors `COUNT` and
  `COUNTA`. These two are the only family members that accept an omitted field.
- **Nearest modern equivalent**: `COUNTIFS` with a `"<>"` condition on the field column, which
  expresses "non-blank and matching" inline instead of through a criteria table.
- **Confused with**: `COUNTA` (no selection stage), `COUNTBLANK` (the complement, and with a
  different treatment of `""`), and `DCOUNT`.

## Notes for implementers

1. **"Non-blank" is a value-kind test with one uncomfortable edge.** Empty and the `Missing`
   marker are blank; every other core value kind — Number, Text, Logical — is present. Whether
   zero-length text belongs on the blank side is the one genuinely contested cell of that table,
   and it is unresolved here.
2. **Two code paths, one function.** The omitted-field mode returns before any column is read,
   so a bad field value is undiagnosable in that mode. Resolve the field only when supplied.
3. **Errors are not "present".** A scanned error surfaces instead of incrementing the count,
   which is a different decision from "count it as non-blank" and is easy to get wrong when the
   counting predicate is written as `!is_blank(cell)`.
4. **Numeric criteria comparison is not exact double comparison** — see the note in
   [the shared mechanism](FUNC.DAVERAGE.md#numeric-comparison-is-not-exact-double-comparison).

## What has not been checked

No Handbook vector suite exists for `DCOUNTA`, and no Excel-comparison evidence record is
recorded for it. Nobody has run a Handbook-owned comparison of this function against Excel.

Inputs I would probe first:

1. **The zero-length string.** A matching record whose field cell holds a formula returning
   `""`. OxFunc counts it. This is the single highest-value probe on the page, because the
   answer also constrains how `DCOUNT`'s and `DGET`'s blank tests should be written, and because
   the same question recurs in the criteria mechanism's blank operand.
2. **A cell that was cleared versus a cell that was never filled**, to confirm both read as
   blank.
3. **Logicals and errors in the field column**, separating "counted", "skipped", and
   "propagated".
4. **The three ways to omit the field**: empty call slot, reference to a blank cell, and `""`.
   Note that `""` in the field slot is read as "omitted" and not as "a header named nothing",
   which is a choice an implementation could plausibly make either way.
5. **Empty selection**, confirming `0` rather than an error.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `DCOUNTA` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-DCOUNTA-001: DCOUNTA counts non-blank matching cells (or records when field is omitted), requires 2+ rows in database/criteria, and rejects non-reference databases at entry (0x800A03EC)

*Quirk; measured; recorded 2026-11-22.*

In Excel 16.0.20527, DCOUNTA(database, [field], criteria) counts every non-blank cell (numbers,
text, logicals, errors) in the chosen field column across matching records (or counts matching
records when field is omitted), matches un-operatored text criteria as case-insensitive
prefixes, requires both database and criteria to be ranges with at least one record row below
headers (#VALUE! otherwise), and rejects literal numbers or array constants in database at
formula entry (COM 0x800A03EC). All 5,676 full-bank DCOUNTA rows match.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `DCOUNTA(A1:B3, "Value", D1:D2)` | database `{"Key","Value";"a0",0;"a1",-49}`; field `Value`; criteria `{"Key";"a*"}` (cells) | `2.0` (`0x4000000000000000`) | Excel 16.0 build 20527, Windows, workbook compatibility 2, en-ZA; bitness and CPU not recorded |
| `DCOUNTA(A1:B5, "Value", D1:D2)` | database `{"Key","Value";"a2",0.11764705882352941;"a3",-47;"a4",-46;"a5",-45}`; field `Value`; criteria `{"Key";"a1"}` (cells) | `0.0` (`0x0000000000000000`) | Excel 16.0 build 20527, Windows, workbook compatibility 2, en-ZA; bitness and CPU not recorded |
| `=DCOUNTA(2,1,B1:B3)` | as typed | `entry rejected (0x800A03EC)` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Measured in CalcFunc-Evidence admit3-answers-dcounta-range-mixed.json,
DnaCalc prefix_metadata row 266, and crates/calc-fml/tests/fixtures/entry/excel-entry.tsv;
5,676/5,676 full-bank rows match.

Evidence:

- DnaCalc `crates/calc-func/src/catalogue/database.rs`, database at `2e9be2b5b7` — non-blank field count, 2-row minimum, and entry rules
- DnaCalc `crates/calc-func/witness-status.tsv`, DCOUNTA at `2e9be2b5b7` — 5,676/5,676 full-bank matches
- CalcFunc-Evidence `readings/excel-20527/admit3-answers-dcounta-range-mixed.json`, admit3-range-mixed-DCOUNTA-0000 at `4ef2c244d7` (readings)
- CalcFunc-Evidence `readings/excel-20527/admit3-answers-dcounta-range-mixed.json`, admit3-range-mixed-DCOUNTA-0002 at `4ef2c244d7` (readings)
- DnaCalc `crates/calc-fml/tests/fixtures/entry/excel-entry.tsv`, 0	=DCOUNTA(2,1,B1:B3) at `2e9be2b5b7` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| database grid | The header-plus-records rectangle passed as *database* |
| criteria grid | The header-plus-conditions rectangle passed as *criteria* |
| field column | The database column selected by the *field* argument |
| non-blank | Any core value kind other than Empty or `Missing` |
| omitted field | The *field* slot delivering `Missing`, an empty cell, or empty text |

## Sources

- Microsoft, "DCOUNTA function" —
  <https://support.microsoft.com/en-us/office/dcounta-function-00232a6d-5a66-4a01-a25b-c1653fda1244>
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md`.
- Handbook call-model chapters 01 and 03 (value kinds, Missing versus Empty, error folding).
- [DAVERAGE](FUNC.DAVERAGE.md) — the family's shared criteria-range mechanism.
- OxFunc `docs/function-lane/FUNCTION_SLICE_DATABASE_FAMILY_CONTRACT_PRELIM.md` and
  `crates/oxfunc_core/src/functions/database_family.rs` at commit `473efa3`.
- `data/functions/FUNC.DCOUNTA.json`, `data/presence/FUNC.DCOUNTA.json`.
