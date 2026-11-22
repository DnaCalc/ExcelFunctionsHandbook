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
  - Notes
  - Page vocabulary
  - Sources
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

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `CODE` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-CHAR-001: CHAR and CODE use the Windows-1252 code page (CHAR(128) is U+20AC, astral characters give CODE 63), while UNICHAR/UNICODE use Unicode scalar values

*Quirk; measured; recorded 2026-11-22.* Applies to CHAR, CODE, UNICHAR, UNICODE.

In Excel 16.0.20527.20088 (UI language 1033 / en-ZA), CHAR maps 1..=255 through Windows-1252
rather than ISO-8859-1, so CHAR(128) is '€' (U+20AC) and CHAR(159) is 'Ÿ' (U+0178). CODE("")
returns #VALUE!, and characters outside Windows-1252 such as an astral emoji
(LEFT(UNICHAR(128512),1)) return 63 ('?'), whereas UNICHAR and UNICODE preserve full Unicode
scalar values (UNICODE(LEFT(UNICHAR(128512),1)) = 128512).

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=CHAR(1)&CHAR(127)&CHAR(128)&CHAR(159)&CHAR(255)` | as typed | `€Ÿÿ` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=CODE("")` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=CODE(LEFT(UNICHAR(128512),1))` | as typed | `63.0` (`0x404f800000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=UNICODE(LEFT(UNICHAR(128512),1))` | as typed | `128512.0` (`0x40ff600000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Measured in Excel 16.0.20527.20088 across f2_rows (rows 1, 4) and dcfl2_text
(rows 12-14) and matched across 1,056/1,056 CHAR, 2,109/2,109 CODE, 1,068/1,068 UNICHAR, and
2,087/2,087 UNICODE full-bank rows.

Evidence:

- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/f2_rows.capture.json`, rows 1 and 4 at `4fb9b728df` — Windows-1252 CHAR mapping and CODE empty-text #VALUE!
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcfl2_text.capture.json`, rows 12 to 14 at `4fb9b728df` — Astral character CODE 63 vs UNICODE 128512
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/f2_rows.capture.json`, sheet f2_rows row 1 column 4 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/f2_rows.capture.json`, sheet f2_rows row 4 column 4 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcfl2_text.capture.json`, sheet dcfl2_text row 14 column 4 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcfl2_text.capture.json`, sheet dcfl2_text row 12 column 4 at `4fb9b728df` (readings)

<!-- efh:notes end -->

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
