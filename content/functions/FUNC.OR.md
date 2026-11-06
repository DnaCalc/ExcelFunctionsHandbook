---
schema: efh.function-page/v1
function_id: FUNC.OR
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
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
family: or_fn
role_in_family: "The disjunction reducer: scans every argument for truth values and returns TRUE on the first truthy one."
---

# OR

## What it does

`OR(logical1, [logical2], …)` returns TRUE if at least one considered value is true, otherwise
FALSE. It accepts up to 255 arguments. Numeric zero is false; other numeric values are true.

`=OR(FALSE,2>1)` returns TRUE. An array or range is scanned as a collection of values and the
result is one logical value, rather than a separate result for every position.

## Try it

Try `=OR(FALSE,2>1)`, `=OR(0,0)` and `=OR({0;1;0})`. A range example needs actual input cells:
enter B2 and B3 in the input-cell editor, then calculate `=OR(B2:B3)`.

## Quirks

Microsoft documents that text and empty cells in a referenced range or array are ignored. If
there is no logical or numeric value to consider, OR returns `#VALUE!`. An all-ignored scan is
different from considering one or more false values.

Do not assume OR is a programming-language short-circuit operator. A true value does not by
itself establish that an error or an expensive expression elsewhere in the call is skipped.
Text supplied directly as an argument also needs its own coercion checks; the rule about text
in a range is not a universal rule about text.

## How to implement

Track both an accumulated truth value and whether any eligible item has been seen. Classify
direct arguments and referenced or array-contained values according to their distinct coercion
rules. Fold eligible values into the accumulator, preserving the required error behaviour.
After a successful scan, return `#VALUE!` if none were eligible, otherwise return the logical
accumulator.

Share the item classifier with AND and XOR so that blank, text and error policies cannot drift.
A fast exit on TRUE is valid only if it preserves argument evaluation and error ordering for the
target Excel build. Test that separately from the truth table.

## Version differences

Microsoft documents OR for Microsoft 365 and Excel 2016 through 2024. No comparison of error
ordering or direct-text coercion between particular builds is recorded here.

## Alternatives

[AND](FUNC.AND.md) requires every considered value to be true. [XOR](FUNC.XOR.md) tests whether
an odd number are true. Use [IF](FUNC.IF.md) to select between calculations. For elementwise
logic in an array, ordinary comparisons or MAP may express the intended shape more directly.

## Sources

- [Microsoft: OR function](https://support.microsoft.com/en-us/excel/functions/or-function).
- [Coercion and lifting](../model/02-coercion-and-lifting.md).
