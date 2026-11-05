---
schema: efh.function-page/v1
function_id: FUNC.COUPDAYS
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-FIN-0030
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
  - Notes
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
steps of `12/frequency` months, and `settlement` selects one period from that ladder. On basis 1
Excel's period comes from its own walk back from `maturity` (FN-COUPDAYS-001).

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

This table is the reference engine's; FN-COUPDAYS-001 measures the same constants in Excel on
bases 0, 2, 3 and 4. Microsoft's page documents the basis values and their
names and does not say what `COUPDAYS` returns for each; it says only that the function returns
"the number of days in the coupon period that contain the settlement date" — a description that
is literally true only for basis 1.

## The additivity that fails

The obvious identity is

>     COUPDAYBS + COUPDAYSNC  =  COUPDAYS

— days elapsed plus days remaining equals the length of the period. Under the reference engine
it **holds for bases 0, 1 and 4, and fails for bases 2 and 3.** Excel departs from the first half
on basis 0, where `COUPDAYSNC` is not always `COUPDAYS` minus `COUPDAYBS` (FN-COUPDAYSNC-001).

The reason follows directly from the table above and the corresponding one on
[COUPDAYBS](FUNC.COUPDAYBS.md): under bases 2 and 3 the two *numerator* functions count real
calendar days, while `COUPDAYS` returns a declared 180 or 182.5. A 184-day real period reported
as a 180-day period cannot have its two halves add up.

This is not a defect in the family so much as a fault line running through it, and it has
already produced observable divergence upstream. The Handbook's research record documents a case
in the same bond model where Excel's `PRICE` derives its days-to-next-coupon as
*period length minus accrued days* rather than counting them directly — which agrees with
`COUPDAYSNC` on 30/360 and actual/actual bases, apart from the basis 0 cases of FN-COUPDAYSNC-001,
and disagrees with it, by whole days, on actual/360 and actual/365. Excel published both numbers, in
the same workbook, for the same bond. That episode is exactly this additivity failure, seen from the
consumer's side.

