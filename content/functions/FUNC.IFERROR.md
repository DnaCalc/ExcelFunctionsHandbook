---
schema: efh.function-page/v1
function_id: FUNC.IFERROR
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
family: iferror
role_in_family: "The total error mask: returns a fallback when the first argument is any error value."
---

# IFERROR

## What it computes

`IFERROR(value, value_if_error)` returns `value` unless `value` is an **error value of any code**,
in which case it returns `value_if_error`.

The test is on the *kind* of the value, not on its content: the function asks "is this an `Error`?"
and nothing else. All fourteen worksheet error codes
([chapter 01](../model/01-value-universe.md)) are masked equally — `#DIV/0!`, `#N/A`, `#VALUE!`,
`#REF!`, `#NAME?`, `#NUM!`, `#NULL!`, and the extended family alike.

This is the crucial property and the crucial danger. `IFERROR` is a **total** mask: it cannot
distinguish a divide-by-zero you expected from a `#REF!` caused by a deleted column or a `#NAME?`
caused by a typo in the very formula it wraps. Wrapping a lookup in `IFERROR` to suppress "not
found" also suppresses every structural break in that lookup, silently and permanently. `IFNA`
exists precisely because that trade is usually a bad one.

`IFERROR` is one of the declared exceptions to the engine's universal error-propagation rule: the
coercion model states that errors propagate through conversion *unless* a family explicitly
declares branching behaviour, and `IFERROR` is named as such a declaration
([chapter 02](../model/02-coercion-and-lifting.md), "Error propagation").

Like `IF`, it is a lazy selector — the fallback is only reached when the primary is an error.

## Arguments

`IFERROR(value, value_if_error)`

- **`value`** — required. The expression to test. In practice this is the whole formula you wanted
  to write; `IFERROR` wraps it.
- **`value_if_error`** — required. The registry records an arity of exactly 2: both minimum and
  maximum. Unlike `IF`'s third argument, this one cannot be omitted.

The commonly misunderstood position is `value_if_error`: readers write `IFERROR(x, "")` intending
"leave the cell blank" and get an empty *string*, which is a `Text` value, not an `Empty` one. It
sorts, counts, and concatenates as text; `ISBLANK` on the result is FALSE. The value universe has
no way for a formula to publish emptiness ([chapter 01](../model/01-value-universe.md), admission
matrix: `Empty` is admitted everywhere except the published result).

## Result and edge cases

Returns whatever kind the selected argument carries.

- **Primary is not an error.** Returned verbatim, including text, logicals, and arrays.
- **Primary is any error.** The fallback is returned, verbatim.
- **Fallback is itself an error.** Returned as-is. `IFERROR(1/0, NA())` is `#N/A` — the mask does
  not iterate.
- **Empty cell as the primary.** The reference engine maps an `Empty` argument to numeric `0`, so
  `IFERROR(A1, "x")` with `A1` blank is `0`, not `"x"` — blank is not an error.
- **Empty text as the primary.** `IFERROR("", "")` is the empty string. Empty text is a perfectly
  good non-error value.
- **Omitted argument slot.** The reference engine maps a `Missing` argument to `#VALUE!`, which
  means an empty slot in the primary position makes the *fallback* the result.
- **Arrays.** The function's own classification is surface-native; an array primary containing a
  mix of errors and values raises the question of whether the mask applies elementwise or to the
  array as a whole. That question is listed below as unchecked, and it is the most consequential
  unknown on this page for modern dynamic-array formulas.

## Errors

| Error | Condition |
|---|---|
| any error | Only by being returned from `value_if_error`, or by propagating from a failure in preparing the arguments themselves. |

`IFERROR` has no error condition of its own beyond argument preparation: its entire purpose is to
be the place errors stop. Microsoft's `IFERROR` page is the documented source for the syntax and
the argument contract; it was not re-fetched at this revision.

## Relationships

- **`IFNA`** is the surgical version: it masks `#N/A` only and lets every other error through. For
  lookup formulas this is almost always the right choice, and the pair `IFERROR`/`IFNA` is the
  clearest "total versus selective" contrast in the function library.
