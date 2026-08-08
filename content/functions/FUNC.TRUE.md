---
schema: efh.function-page/v1
function_id: FUNC.TRUE
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
family: true_fn
role_in_family: "The nullary logical constant TRUE."
---

# TRUE

## What it computes

`TRUE()` is a nullary constant. It returns the `Logical` value TRUE, always, with no input
dependence of any kind.

That is the entire semantics, and the interesting content of the page is elsewhere: why a constant
needs a function at all, and what distinguishes the function `TRUE()` from the bare literal
`TRUE`.

Excel's formula grammar accepts `TRUE` as a literal. `TRUE()` is the *function* form, retained
because the C API and older file formats carry a callable identity for it, and because some
contexts — notably formula text produced by translation between locales — round-trip more
predictably through a function call than through a bare name that could collide with a defined
name. In evaluation, both forms are expected to deliver the same value; the Handbook records them
as distinct catalogue entries because the function has its own registry identity.

## Arguments

`TRUE()`

None. The registry records an arity of exactly 0 — minimum 0, maximum 0. Supplying an argument is
an admission-boundary matter (see [The call pipeline](../model/03-call-pipeline.md), "Arity and the
admission boundary") rather than a runtime error value.

## Result and edge cases

Returns the `Logical` TRUE.

There are no edge cases in the usual sense: no argument to coerce, no array to lift, no boundary
to cross. The only observable facts are structural:

- **Determinism.** Classified `Deterministic` and `NonVolatile`: it does not recalculate on
  workbook events and two calls in one recalculation give the same answer.
- **Coercion downstream.** When a consumer wants a number, TRUE converts to 1 (see
  [Coercion and lifting](../model/02-coercion-and-lifting.md)). When a consumer wants text, it
  renders as the string `TRUE`. Neither of these is a property of this function — they are
  properties of the `Logical` kind — but they are what readers actually come here to find out.
- **Locale.** The *displayed* and *entered* name of the logical value is localized in some Excel
  UI languages; the function identity is not. See the localized-name list rendered from
  `data/functions/FUNC.TRUE.json`.

## Errors

`TRUE()` produces no error value. There is no input that can make it fail at runtime.

An ill-formed call such as `TRUE(1)` is rejected at the admission boundary rather than evaluated to
an error, per the arity model in [The call pipeline](../model/03-call-pipeline.md). Microsoft's
`TRUE` page is the documented source for the syntax; it was not re-fetched at this revision.

## Relationships

- `FALSE()` is the exact counterpart, and everything on this page applies to it with the value
  inverted.
- `NOT(TRUE())` is `FALSE`, and the pair are each other's `NOT` images.
- The literal `TRUE` in formula text is the form almost everyone should write. The function form
  exists for interface completeness, not because it behaves differently.
- `IFS` uses `TRUE` as the conventional final condition to express a default branch — Microsoft's
  `IFS` documentation recommends exactly that idiom. That is the most common place the constant
  appears in real formulas.

## Notes for implementers

1. **Do not fold it away too early.** Constant-folding `TRUE()` to a literal at parse time is
   sound for value, but it erases the call from any dependency or provenance trace that a
   recalculation engine or auditing tool might want. Fold at evaluation, not at parse, unless you
   have decided the trace does not matter.
2. **It is a `Logical`, not the number 1.** An implementation that returns numeric 1 will pass a
   great many tests and then fail on `ISLOGICAL`, on `TYPE`, and on any comparison that
   distinguishes kinds. The value universe treats `Logical` as a distinct kind
   ([chapter 01](../model/01-value-universe.md)).
3. There is no host interaction, no reference argument, and no thread-safety consideration; the
   classification axes record `SafePure`.

## What has not been checked

There is no Handbook vector suite for `TRUE`; `vectors/` publishes nothing at this revision. No
Excel-comparison evidence record names `TRUE`. Nobody has checked the function form against Excel
for this Handbook.

For a constant this is a low-risk gap, but two probes are still worth running, because they test
claims this page makes rather than the value itself:

1. `=ISLOGICAL(TRUE())` and `=TYPE(TRUE())` — confirms the returned kind is `Logical` and not a
   number in disguise.
2. `=TRUE()=TRUE` — confirms the function form and the literal form are indistinguishable to a
   comparison, which is the claim this page makes and has not verified.
3. Entering `TRUE(1)` — establishes whether the extra argument is refused at entry or evaluated to
   an error, which is the open admission-boundary question for nullary functions generally.

## Page vocabulary

| Term | Meaning |
|---|---|
| nullary constant | A function of no arguments whose result never varies |
| admission boundary | The entry-time check that rejects ill-formed calls before evaluation |
| `Logical` | The TRUE/FALSE value kind, distinct from the numbers 1 and 0 |

## Sources

- Microsoft, TRUE function —
  <https://support.microsoft.com/en-us/office/true-function-7652c6e3-8987-48d0-97cd-ef223246b3fb>
  (documented source for syntax; not re-fetched at this revision).
- Handbook call-model chapters
  [01 The value universe](../model/01-value-universe.md),
  [02 Coercion and lifting](../model/02-coercion-and-lifting.md),
  [03 The call pipeline](../model/03-call-pipeline.md).
- `data/functions/FUNC.TRUE.json` — arity, classification axes, localized names.
- `data/presence/FUNC.TRUE.json` — the reference-engine module carrying the surface.
