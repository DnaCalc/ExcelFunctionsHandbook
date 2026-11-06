---
schema: efh.function-page/v1
function_id: FUNC.MULTINOMIAL
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

# MULTINOMIAL

## What it does

`MULTINOMIAL(number1,[number2],...)` counts arrangements of groups with the supplied sizes: factorial of the total divided by the product of each size’s factorial.

## Try it

`=MULTINOMIAL(2,2,1)` returns `30`. Five items with two identical A items, two identical B items and one C item have `5!/(2!*2!*1!)=120/4` distinct arrangements.

## Quirks

Negative arguments give `#NUM!`; nonnumeric arguments give `#VALUE!`. The current kernel truncates admitted sizes toward zero. Large factorial intermediates can overflow even when a ratio-based calculation is practical. Numerical publication near large integers needs its own compatibility evidence.

## How to implement

For a mathematical integer implementation, start `total=0,result=1`. For each nonnegative integer group size c, multiply the result by `choose(total+c,c)` and increase total by c. Compute each choose with exact integer arithmetic or a cancellation-based recurrence. The example multiplies `choose(2,2)=1`, `choose(4,2)=6`, and `choose(5,1)=5`, yielding thirty. This avoids separately constructing all factorials. The current product also contains log/Lanczos publication candidates and bounded rounding heuristics; the exact-integer recipe is not an identification of those Excel last-bit choices.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[COMBIN](FUNC.COMBIN.md) chooses one group; [FACT](FUNC.FACT.md) is the factorial primitive; [PERMUT](FUNC.PERMUT.md) distinguishes ordered selections.

## Sources

- [Microsoft: MULTINOMIAL function](https://support.microsoft.com/en-us/excel/functions/multinomial-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/multinomial.rs); implementation steps and the limits identified in the text.
