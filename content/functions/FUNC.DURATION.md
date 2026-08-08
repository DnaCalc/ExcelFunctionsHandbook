---
schema: efh.function-page/v1
function_id: FUNC.DURATION
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
  The cash-flow-weighted time measure built on the PRICE schedule: same coupon dates, same discount
  ladder, different reduction. MDURATION's input and the family's link between price and yield
  sensitivity.
---

# DURATION

## What it computes

`DURATION(settlement, maturity, coupon, yld, frequency, [basis])` returns the **Macaulay duration**
of a coupon bond with an assumed par value of 100, expressed in **years**.

Macaulay duration is the present-value-weighted average time to receipt of a security's cash flows:

    D = ( Σ t_k · C_k · v^(t_k) ) / ( Σ C_k · v^(t_k) )        with  v = (1 + y/f)^(−f)

where `t_k` is the time in years from settlement to cash flow `k`, `C_k` the cash flow (a coupon of
`100·coupon/f`, plus 100 at maturity), `y` the annual yield and `f` the coupon frequency. The
denominator is the dirty price; the numerator is that price with each term weighted by its own
maturity. So **duration is a weighted mean of times, and it carries units of years** — that is the
whole content of the definition, and it is why a zero-coupon bond's duration is exactly its time to
maturity.

Two identities are worth holding on to:

1. **Zero coupon.** With `coupon = 0` the only cash flow is the redemption, so `D` is the time to
   maturity in years. This is the sanity check that catches most implementation errors.
2. **The price-sensitivity link.** With `P(y)` the price,

       dP/dy = −(D / (1 + y/f)) · P

   which is why the *modified* duration `D/(1 + y/f)` — see [MDURATION](FUNC.MDURATION.md) — is the
   number risk desks actually quote. Macaulay duration is the time; modified duration is the
   sensitivity.

Behaviour in the limits: `D` increases with maturity, decreases with coupon rate, and decreases
with yield. For a perpetual-style long bond `D` approaches `(1 + y/f)/y` from below rather than
growing without bound, which is why a 30-year and a 100-year bond have far more similar durations
than their maturities suggest. As `y → −f` the discount base `1 + y/f` approaches zero and the
expression has a pole; the admissible domain excludes it (see Errors).

### How Excel measures the time axis

The one place where a textbook formula and a spreadsheet function part company is the *time* axis.
Excel does not measure `t_k` from settlement in calendar years. It works in **coupon periods**:

- the coupon schedule is generated backwards from `maturity` at `12/frequency`-month steps until
  the first date at or before `settlement`, giving `n` remaining coupons;
- the settlement's position inside the current coupon period is expressed as a fraction, and the
  exponent for the `k`-th remaining coupon is that fraction plus `k`;
- the final quotient is divided by `frequency` to convert periods into years.

The fractional offset is the accrual measurement, and it depends on `basis`. This is why two calls
that differ only in `basis` can differ in the third decimal of the answer: the day count is not a
presentational choice, it changes the exponent on every discount factor.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `settlement` | The settlement date — when the security trades to the buyer. Required. | — |
| `maturity` | The maturity date — when the security expires. Required. | — |
| `coupon` | The security's **annual** coupon rate, as a decimal fraction. Required. | — |
| `yld` | The security's **annual** yield, as a decimal fraction. Required. | — |
| `frequency` | Coupon payments per year: `1` annual, `2` semi-annual, `4` quarterly. Required. | — |
| `basis` | The day-count basis. Optional. | 0 |

The day-count basis values are the standard five: `0` or omitted = US (NASD) 30/360, `1` =
Actual/actual, `2` = Actual/360, `3` = Actual/365, `4` = European 30/360.

Three argument traps, in decreasing order of how often they bite:

1. **`coupon` and `yld` are annual rates even when `frequency` is 2 or 4.** A 6% semi-annual bond is
   `coupon = 0.06`, `frequency = 2` — not `0.03`. The function divides by `frequency` internally.
2. **`frequency` admits only 1, 2 and 4.** There is no monthly bond here; `frequency = 12` is an
   error, not a slow path.
3. **Dates and `basis` are truncated to integers**, as documented for the whole bond family. There
   is no `issue` date argument: `DURATION` assumes settlement falls inside a regular coupon period.
   Odd first or last periods need `ODDFPRICE`-family thinking, and `DURATION` will not tell you it
   was the wrong function.

## Result and edge cases

