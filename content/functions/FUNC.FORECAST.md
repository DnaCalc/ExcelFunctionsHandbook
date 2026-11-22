---
schema: efh.function-page/v1
function_id: FUNC.FORECAST
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MISC-0007
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Forecast method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.forecast"
    role: "documented description, the a+bx equation, the three documented error conditions, and the deprecation note"
  - work: "Microsoft Learn — WorksheetFunction.Forecast_Linear method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.forecast_linear"
    role: "the replacement surface's documentation, for the wording comparison"
  - work: "Microsoft Support — FORECAST and FORECAST.LINEAR functions"
    locator: "https://support.microsoft.com/en-us/office/forecast-and-forecast-linear-functions-50ca49c9-7b40-4892-94e4-7ad38bbeda99"
    role: "the shared worksheet-surface page; not retrievable at curation time (the host refused the request)"
  - work: "Å. Björck, Numerical Methods for Least Squares Problems"
    locator: "chapters on the normal equations and their conditioning"
    role: "why the centered form of the slope is the right one and the raw-sums form is not"
  - work: "L. Knüsel and B. D. McCullough, published assessments of Excel's statistical procedures"
    locator: null
    role: "the standing external record on Excel's regression and distribution accuracy"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Notes
  - Page vocabulary
  - Sources
family: regression_forecast_family
role_in_family: >-
  The scalar single-predictor member: fits an ordinary least-squares line to one x/y pair of
  vectors and evaluates it at one new point, returning a number rather than an array.
---

## What it computes

`FORECAST(x, known_ys, known_xs)` fits a straight line to the paired data by ordinary least
squares and evaluates that line at `x`. Microsoft gives the model in exactly this form: the
equation is **a + bx**, where a is the intercept and b the slope, and where the sample means
are `AVERAGE(known_x's)` and `AVERAGE(known_y's)`.

Written out, with n paired observations (xᵢ, yᵢ) and means x̄, ȳ:

    b  =  Σ (xᵢ − x̄)(yᵢ − ȳ)  /  Σ (xᵢ − x̄)²
    a  =  ȳ − b·x̄
    FORECAST(x, …)  =  a + b·x

Three things are worth reading off that definition rather than assuming them.

**It is a fit, not an interpolation.** `FORECAST` does not pass through the data points and has
no notion of "nearest observation". If the requested `x` coincides with an observed xᵢ, the
answer is the fitted value at that abscissa, not yᵢ.

**There is no extrapolation guard.** The formula is evaluated at whatever `x` is given, inside
or outside the observed range. The word "forecast" in the name is a statement about intent, not
about a check the function performs.

**The denominator is the sum of squared deviations of x.** It vanishes exactly when every xᵢ is
identical — a vertical scatter with no leverage — and that is the case Microsoft documents as
`#DIV/0!`. It does not vanish when the *y*s are constant: a constant y with varying x is a
perfectly well-posed fit with slope zero.

The quantity b is also, up to scaling, the covariance over the variance: b = cov(x, y)/var(x)
with matching denominators, which is why `SLOPE`, `COVARIANCE.P`/`COVARIANCE.S`, and `CORREL`
all compute pieces of the same sum and why an implementation that shares those pieces is
tempting and dangerous — see **Numerical notes**.

## Arguments

Three arguments, all required; the reference engine declares an arity of exactly 3.

| Argument | Meaning |
|---|---|
| `x` | The abscissa at which to evaluate the fitted line. Documented as "the data point for which you want to predict a value". |
| `known_y's` | The dependent array or range. |
| `known_x's` | The independent array or range. |

`x` is a scalar numeric slot; the two data arguments are vector-shaped and consumed by
scanning rather than by elementwise lifting — this is not a lift kernel. Which values a scan
admits and which it skips is family policy, not engine law; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

The pairing is positional. Nothing checks that the two vectors are *meaningfully* paired; the
only check documented is that they contain the same number of data points.

## Result and edge cases

Returns `Number` — a scalar, never an array.

- **Two points** is the smallest fit with a defined slope, and it reproduces the line through
  them exactly (subject to rounding).
- **One point**, or any input where all xᵢ coincide, has a zero denominator. Documented:
  `#DIV/0!`.
