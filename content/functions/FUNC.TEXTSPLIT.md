---
schema: efh.function-page/v1
function_id: FUNC.TEXTSPLIT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — TEXTSPLIT function"
    locator: "https://support.microsoft.com/en-us/office/textsplit-function-b1ca414e-4c21-4ca0-b1b7-bdecace8a6e7"
    role: "documented signature, argument meanings, padding behaviour"
  - work: "OxFunc — FUNCTION_SLICE_ARRAY_TEXT_SPLIT_FAMILY_CONTRACT_PRELIM.md"
    locator: "docs/function-lane/FUNCTION_SLICE_ARRAY_TEXT_SPLIT_FAMILY_CONTRACT_PRELIM.md"
    role: "upstream admitted-slice contract and its explicit spill-publication scope boundary"
episodes: []
body_sections:
  - "What it computes"
  - "Arguments"
  - "Result and edge cases"
  - "Errors"
  - "Relationships"
  - "Notes for implementers"
  - "What has not been checked"
  - "Page vocabulary"
  - "Sources"
family: array_text_split_family
role_in_family: "The text-to-array half of the split/render pair: turns one string into a
  two-dimensional array of fields."
---

## What it computes

`TEXTSPLIT` is the inverse of joining. It takes one string and returns a rectangular array of the
pieces between delimiter occurrences.

The rule is a two-level tokenisation, applied outer-first:

1. **Rows.** If `row_delimiter` is supplied, split the text at every occurrence of it, producing
   `r` row-segments in order. If it is omitted, there is exactly one row-segment: the whole text.
2. **Columns.** Split each row-segment independently at every occurrence of `col_delimiter`,
   producing `c_1, c_2, …, c_r` fields.
3. **Rectangle.** The result has `r` rows and `max(c_i)` columns. Row `i` supplies its `c_i` fields
   left to right; the remaining `max(c_i) − c_i` cells of that row are filled with the padding
   value.

Two properties of that rule deserve to be said out loud, because they are the source of most
surprises:

- **Splitting is total, not selective.** Every occurrence is a cut. `n` occurrences of a delimiter
  in a segment produce `n+1` fields, including empty ones at the ends and between adjacent
  delimiters. `"a,,b"` split on `","` is three fields, the middle one empty. `ignore_empty`
  exists precisely to opt out of that.
- **The result is ragged-then-padded, not ragged.** Excel's value universe has no ragged arrays;
  an `Array` is rectangular by definition (see [the value universe](../model/01-value-universe.md)).
  So a text whose rows have different field counts cannot be represented faithfully, and the
  padding value is how the function admits that. The default padding is `#N/A`, which is the
  honest choice: those cells contain no data, and `#N/A` is Excel's word for that.

Both delimiter arguments accept an *array* of delimiters rather than a single one, in which case
any element of the array is a cut point. That makes `TEXTSPLIT(t, {",",";"})` a single-pass split
on either character — which is not the same as splitting on the two-character string `",;"`.

## Arguments

`TEXTSPLIT(text, col_delimiter, [row_delimiter], [ignore_empty], [match_mode], [pad_with])`

| Argument | Required | Meaning (as documented by Microsoft) |
|---|---|---|
| `text` | yes | The string to split. |
| `col_delimiter` | yes | The text marking where to split across columns. May be an array constant. |
| `row_delimiter` | no | The text marking where to split down rows. May be an array constant. Omitted means a single row. |
| `ignore_empty` | no | `TRUE` ignores consecutive delimiters (drops empty fields). Defaults to `FALSE`. |
| `match_mode` | no | `0` (default) case-sensitive delimiter matching; `1` case-insensitive. |
| `pad_with` | no | The value used to fill short rows. Defaults to `#N/A`. |

Two positions are routinely misunderstood.

`col_delimiter` is required even when you only want to split into rows. To split a
newline-separated list into a column you must still supply something for position two; passing an
empty string is not the same as omitting it, and the two behave differently.

