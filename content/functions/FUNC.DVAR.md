---
schema: efh.function-page/v1
function_id: FUNC.DVAR
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
  The sample variance of the field column over matching records; the family's conditioning
  case, because its answer depends on which algebraically equivalent formula the implementation
  uses, and the member that carries the fullest account of that for its three statistical
  siblings.
---

# DVAR

## What it computes

`DVAR(database, field, criteria)` selects the records of *database* that satisfy *criteria*,
reads the column named by *field*, and returns the **sample** variance of the numeric values
found there.

With R the set of selected record rows, c the resolved field column,

    V = [ v₁ … vₙ ] = the Numbers in cell (r, c) for r in R, in record order

and n = |V|, the result is

    x̄ = (Σ vᵢ) / n
    s² = ( Σ (vᵢ − x̄)² ) / (n − 1)

Sample variance divides by n − 1, so at least two values are required; with fewer the divisor is
zero or negative and the function is in error territory (see below).

Non-numeric cells in the field column — text, logicals, blanks — are skipped, so n counts
numbers, not records. A matching record with a blank field cell reduces n by one; it does not
contribute a zero.

The selection stage — header matching, AND across a criteria row, OR down the rows, blank
criteria cells, and criteria-expression parsing — is identical for all twelve database functions
and is explained once in
[the criteria-range mechanism](FUNC.DAVERAGE.md#the-criteria-range-mechanism).

### Why the formula, not just the definition

The two textbook formulas for variance are algebraically identical and numerically are not:

    (A)  two-pass:      s² = Σ(vᵢ − x̄)² / (n − 1)
    (B)  sum-of-squares: s² = ( Σvᵢ² − (Σvᵢ)²/n ) / (n − 1)

Form (B) is one pass and subtracts two large nearly-equal quantities, so it loses precision
catastrophically when the mean is large relative to the spread — and can return a negative
variance. Form (A) needs two passes but is well-conditioned. OxFunc computes form (A): it takes
the mean first, then accumulates squared deviations. Which form Excel uses on this path is not
recorded here, and the difference is observable, which is why the probe list below leads with a
constant-offset dataset.

## Arguments

Three arguments, all required at the call site; the reference implementation declares an arity
of exactly 3.

- **`database`** — the header-plus-records range or array.
- **`field`** — the column selector, as header text or as a 1-based column index within the
  database range. Required.
- **`criteria`** — the header-plus-conditions range or array.

## Result and edge cases

Returns a Number, always non-negative when computed by form (A).

- **Fewer than two numeric values selected** — no match, one match, or matches whose field cells
  are mostly blank: `#DIV/0!`. This is the boundary that separates `DVAR` from
  [DVARP](FUNC.DVARP.md), which needs only one value.
- **All selected values equal**: `0`, exactly, under form (A) — every deviation is exactly zero.
  Under form (B) it need not be.
- **Text that looks numeric in the field column** is skipped, per the range-scan side of the
  asymmetry in
  [coercion and lifting](../model/02-coercion-and-lifting.md#direct-arguments-versus-range-scans).
- **Logicals in the field column** are skipped.
- **An error value in a scanned field cell** surfaces; the family declares
  `ErrorCollapseProfile::ReductionFold`.
- **Arrays**: inline array literals resolve to grids and are accepted; `DVAR` is not a lift
  kernel.

## Errors

| Error | Condition |
|---|---|
| `#DIV/0!` | Fewer than two numeric values were collected from the field column of the matching records |
| `#VALUE!` | *database* or *criteria* is not a grid or is empty; a database header cell is blank; a criteria header names no database field; *field* matches no header, or is a non-integer or out-of-range column index |
| propagated | An error value in a header cell, a criteria cell, or a scanned field cell surfaces as that error |

Microsoft's page documents the argument roles, the criteria conventions, and that the function
estimates variance from a sample; the error conditions above are stated from OxFunc's
implementation and family contract and have not been compared against Excel by the Handbook.

## Relationships

- **[DVARP](FUNC.DVARP.md)** is the population form: divisor n instead of n − 1, and one value
  is enough. `DVAR × (n − 1) = DVARP × n` whenever both are defined.
- **[DSTDEV](FUNC.DSTDEV.md)** is `SQRT(DVAR)` — the same selection, the same estimator, one
  extra operation.
- **Nearest modern equivalents**: `VAR.S` combined with a filtered array, or `VAR.S` over a
  `FILTER` result in a dynamic-array workbook. Excel's own compatibility pairing is `VAR`
  (legacy) / `VAR.S` (modern); the database family was not renamed in that pass and `DVAR` keeps
  its name and its sample semantics.
- **Confused with**: `VAR.P`/`DVARP` (the divisor), and `VARA`, which counts logicals and text.

## Notes for implementers

1. **Use the two-pass form.** It is what OxFunc does, it is well-conditioned, and it makes the
   all-equal case exactly zero.
2. **Both passes are sequential left folds** in record order, per the family's declared reduction
   policy. That fixes the mean's last bits, which in turn fixes every deviation.
3. **The minimum-count check precedes the arithmetic**, so `#DIV/0!` is a domain decision, not a
   division that happened to have a zero denominator.
4. **Skip, do not coerce.** Admit only cells whose value kind is Number; n is the count of
   collected numbers.
5. **Numeric criteria comparison is not exact double comparison** — see the note in
   [the shared mechanism](FUNC.DAVERAGE.md#numeric-comparison-is-not-exact-double-comparison).
   For the statistical members this has a second-order effect worth noting: a tolerant match can
   change n, and n appears in the divisor.

## What has not been checked

No Handbook vector suite exists for `DVAR`, and no Excel-comparison evidence record is recorded
for it. Nobody has run a Handbook-owned comparison of this function against Excel.

Inputs I would probe first:

1. **The conditioning probe.** A matching field column of `1e9 + 1`, `1e9 + 2`, `1e9 + 3`. The
   true sample variance is `1`. Form (A) returns `1`; form (B) returns a badly wrong value, and
   on harsher offsets a negative one. This single dataset identifies which formula Excel uses,
   which is the highest-value unknown on this page.
2. **The count boundary.** Selections of exactly zero, one, and two numeric values, to pin where
   `#DIV/0!` starts and stops.
3. **All-equal values**, checking for an exact `0` rather than a tiny residual.
4. **A matching set with blanks interleaved**, confirming that n counts numbers and not records
   — the divisor is directly observable here.
5. **A field column of numeric-looking text**, separating "skipped" from "coerced"; this also
   changes n.
6. **Summation-order sensitivity**: the same multiset of values in two record orders, on data
   scaled so the mean's last bits differ between them.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| database grid | The header-plus-records rectangle passed as *database* |
| criteria grid | The header-plus-conditions rectangle passed as *criteria* |
| field column | The database column selected by the *field* argument |
| value list | The Numbers collected from the field column of the matching records |
| n | The length of the value list — a count of numbers, not of records |
| two-pass form | Mean first, then squared deviations; the well-conditioned formula |
| sum-of-squares form | The one-pass formula that subtracts nearly equal large quantities |

## Sources

- Microsoft, "DVAR function" —
  <https://support.microsoft.com/en-us/office/dvar-function-d6747ca9-99c7-48bb-996e-9d7af00f3ed1>
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md`.
- Handbook call-model chapters 02 and 03 (range-scan coercion, error folding, reduction policy).
- [DAVERAGE](FUNC.DAVERAGE.md) — the family's shared criteria-range mechanism.
- OxFunc `docs/function-lane/FUNCTION_SLICE_DATABASE_FAMILY_CONTRACT_PRELIM.md` rule 9 (the
  sample/population split) and `crates/oxfunc_core/src/functions/database_family.rs` plus
  `crates/oxfunc_core/src/functions/variance_common.rs` at commit `473efa3` — the two-pass
  formulation and the minimum-count checks.
- `data/functions/FUNC.DVAR.json`, `data/presence/FUNC.DVAR.json`.
