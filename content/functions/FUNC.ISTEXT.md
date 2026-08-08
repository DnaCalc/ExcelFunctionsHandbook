---
schema: efh.function-page/v1
function_id: FUNC.ISTEXT
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
  - Page vocabulary
  - Sources
family: is_predicates_family
role_in_family: The Text-kind test; one half of the family's only exact complementary pair.
---

## What it computes

`ISTEXT(value)` returns `TRUE` when the value it receives has kind `Text`, and `FALSE` for every
other kind.

Microsoft's condition is "Value refers to text."

`Text` in Excel's value universe is a sequence of **UTF-16 code units**, capped at 32,767 units
([the value universe](../model/01-value-universe.md)) — not characters and not bytes. Length is
irrelevant to `ISTEXT`: the empty string is `Text`, so `ISTEXT("")` is `TRUE`, and that is the
edge that separates `ISTEXT` from every intuitive notion of "has something in it".

`ISTEXT` classifies; it does not evaluate content. A cell holding the string `"123"`, the string
`"TRUE"`, or the string `"#N/A"` is `Text` in all three cases, and `ISTEXT` says `TRUE` in all
three — even though a reader, and several other Excel functions, would call those a number, a
logical and an error.

## Arguments

`value` — required, exactly one.

Not converted. Microsoft's remark: "The value arguments of the IS functions are not converted."
`ISTEXT` never attempts a to-text conversion of a non-text value; if it did, it would return
`TRUE` for everything, since almost every value has a text rendering.

The argument is a values position: a reference is resolved before the function runs
(`ArgPreparationProfile::ValuesOnlyPreAdapter`).

## Result and edge cases

Returns a `Logical`, or an array of `Logical` when the argument is an array.

- **The empty string is `TRUE`.** `ISTEXT("")` and `ISTEXT(A1)` where `A1` contains `=""` both
  answer `TRUE`, while `ISBLANK` answers `FALSE` for both. Pairing the two predicates is how you
  tell "genuinely nothing" from "a formula produced nothing".
- **`Empty` is `FALSE`.** An empty cell is the `Empty` kind, not zero-length text.
- **`Logical` is `FALSE`**, and so is `Number` — including numbers displayed with a text-looking
  format.
- **`Error` is `FALSE`, and does not propagate**; `ISTEXT` is one of the declared per-family
  exceptions to the propagation discipline in
  [coercion and lifting](../model/02-coercion-and-lifting.md).
- **Arrays** are classified elementwise into a same-shaped mask.
- **Astral characters and lone surrogates.** Text is counted in UTF-16 code units and truncation
  at the interop boundary was observed to be able to split a surrogate pair, leaving ill-formed
  UTF-16 in a real cell ([the value universe](../model/01-value-universe.md)). Such a value is
  still `Text`; `ISTEXT` has no reason to care, but an implementation that validates UTF-16 on
  entry might.

## Errors

`ISTEXT` returns no error of its own for any value it can be given. The only failure available
to it is **arity**: zero arguments, or two. `ISTEXT()` is expected to be refused at formula entry
rather than evaluated ([the call pipeline](../model/03-call-pipeline.md)); the reference engine,
having no entry-time surface, reports `#VALUE!` for both the too-few and the too-many case.

Microsoft's IS-functions page documents no error return for `ISTEXT`.

## Relationships

- **`ISNONTEXT`** is `ISTEXT`'s exact complement: for every value, exactly one of the two is
  `TRUE`. They are the only pair in the IS family with that property, and the reference engine
  implements the second as the logical negation of the first's predicate.
- **`ISNUMBER`** is *not* the complement, though it is often used as though it were.
  `ISTEXT(TRUE)` and `ISNUMBER(TRUE)` are both `FALSE`.
- **`ISBLANK`** disagrees with `ISTEXT` on exactly one important input, the zero-length string;
  see the table on [`FUNC.ISBLANK`](FUNC.ISBLANK.md).
- **`T(value)`** is the projection that pairs with this predicate: it returns the text unchanged
  if the value is text and the empty string otherwise. `T` is to `ISTEXT` what `N` is to
  `ISNUMBER`.
- **`TEXT`, `VALUE`, `NUMBERVALUE`** are the conversions in both directions; `ISTEXT` is the
  test you run before deciding you need one.
- **`TYPE`** returns `2` for exactly the values `ISTEXT` calls `TRUE`.

## Notes for implementers

1. **Zero-length text is text.** Any short-circuit that treats an empty string as "nothing"
   breaks `ISTEXT` and, with it, the `ISBLANK`/`ISTEXT` diagnostic pair that users rely on to
   find `=""` helper columns.
2. **`ISTEXT` and `ISNONTEXT` must be derived from one predicate**, not written twice. Two
   independent implementations of a complementary pair will eventually disagree on some kind that
   was added later.
3. **Do not validate or normalize text on the way in.** Ill-formed UTF-16 is a reachable state in
   real workbooks; a kind test must survive it rather than raise on it.
4. **Rich values project to text in the common case** (a linked data type's fallback display
   value). Whether that projection is what `ISTEXT` sees is a decision an engine must make
   deliberately — and then measure.

## What has not been checked

No Handbook vector suite exists for `ISTEXT`; `vectors/` publishes nothing for this function. No
Excel-comparison evidence record names `ISTEXT` as a subject — **nobody has checked this
function's answers against Excel inside the Handbook's record.** The reference engine's own
answers appear on the battery panel under their own label; they are not Excel's answers.

The probes that would settle the open questions:

1. **Linked data types.** `ISTEXT` of a stock or geography cell. The core projection is the
   fallback display value, typically text, which predicts `TRUE` — but if the predicate sees the
   rich value rather than the projection, the answer flips, and this is the most likely place in
   the whole IS family for a modern-Excel surprise.
2. **The zero-length-string trio** — a literal `""`, a formula returning `""`, and a cell
   containing only an apostrophe — read through `ISTEXT`, `ISBLANK` and `LEN` together.
3. **A number in a cell formatted as text** (`@` format) versus a number stored as text: only
   one is `Text`, and having both on record removes a recurring support question.
4. **Astral text and a deliberately truncated surrogate pair**, to confirm `ISTEXT` is
   indifferent to well-formedness.
5. **Arity at entry** — whether Excel refuses `=ISTEXT()` at entry or evaluates it.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| `Text` | The UTF-16 code-unit string kind, capped at 32,767 units |
| zero-length text | `Text` of length 0; `TRUE` under `ISTEXT`, `FALSE` under `ISBLANK` |
| complement pair | `ISTEXT` and `ISNONTEXT`: exactly one is `TRUE` for any value |
| core projection | The traditional-gamut value a rich value presents to legacy surfaces |

## Sources

- Microsoft, "IS functions" —
  <https://support.microsoft.com/en-us/office/is-functions-0f2d7971-6019-40a0-a171-f2d869135665>.
  Read for this page: the `ISTEXT` and `ISNONTEXT` rows and the non-conversion remark.
- Handbook, [the value universe](../model/01-value-universe.md) — the `Text` kind, the UTF-16
  code-unit cap, the observed surrogate-splitting truncation, and rich-value core projections.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — to-text as an
  explicit primitive and the inspection functions' exemption from error propagation.
- Handbook, [the call pipeline](../model/03-call-pipeline.md) — values-only preparation and the
  admission boundary.
- `data/functions/FUNC.ISTEXT.json` — identity (`xlfIstext`, code 127), arity, declared axes, as
  projected at OxFunc `473efa3`.
