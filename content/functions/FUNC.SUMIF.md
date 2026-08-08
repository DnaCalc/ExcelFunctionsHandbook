---
schema: efh.function-page/v1
function_id: FUNC.SUMIF
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-STRUCT-0001
open_problems: []
references:
  - work: "Microsoft Support — SUMIF function"
    locator: "https://support.microsoft.com/en-us/office/sumif-function-169b8c99-c05c-4483-a712-1697a653039b"
    role: "documented signature, argument meanings, the wildcard set, the long-string limitation, and the sum_range sizing rule"
  - work: "Higham, Accuracy and Stability of Numerical Algorithms"
    locator: "chapter on summation"
    role: "error bounds for the accumulation order of a filtered sum"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - The sum_range sizing rule
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: criteria_family
role_in_family: >-
  The original single-criterion summing member: one predicate over one range, with an optional
  displaced range supplying the values actually added.
---

## What it computes

`SUMIF(range, criteria, [sum_range])` adds the values of `sum_range` at the positions where
`range` satisfies `criteria`. Written as a filtered reduction:

    M   = { i : P(range_i) }            the matched index set
    SUMIF = SUM over i in M of s_i      where s is sum_range, or range itself if omitted

Two things are worth separating in that definition, because the function's whole difficulty
lives in the gap between them:

1. **The predicate `P`** is applied to `range`. It is not an arbitrary expression; it is a
   *criteria value* in the family's small criteria language — a number, a text string, a
   comparison written as text (`">5"`), or a wildcard pattern.
2. **The values summed** come from `sum_range`, a *different* range, addressed positionally
   rather than by intersection. That indirection is the source of the sizing rule below.

Microsoft describes the arguments as the range "you want evaluated by criteria", the criteria
"in the form of a number, expression, a cell reference, text, or a function that defines which
cells will be added", and `sum_range` as "the actual cells to add, if you want to add cells
other than those specified in the range argument".

Mathematically this is nothing more than a masked sum, and the mask is where all the semantics
are. The criteria mechanism is shared across the whole criteria family — `SUMIFS`, `COUNTIF`,
`COUNTIFS`, `AVERAGEIF`, `AVERAGEIFS`, `MAXIFS`, `MINIFS` — which the reference engine
implements in one module.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `range` | The cells evaluated by `criteria`. Required. | — |
| `criteria` | The condition selecting which cells are added. Required. | — |
| `sum_range` | The cells actually added. Optional. | `range` |

The reference engine records an arity of two to three, matching the documented optional third
argument.

**The criteria language, as documented.** Criteria may be a number, an expression, a cell
reference, text, or a function. Text criteria and any criteria containing logical or
mathematical symbols must be enclosed in double quotation marks — so `">5"` is a string that the
function parses as a comparison, not a comparison the formula evaluates. Wildcards are
available: `?` matches a single character, `*` matches any sequence, and `~` escapes a literal
`?` or `*`.

**A documented limitation worth quoting for its unusual candour.** Microsoft states: "The SUMIF
function returns incorrect results when you use it to match strings longer than 255 characters."
Not an error value — *incorrect results*. That is a documented silent-wrong-answer condition,
and it is rare enough in Microsoft's function reference to be worth flagging: a formula matching
long strings is documented to produce a number that is simply not the right number, with no
diagnostic anywhere. Any workbook matching on long free-text keys is in that territory.

## The sum_range sizing rule

This is the position most readers get wrong, and the documentation is explicit about it:
"Sum_range should be the same size and shape as range. If it isn't, performance may suffer, and
the formula will sum a range of cells that starts with the first cell in sum_range but has the
same dimensions as range."

In other words: **`sum_range` is treated as an anchor, not as an extent.** Only its top-left
cell is honoured; the shape is taken from `range`. `SUMIF(A1:A10, ">5", B1)` therefore sums
`B1:B10`, and `SUMIF(A1:A10, ">5", B1:B3)` also sums `B1:B10` — silently reading seven cells the
author never named.

The consequences are worth stating plainly, because none of them produce an error:

- A `sum_range` that is *shorter* than `range` reads past its own end into whatever lies below.
- A `sum_range` whose orientation differs from `range` — a row where `range` is a column — is
  re-shaped to `range`'s dimensions from its anchor, so the cells summed may have no relation to
  the ones written in the formula.
- Inserting or deleting rows inside `range` changes which cells `sum_range` covers, without
  touching the `sum_range` reference at all.

The defensive practice is simply to write both ranges with identical shapes and to keep them
adjacent, so a structural edit moves both together.

