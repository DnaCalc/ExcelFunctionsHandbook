---
schema: efh.function-page/v1
function_id: FUNC.AVEDEV
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
  - Documentation divergences
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: avedev_fn
role_in_family: >-
  The sole member of its module: the mean absolute deviation about the sample mean, the one
  dispersion statistic in Excel that is not built out of squared deviations.
---

# AVEDEV

## What it computes

`AVEDEV(number1, [number2], …)` returns the **mean absolute deviation about the mean** of the
admitted values. For a finite sample \(x_1,\dots,x_n\),

    x̄     = (1/n) · Σ_{i=1..n} x_i
    AVEDEV = (1/n) · Σ_{i=1..n} |x_i − x̄|

Both sums run over the same \(n\); the divisor is \(n\), not \(n-1\). There is no
"population versus sample" pair for this statistic in Excel — `AVEDEV` is the only member.

Three properties are worth having in front of you, because they are the ones that get
misremembered:

1. **It is a deviation about the mean, not about the median.** The functional
   \(c \mapsto \sum_i |x_i - c|\) is minimised at the *median*, not the mean. So `AVEDEV` is
   not the smallest achievable mean absolute deviation of the sample, and it is not the
   statistic usually written MAD in the robust-statistics literature (which is the median
   absolute deviation about the median). Excel has no function for that one.
2. **It is not on the same scale as a standard deviation.** For a sample drawn from a normal
   distribution with standard deviation \(\sigma\), the expected absolute deviation is
   \(\sigma\sqrt{2/\pi} \approx 0.7979\,\sigma\). Comparing an `AVEDEV` against a `STDEV.S` on
   the same data and concluding the data is "less spread than expected" is a scale error, not
   a finding.
3. **It is not differentiable in the data.** \(|x_i - x̄|\) has a corner wherever
   \(x_i = x̄\). Two datasets that differ by an arbitrarily small perturbation can have
   `AVEDEV` values whose derivative with respect to that perturbation flips sign. This matters
   for anything that optimises through `AVEDEV`, and it matters for last-bit comparison: near a
   tie the branch taken by \(|\cdot|\) is decided by the low bits of \(x_i - x̄\).

Range: `AVEDEV` is non-negative, and zero exactly when every admitted value is equal.
Units are the units of the data, unlike a variance.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number1` | The first value, range or array to include. Required. | — |
| `number2 …` | Further values, ranges or arrays. Optional, repeating. | — |

The reference engine at commit `473efa3` declares an arity of 1 to 255 argument slots. Each
slot is an aggregate slot, so a single slot may deliver one scalar, an inline array, or a
whole range.

The admission rule is the shared **direct-versus-range dual policy** that governs the whole
`AVERAGE`-shaped aggregate group (the reference engine's
`AggregateDirectAndRangeDualPolicy`), and it is the single most consequential thing about the
argument list. Microsoft's `AVEDEV` page documents both halves of it:

- A value written **directly** at the call site — `AVEDEV(TRUE, "2")` — is coerced. "Logical
  values and text representations of numbers that you type directly into the list of arguments
  are counted."
- The same value arriving **through a reference or array** is **skipped**, not coerced. "If an
  array or reference argument contains text, logical values, or empty cells, those values are
  ignored; however, cells with the value zero are included."

So `AVEDEV(A1:A2)` and `AVEDEV(A1, A2)` are not interchangeable when the cells hold anything
other than numbers. See [Coercion and lifting](../model/02-coercion-and-lifting.md) for the
general shape of this rule.

Microsoft also documents 1 to 255 arguments, and the caution that "AVEDEV is influenced by the
unit of measurement in the input data" — which is the documentation's way of saying what the
\(\sqrt{2/\pi}\) remark above says: this statistic carries the data's units and is not
standardised.

Empty cells and omitted argument slots contribute nothing and do not enter \(n\); see
[Missing versus Empty](../model/01-value-universe.md#missing-versus-empty).

## Result and edge cases

Returns `Number`.

- **One admitted value.** \(x̄ = x_1\), so the result is exactly `0`. This is a real answer,
  not a degenerate one, and it is why a one-cell `AVEDEV` never errors.
- **All values equal.** Exactly `0`, and exactly the IEEE zero — every deviation is a true
  zero before summation.
- **No admitted values.** The reference engine returns `#NUM!`. Note the contrast with
  [AVERAGE](FUNC.AVERAGE.md), which returns `#DIV/0!` in the corresponding situation; the two
  functions in the same neighbourhood report emptiness with different error codes.
- **Negative values, non-integers, very large values.** All ordinary; the domain is the whole
  of the finite doubles. There is no domain guard on this surface in the reference engine's
  axis record (`arg_domain_guard=none`).
- **Non-finite values.** The projection records `non_finite=allow` for this surface, meaning
  the real-result policy does not itself reject infinities; what a formula can actually deliver
  into the slot is a separate question governed by the call model.
- **Arrays.** The surface lifts natively (`lift_broadcast_profile: surface_native`) — it is a
  reduction, so array arguments are consumed rather than broadcast over.

