---
schema: efh.function-page/v1
function_id: FUNC.N
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
family: n_fn
role_in_family: Sole member of its module; the total value-to-number projection, and the reason
  numeric text silently becomes zero.
---

## What it computes

`N(value)` maps a value to a number by a fixed table. It is a **total function on the scalar
kinds** — it always produces something, and for one class of input that something is a silent
zero.

Microsoft's table, verbatim:

| If value is or refers to | `N` returns |
|---|---|
| A number | That number |
| A date, in one of the built-in date formats available in Microsoft Excel | The serial number of that date |
| `TRUE` | 1 |
| `FALSE` | 0 |
| An error value, such as `#DIV/0!` | The error value |
| Anything else | 0 |

Two rows carry all the risk.

**"Anything else" is 0, and it includes text that reads as a number.** `N("19")` is `0`, not
19. `N` is not a parser. This is the single fact that makes `N` dangerous in a formula that was
meant to salvage imported data: it converts every text cell — meaningful or not, numeric-looking
or not — to zero, with no error and no signal.

**The date row is not a special case.** A date *is* a number in Excel's value universe
([the value universe](../model/01-value-universe.md)); the built-in date format is presentation.
The row exists in Microsoft's table because a reader thinks of a date cell as holding a date, but
mechanically it collapses into the first row. What `N` returns for a date is the serial number
that was already there.

**Errors pass through unchanged.** `N` is not an error-masking function. `N(#DIV/0!)` is
`#DIV/0!`, documented explicitly.

Microsoft's own remark on the function's purpose is worth quoting because it sets expectations
correctly: "It is not generally necessary to use the N function in a formula, because Excel
automatically converts values as necessary. This function is provided for compatibility with
other spreadsheet programs."

## Arguments

`value` — required, exactly one. The published signature is `N(value)`.

Every scalar kind is admissible; none is rejected. The interesting positions are the ones
Microsoft's table does not name:

- **An empty cell.** Falls under "anything else" — `0`. It is the same output as text, which is
  why `N(A1)=0` cannot distinguish a blank from a label.
- **An omitted slot (`Missing`).** Not addressed by the documentation. The reference engine
  treats it as `0`, matching `Empty`; the Handbook has not observed Excel here.
- **An array.** Not addressed by the documentation. The reference engine maps the table over the
  elements and returns a same-shaped array. Whether Excel lifts `N` or takes a single element is
  **not established here**, and it is the page's main open question.
- **A reference.** Resolved to its value before `N` sees it
  (`ArgPreparationProfile::ValuesOnlyPreAdapter`), so `N(A1)` is `N` of `A1`'s contents.

## Result and edge cases

Returns a `Number`, an `Error` (when the input was an error), or an array of those.

- **No conversion failure exists.** `N` never returns `#VALUE!` for a value it can be given.
  Where `VALUE("abc")` errors, `N("abc")` is `0`. That difference — error versus silent zero —
  is the whole reason to prefer `VALUE` when you actually want a parse.
- **Text is always 0**, regardless of content, including the empty string and including
  `"TRUE"`.
- **Logicals convert**, unlike in most range-scan contexts where logicals are skipped. `N(TRUE)`
  is `1`; this is the documented behaviour and is the usual way to turn a boolean into a
  summable number, alongside the double-unary `--`.
- **Percentages, currency and times** are numbers already; `N` returns the stored value, so
  `N(50%)` is `0.5` and `N` of a cell showing `12:00` is `0.5` as well.
- **Errors propagate** rather than becoming a number.
- **Rich values.** What `N` does with a linked data type is not established here; the core
  projection is typically text, which the table would send to `0`.

## Errors

`N` produces no error of its own except for **arity** — zero arguments or two. `N()` is expected
to be refused at formula entry rather than evaluated
([the call pipeline](../model/03-call-pipeline.md)); the reference engine, having no entry-time
surface, reports `#VALUE!` for both.

Every other error on a page featuring `N` came from somewhere else and was passed through.

