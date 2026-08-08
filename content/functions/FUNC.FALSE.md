---
schema: efh.function-page/v1
function_id: FUNC.FALSE
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
family: false_fn
role_in_family: "The nullary logical constant FALSE."
---

# FALSE

## What it computes

`FALSE()` is a nullary constant returning the `Logical` value FALSE. There is no input, so there
is nothing to depend on.

As with `TRUE()`, the substance of the page is the distinction between the *function* `FALSE()`
and the *literal* `FALSE` that Excel's grammar also accepts. The function form carries its own
registry identity and its own callable interface, retained for the C API and for older file
formats. In evaluation the two forms are expected to be indistinguishable.

## Arguments

`FALSE()`

None. The registry records arity exactly 0. An argument-bearing call is an admission-boundary
matter, not a runtime error — see [The call pipeline](../model/03-call-pipeline.md).

## Result and edge cases

Returns the `Logical` FALSE.

- **Determinism.** `Deterministic`, `NonVolatile`, `SafePure`.
- **Coercion downstream.** FALSE converts to the number 0 and to the text `FALSE` when a consumer
  asks; those are properties of the `Logical` kind, not of this function
  ([chapter 02](../model/02-coercion-and-lifting.md)).
- **The zero trap.** `FALSE()` is not the number 0, even though it converts to 0. `FALSE()=0` and
  `ISNUMBER(FALSE())` do not agree with that intuition, and confusing the two is the reason this
  distinction is stated on both constant pages.

## Errors

`FALSE()` produces no error value. There is no runtime failure mode.

Microsoft's `FALSE` page is the documented source for the syntax; it was not re-fetched at this
revision.

## Relationships

- `TRUE()` is the counterpart; the two pages are mirror images.
- `NOT(FALSE())` is `TRUE`.
- The literal `FALSE` is the form to write in ordinary formulas.
- `IFERROR(x, FALSE())` and similar defaults are the common real use of the function form, where
  a literal would read just as well.
- Readers occasionally reach for `FALSE()` expecting an "empty" or "no value" marker. It is not:
  the value universe keeps `Empty` and `Missing` as separate kinds
  ([chapter 01](../model/01-value-universe.md)), and neither is FALSE.

## Notes for implementers

1. Return a `Logical`, never numeric 0. The kind distinction is observable through `TYPE`,
   `ISLOGICAL`, and any kind-sensitive comparison.
2. Constant-folding at parse time is value-sound but erases the call from dependency traces; fold
   at evaluation if you care about provenance.
3. `FALSE()` and `TRUE()` should be the same code path parameterised by one boolean. They are the
   simplest possible pair, and splitting them is how their metadata drifts apart.

## What has not been checked

There is no Handbook vector suite for `FALSE`; `vectors/` publishes nothing at this revision. No
Excel-comparison evidence record names `FALSE`. Nobody has checked the function form against Excel
for this Handbook.

Probes worth running:

1. `=ISLOGICAL(FALSE())` and `=TYPE(FALSE())` — confirms the returned kind.
2. `=FALSE()=FALSE` — confirms function and literal forms are indistinguishable to a comparison.
   This page asserts they are and has not verified it.
3. `=FALSE()=0` — confirms whether comparison coerces across kinds here, which is a value-universe
   question this page deliberately does not answer.
4. Entering `FALSE(0)` — the admission-boundary question for nullary functions.

## Page vocabulary

| Term | Meaning |
|---|---|
| nullary constant | A function of no arguments whose result never varies |
| `Logical` | The TRUE/FALSE value kind, distinct from the numbers 1 and 0 |
| admission boundary | The entry-time check that rejects ill-formed calls before evaluation |

## Sources

- Microsoft, FALSE function —
  <https://support.microsoft.com/en-us/office/false-function-2d58dfa5-9c03-4259-bf8f-f0ae14346904>
  (documented source for syntax; not re-fetched at this revision).
- Handbook call-model chapters
  [01 The value universe](../model/01-value-universe.md),
  [02 Coercion and lifting](../model/02-coercion-and-lifting.md),
  [03 The call pipeline](../model/03-call-pipeline.md).
- `data/functions/FUNC.FALSE.json`, `data/presence/FUNC.FALSE.json`.
