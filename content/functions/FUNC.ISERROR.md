---
schema: efh.function-page/v1
function_id: FUNC.ISERROR
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
role_in_family: The total error test — TRUE for every member of the Error kind, including #N/A.
---

## What it computes

`ISERROR(value)` returns `TRUE` when the value it receives has kind `Error`, and `FALSE` for
every other kind.

That is the whole rule, and its force is in the word *every*. The `Error` kind is a closed
registry of codes ([the value universe](../model/01-value-universe.md) lists fourteen modeled
codes); `ISERROR` does not look at which code it is. Microsoft's documented condition names
seven: "Value refers to any error value (#N/A, #VALUE!, #REF!, #DIV/0!, #NUM!, #NAME?, or
#NULL!)" — the legacy family, the seven that round-trip through every historical Excel surface.

The Handbook states the rule as membership in the `Error` kind rather than membership in that
list of seven, because Excel's modern extended codes (`#SPILL!`, `#CALC!`, `#FIELD!`,
`#BLOCKED!`, `#CONNECT!`, `#BUSY!`, `#GETTING_DATA`) exist on the worksheet and did not exist
when that documentation sentence was written. **Which of the extended codes `ISERROR` answers
`TRUE` for is not established here.** The kind-membership reading predicts `TRUE` for all of
them; the documentation's enumeration predicts nothing about them either way. That gap is real
and is listed below rather than papered over.

The one thing that is definite and is the reason both this function and `ISERR` exist:
**`ISERROR` includes `#N/A`.** `ISERR` excludes it. See "Relationships".

## Arguments

`value` — required, exactly one.

Not converted. Microsoft's IS-functions remark applies: "The value arguments of the IS
functions are not converted." Text that reads as a number stays `Text`, and text that reads as
an error name — the literal string `"#N/A"` typed into a cell as text — stays `Text` and is
therefore `FALSE`. An error *value* and the text that spells an error are different kinds, and
`ISERROR` separates them.

The argument is a values position: references are resolved before the function runs
(`ArgPreparationProfile::ValuesOnlyPreAdapter`).

## Result and edge cases

Returns a `Logical`, or an array of `Logical` when the argument is an array.

- **Errors do not propagate through this function.** `ISERROR` is one of the declared
  per-family exceptions to the propagation discipline in
  [coercion and lifting](../model/02-coercion-and-lifting.md): the error is the subject of the
  test, not a failure to forward. An error input yields `TRUE`, never the error itself.
- **Arrays are classified elementwise.** The reference engine walks an array argument and
  returns an array of the same shape, one `Logical` per element — so `ISERROR` over a lookup
  result is a mask, not a single verdict. Wrapping it in `OR` or `SUMPRODUCT` is how a
  whole-array question gets asked.
- **Empty cells are `FALSE`.** `Empty` is not an error kind.
- **Text, numbers, logicals, references** are all `FALSE`.
- **An uncalled `LAMBDA`** has `#CALC!` as its core projection
  ([the value universe](../model/01-value-universe.md)), so the projected value that reaches
  `ISERROR` is an error. Whether the engine hands `ISERROR` the projection or the rich value
  itself is not established here.

## Errors

`ISERROR` returns no error of its own for any value it can be given — that is the point of the
function. The only failure available to it is **arity**: zero arguments, or two.
`ISERROR()` is expected to be refused at formula entry rather than evaluated (see
[the call pipeline](../model/03-call-pipeline.md)); the reference engine, having no entry-time
surface, reports `#VALUE!` for both the too-few and the too-many case.

Microsoft's IS-functions page documents no error return for `ISERROR`.

## Relationships

This is the classic Excel trap, and it is worth stating in one line before anything else:

> `ISERROR` is `TRUE` for `#N/A`. `ISERR` is `FALSE` for `#N/A`. `ISNA` is `TRUE` only for
> `#N/A`.

Which means, over the `Error` kind, `ISERROR = ISERR OR ISNA`, and the three functions partition
the error space into exactly the two halves that matter in practice: *the lookup found nothing*
(`#N/A`) versus *the formula is broken* (everything else).

- **`ISERR`** — use it when a missing lookup is a legitimate outcome you want to let through
  while still catching genuine faults. Using `ISERROR` there silently swallows the "not found"
  signal, and that is how bad numbers get published.
