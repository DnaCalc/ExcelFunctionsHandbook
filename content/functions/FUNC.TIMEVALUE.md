---
schema: efh.function-page/v1
function_id: FUNC.TIMEVALUE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: TIMEVALUE function"
    locator: "https://support.microsoft.com/en-us/office/timevalue-function-0b615c12-33d8-4431-bf3d-f3eb6d186645"
    role: "documented argument, the discarded date part, and the #VALUE! condition"
  - work: "Microsoft 365 support: DATEVALUE function"
    locator: "https://support.microsoft.com/en-us/office/datevalue-function-df8b07d4-7761-4a93-bc33-b7471bbff252"
    role: "the mirror-image parser, whose documented rules frame this one"
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
role_in_family: "The text-to-fraction parser for times; the mirror image of DATEVALUE and equally
  locale-dependent."
---

## What it computes

`TIMEVALUE` converts text that reads as a time into the fraction of a day it denotes — the same
kind of value [TIME](FUNC.TIME.md) constructs from three numbers, and the fractional counterpart to
what [DATEVALUE](FUNC.DATEVALUE.md) produces from date text.

It is the exact mirror of `DATEVALUE`: where `DATEVALUE` parses the date part of a string and
discards any time, `TIMEVALUE` parses the time part and discards any date. `TIMEVALUE("1/1/2024
13:00")` is 13:00 as a fraction, with the date thrown away.

The result is therefore always in [0, 1), and the caveats about representability on
[TIME](FUNC.TIME.md) apply unchanged: 1/86400 is not exactly representable, so a parsed time is
generally a nearby double rather than the exact value.

As with `DATEVALUE`, the mathematics is trivial and the **specification is the problem**. The
accepted set of time strings is whatever Excel accepts when typing into a cell, which depends on
the machine's locale: 24-hour versus 12-hour forms, the AM/PM markers and their spellings and
spacing, the time separator character, and whether seconds and fractional seconds are permitted are
all locale-scoped and none of them are enumerated in the documentation. The reference engine's own
classification marks this function — with `DATEVALUE`, and nothing else in the date category — as
carrying a locale profile.

## Arguments

`TIMEVALUE(time_text)` — one required argument.

**time_text** — text representing a time, or a reference to a cell containing such text. The
documented examples are of the `"6:45 PM"` and `"18:45"` shape.

The misunderstood case is the same one `DATEVALUE` has: passing a cell that already holds a real
time. A cell holding a time holds a number, and this function exists for text that Excel did *not*
convert on entry — typically imported or concatenated data.

## Result and edge cases

Returns a `Number` in [0, 1).

- **Any date part is discarded**, so a full timestamp string yields only its time.
- **Values at or above 24 hours** have no representation in the result's range. Whether
  `TIMEVALUE("25:00")` errors, wraps to 01:00 (as [TIME](FUNC.TIME.md)'s documented modulo rule
  would), or is simply not accepted by the parser is **not documented and not checked here**. It is
  the most interesting open question about this function, because the two plausible answers come
  from two different parts of the system: the parser's grammar and the constructor's arithmetic.
- **Fractional seconds** — whether `"12:00:00.5"` parses, and whether the fraction survives, is
  undocumented and unverified.
- **Whitespace, AM/PM spelling and separators** are part of the same undocumented, locale-scoped
  accepted set that `DATEVALUE` has.
- **The reference-engine battery** on this page reports the function as not dispatchable: the
  harness cannot call it without a host locale facility. That is a structural property of a
  host-grammar parser, not missing data.

## Errors

As documented:

- `#VALUE!` when `time_text` does not represent a valid time.

An error value passed in propagates in the ordinary way. Nothing here overrides the engine rules in
[coercion and lifting](../model/02-coercion-and-lifting.md).

## Relationships

- **[DATEVALUE](FUNC.DATEVALUE.md)** — the mirror. `DATEVALUE(t) + TIMEVALUE(t)` is the standard
  way to turn one timestamp string into one serial, and it works precisely because each function
  discards what the other keeps.
- **[TIME](FUNC.TIME.md)** — the deterministic, locale-free alternative when the components are
  already numbers. Prefer it wherever the input is not literally text.
- **[HOUR](FUNC.HOUR.md), [MINUTE](FUNC.MINUTE.md), [SECOND](FUNC.SECOND.md)** — the decomposition
  applied to whatever `TIMEVALUE` produces; a natural round-trip test.
- **The `VALUE` function** — the general text-to-number conversion, whose accepted set overlaps this
  one in ways the documentation does not describe.

## Notes for implementers

1. **Locale is an input.** A parser with a hard-coded `HH:MM:SS` grammar implements one
   configuration of this function, not the function.
2. **Decide, and document, the ≥24-hour policy.** The `TIME` constructor's documented modulo rule
   is a tempting default, but `TIMEVALUE` is a parser and parsers usually reject rather than wrap.
   Neither behaviour can be claimed as Excel's without observation.
3. **Preserve the discard rule exactly.** Parsing a full timestamp and returning `frac` of the whole
   serial is *not* the same as parsing the time part, once the date part is out of range or
   ambiguous.
4. **Do not claim compatibility from an inferred grammar.** State the forms accepted; do not state
   that they are the forms Excel accepts.

## What has not been checked

No Handbook vector suite exists for `TIMEVALUE`, and no Excel-comparison evidence is recorded. The
battery harness cannot call the function at all without a host locale facility. Nothing about the
accepted set has been established here. The probes worth running first:

- **A grammar census under one named locale** — `"13:45"`, `"1:45 PM"`, `"1:45PM"`, `"1:45 p.m."`,
  `"13:45:30"`, `"13:45:30.25"`, `"13.45"`, each with and without surrounding whitespace. This is
  the experiment that turns the phrase "a valid time" into a specification.
- **The same census under a locale with a different time separator or AM/PM convention.**
- **`"24:00"`, `"25:00"`, `"23:59:60"`** — settles the over-range policy, the single most
  interesting unknown here.
- **`TIMEVALUE("1/1/2024 13:00")`** — confirms the documented date discard.
- **Round trip** — `HOUR`/`MINUTE`/`SECOND` applied to the result, and `TIMEVALUE` applied to text
  produced by formatting a known `TIME` value.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| locale profile | The host-supplied settings determining separators, AM/PM markers and accepted forms |
| discard rule | This function keeps the time part and drops the date part; `DATEVALUE` does the reverse |
| not dispatchable | Battery outcome meaning the harness cannot call the function without a host facility |
| over-range policy | What the parser does with a time of 24 hours or more; undocumented |

## Sources

- Microsoft 365 support, **TIMEVALUE function** —
  <https://support.microsoft.com/en-us/office/timevalue-function-0b615c12-33d8-4431-bf3d-f3eb6d186645>.
- Microsoft 365 support, **DATEVALUE function** —
  <https://support.microsoft.com/en-us/office/datevalue-function-df8b07d4-7761-4a93-bc33-b7471bbff252>.
  Source of the mirror-image documented rules referred to above.
- [FUNC.TIME](FUNC.TIME.md) — the constructor, its documented range and its modulo rule.
- Handbook call-model chapter [02 coercion and lifting](../model/02-coercion-and-lifting.md).
- `data/functions/FUNC.TIMEVALUE.json`, `data/presence/FUNC.TIMEVALUE.json`,
  `data/battery/FUNC.TIMEVALUE.json`.