- **Constant y, varying x** is fine: slope 0, intercept ȳ.
- **Length mismatch** between the two vectors is documented as `#N/A`.
- **Empty inputs** are documented as `#N/A` too — and here the documentation and the reference
  engine part company. See **Errors**.
- **Large common offsets** in x (dates are the everyday example: five-digit serial numbers with
  a one-digit spread) are the classic accuracy hazard for this function and are discussed
  under **Numerical notes**.
- **Arrays** are consumed as data, not lifted. `FORECAST` never spills.

## Errors

As documented by Microsoft on the VBA reference page for this method:

| Error | Condition |
|---|---|
| `#VALUE!` | `x` is nonnumeric |
| `#N/A` | `known_y's` and `known_x's` are empty, or contain a different number of data points |
| `#DIV/0!` | the variance of `known_x's` equals zero |

**A divergence worth recording.** The documented `#N/A` row covers two distinct situations —
mismatched lengths *and* empty inputs. The reference engine splits them: its kernel returns
`#N/A` for a length mismatch, and falls through to `#DIV/0!` when the common length is zero,
because the empty case reaches the zero-denominator test rather than a dedicated empty test.
So on empty input the documentation says `#N/A` and the reference engine says `#DIV/0!`. The
Handbook has not observed which one Excel produces; the point here is that the two available
statements disagree, and that this is exactly the kind of disagreement this Handbook exists to
publish rather than smooth over.

Errors inside the scanned ranges propagate under the shared coercion discipline.

## Relationships

- **[FORECAST.LINEAR](FUNC.FORECAST.LINEAR.md)** — the modern spelling. The two documentation
  pages describe the same computation in nearly identical words, and the reference engine
  routes both surfaces through one shared kernel. That is a statement about OxFunc, not about
  Excel: **documented sameness of wording is not evidence of identical bits**, and no Handbook
  measurement establishes that Excel's two surfaces agree. Proving they do would require
  comparing them directly, and that comparison has not been made here.
- **A supersession that did not complete.** Microsoft's VBA reference marks this method
  "deprecated in Office 2016 and later versions", yet the catalogue projection still classifies
  `FORECAST` under **Statistical functions** rather than **Compatibility** — unlike, say,
  `GAMMADIST` and `GAMMAINV`, which did move. Recording the mismatch: the documentation calls
  the method deprecated while the published category does not treat it as a compatibility
  surface.
- **`TREND`** — the array-valued generalisation: multiple predictors, multiple evaluation
  points, one call. `FORECAST` is `TREND` restricted to one predictor and one point, *as
  mathematics*. Upstream's record is explicit that it is not the same code path in Excel; see
  **What has not been checked**.
- **`SLOPE` and `INTERCEPT`** — the two halves of the same fit, exposed separately.
  `SLOPE(y,x)*x0 + INTERCEPT(y,x)` is `FORECAST(x0,y,x)` mathematically, and is a different
  arithmetic expression, so agreement in the last bit is not to be assumed.
- **`LINEST`** — the full regression report. Excel's own `FORECAST` is documented by upstream
  as *not* running the `LINEST` pipeline.
- **`FORECAST.ETS` family** — exponential-smoothing time-series forecasting. Same first word,
  entirely different method; a reader looking for seasonality wants those, not this.
- **`GROWTH`** — the exponential analogue, fitted in log-space.

## Numerical notes

**Centered versus raw sums.** The slope can be computed two ways that are algebraically
identical and numerically nothing alike:

    centered:  Σ(xᵢ − x̄)(yᵢ − ȳ) / Σ(xᵢ − x̄)²
    raw:       (nΣxᵢyᵢ − ΣxᵢΣyᵢ) / (nΣxᵢ² − (Σxᵢ)²)

The raw form is the one in most textbooks and most first implementations, and it is a
catastrophic-cancellation machine: when the xᵢ share a large offset — Excel date serials,
timestamps, anything with five significant digits before the interesting ones start —
`nΣxᵢ²` and `(Σxᵢ)²` are enormous and nearly equal, and their difference retains only the
low-order digits that survived rounding. The relative error grows like the square of the ratio
of the mean to the spread. This is the standard result and it is why the centered form, or a
Welford-style updating formula, is the right choice. Björck's treatment of the normal equations
is the reference; the same lesson governs `VAR.S` and `STDEV.S`.

