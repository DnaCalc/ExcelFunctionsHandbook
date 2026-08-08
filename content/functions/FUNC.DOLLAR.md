---
schema: efh.function-page/v1
function_id: FUNC.DOLLAR
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
family: dollar_fn
role_in_family: "The sole member: currency-formatted number-to-text conversion under the host locale."
---

## What it computes

`DOLLAR` rounds a number to a given number of decimal places and returns it as **text**,
formatted as currency. Microsoft's article names the format applied: `$#,##0.00_);($#,##0.00)`
— thousands separated, two decimals by default, negatives in parentheses — and states that the
currency symbol actually used depends on your language settings.

That last clause is the whole character of this function. The name says dollar; the behaviour
says *local currency*. On a host whose locale profile carries a different currency, `DOLLAR`
produces that currency's symbol, separator and decimal count conventions. The Handbook records
this on the entry's axes as a locale dependency: `DOLLAR` is one of the few text functions
whose result is a function of the environment as well as of its arguments, which is also why
the reference engine cannot evaluate it without a host facility.

Decomposed, the operation is:

1. convert the argument to a number by the ordinary rules;
2. round it to `decimals` places — with negative `decimals` rounding to the left of the decimal
   point, so `-1` rounds to the nearest ten;
3. render the rounded value through the locale's currency format, including symbol placement,
   grouping separator, decimal separator, and the negative-number convention;
4. return the rendering as `Text`.

Step 3 is not a pure function of the arguments, and no argument can override it.

## Arguments

`number` — required. The article describes it as a number, a reference to a cell containing a
number, or a formula that evaluates to a number.

`decimals` — optional, the number of digits to the right of the decimal point. **Defaults to 2
when omitted**, per the article. A negative value rounds the number to the left of the decimal
point.

Two positions that get misread:

- `decimals` controls *rounding*, not just display. `DOLLAR(1234.567, -1)` is not `1234.567`
  displayed coarsely; the value rendered has been rounded first, and the digits are gone.
- Whether a non-integer `decimals` is truncated, rounded, or rejected is not stated in the
  article and has not been checked here.

## Result and edge cases

The return kind is `Text`. That is the most consequential fact on this page, and Microsoft's
article puts a warning box around it: numbers stored as text are a common cause of spreadsheet
errors, because many functions ignore them — `SUM`, `AVERAGE`, `MIN`, `MAX`. The article's own
advice is to use the Format Cells dialog or the Accounting Number Format instead, which keeps
the value numeric.

The Handbook's coercion chapter explains why the trap is asymmetric and therefore hard to
notice: `SUM(DOLLAR(5))` as a **direct argument** coerces the text back to a number and gives
5, while the same text sitting in a cell and reached through a **range scan** is ignored and
contributes 0. The same string, the same function, two answers, decided by how the value
arrived ([chapter 02](../model/02-coercion-and-lifting.md)).

Other boundaries:

- **Zero and negatives** render through the format's negative branch, which is parenthesised in
  the documented US format and may be a leading minus elsewhere. This is a locale-visible
  difference in the *shape* of the output, not only in the symbol.
- **An empty cell** as `number` converts to zero under the ordinary numeric coercion, so
  `DOLLAR` of a blank is a rendering of zero rather than empty text.
- **Very large magnitudes** must eventually exceed what a fixed-decimal rendering can express;
  where that boundary lies, and whether it produces an error, is not documented in the article.

## Errors

The article does not enumerate error conditions. The reachable errors are the shared ones: an
error value in either argument propagates, and non-numeric text as `number` fails to-number
conversion and surfaces `#VALUE!` under the shared coercion rules
([chapter 02](../model/02-coercion-and-lifting.md)).

Because step 3 above involves a host facility, a fourth failure mode exists in principle — a
locale profile that cannot render the requested combination — with no documented
worksheet-visible behaviour. Nothing about it has been observed here.

## Relationships

- **`FIXED`** is the near-twin: same rounding-then-render-as-text shape, no currency symbol,
  and an explicit `no_commas` switch that `DOLLAR` does not have. If you want grouped decimal
  text without a currency symbol, `FIXED` is the function. See [`FIXED`](FUNC.FIXED.md).
- **`TEXT`** is the general case, taking an arbitrary format string; `DOLLAR` is `TEXT` with
  the currency format baked in and the format argument replaced by a decimals count.
