---
schema: efh.function-page/v1
function_id: FUNC.TEXTBEFORE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — TEXTBEFORE function"
    locator: "https://support.microsoft.com/en-us/office/textbefore-function-d099c28a-dba8-448e-ac6c-f086d0fa1b29"
    role: "documented signature and argument meanings"
  - work: "Microsoft Support — TEXTAFTER function"
    locator: "https://support.microsoft.com/en-us/office/textafter-function-c8db2546-5b51-416a-9690-c7e6722e90b4"
    role: "documented error table for the shared argument set"
  - work: "OxFunc — FUNCTION_SLICE_TEXT_DELIM_FAMILY_CONTRACT_PRELIM.md"
    locator: "docs/function-lane/FUNCTION_SLICE_TEXT_DELIM_FAMILY_CONTRACT_PRELIM.md"
    role: "upstream admitted-slice contract: empty-delimiter polarity, match_end reading"
  - work: "OxFunc — BUG-FUNC-008 text scalar and delimiter array-support gap"
    locator: "docs/bugs/streams/BUG-FUNC-008_text_scalar_and_delimiter_array_support_gap.md"
    role: "upstream live-Excel observation of which argument positions spill"
episodes: []
body_sections:
  - "What it computes"
  - "Arguments"
  - "Result and edge cases"
  - "Errors"
  - "Relationships"
  - "Notes for implementers"
  - "What has not been checked"
  - "Page vocabulary"
  - "Sources"
family: text_delim_family
role_in_family: "The backward half of the delimiter-slice pair: returns the head of the text before
  a selected delimiter occurrence."
---

## What it computes

`TEXTBEFORE` slices a string at one chosen occurrence of a delimiter and returns everything to the
left of it.

The machinery is identical to [`TEXTAFTER`](FUNC.TEXTAFTER.md) and only the cut point differs. Let
`t` be the text and `d` the delimiter. Under the chosen match mode, enumerate the occurrences of
`d` in `t` left to right as index pairs `(start_1, end_1), …, (start_k, end_k)` with
`end_i = start_i + len(d)`. `instance_num` selects one pair — positive counts from the left,
negative from the right. `TEXTBEFORE` returns the substring of `t` from position `0` up to the
selected `start_i`. `TEXTAFTER` returns the substring from `end_i` to the end.

That single-symbol difference explains the whole family:

1. **The delimiter is in neither result.** `TEXTBEFORE` stops at `start_i`; `TEXTAFTER` resumes at
   `end_i`. Concatenating the two for the same occurrence reconstructs `t` minus one copy of the
   delimiter, not `t`.
2. **A delimiter at position `0` yields the empty string.** `start_1 = 0` and the substring is
   empty. That is a successful cut, not a failure.
3. **Occurrence indexing, not character indexing.** For positional work use `LEFT`/`MID`.

`match_mode = 1` makes the occurrence search case-insensitive, which can enlarge the occurrence
list. `match_end = 1` treats the end of the text as an additional delimiter occurrence; OxFunc's
upstream slice contract records the narrower reading that this synthetic delimiter appears only
when the real delimiter is *otherwise absent*, which for `TEXTBEFORE` is the reading under which
`TEXTBEFORE(t, missing_delimiter, 1, 0, 1)` returns the whole of `t`. The Handbook does not treat
that as settled.

The empty delimiter has a directional answer, and it is the mirror of `TEXTAFTER`'s. OxFunc's
contract records the polarity as `TEXTBEFORE(t, "", n>0) = ""` and `TEXTBEFORE(t, "", n<0) = t`.
The indexing model predicts exactly this if the empty delimiter is taken to have one zero-width
occurrence at position `0` when counting forward and at position `len(t)` when counting backward:
nothing precedes the first, everything precedes the last.

## Arguments

`TEXTBEFORE(text, delimiter, [instance_num], [match_mode], [match_end], [if_not_found])`

Argument meanings below follow Microsoft's documentation for the shared `TEXTAFTER`/`TEXTBEFORE`
argument set, as retrieved from the `TEXTAFTER` page.

