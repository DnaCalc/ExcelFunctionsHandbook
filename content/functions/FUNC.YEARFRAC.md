---
schema: efh.function-page/v1
function_id: FUNC.YEARFRAC
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: YEARFRAC function"
    locator: "https://support.microsoft.com/en-us/office/yearfrac-function-3844141e-c76d-4143-82b6-208454ddc6a8"
    role: "documented basis table, the integer-truncation remark, the error conditions, and the last-day-of-February caveat"
  - work: "Microsoft 365 support: DAYS360 function"
    locator: "https://support.microsoft.com/en-us/office/days360-function-b9a509fd-49ef-407e-94df-0cbda5718c2a"
    role: "the verbatim US (NASD) and European 30/360 adjustment rules that basis 0 and basis 4 rest on"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
family: discount_bill_yearfrac_family
role_in_family: "The day-count engine: converts a date interval into a fraction of a year on a
  selectable basis, and is the shared substrate the discount and bill-pricing functions measure time
  with."
---

## What it computes

`YEARFRAC` converts the interval between two dates into a **fraction of a year**, under a
selectable day-count convention. It is the time axis of fixed-income arithmetic: every accrual,
every discount factor, every bill price ultimately asks "how much of a year is this", and this
function is where Excel answers.

Structurally it is a quotient:

> `YEARFRAC(start, end, basis)` = (days between, counted by `basis`) / (days in a year, per
> `basis`)

Both the numerator and the denominator depend on the basis, and — this is the part that surprises
people — for one basis the denominator is not a constant.

| `basis` | Convention | Numerator | Denominator |
|---|---|---|---|
| 0 or omitted | US (NASD) 30/360 | 30/360-adjusted day count | 360 |
| 1 | Actual/actual | actual calendar days | actual — depends on the period |
| 2 | Actual/360 | actual calendar days | 360 |
| 3 | Actual/365 | actual calendar days | 365 |
| 4 | European 30/360 | 30/360-adjusted day count | 360 |

The two 30/360 bases are the conventions documented verbatim on the
[DAYS360](FUNC.DAYS360.md) page — basis 0 is the US (NASD) method with its asymmetric,
start-date-dependent end-date rule, and basis 4 is the symmetric European method that adjusts only
the 31st. `YEARFRAC(a, b, 0)` and `DAYS360(a, b, FALSE)/360` are the same quantity on the documented
reading of both pages.

**Basis 1 is the hard one.** "Actual/actual" does not name a single algorithm. The denominator must
be some notion of "the length of the year the interval sits in", and when an interval spans several
years, or a leap day, or starts and ends in years of different lengths, there are several defensible
constructions — averaging the lengths of the spanned years, using the length of the year containing
the start, using 365 or 366 according to whether a leap day falls inside the period, and others.
Excel's exact rule is **not stated on the documentation page**, and this Handbook has not
established it. That gap is the single most important open question about this function, because
basis 1 is the one regulated instruments most often specify.

## Arguments

`YEARFRAC(start_date, end_date, [basis])` — two required arguments and one optional.

**start_date** — "a date that represents the start date".

**end_date** — "a date that represents the end date".

**basis** — optional; the day-count convention, per the table above. Omitted means 0, the US
(NASD) 30/360 convention. Microsoft states a general conversion rule for this function that the
rest of the category does not state so plainly: **"All arguments are truncated to integers."** So
fractional serials contribute no fractional days, and a fractional `basis` truncates.

The misunderstood argument is `basis`, and specifically its default. A workbook written against an
Actual/365 instrument that omits `basis` silently gets 30/360 answers, which are close enough to
look right and wrong enough to matter over a portfolio.

## Result and edge cases

Returns a `Number`: a fraction of a year, which may exceed 1 for intervals longer than a year and
is negative when `end_date` precedes `start_date`.

- **Microsoft documents a defect in this function.** Verbatim: `YEARFRAC` "may return an incorrect
  result when using the US (NASD) 30/360 basis, and the start_date is the last day in February".
  That is the vendor stating that basis 0 is wrong on a named input class. The Handbook records it
  as documented; it has not been characterized here — nobody has determined which February end
  dates misbehave, by how much, or whether the same inputs affect [DAYS360](FUNC.DAYS360.md), which
  implements the same adjustment rules.
- **Basis 1 across a leap day** is where the undocumented denominator rule becomes visible. An
  interval of exactly one calendar year should give 1 under any sensible actual/actual construction;
  an interval spanning 29 February is where constructions diverge.
- **Equal dates** give 0.
- **Reversed intervals** give a negative fraction; unlike [DATEDIF](FUNC.DATEDIF.md), no error is
  documented for them.
- **Floating point.** The result is a quotient of small integers by 360, 365, 366 or an
  actual/actual denominator. Division by 360 and 365 is inexact in binary, so the last bits of the
  result depend on the order in which the quotient is formed — which makes `YEARFRAC` one of the few
  functions in this category with genuine last-bit content, and one where an
  `excel-bitexact` implementation has something real to match.

## Errors

As documented on the Microsoft page:

- `#VALUE!` when `start_date` or `end_date` is not a valid date.
- `#NUM!` when `basis` is less than 0 or greater than 4.

Note that the invalid-date condition is `#VALUE!` here, where several siblings in this category
document `#NUM!` for an out-of-range date. The Handbook records the family's inconsistent
documented error values rather than harmonizing them; see
[claim language](../model/06-claim-language.md), rule 5.

