---
schema: efh.function-page/v1
function_id: FUNC.CUMPRINC
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-FIN-0018
  - EV-FIN-0001
  - EV-FIN-0002
  - EV-FIN-0005
  - EV-FIN-0017
open_problems: []
references:
  - work: "Microsoft Learn: WorksheetFunction.CumPrinc method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.cumprinc"
    role: "the parameter list, the type table, the consistent-units rule, the truncation rule and the three documented error conditions"
  - work: "Microsoft Learn: WorksheetFunction.CumIPmt method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.cumipmt"
    role: "the sibling page stating the same error condition with the opposite inequality; cited as the other half of the documentation divergence"
  - work: "Microsoft 365 support: CUMPRINC function"
    locator: "https://support.microsoft.com/en-us/office/cumprinc-function-94a4516d-bd65-41a1-bc16-053a6af4c04d"
    role: "the worksheet-surface page; cited but not retrieved — the host returned HTTP 403 to the Handbook's fetch"
episodes: []
body_sections:
  - What it computes
  - Sign conventions
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: cumulative_finance_family
role_in_family: >-
  The principal half of the cumulative pair: how much of the loan is actually repaid between two
  period numbers, and the surface whose upstream prose and upstream replay describe different
  functions.
---

## What it computes

`CUMPRINC(rate, nper, pv, start_period, end_period, type)` returns the **total principal repaid
on a loan between two period numbers, inclusive** — the amount by which the outstanding balance
falls over that span.

It is built on the same level-payment annuity as its sibling. The model is one equation:

>     pv·(1+r)^n  +  pmt·(1 + r·type)·((1+r)^n − 1)/r  +  fv  =  0

Solving for the payment gives the constant `pmt`; each period then splits it:

>     interest in period k   =  r × (balance outstanding at the start of period k)
>     principal in period k  =  pmt − interest in period k

and

>     CUMPRINC(r, n, pv, s, e, type)  =  Σ_{k = s}^{e}  PPMT(r, k, n, pv, 0, type)

Because `fv` is fixed at zero, summing over the whole schedule gives back the loan:
`CUMPRINC(r, n, pv, 1, n, type)` is `−pv` in exact arithmetic. That identity is the single best
sanity check available for this function, and it is exact in the mathematics and only
approximate in binary64 — which makes it a measurement rather than an assertion.

The reference engine computes the sum term by term, deriving each `PPMT` as
`pmt − IPMT`, in ascending period order.

## Sign conventions

**Money you receive is positive; money you pay is negative.** For a loan `pv` is positive — cash
in — and every payment is negative, so **`CUMPRINC` returns a negative number**. Repaying
principal is money leaving.

The full-schedule identity above is the clearest statement of the convention: borrow `pv`, repay
`−pv`. A model that treats `CUMPRINC` as a positive "principal repaid" figure without negating
it will report the loan growing.

The reference engine enforces the loan orientation: `pv` > 0 and `rate` > 0 are required, so the
deposit convention (negative `pv`) is unavailable.

## Arguments

