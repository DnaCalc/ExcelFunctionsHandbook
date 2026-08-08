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

- `LOWER("İ")`, `LOWER("ẞ")`, and `LOWER("ΟΔΟΣ")` versus `LOWER("ΣΟΦΙΑ")` — three formulas that
  decide the expansion policy and the final-sigma rule at once;
- whether the answers change with the workbook or system language, which would make `LOWER`
  locale- dependent, an axis it does not currently carry;
- `LOWER` of an astral character with a case mapping (Deseret and Adlam are the usual test
  alphabets) and of text containing a variation selector;
- whether the result changes under Compatibility Version 2, which Microsoft's documentation
  records as changing surrogate handling for other text functions.

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
