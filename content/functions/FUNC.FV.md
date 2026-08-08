---
schema: efh.function-page/v1
function_id: FUNC.FV
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-FIN-0001
  - EV-FIN-0002
  - EV-FIN-0005
  - EV-FIN-0012
  - EV-FIN-0014
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
  The annuity equation solved for the terminal balance; the family's forward direction, and the
  member whose evidence record shows it does not share PMT's kernel.
---

# FV

## What it computes

`FV(rate, nper, pmt, [pv], [type])` returns the **future value** of an investment: the balance after
`nper` periods, given a constant periodic `rate`, a constant periodic payment `pmt`, and a starting
balance `pv`.

All five annuity functions in this family — `FV`, `PV`, `PMT`, `NPER`, `RATE` — are the same single
equation solved for different unknowns. That equation, for `rate ≠ 0`, is

    pv·(1 + rate)^nper  +  pmt·(1 + rate·type)·((1 + rate)^nper − 1)/rate  +  fv  =  0

and for `rate = 0` it degenerates to

    pv  +  pmt·nper  +  fv  =  0

`FV` solves it for `fv`:

    FV  =  −( pv·(1 + rate)^nper  +  pmt·(1 + rate·type)·((1 + rate)^nper − 1)/rate )

Read the two terms separately. The first is the starting balance compounded forward. The second is
the accumulated value of the payment stream: `((1+r)^n − 1)/r` is the future value of `n` unit
payments made at the *end* of each period — the annuity-immediate accumulation factor — and the
`(1 + rate·type)` multiplier advances each payment by one period when payments are made at the
*beginning* instead.

The limit as `rate → 0` of `((1+r)^n − 1)/r` is `n`, which is why the degenerate branch is the
linear one and why it is continuous with the general case rather than a special convention.

### The sign convention is the whole function

**Money you pay out is negative; money you receive is positive.** Every term in the equation sits on
the same side of a zero-sum balance, which is why the equation ends `+ fv = 0` rather than
`= fv`, and why `FV` returns the *negated* combination.

The practical consequences:

- A savings plan where you deposit 100 a month starting from nothing is
  `FV(rate, nper, −100, 0)` — with a **negative** `pmt` — and returns a **positive** balance.
- Writing `FV(rate, nper, 100, 0)` returns a negative number of the same magnitude. It is not an
  error and Excel will not warn you; it is the answer to "what if the annuity pays *me* 100 a
  month".
- `pv` and `pmt` must be signed consistently with each other. A positive `pv` (money you were given)
  alongside a negative `pmt` (money you pay) describes a loan being repaid, and the `FV` is the
  outstanding balance — negative if the loan is not yet retired.

Sign-convention mistakes are the single most common cause of an annuity answer that looks wrong, and
they are undetectable from the result alone, because both signs are legitimate answers to
legitimate questions.

### Unit consistency

`rate` and `nper` must be expressed in the **same period**. A 12% annual rate on a four-year monthly
plan is `rate = 0.12/12` and `nper = 4*12`; using `0.12` with `48` prices a 48-year investment at
12% a year. Microsoft's page states this rule explicitly, and it is the second most common error
after signs.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `rate` | The interest rate **per period**. Required. | — |
| `nper` | The total number of payment periods. Required. | — |
| `pmt` | The payment made each period; constant over the life of the annuity. Required. | — |
| `pv` | The present value — the lump sum a series of future payments is worth now. Optional. | 0 |
| `type` | `0` = payments due at the **end** of the period; `1` = at the **beginning**. Optional. | 0 |

- **`pmt` is required in `FV`** even though it is often zero. `FV(rate, nper, 0, −pv)` is the
  compound-growth case, and the zero must be written.
- **`nper` need not be an integer.** The equation is evaluated with a real exponent, so a fractional
  `nper` is computed rather than rejected — meaningful or not.
- **`type` is documented as `0` or `1`.** In the reference engine at commit `473efa3` the argument is
  truncated and any nonzero value selects beginning-of-period, so `type = 2` and `type = −1` behave
  as `1`. That is an implementation fact about the reference engine, not a documented Excel rule,
  and it is on the probe list.
- Omitted optional arguments and blank cells both arrive as `0` in the reference engine, so
  `FV(r, n, p, , 1)` and `FV(r, n, p, 0, 1)` agree.

## Result and edge cases

Returns `Number` — a balance in the same currency units as `pv` and `pmt`, carrying the sign
convention above.

