---
schema: efh.function-page/v1
function_id: FUNC.CONFIDENCE.NORM
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: CONFIDENCE.NORM function"
    locator: "https://support.microsoft.com/en-us/office/confidence-norm-function-7cec58a6-85bb-488d-91c3-63828d4fbfd4"
    role: "retrieved for this pass; the syntax, the alpha/standard_dev/size descriptions, the size-truncation rule and the four documented error conditions"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 26 (Probability Functions), 26.2.22-26.2.23"
    role: "the normal quantile function and its rational approximations - the object this function inverts"
  - work: "Wichura, 'Algorithm AS 241: The Percentage Points of the Normal Distribution', Applied Statistics 37 (1988)"
    locator: "AS 241, PPND16"
    role: "the standard high-accuracy normal quantile algorithm a careful implementation would use"
  - work: "Welinder, notes on Excel's statistical functions (Gnumeric)"
    locator: "Gnumeric documentation, statistical accuracy appendix"
    role: "the long-running external record of accuracy defects in Excel's statistical lane; named as the reference tradition, not as evidence about this surface"
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
family: normal_log_family
role_in_family: >-
  The normal-quantile consumer of the family: it does no distribution arithmetic of its own beyond
  one call to the standard normal inverse, so its accuracy is the inverse's accuracy scaled by
  sigma over the square root of the sample size.
---

# CONFIDENCE.NORM

## What it computes

`CONFIDENCE.NORM(alpha, standard_dev, size)` returns the **half-width** of a two-sided confidence
interval for a population mean, under the assumption that the population standard deviation is
known and the sampling distribution of the mean is normal.

Writing `Φ` for the standard normal cumulative distribution function and `z_q = Φ⁻¹(q)` for its
quantile,

    CONFIDENCE.NORM(α, σ, n)  =  z_{1 − α/2} · σ / √n

and the interval Microsoft's page states is

    x̄ ± CONFIDENCE.NORM(α, σ, n)

where `x̄` is the sample mean. The confidence level is `100·(1 − α)%`, so `α = 0.05` gives the
familiar 95% interval and `z_{0.975} ≈ 1.959963985…`.

Three things follow from the formula and are worth stating plainly:

1. **The function never sees your data.** It takes a spread and a count, not a sample. It is a
   scaling of one normal quantile — the entire mathematical content is `Φ⁻¹`.
2. **It returns a half-width, not an interval.** Readers who expect a pair of endpoints get a
   single number; the endpoints have to be assembled by hand.
3. **`σ` is treated as known.** That is the modelling assumption that separates this function from
   [CONFIDENCE.T](FUNC.CONFIDENCE.T.md), which uses a Student's t quantile with `n − 1` degrees of
   freedom because the spread was estimated from the sample.

Domain and range. The mathematical function is defined for `α ∈ (0, 1)`, `σ > 0`, `n ≥ 1`, and is
positive on all of it. It is decreasing in `α` (a higher confidence level widens the interval),
linear and increasing in `σ`, and decreasing in `n` like `n^(−1/2)` — the familiar square-root law
that makes halving an interval cost four times the sample. As `α → 0⁺` the value diverges, because
`Φ⁻¹(q) → +∞` as `q → 1⁻`; as `α → 1⁻` it tends to `0`.

## Arguments

`CONFIDENCE.NORM(alpha, standard_dev, size)` — three arguments, all required; the reference
engine's registry records an arity of exactly 3.

| Argument | Meaning (as documented) | Admissible values |
|---|---|---|
| `alpha` | "The significance level used to compute the confidence level. The confidence level equals 100*(1 - alpha)%" | `0 < alpha < 1`, per the documented error rule |
| `standard_dev` | "The population standard deviation for the data range and is assumed to be known" | `standard_dev > 0`, per the documented error rule |
| `size` | "The sample size" | `size ≥ 1` after truncation, per the documented error rule |

The commonly misunderstood position is the first. `alpha` is the significance level, **not** the
confidence level: a reader who wants 95% must pass `0.05`, and passing `0.95` silently produces the
90%-two-tailed half-width instead of an error. Nothing in the value or the type catches that
mistake.

`size` is documented as truncated when it is not an integer, so a sample size of `100.9` is used as
`100`. The reference engine implements the same truncation and rejects a truncated size below `1`.

All three are numeric slots subject to ordinary to-number coercion; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number` — a positive half-width.

Documented boundary behaviour, from Microsoft's page:

- Non-numeric argument → `#VALUE!`.
- `alpha ≤ 0` or `alpha ≥ 1` → `#NUM!`. Both ends are closed against you: `alpha = 0` would be an
  infinite interval and `alpha = 1` a degenerate one, and neither is admitted.
- `standard_dev ≤ 0` → `#NUM!`. A zero spread is rejected rather than returning a zero width.
- `size` non-integer → truncated; `size < 1` → `#NUM!`.

