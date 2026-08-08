---
schema: efh.function-page/v1
function_id: FUNC.MIN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - Arguments
  - The two coercion policies
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: min_fn
role_in_family: >-
  The numeric minimum: MAX's exact mirror, and the member where the documented zero for the
  no-numbers case does the most damage.
---

# MIN

## What it computes

`MIN(number1, [number2], …)` returns the smallest of the numeric values it admits.

Mathematically this is the bottom order statistic `x₍₁₎ = min{x₁ … xₙ}` — the infimum of a
finite set, attained, and therefore one of the inputs. `MIN` is a **selector**: it performs no
arithmetic, introduces no rounding error, and returns bits it was given.

`min` is associative, commutative and idempotent with identity `+∞`, so it is a monoid fold —
and, as with [MAX](FUNC.MAX.md), the identity is the one thing Excel cannot represent, so the
specification substitutes a value. Microsoft's choice is `0`, and on `MIN` that choice is more
consequential than on `MAX`; see below.

Everything else about `MIN` is admission policy, and Microsoft documents two different
policies on the same page.

## Arguments

| Argument | Meaning | Notes |
|---|---|---|
| `number1` | The first value | See the note below |
| `number2, …` | Further values | Optional |

Microsoft's page states: "Number1 is optional, subsequent numbers are optional. 1 to 255
numbers for which you want to find the minimum value."

**That first sentence is a documentation defect and the Handbook records it as a finding.**
Calling `number1` *optional* contradicts both the "1 to 255" count in the same sentence and
the syntax line `MIN(number1, [number2], ...)`, in which only the later arguments carry the
optionality brackets. It also contradicts the parallel sentence on the
[MAX](FUNC.MAX.md) page, which reads "Number1 is required, subsequent numbers are optional".
The reference engine's projection declares a minimum arity of 1 for both surfaces, agreeing
with `MAX`'s wording. The straightforward reading is that the `MIN` page has a typographical
error; the Handbook does not assume it, and notes that `=MIN()` with no arguments is refused
at formula entry under the general admission rule for required arguments
([Coercion and lifting](../model/02-coercion-and-lifting.md)), which is not a runtime error
value and would not show up as one.

Argument boundaries carry no meaning: `MIN(A1:A10)`, `MIN(A1:A5, A6:A10)` and
`MIN(A1, …, A10)` all reduce over one pooled set.

## The two coercion policies

Quoted from Microsoft's `MIN` page, which states the split in consecutive sentences:

- "Arguments can either be numbers or names, arrays, or references that contain numbers."
- "Logical values and text representations of numbers that you type directly into the list of
  arguments are counted."
- "If an argument is an array or reference, only numbers in that array or reference are used.
  Empty cells, logical values, or text in the array or reference are ignored."
- "If the arguments contain no numbers, MIN returns 0."
- "Arguments that are error values or text that cannot be translated into numbers cause
  errors."

As a table:

| Value | Typed **directly** into the argument list | Reached inside an **array or reference** |
|---|---|---|
| Number | counted | counted |
| `TRUE` / `FALSE` | counted (as 1 / 0) | **ignored** |
| Text that reads as a number | counted | **ignored** |
| Text that does not read as a number | **error** | ignored |
| Empty cell | — | ignored |
| Error value | error | error |

This is word-for-word the [MAX](FUNC.MAX.md) table with the comparator reversed, which is how
it should be: the two functions are documented to differ in nothing but direction. Microsoft's
page names the same remedy for wanting more values counted: "If you want to include logical
values and text representations of numbers in a reference as part of the calculation, use the
MINA function." See [MINA](FUNC.MINA.md), and read its rules before switching.

## Result and edge cases

Returns `Number`.

- **No numbers among the arguments.** Microsoft documents: "If the arguments contain no
  numbers, MIN returns 0." So `MIN` over a wholly blank range is documented to be `0`.

  **This is a documentation-versus-reference-engine divergence, recorded here as a finding.**
  The reference engine's own battery — OxFunc's answers, no Excel involved — returns an error
  rather than zero for its blank-argument row, while returning zero for the corresponding
  [MINA](FUNC.MINA.md) row. The documented sentence is unambiguous; the engine's `MIN` and
  `MINA` disagree with each other on the same input, which a shared rule would not produce.
  The Handbook does not resolve it — the battery row is labelled rather than fully specified
  and nobody has run the case against Excel — and publishes it as a divergence to be settled.

  **Why it matters more on `MIN` than on `MAX`.** The documented `0` is not below the data; it
  is inside the plausible range. On all-**positive** data — prices, distances, durations,
  counts, every quantity a spreadsheet is usually full of — an empty range makes `MIN` return
  `0`, which is smaller than every real value and therefore wins every subsequent comparison.
  A minimum-of-minimums over several columns reports `0` the moment one column is empty. The
  symmetric failure for `MAX` needs all-negative data, which is rarer. `MIN(MIN(A), MIN(B)) ≠
  MIN(A ∪ B)` whenever one side is empty and the other is entirely positive, and that is the
  common case rather than the exotic one.

