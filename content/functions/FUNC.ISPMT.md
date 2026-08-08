---
schema: efh.function-page/v1
function_id: FUNC.ISPMT
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
  The straight-line-principal outlier: a one-line closed form on a different loan model from the
  annuity members, with a 0-based period index and no iteration, retained for Lotus 1-2-3
  compatibility.
---

# ISPMT

## What it computes

`ISPMT(rate, per, nper, pv)` returns the interest paid during period `per` of a loan in which the
**principal is repaid in equal instalments** — a straight-line amortization, not the constant-total-
payment annuity that [IPMT](FUNC.IPMT.md) models.

The closed form is one line:

    ISPMT = pv × rate × (per/nper − 1)

Read it as interest on the remaining balance. Under straight-line repayment the outstanding
principal after `per` instalments is `pv × (1 − per/nper)`, and the interest charged is `rate` times
that:

    remaining balance = pv × (nper − per)/nper
    interest          = rate × remaining balance = pv × rate × (per/nper − 1) × (−1)

The sign is the family's: with a positive `pv` (money you received) the result is **negative**,
money leaving. That is why the formula is written with `(per/nper − 1)`, which is negative on the
admissible range, rather than the positive `(1 − per/nper)`.

**The period index starts at 0, not at 1.** This falls straight out of the formula and it is the
single most important fact on this page:

| `per` | `ISPMT` | Meaning |
|---|---|---|
| `0` | `−pv·rate` | Interest on the full principal — the first period |
| `nper/2` | `−pv·rate/2` | Half the principal repaid |
| `nper` | `0` | Nothing outstanding — the loan is retired |

So a loan with `nper = 10` has its ten interest charges at `per = 0 … 9`, and `ISPMT(rate, 10, 10,
pv)` is zero rather than the last instalment. `IPMT`, its near-namesake, indexes `1 … nper`. Two
functions whose names differ by one letter, whose signatures look alike, and whose first period is a
different number.

The result is **linear in `per`**: the interest declines by a constant `pv·rate/nper` each period.
That is the visible signature of straight-line amortization, and it is how you tell at a glance
which loan model a column of numbers came from — annuity interest decays geometrically, straight-line
interest decays in a straight line.

`ISPMT` exists for Lotus 1-2-3 compatibility. It is not the function to reach for in new work unless
the straight-line model is genuinely what you mean.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `rate` | The interest rate **per period** for the investment. Required. | — |
| `per` | The period for which the interest is wanted; counted from `0`. Required. | — |
| `nper` | The total number of payment periods. Required. | — |
| `pv` | The present value — the principal. Required. | — |

All four are required; there is no `fv` and no `type`. The absence of `type` is not an oversight:
under straight-line repayment with the instalment at period end there is nothing for a timing switch
to select, and the model has no annuity-due variant.

**`rate` and `nper` must use the same period**, exactly as in the annuity functions. A 10% annual
rate on a monthly schedule is `0.1/12`.

In the reference engine at commit `473efa3` all four slots take ordinary numeric coercion, with an
omitted-slot Missing marker and a blank cell both arriving as `0`.

## Result and edge cases

Returns `Number` — an interest amount carrying the family sign convention.

- **`per = 0`** gives `−pv·rate`, the largest charge. **`per = nper`** gives exactly `0`.
- **`per` outside `0 … nper` is not rejected.** The formula is linear and the reference engine
  applies no range guard at all, so `per = nper + 5` returns a positive number and `per = −3`
  returns a magnitude larger than the first period's. This is a real behavioural difference from
  [IPMT](FUNC.IPMT.md), which errors with `#NUM!` outside its range, and it means a spreadsheet that
  fills an `ISPMT` column one row too far gets a plausible number rather than a visible error.
- **`nper ≤ 0`** is `#NUM!` in the reference engine — the only domain guard the kernel carries.
- **Non-integer `per`** is accepted and interpolates linearly; nothing truncates it.
- **`rate = 0`** returns zero for every period, with no special branch needed.
- **Negative `pv`** flips the sign throughout, as the convention requires.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

Microsoft's `ISPMT` page does not publish an error table in the way the securities functions do.
What the Handbook can state:

| Error | Condition |
|---|---|
| `#NUM!` | `nper ≤ 0` (division by zero periods), or a non-representable result |
| `#DIV/0!` | Not produced by the reference engine — `nper = 0` is diagnosed as `#NUM!` first |
| `#VALUE!` | A numeric slot receives non-numeric text or an unsupported value kind |
| propagated | An error value in any argument surfaces as that error |
| `#VALUE!` | The call is made with any number of arguments other than four |

Notably **absent**: any error for an out-of-range `per`. That is the reference engine's behaviour at
commit `473efa3`; whether Excel agrees is unverified, and it is the first thing on the probe list.

## Relationships

- **[IPMT](FUNC.IPMT.md)** — the same question under a *different loan model*. `IPMT` assumes a
  constant total payment; `ISPMT` assumes constant principal repayment. They agree on nothing except
  at `nper = 1`, and their period indices are offset by one. Substituting one for the other is a
  modelling error, not a refactor.
- **`PPMT`** — the principal component under the annuity model. `ISPMT`'s principal component needs
  no function: it is `pv/nper`, constant.
- **`PMT`** — the constant total payment. Under `ISPMT`'s model the total payment is *not* constant:
  it is `pv/nper + ISPMT(per)`, which declines each period.
