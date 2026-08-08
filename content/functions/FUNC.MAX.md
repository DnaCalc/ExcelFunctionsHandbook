---
schema: efh.function-page/v1
function_id: FUNC.MAX
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
family: max_fn
role_in_family: >-
  The numeric maximum: an order-statistic selector whose entire behavioural surface is its
  admission policy, not its arithmetic.
---

# MAX

## What it computes

`MAX(number1, [number2], …)` returns the largest of the numeric values it admits.

As mathematics this is the top order statistic, `x₍ₙ₎ = max{x₁ … xₙ}` — the supremum of a
finite set, which is attained and is therefore one of the inputs. `MAX` is a **selector**: its
result is bit-identical to one of the values it was given, it performs no arithmetic, and it
introduces no rounding error of its own.

Because there is no arithmetic, everything genuinely interesting about `MAX` lives in one
question: **which values count?** That question has two different answers depending on how the
value reached the function, and Microsoft documents both.

`max` is associative, commutative and idempotent, with identity `−∞`; it is a monoid fold. The
identity is where the specification has to make a choice, because Excel has no `−∞` in its
value universe. Microsoft's choice is documented and stated below.

## Arguments

| Argument | Meaning | Notes |
|---|---|---|
| `number1` | The first value | Required |
| `number2, …` | Further values | Optional |

Microsoft's page states the count: "Number1, number2, ... Number1 is required, subsequent
numbers are optional. 1 to 255 numbers for which you want to find the maximum value." The
reference engine's projection declares the same 1-to-255 arity, so the two agree on this axis.

Argument boundaries carry no meaning: `MAX(A1:A10)`, `MAX(A1:A5, A6:A10)` and
`MAX(A1, A2, …, A10)` all reduce over one pooled set. There is no per-argument grouping and no
short-circuiting.

## The two coercion policies

This is the section that earns the page, and it is quoted rather than paraphrased because the
distinction is exact. Microsoft's `MAX` page states:

- "Arguments can either be numbers or names, arrays, or references that contain numbers."
- "Logical values and text representations of numbers that you type directly into the list of
  arguments are counted."
- "If an argument is an array or reference, only numbers in that array or reference are used.
  Empty cells, logical values, or text in the array or reference are ignored."
- "If the arguments contain no numbers, MAX returns 0 (zero)."
- "Arguments that are error values or text that cannot be translated into numbers cause
  errors."

Read as a table:

| Value | Typed **directly** into the argument list | Reached inside an **array or reference** |
|---|---|---|
| Number | counted | counted |
| `TRUE` / `FALSE` | counted (as 1 / 0) | **ignored** |
| Text that reads as a number | counted | **ignored** |
| Text that does not read as a number | **error** | ignored |
| Empty cell | — | ignored |
| Error value | error | error |

So `MAX(TRUE, 0)` is 1 while `MAX(A1:A2)` with `TRUE` in `A1` and `0` in `A2` is 0. Same
values, same function, different boundary, different rule. This is the flagship asymmetry of
[Coercion and lifting](../model/02-coercion-and-lifting.md), and `MAX` is one of the cleanest
places to see it, because both branches are documented on the same page in consecutive
sentences.

The practical consequence is that `MAX` over a column cannot be made to see logicals or
numeric text. Microsoft's own page names the remedy: "If you want to include logical values
and text representations of numbers in a reference as part of the calculation, use the MAXA
function." See [MAXA](FUNC.MAXA.md), and read its own coercion rules before switching, because
they are not simply "the same but including text".

## Result and edge cases

Returns `Number`.

- **No numbers among the arguments.** Microsoft documents: "If the arguments contain no
  numbers, MAX returns 0 (zero)." So `MAX` over a wholly blank range is documented to be `0`,
  not an error and not the empty value.

  **This is a documentation-versus-reference-engine divergence and it is recorded here as a
  finding.** The reference engine's own battery — OxFunc's answers, no Excel involved — returns
  an error rather than zero for its blank-argument row, while returning zero for the
  corresponding [MAXA](FUNC.MAXA.md) row. Two things follow. First, the documented sentence is
  unambiguous and the engine's answer does not match it. Second, the engine's own `MAX` and
  `MAXA` disagree with each other on the same input, which a shared no-numbers rule would not
  produce. The Handbook does not resolve this: the battery row's construction is labelled
  rather than fully specified, and nobody has run the case against Excel. It is published as a
  divergence to be settled, which is what this Handbook exists to do.

- **A single value.** `MAX(x)` is `x`, including for the largest finite double and the
  smallest subnormal, which is a useful check that no normalisation is applied on the way out.
