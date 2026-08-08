---
schema: efh.function-page/v1
function_id: FUNC.T
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
family: t_fn
role_in_family: Sole member — the text-kind filter, implemented in its own module.
---

# T

## What it computes

`T(value)` is a **type filter**, not a converter. Microsoft's rule:

> If `value` is or refers to text, `T` returns `value`. If `value` does not refer to text, `T`
> returns `""` (empty text).

Read that twice, because the function's name and its one-line catalogue description
("Converts its arguments to text") both point away from what it does. `T` does not render a
number as text. `T(42)` is **not** `"42"` — it is the empty string. A number is not text, so
the filter rejects it.

Stated as a rule over the value universe:

| `value` is | `T(value)` |
|---|---|
| Text | that same text, unchanged |
| Number | `""` |
| Logical | `""` |
| Error | that error — errors are values and propagate |
| Empty | `""` |

`T` is one of only two single-letter worksheet function names (`N` is the other), and the two
are a matched pair: `T` passes text through and blanks everything else, `N` passes numbers
through and zeroes everything else. Both exist for the same reason, which Microsoft states
plainly: they are provided for compatibility with other spreadsheet programs, and you do not
generally need them, because Excel converts values as necessary on its own.

A consequence worth naming: `T` is the shortest way to ask "is this cell text?" without
`ISTEXT` — `T(A1)=""` is true for every non-text value and for empty text alike, which is
exactly why it is not a substitute for `ISTEXT`.

## Arguments

| Argument | Meaning |
|---|---|
| `value` | The value to test. Required, and the only argument. |

There is no second argument and no option. Microsoft's phrasing — "is or **refers to** text" —
is written in reference language, but the Handbook's projected classification for this entry
records the ordinary values-only argument preparation, meaning references are resolved to
values before the function runs, as described in
[The call pipeline](../model/03-call-pipeline.md). On that reading "refers to" is documentation
prose about the common case of a cell reference, not a declaration that `T` inspects references
itself. The presence projection for this entry does name an open upstream defect stream about a
reference wrapper dropping capabilities (`BUG-FUNC-036`), so the reference path here is not
entirely settled.

## Result and edge cases

Returns `Text`, or propagates an `Error`.

The cases that matter:

- **Numbers, including dates and times.** A date is a number wearing a format
  ([The value universe](../model/01-value-universe.md)), so `T` on a date cell is `""`. The
  formatted display never reaches the function.
- **Logicals.** `TRUE` is not text; `T(TRUE)` is `""`.
- **Empty cells.** `""`.
- **Numeric-looking text.** `T("42")` is `"42"`. The value is text; that it reads as a number
  is irrelevant, because `T` filters by kind and not by content. This is the mirror image of
  the `SUM("3")` versus `SUM(A1)` asymmetry described in
  [Coercion and lifting](../model/02-coercion-and-lifting.md), and it is the cleanest available
  demonstration that Excel's kind distinction is real rather than cosmetic.
- **Errors.** An error value propagates rather than becoming `""`. `T` does not swallow errors;
  it is not a guard.

Array arguments are the one behaviour this page cannot pin. Elementwise filtering is what the
shared lifting model would predict for a scalar-shaped kernel, and the Handbook's projected
classification marks the coercion/lift profile of this entry as custom rather than as one of
the standard scalar categories — so prediction is all it is.

## Errors

Microsoft's `T` page documents no error conditions of its own, and the function has no domain
to violate: every value kind has a defined answer. The only errors reachable here are incoming
ones, which propagate under the shared rule in
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Relationships

- **`N`** — the numeric mirror. `N(value)` returns the number if `value` is a number, 1 for
  `TRUE`, a serial for a date, and 0 for text. Same compatibility origin, same single-letter
  name, opposite filter. If you understand one you understand the other.
- **[TEXT](FUNC.TEXT.md)** — the function readers reach for when they meant to render a number
  as text. This is the confusion `T` generates most: `TEXT(42, "0")` is `"42"`; `T(42)` is
  `""`. They are not related and are not alternatives.
- **`ISTEXT`** — the proper type predicate, returning a logical rather than a value. `ISTEXT`
  distinguishes empty text from a non-text value; `T` does not.
- **`VALUE` and [NUMBERVALUE](FUNC.NUMBERVALUE.md)** — the text-to-number direction.
- `T` is **not** a Compatibility-category function and is not superseded by anything. Its
  legacy character is about its *origin* — cross-program compatibility with older spreadsheet
  products — not about its status. Microsoft continues to document it as a current Text
  function.

## Notes for implementers

The whole function is a match on the value kind. The only ways to get it wrong are to convert
instead of filtering (returning `"42"` for `T(42)`), or to absorb errors instead of propagating
them.

The Empty case is worth an explicit branch rather than a fall-through: `T` on an empty cell and
`T` on empty text produce the same visible answer for different reasons, and the shared model
keeps Empty distinct from text precisely so functions can treat them distinctly.

`T` is a good regression canary for a value-kind implementation. Because it returns the input
unchanged for exactly one kind and a constant for every other, a `T` disagreement is almost
always a value-kind bug somewhere upstream rather than a bug in `T`.

## What has not been checked

No Handbook vector suite exists for `T`, and no Excel-comparison evidence record names it.
Nobody has checked this function against Excel within the Handbook's record. The filter rule
above is quoted from Microsoft's page; the per-kind table is that rule applied to the
Handbook's value universe, not a set of observations.

Inputs worth probing first:

1. **`T(42)` and `T("42")` side by side** — one cell each, and together they prove or refute
   the entire premise of the page. If `T(42)` is anything but `""`, everything here is wrong.
2. **`T(TRUE)` and `T(TODAY())`** — the logical and the date, the two kinds users most expect
   to be "text-like".
3. **`T(#N/A)`** — error propagation versus absorption. A function whose job is to return `""`
   for everything unrecognized is exactly the kind of function that might swallow an error, and
   the documented rule does not address errors at all.
4. **`T(A1)` with `A1` empty**, and the same with `A1` holding `""` — whether Empty and empty
   text are distinguishable through `T` at all.
5. **`T({1,"a";TRUE,""})`** — the array case, which the classification leaves open.
6. **`T` over a rich value or a linked data type**, whose core projection is text
   ([The value universe](../model/01-value-universe.md)). Whether `T` sees the projection or
   the rich value is undetermined and is the most interesting modern question about this
   otherwise ancient function.
7. **`T(OFFSET(...))` and other reference-producing arguments**, given the open `BUG-FUNC-036`
   stream on the reference path in this module.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| type filter | Returns the input for one value kind and a constant for all others |
| value kind | The eight-member classification defined in the value-universe chapter |
| core projection | The plain-gamut value a rich value presents to legacy surfaces |

## Sources

- Microsoft, "T function" —
  <https://support.microsoft.com/en-us/office/t-function-fb83aeec-45e7-4924-af95-53e073541228>
  (the is-or-refers-to-text rule, the empty-text result, and the statement that the function
  exists for compatibility with other spreadsheet programs).
- Handbook, [The value universe](../model/01-value-universe.md) — the value kinds, dates as
  formatted numbers, Empty versus Missing, and rich-value core projections.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — error propagation and
  the direct-argument versus range-scan kind asymmetry.
- Handbook, [The call pipeline](../model/03-call-pipeline.md) — values-only argument
  preparation.
- Handbook projections `data/functions/FUNC.T.json` (classification: values-only preparation,
  custom coercion/lift profile) and `data/presence/FUNC.T.json` (sole-member implementing
  module; `BUG-FUNC-036`).
