---
schema: efh.function-page/v1
function_id: FUNC.DSUM
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
  The total of the field column over matching records; the family's plainest aggregate, and the
  one whose empty-selection answer (zero, not an error) sets it apart from DAVERAGE and the
  variance members.
---

# DSUM

## What it computes

`DSUM(database, field, criteria)` selects the records of *database* that satisfy *criteria*,
reads the column named by *field*, and returns the sum of the numeric values found there.

With R the set of selected record rows, c the resolved field column, and

    V = [ v : v = value of cell (r, c) for r in R, in record order, where v is a Number ]

the result is Σ V, summed as a left fold in record order. An empty V sums to 0 — `DSUM` has no
empty-selection error, which is the main behavioural difference between it and
[DAVERAGE](FUNC.DAVERAGE.md).

Non-numeric cells in the field column contribute nothing: text (including text that reads as a
number), logicals, and blanks are skipped rather than coerced to zero. The distinction matters
because "skipped" and "added as zero" give the same total but different behaviour in the sibling
aggregates that count or divide.

The selection stage — how *criteria* picks records, how header matching works, what AND across a
row and OR down the rows mean, what a blank criteria cell means, and how a criteria expression
is parsed — is identical for all twelve database functions and is explained once in
[the criteria-range mechanism](FUNC.DAVERAGE.md#the-criteria-range-mechanism).

## Arguments

Three arguments, all required at the call site; the reference implementation declares an arity
of exactly 3.

- **`database`** — the header-plus-records range or array; the first row is the header row.
- **`field`** — the column selector, as header text or as a 1-based column index *within the
  database range*. Required for `DSUM`; only [DCOUNT](FUNC.DCOUNT.md) and
  [DCOUNTA](FUNC.DCOUNTA.md) accept an omitted field.
- **`criteria`** — the header-plus-conditions range or array.

Argument roles and admissible values are the family's; see
[the shared mechanism](FUNC.DAVERAGE.md#the-criteria-range-mechanism) for the full account,
including the point that a criteria range with no condition rows, or with an all-blank condition
row, selects every record.

## Result and edge cases

Returns a Number.

- **No record matches**: the sum of nothing is `0`. No error.
- **Matching records whose field cells are all blank or all text**: also `0`, by the same route.
  `DSUM` cannot distinguish "nothing matched" from "everything that matched was empty"; if you
  need that distinction, pair it with [DCOUNT](FUNC.DCOUNT.md).
- **Text that looks numeric in the field column** is skipped, not parsed. This is the range-scan
  side of the asymmetry described in
  [coercion and lifting](../model/02-coercion-and-lifting.md#direct-arguments-versus-range-scans).
- **Logicals in the field column** are skipped.
- **An error value in a scanned field cell** surfaces; the family declares
  `ErrorCollapseProfile::ReductionFold`, folding error inputs by Excel's legacy precedence order.
- **Arrays**: inline array literals are accepted wherever ranges are, since both resolve to a
  grid. `DSUM` is not a lift kernel; array arguments are consumed whole.

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | *database* or *criteria* is not a grid or is empty; a database header cell is blank; a criteria header names no database field; *field* matches no header, or is a non-integer or out-of-range column index |
| propagated | An error value in a header cell, a criteria cell, or a scanned field cell surfaces as that error |

`DSUM` has no domain error of its own — no divisor, no minimum sample size. Every error it can
return comes from argument shape, field resolution, or a propagated input error.

Microsoft's page documents the argument roles and criteria conventions; it does not enumerate
these conditions. The table is stated from OxFunc's implementation and family contract, and has
not been compared against Excel by the Handbook.

## Relationships

- **Siblings**: the twelve database functions, sharing the selection mechanism and differing
  only in the aggregate. [DCOUNT](FUNC.DCOUNT.md) counts what `DSUM` adds;
  [DPRODUCT](FUNC.DPRODUCT.md) multiplies it; [DAVERAGE](FUNC.DAVERAGE.md) is `DSUM` over
  `DCOUNT` except at the empty-selection boundary, where `DSUM` returns `0` and `DAVERAGE`
  returns `#DIV/0!`.
- **Nearest modern equivalent**: `SUMIFS` (and `SUMIF`), which take conditions as inline
  arguments rather than as a worksheet-resident criteria table. Neither supersedes the other;
  Excel supports both.
- **Confused with**: `SUM` (no selection stage), `SUMPRODUCT` (often used to emulate criteria
  arithmetically), and `SUBTOTAL`, which filters by hidden rows rather than by criteria.

## Notes for implementers

1. **Reduction order is observable.** The family declares a sequential left fold, so the
   accumulation runs in record order. Pairwise or reassociated summation will differ in the last
   bits on long or badly-scaled columns, and no compensated-summation scheme is implied.
2. **Skip, do not coerce.** The scan admits only cells whose value kind is Number. Coercing
   `"3"` to 3 here would change `DSUM`, `DCOUNT`, and `DAVERAGE` together and in the wrong
   direction.
3. **Numeric criteria comparison is not exact double comparison** — see the note in
   [the shared mechanism](FUNC.DAVERAGE.md#numeric-comparison-is-not-exact-double-comparison).
4. **The zero answer is load-bearing.** Returning an error for an empty selection would be a
   different function; several of the siblings *do* error there, so a shared implementation must
   branch on the aggregate rather than on the selection.

## What has not been checked

No Handbook vector suite exists for `DSUM`, and no Excel-comparison evidence record is recorded
for it. Nobody has run a Handbook-owned comparison of this function against Excel.

Inputs I would probe first, and why:

1. **Empty selection.** A criteria range matching no record. The Handbook states `0`; the
   variance and average siblings error on the same input, so the boundary is where a wrong
   shared implementation shows itself.
2. **A field column of numeric-looking text.** Records that match, whose field cells hold
   `"1"`, `"2"`, `"3"`. Skipping gives `0`; coercing gives `6`. This single probe separates the
   two plausible scan policies.
3. **Summation order and scale.** A matched column of alternating large and small magnitudes —
   for example `1e16, 1, 1, -1e16` in record order — where the left-fold answer differs from a
   reassociated one. Comparing against Excel here would test the declared reduction policy, not
   just the selection stage.
4. **Errors mid-column.** A matching record whose field cell holds `#N/A` while another holds
   `#DIV/0!`, to observe which error survives the fold.
5. **The all-blank criteria row**, which the Handbook says selects every record, so that `DSUM`
   degenerates to `SUM` over the field column.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| database grid | The header-plus-records rectangle passed as *database* |
| criteria grid | The header-plus-conditions rectangle passed as *criteria* |
| field column | The database column selected by the *field* argument |
| value list | The Numbers collected from the field column of the matching records |
| left fold | Accumulation in record order, one value at a time, with no reassociation |

## Sources

- Microsoft, "DSUM function" —
  <https://support.microsoft.com/en-us/office/dsum-function-53181285-0c4b-4f5a-aaa3-529a322be41b>
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md`.
- Handbook call-model chapters 02 and 03 (range-scan coercion, error folding, reduction policy).
- [DAVERAGE](FUNC.DAVERAGE.md) — the family's shared criteria-range mechanism.
- OxFunc `docs/function-lane/FUNCTION_SLICE_DATABASE_FAMILY_CONTRACT_PRELIM.md` and
  `crates/oxfunc_core/src/functions/database_family.rs` at commit `473efa3`.
- `data/functions/FUNC.DSUM.json`, `data/presence/FUNC.DSUM.json`.
