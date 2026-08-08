---
schema: efh.function-page/v1
function_id: FUNC.MIRR
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
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: financial_time_value_family
role_in_family: >-
  The closed-form answer to IRR's question: one root, no iteration, no guess, with the financing and
  reinvestment rates made explicit instead of assumed.
---

# MIRR

## What it computes

`MIRR(values, finance_rate, reinvest_rate)` returns the **modified internal rate of return** of a
series of periodic cash flows: the constant rate at which the discounted cost of the negative flows
grows into the compounded value of the positive flows over the life of the project.

Let `n` be the number of periods (one less than the number of cash flows), and split the series:

    PV_neg  =  Σ_{k: CF_k < 0}  CF_k / (1 + finance_rate)^k          — costs, discounted to time 0
    FV_pos  =  Σ_{k: CF_k > 0}  CF_k · (1 + reinvest_rate)^(n−k)     — returns, compounded to time n

Then

    MIRR  =  ( −FV_pos / PV_neg )^(1/n)  −  1

Read as a sentence: gather every outflow at the start at the rate you actually borrow at, gather
every inflow at the end at the rate you actually reinvest at, and ask what single compound rate
takes the one into the other. That is a well-posed question with exactly one answer.

### Why it exists: the two things it fixes in IRR

`MIRR` is not a variant of [IRR](FUNC.IRR.md); it is a correction of two specific defects in it.

1. **The reinvestment assumption.** `IRR` implicitly assumes every interim inflow is reinvested at
   `IRR` itself. For a project with a 40% internal rate that assumption is heroic. `MIRR` makes the
   reinvestment rate an explicit argument, so the answer says what it assumes.
2. **Multiple roots.** `IRR` solves a degree-`n` polynomial, so a series with several sign changes
   can have several real answers and the one you get depends on `guess`. `MIRR` has **no polynomial
   and no root-finding at all**: it is a single `n`-th root of a ratio of two sums, so **it is always
   unique, always exists on the admissible domain, and takes no `guess` argument**. That is the
   structural difference, and it is the reason to prefer `MIRR` whenever the cash flows are not
   conventional.

Special case worth knowing: when `finance_rate = reinvest_rate = IRR(values)`, `MIRR` returns that
same value. `MIRR` is a generalization of `IRR`, not a competitor to it.

Domain and range: the result is real whenever `FV_pos > 0` and `PV_neg < 0`, which the sign
requirement guarantees. It is bounded below by `−1` and unbounded above.

### Sign convention

The family convention holds: **negative values are payments, positive values are income**. The split
of the series into two sums is *by sign of the cash flow*, so miscoding a cost as positive does not
merely change a number — it moves that flow into the other sum and compounds it forward instead of
discounting it back. A sign error here is qualitatively worse than a sign error in `FV`.

A zero cash flow belongs to neither sum. It still consumes a period.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `values` | An array or reference containing the cash flows, **in the order in which they occur**. Required. | — |
| `finance_rate` | The interest rate paid on the money used in the cash flows — the cost of the negative flows. Required. | — |
| `reinvest_rate` | The interest rate received on the cash flows as they are reinvested — applied to the positive flows. Required. | — |

- **All three are required.** Unlike `IRR` there is no optional argument, because there is nothing to
  guess.
- **Order is meaning.** Like `IRR`, `MIRR` has no dates: position in the array is the time index.
- **`values` must contain at least one positive and at least one negative number**, otherwise one of
  the two sums is empty and the ratio is undefined.
- **The rates are per period**, in the same period as the spacing of `values`.
- **`n` is the number of *periods*, one less than the count of cash flows** — the exponent `1/n` in
  the final root uses that count, and it counts every entry the collector kept, including zeros.

## Result and edge cases

Returns `Number` — a periodic rate as a decimal fraction.

- **No iteration, no `guess`, no non-convergence.** Every failure mode is a domain failure.
- **`finance_rate = reinvest_rate = IRR(values)`** reproduces `IRR`; a useful sanity anchor.
- **`reinvest_rate = 0`** compounds nothing forward: the positive flows are simply summed. Likewise
  `finance_rate = 0` sums the negatives undiscounted. Both are admissible and meaningful.
- **`finance_rate = −1` or `reinvest_rate = −1`** makes a compounding base of zero; the reference
  engine rejects a base within a small tolerance of zero as `#DIV/0!`.
- **Zero entries consume a period** without joining either sum, so padding a series with trailing
  zeros lengthens `n` and lowers the result. This is not a rounding effect; it changes the exponent.
