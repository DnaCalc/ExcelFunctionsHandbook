---
schema: efh.function-page/v1
function_id: FUNC.REPLACE
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
  - REPLACE versus SUBSTITUTE
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
family: text_search_replace_family
role_in_family: The positional writer — overwrites a character window addressed by start and length.
---

# REPLACE

## What it computes

`REPLACE(old_text, start_num, num_chars, new_text)` returns `old_text` with the window of
`num_chars` characters beginning at position `start_num` removed and `new_text` put in its
place. Positions are 1-based.

Equivalently, in terms of the slicing functions:

```
REPLACE(t, s, k, new)  =  LEFT(t, s - 1) & new & MID(t, s + k, LEN(t))
```

Two consequences of that identity are worth reading off it directly, because they are the two
things people get wrong about this function:

1. **`REPLACE` does not look at what it is replacing.** It never inspects the characters in
   the window. It cannot fail to find anything, because it does not search.
2. **The result length is not the input length.** `new_text` need not be `num_chars` long. With
   `num_chars` of 0, `REPLACE` inserts without deleting; with `new_text` empty, it deletes
   without inserting. These are not tricks — they fall straight out of the definition, and
   they are the idiomatic way to do both operations.

`REPLACEB` is the byte-addressed sibling; see [REPLACEB](FUNC.REPLACEB.md).

## Arguments

| Argument | Meaning |
|---|---|
| `old_text` | The string to modify. Required. |
| `start_num` | 1-based position of the first character to replace. Required. |
| `num_chars` | How many characters to remove. Required. |
| `new_text` | What to put in their place. Required. |

All four are required — there is no "replace to the end" default and no optional argument.

`start_num` and `num_chars` are numeric slots subject to ordinary to-number coercion; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

The commonly misread position is `num_chars`. It is a **count of characters to remove from
`old_text`**, not an end position and not the length of `new_text`. `REPLACE("abcdef", 2, 3,
"XY")` removes `bcd` and yields `aXYef`.

## Result and edge cases

Returns `Text`.

Microsoft's `REPLACE` page states the signature and the meaning of each argument but does not
enumerate boundary behaviours the way the `MID` page does. The following are therefore
*expected* rather than documented, and the Handbook has not checked any of them:

- `start_num` past the end of `old_text` — plausibly an append; unverified.
- `num_chars` running past the end of `old_text` — plausibly a clamp to the end; unverified.
- `num_chars` of 0 — insertion, as above; this follows from the definition but is not stated
  on the page.
- `start_num` less than 1 — the sibling functions document `#VALUE!` for this; `REPLACE`'s
  page does not.

Empty, missing and error arguments follow the shared call model. The implementing module named
in the presence projection carries three open upstream defect streams touching this function
(`BUG-FUNC-007`, `BUG-FUNC-008`, `BUG-FUNC-016`), all concerning array and spill support, so
array-shaped arguments are a known soft spot rather than settled behaviour.

## REPLACE versus SUBSTITUTE

These two are the most-confused pair in the text category, and the distinction is clean once
stated. Microsoft states it on the `SUBSTITUTE` page itself:

> Use `SUBSTITUTE` when you want to replace specific text in a text string; use `REPLACE` when
> you want to replace any text that occurs in a specific location in a text string.

| | `REPLACE` | `SUBSTITUTE` |
|---|---|---|
| Addresses the target by | **position** (`start_num`, `num_chars`) | **content** (`old_text` matched literally) |
| Number of edits | exactly one window | every occurrence, or one chosen occurrence |
| If the target is not there | meaningless — there is no target to find | returns the input unchanged |
| Case sensitivity | not applicable | case-**sensitive** |
| Wildcards | not applicable | none |
| Length preserved | no | no |

The one-line test: if your formula computes `start_num` with `FIND` or `SEARCH`, you probably
wanted `SUBSTITUTE`. If your positions come from a fixed-width record layout, you wanted
`REPLACE`.

See [SUBSTITUTE](FUNC.SUBSTITUTE.md) for the other side of this pair.

## Errors

Microsoft's `REPLACE, REPLACEB` page does not carry an error table. The errors reachable here
are the shared ones: an error value in any argument propagates, and non-numeric text in
`start_num` or `num_chars` surfaces `#VALUE!` under the shared coercion rules. A result
exceeding the 32,767 code-unit text cap is the other candidate — the Handbook's value-universe
chapter records `#VALUE!` for the over-cap formula path observed with `REPT`, but whether
`REPLACE` publishes the same way is unverified.

