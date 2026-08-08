---
schema: efh.function-page/v1
function_id: FUNC.DATEVALUE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: DATEVALUE function"
    locator: "https://support.microsoft.com/en-us/office/datevalue-function-df8b07d4-7761-4a93-bc33-b7471bbff252"
    role: "documented argument, the omitted-year rule, the ignored-time rule, the date range and the #VALUE! condition"
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
role_in_family: "The text-to-serial parser for dates; the family's locale-dependent entry point and
  the only member whose answer depends on state outside its arguments."
---

## What it computes

`DATEVALUE` converts text that reads as a date into the corresponding date serial. Microsoft's
description of the argument is worth reading literally: it accepts "text that represents a date in
an Excel date format" — that is, the function's domain is defined by *Excel's* date formats, not by
any fixed grammar. `"1/30/2008"` and `"30-Jan-2008"` are the documented examples.

That phrasing is the whole story of this function. Its domain is:

- **locale-dependent** — whether `"3/4/2024"` is 3 April or 4 March depends on the machine's date
  settings, and the reference engine's own classification marks this function as carrying a locale
  profile, unlike the rest of the date category;
- **clock-dependent** — Microsoft documents that when the year is missing from `date_text`, the
  function "uses the current year from your computer's system clock". A parse of `"3/4"` therefore
  returns a different answer in different years, which makes `DATEVALUE` non-deterministic in a way
  the category's other conversions are not;
- **format-set-dependent** — the accepted forms are those Excel would accept when typing into a
  cell, a set that is not enumerated in the documentation.

Together, those three make `DATEVALUE` the least specifiable function in this assignment. The
mathematics is trivial; the specification is the problem.

Microsoft also documents two firm rules:

- **Time information in `date_text` is ignored.** `DATEVALUE("1/1/2024 13:00")` yields a whole
  serial; the fraction is discarded. [TIMEVALUE](FUNC.TIMEVALUE.md) is the mirror image.
- **The result must fall in range.** Under the default Windows date system, the date must lie
  between 1 January 1900 and 31 December 9999; outside that, `#VALUE!`.

## Arguments

`DATEVALUE(date_text)` — one required argument.

**date_text** — "text that represents a date in an Excel date format, or a reference to a cell that
contains text that represents a date in an Excel date format".

Two argument-position subtleties matter more here than the singular argument count suggests:

1. **A cell that already contains a real date is the wrong input.** If the cell holds a date, it
   holds a number, and `DATEVALUE` is being asked to parse a number. The function exists for text
   that Excel did *not* convert on entry — typically imported data.
2. **The year-omitted case is a live behaviour, not a degenerate one.** Microsoft documents it
   explicitly, and it is the reason a workbook built in one year can produce different values when
   recalculated in the next.

## Result and edge cases

Returns a `Number`: a whole date serial with no time component.

- **Leading and trailing whitespace, separators, and month-name spellings** are all part of the
  undocumented accepted set. The Handbook's
  [coercion chapter](../model/02-coercion-and-lifting.md) already flags the broader text-to-number
  recognizer as an open area with a locale-dependent surface that has been probed only at the
  edges; `DATEVALUE` is where that open area is most exposed.
- **Two-digit years** have a pivot convention (some window mapping "24" to 2024 rather than 1924)
  that this page does not state, because the documentation does not and nobody has checked.
- **A date before 1 January 1900** is documented as `#VALUE!` rather than `#NUM!` — worth noting,
  because out-of-range *serials* elsewhere in the category give `#NUM!`. Here the failure is a
  parse failure, and it takes the parse failure's error value.
- **The reference-engine battery** on this page reports this function as not dispatchable in the
  battery harness — it requires a host facility (a locale profile) that the harness does not
  provide. That is an honest structural fact about the function, not a gap in the data: a parser
  whose grammar is supplied by the host cannot be exercised by a host-free runner.

## Errors

As documented:

- `#VALUE!` when `date_text` does not represent a date Excel can parse, or when the resulting date
  falls outside the representable range.

An error value passed in propagates in the ordinary way.

## Relationships

- **[DATE](FUNC.DATE.md)** — the deterministic alternative. Every piece of advice in this category
  amounts to "use `DATE`, not text", and this function is why the advice exists.
