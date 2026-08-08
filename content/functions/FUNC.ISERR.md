---
schema: efh.function-page/v1
function_id: FUNC.ISERR
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
role_in_family: The error test with #N/A carved out — the member defined by an exclusion rather
  than by a kind.
---

## What it computes

`ISERR(value)` returns `TRUE` when the value it receives has kind `Error` **and** its code is
not `#N/A`. Everything else — including `#N/A` itself — is `FALSE`.

Microsoft states it as "Value refers to any error value except #N/A." It is the only member of
the IS family whose definition is a set difference rather than a kind membership, and that
single exclusion is the whole reason the function is in the product.

The exclusion encodes a semantic distinction Excel makes everywhere else too: `#N/A` means
*the value is not available* — a legitimate, expected outcome of a lookup that found nothing —
while the other error codes mean *this formula is wrong*. `ISERR` is the predicate that asks
"is something broken?" without treating a legitimately absent value as breakage.

Formally, over the `Error` kind:

```
ISERR(v)   = ISERROR(v) AND NOT ISNA(v)
ISERROR(v) = ISERR(v) OR ISNA(v)
```

and `ISERR`, `ISNA` partition the error kind into two disjoint halves.

## Arguments

`value` — required, exactly one.

Not converted. Microsoft's IS-functions remark is explicit: "The value arguments of the IS
functions are not converted." A cell containing the *text* `#N/A` (entered as text, or produced
by `TEXT`/`&`) is `Text`, not `Error`, and `ISERR` returns `FALSE` for it — as does `ISERROR`,
and as does `ISNA`. If a workbook is behaving as though its errors were invisible to these
predicates, the first thing to check is whether they are errors at all or strings that look
like them.

The argument is a values position: a reference is resolved before `ISERR` sees it
(`ArgPreparationProfile::ValuesOnlyPreAdapter`).

## Result and edge cases

Returns a `Logical`, or an array of `Logical` when the argument is an array.

- **`#N/A` is `FALSE`.** This is not an edge case, it is the definition, and it is the single
  fact readers come to this page for.
- **Errors do not propagate.** `ISERR` is one of the declared per-family exceptions to error
  propagation ([coercion and lifting](../model/02-coercion-and-lifting.md)): an error argument
  is classified, not forwarded.
- **Arrays** are classified elementwise; the result is an array of the same shape.
- **`Empty`, `Text`, `Number`, `Logical`, `Reference`** are all `FALSE`.
- **Extended error codes.** Whether `ISERR` answers `TRUE` for `#SPILL!`, `#CALC!`, `#FIELD!`,
  `#BLOCKED!` or `#CONNECT!` is **not established here**. The exclusion is written against
  `#N/A` alone, so the kind-membership reading predicts `TRUE` for all of them, but that
  prediction has not been measured and Microsoft's sentence — which enumerates only the legacy
  seven for `ISERROR` — does not address it.

## Errors

`ISERR` returns no error of its own for any value it can be given. The only failure available
to it is **arity**: zero arguments, or two. `ISERR()` is expected to be refused at formula entry
rather than evaluated ([the call pipeline](../model/03-call-pipeline.md)); the reference engine,
having no entry-time surface, reports `#VALUE!` for both the too-few and the too-many case.

Microsoft's IS-functions page documents no error return for `ISERR`.

## Relationships

`ISERR` exists only in contrast, so the contrast is the section:

| Input | `ISERR` | `ISERROR` | `ISNA` |
|---|---|---|---|
| `#N/A` | `FALSE` | `TRUE` | `TRUE` |
| `#VALUE!`, `#DIV/0!`, `#REF!`, `#NAME?`, `#NUM!`, `#NULL!` | `TRUE` | `TRUE` | `FALSE` |
| anything that is not an error | `FALSE` | `FALSE` | `FALSE` |

- **Choosing between `ISERR` and `ISERROR`** is a semantic decision, not a style one. Wrapping a
  lookup in `IFERROR` or `ISERROR` converts "not found" into whatever fallback you supplied,
  which is often exactly wrong: a missing customer becomes a zero, and the zero is then summed.
  `ISERR` (or `IFNA`, or `ISNA`) keeps "not found" visible.
