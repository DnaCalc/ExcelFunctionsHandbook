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
  - Notes
  - Excel versions, platforms and CPUs
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
recurrence on top. Excel's route for `fv` 0 and `type` 0 has no such recurrence: FN-IPMT-001 gives
it as a direct formula that reuses `PMT`'s quotient.

Three identities hold by construction in exact arithmetic and are the natural tests:

1. **`IPMT(per) + PPMT(per) = PMT`** for every `per`. The payment splits into interest and
   principal, exhaustively. Excel's stored answers do not satisfy it: on one 30-year loan
   `PMT − IPMT` equals `PPMT` in 2 of 12 periods and `PMT − PPMT` equals `IPMT` in 7 (FN-PPMT-001).
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
- **Near the final period** the balance approaches `fv`, and a balance recurrence takes the
  difference of two nearly equal large numbers. Excel's route for `fv` 0 and `type` 0 does not
  (FN-IPMT-001); see *Numerical notes*.
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

- **`PPMT`** — the principal half. `IPMT + PPMT = PMT` exactly in the mathematics, not in Excel's
  stored answers (FN-PPMT-001), and the reference engine literally
  computes `PPMT` as `PMT − IPMT`, which means the two share every error `IPMT` has.
- **`PMT`** — the constant payment `IPMT` decomposes. `IPMT` reuses its quotient (FN-IPMT-001); an
  inaccuracy there propagates into every period's interest.
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

`IPMT` is the most numerically demanding member of this family, for a structural reason: written
from the definition it is `rate` times a *balance*, and the balance is a difference of two
compounding terms that converge toward each other as the loan amortizes.

**Cancellation near payoff.** In the last few periods of a fully-amortizing loan the outstanding
balance is a small residue of two large quantities — the compounded principal and the accumulated
payments — that nearly cancel. The relative error of the balance is amplified by the ratio of those
quantities to their difference, which grows without bound as the balance approaches `fv`. The
reference engine's own commentary at commit `473efa3` records that near-payoff periods still suffer
cancellation and that a fully-stable last-period path is *not* solved in that engine. That is an
implementation fact about a balance recurrence, not about Excel: for `fv` 0 and `type` 0,
FN-IPMT-001 forms interest from `q × em_m`, where `em_m` is an expm1 over the periods left, with no
such subtraction.

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
live-Excel comparisons and that **nothing in those four supports a statement that `IPMT` agrees with
Excel**. The later record is FN-IPMT-001: with `fv` 0 and `type` 0 its route matches all 95 recorded
rows. It stays open for nonzero `fv` and for `type` 1, where `PMT`'s open numerator of `q` and open
factor `K` carry through. The implementing module additionally carries open defect streams touching
this surface — `BUG-FUNC-015`, `BUG-FUNC-034`, `BUG-FUNC-037` and `BUG-FUNC-038` — the second of
which names the type-1 beginning-payment behaviour directly.

Inputs worth probing first:

1. **`IPMT(rate, 1, nper, pv)` with `type = 0`**, whose value must be exactly `−rate·pv`. It is the
   only closed-form value the function produces, it needs no oracle, and any disagreement localizes
   immediately.
2. **`IPMT(rate, 1, nper, pv, 0, 1)` with `type = 1`**, which must be zero. This is the open defect
   lane and the highest-value probe on the page.
3. **`IPMT + PPMT` against `PMT`** for every period of a schedule. In Excel's stored answers it does
   not hold in general (FN-PPMT-001), because the two take different routes (FN-IPMT-001).
4. **The last two periods of a 360-period schedule**, where a balance recurrence has the fewest good
   digits and the route in FN-IPMT-001 has no such subtraction.
5. **Tiny nonzero rates** — `1E-13` and `−1E-13` at `per = 1`, `nper = 360` — to confirm the exact
   zero-rate test rather than an epsilon band, including the sign of the tiny answer.
6. **`per = 0`, `per = nper`, `per = nper + 1`** — the documented `#NUM!` boundary at both ends.
7. **`Σ IPMT` against `CUMIPMT`** over the same range, which crosses two implementations of the
   same quantity.
8. **`type = 2`**, undefined in the documentation and treated as `1` by the reference engine.
9. **Nonzero `fv` and `type = 1`**, which FN-IPMT-001 leaves open.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `IPMT` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-IPMT-001: IPMT and PPMT reuse PMT's quotient, one through expm1 and the other through exp

