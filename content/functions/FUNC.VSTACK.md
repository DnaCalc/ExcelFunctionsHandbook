---
schema: efh.function-page/v1
function_id: FUNC.VSTACK
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
family: dynamic_array_reshape_family
role_in_family: >-
  Concatenates arrays vertically, padding narrow arrays with #N/A to the widest one — the
  row-wise half of the stacking pair.
---

## What it computes

`VSTACK` concatenates its array arguments **vertically**, in argument order.

Given arrays `A₁ … A_p` with dimensions `m₁ × n₁ … m_p × n_p`, the result has

- **rows** `m₁ + m₂ + … + m_p` — the sum,
- **columns** `max(n₁, …, n_p)` — the maximum,

with `A_k`'s rows appearing in order, starting at result row `m₁ + … + m_{k-1} + 1`.

The interesting part is the second dimension. Stacking arrays of unequal width is not an error
and not a truncation: **the narrow arrays are padded on the right with `#N/A`**. That choice is
worth pausing on. Excel could have refused, or padded with blanks, or padded with zeros. It
pads with an error value, which means the padding is *self-announcing* — it shows up in the
result rather than blending into it, and it propagates through arithmetic instead of quietly
contributing `0`.

The practical consequence is the idiom Microsoft itself recommends: wrap the call in `IFERROR`
to substitute your own filler. That works, and it also silently swallows any genuine `#N/A`
that came from the data. Whether that trade is acceptable is the caller's decision, and it is
worth making consciously.

## Arguments

Microsoft documents `VSTACK(array1, [array2], …)`. Every argument is an array or range; the
first is required and the rest are optional and repeating. The projected arity records a
maximum of 255 arguments.

There are no option flags. `VSTACK` has exactly one behaviour, which is unusual in this family
and makes it easy to reason about.

The position most often misunderstood is not an argument at all but the *shape contract*: many
readers expect `VSTACK` to require equal widths, discover that it does not, and only later find
the `#N/A` padding in the corner of a wide result. There is no argument that turns the padding
off.

## Result and edge cases

The return kind is an array, spilled.

- **Single argument.** `VSTACK(A)` returns `A`. It is a legal, if pointless, call, and it is a
  useful shape probe.
- **Scalars.** A scalar argument behaves as a 1×1 array and contributes one row.
- **The `#N/A` padding is part of the value**, not a display artifact: it participates in
  downstream arithmetic and comparisons exactly as any `#N/A` does.
- **Error values inside the inputs** are carried through unchanged. `VSTACK` moves values; it
  does not inspect them.
- **Very large results** hit the grid and the array-size limits; the documentation for adjacent
  reshaping functions names `#NUM!` for oversized arrays, and whether `VSTACK` uses the same
  outcome is not something the Handbook has verified.
- Dynamic-array publication and `#SPILL!` are host-side adaptation, described in
  [the call pipeline](../model/03-call-pipeline.md).

## Errors

The `#N/A` values in the padded region are documented as the *expected result* of stacking
unequal widths, not as a failure — an important distinction for anyone writing a test oracle:
a result containing `#N/A` is not necessarily an error case.

Microsoft's page documents no error return for `VSTACK` itself beyond that padding, and
suggests `IFERROR` as the mitigation. `#SPILL!` arises at publication when the result cannot be
placed.

## Relationships

- **`HSTACK`** is the horizontal twin: columns sum, rows take the maximum, and short arrays are
  padded **below** with `#N/A`. The two are exact transposes of each other, and
  `TRANSPOSE(VSTACK(TRANSPOSE(a), TRANSPOSE(b)))` is `HSTACK(a, b)`.
- **`TOCOL` / `TOROW`** flatten an array into one vector; `VSTACK` preserves two-dimensional
  structure. Stacking single-column arrays with `VSTACK` and flattening with `TOCOL` produce
  different things whenever the inputs are wider than one column.
- **`WRAPROWS` / `WRAPCOLS`** are the inverse direction: one vector to a rectangle.
- **`EXPAND`** pads a single array to a given size with a chosen filler, which is the explicit
  form of what `VSTACK` does implicitly.
- The array literal `{1,2;3,4}` and the `;` row separator are the static equivalents.

## Notes for implementers

- Compute the target width first, then copy; the padding must be materialized, not implied,
  because the result is a dense rectangular array.
- The pad value is `#N/A` specifically, not a generic error and not a blank. An implementation
  that pads with empty produces a result that looks right and sums differently.
- Argument order defines row order. Stacking is not commutative.
- Scalars, 1×1 arrays and references all have to reduce to the same rectangular representation
  before stacking; the reference engine's `BUG-FUNC-026` is a reminder that a 1×1 array must
  not be collapsed to a scalar on the way through.
- The result of stacking a large number of arguments is the natural place for an array-size
  limit check; the limit is a host property, not a kernel property.

