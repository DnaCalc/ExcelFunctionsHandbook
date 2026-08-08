---
schema: efh.function-page/v1
function_id: FUNC.COUNTA
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: COUNTA function"
    locator: "https://support.microsoft.com/en-us/office/counta-function-7dc98875-d5c1-46f1-9a82-53f3219e2509"
    role: "retrieved for this pass; syntax, the counts-any-type-of-information rule including error values and empty text, the does-not-count-truly-empty-cells rule, and the contrast with COUNT"
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
family: counta
role_in_family: >-
  Its own module: the non-emptiness counter, and the one member of the counting cluster whose
  predicate is a classification of the value kind rather than a coercion attempt.
---

# COUNTA

## What it computes

`COUNTA(value1, [value2], …)` counts the arguments and scanned cells that are **not empty**.

The predicate is a single question asked of each value: *is anything here?* It is not "is this a
number", not "does this coerce to a number", and not "does this display something". Microsoft's
page states it as counting cells containing any type of information, "including error values and
empty text" — and, explicitly, not counting truly empty cells.

That gives the following classification, which is the whole function:

| Value | Counted? |
|---|---|
| Number, including `0` | yes |
| Text, including the empty string `""` | yes |
| Logical `TRUE` / `FALSE` | yes |
| Error value, e.g. `#DIV/0!` | yes |
| A formula that returns `""` | yes — the cell contains a formula |
| A genuinely empty cell | no |
| An omitted argument slot | no |

Two consequences are worth stating because they are where readers are surprised.

**`COUNTA` counts errors.** It is the only common counting function that does. This is what makes
`COUNTA` useful as a "how many cells have been filled in" measure and useless as a "how much good
data do I have" measure. `COUNT` answers the second question.

**`COUNTA` counts empty text.** A cell holding `=IF(A1>0,"x","")` is counted whether or not the
condition held, because the cell holds a formula and the formula published a text value — a
zero-length one. This is the single most common cause of `COUNTA` disagreeing with what a human sees
on the screen, and it is documented. Its mirror image is [COUNTBLANK](FUNC.COUNTBLANK.md), which
counts `""` as blank; the two functions therefore both count the same cell, and
`COUNTA(r) + COUNTBLANK(r)` exceeds the cell count of `r` by the number of empty-text cells. That
is not a defect in either function — it is the documented consequence of two different definitions
of "blank" living in one product.

## Arguments

`COUNTA(value1, [value2], ...)` — one to 255 arguments; the registry records `min 1, max 255`.

| Argument | Meaning | Optional |
|---|---|---|
| `value1` | The first value, range, array or reference to examine | no |
| `value2`, … | Up to 254 further such arguments | yes |

The commonly misunderstood position is that there isn't one — every position behaves identically.
What *does* differ is how a value arrives. An omitted slot (`COUNTA(1,,2)`) delivers the Missing
marker and contributes nothing; an empty *cell* reached through a range delivers Empty and likewise
contributes nothing. The two are different kinds arriving by different routes at the same answer
here ([chapter 01](../model/01-value-universe.md)), and `COUNTA` is one of the few functions where
they agree.

## Result and edge cases

Returns `Number` — a non-negative integer count. It never returns an error on account of the data.

- **An error value in a scanned range does not propagate.** It is counted. This is a deliberate
  exception to the general rule that errors flow through functions, and it is what makes `COUNTA`
  survivable on messy data. The reference engine's classification records
  `ErrorCollapseProfile::ReductionFold` for this surface, which is the machinery that decides such
  questions; the observed effect for `COUNTA` is that the error becomes a count of one rather than
  the function's result.
- **`COUNTA()` of a range with nothing in it** is `0`, not an error.
- **Arrays are permitted.** OxFunc's `BUG-FUNC-011` record explicitly names `COUNT` and `COUNTA` as
  *contrast controls* that stay array-permissive while `COUNTBLANK` was narrowed to ranges only —
  and cites a live Excel replay for the contrast. That makes the array-permissiveness of `COUNTA`
  one of the better-anchored structural facts on this page, though the anchor is OxFunc's record,
  not a Handbook measurement.
- **Direct arguments versus scanned cells.** `COUNTA("")` and a cell containing `""` are both
  counted, so unlike the aggregates `COUNTA` shows no direct-versus-scan asymmetry
  ([chapter 02](../model/02-coercion-and-lifting.md)). That is a consequence of the predicate never
  attempting a coercion at all.

The value kind returned is always Number even when nothing was counted, so `ISNUMBER(COUNTA(...))`
is always `TRUE` and cannot be used as a guard.

## Errors

`COUNTA` has no documented data-dependent error conditions — that is the point of it.

| Error | Condition |
|---|---|
| `#VALUE!` | Argument-count or argument-preparation failure in the reference engine |

Reference-engine detail: preparation failures terminate evaluation with an error and a worksheet
error code already carried through preparation is preserved rather than replaced. This is
implementation fact, not documented behaviour.

## Relationships

- **[COUNT](FUNC.COUNT.md)** — the numeric-only counterpart, and the comparison Microsoft's own page
  draws: use `COUNT` if you do not need to count logical values, text, or error values. `COUNT`
  applies a coercion policy; `COUNTA` applies a classification. On a clean numeric column they
  agree; on any real column they do not, and the difference is the diagnostic.
- **[COUNTBLANK](FUNC.COUNTBLANK.md)** — the complement, except that it is not a complement. Both
  count a cell holding `""`. See the arithmetic note above.
- **[COUNTIF](FUNC.COUNTIF.md) / [COUNTIFS](FUNC.COUNTIFS.md)** — the conditional counters
  Microsoft's page points to. Note that `COUNTIF(range,"<>")` is *not* a `COUNTA` substitute: it
  runs the criteria matcher with its own blank-handling rules rather than the non-emptiness
  predicate.
