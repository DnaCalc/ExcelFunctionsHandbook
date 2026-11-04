---
schema: efh.function-page/v1
function_id: FUNC.LOWER
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
role_in_family: "The lowercasing member of the casing trio with UPPER and PROPER."
---

## What it computes

`LOWER` returns its argument with every uppercase letter replaced by its lowercase form.
Microsoft's article states both halves of that: it converts all uppercase letters in a text
string to lowercase, and it "does not change characters in text that are not letters" — digits,
spaces and punctuation pass through untouched.

The interesting part of the definition is what "its lowercase form" means once the text leaves
ASCII. Case mapping in Unicode is not a per-character permutation:

- Some characters lowercase to a *different number* of characters than they started with. `İ`
  (U+0130, Latin capital I with dot above) has a canonical lowercasing of two code points, `i`
  followed by a combining dot above.
- Some lowercasings are **context-dependent**. Greek capital sigma `Σ` lowercases to final
  sigma `ς` at the end of a word and to medial sigma `σ` elsewhere — the same input character,
  two answers, chosen by what is next to it.
- Some characters have no agreed lowercase at all, or acquired one late: capital sharp s `ẞ`
  (U+1E9E) is the standard example.

A worksheet function that must return one text value per input has to take a position on all
three. Microsoft's article takes none of them explicitly, so the exact mapping Excel applies is
documented only by example, and the examples are ASCII.

## Arguments

`text` — required, described in the article as the text you want to convert to lowercase. It is
a text argument, so numbers and logicals convert to text first; `LOWER(TRUE)` is the
lowercasing of the string `TRUE`, not a logical.

There is no locale argument. That absence is load-bearing: the Turkish dotted/dotless-i pair is
the canonical case in which correct lowercasing depends on the language of the text, and
`LOWER` gives a formula no way to say which language it is in.

## Result and edge cases

The return kind is `Text`. Empty text returns empty text.

The signature carried by the data projection for this entry is marked as a **placeholder**, so
the argument name rendered on this page comes from Microsoft's article rather than from the
reference engine's own registry. That is a metadata gap, not a behavioural one, but it is the
reason this page states the signature from the documentation.

Arrays, empty cells and omitted arguments follow the shared call model
([chapter 03](../model/03-call-pipeline.md)).

## Errors

The article lists no error conditions, and the function has no domain to violate. The reachable
errors are the shared ones: an error value arriving as the argument propagates, and a value
kind with no text conversion surfaces `#VALUE!` under the shared coercion rules
([chapter 02](../model/02-coercion-and-lifting.md)).

## Relationships

- **`UPPER`** is the other direction, and is *not* its inverse: `LOWER(UPPER(x))` is not always
  `x`, and neither is `UPPER(LOWER(x))`, because case mapping loses information (`ß` and `SS`)
  and gains context (final sigma).
- **`PROPER`** capitalises the first letter of each word and lowercases the rest, and therefore
  contains a copy of both mappings.
- **`EXACT`** is the function to reach for when the question is whether two strings differ
  *only* by case: `EXACT(a,b)` is case-sensitive, `a=b` is not. Writing `LOWER(a)=LOWER(b)` as
  a case-insensitive comparison is a common idiom and a subtly different predicate, because it
  also folds any character pair that shares a lowercase form.

## Notes for implementers

1. **Choose and record a case-mapping table version.** Unicode's case mappings change between
   versions. An implementation that delegates to a platform's `to_lowercase` inherits whichever
   version that platform shipped, which is not a stable target.
2. **Decide the expansion policy explicitly.** If a mapping produces more than one character,
   does the function emit the expansion or leave the character unchanged? The two policies are
   visibly different on `İ` and on `ẞ`.
3. **Final sigma needs a neighbour test.** Any implementation that maps character-by-character
   without lookahead will get Greek text wrong at word ends.
4. The OxFunc reference engine at commit `473efa3` takes an explicit position on all three: it
   maps `İ` to a single `i`, leaves `ẞ` unchanged, applies the final-sigma rule using an
   alphabetic neighbour test, and falls back to the platform mapping only when that mapping
   produces exactly one character — otherwise it keeps the original. Those are implementation
   facts about OxFunc; the Handbook does not claim Excel does the same, and the shape of the
   special-case list is itself the most useful hint about which probes to run first.

## What has not been checked

No Handbook vector suite exists for `LOWER`, and no Excel-comparison evidence record names it.
Unchecked:

- `LOWER("İ")`, `LOWER("ẞ")`, and `LOWER("ΟΔΟΣ")` versus `LOWER("ΣΟΦΙΑ")` — formulas that decide
  the expansion policy and the rest of the final-sigma rule (FN-LOWER-001 records sigma before a
  full stop as final and before a combining mark as medial);
