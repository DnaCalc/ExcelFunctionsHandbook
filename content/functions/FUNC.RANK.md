---
schema: efh.function-page/v1
function_id: FUNC.RANK
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

# RANK

## What it does

`RANK(number,ref,[order])` returns a number’s position within numeric observations. The default order is descending, so the largest value has rank one; a nonzero order requests ascending rank. It is the compatibility spelling of the tied-equal rank.

## Try it

With `30,20,20,10` in B2:B5, `=RANK(20,B2:B5)` returns `2`. Both twenties have rank two and the ten has rank four: ties leave a gap.

## Quirks

Text, logical values and blanks in the reference are ignored, while zeros and repeated numbers remain observations. Ties share their first occupied rank; they do not receive consecutive ranks or an average. A number absent from the admitted data gives `#N/A` in the current helper.

Microsoft describes any nonzero `order` as ascending. The current wrapper truncates its numeric order toward zero before testing it, so fractional flags between minus one and one select descending. That is an implementation rule, not a measured statement about every Excel build. A nonnumeric target is also `#N/A` in this wrapper; it is not automatically parsed from numeric-looking text.

## How to implement

1. Validate the target as the current wrapper does: preserve a numeric value or worksheet error; other target kinds give `#N/A`.
2. Collect the reference’s numeric observations. Coerce and truncate a supplied order flag, then test zero versus nonzero.
3. Confirm the target occurs. If it is absent, return `#N/A`.
4. Descending rank is `1+count(value>target)`; ascending rank is `1+count(value<target)`. Equal observations contribute no preceding positions, so each member of a tie receives the same result.

With `30,20,20,10`, only thirty exceeds twenty, giving rank two. All three other observations exceed ten, giving rank four. In ascending order thirty is fourth rather than first.

For a single query, counting avoids sorting and uses exact comparisons of the admitted numeric values. A sorted index can support many queries, provided it retains multiplicities and the same equality rule. Pairing this function with ROW-based tie breakers produces unique positions, which is a different ranking convention.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[RANK.EQ](FUNC.RANK.EQ.md) makes this tie rule explicit; [RANK.AVG](FUNC.RANK.AVG.md) averages tied positions; [PERCENTRANK.INC](FUNC.PERCENTRANK.INC.md) returns fractional standing.

## Sources

- [Microsoft: RANK function](https://support.microsoft.com/en-us/excel/functions/rank-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/statistics.rs); implementation steps and the limits identified in the text.
