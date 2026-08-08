---
schema: efh.function-page/v1
function_id: FUNC.COUPDAYS
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn: WorksheetFunction.CoupDays method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.coupdays"
    role: "the parameter list, the basis table, the truncation rule and the four documented error conditions"
  - work: "Microsoft 365 support: COUPDAYS function"
    locator: "https://support.microsoft.com/en-us/office/coupdays-function-cc64380b-315b-4e7b-950c-b30b0a76f671"
    role: "the worksheet-surface page; cited but not retrieved — the host returned HTTP 403 to the Handbook's fetch"
episodes: []
body_sections:
  - What it computes
  - Why the answer is often not a whole number of days
  - The additivity that fails
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: coupon_family
role_in_family: >-
  The denominator: the length of the coupon period containing settlement, and the only member
  whose answer is a convention rather than a calendar measurement.
---

## What it computes

`COUPDAYS(settlement, maturity, frequency, [basis])` returns **the number of days in the coupon
period that contains the settlement date**.

It is the denominator of every accrual fraction in bond arithmetic. Accrued interest is
`COUPDAYBS / COUPDAYS` of a coupon; the discounting exponent in `PRICE` is
`COUPDAYSNC / COUPDAYS`. Get this function wrong and every price and yield built on it is wrong
by the same proportion.

