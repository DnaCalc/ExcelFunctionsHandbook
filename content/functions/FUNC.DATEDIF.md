---
schema: efh.function-page/v1
function_id: FUNC.DATEDIF
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: DATEDIF function"
    locator: "https://support.microsoft.com/en-us/office/datedif-function-25dba1a4-2812-480b-84dd-8b32a451b35c"
    role: "the one Microsoft page for this function: unit table, Lotus 1-2-3 rationale, the MD warning, and the #NUM! condition"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
family: date_value_family
role_in_family: "Interval measurement in calendar units, with six unit codes of markedly uneven
  quality; the category's compatibility survivor."
---

## What it computes

`DATEDIF` measures the interval between two dates in a calendar unit named by a text code. It is
the only function in this category whose *unit* is an argument rather than baked into the name,
and it is the only one Microsoft explicitly describes as retained for compatibility: "Excel
provides the DATEDIF function in order to support older workbooks from Lotus 1-2-3."

Its status is genuinely unusual and worth stating plainly, because readers meet the function
through folklore before they meet it through documentation:

- It **is** documented. Microsoft publishes a support page for it, cited below, with a syntax line,
  a unit table, a caveat and an error condition.
- It is widely reported to be absent from Excel's in-product function surface — formula
  autocomplete, the function wizard, argument tooltips — which is why readers meet it as "hidden"
  or "undocumented". This Handbook has not verified the in-product behaviour in any Excel build,
  so it is recorded here as a widely-reported property and not as a finding. What can be said from
  the record is narrower and sufficient: the function is **documented but unadvertised**.
- Microsoft's page **discourages one of its own unit codes**: "We don't recommend using the 'MD'
  argument, as there are known limitations with it."

The six documented units, verbatim from that page:

| `unit` | Returns |
|---|---|
| `"Y"` | "The number of complete years in the period." |
| `"M"` | "The number of complete months in the period." |
| `"D"` | "The number of days in the period." |
| `"MD"` | "The difference between the days in start_date and end_date. The months and years of the dates are ignored." |
| `"YM"` | "The difference between the months in start_date and end_date. The days and years of the dates are ignored" |
| `"YD"` | "The difference between the days of start_date and end_date. The years of the dates are ignored." |

Two different kinds of thing are in that table, and the mixture is the source of the surprise:

- **`"Y"`, `"M"`, `"D"` are interval counts** — how much whole time has elapsed. `"Y"` is the
  "complete years" that make it the standard age formula, and it is *not* `YEAR(end) − YEAR(start)`.
- **`"MD"`, `"YM"`, `"YD"` are component remainders** — what is left over after the larger units are
  removed. They exist so that `Y`/`YM`/`MD` can be composed into "3 years, 2 months and 17 days".

The remainder units are where the function's reputation comes from. `"MD"` compares two
day-of-month numbers with the months and years discarded; when the end day-of-month is smaller than
the start day-of-month, the subtraction has to borrow from a month whose length depends on which
month you decide to borrow from — and there is no canonical answer. Microsoft's own page reports
the consequence rather than the mechanism: `"MD"` can produce negative values, zero, or inaccurate
results. This Handbook records that as documented vendor guidance, not as a measured
characterization; nobody has determined here *which* inputs misbehave or why.

## Arguments

`DATEDIF(start_date, end_date, unit)` — all three required. Note the argument order: start first,
unlike [DAYS](FUNC.DAYS.md), which takes the end date first.

**start_date** — the beginning of the period, as a date serial. Microsoft's page treats it as a
serial like the rest of the category.

**end_date** — the end of the period. Must not be earlier than `start_date`; see Errors.

**unit** — text, one of the six codes above. The codes are conventionally written uppercase; the
documentation gives them uppercase and does not state whether matching is case-insensitive, which
is a small but real open question for an implementer.

The misunderstood argument is `unit`, in two ways. Readers reach for `"M"` expecting a month
*difference* and get complete months elapsed; and readers reach for `"YD"` expecting "days into the
year" and get something defined by discarding the years from both dates, which is not the same
thing when the two dates sit either side of 29 February.

## Result and edge cases

Returns a `Number`: an integer count in the requested unit.

- **`"Y"` is complete years, and that is why it works for ages.** `DATEDIF(birth, today, "Y")` is
  correct on the day before a birthday and correct on the birthday, where `YEAR(today) −
  YEAR(birth)` is wrong on one of them.
