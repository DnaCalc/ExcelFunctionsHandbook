---
schema: efh.function-page/v1
function_id: FUNC.ACCRINTM
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn: WorksheetFunction.AccrIntM method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.accrintm"
    role: "the parameter list, the $1,000 par default, the basis table, the truncation rule, the four documented error conditions, and the A/D form of the equation"
  - work: "Microsoft 365 support: ACCRINTM function"
    locator: "https://support.microsoft.com/en-us/office/accrintm-function-f62f01f9-5754-4cc4-805b-0e70199328a7"
    role: "the worksheet-surface page; cited but not retrieved — the host returned HTTP 403 to the Handbook's fetch"
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
  The schedule-free accrual: one interval, one year fraction, one multiplication — the member
  whose entire content is the day-count basis, and therefore the cleanest probe of it.
---

## What it computes

`ACCRINTM` returns the accrued interest on a security that pays **all** its interest at
maturity. There are no coupons in between, so there is no schedule to reconstruct: the whole
computation is one interval measured once.

Microsoft's Learn reference states the shape of the equation directly — "*A* = Number of accrued
days counted according to a monthly basis", "*D* = Annual Year Basis", and "For interest at
maturity items, the number of days from the issue date to the maturity date is used". Writing
that out:

>     ACCRINTM = par × rate × (A / D)

with `A` the days from `issue` to `settlement` counted on the chosen `basis`, and `D` the annual
year basis for that same convention (360, 365, or an actual/actual year length). The quotient
`A / D` is exactly the quantity [YEARFRAC](FUNC.YEARFRAC.md) computes, and the reference engine
computes it with the same helper the rest of its bond family uses.

So the mathematics is a single product of three numbers, and **all of the difficulty is in the
year fraction**. The `basis` argument is not a detail on this page; it is the page.

| `basis` | Convention | Numerator `A` | Denominator `D` |
|---|---|---|---|
| 0 or omitted | US (NASD) 30/360 | 30/360-adjusted day count | 360 |
| 1 | Actual/actual | actual calendar days | actual — depends on the years spanned |
| 2 | Actual/360 | actual calendar days | 360 |
| 3 | Actual/365 | actual calendar days | 365 |
| 4 | European 30/360 | 30/360-adjusted day count | 360 |

That table is Microsoft's, verbatim from the Learn page. The basis-1 construction — what
"actual year" means when the interval spans a year boundary or a leap day — is *not* specified
there, and is the same unsettled question described at length on [YEARFRAC](FUNC.YEARFRAC.md).
For an at-maturity instrument this matters more than usual, because the interval is typically
long: a bill with a two-year term crosses at least one year boundary by construction.

### One naming trap

The second argument is called `settlement` on the worksheet, and Microsoft's own Learn page
describes it as "the security's **maturity** date". For an at-maturity security these are the
same date in normal use — the interest stops accruing when the paper matures — but the argument
name and the argument description do not agree, and code written against the name alone will
pass the wrong date on any early-settlement scenario. The Handbook records the naming
discrepancy; it does not resolve which reading Excel implements.

## Arguments

`ACCRINTM(issue, settlement, rate, [par], [basis])`

| Argument | Meaning | Microsoft's Learn page says | Reference engine |
|---|---|---|---|
| `issue` | The security's issue date. | Required | Required |
| `settlement` | Described as the security's maturity date. | Required | Required |
| `rate` | The security's **annual** coupon rate. | Required | Required |
| `par` | The security's par value. | Required — "If you omit par, ACCRINTM uses $1,000." | Optional; defaults to 1000 |
| `basis` | Day-count convention, 0–4. | Optional | Optional; defaults to 0 |

**The `par` row is a documented contradiction.** Microsoft's parameter table marks `par` as
Required and, in the same cell, tells you what happens if you omit it. The reference engine
resolves the contradiction in favour of the sentence: its declared arity admits a three-argument
call, and a missing or empty `par` becomes 1000. This is recorded as a
documentation-versus-reference-engine divergence, not as a claim about Excel.

