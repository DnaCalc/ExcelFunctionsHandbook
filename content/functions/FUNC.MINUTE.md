---
schema: efh.function-page/v1
function_id: FUNC.MINUTE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: MINUTE function"
    locator: "https://support.microsoft.com/en-us/office/minute-function-af728df0-05c4-4b07-9eed-a84801a60589"
    role: "documented behaviour of the serial_number argument and the 0-59 return range"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
family: date_parts_family
role_in_family: "Extracts the minute component from the fractional part of a serial, as an integer
  0 through 59."
---

## What it computes

`MINUTE` reads the minute-within-the-hour out of a serial's fractional part:

> `MINUTE(s)` = ⌊ 1440 · frac(s) ⌋ mod 60, as an integer 0…59

The whole-day part of `s` is discarded, and so is the hour: the result is the minute hand's
position, not a count of minutes. `MINUTE(1.5)` is 0, because 1.5 days is 12:00 — the same reason
`HOUR(1.5)` is 12 rather than 36. For elapsed minutes the arithmetic is `(end − start) * 1440`.

As with the rest of this trio, the mathematical definition is exact and the implementation is not:
`frac(s)` is a binary double and 1/1440 is not representable, so the floor above must be applied to
a value that has been rounded to a whole second first, or minute boundaries will occasionally read
one low. **What rounding rule Excel applies is not stated on the documentation page and has not
been established here.**

## Arguments

`MINUTE(serial_number)` — one required argument: the time whose minute is wanted.

Supply a serial from [TIME](FUNC.TIME.md), [NOW](FUNC.NOW.md), or another formula in preference to
text; text times are parsed with locale rules, as [TIMEVALUE](FUNC.TIMEVALUE.md) makes explicit.
Coercion follows [the ordinary scalar rules](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns a `Number` in 0…59.

- **A whole serial** has no fractional part, so `MINUTE` of any pure date is 0.
- **The hour is discarded**, not accumulated. This is the difference between a clock reading and a
  duration, and it is the most common misuse of the function.
- **Boundary serials.** `MINUTE(TIME(0,30,0))` and `MINUTE(0.5/24)` are the same mathematical
  value and need not be the same double; a naive floor on the raw fraction can yield 29.
- **Negative serials** fall in this category's documented `#NUM!` region.

## Errors

- `#NUM!` when `serial_number` is out of range for the workbook's date-base value.
- `#VALUE!` when the argument cannot be coerced to a number (ordinary coercion behaviour).
- An error argument propagates.

The linked Microsoft page documents no condition specific to `MINUTE` beyond these category-wide
ones.

## Relationships

- **[HOUR](FUNC.HOUR.md), [SECOND](FUNC.SECOND.md)** — the coarser and finer projections of the
  same fraction; the three together invert [TIME](FUNC.TIME.md).
- **[TIME](FUNC.TIME.md)** — the constructor, whose own normalization carries minutes above 59 into
  hours; `MINUTE` is the projection that undoes that carry.
- **Confusable.** `MINUTE(a−b)` for elapsed minutes (wrong — use `(a−b)*1440`), and the `TEXT`
  function's `[m]` format code, which does express elapsed minutes and returns text.

## Notes for implementers

1. **Derive a second-of-day integer once**, then compute `(sod / 60) mod 60`. Independent
   derivations of hour, minute and second from the raw double can produce mutually inconsistent
   triples at boundaries.
2. **`mod 60`, not `mod 100`.** Trivial, but the class of bug that survives review because the
   test data never crossed an hour.
3. **The rounding rule is shared with `HOUR` and `SECOND`** and should be implemented in one place;
   whatever it turns out to be, all three must use the same one or round trips through
   [TIME](FUNC.TIME.md) will not close.

## What has not been checked

No Handbook vector suite exists for `MINUTE`, and no Excel-comparison evidence is recorded. The
battery panel on this page shows the reference engine's answers, obtained without Excel. What to
probe first:

- **Serials one ulp either side of a minute boundary** — the only inputs that discriminate between
  candidate rounding rules.
- **`MINUTE(TIME(h,m,0))` across the full 24×60 grid** — 1,440 rows, trivially generated, and
  enough to characterize the ordinary domain completely.
- **`MINUTE` of a value produced by subtraction**, e.g. two timestamps an hour and a half apart,
  where accumulated representation error is largest.
- **`MINUTE(0)` and the negative boundary.**

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| clock reading | A value determined by position within the hour, discarding whole hours and days |
| second-of-day | The integer 0…86399 a robust implementation derives once and splits |
| round trip | `TIME(HOUR(s),MINUTE(s),SECOND(s))` returning `frac(s)` to the second |

## Sources

- Microsoft 365 support, **MINUTE function** —
  <https://support.microsoft.com/en-us/office/minute-function-af728df0-05c4-4b07-9eed-a84801a60589>.
- [FUNC.TIME](FUNC.TIME.md) — the constructor side of the model this page reads.
- Handbook call-model chapter [02 coercion and lifting](../model/02-coercion-and-lifting.md).
- `data/functions/FUNC.MINUTE.json`, `data/presence/FUNC.MINUTE.json`,
  `data/battery/FUNC.MINUTE.json`.