- whether the answers change with the workbook or system language, which would make `LOWER`
  locale- dependent, an axis it does not currently carry;
- `LOWER` of astral letters other than the Deseret capital U+10400, which stays unchanged
  (FN-LOWER-001), and of text containing a variation selector;
- whether the result differs at compatibility version 1: the readings in FN-LOWER-001 are at
  version 2, which Microsoft's documentation records as changing surrogate handling for other
  text functions.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `LOWER` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-LOWER-001: LOWER leaves letters above U+FFFF alone, and a combining mark stops the final sigma

*Quirk; measured; recorded 2026-11-04.* Applies to LOWER, PROPER.

LOWER did not change the Deseret capital U+10400, a cased letter above U+FFFF, either alone or
inside other text: LOWER("A𐐀Σ") is "a𐐀ς", where the Greek capital sigma at the end becomes
the final form. Greek sigma depends on what follows: followed by a full stop it takes the final
form ς, and followed by a combining mark, U+0300 or U+0301, it takes the medial form σ. PROPER
treats a character above U+FFFF as a word boundary, so PROPER("a𐐀b") is "A𐐀B", with the B
capitalised and the Deseret letter unchanged.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `LOWER(A1)` | text `𐐀` (cells) | `𐐀` | Excel 16.0.20527.20088, Windows 64-bit, AMD Ryzen 7 4800H, workbook compatibility version 2 |
| `LOWER(A1)` | text `A𐐀Σ` (cells) | `a𐐀ς` | Excel 16.0.20527.20088, Windows 64-bit, AMD Ryzen 7 4800H, workbook compatibility version 2 |
| `LOWER(A1)` | text `ΑΣ̀` (cells) | `ασ̀` | Excel 16.0.20527.20088, Windows 64-bit, AMD Ryzen 7 4800H, workbook compatibility version 2 |
| `LOWER(A1)` | text `ΑΣ.` (cells) | `ας.` | Excel 16.0.20527.20088, Windows 64-bit, AMD Ryzen 7 4800H, workbook compatibility version 2 |
| `PROPER(A1)` | text `a𐐀b` (cells) | `A𐐀B` | Excel 16.0.20527.20088, Windows 64-bit, AMD Ryzen 7 4800H, workbook compatibility version 2 |
| `PROPER(A1)` | text `aΣ́` (cells) | `Aσ́` | Excel 16.0.20527.20088, Windows 64-bit, AMD Ryzen 7 4800H, workbook compatibility version 2 |

Basis (measured): 6 recorded LOWER and 4 recorded PROPER readings, in a workbook at
compatibility version 2.

Inferred, not measured: The readings cover one Deseret letter and two combining marks. That
every letter above U+FFFF and every mark behave the same is inferred.

- Open: Behaviour at compatibility version 1 has not been read for these inputs.

Evidence:

- DnaCalc `apps/dna-onecalc/tests/acceptance.rs` at `ae13ca181b` — recorded answers pinned
- DnaCalc `crates/calc-func/tests/captures/fn3_questions/LOWER.reading.json` at `2d0eddcac8` — 6 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn3_questions/PROPER.reading.json` at `2d0eddcac8` — 4 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn3_questions/LOWER.reading.json`, fn3-lower-001 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn3_questions/LOWER.reading.json`, fn3-lower-003 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn3_questions/LOWER.reading.json`, fn3-lower-004 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn3_questions/LOWER.reading.json`, fn3-lower-005 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn3_questions/PROPER.reading.json`, fn3-proper-002 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn3_questions/PROPER.reading.json`, fn3-proper-004 at `2d0eddcac8` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning as used here |
|---|---|
| case mapping | The character-to-character function taking uppercase forms to lowercase forms |
| context-dependent mapping | A mapping whose result depends on neighbouring characters (final sigma) |
| expansion | A mapping whose output is longer than its input, measured in characters |
| placeholder signature | A projected metadata field that is not yet real and is suppressed rather than faked |

## Sources

- Microsoft, "LOWER function" —
  <https://support.microsoft.com/en-us/office/lower-function-3f21df02-a80c-44b2-afaf-81358f9fdeb4>
  (the conversion statement; "LOWER does not change characters in text that are not letters";
  the syntax and argument name). Retrieved for this page.
- Handbook `content/model/02-coercion-and-lifting.md` (to-text conversion, error propagation).
- Handbook `data/functions/FUNC.LOWER.json` — the projected entry, which marks its signature as
  a placeholder.
- OxFunc `crates/oxfunc_core/src/functions/excel_casing.rs` at commit `473efa3` — read for the
  reference engine's special-case list. Implementation fact only.
