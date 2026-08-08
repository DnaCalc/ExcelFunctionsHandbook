---
schema: efh.function-page/v1
function_id: FUNC.SEARCH
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - SEARCH versus FIND
  - Arguments
  - Wildcards
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
family: text_search_replace_family
role_in_family: The case-insensitive, wildcard-accepting locator; FIND's permissive counterpart.
---

# SEARCH

## What it computes

`SEARCH(find_text, within_text, [start_num])` returns the 1-based position, **counted from the
start of `within_text`**, of the first place at or after `start_num` where `find_text` matches.

Two properties define it, and both are in Microsoft's documentation:

1. **The match is not case sensitive.**
2. **`find_text` is a wildcard pattern**, not a literal: `?` matches any single character, `*`
   matches any sequence of characters, and `~` escapes the next `?` or `*` so it can be matched
   literally.

The second property is the one that gets missed. `SEARCH` is not "`FIND` with the case
switched off" — it is a small pattern matcher. Any `find_text` that came from user data or from
another cell may contain `*` or `?`, and if it does, `SEARCH` will not look for it literally.

The returned position is measured from the beginning of `within_text` in both cases, regardless
of `start_num`. `start_num` moves where the scan begins; it does not move the origin of the
coordinate system. `SEARCH("a", "abca", 2)` is 4, not 3.

## SEARCH versus FIND

This is the most-confused pair in the text category. The two functions have the same signature
and different semantics on two axes at once:

| | `SEARCH` | `FIND` |
|---|---|---|
| Case | **not** case sensitive | case sensitive |
| `find_text` is | a **wildcard pattern** (`?`, `*`, `~`) | a **literal** string |
| Byte sibling | `SEARCHB` | `FINDB` |
| Not found | `#VALUE!` | `#VALUE!` |
| Position returned | from the start of `within_text` | from the start of `within_text` |

Microsoft states the case axis explicitly on the `SEARCH` page — if you want a case-sensitive
search, use `FIND` — and documents the wildcard set on the same page.

The practical selection rule:

- Matching a substring **exactly as given**, including a literal `*` or `?` — use `FIND`.
- Matching **case-insensitively**, or matching a pattern — use `SEARCH`.
- Matching case-insensitively but needing `find_text` treated literally — you need `SEARCH`
  with `find_text` escaped, or `FIND` over `UPPER`ed copies of both arguments. There is no
  single function that does it.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `find_text` | The pattern to look for. Required. | — |
| `within_text` | The string to look in. Required. | — |
| `start_num` | 1-based character position at which the scan begins. Optional. | 1 |

Microsoft describes `start_num` as the way to skip a specified number of characters — the
documented use case is finding the *second* occurrence of something by restarting the scan
past the first.

`start_num` is a numeric slot subject to ordinary to-number coercion; `find_text` and
`within_text` are text slots. See
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Wildcards

The documented pattern language is three characters:

| Character | Meaning |
|---|---|
| `?` | Matches any single character |
| `*` | Matches any sequence of characters, including none |
| `~` | Escapes the following `?`, `*` or `~`, matching it literally |

What the documentation does **not** settle, and this page will not invent:

- Whether the match is leftmost-shortest or leftmost-longest when `*` could match several
  ways. Since only the start position is returned, the difference is usually invisible — but
  not always, and it is unverified.
- Whether `~` before a character that is not `?`, `*` or `~` is an error, is dropped, or is
  matched literally.
- Whether a `find_text` consisting only of `*` returns `start_num`, 1, or something else.

Each of these is on the probe list below.

## Result and edge cases

Returns `Number` — a 1-based character position.

Documented behaviours:

- `find_text` not found → `#VALUE!`. There is no "not found" sentinel and no zero return.
  Wrapping in `IFERROR` is the conventional guard.
- `start_num` outside `1 … LEN(within_text)` → `#VALUE!`.

Not documented, and therefore left open here: what `SEARCH("", A1)` returns. An empty pattern
matching at the current position is the usual convention and would give `start_num`, but
Microsoft's page does not say so and the Handbook has not checked.

"Character" is a UTF-16 code unit, consistent with `LEN` and the rest of the text family; see
[The value universe](../model/01-value-universe.md).

Empty, missing and error arguments follow the shared call model. The implementing module named
in the presence projection carries three open upstream defect streams touching this function
(`BUG-FUNC-007`, `BUG-FUNC-008`, `BUG-FUNC-016`), all concerning array and spill support, so
array-shaped arguments are unsettled.

## Errors

As documented on Microsoft's `SEARCH, SEARCHB` page:

| Error | Condition |
|---|---|
| `#VALUE!` | `find_text` is not found in `within_text`. |
| `#VALUE!` | `start_num` is not greater than 0, or is greater than the length of `within_text`. |

