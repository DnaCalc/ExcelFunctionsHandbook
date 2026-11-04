---
schema: efh.function-page/v1
function_id: FUNC.NETWORKDAYS.INTL
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: NETWORKDAYS.INTL function"
    locator: "https://support.microsoft.com/en-us/office/networkdays-intl-function-a9b26239-4f20-46a1-9ab8-4e925bfd5e28"
    role: "documented weekend number table, weekend string rules, and both error conditions"
  - work: "Microsoft 365 support: WORKDAY.INTL function"
    locator: "https://support.microsoft.com/en-us/office/workday-intl-function-a378391c-9ba7-4678-8a39-39611a9bf81d"
    role: "the sibling sharing the same weekend argument, whose page states the all-ones prohibition"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
  - Notes
  - Page vocabulary
  - Sources
family: workday_networkdays_family
role_in_family: "The generalized working-day count: same interval question as NETWORKDAYS, with the
  weekend definition promoted to an argument."
---

## What it computes

`NETWORKDAYS.INTL` counts the whole working days in the interval from `start_date` to `end_date`,
excluding days designated as weekend and any dates listed in `holidays`. It differs from
[NETWORKDAYS](FUNC.NETWORKDAYS.md) in exactly one respect: **the weekend is an argument**, not a
constant, and it can name any subset of the week.

That single change is what makes the function usable outside the Saturday/Sunday world — Friday–
Saturday weekends, single-day weekends, six-day working weeks, and (via the string form)
arbitrary patterns such as "Wednesday and Sunday off".

The count itself is the same filtered enumeration: take the days of the interval, drop the ones
whose weekday is designated weekend, drop the ones in `holidays`, return the remainder.

## Arguments

`NETWORKDAYS.INTL(start_date, end_date, [weekend], [holidays])` — two required and two optional.

**start_date**, **end_date** — the interval endpoints, as date serials.

**weekend** — optional. Two entirely different value shapes are accepted, and they are the reason
this function needs a page.

*As a number*, the documented table is:

| Number | Weekend days |
|---|---|
| 1 or omitted | Saturday, Sunday |
| 2 | Sunday, Monday |
| 3 | Monday, Tuesday |
| 4 | Tuesday, Wednesday |
| 5 | Wednesday, Thursday |
| 6 | Thursday, Friday |
| 7 | Friday, Saturday |
| 11 | Sunday only |
| 12 | Monday only |
| 13 | Tuesday only |
| 14 | Wednesday only |
| 15 | Thursday only |
| 16 | Friday only |
| 17 | Saturday only |

The two blocks have different generators, which is the trap. Codes **1–7** name *consecutive
pairs*, and the pair for code `n` begins on the day `n` positions after Friday — code 1 is
Saturday+Sunday, code 2 is Sunday+Monday, and so on around the week. Codes **11–17** name *single
days*, running Sunday (11) through Saturday (17). There is no code for a non-consecutive pair; for
that you need the string form. Note also that the single-day block starts at Sunday while the
pair block starts at Saturday, so neither block's numbering aligns with the other's, nor with
[WEEKDAY](FUNC.WEEKDAY.md)'s `return_type` numbering. Nothing about these tables is memorable, and
guessing is unwise.

*As a string*, Microsoft documents: "Weekend string values are seven characters long and each
character in the string represents a day of the week, starting with Monday." Each character is
`"1"` for a non-workday or `"0"` for a workday, and "only the characters 1 and 0 are permitted in
the string". So `"0000011"` is the ordinary Saturday/Sunday weekend, and `"0100010"` is Tuesday and
Saturday off.

Three details about the string form:

- **It starts with Monday**, which is not the convention any of this function's numeric tables use
  and not the convention `WEEKDAY`'s default `return_type` uses.
- **`"1111111"` is invalid** — the [WORKDAY.INTL](FUNC.WORKDAY.INTL.md) page states the rule
  explicitly ("at least one workday required"), and the same argument is shared by both functions.
  Whether `NETWORKDAYS.INTL` rejects it identically is a natural check that has not been run here;
  a zero-workday week would make the count trivially 0 rather than undefined, so the two functions
  could defensibly differ.