## Errors

The reference engine at `473efa3` produces:

| Error | Condition |
|---|---|
| `#NUM!` | No value was admitted (every slot was empty, or every value was skipped by the dual policy) |
| `#VALUE!` | A directly-supplied text argument could not be converted to a number; or the call carries no argument slots |
| propagated | An error value anywhere in the admitted data is returned as the function's result |

Error propagation is the aggregate rule, not a special case: the first error encountered in
the scan becomes the result. `AVEDEV` does not carry a declared error-collapse profile in the
projection (`error_collapse_profile: None`), which distinguishes it from `AVERAGE` and
`AVERAGEA`, both of which declare `ReductionFold` — a difference in how competing errors are
folded, and one worth probing rather than assuming.

**Microsoft's `AVEDEV` page documents no error conditions at all.** Its Remarks section covers
the admission rule and the unit-sensitivity caution and stops there. Every row of the table
above is therefore a reference-engine statement, not a documented one.

## Documentation divergences

1. **The empty-data error is undocumented.** The reference engine returns `#NUM!` when nothing
   is admitted. Microsoft's page names no error for that case — or for any case. A reader
   cannot learn from the documentation whether an all-text range yields `#NUM!`, `#DIV/0!`, or
   zero.
2. **Microsoft's `AVEDEV` page and its `AVERAGE` page state opposite direct-argument rules.**
   `AVEDEV`: "Logical values and text representations of numbers that you type directly into
   the list of arguments **are counted**." [AVERAGE](FUNC.AVERAGE.md): "Logical values and text
   representations of numbers that you type directly into the list of arguments **are not
   counted**." The two functions share an admission profile in the reference engine, which
   implements the *counted* reading for both. At most one of the two Microsoft pages describes
   Excel; the Handbook has not checked which.

## Relationships

- **[AVERAGE](FUNC.AVERAGE.md)** — supplies the centre \(x̄\) that `AVEDEV` deviates from, and
  applies the same direct-versus-range admission rule. `AVEDEV` is not `AVERAGE(ABS(range −
  AVERAGE(range)))` in general, because that array formula does not reproduce the skip rule.
- **`DEVSQ`** — the same construction with squares instead of absolute values:
  \(\sum (x_i - x̄)^2\), *unnormalised*. `DEVSQ`, `VAR.P` and `AVEDEV` are three normalisations
  of the same deviation scan.
- **`STDEV.S` / `STDEV.P`** — the squared-deviation dispersion measures. See the \(\sqrt{2/\pi}\)
  scale factor above before comparing them to `AVEDEV` numerically.
- **`MEDIAN`** — the centre that would actually minimise the absolute-deviation sum. There is
  no built-in that computes the median absolute deviation; it needs an array formula.
- **Confused with**: the robust MAD estimator. They share an abbreviation and almost nothing
  else.

## Numerical notes

`AVEDEV` is a **two-pass** statistic and cannot be made one-pass: the mean must be known before
any deviation can be formed, and unlike the variance there is no algebraic expansion
(\(\sum x^2 - n x̄^2\)) that lets a single accumulator stand in, because \(|\cdot|\) does not
expand. Every implementation therefore either stores the data or reads it twice. The reference
engine stores it.

The interesting error analysis is the interaction of two stages:

1. **The mean.** A plain left-to-right accumulation of \(\sum x_i\) has a worst-case relative
   error bound growing like \(n u\) (with \(u\) the unit roundoff) and, in the standard
   analysis, a bound proportional to \(\sum |x_i| / |\sum x_i|\) — the condition number of
   summation. Data of mixed sign, or data with a large common offset, is where this bites.
   Higham's *Accuracy and Stability of Numerical Algorithms* chapter 4 is the standard
   treatment; pairwise summation reduces the growth to \(\log_2 n\), and compensated
   (Kahan/Neumaier) summation removes the \(n\) dependence almost entirely.
2. **The deviations.** \(x_i - x̄\) is a subtraction of nearby quantities. When the data has a
   large offset relative to its spread — say values around \(10^9\) that differ in the third
   decimal — the subtraction is exact in IEEE arithmetic (Sterbenz's lemma applies when the
   operands are within a factor of two), but the *input* \(x̄\) already carries the error of
   stage 1, and that error is inherited by every deviation.

