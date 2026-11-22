---
schema: efh.function-page/v1
function_id: FUNC.ISNUMBER
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
family: isnumber
role_in_family: Sole member of its module; the Number-kind test, and the family's canonical
  demonstration that IS arguments are not converted.
---

## What it computes

`ISNUMBER(value)` returns `TRUE` when the value it receives has kind `Number`, and `FALSE` for
every other kind.

`Number` in Excel's value universe means one thing: an IEEE 754 double-precision (binary64)
floating-point value ([the value universe](../model/01-value-universe.md)). Dates, times,
currency amounts, percentages and durations are all `Number` — they are numbers wearing display
formats — so `ISNUMBER` returns `TRUE` for a cell showing `2024-03-01`, for one showing
`14:30`, and for one showing `$1,250.00`. Formatting is not part of the value.

`Logical` is **not** `Number`. `TRUE` converts to 1 on demand in arithmetic contexts, but its
kind is `Logical`, and `ISNUMBER(TRUE)` is `FALSE`.

`ISNUMBER` is the function Microsoft chose to illustrate the whole family's non-conversion rule,
and the illustration is the most useful sentence on the topic anywhere: in `ISNUMBER("19")`,
`"19"` is not converted from text to a number, so the answer is `FALSE`.

## Arguments

`value` — required, exactly one. The published signature is `ISNUMBER(value)`.

Not converted. Microsoft's remark: "The value arguments of the IS functions are not converted.
Any numeric values that are enclosed in double quotation marks are treated as text." This is the
axis on which `ISNUMBER` differs from every "is this numeric?" test a programmer's instinct
reaches for. It does not parse. It does not try. It reads the kind.

The consequence in real workbooks: numbers imported as text — the leading-apostrophe column, the
CSV field with a trailing space, the figure pasted out of a web page — are `Text`, and
`ISNUMBER` correctly reports `FALSE` on all of them. That is not a limitation, it is the
diagnosis, and it is why `ISNUMBER` is the standard tool for finding text-that-looks-numeric in
an imported range.

The argument is a values position: a reference is resolved before the function runs
(`ArgPreparationProfile::ValuesOnlyPreAdapter`), so `ISNUMBER` cannot see whether its value
arrived as a literal or through `A1`.

## Result and edge cases

Returns a `Logical`, or an array of `Logical` when the argument is an array.

- **`Empty` is `FALSE`.** An empty cell has no number in it; that emptiness maps to `0` in most
  arithmetic contexts, but `ISNUMBER` classifies rather than converts, so the zero never
  materializes.
- **`Error` is `FALSE`, and does not propagate.** `ISNUMBER` is one of the declared per-family
  exceptions to error propagation ([coercion and lifting](../model/02-coercion-and-lifting.md)).
- **Subnormals and extremes are numbers.** The smallest positive subnormal double and the
  largest finite double are `Number`; there is nothing at the edges of the format that changes
  the kind.
- **Non-finite values do not arise.** Excel cells never hold infinities or NaN; functions that
  could produce them publish `#NUM!` instead ([the call pipeline](../model/03-call-pipeline.md)).
  So there is no "is it a number but not finite?" case to worry about.
- **Arrays** are classified elementwise, giving a same-shaped mask — the standard idiom for
  auditing a range: `SUMPRODUCT(--ISNUMBER(A1:A100))` counts how many entries are genuinely
  numeric.
- **Rich values.** What `ISNUMBER` says about a linked data type or an in-cell image, whose core
  projection is typically text, is not established here.

## Errors

`ISNUMBER` returns no error of its own for any value it can be given. The only failure available
to it is **arity**: zero arguments, or two. `ISNUMBER()` is expected to be refused at formula
entry rather than evaluated ([the call pipeline](../model/03-call-pipeline.md)); the reference
engine, having no entry-time surface, reports `#VALUE!` for both the too-few and the too-many
case.

Microsoft's IS-functions page documents no error return for `ISNUMBER`.

## Relationships

