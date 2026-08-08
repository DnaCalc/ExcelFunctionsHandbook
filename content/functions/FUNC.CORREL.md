---
schema: efh.function-page/v1
function_id: FUNC.CORREL
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: CORREL function"
    locator: "https://support.microsoft.com/en-us/office/correl-function-995dcef7-0c0a-4bed-a3fb-239d7b68ca92"
    role: "retrieved for this pass; syntax, the ignore-text-logicals-blanks rule, the #N/A length rule, and the #DIV/0! zero-spread rule"
  - work: "Welford, 'Note on a Method for Calculating Corrected Sums of Squares and Products', Technometrics 4 (1962)"
    locator: "the updating recurrence for sums of squares and cross-products"
    role: "the numerically stable one-pass alternative to the textbook sum-of-products formula"
  - work: "Chan, Golub & LeVeque, 'Algorithms for Computing the Sample Variance: Analysis and Recommendations', The American Statistician 37 (1983)"
    locator: "the comparison of naive, two-pass, and updating formulations"
    role: "the standard analysis of why the textbook correlation formula loses accuracy and by how much"
  - work: "Welinder, notes on Excel's statistical functions (Gnumeric)"
    locator: "Gnumeric documentation, statistical accuracy appendix"
    role: "the external record of Excel's historical use of unstable moment formulas; named as tradition, not as evidence about this surface"
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
family: correl_fn
role_in_family: >-
  Its own module in the reference engine, but a thin one: the whole computation is a call into the
  shared paired-statistics helper that also serves COVARIANCE, SLOPE, INTERCEPT and RSQ.
---

# CORREL

## What it computes

`CORREL(array1, array2)` returns the **Pearson product-moment correlation coefficient** of two
paired data sets.

For pairs `(x₁,y₁), …, (xₙ,yₙ)` with means `x̄` and `ȳ`,

    r  =  Σ (xᵢ − x̄)(yᵢ − ȳ)  /  √( Σ (xᵢ − x̄)² · Σ (yᵢ − ȳ)² )

equivalently `r = cov(x,y) / (σ_x σ_y)`, and equivalently again the cosine of the angle between the
two mean-centred data vectors. Microsoft's page states the same formula with `x̄` and `ȳ` given as
`AVERAGE(array1)` and `AVERAGE(array2)`.

Note that the `n` versus `n − 1` question — population or sample divisor — **does not arise**. The
same divisor appears in the numerator and in both factors of the denominator and cancels
identically. `CORREL` is therefore the one member of this cluster with no population/sample split:
there is no `CORREL.P` and there could not be one.

Range and interpretation. By Cauchy–Schwarz, `r ∈ [−1, 1]` exactly, with `|r| = 1` if and only if
the points lie on a straight line. `r = +1` is a perfectly increasing line, `r = −1` a perfectly
decreasing one, `r = 0` no *linear* association. The last qualifier is the one that costs people
money: `r` measures linear association only, and a perfect parabola through symmetric `x` values has
`r = 0`.

Invariance. `r` is unchanged by any increasing affine transformation of either variable —
`r(ax+b, cy+d) = r(x,y)` for `a, c > 0` — and flips sign when exactly one of `a`, `c` is negative.
This scale invariance is what distinguishes `r` from `COVARIANCE.P`, whose value carries the units
of `x` times the units of `y` and is therefore uncomparable across data sets.

Degenerate cases. `r` is undefined when either variable has zero spread — the denominator vanishes
because every point of that variable sits on its own mean — and undefined when fewer than two pairs
survive. Both surface as errors rather than as values; see below.

## Arguments

`CORREL(array1, array2)` — two arguments, both required; the registry records an arity of exactly 2.

| Argument | Meaning (as documented) |
|---|---|
| `array1` | "A range of cell values" |
| `array2` | "A second range of cell values" |

Pairing is positional. Microsoft's page does not state the traversal order for a rectangular range,
and this page will not invent one; the reference engine flattens both arguments row-major and zips
them, so a `2×3` block pairs with a `3×2` block position by position rather than by geometry. That
is an implementation fact about the reference engine and a good probe.

The documented filtering rule is the important one: **text, logical values and empty cells inside
an array or reference argument are ignored, and cells containing zero are included.** This is a
range-scan policy, not a coercion policy — the text `"3"` in a cell does not become the number `3`
here, it is skipped. See [Coercion and lifting](../model/02-coercion-and-lifting.md) for why the
same value behaves differently as a direct argument than as a scanned cell.

## Result and edge cases

Returns `Number` in `[−1, 1]`.

Documented behaviour:

- **Different numbers of data points → `#N/A`.** Not `#VALUE!`. This is the length check, and it is
  performed on the *raw* extents, before the ignore-rule has removed anything.
- **Either array empty, or a standard deviation of zero → `#DIV/0!`.** A constant column is not a
  correlation of zero; it is an error.