Returns `Number` — a duration in **years**, not in periods and not in days.

- **`settlement ≥ maturity`** is a domain error, not a zero or negative duration.
- **One remaining coupon.** When settlement falls inside the final coupon period the sum has a
  single term and `D` collapses to the remaining fraction of that period expressed in years. The
  reference engine takes an explicit early-return branch for this case and clamps the result at
  zero from below.
- **Zero coupon** is admissible (`coupon = 0`) and gives time to maturity in years.
- **Zero yield** is admissible and makes every discount factor 1, so `D` becomes the simple
  cash-flow-weighted average time.
- **Non-integer `frequency` and `basis`** are truncated toward zero before the range check, so
  `frequency = 2.9` reads as `2`.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

As documented on Microsoft's `DURATION` page:

| Error | Condition |
|---|---|
| `#VALUE!` | `settlement` or `maturity` is not a valid serial date number |
| `#NUM!` | `coupon < 0` or `yld < 0` |
| `#NUM!` | `frequency` is any number other than 1, 2 or 4 |
| `#NUM!` | `basis < 0` or `basis > 4` |
| `#NUM!` | `settlement ≥ maturity` |

The reference engine raises all five with the same codes. Note what the `yld < 0` row costs: a
**negative-yield bond has no `DURATION` in Excel**, even though the mathematics is perfectly well
defined down to `y > −f`. That is a documented restriction of the function's domain, not a
mathematical boundary, and it is worth stating because negative sovereign yields stopped being
hypothetical a decade ago. The reference engine encodes the same restriction.

The reference engine additionally rejects date serials below 1 or beyond the end of year 9999 with
`#VALUE!`, which the documentation does not mention.

## Relationships

- **[MDURATION](FUNC.MDURATION.md)** — modified duration, defined as `DURATION/(1 + yld/frequency)`.
  Same arguments, same errors, one extra division. If you want a price-sensitivity number, that is
  the one.
- **`PRICE`** — the denominator of the duration quotient *is* the dirty price. The two functions
  share their coupon schedule, their day counts and their discount ladder, so a `DURATION`
  discrepancy and a `PRICE` discrepancy on the same bond are usually the same bug seen twice.
- **`YIELD`** — the inverse of `PRICE`, and the usual source of the `yld` argument. Note the loop:
  compute `yld` from a market price with `YIELD`, feed it to `DURATION`, and any day-count
  disagreement enters twice.
- **`COUPNUM` / `COUPDAYBS` / `COUPDAYS` / `COUPDAYSNC`** — the schedule quantities `DURATION`
  computes internally. They are the right instruments for localizing a disagreement: if `COUPDAYBS`
  differs, the duration will differ, and you have found the day count rather than the discounting.
- **`ACCRINT`** — accrued interest, which uses the same accrual span as the fractional offset.
- **Confused with**: the everyday sense of "duration" as time to maturity. For a coupon bond
  duration is always *less* than time to maturity, and a reader who expects the two to match will
  read every answer as wrong.
- There is **no convexity function** in Excel. The second-order term has to be built by hand.

## Numerical notes

`DURATION` is a short sum of well-scaled terms, and the numerator and denominator are both positive
and of similar magnitude, so the reduction itself is benign — no cancellation, no overflow for any
realistic bond. The difficulty is entirely in the *schedule* and in the *discount factor*.

**The schedule is integer calendar arithmetic and is unforgiving.** Coupon dates are generated by
month arithmetic backwards from maturity, with an end-of-month rule: a bond maturing on 31 August
pays on 28/29 February, and a naive "same day number" step lands on the wrong date. Every discount
exponent depends on that schedule, so a single mis-stepped coupon date moves the answer far beyond
any rounding concern.

**The accrual measurement is where 30/360 fine print lives.** The reference engine's own commentary
at commit `473efa3` records that its accrued-days count for the numerator position uses a *modified
start date* variant of US 30/360 rather than the plain one, and that the two diverge only when
settlement falls on the 31st with a month-end previous coupon — the module attributes a very large
observed error to getting that ordering wrong. Whatever the eventual verdict, the lesson
generalizes: in the 30/360 conventions the order in which the start-date and end-date adjustments
are applied is part of the specification, not an implementation detail.

