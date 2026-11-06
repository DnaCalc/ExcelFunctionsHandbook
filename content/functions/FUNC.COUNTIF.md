---
schema: efh.function-page/v1
function_id: FUNC.COUNTIF
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

# COUNTIF

## What it does

`COUNTIF(range, criteria)` counts cells satisfying one condition. The criterion can be a number, text, a cell containing the condition, or a comparison such as `">=10"`.

## Try it

Put `4,11,7,13` in `B2:B5`:

| Formula | Count | Why |
|---|---:|---|
| `=COUNTIF(B2:B5,">=10")` | 2 | Eleven and thirteen qualify |
| `=COUNTIF(B2:B5,">"&B4)` | 2 | B4 supplies seven, so the criterion becomes `">7"` |
| `=COUNTIF(B2:B5,">=7")-COUNTIF(B2:B5,">11")` | 2 | Include seven through eleven and remove larger values |

For text, put `sku-1`, `SKU-2`, `sku-*`, `other` in `D2:D5`. `=COUNTIF(D2:D5,"sku-?")` counts three: case is ignored and the question mark accepts any one final character. `=COUNTIF(D2:D5,"sku-~*")` counts one, matching the literal asterisk.

## Quirks

Text matching is case-insensitive. `*` matches a sequence and `?` one character; prefix either with `~` to match it literally. Operators belong inside quoted criteria. To use an operator with a cell’s value, concatenate them with `&`; placing the cell address inside the quoted text does not dereference it.

A numeric equality test and a numeric ordering test are different predicates. The current matcher can convert numeric-looking text for equality, while ordering criteria compare numeric cells. Preserve cell kinds through matching instead of coercing the whole range to numbers first.

Microsoft documents misleading results when matching strings longer than 255 characters and `#VALUE!` for calculated references into a closed workbook. Leading or trailing spaces can also change a text match. The current implementation’s criteria and Unicode coverage limits are separate from those documented worksheet behaviours.

## How to implement

1. Resolve a single-area range reference. The current implementation requires a reference; a supplied array value is a distinct route and is not silently turned into one.
2. Read the criterion once. Dereference a one-cell criterion reference. Keep numeric, logical, error, empty and text criteria as different kinds; an empty criterion cell becomes numeric zero.
3. For text, recognize the longest leading operator first: `<=`, `>=`, `<>`, then `<`, `>`, `=`. Missing operator means equality. Parse the remaining operand as an error literal, a wildcard pattern, an admitted number or logical, or ordinary text under the current rules.
4. Match each cell with that typed predicate and count matches. Patterns need `*`, `?` and tilde escaping with the supported case mapping. Blank cells and empty strings must remain distinct until the predicate chooses their verdict.
5. Return the count as a number. Do not evaluate the criterion string as arbitrary formula code.

For the numeric example, four and seven fail `>=10`, while eleven and thirteen pass. The interval expression counts three values at least seven, then subtracts the one above eleven, leaving two.

The current code limits criterion text to 255 UTF-16 code units and refuses some unresolved collation, text-coercion and array-criterion cases. A typed implementation refusal reports an unsupported case; it is not evidence that Excel returns a worksheet error there. Error-valued criteria also need their own predicate handling rather than an unconditional exception path.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[COUNTIFS](FUNC.COUNTIFS.md) combines conditions with AND. For OR conditions, account for overlap when adding separate counts. [FILTER](FUNC.FILTER.md) returns matching rows.

## Sources

- [Microsoft: COUNTIF function](https://support.microsoft.com/en-us/excel/get-started/use-the-countif-function-in-microsoft-excel), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc criteria grammar, range validation and count planning](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/conditional.rs); implementation rules and its explicit unsupported cases.