- **`ISERROR`, `ISERR`, `ISNA`** are the inspection functions; `IF(ISERROR(x), y, x)` is the
  pre-2007 idiom `IFERROR` replaced, and it evaluates `x` twice. `ISERR` is `ISERROR` minus `#N/A`
  — the same distinction `IFNA` draws, from the other side.
- **`AGGREGATE`** with the appropriate option skips errors inside an aggregation, which is the
  aggregate-shaped analogue of masking.
- **`IF`** returns branch errors verbatim and is the function people wrongly expect to handle
  errors.
- `IFERROR` arrived in Excel 2007. Workbooks that must open in Excel 2003 use the double-evaluation
  `IF(ISERROR(...))` idiom instead; that is a version-compatibility fact, not a
  Compatibility-category supersession, and Microsoft has not deprecated anything here.

## Notes for implementers

1. **Test the kind, not the code.** Any per-code logic belongs in `IFNA`, not here. If your
   `IFERROR` needs a list of error codes, the design has already gone wrong.
2. **Do not evaluate the fallback eagerly.** The whole point of `IFERROR` over the `ISERROR` idiom
   is single evaluation of the primary and no evaluation of the fallback when it is not needed.
3. **Decide the array policy explicitly and write it down.** Elementwise masking and whole-array
   masking give different answers for any array containing both errors and values, and this is
   exactly where a dynamic-array-era implementation will diverge from a legacy one.
4. **Preserve rich values through the pass-through path.** A non-error primary must be returned
   unchanged, including any rich payload; reconstructing it from its core projection silently
   downgrades linked data types and callables ([chapter 01](../model/01-value-universe.md)).

## What has not been checked

There is no Handbook vector suite for `IFERROR`; `vectors/` publishes nothing at this revision. No
Excel-comparison evidence record names `IFERROR`. Nobody has checked its behaviour against Excel
for this Handbook.

Probes worth running, in priority order:

1. `=IFERROR({1, 1/0, 3}, "x")` — elementwise or whole-array? Everything about `IFERROR` in
   dynamic-array formulas depends on this answer, and no source consulted here settles it.
2. `=IFERROR(A1, "x")` with `A1` empty — confirms whether blank yields `0` (as the reference
   engine does) or the fallback.
3. `=IFERROR(, "x")` — an empty slot in the primary position; the reference engine maps `Missing`
   to `#VALUE!` and therefore returns the fallback, which is a surprising route to the fallback.
4. `=IFERROR(1/0, NA())` — confirms the mask does not iterate over its own fallback.
5. `=IFERROR(#GETTING_DATA, "x")` or another extended-family code, if one can be produced — tests
   whether the mask is really total across the extended error family or only across the legacy
   seven.
6. `=IFERROR(LAMBDA(x,x), "x")` — tests whether a callable's `#CALC!` core projection is seen as an
   error by the mask, which would make `IFERROR` swallow uncalled lambdas.

## Page vocabulary

| Term | Meaning |
|---|---|
| total mask | Branches on error-ness without distinguishing error codes |
| lazy selector | The fallback is only evaluated when the primary is an error |
| declared masking | A per-family exception to the engine's universal error-propagation rule |
| core projection | The legacy-gamut value a rich value presents to older surfaces |

## Sources

- Microsoft, IFERROR function —
  <https://support.microsoft.com/en-us/office/iferror-function-c526fd07-caeb-47b8-8bb6-63f3e417f611>
  (documented source for syntax and argument contract; not re-fetched at this revision).
- Handbook call-model chapters
  [01 The value universe](../model/01-value-universe.md) (the fourteen error codes; the admission
  matrix) and [02 Coercion and lifting](../model/02-coercion-and-lifting.md) (error propagation
  and its declared exceptions, where `IFERROR` is named).
- `data/functions/FUNC.IFERROR.json`, `data/presence/FUNC.IFERROR.json`.
- OxFunc `crates/oxfunc_core/src/functions/iferror.rs` at commit 473efa3 — the any-error test and
  the `Missing`→`#VALUE!`, `Empty`→`0` argument mapping, read as implementation facts about the
  reference engine.
