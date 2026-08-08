---
schema: efh.function-page/v1
function_id: FUNC.MID
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
family: text_slice_family
role_in_family: The interior slice — the only member that takes both a start position and a length.
---

# MID

## What it computes

`MID(text, start_num, num_chars)` returns the contiguous run of `num_chars` characters of
`text` beginning at position `start_num`, where positions are counted from 1 at the first
character.

Written against the character positions of `text`, with `n = LEN(text)`:

```
MID(text, s, k)  =  characters at positions s, s+1, …, min(s + k - 1, n)
```

and the empty string when `s > n`. Note what the clamp does and does not do: the **end** of the
window is clamped to the end of `text`, so asking for more characters than remain is legal and
simply yields the tail. The **start** is not clamped, so `s < 1` is not "start at the
beginning" — it is an error (see Errors).

"Character" here means one UTF-16 code unit, the unit the Handbook's value universe uses for
worksheet text — the same unit `LEN` counts. A character outside the Basic Multilingual Plane
occupies two code units, so `MID` can in principle cut a surrogate pair in half. See
[The value universe](../model/01-value-universe.md), "Text, exactly".

## Arguments

| Argument | Meaning |
|---|---|
| `text` | The string to slice. Required. |
| `start_num` | 1-based position of the first character to take. Required. |
| `num_chars` | How many characters to take. Required, and must not be negative. |

All three arguments are required — `num_chars` has no default, unlike the optional second
argument of `LEFT` and `RIGHT`. This is the position where the family is least uniform, and it
is a frequent source of confusion: `RIGHT(a)` is legal and means one character, `MID(a, 2)` is
not legal at all.

`start_num` and `num_chars` are numeric arguments, so ordinary to-number coercion applies to
whatever arrives in those slots (a logical, numeric text, a referenced number). The general
rules are in [Coercion and lifting](../model/02-coercion-and-lifting.md); nothing about them is
specific to `MID`. Fractional values are the interesting case, and the Handbook has not checked
how they are handled here — see "What has not been checked".

## Result and edge cases

Returns `Text`.

The boundary behaviours Microsoft documents on the `MID` page:

- `start_num` greater than the length of `text` → the empty string `""`, not an error.
- `start_num` within `text` but `start_num + num_chars` running past the end → the characters
  from `start_num` to the end of `text`.
- `num_chars` of zero → the empty string.

Empty, missing and error arguments follow the shared call model rather than any `MID`-specific
rule; see [The call pipeline](../model/03-call-pipeline.md) and
[Coercion and lifting](../model/02-coercion-and-lifting.md). An error value arriving in any
argument propagates.

Array arguments are the one place where behaviour here is genuinely unsettled rather than
merely undocumented. The Handbook's projected presence data for this function names an open
upstream defect stream about array positions and count arguments in the text-slice family
(`BUG-FUNC-007`), so the array/spill shape of `MID` is a known soft spot, not a solved one.

## Errors

As documented on Microsoft's `MID` page:

| Error | Condition |
|---|---|
| `#VALUE!` | `start_num` is less than 1. |
| `#VALUE!` | `num_chars` is negative. |

Beyond those two, `#VALUE!` is also the expected result of a non-numeric-text value in a
numeric argument slot under the shared coercion rules, and any error value passed in
propagates. The Handbook has not verified the error surface against Excel.

## Relationships

- `LEFT` and `RIGHT` are the two anchored slices; `MID` is the free one. `LEFT(t, k)` is
  `MID(t, 1, k)`, and `RIGHT(t, k)` is `MID(t, LEN(t) - k + 1, k)` for `0 <= k <= LEN(t)` —
  the second identity is exactly why `RIGHT` is not simply `MID` with a negative start.
- `MIDB` is the byte-counting sibling. Microsoft documents both on the same page and the
  Handbook splits them into two entries; see [MIDB](FUNC.MIDB.md).
- `REPLACE` is the positional counterpart in the other direction: `MID` reads out a window by
  position, `REPLACE` writes over a window by position, with the same
  `(start_num, num_chars)` addressing.
- `TEXTBEFORE`, `TEXTAFTER` and `TEXTSPLIT` are the modern delimiter-based way to do what
  `MID(…, SEARCH(…), …)` formulas were traditionally written to do. `MID` is not superseded —
  positional slicing is still its own job — but a formula that computes `start_num` from a
  `SEARCH` result is usually clearer written with the newer functions.

## Notes for implementers

The `start_num < 1` rule is the trap. It is tempting to saturate the window at both ends
symmetrically, which silently turns `MID(t, 0, 3)` into a two-character result instead of the
documented `#VALUE!`. Clamp the end; validate the start.

Counting in UTF-16 code units rather than Unicode scalar values is not an implementation
convenience, it is the specified unit: an implementation that counts scalar values will
disagree with Excel on any string containing an astral character, and will disagree in a way
that only shows up on emoji and rarer CJK.

## What has not been checked

No Handbook vector suite exists for `MID`, and no Excel-comparison evidence record names it.
Nobody has checked this function's behaviour against Excel within the Handbook's record. The
documented rules above come from Microsoft's page; the Handbook has confirmed nothing about
them empirically.

The inputs worth probing first, in order of how much they would settle:

1. **Fractional `start_num` and `num_chars`.** Truncation toward zero is the usual assumption
   for this family, but "usual assumption" is not evidence. `MID("abcdef", 2.9, 2.9)` decides
   it in one cell.
2. **`num_chars` beyond the far end combined with `start_num` exactly at `LEN(text)+1`.** The
   documented empty-string rule and the documented clamp meet here, and which one applies
   first is not stated.
3. **Very large `num_chars`** — up to and past the 32,767 code-unit text cap — to see whether
   the cap interacts with the clamp at all.
4. **Astral characters straddling the window edge**, to confirm the code-unit count and to see
   what a split surrogate pair publishes.
5. **Array arguments in each of the three positions**, given the open `BUG-FUNC-007` stream on
   array positions and count arguments in this family.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| code unit | One UTF-16 unit; the unit worksheet text and `LEN` count in |
| window | The half-open run of positions `MID` extracts |
| clamped end | The window's end is truncated to the end of `text` rather than erroring |

## Sources

- Microsoft, "MID, MIDB functions" —
  <https://support.microsoft.com/en-us/office/mid-function-d5f9e25c-d7d6-472e-b568-4ecb12433028>
  (the documented signature and the four boundary rules quoted above).
- Handbook, [The value universe](../model/01-value-universe.md) — text as UTF-16 code units,
  the 32,767-unit cap, and the surrogate-splitting note.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — to-number coercion of
  the numeric argument slots and error propagation.
- Handbook, [The call pipeline](../model/03-call-pipeline.md) — argument preparation and
  result publication.
- Handbook projection `data/presence/FUNC.MID.json` — implementing module and the referenced
  `BUG-FUNC-007` defect stream.