Ordinary coercion failures and error propagation follow the engine rules in
[coercion and lifting](../model/02-coercion-and-lifting.md).

## Relationships

- **[DAYS360](FUNC.DAYS360.md)** — the same two 30/360 conventions, expressed as a day count rather
  than a year fraction. `YEARFRAC(a,b,0)` should be `DAYS360(a,b,FALSE)/360` and `YEARFRAC(a,b,4)`
  should be `DAYS360(a,b,TRUE)/360`. Testing those two identities is the cheapest way to probe the
  documented last-day-of-February defect, since it localizes the disagreement to one of the two
  functions.
- **[DATEDIF](FUNC.DATEDIF.md) with `"Y"`** — whole elapsed years rather than a fraction. Frequently
  substituted for `YEARFRAC` in age calculations, with different answers.
- **[DAYS](FUNC.DAYS.md)** — the raw calendar interval that bases 1, 2 and 3 use as their numerator.
- **The family it lives in.** The reference engine implements `YEARFRAC` in the same module as
  `DISC`, `INTRATE`, `PRICEDISC`, `RECEIVED`, `TBILLEQ`, `TBILLPRICE` and `TBILLYIELD` — the
  discount and bill-pricing functions that consume day counts. That grouping is a real statement
  about the code: those functions' accuracy is downstream of this one's. An error in `YEARFRAC`'s
  basis handling is an error in all of them.
- **[EOMONTH](FUNC.EOMONTH.md)** — a common source of the exact end-of-February dates Microsoft's
  caveat names.

## Notes for implementers

1. **Basis 1 needs a written-down definition before it needs code.** Pick a construction, state it
   precisely (which years, how leap days are counted, what happens for multi-year intervals), and
   do not claim compatibility until it has been checked. This is the one place in this assignment
   where "we implemented what seemed right" is guaranteed to diverge from something.
2. **Share the 30/360 adjustment code with `DAYS360`.** Bases 0 and 4 are exactly that function's
   two methods; two implementations of one rule will eventually disagree, and Microsoft's own
   documented caveat suggests Excel may already contain such a divergence.
3. **Truncate every argument to an integer first**, per the documented rule, including `basis`.
4. **Form the quotient in a fixed, documented order.** The numerator is a small integer and the
   denominator is 360, 365, 366 or a computed value; different groupings give different last bits.
   For a `portable-reproducible` flavour this must be pinned explicitly.
5. **Validate `basis` against 0…4 before anything else**, since it is the one documented `#NUM!`.

## What has not been checked

No Handbook vector suite exists for `YEARFRAC`, and no Excel-comparison evidence is recorded. The
battery on this page is the reference engine answering its own probes; no Excel was involved. This
is the function in this assignment with the most consequential unknowns, and the probes are
correspondingly specific:

- **Basis 1 over intervals spanning 29 February**, and over multi-year intervals with differing
  year lengths. Nothing else will reveal which actual/actual construction Excel uses, and nothing
  in the documentation says.
- **Basis 0 with `start_date` on the last day of February** — the input class Microsoft itself flags
  as possibly incorrect. Characterizing it would turn a vendor caveat into a described behaviour,
  which is exactly the kind of work this Handbook exists to publish.
- **`YEARFRAC(a,b,0)` against `DAYS360(a,b,FALSE)/360`**, and the basis-4 equivalent, across every
  month-end pair in a two-year window. Localizes any 30/360 disagreement to one function.
- **Bases 2 and 3 over long intervals** — the easy cases, useful mainly as a floating-point
  quotient-order probe, since the answer is a simple ratio whose last bits depend on the division
  order.
- **`basis` values −1, 5, 4.9, and text** — confirms the documented `#NUM!` boundary and the
  documented truncation.
- **Reversed intervals**, where no error is documented.

Until at least the basis-1 question is settled, no implementation of this function can honestly
claim to compute what Excel computes, and this page makes no such claim for any implementation.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| day-count basis | The convention determining how days and years are counted (`basis` 0–4) |
| 30/360 | Every month treated as 30 days, every year as 360 |
| actual/actual | Actual calendar days over an actual year length; the construction is unstated in the documentation |
| the February caveat | Microsoft's documented statement that basis 0 may be incorrect when `start_date` is the last day of February |

## Sources

- Microsoft 365 support, **YEARFRAC function** —
  <https://support.microsoft.com/en-us/office/yearfrac-function-3844141e-c76d-4143-82b6-208454ddc6a8>.
  Source of the basis table, the "all arguments are truncated to integers" remark, both error
  conditions, and the last-day-of-February caveat quoted above.
- Microsoft 365 support, **DAYS360 function** —
  <https://support.microsoft.com/en-us/office/days360-function-b9a509fd-49ef-407e-94df-0cbda5718c2a>.
  Source of the verbatim US (NASD) and European 30/360 adjustment rules underlying bases 0 and 4.
- [FUNC.DAYS360](FUNC.DAYS360.md) and [FUNC.DATE](FUNC.DATE.md).
- Handbook chapters [02 coercion and lifting](../model/02-coercion-and-lifting.md) and
  [06 claim language](../model/06-claim-language.md).
- `data/functions/FUNC.YEARFRAC.json`, `data/presence/FUNC.YEARFRAC.json` (the shared
  discount/bill-pricing module), `data/battery/FUNC.YEARFRAC.json`.
