---
schema: efh.function-page/v1
function_id: FUNC.SUBTOTAL
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — SUBTOTAL function"
    locator: "https://support.microsoft.com/en-us/office/subtotal-function-7b027003-f060-4ade-9040-e478765b9939"
    role: "documented signature, the full function_num table, the nested-subtotal rule, the hidden-versus-filtered distinction, the vertical-range design note, and the 3-D reference #VALUE! condition"
  - work: "Welford, Note on a Method for Calculating Corrected Sums of Squares and Products (Technometrics, 1962)"
    locator: null
    role: "the stable one-pass variance recurrence underlying the 7/8/10/11 kernels"
  - work: "Chan, Golub & LeVeque, Algorithms for Computing the Sample Variance (The American Statistician, 1983)"
    locator: null
    role: "the comparison of textbook, two-pass and updating variance formulas and their error behaviour"
  - work: "Higham, Accuracy and Stability of Numerical Algorithms"
    locator: "chapters on summation and on variance computation"
    role: "error bounds for the summation and variance kernels this function dispatches to"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - The two number ranges
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: subtotal_aggregate_family
role_in_family: >-
  The dispatcher: one call site that selects among eleven aggregate kernels and applies a
  visibility filter, and the only aggregate in the category whose answer depends on workbook UI
  state.
---

## What it computes

`SUBTOTAL(function_num, ref1, [ref2], ...)` applies one of eleven aggregate functions to its
references, with two structural behaviours layered on top:

    SUBTOTAL(f, R) = A_f ( { v in R : v is visible under the active policy,
                             and v is not itself part of a nested SUBTOTAL result } )

where `A_f` is the aggregate selected by `function_num`.

**The dispatch table**, exactly as Microsoft documents it:

| `function_num` (includes hidden rows) | `function_num` (ignores hidden rows) | Function |
|---|---|---|
| 1 | 101 | `AVERAGE` |
| 2 | 102 | `COUNT` |
| 3 | 103 | `COUNTA` |
| 4 | 104 | `MAX` |
| 5 | 105 | `MIN` |
| 6 | 106 | `PRODUCT` |
| 7 | 107 | `STDEV` |
| 8 | 108 | `STDEVP` |
| 9 | 109 | `SUM` |
| 10 | 110 | `VAR` |
| 11 | 111 | `VARP` |

`SUBTOTAL` has no mathematics of its own. Everything numerical about it is inherited from the
kernel it selects, and the eleven kernels span three quite different numerical characters: plain
reductions (`SUM`, `PRODUCT`, `MAX`, `MIN`, counting), a ratio (`AVERAGE`), and the four
second-moment functions (`STDEV`, `STDEVP`, `VAR`, `VARP`) whose stability is a research topic in
its own right. That heterogeneity is the point of the *Numerical notes* below.

**The two structural behaviours** are what actually make `SUBTOTAL` different from calling the
aggregate directly:

1. **Nested subtotals are excluded.** "If there are other subtotals within ref1, ref2,… (or
   nested subtotals), these nested subtotals are ignored to avoid double counting." This is what
   permits the classic layout — a grand total that spans a column already containing group
   subtotals — to be written without arithmetic gymnastics. It is a self-referential rule: the
   function inspects the *formulas* in its reference range, not merely their values.
2. **Invisible rows may be excluded**, on two different and independently specified axes. See
   below.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `function_num` | The aggregate selector, from the table above. Required. | — |
| `ref1` | The first range to aggregate. Required. | — |
| `ref2, ...` | Further ranges. Optional. | — |

The reference engine records an arity of two to 255.

The commonly misunderstood position is `function_num`, and specifically the belief that the
`1xx` codes are "the same but better". They are not the same: they differ on manually hidden
rows and agree on everything else. Choosing between them is choosing a policy about hidden data,
and the right choice depends on why the rows are hidden.

`ref1` and its successors are reference arguments, not values. The reference engine records
`arg_preparation_profile: RefsVisibleInAdapter` for this surface — the function receives live
references rather than pre-resolved values, which is exactly what the visibility and
nested-subtotal rules require. A `SUBTOTAL` cannot be given an array literal in place of a range
and behave the same way, because there is no visibility state attached to a literal.

