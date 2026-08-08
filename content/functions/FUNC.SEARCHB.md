---
schema: efh.function-page/v1
function_id: FUNC.SEARCHB
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - SEARCHB versus FINDB
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
family: text_b_compat_family
role_in_family: The byte-position locator; SEARCH's DBCS twin, and the one that returns coordinates FINDB and REPLACEB can consume.
---

# SEARCHB

## What it computes

`SEARCHB(find_text, within_text, [start_num])` locates `find_text` inside `within_text` and
returns the 1-based **byte** position at which it starts, counted from the beginning of
`within_text`.

Everything that makes [SEARCH](FUNC.SEARCH.md) what it is carries over unchanged: the match is
**not case sensitive**, and `find_text` is a **wildcard pattern** in which `?` matches any
single character, `*` matches any sequence, and `~` escapes a following `?` or `*`. The B in
the name changes the coordinate system of the answer and the coordinate system of `start_num`.
It does not change the matching rule.

And the coordinate change itself is conditional. Microsoft's rule for the whole B family:

> The B functions count each double-byte character as 2 **only when a DBCS language has been
> enabled for editing and set as the default language**. Otherwise they count each character
> as 1.

So on a non-DBCS-default installation `SEARCHB` and `SEARCH` return the same numbers for the
same inputs. On a Japanese-, Chinese- or Korean-default installation they return different
numbers, because the same match sits at a different byte offset than character offset. The
active default language is an unlisted input to this function.

## SEARCHB versus FINDB

The same two-axis distinction as the character pair, restated because it is exactly as
confusable at the byte level:

| | `SEARCHB` | `FINDB` |
|---|---|---|
| Case | **not** case sensitive | case sensitive |
| `find_text` is | a **wildcard pattern** (`?`, `*`, `~`) | a **literal** string |
| Returns | byte position | byte position |
| Character twin | [SEARCH](FUNC.SEARCH.md) | `FIND` |

Choosing `SEARCHB` over `FINDB` for the case-insensitivity and then passing a `find_text` that
contains a literal `*` is the same trap it is on the character functions, and it is no easier
to spot here.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `find_text` | The pattern to look for. Required. | — |
| `within_text` | The string to look in. Required. | — |
| `start_num` | 1-based **byte** position at which the scan begins. Optional. | 1 |

`start_num` is in bytes, which is the argument most likely to be wrong in a formula ported from
`SEARCH`. A `start_num` computed from `LEN` is a character coordinate; feeding it to `SEARCHB`
is a unit error that produces plausible wrong answers rather than an error value. `LENB` is
the function that produces the right coordinates.

## Result and edge cases

Returns `Number` — a 1-based byte position.

The behaviours Microsoft documents for `SEARCH` are stated on the same page for `SEARCHB`:
`find_text` not found is `#VALUE!`, and a `start_num` outside the valid range is `#VALUE!` —
with the range measured in bytes here.

The distinctively `SEARCHB` questions, none of which the documentation answers:

- **Does `start_num` have to fall on a character boundary?** An odd `start_num` inside a
  double-byte character is reachable by ordinary arithmetic and is undefined here.
- **Does `?` match one byte or one character?** Under DBCS these differ, and the wildcard
  language is documented in terms of characters while the result is measured in bytes. This is
  the sharpest unresolved question on the page.
- **Is the returned position always on a character boundary?** It should be if matches only
  begin at character boundaries, but that is an inference from a rule nobody has stated.

Empty, missing and error arguments follow the shared call model; see
[Coercion and lifting](../model/02-coercion-and-lifting.md). The implementing module named in
the presence projection carries an open upstream array-support defect stream (`BUG-FUNC-016`),
so array-shaped arguments are unsettled.

## Errors

As documented on Microsoft's `SEARCH, SEARCHB` page:

| Error | Condition |
|---|---|
| `#VALUE!` | `find_text` is not found in `within_text`. |
| `#VALUE!` | `start_num` is not greater than 0, or is greater than the length of `within_text` — in bytes for `SEARCHB`. |

Shared coercion errors and error propagation apply as elsewhere. The Handbook has not verified
any of this against Excel.

## Relationships

- **[SEARCH](FUNC.SEARCH.md)** — the character-position twin, documented by Microsoft on the
  same page and published by the Handbook as its own entry.
- **`FINDB`** — the case-sensitive, literal byte locator. See the comparison above.
- **`LENB`, [MIDB](FUNC.MIDB.md), [REPLACEB](FUNC.REPLACEB.md), `LEFTB`,
  [RIGHTB](FUNC.RIGHTB.md)** — the rest of the B family. `SEARCHB` is the family's coordinate
  producer: its output is exactly what `MIDB` and `REPLACEB` expect as `start_num`, and mixing
  a `SEARCH` result into a `MIDB` call is the classic unit mismatch.
- **[SUBSTITUTE](FUNC.SUBSTITUTE.md)** — there is no `SUBSTITUTEB`, and that is not an
  oversight: by-content substitution never needs byte coordinates. Under DBCS it is usually the
  safer tool.

## Notes for implementers

The function has to hold two coordinate systems at once: the match runs over characters (the
wildcard language is defined in characters) and the answer is reported in bytes. That mapping
has to be built from the same code-page tables `LENB` uses, or the family will not agree with
itself.

The `?`-matches-what question is not an implementation detail to be settled quietly. Pick an
answer, document it, mark it unverified, and make it consistent with whatever `MIDB` does at
partial-character boundaries.

A `SEARCHB` test vector without the default editing language recorded is not a test vector.

## What has not been checked

No Handbook vector suite exists for `SEARCHB`, and no Excel-comparison evidence record names
it. Nobody has checked this function against Excel within the Handbook's record — and the
configuration under which it differs from `SEARCH` at all has not been exercised by the
Handbook in any form.

Inputs worth probing first:

1. **The two-machine baseline**: `SEARCHB("語", "日本語")` on an English-default installation
   (expected 3) and on a Japanese-default installation (expected 5). One formula, two answers,
   and the whole documented claim.
2. **`SEARCHB("?", "日本語")` under DBCS** — whether `?` consumes one byte or one character,
   and what position it reports.
3. **`SEARCHB("*語", "日本語")`** — `*` over multi-byte text, and whether the reported start is
   a character boundary.
4. **An odd `start_num`**: `SEARCHB("語", "日本語", 2)` under DBCS, which begins inside the
   first character.
5. **`start_num` at `LENB(within_text)` and one past it**, to confirm the documented bound is
   measured in bytes.
6. **Case folding on full-width Latin letters** (`Ａ` versus `A`), which is where DBCS text and
   case-insensitivity actually interact.
7. **Cross-checks against `LENB` and `MIDB`** on the same strings, confirming the three
   functions share one byte coordinate system.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| DBCS | Double-byte character set; the encodings in which one character can cost two bytes |
| default editing language | The host setting that decides whether B functions count bytes or characters |
| coordinate producer | `SEARCHB`'s role: its result is the `start_num` other B functions consume |

## Sources

- Microsoft, "SEARCH, SEARCHB functions" —
  <https://support.microsoft.com/en-us/office/search-function-9ab04538-0e55-4719-a72e-b6f54513b495>
  (signature, case-insensitivity, the wildcard set with the tilde escape, the documented
  `#VALUE!` conditions, and the DBCS default-language condition for the B family).
- Handbook, [The value universe](../model/01-value-universe.md) — worksheet text as UTF-16
  code units, which the byte view must be reconciled against.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — argument coercion and
  error propagation.
- Handbook projection `data/presence/FUNC.SEARCHB.json` — the shared `text_b_compat_family`
  implementing module, its sibling set, and the `BUG-FUNC-016` defect stream.
