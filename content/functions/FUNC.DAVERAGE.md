---
schema: efh.function-page/v1
function_id: FUNC.DAVERAGE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - The criteria-range mechanism
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
  The arithmetic mean of the field column over matching records; the family's anchor page,
  which carries the shared criteria-range mechanism the other eleven members reference.
---

# DAVERAGE

## What it computes

`DAVERAGE(database, field, criteria)` selects the records of *database* that satisfy
*criteria*, reads one column of those records — the column named by *field* — and returns the
arithmetic mean of the numeric values it finds there.

Written out: let R be the set of record rows selected by the criteria mechanism below, let c be
the resolved field column, and let

    V = [ v : v = value of cell (r, c) for r in R, in record order, where v is a Number ]

Then the result is (Σ V) / |V|. Non-numeric cells in the field column — text, logicals, blanks —
are not in V at all: they are neither counted in the denominator nor treated as zero. If V is
empty the quotient is undefined and the function is in error territory (see *Result and edge
cases*).

Two things this definition deliberately makes explicit, because Microsoft's one-line
description ("Returns the average of selected database entries") does not:

1. **Selection and aggregation are separate stages.** The criteria mechanism picks *rows*; the
   field argument picks a *column*; the aggregate runs over the intersection. Every one of the
   twelve D-functions shares the first two stages exactly and differs only in the third.
2. **The denominator is a count of numbers, not of records.** A record can match the criteria
   and still contribute nothing, if its cell in the field column is blank or non-numeric.

## The criteria-range mechanism

*This section is the family's shared explanation. The other eleven database functions —
[DCOUNT](FUNC.DCOUNT.md), [DCOUNTA](FUNC.DCOUNTA.md), [DGET](FUNC.DGET.md),
[DMAX](FUNC.DMAX.md), [DMIN](FUNC.DMIN.md), [DPRODUCT](FUNC.DPRODUCT.md),
[DSTDEV](FUNC.DSTDEV.md), [DSTDEVP](FUNC.DSTDEVP.md), [DSUM](FUNC.DSUM.md),
[DVAR](FUNC.DVAR.md), [DVARP](FUNC.DVARP.md) — link here instead of repeating it.*

### The database grid

*database* is a rectangular range or array. Its **first row is a header row**: each cell of it
names a field. Every row below the first is one **record**. The header row is never a record
and never participates in matching as data — it participates only as a set of column names.

Header cells are label-valued: text is used as written; a number or a logical in the header row
is turned into its label form for matching purposes. A blank header cell leaves that column
unnameable, and an error value anywhere in the header row is a hard failure rather than a
skipped column.

### The field argument

*field* names one column of *database*, in either of two forms:

- **as text** — the header label of the column, e.g. `"Yield"`;
- **as a number** — the 1-based column index counting from the left edge of *database*, so `1`
  is the leftmost column of the database range, not of the worksheet.

Both forms are documented by Microsoft. The index form is the position within the given range,
which is the argument position most often misread when the database range does not start at
column A.

### The criteria grid

*criteria* is a rectangular range or array with **its own header row** and one or more rows of
conditions beneath it. It is a small second table whose column headers say which database field
each condition applies to. Its shape is independent of the database's: it may name a subset of
the fields, in any order, and may name the same field more than once.

The matching rule has four parts, and they compose in exactly this order:

1. **Header matching.** For each column of the criteria grid that holds a condition, the
   criteria header is matched against the database's header labels to find the target column.
   The criteria header is what binds a condition to a field — position is irrelevant.
2. **AND across a row.** Within one criteria row, every populated condition cell must hold. A
   record matches that row only if it satisfies all of them.
3. **OR down the rows.** A record is selected if it satisfies *any* criteria row. Multiple rows
   are alternatives.
4. **Blank means no condition.** An empty criteria cell contributes no predicate. This has a
   consequence readers hit in practice: a criteria row that is entirely blank imposes nothing at
   all, so it matches every record and, by rule 3, makes the whole criteria range vacuous. An
   accidental blank row inside the criteria range silently turns a filtered aggregate into an
   unfiltered one. Likewise, a criteria range consisting of a header row with no condition rows
   beneath it selects every record.