- **`+0` and `−0`.** IEEE comparison calls these equal, so which one is returned is a
  tie-breaking decision the specification does not make. `MAX(0, −0)` may legitimately return
  either representation, and the two are distinguishable through the sign bit and through
  `1/x`. Unverified.
- **Text that does not parse, passed directly** — documented to cause an error, and the
  reference engine's battery agrees for the empty-string row.
- **Error values** among the arguments. The reference engine classifies `MAX` with an
  `ErrorCollapseProfile` of `ReductionFold` and a canonical legacy error algebra, which is the
  projection's way of saying that when several different errors are present the reduction
  picks one by a defined rule rather than arbitrarily. Which error wins is a real, observable
  fact that the Handbook has not pinned.
- **Arrays** are consumed by scanning, not lifted elementwise; `MAX` is not a lift kernel.
- **Dates and times** are numbers wearing formats, so `MAX` over a date column returns the
  latest date — correctly, and with the format usually carried through by the cell rather than
  by the function.

## Errors

| Error | Condition | Basis |
|---|---|---|
| `#VALUE!` | A directly-passed argument is text that cannot be translated into a number | Documented on Microsoft's `MAX` page |
| propagated | An argument is an error value; the reduction folds errors by the declared profile | Documented ("arguments that are error values … cause errors"); the fold rule is from the reference engine's classification |

There is no documented `#NUM!` or `#N/A` condition. Note the absence of an "empty input" error
in the documentation — the documented answer for that case is `0`, as discussed above.

## Relationships

- **[MIN](FUNC.MIN.md)** — the exact mirror, same argument shape, same coercion table, same
  documented zero for the no-numbers case. Any behavioural difference between `MAX` and `MIN`
  beyond the comparator is a defect in one of them.
- **[MAXA](FUNC.MAXA.md)** — the variant that counts text and logicals inside references,
  named by Microsoft's own `MAX` page as the remedy. The `A` suffix family (`MAXA`, `MINA`,
  `AVERAGEA`, `COUNTA`, `STDEVA`, `VARA`) shares that intent and does not share one uniform
  rule; each page states its own.
- **[MAXIFS](FUNC.MAXIFS.md)** — the conditional form, which selects the population by criteria
  before taking the maximum. Note that `MAXIFS` returns `0` when no cell meets the criteria,
  which is the same convention the `MAX` documentation states for the empty case.
- **[LARGE](FUNC.LARGE.md)** — the general order-statistic selector; `LARGE(array, 1)` is the
  maximum over the same admitted set, but `LARGE` takes exactly one array and errors on an
  empty one instead of returning zero. The pair `MAX`/`LARGE` is the cleanest illustration of
  the empty-set convention differing between two functions that compute the same thing.
- **[DMAX](FUNC.DMAX.md)** — the database-family maximum, with criteria-range selection.
- **[SUBTOTAL](FUNC.SUBTOTAL.md)** (function numbers 4 and 104) and
  **[AGGREGATE](FUNC.AGGREGATE.md)** (function number 4) — the filter-aware and
  error-skipping maxima. `AGGREGATE` is the only route to a maximum that ignores error values,
  which `MAX` cannot do.
- **Confused with:** `MAX` and `LARGE` on ranges containing text (the same scan policy governs
  both, so they should agree — a disagreement is diagnostic); and with the idea that `MAX` over
  an empty range is an error, which the documentation contradicts.

## Numerical notes

`MAX` has no arithmetic and therefore no rounding error. The engineering questions are three,
and each of them is about definition rather than precision.

**1. The comparison predicate.** A maximum needs a total order. IEEE-754 `>` is a total order
on the doubles Excel can hold, with the single exception that `+0` and `−0` compare equal —
Excel has no NaN in its value universe, so the usual NaN-poisoning problem does not arise at
this level. Whether Excel's `MAX` uses raw `>` or the tolerant, truncation-style
15-significant-digit comparison it applies in some other families is a genuine open question;
for `MAX` a tolerant comparator would only affect which of two near-equal representations is
returned, so the consequence is smaller than it is for lookups, but it is observable.

**2. The fold order.** The reference engine records `numerical_reduction_policy =
SequentialLeftFold` for this surface. For a `max` reduction the fold order cannot change the
*value* — `max` is associative and commutative — but it can change **which representation** is
returned among values that compare equal (`+0` versus `−0`, or two near-equal values under a
tolerant comparator), and it fixes which error wins when several errors are present. A
parallel or tree reduction would be free to differ on both counts. This is the reason a
reduction that looks order-independent is nevertheless pinned to an order in the projection.