- **A single value.** `MIN(x)` is `x`, for the largest finite double and the smallest subnormal
  alike.
- **`+0` and `−0`** compare equal under IEEE, so which representation is returned is
  unspecified and observable through `1/MIN(...)`. Unverified.
- **Text that does not parse, passed directly** — documented to cause an error, and the
  reference engine's battery agrees for the empty-string row.
- **Error values.** The reference engine classifies `MIN` with an `ErrorCollapseProfile` of
  `ReductionFold` and a canonical legacy error algebra: when several different errors are
  present, one wins by a defined rule. Which one is unverified.
- **Arrays** are consumed by scanning, not lifted elementwise.
- **Dates and times** are numbers wearing formats, so `MIN` over a date column is the earliest
  date.

## Errors

| Error | Condition | Basis |
|---|---|---|
| `#VALUE!` | A directly-passed argument is text that cannot be translated into a number | Documented on Microsoft's `MIN` page |
| propagated | An argument is an error value; the reduction folds errors by the declared profile | Documented ("arguments that are error values … cause errors"); the fold rule is from the reference engine's classification |

There is no documented `#NUM!` or `#N/A` condition, and no documented empty-input error — the
documented empty-input answer is `0`.

## Relationships

- **[MAX](FUNC.MAX.md)** — the exact mirror. Same argument shape, same documented coercion
  table, same documented `0`. Any behavioural difference beyond the comparator is a defect in
  one of them, which makes cross-testing the pair unusually productive.
- **[MINA](FUNC.MINA.md)** — the variant counting text and logicals in references, named by
  `MIN`'s own documentation. On `MINA` the zero-conversion trap is at its worst: any text in an
  all-positive column pulls the minimum to `0`.
- **[MINIFS](FUNC.MINIFS.md)** — the conditional form, which returns `0` when nothing matches —
  the same convention, with the same all-positive-data hazard.
- **[SMALL](FUNC.SMALL.md)** — the general order-statistic selector; `SMALL(array, 1)` is the
  minimum over the same admitted set, but `SMALL` errors on an empty array instead of returning
  zero. The `MIN`/`SMALL` pair is the cleanest illustration of two functions computing the same
  thing with different empty-set conventions, and `SMALL` is the safer of the two when "no
  data" must not be confused with "zero".
- **[DMIN](FUNC.DMIN.md)** — the database-family minimum with criteria-range selection.
- **[SUBTOTAL](FUNC.SUBTOTAL.md)** (function numbers 5 and 105) and
  **[AGGREGATE](FUNC.AGGREGATE.md)** (function number 5) — the filter-aware and error-skipping
  minima. `AGGREGATE` is the only route to a minimum that ignores error values.
- **Confused with:** `MIN` and `SMALL` on ranges containing text (same scan policy, so they
  should agree); and with the belief that an empty `MIN` is an error, which the documentation
  contradicts.

## Numerical notes

`MIN` has no arithmetic and therefore no rounding error. Three questions remain, and all three
are about definition.

**1. The comparison predicate.** A minimum needs a total order. IEEE-754 `<` is one on the
doubles Excel can hold, except that `+0` and `−0` compare equal; Excel has no NaN in its
published value universe, so the NaN-poisoning problem does not arise at this level. Whether
Excel's `MIN` uses raw `<` or the tolerant truncation-style 15-significant-digit comparison it
applies in some other families is open. The consequence is limited to which of two equal-
comparing representations is returned, but it is observable.

**2. The fold order.** The reference engine records `numerical_reduction_policy =
SequentialLeftFold`. For a `min` reduction the fold order cannot change the value, but it fixes
which representation survives among values comparing equal (`±0`, or near-equals under a
tolerant comparator) and which error wins in a multi-error reduction. A tree or parallel
reduction would be free to differ on both.

**3. The empty-set identity.** Mathematically `min ∅ = +∞`, which Excel cannot represent, so
the specification substitutes `0`. As argued above, the substitution is directional in its
harm: `0` is a *lower* bound for most real spreadsheet data, so an empty `MIN` does not merely
give a wrong answer, it gives an answer that dominates every correct one downstream.

