---
schema: efh.function-page/v1
function_id: FUNC.TIME
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: TIME function"
    locator: "https://support.microsoft.com/en-us/office/time-function-9a5aff99-8f7d-4611-845e-747d0b8d5457"
    role: "documented argument ranges, overflow rules and the 0 to 0.99988426 result range"
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
role_in_family: "The time constructor: assembles a fraction-of-a-day from hour, minute and second,
  and is the only member of the parts family that builds rather than decomposes."
---

## What it computes

`TIME` is the constructor for the *fractional* half of a date serial, as
[DATE](FUNC.DATE.md) is the constructor for the integer half. It takes three numbers and returns
the fraction of a day that they name:

> `TIME(h, m, s)` = frac( (3600·h + 60·m + s) / 86400 )

That is the whole definition, and reading it as one expression rather than three separate rules
explains everything Microsoft's page documents piecewise. The page says an hour above 23 "will be
divided by 24 and the remainder will be treated as the hour value", that a minute above 59 "will
be converted to hours and minutes", and that a second above 59 "will be converted to hours,
minutes, and seconds". All three are the same statement: the three components are summed into
seconds, and only the position within the day survives.

Two properties follow immediately, and both matter:

1. **The result is always in [0, 1).** Microsoft documents the range as 0 to 0.99988426, which is
   86399/86400 — one second short of a full day. `TIME` cannot return 1, and therefore cannot
   express "24:00" or any duration longer than a day.
2. **`TIME` is a clock, not a stopwatch.** `TIME(25,0,0)` is 01:00, not "25 hours". To express a
   duration you do the division yourself: `25/24`. This is the single most common misuse of the
   function.

Like `DATE`, `TIME` is a normalizing constructor rather than a validator: it does not reject 90
minutes, it carries them.

## Arguments

`TIME(hour, minute, second)` — all three required.

| Argument | Documented admissible range | What out-of-range does |
|---|---|---|
| `hour` | 0 … 32767 | values above 23 wrap modulo 24 |
| `minute` | 0 … 32767 | values above 59 carry into hours |
| `second` | 0 … 32767 | values above 59 carry into minutes and hours |

The 32767 ceiling is the documented upper bound on each argument. Microsoft's page does not state
what happens above it; this Handbook has not checked, and `#NUM!` is a guess, not a finding.

Negative components are the genuinely under-documented case. The page gives ranges starting at 0
and says nothing about negatives, yet `TIME(1,-30,0)` is a perfectly well-formed call. Whether it
yields 00:30, an error, or a wrapped value is unknown here — see below.

## Result and edge cases

Returns a `Number` in [0, 1). Excel will normally show it in a time format; that formatting is
host-side adaptation and not part of the value (see
[the call pipeline](../model/03-call-pipeline.md), stage 4).

Because the result is a binary double and the natural denominator is 86400 — which is not a power
of two — almost every time value is inexact. `TIME(0,0,1)` is not exactly 1/86400. This is not a
defect of `TIME`; it is the same representability fact that makes `=A1-B1` on two timestamps
produce values like 0.20833333333333337. It becomes visible when a time is compared for equality,
or fed back through `HOUR`/`MINUTE`/`SECOND`.

Argument coercion is the ordinary scalar behaviour of
[coercion and lifting](../model/02-coercion-and-lifting.md): numeric text converts, logicals
become 1 and 0, an error argument propagates.

## Errors

Microsoft's TIME page documents overflow handling but does not enumerate error conditions. What
can be said honestly:

- `#VALUE!` from an argument that will not convert to a number is the ordinary coercion outcome,
  not a `TIME`-specific rule.
- An error value passed as any argument propagates.
- Whether arguments above the documented 32767 ceiling, or negative arguments, produce `#NUM!` is
  **not documented on that page and not verified here**.

## Relationships

- **[DATE](FUNC.DATE.md)** — the integer-part constructor. `DATE(...) + TIME(...)` is the
  idiomatic way to build a full timestamp.
- **[HOUR](FUNC.HOUR.md), [MINUTE](FUNC.MINUTE.md), [SECOND](FUNC.SECOND.md)** — the
  decomposition. They are the left inverses of `TIME` on the in-range domain, modulo the
  representability caveat above.
- **[TIMEVALUE](FUNC.TIMEVALUE.md)** — reaches the same fractions from text, and is
  locale-dependent where `TIME` is not.
- **[NOW](FUNC.NOW.md)** — supplies a live timestamp whose fractional part is what `TIME` would
  construct.
- **Confusable.** `TIME` and the `TEXT` function with a time format code are routinely
  interchanged in advice; `TEXT` produces text, `TIME` produces a number.

## Notes for implementers

1. **Sum first, wrap once.** Computing `h mod 24` before adding minutes and seconds gives the
   right answer for the documented examples but is fragile; summing to total seconds and taking
   one modulus is both simpler and closer to what the documentation describes.
2. **Decide the negative policy explicitly.** Whichever behaviour Excel turns out to have, an
   implementation that falls into it accidentally through language-defined `%` semantics is a
   latent divergence. Rust and C `%` truncate toward zero; Python's `%` floors. They disagree
   exactly on the undocumented case.
3. **Do not round the result.** The publication profile for this function is the plain IEEE-754
   double (see [the call pipeline](../model/03-call-pipeline.md)); the visible "0.99988426" in the
   documentation is a display of the maximum, not a rounding rule.

## What has not been checked

No Handbook vector suite exists for `TIME`, and no Excel-comparison evidence is recorded. The
battery panel on this page is the reference engine answering its own probes; no Excel was
involved. The inputs worth probing first, and why:

- **Negative components** — `TIME(1,-30,0)`, `TIME(-1,0,0)`, `TIME(0,0,-1)`. Undocumented, and the
  case where independent implementations are most likely to disagree with each other and with
  Excel.
- **The 32767 ceiling** — `TIME(32767,0,0)` versus `TIME(32768,0,0)`. Whether the documented
  ceiling is an error boundary or merely descriptive.
- **Non-integer components** — `TIME(1.5,0,0)`. Truncation is plausible; nothing here establishes
  it.
- **Exact bit patterns for common times** — `TIME(12,0,0)` is exactly 0.5 and safe, but
  `TIME(0,0,1)`, `TIME(8,30,0)` and similar are the values a round-trip suite would need in order
  to say anything about agreement at all.

A suite here is inexpensive: the input space is small and integral, and the only floating-point
question is which of the two obvious division orders Excel uses.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| date serial | The `Number` denoting a moment; integer part the day, fractional part the time of day |
| normalizing constructor | A constructor that carries out-of-range components rather than rejecting them |
| clock semantics | Result depends only on position within the day; durations beyond 24 h are not representable |

## Sources

- Microsoft 365 support, **TIME function** —
  <https://support.microsoft.com/en-us/office/time-function-9a5aff99-8f7d-4611-845e-747d0b8d5457>.
  Source of the argument ranges, the three overflow rules, and the 0–0.99988426 result range.
- [FUNC.DATE](FUNC.DATE.md) — this Handbook's account of the serial-number model, which this page
  assumes rather than repeats.
- Handbook call-model chapters
  [02 coercion and lifting](../model/02-coercion-and-lifting.md) and
  [03 call pipeline](../model/03-call-pipeline.md).
- `data/functions/FUNC.TIME.json`, `data/presence/FUNC.TIME.json`, `data/battery/FUNC.TIME.json`.
