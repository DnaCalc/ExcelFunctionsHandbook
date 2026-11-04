---
schema: efh.function-page/v1
function_id: FUNC.TEXT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - The locale problem
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
  - Notes
  - Page vocabulary
  - Sources
family: text_fn
role_in_family: The general number-format renderer; the family's full-format engine, of which DOLLAR and FIXED are fixed-format special cases.
---

# TEXT

## What it computes

`TEXT(value, format_text)` applies a number format code to `value` and returns the formatted
rendering as a text string.

The important structural fact is that `format_text` is written in the **same number-format
language as the Format Cells dialog** — the `0`, `#`, `?`, `.`, `,`, `%`, `E+`, `@` placeholder
vocabulary, the four semicolon-separated sections for positive / negative / zero / text, the
`[...]` condition and colour clauses, and the date and time codes. `TEXT` is not a small
formatting helper with its own mini-language; it is the cell-formatting engine exposed as a
function, and Microsoft's page documents it by pointing at the number-format guidance rather
than by listing a closed grammar. Its own note is that it does not support every format code.

The difference from cell formatting is what happens to the result. A formatted cell holds a
number and *displays* it formatted; `TEXT` produces an actual text value. Everything downstream
changes accordingly — sorting, comparison, arithmetic, lookups. Microsoft flags this on the
page: converting numbers to text can make them hard to reference in later calculations. The
common mistake is using `TEXT` to make a number look right when the intent was a cell format.

## The locale problem

`format_text` is a **string interpreted in a language**, and this bites harder than any other
property of the function.

The characters that mean "decimal separator" and "group separator" in a format code are the
ones the workbook's language uses, not fixed `.` and `,`. A format literal that produces
`1.234,56` in one language setting produces something else, or nothing useful, in another —
and the format string is stored in the formula verbatim, so the workbook carries a
language-specific literal across every machine it is opened on. The same is true of the date
and time code letters, which are localized: the code for "year" is not `y` everywhere.

This is not a hypothetical fragility. It is why the number-format guidance Microsoft's `TEXT`
page links documents locale-identifier prefixes in the `[$-...]` form for pinning a format to a
specific language regardless of the host.

Two independent signals in the Handbook's own data agree that this dependence is structural
rather than incidental:

- the projected classification for this entry records a **locale-profile dependency** and a
  composite surface dependency — the only entry among this page's siblings that carries one;
- the reference engine's battery rows for `TEXT` report it as **not dispatchable without a host
  facility**, i.e. the engine cannot evaluate `TEXT` in isolation at all, because the format
  language is not part of the function.

`TEXT` is therefore the mirror image of [NUMBERVALUE](FUNC.NUMBERVALUE.md), which exists
precisely so that the *parsing* direction can escape locale dependence. There is no equivalent
escape hatch in the formatting direction: `TEXT` has no separator arguments, and pinning the
locale means embedding a locale identifier in the format string.

## Arguments

| Argument | Meaning |
|---|---|
| `value` | The value to format. Required. Documented as a numeric value. |
| `format_text` | The number-format code, as a text string. Required. |

Both are required; there is no default format.

`format_text` must be quoted text or a reference to text. A bare `TEXT(A1, 0)` passes the
*number* zero where a format string is expected, and what happens then is a coercion question,
not a formatting one — see [Coercion and lifting](../model/02-coercion-and-lifting.md).

The commonly misunderstood position is `value`: the documentation describes it as numeric, but
number formats have a fourth, text section (`@`). In a four-section format numbers use the other
sections, while a lone `@` applied to a number gives the General display (FN-TEXT-002). What
`TEXT` does with a text `value` under a format with and without an `@` section is not something
this page can state.

## Result and edge cases

Returns `Text`.

Open rather than settled, because Microsoft's page does not address them:

- **Text `value`.** Passed through, formatted by the `@` section, or `#VALUE!` — unverified.
- **Empty `format_text`.** An empty format string is legal in the Format Cells dialog, where it
  hides the value; whether `TEXT(1, "")` returns the empty string is unverified.
- **Logical `value`.** Whether `TRUE` coerces to 1 and formats, or renders as `TRUE`.
- **Unsupported codes.** The page states that not all format codes are supported, but does not
  say what an unsupported code does — error, ignore, or literal.
- **Rounding.** `TEXT(0.5, "0")` requires a rounding decision at the format boundary, and which
  rule applies (half-up, half-even, or the display rounding the cell formatter uses) is not
  stated. This is the single most numerically consequential unanswered question on the page.

Empty, missing and error arguments follow the shared call model. The implementing modules named
in the presence projection carry an open upstream defect stream on conversion, text/date
handling, array lifting and coercion (`BUG-FUNC-028`), so array-shaped arguments are a known
soft spot.

## Errors

Microsoft's `TEXT` page carries no error table. The reachable errors are the shared ones: an
error value in either argument propagates, and a value that cannot convert to the expected kind
surfaces `#VALUE!` under the shared coercion rules. Whether an invalid or unsupported
`format_text` produces `#VALUE!` is unverified and is on the probe list.

## Relationships

