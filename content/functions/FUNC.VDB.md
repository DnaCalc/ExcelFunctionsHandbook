---
schema: efh.function-page/v1
function_id: FUNC.VDB
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — VDB function"
    locator: "https://support.microsoft.com/en-us/office/vdb-function-dde4e207-f3fa-488d-91d2-66d55e861d73"
    role: "syntax, argument meanings, the factor default, the no_switch meaning, the positive-number sentence and the six worked examples"
  - work: "Microsoft Learn — WorksheetFunction.Vdb method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.vdb"
    role: "the parameter table, the statement of when the switch to straight line happens, and the remark on positive numbers"
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
family: depreciation_family
role_in_family: >-
  The general declining-balance member: it covers any span of periods including fractions, and by
  default switches to straight line when that charge is larger. DDB is the fixed single-period
  member with no switch, and SLN the straight line it switches to.
---

# VDB

## What it computes

`VDB(cost, salvage, life, start_period, end_period, [factor], [no_switch])` returns the
**depreciation of an asset over the span from `start_period` to `end_period`**, by a declining
balance method. The name stands for variable declining balance.

Two things set it apart from [DDB](FUNC.DDB.md). It covers a span and not one period, and that span
can start and end at fractions of a period. And it switches to straight line when the declining
charge falls behind, unless told not to.

The underlying schedule works period by period from a book value that starts at `cost`:

    declining(k)  =  book(k−1) × factor / life,  limited to book(k−1) − salvage
    straight(k)   =  (book(k−1) − salvage) / (life − (k−1))

    charge(k)     =  the larger of declining(k) and straight(k)      no_switch FALSE
    charge(k)     =  declining(k)                                    no_switch TRUE

    book(k)       =  book(k−1) − charge(k)

Microsoft states the switch in one sentence: Excel switches to straight line when depreciation is
greater than the declining balance calculation. Once the schedule has switched it stays on the
straight line, because the remaining basis is then spread evenly over the remaining life.

`start_period` and `end_period` are **elapsed time**, not period numbers. The first full period is
the span from `0` to `1`:

- `VDB(c, s, l, 0, 1)` is the first period's charge.
- `VDB(c, s, l, 1, 1)` is `0`, an empty span. It is not the first period.
- `VDB(c, s, l, 0, l)` is the whole life.

A worked schedule, by the rule above, for `cost` 1000, `salvage` 0, `life` 5, `factor` 2:

| Period | Book at start | Declining | Straight line | Charge with switch | Charge with `no_switch` TRUE |
|---|---:|---:|---:|---:|---:|
| 1 | 1000.00 | 400.00 | 200.00 | 400.00 | 400.00 |
| 2 | 600.00 | 240.00 | 150.00 | 240.00 | 240.00 |
| 3 | 360.00 | 144.00 | 120.00 | 144.00 | 144.00 |
| 4 | 216.00 | 86.40 | 108.00 | **108.00** | 86.40 |
| 5 | 108.00 | 43.20 | 108.00 | **108.00** | 51.84 |
| Total | | | | 1000.00 | 922.24 |

The book and declining columns follow the charge with the switch. The switch lands in period 4, and
with it the asset is depreciated down to `salvage` exactly. Without it 77.76 is never charged. That
shortfall is why the switch exists. This table is the Handbook's arithmetic from the rule, not a
reading.

Microsoft's six examples, for cost 2400, salvage 300 and a life of 10 years, all follow the
schedule, with fractional spans prorated inside a period as described below:

| Formula | Meaning | Result |
|---|---|---|
| `=VDB(2400, 300, 3650, 0, 1)` | First day | `1.32` |
| `=VDB(2400, 300, 120, 0, 1)` | First month | `40.00` |
| `=VDB(2400, 300, 10, 0, 1)` | First year | `480.00` |
| `=VDB(2400, 300, 120, 6, 18)` | Month 6 to month 18 | `396.31` |
| `=VDB(2400, 300, 120, 6, 18, 1.5)` | The same at factor 1.5 | `311.81` |
| `=VDB(2400, 300, 10, 0, 0.875, 1.5)` | First fiscal year, part year, factor 1.5 | `315.00` |

