---
schema: efh.function-page/v1
function_id: FUNC.FIND
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
family: text_search_replace_family
role_in_family: "The case-sensitive, literal substring locator; SEARCH is its folding, wildcard-aware sibling."
---

## What it computes

`FIND` locates one text string inside another and returns the **1-based position** at which the
first occurrence starts, counted from the first character of the containing string. Microsoft's
article states it that way: the number of the starting position of `find_text`, counted from
the first character of `within_text`.

Two properties in the article's own words make `FIND` the strict member of the pair: `FIND` is
**case sensitive** and does **not allow wildcard characters**. It is therefore a literal search
— no folding, no pattern language, no locale-aware matching. That is the whole of its
semantics, and it is exactly why it is the right tool for parsing machine-generated text and
the wrong tool for matching what a human typed.

Stated as an operation: return the smallest position `p ≥ start_num` such that the characters
of `within_text` beginning at `p` equal `find_text`, comparing character by character with no
transformation of either side. Positions are counted in characters, and the article is explicit
that `FIND` "always counts each character, whether single-byte or double-byte, as 1, no matter
what the default language setting is" — a guarantee `FINDB` deliberately does not make.

## Arguments

`find_text` — required, the text you want to find.

`within_text` — required, the text containing the text you want to find.

`start_num` — optional, specifying the character at which to start the search. The first
character in `within_text` is character number 1, and omitting `start_num` is documented as
assuming 1.

The argument order is the position readers get wrong most often, and it is worth stating
plainly: the *needle comes first*. `FIND(within, needle)` is a silent bug, not an error,
whenever the two strings happen to contain each other.

`start_num` has two ordinary uses that are worth separating: skipping a known prefix, and
finding the *second* occurrence by feeding it the position of the first plus one. The second
use is the reason the argument exists.

## Result and edge cases

The return kind is `Number`, a positive integer when the search succeeds.

- **The result is absolute, not relative to `start_num`.** `FIND("a","banana",4)` reports the
  position within the whole string, not the offset from position 4. Every off-by-one bug in
  occurrence-walking formulas comes from assuming otherwise.
- **The search is for the *first* occurrence at or after `start_num`.** There is no direction
  argument and no last-occurrence form; finding the last occurrence is the classic
  `SUBSTITUTE`-with-a-sentinel idiom.
- **Case sensitivity is absolute**, including for characters whose case difference is not an
  ASCII letter pair. Whether Excel's comparison is code-unit equality or something normalising
  is not stated in the article and has not been checked here — the same open question that
  [`EXACT`](FUNC.EXACT.md) raises.
- **Empty `find_text`.** What `FIND("", t)` returns is not stated in the text of the article
  retrieved for this page. It is one of the two or three most commonly relied-on edge cases in
  the function, and the Handbook does not assert a value for it.
- Numbers and logicals convert to text before searching, so `FIND(1, 2140)` searches the
  rendering `2140` for the rendering `1`.

## Errors

- **`#VALUE!` when `find_text` does not appear in `within_text`** — stated in the article. This
  is the design decision that shapes every formula built on `FIND`: absence is an error, not a
  zero and not `#N/A`. Wrapping in `IFERROR`, or switching to `TEXTBEFORE`/`TEXTAFTER` which
  take an if-not-found argument, is the standard response.
- Microsoft publishes a dedicated article on correcting `#VALUE!` in `FIND`/`FINDB` and
  `SEARCH`/`SEARCHB`, which is a fair indication of how often the not-found error surprises
  people.
- The conventional expectation is that `start_num` values less than 1, or greater than the
  length of `within_text`, also produce `#VALUE!`. Those conditions are **not** confirmed in
  the article text retrieved for this page, and the Handbook does not assert them.
- Error inputs propagate, and a value kind with no text conversion surfaces `#VALUE!` under the
  shared coercion rules ([chapter 02](../model/02-coercion-and-lifting.md)).

## Relationships

- **`SEARCH`** is the permissive sibling: case-insensitive and wildcard-aware. The pair
  `FIND`/`SEARCH` is the same distinction as [`EXACT`](FUNC.EXACT.md) versus `=`. Choosing
  wrongly between them is the most common defect in text-parsing formulas, because both return
  plausible numbers.
- **[`FINDB`](FUNC.FINDB.md)** is the byte-counting variant, documented in the same article;
  its positions are byte positions rather than character positions.