The coupon period is determined by the schedule described on
[COUPNCD](FUNC.COUPNCD.md#the-coupon-schedule): coupon dates run backwards from `maturity` in
steps of `12/frequency` months, and `settlement` selects one period from that ladder.

## Why the answer is often not a whole number of days

Here is the thing that surprises readers: **`COUPDAYS` does not always count days.** For four of
the five bases it returns a convention, and the convention need not be an integer.

Under the reference engine:

| `basis` | What `COUPDAYS` returns |
|---|---|
| 0 — US (NASD) 30/360 | `360 / frequency` |
| 1 — Actual/actual | the actual calendar length of the period containing `settlement` |
| 2 — Actual/360 | `360 / frequency` |
| 3 — Actual/365 | `365 / frequency` |
| 4 — European 30/360 | `360 / frequency` |

So at `frequency` 2 the answers are 180, 180, 182.5 and 180 for bases 0, 2, 3 and 4 — and
`182.5` is not a number of days that any calendar contains. Under basis 1 the answer really is a
day count, and it varies from period to period: semiannual periods are 181, 182, 183 or 184 days
long depending on which months they span and whether a leap day falls inside.

The logic is coherent once stated: a `/360` or `/365` basis *declares* a year length, and the
coupon period is by definition a `1/frequency` share of that declared year. Only actual/actual
declines to declare one and measures the calendar instead. But the practical consequence is
worth stating plainly: **basis 2 is "actual/360", and yet `COUPDAYS` under basis 2 ignores the
actual calendar entirely.** The "actual" in the name governs the numerator, not the denominator.

This table is the reference engine's. Microsoft's page documents the basis values and their
names and does not say what `COUPDAYS` returns for each; it says only that the function returns
"the number of days in the coupon period that contain the settlement date" — a description that
is literally true only for basis 1.

## The additivity that fails

The obvious identity is

>     COUPDAYBS + COUPDAYSNC  =  COUPDAYS

— days elapsed plus days remaining equals the length of the period. Under the reference engine
it **holds for bases 0, 1 and 4, and fails for bases 2 and 3.**

The reason follows directly from the table above and the corresponding one on
[COUPDAYBS](FUNC.COUPDAYBS.md): under bases 2 and 3 the two *numerator* functions count real
calendar days, while `COUPDAYS` returns a declared 180 or 182.5. A 184-day real period reported
as a 180-day period cannot have its two halves add up.

This is not a defect in the family so much as a fault line running through it, and it has
already produced observable divergence upstream. The Handbook's research record documents a case
in the same bond model where Excel's `PRICE` derives its days-to-next-coupon as
*period length minus accrued days* rather than counting them directly — which agrees with
`COUPDAYSNC` on 30/360 and actual/actual bases and disagrees with it, by whole days, on
actual/360 and actual/365. Excel published both numbers, in the same workbook, for the same
bond. That episode is exactly this additivity failure, seen from the consumer's side.

The reader's practical rule: **on bases 2 and 3, do not derive any of the three day counts from
the other two.** Ask for each one.

## Arguments

`COUPDAYS(settlement, maturity, frequency, [basis])` — the family's shared four arguments;
Microsoft's argument descriptions and the five-row basis table are quoted on
[COUPNCD](FUNC.COUPNCD.md#arguments).

| Argument | Meaning | Required? |
|---|---|---|
| `settlement` | The security's settlement date. | Required |
| `maturity` | The security's maturity date. | Required |
| `frequency` | Coupon payments per year: 1, 2 or 4. | Required |
| `basis` | The day-count basis, 0–4. | Optional, defaults to 0 |

Unlike its five siblings, `COUPDAYS` genuinely depends on `basis` in the strongest possible way:
`basis` chooses between "measure the calendar" and "quote a convention". Omitting `basis` asks
for 30/360, and gets a flat `360/frequency` for every period of every bond.

Microsoft documents that **"All arguments are truncated to integers."**

## Result and edge cases

Returns a `Number`, which may be fractional (182.5 at basis 3, `frequency` 2) and is not always
a possible day count.

- **`settlement` on a coupon date.** Under the reference engine the schedule rolls forward, so
  the period reported is the one *starting* at `settlement`; see
  [COUPNCD](FUNC.COUPNCD.md#the-coupon-schedule). At bases 0, 2 and 4 this is invisible because
  the answer is a constant; at bases 1 and 3 it is not.
- **Basis 1 across a leap day** gives a longer period than the same schedule position in a
  non-leap year. This is the only basis under which `COUPDAYS` varies with the settlement date
  at all.
- **Basis 1 and the roll-forward interaction.** The reference engine uses a slightly different
  period start for the actual/actual measurement than for the other bases — the anchor is taken
  from the schedule's raw backward step rather than from the clamped period start. On month-end
  schedules these can differ by a day, and only basis 1 can see it.
- **`frequency` 1, 2, 4** are the only admitted values, so the constants are 360, 180, 90 (bases
  0, 2, 4) and 365, 182.5, 91.25 (basis 3).
- **`settlement` ≥ `maturity`** is a documented error.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md).

## Errors

Documented on Microsoft's Learn page for this function, in these words (it says "generates an
error" without naming a worksheet error value):

| Condition (documented) | Documented outcome |
|---|---|
| `settlement` or `maturity` is not a valid date | error |
| `frequency` is any number other than 1, 2 or 4 | error |
| `basis` < 0 or `basis` > 4 | error |
| `settlement` ≥ `maturity` | error |

The reference engine maps all four onto `#NUM!`, and also returns `#NUM!` for a non-finite
argument or a date serial outside the representable range. Errors in any argument propagate.

## Relationships

- **[COUPDAYBS](FUNC.COUPDAYBS.md)** and **[COUPDAYSNC](FUNC.COUPDAYSNC.md)** — the numerators
  this function is the denominator of. Read the additivity section above before assuming they
  sum to it.
- **[COUPNCD](FUNC.COUPNCD.md)** and **[COUPPCD](FUNC.COUPPCD.md)** — the two dates that bound
  the period whose length this function reports. Note that `COUPNCD − COUPPCD` is the *actual*
  length, which equals `COUPDAYS` only under basis 1.
- **[COUPNUM](FUNC.COUPNUM.md)** — how many such periods remain.
- **`PRICE`, `YIELD`, `DURATION`, `MDURATION`, `ODDFPRICE`** — the consumers, all of which form
  `COUPDAYSNC / COUPDAYS` as a discounting exponent or `COUPDAYBS / COUPDAYS` as an accrual
  fraction.
- **[YEARFRAC](FUNC.YEARFRAC.md)** — the general day-count engine. `COUPDAYS` is *not*
  `YEARFRAC` scaled: `YEARFRAC` always measures a real interval, while `COUPDAYS` mostly quotes
  a convention.
- **Confused with**: `COUPNCD(...) − COUPPCD(...)`, which is the actual number of days in the
  period and differs from `COUPDAYS` on every basis except 1.

## Numerical notes

1. **Three of the five answers are exact in binary; one is not.** 360, 180, 90, 365 and 91.25
   are all exactly representable; `365/2 = 182.5` is exact too. In fact all ten constants are
   exactly representable, so `COUPDAYS` itself has no rounding error — but the *quotients* built
   from it do, and forming `a / (365/frequency)` is not the same as forming `a × frequency / 365`.
   A consumer targeting Excel compatibility must pin which.
2. **Basis 1's answer is an integer day count** and can be computed in integer arithmetic
   throughout. Doing so removes a whole class of bugs.
3. **The period-start anchor for basis 1 is a decision.** Whether the actual length is measured
   from the clamped period start or from the raw backward step matters only on month-end
   schedules, and only under basis 1 — but it is a real fork in the reference engine's own code.
4. **Do not implement `COUPDAYS` as a subtraction of the two other counts, or vice versa.** The
   additivity failure above means each of the three has to be computed on its own terms. The
   upstream research record shows what happens when a production implementation takes the
   shortcut.
5. There is no argument reduction, no cancellation and no transcendental here. Every difficulty
   in this function is definitional.

## What has not been checked

No Handbook vector suite exists for `COUPDAYS`, and **no evidence record lists this surface in
its subjects**. The shared `coupon_family` module is named by no record either. Nobody has
checked this function against Excel within the Handbook's record. The battery on this page is
the reference engine answering its own probes; no Excel was involved.

The basis table in the second section is the single most consequential unverified claim on this
page, and it is a five-cell experiment.

Inputs worth probing first:

1. **One bond, one settlement, all five bases**, at `frequency` 2. If Excel returns 180, an
   actual count, 180, 182.5 and 180, the convention table above is confirmed in one row. The
   presence of a fractional 182.5 is by itself decisive.
2. **`COUPDAYBS + COUPDAYSNC − COUPDAYS` at bases 2 and 3**, on a period that is not 180 days
   long. A non-zero result publishes the additivity failure directly.
3. **Basis 1 over four consecutive semiannual periods spanning a leap day**, confirming the
   181/182/183/184 variation and locating the leap-day rule.
4. **`settlement` exactly on a coupon date**, at bases 1 and 3, where the roll-forward
   convention is visible in the answer.
5. **A month-end maturity at basis 1**, which is where the two candidate period-start anchors
   diverge.
6. **`frequency` 1 and 4**, confirming that the constants scale as `360/frequency` and
   `365/frequency` rather than being tabulated separately.
7. **`COUPNCD − COUPPCD` against `COUPDAYS`** on each basis — the cheapest way to demonstrate to
   a reader that `COUPDAYS` is usually not a calendar measurement.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| coupon period | The schedule period containing `settlement`; see COUPNCD |
| declared year | The year length a `/360` or `/365` basis asserts, regardless of the calendar |
| convention length | `360/frequency` or `365/frequency`, returned instead of a real day count |
| additivity | The property `COUPDAYBS + COUPDAYSNC = COUPDAYS`, which fails on bases 2 and 3 |
| accrual fraction | `COUPDAYBS / COUPDAYS`, the share of a coupon already earned |

## Sources

- Microsoft Learn, **WorksheetFunction.CoupDays method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.coupdays>. Source of
  the one-line description, the parameter table, the five-row basis table, the "All arguments
  are truncated to integers" rule, the date-entry warning, and the four error conditions.
- Microsoft 365 support, **COUPDAYS function** —
  <https://support.microsoft.com/en-us/office/coupdays-function-cc64380b-315b-4e7b-950c-b30b0a76f671>.
  The worksheet-surface page; **not retrieved** for this entry (the host returned HTTP 403), so
  nothing here is quoted from it.
- Handbook, [COUPNCD](FUNC.COUPNCD.md#the-coupon-schedule) — the shared schedule construction.
- Handbook, `content/lastbit/MATERIAL_W109_PRIMITIVES.md` §18 — the recorded case of Excel's
  `PRICE` deriving days-to-next-coupon by subtraction and disagreeing with `COUPDAYSNC` on the
  actual/360 and actual/365 bases.
- Handbook chapters [01 the value universe](../model/01-value-universe.md),
  [02 coercion and lifting](../model/02-coercion-and-lifting.md),
  [06 claim language](../model/06-claim-language.md).
- OxFunc `crates/oxfunc_core/src/functions/coupon_family.rs` at commit `473efa3` — the reference
  engine's per-basis period-length table, the actual/actual anchor fork, and validation.
- `data/functions/FUNC.COUPDAYS.json`, `data/presence/FUNC.COUPDAYS.json`,
  `data/battery/FUNC.COUPDAYS.json`.
