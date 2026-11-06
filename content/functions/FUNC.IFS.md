---
schema: efh.function-page/v1
function_id: FUNC.IFS
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-STRUCT-0006
open_problems: []
references: []
episodes: []
body_sections:
  - "What it does"
  - "Try it"
  - "Quirks"
  - "How to implement"
  - "Version differences"
  - "Alternatives"
  - "Sources"
family: choose_ifs_family
role_in_family: "The flat condition ladder: scans condition/value pairs left to right and returns the first match."
---

# IFS

## What it does

`IFS(test1, value1, [test2, value2], …)` chooses the value paired with the first true test.
Order matters: put more specific or higher-priority conditions first. Microsoft documents up to
127 condition/value pairs.

There is no separate default argument. A final `TRUE, fallback` pair supplies a default;
without any true test, the result is `#N/A`.

For example, `=IFS(85>=90,"A",85>=80,"B",TRUE,"C")` selects `"B"`.

## Try it

Start with the example's first true condition, then make it false to reach the fallback. Remove
the final `TRUE` pair to see the unmatched case. Keep the condition/value pairs balanced.

## Quirks

Microsoft documents `#VALUE!` if an evaluated condition cannot resolve to TRUE or FALSE, and an
entry-time error message if a condition has no paired value. Those are distinct from an unmatched
but otherwise valid call returning `#N/A`.

The selected value can retain its reference identity. On the recorded build,
`ISBLANK(IFS(TRUE,Data!Z1))` returns TRUE when that cell is empty. Converting every selected
reference to a number would lose this behaviour.

Later arrays do not necessarily enlarge the result. The recorded
`IFS(TRUE,1,{FALSE,TRUE},{2,3})` returns one 1. This observation is about the selected result and
shape; it does not establish that every unselected expression avoids evaluation. Do not infer a
general short-circuit guarantee from the phrase “first true condition”.

## How to implement

Represent the condition/value pairs explicitly and reject unpaired syntax at the formula
boundary. Determine the first matching condition according to Excel's logical coercion and
error rules, then return its corresponding value with its reference or callable identity intact.

Treat argument evaluation policy and result selection as separate design choices. Test an
unselected error, volatile function, recursive lambda and array independently before replacing
IFS with a chain of lazy IF operations. Selecting the same visible value does not prove the
same evaluation behaviour.

For array inputs, measure which conditions establish the output shape and when selection stops
that process. Apply errors per output coordinate where the measured rule calls for it. A blanket
broadcast over every argument contradicts the scalar-selection example above.

## Version differences

Microsoft lists IFS for Office 2019 and later perpetual releases, and Microsoft 365. The named
build observations below cover reference preservation and result shape. They do not establish
an evaluation-order contract for all versions.

## Alternatives

Use nested [IF](FUNC.IF.md) when its branch evaluation behaviour is required. Use
[SWITCH](FUNC.SWITCH.md) when comparing one expression with several candidate values. Use a
lookup table when the conditions encode a growing data table. SUMIFS and COUNTIFS aggregate
matching cells; they do not select a single branch.

## Sources

- [Microsoft: IFS function](https://support.microsoft.com/en-gb/excel/functions/ifs-function).
- [Recorded selector examples](../notes/FN-IFS-001.json).
