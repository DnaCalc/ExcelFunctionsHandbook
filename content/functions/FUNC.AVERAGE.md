---
schema: efh.function-page/v1
function_id: FUNC.AVERAGE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - Arguments
  - What counts, and what is skipped
  - Result and edge cases
  - Errors
  - Documentation divergences
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: average
role_in_family: >-
  The plain arithmetic mean and the reference member of the aggregate group: the function whose
  admission rule every other AVERAGE-shaped surface is described against.
---

# AVERAGE

## What it computes

`AVERAGE(number1, [number2], …)` returns the **arithmetic mean** of the admitted values:

    AVERAGE = (1/n) · Σ_{i=1..n} x_i

where \(n\) is the count of admitted values. That formula is the whole of the mathematics, and
it is also, deceptively, the least interesting part of the function. Two other things carry the
function's real content:

1. **Which values are admitted** — the direct-versus-range dual policy described below, which
   decides \(n\) as well as the numerator. Everything readers get wrong about `AVERAGE` lives
   here.
2. **How the sum is formed** — because a sum of \(n\) doubles is not a mathematical object but
   an ordered sequence of roundings, and different orderings give different answers.

The mean is the minimiser of \(\sum_i (x_i - c)^2\) over \(c\) — that is its defining
variational property, and it is why the mean, not the median, is the centre that `VAR`, `DEVSQ`
and `STDEV` deviate from. Range: the mean of finite values lies between the minimum and the
maximum, a property that plain floating-point summation does **not** guarantee to preserve
under all orderings and roundings.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number1` | The first value, range or array to average. Required. | — |
| `number2 …` | Further values, ranges or arrays. Optional, repeating. | — |

The reference engine at `473efa3` declares 1 to 255 argument slots. A slot may deliver a
scalar, an inline array, or a range; the aggregate expansion flattens all of them into one
scan.

## What counts, and what is skipped

This is the section to read, and it is also where the reference engine and Microsoft's page
disagree — so read the divergences section with it.

Microsoft's `AVERAGE` page documents the range half plainly: "If a range or cell reference
argument contains text, logical values, or empty cells, those values are ignored; however,
cells with the value zero are included." It documents the direct half as an *exclusion*:
"Logical values and text representations of numbers that you type directly into the list of
arguments are not counted."

The reference engine classifies `AVERAGE` under `AggregateDirectAndRangeDualPolicy`, meaning
**the same value is treated differently depending on how it reached the function** — and it
counts the directly-typed values that Microsoft's page says are not counted:

| Value | Written directly at the call site | Arriving through a range or array |
|---|---|---|
| Number | counted | counted (including zeros) |
| Numeric-looking text (`"2"`) | **converted and counted** — Microsoft's page says not counted | skipped (documented) |
| Non-numeric text (`"x"`) | `#VALUE!` (documented) | skipped (documented) |
| Logical `TRUE` / `FALSE` | **counted as 1 / 0** — Microsoft's page says not counted | skipped (documented) |
| Empty cell | — | skipped (documented) |
| Omitted slot | skipped | — |
| Error value | propagates (documented) | propagates (documented) |

Skipped means skipped from *both* the numerator and the denominator: it does not enter as a
zero. That is the difference between `AVERAGE` and [AVERAGEA](FUNC.AVERAGEA.md), and it is why
the two can differ by a large factor on the same range.

The consequence people meet in practice: a column with three numbers and two text cells
averages over three values under `AVERAGE` and over five under `AVERAGEA`. Neither is wrong;
they answer different questions, and the function name is the only place the choice is
recorded.