The two-pass centered form needs the mean before the deviations, so it reads the data twice.
That is a real cost only for very long ranges, and it buys a conditioning improvement that no
amount of care in the raw form can recover.

**What upstream identified for Excel.** The evidence record attached to this page states a
structural conclusion: Excel's `FORECAST` is *not* the `LINEST` pipeline but a centered
plain-double kernel — forward sums to means, one fused loop accumulating both the covariance
and the variance, and publication in the intercept form a + b·x. That identification is
upstream's, it names its own corpus, and the corpus was built alongside the hypothesis rather
than held out — the record says so in as many words. It is a strong structural claim with a
weak-blindness caveat, and both halves travel together.

**Why the fused loop matters.** Accumulating Σ(xᵢ − x̄)(yᵢ − ȳ) and Σ(xᵢ − x̄)² in the same
pass, in the same order, with the same intermediate precision, is a different rounding sequence
from computing them in two loops — even though it is the same mathematics. For a
compatibility-oriented implementation the loop structure is part of the specification, not an
optimisation detail.

**The evaluation step.** Publishing as `a + b·x` rather than as `ȳ + b·(x − x̄)` is another
choice that changes the last bits, and the two are not interchangeable for compatibility even
though they agree mathematically. The centered evaluation form is more accurate; the intercept
form is what upstream identifies.

## What has not been checked

`EV-MISC-0007` names `FORECAST` as a subject, and it is the only evidence record in the
Handbook's collection that does. Read what it actually supports before leaning on it: it
publishes a per-surface count on a corpus that upstream describes as discovery plus adversarial
inputs, not as a held-out test, and the adversarial rows were designed *after* the kernel
hypothesis existed. That makes it a divergence-design set — good for finding disagreement,
weaker as evidence that none remains. The record also states plainly that the same figure is
`FORECAST`'s and not `FORECAST.LINEAR`'s.

No Handbook vector suite exists for `FORECAST`. Nothing in this record establishes agreement
with Excel outside the corpus upstream measured, and the Handbook has performed no measurement
of its own.

Inputs I would probe first, and why:

1. **A large common offset in x with a small spread** — the date-serial case, and the exact
   input on which the centered and raw formulas diverge by orders of magnitude. If a candidate
   implementation is going to fail, it fails here first, and the residual size identifies which
   formula produced it.
2. **The empty-input case**, directly. The documentation says `#N/A`, the reference engine's
   kernel reaches `#DIV/0!`. One probe settles which Excel returns, and it settles a stated
   divergence rather than adding a data point.
3. **Length mismatch with one vector empty** — the two documented `#N/A` clauses overlapping,
   which is where an implementation's ordering of guards becomes visible.
4. **All-x-identical with n = 1 and with n > 1** — both `#DIV/0!` by the documented rule, but
   through different paths in any implementation that special-cases the singleton.
5. **Near-constant x rather than exactly constant** — a spread of a few ulps. This distinguishes
   an exact `den == 0` test from a tolerance test, and the two give different answers on real
   data.
6. **`FORECAST` against `SLOPE`·x + `INTERCEPT`, and against `TREND`, on identical data** —
   metamorphic probes that need no oracle to be informative. Any disagreement proves the
   surfaces are separate computations in Excel, which is a publishable structural fact.
