---
schema: efh.function-page/v1
function_id: FUNC.DISC
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
family: discount_bill_yearfrac_family
role_in_family: >-
  The bank-discount quotation: the rate that, applied to the redemption value over the
  settlement-to-maturity year fraction, reproduces the purchase price. INTRATE's mirror image,
  quoted on the other side of the trade.
---

# DISC

## What it computes

`DISC(settlement, maturity, pr, redemption, [basis])` returns the **discount rate** of a security
bought at price `pr` per 100 of face value and redeemed at `redemption` per 100 at maturity.

The underlying model is *simple bank discount*, not compound interest. Let

- `T` = the year fraction from settlement to maturity under the chosen day-count `basis`,
- `d` = the discount rate the function returns.

The model is the linear price relation

    pr = redemption × (1 − d × T)

and `DISC` inverts it:

    d = (redemption − pr) / redemption × 1/T
      = (1 − pr/redemption) / T

Written the way Microsoft's page writes it, with `B` the number of days in the year implied by
the basis and `DSM` the number of days from settlement to maturity, `T = DSM/B` and

    DISC = (redemption − pr) / redemption × B / DSM

The defining feature — and the reason `DISC` and `INTRATE` are not the same function with the
arguments renamed — is the **denominator of the yield fraction**. `DISC` divides the gain by the
*redemption* value, the amount you get back. `INTRATE` divides it by the *investment*, the amount
you put in. Money-market instruments are quoted both ways, and swapping them is a real pricing
error, not a rounding one.

The two are related exactly: with `r` the `INTRATE` figure on the same dates and amounts,

    r = d / (1 − d·T)        and        d = r / (1 + r·T)

Domain and range: `d` is unbounded above as `T → 0` (a security maturing tomorrow at any discount
implies a huge annualized rate), and `d < 0` whenever `pr > redemption` — a premium purchase gives
a negative discount rate. The function has a pole at `T = 0`, which the settlement-before-maturity
requirement removes from the admissible domain.

Nothing here is compounded. `DISC` is a first-order quotation convention; over long horizons it is
not a rate of return in any compounding sense.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `settlement` | The settlement date — when the security trades to the buyer. Required. | — |
| `maturity` | The maturity date — when the security expires. Required. | — |
| `pr` | The security's price per $100 face value. Required. | — |
| `redemption` | The security's redemption value per $100 face value. Required. | — |
| `basis` | The day-count basis. Optional. | 0 |

Microsoft's page documents the basis values as:

| `basis` | Method |
|---|---|
| 0 or omitted | US (NASD) 30/360 |
| 1 | Actual/actual |
| 2 | Actual/360 |
| 3 | Actual/365 |
| 4 | European 30/360 |

Two documented rules that matter more than they look:

1. **Dates are Excel serial numbers**, and `settlement`, `maturity` and `basis` are **truncated to
   integers**. A date-with-time argument therefore loses its time part silently.
2. **`pr` and `redemption` are per 100 of face value**, not amounts. A bill with 1,000,000 face
   bought for 987,500 is `pr = 98.75`, not `987500`. Because both appear only in the ratio
   `pr/redemption`, passing both as raw amounts happens to give the same answer — but mixing one
   raw amount with one per-100 quote does not, and the function cannot detect it.

The argument slots are numeric and subject to ordinary to-number coercion; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number` — an annualized rate expressed as a decimal fraction, so 5.34% comes back as
`0.0534`.

- **`settlement ≥ maturity`** is a domain error, not a negated result. There is no
  "reversed-dates" reading; see Errors below.
- **`pr > redemption`** is admissible and returns a negative rate.
- **Basis choice changes the answer materially**, not marginally: a 91-day bill priced on
  Actual/360 and on Actual/365 differ in the fourth significant figure of the rate. The basis is
  part of the quotation, not a rounding preference.
- **Non-integer `basis`** is truncated toward zero before the range check, so `basis = 4.9` reads
  as `4`.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md).

## Errors

As documented on Microsoft's `DISC` page:

| Error | Condition |
|---|---|
| `#VALUE!` | `settlement` or `maturity` is not a valid serial date number |
| `#NUM!` | `pr ≤ 0`, or `redemption ≤ 0` |
| `#NUM!` | `basis < 0` or `basis > 4` |
| `#NUM!` | `settlement ≥ maturity` |

The reference engine's implementation raises the same four conditions with the same codes, and
adds one the documentation does not mention: a date serial below 1 (or above the end of year 9999)
is rejected as `#VALUE!`, so the "zero date" is outside the admissible domain rather than being
treated as 1899-12-31. The Handbook has not observed either behaviour in Excel.

Non-numeric text in any slot and error values in any argument surface under the shared coercion
rules.

## Relationships

- **[INTRATE](FUNC.INTRATE.md)** — the same trade quoted on the investment instead of the
  redemption, with the exact conversion given above. The pair is the single most useful thing to
  keep straight in this family.
- **`PRICEDISC`** — the inverse: given `DISC`'s rate, recover `pr`. `PRICEDISC(DISC(...))` should
  round-trip up to floating-point rounding.
- **`YIELDDISC`** — the *yield* of a discounted security, which is the `INTRATE`-side quotation of
  the same instrument; do not read it as an inverse of `DISC`.
