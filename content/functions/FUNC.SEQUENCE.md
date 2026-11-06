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

`SEQUENCE(rows, [columns], [start], [step])` creates a rectangular array of regularly spaced numbers. Missing optional arguments default to one.

## Try it

`=SEQUENCE(2,3,10,2)` spills the rows `10,12,14` and `16,18,20`. The sequence continues across each row before moving to the next.

## Quirks

The result needs an unobstructed spill area. Closing a workbook that supplies a linked dynamic array can produce `#REF!` on refresh. A negative step makes a descending sequence; zero repeats the start value.

## How to implement

Validate dimensions before allocating storage. For zero-based row `r` and column `c`, the mathematical value is `start+(r*columns+c)*step`. Store values in row order and return the requested shape. For `(2,3,10,2)`, indices zero through five give ten through twenty. Allocation bounds and spill collisions belong to the workbook host. The expression identifies the values, not the exact floating-point operation order of every Excel build.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[MAKEARRAY](FUNC.MAKEARRAY.md) computes each entry from coordinates; [RANDARRAY](FUNC.RANDARRAY.md) generates random entries; [SCAN](FUNC.SCAN.md) returns intermediate accumulator values.

## Sources

- [Microsoft: SEQUENCE function](https://support.microsoft.com/en-us/office/sequence-function-57467a98-57e0-4817-9f14-2eb78519ca90), opened 6 November 2026; syntax, documented argument rules and errors.
