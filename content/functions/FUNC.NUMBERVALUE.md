---
schema: efh.function-page/v1
function_id: FUNC.NUMBERVALUE
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
family: number_regex_translate_family
role_in_family: The explicit-separator text-to-number parser; the locale-independent alternative to VALUE.
---

# NUMBERVALUE

## What it computes

`NUMBERVALUE(text, [decimal_separator], [group_separator])` parses `text` as a number using
the two separator characters you name, instead of the two the host locale would have chosen.

That is the entire point of the function, and it is worth stating sharply: `VALUE` asks the
machine what a decimal point looks like; `NUMBERVALUE` lets the formula say so. A workbook
that reads `"1.234,56"` with `NUMBERVALUE(A1, ",", ".")` gets 1234.56 on every machine on
earth. The same string through `VALUE` gets a different answer, or an error, depending on
where the workbook is opened.

The parse Microsoft documents is not a plain decimal scan. Three rules make it its own grammar:

1. **Whitespace is ignored anywhere in `text`**, including in the middle. `" 3 000 "` parses
   as 3000 even without a group separator.
2. **Group separators are cosmetic before the decimal separator and fatal after it.** They may
   appear anywhere in the integer part — Microsoft's page does not require them to be in
   groups of three — but a group separator occurring after the decimal separator is an error.
3. **Percent signs are trailing and cumulative.** A `%` in `text` divides the result by 100,
   and multiple percent signs compound: `"9%%"` is 9 / 100 / 100 = 0.0009.

The third rule is the one that surprises people. It is not a typo tolerance; it is arithmetic.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `text` | The string to parse. Required. | — |
| `decimal_separator` | The character separating the integer and fractional parts. | The host locale's decimal separator |
| `group_separator` | The character separating groups within the integer part. | The host locale's group separator |

Two documented details about the separator arguments:

- **Only the first character of each is used.** Passing `",."` as a separator is not an error
  and not a two-character separator; it is `,`.
- **Omitting them reintroduces exactly the locale dependence the function exists to remove.**
  `NUMBERVALUE(A1)` is not locale-independent. The Handbook's projected metadata for this
  entry classifies its interface as a locale-default profiled parse, which is the same
  statement from the implementation side: the defaults come from the host profile.

If you are using `NUMBERVALUE` for the reason it was added, supply both separators.

## Result and edge cases

Returns `Number`.

Documented boundary behaviours:

- **Empty `text` returns 0**, not an error and not an empty result. This is a real difference
  from `VALUE`, which does not treat the empty string as zero.
- Whitespace-only `text` follows from the ignore-whitespace rule and the empty-string rule,
  though Microsoft states the two rules separately rather than combining them.

Empty cells, missing arguments and incoming error values follow the shared call model; see
[Coercion and lifting](../model/02-coercion-and-lifting.md). Nothing here is specific to
`NUMBERVALUE` except the empty-string-is-zero rule above, which is a function-level decision
rather than an engine one.

The implementing module named in the Handbook's presence projection for this entry also
carries an open upstream defect stream on conversion, text/date handling, array lifting and
coercion (`BUG-FUNC-028`), so the array behaviour of this function is a known soft spot.

## Errors

As documented on Microsoft's `NUMBERVALUE` page:

| Error | Condition |
|---|---|
| `#VALUE!` | `decimal_separator` and `group_separator` are the same character. |
| `#VALUE!` | A group separator occurs after the decimal separator in `text`. |
| `#VALUE!` | Any argument is not valid — including `text` that does not parse under the given separators. |

The Handbook has not verified this list against Excel, and in particular has not established
whether "not valid" covers cases such as an empty-string separator argument or a separator
that is a digit.

## Relationships

- **`VALUE`** is the locale-following text-to-number conversion. The pair is not
  legacy/modern — `VALUE` is not superseded and Microsoft continues to document both. They
  differ in one axis only: where the separator characters come from. Reach for `NUMBERVALUE`
  whenever the text's format is a property of the *data* rather than of the *machine* — parsed
  files, web sources, fixed feed formats.