`ignore_empty` is a *field* filter, not a *delimiter* filter, and its name invites the wrong
mental model. It does not merge adjacent delimiters before splitting; it removes empty fields from
the result — which also removes leading and trailing empties, and can therefore change how many
columns the rectangle has.

`match_mode` affects only how delimiter occurrences are found. It never changes the case of the
returned fields.

## Result and edge cases

The return kind is `Array` — genuinely, not by lifting. `TEXTSPLIT` is one of the functions whose
kernel produces a grid, so the host-side adaptation step described in
[the call pipeline](../model/03-call-pipeline.md) anchors that grid as a dynamic-array spill. Two
consequences belong to the engine, not to this function, and are covered by that chapter: a
blocked spill region publishes `#SPILL!`, and a spilled result is addressable through its anchor
with the `#` operator.

Specific to this function:

- **A one-cell result is still an array.** `TEXTSPLIT("abc", ",")` yields a 1×1 array whose single
  element is `"abc"`. The engine collapses that on publication, but a consuming function sees an
  array, and [coercion and lifting](../model/02-coercion-and-lifting.md) explains why a 1×1 array
  argument is deliberately not flattened to a bare scalar on lift positions.
- **Fields are always text.** `TEXTSPLIT("1,2", ",")` produces the strings `"1"` and `"2"`, not
  numbers. Wrap the result in `VALUE` or `--` if you want numbers, and see
  [`VALUE`](FUNC.VALUE.md) for how locale-dependent that conversion is.
- **The padding value is a value, not a format.** `pad_with` can be text, a number, or an error;
  it occupies the cell exactly as given.
- **Error inputs propagate** ahead of any splitting.
- **`text` is a scalar position.** The upstream slice contract admits scalar text only; whether
  Excel lifts `TEXTSPLIT` over an array-valued `text` (which would require an array of arrays,
  something the value universe does not admit) is not something this page asserts either way.

## Errors

Microsoft's page does not publish an error table for `TEXTSPLIT` (see Sources). What it does state
is that unfilled cells of an uneven result show `#N/A` unless `pad_with` is supplied, and it
recommends `IFNA` or `pad_with` to handle that. Note the framing: `#N/A` here is a *padding value*
occupying data cells, not a failure of the call.

Beyond that, the errors this function can be involved in are the ordinary ones:

- an error value arriving in any argument propagates, per
  [coercion and lifting](../model/02-coercion-and-lifting.md);
- `#SPILL!` when the result cannot be published into the sheet — an engine outcome described in
  [the call pipeline](../model/03-call-pipeline.md), not a `TEXTSPLIT` semantic;
- `#VALUE!` for arguments outside their admitted domains. Which domains, exactly, is not stated on
  Microsoft's page, and this Handbook has not established it.

That last row is a real gap and is listed below rather than papered over.

## Relationships

- `TEXTJOIN` — the inverse operation. `TEXTJOIN` collapses an array into one delimited string;
  `TEXTSPLIT` expands one string into an array. They are not exact inverses: joining loses the
  distinction between an empty field and an absent one, so a round trip through `ignore_empty` is
  lossy.
- [`TEXTAFTER`](FUNC.TEXTAFTER.md) and [`TEXTBEFORE`](FUNC.TEXTBEFORE.md) — the selective members
  of the same idea: one chosen occurrence rather than all of them. `INDEX(TEXTSPLIT(t,d), n)` and
  `TEXTAFTER(t, d, n-1)` answer related questions differently.
- [`ARRAYTOTEXT`](FUNC.ARRAYTOTEXT.md) — shares an implementation module upstream and is the usual
  way to make a `TEXTSPLIT` result inspectable as a single string.
- `SUBSTITUTE` plus `FIND` plus `MID` was the classic pre-dynamic-array splitting idiom, along with
  the Text to Columns command. Neither is superseded in the formal sense; both remain available.
- Confusable with `SPLIT` from VBA and with Power Query's split-column step. Neither shares this
  function's padding or `ignore_empty` semantics.

## Notes for implementers

- **Decide the order of `ignore_empty` and rectangularisation, and record it.** Dropping empty
  fields before computing `max(c_i)` and dropping them after give different column counts for the
  same input. The documentation does not say which happens.
