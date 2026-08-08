---
schema: efh.function-page/v1
function_id: FUNC.CUMIPMT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-FIN-0001
  - EV-FIN-0002
  - EV-FIN-0005
  - EV-FIN-0017
open_problems: []
references:
  - work: "Microsoft Learn: WorksheetFunction.CumIPmt method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.cumipmt"
    role: "the parameter list, the type table, the consistent-units rule, the truncation rule and the three documented error conditions — including the inverted inequality recorded on this page"
  - work: "Microsoft Learn: WorksheetFunction.CumPrinc method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.cumprinc"
    role: "the sibling page stating the same error condition with the opposite inequality; cited as the other half of the documentation divergence"
  - work: "Microsoft 365 support: CUMIPMT function"
    locator: "https://support.microsoft.com/en-us/office/cumipmt-function-61067bb0-9016-427d-b95b-1a752af0e606"
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
  The interest half of the cumulative pair: a running total of the interest components of a
  level-payment annuity between two period numbers, summed term by term rather than closed.
---

## What it computes

`CUMIPMT(rate, nper, pv, start_period, end_period, type)` returns the **total interest paid on a
loan between two period numbers, inclusive**.

It sits on top of the standard level-payment annuity. That model is one equation:

>     pv·(1+r)^n  +  pmt·(1 + r·type)·((1+r)^n − 1)/r  +  fv  =  0

with `r` the per-period rate, `n` the number of periods, and `type` 0 for payments at the end of
each period or 1 for payments at the beginning. Solving it for the payment gives the constant
`pmt` that `PMT` returns; every period then splits that payment into interest and principal:

>     interest in period k  =  r × (balance outstanding at the start of period k)
>     principal in period k =  pmt − interest in period k

and

>     CUMIPMT(r, n, pv, s, e, type)  =  Σ_{k = s}^{e}  IPMT(r, k, n, pv, 0, type)

So `CUMIPMT` answers a question that has a closed form — the outstanding balance after *m*
payments is itself an annuity value — but it is *defined* as a sum, and the difference between
computing the sum and computing the closed form is not academic. The reference engine sums the
per-period `IPMT` terms left to right, in that order, deliberately: its source records the
choice and records that it carries a larger numerical residual than the algebraic alternative
`pmt × (e − s + 1) − CUMPRINC`. Which one Excel does is exactly what the evidence attached to
this page is about.

Note that `fv` is fixed at zero: `CUMIPMT` models a loan amortized to nothing. There is no
argument for a balloon payment, which is the usual reason a real amortization schedule and a
`CUMIPMT` total fail to agree.

## Sign conventions

This is where wrong answers in this family come from, and it is worth stating flatly.

**Money you receive is positive; money you pay is negative.** For a loan:

- `pv` is the amount borrowed — money *received* — and so is **positive**.
- The payment is money *paid* — and so is **negative**.
- `CUMIPMT` sums interest components of those negative payments, so **the result is negative**.

A `CUMIPMT` that returns a positive number has been given inputs with the opposite convention
somewhere, and a model that adds `CUMIPMT` to a cost total rather than subtracting it will be
wrong by twice the interest. Wrapping the call in `ABS` is the conventional fix for presentation
and a common source of sign errors in the arithmetic underneath.

The reference engine enforces the loan orientation: it requires `pv` > 0 and `rate` > 0, so it
cannot be used with the deposit convention (negative `pv`) at all.

## Arguments

`CUMIPMT(rate, nper, pv, start_period, end_period, type)` — six arguments, all required.

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

**Periods are numbered from 1.** `start_period` = 1 is the first payment, not the second and not
the zeroth. This differs from [AMORLINC](FUNC.AMORLINC.md) and [AMORDEGRC](FUNC.AMORDEGRC.md),
which count depreciation periods from 0.

**`rate` and `nper` must use the same unit.** Microsoft states it directly: "If you make monthly
payments on a four-year loan at an annual interest rate of 12 percent, use 12%/12 for rate and
4\*12 for nper. If you make annual payments on the same loan, use 12% for rate and 4 for nper."
Feeding an annual rate with a monthly period count is the second most common error in this
family, after the sign convention.

Microsoft also documents that **"nper, start_period, end_period, and type are truncated to
integers"**. The reference engine truncates the first three and does **not** truncate `type`: it
accepts only values within a tiny tolerance of 0 or 1 and rejects anything else, so a `type` of
0.5 errors under the reference engine where the documented truncation rule would make it 0. That
is a documentation-versus-reference-engine divergence.

## Result and edge cases

Returns a `Number`: a negative amount, in the units of `pv`, under the loan convention the
reference engine enforces.

- **`start_period` = `end_period`** gives the single period's interest — the same quantity
  `IPMT` returns for that period.