See [Coercion and lifting](../model/02-coercion-and-lifting.md) for the general coercion
machinery, and [Missing versus Empty](../model/01-value-universe.md#missing-versus-empty) for
why an omitted slot and an empty cell are different kinds that happen to behave alike here.

## Result and edge cases

Returns `Number`.

- **No admitted values.** The reference engine returns `#DIV/0!` — the denominator is literally
  zero. Compare [AVEDEV](FUNC.AVEDEV.md), which returns `#NUM!` for the corresponding case.
- **One admitted value.** Returns that value. Whether it returns *exactly* that value depends
  on whether \(x/1\) is performed at all; division by one is exact in IEEE arithmetic, so it
  does.
- **Values spanning many magnitudes.** The result is correct to within the accumulated
  summation error, which is not bounded by anything the function documents. See the numerical
  notes.
- **Overflow.** A sum can overflow to infinity even when the mean is perfectly representable —
  \(n\) values near the largest finite double have a finite mean and an infinite sum. Any
  implementation that forms \(\sum x_i\) before dividing inherits this. The projection records
  `non_finite=allow` for this surface, so the real-result policy does not itself intervene.
- **Arrays.** The surface lifts natively (`lift_broadcast_profile: surface_native`); as a
  reduction it consumes array arguments rather than broadcasting over them.

## Errors

The reference engine at `473efa3` produces:

| Error | Condition |
|---|---|
| `#DIV/0!` | No value was admitted |
| `#VALUE!` | A directly-supplied text argument could not be converted to a number |
| propagated | An error value among the admitted data becomes the result |

Microsoft's page documents the propagation row without naming a code: "Arguments that are error
values or text that cannot be translated into numbers cause errors." It does not name the
empty-data error at all, so the `#DIV/0!` row above is a reference-engine statement.

`AVERAGE` declares `error_collapse_profile: ReductionFold` in the projection — a reduction that
folds competing errors, as opposed to `AVEDEV`'s `None`. Which error wins when two different
error values are present is decided by that fold, and the Handbook has not observed it.

## Documentation divergences

1. **Directly-typed logicals and numeric text.** Microsoft's `AVERAGE` page: "Logical values
   and text representations of numbers that you type directly into the list of arguments are
   not counted." The reference engine counts them, through the same
   `AggregateDirectAndRangeDualPolicy` it applies to `AVEDEV`. One of the two is wrong about
   Excel and the Handbook has not checked which. `AVERAGE(TRUE, 2)` is `1.5` under the
   reference engine's rule and `2` under the documented rule — a difference no reader could
   miss, which is what makes this the best probe on the page.
2. **Microsoft's own pages contradict each other on the same sentence.** The
   [AVEDEV](FUNC.AVEDEV.md) page says the directly-typed values "are counted"; this page says
   they "are not counted". The two functions have the same argument shape, the same family and
   the same admission profile in the reference engine.
3. **The empty-data error is undocumented.** Microsoft's page names no error code for the case
   where nothing is admitted. The reference engine returns `#DIV/0!`; `AVEDEV` returns `#NUM!`
   for the same shape of situation. Neither is documented.

## Relationships

- **[AVERAGEA](FUNC.AVERAGEA.md)** — the same reduction with a wider admission rule: text in a
  range counts as `0` and logicals count as 1/0, both of which enlarge \(n\). The pair
  `AVERAGE`/`AVERAGEA` is the clearest example in Excel of the "same statistic, different
  admission" design, which repeats across `MAX`/`MAXA`, `MIN`/`MINA`, `STDEV`/`STDEVA` and
  `VAR`/`VARA`.
- **[AVERAGEIF](FUNC.AVERAGEIF.md) / [AVERAGEIFS](FUNC.AVERAGEIFS.md)** — the same mean over a
  criteria-selected subset. They are implemented in a different module with a different
  admission rule, so they are not `AVERAGE` with a filter bolted on.
- **`SUM` / `COUNT`** — `AVERAGE` is *not* reliably `SUM(range)/COUNT(range)`, even though the
  admission rules line up, because the two-function form rounds twice with a different
  intermediate and because `COUNT`'s treatment of directly-supplied values differs in detail.
  For most data they agree; a page that says they always agree would be overclaiming.
- **`MEDIAN`, `MODE.SNGL`, `TRIMMEAN`, `GEOMEAN`, `HARMEAN`** — the other centres. `TRIMMEAN`
  is the one to reach for when the objection to `AVERAGE` is outliers.
- **`SUBTOTAL(1, …)` and `AGGREGATE(1, …)`** — `AVERAGE` with hidden-row and error-skipping
  options, and the usual right answer when a filtered table is involved.

## Numerical notes

Averaging is the canonical worked example in floating-point error analysis, and it deserves the
treatment even though the formula fits on one line.

**Summation error.** For a plain left-to-right accumulation of \(n\) doubles, the standard
bound (Higham, *Accuracy and Stability of Numerical Algorithms*, chapter 4) is

    |ŝ − s| ≤ γ_{n−1} · Σ|x_i|,     γ_k = k·u / (1 − k·u)

with \(u = 2^{-53}\). The bound is proportional to \(\sum |x_i|\), not to \(|s|\), so the
*relative* error is governed by the condition number \(\sum|x_i| / |\sum x_i|\). Data of mixed
sign that nearly cancels — returns around zero, residuals, differences — can lose every
significant digit while the same data with a constant added stays accurate. This is the
mechanism behind essentially every reported spreadsheet averaging surprise.

**What better implementations do.** Pairwise (cascade) summation replaces \(n\) in the bound by
\(\log_2 n\) at no meaningful cost, which is why it is the default in most numerical libraries.
Compensated summation — Kahan's algorithm, and Neumaier's variant that also handles the case
where the running sum is smaller than the addend — reduces the bound to roughly \(2u\sum|x_i|\)
independent of \(n\). Exact-accumulation schemes (Shewchuk's expansions, Ogita–Rump–Oishi
`Sum2`/`SumK`, or a long accumulator) give a correctly rounded sum. Any of these changes the
last bits of the answer, which is precisely why an implementation targeting a *particular*
engine's output cannot simply adopt the best one.

**The division.** Dividing an accurately summed total by \(n\) adds at most one rounding, so
the division is never the problem. Running-mean formulations — \(m_{k} = m_{k-1} + (x_k -
m_{k-1})/k\), the Welford update — trade the overflow hazard for \(n\) divisions and a
different error profile; they are the right choice when the sum would overflow, and a different
answer when it would not.

**Ordering.** Floating-point addition is commutative but not associative. Sorting by increasing
magnitude before summing is a classical heuristic and improves the typical case for
same-sign data; it changes the answer. So does any parallel or vectorised reduction, which is
why a reduction that must agree bit-by-bit across machines has to fix its association order —
the concern the Handbook's `portable-reproducible` flavour exists to address; see
[About implementation options](../model/07-implementation-options.md).

The reference engine at `473efa3` accumulates left to right in a plain `f64` and divides once.
That is a statement about the reference engine. What Excel does internally is not asserted
here.

## What has not been checked

**Nobody has checked this function against Excel within the Handbook's record.** No Handbook
vector suite exists for `AVERAGE`; no evidence record lists this surface among its subjects;
`data/presence/FUNC.AVERAGE.json` records no mention of it in the reference engine's
discrepancy catalogue, mathematical deviation catalogue, known-exactness-deviation register, or
any open defect stream. Microsoft's page was fetched while this page was written and supplies
the admission remarks quoted above; it names no error codes. Everything else is mathematics, or
a named statement about the reference engine.

Inputs worth probing first:

1. **`AVERAGE(TRUE, 2)` and `AVERAGE("2", 4)`.** This settles divergence 1 outright: the
   documented rule and the reference engine's rule give different numbers, not different edge
   cases. Run the same pair through [AVEDEV](FUNC.AVEDEV.md), whose page states the opposite
   rule, and one Microsoft page is shown to be wrong. Highest-value probe in this batch.
2. **`AVERAGE()` over an empty range** and over a text-only range, to pin the `#DIV/0!` branch
   and confirm it is not `#NUM!` or `#VALUE!`.
3. **The cancellation battery**: `{1e16, 1, -1e16}` in all six orderings, and `{0.1}` repeated
   ten times. Any ordering sensitivity proves plain sequential summation; its absence proves
   something better is happening and would be a genuine finding.
4. **The overflow probe**: two values near `1.7e308` of the same sign. If `AVERAGE` returns a
   finite mean, the implementation is not forming the plain sum first.
5. **A long range of identical values** whose mean is not representable — a thousand copies of
   `0.1` — where the correctly rounded mean and the sequentially summed mean differ in the last
   bits.
6. **Two different error values in one range**, which the `ReductionFold` collapse profile
   claims to arbitrate and which nobody has observed.
7. **`AVERAGE` versus `SUM/COUNT`** on the same awkward data, which is the cheapest way to
   detect that the two paths round differently.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| admitted value | A value that survived the dual policy and entered both numerator and denominator |
| dual policy | Directly-written arguments are coerced; range-derived text and logicals are skipped |
| skipped | Excluded from numerator **and** denominator — not treated as zero |
| condition number of summation | \(\sum\lvert x_i\rvert / \lvert\sum x_i\rvert\); how badly cancellation can hurt |
| compensated summation | Kahan/Neumaier-style accumulation whose error bound does not grow with \(n\) |
| reference engine | OxFunc at commit `473efa3` |

## Sources

- Microsoft, "AVERAGE function" —
  <https://support.microsoft.com/en-us/office/average-function-047bac88-d466-426c-a32b-8f33eb960cf6>
  (the one-line description; the argument text and 255 limit; the six Remarks sentences quoted
  above, including the directly-typed-values rule, the range-ignores rule with its
  zero-cells-included clause, the error-causing-arguments sentence, and the pointers to
  `AVERAGEA`, `AVERAGEIF` and `AVERAGEIFS`).
- Microsoft, "AVEDEV function" —
  <https://support.microsoft.com/en-us/office/avedev-function-58fe8d65-2a84-4dc7-8052-f3f87b5c6639>
  (quoted here only for the contradicting direct-argument rule).
- Handbook projection `data/functions/FUNC.AVERAGE.json` (signature, arity 1–255,
  `AggregateDirectAndRangeDualPolicy`, `ReductionFold` error collapse, the
  `microsoft-english-verbatim` one-line description, and the Microsoft documentation address)
  and `data/presence/FUNC.AVERAGE.json` (implementing module; no defect-stream or catalogue
  mentions).
- OxFunc `crates/oxfunc_core/src/functions/average.rs` and
  `crates/oxfunc_core/src/functions/aggregate_common.rs` at commit `473efa3` — the sequential
  `f64` accumulator, the count, and the `#DIV/0!`-on-empty branch.
- Handbook, [The value universe](../model/01-value-universe.md),
  [Coercion and lifting](../model/02-coercion-and-lifting.md), and
  [About implementation options](../model/07-implementation-options.md).
- N. J. Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd ed., chapter 4 —
  summation error bounds, pairwise and compensated schemes.
- T. Ogita, S. M. Rump and S. Oishi, "Accurate Sum and Dot Product", *SIAM J. Sci. Comput.* 26
  (2005) — the `Sum2`/`SumK` family named in the numerical notes.
- B. P. Welford, "Note on a method for calculating corrected sums of squares and products",
  *Technometrics* 4 (1962) — the running-mean update.
- M. Welinder's work on the Gnumeric statistical functions and the surrounding published
  critiques of spreadsheet statistical accuracy — the standard entry point to this literature.
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md`.