**`rate` is annual.** Unlike [ACCRINT](FUNC.ACCRINT.md) there is no `frequency` to divide it by,
so an annual rate goes in unmodified and the year fraction does all the scaling.

**`par` scales the answer linearly**, which is why a wrong `par` produces a plausible-looking
wrong number rather than an obviously broken one.

Microsoft documents that **"Issue, maturity, and basis are truncated to integers"**, and carries
the standard warning that dates should come from `DATE` or from other formulas rather than being
typed as text. Numeric slots otherwise follow the shared model in
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns a `Number`: an amount of currency in the units of `par`.

- **Reversed or equal dates.** Microsoft documents that `issue` = maturity generates an error.
  The reference engine additionally errors when `issue` is *after* `settlement`, so it never
  returns a negative accrual. Microsoft's page does not state what happens in that case.
- **`rate` = 0.** Microsoft documents this as an error: "If rate = 0 or if par = 0, ACCRINTM
  will generate an error." **The reference engine does not implement that.** It accepts a zero
  rate and returns zero accrued interest, rejecting only a *negative* rate. This is a real
  divergence between the documentation and the reference engine, and it is exactly the kind of
  input a probe should settle first.
- **`par` = 0.** Here documentation and reference engine agree: an error.
- **Long intervals under basis 1** are where the unspecified actual/actual construction becomes
  visible; two defensible constructions differ by a fraction of a day per year spanned.
- **Fractional date serials** contribute no fractional days, per the documented truncation rule.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md).

## Errors

Microsoft's Learn page documents four conditions, in these words (it says "will generate an
error" without naming a worksheet error value):

| Condition (documented) | Documented outcome |
|---|---|
| `issue` or maturity is not a valid date | error |
| `rate` = 0, or `par` = 0 | error |
| `basis` < 0 or `basis` > 4 | error |
| `issue` = maturity | error |

The reference engine's mapping onto specific error values, recorded as the reference engine's
and not as Excel's:

| Error | Condition (reference engine) |
|---|---|
| `#VALUE!` | A date argument is not finite, or truncates outside the representable date range |
| `#VALUE!` | `rate` or `par` is not finite |
| `#NUM!` | `rate` is negative — **but not** `rate` = 0, which the documentation says is an error |
| `#NUM!` | `par` ≤ 0 |
| `#NUM!` | `basis` truncates outside 0–4 |
| `#NUM!` | `issue` ≥ `settlement` |
| propagated | An error value in any argument surfaces as that error |

## Relationships

- **[ACCRINT](FUNC.ACCRINT.md)** — the periodic-coupon counterpart. It needs `frequency` and
  `calc_method` because it must reconstruct a coupon schedule; this function needs neither. The
  two agree only in the degenerate case where the whole interval is one coupon period.
- **[YEARFRAC](FUNC.YEARFRAC.md)** — the day-count engine. `ACCRINTM(i, s, r, p, b)` should be
  `p × r × YEARFRAC(i, s, b)`, and testing that identity is the cheapest way to localize a
  disagreement to one of the two functions.
- **`PRICEMAT` and `YIELDMAT`** — the price and yield of the same at-maturity instrument,
  implemented in the same reference-engine module and consuming the same day-count helper. An
  error in the year fraction is an error in all three.
- **`INTRATE`, `RECEIVED`, `DISC`** — the discount-instrument family, which measures time the
  same way but quotes the result as a rate rather than an amount.
- **Confused with**: `ACCRINT` (above), and with `ISPMT`-style simple-interest arithmetic, which
  uses a period count rather than a day-count basis.

## Numerical notes

1. **There is nothing to lose precision in.** Three finite numbers are multiplied; the only
   rounding is in `A/D` and in the two products. The interesting engineering question is not
   accuracy but *op order*: `par × rate × (A/D)` and `par × (rate × A) / D` differ in the last
   bit, and an implementation targeting Excel compatibility must pin which one it forms.
2. **`D` is a small integer in three of five cases**, so `A/D` is a quotient of small integers
   by 360 or 365 — inexact in binary, and therefore genuinely last-bit-sensitive.
