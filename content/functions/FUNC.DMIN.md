---
schema: efh.function-page/v1
function_id: FUNC.DMIN
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
  The smallest numeric value in the field column over matching records; DMAX's mirror, and the
  member where the family's empty-selection zero is most likely to be mistaken for real data,
  because a column of positive numbers has no true minimum of zero.
---

# DMIN

## What it computes

`DMIN(database, field, criteria)` selects the records of *database* that satisfy *criteria*,
reads the column named by *field*, and returns the smallest of the numeric values found there.

With R the set of selected record rows, c the resolved field column, and

    V = [ v : v = value of cell (r, c) for r in R, where v is a Number ]

the result is min V. Non-numeric cells — text, logicals, blanks — are skipped, not coerced. If V
is empty, OxFunc returns `0`, following the same convention as `MIN` over no numbers.

On a column of strictly positive numbers that zero is indistinguishable in kind from a real
answer, and it is smaller than every value actually present. `DMIN` is therefore the member of
the family where the empty-selection convention does the most damage if it goes unnoticed.

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

- **No numeric value selected**: `0`, not an error. See the warning above.
- **Text that looks numeric in the field column** is skipped, per the range-scan side of the
  asymmetry in
  [coercion and lifting](../model/02-coercion-and-lifting.md#direct-arguments-versus-range-scans).
- **Logicals in the field column** are skipped, so `FALSE` never wins as 0.
- **An error value in a scanned field cell** surfaces; the family declares
  `ErrorCollapseProfile::ReductionFold`.
- **Ties** are invisible: the value is returned, not the record that held it.
- **Arrays**: inline array literals resolve to grids and are accepted; `DMIN` is not a lift
  kernel.

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | *database* or *criteria* is not a grid or is empty; a database header cell is blank; a criteria header names no database field; *field* matches no header, or is a non-integer or out-of-range column index |
| propagated | An error value in a header cell, a criteria cell, or a scanned field cell surfaces as that error |

`DMIN` has no domain error of its own. Microsoft's page documents argument roles and criteria
conventions; the table is stated from OxFunc's implementation and family contract and has not
been compared against Excel by the Handbook.

## Relationships

- **[DMAX](FUNC.DMAX.md)** is the mirror member, identical but for the comparison direction.
- **Nearest modern equivalent**: `MINIFS`, with inline conditions instead of a criteria table.
- **Confused with**: `MIN` (no selection stage), `SMALL` (k-th smallest), and `MINA` (which
  counts logicals and text as values).

## Notes for implementers

1. **The empty-selection zero is a convention.** Apply it after the non-numeric skip, and be
   aware that it is not the identity of the min operation in any useful sense — it is a
   compatibility answer.
2. **Skip, do not coerce.** Admit only cells whose value kind is Number.
3. **Two comparison paths, one function.** Criteria matching uses the tolerant, truncation-style
   comparison lane; the min reduction over the collected values uses ordinary IEEE ordering.
   Keep them separate.
4. **Numeric criteria comparison is not exact double comparison** — see the note in
   [the shared mechanism](FUNC.DAVERAGE.md#numeric-comparison-is-not-exact-double-comparison).

## What has not been checked

No Handbook vector suite exists for `DMIN`, and no Excel-comparison evidence record is recorded
for it. Nobody has run a Handbook-owned comparison of this function against Excel.

Inputs I would probe first:

1. **Empty selection over an all-positive column.** Criteria matching nothing, field column
   `5, 3, 9`. The Handbook states `0`, which is smaller than every value present — the probe
   that most cheaply confirms or refutes the convention.
2. **A matching set whose field cells are all blank**, reaching the same boundary differently.
3. **A field column of numeric-looking text**, separating "skipped" from "coerced".
4. **Negative zero.** A field column containing `-0` alongside `0`, which min may or may not
   distinguish, and which is worth pinning before any suite is written.
5. **Logicals alongside numbers**, confirming `FALSE` is not read as 0.
6. **An error in a matching record's field cell**, confirming propagation rather than skipping.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| database grid | The header-plus-records rectangle passed as *database* |
| criteria grid | The header-plus-conditions rectangle passed as *criteria* |
| field column | The database column selected by the *field* argument |
| value list | The Numbers collected from the field column of the matching records |
| empty-selection zero | The `0` returned when the value list is empty |

## Sources

- Microsoft, "DMIN function" —
  <https://support.microsoft.com/en-us/office/dmin-function-4ae6f1d9-1f26-40f1-a783-6dc3680192a3>
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md`.
- Handbook call-model chapters 02 and 03 (range-scan coercion, error folding).
- [DAVERAGE](FUNC.DAVERAGE.md) — the family's shared criteria-range mechanism.
- OxFunc `docs/function-lane/FUNCTION_SLICE_DATABASE_FAMILY_CONTRACT_PRELIM.md` and
  `crates/oxfunc_core/src/functions/database_family.rs` at commit `473efa3`.
- `data/functions/FUNC.DMIN.json`, `data/presence/FUNC.DMIN.json`.
