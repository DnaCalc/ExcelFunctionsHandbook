---
schema: efh.function-page/v1
function_id: FUNC.LEFTB
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
  - Notes
  - Page vocabulary
  - Sources
family: text_b_compat_family
role_in_family: "The byte-counting prefix member: LEFT with its count measured in bytes."
---

## What it computes

`LEFTB` returns the leading portion of a text string measured in **bytes**, where
[`LEFT`](FUNC.LEFT.md) measures in characters. The unit is the only difference between them,
and it is why the Handbook publishes the pair as two entries against Microsoft's one article.
(Seven published rows document a byte-variant pair this way; splitting all seven is the whole
of the difference between 534 published Excel rows and 541 Handbook entries.)

**What "bytes" means here.** The byte variants come from the era of *double-byte character
sets* (DBCS): host encodings in which some characters occupy one byte and others two. Asking
for the first `n` bytes of such a string is a different question from asking for the first `n`
characters, because the two counts advance at different rates through the same text. Where
every character in the string is single-byte, the two questions coincide and `LEFTB` and `LEFT`
return the same result; they diverge only on text containing double-byte characters, under a
host whose language setting supports DBCS.

`LEFTB` carries one problem `LENB` does not: **a byte count can land in the middle of a
character.** `LEFTB(t, 1)` applied to a string beginning with a double-byte character asks for
half of that character. Every implementation of a byte-slicing function must have a rule for
that case — return a half character, drop it, or pad — and no source cited on this page states
which rule Excel uses. That is the central unresolved question about this function, and it has
no analogue in the character-counting variant.

**What the documentation now says.** The Microsoft article that covers the pair states that
"the LEFTB function is deprecated", and adds that `LEFT` now handles Unicode surrogates through
the Compatibility Version feature (Version 2) — the modern replacement for the problem byte
counting was once used to work around. The text retrieved for this page contains no DBCS
statement, no list of affected languages or language settings, and no definition of the byte
count or of the split-character rule. The Handbook records the byte semantics as the function's
stated purpose and historical definition, and records that the current article does not define
them. It does not assert which locales are affected, because the article does not say.

The definition survives elsewhere in Microsoft's own documentation: the `FIND, FINDB` article
still carries the SBCS/DBCS paragraph and the condition under which a byte variant counts a
double-byte character as 2. That paragraph is quoted on [`FINDB`](FUNC.FINDB.md), and it is the
best surviving statement of what byte counting means anywhere in this family — including for
`LEFTB`, whose own article no longer says.

## Arguments

`text` — required, the text string containing the characters to extract.

`num_bytes` — optional, specifying how many characters to extract **in bytes**. The
corresponding `LEFT` argument is documented as defaulting to 1, requiring a value greater than
or equal to zero, and returning the whole text when it exceeds the length; the same shape is
the natural reading here, and this page treats it as the expected behaviour rather than a
documented one, since the retrieved article states those rules for `LEFT` under the name
`num_chars`.

## Result and edge cases

The return kind is `Text`.

- **On single-byte text the result equals `LEFT`'s.** That is the definition specialising, not
  a coincidence, and it is why an all-ASCII test suite cannot distinguish the two functions.
- **A count landing mid-character** is the interesting case, and it is undefined by every
  source this page can cite.
- **A count beyond the byte length** is expected to yield the whole text, by analogy with
  `LEFT`; unverified.
- **Astral characters** have no natural DBCS byte length at all, so the interaction between
  `LEFTB` and the surrogate handling that the Compatibility Version feature changed is doubly
  undefined.

## Errors

The article enumerates no error conditions for the B variant. Reachable errors are the shared
ones: propagation of an error argument, and `#VALUE!` for a value kind with no text conversion
([chapter 02](../model/02-coercion-and-lifting.md)). Whether a negative `num_bytes` errors the
same way a negative `num_chars` does is unverified.

## Relationships

- **[`LEFT`](FUNC.LEFT.md)** is the character-counting partner, documented in the same article,
  which marks `LEFTB` deprecated. `LEFT` is the function to use in new work.
- **`RIGHTB`, `MIDB`, [`LENB`](FUNC.LENB.md), [`FINDB`](FUNC.FINDB.md), `SEARCHB`, `REPLACEB`**
  are the rest of the byte-variant family. The unit is one shared decision across all seven,
  and so is the mid-character rule.
- **`ASC` and `DBCS`** convert between half-width and full-width forms and are the functions
  people actually reach for when the problem is character width rather than storage width.

A localization note specific to the split: the localized-name row projected for this entry
carries the *paired* name string from the published row, naming both `LEFT` and `LEFTB`
together, because the upstream source is the paired article. That is a projection artifact, not
a localized name for `LEFTB` alone.

## Notes for implementers

1. **Do not implement `LEFTB` as `LEFT`.** They coincide only on single-byte text, and aliasing
   them is implementing the non-DBCS specialisation. Say so if that is what you are shipping.
2. **Name the encoding.** A byte count is meaningless without one, and the historical answer —
   the host's system code page — makes this function environment-dependent in a way `LEFT` is
   not.
3. **Write the mid-character rule down**, whatever you choose, and test it first; it is the
   only behaviour a byte-slicer has that a character-slicer does not.
