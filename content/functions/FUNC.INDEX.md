---
schema: efh.function-page/v1
function_id: FUNC.INDEX
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - The two forms
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
family: index
role_in_family: >-
  Retrieves a value or a reference at given coordinates, in two distinct forms; the catalog's
  canonical reference-returning function and the retrieval half of the INDEX/MATCH pair.
---

## What it computes

`INDEX` retrieves what sits at given coordinates. Stated that plainly it sounds like array
subscripting, and for the simple call it is: `INDEX(a, 2, 3)` is the element in the second row
and third column of `a`.

Two things lift it above subscripting, and between them they account for nearly everything
`INDEX` is used for:

1. **A zero coordinate means "the whole axis".** `INDEX(a, 0, 3)` is the entire third column;
   `INDEX(a, 2, 0)` is the entire second row. Zero is not an out-of-range index, it is a
   quantifier.
2. **Given a reference, `INDEX` returns a reference** — not the reference's value. That is why
   `INDEX` can appear as an endpoint of the range operator, be handed to functions that require
   a range, and serve as a non-volatile substitute for `OFFSET`.

Both properties are consequences of the same underlying fact, recorded in the projection as
`arg_preparation_profile: RefsVisibleInAdapter`: the live reference survives into the function,
and the function decides what to hand back.

## The two forms

Microsoft's documentation presents `INDEX` as two functions sharing a name.

### Array form — `INDEX(array, row_num, [column_num])`

Operates on an array value and returns a value (or, with a zero coordinate, an array). This is
the form you get when the first argument is an array constant, an array-valued expression, or a
range in a context where the result is consumed as a value.

Microsoft documents a convenience for one-dimensional inputs: if `array` has only one row or
only one column, the corresponding index argument is optional. So for a column `a`,
`INDEX(a, 5)` means the fifth element — the missing `column_num` is not defaulted to 1 by
accident, it is defined away because there is only one column.

### Reference form — `INDEX(reference, row_num, [column_num], [area_num])`

Operates on a reference and returns a reference. It gains a fourth argument, `area_num`, which
selects which rectangle of a multi-area reference to index into; Microsoft documents the default
as area 1. This is the form that composes with the reference algebra.

The two forms are not selected by a mode flag. Which one applies follows from what the first
argument is and from what the calling context does with the result — which is exactly why
`INDEX` feels different in different formulas, and why the reference-returning behaviour
surprises people who have only used the array form.

The practical consequences of the reference form are worth listing explicitly, because they are
the reason experienced modellers reach for `INDEX`:

- **As a range endpoint.** `SUM(A1:INDEX(A:A, n))` sums from `A1` to a computed last row. The
  range operator needs references on both sides; because `INDEX` supplies one, this works, and
  it recomputes only when its inputs change.
- **As a non-volatile `OFFSET`.** `OFFSET` is volatile: it recalculates on every recalculation
  of the workbook. `INDEX` is recorded `volatility_class: NonVolatile`. For dynamic-range
  definitions this is the difference between a workbook that recalculates lazily and one that
  does not.
- **As a target for functions that require a range**, such as those that need a reference rather
  than its contents.

`INDEX` is sometimes described as usable "on the left of an assignment", which is a loose way of
saying that it designates a location rather than a value. The worksheet has no assignment, so
the accurate statement is the one above: it returns a reference, and references are what
location-consuming constructs need.

## Arguments

| Argument | Meaning |
|---|---|
| `array` / `reference` | The data to index into. An array value in the array form; a reference — possibly multi-area — in the reference form. Required. |
| `row_num` | Which row, counting from 1 within the array or area. Zero means the whole column axis. Required by arity, though Microsoft documents it as omissible for one-dimensional inputs. |
| `column_num` | Which column, counting from 1. Zero means the whole row axis. Optional. |
| `area_num` | Reference form only: which area of a multi-area reference. Optional; documented default 1. |

Positions that are commonly misunderstood:

- **Coordinates are relative to the array or area, not to the worksheet.** `INDEX(C5:E9, 1, 1)`
  is `C5`, not `A1`.
- **Zero versus omitted.** These are different: zero selects a whole axis; omitted (Missing)
  invokes the one-dimensional convenience or the documented default. The value model keeps
  Missing distinct from every other "nothing"
  ([value universe](../model/01-value-universe.md)), and `INDEX` is one of the functions where
  the distinction is load-bearing rather than pedantic.
