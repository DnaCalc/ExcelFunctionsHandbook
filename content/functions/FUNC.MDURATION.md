---
schema: efh.function-page/v1
function_id: FUNC.MDURATION
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
family: bond_core_family
role_in_family: >-
  DURATION divided by one plus the periodic yield: the family's price-sensitivity measure, carrying
  every one of DURATION's schedule and day-count decisions plus one division.
---

# MDURATION

## What it computes

`MDURATION(settlement, maturity, coupon, yld, frequency, [basis])` returns the **modified duration**
of a coupon bond with an assumed par value of 100.

The definition is one division applied to Macaulay duration:

    MDURATION = DURATION / (1 + yld/frequency)

and that division is what converts a *time* into a *sensitivity*. Where Macaulay duration answers
"on average, how far away is my money", modified duration answers "if the yield moves, how much
does the price move":

    dP/dy = −MDURATION × P            equivalently        ΔP/P ≈ −MDURATION × Δy

So a modified duration of 7.2 means a one-percentage-point rise in yield costs roughly 7.2% of the
price. It is a **first-order** statement: the price/yield curve is convex, so the linear estimate
overstates losses on a rise and understates gains on a fall, and the error grows with the square of
`Δy`. Excel has no convexity function to supply the second-order correction.

Units are years, inherited from `DURATION`; the quantity is often read as "per unit change in
yield", which is dimensionally the same thing because yield is per year.

Microsoft's page names the result **"Macauley modified duration"** — with that spelling of
Macaulay. It is worth knowing so that searches match.

Limiting cases follow `DURATION`'s directly, damped by the divisor:

- **Zero coupon**: `MDURATION` is the time to maturity divided by `1 + yld/frequency`, so it is
  strictly less than the time to maturity for any positive yield.
- **`yld = 0`**: the divisor is 1 and modified duration equals Macaulay duration exactly.
- **`yld → −frequency`**: the divisor approaches zero and the quantity diverges — but that region is
  excluded from the admissible domain long before it is reached, because `yld < 0` is an error.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `settlement` | The settlement date. Required. | — |
| `maturity` | The maturity date. Required. | — |
| `coupon` | The security's **annual** coupon rate, as a decimal fraction. Required. | — |
| `yld` | The security's **annual** yield, as a decimal fraction. Required. | — |
| `frequency` | Coupon payments per year: `1` annual, `2` semi-annual, `4` quarterly. Required. | — |
| `basis` | The day-count basis. Optional. | 0 |

Identical to [DURATION](FUNC.DURATION.md), and the same three traps apply: `coupon` and `yld` are
**annual** rates even at `frequency = 2` or `4`; `frequency` admits only 1, 2 and 4; dates and
`basis` are truncated to integers.

The day-count basis values are the standard five: `0` or omitted = US (NASD) 30/360, `1` =
Actual/actual, `2` = Actual/360, `3` = Actual/365, `4` = European 30/360.

Because the extra division uses `yld/frequency`, the `frequency` argument enters the answer twice —
once through the coupon schedule and once through the divisor. Getting it wrong therefore moves the
result further than it moves `DURATION`.

## Result and edge cases

Returns `Number` — a modified duration in years.

- **`settlement ≥ maturity`** is a domain error.
- **`yld = 0`** makes `MDURATION` and `DURATION` identical; that equality is a useful self-test.
- **One remaining coupon**: inherits `DURATION`'s single-term branch, then divides.
- **Every day-count and schedule subtlety of `DURATION` is present here unchanged.** Modified
  duration adds no new calendar behaviour whatsoever; it adds one division. If an `MDURATION`
  answer looks wrong, check `DURATION` first — the fault is almost certainly upstream of the
  division.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

As documented on Microsoft's `MDURATION` page:

| Error | Condition |
|---|---|
| `#VALUE!` | `settlement` or `maturity` is not a valid serial date number |
| `#NUM!` | `coupon < 0` or `yld < 0` |
| `#NUM!` | `frequency` is any number other than 1, 2 or 4 |
| `#NUM!` | `basis < 0` or `basis > 4` |
| `#NUM!` | `settlement ≥ maturity` |

The reference engine raises all five with the same codes, computing `DURATION` first and dividing
afterwards, so every error condition is `DURATION`'s error condition.

The `yld < 0` restriction deserves the same note it gets on the `DURATION` page: a negative-yield
bond has **no modified duration in Excel**, even though the sensitivity is perfectly well defined
and is precisely the quantity a desk holding negative-yielding paper needs. This is a documented
domain restriction, not a mathematical one.

The reference engine additionally rejects date serials below 1 or beyond the end of year 9999 with
`#VALUE!`; the documentation does not mention this boundary.

## Relationships