- **Array collection differs from `IRR`'s.** This is worth stating flatly, because the two functions
  look like siblings and their argument handling is not the same. `IRR`'s collector, in the
  reference engine at commit `473efa3`, **skips** text, logical values and blank cells inside the
  array. `MIRR`'s collector, in the same engine at the same commit, **coerces** numeric text and
  converts logicals to 1 and 0, including them as cash flows, while still skipping blanks. So a
  `TRUE` in the range is invisible to `IRR` and is a cash flow of 1 to `MIRR`, and non-numeric text
  is ignored by `IRR` but is a `#VALUE!` to `MIRR`. That is an intra-engine divergence between two
  functions readers reasonably expect to behave alike, and the Handbook records it as a finding
  rather than resolving it — see *What has not been checked*.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

| Error | Condition |
|---|---|
| `#DIV/0!` | `values` contains no positive value, or no negative value |
| `#DIV/0!` | Fewer than two cash flows |
| `#DIV/0!` | `1 + finance_rate` or `1 + reinvest_rate` is (within tolerance) zero |
| `#NUM!` | The result is not representable |
| `#VALUE!` | Non-numeric text in the array or in a rate slot |
| propagated | An error value in any argument surfaces as that error |
| `#VALUE!` | The call is made with any number of arguments other than three |

**Note the error code.** The missing-sign condition is `#DIV/0!`, not `#NUM!` — which is what the
same condition produces in [IRR](FUNC.IRR.md). It is a genuine and easily-missed asymmetry between
the two functions, it is documented on Microsoft's page for `MIRR`, and the reference engine
reproduces it. A formula that tests `ISERROR` will not notice; one that tests for a specific code
will.

## Relationships

- **[IRR](FUNC.IRR.md)** — the unmodified measure. `MIRR` differs in three ways at once: explicit
  rates, closed form, unique answer. Where `IRR` returns `#NUM!` for a series with no sign change,
  `MIRR` returns `#DIV/0!` for the same series.
- **`XIRR`** — the dated internal rate of return. There is **no dated `MIRR`**; irregular cash flows
  with an explicit reinvestment assumption have to be built by hand from `XNPV`.
- **`NPV`** — the two sums `MIRR` forms are `NPV`-shaped, and a hand-built `MIRR` is usually written
  with `NPV` and `FV`. The classic single-cell spreadsheet identity is
  `MIRR = (−NPV(reinvest, positives)·(1+reinvest)^n / (NPV(finance, negatives)·(1+finance)))^(1/n) − 1`
  — worth writing out once to see that the function is genuinely closed form.
- **`RRI`** — the compound rate between a present and a future value over `n` periods. `MIRR` **is**
  `RRI(n, −PV_neg, FV_pos)`: the two gathered sums and one compound rate between them. If `RRI` is
  available, that identity is the clearest statement of what `MIRR` means.
- **[FV](FUNC.FV.md)** — the compounding `FV_pos` performs.
- **Confused with**: `IRR` with a `guess`. The `guess` argument in `IRR` selects among genuine
  answers; `MIRR` has one answer and needs no selection.

## Numerical notes

`MIRR` is closed form and well conditioned, and that is its main numerical virtue.

**No root-finding means no conditioning problem.** Everything said on the [IRR](FUNC.IRR.md) page
about flat roots, iteration budgets, guess sensitivity and publication rules for ULP plateaus simply
does not apply here. `MIRR`'s answer is a deterministic function of its inputs evaluated in a fixed
number of operations.

**Two summations and a fractional power.** The accuracy budget is: `n` divisions accumulating into
`PV_neg`, `n` multiplications accumulating into `FV_pos`, one division, one `x^(1/n)`, one
subtraction of 1.

- The **summations** are same-sign within each sum — negatives only, and positives only — so there
  is **no cancellation in the accumulation at all**. This is a real structural advantage over `IRR`,
  whose objective function is a mixed-sign sum by construction.
- The **final subtraction of 1** is the one cancellation site. When `FV_pos/(−PV_neg)` is close to 1
  — a project that barely breaks even — the `n`-th root is very close to 1 and subtracting 1 loses
  precision proportionally. The stable form is `expm1(log1p(ratio − 1)/n)`, or equivalently
  `expm1(ln(ratio)/n)`, which holds full relative accuracy for a near-zero result. The reference
  engine at commit `473efa3` uses the naive `powf(ratio, 1/n) − 1`.