### How a criteria cell is parsed

A populated criteria cell is compiled into a comparison operator plus an operand:

| Cell content | Operator | Operand |
|---|---|---|
| A number | `=` | that number |
| A logical | `=` | that logical |
| Text starting with `<=`, `>=`, `<>`, `<`, `>` or `=` | that operator | the remainder of the text, re-parsed |
| Any other text | `=` with **prefix** semantics | the text |
| Text that is empty, or an operator with nothing after it | the operator | the *blank* operand |

The re-parse of the remainder is ordered: it is tried as a number first, then as the logical
words `TRUE`/`FALSE`, and only then kept as text. So `">=100"` is a numeric comparison and
`">=abc"` is a text comparison, decided by what the right-hand side looks like, not by the
column's contents.

Three consequences worth stating separately:

- **Bare text is a prefix match, not an equality match.** A criteria cell containing `Nor`
  selects `Norway` and `Northwind`. Writing `="Nor"` — an explicit `=` operator — asks for
  whole-value equality instead. This is the single most common surprise in the family, and it is
  documented behaviour on Microsoft's side.
- **`*` and `?` are wildcards** in equality and inequality comparisons on text, with `~` as the
  escape prefix for a literal `*` or `?`.
- **The blank operand is how you test emptiness.** A cell containing just `=` selects records
  whose field cell is blank; a cell containing just `<>` selects records whose field cell is not
  blank.

Matching is kind-sensitive: a numeric condition tests numeric cells (and text cells that parse
as numbers); a logical condition tests logical cells (and text that reads as `TRUE`/`FALSE`); a
text condition tests text cells. A number in the data is not matched by a text condition, and
vice versa. Text comparison in OxFunc's implementation is case-insensitive.

### Numeric comparison is not exact double comparison

The criteria comparison path is not plain IEEE-754 equality. OxFunc's bug record
`BUG-FUNC-004`, signed off against live Excel 16.0 build 20026, records that the shared
criteria/database comparison helper normalizes both sides by truncation to fifteen significant
decimal digits, so that `0.1+0.2` compares equal to `0.3`, while the exact-match lookup family
(`MATCH`, `XMATCH`, `DELTA`) stays exact on the same inputs. That is an upstream record with its
own scope — one Excel build, one platform, the probe rows listed in it — not a Handbook claim
about all Excel builds. It is repeated here because an implementer who reaches for `==` on
doubles will diverge from that record immediately, and because the same helper serves the
`COUNTIF`/`SUMIF` criteria family.

### What is outside the described mechanism

OxFunc's family contract explicitly excludes criteria formulas evaluated in full worksheet
context — the "computed criteria" idiom, where a criteria header names no field and the cell
below holds a formula referring to the first record. The Handbook has not checked what Excel
does with such a criteria cell, and nothing on these twelve pages describes it. Locale-sensitive
parsing of criteria text beyond plain numbers and the six comparison operators is likewise
outside the described slice.

## Arguments

Three arguments, all required at the call site; the reference implementation declares an arity
of exactly 3.

- **`database`** — the header-plus-records range or array. A scalar is not a table and fails.
- **`field`** — the column selector: header text or 1-based column index within *database*. For
  `DAVERAGE` the field is genuinely required; the omitted-field convenience exists only for
  [DCOUNT](FUNC.DCOUNT.md) and [DCOUNTA](FUNC.DCOUNTA.md).
- **`criteria`** — the header-plus-conditions range or array described above.