- **`DOLLAR` and `FIXED`** are `TEXT` with the format fixed in advance — currency formatting
  with a decimal count, and fixed-decimal formatting with an optional comma switch. All three
  share an implementing module in the reference engine. Reach for them when the format is
  exactly what they do; reach for `TEXT` when it is not. The format is not the only difference:
  on an amount that rounds to zero `DOLLAR` keeps the minus sign, while `TEXT` and `FIXED` drop it
  (FN-DOLLAR-001).
- **`VALUE`** is the inverse direction, and **[NUMBERVALUE](FUNC.NUMBERVALUE.md)** is the
  locale-independent inverse. The asymmetry is worth stating: the parse direction has an
  explicit-separator escape hatch, the format direction does not.
- **[T](FUNC.T.md)** is the function readers confuse with this one on the strength of its
  catalogue description. `T` is a type filter that returns `""` for a number; `TEXT` renders a
  number. They have nothing in common.
- **`TEXTJOIN`, `CONCAT`, `&`** consume `TEXT` output constantly — building a sentence around a
  formatted figure is the function's most common use.
- **Cell number formatting** is the alternative that is usually correct: if the goal is
  appearance rather than a text value, format the cell and leave the number a number.
- `TEXT` is not a Compatibility-category function and is not superseded.

## Notes for implementers

`TEXT` cannot be implemented as a pure kernel. The format language is host state — the
workbook's language settings decide what the separator and date-code characters mean — so the
function needs a locale profile threaded in, and any test vector for it is incomplete without
recording which profile was active. The reference engine's own inability to dispatch `TEXT`
without a host facility is that fact showing up in practice, not a gap in the engine.

The rounding rule at the format boundary needs to be established before anything else, because
it is the part that produces silently wrong numbers rather than visibly wrong strings.
Rendering is the easy half of this function; deciding the last digit is the hard half.

Section selection — positive / negative / zero / text — is a per-format parse, not a per-value
branch, and the number of sections present changes the meaning of the ones that are there. That
parse has to happen once per format string, not once per value, or performance and semantics
both suffer.

## What has not been checked

No Handbook vector suite exists for `TEXT`, and no Excel-comparison evidence record names it.
Apart from the readings in the notes below, nobody has checked this function against Excel
within the Handbook's record. Two structural
facts on this page do come from the Handbook's own projected data — the locale-profile
dependency in the classification, and the not-dispatchable battery outcome — and neither is an
observation of Excel. Apart from those notes, everything about what a given format code produces
is Microsoft's documentation or is open.

`TEXT` is also the function on this list where the Handbook is furthest from being able to publish
a suite, because the reference engine cannot evaluate it standalone. Any comparison suite for
`TEXT` needs the host facility built first; that is a prerequisite, not a probe.

Inputs worth probing first, once it can be evaluated at all:

1. **Rounding at the format boundary**: `TEXT(0.5, "0")`, `TEXT(1.5, "0")`, `TEXT(2.5, "0")`,
   `TEXT(1.005, "0.00")`. Four cells that decide the rounding rule and expose whether the
   binary representation of the input leaks through.
2. **The same format string under two workbook languages** — an English and a German
   installation formatting `1234.5` with `"#,##0.00"`. This is the locale claim itself, and it
   is unverified in the Handbook.
3. **A locale-pinned format**, `TEXT(TODAY(), "[$-409]mmmm")`, on both machines, to confirm the
   escape mechanism works as the number-format guidance describes. On one machine tagged formats
   work, with names that differ from common locale data (FN-TEXT-001).
4. **Text `value`**: `TEXT("abc", "0")` and `TEXT("abc", "@")`.
5. **Empty and malformed `format_text`**: `TEXT(1, "")`, `TEXT(1, "zzz")`, `TEXT(1, "0;0;0;0;0")`
   with a fifth section.
6. **Section selection**: `TEXT(-1, "0.00;(0.00);-;@")` across a positive, negative, zero and
   text value. For numbers, `@` is the text section and the other sections apply (FN-TEXT-002).
7. **Extreme values**: `TEXT(1E308, "0")` and `TEXT(1E-308, "0.00")`, where the rendering
   length and the format interact.
8. **Array `value`**, given the open `BUG-FUNC-028` stream on this module.
9. **Earlier Japanese eras**; FN-TEXT-001 reads Heisei and Reiwa only.
10. **Formats mixing `@` with digit placeholders**, which FN-TEXT-002 leaves open.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `TEXT` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-DOLLAR-001: DOLLAR keeps the minus sign on an amount that rounds to zero; FIXED and TEXT drop it

*Quirk; measured; recorded 2026-11-04.* Applies to DOLLAR, FIXED, TEXT.

-0.005 rounded to no decimals is zero. DOLLAR(-0.005,0) shows that as -R0 in en-ZA settings,
keeping the sign of the input, while FIXED(-0.005,0), TEXT(-0.005,0) and TEXT(-0.005,"0") show
0.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=DOLLAR(-0.005,0)` | as typed | `-R0` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=FIXED(-0.005,0)` | as typed | `0` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=TEXT(-0.005,0)` | as typed | `0` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=TEXT(-0.005,"0")` | as typed | `0` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Four recorded readings of the same value.

