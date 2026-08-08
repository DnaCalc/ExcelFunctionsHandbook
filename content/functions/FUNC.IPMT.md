---
schema: efh.function-page/v1
function_id: FUNC.IPMT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-FIN-0001
  - EV-FIN-0002
  - EV-FIN-0005
  - EV-FIN-0017
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
  The interest half of one scheduled payment: the outstanding balance entering the period times the
  periodic rate. PPMT's complement, and the member whose type=1 first period carries an open defect
  stream.
---

# IPMT

## What it computes

`IPMT(rate, per, nper, pv, [fv], [type])` returns the **interest portion of the payment made in
period `per`** of an annuity with constant periodic `rate` and constant payment.

The definition is one line, and everything else follows from it:

    IPMT(per) = rate × B(per)

where `B(per)` is the **outstanding balance entering period `per`** — the balance after `per − 1`
payments have been made. Interest is charged on what you still owe at the start of the period, and
nothing else in the function's behaviour is arbitrary once that is fixed.

The balance itself is the same annuity equation the rest of this family solves. After `k` payments,

    B(k+1) = −( pv·(1 + rate)^k  +  PMT·(1 + rate·type)·((1 + rate)^k − 1)/rate )

with `PMT = PMT(rate, nper, pv, fv, type)`. So `IPMT` is a *derived* quantity: it needs the payment,
and the payment needs the whole annuity. This is why an `IPMT` implementation is structurally more
fragile than a `PMT` implementation — it inherits every error in `PMT` and then adds a balance
recurrence on top.

Three exact identities hold by construction and are the natural tests:

1. **`IPMT(per) + PPMT(per) = PMT`** for every `per`. The payment splits into interest and
   principal, exhaustively.
2. **`Σ IPMT(k) for k = 1..nper = CUMIPMT(1, nper)`**, and likewise for principal.
3. **`IPMT` decreases in magnitude with `per`** for a conventional amortizing loan, because the
   balance falls; `PPMT` rises by the same amount.

### The `type = 1` first period

With `type = 1` payments are made at the *beginning* of each period. The first payment therefore
occurs at time zero, before any interest has accrued, so

    IPMT(1) = 0        when type = 1

and the balance entering period 2 is reduced by that whole first payment before the first interest
charge. This is not a rounding convention; it is what beginning-of-period timing means. It is also
the sharpest edge in the function, and it is the subject of an open defect stream recorded in the
evidence layer — see *What has not been checked*.

With `type = 0` the first payment is at the end of period 1, so `IPMT(1) = rate × pv` (negated by
the sign convention), which is the cleanest closed-form value the function produces and the best
first probe.

### The sign convention

The family's convention holds here unchanged: **money you pay out is negative**. For a loan entered
as a positive `pv` (cash you received), `PMT`, `IPMT` and `PPMT` are all negative — they are money
leaving. Reading `IPMT` as "the interest I paid" and expecting a positive number is the standard
misreading; wrap it in `ABS` or negate `pv`, consistently.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `rate` | The interest rate **per period**. Required. | — |
| `per` | The period whose interest is wanted; must be in the range `1` to `nper`. Required. | — |
| `nper` | The total number of payment periods. Required. | — |
| `pv` | The present value — the lump sum the future payments are worth now. Required. | — |
| `fv` | The balance to reach after the last payment. Optional. | 0 |
| `type` | `0` = payments at the **end** of the period; `1` = at the **beginning**. Optional. | 0 |

- **Argument order is `rate, per, nper` — `per` before `nper`.** Its sibling `PMT` has no `per`, so
  a formula edited from `PMT` to `IPMT` by inserting an argument in the wrong slot computes a
  plausible wrong answer. This is the most common structural mistake with this function.
- **`per` is 1-based** and must satisfy `1 ≤ per ≤ nper`. Contrast [ISPMT](FUNC.ISPMT.md), whose
  period index runs from 0 — the two functions have similar names, similar signatures and different
  indexing.
- **`rate` and `nper` must use the same period.** A 12% annual rate on a monthly schedule is
  `0.12/12` with `nper` in months.
- Omitted optional arguments and blank cells both arrive as `0` in the reference engine.
- **`type`** is documented as `0` or `1`; the reference engine truncates it and treats any nonzero
  value as beginning-of-period.

## Result and edge cases

Returns `Number` — an interest amount carrying the family sign convention.

- **`per` outside `1 … nper`** is `#NUM!`. Note that `per = 0` is an error here even though it is
  the natural first index in `ISPMT`.
