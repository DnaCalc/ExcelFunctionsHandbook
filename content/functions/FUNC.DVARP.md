---
schema: efh.function-page/v1
function_id: FUNC.DVARP
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
  The population variance of the field column over matching records; the divisor-n member of the
  statistical quartet, and the one that is defined on a single selected value where DVAR is not.
---

# DVARP

## What it computes

`DVARP(database, field, criteria)` selects the records of *database* that satisfy *criteria*,
reads the column named by *field*, and returns the **population** variance of the numeric values
found there.

With V = [ v₁ … vₙ ] the Numbers collected from the field column of the matching records, in
record order, and n = |V|:

    x̄  = (Σ vᵢ) / n
    σ² = ( Σ (vᵢ − x̄)² ) / n

The only difference from [DVAR](FUNC.DVAR.md) is the divisor: n rather than n − 1. That single
change has two consequences worth stating separately, because readers usually notice the first
and are caught by the second:

1. `DVARP` is always the smaller of the two, by the factor (n − 1)/n.
2. `DVARP` is **defined for n = 1** — the variance of a one-element population is exactly zero —
   whereas `DVAR` requires two values. The two functions therefore disagree about whether a
   given selection is an error at all, not merely about the number.

Choosing between them is a modelling question, not a numerical one: divide by n when the
selected records *are* the whole population you are describing, and by n − 1 when they are a
sample drawn from a larger one. A criteria-filtered subset of a spreadsheet is very often the
whole population of interest, which is why `DVARP` is less exotic here than `VAR.P` is in
general statistics.

Non-numeric cells in the field column — text, logicals, blanks — are skipped, so n counts
numbers, not records.

