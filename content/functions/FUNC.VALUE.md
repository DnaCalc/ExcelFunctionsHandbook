---
schema: efh.function-page/v1
function_id: FUNC.VALUE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — VALUE function"
    locator: "https://support.microsoft.com/en-us/office/value-function-257d0108-07dc-437d-ae1c-bc2d3953d8c2"
    role: "documented signature, description and error conditions"
  - work: "OxFunc — FUNCTION_SLICE_VALUE_CONTRACT_PRELIM.md"
    locator: "docs/function-lane/FUNCTION_SLICE_VALUE_CONTRACT_PRELIM.md"
    role: "upstream locale-parse seam contract and the empirically pinned accept/reject lanes"
episodes: []
body_sections:
  - "What it computes"
  - "Arguments"
  - "Result and edge cases"
  - "Errors"
  - "Relationships"
  - "Notes for implementers"
  - "What has not been checked"
  - "Notes"
  - "Page vocabulary"
  - "Sources"
family: value_fn
role_in_family: "The text-to-number parser; the locale-facing member of the number/format group."
---

## What it computes

`VALUE` parses text into a number.

It is tempting to describe this as "the inverse of `TEXT`", and that is the right intuition, but
the honest description is narrower and stranger: **`VALUE` accepts the text forms that Excel itself
would recognise as a number if you typed them into a cell, under the current locale, and returns
the number they denote.** That is not a fixed grammar. It is a grammar parameterised by the
machine's regional settings.

The forms in play include, at minimum: plain decimals, scientific notation, digit-group separators,
a currency symbol, a percent sign, and date and time text. Each of those brings a locale
dependency:

- the **decimal separator** is `.` in some locales and `,` in others, which means `"1,5"` is one
  and a half in one place and fifteen — or a rejection — in another;
- the **group separator** is the mirror image of that problem, and a space in several locales;
- the **currency symbol** is per-locale, and `VALUE` accepting `"$1.00"` says nothing about whether
  it accepts `"€1,00"`; in the recorded en-ZA settings `"€1"` is read and `"$1"` is not
  (FN-VALUE-003);
- **date text** is the worst case, because `"1/2/2024"` denotes two different days in two locales
  and returns a *serial number*, not the digits you can see.

The Handbook's shared coercion chapter already draws this boundary: it pins the plain-number core
and states that the locale-dependent surface — separators, currency, percent, date and time text —
is an open area. See [coercion and lifting](../model/02-coercion-and-lifting.md). `VALUE` is the
function where that open area is the entire subject matter.

Upstream OxFunc treats `VALUE` the same way, and usefully so: its slice contract routes text
through an explicit locale-profile parser seam rather than a fixed grammar, and records which lanes
were empirically pinned on one host profile — plain decimal text, space-grouped numeric text,
currency text with the local symbol, percent text, and ISO `yyyy-mm-dd` date text were accepted;
slash-style date text and blank or whitespace-only text were rejected — under Excel 16.0 build
19725 with a named channel and host profile. That is an upstream observation on one host, recorded
with its scope, and it is exactly the kind of result that does not generalise: change the locale
and the accept/reject split is expected to move. In the en-ZA settings of FN-VALUE-002,
slash-style `"2024/02/01"` is accepted.

Two structural facts complete the definition. A number arriving as the argument passes through
unchanged. And a successful date-text parse returns the date's **serial number** — a plain
`Number`, formatting-free — which is why `VALUE("2024-01-02")` shows as a large integer unless the
cell is formatted as a date.

## Arguments

`VALUE(text)`

| Argument | Required | Meaning |
|---|---|---|
| `text` | yes | The text to convert to a number. |

One argument. The subtleties are all about what may occupy it:

- **Already-numeric input** passes through. `VALUE(5)` is `5`.
- **Logicals** are the surprising rejection: upstream's contract records logical, blank, missing,
  and unsupported aggregate or reference payloads as `#VALUE!` in its admitted slice — so `VALUE`
  is *stricter* than the general to-number coercion, which maps `TRUE` to `1`. That is a real
  difference between `VALUE(TRUE)` and `SUM(TRUE)`, and it is upstream's recorded behaviour rather
  than a Handbook finding.
- **Leading and trailing whitespace** is ignored by the general to-number rule; whether `VALUE`
  matches that exactly, and whether a whitespace-only string is rejected as upstream records, is
  worth confirming rather than assuming.
- **An empty referenced cell** delivers `Empty`, which is a distinct outcome from an omitted
  argument; see [the value universe](../model/01-value-universe.md).

## Result and edge cases

The return kind is `Number`.

Edges that matter in practice:

- **Percent text divides.** If `"50%"` is accepted, the result is `0.5`, not `50`. A pipeline that
  parses percentages with `VALUE` and then divides by 100 is wrong by a factor of 100 exactly when
  the parse succeeds.
- **Date and time text returns serials.** Time text returns a fraction of a day. `VALUE("12:00")`
  is `0.5` if accepted.
- **Overflow is not infinity.** The shared coercion rule states that text parsing must produce a
  finite number; text denoting an overflow is treated as non-numeric rather than as infinity.
- **Precision is the parser's, not the text's.** `VALUE` of a 30-digit numeral returns the nearest
  double, silently. Round-tripping through `TEXT` and back is lossy in both directions and the
  losses do not cancel.
- **Arrays.** Whether `VALUE` lifts over an array argument is not something this page asserts; the
  general rules are in [coercion and lifting](../model/02-coercion-and-lifting.md), and the
  specific answer for `VALUE` is listed as an open probe below.
- **Errors propagate.**

## Errors

Microsoft's page is the authority on the documented error conditions and is linked below; it was
not retrievable while this entry was written, so this section states expectations rather than
quoting it.

| Error | Status |
|---|---|
| `#VALUE!` when the text is not in a recognised numeric or date/time format | expected; this is the function's characteristic failure |
| `#VALUE!` for logical, blank, and unsupported argument kinds | recorded upstream for its admitted slice; not a Handbook claim, and notably stricter than general to-number coercion |
| any error value, propagated from the argument | follows from the universal error-propagation rule |

Note what *is not* here: there is no error for precision loss, and none for a value that parses but
means something other than what the author intended. `VALUE` fails loudly or succeeds silently.

## Relationships

- `TEXT` — the opposite direction, and the function `VALUE` is usually paired with. `TEXT` is
  equally locale-dependent, in the format-string argument as well as the output, which means a
  `VALUE(TEXT(x, fmt))` round trip is doubly exposed to locale.
- `NUMBERVALUE` — the function you probably want. It takes explicit decimal and group separator
  arguments, which removes the ambient-locale dependency that makes `VALUE` non-portable.
  `NUMBERVALUE` does not supersede `VALUE` in Microsoft's catalogue and both remain current, but
  for any workbook that travels between machines, `NUMBERVALUE` is the safer instrument. Upstream
  notes that `NUMBERVALUE`'s own locale-default lanes are separately blocked and unresolved, so it
  is not a free lunch either.
- `--` (double unary minus) and `+0` are the idiomatic shorthands for text-to-number coercion. They
  go through the *general* to-number rule described in
  [coercion and lifting](../model/02-coercion-and-lifting.md), not through `VALUE`'s parser, so
  they are not interchangeable with `VALUE` — most visibly for logicals.
- `DATEVALUE` and `TIMEVALUE` — the specialised parsers for the date and time lanes that `VALUE`
  also happens to cover. When you know you have a date, they say so in the formula and are easier
  to reason about.
- `VALUETOTEXT` is *not* the inverse of `VALUE` despite the name symmetry; see
  [`VALUETOTEXT`](FUNC.VALUETOTEXT.md), which renders any value as text for display or for
  formula-bar round-tripping.
