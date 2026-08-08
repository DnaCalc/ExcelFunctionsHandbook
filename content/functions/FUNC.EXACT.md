---
schema: efh.function-page/v1
function_id: FUNC.EXACT
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
family: exact_fn
role_in_family: "The sole member: case-sensitive text equality, returned as a logical."
---

## What it computes

`EXACT` compares two text values and returns `TRUE` if they are the same and `FALSE` otherwise.
Microsoft's article adds the two qualifications that make it a distinct function rather than a
synonym for `=`: the comparison **is case-sensitive**, and it **ignores formatting
differences**.

Both qualifications are worth stating precisely.

*Case-sensitive* means `EXACT("Word","word")` is `FALSE`, where the equality operator
`A1="word"` would be `TRUE` for the same pair. This is the whole reason the function exists:
Excel's `=` comparison of text is case-insensitive, and there is no operator that is not.

*Ignores formatting* means the comparison is over the stored text, not over its presentation. A
cell displaying `1.50` in a currency format and a cell displaying `1.5` hold the same number
and the same text projection of it; a bold cell and a plain cell holding `abc` compare equal.

What the documentation does not say is what "the same" means below the level of characters.
Text in the Handbook's value model is a sequence of UTF-16 code units
([chapter 01](../model/01-value-universe.md)), and two strings can look identical, print
identically, and still differ:

- **é as one code point** (U+00E9) versus **e followed by a combining acute** (U+0065 U+0301).
  These are canonically equivalent under Unicode normalisation and different as code-unit
  sequences.
- The same glyph reached through different variation selectors or through compatibility
  characters.

An implementation that normalises before comparing and one that compares code units answer
differently on those inputs. Microsoft's article does not choose; it is the sharpest unresolved
question on this page.

## Arguments

`text1`, `text2` — both required; the article describes them simply as the first and second
text strings. Both are text arguments, so a number or a logical converts to text before
comparison. Two consequences readers hit regularly:

- `EXACT(1,"1")` compares the *text* `1` against the text `1`. The number's General rendering
  is what is compared, so this is a comparison of renderings, not of values.
- `EXACT(A1,"")` with `A1` an empty cell compares empty text against empty text. Empty and
  empty-string are different value kinds at the call boundary
  ([chapter 01](../model/01-value-universe.md)) but they converge here, which is why `EXACT`
  cannot distinguish a blank cell from a cell containing `=""`. Use `ISBLANK` for that
  question.

There is no third argument: no locale, no normalisation form, no "ignore whitespace" option.

## Result and edge cases

The return kind is `Logical` — `TRUE` or `FALSE`, not 1 or 0, though it converts to those in
numeric contexts.

- Leading and trailing spaces count. Microsoft's own example set includes `"w ord"` versus
  `"word"` as `FALSE`; a difference in internal spacing is a difference.
- Two empty texts compare `TRUE`.
- The function is symmetric and reflexive on any input it accepts. It is not transitive across
  coercion boundaries in the way readers assume: `EXACT(1,"1")` and `EXACT("1","1.0")` do not
  compose, because the coercion happens once, on entry.

## Errors

The article lists no error conditions of the function's own. Reachable errors are the shared
ones: an error value in either argument propagates
([chapter 02](../model/02-coercion-and-lifting.md)), and a value kind with no text conversion
surfaces `#VALUE!` under the shared coercion rules.

Note the asymmetry with the comparison operator: `="a"="a"` never errors, while `EXACT`
inherits the whole coercion pipeline and therefore can.

## Relationships

- **`=` (the equality operator, `FUNC.OP_EQ`)** is the case-insensitive counterpart. Choosing
  between them is the single most common reason to reach for `EXACT`.
- **`FIND` / `SEARCH`** carry the same case-sensitivity split for substring search: `FIND` is
  case-sensitive, `SEARCH` is not. `EXACT` is to `=` as `FIND` is to `SEARCH`.
- **`LOWER` / `UPPER`** are the usual way to build a case-insensitive comparison explicitly.
  Note that `EXACT(LOWER(a),LOWER(b))` and `a=b` are not the same predicate — case folding
  merges pairs that Excel's own comparison may or may not merge.
- **`DELTA`** is the numeric analogue (equality as a returned value rather than a branch), for
  numbers rather than text.

## Notes for implementers

1. **Pick a comparison level and declare it.** Code-unit equality, code-point equality,
   NFC-then- compare, and full case-sensitive collation are four different functions. They
   agree on ASCII, so ASCII tests cannot distinguish them.
2. **Do not use a locale collator.** Collation is designed to make visually-equivalent strings
   compare equal and is exactly the wrong tool here; `EXACT` is a sameness test, not an
   ordering.
3. **Ill-formed UTF-16 must not panic.** The value-universe chapter records lone surrogates as
   a reachable state in Excel text, so the comparison has to be defined over raw code units
   even if the rest of the implementation decodes.
4. The OxFunc reference engine at commit `473efa3` compares UTF-16 code-unit sequences
   directly, with no normalisation, and its own tests pin the precomposed-versus-combining pair
   as `FALSE`. Implementation fact about OxFunc only.

## What has not been checked

No Handbook vector suite exists for `EXACT`, and no Excel-comparison evidence record names it.
Unchecked:

- `EXACT("é","é")` with the two arguments built as precomposed and as base-plus-combining — the
  one probe that decides between code-unit comparison and normalising comparison;
- `EXACT` on identical astral characters and on a lone surrogate pasted into a cell;
- `EXACT` on a Turkish dotted/dotless i pair, which distinguishes code-unit comparison from any
  locale-aware comparison;
- `EXACT` of a number against its own `TEXT` rendering under a non-English decimal separator,
  which would show whether the to-text conversion at this call site is locale-sensitive;
- whether `EXACT` lifts elementwise over array arguments in both positions, and how shapes
  broadcast.

## Page vocabulary

| Term | Meaning as used here |
|---|---|
| code-unit equality | Sameness decided by comparing UTF-16 code-unit sequences with no normalisation |
| canonical equivalence | Unicode's relation between sequences that denote the same abstract text |
| case-sensitive | Distinguishing uppercase from lowercase; the property `=` does not have |

## Sources

- Microsoft, "EXACT function" —
  <https://support.microsoft.com/en-us/office/exact-function-d3087698-fc15-4a15-9631-12575cf29926>
  (the comparison statement; case sensitivity; that formatting differences are disregarded; the
  `"w ord"` versus `"word"` example). Retrieved for this page.
- Handbook `content/model/01-value-universe.md` (text as UTF-16 code units; Empty versus empty
  text) and `content/model/02-coercion-and-lifting.md` (to-text conversion, error propagation).
- OxFunc `crates/oxfunc_core/src/functions/exact_fn.rs` at commit `473efa3` — read for the
  reference engine's comparison level. Implementation fact only.