- **It is text, not a number.** A `weekend` of `0000011` written without quotes is the number 11,
  which is a valid *numeric* code meaning "Sunday only" — a silent, plausible-looking wrong answer
  rather than an error. This is the sharpest foot-gun in the function.

**holidays** — optional range or array of dates to exclude, as on
[NETWORKDAYS](FUNC.NETWORKDAYS.md).

## Result and edge cases

Returns a `Number`: a count of working days.

- **Inclusivity of the endpoints** is not stated on the Microsoft page, exactly as with
  `NETWORKDAYS`. The expected behaviour is that both endpoints count; unverified here.
- **Reversed intervals** — likewise undocumented and unverified.
- **A weekend covering six days** leaves one working day per week and is legal.
- **Holidays landing on weekend days** should contribute nothing, since the operation is exclusion;
  undocumented.
- **Blank cells in a `holidays` range** raise the Empty-versus-Missing question from
  [the value universe](../model/01-value-universe.md), and serial 0 is inside the representable
  range — so a blank interpreted as a number is a date. Undocumented and a real hazard.

## Errors

As documented on the Microsoft page:

- `#NUM!` when `start_date` or `end_date` is out of the valid date range.
- `#VALUE!` when "a weekend string is of invalid length or contains invalid characters".

Compare [NETWORKDAYS](FUNC.NETWORKDAYS.md), whose page documents `#VALUE!` "if any argument is not a
valid date" — a different error value for what looks like the same failure. The Handbook records
the discrepancy rather than resolving it by assumption; see
[claim language](../model/06-claim-language.md), rule 5.

The documented `#VALUE!` condition names only *string* invalidity. What an out-of-table **numeric**
weekend code does — 8, 10, 18, 0 — is not stated on that page and is not asserted here.

## Relationships

- **[NETWORKDAYS](FUNC.NETWORKDAYS.md)** — the un-generalized original. `NETWORKDAYS(a,b,h)` should
  equal `NETWORKDAYS.INTL(a,b,1,h)`; that equivalence is unverified here and is the family's
  cleanest cross-check.
- **[WORKDAY.INTL](FUNC.WORKDAY.INTL.md)** — the inverse operation with the identical `weekend`
  argument. The two should agree about which days are weekends by construction, and their
  documented error tables should therefore match; they are worded differently.
- **[WORKDAY](FUNC.WORKDAY.md)** — the un-generalized inverse.
- **[WEEKDAY](FUNC.WEEKDAY.md)** — the underlying day-of-week notion, with yet another numbering
  convention. This family contains four mutually non-aligned weekday numberings: `WEEKDAY`'s
  legacy codes, `WEEKDAY`'s 11–17 block, `.INTL`'s 1–7 pairs, and `.INTL`'s Monday-first string.
  That is a genuine design fact about the surface and the source of most `.INTL` bugs.

## Notes for implementers

1. **Normalize `weekend` to a seven-bit mask first**, from either the numeric code or the string,
   and reject invalid inputs at that point. Every downstream step then sees one representation.
2. **Get the Monday-first string convention right**, and write a test that fails if the mask is
   reversed — a reversed mask is symmetric for the common `"0000011"`/`"1100000"` confusion only in
   the sense that both look plausible.
3. **Use whole-week arithmetic, not enumeration.** With a mask, the count over `n` whole weeks is
   `n × popcount(¬mask)`, and only the partial week at each end needs a loop of at most six
   iterations. Enumeration over a century-scale interval is reachable and slow.
4. **Decide the all-weekend (`"1111111"`) policy deliberately** and keep it consistent with
   `WORKDAY.INTL`, where the documentation is explicit that the string is invalid.
5. **Do not accept a numeric `weekend` that happens to look like a mask.** `11` is a table code, not
   a two-day pattern.

## What has not been checked

No Handbook vector suite exists for `NETWORKDAYS.INTL`, and no Excel-comparison evidence is
recorded. The battery on this page is the reference engine answering its own probes; no Excel was
involved. Probes worth running first:

