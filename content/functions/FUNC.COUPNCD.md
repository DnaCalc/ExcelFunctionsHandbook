---
schema: efh.function-page/v1
function_id: FUNC.COUPNCD
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn: WorksheetFunction.CoupNcd method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.coupncd"
    role: "the parameter list, the basis table, the truncation rule and the four documented error conditions"
  - work: "Microsoft 365 support: COUPNCD function"
    locator: "https://support.microsoft.com/en-us/office/coupncd-function-fd962fef-506b-4d9d-8590-16df5393691f"
    role: "the worksheet-surface page; cited but not retrieved — the host returned HTTP 403 to the Handbook's fetch"
episodes: []
body_sections:
  - What it computes
  - The coupon schedule
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
  The forward end of the schedule, and the family's canonical construction: every other member
  is a measurement taken against the coupon dates this function returns.
---

## What it computes

`COUPNCD(settlement, maturity, frequency, [basis])` returns **the next coupon date strictly
after the settlement date** — as a date serial, which Excel's date formatting renders as a date.

Microsoft's Learn page describes it as returning "a number that represents the next coupon date
after the settlement date", and declares the return type as `Double`. It is a date-valued
function returning a plain number.

The whole content of the function is the schedule it derives, so this page hosts the
description that the other five coupon functions refer to.

## The coupon schedule

None of the six coupon functions is given a coupon schedule. They are given a **maturity date**
and a **frequency**, and they reconstruct the schedule from those two facts:

>     the coupon dates are  maturity,  maturity − p,  maturity − 2p,  maturity − 3p, …
>     where p = 12 / frequency  months

Coupon dates are therefore anchored at *maturity*, counted **backwards**, and there is no issue
date to stop the schedule; FN-COUPNUM-001 records what Excel does when it runs back past the start
of 1900. `settlement` selects a period from that ladder: the one whose start is at or before
`settlement` and whose end is after it.

Three consequences do most of the work on all six pages:

1. **The day of the month comes from maturity, not from settlement.** A bond maturing on the
   15th has coupons on the 15th. Everything else about the trade is irrelevant to the schedule.
2. **Month arithmetic needs a month-end rule.** Stepping back six months from 31 August must land
   somewhere in February, and "the 31st" does not exist there. The reference engine's rule is
   *month-end sticks to month-end*: if the anchor date is the last day of its month, every
   generated date is the last day of its month; otherwise the day number is clamped down to the
   length of the target month. That rule makes the schedule from a 30 June maturity differ from
   the schedule from a 30 September maturity — one is a month end, the other is not.
3. **Settlement exactly on a coupon date rolls forward.** Under the reference engine, if
   `settlement` lands on a generated coupon date, that date is treated as the *start* of the
   current period rather than its end. The practical effect is that `COUPPCD` returns
   `settlement` itself, `COUPDAYBS` returns zero, `COUPNCD` returns the following coupon date,
   and `COUPNUM` counts one fewer remaining coupon. Microsoft's pages do not state this
   convention; it is the reference engine's. The notes below fit it without reading `COUPNCD` on a
   coupon date: `COUPNUM(34,400,2,0)` is 2 with coupon dates at 34, 216 and 400 (FN-COUPNUM-001),
   and on basis 1 `COUPDAYS` stops at the last date still after settlement (FN-COUPDAYS-001).

The schedule is *quasi*-periodic, not periodic: because the month-end rule clamps, stepping back
`n` periods is not always the same as stepping back one period `n` times from an intermediate
date. Implementations that step iteratively and implementations that compute the offset in one
jump can disagree.

**None of this is documented.** Microsoft's pages for all six functions document the arguments,
the basis table, the truncation rule and four error conditions, and say nothing about how the
schedule is generated. The construction above is the reference engine's, read from its source,
and it is the natural reading of the market convention. The notes below verify only parts of it:
in FN-COUPDAYS-001 `COUPNCD` and `COUPPCD` keep month ends for one month-end maturity, while
`COUPDAYS` on basis 1 clamps its way back from maturity instead.

## Arguments

`COUPNCD(settlement, maturity, frequency, [basis])` — the same four arguments as every other
member of the family.

