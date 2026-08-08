---
schema: efh.function-page/v1
function_id: FUNC.AGGREGATE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Aggregate method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.aggregate"
    role: "the documented function_num table (1-19), the option table (0-7), the per-function_num reference-type constraints, and two documented #VALUE! conditions"
  - work: "Microsoft Support — AGGREGATE function"
    locator: "https://support.microsoft.com/en-us/office/aggregate-function-43b9278e-6aa7-4f17-92b6-e19993fa26df"
    role: "the canonical worksheet article; not retrieved for this pass (fetch returned HTTP 403)"
  - work: "Higham, Accuracy and Stability of Numerical Algorithms"
    locator: "chapter 4, summation"
    role: "why the order of a sequential summation is part of its specification, and what compensated summation would change"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: subtotal_aggregate_family
role_in_family: >-
  The nineteen-way dispatcher with an explicit ignore policy: SUBTOTAL's successor, sharing its
  module, and the only member whose second argument turns error-skipping and hidden-row-skipping
  into an argument rather than a function-number convention.
---

## What it computes

`AGGREGATE(function_num, options, ref1, [ref2], ...)` applies one of nineteen aggregate functions
to its reference arguments, under an explicit policy about what to skip.

It is not one function but a dispatcher over a table. Microsoft's Learn reference documents the
selector as a number from 1 to 19:

| `function_num` | Function | | `function_num` | Function |
|---|---|---|---|---|
| 1 | `AVERAGE` | | 11 | `VAR.P` |
| 2 | `COUNT` | | 12 | `MEDIAN` |
| 3 | `COUNTA` | | 13 | `MODE.SNGL` |
| 4 | `MAX` | | 14 | `LARGE` |
| 5 | `MIN` | | 15 | `SMALL` |
| 6 | `PRODUCT` | | 16 | `PERCENTILE.INC` |
| 7 | `STDEV.S` | | 17 | `QUARTILE.INC` |
| 8 | `STDEV.P` | | 18 | `PERCENTILE.EXC` |
| 9 | `SUM` | | 19 | `QUARTILE.EXC` |
| 10 | `VAR.S` | | | |

and the second argument as an ignore policy, also documented on that page:

| `options` | Behaviour |
|---|---|
| 0 or omitted | Ignore nested `SUBTOTAL` and `AGGREGATE` functions |
| 1 | Ignore hidden rows, nested `SUBTOTAL` and `AGGREGATE` functions |
| 2 | Ignore error values, nested `SUBTOTAL` and `AGGREGATE` functions |
| 3 | Ignore hidden rows, error values, nested `SUBTOTAL` and `AGGREGATE` functions |
| 4 | Ignore nothing |
| 5 | Ignore hidden rows |
| 6 | Ignore error values |
| 7 | Ignore hidden rows and error values |

The two-bit structure is worth naming, because the table above is really a product: bit 0 is
"ignore hidden rows", bit 1 is "ignore error values", and everything below 4 additionally ignores
nested `SUBTOTAL` and `AGGREGATE` calls. Options `0`-`3` are the nesting-aware quadrant; `4`-`7`
are the same four policies without the nesting rule.

**What `AGGREGATE` is for** is the error-skipping column. Every other route to "average this
column but ignore the `#N/A`s" is an array formula or an `IFERROR` wrapper over each cell;
`AGGREGATE` makes it argument 2. That is the reason the function exists, and it is why the
coercion chapter names `AGGREGATE` as the declared exception to the rule that coercion never
silently discards a worksheet error — see
[Coercion and lifting](../model/02-coercion-and-lifting.md). The skipping is a per-family
declaration, visible here, not an implicit engine behaviour.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `function_num` | Which of the nineteen aggregates to apply | yes |
| `options` | Which values to skip | yes at the worksheet call site; Microsoft documents `0` as the behaviour when it is omitted |
| `ref1` | The first reference (or, for the order-statistic selectors, the data) | yes |
| `ref2`, ... | Further references, or the `k` argument for the order-statistic selectors | no |

