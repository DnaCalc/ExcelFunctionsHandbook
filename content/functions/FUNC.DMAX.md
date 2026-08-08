---
schema: efh.function-page/v1
function_id: FUNC.DMAX
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
  The largest numeric value in the field column over matching records; with DMIN it forms the
  family's order-statistic pair, and it shares their characteristic answer of zero — not an
  error — when nothing numeric was selected.
---

# DMAX

## What it computes

`DMAX(database, field, criteria)` selects the records of *database* that satisfy *criteria*,
reads the column named by *field*, and returns the largest of the numeric values found there.

With R the set of selected record rows, c the resolved field column, and

    V = [ v : v = value of cell (r, c) for r in R, where v is a Number ]

the result is max V. Non-numeric cells — text, logicals, blanks — are not candidates: they are
skipped, not coerced. If V is empty the maximum is undefined over the reals, and OxFunc returns
`0`, following the same convention as `MAX` over no numbers.

That zero is a genuine hazard on a column of negative numbers: a criteria range that matches
nothing, or matches only records with blank field cells, gives `0` — a value larger than every
number actually in the column.

The selection stage — header matching, AND across a criteria row, OR down the rows, blank
criteria cells, and criteria-expression parsing — is identical for all twelve database functions
and is explained once in
[the criteria-range mechanism](FUNC.DAVERAGE.md#the-criteria-range-mechanism).

## Arguments

Three arguments, all required at the call site; the reference implementation declares an arity
of exactly 3.

- **`database`** — the header-plus-records range or array.
- **`field`** — the column selector, as header text or as a 1-based column index within the
  database range. Required; only [DCOUNT](FUNC.DCOUNT.md) and [DCOUNTA](FUNC.DCOUNTA.md) accept
  an omitted field.
- **`criteria`** — the header-plus-conditions range or array.

## Result and edge cases

Returns a Number.

- **No numeric value selected** — because no record matched, or because every matching record's
  field cell is blank or non-numeric: `0`, not an error. Pair with
  [DCOUNT](FUNC.DCOUNT.md) if you need to tell "the maximum is zero" from "there was nothing to
  maximize".
- **Text that looks numeric in the field column** is skipped, per the range-scan side of the
  asymmetry in
  [coercion and lifting](../model/02-coercion-and-lifting.md#direct-arguments-versus-range-scans).
- **Logicals in the field column** are skipped, so `TRUE` never wins as 1.
- **An error value in a scanned field cell** surfaces; the family declares
  `ErrorCollapseProfile::ReductionFold`.
- **Ties** are invisible: `DMAX` returns the value, not which record held it. Use
  [DGET](FUNC.DGET.md) with a criteria row keyed on the maximum if you need the record.
- **Arrays**: inline array literals resolve to grids and are accepted; `DMAX` is not a lift
  kernel.

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | *database* or *criteria* is not a grid or is empty; a database header cell is blank; a criteria header names no database field; *field* matches no header, or is a non-integer or out-of-range column index |
| propagated | An error value in a header cell, a criteria cell, or a scanned field cell surfaces as that error |

`DMAX` has no domain error of its own. Microsoft's page documents argument roles and criteria
conventions; the table is stated from OxFunc's implementation and family contract, and has not
been compared against Excel by the Handbook.

## Relationships

- **[DMIN](FUNC.DMIN.md)** is the mirror member, identical in every respect but the comparison
  direction — including the empty-selection zero.
- **Nearest modern equivalent**: `MAXIFS`, which takes conditions inline instead of as a
  criteria table. Neither supersedes the other; Excel supports both.
- **Confused with**: `MAX` (no selection stage), `LARGE` (k-th largest, no selection stage), and
  `MAXA` (which does count logicals and text as values).

## Notes for implementers

1. **The empty-selection answer is a convention, not a mathematical result.** Returning `0`
   rather than an error is a deliberate compatibility choice, and it must be applied *after*
   skipping non-numeric cells, not as a substitute for them.
2. **Skip, do not coerce.** Admit only cells whose value kind is Number.
3. **Comparison of the collected values is ordinary IEEE ordering** in OxFunc's reduction; the
   truncation-style tolerance applies to *criteria matching*, not to the max/min reduction. Do
   not let the two comparison paths share a helper by accident.
4. **Numeric criteria comparison is not exact double comparison** — see the note in
   [the shared mechanism](FUNC.DAVERAGE.md#numeric-comparison-is-not-exact-double-comparison).

## What has not been checked

No Handbook vector suite exists for `DMAX`, and no Excel-comparison evidence record is recorded
for it. Nobody has run a Handbook-owned comparison of this function against Excel.

Inputs I would probe first:

1. **Empty selection over an all-negative column.** Criteria matching nothing, field column
   `-5, -3, -9`. The Handbook states `0`; the alternative reading is an error or the true
   maximum `-3`. This one probe carries most of the page's uncertainty.
2. **A matching set whose field cells are all text or all blank**, which is the same boundary
   reached by a different route.
3. **A field column of numeric-looking text**, separating "skipped" from "coerced".
4. **Logicals in the field column** alongside numbers, to confirm `TRUE` is not read as 1.
5. **Duplicate maxima**, confirming the value is returned without regard to record order — and,
   more usefully, that record order does not perturb the result at all.
6. **An error in a matching record's field cell versus in a non-matching record's**, to confirm
   only scanned cells propagate.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| database grid | The header-plus-records rectangle passed as *database* |
| criteria grid | The header-plus-conditions rectangle passed as *criteria* |
| field column | The database column selected by the *field* argument |
| value list | The Numbers collected from the field column of the matching records |
| empty-selection zero | The `0` returned when the value list is empty |

## Sources

- Microsoft, "DMAX function" —
  <https://support.microsoft.com/en-us/office/dmax-function-f4e8209d-8958-4c3d-a1ee-6351665d41c2>
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md`.
- Handbook call-model chapters 02 and 03 (range-scan coercion, error folding).
- [DAVERAGE](FUNC.DAVERAGE.md) — the family's shared criteria-range mechanism.
- OxFunc `docs/function-lane/FUNCTION_SLICE_DATABASE_FAMILY_CONTRACT_PRELIM.md` and
  `crates/oxfunc_core/src/functions/database_family.rs` at commit `473efa3`.
- `data/functions/FUNC.DMAX.json`, `data/presence/FUNC.DMAX.json`.