| Argument | Meaning | Required? |
|---|---|---|
| `settlement` | The security's settlement date — "the date after the issue date when the security is traded to the buyer". | Required |
| `maturity` | The security's maturity date — "the date when the security expires". | Required |
| `frequency` | Coupon payments per year. Microsoft: "For annual payments, frequency = 1; for semiannual, frequency = 2; for quarterly, frequency = 4." | Required |
| `basis` | The day-count basis, 0–4. | Optional, defaults to 0 |

The argument descriptions are quoted from Microsoft's Learn page.

`basis` is documented on this page with the standard five-row table:

| `basis` | Day count basis |
|---|---|
| 0 or omitted | US (NASD) 30/360 |
| 1 | Actual/actual |
| 2 | Actual/360 |
| 3 | Actual/365 |
| 4 | European 30/360 |

**`basis` cannot change this function's answer.** A coupon date is a calendar date; no day-count
convention moves it. `COUPNCD` nevertheless accepts `basis` and validates it, so an out-of-range
`basis` turns a perfectly well-defined date into an error. That is a real, observable dependency
on an argument that does not participate in the computation — worth knowing when a `COUPNCD`
formula starts erroring after someone edits a shared basis cell.

Microsoft documents that **"All arguments are truncated to integers."** Dates should be produced
by `DATE` or by other formulas rather than typed as text; ordinary coercion rules are in
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns a `Number`: an Excel date serial. Whether Excel attaches a date-format presentation hint
to the result — the mechanism described in
[The value universe](../model/01-value-universe.md) for `TODAY` — has not been established here;
the reference engine returns a bare number.

- **`settlement` on a coupon date** returns the *following* coupon date under the reference
  engine, not `settlement` itself. See the schedule section above; this is undocumented.
- **`settlement` one day before maturity** returns `maturity`, which is always a coupon date by
  construction.
- **`settlement` ≥ `maturity`** is a documented error.
- **Month-end maturities** propagate month-end coupon dates: a 31 August maturity at
  `frequency` 2 gives a 28 or 29 February coupon, not a 28th-of-the-month coupon.
- **Very early settlements** are accepted: a settlement decades before maturity simply selects a
  period far down the ladder, and `COUPNCD` still answers when the coupon before settlement falls
  before 1900 (FN-COUPNUM-001). The reference engine walks the schedule one period at a time to
  find it, so the cost grows with the gap.
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

The reference engine maps all four onto `#NUM!`, and additionally returns `#NUM!` for a
non-finite argument and for a date serial outside the representable range. Error values in any
argument propagate. The reference engine's choice of `#NUM!` for an invalid date — rather than
the `#VALUE!` that [ACCRINT](FUNC.ACCRINT.md) uses for the same failure in the same engine — is
an internal inconsistency worth noting; the documentation names no code at all, so neither is
contradicted.

## Relationships

- **[COUPPCD](FUNC.COUPPCD.md)** — the backward end of the same period. `COUPPCD` and `COUPNCD`
  bracket `settlement`, and together they define the current coupon period.
- **[COUPDAYBS](FUNC.COUPDAYBS.md)**, **[COUPDAYSNC](FUNC.COUPDAYSNC.md)**,
  **[COUPDAYS](FUNC.COUPDAYS.md)** — the three day counts taken against those two dates, though
  on basis 1 `COUPDAYS` can use another interval (FN-COUPDAYS-001). Note
  that the three do not always add up; see the additivity discussion on
  [COUPDAYS](FUNC.COUPDAYS.md).
- **[COUPNUM](FUNC.COUPNUM.md)** — how many coupon dates remain from `settlement` to `maturity`
  on the same ladder.
- **`PRICE`, `YIELD`, `DURATION`, `MDURATION`** — the consumers. They rebuild this same schedule
  internally rather than calling these functions, which is why a schedule disagreement between
  Excel's `COUPNCD` and Excel's `PRICE` is possible in principle and has been observed for the
  *day counts* in this family; see [COUPDAYS](FUNC.COUPDAYS.md).
- **[EDATE](FUNC.EDATE.md)** and **[EOMONTH](FUNC.EOMONTH.md)** — the general-purpose
  month-stepping functions. `EDATE` clamps the day of month; it does **not** implement
  "month-end sticks to month-end". Rebuilding a coupon schedule with `EDATE` therefore
  reproduces `COUPNCD` for ordinary maturities and diverges for month-end ones.
- **Confused with**: `EDATE(maturity, ±n × 12/frequency)`, for exactly that reason.

## Numerical notes