## The two number ranges

The distinction between `1-11` and `101-111` is not the same as the distinction between hidden
and filtered, and conflating them is the most common `SUBTOTAL` mistake. Microsoft's two
statements, side by side:

- Codes `1-11` **include** manually hidden rows; codes `101-111` **exclude** them.
- "The SUBTOTAL function ignores any rows that are not included in the result of a filter, no
  matter which function_num value you use."

So there are two ways for a row to be invisible and they are treated differently:

| Row is invisible because… | `1-11` | `101-111` |
|---|---|---|
| a filter excluded it | ignored | ignored |
| someone hid the row manually | **included** | ignored |

Filtering is unconditional; manual hiding is the axis the code range controls. A reader who
hides rows by hand and expects `SUBTOTAL(9, ...)` to drop them will get a total that silently
includes them.

Two further documented constraints belong here:

- **`SUBTOTAL` is designed for vertical ranges.** Microsoft states it plainly: it "is designed
  for columns of data, or vertical ranges. It is not designed for rows of data, or horizontal
  ranges." Hidden *columns* have no corresponding rule, which is the practical content of the
  statement — the visibility machinery is row-shaped.
- **3-D references are rejected.** "If any of the references are 3-D references, SUBTOTAL returns
  the #VALUE! error value."

## Result and edge cases

Returns `Number`. The kind and meaning of that number are whatever the selected aggregate
produces.

- **Empty visible set.** What `SUBTOTAL(1, ...)` returns when every row is filtered out is not
  documented; `AVERAGE` over nothing is `#DIV/0!` and `SUM` over nothing is zero, so the answer
  depends on the kernel. The Handbook has not checked.
- **`function_num` out of the documented set** — `0`, `12`, `100`, `112`, a non-integer — is not
  documented. `#VALUE!` is the plausible answer and the Handbook does not assert it.
- **Text, logicals and empty cells** in the references follow each kernel's own scan policy,
  which is not uniform across the eleven: `COUNT` and `COUNTA` differ from each other by exactly
  this, and that difference is preserved through the dispatch.
- **Nested `SUBTOTAL` detection** applies to `SUBTOTAL` calls inside the referenced range. Whether
  `AGGREGATE` calls are also excluded is not stated on the page, and the two functions share an
  implementing module in the reference engine — which raises the question without answering it.

The reference engine's classification of this surface is unusually rich and worth reading
alongside the behaviour: `host_interaction_class: WorkbookState`,
`thread_safety_class: HostSerialized`, `fec_dependency_profile: Composite`,
`error_collapse_profile: ReductionFold` with `numerical_reduction_policy=SequentialLeftFold`,
`coercion_lift_profile: Custom`, and a projected `real_result_policy` of `arg_domain_guard=none`
with `non_finite=allow`.

**A tension worth recording.** The same classification declares
`determinism_class: Deterministic` and `volatility_class: NonVolatile`, while
`host_interaction_class: WorkbookState` says the answer depends on workbook state. Row visibility
is state that a user changes through the interface rather than by editing a cell, so a
`SUBTOTAL` using a `101-111` code can have its correct answer change without any cell in its
dependency graph changing. Whether that is reconcilable with a non-volatile, deterministic
classification depends on what the classification's "state" and "determinism" are scoped to, and
the projection does not say. The Handbook records the tension rather than resolving it; it is a
live question about the axis definitions, not a claim about Excel.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | Any reference is a 3-D reference | Documented by Microsoft |
| `#VALUE!` | `function_num` is outside the documented set | Not documented; plausible, unverified |
| `#DIV/0!` | The selected kernel is `AVERAGE`, `STDEV`, `VAR` (or their `1xx` forms) and the visible set is too small | Inherited from the kernel; not documented on this page |
| propagated | An error value in a visible cell surfaces as that error | Shared coercion rules and the module's `ReductionFold` |

## Relationships

