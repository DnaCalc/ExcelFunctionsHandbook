---
schema: efh.function-page/v1
function_id: FUNC.FINDB
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
family: text_b_compat_family
role_in_family: "The byte-counting locator: FIND with positions measured in bytes."
---

## What it computes

`FINDB` locates one text string inside another and returns the starting position of the first
occurrence, exactly as [`FIND`](FUNC.FIND.md) does — but the position is counted in **bytes**
rather than in characters. It is case sensitive and does not allow wildcards, on the same terms
as `FIND`.

Microsoft's article states the distinction more precisely than any other article in the
byte-variant set, and this page quotes its content rather than reconstructing it: `FIND` is
intended for use with languages that use the **single-byte character set** (SBCS), whereas
`FINDB` is intended for languages that use the **double-byte character set** (DBCS). `FIND`
always counts each character, whether single-byte or double-byte, as 1, no matter what the
default language setting is. `FINDB` counts each double-byte character as 2 **when you have
enabled the editing of a language that supports DBCS and then set it as the default language**.

That last clause is the part worth reading twice, because it makes `FINDB`'s answer depend on
three things rather than one:

1. the text — how many of its characters are double-byte under the host encoding;
2. the host's enabled editing languages;
3. which of them is set as the default language.

Change none of the text and only the third, and the same formula returns a different number.
This is the sharpest statement in Microsoft's documentation that the byte variants are
environment-dependent functions, and it is why the Handbook publishes `FINDB` as a separate
entry rather than as a note on `FIND`. (Seven published rows document a byte-variant pair this
way; splitting all seven is the whole of the difference between 534 published Excel rows and
541 Handbook entries.)

Two consequences follow directly. When the DBCS condition is not met, `FINDB` counts every
character as 1 and its results coincide with `FIND`'s — so an all-ASCII or Western-host test
suite cannot distinguish the two functions at all. And when it is met, a returned byte position
is *not* a valid character position: feeding a `FINDB` result into `MID` rather than `MIDB` is
a byte/character unit error that produces plausible wrong answers rather than errors.

The article text retrieved for this page states the DBCS condition in terms of enabled and
default editing languages, and does not, in that text, enumerate which languages support DBCS.
This page therefore does not name them.

## Arguments

`find_text` — required, the text you want to find.

`within_text` — required, the text containing the text you want to find.

`start_num` — optional, the character at which to start the search; the first character in
`within_text` is character number 1, and omitting it is documented as assuming 1. Note that the
article's argument description speaks of *characters* for this argument while the returned
position is in bytes for `FINDB`; whether `start_num` is interpreted in bytes or characters
under the DBCS condition is a real question that the retrieved text does not resolve.

As with `FIND`, the needle comes first. `FINDB(within, needle)` is a silent bug.

## Result and edge cases

The return kind is `Number`.

- **A `start_num` or a result can fall inside a character.** This is the byte-variant hazard
  with no character-variant analogue: byte position 2 of a string beginning with a double-byte
  character is the second half of that character. What `FINDB` does when a match would begin
  there, and what it does when `start_num` names such a position, is undefined by every source
  this page can cite.
- **Under the non-DBCS condition the function coincides with `FIND`**, per the article's own
  statement.
- **Astral characters** have no natural DBCS byte length at all, and the Compatibility Version
  feature that Microsoft documents for surrogate handling elsewhere in the text family has no
  stated interaction with byte counting.

## Errors

- **`#VALUE!` when `find_text` does not appear in `within_text`** — stated in the article for
  both members of the pair.
- Microsoft publishes a dedicated article on correcting `#VALUE!` in `FIND`/`FINDB` and
  `SEARCH`/`SEARCHB`.
- Whether an out-of-range or mid-character `start_num` errors, and with which code, is not
  stated in the retrieved text.
- Error inputs propagate, and a value kind with no text conversion surfaces `#VALUE!` under the
  shared coercion rules ([chapter 02](../model/02-coercion-and-lifting.md)).

## Relationships

- **[`FIND`](FUNC.FIND.md)** is the character-counting partner, documented in the same article.
  It is the function to use unless you specifically need byte offsets.
- **`SEARCHB`** is the case-insensitive, wildcard-aware byte variant — `FINDB` is to `SEARCHB`
  as `FIND` is to `SEARCH`.