Non-numeric text in `start_num` and non-convertible values in the text slots surface `#VALUE!`
under the shared coercion rules, and error values in any argument propagate. The Handbook has
not verified any of this against Excel.

## Relationships

- **`FIND`** — the case-sensitive, literal counterpart. See the comparison table above. Neither
  supersedes the other; both are current.
- **[SEARCHB](FUNC.SEARCHB.md)** — the byte-position sibling, documented by Microsoft on the
  same page and published by the Handbook as its own entry.
- **[SUBSTITUTE](FUNC.SUBSTITUTE.md)** — the by-content editor. Note the asymmetry that trips
  people: `SEARCH` is case-**in**sensitive and takes wildcards, while `SUBSTITUTE` is
  case-**sensitive** and takes none. A formula that locates with `SEARCH` and edits with
  `SUBSTITUTE` is using two different matching rules on the same string.
- **[REPLACE](FUNC.REPLACE.md)** — the by-position editor, and the usual consumer of a `SEARCH`
  result.
- **`ISNUMBER(SEARCH(x, y))`** is the idiomatic case-insensitive "contains" test, precisely
  because `SEARCH` errors rather than returning zero.
- **`XMATCH`** with its wildcard match mode does pattern matching over a lookup array rather
  than within a single string — a different job that reuses the same wildcard vocabulary.
- **`REGEXTEST` / `REGEXEXTRACT`** are the modern route when the three-character wildcard
  language is not enough.

## Notes for implementers

The wildcard engine is the whole implementation. Case folding is the easy half; deciding what
`*` and `?` mean over UTF-16 code units is the hard half, because `?` matching "any single
character" has to choose between one code unit and one scalar value, and those differ on every
astral character.

Case-insensitivity is locale-sensitive in the same way `PROPER`'s capitalization is: Turkish
dotted and dotless i, and full-versus-simple case folding, both change what matches. An
invariant fold is a defensible default and is a decision to record, not a neutral one.

`start_num` bounds are checked before the search, and the check is against `LEN(within_text)`,
not against `LEN(within_text) + 1`, so scanning from just past the end is documented as an
error rather than as a not-found. That is a genuine asymmetry with the way many string
libraries behave.

The returned position is absolute, not relative to `start_num`. An implementation that runs the
search over a suffix has to add the offset back.

## What has not been checked

No Handbook vector suite exists for `SEARCH`, and no Excel-comparison evidence record names
it. Nobody has checked this function against Excel within the Handbook's record. Everything
above marked as documented comes from Microsoft's page; the case-insensitivity rule, the
wildcard set and the two error conditions are documented, and nothing else on this page is.

Inputs worth probing first:

1. **`SEARCH("*", "abc")` and `SEARCH("?", "abc")`** — the two cheapest probes that prove
   `find_text` is a pattern rather than a literal, which is the single fact most readers get
   wrong.
2. **`SEARCH("~*", "a*b")`** versus `SEARCH("*", "a*b")` — the escape mechanism, and whether it
   works as documented.
3. **`SEARCH("~a", "abc")`** — a tilde before an ordinary character, which the documentation
   does not cover.
4. **`SEARCH("", "abc")` and `SEARCH("", "abc", 2)`** — the empty-pattern case.
5. **`SEARCH("a", "abc", 4)` and `SEARCH("a", "abc", 0)`** — the two ends of the documented
   `start_num` bound, including the just-past-the-end position that a string library would
   usually accept.
6. **Case folding beyond ASCII**: `SEARCH("İ", "i")` and `SEARCH("ß", "SS")`, and the same
   under a Turkish default language.
7. **`SEARCH("?", "😀")`** — whether `?` consumes one code unit or one character.
8. **Array arguments in each position**, given the three open defect streams on this module.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| wildcard pattern | The `?`/`*`/`~` matching language `find_text` is interpreted in |
| absolute position | The result is counted from the start of `within_text`, not from `start_num` |
| case fold | The case-insensitive comparison; its exact rule is locale-sensitive and unverified here |

## Sources

- Microsoft, "SEARCH, SEARCHB functions" —
  <https://support.microsoft.com/en-us/office/search-function-9ab04538-0e55-4719-a72e-b6f54513b495>
  (signature, case-insensitivity and the pointer to `FIND` for case-sensitive matching, the
  wildcard set including the tilde escape, the `start_num` skip usage, and the two documented
  `#VALUE!` conditions).
- Handbook, [The value universe](../model/01-value-universe.md) — text as UTF-16 code units.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — coercion of the
  numeric and text argument slots and error propagation.
- Handbook projection `data/presence/FUNC.SEARCH.json` — implementing module, sibling set, and
  the `BUG-FUNC-007`, `BUG-FUNC-008`, `BUG-FUNC-016` defect streams.