- **`AGGREGATE`** — the successor, and the function that shares this one's implementing module in
  the reference engine. `AGGREGATE` extends the idea in two directions: a wider function set
  (nineteen, adding `MEDIAN`, `MODE.SNGL`, `LARGE`, `SMALL`, `PERCENTILE`, `QUARTILE` and their
  relatives) and an explicit *options* argument that separates the hidden-row policy from an
  error-ignoring policy and a nested-function policy. Where `SUBTOTAL` encodes its policy in the
  hundreds digit of a magic number, `AGGREGATE` gives it its own argument. For new work
  `AGGREGATE` is the clearer function; `SUBTOTAL` remains what filtered tables and the
  Subtotal command produce.
- **The eleven kernels** — `AVERAGE`, `COUNT`, `COUNTA`, `MAX`, `MIN`, `PRODUCT`, `STDEV`,
  `STDEVP`, `SUM`, `VAR`, `VARP`. Note that the dispatch table names the *compatibility* forms
  (`STDEV`, `STDEVP`, `VAR`, `VARP`), not the modern `STDEV.S` / `STDEV.P` / `VAR.S` / `VAR.P`
  names. `SUBTOTAL` is a fixed table written before that renaming and it did not move.
- **`SUMIFS` / `COUNTIFS`** — the criteria-based alternative when the filter is a data condition
  rather than a UI state. They are deterministic in a way `SUBTOTAL` is not, because their filter
  lives in the formula.
- **`SUM`** — the direct call, which includes everything regardless of visibility and does not
  skip nested subtotals. Replacing `SUBTOTAL(9, ...)` with `SUM(...)` in a table that has group
  subtotals double-counts.
- **Confused with**: the **Subtotal command** on the Data tab, which is a UI feature that inserts
  `SUBTOTAL` formulas and grouping outlines. The function and the command share a name and are
  not the same thing.

## Numerical notes

`SUBTOTAL` performs no arithmetic of its own; its numerical behaviour is a union of eleven
kernels' behaviours plus one property of its own.

**The property of its own: the summand set is not stable under presentation.** Filtering, hiding
and unhiding change which values enter the aggregate. That is by design, but it means a
`SUBTOTAL` result is not reproducible from the cell values alone — you need the visibility state
too. For an audit trail, or for any comparison of two workbooks, that is a materially different
situation from every other aggregate in the category. It is also why the reference engine's
`HostSerialized` thread-safety class is the right call: the function has to consult state the
calculation engine does not own.

**The kernels split into three difficulty classes.**

1. **`SUM`, `PRODUCT`, `COUNT`, `COUNTA`, `MAX`, `MIN`.** `SUM` inherits the ordinary summation
   analysis — the reference engine declares a sequential left fold for this module, so worst-case
   error grows linearly in the number of terms and the accumulation order is sheet order. Higham's
   pairwise and compensated alternatives apply and cannot be requested. `PRODUCT` is worse
   behaved than `SUM` in one specific way: a long product overflows or underflows even when the
   result is representable, because the running product has no scaling; the standard remedy is to
   accumulate in the log domain or to renormalize the exponent periodically. `MAX`, `MIN` and the
   counters are exact.
2. **`AVERAGE`.** A sum followed by a division: one extra rounding, and the sum's error carried
   through. Nothing structural.
3. **`STDEV`, `STDEVP`, `VAR`, `VARP` — the interesting ones.** The textbook computational
   formula `(SUM x^2 - n*mean^2)/(n-1)` subtracts two large nearly-equal quantities and can
   return a negative variance on data with a large mean and small spread. Chan, Golub and
   LeVeque's classic paper catalogues the alternatives and their error behaviour: the two-pass
   algorithm (compute the mean, then the sum of squared deviations) is accurate and needs the
   data twice; Welford's updating recurrence is accurate, needs one pass, and is what most modern
   libraries use. Which of these a given implementation of `STDEV` uses is not something the
   worksheet can see — and `SUBTOTAL(7, ...)` inherits it wholesale.

