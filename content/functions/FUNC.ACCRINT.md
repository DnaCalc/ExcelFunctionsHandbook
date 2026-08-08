---
schema: efh.function-page/v1
function_id: FUNC.ACCRINT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-FIN-0020
open_problems: []
references:
  - work: "Microsoft Learn: WorksheetFunction.AccrInt method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.accrint"
    role: "the documented parameter list, the required/optional split, and the day-count basis table"
  - work: "Microsoft 365 support: ACCRINT function"
    locator: "https://support.microsoft.com/en-us/office/accrint-function-fe45d089-6722-4fb3-9379-e1f911d8dc74"
    role: "the worksheet-surface page that carries calc_method and a published accrual equation; cited but not retrieved — the host returned HTTP 403 to the Handbook's fetch"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - The calc_method argument
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: bond_core_family
role_in_family: >-
  The accrual half of the bond core: it measures elapsed coupon time rather than discounting
  future cash, and it is the only member whose answer is a sum over a reconstructed coupon
  schedule rather than a closed form.
---

## What it computes

`ACCRINT` returns the **accrued interest** on a security that pays periodic coupons — the
interest that has built up between the security's issue date and its settlement date, and
that a buyer therefore owes the seller on top of the price.

The arithmetic is a rate times a principal times a length of time:

> accrued = `par` × `rate` × (accrued time, measured in years on the chosen `basis`)

Written the way coupon markets write it, the year is divided into `frequency` coupon periods
and the elapsed time is counted in *periods* rather than in years:

>     accrued = par × (rate / frequency) × Σ (A_i / NL_i)

where the sum runs over the quasi-coupon periods touched by the accrual interval, `A_i` is the
number of accrued days inside period *i* counted on the `basis`, and `NL_i` is the normal
length of period *i* on the same `basis`. A period that is entirely inside the interval
contributes exactly 1; the two end periods contribute fractions.

That decomposition is what makes `ACCRINT` different from its at-maturity sibling
[ACCRINTM](FUNC.ACCRINTM.md), which needs no schedule at all: `ACCRINTM` measures one interval
against one year, while `ACCRINT` has to *reconstruct the coupon schedule* backwards from
`first_interest`, decide which periods the interval crosses, and normalize each piece by that
period's own length. Every hard question about this function is a question about that
reconstruction.

Microsoft's worksheet-function page publishes an equation of the shape above. That page was not
retrieved for this entry — the support host returned HTTP 403 — so the formula printed here is
stated as the standard accrual identity in the Handbook's own voice, not quoted from Microsoft.
The parameter list, the required/optional split and the basis table below **are** taken from
Microsoft's Learn reference for the same function.

### Two lengths, and which one normalizes

The subtle part is `NL_i`. For a whole interior period there is nothing to decide. For the
period that contains `settlement`, and for the stub period that contains `issue`, the
implementation has to choose between:

1. the **actual** length of that particular period (which under actual/actual differs from
   period to period, because half-years are 181, 182, 183 or 184 days long), and
2. a **canonical** length — the length of the last coupon period before `first_interest`, used
   for every fractional piece regardless of where it sits.

The reference engine normalizes the settlement-side fraction by the *canonical* length, and
uses a period's own actual length only for the actual/actual issue stub. Those are different
functions of the same inputs, and they disagree on ordinary bonds. Nothing in the documentation
consulted here settles which one Excel uses; the reference engine's choice was identified
against live-Excel rows, and the record of that work is the evidence attached to this page.

## Arguments

`ACCRINT(issue, first_interest, settlement, rate, par, frequency, [basis], [calc_method])`

| Argument | Meaning | Required? |
|---|---|---|
| `issue` | The security's issue date — where accrual starts. | Required |
| `first_interest` | The security's first interest (first coupon) date. | Required |
| `settlement` | The date the trade settles — where accrual stops. | Required |
| `rate` | The security's **annual** coupon rate. | Required |
| `par` | The security's par value. | Required in the documented list |
| `frequency` | Coupon payments per year: 1, 2 or 4. | Required |
| `basis` | Day-count convention, 0–4. | Optional, defaults to 0 |
| `calc_method` | Which accrual construction to use. | Optional, defaults to TRUE |

