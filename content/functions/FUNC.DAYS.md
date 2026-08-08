---
schema: efh.function-page/v1
function_id: FUNC.DAYS
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: DAYS function"
    locator: "https://support.microsoft.com/en-us/office/days-function-57740535-d549-4395-8728-0f07bff0b9df"
    role: "documented argument order, the DATEVALUE treatment of text arguments, and the error conditions"
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
role_in_family: "Measures an interval: the signed number of days between two dates. The only
  member of the parts family that consumes two serials rather than decomposing one."
---

## What it computes

`DAYS` returns the signed number of days from `start_date` to `end_date`. Microsoft's page states
the rule directly: when both arguments are numbers, `DAYS` computes **end − start**. So it is
subtraction with a name — and with a documented coercion policy that plain subtraction does not
have.

Two things make it worth a page of its own rather than a footnote to the `−` operator:

1. **The argument order is end-first.** `DAYS(end_date, start_date)`. Every other two-date
   function in the category — [DATEDIF](FUNC.DATEDIF.md), [DAYS360](FUNC.DAYS360.md),
   [NETWORKDAYS](FUNC.NETWORKDAYS.md), [YEARFRAC](FUNC.YEARFRAC.md) — takes start first. `DAYS` is
   the exception, and getting it wrong silently returns the negation rather than an error.
2. **Text arguments are routed through `DATEVALUE`, not through ordinary numeric coercion.**
   Microsoft documents that "if either one of the date arguments is text, that argument is treated
   as `DATEVALUE(date_text)`". This is a per-function coercion policy, not the engine-wide
   text-to-number rule described in
   [coercion and lifting](../model/02-coercion-and-lifting.md) — and it drags `DAYS` into locale
   dependence that the numeric path does not have. `DAYS("3/1/2024", "1/1/2024")` is a
   locale-sensitive expression; `DAYS(45352, 45292)` is not.

## Arguments

`DAYS(end_date, start_date)` — both required.

**end_date** — the later endpoint by convention, though nothing requires it to be later; a
start after the end simply gives a negative result.

**start_date** — the earlier endpoint by convention.

Both arguments accept either a numeric serial or text. The documentation's own wording keeps the
two paths separate: numbers are subtracted, text is parsed by `DATEVALUE` first. Microsoft also
notes that the result is an integer without time components, so fractional serials contribute no
fractional days.

The argument position most often misused is the first one. Because the function is named for a
quantity rather than a direction, readers frequently write `DAYS(start, end)` and get a negative
number that then flows quietly into a downstream calculation.

## Result and edge cases

Returns a `Number`: an integer count of days, negative when `start_date` is later than `end_date`.

- **Equal dates** give 0.
- **Fractional serials** do not produce a fractional answer; Microsoft's remark that the result
  carries no time component says the truncation happens, but the page does not say whether each
  argument is truncated before subtracting or the difference is truncated afterwards. Those two
  rules disagree — `DAYS(2.4, 1.6)` is 1 under the first and 0 under the second. **This is not
  settled here.**
- **Straddling the 1900 leap-year artefact.** An interval whose endpoints lie on opposite sides of
  serial 60 counts the phantom 29 February 1900 as a real day. `DAYS(61, 59)` is 2 as arithmetic
  and one day too many as a calendar fact. See [FUNC.DATE](FUNC.DATE.md).
- **Mixed numeric/text arguments** take one path each, per the documented rule.

## Errors

As documented on the Microsoft page:

- `#NUM!` when numeric date arguments fall outside the range of valid dates.
- `#VALUE!` when a text argument cannot be parsed as a valid date.

An error value passed as either argument propagates in the ordinary way.

## Relationships

- **The `−` operator.** `end − start` on two date serials gives the same number for numeric
  arguments and is not equivalent for text arguments, because `−` uses the engine's ordinary
  text-to-number coercion while `DAYS` is documented to use `DATEVALUE`. This is the sharpest
  reason to prefer `DAYS` when inputs may be text — and to prefer `−` when they may not, since
  `DAYS` then buys nothing but a locale dependency.
- **[DATEDIF](FUNC.DATEDIF.md) with unit `"D"`** — the same count, but with start-first argument
  order and with a documented `#NUM!` when start is after end, where `DAYS` simply goes negative.
- **[DAYS360](FUNC.DAYS360.md)** — an interval in *accounting* days, not calendar days; the two
  functions answer different questions and are routinely confused.
- **[NETWORKDAYS](FUNC.NETWORKDAYS.md)** — calendar days minus weekends and holidays.
- **[DAY](FUNC.DAY.md)** — the near-homonym, and unrelated: day-of-month of a single date.

## Notes for implementers

1. **Implement the two coercion paths separately and visibly.** A single "coerce to number"
   front-end will silently give `DAYS` the wrong behaviour for text, because ordinary numeric
   coercion does not parse "1-Jan-2024" and `DATEVALUE` does.
2. **Locale is an input.** Once text arguments route through `DATEVALUE`, the function inherits
   `DATEVALUE`'s locale profile — the reference engine's own classification marks `DATEVALUE` (not
   `DAYS`) as locale-dependent, which is a modelling choice worth being aware of when reasoning
   about the composite.
3. **Pin the truncation order** before writing tests, and state it in the implementation, since the
   documentation does not.
4. **Do not "fix" the leap-year artefact here.** The count is defined on serials; correcting for
   the phantom day would make `DAYS` disagree with subtraction, which is the one thing the
   documentation guarantees it does not do.

## What has not been checked

No Handbook vector suite exists for `DAYS`, and no Excel-comparison evidence is recorded. The
battery panel on this page is the reference engine answering its own probes; no Excel was
involved. The probes worth running first:

- **`DAYS(2.4, 1.6)` and similar fractional pairs** — settles the truncation-order question the
  documentation leaves open.
- **`DAYS("1-Jan-2024", "1/1/2024")` under two different locales** — confirms the documented
  `DATEVALUE` routing and shows how far the locale dependence reaches.
- **`DAYS(61, 59)` and `DAYS(59, 61)`** — the phantom-day window, and the sign convention in one
  test.
- **`DAYS(2958466, 1)`** — the first serial past the documented ceiling, to locate the `#NUM!`
  boundary.
- **Mixed error/logical arguments** — `DAYS(TRUE, FALSE)`, `DAYS(#N/A, 1)` — to confirm that the
  ordinary propagation rules apply on top of the special text path.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| end-first argument order | `DAYS(end_date, start_date)`; the reverse of the rest of the category |
| DATEVALUE routing | The documented rule that a text argument is parsed as a date, not coerced as a number |
| accounting days | The 30/360 convention used by `DAYS360`, distinct from calendar days |

## Sources

- Microsoft 365 support, **DAYS function** —
  <https://support.microsoft.com/en-us/office/days-function-57740535-d549-4395-8728-0f07bff0b9df>.
  Source of the argument order, the `EndDate–StartDate` rule, the `DATEVALUE` treatment of text
  arguments, and both error conditions.
- [FUNC.DATE](FUNC.DATE.md) — serial-number model and the 1900 leap-year artefact.
- [FUNC.DATEVALUE](FUNC.DATEVALUE.md) — the parser this function delegates to for text.
- Handbook call-model chapter [02 coercion and lifting](../model/02-coercion-and-lifting.md), whose
  per-family policy framing this function is a clean instance of.
- `data/functions/FUNC.DAYS.json`, `data/presence/FUNC.DAYS.json`, `data/battery/FUNC.DAYS.json`.
