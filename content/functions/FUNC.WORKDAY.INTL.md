---
schema: efh.function-page/v1
function_id: FUNC.WORKDAY.INTL
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: WORKDAY.INTL function"
    locator: "https://support.microsoft.com/en-us/office/workday-intl-function-a378391c-9ba7-4678-8a39-39611a9bf81d"
    role: "documented weekend table, weekend string rules including the all-ones prohibition, and the full error table"
  - work: "Microsoft 365 support: NETWORKDAYS.INTL function"
    locator: "https://support.microsoft.com/en-us/office/networkdays-intl-function-a9b26239-4f20-46a1-9ab8-4e925bfd5e28"
    role: "the sibling sharing the identical weekend argument"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
family: workday_networkdays_family
role_in_family: "The generalized working-day offset: same operation as WORKDAY, with the weekend
  definition promoted to an argument."
---

## What it computes

`WORKDAY.INTL` returns the date lying a given number of working days before or after a start date,
where **which days are weekend is an argument** rather than a fixed Saturday and Sunday. It is to
[WORKDAY](FUNC.WORKDAY.md) what [NETWORKDAYS.INTL](FUNC.NETWORKDAYS.INTL.md) is to
[NETWORKDAYS](FUNC.NETWORKDAYS.md), and it shares the `.INTL` weekend argument with the former
exactly.

The operation is unchanged from `WORKDAY`: step away from `start_date` in the direction of `days`,
consuming one unit of the count for each working day stepped onto, and return where you land. Only
the definition of "working day" is now parameterized.

## Arguments

`WORKDAY.INTL(start_date, days, [weekend], [holidays])` — two required and two optional.

**start_date** — the date to offset from.

**days** — the number of working days to move; positive forward, negative backward. `WORKDAY`'s
page documents that a non-integer `days` is truncated, and the same rule is expected here.

**weekend** — optional; the same two-shaped argument documented for `NETWORKDAYS.INTL`.

*As a number:*

| Number | Weekend days |
|---|---|
| 1 (default) | Saturday, Sunday |
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

Codes 1–7 are consecutive pairs starting one day later each time from Saturday; codes 11–17 are
single days running Sunday through Saturday. The two blocks do not share an origin, and neither
aligns with [WEEKDAY](FUNC.WEEKDAY.md)'s numbering.

*As a string:* seven characters, one per weekday **starting with Monday**, `"1"` for a non-workday
and `"0"` for a workday; only those two characters are permitted. Microsoft's `WORKDAY.INTL` page
adds a rule its sibling's page does not state in the same words: **`"1111111"` is invalid**, because
at least one workday is required. That prohibition is structurally necessary here in a way it is
not for a counting function — a week with no working days makes the offset non-terminating, whereas
a count over such a week is simply zero.

**holidays** — optional range or array of dates to exclude.

The foot-gun is unchanged from the sibling and worth repeating: an unquoted `0000011` is the
*number* 11, a valid weekend code meaning "Sunday only". It produces a plausible wrong answer, not
an error.

## Result and edge cases

Returns a `Number`: a whole date serial.

- **`days = 0`** exposes the counting convention, exactly as on [WORKDAY](FUNC.WORKDAY.md), and the
  page is equally silent about it. Whether a start date falling on a weekend is returned unchanged
  or snapped is the diagnostic case.
- **A six-day weekend** leaves one working day per week, which is legal and makes offsets large in
  calendar terms — a good stress case for any closed-form implementation.
- **Holidays that fall on weekend days** should contribute nothing; undocumented.
- **Consecutive holidays** require the exclusion pass to iterate; see the implementer notes.
- **Blank cells in a `holidays` range** raise the Empty-versus-Missing question from
  [the value universe](../model/01-value-universe.md); serial 0 is a representable date.

## Errors

Documented on the Microsoft page, and this is the fullest error table in the family:

- `#NUM!` when "start_date is out of range for the current date base value".
- `#NUM!` when "any date in holidays is out of range for the current date base value".
- `#NUM!` when "start_date plus day-offset yields an invalid date".
- `#VALUE!` when "a weekend string is of invalid length or contains invalid characters".

Two observations the Handbook records rather than smooths over:

1. **The holidays condition is documented here and not on [WORKDAY](FUNC.WORKDAY.md)'s page**,
   which mentions only "any argument is not a valid date" (`#VALUE!`). Two functions performing the
   same holiday exclusion, documented with different error values for an out-of-range holiday.