**3. The empty-set identity.** Mathematically `max ∅ = −∞`, which Excel cannot represent, so
the specification substitutes `0`. That substitution is not innocuous: it makes `MAX`
non-associative across the empty case in a way that matters for incremental computation. If a
range is empty, `MAX` returns a value inside the data's range rather than below it, so
`MAX(MAX(A), MAX(B))` is not `MAX(A ∪ B)` when one of `A` or `B` is empty and the other is
entirely negative. A workbook that computes column maxima and then takes the maximum of those
will report `0` for a set of all-negative data with one empty column. That is a real, silent
correctness trap and it follows directly from the documented rule.

For an implementer: initialise the accumulator with a sentinel *and a found flag*, never with
`0` or with `f64::NEG_INFINITY` alone — the first is wrong for all-negative data and the second
produces `−∞`, which is outside Excel's value universe. Track "have I seen a number" separately
from "what is the best so far", and apply the documented zero only at the end.

## What has not been checked

No evidence record lists `FUNC.MAX` among its subjects, and no count in the Handbook's
evidence layer touches this surface. Nobody has checked `MAX` against Excel within the
Handbook's record. No Handbook vector suite exists.

Microsoft's `MAX` page was successfully retrieved for this curation pass, so the coercion table
and the empty-case rule above are quoted documentation rather than recollection. That makes
this page unusual in the set — most of its neighbours could not be retrieved — and it is
precisely why the divergence above could be stated with confidence about the documentation
side.

**The divergence, restated as a finding.** Microsoft documents "If the arguments contain no
numbers, MAX returns 0 (zero)." The reference engine's own battery returns an error for its
blank-argument row for `MAX` while returning zero for the same row for `MAXA`. Either the
battery row does not construct what its label suggests, or the reference engine diverges from
the documented rule on one of the most commonly hit inputs in the product. Settling it takes
one cell.

Inputs I would probe first, and why:

1. **`=MAX(A1:A3)` over three genuinely blank cells**, and `=MAX(A1:A3)` where the three cells
   contain text. Both are documented to be `0`. This is the divergence probe and it is the
   first thing to run.
2. **`=MAX(A1:A3)` over three blanks, compared with `=MAXA(A1:A3)`** on the same range — the
   engine disagreement, checked against Excel.
3. **The direct-versus-scan pair**: `=MAX(TRUE, 0)`, `=MAX("3", 2)` against the same values in
   cells. Four cells that pin both rows of the documented coercion table.
4. **`=MAX(0, -0)`** and `=MAX(-0, 0)`, read through `=1/MAX(...)` to expose the sign of zero.
   The only way `MAX` can return "the wrong bits" while returning the right number.
5. **Two different error values in one call**, in both orders — `=MAX(#N/A, #DIV/0!)` and the
   reverse — to pin the error-fold rule the projection declares.
6. **`=MAX(A1:A3)` with one error among two numbers**, confirming that errors are not skipped
   in a scanned range (the shared rule says they are not, and `AGGREGATE` exists precisely
   because of it).
7. **All-negative data with one empty column**, through a two-level `MAX` of `MAX`s — the
   associativity trap. Not a bug hunt so much as a documentation exhibit.
8. **256 arguments**, one past the documented limit, to confirm entry-time refusal rather than
   a runtime error.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| selector | A function whose result is bit-identical to one of its inputs |
| direct argument | A value typed into the argument list; counted including logicals and numeric text |
| scanned range | A value reached inside an array or reference; text, logicals and blanks ignored |
| empty-set identity | The documented `0` returned when no numbers are present |
| error fold | The rule choosing which error value survives a reduction over several |
| `ReductionFold` | The reference engine's error-collapse profile for this surface |

## Sources

- Microsoft, "MAX function" —
  <https://support.microsoft.com/en-us/office/max-function-e0012414-9ac8-4b34-9a47-73e662c08098>
  — retrieved for this curation pass. Source of the 1-to-255 argument count, both coercion
  rules quoted above, the "returns 0 (zero)" rule for the no-numbers case, the error rule, and
  the pointer to `MAXA`.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md) — the direct-argument
  versus range-scan asymmetry, of which this function is the canonical instance.
- Handbook [The value universe](../model/01-value-universe.md) — value kinds, the absence of
  infinities and NaN from the published gamut, and the Empty/Missing distinction.
- Handbook projections `data/functions/FUNC.MAX.json` (arity 1–255, `xlfMax` code 7,
  `AggregateDirectAndRangeDualPolicy`, `ReductionFold`, `SequentialLeftFold`,
  `CanonicalExcelLegacy` error algebra) and `data/presence/FUNC.MAX.json` (module `max_fn.rs`,
  unshared, no defect stream named).