7. **Text and logical values inside the scanned ranges**, to pin the scan policy, which the
   documentation does not state for this function.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `FORECAST` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-FORECAST-001: FORECAST and FORECAST.LINEAR use plain binary64 sample-mean OLS (not TREND's QR pipeline) and coerce scalar pairs

*Detail for matching Excel's bits; measured; recorded 2026-11-22.* Applies to FORECAST, FORECAST.LINEAR.

In Excel 16.0 builds 20131 and 20527, FORECAST(x, known_ys, known_xs) and FORECAST.LINEAR(x,
known_ys, known_xs) validate singleton blank/error arguments in formula order, return #N/A on
shape mismatch, coerce direct scalar known_ys/known_xs while skipping non-numeric elements in
ranges/arrays, and compute a + b*x using plain binary64 sample means and centered sums (unlike
TREND, which routes through LINEST's stored-PC64 Householder QR pipeline and gives different
low bits), returning #DIV/0! when n == 0 or sum((x - mean_x)^2) == 0. Matches 2,984/2,984
full-bank and 70/70 pinned FORECAST rows, and 2,910/2,910 full-bank FORECAST.LINEAR rows.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=FORECAST(4,Data!A1:A3,Data!E1:E3)` | as typed | `5.0` (`0x4014000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=FORECAST.LINEAR(4,Data!A1:A3,Data!E1:E3)` | as typed | `5.0` (`0x4014000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `FORECAST(0, {2;4.1;5.9;8.05}, {1;2;3;4})` | x `0.0` (`0x0000000000000000`); known_ys `{2;4.1;5.9;8.05}`; known_xs `{1;2;3;4}` (cells) | `0.024999999999999467` (`0x3f99999999999900`) | Excel 16.0 build 20131, Windows, AMD Ryzen 7 4800H; bitness not recorded |
| `=FORECAST.LINEAR(2,"",2)` | as typed | `#DIV/0!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Measured across Excel 16.0 build 20131 (70/70 pinned FORECAST rows in
witnesses/FORECAST.json) and Excel 16.0.20527 (2,984 FORECAST and 2,910 FORECAST.LINEAR
full-bank rows, prefix_metadata rows 89-90, and fn4_questions ql136-145).

Evidence:

- DnaCalc `crates/calc-func/src/ported/regression_forecast_family.rs`, forecast_pair_kernel at `39c98c7193` — direct binary64 OLS distinct from TREND's QR pipeline
- DnaCalc `crates/calc-func/src/catalogue/statistics.rs`, forecast; forecast_legacy; forecast_linear at `39c98c7193` — PairScalars::Coerce handling
- DnaCalc `crates/calc-func/witness-status.tsv`, FORECAST; FORECAST.LINEAR at `39c98c7193` — 2,984 + 2,910 full-bank and 70/70 pinned rows matched
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/prefix_metadata.capture.json`, sheet prefix_metadata row 89 column 1 at `39c98c7193` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/prefix_metadata.capture.json`, sheet prefix_metadata row 90 column 1 at `39c98c7193` (readings)
- DnaCalc `crates/calc-func/tests/witnesses/FORECAST.json`, oc8 at `39c98c7193` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-list-controls.capture.json`, sheet ql137 row 20 column 10 at `39c98c7193` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| ordinary least squares | The fit minimising Σ(yᵢ − a − b·xᵢ)² |
| centered form | Slope computed from deviations about the means |
| raw-sums form | Slope computed from Σx, Σy, Σx², Σxy without centering |
| catastrophic cancellation | Loss of all significant digits when two nearly equal large quantities are subtracted |
| fused loop | Accumulating covariance and variance in one pass, which fixes the rounding order |
| divergence-design corpus | Inputs constructed after a hypothesis to try to break it; not a blind test |

## Sources

- Microsoft Learn, "WorksheetFunction.Forecast method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.forecast>
  (the a + bx equation with the sample means named, the three documented error conditions, and
  the note that the method is deprecated in Office 2016 and later). The shared worksheet-surface
  page at `support.microsoft.com` was not retrievable at curation time.
- Microsoft Learn, "WorksheetFunction.Forecast_Linear method (Excel)" — the replacement
  surface, whose Remarks are near-identical in wording.
- Handbook evidence record `EV-MISC-0007` — the structural identification of Excel's kernel
  and the corpus caveats stated above. The record's own figures render mechanically beside this
  page; the prose deliberately does not restate them.
- Å. Björck, *Numerical Methods for Least Squares Problems* — conditioning of the normal
  equations, the basis for the centered-versus-raw discussion.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — scan versus direct
  argument policy and error propagation.
- Handbook projections `data/functions/FUNC.FORECAST.json` (arity, category, custom coercion
  and kernel classes) and `data/presence/FUNC.FORECAST.json` (the shared
  `regression_forecast_family` module, held jointly with `FORECAST.LINEAR`, `GROWTH`, `LINEST`,
  `LOGEST` and `TREND`).
- OxFunc `crates/oxfunc_core/src/functions/regression_forecast_family.rs` at commit `473efa3` —
  the shared `forecast_pair_kernel`, its centered accumulation, and the `#N/A` / `#DIV/0!`
  ordering described under **Errors**.
