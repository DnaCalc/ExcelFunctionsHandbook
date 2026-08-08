---
schema: efh.function-page/v1
function_id: FUNC.ISREF
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
  - Page vocabulary
  - Sources
family: is_predicates_family
role_in_family: The reference-aware member — the only sibling whose argument is not dereferenced
  before it runs.
---

## What it computes

`ISREF(value)` returns `TRUE` when the argument it receives is a **reference** — a designator
for one or more cell ranges — and `FALSE` when it is any other kind of value.

Microsoft's condition: "Value refers to a reference."

This is the one member of the IS family that is not a test on a *value's* kind at all. Every
sibling receives values: references are resolved to their contents before the function runs. If
`ISREF` worked that way it would be useless, because it could never see a reference. So `ISREF`
is declared **reference-aware** (`ArgPreparationProfile::RefsVisibleInAdapter`, the deviation
described in [the call pipeline](../model/03-call-pipeline.md)): the live reference survives
argument preparation and arrives at the function intact, and `ISREF` reports whether it is one.

The kinds it answers `TRUE` for are the reference shapes listed in
[the value universe](../model/01-value-universe.md): single cell (`A1`), area (`A1:B10`),
multi-area union, three-dimensional (`Sheet1:Sheet3!A1`), structured (`Table1[Amount]`) and
spill anchor (`B1#`). Whether every one of those shapes actually answers `TRUE` in Excel is
listed below as unmeasured.

## Arguments

`value` — required, exactly one.

The argument is **not** a values position. It is the only IS-family argument that is not, and
that is the entire design of the function. Two consequences follow:

1. **What you write in the call matters, not what it evaluates to.** `ISREF(A1)` asks about the
   expression `A1`, not about the number sitting in `A1`. `ISREF(1)` is `FALSE` because the
   literal `1` is not a reference, regardless of what `A1` contains.
2. **Reference-producing functions can be tested.** `ISREF(INDIRECT("A1"))` and
   `ISREF(OFFSET(A1,1,1))` ask whether those calls produced a usable reference, and that is the
   idiom the function exists for.

The classic use is validating a sheet name before building a reference from text:
`ISREF(INDIRECT("'"&name&"'!A1"))` is `TRUE` when the sheet exists and `FALSE` when it does not,
which is how a workbook checks for a sheet without a macro.

## Result and edge cases

Returns a `Logical`. There is no array lift: `ISREF` inspects one argument expression, and the
reference engine does not route it through the family's elementwise path.

- **`ISREF(#REF!)`** is the interesting case. `#REF!` is an `Error` value, not a reference — the
  reference it once designated is gone. The expected answer is therefore `FALSE`, which means
  `ISREF` is *not* a "is this reference still valid?" test in the way its name suggests; it is
  closer to one, because a broken reference degrades to an error rather than staying a
  reference. **This has not been measured here** and it is the first thing this page would probe.
- **An array literal is `FALSE`.** `{1,2;3,4}` is an `Array`, a different kind entirely.
- **A defined name** that names a range is expected `TRUE`; a defined name that names a constant
  or a formula is expected `FALSE`. Not measured here.
- **A spill anchor `B1#`** designates a range and is expected `TRUE`. Not measured here.
- **Numbers, text, logicals, errors and empties** are all `FALSE`.

## Errors

`ISREF` returns no error of its own for any argument it can be given. The only failure available
to it is **arity**: zero arguments, or two. `ISREF()` is expected to be refused at formula entry
rather than evaluated ([the call pipeline](../model/03-call-pipeline.md)); the reference engine,
having no entry-time surface, reports `#VALUE!` for both the too-few and the too-many case.

Note the asymmetry worth remembering: `ISREF` returns `FALSE` where you might expect an error,
because "not a reference" is an answer, not a failure. `INDIRECT` of a bad address returns
`#REF!`; `ISREF` of that `#REF!` returns `FALSE`. The error is produced by the inner function,
never by `ISREF`.

Microsoft's IS-functions page documents no error return for `ISREF`.

## Relationships