The last example shows the proration: a full first year at factor 1.5 is 360, and seven eighths of
that is 315.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `cost` | The initial cost of the asset. Required. | none |
| `salvage` | The value at the end of the depreciation. Can be 0. Required. | none |
| `life` | The number of periods over which the asset is depreciated. Required. | none |
| `start_period` | The start of the span, in the same units as `life`. Required. | none |
| `end_period` | The end of the span, in the same units as `life`. Required. | none |
| `factor` | The rate at which the balance declines. Optional. | 2 if left off the end of the call |
| `no_switch` | `TRUE` to stay on declining balance; `FALSE` to allow the switch. Optional. | `FALSE` |

- **An empty `factor` slot is not the default.** Microsoft says an omitted `factor` is 2. The notes
  below record that this holds only when the argument is left off the end of the call. An empty
  slot, as in `VDB(c, s, l, a, b, , FALSE)`, is read as `0`, which gives the straight-line amount.
  To reach `no_switch` while keeping double declining, write the 2.
- **A blank cell as `factor` behaves like the empty slot.** In the note's example a blank cell, and
  a cell holding `0`, both give the same 90 as the empty slot. A cell holding `-1` gives `#NUM!`.
- **Text in `factor` and `no_switch` is treated differently in each slot.** `factor` reads `"2"` as 2,
  and gives `#VALUE!` for `"TRUE"`, `"FALSE"` and `""`. In the `no_switch` slot of the same recorded
  run, `"TRUE"` and `"FALSE"` are accepted, and `"2"` and `""` give `#VALUE!`.
- **Units must match.** `life`, `start_period` and `end_period` are in the same units. Monthly
  depreciation of a ten-year asset uses `life` 120, as in the examples above.

The reference engine at commit `473efa3` treats an empty or blank `factor` as the default of 2. The
notes below show that this is not Excel's behaviour.

## Result and edge cases

Returns `Number`.

Recorded behaviour, from readings on Excel 16.0 build 20430, Windows 64-bit, CPU not recorded,
classified for this page:

- **`start_period = end_period`** gives `0` when the span lies inside the life.
- **`factor = 0`** gives straight line. With `no_switch` FALSE and `cost` above `salvage`, 10,427 of
  10,686 recorded answers are within 1e−9 relative of `(cost − salvage) / life × (end − start)`.
  The other 259 all have `cost` below 1e−38. With `no_switch` TRUE, all 9,322 recorded answers are
  `0`, because there is nothing to switch to and the declining rate is zero.
- **`salvage` above `cost`** gives a **negative** number, not an error:
  `VDB(1, 1000, 1, 0, 0.5, 2, FALSE)` is `-499.5`.
- **`cost = 0`** is admitted, and with a positive `salvage` also gives a negative number:
  `VDB(0, 1, 1, 0, 0.5, 2, FALSE)` is `-0.5`.
- **`salvage = 0`** is admitted, as Microsoft's parameter table says.
- **Inputs near the limits of the number range** can give `#NUM!` with no other condition met, for
  example with an enormous `factor`.

**The span is not additive.** This is the most important behaviour on the page, and the arithmetic
of the schedule would not lead you to expect it. For `cost` 1000, `salvage` 100, `life` 1.5 and
`factor` 1:

| Span | Excel |
|---|---:|
| 0 to 1.25 | 783.3333333333333 |
| 1.25 to 1.5 | 146.66666666666669 |
| Sum of the two | 930.0 |
| 0 to 1.5 | 900.0 |

Splitting a span and adding the pieces does not give the whole. The recorded research on these
readings concludes that a fractional part of `start_period` is applied first, which changes the
basis on which later whole periods decline. So a depreciation schedule built from consecutive
fractional spans need not total `cost − salvage`.

