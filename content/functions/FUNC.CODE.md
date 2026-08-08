---
schema: efh.function-page/v1
function_id: FUNC.CODE
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
family: text_scalar_misc
role_in_family: "The character-to-code direction of the text/number pair; CHAR is its inverse."
---

## What it computes

`CODE` takes a text value, looks at its **first character only**, and returns the numeric code
of that character in the host's character set. Microsoft's article says exactly that the
returned code "corresponds to the character set used by your computer", and names the Macintosh
character set on Macintosh and ANSI on Windows.

Everything after the first character is discarded. `CODE` is therefore not a hash, not a
checksum and not a length-sensitive function: `CODE("A")` and `CODE("Alphabet")` are the same
call as far as the result is concerned.

The function is the intended inverse of `CHAR`, and inherits the same open question in the same
place: for characters inside ASCII every candidate encoding agrees, and above 127 they do not.
See [`CHAR`](FUNC.CHAR.md) for the statement of that question; it is one question, not two.

## Arguments

`text` — required, the text whose first character is to be coded. The argument is a text
argument, so the ordinary to-text conversion applies: a number arrives as its General-formatted
rendering and a logical as `TRUE` or `FALSE`, which means `CODE` of a number is the code of the
first *digit character* of that number's rendering, and `CODE(TRUE)` is the code of `T`. This
is the argument position readers most often misread — the function looks numeric and behaves
textually.

There is no second argument, no index, and no way to ask for the code of the *n*-th character;
compose with `MID` for that.

## Result and edge cases

The return kind is `Number`, and it is a small non-negative integer whenever the call succeeds.

- **Empty text.** `CODE("")` has no first character, so the function has nothing to return.
  What Excel does here is not stated in Microsoft's article text and has not been checked for
  this Handbook.
- **Surrogate pairs.** A character outside the Basic Multilingual Plane is two UTF-16 code
  units. Whether Excel's `CODE` reports the leading surrogate, some replacement, or an error is
  an open question, and it is a live one now that Microsoft has begun changing surrogate
  handling under the Compatibility Version feature — see [`LEN`](FUNC.LEN.md), whose
  documentation records exactly such a change.
- Empty cells, omitted arguments and arrays follow the shared call model in
  [chapter 03](../model/03-call-pipeline.md); nothing here is function-specific.

## Errors

Microsoft's article for `CODE` does not, in its own text, enumerate error conditions. The
conventional expectation is that an empty first argument cannot produce a number and must
surface an error, but the Handbook does not assert an error code it has not seen documented or
measured. The battery panel on this page shows what the reference engine returns; that is the
reference engine's behaviour, not Excel's.

Error inputs propagate rather than convert ([chapter 02](../model/02-coercion-and-lifting.md)).

## Relationships

- **`CHAR`** — the inverse direction, sharing the host-character-set dependency.
- **`UNICODE`** — the Unicode-explicit counterpart, returning the code point rather than a
  host-encoding index. Where a formula must be portable across hosts and locales, `UNICODE` is
  the function that means what it says.
- **`UNICHAR`** — inverse of `UNICODE`, and thus the modern counterpart of `CHAR`.
- Neither `CODE` nor `CHAR` is a Compatibility-category function; Microsoft has not superseded
  either. The Unicode pair was added alongside them, not in place of them.

## Notes for implementers

Three decisions are load-bearing and none of them is settled by the documentation:

1. **Which encoding.** "The character set used by your computer" is an instruction to be
   host-dependent. A single-encoding implementation is making a choice; it should say which.
2. **First character or first code unit.** These differ exactly for astral characters. An
   implementation that reads the first UTF-16 code unit will report a lone surrogate value for
   an emoji; one that decodes first will report either a code point above 0xFFFF (which cannot
   be a byte-set index) or an error.
3. **Round-tripping.** `CODE(CHAR(n)) = n` is the property users assume. It holds for whatever
   subrange the two functions agree on, and any implementation should test the round trip
   across the entire 1–255 range rather than on ASCII samples, because ASCII is precisely where
   the interesting disagreements are absent.

The OxFunc reference engine at commit `473efa3` returns the numeric value of the first UTF-16
code unit. That is an implementation fact about OxFunc, and it takes decision 2 in the "first
code unit" direction and decision 1 in the "no host encoding" direction.

## What has not been checked

No Handbook vector suite exists for `CODE`, and no Excel-comparison evidence record names it.
Unchecked, in priority order:

- `CODE` of each character Excel's `CHAR` produces for 128–255 on a Windows host — the round
  trip is the cheapest probe that characterises both functions at once;
- `CODE("")`, and whether the resulting error is `#VALUE!`;
- `CODE` of an emoji and of a precomposed-versus-combining accented character, both in a legacy
  workbook and in a workbook set to Compatibility Version 2;
- whether the result depends on the Windows system ANSI code page.

## Page vocabulary

| Term | Meaning as used here |
|---|---|
| host character set | The character set the running Excel host resolves character codes against |
| astral character | A Unicode character outside the Basic Multilingual Plane; two UTF-16 code units |
| round trip | The property `CODE(CHAR(n)) = n` over a stated range |

## Sources

- Microsoft, "CODE function" —
  <https://support.microsoft.com/en-us/office/code-function-c32b692b-2ed0-4a04-bdd9-75640144b928>
  (first-character rule; the "character set used by your computer" statement; the
  Macintosh/Windows-ANSI split). Retrieved for this page.
- Microsoft, "LEN, LENB functions" — cited here only for the Compatibility Version surrogate
  change that makes the surrogate question live:
  <https://support.microsoft.com/en-us/office/len-function-29236f94-cedc-429d-affd-b5e33d2c67cb>
- Handbook `content/model/02-coercion-and-lifting.md` (to-text conversion, error propagation)
  and `content/model/03-call-pipeline.md` (argument preparation).
- OxFunc `crates/oxfunc_core/src/functions/text_scalar_misc.rs` at commit `473efa3` —
  implementation fact only.
