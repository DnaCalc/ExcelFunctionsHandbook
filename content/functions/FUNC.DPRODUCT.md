---
schema: efh.function-page/v1
function_id: FUNC.DPRODUCT
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
  The product of the field column over matching records; the family's multiplicative reducer,
  and the member whose empty-selection answer of zero contradicts the mathematical identity
  (an empty product is one), which makes it the sharpest test of the family's conventions.
---

# DPRODUCT

## What it computes

`DPRODUCT(database, field, criteria)` selects the records of *database* that satisfy *criteria*,
reads the column named by *field*, and returns the product of the numeric values found there.

With R the set of selected record rows, c the resolved field column, and

    V = [ v : v = value of cell (r, c) for r in R, in record order, where v is a Number ]

the result is Π V, multiplied as a left fold in record order — **except** when V is empty, where
OxFunc returns `0` rather than the empty product `1`. That is a deliberate compatibility
convention inherited from `PRODUCT`, not an oversight, and it is the most surprising single fact
on this page: `DPRODUCT` is not multiplicatively well-behaved at its own boundary.

Non-numeric cells in the field column contribute nothing: text (including text that reads as a
number), logicals, and blanks are skipped rather than coerced. Note the asymmetry with
[DSUM](FUNC.DSUM.md): skipping a cell is the same as adding zero for a sum, but it is *not* the
same as multiplying by one — it is what makes the product well-defined at all, since coercing a
blank to zero would annihilate every result containing one.