Where this applies can be read from the recorded answers. Taking every case where a span and its
two pieces were all read, and calling a split additive when the pieces sum to the whole within 1e−9
relative:

| Path | First piece starts | Split point | Additive |
|---|---|---:|---:|
| `no_switch` TRUE | any | any | 374 of 374 |
| switch allowed | whole period | whole period | 837 of 837 |
| switch allowed | whole period | fraction | 715 of 1,002 |
| switch allowed | fraction | whole period | 410 of 456 |
| switch allowed | fraction | fraction | 161 of 286 |

So the behaviour belongs to the switching path, and appears only when one of the pieces starts at a
fraction. A schedule of whole periods adds up.

## Errors

Microsoft documents no error values for `VDB`. It says only that "All arguments except no_switch
must be positive numbers", which is contradicted in the same place by the statement that `salvage`
can be 0, and by readings: `cost`, `salvage`, `start_period` and `factor` can each be 0.

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | `cost < 0` | recorded readings, build 20430 |
| `#NUM!` | `salvage < 0` | recorded readings, build 20430 |
| `#NUM!` | `end_period < start_period` | recorded readings, build 20430 |
| `#NUM!` | `end_period > life` | recorded readings, build 20430 |
| `#NUM!` | `factor < 0` | one recorded reading, `factor` −1, build 20430 |
| `#VALUE!` | Text in `factor` or `no_switch` that the slot does not accept, as above | recorded readings and the notes below |
| propagated | `#N/A` in `factor` or `no_switch` surfaces as `#N/A` | recorded readings |
| `#NUM!` | `life ≤ 0`, `start_period < 0` | reference engine only; see below |

The reference engine at commit `473efa3` differs from those readings in three places. It rejects
`factor = 0` as `#NUM!`, where Excel returns the straight-line amount. It does not reject
`end_period > life`, and stops at the end of the life. And it prorates a span by simple overlap,
which makes it additive. The reading tables above are the record.

## Relationships

- **[DDB](FUNC.DDB.md)** is the single-period declining charge with no switch. The DDB page proposes
  `VDB(c, s, l, p−1, p, f, TRUE)` as an identity with `DDB(c, s, l, p, f)`, and the reference engine
  routes both through one kernel. That identity has not been tested against Excel for this page.
- **`SLN`** is the straight line. `VDB` with `factor` 0 and the switch allowed gives the `SLN` amount
  times the length of the span, as above.
- **[DB](FUNC.DB.md)** is declining balance at a derived, rounded rate, with no `factor` and no
  switch, and a partial first year set in months.
- **`SYD`** is sum-of-years digits, the other accelerated method in the family.
- **[AMORLINC](FUNC.AMORLINC.md)** and **[AMORDEGRC](FUNC.AMORDEGRC.md)** are the French accounting
  pair, which prorate on a different basis.
- **Confused with**: tax depreciation tables, which fix their own conventions for the year of
  purchase. `VDB` with a fractional span can approximate one, and is not the same thing.

## Numerical notes

The notes below cover how the arguments are read. They say nothing about the arithmetic of the
result, so this section gives the shape of that arithmetic and the last-unit questions remain open.

**The schedule is a recurrence.** Period `k` needs the book value after `k − 1` charges, so the cost
grows with the period and each earlier rounding is inherited. In exact arithmetic the declining
part has a closed form, `cost × (1 − factor/life)^k`, but it is not the same floating-point
number as the recurrence, and it does not know about the limit at `salvage` or the switch.

**The comparison is a branch.** At the crossover the two charges are close, and which one is used is
decided by a floating-point comparison. The reference engine keeps the declining charge when the two
are equal. The recorded research concludes the same, that the switch needs the straight line to be
strictly larger.

