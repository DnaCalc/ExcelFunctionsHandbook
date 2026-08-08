---
schema: efh.function-page/v1
function_id: FUNC.ISNA
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
role_in_family: The single-code test — TRUE for exactly one member of the Error kind.
---

## What it computes

`ISNA(value)` returns `TRUE` when the value it receives has kind `Error` and its code is
`#N/A`, and `FALSE` otherwise.

Microsoft states the condition as "Value refers to the #N/A (value not available) error value."
It is the narrowest predicate in the IS family: one kind, one code.

`#N/A` is not an ordinary failure. It is Excel's conventional answer for *a lookup that found
nothing* and, under array broadcasting, for *a coordinate that exists in the result shape but
not in this argument* ([the call pipeline](../model/03-call-pipeline.md)). `ISNA` is therefore
usually asked as a business question — "did this key match?" — rather than as a diagnostic one.

## Arguments

`value` — required, exactly one.

Not converted; Microsoft's IS-functions remark applies ("The value arguments of the IS
functions are not converted"). A cell holding the *text* `"#N/A"` is `Text`, not `Error`, and
`ISNA` returns `FALSE` for it. This is a real failure mode when error values have been round
tripped through a CSV, a paste-as-values, or a `TEXT` call: the workbook looks full of `#N/A`
and every `ISNA` says `FALSE`.

The argument is a values position: references are resolved before the function runs
(`ArgPreparationProfile::ValuesOnlyPreAdapter`).

## Result and edge cases

Returns a `Logical`, or an array of `Logical` when the argument is an array.

- **Every other error code is `FALSE`** — `#VALUE!`, `#DIV/0!`, `#REF!`, `#NAME?`, `#NUM!`,
  `#NULL!`, and the extended codes.
- **Errors do not propagate.** `ISNA` is one of the declared per-family exceptions to the
  propagation discipline in [coercion and lifting](../model/02-coercion-and-lifting.md).
- **Arrays are classified elementwise**, producing a same-shaped mask. This is `ISNA`'s most
  common modern use: `ISNA(XMATCH(keys, table))` gives a per-key "not found" mask that
  `SUM`/`FILTER`/`BYROW` can consume, and the reference engine carries a regression test for
  exactly that reduction-mask shape.
- **`Empty` is `FALSE`.** An empty cell is not `#N/A`, even though both often mean "no data".
- **Two sources of `#N/A`.** The value produced by `NA()`, the value produced by a failed
  lookup, and the value produced positionally by broadcasting are — as far as the value model
  is concerned — the same value. The Handbook has not separately confirmed that Excel treats
  all three identically under `ISNA`, but the model gives no mechanism by which they could
  differ.

## Errors

`ISNA` returns no error of its own for any value it can be given. The only failure available to
it is **arity**: zero arguments, or two. `ISNA()` is expected to be refused at formula entry
rather than evaluated ([the call pipeline](../model/03-call-pipeline.md)); the reference
engine, having no entry-time surface, reports `#VALUE!` for both the too-few and the too-many
case.

Microsoft's IS-functions page documents no error return for `ISNA`.

## Relationships

- **`ISERR`** is `ISNA`'s exact complement inside the `Error` kind: `ISERROR = ISERR OR ISNA`,
  and the two are disjoint. See [`FUNC.ISERR`](FUNC.ISERR.md) for the full contrast table.
- **`IFNA(value, value_if_na)`** supersedes the `IF(ISNA(x), y, x)` idiom for the common case:
  it evaluates `x` once and branches only on `#N/A`, leaving other errors to surface. Where
  `IFNA` fits, prefer it; `ISNA` remains the right tool when you need the boolean itself — as a
  mask, a count, or a conditional-format condition.
- **`IFERROR`** is the wrong pair for `ISNA`: it catches `#N/A` *and* everything else, which is
  the mistake `IFNA` was added to Excel 2013 to fix.
- **`NA()`** manufactures the value this predicate tests for. `ISNA(NA())` is `TRUE` and is the
  cheapest smoke test of both functions at once.
- **`ERROR.TYPE`** returns `7` for `#N/A`; `ERROR.TYPE(v) = 7` is an equivalent condition on
  error input but errors rather than returning `FALSE` on non-error input.
- **`VLOOKUP` / `XLOOKUP` / `MATCH` / `XMATCH`** are where `#N/A` comes from in practice.
  `XLOOKUP`'s `if_not_found` argument removes the need for `ISNA` in the simple case; `ISNA`
  survives for the array and diagnostic cases.

## Notes for implementers

1. **Compare the code, not the rendering.** Error display text is localized on some surfaces;
   the code is the identity.
2. **Keep `ISNA`'s array path and `ISERROR`'s array path in the same test.** They are the two
   predicates in this family that the reference engine classifies with a dedicated array walk
   rather than the shared lift path, so their shapes need independent evidence.
3. **A positional `#N/A` from broadcasting must be a real `#N/A`,** not a sentinel. If an engine
   invents a distinct "missing coordinate" marker, `ISNA` will disagree with Excel on every
   ragged broadcast.
4. **`ISNA` is a mask generator, so its array result shape is load-bearing.** A 1x1 collapse or
   an implicit intersection in the wrong place changes downstream `SUM` totals silently.

## What has not been checked

No Handbook vector suite exists for `ISNA`; `vectors/` publishes nothing for this function. No
Excel-comparison evidence record names `ISNA` as a subject — **nobody has checked this
function's answers against Excel inside the Handbook's record.** The reference engine's own
answers appear on the battery panel under their own label; they are not Excel's answers.

The probes that would settle the open questions:

1. **The three provenances of `#N/A`** — `NA()`, a failed `MATCH`, and a broadcast coordinate
   with no source element — read through `ISNA` in one workbook, to confirm they are one value.
2. **`ISNA` over a spilled lookup result** versus over the same result captured with
   `Ctrl+Shift+Enter`, cell by cell, to pin the array shape against Excel rather than against
   the reference engine's design.
3. **Text that spells an error.** `ISNA("#N/A")` and `ISNA(A1)` where `A1` holds the string
   `#N/A` — expected `FALSE` in both cases, and worth having on record because the failure mode
   is so common in imported data.
4. **`#GETTING_DATA` and the extended codes** — expected `FALSE`, unmeasured.
5. **Arity at entry** — whether Excel refuses `=ISNA()` at entry or evaluates it.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| `#N/A` | The "value not available" error code; the sole `TRUE` case for `ISNA` |
| mask | The same-shaped array of `Logical` produced from an array argument |
| positional `#N/A` | The `#N/A` broadcasting supplies for a coordinate absent from an argument |
| provenance | Where a given `#N/A` came from; the value model says it does not change identity |

## Sources

- Microsoft, "IS functions" —
  <https://support.microsoft.com/en-us/office/is-functions-0f2d7971-6019-40a0-a171-f2d869135665>.
  Read for this page: the `ISNA` row and the non-conversion remark.
- Microsoft, "ERROR.TYPE function" —
  <https://support.microsoft.com/en-us/office/error-type-function-10958677-7c8d-44f7-ae77-b9a9ee6eefaa>.
  Read for the code table row assigning 7 to `#N/A`.
- Handbook, [the value universe](../model/01-value-universe.md) — the `Error` kind and the
  place of `#N/A` in it.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — the propagation
  discipline and the inspection functions' declared exemption.
- Handbook, [the call pipeline](../model/03-call-pipeline.md) — broadcasting's positional
  `#N/A`.
- `data/functions/FUNC.ISNA.json` — identity (`xlfIsna`, code 2), arity, declared axes, as
  projected at OxFunc `473efa3`.