- `ISNUMBER(VALUE(x))` is the common "does this parse" test, and it inherits every locale question
  on this page.

## Notes for implementers

- **Make the locale an explicit input, not ambient state.** Upstream's design — a locale-profile
  seam queried from the host, with the parser written against the profile — is the right shape.
  A parser that reads the process locale at call time is untestable and irreproducible.
- **Do not use the host language's number parser.** `strtod`, `double.Parse`, and `f64::from_str`
  each accept things Excel does not (hex floats, `inf`, `nan`, underscores in some languages) and
  reject things Excel does (currency symbols, percent, grouped digits). Every one of those
  mismatches is a silent divergence.
- **Reject non-finite parses.** The shared rule is explicit: text that parses to an overflow is
  non-numeric, not infinite.
- **Keep the date lane separate from the number lane**, and be prepared for the two to disagree
  about the same string — `"1.5"` as a number and as a date are both plausible readings, and the
  precedence between them is a real decision that must be recorded.
- **Percent must compose with grouping** rather than being a separate grammar, but it does not
  combine with currency: `"€1%"` is `#VALUE!` in the recorded en-ZA settings (FN-VALUE-003). It
  also counts toward the exponent range check: `"0e-307"` is 0 but `"0e-307%"` is `#VALUE!`
  (FN-ABS-001).
- **Round once.** Parse to the exact decimal value and round to the nearest double a single time;
  parsing digit-by-digit with accumulating multiplication introduces error that no test of small
  numerals will catch. Excel also checks the exponent range when the mantissa is zero, so
  `"0e309"` is `#VALUE!` (FN-ABS-001).

## What has not been checked

There is no Handbook vector suite for `VALUE`, and no Handbook evidence record comparing any
implementation against Excel. Apart from the notes below, nothing on this page is a measurement.
The accept/reject lanes quoted from upstream are scoped to one Excel build, one channel, and one
host profile, and the whole point of this function is that they are expected to move when any of
those change.

`VALUE` is also one of the functions the Handbook's generated battery cannot exercise: the upstream
reference implementation reports it as requiring a host facility, so no dispatched outcome exists
for the standard probe rows. That is a fact about the reference engine's seam, not about Excel.

The probes that would settle the most, in order:

1. **The locale matrix.** Run the same short list of strings — `"1.5"`, `"1,5"`, `"1 234,5"`,
   `"1,234.5"`, `"$1.00"`, `"€1,00"`, `"50%"`, `"1E3"`, `"2024-01-02"`, `"1/2/2024"`, `"12:00"`,
   `" 1 "`, `""` — on at least an `en-US`, a `de-DE`, and a `fr-FR` machine. That single sheet is
   the entire open question, and it is the highest-value probe in this part of the catalogue.
2. **Logicals and blanks.** `VALUE(TRUE)`, `VALUE(A1)` with `A1` empty, and `VALUE(" ")`, to
   confirm or refute upstream's recorded strictness against Excel directly.
3. **Array lifting.** `VALUE({"1","2"})` — one cell, and it determines whether `VALUE` participates
   in dynamic-array pipelines at all.
4. **The date/number precedence.** Strings that are legal under both readings, such as `"1.5"` in a
   locale where `.` is also a date separator.
5. **Precision.** `VALUE` of a 17-significant-digit numeral and of a value near the subnormal
   boundary, compared against a correctly rounded reference. This is where a vector suite would
   have real teeth, because it is checkable without Excel once the vectors exist.
6. **Whitespace and sign forms.** Leading `+`, and non-breaking spaces as group separators.
   Parentheses are read as a negative sign in the recorded en-ZA settings (FN-VALUE-003).
7. **Digit blocks beyond those in FN-VALUE-001** (Bengali, Tibetan, Khmer and so on), and whether
   the set depends on the Windows or workbook locale.
8. **Three-digit years, and other date orders and locales**, which FN-VALUE-002 leaves open.
9. **Why the euro sign is accepted in en-ZA settings and `$` is not**, and the other locales, which
   FN-VALUE-003 leaves open.

