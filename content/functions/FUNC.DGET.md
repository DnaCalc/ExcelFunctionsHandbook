---
schema: efh.function-page/v1
function_id: FUNC.DGET
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
  The family's selector rather than a reducer: it returns the field value of the single matching
  record, and is the only member that treats "no match" and "more than one match" as distinct
  errors — and the only one not declared as an error-folding reduction.
---

# DGET

## What it computes

`DGET(database, field, criteria)` selects the records of *database* that satisfy *criteria* and
returns the value held in the column named by *field* — but only if **exactly one** record
matches.

    |R| = 0  →  #VALUE!
    |R| = 1  →  the value of cell (the matching record, field column)
    |R| > 1  →  #NUM!

`DGET` is not an aggregate. It performs no arithmetic and applies no scan policy: it hands back
one cell's value, whatever kind that value has. That makes it the family's lookup, and it is the
reason the reference implementation declares `ErrorCollapseProfile::None` for `DGET` while every
other member declares `ReductionFold` — there is no reduction over many inputs in which
competing errors could need folding.

The uniqueness requirement is the point of the function. `DGET` is a lookup that refuses to
guess: where `VLOOKUP` silently returns the first of several matches, `DGET` reports that the
question was ambiguous.

The selection stage — header matching, AND across a criteria row, OR down the rows, blank
criteria cells, and criteria-expression parsing — is identical for all twelve database functions
and is explained once in
[the criteria-range mechanism](FUNC.DAVERAGE.md#the-criteria-range-mechanism).

## Arguments

Three arguments, all required at the call site; the reference implementation declares an arity
of exactly 3.

- **`database`** — the header-plus-records range or array.
- **`field`** — the column selector, as header text or as a 1-based column index within the
  database range. Required: unlike [DCOUNT](FUNC.DCOUNT.md) and [DCOUNTA](FUNC.DCOUNTA.md),
  `DGET` has nothing to return without a column.
- **`criteria`** — the header-plus-conditions range or array. Note the family rule that bites
  hardest here: a criteria range with no condition rows, or with an all-blank condition row,
  selects **every** record — so on any database with two or more records that mistake produces
  `#NUM!`, not the "first record" a reader might expect.

## Result and edge cases

Returns whatever kind the selected cell holds: Number, Text, or Logical.

- **The selected cell is blank**: OxFunc returns the number `0`. This follows the general
  worksheet rule that a blank never reaches a published result — see
  [the value universe](../model/01-value-universe.md#the-boundary-model) — but it means `DGET`
  cannot report "that field is empty" as anything other than zero.
- **The selected cell holds an error value**: that error is returned. `DGET` passes the cell
  through rather than diagnosing it.
- **No match**: `#VALUE!`. **More than one match**: `#NUM!`. Both are documented by Microsoft
  and both are pinned in OxFunc's family contract. Note that the two errors are, at first
  glance, the wrong way round relative to intuition — "not found" is usually `#N/A` elsewhere in
  Excel, and `DGET` does not use `#N/A` at all.
- **Text is returned as text.** `DGET` is the only family member that can return a non-numeric
  result, which matters when its output feeds arithmetic downstream.
- **Arrays**: inline array literals resolve to grids and are accepted; `DGET` is not a lift
  kernel, and it never returns an array.

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | No record matches the criteria |
| `#NUM!` | More than one record matches the criteria |
| `#VALUE!` | *database* or *criteria* is not a grid or is empty; a database header cell is blank; a criteria header names no database field; *field* matches no header, or is a non-integer or out-of-range column index |
| propagated | An error value in a header cell or a criteria cell surfaces as that error; an error value in the *selected* cell is returned as the function's result |

The no-match and multi-match rows are documented behaviour: Microsoft's page states them, and
OxFunc's family contract pins the same pair. The argument-shape rows are stated from OxFunc's
implementation and have not been compared against Excel by the Handbook.

## Relationships

- **Siblings**: the other eleven database functions, which share the selection stage and reduce
  the field column instead of selecting from it. Where they answer "how much", `DGET` answers
  "which".
- **Nearest modern equivalents**: `XLOOKUP` and `FILTER`. `FILTER` is the closest in spirit —
  criteria in, records out — but returns every match as a spilled array rather than insisting on
  uniqueness. `XLOOKUP` returns the first match and takes a not-found argument. Neither
  supersedes `DGET`; Excel retains all three.
- **Confused with**: `VLOOKUP` and `INDEX`/`MATCH`, both of which return the first match without
  complaint. Replacing `DGET` with `VLOOKUP` to "fix" a `#NUM!` discards the very check that was
  reporting a real ambiguity in the data.

## Notes for implementers

1. **Count matches before reading any cell.** The uniqueness test governs the whole result, and
   an implementation that reads the first match eagerly will produce a value where an error is
   required.
2. **Do not fold errors.** `DGET` declares `ErrorCollapseProfile::None`. Errors in the selected
   cell pass through; errors elsewhere in the field column are irrelevant, because `DGET` never
   looks at non-matching records' field cells.
3. **Preserve the value kind.** The result is a copy of a cell, not a coerced number. The one
   normalization OxFunc applies is blank → `0`.
4. **The two error codes are not interchangeable and are not `#N/A`.** Getting the pair backwards
   is a silent semantic bug: both are errors, so a naive test that only checks `ISERROR` will
   not catch it.
5. **Numeric criteria comparison is not exact double comparison** — see the note in
   [the shared mechanism](FUNC.DAVERAGE.md#numeric-comparison-is-not-exact-double-comparison).
   For `DGET` the consequence is sharper than for the aggregates: a tolerant comparison can turn
   a unique match into a double match, converting a value into `#NUM!`.

## What has not been checked

No Handbook vector suite exists for `DGET`, and no Excel-comparison evidence record is recorded
for it. The no-match/multi-match error pair is documented by Microsoft and pinned in OxFunc's
family contract, but the Handbook has not observed it in Excel itself.

Inputs I would probe first:

1. **Exactly-one, zero, and two matches** on the same database, to confirm the `#VALUE!` /
   value / `#NUM!` split and that neither branch returns `#N/A`.
2. **A blank selected cell.** OxFunc returns `0`; an alternative reading returns empty, which
   would then publish as `0` anyway — but the two differ when the result feeds a function that
   distinguishes blank from zero.
3. **A selected cell holding text, a logical, and an error**, one probe each, to confirm the
   kind is preserved and the error is passed through rather than replaced.
4. **The vacuous criteria range** — header row only — against a database with one record and
   with two, which distinguishes "selects everything" from "selects nothing" in the shared
   mechanism, using `DGET`'s error split as the detector. This is a better probe of the shared
   mechanism than any aggregate provides, because the two readings give different *error codes*
   rather than differing numbers.
5. **Near-equal numeric criteria that make a match ambiguous**: two records whose field values
   are `0.3` and `0.1+0.2`, with a criterion of `0.3`. Under exact comparison this is a unique
   match; under the truncation-style lane it is two matches and therefore `#NUM!`.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| database grid | The header-plus-records rectangle passed as *database* |
| criteria grid | The header-plus-conditions rectangle passed as *criteria* |
| field column | The database column selected by the *field* argument |
| selected cell | The intersection of the unique matching record and the field column |
| uniqueness requirement | The rule that exactly one record must match |

## Sources

- Microsoft, "DGET function" —
  <https://support.microsoft.com/en-us/office/dget-function-455568bf-4eef-45f7-90f0-ec250d00892e>
  (the no-match and multi-match error behaviour).
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md`.
- Handbook call-model chapters 01 and 03 (value kinds, the raw-versus-published boundary, error
  collapse profiles).
- [DAVERAGE](FUNC.DAVERAGE.md) — the family's shared criteria-range mechanism.
- OxFunc `docs/function-lane/FUNCTION_SLICE_DATABASE_FAMILY_CONTRACT_PRELIM.md` rule 7 and
  `crates/oxfunc_core/src/functions/database_family.rs` at commit `473efa3`.
- `data/functions/FUNC.DGET.json` (which records `error_collapse_profile: None` for this member
  alone), `data/presence/FUNC.DGET.json`.