- **`start_period` = 1 with `type` = 1** is the case an attached defect stream names: under a
  beginning-of-period annuity the first payment happens before any interest has accrued, so the
  first period's interest component is zero, and every implementation has to special-case it.
  See "What has not been checked".
- **`end_period` > `nper`** is rejected by the reference engine. Microsoft's documented error
  list does not contain this condition — it constrains `start_period` and `end_period` against 1
  and against each other, but not against `nper`. Another documented-versus-implemented
  divergence, and one that changes an answer into an error.
- **`rate` = 0** is rejected by the reference engine, so the degenerate zero-interest loan (where
  the answer would be exactly zero) is not available.
- **Very small `rate`** is where the arithmetic gets interesting rather than the semantics; see
  the numerical notes.
- **Long ranges** cost time proportional to `end_period − start_period`, because the sum is
  evaluated term by term.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

Microsoft's Learn page documents three conditions. The first of them is printed with the
inequalities the wrong way round:

> "If rate > 0, nper > 0, or pv > 0, **CumIPmt** generates an error."

Read literally, that sentence says a positive rate on a positive loan is an error, which would
make the function unusable. **The sibling page for [CUMPRINC](FUNC.CUMPRINC.md) states the same
condition as "If rate ≤ 0, nper ≤ 0, or pv ≤ 0"** — the reading everything else supports. The
Handbook records the `CumIPmt` page's inequality as a documentation defect, evidenced by its own
sibling page, and does not adopt it.

The three documented conditions, with that correction noted:

| Condition (documented) | Documented outcome |
|---|---|
| `rate` ≤ 0, `nper` ≤ 0, or `pv` ≤ 0 (page prints `>` for all three) | error |
| `start_period` < 1, `end_period` < 1, or `start_period` > `end_period` | error |
| `type` is any number other than 0 or 1 | error |

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

- **[CUMPRINC](FUNC.CUMPRINC.md)** — the principal half. Same six arguments, same validation,
  same period range. The identity `CUMIPMT + CUMPRINC = pmt × (end_period − start_period + 1)`
  holds exactly in the mathematics and only approximately in floating point, which makes it a
  useful consistency probe rather than a substitute for either function.
- **`IPMT`** — the single-period interest component. `CUMIPMT` is its sum, and the two must agree
  on a one-period range.
- **`PPMT`** — the single-period principal component, and `CUMPRINC`'s summand.
- **`PMT`** — the payment the whole family is built on. Every `CUMIPMT` evaluation computes it
  first, so any `PMT` error is inherited.
- **`FV`, `PV`, `NPER`, `RATE`** — the other faces of the same annuity equation. The Handbook's
  research record documents that Excel does **not** share one annuity helper across them: `PMT`
  uses a cancellation-safe discount form where `FV` and `PV` use the naive power form. A page
  that assumes one shared kernel will mispredict all of them.
- **`ISPMT`** — a different interest function entirely, for loans where principal is repaid in
  equal instalments rather than the payment being level. Not a substitute.
- **Confused with**: `IPMT` (single period versus range) and `ISPMT` (level principal versus
  level payment).

## Numerical notes

1. **Cancellation is the whole story at small rates.** The annuity factor `((1+r)^n − 1)/r`
   loses almost all its significant digits when `r` is a few millionths and `(1+r)^n` is a hair
   above 1. The reference engine avoids it by working in `log1p`/`expm1` form:
   `(1+r)^{-n}` as `exp(−n·log1p(r))` and `1 − (1+r)^{-n}` as `−expm1(−n·log1p(r))`, so the
   dangerous subtraction never happens. The Handbook's research record shows Excel's own `PMT`
   taking the same defensive route while its `FV` and `PV` keep the naive one — one equation,
   two different numerical characters.
2. **Summation order is observable.** Adding `end − start + 1` terms left to right, right to
   left, or pairwise gives different last bits. The reference engine sums left to right and its
   source records that this is the model of Excel's loop, not a convenience.
3. **The algebraic shortcut is a different function numerically.** `pmt × count − CUMPRINC` and
   `Σ IPMT` are equal in exact arithmetic and not in binary64. Choosing between them is a
   compatibility decision, not an optimization.
4. **The balance recurrence amplifies.** Each term is `rate × balance`, and the balance is itself
   an annuity value; an error in `pmt` enters every term with the same sign, so residuals
   accumulate rather than cancel across a long range. This is why cumulative functions are
   harder than their single-period siblings.
5. **`type` = 1 shifts the whole schedule by one payment** and makes the first period's interest
   exactly zero. A single mis-handled first period changes every subsequent balance.

## What has not been checked

Four evidence records list this surface among their subjects, and their counts, corpora and
warnings render mechanically beside this page rather than being restated here:

- **`EV-FIN-0001`** — an open-discrepancy record covering a replay of a live-Excel corpus through
  the production reference engine. It is a **group** measurement over ten annuity surfaces, and
  its own reader warning forbids attributing the aggregate to any one of them.
