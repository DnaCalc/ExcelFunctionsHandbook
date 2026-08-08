---
schema: efh.function-page/v1
function_id: FUNC.RIGHTB
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
role_in_family: The byte-counting tail slice; RIGHT's DBCS twin.
---

# RIGHTB

## What it computes

`RIGHTB(text, [num_bytes])` returns the last `num_bytes` **bytes** of `text`.

It is [RIGHT](FUNC.RIGHT.md) with one unit changed, and the change only takes effect under one
condition. Microsoft's rule for the whole B family:

> The B functions count each double-byte character as 2 **only when a DBCS language has been
> enabled for editing and set as the default language**. Otherwise they count each character
> as 1.

So on most installations `RIGHTB` is `RIGHT`. On a machine whose default editing language is
Japanese, Chinese (Simplified or Traditional) or Korean, the same call takes a different — and
generally shorter — slice of the same string. The active default language is an unlisted input
to this function, and any recorded result for it that omits that setting is incomplete.

The bytes are those of the **DBCS code page for the active language** — Shift-JIS, GBK, Big5,
EUC-KR — not UTF-8 and not UTF-16.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `text` | The string to slice. Required. | — |
| `num_bytes` | How many bytes to take from the end. Optional; must be zero or greater. | 1 |

The signature is `RIGHT`'s with `num_chars` renamed. The count is optional here just as it is
on `RIGHT`, and defaults to 1 — which under DBCS means "one byte", i.e. potentially half a
character. That is not a hypothetical: `RIGHTB(A1)` on a string ending in a double-byte
character is the partial-character case, reached by the shortest possible call.

## Result and edge cases

Returns `Text`.

The rules Microsoft documents for `RIGHT` are stated on the same page for `RIGHTB` with
"characters" replaced by "bytes": the count must be non-negative, a count greater than the
length of `text` returns all of `text`, and an omitted count means 1.

The distinctively `RIGHTB` question is undocumented: **what happens when the byte window
begins inside a double-byte character.** Because the window is anchored at the end and
measured backwards in bytes, an odd `num_bytes` over double-byte text lands mid-character by
construction. Excel could drop the partial character, return its orphaned byte, round the
boundary outward to include the whole character, or error. Microsoft's page names none of
these, and the Handbook does not know which it is.

Empty, missing and error arguments follow the shared call model; see
[Coercion and lifting](../model/02-coercion-and-lifting.md). The implementing module named in
the presence projection carries an open upstream defect stream on array positions and count
arguments in the slice family (`BUG-FUNC-007`), so array-shaped arguments are unsettled.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | `num_bytes` is negative. | Implied by the documented non-negativity requirement; the page states the requirement, not the error code. |
| `#VALUE!` | Non-numeric text in `num_bytes`, or a value that cannot convert to text in `text`. | Shared coercion rules, not `RIGHTB`-specific. |

Whether a partial-character window is an error condition at all is the open question above.

## Relationships

- **[RIGHT](FUNC.RIGHT.md)** — the character-counting twin, documented by Microsoft on the same
  page. The Handbook publishes them as two entries because they count in two different units.
- **`LEFTB`** — the head-anchored byte slice, and the one place where the partial-character
  question can be studied from the other end of the string.
- **`LENB`, `FINDB`, `SEARCHB`, [MIDB](FUNC.MIDB.md), [REPLACEB](FUNC.REPLACEB.md)** — the rest
  of the B family, sharing one implementing module and this page's central caveat.
- **`LENB`** is the only function that produces coordinates in the same byte space, so it is
  the companion for reasoning about `RIGHTB` at all.

## Notes for implementers

Backwards byte counting over a variable-width encoding is not a symmetric operation. Finding
the byte `n - k + 1` requires knowing the byte length of every character in the string, so
`RIGHTB` cannot be implemented as a reverse scan of characters — it needs the full byte
accounting that `LENB` computes.

The default of 1 makes the partial-character case the *cheapest* call to write, not the rarest,
which is an argument for deciding it deliberately rather than letting it fall out of an
implementation detail.

A test vector for `RIGHTB` that does not record the default editing language records nothing.
The same inputs have two legitimately different correct answers.

## What has not been checked

No Handbook vector suite exists for `RIGHTB`, and no Excel-comparison evidence record names
it. Nobody has checked this function against Excel within the Handbook's record — and the
configuration in which its distinguishing behaviour exists at all, a DBCS default editing
language, has not been exercised by the Handbook in any form.

Inputs worth probing first:

1. **The two-machine baseline**: `RIGHTB("日本語", 2)` on an English-default installation and
   on a Japanese-default installation. Until those differ as documented, nothing else here is
   worth measuring.
2. **The odd count**: `RIGHTB("日本語", 1)` and the bare `RIGHTB("日本語")` under DBCS — the
   partial-character case, reached by the default.
3. **`RIGHTB("日本語", 3)`** — one whole character plus one orphaned byte, which distinguishes
   "drop the partial character" from "round outward".
4. **Mixed-width text**: `RIGHTB("日A", 2)` and `RIGHTB("A日", 2)`, confirming byte accounting
   is per character rather than a whole-string mode.
5. **`num_bytes` greater than `LENB(text)`**, to confirm the clamp survives the unit change.
6. **`RIGHTB("abc", -1)`**, the negative case whose error code this page infers rather than
   cites, probed alongside `RIGHT("abc", -1)` so the answers can be compared.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| DBCS | Double-byte character set; the encodings in which one character can cost two bytes |
| default editing language | The host setting that decides whether B functions count bytes or characters |
| partial-character window | A byte window whose edge falls inside a double-byte character |

## Sources

- Microsoft, "RIGHT, RIGHTB functions" —
  <https://support.microsoft.com/en-us/office/right-function-240267ee-9afa-4639-a02b-f19e1786cf2f>
  (signature, the non-negative requirement, the over-long clamp, the default of 1, and the
  DBCS default-language condition for the B family).
- Handbook, [The value universe](../model/01-value-universe.md) — worksheet text as UTF-16
  code units, which the byte view must be reconciled against.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — numeric-argument
  coercion and error propagation.
- Handbook projection `data/presence/FUNC.RIGHTB.json` — the shared `text_b_compat_family`
  implementing module, its sibling set, and the `BUG-FUNC-007` defect stream.
