---
schema: efh.function-page/v1
function_id: FUNC.PERCENTOF
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

# PERCENTOF

## What it does

`PERCENTOF(data_subset,data_all)` sums the first set and divides by the sum of the second. It is useful as a two-argument aggregation callback in GROUPBY and PIVOTBY. The result is a fraction; percentage cell formatting changes its display.

## Try it

`=PERCENTOF({2,3},{2,3,5,10})` returns `0.25`, displayed as 25 percent with percentage formatting. The subset sums to five and the full set to twenty.

The denominator is a supplied comparison set. In a pivot cell with amount three, `PERCENTOF(3,{3,5})` gives `0.375`, while `PERCENTOF(3,{3,7})` gives `0.3`. The same numerator can therefore describe a column share or a row share. PIVOTBY's relative_to argument chooses that second set for a callback.

## Quirks

The function does not verify that the first set is a literal subset of the second; supplied totals can produce a value above one or below zero. A zero denominator gives `#DIV/0!`. In recorded build-20527 controls, direct numeric text/logicals convert, while errors held in a reference or array collapse to `#NUM!`; a direct error retains its own code.

## How to implement

Read each argument under its aggregate admission rules, accumulating each total with the established arithmetic-site addition. Divide subset total by whole total at a stored site and apply zero-denominator/error policy. In a grouping callback the caller supplies both the local values and the appropriate comparison set; the latter need not be the entire original data table. The example gives `5/20=0.25`. Keep collection origin so direct and held error handling do not become indistinguishable.

## Version differences

The current implementation comments retain COM readings from Excel 16.0 build 20527 on 30 October 2026. They cover direct versus held coercion/errors, including `PERCENTOF(1,4)=0.25`. No same-input comparison with another named build is recorded here. Microsoft’s opened page lists Excel for Microsoft 365; that is product availability, not a measured build comparison.

## Alternatives

`SUM(subset)/SUM(all)` expresses the documented arithmetic relationship. [GROUPBY](FUNC.GROUPBY.md) and [PIVOTBY](FUNC.PIVOTBY.md) supply grouping context.

## Sources

- [Microsoft: PERCENTOF function](https://support.microsoft.com/en-us/excel/functions/percentof-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/group_pivot.rs); implementation steps and the limits identified in the text.