- **`ISTEXT`** is its near-complement for the two kinds that matter in imported data, but the
  two are not complements in general: `ISNUMBER` and `ISTEXT` are both `FALSE` for `Empty`,
  `Logical` and `Error`.
- **`ISNONTEXT`** is the true complement of `ISTEXT`, not of `ISNUMBER`. `ISNONTEXT(TRUE)` is
  `TRUE`; `ISNUMBER(TRUE)` is `FALSE`.
- **`N`** is the conversion `ISNUMBER` declines to perform, and it does not help: `N` maps *all*
  text to `0`, so `N("19")` is `0`, not `19`.
- **`VALUE`** and **`NUMBERVALUE`** are the functions that actually parse text into a number,
  and they are the correct follow-up once `ISNUMBER` has told you a column is text. `NUMBERVALUE`
  takes explicit decimal and group separators and is the one to reach for when the text came
  from another locale.
- **`TYPE`** returns `1` for exactly the values `ISNUMBER` calls `TRUE`.
- **`ISNUMBER(SEARCH(needle, haystack))`** is the canonical substring test, because `SEARCH`
  returns a position or `#VALUE!` and `ISNUMBER` converts that pair into a clean boolean. It is
  probably this function's single most common appearance in real formulas.

## Notes for implementers

1. **Resist adding a numeric-text fast path.** Every implementer's instinct is that `ISNUMBER`
   should say `TRUE` for `"19"`. Excel documents that it does not, and a workbook's error
   auditing depends on it not doing so.
2. **Dates must be `Number`, not a separate date kind.** An engine that models dates as their own
   value kind will get `ISNUMBER` wrong on every date cell. The format is presentation; the value
   is a serial number.
3. **`ISNUMBER` classifies arrays with a dedicated walk in the reference engine** rather than the
   generic scalar lift used by some siblings. It reaches the same shape; it is not the same code
   path, so it needs its own array-shape evidence.
4. **`Empty` must stay distinct from `Number(0)` through argument preparation.** Collapsing them
   early flips `ISNUMBER` on every blank cell.

## What has not been checked

No Handbook vector suite exists for `ISNUMBER`; `vectors/` publishes nothing for this function.
No Excel-comparison evidence record names `ISNUMBER` as a subject — **nobody has checked this
function's answers against Excel inside the Handbook's record.** The reference engine's own
answers appear on the battery panel under their own label; they are not Excel's answers.

The probes that would settle the open questions, chosen because each one could plausibly go the
other way:

1. **Rich values.** `ISNUMBER` of a linked data type cell (a stock, a geography) and of a cell
   holding an in-cell image. The core projection is usually text, which predicts `FALSE`, but
   the rich layer is exactly where a projection rule could differ.
2. **A number formatted as text by the cell format** (format code `@`) versus a number stored as
   text. These are different states and only one of them is `Text`; the pair should be on record
   together.
3. **Values arriving from an add-in through the C API** — the raw-return boundary is broader
   than the published-result boundary
   ([the value universe](../model/01-value-universe.md)), so a value that normalizes on
   publication may be classified differently mid-evaluation.
4. **A date before 1900 and the 1900 leap-year artefact** — still `Number`, expected, unmeasured.
5. **Arity at entry** — whether Excel refuses `=ISNUMBER()` at entry or evaluates it.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `ISNUMBER` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-ISNUMBER-001: ISNUMBER, ISTEXT, ISNONTEXT, and ISLOGICAL perform no coercion, treating numeric/logical text as text and rich images as ISNONTEXT TRUE

*Quirk; measured; recorded 2026-11-22.* Applies to ISLOGICAL, ISNONTEXT, ISNUMBER, ISTEXT.