- The **exponent `1/n`** is inexact for every `n` that is not a power of two, introducing a rounding
  before the power is evaluated.

**The discount and compound factors are `pow` calls with integer exponents.** The reference engine
raises `1 + rate` to `k` and to `n − k` with a general floating-point power rather than by repeated
multiplication or binary exponentiation. For integer exponents those routes differ in the last bits,
and the choice is one of the standard axes on which financial implementations disagree. A more
accurate arrangement computes the discount factors incrementally, reusing each from the last, which
is faster but accumulates error differently — faster and *less* accurate, which is the usual
trade.

The Handbook does not claim what Excel does internally.

## What has not been checked

No Handbook vector suite exists for `MIRR`; `vectors/` publishes nothing at this revision, so no
suite-scoped claim exists for it. No Excel-comparison evidence record names `MIRR` in its subjects;
the financial records covering this implementing module name other surfaces, and the Handbook does
not attribute a group measurement to a surface a record does not list. **The family containing
`MIRR` has been measured against live Excel; this surface has not been measured separately.**
Nobody has checked `MIRR` against Excel within the Handbook's record.

The argument meanings, the sign requirement and the `#DIV/0!` code are Microsoft's documented
statements; the array-collection policy, the tolerance-based rate guard, the evaluation form and the
choice of power routine are read from the reference engine's source at commit `473efa3`.

One finding is recorded here rather than resolved: **`IRR` and `MIRR` collect their `values` array
by different rules in the reference engine** — `IRR` skips text and logicals, `MIRR` coerces them.
Whether Excel treats the two the same way is unverified, and the divergence is directly observable.

Inputs worth probing first:

1. **A range containing `TRUE` and a range containing numeric text**, passed to `IRR` and `MIRR`
   side by side. This is the intra-engine divergence above, it costs two cells, and it produces
   different answers rather than different error codes — the most publishable kind of finding.
2. **`MIRR(values, r, r)` with `r = IRR(values)`**, which must return `r`. A known expected value
   with no external oracle, and it crosses two functions and two implementations.
3. **`MIRR` against `RRI(n, −PV_neg, FV_pos)`** with the two sums built by hand, which pins the
   definition independently of the implementation.
4. **A series with no sign change**, confirming `#DIV/0!` rather than `#NUM!` — the code asymmetry
   with `IRR`.
5. **A barely-break-even series**, where the ratio is near 1 and the final subtraction of 1 is the
   dominant error source; compare against `expm1(ln(ratio)/n)` at higher precision.
6. **Trailing zero cash flows appended to a series**, which must lower the result by lengthening
   `n`; confirms that zeros consume periods.
7. **`reinvest_rate = 0` and `finance_rate = 0`**, the two degenerate but admissible settings.
8. **`finance_rate = −1`**, testing the tolerance-based `#DIV/0!` guard, and a rate just outside that
   tolerance, testing where the tolerance sits.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| modified internal rate of return | The compound rate taking the discounted costs into the compounded returns |
| finance rate | The rate at which the negative cash flows are discounted to time zero |
| reinvest rate | The rate at which the positive cash flows are compounded to the final period |
| reinvestment assumption | What `IRR` assumes implicitly and `MIRR` states explicitly |
| closed form | Evaluated in a fixed number of operations; no iteration and no `guess` |
| period count `n` | One less than the number of collected cash flows; the exponent of the final root |

## Sources

- Microsoft, "MIRR function" —
  <https://support.microsoft.com/en-us/office/mirr-function-b020f038-7492-4fb4-93c1-35c345b53524>
  (syntax, the three argument descriptions, the ordering requirement on `values`, the requirement
  for at least one positive and one negative value, and the `#DIV/0!` condition).
- Handbook, [IRR](FUNC.IRR.md) — the unmodified measure, its multiple-root behaviour and its
  differing array-collection rule.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- OxFunc `crates/oxfunc_core/src/functions/financial_time_value_family.rs` at commit `473efa3` —
  the `mirr` kernel, its sign and rate guards, its per-flow `powf` factors, its final naive root and
  the `numeric_sequence_from_args` collector it shares with the other non-`IRR` members; read as
  implementation facts about that engine, and contrasted with
  `crates/oxfunc_core/src/functions/cashflow_rate_family.rs` at the same commit, which carries
  `IRR`'s different collector.
- Handbook projections `data/functions/FUNC.MIRR.json` and `data/presence/FUNC.MIRR.json`.