- **All fourteen numeric weekend codes on one known week** — fourteen rows that pin the entire
  numeric table, which is the part of this function nobody remembers correctly.
- **The seven single-bit strings** `"1000000"` … `"0000001"` on the same week — pins the Monday-first
  convention definitively.
- **`"1111111"`** on both `NETWORKDAYS.INTL` and `WORKDAY.INTL` — settles whether the documented
  prohibition applies to both.
- **Out-of-table numeric codes** — 0, 8, 10, 18 — where the documentation is silent.
- **`NETWORKDAYS(a,b,h)` against `NETWORKDAYS.INTL(a,b,1,h)`** over a few hundred pairs.
- **Endpoint inclusivity and reversed intervals**, as on the `NETWORKDAYS` page.
- **A `holidays` range with blanks, duplicates and weekend dates.**

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `NETWORKDAYS.INTL` that an implementer matching Excel, or a reader
explaining an unexpected Excel result, needs to know. Each note gives exact binary64 inputs,
Excel's answer and where it was recorded; "measured" means the statement is read off recorded
Excel answers, "inferred" marks what goes beyond them. Where older text on this page says a
behaviour was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

### FN-NETWORKDAYS.INTL-001: NETWORKDAYS.INTL checks weekend before the dates, WORKDAY.INTL after them

*Quirk; measured; recorded 2026-11-04.* Applies to NETWORKDAYS.INTL, WORKDAY.INTL.

The two functions read their arguments in different orders. NETWORKDAYS.INTL decodes weekend
first: with #REF! in the cell for start_date and weekend 0, the answer is #NUM!, from the
invalid weekend. WORKDAY.INTL reads start_date first and gives #REF! for the same arguments. An
error held in the weekend cell is also treated differently: with start 2 and second argument 5,
NETWORKDAYS.INTL gives #VALUE! and WORKDAY.INTL gives #NUM!. Neither returns the error itself.
A logical weekend is read as a number: NETWORKDAYS.INTL(2,5,TRUE) is 4, as with weekend 1, and
FALSE is #NUM!, as with weekend 0. WORKDAY.INTL does the same.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `NETWORKDAYS.INTL(A1, 5, 0)` | start_date `#REF!`; end_date `5` (`0x4014000000000000`); weekend `0` (`0x0000000000000000`) (cells) | `#NUM!` | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `WORKDAY.INTL(A1, 5, 0)` | start_date `#REF!`; days `5` (`0x4014000000000000`); weekend `0` (`0x0000000000000000`) (cells) | `#REF!` | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `NETWORKDAYS.INTL(2, 5, A3)` | start_date `2` (`0x4000000000000000`); end_date `5` (`0x4014000000000000`); weekend `#REF!` (cells) | `#VALUE!` | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `WORKDAY.INTL(2, 5, A3)` | start_date `2` (`0x4000000000000000`); days `5` (`0x4014000000000000`); weekend `#REF!` (cells) | `#NUM!` | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `=NETWORKDAYS.INTL(2,5,TRUE)` | as typed | `4` (`0x4010000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=NETWORKDAYS.INTL(2,5,FALSE)` | as typed | `#NUM!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=WORKDAY.INTL(2,2,TRUE)` | as typed | `4` (`0x4010000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=WORKDAY.INTL(2,2,FALSE)` | as typed | `#NUM!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): 19 NETWORKDAYS.INTL and 20 WORKDAY.INTL readings with errors held in cells,
and the logical weekend readings of the argument sheets.

Evidence:

- DnaCalc `apps/dna-onecalc/tests/acceptance.rs`, delivered_scalar_readings_follow_the_application_route at `287c5b9a3b` — recorded answers pinned
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/NETWORKDAYS.INTL.reading.json` at `2d0eddcac8` — 19 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/WORKDAY.INTL.reading.json` at `2d0eddcac8` — 20 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/NETWORKDAYS.INTL.reading.json`, fn7-NETWORKDAYS.INTL-weekend-first-order-2 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/WORKDAY.INTL.reading.json`, fn7-WORKDAY.INTL-weekend-first-order-2 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/NETWORKDAYS.INTL.reading.json`, fn7-NETWORKDAYS.INTL-admission-order-6 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/WORKDAY.INTL.reading.json`, fn7-WORKDAY.INTL-admission-order-6 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa043, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa044, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa065, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa066, row 20, column 10 at `2d0eddcac8` (readings)