- **`TEXT`** is the inverse direction and has the mirror-image locale problem: `TEXT` takes a
  format string that is itself interpreted in a language. There is no `TEXT` counterpart to
  `NUMBERVALUE`'s explicit separators; see [TEXT](FUNC.TEXT.md).
- **Implicit coercion** — what happens when text lands in a numeric argument slot — is a third,
  separate rule set, described in
  [Coercion and lifting](../model/02-coercion-and-lifting.md). It is neither `VALUE` nor
  `NUMBERVALUE`, and the chapter is explicit that Excel's full text-to-number recognizer is
  broader than the pinned baseline.
- `DOLLAR` and `FIXED` produce formatted numeric text that `NUMBERVALUE` can often read back,
  given the right separators.

## Notes for implementers

The locale dependence does not disappear when the separators are supplied — it only shrinks.
Currency symbols, sign conventions and non-ASCII digits are not covered by the two separator
arguments, and Microsoft's page does not say what the parser does with them. An implementation
has to decide, and should record the decision as a decision.

Cumulative percent handling has to be applied after the numeric parse, not as a prefix scan,
and each `%` divides once. Implementations that treat `%` as a flag rather than a count will
be right on `"9%"` and wrong on `"9%%"`.

The "only the first character is used" rule for separators means the separator arguments
cannot be validated by length. They are validated by comparison — the two must differ — and
that comparison is on the first characters, not the whole strings.

## What has not been checked

No Handbook vector suite exists for `NUMBERVALUE`, and no Excel-comparison evidence record
names it. Nobody has checked this function against Excel within the Handbook's record.
Everything above is Microsoft's documented behaviour plus the Handbook's own projected
metadata; no observation of Excel supports it here.

Inputs worth probing first:

1. **`NUMBERVALUE("9%%")`** — the cumulative-percent rule, and the cheapest single probe on
   this page. Then `"%9"` and `"9%%%"`, which Microsoft's page does not address.
2. **Irregular grouping**: `NUMBERVALUE("1,23,4.5", ".", ",")`. Whether the parser enforces
   group sizes at all decides how permissive it really is.
3. **Multi-character separators**: `NUMBERVALUE("1.234,56", ",.", ".,")` should, under the
   first-character rule, be identical to `NUMBERVALUE("1.234,56", ",", ".")`.
4. **The empty-string separator**: `NUMBERVALUE("1.5", "", ",")`. This is exactly the "not
   valid" boundary the documented error list leaves undefined.
5. **Scientific notation, signs, and currency symbols** — `"−1.5E3"` with a Unicode minus,
   `"$1,234.56"`, `"(1,234)"` — none of which Microsoft's page mentions, and each of which a
   real data feed produces.
6. **Whitespace species**: the ignore-whitespace rule says "empty spaces"; whether it covers
   tabs, non-breaking spaces and the assorted Unicode space characters is unstated and matters
   enormously for text pasted from the web.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| decimal separator | The character between integer and fractional parts, named by the caller |
| group separator | The thousands-style separator inside the integer part, named by the caller |
| locale-default profiled parse | The projected interface classification: defaults are taken from the host locale profile |

## Sources

- Microsoft, "NUMBERVALUE function" —
  <https://support.microsoft.com/en-us/office/numbervalue-function-1b05c8cf-2bfa-4437-af70-596c7ea7d879>
  (signature, separator defaults, first-character rule, whitespace rule, cumulative percent
  rule, empty-text-is-zero, and the documented error conditions).
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — the implicit
  text-to-number rules, and the explicit note that Excel's full recognizer is broader than the
  pinned baseline.
- Handbook, [The value universe](../model/01-value-universe.md) — value kinds and error codes.
- Handbook projections `data/functions/FUNC.NUMBERVALUE.json` (interface classification
  `locale_default_profiled_parse`) and `data/presence/FUNC.NUMBERVALUE.json` (implementing
  module and the `BUG-FUNC-028` defect stream).