| Argument | Required | Meaning |
|---|---|---|
| `text` | yes | The string being searched. Wildcards are not accepted; the delimiter matches literally. |
| `delimiter` | yes | The string that marks the cut point. May be multi-character. |
| `instance_num` | no | Which occurrence to cut at. Defaults to `1`; negative counts from the end; `0` is documented as an error. |
| `match_mode` | no | `0` (default) case-sensitive, `1` case-insensitive. |
| `match_end` | no | `0` (default) plain matching; `1` treats the end of the text as a delimiter. |
| `if_not_found` | no | The value returned instead of `#N/A` when the requested occurrence does not exist. |

The third position is the one most often misread: `instance_num` is a signed occurrence index, not
a character count. `TEXTBEFORE("a-b-c", "-", -1)` is `"a-b"` — everything before the *last*
hyphen — which is the idiom for "strip the final segment" and is not obtainable by any positive
index without first counting the delimiters.

`if_not_found` catches only the no-match case. An argument-domain error still surfaces through it.

## Result and edge cases

The return kind is `Text`, possibly the empty string. The function is a scalar kernel; array
arguments are handled by the engine's lifting, described in
[coercion and lifting](../model/02-coercion-and-lifting.md), and the spill of a lifted result is
the host-side adaptation step in [the call pipeline](../model/03-call-pipeline.md).

Specific to this function: an upstream OxFunc defect record (`BUG-FUNC-008`) reports a live-Excel
replay in which `TEXTBEFORE` spilled over an array-valued `text` argument and over an array-valued
`instance_num` argument, while widening the lift to an array-valued *delimiter* is explicitly not
admitted by that evidence. So `TEXTBEFORE("a-b-c", "-", SEQUENCE(3))` produces a column; an array
delimiter is an open question. That is an upstream observation under one Excel build, not a
Handbook claim.

Further boundaries:

- **Non-text arguments** convert by the ordinary rules; `TRUE` becomes the string `TRUE`, a number
  becomes its general-format rendering.
- **An empty referenced cell** delivers `Empty`, which the model keeps distinct from an omitted
  argument — see [the value universe](../model/01-value-universe.md).
- **Errors propagate** rather than being absorbed by coercion.
- **Empty string and `#N/A` mean different things.** Empty means the cut succeeded with nothing to
  its left; `#N/A` (or `if_not_found`) means the occurrence did not exist. A caller that treats
  them alike will silently mishandle leading delimiters.

## Errors

The error table below is taken from Microsoft's **`TEXTAFTER`** page, which the Handbook retrieved
while writing this entry; the `TEXTBEFORE` page was not retrievable at that time. The two functions
share an argument set, so the same conditions are the expected ones here — but that is an
inference, and the `TEXTBEFORE` page is the authority for `TEXTBEFORE`.

| Error | Condition (documented for `TEXTAFTER`; expected for `TEXTBEFORE`) |
|---|---|
| `#N/A` | The delimiter is not found, or `instance_num` exceeds the number of occurrences. |
| `#VALUE!` | `instance_num` is `0`, or its magnitude exceeds the length of the text. |

`if_not_found` replaces the `#N/A` only. Errors arriving in arguments propagate ahead of both rows.

## Relationships

- [`TEXTAFTER`](FUNC.TEXTAFTER.md) — the mirror function; identical arguments, opposite side of the
  cut, shared implementation module upstream.
- [`TEXTSPLIT`](FUNC.TEXTSPLIT.md) — cuts at every occurrence at once and returns an array.
- `LEFT`, `MID` — positional slicing, for when you know the index rather than the marker.
- `FIND` / `SEARCH` with `LEFT` is the legacy idiom, still fully supported. `LEFT(t, FIND(d,t)-1)`
  is roughly `TEXTBEFORE(t, d)` except that it errors instead of returning `#N/A`, has no negative
  indexing, and needs `SEARCH` swapped in for case-insensitive matching.