- **`area_num` exists only in the reference form**, and a multi-area reference passed inline
  needs the double-parenthesis grammar described on the `AREAS` page, because the comma would
  otherwise separate arguments.

## Result and edge cases

Return kind: a scalar, an `Array`, or a `Reference`, depending on form and on whether a
coordinate is zero.

- **Both coordinates zero, reference form** — the whole reference (or the whole selected area).
- **One coordinate zero** — a whole row or column. In the array form this is an array that
  spills in modern Excel and required array entry historically; in the reference form it is a
  reference to that row or column, which is the version that composes with `SUM` and the range
  operator.
- **Single-cell result in the reference form.** It is still a reference; the surrounding
  expression decides whether it is dereferenced. A bare `INDEX` in a cell displays the value.
- **Blank located cell.** Empty is admitted at the raw-return boundary but not at the published
  one, so an `INDEX` onto an empty cell publishes as numeric zero rather than as blank — the
  general normalization described in [the value universe](../model/01-value-universe.md), not
  something `INDEX` decides.
- **Error in the located cell** is simply the result.
- **Array-valued coordinates.** The reference implementation's kernel carries both a scalar
  selector and an array selector, so a vector of row numbers is a shape its structure
  anticipates. What Excel returns for `INDEX(a, {1;3})` — a spilled vector, an implicit
  intersection, or an error — is not established here, and it differs between the pre- and
  post-dynamic-array eras.
- **Implicit intersection.** In pre-dynamic-array Excel, an `INDEX` result that was an array or
  multi-cell reference in a scalar context underwent implicit intersection; modern Excel spills
  instead unless `@` is written. Formulas migrated from older workbooks carry the `@` marker for
  exactly this reason, and the difference is a compatibility question rather than a semantic
  one.

## Errors

Microsoft's page documents:

- `#REF!` when `row_num`, `column_num` or `area_num` is out of range for the given array or
  reference — including an `area_num` larger than the number of areas present.
- `#VALUE!` for arguments that are not usable as indices.

The split follows the same logic as `HLOOKUP`'s: a coordinate that points outside the data is a
reference failure; a coordinate that is not a usable number is a value failure. The reference
implementation's error enumeration mirrors this shape, carrying distinct out-of-bounds and
invalid-index cases.

An error value supplied as an argument propagates under the universal rule in
[coercion and lifting](../model/02-coercion-and-lifting.md).

## Relationships

- **`MATCH`** is the other half of the `INDEX`/`MATCH` idiom: `MATCH` computes a position,
  `INDEX` retrieves it. The pair replaces `VLOOKUP`/`HLOOKUP` and removes their two structural
  limits — the key column need not be first, and the result index need not be counted by hand.
- **`XLOOKUP`** subsumes the common `INDEX`/`MATCH` case in one call. `INDEX`/`MATCH` remains
  the more flexible construction, particularly for two-dimensional lookups
  (`INDEX(a, MATCH(...), MATCH(...))`) and for anything needing a reference result.
- **`OFFSET`** also computes a reference and is volatile; `INDEX` is not. Where an `OFFSET`
  formula can be rewritten with `INDEX`, it usually should be.
- **`AREAS`** counts the areas that `area_num` selects among; `INDEX(ref, r, c, AREAS(ref))` is
  the "last area" idiom.
- **`CHOOSEROWS` / `CHOOSECOLS`** are the modern, array-returning way to do the zero-coordinate
  trick for several rows or columns at once — but they return values, not references, so they
  do not substitute for the reference form.
- **`INDIRECT`** is the other route to a computed reference, via text. It is volatile and
  invisible to the dependency graph; `INDEX` is neither. When both would work, `INDEX` is the
  better-behaved choice.
- **The range operator** (`FUNC.OP_RANGE_REF`, `:`) is `INDEX`'s most important collaborator in
  the reference form.

## Notes for implementers

- **The reference must not be dereferenced early.** This is the whole function. An
  implementation that resolves `reference` to values on entry can still return correct scalars
  and will fail every composition — range endpoints, `SUM` over a computed column, dynamic named
  ranges. The correctness of `INDEX` is not visible from its scalar results.
- **Zero is a distinct branch, in each coordinate independently.** Four combinations
  (both nonzero, row zero, column zero, both zero), and the both-zero case is meaningful only in
  the reference form.