- **`"D"` is a plain day count**, and Microsoft's page says so explicitly, adding: "If you want to
  find the number of days between two dates, simply subtract the later date from the earlier date."
  That is an unusually candid piece of documentation — the vendor telling you the function is
  unnecessary for its simplest unit. (The sentence as published says to subtract "the later date
  from the earlier date", which would give a negative count; the intent is plainly the other way
  round. Quoted here as written, per the Handbook's practice of not silently repairing sources.)
- **`"MD"` is flagged by Microsoft as unreliable** and should be treated as such.
- **Leap-day arguments** are where `"YM"` and `"YD"` are most likely to surprise, because
  discarding the year from 29 February leaves a date that does not exist in most years.
- **Equal dates** give 0 in every unit.
- **Time components** — the page does not state whether fractional serials are truncated before the
  comparison. Unverified here.

## Errors

As documented on the Microsoft page:

- `#NUM!` "if the start_date is greater than the end_date". This is a strict, documented asymmetry:
  `DATEDIF` refuses negative intervals where plain subtraction and [DAYS](FUNC.DAYS.md) simply go
  negative. Any wrapper that feeds unsorted date pairs to `DATEDIF` will surface `#NUM!` for half
  of them.

`#VALUE!` for an unrecognized `unit` code is the behaviour readers expect and is **not stated on
that page**; it is not asserted here. Ordinary coercion failures and error propagation follow the
engine rules in [coercion and lifting](../model/02-coercion-and-lifting.md).

## Relationships

- **[DAYS](FUNC.DAYS.md)** and the `−` operator — the direct competitors for `"D"`, both of which
  tolerate a negative interval where `DATEDIF` errors, and one of which Microsoft's own page
  recommends over `DATEDIF`.
- **[YEARFRAC](FUNC.YEARFRAC.md)** — the *fractional* interval in years, on a selectable day-count
  basis. `YEARFRAC` answers "how much of a year", `DATEDIF("Y")` answers "how many whole years";
  they are frequently substituted for one another and are not interchangeable.
- **[EDATE](FUNC.EDATE.md)** — the inverse operation for months: `DATEDIF(a, EDATE(a, n), "M")`
  should be `n` for any `n ≥ 0`, which is the obvious metamorphic relation for this function and,
  as far as this Handbook records, has never been run.
- **[DAYS360](FUNC.DAYS360.md)** — also an interval, but on the accounting 30/360 convention.
- **Compatibility siblings.** `DATEDIF` is not a Compatibility-category function in Microsoft's
  taxonomy — it sits in Date and time — but it is a compatibility artefact all the same, with a
  named provenance: Lotus 1-2-3.

## Notes for implementers

1. **Implement the three interval units and the three remainder units as separate ideas.** The
   remainder units are not "the interval units modulo something"; they are defined by discarding
   components of the *operands*, before any subtraction.
2. **`"MD"` needs a documented borrowing rule, and the documentation does not supply one.** Whatever
   an implementation does, it should say so, and it should not claim to match Excel until someone
   has looked. This is the single largest unknown in the function.
3. **Enforce the `start > end` error before anything else.** It is the one documented error and it
   changes the shape of the whole contract.
4. **Decide and record the `unit` matching rule** — case sensitivity, surrounding whitespace,
   whether a non-text argument coerces. None of it is documented.
5. **Do not implement `"Y"` as a year subtraction with a correction.** Compute it as "the largest
   `n` such that `start` advanced by `n` years is not after `end`", which is correct by
   construction and makes the leap-day cases fall out rather than needing patches.

## What has not been checked

No Handbook vector suite exists for `DATEDIF`, and no Excel-comparison evidence is recorded. The
battery panel on this page shows the reference engine answering its own probes; no Excel was
involved. `DATEDIF` is the function in this assignment with the widest gap between what is
documented and what an implementer needs, so the list of unrun experiments is long and each item is
load-bearing:

- **`"MD"` across a full year of day-of-month pairs**, including 31 January → 1 March, 31 March →
  30 April, and every end-of-month to end-of-month step. This is the only way to learn what
  Microsoft's "known limitations" actually are, and it is the highest-value experiment on this
  page.
- **`"YM"` and `"YD"` with a 29 February operand** — what happens when discarding the year leaves a
  date that does not exist.
- **`"Y"` on birthdays** — `DATEDIF(DATE(2000,2,29), DATE(2001,2,28), "Y")` and the same for
  2001-03-01. Whether a leap-day anniversary is reached on 28 February or 1 March is a real
  question with legal consequences and no documented answer.
- **An unrecognized `unit`** — `DATEDIF(a, b, "X")`, `""`, `"y"` lowercase, `1`. Settles both the
  error value and the matching rule.
- **`start_date` one day after `end_date`** — confirms the documented `#NUM!` boundary is strict
  rather than tolerant of equality.
- **Fractional serials** — whether the time of day participates.
- **The metamorphic relations** — `DATEDIF(a, EDATE(a,n), "M") = n`, and `DATEDIF(a,b,"D") = b − a`
  for `b ≥ a`. Cheap, and they would catch a large class of implementation error without needing an
  Excel oracle at all.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| interval unit | `"Y"`, `"M"`, `"D"` — how much whole time elapsed between the dates |
| remainder unit | `"MD"`, `"YM"`, `"YD"` — what is left after larger components are discarded |
| documented but unadvertised | Published on a Microsoft support page, but absent from Excel's in-product function surface |
| borrowing rule | The (undocumented) choice of which month's length to borrow when a `"MD"` day subtraction goes negative |

## Sources

- Microsoft 365 support, **DATEDIF function** —
  <https://support.microsoft.com/en-us/office/datedif-function-25dba1a4-2812-480b-84dd-8b32a451b35c>.
  Source of the syntax, the six-unit table quoted verbatim above, the Lotus 1-2-3 rationale, the
  "we don't recommend using the 'MD' argument" warning, the `#NUM!` condition, and the suggestion
  to subtract dates directly instead.
- [FUNC.DATE](FUNC.DATE.md) — the serial-number model this function measures on.
- Handbook call-model chapter [02 coercion and lifting](../model/02-coercion-and-lifting.md).
- `data/functions/FUNC.DATEDIF.json`, `data/presence/FUNC.DATEDIF.json`,
  `data/battery/FUNC.DATEDIF.json`.