**The discount factor is a `pow`, and `pow` is not one function.** The exponents are integers when
settlement falls exactly on a coupon date and fractional otherwise. Integer exponents can be
evaluated by binary exponentiation (square-and-multiply on plain multiplies) or by `exp(y·ln x)`,
and the two differ in the last bits. Fractional exponents can be evaluated through a modern
correctly-rounded `pow` or through a legacy `exp(ln)` chain, and those differ too. The reference
engine deliberately implements both staging cases and documents that it selects them to match
observed Excel behaviour. The Handbook does **not** assert what Excel does internally; what it can
say is that this is the axis on which careful implementations of bond math disagree, and that any
claim of agreement that has not distinguished on-coupon from off-coupon settlement has not tested
the interesting half.

**Association order.** The reference engine's commentary records that the numerator weight is
grouped `(diff·cash)/disc` rather than `diff·(cash/disc)`, and that the final reduction associates
as `num/den/f`. These are the choices that separate a one-ULP answer from a several-ULP answer;
they are recorded here as implementation facts about the reference engine, not as claims about
Excel.

For the underlying mathematics, Macaulay's weighted-average-time construction and the
`dP/dy = −D_mod·P` link are standard; any fixed-income text develops them, and the discrete-schedule
form used by spreadsheet functions is the one in the market conventions literature rather than in
the continuous-time treatment.

## What has not been checked

No Handbook vector suite exists for `DURATION`; `vectors/` publishes nothing at this revision, so no
suite-scoped claim exists for it. No Excel-comparison evidence record names `DURATION` in its
subjects. **Nobody has checked this function against Excel within the Handbook's record.** The
argument meanings, the frequency and basis restrictions and the five error rows are Microsoft's
documented statements; the schedule construction, the accrual variant, the `pow` staging and the
association order are read from the reference engine's source at commit `473efa3` and are
implementation facts about that engine.

Inputs worth probing first, in priority order:

1. **A zero-coupon call** — `coupon = 0` — where the answer must equal the time to maturity in
   years under the chosen basis. This is the cheapest probe that tests the schedule and the time
   axis simultaneously, and its expected value is known independently of any implementation.
2. **A settlement date exactly on a coupon date**, versus one day after. On-coupon settlement makes
   every discount exponent an integer and takes the other `pow` staging path; the pair separates
   the two branches with two cells.
3. **Settlement on the 31st with a month-end previous coupon** — the accrual-variant break the
   reference engine's commentary identifies. Run it at `basis = 0` and at `basis = 4`.
4. **The same bond at all five bases**, which quantifies how much of any disagreement is day count
   rather than discounting.
5. **`frequency = 1, 2, 4` on the same bond**, and `frequency = 12` to confirm the `#NUM!`.
6. **`yld = 0`**, admissible and degenerate, where every discount factor is 1 and the answer is a
   plain weighted mean — a second independently-checkable expected value.
7. **A negative `yld`** to confirm the documented `#NUM!`, and a bond maturing inside the current
   coupon period to exercise the single-term branch.
8. **`MDURATION` against `DURATION/(1 + yld/frequency)` on the same inputs** — a metamorphic check
   that costs one cell and pins the relationship between the two functions.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| Macaulay duration | The present-value-weighted average time to a security's cash flows, in years |
| coupon schedule | The dates generated backwards from maturity at `12/frequency`-month steps |
| fractional offset | Settlement's position inside the current coupon period, as a fraction of it |
| dirty price | The discounted cash-flow sum that forms the quotient's denominator |
| on-coupon settlement | Settlement falling exactly on a coupon date, making all exponents integers |
| day-count basis | The `basis` argument's calendar convention, one of the five documented methods |

## Sources

- Microsoft, "DURATION function" —
  <https://support.microsoft.com/en-us/office/duration-function-b254ea57-eadc-4602-a86a-c8e369334038>
  (syntax, argument meanings, the frequency values, the day-count basis table, the truncation
  remarks, and the documented `#VALUE!` and `#NUM!` conditions).
- Handbook, [MDURATION](FUNC.MDURATION.md) — the modified-duration sibling and the exact relation.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- OxFunc `crates/oxfunc_core/src/functions/bond_core_family.rs` at commit `473efa3` — the
  `duration_kernel`, the coupon-schedule generator, the accrued-days variant, the two-case `pow`
  staging and the recorded association order, all read as implementation facts about that engine.
- Handbook projections `data/functions/FUNC.DURATION.json` and `data/presence/FUNC.DURATION.json`
  (arity, classification axes, implementing module and the bond-family sibling set).
