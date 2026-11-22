---
schema: efh.function-page/v1
function_id: FUNC.DSTDEVP
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
  The population standard deviation of the field column over matching records — DVARP followed
  by a square root — and the statistical member with the loosest requirement on the selection,
  since a single matching value is enough.
---

# DSTDEVP

## What it computes

`DSTDEVP(database, field, criteria)` selects the records of *database* that satisfy *criteria*,
reads the column named by *field*, and returns the **population** standard deviation of the
numeric values found there.

With V = [ v₁ … vₙ ] the Numbers collected from the field column of the matching records, in
record order, and n = |V|:

    x̄ = (Σ vᵢ) / n
    σ = √( Σ (vᵢ − x̄)² / n )

Operationally it is [DVARP](FUNC.DVARP.md) followed by one square root; the reference
implementation computes it exactly that way. The divisor is n, not n − 1, so a single numeric
value is enough — and yields exactly `0`, the standard deviation of a one-element population.

Non-numeric cells in the field column — text, logicals, blanks — are skipped, so n counts
numbers, not records.

The selection stage — header matching, AND across a criteria row, OR down the rows, blank
criteria cells, and criteria-expression parsing — is identical for all twelve database functions
and is explained once in
[the criteria-range mechanism](FUNC.DAVERAGE.md#the-criteria-range-mechanism).

Two accounts carry over from the sibling pages and are not repeated here: which variance formula
is used, on [the DVAR page](FUNC.DVAR.md#why-the-formula-not-just-the-definition), and the extra
rounding introduced by taking a square root of an already-rounded variance, on
[the DSTDEV page](FUNC.DSTDEV.md#the-extra-rounding-step).

## Arguments

Three arguments, all required at the call site; the reference implementation declares an arity
of exactly 3.

- **`database`** — the header-plus-records range or array.
- **`field`** — the column selector, as header text or as a 1-based column index within the
  database range. Required.
- **`criteria`** — the header-plus-conditions range or array.

## Result and edge cases

Returns a Number, non-negative.

- **No numeric value selected**: `#DIV/0!`, from the variance stage.
- **Exactly one numeric value selected**: `0`. [DSTDEV](FUNC.DSTDEV.md) returns `#DIV/0!` on the
  same input; this is the cleanest observable difference between the pair.
- **All selected values equal**: `0`, exactly.
- **Text that looks numeric in the field column** is skipped, per the range-scan side of the
  asymmetry in
  [coercion and lifting](../model/02-coercion-and-lifting.md#direct-arguments-versus-range-scans).
- **Logicals in the field column** are skipped.
- **An error value in a scanned field cell** surfaces; the family declares
  `ErrorCollapseProfile::ReductionFold`.
- **Arrays**: inline array literals resolve to grids and are accepted; `DSTDEVP` is not a lift
  kernel.
- **No negative-argument path**, for the same reason as `DSTDEV`: the two-pass variance cannot
  be negative, so the square root has no domain failure.

## Errors

| Error | Condition |
|---|---|
| `#DIV/0!` | No numeric value was collected from the field column of the matching records |
| `#VALUE!` | *database* or *criteria* is not a grid or is empty; a database header cell is blank; a criteria header names no database field; *field* matches no header, or is a non-integer or out-of-range column index |
| propagated | An error value in a header cell, a criteria cell, or a scanned field cell surfaces as that error |

Microsoft's page documents the argument roles, the criteria conventions, and that the function
calculates the standard deviation over an entire population; the error conditions are stated
from OxFunc's implementation and family contract and have not been compared against Excel by the
Handbook.

## Relationships

- **[DSTDEV](FUNC.DSTDEV.md)** is the sample form: divisor n − 1, minimum two values.
- **[DVARP](FUNC.DVARP.md)** is the square of this function.
- **Nearest modern equivalent**: `STDEV.P` over a filtered array. Excel's compatibility rename
  pass produced `STDEVP` / `STDEV.P`; the database family was left out of it, so `DSTDEVP` is
  not a deprecated name and has no `.P`-suffixed replacement.
- **Confused with**: `DSTDEV` (the divisor and the minimum count) and `STDEVPA`, which counts
  logicals and text as values.

## Notes for implementers

1. **Guard on n = 0 only**, and share nothing with the sample member except the accumulation —
   switching the divisor without switching the minimum count is the natural bug in this quartet.
2. **Compose, do not reformulate**: variance first, then one square root.
3. **The n = 1 answer should be exactly `+0`.**
4. **Skip, do not coerce.** Admit only cells whose value kind is Number.
5. **Numeric criteria comparison is not exact double comparison** — see the note in
   [the shared mechanism](FUNC.DAVERAGE.md#numeric-comparison-is-not-exact-double-comparison).

## What has not been checked

No Handbook vector suite exists for `DSTDEVP`, and no Excel-comparison evidence record is
recorded for it. Nobody has run a Handbook-owned comparison of this function against Excel.

Inputs I would probe first:

1. **The one-value selection**, run through `DSTDEVP` and `DSTDEV` side by side: `0` against
   `#DIV/0!`. This pins the divisor split and the minimum-count split in a single probe.
2. **The composition test**: `DSTDEVP` against `SQRT(DVARP(...))` in Excel, on the same
   selection. Agreement everywhere collapses this page's remaining uncertainty onto
   [DVARP](FUNC.DVARP.md).
3. **The empty selection**, confirming `#DIV/0!`.
4. **The conditioning probe** from [DVAR](FUNC.DVAR.md): `1e9 + 1`, `1e9 + 2`, `1e9 + 3`, whose
   true population standard deviation is √(2/3).
5. **All-equal values**, checking for exact `0`.
6. **A two-value selection**, where `DSTDEVP` and `DSTDEV` differ by exactly the factor √2 in
   exact arithmetic — a good check on whether the two share one accumulation or two.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `DSTDEVP` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-DSTDEV-001: DSTDEV, DSTDEVP, DVAR, and DVARP compute variance via raw x87 PC64-stored moments with centered fallback, returning #DIV/0! on <2 (sample) or 0 (population) numbers

*Detail for matching Excel's bits; measured; recorded 2026-11-22.* Applies to DSTDEV, DSTDEVP, DVAR, DVARP.

In Excel 16.0.20527, DSTDEV, DSTDEVP, DVAR, and DVARP collect numbers in matching records'
field cells (ignoring text, logicals, and blanks while propagating errors), reject
non-reference database arguments at formula entry (0x800A03EC), return #DIV/0! on 0 matching
numbers (and on 1 matching number for sample DSTDEV/DVAR, whereas population DSTDEVP/DVARP
return 0), and evaluate variance through Excel's x87 PC64-stored raw-moment sequence (switching
to centered deviations when square_total / count / squares > 0.99, variance * 1000 <= |mean|,
or non-finite, with staged sqrt for DSTDEV/DSTDEVP). All 6,507 DSTDEV, 5,673 DSTDEVP, 5,669
DVAR, and 5,670 DVARP full-bank rows match.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `DSTDEV(A1:B3, "Value", D1:D2)` | database `{"Key","Value";"a0",0;"a1",-49}`; field `Value`; criteria `{"Key";"a*"}` (cells) | `34.64823227814083` (`0x404152f9467969c9`) | Excel 16.0 build 20527, Windows, workbook compatibility 2, en-ZA; bitness and CPU not recorded |
| `DSTDEVP(A1:B3, "Value", D1:D2)` | database `{"Key","Value";"a0",0;"a1",-49}`; field `Value`; criteria `{"Key";"a*"}` (cells) | `24.5` (`0x4038800000000000`) | Excel 16.0 build 20527, Windows, workbook compatibility 2, en-ZA; bitness and CPU not recorded |
| `DVAR(A1:B3, "Value", D1:D2)` | database `{"Key","Value";"a0",0;"a1",-49}`; field `Value`; criteria `{"Key";"a*"}` (cells) | `1200.5` (`0x4092c20000000000`) | Excel 16.0 build 20527, Windows, workbook compatibility 2, en-ZA; bitness and CPU not recorded |
| `DVARP(A1:B3, "Value", D1:D2)` | database `{"Key","Value";"a0",0;"a1",-49}`; field `Value`; criteria `{"Key";"a*"}` (cells) | `600.25` (`0x4082c20000000000`) | Excel 16.0 build 20527, Windows, workbook compatibility 2, en-ZA; bitness and CPU not recorded |
| `DSTDEV(A1:B4, "Value", D1:D2)` | database `{"Key","Value";"a6",0.35294117647058826;"a0",-43;"a1",-42}`; field `Value`; criteria `{"Key";"a1"}` (cells) | `#DIV/0!` | Excel 16.0 build 20527, Windows, workbook compatibility 2, en-ZA; bitness and CPU not recorded |

Basis (measured): Measured in CalcFunc-Evidence
admit3-answers-{dstdev,dstdevp,dvar,dvarp}-range-mixed.json, DnaCalc prefix_metadata rows
272-276, and crates/calc-fml/tests/fixtures/entry/excel-entry.tsv; all 23,519 full-bank rows
across the four functions match.

Evidence:

- DnaCalc `crates/calc-func/src/catalogue/database.rs`, database at `2e9be2b5b7` — database variance dispatch
- DnaCalc `crates/calc-func/src/ported/variance_common.rs`, variance_from_values; stdev_from_values at `2e9be2b5b7` — x87 PC64 raw-moment and centered variance rules
- DnaCalc `crates/calc-func/witness-status.tsv`, DSTDEV; DSTDEVP; DVAR; DVARP at `2e9be2b5b7` — 6,507 + 5,673 + 5,669 + 5,670 full-bank matches
- CalcFunc-Evidence `readings/excel-20527/admit3-answers-dstdev-range-mixed.json`, admit3-range-mixed-DSTDEV-0000 at `4ef2c244d7` (readings)
- CalcFunc-Evidence `readings/excel-20527/admit3-answers-dstdevp-range-mixed.json`, admit3-range-mixed-DSTDEVP-0000 at `4ef2c244d7` (readings)
- CalcFunc-Evidence `readings/excel-20527/admit3-answers-dvar-range-mixed.json`, admit3-range-mixed-DVAR-0000 at `4ef2c244d7` (readings)
- CalcFunc-Evidence `readings/excel-20527/admit3-answers-dvarp-range-mixed.json`, admit3-range-mixed-DVARP-0000 at `4ef2c244d7` (readings)
- CalcFunc-Evidence `readings/excel-20527/admit3-answers-dstdev-range-mixed.json`, admit3-range-mixed-DSTDEV-0006 at `4ef2c244d7` (readings)

<!-- efh:notes end -->

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

- Microsoft, "DSTDEVP function" —
  <https://support.microsoft.com/en-us/office/dstdevp-function-04b78995-da03-4813-bbd9-d74fd0f5d94b>
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md`.
- Handbook call-model chapters 02 and 03 (range-scan coercion, error folding, precision
  publication).
- [DAVERAGE](FUNC.DAVERAGE.md), [DVAR](FUNC.DVAR.md) and [DSTDEV](FUNC.DSTDEV.md) — the shared
  criteria mechanism, the variance-formula account, and the composed-route account.
- OxFunc `docs/function-lane/FUNCTION_SLICE_DATABASE_FAMILY_CONTRACT_PRELIM.md` rule 9, and
  `crates/oxfunc_core/src/functions/variance_common.rs` at commit `473efa3`.
- `data/functions/FUNC.DSTDEVP.json`, `data/presence/FUNC.DSTDEVP.json`.