- **Overlapping delimiters need a policy**, as in the rest of this family. Splitting `"aaa"` on
  `"aa"` depends on whether the scan advances one character or past the match.
- **An array of delimiters needs a precedence rule at a shared position.** If `col_delimiter` is
  `{"ab","a"}` and the text contains `"ab"`, longest-first and first-listed give different splits.
  Nothing documented settles this.
- **A delimiter that is the empty string** has no defined cut points under the model above. Treat
  it as an explicit case rather than letting an empty-match loop run away.
- **Row and column delimiters can collide.** If the same string is passed for both, the outer
  split consumes every occurrence and the inner split finds none. That is a consequence of
  outer-first ordering, and it is worth a test.
- **Publication is not your problem.** Produce the rectangle; `#SPILL!` belongs to the host, per
  [the call pipeline](../model/03-call-pipeline.md). OxFunc's own slice contract is explicit that
  its packet proves core array semantics and deliberately does not claim spill-host publication
  closure — a distinction worth preserving in any port.

## What has not been checked

There is no Handbook vector suite for `TEXTSPLIT`, and no Handbook evidence record comparing any
implementation against Excel. Nothing here is a measurement. The upstream OxFunc contract for this
family is marked provisional and states its own boundary: it captures core array semantics through
a scalarised witness path and does not claim broader spill-publication or Unicode-collation
closure.

The probes that would settle the open questions, roughly in order of how much behaviour they pin:

1. **`ignore_empty` and shape.** `TEXTSPLIT("a,,b" & CHAR(10) & "c", ",", CHAR(10), TRUE)` shows
   whether empties are dropped before or after the column count is fixed, and whether a row can
   collapse to zero fields.
2. **Delimiter-array precedence.** `TEXTSPLIT("xaby", {"ab","a"})` versus `TEXTSPLIT("xaby",
   {"a","ab"})` separates longest-match from first-listed.
3. **Overlap policy.** `TEXTSPLIT("aaaa", "aa")`.
4. **Empty-string delimiter.** `TEXTSPLIT("abc", "")` — does it split into characters, return the
   whole string, or error? All three are plausible and none is documented.
5. **Same string as both delimiters.** `TEXTSPLIT("a,b", ",", ",")`.
6. **`match_mode` folding beyond ASCII**, using a Turkish dotted/dotless `i` pair.
7. **The argument-domain errors.** Sweep `ignore_empty` and `match_mode` with values outside
   `{0,1}` / `{TRUE,FALSE}`, and with text, to find where `#VALUE!` actually starts.
8. **Very wide and very tall results**, to find where the function meets the grid limits and which
   error is published there.

Until those are recorded, the definition above is a model of the documented behaviour, not a
verified account of it.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| row-segment | A piece of the text produced by the outer split on `row_delimiter` |
| field | A piece of a row-segment produced by the inner split on `col_delimiter` |
| rectangularisation | Padding short rows so the result is a legal rectangular `Array` |
| padding value | The value written into cells that carry no field; `#N/A` unless `pad_with` is given |
| spill | The host-side publication of an array result into the grid; see the call-pipeline chapter |

## Sources

- Microsoft Support, TEXTSPLIT function —
  <https://support.microsoft.com/en-us/office/textsplit-function-b1ca414e-4c21-4ca0-b1b7-bdecace8a6e7>
  (retrieved for this page; source of the signature, argument meanings, and the padding note. It
  publishes no error table.)
- OxFunc `docs/function-lane/FUNCTION_SLICE_ARRAY_TEXT_SPLIT_FAMILY_CONTRACT_PRELIM.md` — the
  admitted current-baseline slice, the `#N/A` default padding, and the explicit statement that the
  packet does not claim spill-host publication closure. Provisional by its own statement.
- Handbook `content/model/01-value-universe.md` (arrays are rectangular; `#N/A` and `#SPILL!`),
  `02-coercion-and-lifting.md`, `03-call-pipeline.md` (host-side adaptation and spill).
