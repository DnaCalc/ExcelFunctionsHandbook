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
number formats have a fourth, text section (`@`), which only applies to text input. What `TEXT`
does with a text `value` under a format with and without an `@` section is not something this
page can state.

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
  exactly what they do; reach for `TEXT` when it is not.
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
Nobody has checked this function against Excel within the Handbook's record. Two structural
facts on this page do come from the Handbook's own projected data — the locale-profile
dependency in the classification, and the not-dispatchable battery outcome — and neither is an
observation of Excel. Everything about what a given format code produces is Microsoft's
documentation or is open.

`TEXT` is also the function on this list where the Handbook is furthest from being able to make
a claim, because the reference engine cannot evaluate it standalone. Any comparison suite for
`TEXT` needs the host facility built first; that is a prerequisite, not a probe.

Inputs worth probing first, once it can be evaluated at all:

1. **Rounding at the format boundary**: `TEXT(0.5, "0")`, `TEXT(1.5, "0")`, `TEXT(2.5, "0")`,
   `TEXT(1.005, "0.00")`. Four cells that decide the rounding rule and expose whether the
   binary representation of the input leaks through.
2. **The same format string under two workbook languages** — an English and a German
   installation formatting `1234.5` with `"#,##0.00"`. This is the locale claim itself, and it
   is unverified in the Handbook.
3. **A locale-pinned format**, `TEXT(TODAY(), "[$-409]mmmm")`, on both machines, to confirm the
   escape mechanism works as the number-format guidance describes.
4. **Text `value`**: `TEXT("abc", "0")` and `TEXT("abc", "@")`.
5. **Empty and malformed `format_text`**: `TEXT(1, "")`, `TEXT(1, "zzz")`, `TEXT(1, "0;0;0;0;0")`
   with a fifth section.
6. **Section selection**: `TEXT(-1, "0.00;(0.00);-;@")` across a positive, negative, zero and
   text value.
7. **Extreme values**: `TEXT(1E308, "0")` and `TEXT(1E-308, "0.00")`, where the rendering
   length and the format interact.
8. **Array `value`**, given the open `BUG-FUNC-028` stream on this module.

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
