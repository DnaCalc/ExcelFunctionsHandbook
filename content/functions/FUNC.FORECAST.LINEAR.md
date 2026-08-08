---
schema: efh.function-page/v1
function_id: FUNC.FORECAST.LINEAR
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MISC-0007
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Forecast_Linear method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.forecast_linear"
    role: "documented description, the a+bx equation, and the three documented error conditions"
  - work: "Microsoft Learn — WorksheetFunction.Forecast method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.forecast"
    role: "the legacy surface's documentation, including its deprecation note"
  - work: "Microsoft Support — FORECAST and FORECAST.LINEAR functions"
    locator: "https://support.microsoft.com/en-us/office/forecast-and-forecast-linear-functions-50ca49c9-7b40-4892-94e4-7ad38bbeda99"
    role: "the shared worksheet-surface page; not retrievable at curation time (the host refused the request)"
  - work: "Å. Björck, Numerical Methods for Least Squares Problems"
    locator: "chapters on the normal equations and their conditioning"
    role: "the conditioning argument behind the centered slope formula"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - The relationship to FORECAST, stated precisely
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: regression_forecast_family
role_in_family: >-
  The modern spelling of the scalar single-predictor forecast; shares one kernel with FORECAST
  in the reference engine and carries no independent Excel measurement of its own.
---

## What it computes

`FORECAST.LINEAR(x, known_ys, known_xs)` fits a straight line to the paired data by ordinary
least squares and evaluates it at `x`. Microsoft's documented equation is **a + bx**, with the
sample means named as `AVERAGE(all known_x)` and `AVERAGE(all known_y)`.

With n paired observations (xᵢ, yᵢ) and means x̄, ȳ:

    b  =  Σ (xᵢ − x̄)(yᵢ − ȳ)  /  Σ (xᵢ − x̄)²
    a  =  ȳ − b·x̄
    FORECAST.LINEAR(x, …)  =  a + b·x

The full mathematical treatment — that this is a fit and not an interpolation, that no
extrapolation guard exists, that the denominator vanishes exactly when the xᵢ are all equal
and *not* when the yᵢ are — is set out on the [FORECAST](FUNC.FORECAST.md) page and is not
repeated here. The name is the modern one: `LINEAR` distinguishes this member from the
`FORECAST.ETS` exponential-smoothing family introduced at the same time, which is why the
rename happened at all.

## Arguments

Three arguments, all required; the reference engine declares an arity of exactly 3.

| Argument | Meaning |
|---|---|
| `x` | The abscissa at which to evaluate the fitted line. |
| `known_y's` | The dependent array or range. |
| `known_x's` | The independent array or range. |

`x` is a scalar numeric slot; the two data arguments are consumed by scanning, not by
elementwise lifting. See [Coercion and lifting](../model/02-coercion-and-lifting.md).

## The relationship to FORECAST, stated precisely

This is a dotted-name/legacy-name pair, and the Handbook's rule for such pairs applies in full:
**a shared documentation page and matching wording are not evidence of a shared computation.**

What can be said, with its warrant attached:

1. **The documentation describes the same computation.** Both VBA reference pages give the
   equation as a + bx with the same definitions of a and b, and list the same three error
   conditions. That is a documented statement about intent.
2. **Microsoft marks the legacy method deprecated.** The `Forecast` page carries "This member
   is deprecated in Office 2016 and later versions"; the `Forecast_Linear` page carries no such
   note. That is a supersession statement.
3. **The supersession did not move the category.** The catalogue projection classifies *both*
   `FORECAST` and `FORECAST.LINEAR` under **Statistical functions**. `FORECAST` was not moved
   to **Compatibility**, unlike other superseded statistical surfaces such as `GAMMADIST` and
   `GAMMAINV`. Recorded here as a documentation-versus-catalogue mismatch.
4. **The reference engine treats them as one computation.** OxFunc routes both surfaces through
   the same `forecast_pair_kernel`. That is a fact about OxFunc.
5. **Nothing establishes that Excel does.** No Handbook measurement compares Excel's two
   surfaces against each other. The evidence record attached to this page says explicitly that
   its rows are `FORECAST`'s, that there is no `FORECAST.LINEAR` probe anywhere in the corpus,
   and that inheriting the figure to this surface would be a shared-kernel *inference* rather
   than a measurement.

Point 5 is the load-bearing one. Anyone migrating formulas from `FORECAST` to
`FORECAST.LINEAR` on the assumption that results are unchanged in the last bit is making an
assumption this Handbook cannot support.

## Result and edge cases

Returns `Number` — a scalar, never an array.

