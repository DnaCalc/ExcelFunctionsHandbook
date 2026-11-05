---
schema: efh.function-page/v1
function_id: FUNC.COUPPCD
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-FIN-0030
open_problems: []
references:
  - work: "Microsoft Learn: WorksheetFunction.CoupPcd method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.couppcd"
    role: "the parameter list, the basis table, the truncation rule and the four documented error conditions; note that this page carries no description sentence"
  - work: "Microsoft 365 support: COUPPCD function"
    locator: "https://support.microsoft.com/en-us/office/couppcd-function-2eb50473-6ee9-4052-a206-77a9a385d5b3"
    role: "the worksheet-surface page, and the source of the one-line description carried in the Handbook's projection; cited but not retrieved — the host returned HTTP 403 to the Handbook's fetch"
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
  - Page vocabulary
  - Sources
family: coupon_family
role_in_family: >-
  The backward end of the schedule: the coupon date accrual is measured from, and the member
  whose Microsoft Learn reference page has lost its description sentence.
---

## What it computes

`COUPPCD(settlement, maturity, frequency, [basis])` returns **the coupon date on or before the
settlement date** — the start of the current coupon period — as an Excel date serial.

It is the mirror of [COUPNCD](FUNC.COUPNCD.md), and the two together bracket the period that
[COUPDAYBS](FUNC.COUPDAYBS.md), [COUPDAYSNC](FUNC.COUPDAYSNC.md) and
[COUPDAYS](FUNC.COUPDAYS.md) measure. The schedule it selects from is described once, on
[COUPNCD](FUNC.COUPNCD.md#the-coupon-schedule): coupon dates run backwards from `maturity` in
steps of `12/frequency` months, with month-end anchoring, and there is no issue date to stop
them.

**The previous coupon date is notional, not historical.** The schedule is generated from
`maturity` alone, so `COUPPCD` will happily return a date before the security existed. That is
the intended behaviour for accrual arithmetic on a seasoned bond, and it is a trap for anyone
using the function to ask "when was the last coupon actually paid".

### A documentation gap worth recording

Microsoft's Learn reference page for this function is **missing its description sentence**. Every
sibling page opens with a one-line statement of what the function returns — "Returns the number
of days from the beginning of the coupon period to the settlement date", and so on. The
`CoupPcd` page goes straight from the title to the Syntax heading. The parameter table, the basis
table, the date notes and the four error conditions are all present; only the sentence saying
what the function is for is absent. The description carried in the Handbook's own projection —
"Returns the previous coupon date before the settlement date" — comes from Microsoft's
worksheet-surface support page, which was not retrieved for this entry.

*The quoted one-line description is Microsoft's text, from its support article <https://support.microsoft.com/en-us/office/couppcd-function-2eb50473-6ee9-4052-a206-77a9a385d5b3>.*

Note also that the projected description says "**before** the settlement date", while the
reference engine returns `settlement` itself when settlement lands exactly on a coupon date. On
that input the description and the reference engine disagree, and Microsoft's Learn page says
nothing either way.

## Arguments

`COUPPCD(settlement, maturity, frequency, [basis])` — the family's shared four arguments, with
Microsoft's wording quoted on [COUPDAYBS](FUNC.COUPDAYBS.md#arguments).

| Argument | Meaning | Required? |
|---|---|---|
| `settlement` | The security's settlement date. | Required |
| `maturity` | The security's maturity date. | Required |
| `frequency` | Coupon payments per year: 1, 2 or 4. | Required |
| `basis` | The day-count basis, 0–4. | Optional, defaults to 0 |

The standard five-row basis table is documented on this page — 0 or omitted = US (NASD) 30/360,
1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360.

**`basis` cannot change the answer.** A coupon date is a calendar date; no day-count convention
moves it. `basis` is nevertheless validated, so an out-of-range value converts a well-defined
date into an error. The same non-participating-but-fatal argument appears on
[COUPNCD](FUNC.COUPNCD.md) and [COUPNUM](FUNC.COUPNUM.md).

Microsoft documents that **"All arguments are truncated to integers."** Dates should be produced
by `DATE` or by other formulas rather than typed as text; ordinary coercion rules are in
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns a `Number`: an Excel date serial. Whether Excel attaches a date-format presentation hint
to the result — the mechanism described in
[The value universe](../model/01-value-universe.md) — has not been established here; the
reference engine returns a bare number.

- **`settlement` exactly on a coupon date** returns `settlement` itself under the reference
  engine, because the schedule rolls forward and treats that date as the start of the new
  period. This contradicts the "previous coupon date **before** the settlement date" wording of
  the projected description, and Microsoft's Learn page does not resolve it. Probing it is the
  first item below.
- **Month-end maturities** produce month-end previous-coupon dates: a 31 August maturity at
  `frequency` 2 gives 28 or 29 February, not the 28th of some other month.
- **Early settlements** are accepted; the schedule extends backwards past any plausible issue
  date, and when the coupon before settlement falls before 1900 Excel returns 0 (FN-COUPNUM-001).
- **`settlement` ≥ `maturity`** is a documented error, so the function never returns `maturity`
  itself as a previous coupon date.
- The reference engine clamps a computed previous-coupon serial at zero, so extremely early
  settlements cannot produce a negative serial.
- Empty, missing and error arguments follow the shared call model.

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

- **[COUPNCD](FUNC.COUPNCD.md)** — the forward end of the same period, and the page that hosts
  the shared schedule description.
- **[COUPDAYBS](FUNC.COUPDAYBS.md)** — the day count from this date to `settlement`. On bases 1,
  2 and 3 it equals `settlement − COUPPCD`; on the 30/360 bases it does not.
- **[COUPDAYS](FUNC.COUPDAYS.md)** — the period length. `COUPNCD − COUPPCD` is the *actual*
  length and equals `COUPDAYS` only under basis 1, and not always there (FN-COUPDAYS-001).
- **[COUPNUM](FUNC.COUPNUM.md)** — the count of remaining coupons on the same ladder.
- **[EDATE](FUNC.EDATE.md)** — the general month-stepping function. `EDATE` clamps the day of
  month but does not make month-ends sticky, so `EDATE(maturity, −n × 12/frequency)` reproduces
  `COUPPCD` for ordinary maturities and diverges for month-end ones.
- **[ACCRINT](FUNC.ACCRINT.md)** — the function that does know an issue date, for cases where
  the notional previous coupon date is the wrong accrual anchor.
- **Confused with**: "the date of the last coupon actually paid", which this function does not
  answer for a bond issued after the notional date it returns.

## Numerical notes

1. **The answer is an integer-valued date serial**; there is no floating-point content.
2. **Iterating and indexing differ.** Stepping back one period at a time under a clamping month
   rule is not the same as computing the period index and jumping. The reference engine
   iterates, and its cost grows with the settlement-to-maturity gap.
3. **Month-end anchoring is a modelling decision** with no documentary support, and it is where
   two careful implementations will part company. State the rule explicitly rather than
   inheriting it from whatever a date library happens to do.
4. **Validate `basis` and then ignore it.** Computing this date through a day-count routine
   would introduce a dependency the function does not have.
5. **The roll-forward branch** decides the equality case and should be settled before anything
   else, because it also fixes `COUPDAYBS`'s zero and `COUPNUM`'s count on the same input.

## What has not been checked

No Handbook vector suite exists for `COUPPCD`. The six-surface record `EV-FIN-0030` lists this
surface and holds no pass rate: it has a capture count of 6 × 1,060 rows, which says nothing about
how they scored, and an uncounted statement that the reference engine's six functions matched Excel
in one scan on the older build 20026. The shared `coupon_family` module is named by no record. The
Excel readings of `COUPPCD` on this page are in FN-COUPDAYS-001 and FN-COUPNUM-001. The battery on
this page is the reference engine answering its own probes; no Excel was involved.

Two things on this page are documentation findings rather than measurements: the missing
description sentence on Microsoft's Learn page, and the "before the settlement date" wording that
the reference engine contradicts on the equality case.

Inputs worth probing first:

1. **`settlement` exactly on a coupon date.** Does Excel return `settlement` or the coupon date
   before it? This single observation settles the wording question and simultaneously fixes
   `COUPDAYBS`, `COUPDAYSNC`, `COUPNCD` and `COUPNUM` on the same input. The notes below read
   `COUPNUM` on such a date, not `COUPPCD`.
2. **Month-end maturities** — 31 January, 28/29 February, 30 April, 31 August — at each
   `frequency`, several periods before maturity. This decides month-end anchoring against day
   clamping. FN-COUPDAYS-001 has one month-end maturity, where `COUPPCD` keeps the month end.
3. **`COUPPCD` against `EDATE(COUPNCD(...), −12/frequency)`**, which isolates the month rule.
4. **A settlement decades before maturity**, testing whether an iterating walk and a direct
   index computation agree.
5. **A 29 February maturity**, combining the month-end and leap-year questions.
6. **An out-of-range `basis`** on an otherwise valid call, confirming that an argument which
   cannot affect the answer can still destroy it.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `COUPPCD` that an implementer matching Excel, or a reader explaining an
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

### FN-COUPNUM-001: COUPNUM is #NUM! when the coupon before settlement falls before 1900

*Quirk; measured; recorded 2026-11-04.* Applies to COUPNCD, COUPNUM, COUPPCD.

If the coupon date before settlement, counted back from maturity, falls before 1 January 1900,
COUPNUM gives #NUM!. The other coupon functions still answer for the same inputs: COUPPCD
returns 0, COUPNCD the next coupon, and COUPDAYS, COUPDAYBS and COUPDAYSNC a day count. With
maturity serial 400 (1901-02-03) and semi-annual coupons, the coupon dates are serials 34 and
216: settlement 33 gives #NUM!, settlement 34 gives 2. With maturity 367, settlement 0 gives
#NUM! and settlement 1 gives 2. This covers settlement 0 too: COUPNUM(0,m,2,0) was #NUM! for
every maturity read.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `COUPNUM(33, 400, 2, 0)` | settlement `33` (`0x4040800000000000`); maturity `400` (`0x4079000000000000`); frequency `2` (`0x4000000000000000`); basis `0` (`0x0000000000000000`) (cells) | `#NUM!` | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `COUPNUM(34, 400, 2, 0)` | settlement `34` (`0x4041000000000000`); maturity `400` (`0x4079000000000000`); frequency `2` (`0x4000000000000000`); basis `0` (`0x0000000000000000`) (cells) | `2` (`0x4000000000000000`) | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `COUPNUM(0, 367, 2, 0)` | settlement `0` (`0x0000000000000000`); maturity `367` (`0x4076f00000000000`); frequency `2` (`0x4000000000000000`); basis `0` (`0x0000000000000000`) (cells) | `#NUM!` | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `=COUPNUM(0,182,2,0)` | as typed | `#NUM!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=COUPNUM(A1,400,2,0)` | A1 `0` (`0x0000000000000000`) (cells) | `#NUM!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=COUPPCD(A1,400,2,0)` | A1 `0` (`0x0000000000000000`) (cells) | `0` (`0x0000000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=COUPNCD(A1,400,2,0)` | A1 `0` (`0x0000000000000000`) (cells) | `34` (`0x4041000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): The rule matches all 429 recorded COUPNUM readings (325 #NUM!, 104 counts)
when recomputed independently from the calendar, and the 24 COUPNUM readings of the
date-boundary sheets. COUPPCD, COUPNCD, COUPDAYS, COUPDAYBS and COUPDAYSNC companions come from
the same sheets.

Evidence:

- DnaCalc `crates/calc-func/src/ported/coupon_family.rs`, coupnum_kernel at `2033d9feb7` — the limit as implemented
- DnaCalc `apps/dna-onecalc/tests/acceptance.rs`, delivered_fn4_coupon_counts_at_calendar_start at `2d0eddcac8` — recorded answers pinned
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/COUPNUM.reading.json` at `2d0eddcac8` — 429 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-date-domain.capture.json`, sheets qd153 to qd236 at `2d0eddcac8` — boundary sheets for all six coupon functions
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/COUPNUM.reading.json`, fn7-COUPNUM-calendar-boundary-127 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/COUPNUM.reading.json`, fn7-COUPNUM-calendar-boundary-128 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/COUPNUM.reading.json`, fn7-COUPNUM-calendar-boundary-85 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-date-domain.capture.json`, sheet qd230, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-date-domain.capture.json`, sheet qd205, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-date-domain.capture.json`, sheet qd217, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-date-domain.capture.json`, sheet qd193, row 20, column 10 at `2d0eddcac8` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| previous coupon date | The schedule date on or before `settlement`; the start of the current period |
| notional coupon date | A schedule date generated backwards from `maturity`, whether or not a coupon was paid then |
| month-end anchoring | The rule that a month-end anchor generates month-end dates |
| roll forward | The reference engine's treatment of a settlement landing exactly on a coupon date |
| date serial | The number Excel uses to represent a date; see the value-universe chapter |

## Sources

- Microsoft Learn, **WorksheetFunction.CoupPcd method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.couppcd>. Source of
  the parameter table, the five-row basis table, the `Double` return type, the "All arguments
  are truncated to integers" rule, the date-entry warning, and the four error conditions. This
  page carries **no description sentence**, which is recorded above as a documentation finding.
- Microsoft 365 support, **COUPPCD function** —
  <https://support.microsoft.com/en-us/office/couppcd-function-2eb50473-6ee9-4052-a206-77a9a385d5b3>.
  The worksheet-surface page and the origin of the projected description "Returns the previous
  coupon date before the settlement date"; **not retrieved** for this entry (the host returned
  HTTP 403), so nothing here is quoted from it.
- Handbook, [COUPNCD](FUNC.COUPNCD.md#the-coupon-schedule) — the shared schedule construction.
- Handbook evidence record `EV-FIN-0030` — what the six `COUP*` figures are, and are not.
- Handbook chapters [01 the value universe](../model/01-value-universe.md),
  [02 coercion and lifting](../model/02-coercion-and-lifting.md),
  [06 claim language](../model/06-claim-language.md).
- OxFunc `crates/oxfunc_core/src/functions/coupon_family.rs` at commit `473efa3`.
- `data/functions/FUNC.COUPPCD.json` (the projected description quoted above),
  `data/presence/FUNC.COUPPCD.json`, `data/battery/FUNC.COUPPCD.json`.
