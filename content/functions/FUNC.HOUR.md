---
schema: efh.function-page/v1
function_id: FUNC.HOUR
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: HOUR function"
    locator: "https://support.microsoft.com/en-us/office/hour-function-a3afa879-86cb-4339-b1b5-2dd2d7310ac7"
    role: "documented behaviour of the serial_number argument and the 0-23 return range"
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
role_in_family: "Extracts the hour component from the fractional part of a serial, as an integer
  0 through 23."
---

## What it computes

`HOUR` reads the hour-of-day out of a serial's fractional part. Conceptually:

> `HOUR(s)` = ⌊ 24 · frac(s) ⌋, as an integer 0…23

The integer part of `s` — which day it is — is irrelevant and discarded. This is the property that
makes `HOUR` a **clock reading and not a duration**: `HOUR(1.5)` is 12, not 36. A value of 1.5 days
displayed as a duration is thirty-six hours, but `HOUR` reports the position within the day, so it
answers 12. Anyone computing elapsed hours wants `(end − start) * 24`, not `HOUR(end − start)`.

The caveat that keeps this from being a one-line function is that `frac(s)` is a binary double and
the natural denominators of clock time — 24, 1440, 86400 — are not powers of two. A serial that
"is" 08:00 is generally not exactly 1/3, so the floor above cannot be applied naively to the raw
fraction without occasionally reporting 07 instead of 08. Some rounding to the nearest
representable second is therefore inevitable in any implementation; **what rule Excel uses is not
stated on the documentation page, and has not been established here.** See below.

## Arguments

`HOUR(serial_number)` — one required argument: the time whose hour is wanted, as a serial.

Microsoft's category guidance applies: prefer serials produced by [TIME](FUNC.TIME.md),
[NOW](FUNC.NOW.md), or another formula over text, whose interpretation is locale-dependent.
Argument coercion is the ordinary scalar behaviour of
[coercion and lifting](../model/02-coercion-and-lifting.md); numeric text and logicals convert,
errors propagate.

## Result and edge cases

Returns a `Number` in 0…23.

- **Whole serials** have zero fractional part, so `HOUR` of any pure date is 0.
- **The day component is ignored entirely**, including for serials representing more than one day.
- **Values at exact hour boundaries** are where the representability question lands:
  `HOUR(TIME(8,0,0))` and `HOUR(8/24)` are the same mathematical quantity but need not be the same
  double, and a floor applied to a value one ulp below the boundary gives the wrong hour.
- **Negative serials** are the documented `#NUM!` region for this category.

## Errors

- `#NUM!` when `serial_number` is out of range for the workbook's date-base value.
- `#VALUE!` when the argument will not convert to a number (ordinary coercion behaviour).
- An error argument propagates.

These are the category-wide conditions documented for the date-and-time functions; the linked page
does not add a condition specific to `HOUR`.

## Relationships

- **[MINUTE](FUNC.MINUTE.md), [SECOND](FUNC.SECOND.md)** — the finer projections of the same
  fraction. Together the three invert [TIME](FUNC.TIME.md) on its in-range domain.
- **[TIME](FUNC.TIME.md)** — the constructor. `TIME(HOUR(s),MINUTE(s),SECOND(s))` should return
  `frac(s)` rounded to the second; whether it does exactly is the round-trip test this family
  needs.
- **[NOW](FUNC.NOW.md)** — the usual source of a serial with a meaningful fractional part.
- **Confusable.** `HOUR(a−b)` for elapsed time, discussed above; and the `TEXT` function with an
  `[h]` format code, which *does* express durations beyond 24 hours and returns text.

## Notes for implementers

1. **Convert to whole seconds once, then split.** Compute a rounded second-of-day from the
   fraction and derive hour, minute and second from that single integer. Deriving each component
   independently from the raw double lets the three functions disagree with each other — reporting
   09:59:60-style inconsistencies at boundaries.
2. **The rounding rule is the whole compatibility question.** Round-half-even, round-half-up, and
   nearest-representable-second all give the same answer for almost every input and differ on a
   thin set — which is exactly the set a vector suite has to contain to say anything.
3. **Do not model this as a duration extractor.** `frac` first, always.

## What has not been checked

No Handbook vector suite exists for `HOUR`, and no Excel-comparison evidence is recorded. The
battery on this page is the reference engine answering its own probes; no Excel was involved.
Probes that would settle real questions:

- **Serials one ulp either side of each exact hour boundary** — `nextafter(8/24, 0)` and
  `nextafter(8/24, 1)`. This is the only family of inputs that can distinguish the candidate
  rounding rules, and it is the reason a suite for this function is worth building.
- **`HOUR(TIME(h,0,0))` for all 24 hours** — the cheapest round-trip check, and one that any
  implementation should pass before anything harder is attempted.
- **`HOUR(1.5)`, `HOUR(2.75)`** — confirming the discard-the-day rule from Excel rather than from
  the definition.
- **`HOUR(0)` and the negative boundary** — where `#NUM!` starts.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| clock reading | A value determined by position within the day, discarding whole days |
| second-of-day | The integer 0…86399 that a robust implementation derives once and splits |
| representability | The fact that 1/24, 1/1440 and 1/86400 are not exact in binary floating point |

## Sources

- Microsoft 365 support, **HOUR function** —
  <https://support.microsoft.com/en-us/office/hour-function-a3afa879-86cb-4339-b1b5-2dd2d7310ac7>.
- [FUNC.TIME](FUNC.TIME.md) and [FUNC.DATE](FUNC.DATE.md) — the constructor side of the serial
  model this page reads.
- Handbook call-model chapter [02 coercion and lifting](../model/02-coercion-and-lifting.md).
- `data/functions/FUNC.HOUR.json`, `data/presence/FUNC.HOUR.json`, `data/battery/FUNC.HOUR.json`.