The edge-case inventory is identical in shape to `FORECAST`'s: two points fit exactly; all-x
identical is a zero denominator; constant y is a well-posed fit with zero slope; length
mismatch and empty input are documented as `#N/A`; large common offsets in x are the accuracy
hazard. See [FORECAST](FUNC.FORECAST.md#result-and-edge-cases). Whether the two surfaces
actually agree on any of these in Excel is unmeasured, which is precisely why the inventory is
worth probing on both surfaces rather than on one.

## Errors

As documented by Microsoft on the VBA reference page for this method:

| Error | Condition |
|---|---|
| `#VALUE!` | `x` is nonnumeric |
| `#N/A` | the `known_y` and `known_x` parameters are empty, or contain a different number of data points |
| `#DIV/0!` | the variance of the `known_x` parameters equals zero |

**The same documentation-versus-engine divergence as on the legacy surface.** The documented
`#N/A` row covers empty inputs as well as mismatched lengths; the shared reference kernel
returns `#N/A` only for a length mismatch and falls through to `#DIV/0!` when the common length
is zero. Documented answer and reference-engine answer differ on empty input, and the Handbook
has not observed which one Excel gives.

## Relationships

- **[FORECAST](FUNC.FORECAST.md)** — the legacy spelling, documented as deprecated. The precise
  statement of what is and is not known about the pair is in the section above.
- **`FORECAST.ETS`, `FORECAST.ETS.CONFINT`, `FORECAST.ETS.SEASONALITY`, `FORECAST.ETS.STAT`** —
  the exponential-smoothing time-series family that shares the `FORECAST.` prefix and shares
  nothing else. The `.LINEAR` suffix exists to keep these apart. A reader who wants seasonality
  handling wants the ETS members; a reader who wants a straight line wants this one.
- **`TREND`** — the array-valued generalisation to several predictors and several evaluation
  points.
- **`SLOPE` and `INTERCEPT`** — the same fit exposed as its two coefficients.
- **`LINEST`** — the full regression report.
- **`GROWTH`** — the exponential analogue, fitted in log-space.

## Numerical notes

The numerical content of this function is the numerical content of the least-squares slope, and
it is set out on the [FORECAST page](FUNC.FORECAST.md#numerical-notes): centered accumulation
versus the raw-sums form, the catastrophic cancellation that the raw form suffers when the xᵢ
share a large offset, the conditioning argument from Björck, and the fact that publishing as
a + b·x rather than ȳ + b·(x − x̄) changes the last bits without changing the mathematics.

One point belongs specifically here. Because the reference engine shares a kernel between the
two surfaces, **the reference engine cannot exhibit a difference between them** — any
comparison run against OxFunc will report identity by construction. That is a property of the
implementation, not evidence about Excel, and it means the shared-kernel design removes the
very difference a reader would most like measured. A probe that compares Excel's two surfaces
directly, with no reference engine in the loop, is the only thing that can settle it.

## What has not been checked

`EV-MISC-0007` names `FORECAST.LINEAR` as a subject, and it is the only evidence record in the
Handbook's collection that does — but read what it says about this surface. It states that
there is **no `FORECAST.LINEAR` probe anywhere in the corpus**; that the counted rows belong to
`FORECAST`; that this surface is named in a promotion headline and shares a kernel, so the
figure is inheritable only by shared-kernel argument, which the record calls an inference and
not a measurement; and that this surface's only row of its own is a pre-fix drift in an older
reconnaissance corpus.

So: **`FORECAST.LINEAR` has no per-surface Excel measurement in this record.** The honest
sentence is that the family was measured and this surface was not measured separately.

No Handbook vector suite exists for it either.

Inputs I would probe first, and why:

1. **`FORECAST` and `FORECAST.LINEAR` on identical data, compared to each other**, over a wide
   sweep. This is the single highest-value probe on the page: it needs no external oracle, it
   directly attacks the assumption everyone makes when migrating formulas, and either outcome
   is publishable — identity across a large sweep is real evidence for the pair, and any
   disagreement is a finding.
2. **The large-common-offset case** — date serials with a small spread — on this surface
   specifically, since the identification work behind the evidence record was done through the
   legacy spelling.
3. **The empty-input case**, to settle the documented-`#N/A`-versus-engine-`#DIV/0!`
   divergence recorded above.
4. **All-x-identical, and near-identical x differing by a few ulps**, distinguishing an exact
   zero-denominator test from a tolerance test.
5. **`FORECAST.LINEAR` against `SLOPE`·x + `INTERCEPT` and against `TREND`**, as metamorphic
   probes that reveal whether Excel shares code between these surfaces.
6. **Text and logical values inside the scanned ranges**, since the documentation states no
   scan policy for this function.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| dotted-name pair | A modern `NAME.QUALIFIER` surface alongside a legacy `NAME` surface |
| shared-kernel inference | Attributing one surface's measurement to another because an implementation shares code; an inference, not a measurement |
| centered form | Slope computed from deviations about the means |
| ETS family | The `FORECAST.ETS*` exponential-smoothing functions, unrelated to this one |
| supersession | A documented replacement relationship; here, documented without a category move |

## Sources

- Microsoft Learn, "WorksheetFunction.Forecast_Linear method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.forecast_linear>
  (the a + bx equation with the sample means named, and the three documented error conditions).
- Microsoft Learn, "WorksheetFunction.Forecast method (Excel)" — the legacy surface, including
  the deprecation note quoted in the comparison section. The shared worksheet-surface page at
  `support.microsoft.com` was not retrievable at curation time.
- Handbook evidence record `EV-MISC-0007` — including its explicit statement that no
  `FORECAST.LINEAR` probe exists in the corpus. The record's figures render mechanically beside
  this page; the prose deliberately does not restate them.
- Å. Björck, *Numerical Methods for Least Squares Problems* — the conditioning argument.
- Handbook, [FORECAST](FUNC.FORECAST.md) — the shared mathematics and numerical treatment.
- Handbook projections `data/functions/FUNC.FORECAST.LINEAR.json` (arity, category) and
  `data/presence/FUNC.FORECAST.LINEAR.json` (the shared `regression_forecast_family` module).
- OxFunc `crates/oxfunc_core/src/functions/regression_forecast_family.rs` at commit `473efa3` —
  the single `forecast_pair_kernel` both surfaces route through, and the `#N/A` / `#DIV/0!`
  ordering described under **Errors**.