- **[DURATION](FUNC.DURATION.md)** — Macaulay duration, the numerator. The relation
  `MDURATION = DURATION/(1 + yld/frequency)` is exact in the definition and is the natural
  metamorphic test for both functions at once.
- **`PRICE`** — the price whose sensitivity `MDURATION` measures. Together they give the standard
  first-order estimate `ΔP ≈ −MDURATION × P × Δy`, and comparing that estimate against a direct
  re-pricing at the shifted yield is the honest way to see how far first order gets you.
- **`YIELD`** — the usual source of `yld`.
- **`COUPNUM` / `COUPDAYBS` / `COUPDAYS`** — the schedule instruments for localizing a
  disagreement, exactly as for `DURATION`.
- **Confused with**: Macaulay duration itself. The two differ by a factor that, for a semi-annual
  bond at a 6% yield, is about 3% — small enough to look like a rounding difference and large
  enough to matter in a hedge ratio.
- **Not available**: convexity, the second-order term. Excel provides no function for it.

## Numerical notes

The additional arithmetic is one division by `1 + yld/frequency`, a quantity of order 1 for any
admissible input. It introduces one rounding and no instability: there is no cancellation, no
overflow, and the divisor cannot approach zero inside the admissible domain because `yld < 0` is
rejected.

That makes `MDURATION` a clean amplifier for whatever `DURATION` does. All the genuine numerical
difficulty — the backwards coupon schedule with its end-of-month rule, the 30/360 accrual variants
and the order in which their start-date and end-date adjustments apply, the two different `pow`
stagings for integer and fractional discount exponents — belongs to `DURATION` and is discussed on
that page. `MDURATION` inherits every one of them and can add at most one ULP of its own.

One consequence worth stating: because the two functions differ by an exactly specified division, a
disagreement between an implementation's `MDURATION` and `DURATION/(1 + yld/f)` computed on the
sheet is diagnostic. If the identity holds, any error is in the schedule or the discounting; if it
fails, the implementation has computed modified duration by some other route.

## What has not been checked

No Handbook vector suite exists for `MDURATION`; `vectors/` publishes nothing at this revision, so
no suite-scoped claim exists for it. No Excel-comparison evidence record names `MDURATION` in its
subjects. **Nobody has checked this function against Excel within the Handbook's record.** The
argument meanings, the frequency and basis restrictions and the five error rows are Microsoft's
documented statements; the reference engine's construction — `DURATION` then divide — is read from
its source at commit `473efa3`.

Inputs worth probing first:

1. **`MDURATION` against `DURATION/(1 + yld/frequency)` on the same inputs.** One cell, and it
   settles whether the division is the only difference. Everything else on this list is really a
   `DURATION` probe.
2. **`yld = 0`**, where `MDURATION` must equal `DURATION` exactly — a second identity with a known
   expected value.
3. **A zero-coupon bond**, where `MDURATION` must be the time to maturity divided by
   `1 + yld/frequency`; the expected value is independently computable.
4. **`frequency = 1, 2, 4` on the same bond**, which moves the divisor as well as the schedule and
   therefore exercises the one place `MDURATION` uses `frequency` on its own account.
5. **Settlement on a coupon date versus one day later**, and **settlement on the 31st with a
   month-end previous coupon** — inherited from `DURATION`, where the reference engine's own
   commentary locates its sharpest calendar break.
6. **A negative `yld`**, to confirm the documented `#NUM!` rather than a large finite sensitivity.
7. **The first-order estimate against a re-pricing**: `PRICE` at `y` and at `y + 0.0001` versus
   `−MDURATION × P × 0.0001`, which measures the convexity Excel does not expose.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| modified duration | Macaulay duration divided by one plus the periodic yield; a price sensitivity |
| periodic yield | `yld/frequency`, the yield per coupon period |
| first-order estimate | `ΔP/P ≈ −MDURATION × Δy`, the linear price-change approximation |
| convexity | The neglected second-order term; no Excel function computes it |
| Macauley | Microsoft's spelling of Macaulay on this function's page |

## Sources

- Microsoft, "MDURATION function" —
  <https://support.microsoft.com/en-us/office/mduration-function-b3786a69-4f20-469a-94ad-33e5b90a763c>
  (syntax, the "Macauley modified duration" description, argument meanings, frequency values, the
  day-count basis table, the truncation remarks, and the documented `#VALUE!` and `#NUM!`
  conditions).
- Handbook, [DURATION](FUNC.DURATION.md) — the Macaulay numerator, its schedule construction and the
  numerical discussion this page defers to.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- OxFunc `crates/oxfunc_core/src/functions/bond_core_family.rs` at commit `473efa3` — the
  `mduration_kernel` (a call to `duration_kernel` followed by the division) and the shared bond
  schedule, day-count and guard machinery.
- Handbook projections `data/functions/FUNC.MDURATION.json` and
  `data/presence/FUNC.MDURATION.json`.