- **`CUMIPMT`** — cumulative interest under the annuity model. There is no `ISPMT` cumulative
  sibling; the sum is closed-form, `pv·rate·(nper − 1)/2` over `per = 0 … nper − 1`.
- **`SLN`** — straight-line *depreciation*, the same "equal instalments" idea applied to an asset
  rather than a debt. The two functions are conceptual cousins and are often used together.
- **Confused with**: `IPMT`, constantly. If a model's interest column is a straight line, it came
  from `ISPMT`; if it curves, it came from `IPMT`.

## Numerical notes

`ISPMT` is the numerically easiest function in this batch and it is worth saying why, because the
contrast is instructive.

The expression `pv × rate × (per/nper − 1)` has one division, two multiplications and one
subtraction, and **no cancellation of consequence**. The subtraction `per/nper − 1` loses precision
only as `per → nper`, where the true answer is approaching zero anyway, so the *relative* error of a
vanishing quantity degrades while the *absolute* error stays bounded by rounding. Nothing
compounds, nothing iterates, no transcendental function is involved. There is no small-rate regime,
no `expm1`/`log1p` question, no evaluation-order decision of any consequence.

That is a direct consequence of the model: straight-line amortization is linear, and linear
functions are numerically boring. Every difficulty catalogued on the [IPMT](FUNC.IPMT.md) page —
the near-payoff cancellation, the dependence on an accurate `PMT`, the `expm1` substrate, the
epsilon-band trap around zero rate — exists because the annuity balance is *exponential*. `ISPMT`
buys its simplicity by modelling a different loan.

The only evaluation-order question worth noting is association: `(pv × rate) × (per/nper − 1)`
versus `pv × (rate × (per/nper − 1))` differ in the last bit for some inputs. The reference engine
evaluates left to right as `pv * rate * (per/nper - 1.0)`. That is an implementation fact, not a
claim about Excel.

## What has not been checked

No Handbook vector suite exists for `ISPMT`; `vectors/` publishes nothing at this revision, so no
suite-scoped claim exists for it. No Excel-comparison evidence record names `ISPMT` in its subjects;
the financial records covering this implementing module name other surfaces, and the Handbook does
not attribute a group measurement to a surface a record does not list. **The family containing
`ISPMT` has been measured against live Excel; this surface has not been measured separately.**
Nobody has checked `ISPMT` against Excel within the Handbook's record.

The argument meanings and the unit-consistency rule are Microsoft's documented statements. The
0-based period indexing is derived here from the defining formula rather than quoted, and readers
should treat it as a derivation to be confirmed. The absence of a `per` range guard, the `nper ≤ 0`
error and the association order are read from the reference engine's source at commit `473efa3`.

Inputs worth probing first:

1. **`ISPMT(rate, 0, nper, pv)` and `ISPMT(rate, nper, nper, pv)`** — the two ends. The first must
   be `−pv·rate` and the second exactly `0`. Together they pin the indexing convention that the
   whole page turns on, and both expected values are known without an oracle.
2. **`per` outside the range** — `per = nper + 1` and `per = −1` — which the reference engine
   computes rather than rejecting. If Excel errors here, that is a documented-versus-behaviour
   divergence worth publishing.
3. **`ISPMT` against `IPMT` at `nper = 1`**, the one configuration where the two models coincide,
   modulo the index offset.
4. **A full column at `nper = 12`**, checking that successive differences are the constant
   `−pv·rate/nper` — the linearity signature, which catches any accidental exponential term.
5. **`nper = 0`**, to confirm `#NUM!` rather than `#DIV/0!`, since the natural reading of a division
   by zero periods would give the latter.
6. **Non-integer `per`**, e.g. `per = 1.5`, to confirm interpolation rather than truncation.
7. **`rate = 0`** and **negative `pv`**, the two trivial sign and zero cases.
8. **`Σ ISPMT(per) for per = 0 … nper−1` against `pv·rate·(nper − 1)/2`** — a closed-form total that
   needs no oracle and exercises accumulation over a whole schedule.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| straight-line amortization | Equal principal instalments each period; the model `ISPMT` assumes |
| 0-based period index | `per` counted from `0`, so the last interest charge is at `per = nper − 1` |
| declining total payment | The consequence of the model: principal is constant, interest falls, the total falls |
| annuity model | The constant-total-payment model used by `IPMT`, `PPMT` and `PMT` |
| sign convention | Money paid out is negative; `ISPMT` is negative for a positive `pv` |

## Sources

- Microsoft, "ISPMT function" —
  <https://support.microsoft.com/en-us/office/ispmt-function-fa58adb6-9d39-4ce0-8f43-75399cea56cc>
  (syntax, the four argument descriptions, and the unit-consistency rule for `rate` and `nper`).
- Handbook, [IPMT](FUNC.IPMT.md) — the annuity-model counterpart, its indexing and its numerical
  difficulties, against which this page's contrast is drawn.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- OxFunc `crates/oxfunc_core/src/functions/financial_time_value_family.rs` at commit `473efa3` —
  the `ispmt` kernel, its single `nper <= 0` guard, its absence of a `per` range check and its
  left-to-right association, read as implementation facts about that engine.
- Handbook projections `data/functions/FUNC.ISPMT.json` and `data/presence/FUNC.ISPMT.json` (arity,
  classification axes, implementing module and the fifteen-surface family it shares).