- Text, logicals and blanks inside the ranges are ignored; zeros are included.

Behaviour the documentation does not settle:

- **Where the ignore-rule sits relative to the pairing.** If `x₃` is text and `y₃` is a number,
  is the third pair dropped entirely, or is `y₃` retained and re-paired with `x₄`? The only
  defensible answer is pairwise deletion — drop the pair — and the reference engine does exactly
  that, but Microsoft's page does not say so and the Handbook has not checked Excel.
- **Whether the length check counts cells or surviving numbers.** The reference engine compares
  flattened extents before filtering, so two ranges of equal size but different text content pass
  the check; two ranges of different size fail it even if the same number of numeric values
  survive. Undocumented.
- **Whether the result is clamped to `[−1, 1]`.** Floating-point evaluation of the ratio can
  produce a value a few ulps outside the interval on nearly collinear data. The reference engine
  does not clamp. Excel's behaviour is unknown, and `CORREL` returning `1.0000000000000002` would
  be a visible and reportable defect.

Errors in a scanned cell propagate: an error value inside either range becomes the result, which is
consistent with the general rule that coercion never silently discards a worksheet error
([chapter 02](../model/02-coercion-and-lifting.md)).

## Errors

| Error | Condition | Source |
|---|---|---|
| `#N/A` | `array1` and `array2` have different numbers of data points | documented |
| `#DIV/0!` | Either array is empty, or the standard deviation of either is zero | documented |
| propagated | An error value in a scanned cell surfaces as the result | reference engine; not documented |
| `#VALUE!` | Wrong argument count, or an argument kind the surface cannot flatten | reference engine; not documented |

The reference engine also produces `#DIV/0!` when fewer than two numeric pairs survive filtering,
which subsumes the documented empty-array row.

## Relationships

- **[PEARSON](FUNC.PEARSON.md)** — documented as computing the same coefficient. Excel ships two
  spellings of one statistic. Neither is deprecated, neither is in the Compatibility category, and
  the Handbook has **not** checked whether they return identical bits; a shared definition is not a
  shared implementation. Comparing them is one of the cheapest interesting probes in the whole
  statistical lane.
- **[RSQ](FUNC.RSQ.md)** — documented as `r²`. The reference engine literally computes `RSQ` as
  `CORREL` squared, which means any error in `CORREL` is doubled in relative terms in `RSQ`, and
  `RSQ` cannot be more accurate than `CORREL`. Whether Excel composes them the same way is
  unchecked, and `RSQ(x,y)` against `CORREL(x,y)^2` is a good identity probe.
- **[COVARIANCE.P](FUNC.COVARIANCE.P.md) / [COVARIANCE.S](FUNC.COVARIANCE.S.md)** — the unnormalized
  cousins. `CORREL` is covariance divided by the product of the standard deviations, and the
  population/sample choice cancels out of that ratio.
- **[SLOPE](FUNC.SLOPE.md)** — related by `slope = r · s_y / s_x`. `SLOPE` and `CORREL` share the
  reference engine's paired-statistics helper.
- **[FISHER](FUNC.FISHER.md)** — the variance-stabilizing transform `artanh(r)` used to build
  confidence intervals for a correlation. It is the natural next function after this one, and its
  own page is where the `r → ±1` sensitivity gets its proper treatment.
- **Confused with**: causation, and with `SLOPE`. `r` is unitless and symmetric in its arguments;
  `SLOPE` is neither.

## Numerical notes

Correlation is the classic worked example in the numerical-statistics literature, and for a good
reason: the textbook formula is a subtraction of two nearly equal large numbers.

**The unstable form.** Expanding the centred sums gives

    Σ(xᵢ−x̄)(yᵢ−ȳ) = Σxᵢyᵢ − (Σxᵢ)(Σyᵢ)/n

which is attractive because it accumulates in one pass with five running sums. It is also
catastrophic when the data have a large mean relative to their spread: both terms grow like `n·x̄·ȳ`
while their difference is only `n·cov`, so the relative error is amplified by roughly
`|x̄ȳ| / |cov|`. On data like `(10⁸ + small)` the correlation can come out as `0`, as `2`, or as
`#DIV/0!` from a negative sum of squares. Chan, Golub & LeVeque (1983) is the standard analysis of
exactly this failure, and the Gnumeric/Welinder tradition of auditing spreadsheet statistics grew
out of finding this formula in shipped products.

**The stable forms.** Two passes — compute the means, then accumulate the centred products — is
accurate and simple, and is what the reference engine does. Welford-style updating recurrences give
the same accuracy in one pass and are the right choice when the data cannot be revisited.
Pairwise or compensated (Kahan) summation of the centred products buys back the accumulation error
for large `n`.