The reference engine records an arity of 3 to 255, with `ref2` marked as a repeating optional
parameter. Microsoft's Learn method signature exposes 30 argument slots (`Arg1`-`Arg30`); the
worksheet surface accepts more. **The Handbook records that as a documentation-versus-surface
mismatch rather than reconciling it**: the VBA method and the worksheet function are different
front doors to the same feature and their documented capacities differ.

### The argument position that catches everyone

For `function_num` 14 through 19 — `LARGE`, `SMALL`, `PERCENTILE.INC`, `QUARTILE.INC`,
`PERCENTILE.EXC`, `QUARTILE.EXC` — the *second* reference argument is not more data. It is the
`k` argument of the selected function. So `AGGREGATE(14, 6, A1:A100, 3)` is "the third largest,
ignoring errors", and the trailing `3` is a rank, not a range. For `function_num` 1 through 13 the
same slot *is* more data. One signature, two meanings, keyed on the first argument's value.

Microsoft's Learn reference documents the consequences of getting this wrong, and they are worth
quoting in effect: if a second `ref` argument is required but not provided, `AGGREGATE` returns a
`#VALUE!` error.

### Reference types

The Learn page's constraint table is unusually specific, and it is where `AGGREGATE` departs
sharply from ordinary aggregates:

- For `function_num` 1-13, the reference arguments accept cell references, unions, intersections,
  defined names and structured references, and **explicitly do not accept actual data or arrays**.
- For `function_num` 14-17, arrays and literal data *are* accepted in the first two slots, and no
  references are allowed from the third onward.

So `AGGREGATE(9, 6, {1,2,3})` is documented as invalid while `AGGREGATE(14, 6, {1,2,3}, 1)` is
documented as valid. That is a real and unusual asymmetry, and it follows from the first group
being reference-consuming aggregations while the second group needs a materialised list to sort.

**A gap in that table worth recording**: it covers `function_num` 1-13 and 14-17 and **stops**.
`function_num` 18 and 19 — `PERCENTILE.EXC` and `QUARTILE.EXC` — are in the selector table and
absent from the constraint table. Their reference-type rules are documented nowhere on the page.

## Result and edge cases

Returns whatever kind the selected aggregate returns — in practice `Number`, or an `Error`.

The reference engine's classification of this surface is the most interesting thing the projection
records about it:

| Axis | Value | What it means here |
|---|---|---|
| `arg_preparation_profile` | `RefsVisibleInAdapter` | The function sees references, not resolved values — it must, to know which rows are hidden |
| `host_interaction_class` | `WorkbookState` | It reads state outside its arguments: row visibility |
| `thread_safety_class` | `HostSerialized` | It cannot run on an arbitrary calculation thread |
| `error_collapse_profile` | `ReductionFold` | Errors are folded through a reduction, not passed through |
| `numerical_reduction_policy` | `SequentialLeftFold` | The reduction order is specified, left to right |
| `error_algebra` | `CanonicalExcelLegacy` | Which error wins when several are present follows the legacy precedence |
| `volatility_class` | `NonVolatile` | Despite reading workbook state, it is not declared volatile |

The last two rows deserve emphasis. `AGGREGATE` reads *hidden-row state*, which is presentation
state rather than cell content, and it is nevertheless classified non-volatile. That combination —
depends on workbook state, does not recalculate on every change — is the source of the practical
complaint that `AGGREGATE` results can go stale after rows are hidden or shown, and it is a
behaviour worth probing rather than assuming.

**"Hidden" means hidden rows.** The Learn documentation says rows throughout. Hidden *columns* are
not mentioned, and filtered-out rows versus manually hidden rows is a distinction the
documentation does not draw here at all.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | A second `ref` argument is required (`function_num` 14-19) but not provided | Microsoft Learn `WorksheetFunction.Aggregate` |
| `#VALUE!` | One or more references is a 3-D reference | Microsoft Learn `WorksheetFunction.Aggregate` |
| propagated / folded | Error values in the data, when `options` does not include error-skipping, reach the result under the `ReductionFold` collapse profile and the legacy error algebra | Reference engine classification |