- **`rate = 0`** makes every interest charge zero, so `IPMT` returns zero for every `per`. The
  reference engine special-cases exactly `rate == 0` — not a tolerance band — and its commentary
  records that a merely tiny rate (positive or negative) flows through the general path and returns
  a correspondingly tiny signed value rather than being flattened to zero. That distinction is
  observable and is a probe target.
- **`type = 1` with `per = 1`** returns zero, as above.
- **Near the final period** the balance approaches `fv`, and the interest is the difference of two
  nearly equal large numbers; see *Numerical notes*.
- **`rate ≤ −1`** is rejected as `#NUM!` in the reference engine, following `PMT`'s domain guard
  rather than `FV`'s more permissive one. The siblings differ here and the divergence is live; see
  the [FV](FUNC.FV.md) page.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

Microsoft's `IPMT` page does not publish a table of error conditions in the way the securities
functions do. What the Handbook can state:

| Error | Condition |
|---|---|
| `#NUM!` | `per < 1` or `per > nper` |
| `#NUM!` | `rate ≤ −1`, or a non-representable result |
| `#VALUE!` | A numeric slot receives non-numeric text or an unsupported value kind |
| propagated | An error value in any argument surfaces as that error |
| `#VALUE!` | The call is made with fewer than four or more than six arguments |

The `per` range condition is the one every reference states; the others are the reference engine's
behaviour at commit `473efa3` under the shared call model. The Handbook has not observed any of
them in Excel.

## Relationships

- **`PPMT`** — the principal half. `IPMT + PPMT = PMT` exactly, and the reference engine literally
  computes `PPMT` as `PMT − IPMT`, which means the two share every error `IPMT` has.
- **`PMT`** — the constant payment `IPMT` decomposes. `IPMT` calls it internally; a `PMT`
  inaccuracy propagates into every period's interest.
- **`CUMIPMT` / `CUMPRINC`** — the running sums over a range of periods, and the natural consistency
  check against a column of `IPMT` calls.
- **[ISPMT](FUNC.ISPMT.md)** — a *different* loan model entirely: straight-line principal repayment
  rather than a constant total payment, and a 0-based period index. The names are one letter apart
  and the functions are not comparable.
- **[FV](FUNC.FV.md) / `PV` / `NPER` / `RATE`** — the rest of the annuity family, all solving the
  same equation.
- **Confused with**: the interest actually charged by a real lender, which depends on day counts,
  rounding to cents and payment-date conventions that no Excel annuity function models. `IPMT` is
  the idealized schedule, and a bank statement will not match it to the penny.

## Numerical notes

`IPMT` is the most numerically demanding member of this family, for a structural reason: it is
`rate` times a *balance*, and the balance is a difference of two compounding terms that converge
toward each other as the loan amortizes.

**Cancellation near payoff.** In the last few periods of a fully-amortizing loan the outstanding
balance is a small residue of two large quantities — the compounded principal and the accumulated
payments — that nearly cancel. The relative error of the balance is amplified by the ratio of those
quantities to their difference, which grows without bound as the balance approaches `fv`. The
reference engine's own commentary at commit `473efa3` records that near-payoff periods still suffer
cancellation and that a fully-stable last-period path is *not* solved in that engine. That is an
implementation fact worth reading as a general warning: an `IPMT` figure in the last periods of a
long schedule carries far fewer good digits than one in the first.

**The chain of dependencies.** `IPMT` needs `PMT`, and `PMT` needs an accurate `((1+r)^n − 1)/r`.
The whole family's small-rate discipline — `expm1(n·log1p(r))` instead of `(1+r)^n − 1`, and an
**exact** `rate == 0` test instead of an epsilon band — matters twice here, once inside `PMT` and
once inside the balance recurrence. The reference engine's commentary records that it deliberately
routes the balance recurrence through the same op-order as its `PMT` so that the two do not drift
apart; that is a coherence decision, and it is the right kind of decision, but it also means an
error in the shared substrate appears in both places at once and cannot be caught by comparing them.

**The `type = 1` reconstruction.** Beginning-of-period timing shifts the balance recurrence by one
period and subtracts one payment. Getting that shift wrong produces an answer that is correct in the
middle of the schedule and wrong at the ends — the hardest kind of error to notice. The evidence
layer carries an open stream on exactly this, and the record's own reader warning notes that the
witnesses supporting it agree at a *tolerant* predicate rather than at the bit level.

For the underlying identities — the amortization recurrence and the interest/principal split — any
actuarial or corporate-finance text develops them; the numerical treatment of `expm1`/`log1p` is the
`fdlibm` lineage that modern libms inherit.