**The dispatch adds one more consideration: the visible subset changes the conditioning.** A
variance over a filtered subset can be far worse conditioned than the variance over the whole
column, because filtering can leave a set with a large mean and a tiny spread. A `SUBTOTAL(10,
...)` that looks stable on the unfiltered data can become numerically fragile the moment a filter
is applied, and nothing in the interface signals it.

The Handbook makes no claim about which variance algorithm Excel uses in any of these kernels.

## What has not been checked

No Handbook evidence record lists `FUNC.SUBTOTAL` in its subjects, and no Handbook vector suite
exists for it. **Nobody has checked this function against Excel within the Handbook's record.**
Every behavioural statement above is either quoted from Microsoft's page, taken from the
reference engine's declared axes, or general numerical analysis — none of it is a Handbook
observation of Excel.

Probes worth running first:

1. **Hidden versus filtered, on both code ranges.** The four-cell table above, realized: one
   column, one row hidden manually, one row filtered out, evaluated with `9` and with `109`. This
   is the documented behaviour and it is the thing readers most often have wrong, so confirming it
   is worth doing before anything else.
2. **Recalculation on visibility change.** Hide a row and observe whether a `109` subtotal updates
   without any other edit. This is the probe that turns the determinism tension recorded above
   into a fact, and it is not addressed by any documentation the Handbook has.
3. **Nested detection scope.** A range containing a nested `SUBTOTAL`, and separately a range
   containing a nested `AGGREGATE`, to find out whether the exclusion rule covers both — a
   question raised by the shared implementing module and answered nowhere.
4. **`function_num` boundaries**: `0`, `12`, `100`, `112`, `9.5`, text `"9"`, and an array-valued
   `function_num`.
5. **Horizontal ranges**, against the documented "not designed for" statement, which stops short
   of saying what happens.
6. **3-D references**, confirming the documented `#VALUE!`.
7. **The empty visible set** for each of the eleven kernels — the case where filtering removes
   everything.
8. **Variance conditioning.** `SUBTOTAL(10, ...)` over a filtered subset with a large mean and a
   tiny spread, compared against a high-precision reference, which would reveal whether the
   underlying kernel uses the textbook formula or a stable one.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| dispatch table | The `function_num` mapping from a code to one of eleven aggregates |
| the two number ranges | `1-11` versus `101-111`, differing only on manually hidden rows |
| nested-subtotal exclusion | The documented rule that `SUBTOTAL` results inside the range are skipped |
| visibility state | Row hiding and filtering — workbook state the answer depends on |
| kernel | The aggregate function selected by `function_num` |

## Sources

- Microsoft, "SUBTOTAL function" —
  <https://support.microsoft.com/en-us/office/subtotal-function-7b027003-f060-4ade-9040-e478765b9939>
  (signature; the full `function_num` table reproduced above; "these nested subtotals are ignored
  to avoid double counting"; "The SUBTOTAL function ignores any rows that are not included in the
  result of a filter, no matter which function_num value you use"; the vertical-range design
  note; and "If any of the references are 3-D references, SUBTOTAL returns the #VALUE! error
  value").
- Welford, "Note on a Method for Calculating Corrected Sums of Squares and Products",
  *Technometrics*, 1962 — the stable updating variance recurrence.
- Chan, Golub & LeVeque, "Algorithms for Computing the Sample Variance: Analysis and
  Recommendations", *The American Statistician*, 1983 — the comparison of variance formulations.
- Higham, *Accuracy and Stability of Numerical Algorithms* — summation and variance error bounds.
- Handbook projections `data/functions/FUNC.SUBTOTAL.json` (arity 2-255,
  `host_interaction_class: WorkbookState`, `thread_safety_class: HostSerialized`,
  `determinism_class: Deterministic`, `volatility_class: NonVolatile`,
  `error_collapse_profile: ReductionFold`, `numerical_reduction_policy=SequentialLeftFold`,
  `arg_preparation_profile: RefsVisibleInAdapter`) and `data/presence/FUNC.SUBTOTAL.json` (module
  shared with `FUNC.AGGREGATE`).
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [The value universe](../model/01-value-universe.md).