## What has not been checked

No Handbook vector suite exists for `VSTACK`, and no Handbook evidence record is attached to
this page. Nothing here claims agreement with Excel for any implementation.

First probes:

1. **Unequal widths in every position** — narrow first, narrow last, narrow in the middle,
   and several different widths at once — reading the padded region back explicitly, since the
   `#N/A` padding is the function's one non-obvious behaviour.
2. **The padded region's identity**: is it exactly `#N/A`, and does `ISNA` see it as such?
3. **Genuine `#N/A` in the data** alongside padding, to confirm they are indistinguishable in
   the result (which is what makes the `IFERROR` idiom lossy).
4. **Scalar and 1×1 arguments**, and a single-argument call, read back through `ROWS`,
   `COLUMNS` and `TYPE`.
5. **Size limits**: enough rows to exceed the grid, to pin the error.
6. **Argument-count limits** at the projected maximum of 255.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `VSTACK` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-HSTACK-001: A direct error stops these array functions; the same error in an array or cell is an element

*Quirk; measured; recorded 2026-11-04.* Applies to BYCOL, BYROW, EXPAND, HSTACK, MAP, TAKE, TOCOL, TOROW, VSTACK.

An error written directly as an argument of these functions is the answer for the whole call.
It is not placed in the result: HSTACK(1,#VALUE!) is #VALUE!, not a row holding 1 and an error,
and TAKE(#N/A,0) is #N/A. With MAP the LAMBDA does not run at all:
MAP(NA(),LAMBDA(xx,IFERROR(xx,3))) is #N/A. The same error inside an array, or in a referenced
cell, is an ordinary element: MAP({#N/A},LAMBDA(xx,3)) is 3, and HSTACK(Data!C1,{1;2}) with
#N/A in Data!C1 is a 2 by 2 array with #N/A in its first column. TOCOL and TOROW show the
difference with ignore 2: TOCOL(#N/A,2) is #N/A, while TOCOL({#N/A},2) drops the error, has
nothing left, and is #CALC!. EXPAND is an exception on the cell side: EXPAND(Data!C1,2,2,"pad")
is #N/A for the whole call. BYROW and BYCOL check the direct error first too:
BYROW(#N/A,#VALUE!) is #N/A, while BYROW({#N/A},2), where 2 is not a LAMBDA, is #VALUE!.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=HSTACK(1,#VALUE!)` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=TAKE(#N/A,0)` | as typed | `#N/A` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=MAP(NA(),LAMBDA(xx,IFERROR(xx,3)))` | as typed | `#N/A` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=MAP({#N/A},LAMBDA(xx,3))` | as typed | `3` (`0x4008000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=HSTACK(Data!C1,{1;2})` | Data!C1 `#N/A` (cells) | `{#N/A,1;#N/A,2}` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=TOCOL(#N/A,2)` | as typed | `#N/A` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=TOCOL({#N/A},2)` | as typed | `#CALC!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=EXPAND(Data!C1,2,2,"pad")` | Data!C1 `#N/A` (cells) | `#N/A` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=BYROW(#N/A,#VALUE!)` | as typed | `#N/A` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=BYROW({#N/A},2)` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): 27 recorded readings in dynamic-array Excel, each with size observers.

- Open: Competing errors in later positions have not been read in a published capture.

Evidence:

- DnaCalc `apps/dna-onecalc/tests/acceptance.rs`, scalar_errors_keep_their_array_argument_origins at `2d0eddcac8` — recorded readings pinned
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheets fn2_q001 to fn2_q007, fn2_q031 to fn2_q042, fn2_q232 to fn2_q239 at `2d0eddcac8` — recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q004, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q232, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q031, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q032, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q238, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q233, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q234, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q237, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q040, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn2_questions/oracle.capture.json`, sheet fn2_q038, row 20, column 10 at `2d0eddcac8` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| stacking | Concatenation along one axis, preserving two-dimensional structure |
| `#N/A` padding | The documented filler placed in the columns a narrow array does not reach |
| transpose twin | `HSTACK`, the same operation along the other axis |

## Sources

- Microsoft, *VSTACK function* —
  <https://support.microsoft.com/en-us/office/vstack-function-a4b86897-be0f-48fc-adca-fcc10d795a9c>
  (syntax, the sum-of-rows / max-of-columns result dimensions, the `#N/A` padding for narrower
  arrays, and the `IFERROR` mitigation). Retrieved for this page.
- Handbook `content/model/01-value-universe.md` and `content/model/03-call-pipeline.md`.
- OxFunc bug stream `docs/bugs/streams/BUG-FUNC-026_take_1x1_scalar_publication_mismatch.md` —
  the 1×1 array shape distinction relevant to every member of this family.
- Handbook `data/functions/FUNC.VSTACK.json` (signature, arity, classification axes).
