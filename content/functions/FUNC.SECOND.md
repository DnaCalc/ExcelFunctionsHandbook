---
schema: efh.function-page/v1
function_id: FUNC.SECOND
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: SECOND function"
    locator: "https://support.microsoft.com/en-us/office/second-function-740d1cfc-553c-4099-b668-80eaa24e8af1"
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
role_in_family: "Extracts the second component from the fractional part of a serial; the finest
  projection Excel's time model exposes, and the one where floating-point rounding is visible."
---

## What it computes

`SECOND` reads the second-within-the-minute out of a serial's fractional part:

> `SECOND(s)` = ⌊ 86400 · frac(s) ⌋ mod 60, as an integer 0…59

Days, hours and minutes are all discarded; the result is a clock reading, not a count. `SECOND` is
also the end of the road: Excel's time model exposes nothing finer, so sub-second information
present in the double is not reachable through this family.

This is the function where the representability problem in Excel's time model becomes visible
rather than theoretical. The fraction of a day corresponding to one second is 1/86400, which is
not representable in binary; a serial built to mean 08:30:00 differs from the exact mathematical
value by a fraction of an ulp, and 86400 times that error can push the floor across an integer
boundary. Every credible implementation therefore rounds the fraction to a whole second before
splitting it. **The exact rounding rule Excel uses is not stated on the documentation page and has
not been established for this Handbook** — which is precisely why this function is worth a suite.

## Arguments

`SECOND(serial_number)` — one required argument: the time whose second is wanted.

Microsoft's category guidance applies: prefer serials from [TIME](FUNC.TIME.md),
[NOW](FUNC.NOW.md) or another formula over text, whose parse is locale-dependent. Coercion follows
[the ordinary scalar rules](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns a `Number` in 0…59.

- **Whole serials** have no fractional part; `SECOND` of a pure date is 0.
- **No sub-second precision is exposed.** A serial carrying half a second of extra offset reports
  the same integer as one that does not — until the rounding rule pushes it over.
- **Values produced by subtraction** are where mismatches surface first. The difference of two
  timestamps accumulates representation error from both operands, so `SECOND(b−a)` is the most
  fragile common use of the function.
- **Negative serials** fall in this category's documented `#NUM!` region.

## Errors

- `#NUM!` when `serial_number` is out of range for the workbook's date-base value.
- `#VALUE!` when the argument cannot be coerced to a number (ordinary coercion behaviour).
- An error argument propagates.

The linked Microsoft page documents no `SECOND`-specific condition beyond these.

## Relationships

- **[HOUR](FUNC.HOUR.md), [MINUTE](FUNC.MINUTE.md)** — the coarser projections. The three together
  invert [TIME](FUNC.TIME.md) to whole-second resolution.
- **[TIME](FUNC.TIME.md)** — the constructor. `TIME(HOUR(s),MINUTE(s),SECOND(s))` versus `frac(s)`
  is the round trip that any rounding-rule hypothesis has to survive.
- **[NOW](FUNC.NOW.md)** — whether `NOW` even carries a nonzero seconds component is a property of
  the host's clock granularity, not of `SECOND`.
- **Confusable.** `SECOND(b−a)` for elapsed seconds; the correct expression is `(b−a)*86400`.

## Notes for implementers

1. **Round to a whole second-of-day once, in shared code.** `HOUR`, `MINUTE` and `SECOND` must all
   read the same integer or their answers can be mutually inconsistent — a triple like 08:59:60 is
   reachable if each function rounds independently.
2. **Choose the rounding mode deliberately and record the choice.** Round-half-up, round-half-even
   and "nearest representable second" agree almost everywhere and disagree on a thin, findable set.
   An implementation that has not written down which one it uses does not know what it does.
3. **Guard the carry.** Rounding 59.6 seconds up must carry into the minute and, at 23:59:59.6,
   into the day. An implementation that rounds seconds without carrying will emit 60.

## What has not been checked

No Handbook vector suite exists for `SECOND`, and no Excel-comparison evidence is recorded. The
battery on this page is the reference engine's own answer set; no Excel was involved. The probes
that would actually settle something:

- **Serials one ulp either side of every second boundary in a representative hour.** This is the
  discriminating set for the rounding rule and the reason this function, alone in the parts family,
  deserves a large suite rather than a small one.
- **`SECOND` of differences** — build two timestamps a known number of seconds apart via
  `DATE`+`TIME`, subtract, and read the seconds back. Round-trip failures here are what users
  actually report.
- **The 23:59:59.5 region** — whether rounding carries into the next day, and what `HOUR` and
  `MINUTE` report for the same input.
- **`SECOND(0)` and the negative boundary.**

Until such a suite exists, no statement about `SECOND` agreeing with Excel at the boundary can be
made in either direction, and this page makes none.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| clock reading | A value determined by position within the minute, discarding larger units |
| second-of-day | The integer 0…86399 that shared code should derive once |
| discriminating set | The thin set of inputs on which candidate rounding rules disagree |

## Sources

- Microsoft 365 support, **SECOND function** —
  <https://support.microsoft.com/en-us/office/second-function-740d1cfc-553c-4099-b668-80eaa24e8af1>.
- [FUNC.TIME](FUNC.TIME.md) — the constructor and its documented 0…0.99988426 range.
- Handbook call-model chapter [02 coercion and lifting](../model/02-coercion-and-lifting.md).
- `data/functions/FUNC.SECOND.json`, `data/presence/FUNC.SECOND.json`,
  `data/battery/FUNC.SECOND.json`.