- **[TIMEVALUE](FUNC.TIMEVALUE.md)** — the mirror: parses the time part and ignores the date part,
  where `DATEVALUE` parses the date part and ignores the time part. Composing them —
  `DATEVALUE(t) + TIMEVALUE(t)` — is the documented-behaviour way to parse a full timestamp from
  one string.
- **[DAYS](FUNC.DAYS.md)** — documented to route *its* text arguments through `DATEVALUE`, which
  imports this function's locale dependence into an otherwise numeric function.
- **The `VALUE` function** — the general text-to-number conversion, which also accepts date text in
  practice; the relationship between the two accepted sets is not documented and not checked here.
- **Confusable.** `DATEVALUE` and `DATE` are interchanged constantly in advice, and they have
  nothing in common but a name and a return kind.

## Notes for implementers

1. **The grammar is host state, not a constant.** An implementation must take the locale (or an
   explicit format profile) as an input. A hard-coded `MM/DD/YYYY` parser is not an implementation
   of this function; it is an implementation of one of its configurations.
2. **The system clock is an input too.** The year-omitted rule makes this function
   time-dependent — a property it shares only with [NOW](FUNC.NOW.md) and [TODAY](FUNC.TODAY.md) in
   this category, and unlike them it is not declared volatile. A cached result of `DATEVALUE("3/4")`
   can therefore become quietly wrong at a year boundary.
3. **Fail loudly on ambiguity.** Where the configured format admits two readings, there is no
   correct silent choice.
4. **Do not attempt an `excel-bitexact` claim from a grammar you inferred.** Until the accepted set
   is characterized under a named locale, an implementation can only state which forms it accepts,
   not which forms Excel accepts.

## What has not been checked

No Handbook vector suite exists for `DATEVALUE`, and no Excel-comparison evidence is recorded. The
reference-engine battery cannot even call it without a host locale facility. Nothing about this
function's accepted set has been established here. The probes that would begin to fix that:

- **A grammar census under one named locale** — ISO `2024-03-04`, `3/4/2024`, `4-Mar-2024`,
  `March 4, 2024`, `4 March 2024`, `Mar 4`, `2024/3/4`, each with and without surrounding
  whitespace. This is the experiment that turns "Excel date format" from a phrase into a
  specification, and it has to be repeated per locale.
- **The same census under a second locale** with a different date order, which is the only way to
  separate grammar from locale.
- **Two-digit years** — `"1/1/29"` and `"1/1/30"`, to locate the pivot.
- **The year-omitted rule** — `"3/4"`, evaluated with the system clock set to two different years.
- **The range boundary** — `"12/31/1899"` and `"1/1/1900"`, confirming `#VALUE!` rather than
  `#NUM!`.
- **Text carrying a time** — confirming the documented discard, and checking whether the discard is
  a truncation or a parse-and-drop.
- **A 1904-system workbook**, where the documented lower bound moves.

Until a locale-scoped census exists, this Handbook can describe `DATEVALUE`'s contract but cannot
say what it accepts, and this page does not pretend otherwise.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| Excel date format | The (unenumerated) set of text forms Excel recognizes as dates; locale-dependent |
| locale profile | The host-supplied settings that determine date order, separators and month names |
| year-omitted rule | The documented behaviour of taking the year from the system clock when text has none |
| not dispatchable | Battery outcome meaning the harness cannot call the function without a host facility |

## Sources

- Microsoft 365 support, **DATEVALUE function** —
  <https://support.microsoft.com/en-us/office/datevalue-function-df8b07d4-7761-4a93-bc33-b7471bbff252>.
  Source of the argument description, the documented date range, the year-omitted rule, the
  ignored-time rule and the `#VALUE!` condition.
- [FUNC.DATE](FUNC.DATE.md) — the serial-number model and the 1900/1904 systems.
- Handbook call-model chapter [02 coercion and lifting](../model/02-coercion-and-lifting.md), whose
  open question about the locale-dependent text recognizer this function embodies.
- `data/functions/FUNC.DATEVALUE.json`, `data/presence/FUNC.DATEVALUE.json`,
  `data/battery/FUNC.DATEVALUE.json`.
