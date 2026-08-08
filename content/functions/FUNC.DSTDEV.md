---
schema: efh.function-page/v1
function_id: FUNC.DSTDEV
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
  The sample standard deviation of the field column over matching records — DVAR followed by a
  square root — and one of the two members whose result carries an extra rounding step beyond
  the aggregate itself.
---

# DSTDEV

## What it computes

`DSTDEV(database, field, criteria)` selects the records of *database* that satisfy *criteria*,
reads the column named by *field*, and returns the **sample** standard deviation of the numeric
values found there.

With V = [ v₁ … vₙ ] the Numbers collected from the field column of the matching records, in
record order, and n = |V|:

    x̄ = (Σ vᵢ) / n
    s  = √( Σ (vᵢ − x̄)² / (n − 1) )

Operationally it is [DVAR](FUNC.DVAR.md) followed by one square root; the reference
implementation computes it exactly that way. Sample semantics mean the divisor is n − 1 and at
least two numeric values are required.

Non-numeric cells in the field column — text, logicals, blanks — are skipped, so n counts
numbers, not records.

The selection stage — header matching, AND across a criteria row, OR down the rows, blank
criteria cells, and criteria-expression parsing — is identical for all twelve database functions
and is explained once in
[the criteria-range mechanism](FUNC.DAVERAGE.md#the-criteria-range-mechanism).

The numerical account of which variance formula is used — the well-conditioned two-pass form
that OxFunc implements, versus the one-pass sum-of-squares form — is given in full on
[the DVAR page](FUNC.DVAR.md#why-the-formula-not-just-the-definition) and applies here
unchanged.

### The extra rounding step

`DSTDEV` is not a single rounded operation. The variance is computed and rounded to a double,
and *then* the square root of that rounded value is taken and rounded again. The result is
therefore not in general the correctly rounded standard deviation of the input data: two
roundings sit between the exact answer and the published one. IEEE-754 square root is itself
correctly rounded, so the discrepancy comes entirely from the intermediate — which means an
implementation that computed s directly, more accurately, would *differ* from the composed
route on some inputs. Matching a composed route is a compatibility target, not an accuracy
target, and the Handbook has not established which route Excel takes.

## Arguments

Three arguments, all required at the call site; the reference implementation declares an arity
of exactly 3.

- **`database`** — the header-plus-records range or array.
- **`field`** — the column selector, as header text or as a 1-based column index within the
  database range. Required.
- **`criteria`** — the header-plus-conditions range or array.

## Result and edge cases

Returns a Number, non-negative.

- **Fewer than two numeric values selected**: `#DIV/0!` — the error comes from the variance
  stage, before any square root is attempted.
- **All selected values equal**: `0`, exactly, since the variance is exactly zero and √0 is 0.
- **Text that looks numeric in the field column** is skipped, per the range-scan side of the
  asymmetry in
  [coercion and lifting](../model/02-coercion-and-lifting.md#direct-arguments-versus-range-scans).
- **Logicals in the field column** are skipped.
- **An error value in a scanned field cell** surfaces; the family declares
  `ErrorCollapseProfile::ReductionFold`.
- **Arrays**: inline array literals resolve to grids and are accepted; `DSTDEV` is not a lift
  kernel.
- **No negative-argument path.** Because the two-pass variance cannot be negative, the square
  root has no domain failure. An implementation using the sum-of-squares form could produce a
  negative variance and hence a `#NUM!` from the root — an error `DSTDEV` should never be able
  to return.

## Errors

| Error | Condition |
|---|---|
| `#DIV/0!` | Fewer than two numeric values were collected from the field column of the matching records |
| `#VALUE!` | *database* or *criteria* is not a grid or is empty; a database header cell is blank; a criteria header names no database field; *field* matches no header, or is a non-integer or out-of-range column index |
| propagated | An error value in a header cell, a criteria cell, or a scanned field cell surfaces as that error |

Microsoft's page documents the argument roles, the criteria conventions, and that the function
estimates the standard deviation from a sample; the error conditions are stated from OxFunc's
implementation and family contract and have not been compared against Excel by the Handbook.

## Relationships

- **[DSTDEVP](FUNC.DSTDEVP.md)** is the population form: divisor n, and one value is enough.
- **[DVAR](FUNC.DVAR.md)** is the square of this function, and the page that carries the shared
  numerical discussion.
- **Nearest modern equivalent**: `STDEV.S` over a filtered array. Excel's compatibility rename
  pass produced `STDEV` / `STDEV.S`; the database family was not renamed, so `DSTDEV` is not a
  deprecated name and has no `.S`-suffixed replacement.
- **Confused with**: `DSTDEVP` (the divisor and the minimum count) and `STDEVA`, which counts
  logicals and text.

## Notes for implementers

1. **Compose, do not reformulate.** Computing the variance and taking its square root is what
   the reference implementation does; a more accurate direct route is a different function in
   the last bits.
2. **Guard the count before the arithmetic.** `#DIV/0!` at n < 2 is a domain decision.
3. **Both accumulation passes are sequential left folds** in record order, per the family's
   declared reduction policy.
4. **Skip, do not coerce.** Admit only cells whose value kind is Number.
5. **Numeric criteria comparison is not exact double comparison** — see the note in
   [the shared mechanism](FUNC.DAVERAGE.md#numeric-comparison-is-not-exact-double-comparison).

## What has not been checked

No Handbook vector suite exists for `DSTDEV`, and no Excel-comparison evidence record is
recorded for it. Nobody has run a Handbook-owned comparison of this function against Excel.

Inputs I would probe first:

1. **The composition test.** For the same selection, compare `DSTDEV` against `SQRT(DVAR(...))`
   in Excel itself. If they agree on every probe, the composed route is confirmed and the
   remaining uncertainty collapses onto [DVAR](FUNC.DVAR.md). If they ever differ, `DSTDEV` has
   its own kernel and needs its own suite. This is the cheapest high-information probe on the
   page and it needs no external reference values.
2. **The count boundary**: selections of zero, one, and two numeric values.
3. **The conditioning probe** from [DVAR](FUNC.DVAR.md): `1e9 + 1`, `1e9 + 2`, `1e9 + 3`, whose
   true sample standard deviation is exactly `1`.
4. **All-equal values**, checking for exact `0` rather than a tiny positive residual — a
   sum-of-squares implementation can produce either that or `#NUM!` here.
5. **A matching set with blanks interleaved**, confirming n counts numbers rather than records.
6. **Values spanning many orders of magnitude**, where the intermediate variance may overflow or
   underflow before the square root pulls it back into range.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| database grid | The header-plus-records rectangle passed as *database* |
| criteria grid | The header-plus-conditions rectangle passed as *criteria* |
| field column | The database column selected by the *field* argument |
| value list | The Numbers collected from the field column of the matching records |
| n | The length of the value list — a count of numbers, not of records |
| composed route | Computing the variance, rounding it, then taking the square root |

## Sources

- Microsoft, "DSTDEV function" —
  <https://support.microsoft.com/en-us/office/dstdev-function-026b8c73-616d-4b5e-b072-241871c4ab96>
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md`.
- Handbook call-model chapters 02 and 03 (range-scan coercion, error folding, precision
  publication).
- [DAVERAGE](FUNC.DAVERAGE.md) — the family's shared criteria-range mechanism;
  [DVAR](FUNC.DVAR.md) — the shared numerical account of the variance formula.
- OxFunc `docs/function-lane/FUNCTION_SLICE_DATABASE_FAMILY_CONTRACT_PRELIM.md` rule 9, and
  `crates/oxfunc_core/src/functions/variance_common.rs` at commit `473efa3` (the standard
  deviation is the square root of the variance value, computed in that order).
- `data/functions/FUNC.DSTDEV.json`, `data/presence/FUNC.DSTDEV.json`.