2. **Only the string form of `weekend` has a documented invalidity condition.** What an
   out-of-table numeric code does — 0, 8, 10, 18 — is not stated, and is not asserted here.

## Relationships

- **[WORKDAY](FUNC.WORKDAY.md)** — the un-generalized original. `WORKDAY(a,n,h)` should equal
  `WORKDAY.INTL(a,n,1,h)`; unverified here.
- **[NETWORKDAYS.INTL](FUNC.NETWORKDAYS.INTL.md)** — the inverse operation, sharing this exact
  `weekend` argument. Their weekend interpretations must agree or the family is incoherent; that
  they do is assumed and unchecked.
- **[NETWORKDAYS](FUNC.NETWORKDAYS.md)** — the un-generalized inverse.
- **[EDATE](FUNC.EDATE.md)** — month offsetting, the other "move a date by whole units" function,
  with the same documented truncation rule for its offset argument.

## Notes for implementers

1. **Normalize `weekend` to a seven-bit mask before anything else**, from either shape, and reject
   the all-ones mask here even if the counting sibling tolerates it — an offset over a week with no
   working days does not terminate.
2. **Use whole-week arithmetic.** With `w = popcount(¬mask)` working days per week, `days` decomposes
   into `⌊days/w⌋` whole weeks plus a remainder resolved against the start day's position in the
   week. Stepping day by day is correct and unusably slow across the serial range.
3. **Iterate the holiday exclusion to a fixed point.** Skipping a holiday moves the target, which
   may itself be a holiday; a single pass is wrong for consecutive holidays.
4. **Keep the `days = 0` convention identical to `WORKDAY`'s**, whatever it turns out to be.
5. **Do not share the error path with `WORKDAY`** until the documented divergence above is resolved
   by observation.

## What has not been checked

No Handbook vector suite exists for `WORKDAY.INTL`, and no Excel-comparison evidence is recorded.
The battery on this page is the reference engine answering its own probes; no Excel was involved.
Probes worth running first:

- **All fourteen numeric weekend codes with `days = 1` from a known Monday** — fourteen rows that
  pin the numeric table.
- **The seven single-bit strings** on the same start date — pins the Monday-first convention.
- **`"1111111"`** — confirms the documented rejection, and shows whether
  `NETWORKDAYS.INTL` agrees.
- **`days = 0` from each of the seven weekdays** under weekend code 1 — settles the counting
  convention.
- **Out-of-table numeric codes** — 0, 8, 10, 18 — where the documentation is silent.
- **An out-of-range date inside `holidays`** — resolves the documented divergence with `WORKDAY`.
- **Consecutive holidays**, testing the fixed-point requirement.
- **`WORKDAY(a,n,h)` against `WORKDAY.INTL(a,n,1,h)`** across a few hundred cases.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| weekend code | The numeric argument selecting a documented weekend pattern (1–7, 11–17) |
| weekend string | The seven-character Monday-first `0`/`1` pattern; `"1111111"` is documented invalid |
| weekend mask | An implementation's normalized seven-bit representation of either form |
| holiday fixed-point | Re-applying holiday exclusion until the result stops moving |

## Sources

- Microsoft 365 support, **WORKDAY.INTL function** —
  <https://support.microsoft.com/en-us/office/workday-intl-function-a378391c-9ba7-4678-8a39-39611a9bf81d>.
  Source of the weekend number table, the weekend string rules including the "at least one workday
  required" statement, and all four documented error conditions.
- Microsoft 365 support, **NETWORKDAYS.INTL function** —
  <https://support.microsoft.com/en-us/office/networkdays-intl-function-a9b26239-4f20-46a1-9ab8-4e925bfd5e28>.
- Microsoft 365 support, **WORKDAY function** —
  <https://support.microsoft.com/en-us/office/workday-function-f764a5b7-05fc-4494-9486-60d494efbf33>.
  Source of the `days` truncation rule and of the contrasting error conditions.
- Handbook call-model chapter [01 value universe](../model/01-value-universe.md).
- `data/functions/FUNC.WORKDAY.INTL.json`, `data/presence/FUNC.WORKDAY.INTL.json`,
  `data/battery/FUNC.WORKDAY.INTL.json`.