3. **Basis 1 needs a written specification before it needs code.** Which years contribute,
   how leap days are counted, and what happens across multi-year intervals are all choices, and
   the documentation makes none of them.
4. **Share the 30/360 routine.** The Handbook's research record documents two routines
   implementing "the same" US 30/360 rule that differed only in the order of two adjustments and
   disagreed by a day on month-end pairs. The reference engine's bond family and its
   depreciation family in fact contain two such routines; see [AMORLINC](FUNC.AMORLINC.md).
5. **Truncate all three integer-valued arguments first**, per the documented rule, before any
   arithmetic — including `basis`.

## What has not been checked

No Handbook vector suite exists for `ACCRINTM`, and **no evidence record lists this surface in
its subjects**. The `bond_core_family` module that implements it is measured — the family's
`ACCRINT` and `PRICE` surfaces carry open records — but this surface was not measured
separately, and family-level evidence may not be attributed to it. Nobody has checked
`ACCRINTM` against Excel within the Handbook's record.

The battery on this page is the reference engine answering its own probes; no Excel was
involved.

Inputs worth probing first:

1. **`rate` = 0**, with everything else valid. Microsoft documents an error; the reference
   engine returns zero. One cell settles a published contradiction.
2. **A three-argument call** — `ACCRINTM(issue, settlement, rate)` — and the same call with an
   empty fourth slot. This settles whether `par` is genuinely optional and whether the
   documented $1,000 default applies.
3. **`issue` after `settlement`.** Undocumented; the reference engine errors, but a signed
   result is equally defensible and is what a pure `par × rate × YEARFRAC` reading would give.
4. **`ACCRINTM(i, s, r, p, b)` against `p × r × YEARFRAC(i, s, b)`** for every `basis`, over a
   grid of intervals crossing year ends and leap days. Any divergence localizes to one of the
   two functions, and basis 1 is where it will appear if it appears anywhere.
5. **Basis 0 with `issue` on the last day of February** — the input class Microsoft itself flags
   as possibly incorrect for `YEARFRAC`, tested here through a second consumer of the same rule.
6. **`basis` = −1, 5 and 4.9**, confirming the documented range and the documented truncation.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| at-maturity security | An instrument paying all its interest once, at maturity, with no intermediate coupons |
| `A` | Days accrued from `issue` to `settlement`, counted on the chosen basis (Microsoft's notation) |
| `D` | The annual year basis for that convention (Microsoft's notation) |
| year fraction | The quotient `A/D`; the same quantity `YEARFRAC` returns |
| day-count basis | The `basis` argument's convention for counting days and years (0–4) |

## Sources

- Microsoft Learn, **WorksheetFunction.AccrIntM method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.accrintm>. Source of
  the parameter table (including the Required marking on `par` and the "$1,000" sentence in the
  same cell), the basis table, the truncation rule, the four error conditions, the `A` and `D`
  definitions, and the date-entry warning.
- Microsoft 365 support, **ACCRINTM function** —
  <https://support.microsoft.com/en-us/office/accrintm-function-f62f01f9-5754-4cc4-805b-0e70199328a7>.
  The worksheet-surface page; cited for completeness and **not retrieved** for this entry (the
  host returned HTTP 403), so nothing here is quoted from it.
- Handbook, [YEARFRAC](FUNC.YEARFRAC.md) — the day-count basis axis and the open actual/actual
  question, stated once and not restated here.
- Handbook chapters [01 the value universe](../model/01-value-universe.md),
  [02 coercion and lifting](../model/02-coercion-and-lifting.md),
  [06 claim language](../model/06-claim-language.md).
- OxFunc `crates/oxfunc_core/src/functions/bond_core_family.rs` at commit `473efa3` — the
  reference engine's kernel and argument validation, read for every statement attributed to the
  reference engine above.
- `data/functions/FUNC.ACCRINTM.json`, `data/presence/FUNC.ACCRINTM.json`,
  `data/battery/FUNC.ACCRINTM.json`.