### FN-WORKDAY-001: An empty cell as a first date is serial 0, but a logical in that cell is #VALUE!

*Quirk; measured; recorded 2026-11-04.* Applies to NETWORKDAYS, NETWORKDAYS.INTL, TBILLEQ, TBILLPRICE, TBILLYIELD, WORKDAY, WORKDAY.INTL, YEARFRAC.

A reference to an empty cell as the first date of these functions reads as serial 0 and gives a
normal answer: NETWORKDAYS(B1,5) is 4 and WORKDAY(B1,2) is 3. YEARFRAC(B1,5,0) is 5/360. TRUE
in the same cell gives #VALUE!, as a direct TRUE does, so a blank is not treated as the logical
FALSE either.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=NETWORKDAYS(B1,5)` | B1 `(empty cell)` (cells) | `4` (`0x4010000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=NETWORKDAYS(B1,5)` | B1 `TRUE` (cells) | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=WORKDAY(B1,2)` | B1 `(empty cell)` (cells) | `3` (`0x4008000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=WORKDAY(B1,2)` | B1 `TRUE` (cells) | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=NETWORKDAYS.INTL(B1,5,1)` | B1 `(empty cell)` (cells) | `4` (`0x4010000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=WORKDAY.INTL(B1,2,1)` | B1 `(empty cell)` (cells) | `3` (`0x4008000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=YEARFRAC(B1,5,0)` | B1 `(empty cell)` (cells) | `0.013888888888888888` (`0x3f8c71c71c71c71c`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=TBILLEQ(B1,5,0.1)` | B1 `(empty cell)` (cells) | `0.10152990264255911` (`0x3fb9fddd1b6a757f`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): 8 recorded readings with an empty B1, one per function, each paired with a
reading with TRUE in B1.

- Open: An argument left empty in the call was not read here for YEARFRAC and the T-bill functions.

Evidence:

- DnaCalc `apps/dna-onecalc/tests/acceptance.rs`, delivered_fn4_blank_date_references_read_serial_zero at `c4c5dde7e2` — recorded answers pinned
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheets qa031 and qa032, and the like pairs of each function at `2d0eddcac8` — recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa032, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa031, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa054, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa053, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa040, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa062, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa072, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa082, row 20, column 10 at `2d0eddcac8` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| weekend code | The numeric argument selecting a documented weekend pattern (1–7, 11–17) |
| weekend string | The seven-character Monday-first `0`/`1` pattern |
| weekend mask | An implementation's normalized seven-bit representation of either form |
| the `.INTL` generalization | Promoting the weekend definition from a constant to an argument |

## Sources

- Microsoft 365 support, **NETWORKDAYS.INTL function** —
  <https://support.microsoft.com/en-us/office/networkdays-intl-function-a9b26239-4f20-46a1-9ab8-4e925bfd5e28>.
  Source of the weekend number table, the weekend string rules quoted above, and both documented
  error conditions.
- Microsoft 365 support, **WORKDAY.INTL function** —
  <https://support.microsoft.com/en-us/office/workday-intl-function-a378391c-9ba7-4678-8a39-39611a9bf81d>.
  Source of the "at least one workday required" statement about `"1111111"`.
- Microsoft 365 support, **NETWORKDAYS function** —
  <https://support.microsoft.com/en-us/office/networkdays-function-48e717bf-a7a3-495f-969e-5005e3eb18e7>.
  Cited for the contrasting error condition.
- Handbook call-model chapters [01 value universe](../model/01-value-universe.md) and
  [06 claim language](../model/06-claim-language.md).
- `data/functions/FUNC.NETWORKDAYS.INTL.json`, `data/presence/FUNC.NETWORKDAYS.INTL.json`,
  `data/battery/FUNC.NETWORKDAYS.INTL.json`.