- **`INDIRECT`** is `ISREF`'s usual companion: `INDIRECT` builds a reference from text at
  evaluation time (`eval_time_deref`) and `ISREF` reports whether the build succeeded.
- **`OFFSET`, `INDEX` (reference form), `CHOOSE` over ranges** also produce references and can be
  tested the same way.
- **`AREAS`** counts the areas in a reference; between them, `ISREF` and `AREAS` are the two
  functions that treat a reference as an object rather than as a source of values.
- **`ROW`, `COLUMN`, `CELL`, `SHEET`, `ISFORMULA`, `FORMULATEXT`** are the other reference-aware
  functions in this Handbook's catalogue; they share `ISREF`'s argument-preparation deviation
  and none of its semantics.
- **`ISERROR`** is what you want if the real question is "did this expression break?" — `ISREF`
  answers a narrower and different question.

## Notes for implementers

1. **`ISREF` cannot be implemented on a values-only pipeline.** If the dispatch layer resolves
   references before calling, `ISREF` will return `FALSE` for everything, silently. This is the
   function that proves an engine has a reference-aware argument path at all.
2. **Do not dereference inside the function either.** `ISREF` must not touch the cells the
   reference designates; it inspects the designator. Dereferencing turns a cheap test into a
   potentially expensive one and can raise errors that `ISREF` should never produce.
3. **Every reference shape needs an explicit decision.** Single cell, area, multi-area, 3-D,
   structured, spill anchor — six shapes, and an engine that models only the first two will
   quietly disagree with Excel on the rest.
4. **No array lift.** `ISREF` is a single-argument inspection, not an elementwise classifier.
   Routing it through the family's lift helper would change its behaviour on array arguments.

## What has not been checked

No Handbook vector suite exists for `ISREF`; `vectors/` publishes nothing for this function. No
Excel-comparison evidence record names `ISREF` as a subject — **nobody has checked this
function's answers against Excel inside the Handbook's record.** The reference engine's own
answers appear on the battery panel under their own label; they are not Excel's answers, and
note that the battery's twelve standard inputs are all values, so it exercises only the `FALSE`
side of this function.

The probes that would settle the open questions, in priority order:

1. **`ISREF(#REF!)`** — produce a genuine `#REF!` by deleting a referenced row, then test. The
   page predicts `FALSE`; a `TRUE` would be a significant finding about how Excel classifies a
   broken reference.
2. **Each of the six reference shapes**: `A1`, `A1:B10`, `(A1:A2,C1:C2)`, `Sheet1:Sheet3!A1`,
   `Table1[Amount]`, `B1#`. This is six cells and would turn the shape table above from a model
   statement into an observation.
3. **Defined names** of three sorts — a range name, a constant name, a formula name — since the
   name resolution step happens before the function and could go either way.
4. **`ISREF(INDIRECT(...))` for a missing sheet**, the function's headline use, which the
   Handbook currently states from the idiom's reputation rather than from measurement.
5. **An array argument.** Whether Excel lifts `ISREF` over `{...}` or answers once. The reference
   engine answers once; that is a design choice with no evidence behind it.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| reference-aware | Declared to receive live references rather than resolved values |
| reference shape | One of single cell, area, multi-area, 3-D, structured, spill anchor |
| designator | What a reference is: an address, not the values at that address |
| `eval_time_deref` | The pattern where a reference is built and resolved during evaluation |

## Sources

- Microsoft, "IS functions" —
  <https://support.microsoft.com/en-us/office/is-functions-0f2d7971-6019-40a0-a171-f2d869135665>.
  Read for this page: the `ISREF` row and the non-conversion remark.
- Handbook, [the value universe](../model/01-value-universe.md) — the `Reference` kind and its
  six shapes.
- Handbook, [the call pipeline](../model/03-call-pipeline.md) — `ArgPreparationProfile` and the
  reference-aware deviation, with `ISREF`'s siblings in that class.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — reference resolution
  as an explicit step preceding conversion.
- `data/functions/FUNC.ISREF.json` — identity (`xlfIsref`, code 105), arity, and the declared
  `RefsVisibleInAdapter` preparation profile, as projected at OxFunc `473efa3`.