- **`rate = 0`** takes the linear branch: `FV = −(pv + pmt·nper)`.
- **`nper = 0`** returns `−pv`: no time has passed and no payments have been made.
- **`rate ≤ −1`.** Mathematically `(1 + rate)^nper` has no real value for a negative base at a
  non-integer exponent, and the family's siblings differ here. `PMT` rejects `rate ≤ −1` with
  `#NUM!`; `FV` in the reference engine deliberately allows a non-positive base through and computes
  where it can. That divergence between siblings is not incidental — it is the subject of an open
  defect stream recorded in the evidence layer (see *What has not been checked*), and it means
  `FV`'s behaviour at `rate ≤ −1` should be treated as unsettled rather than as specified.
- **Overflow.** Large `rate` with large `nper` overflows the compounding factor; the reference
  engine converts a non-finite result to `#NUM!` rather than publishing an infinity.
- **`type` beyond `{0,1}`**: see *Arguments*.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

Microsoft's page for `FV` does not publish a table of error conditions in the way the securities
functions do; what the Handbook can state is what the shared call model and the reference engine
produce:

| Error | Condition |
|---|---|
| `#VALUE!` | A numeric slot receives non-numeric text or an unsupported value kind |
| propagated | An error value in any argument surfaces as that error |
| `#NUM!` | The computation is not representable — the compounding factor or the result overflows |
| `#VALUE!` | The call is made with fewer than three or more than five arguments |

The `#NUM!` row is the reference engine's non-finite guard, not a documented Excel condition. The
Handbook has not observed any of these in Excel.

## Relationships

- **`PV`** — the same equation solved for `pv`. `PV(rate, nper, pmt, −FV(rate, nper, pmt, −pv))`
  recovers `pv`, and that round trip is the family's most useful self-test.
- **`PMT`** — solved for the payment. Note from the evidence layer that `FV` and `PMT` are **not**
  computed by the same kernel in the reference engine; a `PMT`-shaped identity checked against `FV`
  is genuinely testing two different pieces of code.
- **`NPER`** — solved for the number of periods; the only sibling whose solution needs a logarithm.
- **[IPMT](FUNC.IPMT.md)** and `PPMT` — the split of one payment into interest and principal, both
  defined in terms of the balance this equation tracks.
- **`RATE`** — solved for the rate, and the only sibling that cannot be solved in closed form: it
  iterates.
- **[FVSCHEDULE](FUNC.FVSCHEDULE.md)** — future value under a *varying* rate and no payment stream.
  When the rate is not constant, `FV` is the wrong function and `FVSCHEDULE` is the right one.
- **[EFFECT](FUNC.EFFECT.md) / [NOMINAL](FUNC.NOMINAL.md)** — the conversions that get you a correct
  *periodic* rate to feed here. `FV(EFFECT(r,12), 1, 0, −P)` and `FV(r/12, 12, 0, −P)` agree by
  construction.
- **`NPV`** — discounting an arbitrary cash-flow sequence. `FV` requires the payment to be constant;
  `NPV` does not, and that is the dividing line between the two halves of this family.

## Numerical notes

`FV` looks like two multiplications and a division, and the accuracy is nearly all in
`((1 + r)^n − 1)/r`.

**The small-rate regime is the hard one.** For `|r·n|` small — a low rate over a modest horizon —
`(1+r)^n` is close to 1, so the subtraction cancels catastrophically and the surviving digits are
then divided by a small `r`. The naive form loses roughly as many significant digits as `r` has
leading zeros. The classical remedy is the same one that appears throughout this family:

    ((1 + r)^n − 1)/r  =  expm1(n · log1p(r)) / r

`log1p` computes `ln(1 + r)` without forming `1 + r`, and `expm1` computes `e^x − 1` without forming
the near-1 intermediate. Together they hold full relative accuracy down to the smallest normal
rates. The `expm1`/`log1p` pair is standard in the `fdlibm` lineage every modern libm inherits, and
the annuity application of it is textbook actuarial numerics.

**A merely-tiny rate is not zero.** An implementation that collapses `|r| < ε` to the linear branch
for some fixed `ε` will publish an answer that is smooth but wrong near the threshold, and the error
appears as a discontinuity that no amount of downstream rounding hides. The reference engine's
commentary at commit `473efa3` records that this exact substitution was tried and abandoned in the
sibling kernels in favour of an **exact** `rate == 0` test, with a tiny nonzero rate flowing through
the general path.

**The evaluation form is a decision, not a detail.** Whether the compounding factor is formed as
`(1+r)^n` and the annuity term derived from it, or the annuity term is formed first and the
compounding factor derived, changes the last bits. So does whether the power is evaluated by
`exp(n·ln(1+r))`, by binary exponentiation, or by a correctly rounded `pow`. The reference engine
takes one route for `FV` and a different, explicitly cancellation-stable discount route for `PMT` —
a structural fact recorded in the evidence layer, and the reason a `PMT`-derived cross-check on `FV`
is a real test rather than a tautology.