- **[`LEFTB`](FUNC.LEFTB.md), `RIGHTB`, `MIDB`, [`LENB`](FUNC.LENB.md), `REPLACEB`** are the
  rest of the byte-variant family, and are the functions a `FINDB` result must be fed into for
  the units to agree. Note that the `LEFT` and `LEN` articles now mark their byte variants
  deprecated and no longer state the DBCS mechanism at all; this article still does, which
  makes it the best surviving documentation of what byte counting means anywhere in the family.
- **`ASC` and `DBCS`** convert between half-width and full-width forms.

A localization note specific to the split: the localized-name row projected for this entry
carries the *paired* name string from the published row, naming both `FIND` and `FINDB`
together, because the upstream source is the paired article. That is a projection artifact, not
a localized name for `FINDB` alone.

## Notes for implementers

1. **Do not implement `FINDB` as `FIND`.** They coincide only under the non-DBCS condition, and
   aliasing them ships that specialisation. Say so if that is what you are shipping.
2. **The DBCS condition is host state, not text state.** An implementation faithful to the
   documentation needs the enabled-languages and default-language settings as inputs, which
   makes this function's observation context wider than an Excel build and a platform.
3. **Byte positions and character positions must not be interchanged internally**, or the bug
   the function exists to avoid reappears inside the implementation.
4. **Write down the mid-character rules** for both `start_num` and the returned position.
5. The OxFunc reference engine at commit `473efa3` implements `FINDB` by delegating directly to
   its `FIND` kernel, which searches UTF-16 code-unit sequences. It therefore has no byte
   semantics today, and inherits `FIND`'s undocumented empty-needle and `start_num` positions
   wholesale. Implementation fact about OxFunc only.

## What has not been checked

No Handbook vector suite exists for `FINDB`, and no Excel-comparison evidence record names it.
Nobody has checked this function against Excel — and, as with every byte variant, the only
inputs that could distinguish it from its partner are the ones nobody has run.

A projection note: `data/functions/FUNC.FINDB.json` carries no `xlcall` symbol, and its
`english_description` is the paired row's description of `FIND` rather than a description of
`FINDB`. Metadata observations, not behavioural ones.

Probes that would settle the page, in order:

- `FINDB` and `FIND` for the same needle in a haystack of Japanese kana, on a host with a DBCS
  language enabled and set as default. One pair of cells decides whether the documented byte
  semantics are live in current builds.
- The same pair with the default language changed and nothing else — the direct test of the
  article's "set it as the default language" clause.
- `FINDB` with a `start_num` naming the second byte of a double-byte character, and a search
  whose match would begin there.
- `FINDB("","abc")` and out-of-range `start_num` values, matching the `FIND` probe list so the
  two pages can be compared row for row.
- `FINDB` on half-width versus full-width forms of the same characters.

## Page vocabulary

| Term | Meaning as used here |
|---|---|
| SBCS / DBCS | Single- and double-byte character sets; the encoding families the article names |
| DBCS condition | A DBCS-supporting editing language enabled *and* set as the default language |
| byte position | An offset measured in bytes of the host encoding, not in characters |
| mid-character split | A byte offset that falls inside a multi-byte character |

## Sources

- Microsoft, "FIND, FINDB functions" —
  <https://support.microsoft.com/en-us/office/find-function-c7912941-af2a-4bdf-a553-d0d89b0a0628>
  (the SBCS/DBCS paragraph, including the "counts each double-byte character as 2 when you have
  enabled the editing of a language that supports DBCS and then set it as the default language"
  condition; the argument descriptions; case sensitivity and the absence of wildcards; the
  `#VALUE!` on not-found). Retrieved for this page through a search result for that article; a
  direct request for the same URL did not serve it.
- Microsoft, "How to correct a #VALUE! error in FIND/FINDB and SEARCH/SEARCHB functions" —
  <https://support.microsoft.com/en-us/excel/how-to-correct-a-value-error-in-find-findb-and-search-searchb-functions>
- Handbook `CHARTER.md` section 4 (the seven split byte-variant rows).
- Handbook `data/functions/FUNC.FINDB.json` (the paired description and localized-name
  projections noted above).
- OxFunc `crates/oxfunc_core/src/functions/text_b_compat_family.rs` at commit `473efa3` — read
  for the reference engine's delegation to the character kernels. Implementation fact only.