- **`IFNA(x, y)`** is the modern branch form for the `#N/A`-only case; there is no built-in
  `IFERR` that corresponds to `ISERR`, so `IF(ISERR(x), y, x)` remains the idiom when you
  genuinely want "handle faults, let `#N/A` through" — at the cost of evaluating `x` twice.
- **`ERROR.TYPE`** gives the code number instead of a two-way split; `ERROR.TYPE(v) <> 7` is
  another way to write the `ISERR` condition, though it errors rather than returning `FALSE` on
  non-error input.
- **`NA()`** is the function that manufactures the value `ISERR` is defined to ignore.

## Notes for implementers

1. **Implement the exclusion at the code level, not by string comparison of a rendered error.**
   Error display text is localized in some surfaces; the code is not.
2. **Test `ISERR` and `ISERROR` against the same error corpus in the same run.** The two
   functions differ by exactly one row, and a bug that gets `#N/A` wrong in one of them is
   invisible if the two are tested separately with different inputs.
3. **The error must reach the kernel.** A dispatch layer that short-circuits on error inputs
   makes `ISERR` return the error instead of a `Logical`. The propagation exception has to be
   declared at the function.
4. **When adding a new error code to an engine, `ISERR` needs a deliberate decision, not a
   default.** Any code that is not `#N/A` falls into `ISERR`'s `TRUE` half automatically under
   the kind-membership reading — which may or may not be what Excel does for the transient
   placeholder codes.

## What has not been checked

No Handbook vector suite exists for `ISERR`; `vectors/` publishes nothing for this function. No
Excel-comparison evidence record names `ISERR` as a subject — **nobody has checked this
function's answers against Excel inside the Handbook's record.** The reference engine's own
answers appear on the battery panel under their own label; they are not Excel's answers.

The probes that would settle the open questions:

1. **The extended codes, one at a time.** Produce `#SPILL!` (block a spill range), `#CALC!` (an
   uncalled `LAMBDA`), `#FIELD!` (a bad field on a linked data type), then read `ISERR` and
   `ISERROR` on each. This is the page's biggest gap and is a half-hour of work.
2. **`#GETTING_DATA`** — it is the one code that Microsoft's own `ERROR.TYPE` table assigns a
   number to (8) while `ISERROR`'s sentence omits it. Whether `ISERR` calls it `TRUE` is
   genuinely unknown and would be a satisfying divergence to find either way.
3. **The `#N/A` produced by broadcasting.** Under array lifting, a coordinate present in the
   result shape but absent from an argument yields `#N/A`
   ([the call pipeline](../model/03-call-pipeline.md)). Confirm that such a positional `#N/A` is
   the same value as `NA()`'s, and so is also excluded by `ISERR`.
4. **Arity at entry** — whether Excel refuses `=ISERR()` at entry or evaluates it to an error.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| the `#N/A` exclusion | The set difference that defines `ISERR`; the reason the function exists |
| legacy seven | `#NULL!`, `#DIV/0!`, `#VALUE!`, `#REF!`, `#NAME?`, `#NUM!`, `#N/A` |
| extended codes | Modern worksheet errors whose `ISERR` answer is not established here |
| "not found" versus "broken" | The semantic split `ISERR` encodes and `ISERROR` erases |

## Sources

- Microsoft, "IS functions" —
  <https://support.microsoft.com/en-us/office/is-functions-0f2d7971-6019-40a0-a171-f2d869135665>.
  Read for this page: the `ISERR` row ("any error value except #N/A"), the `ISERROR` row, and
  the non-conversion remark.
- Microsoft, "ERROR.TYPE function" —
  <https://support.microsoft.com/en-us/office/error-type-function-10958677-7c8d-44f7-ae77-b9a9ee6eefaa>.
  Read for the code table, which assigns 7 to `#N/A` and 8 to `#GETTING_DATA`.
- Handbook, [the value universe](../model/01-value-universe.md) — the `Error` kind and the
  legacy/extended split.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — the propagation
  discipline and its declared exceptions.
- Handbook, [the call pipeline](../model/03-call-pipeline.md) — broadcasting's positional
  `#N/A` and the admission boundary.
- `data/functions/FUNC.ISERR.json` — identity (`xlfIserr`, code 126), arity, declared axes, as
  projected at OxFunc `473efa3`.