Until the locale matrix exists, no statement about which strings `VALUE` accepts should be read as
general.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `VALUE` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-ABS-001: Text to number checks the exponent range even when the mantissa is zero

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.* Applies to ABS, VALUE.

Converting text to a number, measured through ABS and VALUE, rejects a zero mantissa with an
out-of-range exponent: ABS("0e-309") is #VALUE! though the text denotes 0, and ABS("0e-308") is
0. The exponent counts as written, less the fraction digits, less 2 for a percent sign, so
ABS("0.0e309") is 0, ABS("0e309") is #VALUE!, and ABS("0e-307%") is #VALUE!.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `=ABS("0e-309")` | as typed | `#VALUE!` | `0` (`0x0000000000000000`) | Excel build not recorded in this run; W111 bulk captures of the same date (2026-09-29) record Excel 16.0 build 20430, Windows 64-bit, workbook compatibility 2 |
| `=ABS("0e-308")` | as typed | `0` (`0x0000000000000000`) | — | Excel build not recorded in this run; W111 bulk captures of the same date (2026-09-29) record Excel 16.0 build 20430, Windows 64-bit, workbook compatibility 2 |
| `=ABS("0e-307%")` | as typed | `#VALUE!` | — | Excel build not recorded in this run; W111 bulk captures of the same date (2026-09-29) record Excel 16.0 build 20430, Windows 64-bit, workbook compatibility 2 |
| `=ABS("0e309")` | as typed | `#VALUE!` | `0` (`0x0000000000000000`) | Excel build not recorded in this run; W111 bulk captures of the same date (2026-09-29) record Excel 16.0 build 20430, Windows 64-bit, workbook compatibility 2 |
| `=ABS("0.0e309")` | as typed | `0` (`0x0000000000000000`) | — | Excel build not recorded in this run; W111 bulk captures of the same date (2026-09-29) record Excel 16.0 build 20430, Windows 64-bit, workbook compatibility 2 |
| `=VALUE("0e-307")` | as typed | `0` (`0x0000000000000000`) | — | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("0e-307%")` | as typed | `#VALUE!` | — | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("0e309")` | as typed | `#VALUE!` | — | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("0.0e309")` | as typed | `0` (`0x0000000000000000`) | — | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("0e309%")` | as typed | `0` (`0x0000000000000000`) | — | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("0e311%")` | as typed | `#VALUE!` | — | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Correct value: The text denotes zero: a zero mantissa is 0 at any exponent.

Basis (measured): About 120 recorded zero-mantissa cases through ABS, and 72 through VALUE. All
72 VALUE readings fit a limit of -308 to 308 on the exponent as counted here.

Inferred, not measured: The exponent formula is inferred, though it fits every reading; that
functions other than ABS and VALUE share this conversion is inferred.

Evidence:

- DnaCalc `apps/dna-onecalc/tests/acceptance.rs`, recorded_zero_percentage_exponents_are_validated at `344c166787` — recorded readings pinned
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheets qt097 to qt168 at `2d0eddcac8` — 72 recorded VALUE readings
- OxFunc-Evidence `runs/w111-numeric-text-percent-bounds-20260929/outcomes/excel.jsonl.gz`, w111textbounds-00008-abs-boundary-2-0 at `e26b7bddb8` (readings)
- OxFunc-Evidence `runs/w111-numeric-text-percent-bounds-20260929/outcomes/excel.jsonl.gz`, w111textbounds-00012-abs-boundary-3-0 at `e26b7bddb8` (readings)
- OxFunc-Evidence `runs/w111-numeric-text-percent-bounds-20260929/outcomes/excel.jsonl.gz`, w111textbounds-00017-abs-boundary-4-1 at `e26b7bddb8` (readings)
- OxFunc-Evidence `runs/w111-numeric-text-percent-bounds-20260929/outcomes/excel.jsonl.gz`, w111textbounds-00028-abs-boundary-7-0 at `e26b7bddb8` (readings)
- OxFunc-Evidence `runs/w111-numeric-text-percent-bounds-20260929/outcomes/excel.jsonl.gz`, w111textbounds-00108-abs-boundary-27-0 at `e26b7bddb8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt099, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt100, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt107, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt143, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt108, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt112, row 20, column 10 at `2d0eddcac8` (readings)

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

### FN-VALUE-003: VALUE takes one percent sign anywhere around parentheses, and a euro sign but not a dollar

*Quirk; measured; recorded 2026-11-04.*

VALUE reads parentheses as a negative sign and one percent sign in any of these places: "(1)%",
"(1) %", "(1%)", "(%1)" and "%(1)" are all -0.01. A second percent sign, as in "(1%)%", "%(1)%"
and "(1)%%", gives #VALUE!, as does trailing text. The euro sign is read as a currency sign in
the recorded en-ZA settings: "€1", "1€", "€ 1", "€-1", "-€1", "(€1)" and "€(1)" are 1 or -1.
"$1" and "£1" are #VALUE! there. Currency and percent do not combine: "€1%", "(€1)%" and
"(R1)%" are #VALUE!.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=VALUE("(1)%")` | as typed | `-0.01` (`0xbf847ae147ae147b`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("(%1)")` | as typed | `-0.01` (`0xbf847ae147ae147b`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("%(1)")` | as typed | `-0.01` (`0xbf847ae147ae147b`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("(1%)%")` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("€1")` | as typed | `1` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("1€")` | as typed | `1` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("€(1)")` | as typed | `-1` (`0xbff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("€1%")` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("$1")` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("£1")` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): 21 recorded VALUE readings.

- Open: Why the euro sign is accepted in en-ZA settings, and "$" is not, is not known. The other locales have not been read.

Evidence:

- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4.capture.json`, sheets q072 to q092 at `2d0eddcac8` — 21 recorded readings
- DnaCalc `apps/dna-onecalc/tests/acceptance.rs`, recorded_parenthesized_percent_text_keeps_its_suffix at `2d0eddcac8` — the same parenthesised percent forms through INT
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4.capture.json`, sheet q072, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4.capture.json`, sheet q075, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4.capture.json`, sheet q076, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4.capture.json`, sheet q077, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4.capture.json`, sheet q083, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4.capture.json`, sheet q084, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4.capture.json`, sheet q089, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4.capture.json`, sheet q090, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4.capture.json`, sheet q091, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4.capture.json`, sheet q092, row 20, column 10 at `2d0eddcac8` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| locale profile | The named set of separators, symbols and date orders the parser is written against |
| lane | One recognised text form (plain decimal, percent, currency, ISO date, …) |
| serial number | Excel's numeric representation of a date or time; what a date-text parse returns |
| ambient locale | Regional settings read from the machine rather than passed as an argument |
| host facility | An engine seam that must be supplied by the host; why the generated battery cannot dispatch this function |

## Sources

- Microsoft Support, VALUE function —
  <https://support.microsoft.com/en-us/office/value-function-257d0108-07dc-437d-ae1c-bc2d3953d8c2>
  (the authority for signature, description and documented errors; not retrievable while this entry
  was written, so it is cited by reference and nothing here paraphrases it).
- OxFunc `docs/function-lane/FUNCTION_SLICE_VALUE_CONTRACT_PRELIM.md` — the locale-profile parser
  seam, the pinned accept and reject lanes with their Excel build, channel and host-profile scope,
  and the recorded `#VALUE!` for logical, blank, missing and unsupported argument kinds.
  Preliminary by its own statement.
- Handbook `content/model/02-coercion-and-lifting.md` — the general to-number rule, the
  finite-parse requirement, and the explicit statement that the locale-dependent recognition
  surface is an open area.
- Handbook `content/model/01-value-universe.md` — Empty versus Missing, and the `Number` kind.
