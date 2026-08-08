---
schema: efh.function-page/v1
function_id: FUNC.ERROR.TYPE
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
family: error_type_fn
role_in_family: Sole member of its module; the only function that names which error, and the only
  one that answers "no error" with an error.
---

## What it computes

`ERROR.TYPE(error_val)` maps an error value to a number identifying its code, and returns `#N/A`
for anything that is not an error.

Microsoft's table, verbatim:

| Error value | Number |
|---|---|
| `#NULL!` | 1 |
| `#DIV/0!` | 2 |
| `#VALUE!` | 3 |
| `#REF!` | 4 |
| `#NAME?` | 5 |
| `#NUM!` | 6 |
| `#N/A` | 7 |
| `#GETTING_DATA` | 8 |
| Anything else | `#N/A` |

Two features of this table are worth pausing on.

**The order is Excel's canonical legacy error order.** The eight codes are numbered in the same
sequence Excel uses when several errors compete inside a reduction or a branch selector
(`ErrorAlgebra::CanonicalExcelLegacy` in [the call pipeline](../model/03-call-pipeline.md)).
`ERROR.TYPE` is therefore not just a lookup table — it is the published form of an ordering that
the engine also uses internally.

**The last row is a deliberate confusion.** `ERROR.TYPE(5)` returns `#N/A`, and `ERROR.TYPE(#N/A)`
returns `7`. The function answers "this is not an error" by producing an error, which means the
naive test `ERROR.TYPE(x) > 0` cannot distinguish "no error" from "the error is `#N/A`". Guard
`ERROR.TYPE` with `ISERROR` or `ISNA` before trusting its result — Microsoft's own description
recommends exactly that pattern: "You can use ERROR.TYPE in an IF function to test for an error
value and return a text string, such as a message, instead of the error value."

**`#GETTING_DATA` has a code but no `ISERROR` sentence.** Microsoft's `ISERROR` row enumerates
seven codes and stops at `#N/A`; this table adds an eighth. The two documentation pages were
written at different times and the Handbook does not know which is currently true of the running
product for that code.

## Arguments

`error_val` — required, exactly one. The published signature is `ERROR.TYPE(error_val)`.

Microsoft's argument description: "The error value whose identifying number you want to find.
Although error_val can be the actual error value, it will usually be a reference to a cell
containing a formula that you want to test."

That sentence describes the normal use. `ERROR.TYPE(A1)` where `A1` holds a broken formula is
how the function is meant to be called; `ERROR.TYPE(#DIV/0!)` with a literal error is legal and
mostly appears in tests.

The argument is a values position: a reference is resolved before the function runs
(`ArgPreparationProfile::ValuesOnlyPreAdapter`), so it is the cell's *result* that is classified.

## Result and edge cases

Returns a `Number` (1–8) for an error input, or the `Error` `#N/A` for everything else.

- **Errors do not propagate through this function.** Like the IS classifiers, `ERROR.TYPE`
  inspects rather than forwards, and that is what makes it usable at all.
- **`#N/A` in, `7` out.** The one input for which the function's success value and its failure
  value are related — and the reason the guard is needed.
- **Empty, text, numbers, logicals: `#N/A`.** All "anything else".
- **An array argument.** Not addressed by the documentation. The reference engine classifies the
  argument as a whole rather than elementwise, returning a single `#N/A` for an array of
  numbers. Whether Excel lifts `ERROR.TYPE` over an array is **not established here** and is the
  page's main open question — it matters because an elementwise `ERROR.TYPE` would be a useful
  diagnostic over a range and a whole-argument one is not.
- **The extended error codes.** `#SPILL!`, `#CALC!`, `#FIELD!`, `#BLOCKED!`, `#CONNECT!` and
  `#BUSY!` are absent from the published table. Under the table's own last row they would return
  `#N/A`, which would make `ERROR.TYPE` unable to name the most common modern error, `#SPILL!`.
  The reference engine's declared code map covers 1–8 and returns `#N/A` for the rest. **Nobody
  has checked what current Excel does**, and this is probe 1 below.

## Errors

`ERROR.TYPE` returns `#N/A` as its ordinary "not an error" answer — that is a result, not a
failure.

The only genuine failure available to it is **arity**: zero arguments, or two. `ERROR.TYPE()` is
expected to be refused at formula entry rather than evaluated
([the call pipeline](../model/03-call-pipeline.md)); the reference engine, having no entry-time
surface, reports `#VALUE!` for both.

Note that the function can never return `#VALUE!` from a *value* — every value has an answer.

## Relationships