**Fractions are where the formula stops being obvious.** Microsoft's text gives no rule for a
fractional span. The examples show proration inside a period. The non-additive table above shows
that this is not the whole story on the switching path: a fractional start also changes the
schedule that follows. The reference engine's overlap proration is additive on every path, so it
cannot reproduce the table.

**Tiny values.** With `factor` 0 and very small costs, recorded answers depart from the straight-line
amount, and some are `0` where that amount would be a subnormal number. This is consistent with
intermediate results being published as zero when they fall below the normal range, but the
arithmetic is not established here.

**No transcendental functions.** Unlike [DB](FUNC.DB.md), `VDB` needs only addition, subtraction,
multiplication, division and comparison. Any last-unit difference is therefore about the order and
precision of those operations.

## What has not been checked

No Handbook vector suite exists for `VDB`; `vectors/` publishes nothing at this revision, so no
suite-scoped claim exists for it. No evidence record lists this surface as a subject.

Unlike many pages, this one has a large body of readings behind it: the Excel versions section of
this page lists them. Most of the facts above were read from them for this page. What is still not
known after those readings and the notes below:

1. **A published route for the answer.** No note gives Excel's sequence of operations for `VDB`, so
   this page does not say how to reproduce its last bits. Retained research on the switched schedule
   exists and has not been reviewed into a note.
2. **The rule for a fractional `start_period`.** The non-additive table is measured. The explanation
   of it, that the fraction applies first, is the research's conclusion, stated here without a
   formula.
3. **`life ≤ 0` and `start_period < 0`.** Neither appears in the 165,734 distinct inputs surveyed
   for this page. The table under *Errors* gives the reference engine's answer for them, and Excel's
   is unread here. Negative `factor` rests on one reading.
4. **Very large period numbers.** One capture returned answers up to a `start_period` of `2^32 − 2`.
   At `2^32 − 1`, Excel stopped making progress while the formula was being entered, and the run was
   cancelled by hand after more than 120 CPU seconds. Neither a number nor an error can be inferred
   for that row or the 26 queued after it.
5. **`no_switch` given as a number other than 0 or 1.** Every surveyed numeric reading has `no_switch`
   0 or 1.
6. **`DDB` against `VDB` over the same period**, as noted under *Relationships*.
7. **Other environments.** No input has been read in two environments, so no difference is known and
   none is ruled out. The captures do not record the CPU. Nothing has been read on a Mac, in Excel
   for the web, on 32-bit Excel or on an Intel CPU.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `VDB` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-VDB-001: An empty factor slot in VDB is 0, not the default 2

*Documentation differs from behaviour; measured; recorded 2026-11-04.*

VDB defaults factor to 2 only when the argument is left off the end of the call. An empty slot,
as in VDB(c,s,l,a,b,,FALSE), is read as 0, and the result is the straight-line amount: 90 here,
where factor 2 gives 160. Text factors other than numbers give #VALUE!, including "TRUE" and
"FALSE", while "2" is read as 2.

