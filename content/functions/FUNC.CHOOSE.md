---
schema: efh.function-page/v1
function_id: FUNC.CHOOSE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
  - Notes
  - Page vocabulary
  - Sources
family: choose_ifs_family
role_in_family: >-
  Selects one argument by ordinal position; the family's index-driven selector, as against
  IFS, which selects by the first satisfied condition.
---

## What it computes

`CHOOSE(index_num, value1, value2, …)` is positional selection: it returns the argument at
ordinal position `index_num` in the value list. `index_num` of 1 selects `value1`, 2 selects
`value2`, and so on. That is the entire semantic core.

Two properties make it more than a table lookup written sideways.

**It selects arguments, not values.** The projection records
`arg_preparation_profile: RefsVisibleInAdapter` — the arguments arrive as live references where
they are references, so `CHOOSE` can hand back a *reference* rather than the reference's
contents. This is why `SUM(CHOOSE(2, A1:A10, B1:B10))` sums a column: `CHOOSE` returns the
range, and `SUM` scans it. A function that returned only values could not do that.

**It is a branch selector in the pipeline's error model.** The projection records
`error_collapse_profile: SelectorBranch` with `error_algebra: CanonicalExcelLegacy` — the same
classification carried by `IF`, `IFS`, `IFERROR`, `IFNA` and `SWITCH`. See
[the call pipeline](../model/03-call-pipeline.md) for what that axis means: these are the
functions that choose among branches which may themselves be errors, and Excel's legacy
error-precedence order governs the collapse when several errors compete.

## Arguments

| Argument | Meaning |
|---|---|
| `index_num` | Which value to return, counting from 1. Microsoft documents that it must be a number between 1 and 254, a formula yielding such a number, or a reference to a cell containing one. Microsoft further documents that a fractional `index_num` is truncated to the lowest integer before use. |
| `value1` | The first selectable argument. Required. |
| `value2`, … | Further selectable arguments, up to the arity ceiling. Optional. |

The arity is 2 to 255 — one index plus up to 254 values, which is exactly the ceiling
Microsoft's documented `index_num` range implies.

`value` arguments may be numbers, text, cell references, defined names, formulas, functions, or
— importantly — ranges. The misunderstanding to head off: **the values are not a list you index
into, they are argument slots.** You cannot pass an array of candidates in one argument and have
`CHOOSE` pick from inside it; `CHOOSE({10,20,30}, …)` indexes the argument list with an array of
indices, which is a different operation (see below).

## Result and edge cases

The return kind is whatever the selected argument is: a scalar, an array, or a reference. With a
scalar `index_num` there is no coercion of the selected value — that is what "returns it verbatim"
means for a branch selector. An array `index_num` differs, as the first item below says.

Specific behaviours:

- **Array `index_num`.** Microsoft documents that if `index_num` is an array, every value is
  evaluated when `CHOOSE` is evaluated. Read that carefully: it is a statement about
  *evaluation*, not about the result. The practical consequence is that `CHOOSE` is not lazy
  in the array case, so side conditions you were relying on to stay unevaluated (a branch that
  would error, a slow branch) are evaluated anyway. In dynamic-array Excel `CHOOSE` then returns
  an array of selections, and selected empty cells come back as the number 0 rather than as
  blank references (FN-CHOOSE-001).