## Result and edge cases

Returns `Number`.

- **No matches**: zero. `SUMIF` does not report an empty match set as an error, which means an
  entirely mistyped criterion is indistinguishable from a genuine zero total.
- **Text and logicals in `sum_range`**: not addressed on the `SUMIF` page. The aggregate family's
  range-scan policy is the governing rule; see
  [Coercion and lifting](../model/02-coercion-and-lifting.md#direct-arguments-versus-range-scans).
  The Handbook has not checked which scan policy this surface uses.
- **Error values** in either range: not addressed by the documentation. The reference engine
  declares `error_collapse_profile: ReductionFold` with `error_algebra=CanonicalExcelLegacy` for
  this module — a fold, not a mask.
- **Numeric criteria matching is not exact IEEE equality.** This is the substantive finding on
  this surface and it is discussed under *Numerical notes*.

The reference engine records `arg_preparation_profile: RefsVisibleInAdapter` (the function sees
live references, which is what makes the anchor-and-reshape behaviour expressible),
`coercion_lift_profile: Custom`, `numerical_reduction_policy=SequentialLeftFold`, and a projected
`real_result_policy` of `arg_domain_guard=none` with `non_finite=allow`.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | An argument does not resolve to a range or a usable criteria value | Shared call model; not enumerated on the `SUMIF` page |
| propagated | An error value among the scanned data surfaces as that error | Shared coercion rules and the module's `ReductionFold` |

Microsoft's page documents no error conditions for `SUMIF`. It documents a *wrong-answer*
condition instead (the long-string limitation above), which is a different and arguably worse
thing.

## Relationships

- **`SUMIFS`** — the multi-criterion successor, and note the **argument order is different**:
  `SUMIFS(sum_range, criteria_range1, criteria1, ...)` puts the summed range first, where
  `SUMIF` puts it last. Converting one to the other by adding a criterion pair does not work;
  the arguments have to be rearranged. `SUMIFS` also requires its ranges to match in shape rather
  than reshaping from an anchor, which makes it the safer function even for a single criterion.
- **`COUNTIF` / `COUNTIFS`** — the same predicate machinery with a counting reduction. `COUNTIF`
  is a co-subject of the evidence record attached to this page.
- **`AVERAGEIF` / `AVERAGEIFS`, `MAXIFS`, `MINIFS`** — the rest of the module.
- **`SUMPRODUCT`** — the general alternative, whose conditions are arbitrary array expressions
  rather than criteria strings. It is more general and less readable; `SUMIF` is the right tool
  when the condition fits the criteria language.
- **`FILTER` + `SUM`** — the modern dynamic-array route, which makes the matched set visible as
  a value rather than hiding it inside the aggregation.
- **`DSUM`** — the database-family equivalent, with criteria expressed as a range rather than as
  a string.
- **Confused with**: `SUMIFS` (argument order), and with `SUMIF` over a `sum_range` of the wrong
  shape (the sizing rule above).

## Numerical notes

`SUMIF` has two numerical characters, and they are unrelated to each other.

**1. The comparison is tolerant, not exact.** The evidence record attached to this page,
`EV-STRUCT-0001`, is a live-verification record of a two-arm experiment in which a set of
comparison surfaces — the six comparison operators, `SWITCH`, `COUNTIF` and `SUMIF` — were routed
through a shared numeric-comparison helper that **truncates each operand to a fixed number of
significant decimal digits, toward zero, and then compares the truncated values exactly**. The
record names the helper, the digit count, and the exhaustive caller set, and it records that the
reference engine's criteria-family module is one of those callers. It also identifies a
deliberate *control* arm — `MATCH`, `XMATCH` and `DELTA` — which was left on raw IEEE equality,
and the record's own reader warning is that those three must not be read as tolerant.

What that means for a reader of `SUMIF`: a criterion of `0.3` can match a stored value of
`0.1+0.2`, even though those are different doubles. Whether you regard that as a bug or as the
only humane behaviour for a spreadsheet, it is a real semantic property and it changes which
cells enter the sum. It also means the matched set is not determined by the bits alone, so two
implementations that agree on IEEE arithmetic can still disagree on `SUMIF`.

**The record's counts are not this surface's counts.** `EV-STRUCT-0001` carries an explicit
reader warning that its total is over hand-picked lanes spanning both arms, not a pass rate for
any surface, and it records an unresolved discrepancy in the upstream lane enumeration. `SUMIF`
appears there with the subject role `body-only`. **This page states none of those figures; the
evidence layer renders them beside it with their warnings attached.**

The record makes one further point that is easy to misread in the other direction: because its
axis is structural admission, an absence of a *numeric comparison* record for this surface means
an absence of a bit-level comparison, not an absence of numeric evidence — the record is
entirely about comparing numbers.

**2. The summation is an ordinary left fold.** Once the mask is decided, `SUMIF` is a plain sum
over the matched positions, and the reference engine declares a sequential left fold. Higham's
summation analysis applies unchanged: worst-case error growing linearly in the number of matched
terms, with pairwise or compensated summation available as strictly better alternatives that the
worksheet cannot request. Because the matched set is typically a scattered subset of a range, the
accumulation order is the range order, not the sorted-magnitude order — so the swallowing
failure mode (small terms lost against a large accumulator) is entirely dependent on how the data
happens to be laid out on the sheet. Re-sorting rows can change the last bits of a `SUMIF` even
when it changes nothing about the data.

## What has not been checked

The evidence attached to this page is `EV-STRUCT-0001`, which lists `FUNC.SUMIF` in its subjects
with the role `body-only`. It is a live-verification record about a numeric-comparison helper,
and it carries an explicit warning against reading its total as any surface's pass rate.

No Handbook vector suite exists for `SUMIF`. Beyond that record, the Handbook has not observed
`SUMIF` in Excel, and nothing on this page is a statement that any implementation agrees with
Excel.

Probes worth running first:

1. **The tolerant-comparison boundary.** `SUMIF({0.3, 0.1+0.2}, 0.3)` and neighbouring
   constructions, including a pair engineered to straddle the truncation boundary the record
   names. This decides, for `SUMIF` specifically rather than for the helper, where the tolerance
   actually sits — and the record's own open question about lane decomposition makes it the most
   valuable probe on the page.
2. **The `sum_range` sizing rule.** `SUMIF(A1:A10, ">5", B1)` and `SUMIF(A1:A10, ">5", B1:B3)`
   with distinguishable values below `B3`, confirming that cells outside the written reference
   are read. Also a row `sum_range` against a column `range`.
3. **The long-string limitation.** A criterion and matching data just under and just over the
   documented 255-character threshold, to characterize the documented incorrect-result condition
   — which is documented as existing but not as to *what* it returns.
4. **Wildcards**: `?`, `*`, `~*`, and a criterion that is only `*`; plus a criterion drawn from a
   cell whose text happens to contain `*`, which is the case that silently changes a literal
   match into a pattern match.
5. **Error values** in `range` and in `sum_range` separately — fold or mask.
6. **Text, logicals and empty cells** in `sum_range`, to pin the scan policy for this surface
   rather than inheriting it from `SUM`.
7. **Order dependence**: the same data with rows reversed, which detects the left fold.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| criteria language | The small predicate language of numbers, comparison strings and wildcards |
| anchor-and-reshape | The documented rule that `sum_range` supplies only a top-left cell; the shape comes from `range` |
| tolerant comparison | Numeric matching that truncates operands before comparing, rather than comparing bits |
| control arm | The surfaces deliberately left on exact IEEE equality in the record attached here |
| left fold | Adding matched terms in sheet order into one accumulator |

## Sources

- Microsoft, "SUMIF function" —
  <https://support.microsoft.com/en-us/office/sumif-function-169b8c99-c05c-4483-a712-1697a653039b>
  (signature; the argument descriptions; "The SUMIF function returns incorrect results when you
  use it to match strings longer than 255 characters"; the wildcard set; the quotation-mark rule
  for text and symbolic criteria; and the `sum_range` sizing rule quoted above).
- Handbook evidence record `EV-STRUCT-0001` (subjects include `FUNC.SUMIF`, role `body-only`) —
  the tolerant-versus-exact comparison family split, with its own reader warning and open
  question.
- Higham, *Accuracy and Stability of Numerical Algorithms* — summation error bounds and the
  order dependence of a fold.
- Handbook projections `data/functions/FUNC.SUMIF.json` (arity 2-3,
  `error_collapse_profile: ReductionFold`, `numerical_reduction_policy=SequentialLeftFold`,
  `arg_preparation_profile: RefsVisibleInAdapter`) and `data/presence/FUNC.SUMIF.json` (module
  shared with `AVERAGEIF`, `AVERAGEIFS`, `COUNTIF`, `COUNTIFS`, `MAXIFS`, `MINIFS`, `SUMIFS`).
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md).