## Relationships

- **`T(value)`** is the mirror image: the total text projection, returning the text unchanged if
  the value is text and the empty string otherwise. `N` and `T` are a pair, both present for
  compatibility with older spreadsheet products, and both total.
- **`VALUE(text)`** is the function that actually parses. Use it when non-numeric text should be
  an error rather than a zero.
- **`NUMBERVALUE(text, decimal_separator, group_separator)`** is the locale-explicit parser and
  the right choice for text that came from another locale.
- **`ISNUMBER`** is the test that tells you which branch you are in before you commit to `N`.
- **`--` (double unary)** is the idiomatic modern replacement for `N` on boolean masks; it
  coerces through arithmetic, which means it *errors* on non-numeric text where `N` returns
  zero. The two are not interchangeable on messy data.
- **`SUM` and the aggregate family** show the same numeric-text asymmetry from the other side:
  `SUM("3")` is 3 as a direct argument but a text cell in a scanned range contributes nothing
  ([coercion and lifting](../model/02-coercion-and-lifting.md)). `N` is stricter than direct
  argument coercion and matches the range-scan outcome.

## Notes for implementers

1. **Do not implement `N` by calling the general to-number coercion.** The general primitive
   parses numeric text; `N` must not. Routing `N` through it is an easy mistake that makes
   `N("19")` return 19 and passes every test written by someone who expected that.
2. **Errors must pass through, not be caught.** `N` is not an inspection function; the error row
   in the table is a pass-through, not a classification.
3. **`Empty` and `Missing` both land on `0`,** but they arrive by different routes and should be
   handled explicitly rather than by falling through a default.
4. **The array decision is load-bearing and undocumented.** Whichever way an engine goes —
   elementwise map, or take the first element — it should be recorded as a decision awaiting
   evidence, because the documentation does not settle it.

## What has not been checked

No Handbook vector suite exists for `N`; `vectors/` publishes nothing for this function. No
Excel-comparison evidence record names `N` as a subject — **nobody has checked this function's
answers against Excel inside the Handbook's record.** The reference engine's own answers appear
on the battery panel under their own label; they are not Excel's answers.

The probes that would settle the open questions:

1. **`N({1,2;3,4})` and `N(A1:B2)`**, spilled and un-spilled. The documentation is silent on
   arrays and the reference engine maps elementwise; a single-element answer would be an equally
   defensible design, so this is genuinely unknown and is the page's biggest gap.
2. **`N` of a reference to a multi-cell range** under implicit intersection versus under dynamic
   arrays — the same question in the reference-shaped form.
3. **`N("19")` and `N(" 19 ")` as direct arguments and as cell contents.** The documented answer
   is `0` in every case; confirming it removes the most common misreading of the function.
4. **An omitted slot inside a `LAMBDA`**, to pin the `Missing` row the documentation does not
   state.
5. **A linked data type and an uncalled `LAMBDA`** (core projection `#CALC!`), to find out
   whether `N` sees the projection — in which case the lambda case would return an error, not a
   zero.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `N` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-GROUPBY-001: GROUPBY and PIVOTBY hold an array returned by their LAMBDA inside its cell; INDEX boxes it unless computed, and N(boxed) is #VALUE!

*Quirk; measured; recorded 2026-11-22.* Applies to GROUPBY, INDEX, N, PIVOTBY, TYPE.