The Handbook does not claim what Excel does internally.

## What has not been checked

No Handbook vector suite exists for `FV`; `vectors/` publishes nothing at this revision, so no
suite-scoped claim exists for it.

`FV` is, however, one of the better-evidenced surfaces in this batch. Five Excel-comparison records
name it in their subjects, and each says something different about how much is actually known:

- **EV-FIN-0001** — the one production replay of the reference engine against live Excel that covers
  this annuity group on a single corpus under a single predicate. Its figures are a **group total**
  over ten surfaces and its own reader warning forbids distributing them to any one surface,
  including this one.
- **EV-FIN-0002** — a per-surface split of that same rollup, **recomputed by the Handbook** rather
  than published by the upstream source. Its reader warning states plainly that it is not a
  live-Excel measurement in its own right and may not sit beside an oracle run.
- **EV-FIN-0005** — per-surface scores from a **research model**, which the upstream source
  explicitly forbids being reported as reference-engine pass rates. The record exists so that the
  disclaimer travels with the numbers.
- **EV-FIN-0012** — an **open defect stream** on the negative-base-rate lane, jointly covering `FV`
  and two siblings, whose repair is recorded as not landed. This is why `FV` cannot be rendered as
  having no open defect, and it is why the `rate ≤ −1` row in *Result and edge cases* is marked
  unsettled.
- **EV-FIN-0014** — a substrate identification whose most important content for this page is a
  *negative* structural finding: `FV` does not share `PMT`'s kernel. Its scores rate a candidate
  research kernel rather than the shipped surface, and its reader warning says so.

Read those five records for their own scopes, predicates and caveats. What none of them supports is
a statement that `FV` agrees with Excel: two are group totals, one is Handbook arithmetic, one rates
a research model, one is an open defect, and no suite exists.

Inputs worth probing first:

1. **`rate = −1` and `rate < −1` with a non-integer `nper`** — the open defect lane, and the one
   place where `FV` and `PMT` are recorded as disagreeing with each other on the admissible domain.
   This probe is where the live divergence is.
2. **Tiny nonzero rates** — `rate = 1E-13` and `rate = −1E-13` at `nper = 360` — against
   `expm1(n·log1p(r))/r` evaluated to higher precision. Any implementation with an epsilon band
   around zero shows a discontinuity here.
3. **Exactly `rate = 0`**, confirming the linear branch and that its sign convention matches the
   general branch's limit.
4. **`type = 2` and `type = −1`**, which the documentation does not define and the reference engine
   treats as `1`.
5. **A `PV`/`FV` round trip** over a grid of rates and horizons, which needs no external oracle and
   catches any asymmetry between the two derivations.
6. **`FV(EFFECT(r, 12), 1, 0, −1)` against `FV(r/12, 12, 0, −1)`** — an identity that must hold by
   construction and that crosses two different kernels in the same family.
7. **`nper = 0`** (expect `−pv`) and **fractional `nper`**, the degenerate and the
   outside-the-model cases.
8. **Overflow**: a large `rate` with a large `nper`, to see whether the result is `#NUM!` or an
   infinity.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| annuity equation | The single balance equation all five annuity functions solve for different unknowns |
| annuity-immediate | Payments at the end of each period, `type = 0` |
| annuity-due | Payments at the beginning of each period, `type = 1` |
| accumulation factor | `((1 + rate)^nper − 1)/rate`, the future value of `nper` unit end-of-period payments |
| sign convention | Money paid out is negative, money received is positive; all terms sum to zero |
| periodic rate | `rate` expressed in the same period as `nper` |

## Sources

- Microsoft, "FV function" —
  <https://support.microsoft.com/en-us/office/fv-function-2eef9f44-a084-4c61-bdd8-4fe4bb1b71b3>
  (syntax, argument meanings, the unit-consistency rule for `rate` and `nper`, the `type` values,
  and the cash-you-pay-out sign convention).
- Handbook evidence records `EV-FIN-0001`, `EV-FIN-0002`, `EV-FIN-0005`, `EV-FIN-0012` and
  `EV-FIN-0014`, each with its own reader warning and scope.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- OxFunc `crates/oxfunc_core/src/functions/financial_time_value_family.rs` at commit `473efa3` —
  the `fv` kernel, the shared balance equation, the non-positive-base allowance, the `type`
  truncation and the recorded exact-zero-rate discipline in the sibling kernels, all read as
  implementation facts about that engine.
- Handbook projections `data/functions/FUNC.FV.json` and `data/presence/FUNC.FV.json` (arity,
  classification axes, implementing module, the fifteen-surface family and the `BUG-FUNC-015`,
  `BUG-FUNC-034` and `BUG-FUNC-038` defect streams touching it).