*Detail for matching Excel's bits; measured; recorded 2026-11-04.* Applies to IPMT, PPMT.

With RN(v) meaning v rounded to 64 bits on the x87 unit and then stored to binary64, take L and
q from PMT (FN-PMT-001; with nonzero fv, q = RN(N/em) with an fv numerator that is open). For
period k of nper with end-of-period payments, m = nper-k+1, t = -(m*L), em_m is the expm1 of t
by the route in FN-PMT-001, and u = exp(t) stored. Then IPMT = -RN(RN(RN(q*em_m) -
RN(fv*u))*rate) and PPMT = RN(RN(RN(q - fv)*u)*rate). Interest takes the Kahan expm1, and
principal takes the exponential it starts from: 1+em_m fits 36 of 74 principal rows where u
fits all 74. So neither answer is the payment less the other (FN-PPMT-001).

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `=IPMT(0.05/12,12,360,200000)` | as typed | `-822.0867740866778` (`0xc089b0b1b69cc35f`) | `-822.0867740866777` (`0xc089b0b1b69cc35e`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=PPMT(0.05/12,12,360,200000)` | as typed | `-251.5564719376004` (`0xc06f71ce9e3ca456`) | `-251.5564719376003` (`0xc06f71ce9e3ca452`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Correct value: mpmath at 800 bits with rate the binary64 0x3f71111111111111 that 0.05/12
evaluates to; correctly rounded.

Basis (measured): fv 0, type 0: IPMT 95 of 95 and PPMT 74 of 74 recorded rows, plus 64 of 64
fresh PPMT(r,1,1,pv) and PPMT(r,1,2,pv) readings. Nonzero fv, type 0: PPMT 25 of 27, IPMT 18 of
27. Over every recorded row in the evidence repository IPMT matches 48 of 57 and PPMT 26 of 33.
Replaying the formulas in exact rational arithmetic, with exp and ln correctly rounded, gives
all 12 PPMT and all 12 IPMT answers of the loan below.

- Open: Nonzero fv: PMT's open numerator of q carries through.
- Open: type 1: PMT's open factor K carries through.

Evidence:

- [`content/notes/evidence/2026-11-04/annuity-EVIDENCE.txt`](../../content/notes/evidence/2026-11-04/annuity-EVIDENCE.txt), section UPDATE — research write-up
- DnaCalc `crates/calc-func/src/ported/financial_time_value_family.rs`, end_payment_parts at `6c6cbd31c9` — the formulas in the product; the commit message carries the row counts
- DnaCalc `crates/calc-func/tests/f3_financial.rs`, ipmt_and_ppmt_use_the_quotient_of_pmt at `6c6cbd31c9` — eight catalogue readings; failed before the change
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcj9k_annuity.capture.json`, sheet dcj9k_annuity, row 750, column 4 at `c9f9d35416` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcj9k_annuity.capture.json`, sheet dcj9k_annuity, row 749, column 4 at `c9f9d35416` (readings)

<!-- efh:notes end -->

## Excel versions, platforms and CPUs

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.IPMT.json; edits here are overwritten -->

Readings of IPMT with a recorded Excel build: 388 input(s) over 2 recorded environment(s),
Excel 16.0 build(s) 19929, 20131, all on Windows; recorded CPUs: AMD Zen 2. 31 input(s) were
also read in captures that record no build (some may repeat inputs counted above); they are not
counted. No input has been read in two environments, so no difference is known and none is
ruled out. A CPU dependence is expected and untested (see below). No reading from a Mac, Excel
for the web, 32-bit Excel or an Intel CPU is recorded.

Readings of `IPMT` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 357 | date not recorded |
| 16.0 build 20131 | Windows, bitness not recorded | AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | 31 | 2026-07-23 |

No check has read the same inputs in two environments, so the absence of a listed difference says nothing about other builds, platforms or CPUs.

Differences observed: none.

Expected, not yet tested (XS-IPMT-001, cpu): Handbook evidence record EV-FIN-0005 marks 7 of
its counts for this function as CPU-scoped: the counted result depends on how the host CPU
executes x87 instructions. Every reading recorded here was taken on an AMD Zen 2 processor or
on a host whose CPU was not recorded; no reading on an Intel CPU, a Mac or Excel for the web
exists, so a CPU difference is neither observed nor ruled out.

Evidence for every row: [`content/evidence/environments/FUNC.IPMT.json`](../../content/evidence/environments/FUNC.IPMT.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

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
