---
schema: efh.function-page/v1
function_id: FUNC.NA
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
family: na_fn
role_in_family: Sole member of its module; the nullary constant that manufactures an error value
  on purpose.
---

## What it computes

`NA()` returns the error value `#N/A`. That is the entire specification.

It is a **nullary constant** (`KernelSignatureClass::NullaryConst`, arity min 0 max 0):
no arguments, no inputs, no state, no dependence on the calling cell. It is the only function in
the Information category whose purpose is to *create* an error rather than to inspect one, and
one of very few functions anywhere in Excel whose normal, correct, intended result is an error
value.

The reason it exists is that `#N/A` in Excel does not mean "something went wrong". It means
*value not available* — a first-class statement about data. Writing `NA()` into a cell is a
deliberate assertion that the number belongs there but is not known, and it has two concrete
consequences that a blank or a zero does not:

1. **It propagates.** Any formula that consumes the cell inherits `#N/A`, so a total computed
   from incomplete data announces its incompleteness instead of quietly under-reporting.
2. **Charts skip it.** A data point of `#N/A` is omitted from a line rather than plotted at zero,
   which is the difference between a gap in a series and a spike to the axis.

Zero lies, blank is ambiguous, `NA()` is honest. That is the design.

## Arguments

None. The signature is `NA()`, arity exactly zero.

The empty parentheses are required — the name `NA` on its own is a defined-name reference, not a
function call, and will produce `#NAME?` if no such name exists. This is the same rule that
applies to `TRUE()`, `TODAY()` and `RAND()`, but `NA` is the one people most often type without
parentheses, because `#N/A` is what they are thinking of and it has no parentheses in it.

Supplying an argument is an arity failure; see "Errors".

## Result and edge cases

Returns the `Error` value `#N/A`, always, on every call.

- **It is a value, not an exception.** `#N/A` flows through arithmetic and function calls like
  any other scalar ([the value universe](../model/01-value-universe.md)); nothing is thrown and
  nothing is caught.
- **Every `#N/A` is the same `#N/A`.** The value model gives errors a code and no provenance, so
  the `#N/A` from `NA()`, the one from a failed `MATCH`, and the one broadcasting supplies for a
  missing coordinate ([the call pipeline](../model/03-call-pipeline.md)) are indistinguishable
  downstream. Nothing can tell them apart, which is worth knowing before designing a workbook
  that would like to.
- **Deterministic and non-volatile.** `NA()` does not force recalculation and does not depend on
  the calling cell — unlike `CELL` or `INFO`, the other members of the Information category that
  take little or no input.
- **In array formulas** it is a scalar and broadcasts like any constant.
- **Typing `#N/A` directly into a cell** is a recognised way to enter the same value in Excel's
  user interface; `NA()` is the formula form. Whether the two produce values that are equal in
  every respect has not been checked here, though the value model offers no way for them to
  differ.

## Errors

`NA()` *returns* an error by design; it does not *fail*.

The only failure available to it is **arity**: any argument at all. `NA(1)` is expected to be
refused at formula entry rather than evaluated
([the call pipeline](../model/03-call-pipeline.md)); the reference engine, having no entry-time
surface, reports `#VALUE!`.

Note the distinction the Handbook keeps: `#N/A` as the *result* of `NA()` is success. `#VALUE!`
from `NA(1)` is failure. Two error values, two entirely different meanings, in the same
function's row.

## Relationships

- **`ISNA`** is the predicate that tests for exactly what `NA()` produces. `ISNA(NA())` is
  `TRUE`, and it is the one-line smoke test for both functions.
- **`IFNA(value, value_if_na)`** is the modern branch on this value, and `IFERROR` is the broader
  one that also catches everything else. If a workbook deliberately writes `NA()` to mark missing
  data, wrapping downstream formulas in `IFERROR` destroys the signal; `IFNA` is not the culprit
  because it at least catches only the intended value, but the same caution applies.
- **`ISERR`** deliberately excludes `#N/A`, which is the complement of what `NA()` does: `NA()`
  makes the value that `ISERR` is defined to ignore.
- **`ERROR.TYPE(NA())`** is `7`.
- **`NA()` versus a blank cell** is a modelling choice. A blank is skipped by aggregates and
  treated as zero in scalar arithmetic; `NA()` stops the aggregate. If a total must not be
  published from incomplete inputs, `NA()` is the mechanism.
- **`XLOOKUP`'s `if_not_found`** and **`VLOOKUP`'s natural `#N/A`** are the functions that
  produce this value without being asked; `NA()` is how you produce it on purpose.

## Notes for implementers

1. **`NA()` is a constant kernel, so it needs no evaluation context**, no resolver and no host.
   An engine that routes it through a general dispatch path with argument preparation is doing
   work that cannot affect the answer.
2. **Arity zero, exactly.** Minimum and maximum are both 0. A function whose declared arity is
   "at most zero" and whose battery cannot express "no arguments" will produce a confusing row;
   this is a known shape wrinkle in mechanical probing, not a behaviour.
3. **Do not represent `#N/A` as a distinguishable "user `#N/A`".** Adding provenance would make
   an engine diverge from Excel in a way no workbook could observe directly but every comparison
   run would.
4. **`NA()` must not be optimised into a blank or a zero** by any constant-folding pass; the
   error value is the result.

## What has not been checked

No Handbook vector suite exists for `NA`; `vectors/` publishes nothing for this function. No
Excel-comparison evidence record names `NA` as a subject — **nobody has checked this function's
answers against Excel inside the Handbook's record.** The reference engine's own answers appear
on the battery panel under their own label; they are not Excel's answers.

This is the simplest function in the assignment and the shortest list of open questions, but the
list is not empty:

1. **`NA()` versus a hand-typed `#N/A`** read through `ISNA`, `ERROR.TYPE`, `TYPE` and a
   comparison operator, to confirm the two are one value.
2. **Arity at entry** — whether Excel refuses `=NA(1)` at entry or evaluates it to an error, and
   which error. This is a small contribution to the Handbook's open admission-boundary question,
   and `NA` is an unusually clean place to ask it because the zero-argument case is the normal
   one.
3. **Chart and aggregate behaviour**, which are host obligations rather than function semantics
   but are the whole reason people use `NA()`; they belong on record somewhere even if not as a
   function claim.
4. **`NA()` inside a `LAMBDA` and inside `MAP`/`REDUCE`**, to confirm the constant survives the
   callable helpers unchanged rather than being caught by a helper's error handling.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| nullary constant | A kernel with no arguments whose result never varies |
| `#N/A` | The "value not available" error code; `NA()`'s only result |
| provenance | Where an error came from; the value model does not record it |
| deliberate error | An error value that is the correct answer, not a failure |

## Sources

- Microsoft, "NA function" —
  <https://support.microsoft.com/en-us/office/na-function-5469c2d1-a90c-4fb5-9bbc-64bd9bb6b47c>.
  The documentation of record for this function; the Handbook's projected short description,
  "Returns the error value #N/A", is Microsoft's English verbatim.
- Handbook, [the value universe](../model/01-value-universe.md) — errors as first-class values
  and the place of `#N/A` in the closed code registry.
- Handbook, [the call pipeline](../model/03-call-pipeline.md) — `NullaryConst` kernels,
  broadcasting's positional `#N/A`, and the admission boundary for arity.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — error propagation as
  engine law, which is what makes `NA()` useful.
- `data/functions/FUNC.NA.json` — identity (`xlfNa`, code 10), the published signature `NA()`,
  arity 0–0, declared axes, as projected at OxFunc `473efa3`.
