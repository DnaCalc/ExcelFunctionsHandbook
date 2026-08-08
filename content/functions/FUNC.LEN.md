---
schema: efh.function-page/v1
function_id: FUNC.LEN
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
role_in_family: "The measuring member: returns the length that LEFT, RIGHT and MID index into."
---

## What it computes

`LEN` returns the number of characters in a text string. Microsoft's article adds the
clarification readers most often need: "Spaces count as characters." `LEN` measures the stored
text, not the displayed text and not the visible ink.

The entire subtlety of this function is the word *character*, and Excel's own answer to it has
changed. The Handbook's value-universe chapter records the historical model — worksheet text is
a sequence of **UTF-16 code units**, so a character outside the Basic Multilingual Plane
occupies two units and `LEN` of a single emoji is 2. Microsoft's current article records a
change to exactly that:

> in workbooks set to Compatibility Version 2, `LEN` has improved behavior with surrogate
> pairs, counting them as one character instead of two, while variation selectors (commonly
> used with emojis) are still counted as separate characters.

So `LEN` is now a **two-answer function**, and which answer you get is a property of the
workbook, not of the input. A page that states one number for `LEN("😀")` without naming the
compatibility version is stating half a fact.

Note also what the change does *not* cover: a variation selector still counts on its own. So a
composed emoji — a base character, a variation selector, possibly a zero-width joiner and a
skin-tone modifier — still counts as several under Compatibility Version 2, even though it
renders as one glyph. The counting unit under Version 2 is the Unicode code point, not the
grapheme cluster.

## Arguments

`text` — required; the article describes it as the text whose length you want, and states that
spaces count. It is a text argument, so numbers and logicals convert first: `LEN` of a number
is the length of that number's General rendering, which is why `LEN(1/3)` is a large number and
not something meaningful about the value.

There is no second argument — no counting mode, and in particular no way to ask for the other
compatibility version's answer.

## Result and edge cases

The return kind is `Number`, always a non-negative integer.

- **Empty text is 0**; an empty cell converts to empty text and is likewise 0. `LEN` therefore
  cannot distinguish a blank cell from a cell containing `=""` — use `ISBLANK`.
- **Trailing spaces count**, which is the standard way of detecting them:
  `LEN(a)<>LEN(TRIM(a))`.
- **Formatting never counts.** A cell displayed as `$1,234.57` holding the number 1234.567 has
  the `LEN` of `1234.567`, not of the display. Compose with `TEXT` when the display is what you
  mean.
- **The cell text cap is 32,767 code units**, so `LEN` on ordinary cell content is bounded by
  that; values built inside a formula are bounded the same way
  ([chapter 01](../model/01-value-universe.md)).

## Errors

The article lists no error conditions for `LEN`, and the function has no domain to violate.
Reachable errors are the shared ones: an error value in the argument propagates, and a value
kind with no text conversion surfaces `#VALUE!` under the shared coercion rules
([chapter 02](../model/02-coercion-and-lifting.md)).

## Relationships

- **`LENB`** is the byte-counting variant. Microsoft's article now states in one line that "the
  LENB function is deprecated", and the Handbook publishes it as a separate entry because it is
  a different function. See [`LENB`](FUNC.LENB.md) for what byte counting means and for what
  the current documentation no longer says about it.
- **`LEFT`, `RIGHT`, `MID`** index into the same string in the same units, which is why they
  belong to one family: `LEFT(a, LEN(a))` should be `a` under any consistent unit choice, and
  that identity is the cheapest self-consistency test for an implementation.
- **`TRIM`** and **`CLEAN`** are the usual companions when a `LEN` result is surprising: the
  extra characters are almost always whitespace or control characters.
- **`SUBSTITUTE`** underlies the standard counting idiom `LEN(a)-LEN(SUBSTITUTE(a,x,""))`,
  which counts occurrences of `x` — an idiom whose correctness depends on `LEN`'s unit and
  `SUBSTITUTE`'s matching using the same unit.

## Notes for implementers

1. **The counting unit is a declared axis, not a detail.** Three candidates are live — UTF-16
   code units, Unicode code points, grapheme clusters — and Microsoft's documentation now
   places Excel on the first for legacy workbooks and the second for Compatibility Version 2,
   explicitly not the third.
2. **Compatibility version is an observation-context axis.** Any claim about `LEN` on astral
   text has to name it, in the same way a numeric claim names the Excel build. The Handbook's
   existing observation-context vocabulary already carries a workbook-mode field; this is what
   it is for.
3. **Ill-formed UTF-16 is reachable.** The value-universe chapter records interop truncation
   producing a dangling high surrogate. A code-point counter must decide what an unpaired
   surrogate contributes before it meets one.
4. The OxFunc reference engine at commit `473efa3` counts UTF-16 code units for `LEN` — the
   legacy answer. Implementation fact about OxFunc only.

## What has not been checked

No Handbook vector suite exists for `LEN`, and no Excel-comparison evidence record names it.

There is also a projection caveat specific to this entry: `data/presence/FUNC.LEN.json` records
the name-match confidence for `LEN` as **low**, matched under a guarded-short-name rule. The
link from this entry to its implementing module is therefore weaker than for its siblings, and
any inference from module-level counts to `LEN` specifically should be treated accordingly.

Unchecked, and worth probing in this order:

- `LEN` of a single astral character — an emoji, a rare CJK extension character — in a legacy
  workbook and in a workbook set to Compatibility Version 2. Two cells settle the central claim
  of this page.
- `LEN` of a base character plus a variation selector, and of a zero-width-joiner emoji
  sequence, under Version 2, to confirm the code-point-not-grapheme reading.
- `LEN` of a lone surrogate introduced through the interop truncation path the value-universe
  chapter documents.
- `LEN` of a number, a logical and an error, to pin the conversion at this call site.
- Whether `LEFT`/`MID`/`RIGHT` switch units together with `LEN` under Version 2, or
  independently. If they diverge, the family identity `LEFT(a,LEN(a)) = a` breaks, and that
  would be a significant finding.

## Page vocabulary

| Term | Meaning as used here |
|---|---|
| code unit | One UTF-16 unit; the legacy counting unit for worksheet text |
| code point | One Unicode scalar value; the Compatibility Version 2 counting unit per Microsoft |
| grapheme cluster | What a reader calls one character; explicitly *not* the unit either version uses |
| Compatibility Version | A workbook-level setting that selects among documented behaviour generations |

## Sources

- Microsoft, "LEN function" —
  <https://support.microsoft.com/en-us/office/len-function-29236f94-cedc-429d-affd-b5e33d2c67cb>
  ("Spaces count as characters"; "The LENB function is deprecated"; the Compatibility Version 2
  surrogate-pair change and the variation-selector exception). Retrieved for this page.
- Handbook `content/model/01-value-universe.md` (worksheet text as UTF-16 code units, the
  32,767 cap, the reachable dangling-surrogate state) and
  `content/model/02-coercion-and-lifting.md`.
- Handbook `data/presence/FUNC.LEN.json` (the low name-match confidence noted above).
- OxFunc `crates/oxfunc_core/src/functions/text_slice_family.rs` at commit `473efa3` —
  implementation fact only.