Evidence:

- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheets qt169 to qt172 at `2d0eddcac8` — recorded readings
- DnaCalc `apps/dna-onecalc/tests/acceptance.rs`, recorded_dollar_retains_a_negative_sign_after_rounding at `2d0eddcac8` — DOLLAR readings pinned
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt172, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt171, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt169, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt170, row 20, column 10 at `2d0eddcac8` (readings)

### FN-TEXT-001: Locale-tagged month, weekday and era names in TEXT follow their own table

*Quirk; measured; recorded 2026-11-04.*

With a locale tag in the format, TEXT's names differ from common locale data. [$-407] (German)
gives mmm for March as Mrz, while mmmm gives März. [$-411] (Japanese) gives mmm as a bare
number, 1, while mmmm gives 1月. [$-419] (Russian) capitalises mmmm (Январь) but not mmm (янв),
and capitalises ddd (Ср) but not dddd. The first year of the Reiwa era prints as 1, not 元年; g,
gg and ggg give R, 令 and 令和, switching on 2019-05-01.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=TEXT(DATE(2023,3,15),"[$-407]mmm")` | as typed | `Mrz` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=TEXT(DATE(2023,1,15),"[$-411]mmm")` | as typed | `1` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=TEXT(DATE(2019,5,1),"[$-411]ggge")` | as typed | `令和1` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): 170 recorded TEXT readings on the fn3_text sheet.

- Open: Era readings cover Heisei and Reiwa only. Earlier Japanese eras have not been read.

Evidence:

- DnaCalc `apps/dna-onecalc/tests/acceptance.rs`, locale_tagged_date_names_follow_the_retained_fn3_readings at `f4930602d7` — recorded readings pinned
- DnaCalc `crates/calc-func/tests/captures/fn3_questions/oracle.capture.json`, sheet fn3_text, row 6, column 4 at `eceb809985` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn3_questions/oracle.capture.json`, sheet fn3_text, row 78, column 4 at `eceb809985` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn3_questions/oracle.capture.json`, sheet fn3_text, row 164, column 4 at `eceb809985` (readings)

### FN-TEXT-002: In TEXT, an @ applied to a number gives the General display, not the full number

*Quirk; measured; recorded 2026-11-04.*

A format of @ or @! on a number gives what General gives, not the digits of the number.
TEXT(123456789012345,"@") is 1.23457E+14 and TEXT(1234567890.5,"@") is 1234567891. A quoted @
is a literal: TEXT(x,"""@""") is @ for a positive x, and -@ for a negative one, because the
ordinary sign rule applies. In a format with four sections, @ is the text section and numbers
use the others: TEXT(123456789012345,"0;0;0;@") shows all 15 digits, and
TEXT(-123456789012345,"0;0;0;@") shows them with no minus sign, because the second section has
none.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=TEXT(123456789012345,"@")` | as typed | `1.23457E+14` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=TEXT(123456789012345,"General")` | as typed | `1.23457E+14` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=TEXT(1234567890.5,"@")` | as typed | `1234567891` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=TEXT(123456789012345,"""@""")` | as typed | `@` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=TEXT(-123456789012345,"""@""")` | as typed | `-@` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=TEXT(123456789012345,"0;0;0;@")` | as typed | `123456789012345` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=TEXT(-123456789012345,"0;0;0;@")` | as typed | `123456789012345` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): 30 recorded readings: six numbers against the formats @, General, quoted @,
@! and 0;0;0;@. @ and @! equalled General for all six numbers.

- Open: Formats mixing @ with digit placeholders have not been read.

Evidence:

- DnaCalc `apps/dna-onecalc/tests/acceptance.rs`, delivered_fn4_numeric_text_placeholders_use_general_display at `179455f24c` — recorded answers pinned
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheets qt001 to qt030 at `2d0eddcac8` — 30 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt001, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt002, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt011, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt003, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt008, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt005, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt010, row 20, column 10 at `2d0eddcac8` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| format code | The number-format language `format_text` is written in — the Format Cells vocabulary |
| section | One of the up-to-four semicolon-separated parts of a format code (positive/negative/zero/text) |
| locale profile | The host language state that decides what the format code characters mean |
| not dispatchable | The reference engine cannot evaluate this function without a host facility |

## Sources

- Microsoft, "TEXT function" —
  <https://support.microsoft.com/en-us/office/text-function-20d5ac4d-7b94-49fd-bb38-93d29371225c>
  (signature, argument meanings, the format-code guidance it links, the note that not all
  format codes are supported, and the warning that the result is text).
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — to-text conversion
  stated as outline-level, and the locale-dependent surface of number recognition flagged there
  as an open area.
- Handbook, [The value universe](../model/01-value-universe.md) — dates and currency as numbers
  wearing formats, and the presentation-hint model that keeps formatting out of value identity.
- Handbook projections `data/functions/FUNC.TEXT.json` (classification: locale-profile
  dependency, composite surface dependency), `data/presence/FUNC.TEXT.json` (implementing
  modules and the `BUG-FUNC-028` defect stream), and `data/battery/FUNC.TEXT.json` (rows
  reported as not dispatchable without a host facility).