- **Missing is a third state alongside zero and a positive index.** The one-dimensional
  convenience means an implementation must know whether the argument was omitted, not merely
  what its value was — which is exactly the Missing-versus-Empty distinction the shared model
  insists on carrying to the call boundary.
- **Index coercion has a documented shape in the reference implementation**: the index is
  coerced to a number and rejected unless it is finite, non-negative and integral. Non-negative
  rather than positive, because zero is legal — an implementation that validates `> 0` breaks
  the whole-axis form.
- **`area_num` selection precedes coordinate bounds checking**, since the bounds depend on which
  area was selected. Doing it in the other order range-checks against the wrong rectangle.
- **The array-selector path** in the reference implementation (a selector that may be a scalar
  or an array) is the seam where dynamic-array behaviour enters. It is also the least
  established part of this page.

## What has not been checked

No Handbook vector suite exists for `INDEX`, and no Excel-comparison evidence record is recorded
for it. Nobody has checked this function's behaviour against Excel here. The reference
implementation carries the structure described above, but structure is not evidence about Excel.

The probes that would settle the most, in order of how much they would change this page:

1. **Reference-ness, tested through composition rather than through results.** Three calls that
   a value-returning implementation cannot satisfy: `SUM(A1:INDEX(A1:A10, 5))`,
   `SUM(INDEX(A1:C10, 0, 2))`, and `ROW(INDEX(A1:C10, 2, 2))`. If these behave, the reference
   form is real; if `ROW` reports the located cell's row, it is real and composable. Nothing
   else on this page matters as much.
2. **The array/reference form boundary.** The same call in a value context and a reference
   context, to establish what actually selects the form — the argument's kind, the argument
   count, or the consuming context.
3. **Array-valued `row_num` / `column_num`** — `INDEX(a, {1;3})` and `INDEX(a, SEQUENCE(3))` —
   in modern Excel. This is the largest genuine unknown, and its answer decides whether `INDEX`
   is a dynamic-array function or a scalar one with a spilling special case.
4. **Zero coordinates in both forms**, including both-zero, and in the array form where there is
   no reference to return.
5. **Multi-area references**: `area_num` of 1, of the area count, of one beyond, and of zero,
   plus a defined name bound to a union to separate parse-time from value-time area structure.
6. **The one-dimensional convenience**: `INDEX(column, 5)` and `INDEX(row, 5)`, confirming that
   the omitted argument is defined away rather than defaulted, and `INDEX(column, 5, 1)` for the
   explicit form.
7. **Index domain edges**: negative, fractional (either side of an integer, to pin rounding or
   truncation), text that reads as a number, a logical, and an error value.

Items 1 and 3 are the two whose answers would rewrite sections above rather than extend them.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| array form | `INDEX(array, row_num, [column_num])`; operates on and returns values |
| reference form | `INDEX(reference, row_num, [column_num], [area_num])`; returns a reference |
| whole-axis index | A zero coordinate, selecting an entire row or column |
| area | One rectangle of a multi-area reference, selected by `area_num` |
| implicit intersection | Pre-dynamic-array reduction of a multi-cell result to one cell; written `@` in modern Excel |
| non-volatile | Recalculates only when its inputs change, unlike `OFFSET` and `INDIRECT` |

## Sources

- Microsoft, INDEX function —
  <https://support.microsoft.com/en-us/office/index-function-a5dcf0dd-996d-40a4-a822-b56b061328bd>
  (the array and reference forms, the zero-coordinate whole-row/whole-column rule, the
  one-dimensional omissible-argument convenience, `area_num` and its default, and the `#REF!`
  and `#VALUE!` conditions).
- Handbook `content/model/01-value-universe.md` (reference kind and shapes; Missing versus
  Empty; the raw-return versus published-result boundary).
- Handbook `content/model/02-coercion-and-lifting.md` (error propagation; reference resolution
  as an explicit step).
- Handbook `content/model/03-call-pipeline.md` (`RefsVisibleInAdapter`; reference-sensitive
  functions returning references; operators as functions, including the range operator).
- Handbook `data/functions/FUNC.INDEX.json` and `data/presence/FUNC.INDEX.json` (signature,
  arity 2–4, classification axes, implementing module).
- OxFunc `crates/oxfunc_core/src/functions/index.rs` at commit `473efa3` — read for the shape of
  its index coercion (finite, non-negative, integral) and its scalar-or-array selector
  structure. Read as reference-implementation structure, not as evidence about Excel.