Behaviour the documentation does not settle, and this page will not invent: what happens for
non-finite numeric inputs, and whether `size` truncation is toward zero or toward negative infinity
for negative inputs (the two agree only on non-negatives, and negative sizes are rejected anyway,
so the question survives only as an ordering-of-checks question). The reference engine rejects
non-finite `standard_dev` and `size` with `#NUM!` and truncates toward zero; that is an
implementation fact about the reference engine, not a statement about Excel.

Empty, missing and error arguments follow the shared call model
([chapter 02](../model/02-coercion-and-lifting.md)). One asymmetry inside the reference engine is
worth recording because it is visible in the mechanically rendered battery beside this page: on the
all-empty call this surface and `CONFIDENCE.T` do not agree, because the two functions sit in
different modules with different empty-argument handling. Whether Excel makes the same distinction
is unchecked.

Array arguments: the projection records `lift_broadcast_profile: surface_native` for this surface,
while the legacy `CONFIDENCE` spelling is recorded with an explicit by-index lift over its first
three positions. That the modern and legacy spellings carry different lift profiles is a structural
fact of the reference engine's registry and is the sort of difference an alias is usually assumed
not to have.

## Errors

As documented on Microsoft's `CONFIDENCE.NORM` page:

| Error | Condition |
|---|---|
| `#VALUE!` | Any argument is nonnumeric |
| `#NUM!` | `alpha ≤ 0` or `alpha ≥ 1` |
| `#NUM!` | `standard_dev ≤ 0` |
| `#NUM!` | `size < 1` (after truncation) |

The reference engine adds `#NUM!` for non-finite `standard_dev` or `size`; that row is an
implementation fact, not a documented one. Error values in any argument propagate under the shared
coercion rules.

## Relationships

- **[CONFIDENCE](FUNC.CONFIDENCE.md)** — the legacy spelling. Excel's Compatibility category keeps
  it, and OxFunc's statistical-distribution contract states that `CONFIDENCE` and
  `CONFIDENCE.NORM` share the same current-baseline normal-confidence kernel. That is a statement
  about the reference engine. **It is not proof that Excel computes the two identically**: proving
  identity of two Excel surfaces requires probing Excel, and the Handbook's evidence layer records
  exactly that kind of work being done for other legacy/modern pairs — and records, for the pairs
  where it was done, that no inverse pair has ever been proven identical. Treat "legacy alias" as a
  hypothesis with a documented motive, not as a measured fact.
- **[CONFIDENCE.T](FUNC.CONFIDENCE.T.md)** — the sibling for an *estimated* standard deviation. The
  two differ by exactly one substitution: `z_{1−α/2}` becomes `t_{1−α/2, n−1}`. Because
  `t_{q,ν} > z_q` for every finite `ν`, `CONFIDENCE.T` is always the wider of the two on the same
  inputs, converging to `CONFIDENCE.NORM` as `n → ∞`. Choosing the normal form when the spread came
  from the sample understates the interval, and for small `n` it understates it badly.
- **[NORM.S.INV](FUNC.NORM.S.INV.md)** — the quantile this function calls.
  `CONFIDENCE.NORM(α, σ, n)` and `NORM.S.INV(1 − α/2) * σ / SQRT(n)` are the same expression; the
  second is the one to use when you want to see the pieces.
- **[Z.TEST](FUNC.Z.TEST.md)** — the hypothesis test that pairs with this interval.
- **Confused with**: `CONFIDENCE.NORM` is not a margin *of a proportion*. Applying it to a
  proportion requires `σ = √(p(1−p))`, which the function will not compute for you.

## Numerical notes

The whole numerical difficulty is `Φ⁻¹`, and it is a genuinely hard function to evaluate well.

- **Why it is hard.** `Φ⁻¹(q)` has infinite slope at both ends of `(0,1)` and the naive route —
  invert `erf` by iteration on a series for `Φ` — loses relative accuracy in the tails, where the
  quantity of interest is large and the probability that determines it is tiny. Every digit of `q`
  near `1` is worth a great deal of `z`, so cancellation in forming `1 − α/2` is a real hazard.
- **The cancellation this function walks straight into.** The argument passed to the quantile is
  `1 − α/2`. For small `α` that expression is a subtraction of a small number from `1`: the result
  is representable, but every bit of `α` below the `2^−53` relative scale of `1` is discarded before
  the quantile ever sees it. A careful implementation for very small `α` would work in the
  complementary variable — evaluate the upper-tail quantile at `α/2` directly — rather than forming
  `1 − α/2` at all. This is the same complement-staging idea that OxFunc records as decisive for
  the chi-square and F inverses, where it invert the published right-tail surface directly instead
  of inverting the CDF at `1 − p`.
- **The literature.** Abramowitz & Stegun chapter 26 gives the classical rational approximations
  (26.2.22, 26.2.23) with their absolute-error bounds — good enough for tables, not for double
  precision. The modern standard is Wichura's AS 241 (`PPND16`), accurate to about 16 significant
  digits over the whole open interval; Moro's and Acklam's approximations are the usual fast
  alternatives, and Cephes and Boost both carry well-tested `ndtri`/`erfc_inv` routines. Anyone
  implementing this function is really choosing among those.