## What has not been checked

No Handbook vector suite exists for `IPMT`; `vectors/` publishes nothing at this revision, so no
suite-scoped claim exists for it.

Four Excel-comparison records name `IPMT` in their subjects, and each is narrower than it looks:

- **EV-FIN-0001** — the production replay covering this annuity group on one corpus under one
  predicate. Its figures are a **group total** over ten surfaces; its own reader warning forbids
  attributing them to any single surface.
- **EV-FIN-0002** — the per-surface split of that rollup, **recomputed by the Handbook**, not
  published upstream. Its reader warning states that it is not a live-Excel measurement in its own
  right.
- **EV-FIN-0005** — per-surface scores from a **research model**, which the upstream source
  explicitly forbids being reported as reference-engine pass rates.
- **EV-FIN-0017** — the open **type-1 first-period defect stream** covering `IPMT` and three
  siblings. It deliberately publishes **no count**, because its live-Excel witnesses agree at a
  tolerant decimal predicate rather than at the bit level; its reader warning says that rendering
  these surfaces as exact on the strength of it would be quoting a tolerant comparison as an exact
  one.

Read those four for their own scopes. Taken together they establish that `IPMT` has been inside
live-Excel comparisons and that **nothing in the record supports a statement that `IPMT` agrees with
Excel**. The implementing module additionally carries open defect streams touching this surface —
`BUG-FUNC-015`, `BUG-FUNC-034`, `BUG-FUNC-037` and `BUG-FUNC-038` — the second of which names the
type-1 beginning-payment behaviour directly.

Inputs worth probing first:

1. **`IPMT(rate, 1, nper, pv)` with `type = 0`**, whose value must be exactly `−rate·pv`. It is the
   only closed-form value the function produces, it needs no oracle, and any disagreement localizes
   immediately.
2. **`IPMT(rate, 1, nper, pv, 0, 1)` with `type = 1`**, which must be zero. This is the open defect
   lane and the highest-value probe on the page.
3. **`IPMT + PPMT` against `PMT`** for every period of a schedule — an identity that must hold to
   the last bit if the two are derived consistently, and that is cheap to run over a whole column.
4. **The last two periods of a 360-period schedule**, where the cancellation analysis above predicts
   the fewest good digits.
5. **Tiny nonzero rates** — `1E-13` and `−1E-13` at `per = 1`, `nper = 360` — to confirm the exact
   zero-rate test rather than an epsilon band, including the sign of the tiny answer.
6. **`per = 0`, `per = nper`, `per = nper + 1`** — the documented `#NUM!` boundary at both ends.
7. **`Σ IPMT` against `CUMIPMT`** over the same range, which crosses two implementations of the
   same quantity.
8. **`type = 2`**, undefined in the documentation and treated as `1` by the reference engine.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| outstanding balance | The amount still owed entering a period; `IPMT` is `rate` times this |
| amortization schedule | The period-by-period split of a constant payment into interest and principal |
| annuity-due | Beginning-of-period payments, `type = 1`, where the first period's interest is zero |
| near-payoff cancellation | The precision loss when the balance is a small residue of two large terms |
| sign convention | Money paid out is negative; a loan's `IPMT` is negative for a positive `pv` |

## Sources

- Microsoft, "IPMT function" —
  <https://support.microsoft.com/en-us/office/ipmt-function-5cce0ad6-8402-4a41-8d29-61a0b054cb6f>
  (syntax, argument meanings including the `1 ≤ per ≤ nper` requirement, the `type` values, the
  unit-consistency rule and the sign convention).
- Handbook evidence records `EV-FIN-0001`, `EV-FIN-0002`, `EV-FIN-0005` and `EV-FIN-0017`, each with
  its own reader warning and scope.
- Handbook, [FV](FUNC.FV.md) — the shared annuity equation and the sibling domain divergence at
  `rate ≤ −1`.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- OxFunc `crates/oxfunc_core/src/functions/financial_time_value_family.rs` at commit `473efa3` —
  the `ipmt` kernel, its balance recurrence, its exact zero-rate test, its `per` range guard and its
  recorded near-payoff cancellation residual, read as implementation facts about that engine.
- Handbook projections `data/functions/FUNC.IPMT.json` and `data/presence/FUNC.IPMT.json` (arity,
  classification axes, implementing module and the `BUG-FUNC-015`, `BUG-FUNC-034`, `BUG-FUNC-037`
  and `BUG-FUNC-038` defect streams).