The selection stage — header matching, AND across a criteria row, OR down the rows, blank
criteria cells, and criteria-expression parsing — is identical for all twelve database functions
and is explained once in
[the criteria-range mechanism](FUNC.DAVERAGE.md#the-criteria-range-mechanism).

The numerical account of *which* variance formula is used — the well-conditioned two-pass form
that OxFunc implements, versus the one-pass sum-of-squares form that loses precision on
offset data — is given in full on
[the DVAR page](FUNC.DVAR.md#why-the-formula-not-just-the-definition) and applies here
unchanged.

## Arguments

Three arguments, all required at the call site; the reference implementation declares an arity
of exactly 3.

- **`database`** — the header-plus-records range or array.
- **`field`** — the column selector, as header text or as a 1-based column index within the
  database range. Required.
- **`criteria`** — the header-plus-conditions range or array.

## Result and edge cases

Returns a Number, always non-negative under the two-pass form.

- **No numeric value selected**: `#DIV/0!`. Unlike the count and sum members, the statistical
  members do error on an empty selection.
- **Exactly one numeric value selected**: `0`, not an error. This is the sharpest observable
  difference from [DVAR](FUNC.DVAR.md), which returns `#DIV/0!` on the same input.
- **All selected values equal**: `0`, exactly, under the two-pass form.
- **Text that looks numeric in the field column** is skipped, per the range-scan side of the
  asymmetry in
  [coercion and lifting](../model/02-coercion-and-lifting.md#direct-arguments-versus-range-scans).
- **Logicals in the field column** are skipped.
- **An error value in a scanned field cell** surfaces; the family declares
  `ErrorCollapseProfile::ReductionFold`.
- **Arrays**: inline array literals resolve to grids and are accepted; `DVARP` is not a lift
  kernel.

## Errors

| Error | Condition |
|---|---|
| `#DIV/0!` | No numeric value was collected from the field column of the matching records |
| `#VALUE!` | *database* or *criteria* is not a grid or is empty; a database header cell is blank; a criteria header names no database field; *field* matches no header, or is a non-integer or out-of-range column index |
| propagated | An error value in a header cell, a criteria cell, or a scanned field cell surfaces as that error |

Microsoft's page documents the argument roles, the criteria conventions, and that the function
calculates variance over an entire population; the error conditions are stated from OxFunc's
implementation and family contract and have not been compared against Excel by the Handbook.

## Relationships

- **[DVAR](FUNC.DVAR.md)** is the sample form. `DVARP × n = DVAR × (n − 1)` whenever both are
  defined.
- **[DSTDEVP](FUNC.DSTDEVP.md)** is `SQRT(DVARP)`.
- **Nearest modern equivalent**: `VAR.P` over a filtered array. Excel's compatibility rename
  pass produced the `VARP` / `VAR.P` pair; the database family was left alone, so `DVARP` is not
  a deprecated name and has no `.P`-suffixed replacement.
- **Confused with**: `DVAR` (the divisor and the one-value boundary) and `VARPA`, which counts
  logicals and text as values.

## Notes for implementers

1. **Guard on n = 0 only.** The sample and population members differ in exactly this predicate;
   sharing the code path and switching only the divisor, without also switching the minimum
   count, is the natural bug here.
2. **Use the two-pass form** — see [DVAR](FUNC.DVAR.md#why-the-formula-not-just-the-definition).
3. **The n = 1 answer should be exactly `+0`**, not a denormal residual. The two-pass form gives
   that for free; a sum-of-squares form may not.
4. **Skip, do not coerce.** Admit only cells whose value kind is Number.
5. **Numeric criteria comparison is not exact double comparison** — see the note in
   [the shared mechanism](FUNC.DAVERAGE.md#numeric-comparison-is-not-exact-double-comparison).
   A tolerant match changes n, and here n is the divisor.

## What has not been checked

No Handbook vector suite exists for `DVARP`, and no Excel-comparison evidence record is recorded
for it. Nobody has run a Handbook-owned comparison of this function against Excel.

Inputs I would probe first:

1. **The one-value selection.** Criteria matching exactly one record with a numeric field cell.
   `DVARP` should give `0` and `DVAR` should give `#DIV/0!` on the identical input; running both
   side by side tests the divisor split and the minimum-count split at once.
2. **The empty selection**, confirming `#DIV/0!` rather than `0`.
3. **The conditioning probe** from [DVAR](FUNC.DVAR.md): `1e9 + 1`, `1e9 + 2`, `1e9 + 3`, whose
   true population variance is exactly 2/3.
4. **The exact ratio.** The same selection through `DVAR` and `DVARP`, checking whether
   `DVARP × n` and `DVAR × (n − 1)` agree to the last bit or only to within rounding — which
   reveals whether the two share one accumulation.
5. **All-equal values**, checking for exact `0`.
6. **A matching set with blanks interleaved**, confirming n counts numbers rather than records.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| database grid | The header-plus-records rectangle passed as *database* |
| criteria grid | The header-plus-conditions rectangle passed as *criteria* |
| field column | The database column selected by the *field* argument |
| value list | The Numbers collected from the field column of the matching records |
| n | The length of the value list — a count of numbers, not of records |
| population divisor | Dividing the summed squared deviations by n rather than n − 1 |

## Sources

- Microsoft, "DVARP function" —
  <https://support.microsoft.com/en-us/office/dvarp-function-eb0ba387-9cb7-45c8-81e9-0394912502fc>
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md`.
- Handbook call-model chapters 02 and 03 (range-scan coercion, error folding).
- [DAVERAGE](FUNC.DAVERAGE.md) — the family's shared criteria-range mechanism;
  [DVAR](FUNC.DVAR.md) — the shared numerical account of the variance formula.
- OxFunc `docs/function-lane/FUNCTION_SLICE_DATABASE_FAMILY_CONTRACT_PRELIM.md` rule 9, and
  `crates/oxfunc_core/src/functions/variance_common.rs` at commit `473efa3` (the divisor split
  and the minimum-count checks).
- `data/functions/FUNC.DVARP.json`, `data/presence/FUNC.DVARP.json`.