The selection stage — header matching, AND across a criteria row, OR down the rows, blank
criteria cells, and criteria-expression parsing — is identical for all twelve database functions
and is explained once in
[the criteria-range mechanism](FUNC.DAVERAGE.md#the-criteria-range-mechanism).

## Arguments

Three arguments, all required at the call site; the reference implementation declares an arity
of exactly 3.

- **`database`** — the header-plus-records range or array.
- **`field`** — the column selector, as header text or as a 1-based column index within the
  database range. Required.
- **`criteria`** — the header-plus-conditions range or array.

## Result and edge cases

Returns a Number.

- **No numeric value selected**: `0`. See above; this is the convention, not the empty product.
- **A single matching value**: that value, unchanged.
- **A zero anywhere in the value list**: `0`, for the ordinary reason.
- **Overflow.** Products grow fast. The family's declared real-result policy is
  `non_finite=allow` with no argument-domain guard, meaning the reference implementation does
  not itself convert an overflowing product to `#NUM!` — see
  [result publication](../model/03-call-pipeline.md#stage-4-result-publication) for what that
  axis means. What Excel publishes for an overflowing `DPRODUCT` is unchecked, and it is the
  most consequential open question on this page.
- **Text that looks numeric in the field column** is skipped, per the range-scan side of the
  asymmetry in
  [coercion and lifting](../model/02-coercion-and-lifting.md#direct-arguments-versus-range-scans).
- **Logicals in the field column** are skipped, so `TRUE` never contributes a factor of 1 and
  `FALSE` never annihilates the product.
- **An error value in a scanned field cell** surfaces; the family declares
  `ErrorCollapseProfile::ReductionFold`.
- **Arrays**: inline array literals resolve to grids and are accepted; `DPRODUCT` is not a lift
  kernel.

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | *database* or *criteria* is not a grid or is empty; a database header cell is blank; a criteria header names no database field; *field* matches no header, or is a non-integer or out-of-range column index |
| propagated | An error value in a header cell, a criteria cell, or a scanned field cell surfaces as that error |

No overflow error is declared by the reference implementation's axes. Microsoft's page documents
argument roles and criteria conventions; the table is stated from OxFunc's implementation and
family contract and has not been compared against Excel by the Handbook.

## Relationships

- **[DSUM](FUNC.DSUM.md)** is the additive counterpart, with the same skip policy and the same
  empty-selection zero — which is the correct additive identity there and the wrong
  multiplicative one here.
- **Nearest equivalent**: `PRODUCT` combined with `IF`, entered as an array formula, or
  `EXP(SUMIFS(LN(...)))` for strictly positive data. Excel has no `PRODUCTIFS`, which makes
  `DPRODUCT` the only criteria-driven product in the worksheet surface — a genuinely unique role
  in the function catalogue.
- **Confused with**: `PRODUCT` (no selection stage) and `SUMPRODUCT` (which sums products across
  columns rather than multiplying down one).

## Notes for implementers

1. **Reduction order is observable.** The family declares a sequential left fold. Floating-point
   multiplication is not associative, so a reassociated or pairwise product will differ in the
   last bits, and on values near the representable range the *order alone* can decide whether an
   intermediate overflows.
2. **The empty-selection zero must be special-cased.** A fold with initial value `1` gives `1`
   for an empty list, which is mathematically right and behaviourally wrong here. Branch on
   emptiness explicitly.
3. **Skip, do not coerce.** Admit only cells whose value kind is Number. Coercing blanks to zero
   would turn most real columns into `0`.
4. **Numeric criteria comparison is not exact double comparison** — see the note in
   [the shared mechanism](FUNC.DAVERAGE.md#numeric-comparison-is-not-exact-double-comparison).

## What has not been checked

No Handbook vector suite exists for `DPRODUCT`, and no Excel-comparison evidence record is
recorded for it. Nobody has run a Handbook-owned comparison of this function against Excel.

Inputs I would probe first:

1. **Empty selection.** Criteria matching nothing. The Handbook states `0`; the mathematical
   answer is `1`. Nothing else on this page is as cheap to test or as easy to get wrong.
2. **Overflow.** A matching column of, say, twenty cells each holding `1e30`. Candidate answers
   are `#NUM!`, positive infinity rendered somehow, or a spurious finite value. The declared axis
   says the implementation does not convert it; Excel's answer is unknown to the Handbook.
3. **Underflow to zero**, the mirror probe: twenty cells of `1e-30`.
4. **Order sensitivity**: the same multiset of factors in two record orders, chosen so that one
   order overflows an intermediate and the other does not — for example `1e300, 1e300, 1e-300`
   against `1e300, 1e-300, 1e300`.
5. **A field column of numeric-looking text**, separating "skipped" from "coerced".
6. **Logicals in the field column**, confirming `FALSE` does not zero the product.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `DPRODUCT` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-SUBTOTAL-001: SUBTOTAL, AGGREGATE, and the D-functions reject literal numbers or arrays in their reference slots at formula entry (0x800A03EC)

*Quirk; measured; recorded 2026-11-22.* Applies to AGGREGATE, DAVERAGE, DCOUNT, DMAX, DMIN, DPRODUCT, DSUM, SUBTOTAL.

In Excel 16.0.20527.20088, SUBTOTAL (ref1/ref2...), AGGREGATE (reference/ref1... in reference
form), and database functions (DSUM, DCOUNT, DAVERAGE, DMIN, DMAX, DPRODUCT, database argument)
require actual range references in those parameter positions: supplying a literal number or
array constant (such as =SUBTOTAL(9,1,2,3,4), =AGGREGATE(9,4,1,2,3,4), or
=DSUM({1,2;3,4},1,{1})) is rejected by Excel at formula entry (COM 0x800A03EC) in both legacy
and dynamic forms.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=SUBTOTAL(9.0,1.0,2.0,3.0,4.0)` | as typed | `entry rejected (0x800A03EC)` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=AGGREGATE(9.0,4.0,1.0,2.0,3.0,4.0)` | as typed | `entry rejected (0x800A03EC)` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=DSUM({1,2;3,4},1.0,{1})` | as typed | `entry rejected (0x800A03EC)` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Measured by live Excel 16.0.20527 formula entry across all 18 entry-rejected
rows of jnt_aggregate_arrays (rows 61-71 and 111-116) and
crates/calc-fml/tests/fixtures/entry/excel-entry.tsv.

Evidence:

- DnaCalc `crates/calc-func/tests/captures/jnt/sections/jnt_aggregate_arrays.capture.json`, rows 61 to 71 and 111 to 116 at `4fb9b728df` — Live Excel 16.0.20527 formula-entry rejections
- DnaCalc `docs/plan/corpus-failures-2026-11-11.md`, Section A at `4fb9b728df` — Analysis and live read-back of the 18 entry-rejected cells
- DnaCalc `crates/calc-fml/tests/fixtures/entry/excel-entry.tsv` at `4fb9b728df` — Measured Excel 16.0.20527 formula-entry acceptance table
- DnaCalc `crates/calc-func/tests/captures/jnt/sections/jnt_aggregate_arrays.capture.json`, sheet jnt_aggregate_arrays row 61 column 4 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/jnt/sections/jnt_aggregate_arrays.capture.json`, sheet jnt_aggregate_arrays row 67 column 4 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/jnt/sections/jnt_aggregate_arrays.capture.json`, sheet jnt_aggregate_arrays row 111 column 4 at `4fb9b728df` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| database grid | The header-plus-records rectangle passed as *database* |
| criteria grid | The header-plus-conditions rectangle passed as *criteria* |
| field column | The database column selected by the *field* argument |
| value list | The Numbers collected from the field column of the matching records |
| empty product | The mathematical value of a product over no factors, namely 1 |
| empty-selection zero | The `0` this function returns instead of the empty product |

## Sources

- Microsoft, "DPRODUCT function" —
  <https://support.microsoft.com/en-us/office/dproduct-function-4f96b13e-d49c-47a7-b769-22f6d017cb31>
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md`.
- Handbook call-model chapters 02 and 03 (range-scan coercion, error folding, result
  publication and the non-finite axis).
- [DAVERAGE](FUNC.DAVERAGE.md) — the family's shared criteria-range mechanism.
- OxFunc `docs/function-lane/FUNCTION_SLICE_DATABASE_FAMILY_CONTRACT_PRELIM.md` and
  `crates/oxfunc_core/src/functions/database_family.rs` at commit `473efa3`.
- `data/functions/FUNC.DPRODUCT.json` (real-result policy: no argument-domain guard,
  non-finite allowed), `data/presence/FUNC.DPRODUCT.json`.
