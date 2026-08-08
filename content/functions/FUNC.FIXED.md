---
schema: efh.function-page/v1
function_id: FUNC.FIXED
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
family: fixed_fn
role_in_family: "The sole member: fixed-decimal number-to-text conversion with optional grouping."
---

## What it computes

`FIXED` does three things in one call, and Microsoft's article states them in order: it rounds
a number to the specified number of decimals, formats it in decimal format using a period and
commas, and returns the result as **text**.

Written out:

1. convert the argument to a number;
2. round to `decimals` places, where a negative `decimals` rounds to the left of the decimal
   point (the article's own example: `FIXED(1234.567, -1)` is `1,230`);
3. render with exactly `decimals` digits after the decimal separator — padding with zeros if
   the value has fewer — and with thousands grouping unless `no_commas` is `TRUE`;
4. return `Text`.

Step 3 is what distinguishes `FIXED` from `ROUND`: `ROUND(1.5, 2)` is the number 1.5, while
`FIXED(1.5, 2)` is the text `1.50`. The trailing zero is the point of the function.

The article speaks of "a period and commas". The entry's axes record a locale dependency, and
the sibling `DOLLAR` article states explicitly that its separators follow language settings.
Whether `FIXED`'s separators likewise follow the workbook or system locale is the natural
reading and is the behaviour the reference engine implements, but the `FIXED` article itself
does not say so, so the Handbook records it as expected rather than documented.

## Arguments

`number` — required, the value to round and convert to text.

`decimals` — optional, the number of digits to the right of the decimal point. **Defaults to
2** per the article. Negative values round to the left of the decimal point.

`no_commas` — optional logical. When `TRUE`, the returned text contains no thousands
separators; commas appear by default. Note the polarity: the argument is a *suppression* flag,
so the default `FALSE` means "do include grouping". This is the argument position readers most
often invert.

The article also records a precision remark worth carrying: Excel numbers hold at most 15
significant digits, but `decimals` may be as large as 127. The two facts together mean a large
`decimals` cannot conjure information — beyond the 15 significant digits available, the extra
places render whatever the binary double actually is, not the decimal literal the user typed.

## Result and edge cases

The return kind is `Text`.

- **Padding is guaranteed.** `FIXED(44.332)` is documented as `44.33`; a value with fewer
  decimals is padded rather than shortened.
- **The result is text, and the article says so in contrast with cell formatting**: formatting
  a cell through the Cells command leaves the value a number, while `FIXED` converts its result
  to text. The same downstream trap applies as for `DOLLAR` — a range scan by `SUM` or
  `AVERAGE` ignores numeric-looking text, while the same text passed as a direct argument
  coerces back to a number ([chapter 02](../model/02-coercion-and-lifting.md)).
- **Negative `decimals` beyond the magnitude of the number** must round the value to zero; how
  the sign of a negative input survives that is not documented and has not been checked here.
- Empty cells, omitted arguments and error inputs follow the shared call model.

## Errors

The article enumerates no error conditions. Reachable errors are the shared ones: an error
value in any argument propagates, and non-numeric text as `number` surfaces `#VALUE!` under the
shared coercion rules.

`decimals` is documented as admitting values up to 127; what happens beyond that, and what
happens for a very large negative `decimals`, is not stated.

## Relationships

- **`DOLLAR`** is the currency-symbol counterpart: same rounding-then-text shape, plus a
  symbol, and without the `no_commas` switch. See [`DOLLAR`](FUNC.DOLLAR.md).
- **`TEXT`** is the general form; anything `FIXED` produces can be produced by `TEXT` with an
  appropriate format string, at the cost of writing the format string.
- **`ROUND`** performs step 2 alone and returns a number. `FIXED` is not a rounding function
  that happens to return text — it is a *rendering* function whose rounding you can specify,
  and the distinction matters because the result cannot be arithmetic on afterwards without
  conversion.
- **`VALUE`** / **`NUMBERVALUE`** invert it, locale permitting.
- Excel's *Fixed Decimal* entry option (Advanced settings) shares the word and has nothing to
  do with this function.

## Notes for implementers

1. **Round once, render deterministically.** Round-then-format and format-with-precision can
   differ at ties and at values that are not exactly representable; the documented behaviour
   has an explicit rounding step, and it should be a real, separate step.
2. **Grouping is locale structure, not a comma.** Group size is not universally three, and the
   separator is not universally a comma. `no_commas` should suppress grouping, whatever the
   grouping convention is.
3. **The 15-significant-digit remark is a real constraint on tests.** A vector suite built from
   decimal literals with more than 15 significant digits is testing the parser, not `FIXED`.
4. The OxFunc reference engine at commit `473efa3` uses a constant default of 2 for `decimals`,
   truncates a non-integer `decimals` toward zero, treats any non-logical `no_commas` as
   "nonzero means TRUE", and maps an empty `number` to zero. Implementation facts about OxFunc
   only.

## What has not been checked

No Handbook vector suite exists for `FIXED`, and no Excel-comparison evidence record takes it
as a subject. `FUNC.FIXED` appears in the group member list of the structural sweep recorded as
`EV-STRUCT-0009`, whose subjects are `FUNC.ARABIC` and `FUNC.DECIMAL`; that record's own reader
warning forbids attributing its figures to any single surface. Nothing here is measured.

As with `DOLLAR`, the Handbook's dispatch battery cannot exercise this entry at all — its
surface dependency is composite, requiring a host facility — so the battery panel records it as
not dispatchable.

Probes that would settle the page:

- `FIXED(1234.567,1)`, `FIXED(1234.567,-1)`, `FIXED(44.332)`, `FIXED(-0.4,0)` and
  `FIXED(1234.567,2,TRUE)` on an en-US host, to pin the documented examples and the `no_commas`
  polarity;
- the same five formulas on a locale using `.` as the grouping separator and `,` as the decimal
  separator — the single probe that decides whether `FIXED` is locale-dependent, which its own
  documentation does not say;
- `FIXED(1,130)` and `FIXED(1,-20)`, the two ends of the documented `decimals` range;
- `FIXED(0.1,20)`, which exposes whether the rendering shows the binary double's true expansion
  or pads with zeros after 15 significant digits;
- half-way ties (`FIXED(2.5,0)`, `FIXED(-2.5,0)`, `FIXED(1.005,2)`) to pin the rounding rule.

## Page vocabulary

| Term | Meaning as used here |
|---|---|
| grouping | Insertion of thousands separators; suppressed by `no_commas` |
| padding | Emitting trailing zeros so the result has exactly `decimals` places |
| composite host facility | An entry whose evaluation needs host state the reference engine cannot supply |

## Sources

- Microsoft, "FIXED function" —
  <https://support.microsoft.com/en-us/office/fixed-function-ffd5723c-324c-45e9-8b96-e41be2a8274a>
  (the round/format/return-as-text statement; `decimals` defaulting to 2 and negative values
  rounding left of the decimal point; `no_commas`; the 15-significant-digit and 127-decimal
  remark; the contrast with cell formatting; the `1,234.6`, `1,230` and `44.33` examples).
  Retrieved for this page.
- Microsoft, "DOLLAR function" — cited only for the explicit statement that the sibling's
  separators follow language settings:
  <https://support.microsoft.com/en-us/office/dollar-function-a6cd05d9-9740-4ad3-a469-8109d18ff611>
- Handbook `content/model/02-coercion-and-lifting.md` (the direct-argument versus range-scan
  asymmetry behind the numbers-as-text trap).
- Handbook `content/evidence/records/EV-STRUCT-0009.json` — cited only to state that `FIXED`
  appears in that run's group member list and is not one of its subjects.
- OxFunc `crates/oxfunc_core/src/functions/fixed_fn.rs` and
  `crates/oxfunc_core/src/locale_format.rs` at commit `473efa3` — read for the reference
  engine's defaults and coercions. Implementation fact only.