- `TEXTBEFORE` supersedes nothing; the older composition remains valid.

## Notes for implementers

- **Fix an overlap policy for multi-character delimiters and record it.** Scanning `"aaa"` for
  `"aa"` gives a different occurrence count depending on whether you advance by one character or
  past the match. Microsoft's page does not state which Excel does.
- **Resolve negative `instance_num` against the same occurrence list** you built left to right. A
  separate right-to-left scan can disagree for overlapping delimiters.
- **Case-insensitive matching is a folding choice.** The upstream contract records ASCII folding
  only as observed. A full Unicode fold may find occurrences Excel does not.
- **String indices are UTF-16 code units** in this Handbook's model; astral characters make the
  choice observable. See [the value universe](../model/01-value-universe.md).
- **Do not lift the delimiter position on symmetry grounds.** The evidence admits `text` and
  `instance_num`; inventing the third is how invented behaviour ships.

## What has not been checked

There is no Handbook vector suite for `TEXTBEFORE`, and no Handbook evidence record comparing any
implementation against Excel for it. Nothing on this page is a measurement.

Open questions and the probes that would settle them:

1. **Overlap policy.** `TEXTBEFORE("aaaa", "aa", 2)` distinguishes the two policies in one cell;
   sweep `instance_num` positive and negative.
2. **`match_end` semantics.** `TEXTBEFORE("a-b", "-", 2, 0, 1)` against `TEXTBEFORE("a-b", "x", 1,
   0, 1)` separates OxFunc's "only when otherwise absent" reading from the broader one.
3. **Whether the *start* of the text also counts as a synthetic delimiter under `match_end`**,
   which is the case that matters for `TEXTBEFORE` with negative `instance_num`. Probe
   `TEXTBEFORE("a-b", "-", -2, 0, 1)`.
4. **Folding beyond ASCII.** `TEXTBEFORE("straße", "SS", 1, 1)` and a Turkish dotted/dotless `i`
   pair separate ASCII, simple, and full case folding.
5. **Array-valued `delimiter`.** `TEXTBEFORE("a-b_c", {"-","_"})` settles in one probe whether the
   second position lifts.
6. **The documented `#VALUE!` rule tied to text length.** `TEXTBEFORE("ab", "x", 3)` against
   `TEXTBEFORE("abcd", "x", 3)` tests whether the error really depends on the length of the text
   rather than on the number of occurrences.

Until those are recorded, read every unattributed statement above as a model rather than a finding.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| occurrence list | The left-to-right enumeration of delimiter match positions that `instance_num` indexes |
| polarity | The direction-dependent answer for a zero-width (empty) delimiter |
| lift position | An argument position over which the engine broadcasts this scalar kernel |
| `if_not_found` | The sixth argument; replaces the no-match `#N/A` only |
| upstream observation | A behaviour recorded by OxFunc under a named Excel build; not a Handbook claim |

## Sources

- Microsoft Support, TEXTBEFORE function —
  <https://support.microsoft.com/en-us/office/textbefore-function-d099c28a-dba8-448e-ac6c-f086d0fa1b29>
  (the authority for this function; not retrievable while this entry was written, so it is cited by
  reference and nothing here paraphrases it).
- Microsoft Support, TEXTAFTER function —
  <https://support.microsoft.com/en-us/office/textafter-function-c8db2546-5b51-416a-9690-c7e6722e90b4>
  (retrieved for this page; carries the shared argument and error tables).
- OxFunc `docs/function-lane/FUNCTION_SLICE_TEXT_DELIM_FAMILY_CONTRACT_PRELIM.md` — admitted
  current-baseline slice, empty-delimiter polarity, `match_end` reading. Provisional by its own
  statement.
- OxFunc `docs/bugs/streams/BUG-FUNC-008_text_scalar_and_delimiter_array_support_gap.md` — the
  live-Excel replay of spilling argument positions and the non-admission of an array delimiter.
- Handbook `content/model/01-value-universe.md`, `02-coercion-and-lifting.md`,
  `03-call-pipeline.md`.