- **`ROWS`/`COLUMNS`** — the shape questions. `ROWS(r)*COLUMNS(r) − COUNTA(r)` counts truly empty
  cells and is not the same as `COUNTBLANK(r)`, again because of empty text.
- **Confused with**: `COUNTA` as a data-quality measure. A column of `#N/A` is fully populated
  according to `COUNTA`.

## Notes for implementers

1. **Classify, do not coerce.** The predicate is on the value kind, and any implementation that
   reaches for a to-number conversion has already lost — it will either count `"abc"` as zero or
   fail on it. The reference engine's contract states this directly: prepared values are counted by
   emptiness classification rather than numeric coercion.
2. **Empty text is text.** A zero-length string is a Text value and counts. This is the rule most
   often got wrong, and it must be decided at the value-kind level, not by asking whether the string
   renders as something.
3. **Do not fold errors.** An incoming error is a value to count, not a result to propagate. This is
   an explicit per-family exception to the universal error-propagation discipline
   ([chapter 02](../model/02-coercion-and-lifting.md)) and needs to be declared, not inherited.
4. **Distinguish Missing from Empty at the boundary even though they agree here.** They agree for
   `COUNTA` and disagree for other members of the counting cluster; conflating them upstream makes
   the other functions unimplementable.
5. **Sparse ranges are a performance question, not a semantic one.** The reference engine takes a
   sparse path for reference-derived values and a dense path otherwise; both must give the same
   count, and a whole-column reference makes the difference between the two paths very visible.

## What has not been checked

No Handbook vector suite exists for `COUNTA`; `vectors/` publishes nothing at this revision. No
Excel-comparison evidence record names `COUNTA` in its subjects. The Handbook has not itself
compared this surface to Excel.

What exists nearby: OxFunc's `BUG-FUNC-011` stream records a live Excel replay in which `COUNTA`
appears as an array-permissive contrast control, and OxFunc's `COUNTA` slice contract records a
bounded local empirical baseline naming an Excel build, a channel and a locale. Those are upstream
observations with their own scope; they are not Handbook measurements, and neither of them sweeps
this surface. The B1 battery rendered beside this page reports the reference engine's answers on a
fixed edge-input row set — again, not Excel.

Inputs worth probing first:

1. **A cell containing `=""` and a cell containing a literal empty string typed in** — both should
   count, and the two entry routes are worth separating because only one of them is a formula.
2. **A cell that was filled and then cleared with Delete, versus one cleared with Backspace-Enter**
   — the classic way a "truly empty" cell turns out not to be. This is the probe that most often
   explains a `COUNTA` a user calls wrong.
3. **Each of the fourteen error codes in turn**, including the extended-family ones such as
   `#SPILL!`, `#CALC!` and `#FIELD!` ([chapter 01](../model/01-value-universe.md)). The documented
   rule says error values are counted; it does not say *which* error values, and the extended family
   postdates the wording.
4. **A rich value** — a linked data type cell, and an in-cell image. Their core projections are
   text and the model does not say what `COUNTA` sees. Entirely unaddressed by the documentation.
5. **An uncalled `LAMBDA` in a cell**, whose core projection is `#CALC!`.
6. **An inline array literal in a direct argument slot**, `COUNTA({1,"",TRUE})`, against the same
   values in a range — confirms the array permissiveness from the Handbook's own side rather than
   inheriting OxFunc's replay.
7. **`COUNTA(1,,2)`** — the omitted middle slot, which must contribute nothing.
8. **A whole-column reference on a sheet with one filled cell** — checks that the sparse and dense
   paths agree and that empty cells outside the used range are not counted.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| non-emptiness predicate | The single test `COUNTA` applies: is there a value here at all |
| empty text | A zero-length string, `""`; a Text value, and counted |
| truly empty cell | A cell with no content; the only thing `COUNTA` does not count |
| classification | Deciding by value kind rather than by attempting a conversion |
| contrast control | A surface deliberately left permissive in an upstream experiment, to bound a change made elsewhere |

## Sources

- Microsoft, "COUNTA function" —
  <https://support.microsoft.com/en-us/office/counta-function-7dc98875-d5c1-46f1-9a82-53f3219e2509>
  (retrieved for this pass: syntax and the 255-argument maximum, the rule that `COUNTA` counts cells
  containing any type of information including error values and empty text, that it counts formulas
  returning empty strings, that it does not count truly empty cells, and the pointer to `COUNT` when
  logical, text and error values should be excluded).
- Handbook call-model chapters [01 The value universe](../model/01-value-universe.md) (value kinds,
  Missing versus Empty, the fourteen error codes) and
  [02 Coercion and lifting](../model/02-coercion-and-lifting.md) (error propagation as a per-family
  declaration; the direct-argument versus range-scan distinction).
- Handbook projections `data/functions/FUNC.COUNTA.json` (arity, `ReductionFold` error-collapse
  profile, aggregate dual policy) and `data/presence/FUNC.COUNTA.json`.
- OxFunc `docs/function-lane/FUNCTION_SLICE_COUNTA_CONTRACT_PRELIM.md` — the emptiness-classification
  rule, the "text, logical, numeric, error and dereferenced reference values all count" statement,
  and the named local empirical baseline; and
  `docs/bugs/streams/BUG-FUNC-011_countblank_range_only_parity_gap.md` — the live Excel replay in
  which `COUNTA` is recorded as an array-permissive contrast control. Both read as upstream records
  with their own scope, not as Handbook measurements.
- OxFunc `crates/oxfunc_core/src/functions/counta.rs` at commit `473efa3`.