There is no floating-point content here worth the name: the answer is an integer-valued date
serial. The engineering content is all in the schedule.

1. **Iterate or jump, but be consistent.** Stepping back one period at a time and computing the
   period index directly give different answers under a clamping month rule. The reference
   engine iterates.
2. **The month-end predicate is a decision, not a fact.** "Is this date the last day of its
   month?" is cheap; deciding that the answer should *propagate* to every generated date is a
   modelling choice with no documentary support.
3. **Cost is linear in the settlement-to-maturity gap** under an iterating implementation. A
   30-year quarterly bond settled at issue costs 120 iterations, which is fine; a synthetic test
   with a maturity centuries out is not.
4. **Do not route this through a day-count routine.** `basis` must be *validated* and then
   ignored. An implementation that computes the answer via a year fraction will introduce
   basis-dependence into a function that has none.

## What has not been checked

No Handbook vector suite exists for `COUPNCD`, and **no evidence record lists this surface in
its subjects**. The `coupon_family` module that implements all six coupon functions is named by
no record either. The Excel readings of `COUPNCD` on this page are in FN-COUPDAYS-001 and
FN-COUPNUM-001.
The battery on this page is the reference engine answering its own probes; no Excel was involved.

The schedule construction described above is only partly measured, and it is the load-bearing
unknown for the whole family. It
is also unusually cheap to probe, because the answer is a date rather than a float: a
disagreement is visible without any tolerance discussion.

Inputs worth probing first:

1. **`settlement` exactly on a coupon date.** The reference engine rolls forward. If Excel
   returns `settlement` itself, four of the six functions in this family change their answers at
   once. This is the highest-value single cell on the page. The notes below read `COUPNUM` on
   such a date, not `COUPNCD`.
2. **Month-end maturities**: 31 January, 28/29 February, 30 April, 31 August, at each
   `frequency`, with settlements several periods earlier. This decides "month-end sticks" versus
   "clamp the day number" — the two rules differ on 31 August at `frequency` 2. FN-COUPDAYS-001
   has one month-end maturity, where `COUPNCD` keeps the month end.
3. **A 29 February maturity**, which forces the month-end question and the leap-year question
   together.
4. **Settlement one day before and one day after a generated coupon date**, bracketing the
   roll-forward boundary.
5. **`COUPNCD` against `EDATE(COUPPCD(...), 12/frequency)`** — if the two disagree, the
   disagreement is in the month rule and nowhere else.
6. **An out-of-range `basis` on an otherwise valid call**, confirming that an argument which
   cannot affect the answer can still destroy it.
7. **A settlement many decades before maturity**, to see whether Excel's schedule walk agrees
   with a direct index computation.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `COUPNCD` that an implementer matching Excel, or a reader explaining an
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
| coupon schedule | The backward ladder of dates generated from `maturity` at `12/frequency`-month steps |
| coupon date | Any date on that ladder |
| current period | The schedule period containing `settlement` |
| month-end anchoring | The rule that a month-end anchor generates month-end dates |
| roll forward | The reference engine's treatment of a settlement landing exactly on a coupon date |

## Sources

- Microsoft Learn, **WorksheetFunction.CoupNcd method (Excel)** —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.coupncd>. Source of
  the one-line description, the parameter table with its quoted argument descriptions, the
  five-row basis table, the `Double` return type, the "All arguments are truncated to integers"
  rule, the date-entry warning, and the four error conditions.
- Microsoft 365 support, **COUPNCD function** —
  <https://support.microsoft.com/en-us/office/coupncd-function-fd962fef-506b-4d9d-8590-16df5393691f>.
  The worksheet-surface page; **not retrieved** for this entry (the host returned HTTP 403), so
  nothing here is quoted from it.
- Handbook chapters [01 the value universe](../model/01-value-universe.md),
  [02 coercion and lifting](../model/02-coercion-and-lifting.md),
  [06 claim language](../model/06-claim-language.md).
- OxFunc `crates/oxfunc_core/src/functions/coupon_family.rs` at commit `473efa3` — the reference
  engine's schedule generation, month-end anchoring, roll-forward rule and validation, read for
  every statement attributed to the reference engine above.
- `data/functions/FUNC.COUPNCD.json`, `data/presence/FUNC.COUPNCD.json`,
  `data/battery/FUNC.COUPNCD.json`.