Everything else — an out-of-range `function_num`, an out-of-range `options`, a `ref1` of the wrong
kind for the selected function — is undocumented on the page that was retrieved. Those are on the
probe list below.

## Relationships

- **`SUBTOTAL`** — the predecessor, and the sibling that shares this surface's implementing module
  in the reference engine. `SUBTOTAL` encodes its ignore policy in the *function number* (adding
  100 means "ignore hidden rows"), where `AGGREGATE` gives it a separate argument and adds error
  skipping, which `SUBTOTAL` has no way to express. `AGGREGATE` also reaches eleven aggregates
  `SUBTOTAL` does not, including the order statistics.
- **The nineteen selected functions** — each has its own Handbook page, and each is the authority
  on what its own aggregation means. `AGGREGATE` changes *which values reach* those functions, not
  what they compute.
- **`IFERROR`, `ISERROR`, `IFNA`** — the pre-`AGGREGATE` idiom for error skipping, one cell at a
  time.
- **`FILTER`** and the dynamic-array functions — the modern alternative, which does the selecting
  explicitly and then aggregates normally. Where `AGGREGATE`'s policy is a magic number, `FILTER`'s
  is a visible predicate; that is usually the better formula, and it does not know about hidden
  rows.
- **`COUNTIFS` / `SUMIFS` / `AVERAGEIFS`** — criteria-based selection rather than
  error-and-visibility-based selection. Different axis, frequently confused.
- **Confused with**: `SUBTOTAL`, constantly, and by readers who assume `AGGREGATE`'s option 0 and
  `SUBTOTAL`'s default behave alike.

## Numerical notes

`AGGREGATE` has no numerics of its own — it computes nothing — but it fixes two things that are
usually left implicit, and both belong on this page.

**The reduction order is specified.** The projection records
`numerical_reduction_policy=SequentialLeftFold`. That means the aggregation is a left fold over the
values in order, not a tree reduction, not a vectorised partial-sums arrangement, and not a
compensated sum. For `SUM`, `AVERAGE`, `VAR.*` and `STDEV.*` over data with mixed magnitudes, the
order of summation changes the result in the last bits, and a specified order is what makes the
result reproducible. It is also what makes it *worse* than it could be: Kahan or Neumaier
compensated summation would give a far smaller error bound at a small constant cost. Higham's
chapter on summation is the standard statement of the trade-off. A specified sequential fold is a
compatibility decision, not an accuracy one.

**The skip changes the reduction, not the arithmetic.** Skipping error values removes terms from
the fold. Skipping hidden rows removes different terms. Neither reweights anything, so
`AGGREGATE(1, 6, ...)` — average, ignoring errors — is the mean of the *surviving* values, with a
denominator that depends on how many errors there were. That is the intended reading, and it is
worth stating because the alternative reading (treat errors as zero) would give a different and
also plausible answer.

**Variance and standard deviation inherit their own hazard.** `function_num` 7, 8, 10 and 11 select
`STDEV.S`, `STDEV.P`, `VAR.S`, `VAR.P`. Whether those are computed by the textbook
sum-of-squares-minus-square-of-sum formula or by a numerically stable two-pass or Welford recurrence
is a property of *those* functions, not of `AGGREGATE`, and it is the dominant accuracy question
when the data has a large mean and a small spread. See those pages.

## What has not been checked

**No evidence record in the Handbook names `FUNC.AGGREGATE`**, and no Handbook vector suite exists
for it. Nobody has checked this function against Excel within the Handbook's record.

More than that: **the reference engine's battery could not call this function at all.** Every row
of the battery rendered beside this page reports the surface as not dispatchable, because it
requires a host facility — the workbook state that tells it which rows are hidden. So the usual
minimum, "here is what the reference engine returns", does not exist for `AGGREGATE` either. This
is the least-evidenced surface in this batch by a wide margin, and the reason is structural rather
than accidental: a function that reads presentation state cannot be exercised without a host.