Measured in Excel 16.0.20527 through COM (dc-h9u), when a GROUPBY or PIVOTBY callback LAMBDA
returns an array, Excel keeps that array held inside its result cell rather than flattening or
failing immediately: a cell spilling that held array shows #VALUE!, an operator over the result
is #VALUE! in a dynamic-array formula (and takes the first element in a legacy formula),
SUM/COUNT/COUNTA read across its elements, and MAX/MIN/AVERAGE/PRODUCT/SUMSQ give #N/A.
INDEX(GROUPBY(...), 1, 2) returns a boxed array that displays its first element (so *10
multiplies the first element and TYPE gives 1, while SUM/ROWS/CONCAT read the whole array);
however N(INDEX(GROUPBY(...), 1, 2)) returns #VALUE!, and omitting the column selector as
INDEX(INDEX(GROUPBY(...), 0, 2), 1) leaves the inner array held (#VALUE!). When the group
LAMBDA builds a computed array (for example LAMBDA(x, x+0), TRANSPOSE(x), or FILTER(x, x>0)),
INDEX(GROUPBY(...), 1, 2) spills that array in a dynamic-array formula. In PIVOTBY with several
value columns and one function, Total and column subtotal columns are blank; with several
functions they carry values.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=INDEX(GROUPBY({"East";"West";"East";"north";"West";"east"},{1;2;3;4;5;6},LAMBDA(x,x)),1,2)` | as typed | `1.0` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=INDEX(GROUPBY({"East";"West";"East";"north";"West";"east"},{1;2;3;4;5;6},LAMBDA(x,x)),2,2)` | as typed | `4.0` (`0x4010000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=ROWS(GROUPBY({"East";"West";"East";"north";"West";"east"},{1;2;3;4;5;6},LAMBDA(x,x)))` | as typed | `4.0` (`0x4010000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Read from Excel 16.0.20527 COM measurements recorded in DnaCalc commits
ad40ea44 (about 160 dynamic-array and legacy grouping formulas), a61f6695 (15 PIVOTBY
multi-field depth calls), d96d8cba (held_arrays explicit dynamic-formula tests), and 2,055
grouping rows in catalogue sheet dch9u_functions.

- Open: Worksheet entry, save, and unmeasured held-array call shapes for GROUPBY/PIVOTBY are tracked in dc-h9u.2.

Evidence:

- DnaCalc `crates/calc-host/tests/held_arrays.rs`, dc_h9u__a_groupby_cell_holds_its_lambdas_array; dc_h9u__n_of_a_boxed_group_is_value_error; dc_h9u__index_of_a_computed_group_spills at `d96d8cbaee` — End-to-end dynamic-array Excel 16.0.20527 expectations (gp1)
- DnaCalc `crates/calc-func/src/catalogue/grouping.rs` at `ad40ea44de` — GROUPBY/PIVOTBY boxed/held array semantics and PIVOTBY multi-value subtotals
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dch9u_functions.capture.json`, rows 585 to 630 at `c9f9d35416` — Captured Excel 16.0.20527.20088 catalogue observations
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dch9u_functions.capture.json`, sheet dch9u_functions row 618 column 4 at `c9f9d35416` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dch9u_functions.capture.json`, sheet dch9u_functions row 620 column 4 at `c9f9d35416` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dch9u_functions.capture.json`, sheet dch9u_functions row 615 column 4 at `c9f9d35416` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| total projection | A mapping defined for every input kind, with no failure case |
| silent zero | The `0` returned for text and empties; no error, no signal |
| serial number | The number a date already is; the date row of the table is not a conversion |
| pass-through | An error argument returned unchanged rather than classified |

## Sources

- Microsoft, "N function" —
  <https://support.microsoft.com/en-us/office/n-function-a624cad1-3635-4208-b54a-29733d1278c9>.
  Read for this page: the syntax, the `value` argument description, the full conversion table
  verbatim, and both remarks (the compatibility statement and the date-serial explanation).
- Handbook, [the value universe](../model/01-value-universe.md) — dates as formatted numbers,
  the `Empty`/`Missing` distinction, rich-value core projections.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — the to-number
  primitive `N` deliberately does not use, and the direct-argument versus range-scan asymmetry.
- Handbook, [the call pipeline](../model/03-call-pipeline.md) — values-only preparation and the
  admission boundary.
- `data/functions/FUNC.N.json` — identity (`xlfN`, code 131), the published signature `N(value)`,
  arity, declared axes, as projected at OxFunc `473efa3`.