**`rate` is annual, `frequency` divides it.** The single most common modelling error in this
family is feeding a per-period rate into an argument that Excel will divide by `frequency`
again.

**`par` is a scale factor, not a shape.** The result is linear in `par`, so a wrong `par`
produces a proportionally wrong answer that no sanity check on the *shape* of the number will
catch. The reference engine substitutes 1000 when the slot is present but empty (`,,`), which
is the same default Microsoft documents for the sibling `ACCRINTM`.

**`basis` selects the day-count convention** — 0 or omitted = US (NASD) 30/360, 1 =
actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360 — exactly the table on
Microsoft's Learn page for this function, and the same axis described at length on
[YEARFRAC](FUNC.YEARFRAC.md). A workbook that omits `basis` is asking for 30/360, whatever the
instrument's term sheet says.

The date arguments are numeric slots holding Excel date serials, subject to ordinary to-number
coercion; see [Coercion and lifting](../model/02-coercion-and-lifting.md). Microsoft's page
carries the standard warning that dates should be produced by `DATE` or by other formulas
rather than typed as text.

**Note a documentation gap.** Microsoft's Learn reference for this function lists **seven**
parameters, ending at `basis`. The eighth argument, `calc_method`, does not appear there at all,
although the reference engine's registry admits eight and the worksheet surface has carried
`calc_method` since the function moved out of the Analysis ToolPak. The Handbook records that as
a divergence between Microsoft's own two reference surfaces, not as a fact about Excel.

## The calc_method argument

`calc_method` is documented — on the worksheet-function page, not on the Learn page consulted
here — as a choice of *starting line*: TRUE accrues from `issue`, FALSE accrues from
`first_interest`.

**The Handbook's upstream research record states that this description is wrong on both counts.**
Under the reference engine, which was identified against live-Excel rows, both settings accrue
from `issue`. What FALSE selects is not a later start date but an *older arithmetic*: one flat
fraction with no schedule walk — except that when `issue` falls in a coupon period earlier than
the last period before `first_interest`, every whole coupon period in between is dropped from
the sum entirely.

Two consequences follow, and both are visible without any last-bit machinery:

1. **Accrual is not monotone in issue date under FALSE.** A bond issued three periods before
   `first_interest` can accrue *less* than one issued a single period before, because the
   intervening whole periods vanish.
2. **The result can be negative.** For early enough settlements the dropped periods make the
   sum negative — accrued interest that un-accrues — and the reference engine publishes it
   without complaint.

Under TRUE the schedule is walked period by period, and the reference engine sums the collected
per-period terms **backwards**, from the settlement side toward issue. Summation order is not a
detail here: it is the difference between reproducing a spreadsheet and not, on a measurable
fraction of rows.

This section describes the reference engine and the Handbook's research record. It does not
state what Excel does; it states that the documented description of `calc_method` and the
identified behaviour disagree, which is precisely the kind of divergence this Handbook exists
to publish.

## Result and edge cases

Returns a `Number`: an amount of currency in the same units as `par`, not a rate and not a
percentage.

- **Ordering is enforced, but only partly.** The reference engine requires `issue` <
  `first_interest` and `issue` < `settlement`, and errors otherwise. It does **not** require
  `settlement` ≤ `first_interest`; settlement past the first coupon date is a normal, supported
  case and is where the period-walk branch does its work.
- **`rate` = 0** is accepted by the reference engine and yields zero accrued interest. Note that
  Microsoft documents the corresponding input as an error for the sibling `ACCRINTM`; see the
  divergence recorded on [ACCRINTM](FUNC.ACCRINTM.md).
- **Settlement exactly on a coupon date** is the interesting boundary. Under the reference
  engine's TRUE path the final period is always measured as days divided by the canonical
  length, so it stays a fraction rather than snapping to a whole period.
- **Month-end and leap-day dates** are where the schedule reconstruction earns or loses its
  accuracy: the backward walk from `first_interest` preserves month-end anchoring, so a coupon
  dated the 30th of a 30-day month and one dated the 31st generate different schedules.