For an implementer, the rule follows directly: initialise with a **found flag** plus a
sentinel, never with `0` and never with `f64::INFINITY` alone. `0` is wrong for all-positive
data; `+∞` produces a value outside Excel's value universe. Track "have I seen a number"
separately from "what is the best so far", and apply the documented zero only at the end. The
same advice appears on [MAX](FUNC.MAX.md) and is worth repeating because the two functions are
usually implemented by the same person on the same afternoon and the mirror case is easy to
get subtly different.

## What has not been checked

No evidence record lists `FUNC.MIN` among its subjects, and no count in the Handbook's evidence
layer touches this surface. Nobody has checked `MIN` against Excel within the Handbook's
record. No Handbook vector suite exists.

Microsoft's `MIN` page was retrieved for this curation pass, so the coercion table, the
empty-case rule and the argument sentence quoted above are documentation rather than
recollection.

Two findings from this pass:

1. **The documented empty case.** Microsoft documents "If the arguments contain no numbers,
   MIN returns 0." The reference engine's own battery returns an error for its blank-argument
   row for `MIN` while returning zero for the same row for `MINA`. Either the battery row does
   not construct what its label suggests, or the reference engine diverges from the documented
   rule. One cell settles it.
2. **The `number1` optionality sentence.** Microsoft's `MIN` page calls `number1` optional
   while the same sentence says "1 to 255 numbers", the syntax line brackets only `number2`,
   and the parallel `MAX` page says "required". Recorded as a documentation inconsistency, not
   as a behavioural claim.

Inputs I would probe first, and why:

1. **`=MIN(A1:A3)` over three genuinely blank cells**, and over three text cells. Both are
   documented to be `0`. This is the divergence probe.
2. **`=MIN(A1:A3)` over blanks, compared with `=MINA(A1:A3)`** on the same range — the engine
   disagreement, checked against Excel.
3. **All-positive data with one empty column, through a two-level `MIN` of `MIN`s** — the
   associativity trap in the form real workbooks meet it.
4. **The direct-versus-scan pair**: `=MIN(TRUE, 5)` and `=MIN("3", 5)` against the same values
   in cells. Four cells pin both documented rows.
5. **`=MIN(0, -0)`** and `=MIN(-0, 0)`, read through `=1/MIN(...)` — the only way `MIN` can
   return the wrong bits while returning the right number.
6. **Two different error values in one call, in both orders**, to pin the declared error fold.
7. **`=MIN(A1:A3)` with one error among two numbers**, confirming errors are not skipped in a
   scanned range — the reason `AGGREGATE` exists.
8. **`=MIN()`**, to confirm entry-time refusal and thereby settle the `number1` optionality
   sentence behaviourally.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| selector | A function whose result is bit-identical to one of its inputs |
| direct argument | A value typed into the argument list; counted including logicals and numeric text |
| scanned range | A value reached inside an array or reference; text, logicals and blanks ignored |
| empty-set identity | The documented `0` returned when no numbers are present |
| dominating zero | The reason the empty-set `0` is worse for `MIN`: it undercuts all-positive data |
| error fold | The rule choosing which error survives a reduction over several |

## Sources

- Microsoft, "MIN function" —
  <https://support.microsoft.com/en-us/office/min-function-61635d12-920f-4ce2-a70f-96f202dcc152>
  — retrieved for this curation pass. Source of the syntax, the argument sentence (including
  the "optional" wording recorded as a finding), both coercion rules, the "returns 0" rule for
  the no-numbers case, the error rule, and the pointer to `MINA`.
- Microsoft, "MAX function" —
  <https://support.microsoft.com/en-us/office/max-function-e0012414-9ac8-4b34-9a47-73e662c08098>
  — retrieved for this pass; the parallel "Number1 is required" wording against which the `MIN`
  page's sentence was compared.
- Handbook [MAX](FUNC.MAX.md) — the mirror surface and the shared empty-set divergence finding.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md) — the direct-argument
  versus range-scan asymmetry and the entry-time admission rule for required arguments.
- Handbook [The value universe](../model/01-value-universe.md) — the absence of infinities from
  the published value gamut.
- Handbook projections `data/functions/FUNC.MIN.json` (arity 1–255, `xlfMin` code 6,
  `AggregateDirectAndRangeDualPolicy`, `ReductionFold`, `SequentialLeftFold`,
  `CanonicalExcelLegacy` error algebra) and `data/presence/FUNC.MIN.json` (module `min_fn.rs`,
  unshared, no defect stream named).