- **`TBILLYIELD` / `TBILLPRICE` / `TBILLEQ`** — the same discount arithmetic hard-wired to the
  Treasury-bill Actual/360 convention, with no `basis` argument and their own 182-day rules.
- **`YEARFRAC`** — the year-fraction machinery `DISC` depends on, exposed directly. If a `DISC`
  answer looks wrong, evaluating `YEARFRAC(settlement, maturity, basis)` is the fastest way to
  find out whether the day count or the price ratio is the culprit.

## Numerical notes

`DISC` is arithmetically trivial and definitionally hard. Almost all of the difficulty is in `T`.

**Day counts are integer arithmetic on a calendar, and the calendar rules are the algorithm.** The
30/360 conventions are not approximations of the actual calendar; they are separate, exactly
specified counting rules, and the US (NASD) and European variants differ precisely in how they
treat the 31st and the end of February. An implementation that gets the price ratio right to the
last bit and the end-of-month adjustment wrong is wrong by cents, not by ULPs.

**The Actual/actual basis is where implementations quietly disagree.** For a span crossing a year
boundary there is more than one defensible reading of "the number of days in the year": accumulate
each calendar year's fraction against that year's own length, or divide the actual day count by an
averaged year length. Both appear in real code. In the reference engine at this commit the two
financial families do not share one answer: the discount/bill module accumulates per calendar year
in the style of `YEARFRAC`, while the bond module (`PRICE`, `DURATION` and their siblings) computes
an averaged year length over the whole span. That is a divergence inside the reference engine
itself and the Handbook records it as a finding rather than resolving it — see *What has not been
checked*.

**Floating-point form.** The documented expression computes `(redemption − pr)/redemption` and then
multiplies by `B/DSM`; the reference engine computes `(1 − pr/redemption)/T`. The two are
algebraically identical and are not identical in binary64: they differ in where the single
subtraction lands relative to the division. For `pr` close to `redemption` — a bill maturing in
days — the subtraction is a cancellation site in both forms, and the relative error in the small
difference is amplified by the `1/T` factor, which is itself large exactly when the cancellation is
worst. This is the one genuine numerical hazard in the function, and it is intrinsic to the
quantity, not to any particular arrangement of the arithmetic.

## What has not been checked

No Handbook vector suite exists for `DISC`; `vectors/` publishes nothing at this revision, so no
suite-scoped claim exists for it. No Excel-comparison evidence record names `DISC` in its subjects.
**Nobody has checked this function against Excel within the Handbook's record.** The four error
rows above are Microsoft's documented conditions; the equation is Microsoft's documented equation;
everything said about the reference engine is read from its source at commit `473efa3` and is an
implementation fact, not an Excel observation.

Inputs worth probing first, in priority order:

1. **`DISC` and `YEARFRAC` on the same dates with `basis = 1`**, across a span that crosses a year
   boundary and across a span inside one calendar year. This is the cheapest test of the
   intra-engine divergence described above, and it is the probe most likely to move a real answer.
2. **The same `DISC` call at `basis = 1` compared with `PRICE`/`DURATION` on a span of the same
   length**, which is the cross-family half of the same question: whether Excel uses one
   Actual/actual rule for money-market functions and another for bonds.
3. **A settlement on the 31st with a maturity on the 30th, and both end-of-February cases**, at
   `basis = 0` and `basis = 4`, which separate the US and European 30/360 adjustments.
4. **`settlement = maturity`** and **`settlement = maturity − 1`** — the documented `#NUM!` boundary
   and the shortest admissible span, where the `1/T` amplification is largest.
5. **`pr` greater than `redemption`**, to confirm that a negative rate is returned rather than an
   error.
6. **A date serial of `0`**, which the reference engine refuses as `#VALUE!` and which the
   documentation does not mention at all.
7. **Fractional `basis` and fractional date serials** — `basis = 4.9`, `settlement = 45000.75` —
   to confirm the documented truncation is truncation toward zero rather than rounding.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| bank discount | A rate quoted as a fraction of the redemption value, not of the price paid |
| day-count basis | The `basis` argument's calendar convention, one of the five documented methods |
| year fraction `T` | The settlement-to-maturity span expressed in years under the chosen basis |
| per-100 quotation | Prices and redemption values expressed per 100 units of face value |
| DSM / B | Microsoft's names for the settlement-to-maturity day count and the basis year length |

## Sources

- Microsoft, "DISC function" —
  <https://support.microsoft.com/en-us/office/disc-function-71fce9f3-3f05-4acf-a5a3-eac6ef4daa53>
  (syntax, argument meanings, the day-count basis table, the serial-date and truncation remarks,
  and the `#VALUE!` and `#NUM!` conditions listed above).
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md) — argument kinds, numeric coercion
  and error propagation.
- Handbook, [Claim language and honesty](../model/06-claim-language.md) — the scoping rules this
  page's claims are written under.
- OxFunc `crates/oxfunc_core/src/functions/discount_bill_yearfrac_family.rs` at commit `473efa3`
  (the reference engine's `disc_kernel`, its year-fraction routine, and its date and basis guards)
  and `crates/oxfunc_core/src/functions/bond_core_family.rs` at the same commit (the bond family's
  differing Actual/actual year length).
- Handbook projections `data/functions/FUNC.DISC.json` and `data/presence/FUNC.DISC.json`
  (arity, classification axes, implementing module and sibling set).
