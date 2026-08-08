---
schema: efh.function-page/v1
function_id: FUNC.LENB
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
role_in_family: "The byte-counting member: LEN's unit changed from characters to bytes."
---

## What it computes

`LENB` returns the number of **bytes** used to represent a text string, where `LEN` returns the
number of characters. That one substitution — bytes for characters — is the entire difference
between the two functions, and it is why the Handbook publishes them as two entries against
Microsoft's one article. (Seven published rows document a byte-variant pair this way; the
Handbook splits all seven, which is the whole of the difference between 534 published Excel
rows and 541 Handbook entries.)

**What "bytes" means here.** The byte-variant functions come from the era in which East Asian
text was stored in a *double-byte character set* (DBCS): a host encoding in which some
characters occupy one byte and others two. Under such an encoding a string's byte length is not
a fixed multiple of its character length — it is one byte per single-byte character plus two
per double-byte character — so "how many bytes is this string" and "how many characters is this
string" are genuinely different questions with genuinely different answers, and a program
working against fixed-width record layouts needs the first one.

The standard consequence follows arithmetically: where every character in the string is
single-byte, `LENB` and `LEN` return the same number. The two functions diverge only on text
containing double-byte characters, under a host whose language setting supports DBCS. This
makes `LENB` an easy function to believe you have tested when you have not — an all-ASCII test
suite cannot tell the two functions apart at all.

**What the documentation now says.** The Microsoft article that covers this pair states, in one
line, that "the LENB function is deprecated". The text retrieved for this page contains no
statement about DBCS, no list of the languages or language settings under which `LENB` differs
from `LEN`, and no definition of the byte count. The Handbook therefore records the
byte-versus-character distinction as the function's *stated purpose and historical definition*,
and records that the current article does not define it. It does not assert which languages or
code pages are affected, because the article does not say.

The definition has not vanished from Microsoft's documentation altogether: the `FIND, FINDB`
article still carries the SBCS/DBCS paragraph, including the condition under which a byte
variant counts a double-byte character as 2. That paragraph is quoted on
[`FINDB`](FUNC.FINDB.md), and it is the best surviving statement of what byte counting means
anywhere in this family — including for `LENB`, whose own article no longer says.

## Arguments

`text` — required, the text whose byte length you want. It is the same argument as `LEN`'s,
with the same to-text conversion of numbers and logicals.

There is no encoding argument. The encoding is the host's, which is exactly why this function's
answer is environment-dependent in a way `LEN`'s (mostly) is not.

## Result and edge cases

The return kind is `Number`, a non-negative integer.

- **Empty text is 0** under any encoding.
- **On single-byte text the result equals `LEN`'s.** That is not a coincidence to be worked
  around; it is the definition specialising.
- The astral-character question that dominates [`LEN`](FUNC.LEN.md) — surrogate pairs, and the
  Compatibility Version 2 change to how they are counted — has no documented counterpart here.
  What a byte count is supposed to mean for a character that the historical DBCS encodings
  could not represent at all is undefined by any source this page can cite.

## Errors

The article enumerates no error conditions. Reachable errors are the shared ones: propagation
of an error argument, and `#VALUE!` for a value kind with no text conversion
([chapter 02](../model/02-coercion-and-lifting.md)).

## Relationships

- **[`LEN`](FUNC.LEN.md)** is the character-counting partner and the function to use in new
  work. Microsoft documents both in one article and marks `LENB` deprecated there.
- **`LEFTB`, `RIGHTB`, `MIDB`, `FINDB`, `SEARCHB`, `REPLACEB`** are the rest of the
  byte-variant family. They share the same unit substitution and the same deprecation posture,
  and an implementation should treat the unit as one shared decision rather than seven. See
  [`LEFTB`](FUNC.LEFTB.md) and [`FINDB`](FUNC.FINDB.md).
- **`ASC` and `DBCS`** are the width-conversion functions from the same lineage, converting
  between full-width and half-width forms.

A localization note that belongs on this page: the localized-name row projected for this entry
carries the *paired* name from the published row — the string names both `LEN` and `LENB`
together — because the upstream source is the paired article. That is a projection artifact of
the split, not a localized name for `LENB` alone.

## Notes for implementers

1. **Do not implement `LENB` as `LEN`.** They coincide only on single-byte text. An
   implementation that aliases them is implementing the non-DBCS specialisation and should say
   so rather than claim the function.
2. **The byte count needs a named encoding.** "Bytes" is meaningless without one. An
   implementation must state which encoding it measures in and where that encoding comes from
   (system ANSI code page, workbook setting, or a fixed choice).
3. **Astral characters need a stated policy** for the reason given above: they have no natural
   DBCS byte length.
4. The OxFunc reference engine at commit `473efa3` implements `LENB` by counting UTF-16 code
   units — that is, it implements the specialisation in which the answer coincides with `LEN`
   on BMP text. Its whole byte-variant family is built the same way, by delegating to the
   character-variant kernels. Implementation fact about OxFunc only, and the honest reading is
   that the reference engine currently has no byte semantics to compare against Excel.

## What has not been checked

No Handbook vector suite exists for `LENB`, and no Excel-comparison evidence record names it.
Nobody has checked this function against Excel at all — and for a byte-variant function that is
a sharper statement than usual, because the only inputs that can distinguish it from `LEN` are
exactly the inputs nobody has run.

Probes that would settle the page, in order:

- `LENB` of a string of Japanese kana on a host with a Japanese system locale, next to `LEN` of
  the same string. One pair of cells decides whether the byte semantics are live in current
  Excel at all.
- The same pair on a Western host, which tests whether the divergence is a property of the
  *text* or of the *host language setting*.
- `LENB` of an emoji and of a lone surrogate, which are undefined by every source cited here.
- `LENB` of half-width versus full-width forms of the same characters, the classic DBCS pair.
- Whether the deprecation is documentation-only or has any behavioural component in current
  builds.

Until at least the first of those exists, the Handbook's position is that `LENB`'s byte
semantics are documented historically, undefined in the current article, and unmeasured here.

## Page vocabulary

| Term | Meaning as used here |
|---|---|
| DBCS | Double-byte character set: a host encoding mixing one- and two-byte characters |
| byte variant | A function identical to its partner except that its unit is bytes, not characters |
| unit substitution | The single semantic difference between a B function and its partner |
| paired published row | One Microsoft article documenting two functions; the Handbook splits it |

## Sources

- Microsoft, "LEN function" (the article that covers the pair) —
  <https://support.microsoft.com/en-us/office/len-function-29236f94-cedc-429d-affd-b5e33d2c67cb>
  ("The LENB function is deprecated"; no DBCS or language statement in the text retrieved).
  Retrieved for this page.
- Microsoft, "FIND, FINDB functions" — cited only for the surviving SBCS/DBCS paragraph:
  <https://support.microsoft.com/en-us/office/find-function-c7912941-af2a-4bdf-a553-d0d89b0a0628>
- Handbook `CHARTER.md` section 4 (the seven split byte-variant rows) and
  `content/model/02-coercion-and-lifting.md`.
- Handbook `data/functions/FUNC.LENB.json` (the paired localized-name projection noted above).
- OxFunc `crates/oxfunc_core/src/functions/text_b_compat_family.rs` at commit `473efa3` — read
  for the reference engine's delegation to the character kernels. Implementation fact only.
