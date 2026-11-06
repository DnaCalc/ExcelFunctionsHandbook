---
schema: efh.function-page/v1
function_id: FUNC.COUNTIFS
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it does
  - Try it
  - Quirks
  - How to implement
  - Version differences
  - Alternatives
  - Sources
family: null
role_in_family: null
---

# COUNTIFS

## What it does

`COUNTIFS(criteria_range1,criteria1,[criteria_range2,criteria2],...)` counts positions where **all** paired conditions hold. It combines conditions with AND. Up to 127 range/criterion pairs are documented. The ranges align by their row and column offsets and need not be adjacent on the sheet.

## Try it

Use these four rows:

| Row | B: amount | C: region |
|---:|---:|---|
| 2 | 4 | east |
| 3 | 11 | east |
| 4 | 7 | west |
| 5 | 13 | east |

`=COUNTIFS(B2:B5,">=10",C2:C5,"east")` returns `2`, for rows three and five. Repeating a range creates an interval: `=COUNTIFS(B2:B5,">=7",B2:B5,"<=11")` returns `2`.

Adding separate counts implements an OR only when overlap is handled. Here `COUNTIF(B2:B5,">=10")+COUNTIF(C2:C5,"east")` is five, because rows three and five are counted twice. Subtract their intersection, the first COUNTIFS result, to get three distinct rows meeting either condition.

## Quirks

Every criteria range must have the same row and column dimensions as the first. Equal total cell counts in different shapes are not enough. The current implementation returns `#VALUE!` for a shape mismatch before reading any criterion.

Conditions on the same range still use AND. For example, `">=7"` and `"<=11"` define an inclusive interval; the pair `">=10"` and `"<5"` cannot match one numeric value. For OR, count a union without double-counting its overlap.

A criterion referencing an empty cell is treated as zero. This does not make every blank tested cell equivalent to zero under every predicate. Text criteria retain case-insensitive matching and `?`, `*`, `~` wildcard syntax; preserve leading and trailing spaces when their presence is meaningful.

## How to implement

1. Validate the argument pairs and resolve all range references. Check row and column dimensions before reading criteria or range values, preserving the current error order.
2. Read each criterion once and compile the typed [COUNTIF predicate](FUNC.COUNTIF.md#how-to-implement): operator, operand kind and any wildcard tokens.
3. Align ranges by offset inside their own rectangles. At a position, combine every predicate’s verdict with AND. A known false verdict rules that position out; do not add independent per-range counts.
4. Increment once for each position satisfying all conditions. Return one numeric count after visiting the aligned positions.

In the main example, B2/C2 fails the amount condition, B3/C3 passes both, B4/C4 fails, and B5/C5 passes both, giving two. For the interval example, seven and eleven pass both tests.

The current sparse implementation keeps verdicts for visited cells and counts matching blank positions in bulk, so whole-column references need not allocate a Boolean entry for every worksheet cell. It preserves unresolved verdicts and refuses only when one can affect the count. Copying its supported predicates is an implementation route; the explicitly unmodeled text, array and collation cases still need evidence before coverage can be extended.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[COUNTIF](FUNC.COUNTIF.md) handles one condition; [SUMIFS](FUNC.SUMIFS.md) sums matching amounts; [FILTER](FUNC.FILTER.md) exposes the matching records.

## Sources

- [Microsoft: COUNTIFS function](https://support.microsoft.com/en-us/excel/functions/countifs-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc criteria grammar, range validation and count planning](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/conditional.rs); implementation rules and its explicit unsupported cases.