- **`ISERROR` / `ISERR` / `ISNA`** answer *whether*; `ERROR.TYPE` answers *which*. The idiomatic
  pairing is `IF(ISERROR(x), CHOOSE(ERROR.TYPE(x), "…", "…", …), x)`, and the guard is not
  optional for the reason given above.
- **`TYPE`** returns `16` for every error without distinguishing them; `ERROR.TYPE` is its
  refinement.
- **`IFERROR` / `IFNA`** replace the whole diagnostic pattern when you only want a fallback
  rather than a message. `ERROR.TYPE` survives because a fallback is not a diagnosis: a
  monitoring sheet that reports *how many* `#REF!`s appeared after a restructure needs the code,
  not a substitute value.
- **`NA()`** manufactures the value that maps to `7`.
- **`ERROR.TYPE`'s ordering and Excel's error precedence** are the same sequence; a page or a
  test that needs the precedence order can read it off this table.

## Notes for implementers

1. **Emit the published numbers literally.** The codes are Excel's, not an internal enum's
   ordinals, and the mapping must not drift if the internal enum is reordered.
2. **The "anything else" row must return `#N/A`, not `0` and not an error of its own.** The
   temptation to return `0` for "no error" is strong and wrong.
3. **Decide the array question explicitly.** Whole-argument classification and elementwise
   lifting are both defensible; the documentation settles neither, so whichever is chosen should
   be recorded as a decision awaiting evidence.
4. **Any error code the engine can produce but cannot name must fall to `#N/A` deliberately.**
   Silently returning a made-up number for `#SPILL!` would be worse than the documented gap.
5. **The error must reach the kernel** rather than being short-circuited by the dispatch layer.

## What has not been checked

No Handbook vector suite exists for `ERROR.TYPE`; `vectors/` publishes nothing for this function.
No Excel-comparison evidence record names `ERROR.TYPE` as a subject — **nobody has checked this
function's answers against Excel inside the Handbook's record.** The reference engine's own
answers appear on the battery panel under their own label; they are not Excel's answers.

The probes that would settle the open questions, in priority order:

1. **The extended error codes.** `ERROR.TYPE` of a deliberately produced `#SPILL!`, `#CALC!`,
   `#FIELD!`, `#BLOCKED!` and `#CONNECT!` on a build that has them. Either they return `#N/A` —
   confirming that Excel's most visible modern error is invisible to its own error-diagnosis
   function, which would be a genuinely notable published finding — or they return codes that
   Microsoft's table does not document, which would be more notable still.
2. **`#GETTING_DATA`.** It has a documented code (8) but is missing from `ISERROR`'s enumeration.
   Reading it through `ERROR.TYPE`, `ISERROR` and `ISERR` in one probe resolves a documentation
   inconsistency that spans two pages.
3. **An array argument** — `ERROR.TYPE({#N/A, 1})` and `ERROR.TYPE(A1:A3)` with a mixed range,
   spilled and un-spilled.
4. **Each of the eight documented codes**, produced naturally rather than as literals
   (`1/0`, `SQRT(-1)`, a deleted-reference `#REF!`, an undefined name, `A1 A2` for `#NULL!`), to
   confirm the mapping end to end.
5. **Arity at entry** — whether Excel refuses `=ERROR.TYPE()` at entry or evaluates it.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| code 1–8 | The published identifying numbers for the eight named error values |
| the guard | Testing with `ISERROR`/`ISNA` before trusting an `ERROR.TYPE` result |
| canonical legacy order | Excel's error-precedence sequence, which this table's numbering follows |
| extended codes | Modern worksheet errors absent from the published table |

## Sources

- Microsoft, "ERROR.TYPE function" —
  <https://support.microsoft.com/en-us/office/error-type-function-10958677-7c8d-44f7-ae77-b9a9ee6eefaa>.
  Read for this page: the syntax, the `error_val` argument description verbatim, the full code
  table including the "Anything else" row, and the usage sentence recommending the `IF` guard.
- Microsoft, "IS functions" —
  <https://support.microsoft.com/en-us/office/is-functions-0f2d7971-6019-40a0-a171-f2d869135665>.
  Read for the `ISERROR` row, whose seven-code enumeration omits `#GETTING_DATA`.
- Handbook, [the value universe](../model/01-value-universe.md) — the fourteen modeled error
  codes and the legacy/extended split.
- Handbook, [the call pipeline](../model/03-call-pipeline.md) —
  `ErrorAlgebra::CanonicalExcelLegacy` and the admission boundary.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — the propagation
  discipline and the inspection functions' exemption.
- `data/functions/FUNC.ERROR.TYPE.json` — identity, the published signature
  `ERROR.TYPE(error_val)`, arity, declared axes, as projected at OxFunc `473efa3`.