- **Reference results.** Because the selected argument can be a reference, `CHOOSE` composes
  with range-consuming functions. Whether the result can serve as an endpoint of the range
  operator (as `INDEX`'s reference form can) is an open question on this page.
- **Omitted argument slots.** `CHOOSE(2, A1, , B1)` delivers the Missing marker into a value
  slot. Missing and Empty are distinct at the call boundary
  ([value universe](../model/01-value-universe.md)); what `CHOOSE` returns when the *selected*
  slot is Missing is a genuine question, since Missing is not admitted at the published-result
  boundary.
- **`index_num` from text.** A direct text argument that reads as a number coerces under the
  ordinary rules in [coercion and lifting](../model/02-coercion-and-lifting.md); text arriving
  from a scanned cell is a different lane. `CHOOSE` is not an aggregate, so the direct/scan
  asymmetry that governs `SUM` does not obviously apply — but "does not obviously apply" is not
  a finding.

## Errors

Microsoft's page documents the failure conditions in terms of `index_num`:

- `index_num` less than 1, or greater than the number of values supplied, produces `#VALUE!`.
- `index_num` that is a fraction is truncated to the lowest integer before being used, so
  fractions are not by themselves an error.

Beyond that, `CHOOSE` is a selector: an error in the *selected* argument is the result, and
errors in unselected arguments are — subject to the array-`index_num` evaluation caveat above —
not the result. When several errors compete, the `SelectorBranch` collapse applies Excel's
canonical legacy precedence order (`ErrorAlgebra::CanonicalExcelLegacy` in the projection).

## Relationships

- **`SWITCH`** is the modern equivalent for matching a value against a list of cases. Where
  `CHOOSE` indexes by position, `SWITCH` matches by equality and supports a default. New code
  that is really doing case analysis should use `SWITCH`.
- **`IFS`** — `CHOOSE`'s module sibling in the reference implementation — selects by the first
  satisfied condition rather than by ordinal.
- **`INDEX`** is the near-neighbour that indexes into *data* rather than into an argument list.
  If your candidates live in cells, `INDEX` is the right tool; `CHOOSE` is for candidates that
  live in the formula.
- **`CHOOSECOLS` and `CHOOSEROWS`** share a prefix and nothing else. They select columns or
  rows from a single array; they are dynamic-array reshapers, not branch selectors. The naming
  collision is the most reliable source of confusion in this corner of the catalog.
- **`LOOKUP` with a constant vector** does the same job for larger candidate lists and keeps the
  candidates out of the formula text.

## Notes for implementers

- **Selection must precede evaluation for the scalar case, and cannot for the array case.**
  Those are two different execution strategies for one function, and the documented array
  caveat is the seam between them. An implementation that always evaluates all arguments will
  agree with Excel on results and disagree on cost and on error visibility for branches that
  throw; an implementation that always defers will disagree with the documented array
  behaviour.
- **The reference must survive selection.** Dereferencing a `value` argument before returning it
  breaks `SUM(CHOOSE(2, A1:A10, B1:B10))` in a way that unit tests on scalars will never catch.
  This is the practical content of `RefsVisibleInAdapter` for this function.
- **The 254-value ceiling is documented in terms of `index_num`, and the arity ceiling is 255
  arguments.** Those are two statements of one limit; an implementation should enforce it in
  one place.
- The reference implementation places `CHOOSE` with `IFS` in a shared module, which is a
  statement about code organization rather than about Excel: the two functions share a
  selector skeleton.

## What has not been checked

No Handbook vector suite exists for `CHOOSE`, and no Excel-comparison evidence record is
recorded for it. Apart from the two readings in FN-CHOOSE-001, nobody has checked this function
against Excel here.

The probes that would matter most, in order:

1. **`CHOOSE` with an array `index_num` in modern Excel** lifts: `CHOOSE({1,2}, Data!Z1, Data!Z2)`
   is a 1 by 2 array, with selected empty cells as 0 (FN-CHOOSE-001).
2. **`CHOOSE` returning a range**, consumed three ways: by `SUM` (scan), by the range operator
   (`A1:CHOOSE(1, C5, D5)`), and bare in a cell. These three distinguish "returns a reference"
   from "returns the reference's value" from "returns a reference that is not composable".
   With a scalar index the reference survives: `ISREF(CHOOSE(1, ref))` is `TRUE` (FN-CHOOSE-001).
3. **Boundary indices**: 0, negative, exactly the value count, one past it, 254, 255, and a
   fraction just below an integer (`0.999`, `1.999`) to confirm truncation direction against
   the documented "truncated to the lowest integer".
4. **A Missing slot selected** (`CHOOSE(2, 1, , 3)`) and an Empty cell selected, recorded
   separately — the value model insists they are distinct kinds and this is a cheap place to
   see whether Excel agrees.
5. **Error competition**: several error-valued arguments with an out-of-range index, to observe
   the `SelectorBranch` precedence order in practice.

Item 2 is the one whose answer would change the prose above rather than extend it.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `CHOOSE` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-CHOOSE-001: An array index in CHOOSE turns selected blank cells into 0

*Quirk; measured; recorded 2026-11-04.*

When an array index_num lifts CHOOSE in dynamic-array Excel, selected empty cells come back as
the number 0 rather than as blank references, so ISBLANK over them is FALSE. With a scalar
index the selected reference survives: ISREF(CHOOSE(1,ref)) was recorded TRUE.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=ISBLANK(CHOOSE({1,2},Data!Z1,Data!Z2))` | as typed | `{FALSE,FALSE}` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=ISREF(CHOOSE(1,[ylr-source.xlsx]Sheet1!A1))` | as typed | `TRUE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): One recorded array reading, with ROWS 1 and COLUMNS 2 from observer cells,
and one scalar ISREF reading with a reference to another workbook.

Inferred, not measured: That a scalar index over an empty cell gives ISBLANK TRUE is inferred
from the ISREF reading. No ISBLANK reading of it is recorded.

Evidence:

- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q193 at `eceb809985`
- DnaCalc `.beads/issues.jsonl`, dc-b1l.24 at `1ec418b2c1` — review record
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q193, row 20, column 10 at `eceb809985` (readings)
- DnaCalc `crates/calc-host/tests/captures/dc_ylr/oracle.capture.json`, sheet Main, row 54, column 1 at `2d0eddcac8` — The recorded formula names the source workbook by its full path (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| branch selector | A function that chooses among argument branches; `ErrorCollapseProfile::SelectorBranch` |
| ordinal selection | Selection by position in the argument list, counting from 1 |
| verbatim return | The selected argument is returned without coercion |
| value slot | One `value` argument position, as distinct from an element inside an array |
| legacy error precedence | Excel's classic ordering used when several error inputs compete |

## Sources

- Microsoft, CHOOSE function —
  <https://support.microsoft.com/en-us/office/choose-function-fc5c184f-cb62-4ec7-a46e-38653b98f5bc>
  (the 1–254 `index_num` range, truncation of fractional indices, `#VALUE!` for out-of-range
  indices, and the array-`index_num` evaluation note).
- Handbook `content/model/01-value-universe.md` (Missing versus Empty; reference kind).
- Handbook `content/model/02-coercion-and-lifting.md` (direct-argument coercion; error
  propagation).
- Handbook `content/model/03-call-pipeline.md` (`RefsVisibleInAdapter`, `SelectorBranch`,
  `ErrorAlgebra::CanonicalExcelLegacy`, lazy branch selectors).
- Handbook `data/functions/FUNC.CHOOSE.json` and `data/presence/FUNC.CHOOSE.json` (arity,
  classification axes, implementing module shared with `IFS`).