Documentation: If factor is omitted, it is assumed to be 2 (the double-declining balance
method).
(https://support.microsoft.com/en-us/office/vdb-function-dde4e207-f3fa-488d-91d2-66d55e861d73)

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=VDB(A100,A101,A102,A103,A104,,FALSE)` | cost `1000` (`0x408f400000000000`); salvage `100` (`0x4059000000000000`); life `10` (`0x4024000000000000`); start_period `1` (`0x3ff0000000000000`); end_period `2` (`0x4000000000000000`) (cells) | `90` (`0x4056800000000000`) | Excel build not recorded in this run; W111 bulk captures of the same date (2026-09-29) record Excel 16.0 build 20430, Windows 64-bit, workbook compatibility 2 |
| `=VDB(A100,A101,A102,A103,A104,"2",FALSE)` | cost `1000` (`0x408f400000000000`); salvage `100` (`0x4059000000000000`); life `10` (`0x4024000000000000`); start_period `1` (`0x3ff0000000000000`); end_period `2` (`0x4000000000000000`) (cells) | `160` (`0x4064000000000000`) | Excel build not recorded in this run; W111 bulk captures of the same date (2026-09-29) record Excel 16.0 build 20430, Windows 64-bit, workbook compatibility 2 |
| `=VDB(A100,A101,A102,A103,A104,"TRUE",FALSE)` | cost `1000` (`0x408f400000000000`); salvage `100` (`0x4059000000000000`); life `10` (`0x4024000000000000`); start_period `1` (`0x3ff0000000000000`); end_period `2` (`0x4000000000000000`) (cells) | `#VALUE!` | Excel build not recorded in this run; W111 bulk captures of the same date (2026-09-29) record Excel 16.0 build 20430, Windows 64-bit, workbook compatibility 2 |

Basis (measured): Ten recorded option readings (cases 00022 to 00031 of the run). A rule
reading the required arguments, then factor, then no_switch reproduces all 142 retained padding
calls.

Evidence:

- DnaCalc `apps/dna-onecalc/tests/acceptance.rs`, recorded_vdb_explicit_missing_factor_is_zero at `cfdc059510` — recorded readings pinned
- OxFunc-Evidence `runs/w111-depreciation-options-20260929/outcomes/excel.jsonl.gz`, w111dep-options-00022-vdb-optional-5-22 at `e26b7bddb8` (readings)
- OxFunc-Evidence `runs/w111-depreciation-options-20260929/outcomes/excel.jsonl.gz`, w111dep-options-00025-vdb-optional-5-25 at `e26b7bddb8` (readings)
- OxFunc-Evidence `runs/w111-depreciation-options-20260929/outcomes/excel.jsonl.gz`, w111dep-options-00026-vdb-optional-5-26 at `e26b7bddb8` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| declining balance | A constant fraction of the falling book value charged each period |
| book value | Cost less depreciation already taken |
| factor | The multiplier on the straight-line rate; `factor/life` is the declining rate |
| switch | Moving to the straight-line charge when it is larger than the declining one |
| span | The interval from `start_period` to `end_period`, measured in elapsed periods |
| proration | Taking a fraction of a period's charge for a fraction of the period |
| non-additive | The whole span differs from the sum of its parts |

## Sources

- Microsoft Support, "VDB function" —
  <https://support.microsoft.com/en-us/office/vdb-function-dde4e207-f3fa-488d-91d2-66d55e861d73>
  (syntax, argument meanings, the positive-number sentence and the six examples). Read through a
  fetch that returned a summary with quoted text, not the raw page.
- Microsoft Learn, "WorksheetFunction.Vdb method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.vdb> (the parameter table,
  the switch remarks and the positive-number remark).
- The recorded `VDB` readings listed in this page's Excel versions section, Excel 16.0 build 20430:
  the 165,734 distinct numeric inputs surveyed for the *Result and edge cases* and *Errors* tables
  and the additivity table, the option run the notes below cite, and the three non-additive
  readings. The research write-up retained with those readings is the source of the fractional-start
  and strict-comparison conclusions and of the large-period account.
- Handbook, [Function notes](../notes/NOTES.md).
- Handbook, [DDB](FUNC.DDB.md) and [DB](FUNC.DB.md) — the sibling methods.
- Handbook chapters [01 the value universe](../model/01-value-universe.md),
  [02 coercion and lifting](../model/02-coercion-and-lifting.md) and
  [06 claim language](../model/06-claim-language.md).
- OxFunc `crates/oxfunc_core/src/functions/depreciation_family.rs` at commit `473efa3` — the
  declining-interval kernel, its overlap proration, its validation rules and its optional-argument
  defaults, read as implementation facts about that engine.
- Handbook projections `data/functions/FUNC.VDB.json` and `data/presence/FUNC.VDB.json` (arity,
  classification axes, implementing module and sibling set).