The presence projection records that the implementing module is shared with `SUBTOTAL` and is named
in one defect stream (`BUG-FUNC-011`, a range-only parity gap filed against `COUNTBLANK`), and that
a family contract document exists for the pair upstream.

The documented statements above — the two tables, the reference-type constraints and the two
`#VALUE!` conditions — come from Microsoft's Learn `WorksheetFunction.Aggregate` reference, which
was retrieved. Microsoft's worksheet article was not retrieved for this pass (HTTP 403).

Probes worth running first, and they need a live workbook rather than a kernel:

1. **The hidden-row axis, directly.** Build a column, aggregate it with `options` 5, hide a row,
   and observe whether the result updates without a forced recalculation. This is the single probe
   that tests the non-volatile-yet-workbook-state-dependent classification, and it is the behaviour
   users actually get burned by.
2. **Manually hidden rows versus filtered-out rows**, which the documentation does not distinguish.
   Two probes, and they may well differ.
3. **Hidden columns**, which the documentation does not mention at all.
4. **`function_num` 18 and 19 with array and literal arguments** — the rows missing from the
   documented constraint table. Whether they follow the 14-17 rule or the 1-13 rule is unstated.
5. **`AGGREGATE(9, 6, {1,2,3})`** — a literal array to a group-1-13 selector, which the
   documentation says is invalid. Confirming the error code turns a documented prohibition into an
   observed one.
6. **Out-of-range selectors**: `function_num` 0, 20 and a non-integer; `options` 8 and a
   non-integer. The truncation-or-reject question is undocumented in both slots.
7. **Nesting**, which options 0-3 exist to control: an `AGGREGATE` over a range containing another
   `AGGREGATE`, at each option value.
8. **Summation order**, by aggregating a sequence constructed so that left-to-right and
   right-to-left folds differ in the last bit. This distinguishes the declared `SequentialLeftFold`
   from a compensated or reordered sum without needing to see any source.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| ignore policy | The `options` argument: which values the aggregation skips |
| order-statistic selector | `function_num` 14-19, whose second reference slot is a rank or quantile rather than data |
| `ReductionFold` | The error-collapse profile: errors are folded through a reduction rather than passed through |
| `SequentialLeftFold` | The declared summation order: left to right, uncompensated |
| workbook state | Data outside the arguments — here, row visibility — that the function reads |
| not dispatchable | A battery outcome meaning the surface could not be called without a host facility |

## Sources

- Microsoft Learn, "WorksheetFunction.Aggregate method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.aggregate> (retrieved:
  the 1-19 `function_num` table, the 0-7 `options` table, the reference-type constraint table for
  `function_num` 1-13 and 14-17, and the two documented `#VALUE!` conditions).
- Microsoft, "AGGREGATE function" —
  <https://support.microsoft.com/en-us/office/aggregate-function-43b9278e-6aa7-4f17-92b6-e19993fa26df>
  — the canonical worksheet article. **Not retrieved for this pass** (HTTP 403).
- Higham, *Accuracy and Stability of Numerical Algorithms*, chapter 4 — summation error bounds and
  what compensated summation buys.
- Handbook projections `data/functions/FUNC.AGGREGATE.json` (arity 3-255,
  `RefsVisibleInAdapter`, `WorkbookState`, `HostSerialized`, `ReductionFold`,
  `numerical_reduction_policy=SequentialLeftFold`, `error_algebra=CanonicalExcelLegacy`) and
  `data/presence/FUNC.AGGREGATE.json` (module shared with `FUNC.SUBTOTAL`; the `BUG-FUNC-011`
  defect stream; the upstream family contract document).
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md) — `AGGREGATE` as the
  declared exception to the no-silent-error-discard rule — and
  [The value universe](../model/01-value-universe.md).