- **`SEARCHB`** completes the square.
- **`MID`, [`LEFT`](FUNC.LEFT.md), `RIGHT`** are what the returned position is almost always
  fed into; the `-1` and `+1` adjustments in those compositions are where `FIND`'s 1-based,
  absolute result matters.
- **`TEXTBEFORE`, `TEXTAFTER`, `TEXTSPLIT`** are the modern replacements for the whole
  `LEFT`/`MID`+`FIND` idiom. They are not deprecating `FIND`, but a formula that only needs the
  text around a delimiter should not be computing a position at all.
- **`SUBSTITUTE`** and **`REPLACE`** share this entry's implementing module in the reference
  engine and share the same position conventions.

## Notes for implementers

1. **1-based, absolute, first-match.** All three are separately observable and all three are
   easy to get wrong in a port from a 0-based language.
2. **The empty-needle case needs an explicit rule**, and it should be tested against Excel
   rather than reasoned about: both "matches immediately at `start_num`" and "errors" are
   defensible and the documentation retrieved here settles neither.
3. **Do not fold, do not normalise, do not treat `*` or `?` as special.** `FIND`'s value is
   that it is the function which does none of those things.
4. **Comparison unit must match the slicing family's unit**, or positions returned by `FIND`
   will not be valid inputs to `MID` on astral text.
5. The OxFunc reference engine at commit `473efa3` searches UTF-16 code-unit sequences, is
   1-based and absolute, treats `start_num = 0` and a `start_num` past the end as `#VALUE!`
   domain errors, and returns `start_num` itself for an empty needle when `start_num ≤
   LEN(within_text) + 1`. Implementation facts about OxFunc only — note that the last of these
   is precisely the undocumented case above, so the reference engine has taken a position the
   documentation has not.

## What has not been checked

No Handbook vector suite exists for `FIND`, and no Excel-comparison evidence record names it.

A projection note specific to this entry: `data/functions/FUNC.FIND.json` carries no `xlcall`
symbol for `FIND`, and the Microsoft URL it carries did not serve the function article when it
was requested directly for this page — the documented text quoted above was retrieved through a
search result for the same article. Both are metadata observations, not behavioural ones.

Probes that would settle the page, in order:

- `FIND("","abc")`, `FIND("","abc",4)`, `FIND("","abc",5)` — the empty-needle rule and its
  interaction with `start_num`, which the reference engine has already committed to and the
  documentation has not.
- `FIND("a","abc",0)`, `FIND("a","abc",-1)`, `FIND("a","abc",99)` — the three `start_num`
  boundaries, and which error each produces.
- `FIND("a","abc",2.7)` — the truncation rule for a fractional `start_num`.
- `FIND` with a precomposed accented needle against a decomposed haystack — the normalisation
  question shared with `EXACT`.
- `FIND` on astral text, checking that the returned position is usable by `MID` under both
  compatibility versions.

## Page vocabulary

| Term | Meaning as used here |
|---|---|
| 1-based position | The first character is position 1, not 0 |
| absolute result | The returned position is measured from the start of `within_text`, not from `start_num` |
| literal search | No case folding, no wildcards, no normalisation |
| SBCS | Single-byte character set; the encoding family `FIND` is documented as intended for |

## Sources

- Microsoft, "FIND, FINDB functions" —
  <https://support.microsoft.com/en-us/office/find-function-c7912941-af2a-4bdf-a553-d0d89b0a0628>
  (the starting-position definition; the SBCS/DBCS split and the "always counts each character
  … as 1" guarantee; `find_text`, `within_text`, `start_num` and the assumed 1; case
  sensitivity and the absence of wildcards; the `#VALUE!` on not-found). Retrieved for this
  page through a search result for that article; a direct request for the same URL did not
  serve it.
- Microsoft, "How to correct a #VALUE! error in FIND/FINDB and SEARCH/SEARCHB functions" —
  <https://support.microsoft.com/en-us/excel/how-to-correct-a-value-error-in-find-findb-and-search-searchb-functions>
- Handbook `content/model/02-coercion-and-lifting.md` (to-text conversion, error propagation).
- OxFunc `crates/oxfunc_core/src/functions/text_search_replace_family.rs` at commit `473efa3` —
  read for the reference engine's empty-needle and `start_num` rules. Implementation fact only.
