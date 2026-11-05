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
  - Notes
  - Page vocabulary
  - Sources
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
- **Two-digit years** have a pivot convention that the documentation does not state. Through
  `VALUE`, in en-ZA year-month-day order, FN-VALUE-002 reads "24/2/1" as 2024 and "1/2/1" as 2001.
  Where the window turns over has not been read.
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
  practice; the relationship between the two accepted sets is not documented, and it is checked
  here only for digit scripts (FN-VALUE-001) and the digits of year, month and day (FN-VALUE-002).
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

No Handbook vector suite exists for `DATEVALUE`. The Excel readings on this page are in
FN-VALUE-001 and FN-VALUE-002, under en-ZA settings. The
reference-engine battery cannot even call it without a host locale facility. Little of this
function's accepted set has been established here: four digit scripts in a year-month-day date,
and the limit of two digits on month and day. The probes that would extend that:

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
- **Other digit blocks**, such as Bengali, Tibetan and Khmer, and whether the set of digits depends
  on the Windows or workbook locale, both unread in FN-VALUE-001.
- **Three-digit years**, unread in FN-VALUE-002.

Until a locale-scoped census exists, this Handbook can describe `DATEVALUE`'s contract but cannot
say in full what it accepts, and this page does not pretend otherwise.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `DATEVALUE` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-VALUE-001: Text to number reads Arabic, Persian, Devanagari and Thai digits, but not fullwidth digits

*Quirk; measured; recorded 2026-11-04.* Applies to DATEVALUE, SQRT, VALUE.

In the recorded en-ZA settings, converting text to a number accepts Arabic-Indic (U+0660 to
U+0669), extended Arabic-Indic (U+06F0 to U+06F9), Devanagari (U+0966 to U+096F) and Thai
(U+0E50 to U+0E59) digits. They work alone, in runs, with a decimal point, in the exponent,
with a sign or percent, and mixed with ASCII digits or with each other: VALUE("1٢3") is 123,
and SQRT of a four-digit string using four scripts is the root of 1234. Fullwidth digits
(U+FF10 to U+FF19) give #VALUE! alone or mixed. DATEVALUE reads the four accepted scripts in a
date written year-month-day.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=VALUE("١")` | as typed | `1` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("١٢٣")` | as typed | `123` (`0x405ec00000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("1٢3")` | as typed | `123` (`0x405ec00000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("١e2")` | as typed | `100` (`0x4059000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("१")` | as typed | `1` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("๑")` | as typed | `1` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("１")` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("1２3")` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=SQRT("۱٢३๔")` | as typed | `35.12833614050059` (`0x4041906d51932c26`) | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `=DATEVALUE("٢٠٢٤-٠١-٠٢")` | as typed | `45293` (`0x40e61da000000000`) | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |

Basis (measured): 66 recorded VALUE readings (all ten digits of each of the five scripts, then
composed forms), 10 SQRT and 4 DATEVALUE readings.

Inferred, not measured: That other text-to-number routes, such as arithmetic operators, treat
digits the same way is inferred from VALUE and SQRT agreeing.

- Open: Other digit blocks (Bengali, Tibetan, Khmer and so on) have not been read.
- Open: Whether the set depends on the Windows or workbook locale has not been read.

Evidence:

- DnaCalc `apps/dna-onecalc/tests/acceptance.rs`, delivered_fn4_digit_scripts_and_numeric_date_widths at `55697ab4f7` — recorded answers pinned
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheets qt031 to qt096 at `2d0eddcac8` — 66 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/SQRT.reading.json` at `2d0eddcac8` — 10 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/DATEVALUE.reading.json` at `2d0eddcac8` — 6 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt042, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt081, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt084, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt088, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt062, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt072, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt032, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt093, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/SQRT.reading.json`, fn7-SQRT-digits-6 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/DATEVALUE.reading.json`, fn7-DATEVALUE-digits-1 at `2d0eddcac8` (readings)

### FN-VALUE-002: A date as text takes at most two digits for month and day

*Quirk; measured; recorded 2026-11-04.* Applies to DATEVALUE, VALUE.

In a three-part numeric date, leading zeros are not free. VALUE("2024/02/01") is 45323, but
VALUE("2024/02/001") and VALUE("2024/002/01") are #VALUE!, and so is any month or day of three
or more digits in the 45 forms read. A year of one, two or four digits is read: "1/2/1" and
"1/02/01" are 2001-02-01, "24/2/1" is 2024-02-01, while "0001/2/1" is #VALUE!. Month 0 and day
0 give #VALUE!. The same limit holds with hyphens: "2024-02-001" through VALUE, and
"2024-002-01" and "2024-02-001" through DATEVALUE, are #VALUE!. The settings of this run are
en-ZA: decimal point, space as thousands separator, year-month-day date order.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=VALUE("2024/02/01")` | as typed | `45323` (`0x40e6216000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("2024/02/001")` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("2024/002/01")` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("1/2/1")` | as typed | `36923` (`0x40e2076000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("24/2/1")` | as typed | `45323` (`0x40e6216000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("0001/2/1")` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("2024/02/0")` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("2024-02-001")` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=DATEVALUE("2024-002-01")` | as typed | `#VALUE!` | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |

Basis (measured): 45 recorded VALUE readings covering year, month and day tokens of one to five
digits, and 2 DATEVALUE readings.

- Open: Three-digit years were not read.
- Open: Other date orders and locales were not read.

Evidence:

- DnaCalc `apps/dna-onecalc/tests/acceptance.rs`, delivered_fn4_digit_scripts_and_numeric_date_widths at `55697ab4f7` — recorded answers pinned
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-list-controls.capture.json`, sheets ql041 to ql085 at `2d0eddcac8` — 45 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-list-controls.capture.json`, sheet ql078, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-list-controls.capture.json`, sheet ql079, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-list-controls.capture.json`, sheet ql081, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-list-controls.capture.json`, sheet ql041, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-list-controls.capture.json`, sheet ql065, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-list-controls.capture.json`, sheet ql057, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-list-controls.capture.json`, sheet ql084, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-list-controls.capture.json`, sheet ql085, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/DATEVALUE.reading.json`, fn7-DATEVALUE-digits-5 at `2d0eddcac8` (readings)

<!-- efh:notes end -->

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
