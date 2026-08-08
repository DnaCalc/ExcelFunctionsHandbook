---
schema: efh.function-page/v1
function_id: FUNC.LEFT
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
family: text_slice_family
role_in_family: "The prefix member: the leading slice, with RIGHT and MID covering the other two."
---

## What it computes

`LEFT` returns the first characters of a text string — a prefix of the requested length.
Microsoft's article states it as returning the first character or characters in a text string,
based on the number of characters you specify.

Precisely: with `t` the text and `n` the requested count, the result is the first `min(n,
LEN(t))` characters of `t`. There is no error for asking for more than exists; the article says
that if `num_chars` is greater than the length of the text, `LEFT` returns all of the text.
That saturating behaviour is what makes `LEFT` usable directly on ragged data without a guard.

The unit of "character" is shared with the rest of the slicing family and is discussed on
[`LEN`](FUNC.LEN.md), where Microsoft documents a Compatibility Version 2 change to how
surrogate pairs are counted. The `LEFT` article carries the corresponding note from the other
side: it states that `LEFT` now handles Unicode surrogates through the Compatibility Version
feature (Version 2). Whatever unit `LEN` counts in, `LEFT` should slice in — but the Handbook
has not checked that they move together, and it is the single most consequential unchecked
question about this family.

## Arguments

`text` — required, the text string containing the characters to extract.

`num_chars` — optional. Per the article: it specifies how many characters to extract, it must
be greater than or equal to zero, if it exceeds the text length the entire text is returned,
and it **defaults to 1 if omitted**.

That default is the argument fact readers most often get wrong, in both directions. `LEFT(a)`
is not "the whole string" and it is not an error; it is the first character. And because the
default is 1 rather than 0, `LEFT` and `MID` do not agree on what an unspecified count means —
`MID` has no optional count at all.

`num_chars = 0` is explicitly admissible (the documented constraint is "greater than or equal
to zero") and yields empty text.

## Result and edge cases

The return kind is `Text`.

- **Zero count** yields empty text, not an error.
- **Count beyond the length** yields the whole string, per the article.
- **Non-text arguments convert first.** `LEFT(1234.5, 2)` slices the *rendering* `1234.5`, so
  it is `12`. This is the reason `LEFT` is a poor tool for extracting digits from numbers: the
  rendering, not the value, is what gets cut, and the rendering depends on how the number
  formats under General.
- **A negative count** violates the documented constraint. The article states the constraint
  but does not state the consequence of breaking it; see "Errors".
- Empty cells, omitted arguments and arrays follow the shared call model
  ([chapter 03](../model/03-call-pipeline.md)).

## Errors

Microsoft's article states that `num_chars` must be greater than or equal to zero, and does not
name the error produced when it is not. The Handbook records the constraint as documented and
the error value as unverified.

Error inputs propagate, and a value kind with no text conversion surfaces `#VALUE!` under the
shared coercion rules ([chapter 02](../model/02-coercion-and-lifting.md)).

## Relationships

- **`RIGHT`** is the mirror; **`MID`** generalises both with an explicit start position. All
  three are one family and should share one unit decision.
- **[`LEN`](FUNC.LEN.md)** supplies the bound: `LEFT(a, LEN(a))` is `a`, and that identity is
  the cheapest test that the family's unit choices agree.
- **[`LEFTB`](FUNC.LEFTB.md)** is the byte-counting variant, documented in the same Microsoft
  article and marked there as deprecated.
- **`TEXTBEFORE`** is the modern delimiter-driven alternative and is usually what a formula
  written as `LEFT(a, FIND(x,a)-1)` actually wants — including because `TEXTBEFORE` can be told
  what to do when the delimiter is absent, where the `LEFT`/`FIND` composition simply errors.
- **`TRIM`** is the usual companion when the slice is being taken from fixed-width imported
  data.

## Notes for implementers

1. **Saturate, do not error, on an over-long count.** This is documented behaviour and it is
   the behaviour that makes the function composable.
2. **Decide what a fractional count means.** The documentation states a constraint on sign, not
   on integrality. Truncation toward zero is the natural reading; it is a decision, not a
   given.
3. **Slice on the same unit the length function counts in**, and — given the Compatibility
   Version change — parameterise that unit rather than hard-coding it.
4. **Never split a surrogate pair silently.** Under a code-unit slicer, `LEFT` on astral text
   can produce a lone high surrogate, which the value-universe chapter records as a reachable
   state. Producing it may be correct emulation; producing it *unknowingly* is a defect.
5. The OxFunc reference engine at commit `473efa3` slices UTF-16 code units, truncates the
   count toward zero, treats a negative count as a `#VALUE!` domain error, and saturates at the
   string length. Implementation facts about OxFunc only.

## What has not been checked

No Handbook vector suite exists for `LEFT`, and no Excel-comparison evidence record names it.
Unchecked, most valuable first:

- `LEFT` of a two-emoji string with `num_chars = 1`, in a legacy workbook and in a
  Compatibility Version 2 workbook. This is the probe the `LEFT` article's own surrogate note
  invites, and it also tests whether `LEFT` and `LEN` changed units together.
- `LEFT(a,-1)` — which error, and whether it is refused at entry or at evaluation.
- `LEFT(a, 2.7)` — the truncation rule for a fractional count.
- `LEFT(a, 1e15)` — behaviour at an absurdly large but finite count, where saturation and
  overflow handling meet.
- `LEFT` of a number under a non-English locale, to pin whether the to-text rendering is
  locale-sensitive at this call site.

## Page vocabulary

| Term | Meaning as used here |
|---|---|
| saturating count | A count larger than the string yields the whole string rather than an error |
| slicing unit | The unit positions and counts are measured in: code unit, code point, or grapheme |
| Compatibility Version | A workbook-level setting selecting among documented behaviour generations |

## Sources

- Microsoft, "LEFT function" —
  <https://support.microsoft.com/en-us/office/left-function-9203d2d2-7960-479b-84c6-1ea52b99640c>
  (the first-characters statement; `num_chars` must be ≥ 0, defaults to 1, and returns the
  whole text when it exceeds the length; the LEFTB deprecation line and the Compatibility
  Version 2 surrogate note). Retrieved for this page.
- Handbook `content/model/01-value-universe.md` (UTF-16 code units and the reachable
  lone-surrogate state) and `content/model/03-call-pipeline.md`.
- OxFunc `crates/oxfunc_core/src/functions/text_slice_family.rs` at commit `473efa3` —
  implementation fact only.