In Excel 16.0 builds 20430 and 20527, ISNUMBER, ISTEXT, ISNONTEXT, and ISLOGICAL test the
runtime value type without implicit conversion: ISNUMBER("19") and ISNUMBER(TRUE) are FALSE,
ISLOGICAL("TRUE") and ISLOGICAL(0) are FALSE, ISTEXT("") is TRUE, and ISNONTEXT is TRUE for
numbers, logicals, blank cells, worksheet errors, and _webimage pictures (while ISTEXT,
ISNUMBER, and ISLOGICAL are FALSE for _webimage pictures). Each matches 1,227/1,227 full-bank
and 120/120 pinned rows.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=ISNUMBER("19")` | as typed | `FALSE` | Excel 16.0.20527.20196 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=ISTEXT("")` | as typed | `TRUE` | Excel 16.0.20527.20196 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=ISNONTEXT(A1)` | as typed | `TRUE` | Excel 16.0.20527.20196 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=ISLOGICAL(A11)` | as typed | `FALSE` | Excel 16.0.20527.20196 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `ISNUMBER(0)` | value `0.0` (`0x0000000000000000`) (cells) | `TRUE` | Excel 16.0 build 20430, Windows 64-bit, cell_value2_bulk; CPU not recorded |

Basis (measured): Measured across Excel 16.0.20527.20196 (dc_nzpw rows 39-61, 152-155,
177-180), Excel 16.0.20527.20088 (jnt_logical_info rows 18-21, 115-130), Excel 16.0.20527
(fetch.rs rows 516-517), and 1,227/1,227 full-bank and 120/120 pinned rows for each function.

Evidence:

- DnaCalc `crates/calc-func/src/catalogue/information.rs`, isnumber; istext; isnontext; islogical (lines 154-192) at `c08fee0a47` — Strict runtime type checks without coercion
- DnaCalc `crates/calc-func/witness-status.tsv`, ISLOGICAL; ISNONTEXT; ISNUMBER; ISTEXT at `c08fee0a47` — 1,227/1,227 full-bank and 120/120 pinned rows matched for each
- DnaCalc `crates/calc-func/tests/captures/dc_nzpw/oracle.capture.json`, sheet information_functions row 42 column 4 at `c08fee0a47` (readings)
- DnaCalc `crates/calc-func/tests/captures/dc_nzpw/oracle.capture.json`, sheet information_functions row 48 column 4 at `c08fee0a47` (readings)
- DnaCalc `crates/calc-func/tests/captures/dc_nzpw/oracle.capture.json`, sheet information_functions row 51 column 4 at `c08fee0a47` — A1 is blank; #N/A in B7 (row 54) is also TRUE (readings)
- DnaCalc `crates/calc-func/tests/captures/dc_nzpw/oracle.capture.json`, sheet information_functions row 59 column 4 at `c08fee0a47` — A11 holds text "TRUE" (readings)
- DnaCalc `crates/calc-func/tests/witnesses/ISNUMBER.json`, w111broad-20260929-ISNUMBER-00000 at `c08fee0a47` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| `Number` | The IEEE 754 binary64 value kind; includes dates, times, currency, percentages |
| non-conversion | The documented rule that IS-function arguments are inspected, not coerced |
| numeric text | `Text` that would parse as a number; `ISNUMBER` reports `FALSE` for it |
| mask | The same-shaped array of `Logical` produced from an array argument |

## Sources

- Microsoft, "IS functions" —
  <https://support.microsoft.com/en-us/office/is-functions-0f2d7971-6019-40a0-a171-f2d869135665>.
  Read for this page: the `ISNUMBER` row, the non-conversion remark, and the `ISNUMBER("19")`
  example given verbatim in that remark.
- Handbook, [the value universe](../model/01-value-universe.md) — the `Number` kind, dates as
  formatted numbers, the rich-value core projection, and the boundary admission matrix.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — text-to-number
  conversion as a separate, opt-in operation, and the inspection functions' exemption from error
  propagation.
- Handbook, [the call pipeline](../model/03-call-pipeline.md) — values-only preparation and the
  absence of non-finite values at the cell boundary.
- `data/functions/FUNC.ISNUMBER.json` — identity (`xlfIsnumber`, code 128), the published
  signature `ISNUMBER(value)`, arity, declared axes, as projected at OxFunc `473efa3`.