- **Fractional arguments.** Date and `basis` arguments are truncated toward zero before use.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

Microsoft's Learn reference for this function documents no error conditions at all — it carries
only the parameter table and the basis table. What follows is therefore the **reference
engine's** behaviour, recorded as such, plus the conditions Microsoft documents for the sibling
functions in this family:

| Error | Condition (reference engine) |
|---|---|
| `#VALUE!` | A date argument is not finite, or its truncated serial falls outside the representable date range |
| `#VALUE!` | `rate` or `par` is not finite |
| `#NUM!` | `rate` is negative, or `par` is not strictly positive |
| `#NUM!` | `frequency` is anything other than 1, 2 or 4 |
| `#NUM!` | `basis` truncates to something outside 0–4 |
| `#NUM!` | `issue` is not strictly earlier than both `first_interest` and `settlement` |
| propagated | An error value in any argument surfaces as that error |

The frequency and basis conditions match what Microsoft documents verbatim for
[COUPDAYBS](FUNC.COUPDAYBS.md) and its siblings, so they are safe to expect; the split between
`#VALUE!` and `#NUM!` on the date and scalar arguments is the reference engine's and has not
been checked against Excel here.

## Relationships

- **[ACCRINTM](FUNC.ACCRINTM.md)** — the same question for a security that pays all its interest
  at maturity. No coupon schedule, no `frequency`, no `calc_method`; one interval over one year
  fraction. If you are unsure which you need, the test is whether the instrument pays coupons
  before maturity.
- **The coupon-date functions** — [COUPPCD](FUNC.COUPPCD.md), [COUPNCD](FUNC.COUPNCD.md),
  [COUPDAYBS](FUNC.COUPDAYBS.md), [COUPDAYS](FUNC.COUPDAYS.md) — expose the schedule that
  `ACCRINT` reconstructs internally. They are the right instruments for auditing an `ACCRINT`
  result: if `COUPDAYBS`/`COUPDAYS` disagrees with the fraction implied by `ACCRINT`, the
  disagreement is localized to one of the two.
- **`PRICE` and `YIELD`** — the discounting half of the same bond model, implemented in the same
  reference-engine module. A bond's invoice price is `PRICE` plus this function's accrued
  interest, so a sign or day-count error here shows up as a settlement discrepancy, not as a
  wrong-looking price.
- **[YEARFRAC](FUNC.YEARFRAC.md)** — the standalone day-count engine. `ACCRINT` on a single
  whole period is a `YEARFRAC` computation wearing a coupon schedule.
- **Confused with**: `ACCRINTM` (above), and with simple `rate × par × YEARFRAC(...)` hand
  arithmetic, which reproduces `ACCRINT` only when the interval lies inside one coupon period.

## Numerical notes

The difficulty here is not floating point in the usual sense — there is no cancellation
catastrophe and no argument reduction. It is **combinatorial**: getting the same *sequence of
small quotients* that Excel forms, and then adding them in the same order.

1. **Schedule generation is the algorithm.** Walking back from `first_interest` by
   `12/frequency` months, with month-end anchoring, is the whole of the hard part. Two
   defensible month-arithmetic rules (clamp the day of month, versus remember that the anchor
   was a month end) produce schedules that differ by a day several times a year.
2. **Two 30/360 implementations will eventually disagree.** The Handbook's own research record
   documents a case where two routines implementing "the same" US 30/360 rule differed only in
   which adjustment they applied first, and disagreed by one day on month-end pairs. Share one
   day-count routine; do not re-derive it per family.
3. **Summation order is observable.** The per-period terms are `O(1)`-sized quotients summed
   into an accumulator; forward and backward summation differ in the last bit on a real fraction
   of inputs. A `portable-reproducible` implementation must pin the order explicitly, and an
   `excel-bitexact` one must pin it to whatever Excel's loop does — which the reference engine
   takes to be settlement-first.
4. **The canonical-versus-actual period length choice** (above) is a *semantic* choice with
   numerical consequences far larger than one ULP. It belongs in the specification, not in the
   rounding discussion.
