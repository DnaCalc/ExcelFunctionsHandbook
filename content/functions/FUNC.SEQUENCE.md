---
schema: efh.function-page/v1
function_id: FUNC.SEQUENCE
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

# SEQUENCE

## What it does

`SEQUENCE(rows, [columns], [start], [step])` creates a rectangular array of regularly spaced numbers. Optional arguments default to one. Microsoft also permits omitting `rows` when at least one other argument is supplied; the missing row count then defaults to one.

## Try it

`=SEQUENCE(2,3,10,2)` spills these two rows:

| Column 1 | Column 2 | Column 3 |
|---:|---:|---:|
| 10 | 12 | 14 |
| 16 | 18 | 20 |

The sequence continues across the first row before moving to the second. `=SEQUENCE(,3,5,2)` produces one row, `5,7,9`. A negative step descends, while a zero step repeats the start value.

## Quirks

The output needs enough free worksheet cells to spill. The function’s array result and the workbook’s placement decision are separate: a correct two-by-three array can still encounter a spill obstruction. Microsoft documents limited support for dynamic arrays linked across workbooks; closing the source workbook can give `#REF!` when the link refreshes.

The current numeric implementation accepts positive integer dimensions. A zero dimension gives `#CALC!`; a negative or fractional dimension gives `#VALUE!`. Omitted or blank arguments take their defaults. These error choices come from the implementation, since the opened Microsoft function page does not enumerate those dimension errors.

## How to implement

1. Prepare all four arguments, preserving formed worksheet errors. Apply the defaults before dimension checks; omission and numeric zero are different cases.
2. Require finite, positive integer row and column counts. Check their product for overflow and against the host’s output limit before allocating.
3. For zero-based output index `i`, compute `start+i*step` with a multiply followed by an add. Store it at row `i div columns`, column `i mod columns`.
4. Return an array with the requested shape. The workbook host checks its spill destination.

This is the current code’s direct-index calculation. Do not silently replace it with repeated `previous+step`: accumulated rounding can change the sequence. For example, the current route for `SEQUENCE(1,3,10000000000000000,1)` gives `10000000000000000`, `10000000000000000`, `10000000000000002`; repeated binary64 addition of one would stay at the first value. This distinguishes two implementable algorithms, not an identified arithmetic route for every Excel build.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[MAKEARRAY](FUNC.MAKEARRAY.md) computes each entry from coordinates; [RANDARRAY](FUNC.RANDARRAY.md) generates random entries; [SCAN](FUNC.SCAN.md) returns intermediate accumulator values.

## Sources

- [Microsoft: SEQUENCE function](https://support.microsoft.com/en-us/office/sequence-function-57467a98-57e0-4817-9f14-2eb78519ca90), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc SEQUENCE preparation and direct-index generation, `SequenceWork`](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/dynamic_array_reshape_family.rs).