There is a mitigating structure worth knowing. Write
\(g(c) = \sum_i |x_i - c|\). Away from ties, \(g'(c) = \#\{i : x_i < c\} - \#\{i : x_i > c\}\).
So an error \(\delta\) in the computed mean perturbs the deviation sum by roughly
\(\delta \cdot (\text{below} - \text{above})\). On data that is roughly symmetric about its
mean this factor is small, and `AVEDEV` is far less sensitive to the accuracy of \(x̄\) than
one might fear. On strongly skewed data the factor approaches \(\pm n\), and the mean's error
is amplified by the sample size. This is the analysis that tells you which datasets to build a
probe suite from.

A careful implementation would: accumulate the mean with a compensated or pairwise sum; form
the deviations against the *rounded* mean actually used (not a higher-precision one, if the
goal is reproducing a particular engine); and accumulate the absolute deviations with the same
discipline. The reference engine at `473efa3` does none of this — it uses plain sequential
`f64` accumulation for both passes, and divides by `n` as an `f64`. That is a statement about
the reference engine, not about Excel.

Excel's statistical lane has a documented history of exactly this class of weakness; Morten
Welinder's work on Gnumeric's statistical functions, and the accompanying published critiques
of Excel's statistical accuracy, are the standard entry point to that literature and are worth
reading before assuming any spreadsheet's dispersion statistic is well conditioned.

## What has not been checked

**Nobody has checked this function against Excel within the Handbook's record.** No Handbook
vector suite exists for `AVEDEV`, no evidence record lists this surface among its subjects, and
this surface does not appear in the reference engine's discrepancy catalogue, its mathematical
deviation catalogue, or its known-exactness-deviation register. Microsoft's page was fetched
while this page was written, and supplies the admission rule and the defining formula and
nothing else; every error condition on this page is a reference-engine statement. Everything
else above is mathematics.

Inputs worth probing first, in order of what they would settle:

1. **`AVEDEV()` over an empty range, and over a range of only text.** The cheapest probe of the
   `#NUM!`-versus-`#DIV/0!` split against `AVERAGE`, and the only way to settle a case
   Microsoft's page does not mention.
2. **`AVEDEV(TRUE, "2")` against `AVEDEV(A1:A2)` with the same two values in cells.** The
   direct-versus-range dual policy, isolated. This probe also adjudicates the contradiction
   between Microsoft's `AVEDEV` and `AVERAGE` pages: run it on both functions and one of the
   two pages is shown to be wrong.
3. **`AVEDEV(1E9, 1E9+0.001, 1E9+0.002)`** and the same data shifted to `0, 0.001, 0.002`. The
   offset-cancellation probe: any difference beyond the exactly-representable shift is
   summation error, and it localises which pass is responsible.
4. **Strongly skewed data** — one value far from a tight cluster — which is where the
   \((\text{below} - \text{above})\) amplification factor is largest and where an engine that
   computes the mean differently will diverge first.
5. **A value exactly equal to the mean**, so that a deviation is exactly zero and the \(|\cdot|\)
   corner is exercised: `AVEDEV(1, 2, 3)`.
6. **Ordering**: the same multiset supplied in ascending, descending and interleaved order.
   Any difference proves the accumulation is order-sensitive plain summation, which is exactly
   what the reference engine does and what a compensated implementation would not do.
7. **An error value inside an otherwise valid range**, and two *different* error values, to see
   which one surfaces — the `error_collapse_profile: None` record on this surface versus
   `ReductionFold` on `AVERAGE` predicts a difference nobody has observed.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| admitted value | A value that survived the dual policy and entered both the mean and the deviation sum |
| dual policy | The rule that directly-written arguments are coerced while range-derived values are skipped |
| mean absolute deviation | \((1/n)\sum|x_i − x̄|\); deviation about the **mean**, divisor \(n\) |
| two-pass | An algorithm that must read the data once for the centre and again for the deviations |
| reference engine | OxFunc at commit `473efa3`, the Handbook's strongest evidence source and not its definition |

## Sources

- Microsoft, "AVEDEV function" —
  <https://support.microsoft.com/en-us/office/avedev-function-58fe8d65-2a84-4dc7-8052-f3f87b5c6639>
  (the defining formula; the 1-to-255 argument count; the unit-sensitivity caution; the
  directly-typed logicals-and-numeric-text rule; the ignored-in-array-or-reference rule with
  its zero-cells-included clause; and the absence of any documented error condition).
- Microsoft, "AVERAGE function" —
  <https://support.microsoft.com/en-us/office/average-function-047bac88-d466-426c-a32b-8f33eb960cf6>
  (quoted here only for the contradicting direct-argument rule).
- Handbook projection `data/functions/FUNC.AVEDEV.json` (signature, arity, classification axes,
  the `microsoft-english-verbatim` one-line description, and the Microsoft documentation
  address) and `data/presence/FUNC.AVEDEV.json` (implementing module, absence from every defect
  and discrepancy register).
- OxFunc `crates/oxfunc_core/src/functions/avedev_fn.rs` and
  `crates/oxfunc_core/src/functions/aggregate_common.rs` at commit `473efa3` — the two-pass
  plain-`f64` kernel, the `#NUM!`-on-empty branch, and the dual-policy admission helper.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md) — value kinds, empty versus
  missing, and the coercion rules this page links rather than restates.
- N. J. Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd ed., chapter 4
  (summation: error bounds, pairwise and compensated schemes) — the analysis behind the
  numerical notes above.
- M. Welinder's work on the Gnumeric statistical functions, and the published critiques of
  spreadsheet statistical accuracy that surround it — named here as the standard entry point
  to the literature on this failure class, not as evidence about this surface.
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md`.