5. There is no known closed form that avoids the schedule for a general `issue`; attempts to
   shortcut it with a single `YEARFRAC` are correct only within one period.

## What has not been checked

One evidence record names this surface in its subjects: **EV-FIN-0020**, an open-discrepancy
record covering the identification of this function against live Excel, with a held-out gating
discipline and a residual set of rows that the source explicitly does not accept. Its counts,
corpora and warnings render mechanically beside this page; they are not restated here, and the
record's own reader warning about mixed versus held-out corpora travels with them. The record
being open is the honest summary: this surface has been measured, and it is not settled.

No Handbook vector suite exists for `ACCRINT`. The battery shown on this page is the reference
engine answering its own probes; no Excel was involved in producing it.

Inputs worth probing first:

1. **`calc_method` FALSE with `issue` several coupon periods before `first_interest`**, sweeping
   `issue` backwards one period at a time. This is the cheapest probe that exhibits the
   vanishing-period behaviour, and the first sign change in the result is a single decisive
   observation.
2. **`calc_method` TRUE versus FALSE on the same inputs** where `issue` sits inside the last
   period before `first_interest` — the case where the two constructions ought to coincide.
   Any difference there localizes the disagreement precisely.
3. **Settlement exactly on a coupon date**, and one day either side, on each basis. This
   distinguishes canonical-length from actual-length normalization better than any interior
   point.
4. **Month-end schedules**: `first_interest` on 28 February, 29 February, 30 and 31 of a month,
   at each `frequency`, with settlement several periods later. The schedule-anchoring rule is
   the highest-value unknown in the whole family.
5. **Bases 1 and 3 across a leap day**, where the actual/actual period length and the canonical
   length are guaranteed to differ.
6. **`rate` = 0 and `par` omitted** (`,,`), to pin the two defaults and to test whether Excel
   applies the sibling's documented rate-of-zero error here.
7. **`ACCRINT` against `COUPDAYBS`/`COUPDAYS`** on a single-period interval, which should
   reduce to `par × rate/frequency × COUPDAYBS/COUPDAYS` and localizes any mismatch to one
   function.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| accrued interest | Interest earned but not yet paid, owed by the buyer to the seller at settlement |
| quasi-coupon period | A schedule period generated backwards from `first_interest`, whether or not a coupon was actually paid in it |
| canonical length | The length of the last coupon period before `first_interest`, used as a normalizing denominator |
| schedule walk | Reconstructing coupon dates by stepping `12/frequency` months back from `first_interest` |
| day-count basis | The `basis` argument's convention for counting days and years (0–4) |

## Sources

- Microsoft Learn, **WorksheetFunction.AccrInt method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.accrint>. Source of
  the parameter list with its required/optional split, the day-count basis table, and the
  date-entry warning. This page documents seven parameters and does not mention `calc_method`.
- Microsoft 365 support, **ACCRINT function** —
  <https://support.microsoft.com/en-us/office/accrint-function-fe45d089-6722-4fb3-9379-e1f911d8dc74>.
  The worksheet-surface page, which carries `calc_method` and a published accrual equation.
  Cited for completeness; **not retrieved** for this entry (the host returned HTTP 403), so
  nothing on this page is quoted from it.
- Handbook evidence record `EV-FIN-0020` — the open identification record naming this surface.
- Handbook, `content/lastbit/MATERIAL_W109_PRIMITIVES.md` §20 — the research narrative for the
  `calc_method` divergence and the backward summation order.
- Handbook chapters [01 the value universe](../model/01-value-universe.md),
  [02 coercion and lifting](../model/02-coercion-and-lifting.md),
  [06 claim language](../model/06-claim-language.md).
- OxFunc `crates/oxfunc_core/src/functions/bond_core_family.rs` at commit `473efa3` — the
  reference engine's kernel, argument validation and schedule reconstruction, read for the
  behaviour attributed to the reference engine above.
- `data/functions/FUNC.ACCRINT.json`, `data/presence/FUNC.ACCRINT.json`,
  `data/battery/FUNC.ACCRINT.json`.