The reader's practical rule: **on bases 0, 2 and 3, do not derive any of the three day counts
from the other two.** Ask for each one.

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
  [COUPNCD](FUNC.COUPNCD.md#the-coupon-schedule). At bases 0, 2, 3 and 4 this is invisible
  because the answer is a constant (FN-COUPDAYS-001); at basis 1 it is not.
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
  length, which equals `COUPDAYS` only under basis 1, and not always there: FN-COUPDAYS-001
  records 184 against 183.
- **[COUPNUM](FUNC.COUPNUM.md)** — how many such periods remain.
- **`PRICE`, `YIELD`, `DURATION`, `MDURATION`, `ODDFPRICE`** — the consumers, all of which form
  `COUPDAYSNC / COUPDAYS` as a discounting exponent or `COUPDAYBS / COUPDAYS` as an accrual
  fraction.
- **[YEARFRAC](FUNC.YEARFRAC.md)** — the general day-count engine. `COUPDAYS` is *not*
  `YEARFRAC` scaled: `YEARFRAC` always measures a real interval, while `COUPDAYS` mostly quotes
  a convention.
- **Confused with**: `COUPNCD(...) − COUPPCD(...)`, which is the actual number of days in the
  period and differs from `COUPDAYS` on every basis except 1, and on basis 1 in the cases
  FN-COUPDAYS-001 records.

## Numerical notes

1. **Three of the five answers are exact in binary; one is not.** 360, 180, 90, 365 and 91.25
   are all exactly representable; `365/2 = 182.5` is exact too. In fact all ten constants are
   exactly representable, so `COUPDAYS` itself has no rounding error — but the *quotients* built
   from it do, and forming `a / (365/frequency)` is not the same as forming `a × frequency / 365`.
   A consumer targeting Excel compatibility must pin which.
2. **Basis 1's answer is an integer day count** and can be computed in integer arithmetic
   throughout. Doing so removes a whole class of bugs.
3. **The period-start anchor for basis 1 is a decision.** Whether the actual length is measured
   from the clamped period start or from the raw backward step is a real fork in the reference
   engine's own code. Excel's rule is in FN-COUPDAYS-001: a clamped walk back from `maturity`, with
   month ends kept only on the last step.
4. **Do not implement `COUPDAYS` as a subtraction of the two other counts, or vice versa.** The
   additivity failure above means each of the three has to be computed on its own terms. The
   upstream research record shows what happens when a production implementation takes the
   shortcut.
5. There is no argument reduction, no cancellation and no transcendental here. Every difficulty
   in this function is definitional.

## What has not been checked

No Handbook vector suite exists for `COUPDAYS`. The six-surface record `EV-FIN-0030` lists this
surface and holds no pass rate: it has a capture count of 6 × 1,060 rows, which says nothing about
how they scored, and an uncounted statement that the reference engine's six functions matched Excel
in one scan on the older build 20026. The shared `coupon_family` module is named by no record. The
Excel readings on this page are in FN-COUPDAYS-001 and FN-COUPDAYSNC-001. The battery on this page
is the reference engine answering its own probes; no Excel was involved.

FN-COUPDAYS-001 settles the basis table in the second section: Excel returns `360/frequency` on
bases 0, 2 and 4, `365/frequency` on basis 3, and on basis 1 actual days by the walk that note
gives. That walk stops at the last date still after settlement, so a settlement on a coupon date
falls in the period after it, and it keeps month ends only on the last step. That Excel's code
works this way is inferred.

Inputs worth probing first:

1. **`COUPDAYBS + COUPDAYSNC − COUPDAYS` at bases 2 and 3**, on a period that is not 180 days
   long. A non-zero result publishes the additivity failure directly.
2. **`COUPNCD − COUPPCD` against `COUPDAYS`** on each basis — the cheapest way to demonstrate to
   a reader that `COUPDAYS` is usually not a calendar measurement. On basis 1 the two can differ
   (FN-COUPDAYS-001).
3. **Basis 1 with a settlement or maturity before serial 61**, where Excel's calendar differs from
   the Gregorian one and FN-COUPDAYS-001 has no recorded row.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `COUPDAYS` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-COUPDAYS-001: On basis 1 COUPDAYS walks back from maturity by its own rule, not COUPPCD to COUPNCD

*Quirk; measured; recorded 2026-11-04.* Applies to COUPDAYS, COUPNCD, COUPPCD.

On basis 1 (actual/actual) COUPDAYS does not always equal COUPNCD minus COUPPCD. Its rule:
start at maturity and step back by 12/frequency months. Each step keeps the day of the month
and clamps it to the length of the target month, and the clamped date is the start of the next
step, so passing through February lowers the day for every earlier date. Stop at the last date
still after settlement, call it next. The answer is the number of actual days from one period
before next up to next, and this last step keeps month ends: if next is the last day of its
month, the start is the last day of its month. On the other bases the answer is 360/frequency
(bases 0, 2, 4) or 365/frequency (basis 3). With this rule, all 5,584 recorded rows match
(4,072 distinct inputs): 5,568 numbers and 16 #NUM!, from Excel builds 20131 and 20430. Of the
967 basis 1 rows, 81 (49 distinct inputs) differ from the day count between the dates of a
schedule anchored on maturity with month ends kept. For maturity 1998-06-30 with semi-annual
coupons, COUPDAYS(35748,35976,2,1) is 183, from 1997-06-30 to 1997-12-30; the anchored schedule
gives 184. In one recorded case all three functions were read on the same inputs:
COUPNCD(44043,2958465,2,1) less COUPPCD is 44196 - 44012 = 184, and COUPDAYS is 183.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `COUPDAYS(35748, 35976, 2, 1)` | settlement `35748` (`0x40e1748000000000`); maturity `35976` (`0x40e1910000000000`); frequency `2` (`0x4000000000000000`); basis `1` (`0x3ff0000000000000`) (cells) | `183` (`0x4066e00000000000`) | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |
| `COUPDAYS(36452, 36891, 4, 1)` | settlement `36452` (`0x40e1cc8000000000`); maturity `36891` (`0x40e2036000000000`); frequency `4` (`0x4010000000000000`); basis `1` (`0x3ff0000000000000`) (cells) | `91` (`0x4056c00000000000`) | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |
| `COUPDAYS(36812, 47907, 4, 1)` | settlement `36812` (`0x40e1f98000000000`); maturity `47907` (`0x40e7646000000000`); frequency `4` (`0x4010000000000000`); basis `1` (`0x3ff0000000000000`) (cells) | `92` (`0x4057000000000000`) | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |
| `COUPDAYS(37015, 38137, 4, 1)` | settlement `37015` (`0x40e212e000000000`); maturity `38137` (`0x40e29f2000000000`); frequency `4` (`0x4010000000000000`); basis `1` (`0x3ff0000000000000`) (cells) | `89` (`0x4056400000000000`) | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |
| `COUPDAYS(38044, 38411, 2, 1)` | settlement `38044` (`0x40e2938000000000`); maturity `38411` (`0x40e2c16000000000`); frequency `2` (`0x4000000000000000`); basis `1` (`0x3ff0000000000000`) (cells) | `184` (`0x4067000000000000`) | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |
| `COUPDAYS(34911, 35216, 2, 1)` | settlement `34911` (`0x40e10be000000000`); maturity `35216` (`0x40e1320000000000`); frequency `2` (`0x4000000000000000`); basis `1` (`0x3ff0000000000000`) (cells) | `183` (`0x4066e00000000000`) | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |
| `COUPPCD(44043, 2958465, 2, 1)` | settlement `44043` (`0x40e5816000000000`); maturity `2958465` (`0x4146924080000000`); frequency `2` (`0x4000000000000000`); basis `1` (`0x3ff0000000000000`) (cells) | `44012` (`0x40e57d8000000000`) | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |
| `COUPNCD(44043, 2958465, 2, 1)` | settlement `44043` (`0x40e5816000000000`); maturity `2958465` (`0x4146924080000000`); frequency `2` (`0x4000000000000000`); basis `1` (`0x3ff0000000000000`) (cells) | `44196` (`0x40e5948000000000`) | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |
| `COUPDAYS(44043, 2958465, 2, 1)` | settlement `44043` (`0x40e5816000000000`); maturity `2958465` (`0x4146924080000000`); frequency `2` (`0x4000000000000000`); basis `1` (`0x3ff0000000000000`) (cells) | `183` (`0x4066e00000000000`) | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |

Basis (measured): All 5,584 rows of three recorded sets: 3,012 and 1,512 rows from build 20430,
1,060 from build 20131. The rule was recomputed independently from the calendar and matched all
5,566 number rows with settlement at serial 61 or later, on every basis. The other two number
rows have settlement 0 and basis 0, and read 180. In a study of 42 ways to walk and measure the
schedule, this was the only one to match all 965 rows with basis exactly 1.

Inferred, not measured: The rule reproduces every recorded row; that Excel's code works this
way is inferred. For basis 1 no settlement or maturity before serial 61, where Excel's calendar
differs from the Gregorian one, is in the recorded rows.

Evidence:

- DnaCalc `crates/calc-func/src/ported/coupon_family.rs`, coupdays_kernel at `14910720ec` — the rule as implemented
- DnaCalc `crates/calc-func/tests/f3_financial.rs`, coupdays_actual_actual_uses_its_recorded_calendar_interval at `14910720ec` — six recorded calls pinned
- DnaCalc-Research `evidence/round24/coupon/coupon-calendar-summary.json` at `19d810d45a` — the 42 schedule variants compared on 965 basis 1 rows
- DnaCalc-Research `evidence/round24/coupon/coupdays-registered-delta.json` at `19d810d45a` — 5,584 of 5,584 rows matched
- DnaCalc `crates/calc-func/witness-status.tsv`, COUPDAYS at `2d0eddcac8` — matched rows per function
- OxFunc-Evidence `runs/w111-5-20260924/answers/answers-w1115-coupdays.json.gz` at `e26b7bddb8` — 1,512 recorded rows, inputs read again
- OxFunc-Evidence `cache/oracle-20430/build-20430/COUPDAYS.jsonl.gz`, COUPDAYS|0x40e1748000000000|0x40e1910000000000|0x4000000000000000|0x3ff0000000000000 at `e26b7bddb8` (readings)
- OxFunc-Evidence `cache/oracle-20430/build-20430/COUPDAYS.jsonl.gz`, COUPDAYS|0x40e1cc8000000000|0x40e2036000000000|0x4010000000000000|0x3ff0000000000000 at `e26b7bddb8` (readings)
- OxFunc-Evidence `cache/oracle-20430/build-20430/COUPDAYS.jsonl.gz`, COUPDAYS|0x40e1f98000000000|0x40e7646000000000|0x4010000000000000|0x3ff0000000000000 at `e26b7bddb8` (readings)
- OxFunc-Evidence `cache/oracle-20430/build-20430/COUPDAYS.jsonl.gz`, COUPDAYS|0x40e212e000000000|0x40e29f2000000000|0x4010000000000000|0x3ff0000000000000 at `e26b7bddb8` (readings)
- OxFunc-Evidence `cache/oracle-20430/build-20430/COUPDAYS.jsonl.gz`, COUPDAYS|0x40e2938000000000|0x40e2c16000000000|0x4000000000000000|0x3ff0000000000000 at `e26b7bddb8` (readings)
- OxFunc-Evidence `cache/oracle-20430/build-20430/COUPDAYS.jsonl.gz`, COUPDAYS|0x40e10be000000000|0x40e1320000000000|0x4000000000000000|0x3ff0000000000000 at `e26b7bddb8` (readings)
- OxFunc-Evidence `cache/oracle-20430/build-20430/COUPPCD.jsonl.gz`, COUPPCD|0x40e5816000000000|0x4146924080000000|0x4000000000000000|0x3ff0000000000000 at `e26b7bddb8` (readings)
- OxFunc-Evidence `cache/oracle-20430/build-20430/COUPNCD.jsonl.gz`, COUPNCD|0x40e5816000000000|0x4146924080000000|0x4000000000000000|0x3ff0000000000000 at `e26b7bddb8` (readings)
- OxFunc-Evidence `cache/oracle-20430/build-20430/COUPDAYS.jsonl.gz`, COUPDAYS|0x40e5816000000000|0x4146924080000000|0x4000000000000000|0x3ff0000000000000 at `e26b7bddb8` (readings)

### FN-COUPDAYSNC-001: On basis 0 COUPDAYSNC is not always COUPDAYS minus COUPDAYBS

*Quirk; measured; recorded 2026-11-04.* Applies to COUPDAYBS, COUPDAYS, COUPDAYSNC.

On basis 0 (US 30/360), COUPDAYSNC is P - A. A is COUPDAYBS, the accrued days. P is a 30/360
count between the actual previous and next coupon dates, with day 31 and the last day of
February both counted as day 30. COUPDAYS on basis 0 is always 360/frequency, but P need not
be: when a coupon falls on the last day of February and maturity is not a month end, P can be
88, 89 or 92 for a quarter and 178 or 182 for a half year. So COUPDAYSNC(39923,50919,4,0), with
coupons 2009-02-28 and 2009-05-29, is 39: P is 89 and A is 50, where 90 - 50 would give 40.
With no coupon date before settlement in Excel's calendar, the period starts at serial 0:
COUPDAYSNC(33,400,2,0) is 1, and for settlement 61, maturity 456 and annual coupons COUPDAYBS
plus COUPDAYSNC is 90, not 360. This rule matches all 1,664 recorded number rows on basis 0,
733 of them chosen near February, month ends and the start of the calendar.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `COUPDAYSNC(39923, 50919, 4, 0)` | settlement `39923` (`0x40e37e6000000000`); maturity `50919` (`0x40e8dce000000000`); frequency `4` (`0x4010000000000000`); basis `0` (`0x0000000000000000`) (cells) | `39` (`0x4043800000000000`) | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `COUPDAYSNC(45986, 57038, 2, 0)` | settlement `45986` (`0x40e6744000000000`); maturity `57038` (`0x40ebd9c000000000`); frequency `2` (`0x4000000000000000`); basis `0` (`0x0000000000000000`) (cells) | `95` (`0x4057c00000000000`) | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `COUPDAYSNC(46478, 46627, 4, 0)` | settlement `46478` (`0x40e6b1c000000000`); maturity `46627` (`0x40e6c46000000000`); frequency `4` (`0x4010000000000000`); basis `0` (`0x0000000000000000`) (cells) | `57` (`0x404c800000000000`) | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `COUPDAYSNC(33, 400, 2, 0)` | settlement `33` (`0x4040800000000000`); maturity `400` (`0x4079000000000000`); frequency `2` (`0x4000000000000000`); basis `0` (`0x0000000000000000`) (cells) | `1` (`0x3ff0000000000000`) | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `COUPDAYBS(61, 456, 1, 0)` | settlement `61` (`0x404e800000000000`); maturity `456` (`0x407c800000000000`); frequency `1` (`0x3ff0000000000000`); basis `0` (`0x0000000000000000`) (cells) | `61` (`0x404e800000000000`) | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `COUPDAYSNC(61, 456, 1, 0)` | settlement `61` (`0x404e800000000000`); maturity `456` (`0x407c800000000000`); frequency `1` (`0x3ff0000000000000`); basis `0` (`0x0000000000000000`) (cells) | `29` (`0x403d000000000000`) | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |

Basis (measured): 1,664 recorded rows with a number answer on basis 0, from builds 20131, 20430
and 20527, recomputed independently from the calendar. An implementation of these rules matches
all 33,504 selected readings of the six coupon functions. Fictitious 1900-02-29 is not a month
end here.

Inferred, not measured: The split into P and A reproduces every row; that Excel's code forms
these two counts is inferred. For the February cases in this note only COUPDAYSNC is recorded:
their A comes from the rule, not from a COUPDAYBS reading.

Evidence:

- DnaCalc `crates/calc-func/src/ported/coupon_family.rs`, coupdaysnc_kernel at `7ec08d6707` — the rule as implemented
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/COUPDAYSNC.reading.json` at `2d0eddcac8` — 733 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/COUPDAYBS.reading.json` at `2d0eddcac8` — 120 recorded readings
- DnaCalc `.beads/issues.jsonl`, dc-zmk.4 at `2d0eddcac8` — the 33,504 selected coupon readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/COUPDAYSNC.reading.json`, fn7-COUPDAYSNC-remaining-calendar-1 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/COUPDAYSNC.reading.json`, fn7-COUPDAYSNC-remaining-calendar-2 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/COUPDAYSNC.reading.json`, fn7-COUPDAYSNC-full-period-holdout-90 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/COUPDAYSNC.reading.json`, fn7-COUPDAYSNC-calendar-boundary-127 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/COUPDAYBS.reading.json`, fn7-COUPDAYBS-fake-february-43 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/COUPDAYSNC.reading.json`, fn7-COUPDAYSNC-fake-february-43 at `2d0eddcac8` (readings)

<!-- efh:notes end -->

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
- Handbook evidence record `EV-FIN-0030` — what the six `COUP*` figures are, and are not.
- Handbook chapters [01 the value universe](../model/01-value-universe.md),
  [02 coercion and lifting](../model/02-coercion-and-lifting.md),
  [06 claim language](../model/06-claim-language.md).
- OxFunc `crates/oxfunc_core/src/functions/coupon_family.rs` at commit `473efa3` — the reference
  engine's per-basis period-length table, the actual/actual anchor fork, and validation.
- `data/functions/FUNC.COUPDAYS.json`, `data/presence/FUNC.COUPDAYS.json`,
  `data/battery/FUNC.COUPDAYS.json`.