- **The rest is trivial and still worth care.** `σ/√n` is one square root and one division. Forming
  `z · σ / √n` as `(z * σ) / sqrt(n)` and as `z * (σ / sqrt(n))` differ in the last bit; a
  compatibility implementation has to pick the same association Excel picks, and the Handbook does
  not know which that is.
- **Truncation before the square root.** Because `size` is truncated first, the divisor is
  `√⌊n⌋`, not `√n`. An implementation that truncates after taking the root would agree on integer
  inputs and disagree everywhere else — a bug that hides from every integer test case.

Do not read any of this as a claim about Excel's internal algorithm. What the Handbook can say is
that this surface's accuracy is exactly the accuracy of whatever normal quantile sits underneath
it, and that nobody has measured that here.

## What has not been checked

No Handbook vector suite exists for `CONFIDENCE.NORM`; `vectors/` publishes nothing at this
revision, so no suite-scoped claim exists for this surface. No Excel-comparison evidence record
lists `CONFIDENCE.NORM` in its subjects. Nobody has checked this function against Excel within the
Handbook's record.

What does exist: the projected registry metadata, the mechanically rendered B1 battery beside this
page (reference-engine outcomes on a fixed edge-input row set, not Excel), and OxFunc's
statistical-distribution contract asserting that `CONFIDENCE` and `CONFIDENCE.NORM` share a kernel.
The battery's rows are the reference engine's answers; the Handbook has not compared any of them to
Excel.

Inputs worth probing first, and why:

1. **`CONFIDENCE.NORM(0.05, 1, 1)`** — the smallest legal sample. It should return `z_{0.975}`
   exactly, which makes it a direct read-out of Excel's normal quantile at `0.975` with no other
   arithmetic in the way. This is the single cheapest probe of the substrate and should be run
   first.
2. **`CONFIDENCE.NORM(α, 1, 1)` swept over tiny `α`** — `1e-8`, `1e-12`, `1e-15`, `1e-17`. This is
   where `1 − α/2` collapses to `1` and the answer either saturates or errors. It distinguishes an
   implementation that works in the complement from one that does not, and it is the most
   informative sweep on the page.
3. **`CONFIDENCE.NORM(0.05, 1, 100.9)` against `…, 100)`** — confirms truncation happens before the
   square root, not after.
4. **`CONFIDENCE.NORM(0.05, 1, 0.5)`** — a positive size that truncates to zero. Documented as
   `#NUM!`; worth confirming that the truncation-then-test order is what Excel uses.
5. **The same three arguments through `CONFIDENCE` and through `NORM.S.INV(1-α/2)*σ/SQRT(n)`** —
   three routes to one number. Any disagreement in the last bits localises whether the legacy alias
   really shares a kernel and whether Excel's `CONFIDENCE.NORM` is a composition of its own public
   parts.
6. **Non-finite and denormal `standard_dev`** — the reference engine rejects non-finite inputs with
   `#NUM!`; the documentation says nothing, so Excel's answer is unknown.
7. **An array in each position**, given the recorded difference in lift profile between this
   surface and the legacy `CONFIDENCE` spelling.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| half-width | The single number returned; the interval is the mean plus and minus it |
| significance level | `alpha`; the confidence level is `100·(1 − alpha)%` |
| normal quantile | `Φ⁻¹`, the inverse standard normal CDF; `NORM.S.INV` on the worksheet |
| complement staging | Evaluating a tail quantity directly rather than as `1 −` its complement |
| B1 battery | The fixed edge-input row set the Handbook runs against the reference engine, rendered mechanically beside this page |

## Sources

- Microsoft, "CONFIDENCE.NORM function" —
  <https://support.microsoft.com/en-us/office/confidence-norm-function-7cec58a6-85bb-488d-91c3-63828d4fbfd4>
  (retrieved for this pass: syntax, argument descriptions, the `100*(1 - alpha)%` relation, the
  `x̄ ± CONFIDENCE.NORM` interval statement, the size-truncation rule, and the four documented
  error conditions).
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 26 — the normal distribution
  and its quantile approximations.
- Wichura, "Algorithm AS 241: The Percentage Points of the Normal Distribution", *Applied
  Statistics* 37 (1988) — the modern double-precision normal quantile.
- Handbook call-model chapters [01 The value universe](../model/01-value-universe.md) and
  [02 Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.CONFIDENCE.NORM.json` (arity, classification, axes,
  documented description) and `data/presence/FUNC.CONFIDENCE.NORM.json` (implementing module and
  the twelve surfaces sharing it).
- OxFunc `docs/function-lane/FUNCTION_SLICE_STATISTICAL_DISTRIBUTIONS_AND_COMPAT_A_CONTRACT_PRELIM.md`
  section 5 and `crates/oxfunc_core/src/functions/normal_log_family.rs` at commit `473efa3` — the
  shared-kernel statement for `CONFIDENCE`/`CONFIDENCE.NORM`, the size-truncation guard, and the
  `#NUM!` domain rejections, read as implementation facts about the reference engine.