- **`ISNA`** — the narrow test; the complement of `ISERR` within the error kind.
- **`IFERROR(x, y)`** supersedes the `IF(ISERROR(x), y, x)` idiom, evaluates `x` once, and is
  the modern form. `IFNA(x, y)` is the `#N/A`-only counterpart, paired with `ISNA` the way
  `IFERROR` is paired with `ISERROR`.
- **`ERROR.TYPE`** answers *which* error rather than *whether*; it returns a code 1–8 for an
  error input and `#N/A` for anything else.
- **`AGGREGATE`** with the appropriate option skips errors during a scan instead of testing
  them — the other declared way to mask errors named in
  [coercion and lifting](../model/02-coercion-and-lifting.md).

## Notes for implementers

1. **Do not implement `ISERROR` as a list membership test against the seven legacy codes.** A
   list will silently return `FALSE` for `#SPILL!` and `#CALC!` on a modern build. Test the
   kind. Then record, separately and honestly, whether Excel agrees — that is a measurement, not
   a design decision.
2. **The error must reach the kernel.** Any layer that short-circuits on an error input before
   dispatch will make `ISERROR` return the error instead of `TRUE`. The propagation exception
   has to be declared at the function, not hoped for.
3. **`ISERROR` and `ISNA` classify arrays elementwise while some sibling predicates route
   through a generic lift path.** They reach the same shape, but the two routes are not the same
   code, so an array-shape divergence would show up in one and not the other. Test both.
4. **Do not confuse the C-API error enumeration with the worksheet code set.** The extended
   family does not map one-to-one onto the legacy enumerations, and whether a code survives a
   boundary crossing is a per-code, per-version fact.

## What has not been checked

No Handbook vector suite exists for `ISERROR`; `vectors/` publishes nothing for this function.
No Excel-comparison evidence record names `ISERROR` as a subject — **nobody has checked this
function's answers against Excel inside the Handbook's record.** The reference engine's own
answers appear on the battery panel under their own label and are not Excel's answers.

Specifically unresolved, and the probes that would settle each:

1. **The extended error codes.** `ISERROR` applied to a deliberately produced `#SPILL!`,
   `#CALC!`, `#FIELD!`, `#BLOCKED!` and `#CONNECT!` on a build that has them. This is the
   largest open question on the page and the cheapest to answer.
2. **`#GETTING_DATA` and `#BUSY!`** — transient placeholders. Whether `ISERROR` can even
   observe one, and what it says while the value is unresolved, needs a live external-data
   probe; a transient value is hard to catch and the answer may be timing-dependent.
3. **Rich values.** `ISERROR` of a linked data type whose field lookup failed, and of an
   uncalled `LAMBDA` (core projection `#CALC!`), to find out whether the predicate sees the
   projection or the payload.
4. **Array shape.** `ISERROR` over a 2-D array containing a mix of errors and non-errors, spilled
   and un-spilled, checked cell by cell against Excel — the reference engine's elementwise result
   is a design choice until it is measured.
5. **Arity at entry.** Whether Excel refuses `=ISERROR()` at entry or evaluates it, which the
   Handbook records as an open admission-boundary question generally, not just here.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| `Error` kind | The value kind whose members are the worksheet error codes |
| legacy family | The seven codes that round-trip every historical Excel surface; the seven Microsoft's sentence names |
| extended family | Modern worksheet codes (`#SPILL!`, `#CALC!`, …) whose `ISERROR` answer is not established here |
| mask | An array of `Logical` produced by classifying an array argument elementwise |

## Sources

- Microsoft, "IS functions" —
  <https://support.microsoft.com/en-us/office/is-functions-0f2d7971-6019-40a0-a171-f2d869135665>.
  Read for this page: the `ISERROR` row naming the seven legacy codes, the `ISERR` row, and the
  remark that IS-function arguments are not converted.
- Handbook, [the value universe](../model/01-value-universe.md) — the `Error` kind, the
  fourteen modeled codes, the legacy/extended split, and the `#CALC!` core projection of an
  uncalled lambda.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — error propagation as
  engine law and the declared per-family exceptions, `ISERROR` among them.
- Handbook, [the call pipeline](../model/03-call-pipeline.md) — the admission-versus-runtime
  boundary for arity failures.
- `data/functions/FUNC.ISERROR.json` — identity (`xlfIserror`, code 3), arity, declared axes,
  as projected at OxFunc `473efa3`.