Note the interaction with the call model: leaving the middle slot empty, as in
`DAVERAGE(A1:C9,,E1:F2)`, still passes three arguments. The empty slot delivers the `Missing`
marker described in [the value universe](../model/01-value-universe.md#missing-versus-empty),
not a shorter call, which is why an arity of exactly 3 and an "omitted field" behaviour are not
in conflict.

All three arguments are reference-shaped in ordinary use; the family is declared
`ArgPreparationProfile::RefsVisibleInAdapter` in [the call pipeline](../model/03-call-pipeline.md),
so the references arrive live and are resolved to grids by the function itself.

## Result and edge cases

Returns a Number.

- **No record matches**, or every matching record has a non-numeric field cell: the value list
  is empty, the mean is undefined, and OxFunc returns `#DIV/0!`.
- **Text that looks numeric in the field column** is not averaged. The field-column scan admits
  only cells whose value kind is Number; a cell holding the text `"3"` is skipped. This follows
  the range-scan side of the direct-versus-scan asymmetry described in
  [coercion and lifting](../model/02-coercion-and-lifting.md#direct-arguments-versus-range-scans),
  and the family is registered there as carrying its own policy row.
- **Logicals in the field column** are skipped for the same reason.
- **Blank cells in the field column** are skipped — they do not pull the mean toward zero.
- **An error value in a scanned field cell** surfaces; the family's declared
  `ErrorCollapseProfile::ReductionFold` folds error inputs by Excel's legacy precedence order.
- **Arrays**: an inline array literal is accepted wherever a range is, because both resolve to a
  grid. `DAVERAGE` is not a lift kernel — an array argument is consumed whole, never mapped
  elementwise.

## Errors

As implemented in OxFunc at the curated commit, and consistent with the family contract:

| Error | Condition |
|---|---|
| `#VALUE!` | *database* or *criteria* is not a grid, or is empty; a header cell in *database* is blank; a criteria header names no database field; *field* is text that matches no header, a non-integer or out-of-range column index, or a value of an unusable kind |
| `#DIV/0!` | No numeric value was collected from the field column of the matching records |
| propagated | An error value in a header cell, a criteria cell, or a scanned field cell surfaces as that error |

Microsoft's page for the function documents the argument roles and the criteria conventions; it
does not enumerate the error conditions in this table. Everything above is stated from OxFunc's
implementation and family contract, not from Microsoft's documentation, and none of it has been
compared against Excel by the Handbook.

## Relationships

- **Siblings**: the twelve-member database family, all sharing the mechanism above and differing
  only in the aggregate. See the family list at the head of that section.
- **Nearest modern equivalent**: `AVERAGEIFS`, which expresses the same filtered-average idea
  with inline condition arguments instead of a criteria grid. It is not a replacement — Excel
  supports both, and the D-functions remain the only members of this group that take criteria as
  a *worksheet-resident table*, which is what makes them useful when the conditions are meant to
  be edited by the user rather than by the formula author.
- **Confused with**: `AVERAGE` (no selection stage), `AVERAGEA` (counts text and logicals),
  and `SUBTOTAL`/`AGGREGATE` (filter-aware, but by hidden rows rather than by criteria).
- **The criteria vocabulary is shared with** `COUNTIF`, `SUMIF`, and the `*IFS` family: the
  operator prefixes, prefix matching, and wildcard handling are the same idea, and OxFunc routes
  their numeric comparisons through the same helper.

## Notes for implementers

1. **Resolve in the documented order.** Parse the database grid, then the criteria grid, then
   the field index, then match. The field index must be resolved against the database headers,
   which means a bad field argument is detectable before any row is examined — and the order
   determines which error wins when more than one argument is wrong.
2. **Do not use `==` on doubles for numeric criteria.** See the comparison note above.
3. **Reduction order is observable.** The family declares a sequential left fold; summing the
   matched values in record order and dividing at the end is not the same in the last bits as
   any reassociated or pairwise sum. Two implementations that disagree only in summation order
   will disagree on long, badly-scaled columns.
4. **Case-insensitive header matching** is what OxFunc does for both the field argument and the
   criteria headers. Whether Excel agrees, and how either treats leading or trailing spaces in a
   header, is unchecked.
5. **The empty criteria grid is a real input.** A criteria range with only a header row selects
   every record. So does a criteria row whose cells are all blank. Both are easy to get wrong by
   treating "no predicates" as "no matches".

## What has not been checked

No Handbook vector suite exists for `DAVERAGE`, and no Excel-comparison evidence record is
recorded for it. The mechanism above is drawn from Microsoft's documentation for the argument
roles and criteria conventions, and from OxFunc's family contract and implementation for
everything else; nobody has run a Handbook-owned comparison of this function against Excel.

The inputs that would settle the most, in the order I would probe them:

1. **The all-blank criteria row.** `DAVERAGE` over a criteria range whose second row is empty.
   The Handbook asserts this selects every record; that follows from AND-over-no-predicates, but
   it is exactly the kind of vacuous-truth case an implementation can get backwards.
2. **Duplicate criteria headers in one row.** OxFunc's family contract states that duplicate
   criteria headers within a single row are OR'd; the module's row compiler, read at the curated
   commit, requires every populated cell in a row to match, which is AND. These two statements
   disagree, and the Handbook does not resolve the disagreement here. The probe is a criteria
   row with two columns both headed with the same field name, holding `>10` and `<20`: an AND
   reading selects the band between them, an OR reading selects almost everything.
3. **The empty-result path.** A criteria range matching no record, and a criteria range matching
   records whose field cells are all blank or all text. `#DIV/0!` is what OxFunc returns; whether
   Excel distinguishes those two situations is unchecked.
4. **Prefix versus equality.** `Nor` against `Norway` and `="Nor"` against `Norway`, plus the
   wildcard and `~`-escaped forms, and the same probes with trailing spaces in the data.
5. **Kind-crossing matches.** The numeric criterion `100` against a field cell holding the text
   `"100"`, and the text criterion `100` (entered as text) against a numeric cell.
6. **Near-equal numeric criteria.** `0.1+0.2` as a criterion against a field cell holding `0.3`,
   which is the `BUG-FUNC-004` lane, re-run under the Handbook's own observation context rather
   than inherited from OxFunc's.
7. **Field-index boundaries.** Index `0`, index one past the last column, and a fractional
   index.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| database grid | The header-plus-records rectangle passed as *database* |
| criteria grid | The header-plus-conditions rectangle passed as *criteria* |
| record | One row of the database grid below the header row |
| field column | The database column selected by the *field* argument |
| prefix match | Bare criteria text matches any value beginning with that text |
| blank operand | A criteria cell of `=` or `<>` alone, testing emptiness of the field cell |
| value list | The Numbers collected from the field column of the matching records |

## Sources

- Microsoft, "DAVERAGE function" —
  <https://support.microsoft.com/en-us/office/daverage-function-a6a2d5ac-4b4b-48cd-a1d8-7b37834e5aee>
  (argument roles, the criteria-range convention, prefix and wildcard matching).
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md` (claim
  language and the scoping rules this page is written under).
- Handbook call-model chapters 01, 02 and 03 (value kinds, Missing versus Empty, the
  direct-argument versus range-scan asymmetry, argument preparation and error folding).
- OxFunc `docs/function-lane/FUNCTION_SLICE_DATABASE_FAMILY_CONTRACT_PRELIM.md` — the admitted
  semantic slice for the twelve members, and the out-of-scope list. Preliminary by its own
  status line.
- OxFunc `docs/function-lane/FUNCTION_SLICE_DATABASE_FAMILY_CURRENT_BASELINE_PROMOTION_PRELIM.md`
  — the same rules carried into the current-baseline snapshot.
- OxFunc `docs/bugs/streams/BUG-FUNC-004_numeric_comparison_tolerance_family_split.md` — the
  truncation-style fifteen-significant-digit comparison lane, with its own observation context
  (live Excel 16.0 build 20026).
- OxFunc `crates/oxfunc_core/src/functions/database_family.rs` at commit `473efa3` — the
  implementation the behavioural statements above describe.
- `data/functions/FUNC.DAVERAGE.json`, `data/presence/FUNC.DAVERAGE.json` (arity, classification
  axes, implementing module).
