---
schema: efh.function-page/v1
function_id: FUNC.DCOUNT
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
family: database_family
role_in_family: >-
  Counts the numeric cells of the field column over matching records, and is one of the two
  members whose field argument may be omitted — in which case it counts matching records
  instead, making it the family's record-counter.
---

# DCOUNT

## What it computes

`DCOUNT(database, field, criteria)` selects the records of *database* that satisfy *criteria*
and returns how many of them hold a **number** in the column named by *field*.

With R the set of selected record rows and c the resolved field column:

    result = | { r in R : value of cell (r, c) is a Number } |

If *field* is omitted, the column stage disappears and the function returns `|R|` — the count of
matching records, regardless of what any column contains. That two-mode behaviour is what makes
`DCOUNT` the family's record counter as well as its value counter.

"Is a Number" is a test on the value kind, not a parse. Text that reads as a number is not
counted; neither are logicals, blanks, or empty strings.

The selection stage — header matching, AND across a criteria row, OR down the rows, blank
criteria cells, and how a criteria expression is parsed — is identical for all twelve database
functions and is explained once in
[the criteria-range mechanism](FUNC.DAVERAGE.md#the-criteria-range-mechanism).

## Arguments

Three argument slots; the reference implementation declares an arity of exactly 3.

- **`database`** — the header-plus-records range or array.
- **`field`** — the column selector, as header text or as a 1-based column index within the
  database range. **Optional in meaning, not in position.** Writing `DCOUNT(A1:C9,,E1:F2)`
  leaves the slot empty, which delivers the `Missing` marker described in
  [the value universe](../model/01-value-universe.md#missing-versus-empty) — three arguments are
  still passed, so an arity of exactly 3 and an omitted field are not in conflict. OxFunc also
  accepts an empty cell reference and an empty text string in this slot as "omitted".
- **`criteria`** — the header-plus-conditions range or array.

## Result and edge cases

Returns a Number — a count, so always a non-negative integer.

- **No record matches**: `0`. `DCOUNT` never errors on an empty selection.
- **Field omitted**: the result is the number of matching records. Combined with an all-blank
  criteria row (which selects everything), this degenerates to the record count of the database.
- **Text that looks numeric in the field column** is not counted. This is the range-scan side of
  the asymmetry in
  [coercion and lifting](../model/02-coercion-and-lifting.md#direct-arguments-versus-range-scans).
- **Logicals and blanks in the field column** are not counted.
- **An error value in a scanned field cell** surfaces rather than being counted or skipped; the
  family declares `ErrorCollapseProfile::ReductionFold`.
- **Arrays**: inline array literals resolve to grids and are accepted; `DCOUNT` is not a lift
  kernel.

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | *database* or *criteria* is not a grid or is empty; a database header cell is blank; a criteria header names no database field; a *supplied* *field* matches no header, or is a non-integer or out-of-range column index |
| propagated | An error value in a header cell, a criteria cell, or a scanned field cell surfaces as that error |

There is no count-specific domain error. Microsoft's page documents the argument roles and the
criteria conventions; the conditions above are stated from OxFunc's implementation and family
contract and have not been compared against Excel by the Handbook.

## Relationships

- **`DCOUNTA`** is the sibling that counts *non-blank* cells instead of numeric ones. The pair
  mirrors `COUNT` and `COUNTA` exactly: `DCOUNT` counts numbers, `DCOUNTA` counts anything
  present. Both accept an omitted field; no other family member does.
- **[DSUM](FUNC.DSUM.md) and [DAVERAGE](FUNC.DAVERAGE.md)** run over the same value list.
  `DAVERAGE` is `DSUM` divided by `DCOUNT` except when the count is zero, where `DAVERAGE`
  returns `#DIV/0!` and `DCOUNT` returns `0`.
- **Nearest modern equivalent**: `COUNTIFS` (and `COUNTIF`), which take conditions inline
  instead of as a criteria table. Neither supersedes the other.
- **Confused with**: `COUNT` (no selection stage) and `DCOUNTA`; and, when the field is omitted,
  with `ROWS`, which counts rows without any selection at all.

## Notes for implementers

1. **Two code paths, one function.** The omitted-field mode short-circuits before any column is
   read, so a bad *field* value cannot be diagnosed in that mode — there is nothing to diagnose.
   Resolve the field only when one was supplied.
2. **Distinguish Missing from Empty carefully.** Both are treated as "omitted" here, but they
   arrive by different routes (an empty call slot versus a reference to a blank cell), and the
   call model keeps them distinct on purpose. An implementation that collapses them upstream
   loses the ability to make them differ later if evidence says they should.
3. **Count by value kind, not by convertibility.** The whole difference between `DCOUNT` and
   `DCOUNTA` lives in this predicate.
4. **Numeric criteria comparison is not exact double comparison** — see the note in
   [the shared mechanism](FUNC.DAVERAGE.md#numeric-comparison-is-not-exact-double-comparison).

## What has not been checked

No Handbook vector suite exists for `DCOUNT`, and no Excel-comparison evidence record is
recorded for it. Nobody has run a Handbook-owned comparison of this function against Excel.

Inputs I would probe first:

1. **The three ways to omit the field**: an empty call slot, a reference to a blank cell, and
   an empty text literal `""`. OxFunc treats all three as omitted. If Excel disagrees on any of
   them, this is where it shows, and the difference is a `#VALUE!` versus a count.
2. **A field column of numeric-looking text.** Matching records whose field cells hold `"1"`,
   `"2"`, `"3"`. Counting gives 3, skipping gives 0.
3. **A field column of logicals.** Same shape of question, different value kind.
4. **Empty selection with and without a field**, to confirm both return `0` rather than an
   error.
5. **A field argument that names a column present in the criteria grid but not in the
   database**, which should be a field-resolution failure rather than a zero count.
6. **Field index boundaries**: `0`, one past the last column, and a fractional index.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| database grid | The header-plus-records rectangle passed as *database* |
| criteria grid | The header-plus-conditions rectangle passed as *criteria* |
| field column | The database column selected by the *field* argument |
| omitted field | The *field* slot delivering `Missing`, an empty cell, or empty text |
| record count mode | The omitted-field behaviour: count matching records, read no column |

## Sources

- Microsoft, "DCOUNT function" —
  <https://support.microsoft.com/en-us/office/dcount-function-c1fc7b93-fb0d-4d8d-97db-8d5f076eaeb1>
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md`.
- Handbook call-model chapters 01, 02 and 03 (Missing versus Empty, range-scan coercion, error
  folding).
- [DAVERAGE](FUNC.DAVERAGE.md) — the family's shared criteria-range mechanism.
- OxFunc `docs/function-lane/FUNCTION_SLICE_DATABASE_FAMILY_CONTRACT_PRELIM.md` (rule 8:
  omitted-field handling for `DCOUNT` is in the admitted slice) and
  `crates/oxfunc_core/src/functions/database_family.rs` at commit `473efa3`.
- `data/functions/FUNC.DCOUNT.json`, `data/presence/FUNC.DCOUNT.json`.