`CUMPRINC(rate, nper, pv, start_period, end_period, type)` — six arguments, all required, and
identical in meaning to [CUMIPMT](FUNC.CUMIPMT.md#arguments).

| Argument | Meaning (Microsoft's wording) | Notes |
|---|---|---|
| `rate` | "The interest rate." | **Per period**, not per year |
| `nper` | "The total number of payment periods." | Truncated to an integer |
| `pv` | "The present value." | The loan amount; positive for a loan |
| `start_period` | "The first period in the calculation. Payment periods are numbered beginning with 1." | Truncated |
| `end_period` | "The last period in the calculation." | Truncated |
| `type` | "The timing of the payment." | 0 or 1 |

| `type` | Timing (Microsoft's table) |
|---|---|
| 0 (zero) | Payment at the end of the period |
| 1 | Payment at the beginning of the period |

**Periods are numbered from 1.** **`rate` and `nper` must use the same unit** — Microsoft:
"If you make monthly payments on a four-year loan at an annual interest rate of 12 percent, use
12%/12 for rate and 4\*12 for nper."

Microsoft documents that **"nper, start_period, end_period, and type are truncated to
integers"**. The reference engine truncates the first three and does **not** truncate `type`,
accepting only values within a tiny tolerance of 0 or 1. A `type` of 0.5 therefore errors under
the reference engine where the documented rule would make it 0 — a
documentation-versus-reference-engine divergence, shared with the sibling.

## Result and edge cases

Returns a `Number`: a negative amount in the units of `pv`, under the loan convention the
reference engine enforces.

- **`start_period` = 1, `end_period` = `nper`** returns `−pv` in exact arithmetic. Any deviation
  is accumulated floating-point error, and measuring it is the most informative single experiment
  on this page.
- **`type` = 1** shifts the schedule: the first payment happens before interest accrues, so the
  first period's principal component is the whole payment. This is the same first-period
  boundary an attached defect stream names for the interest side.
- **`end_period` > `nper`** is rejected by the reference engine. Microsoft's documented error
  list constrains `start_period` and `end_period` against 1 and against each other, but **not**
  against `nper`. Divergence recorded.
- **`rate` = 0** is rejected by the reference engine, so the degenerate zero-interest loan —
  where each period repays exactly `pv/nper` — is unavailable.
- **Early periods repay little principal**, late periods almost all of it; the function is
  strongly non-linear in `start_period` even though the payment is constant.
- **Long ranges** cost time proportional to the number of periods summed.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

Microsoft's Learn page for this function documents three conditions, and states the first one
with the inequalities the right way round:

> "If rate ≤ 0, nper ≤ 0, or pv ≤ 0, **CumPrinc** generates an error."

| Condition (documented) | Documented outcome |
|---|---|
| `rate` ≤ 0, `nper` ≤ 0, or `pv` ≤ 0 | error |
| `start_period` < 1, `end_period` < 1, or `start_period` > `end_period` | error |
| `type` is any number other than 0 or 1 | error |

**The sibling page disagrees with this one.** Microsoft's `CumIPmt` page states the same
condition as "If rate > 0, nper > 0, or pv > 0 … generates an error" — the inequalities
inverted. Two pages documenting the same validation for two functions that share an
implementation cannot both be right; this page's form is the one the rest of the documentation
and every implementation support. Recorded on [CUMIPMT](FUNC.CUMIPMT.md#errors) as a
documentation defect.

The reference engine's conditions, recorded as the reference engine's:

| Error | Condition (reference engine) |
|---|---|
| `#NUM!` | `rate` ≤ 0, or `pv` ≤ 0 |
| `#NUM!` | `nper`, `start_period` or `end_period` truncates below 1 |
| `#NUM!` | `start_period` > `end_period`, or `end_period` > `nper` |
| `#NUM!` | `type` is not within a tiny tolerance of 0 or 1 |
| `#VALUE!` | `rate` or `pv` is not finite, or a period argument is not finite |
| propagated | An error value in any argument surfaces as that error |

## Relationships

- **[CUMIPMT](FUNC.CUMIPMT.md)** — the interest half. `CUMIPMT + CUMPRINC` equals
  `pmt × (end_period − start_period + 1)` in exact arithmetic; in floating point the residual is
  a measurement of both functions at once.
- **`PPMT`** — the single-period principal component and this function's summand. The two must
  agree on a one-period range.
- **`IPMT`** — the interest component, and `CUMIPMT`'s summand.
- **`PMT`** — the payment every evaluation starts from. Any `PMT` error is inherited by both
  cumulative functions with the same sign in every term.
- **`FV`, `PV`, `NPER`, `RATE`** — the other faces of the same annuity equation. The Handbook's
  research record documents that Excel does **not** share one annuity kernel across them: `PMT`
  uses a cancellation-safe discount form where `FV` and `PV` use the naive power form.
- **`ISPMT`** — a different loan model (level principal rather than level payment); not a
  substitute.
- **Confused with**: `PPMT` (single period versus range), and with a hand-built amortization
  table, which will disagree in the last digits precisely because it accumulates in a different
  order.

## Numerical notes

1. **The subtraction is where the digits go.** Each summand is `pmt − r × balance`; early in a
   loan those two quantities are close, so the principal component is a small difference of two
   larger numbers. That is ordinary cancellation, and it means `CUMPRINC` over an early range is
   intrinsically harder than `CUMIPMT` over the same range.
2. **The annuity factor is cancellation-prone at small rates.** `((1+r)^n − 1)/r` loses most of
   its significant digits when `r` is a few millionths. The reference engine works in
   `log1p`/`expm1` form — `exp(−n·log1p(r))` and `−expm1(−n·log1p(r))` — so that subtraction
   never happens. The Handbook's research record shows Excel's `PMT` making the same choice while
   its `FV` and `PV` do not, which is why one annuity kernel does not predict all five functions.
3. **Summation order is observable**, and so is the choice between summing `PPMT` terms and
   forming `pmt × count − CUMIPMT`. Those are different functions in binary64.
4. **The full-schedule identity is a calibration instrument.** `CUMPRINC(r, n, pv, 1, n, type)`
   should be `−pv`; the deviation is a direct, dimensioned measure of the accumulated error, and
   it needs no oracle to observe.
5. **Boundary periods are where implementations fork**: the first period under `type` = 1, and
   the last period, where the remaining balance should land exactly on zero rather than on a
   small residual.

## What has not been checked

Five evidence records list this surface among their subjects. Their counts, corpora and warnings
render mechanically beside this page and are not restated here:

- **`EV-FIN-0018`** — an open-discrepancy record specific to this surface, and the most
  interesting one on the page. It records that the upstream catalogue row's *prose* — which
  localises the problem to boundary-sensitive accumulation on the strength of two reconciliation
  witnesses — is **stale against a primary artifact that a neighbouring row cites by path**. A
  small-residual boundary story and the replay's worst row cannot both be describing the same
  function, and the prose is the older of the two. The record exists so that the contradiction is
  published rather than smoothed. It requires a companion record to render alongside it.
- **`EV-FIN-0001`** — the group replay of a live-Excel corpus through the production reference
  engine, covering ten annuity surfaces. Its reader warning forbids attributing the aggregate to
  any single surface.
- **`EV-FIN-0002`** — the per-surface split of that corpus, **recomputed by the Handbook** and
  stated by no upstream sentence. Class: local verification, not a live-Excel measurement.
- **`EV-FIN-0005`** — per-surface scores from a research model on a *different* corpus, which the
  upstream source itself disclaims as not being reference-engine pass rates.
- **`EV-FIN-0017`** — the `type` = 1 first-period defect stream, shared with `IPMT`, `PPMT` and
  [CUMIPMT](FUNC.CUMIPMT.md). Verified against live Excel, but at a decimal tolerance rather than
  at the level of the stored bits; the record deliberately publishes no count, and warns that
  treating it as an exactness result quotes a tolerant comparison as an exact one.

The honest summary: **this surface has been measured against live Excel, it does not agree, and
the upstream record's own description of why is contradicted by its own replay.** That
contradiction is published, not resolved.

No Handbook vector suite exists for `CUMPRINC`. The battery on this page is the reference engine
answering its own probes; no Excel was involved in producing it.

Inputs worth probing first:

1. **`CUMPRINC(r, n, pv, 1, n, type)` against `−pv`**, across a grid of rates and term lengths.
   Exact in the mathematics; the deviation is a self-calibrating error measurement that needs no
   oracle, and it is the fastest way to find the input regions where this function is weakest.
2. **`start_period` = `end_period` = 1 with `type` = 1**, the first-period case the defect stream
   names, where the principal component should be the entire payment.
3. **`end_period` > `nper`.** The reference engine errors; the documentation lists no such
   condition.
4. **`type` = 0.5**, where the documented truncation and the reference engine disagree.
5. **Very small rates with large `nper`**, the regime where the naive annuity factor and the
   `log1p`/`expm1` form part company.
6. **`CUMPRINC + CUMIPMT` against `PMT × count`** over the same range, measuring both functions
   with one identity.
7. **Early ranges versus late ranges** of the same schedule, to see whether the residual tracks
   the cancellation in the early principal components as the boundary-accumulation hypothesis in
   `EV-FIN-0018` predicts — the probe that would confirm or retire that stale prose.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| principal component | The part of a level payment that reduces the outstanding balance |
| level payment | The constant `pmt` that amortizes `pv` to zero over `nper` periods |
| annuity factor | `((1+r)^n − 1)/r`; the cancellation-prone quantity at small rates |
| full-schedule identity | `CUMPRINC(r, n, pv, 1, n, type) = −pv`, exact in the mathematics |
| sign convention | Money received is positive, money paid is negative; `CUMPRINC` is negative for a loan |

## Sources

- Microsoft Learn, **WorksheetFunction.CumPrinc method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.cumprinc>. Source of
  the parameter table, the `type` table, the consistent-units paragraph, the truncation rule and
  the three error conditions quoted above.
- Microsoft Learn, **WorksheetFunction.CumIPmt method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.cumipmt>. Cited for
  the inverted statement of the same error condition, recorded as a documentation defect.
- Microsoft 365 support, **CUMPRINC function** —
  <https://support.microsoft.com/en-us/office/cumprinc-function-94a4516d-bd65-41a1-bc16-053a6af4c04d>.
  The worksheet-surface page; **not retrieved** for this entry (the host returned HTTP 403).
- Handbook evidence records `EV-FIN-0018`, `EV-FIN-0001`, `EV-FIN-0002`, `EV-FIN-0005` and
  `EV-FIN-0017`, each of which lists this surface in its subjects, and each of which carries its
  own reader warning.
- Handbook, `content/lastbit/MATERIAL_W109_PRIMITIVES.md` §22 — the recorded finding that Excel's
  `PMT` uses a cancellation-safe form its siblings do not.
- Handbook chapters [01 the value universe](../model/01-value-universe.md),
  [02 coercion and lifting](../model/02-coercion-and-lifting.md),
  [06 claim language](../model/06-claim-language.md),
  [07 implementation options](../model/07-implementation-options.md).
- OxFunc `crates/oxfunc_core/src/functions/cumulative_finance_family.rs` at commit `473efa3` —
  the reference engine's payment kernel, balance recurrence, summation order and validation.
- `data/functions/FUNC.CUMPRINC.json`, `data/presence/FUNC.CUMPRINC.json`,
  `data/battery/FUNC.CUMPRINC.json`.