4. **Guard the text-cap and surrogate interactions.** A byte slicer built over UTF-16 storage
   can produce ill-formed text through two independent mechanisms — surrogate splitting and
   DBCS splitting — and the value-universe chapter records ill-formed UTF-16 as a reachable
   state that downstream code must survive.
5. The OxFunc reference engine at commit `473efa3` implements `LEFTB` by delegating directly to
   its `LEFT` kernel, which slices UTF-16 code units. It therefore has no byte semantics at all
   today. Implementation fact about OxFunc only.

## What has not been checked

No Handbook vector suite exists for `LEFTB`, and no Excel-comparison evidence record names it.
Nobody has checked this function against Excel — and the inputs that could distinguish it from
`LEFT` are precisely the inputs nobody has run.

Probes that would settle the page, in order:

- `LEFTB` and `LEFT` with count 1 on a string beginning with a Japanese kana character, on a
  host with a Japanese system locale. One pair of cells decides both whether byte semantics are
  live and what the mid-character rule is.
- The same pair on a Western host, separating "property of the text" from "property of the host
  language setting".
- `LEFTB(t,0)` and `LEFTB(t,-1)`, to pin the zero and negative boundaries against `LEFT`'s
  documented ones.
- `LEFTB` on an emoji, in a legacy workbook and in a Compatibility Version 2 workbook.
- `LEFTB` on half-width versus full-width forms of the same characters.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `LEFTB` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-LENB-001: Under a single-byte Windows locale such as en-ZA, the B-suffixed text functions count characters rather than UTF-8 or UTF-16 bytes

*Quirk; measured; recorded 2026-11-22.* Applies to FINDB, LEFTB, LENB, MIDB, REPLACEB, RIGHTB, SEARCHB.

In Excel 16.0.20527.20088 with user culture en-ZA (a single-byte Windows code page), LENB,
LEFTB, RIGHTB, MIDB, FINDB, SEARCHB, and REPLACEB count characters rather than UTF-8 or UTF-16
bytes: LENB(UNICHAR(20013)) returns 1, LEFTB(UNICHAR(20013)&"abc",2) returns "中a", and
FINDB("b",UNICHAR(20013)&"b") returns 2 (double-byte counting applies only under DBCS languages
such as ja-JP, ko-KR, or zh-CN).

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=LENB(UNICHAR(20013))` | as typed | `1.0` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=LEFTB(UNICHAR(20013)&"abc",2)` | as typed | `中a` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=FINDB("b",UNICHAR(20013)&"b")` | as typed | `2.0` (`0x4000000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=REPLACEB(UNICHAR(20013)&"abc",2,1,"x")` | as typed | `中xbc` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Measured across rows 18-27 of dczsn3_bytes in Excel 16.0.20527.20088 (en-ZA)
and matched across full-bank rows (LENB 2,083/2,083, LEFTB 2,478/2,478, RIGHTB 2,481/2,481,
MIDB 2,913/2,913, FINDB 2,861/2,861, SEARCHB 2,861/2,861, REPLACEB 2,951/2,951).

- Open: DBCS locales (ja-JP, ko-KR, zh-CN) remain uncaptured and refused in calc-func (dc-zsn.3).

Evidence:

- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dczsn3_bytes.capture.json`, rows 18 to 27 at `4fb9b728df` — Excel 16.0.20527.20088 single-byte locale B-function measurements
- DnaCalc `crates/calc-func/src/catalogue/text_functions.rs`, single_byte at `4fb9b728df` — Single-byte vs DBCS locale guard
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dczsn3_bytes.capture.json`, sheet dczsn3_bytes row 18 column 4 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dczsn3_bytes.capture.json`, sheet dczsn3_bytes row 21 column 4 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dczsn3_bytes.capture.json`, sheet dczsn3_bytes row 24 column 4 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dczsn3_bytes.capture.json`, sheet dczsn3_bytes row 26 column 4 at `4fb9b728df` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning as used here |
|---|---|
| DBCS | Double-byte character set: a host encoding mixing one- and two-byte characters |
| byte variant | A function identical to its partner except that its unit is bytes, not characters |
| mid-character split | A byte count that ends inside a multi-byte character |
| paired published row | One Microsoft article documenting two functions; the Handbook splits it |

## Sources

- Microsoft, "LEFT function" (the article that covers the pair) —
  <https://support.microsoft.com/en-us/office/left-function-9203d2d2-7960-479b-84c6-1ea52b99640c>
  ("The LEFTB function is deprecated"; the Compatibility Version 2 surrogate note; the
  `num_chars` rules quoted here as the expected shape for `num_bytes`; no DBCS or language
  statement in the text retrieved). Retrieved for this page.
- Microsoft, "FIND, FINDB functions" — cited only for the surviving SBCS/DBCS paragraph:
  <https://support.microsoft.com/en-us/office/find-function-c7912941-af2a-4bdf-a553-d0d89b0a0628>
- Handbook `CHARTER.md` section 4 (the seven split byte-variant rows) and
  `content/model/01-value-universe.md` (ill-formed UTF-16 as a reachable state).
- Handbook `data/functions/FUNC.LEFTB.json` (the paired localized-name projection noted above).
- OxFunc `crates/oxfunc_core/src/functions/text_b_compat_family.rs` at commit `473efa3` — read
  for the reference engine's delegation to the character kernels. Implementation fact only.