## Relationships

- **[SUBSTITUTE](FUNC.SUBSTITUTE.md)** — the by-content counterpart. See the section above.
- **[MID](FUNC.MID.md)** — the same `(start_num, num_chars)` addressing, reading instead of
  writing. `MID` extracts the window `REPLACE` overwrites.
- **[REPLACEB](FUNC.REPLACEB.md)** — the byte-addressed sibling, documented by Microsoft on the
  same page and published by the Handbook as a separate entry.
- **`FIND` and `SEARCH`** are what supplies `start_num` when the window is not known in
  advance. `SEARCH` is case-insensitive and takes wildcards; `FIND` is neither. Choosing
  between them changes what `REPLACE` overwrites.
- **`TEXTBEFORE` / `TEXTAFTER`** cover many `REPLACE`-with-a-computed-position formulas more
  directly.

## Notes for implementers

The three-piece concatenation identity above is the implementation, and the only subtleties
are at its edges: what `LEFT(t, s-1)` does when `s-1` exceeds `LEN(t)`, and what `MID(t, s+k,
LEN(t))` does when `s+k` exceeds it. Both must be defined so that a window entirely past the
end degrades gracefully rather than producing a `#VALUE!` from an intermediate step.

Because `REPLACE` can grow its input, it is one of the functions that can manufacture a string
over the 32,767 code-unit cap from inputs that are each individually legal. Whatever the
publication behaviour turns out to be, it has to be applied at the function's own boundary and
not left to the cell.

Counting is in UTF-16 code units, consistent with `LEN` and `MID`. A `REPLACE` window can
therefore split a surrogate pair, and the implementation has to survive producing ill-formed
UTF-16 — the Handbook's value-universe chapter records that state as reachable in practice.

## What has not been checked

No Handbook vector suite exists for `REPLACE`, and no Excel-comparison evidence record names
it. Nobody has checked this function against Excel within the Handbook's record. Microsoft's
page for it is unusually thin on remarks, so the fraction of this page that rests on
documentation rather than inference is smaller here than on most text-function pages — the
`REPLACE versus SUBSTITUTE` table's left column is largely derived from the signature.

Inputs worth probing first:

1. **`REPLACE("abcdef", 0, 2, "X")` and `REPLACE("abcdef", -1, 2, "X")`** — the `start_num < 1`
   case, which the sibling functions document as `#VALUE!` and this one does not.
2. **`REPLACE("abc", 10, 2, "X")`** — a window entirely past the end. Append, unchanged input,
   or error are all plausible and the page does not choose.
3. **`REPLACE("abcdef", 3, 0, "XY")`** — the zero-length insert, which is the function's most
   useful undocumented behaviour.
4. **`REPLACE("abcdef", 3, 100, "XY")`** — the over-long window and whether it clamps.
5. **Fractional `start_num` and `num_chars`** — `REPLACE("abcdef", 2.9, 1.9, "X")`.
6. **An over-cap result** built from `REPT`, to see whether `REPLACE` errors or truncates at
   the text cap.
7. **Array arguments in each position**, given the three open defect streams on this module.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| window | The `num_chars` run starting at `start_num` that `REPLACE` overwrites |
| by-position | Addressing the edit by coordinates rather than by matching content |
| text cap | The 32,767 UTF-16 code-unit limit on worksheet text |

## Sources

- Microsoft, "REPLACE, REPLACEB functions" —
  <https://support.microsoft.com/en-us/office/replace-function-8d799074-2425-4a8a-84bc-82472868878a>
  (signature and argument meanings).
- Microsoft, "SUBSTITUTE function" —
  <https://support.microsoft.com/en-us/office/substitute-function-6434944e-a904-4336-a9b0-1e58df3bc332>
  (the quoted by-content versus by-location guidance).
- Handbook, [The value universe](../model/01-value-universe.md) — the 32,767 code-unit text
  cap, the observed `#VALUE!` on the over-cap formula path, and surrogate splitting.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — numeric-argument
  coercion and error propagation.
- Handbook projection `data/presence/FUNC.REPLACE.json` — implementing module and the
  `BUG-FUNC-007`, `BUG-FUNC-008`, `BUG-FUNC-016` defect streams.