**A detail worth its own sentence.** The reference engine forms the denominator as
`√(var_x) · √(var_y)`, not as `√(var_x · var_y)`. The two differ in the last bit in general, and the
second can overflow or underflow where the first does not. Which one Excel uses is unknown and is a
last-bit question a vector suite would settle immediately.

**Overflow.** With the centred two-pass form, `Σ(xᵢ−x̄)²` can still overflow for data near the
double-precision ceiling. Scaling by the maximum absolute deviation before accumulating — the same
trick used in a careful `hypot` — is the standard defence and costs one extra pass.

**Range guarantee.** Mathematically `|r| ≤ 1`; numerically, the ratio of independently rounded
quantities is not guaranteed to satisfy it. An implementation that intends to promise the bound has
to clamp explicitly, and clamping is a decision to record rather than a neutral cleanup: it changes
what `ACOS(CORREL(...))` does at the boundary.

None of this is a claim about how Excel computes `CORREL`. It is the map of where the errors live
for anyone measuring it.

## What has not been checked

No Handbook vector suite exists for `CORREL`; `vectors/` publishes nothing at this revision. No
Excel-comparison evidence record names this surface in its subjects. **Nobody has checked this
function against Excel within the Handbook's record.**

What exists: the documented rules quoted above, the projected registry metadata, and the
mechanically rendered B1 battery beside this page — which reports the reference engine's answers on
a fixed edge-input row set, not Excel's. The reference engine's own module carries no unit tests of
its own; the shared paired-statistics helper it calls does.

Inputs worth probing first:

1. **A high-mean, low-variance pair** — `x = 10⁸ + {1,2,3,4}`, `y = 10⁸ + {2,4,6,8}`, whose true
   `r` is exactly `1`. This is the single most informative probe on the page: the naive one-pass
   formula fails visibly here and the two-pass formula does not, so one cell distinguishes the two
   families of implementation.
2. **The same pair scaled to `10¹⁵`** — pushes the naive form past complete cancellation.
3. **Perfectly collinear data** — `y = 2x + 1` on ordinary values. Does Excel return exactly `1.0`,
   or `0.9999999999999999`? And does it ever return a value greater than `1`?
4. **A constant `y` column** — the documented `#DIV/0!`, and worth confirming it is not `0`.
5. **A single pair, and zero surviving pairs** — the boundary of the documented empty-array rule.
6. **Ranges of the same cell count but different shape** — `A1:C2` against `E1:F3`. Distinguishes
   row-major flattening from a geometric pairing, and neither is documented.
7. **Text and blanks in one column only**, at matched and unmatched positions — pins whether
   deletion is pairwise or per-column, which the documentation leaves open and which changes the
   answer, not just the error.
8. **An error value in one cell of one range** — confirms propagation and which error wins if two
   different errors appear.
9. **`CORREL` against `PEARSON` and against `RSQ`** on every case above — three surfaces that
   should agree by definition, and any disagreement is a finding in itself.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| pairwise deletion | Dropping a whole `(x,y)` pair when either member is non-numeric |
| centred | Mean-subtracted, as in `xᵢ − x̄` |
| naive one-pass formula | `Σxy − (Σx)(Σy)/n`; fast, and catastrophic on high-mean data |
| two-pass formula | Compute the means first, then accumulate centred products |
| B1 battery | The fixed edge-input row set the Handbook runs against the reference engine, rendered mechanically beside this page |

## Sources

- Microsoft, "CORREL function" —
  <https://support.microsoft.com/en-us/office/correl-function-995dcef7-0c0a-4bed-a3fb-239d7b68ca92>
  (retrieved for this pass: syntax, the ignore-text-logicals-and-blanks rule with zeros included,
  the `#N/A` differing-length rule, the `#DIV/0!` empty-or-zero-standard-deviation rule, and the
  formula with `x̄`, `ȳ` as `AVERAGE(array1)`, `AVERAGE(array2)`).
- Chan, Golub & LeVeque, "Algorithms for Computing the Sample Variance", *The American
  Statistician* 37 (1983) — the error analysis of naive versus two-pass versus updating formulas.
- Welford, "Note on a Method for Calculating Corrected Sums of Squares and Products",
  *Technometrics* 4 (1962) — the stable updating recurrence.
- Handbook call-model chapters [01 The value universe](../model/01-value-universe.md) and
  [02 Coercion and lifting](../model/02-coercion-and-lifting.md) — the direct-argument versus
  range-scan distinction that governs the ignore rule.
- Handbook projections `data/functions/FUNC.CORREL.json`, `data/presence/FUNC.CORREL.json`.
- OxFunc `crates/oxfunc_core/src/functions/correl_fn.rs` and
  `crates/oxfunc_core/src/functions/paired_stats_common.rs` at commit `473efa3` — the row-major
  zip, the `#N/A` extent check, pairwise deletion, the two-pass centred accumulation, and the
  `√var_x · √var_y` denominator association, read as implementation facts about the reference
  engine.