- **`EV-FIN-0002`** — the per-surface split of that same corpus, **recomputed by the Handbook**
  rather than published by any upstream sentence. Its class is local verification, not a
  live-Excel measurement, and its status note characterises this surface as the weakest in the
  financial domain and observes that no upstream prose states its figure at all. The record
  requires companion records to render alongside it.
- **`EV-FIN-0005`** — per-surface scores from a research model on a *different* corpus, which the
  upstream source itself disclaims as not being reference-engine pass rates. Recorded so that
  the disclaimer travels with the numbers.
- **`EV-FIN-0017`** — a defect stream on the `type` = 1 first-period interest, shared with
  `IPMT`, `PPMT` and [CUMPRINC](FUNC.CUMPRINC.md). It was checked against live Excel, but the
  witnesses were compared at a decimal tolerance rather than at the level of the stored bits,
  and the record therefore deliberately publishes **no count**. Its warning is explicit: a page
  that treats this stream as an exactness result is quoting a tolerant comparison as an exact
  one, and this page does not.

What that adds up to: **this surface has been measured against live Excel and it does not agree
with it.** The disagreement is open, its size is recorded in the evidence layer, and the
`type` = 1 first period is a named, reproducible defect.

No Handbook vector suite exists for `CUMIPMT`. The battery on this page is the reference engine
answering its own probes; no Excel was involved in producing it.

Inputs worth probing first:

1. **`start_period` = `end_period` = 1 with `type` = 1**, the first-period case the defect stream
   names, at several rates. One cell per rate, and the expected answer is exactly zero interest.
2. **`end_period` > `nper`.** The reference engine errors; Microsoft documents no such condition.
   Whether Excel returns a number here is a one-cell question with a large blast radius.
3. **`type` = 0.5.** Documented truncation says 0; the reference engine errors.
4. **`rate` = 0**, rejected by the reference engine and by the corrected reading of the
   documentation, where the mathematically correct answer is exactly zero.
5. **Very small rates** — `rate` around 1e−7 with a large `nper` — where the naive annuity factor
   and the `log1p`/`expm1` form diverge visibly. This is the input class that separates the two
   implementation strategies.
6. **`CUMIPMT + CUMPRINC` against `PMT × count`** over the same range, which is exact in the
   mathematics and measures the accumulated floating-point difference.
7. **`CUMIPMT` over `1…nper` against `PMT × nper − pv`**, the total-interest identity for a fully
   amortized loan.
8. **A long range at both `type` values**, to characterise whether the residual grows with the
   number of summed terms.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| level payment | The constant `pmt` that amortizes `pv` to zero over `nper` periods |
| annuity factor | `((1+r)^n − 1)/r`; the cancellation-prone quantity at small rates |
| type | Payment timing: 0 at the end of each period, 1 at the beginning |
| sign convention | Money received is positive, money paid is negative; `CUMIPMT` is negative for a loan |
| period range | The inclusive span `start_period … end_period` the sum runs over |

## Sources

- Microsoft Learn, **WorksheetFunction.CumIPmt method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.cumipmt>. Source of
  the parameter table, the `type` table, the consistent-units paragraph quoted above, the
  truncation rule, and the three error conditions — including the "If rate > 0, nper > 0, or
  pv > 0" sentence recorded above as a documentation defect.
- Microsoft Learn, **WorksheetFunction.CumPrinc method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.cumprinc>. Cited as
  the evidence for that defect: the same condition, stated with "≤".
- Microsoft 365 support, **CUMIPMT function** —
  <https://support.microsoft.com/en-us/office/cumipmt-function-61067bb0-9016-427d-b95b-1a752af0e606>.
  The worksheet-surface page; **not retrieved** for this entry (the host returned HTTP 403).
- Handbook evidence records `EV-FIN-0001`, `EV-FIN-0002`, `EV-FIN-0005`, `EV-FIN-0017`, each of
  which lists this surface in its subjects, and each of which carries its own reader warning.
- Handbook, `content/lastbit/MATERIAL_W109_PRIMITIVES.md` §22 — the recorded finding that Excel's
  `PMT` uses a cancellation-safe form its siblings do not.
- Handbook chapters [01 the value universe](../model/01-value-universe.md),
  [02 coercion and lifting](../model/02-coercion-and-lifting.md),
  [06 claim language](../model/06-claim-language.md),
  [07 implementation options](../model/07-implementation-options.md).
- OxFunc `crates/oxfunc_core/src/functions/cumulative_finance_family.rs` at commit `473efa3` —
  the reference engine's payment kernel, balance recurrence, summation order and validation.
- `data/functions/FUNC.CUMIPMT.json`, `data/presence/FUNC.CUMIPMT.json`,
  `data/battery/FUNC.CUMIPMT.json`.