- **`VALUE`** and **`NUMBERVALUE`** are the inverse direction, parsing formatted text back to a
  number — and `NUMBERVALUE` exists precisely because the inverse is locale-dependent too.
- **`DOLLARDE` and `DOLLARFR`** share the word but not the job: they convert between fractional
  and decimal *notations* of a price and return numbers, not text. They are unrelated to this
  function despite the name collision, and readers confuse them regularly.
- **Cell number formatting** is the non-function alternative the documentation recommends.

## Notes for implementers

1. **The locale profile is an input.** An implementation that hard-codes `$` and `,` is
   implementing the en-US instance of this function, not the function. State the profile as
   part of the result's scope, exactly as an Excel build and platform would be.
2. **Round before rendering, once.** Rounding inside the formatter and rounding beforehand can
   differ at ties; the documented behaviour is a rounding step with an explicit decimals
   argument, so keep it explicit.
3. **Negative `decimals` is a rounding magnitude, not a formatting flag**, and must still
   render with the format's decimal count — `FIXED(1234.567,-1)` is documented as `1,230`, and
   the analogous `DOLLAR` case is worth pinning by probe rather than by analogy.
4. The OxFunc reference engine at commit `473efa3` takes the decimals default from the locale
   profile's currency-decimals field rather than using a constant 2, truncates a non-integer
   `decimals` toward zero, and maps an empty argument to zero. Those are implementation facts
   about OxFunc; note that the first of them is a visible difference from the documented
   default of 2 on any profile whose currency does not use two decimals.

## What has not been checked

No Handbook vector suite exists for `DOLLAR`, and no Excel-comparison evidence record takes it
as a subject. `FUNC.DOLLAR` appears in the group member list of the structural sweep recorded
as `EV-STRUCT-0009`, but that record's subjects are `FUNC.ARABIC` and `FUNC.DECIMAL` and its
own reader warning forbids attributing the run's figures to any single surface. So nothing here
is measured.

This function additionally cannot be exercised by the Handbook's dispatch battery at all: the
entry's surface dependency is composite — it requires a host facility — so the battery panel on
this page records it as not dispatchable rather than showing outcomes. That is a second,
independent reason the evidence column is empty.

The probes that would settle the page:

- `DOLLAR(1234.567)`, `DOLLAR(1234.567,2)`, `DOLLAR(1234.567,-1)`, `DOLLAR(-1234.567,2)` and
  `DOLLAR(0)` captured on at least three locales — one two-decimal currency, one zero-decimal
  currency (JPY), and one whose negative format is not parenthesised. This settles both the
  default and the negative-branch shape.
- `DOLLAR(1,2.7)` — the non-integer `decimals` rule.
- `DOLLAR("1234.567")` with a locale whose decimal separator is a comma, which tests whether
  the argument's text-to-number parse is locale-sensitive at this call site.
- `SUM` over a range of `DOLLAR` results versus `SUM` of a direct `DOLLAR` call, to demonstrate
  the documented text trap concretely in the Handbook's own vocabulary.

## Page vocabulary

| Term | Meaning as used here |
|---|---|
| locale profile | The host-supplied set of currency symbol, separators and decimal conventions |
| composite host facility | An entry whose evaluation needs host state the reference engine cannot supply |
| number stored as text | A numeric-looking text value, skipped by range scans in aggregates |

## Sources

- Microsoft, "DOLLAR function" —
  <https://support.microsoft.com/en-us/office/dollar-function-a6cd05d9-9740-4ad3-a469-8109d18ff611>
  (the `$#,##0.00_);($#,##0.00)` format and its language-setting dependence; `decimals`
  defaulting to 2; negative `decimals` rounding to the left of the decimal point; the
  numbers-stored-as-text warning and the Format Cells recommendation). Retrieved for this page.
- Handbook `content/model/02-coercion-and-lifting.md` (the direct-argument versus range-scan
  asymmetry that makes the text trap observable).
- Handbook `content/evidence/records/EV-STRUCT-0009.json` — cited only to state that `DOLLAR`
  appears in that run's group member list and is not one of its subjects.
- OxFunc `crates/oxfunc_core/src/functions/dollar_fn.rs` and
  `crates/oxfunc_core/src/locale_format.rs` at commit `473efa3` — read for the reference
  engine's decimals default and coercion behaviour. Implementation fact only.
