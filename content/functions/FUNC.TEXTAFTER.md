---
schema: efh.function-page/v1
function_id: FUNC.TEXTAFTER
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — TEXTAFTER function"
    locator: "https://support.microsoft.com/en-us/office/textafter-function-c8db2546-5b51-416a-9690-c7e6722e90b4"
    role: "documented signature, argument meanings and documented error conditions"
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
role_in_family: "The forward half of the delimiter-slice pair: returns the tail of the text after a
  selected delimiter occurrence."
---

## What it computes

`TEXTAFTER` slices a string at one chosen occurrence of a delimiter and returns everything to the
right of it.

State it as an indexing problem, because that is what it is. Let `t` be the text and `d` the
delimiter. Under the chosen match mode, enumerate the occurrences of `d` inside `t` in
left-to-right order as index pairs `(start_1, end_1), (start_2, end_2), …, (start_k, end_k)`, where
`end_i = start_i + len(d)`. The `instance_num` argument selects one of them: a positive `n` selects
the `n`-th pair counted from the left, a negative `n` selects the `|n|`-th counted from the right.
The result is the substring of `t` from the selected `end_i` to the end of `t`.

Three consequences follow directly and are worth stating rather than discovering:

1. **The delimiter itself is not in the result.** The cut is at `end_i`, not `start_i`. That is the
   entire difference between this function and [`TEXTBEFORE`](FUNC.TEXTBEFORE.md), which cuts at
   `start_i` and keeps the left side.
2. **A delimiter at the very end of `t` yields the empty string**, not a missing value. `end_k`
   equals `len(t)` and the substring is empty. Empty text is a perfectly good result here.
3. **Selection is by occurrence, not by position.** There is no "find the delimiter nearest
   character 12" mode. If you want positional slicing, you want `MID`/`LEFT`/`RIGHT`, not this
   family.

Two arguments perturb the occurrence list rather than the slicing rule. `match_mode = 1` makes the
occurrence search case-insensitive, which can add occurrences that a case-sensitive scan would not
find. `match_end = 1` treats the end of the text as an additional delimiter occurrence; OxFunc's
upstream slice contract records the narrower reading that the synthetic terminal delimiter is added
when the delimiter is *otherwise absent*. The Handbook does not treat that as settled — see
"What has not been checked".

The empty delimiter is a genuine special case with a directional answer. OxFunc's contract records
Excel's polarity as `TEXTAFTER(t, "", n>0) = t` and `TEXTAFTER(t, "", n<0) = ""`. That is exactly
what the indexing model above predicts if the empty delimiter is taken to have one zero-width
occurrence, located at position `0` when counting forward and at position `len(t)` when counting
backward — the whole string lies after the first, and nothing lies after the last.

## Arguments

`TEXTAFTER(text, delimiter, [instance_num], [match_mode], [match_end], [if_not_found])`

| Argument | Required | Meaning (as documented by Microsoft) |
|---|---|---|
| `text` | yes | The string being searched. Wildcards are not accepted; the delimiter is matched literally. |
| `delimiter` | yes | The string that marks the cut point. May be more than one character. |
| `instance_num` | no | Which occurrence to cut at. Defaults to `1`. Negative counts from the end. `0` is documented as an error. |
| `match_mode` | no | `0` (default) case-sensitive, `1` case-insensitive. |
| `match_end` | no | `0` (default) plain matching; `1` treats the end of the text as a delimiter. |
| `if_not_found` | no | The value returned instead of `#N/A` when the requested occurrence does not exist. |

The argument position readers misjudge most often is the third. `instance_num` is an *occurrence
index*, and it is signed; it is not a character position and not a count of characters to skip.
`TEXTAFTER("a-b-c", "-", 2)` is `"c"`, and `TEXTAFTER("a-b-c", "-", -1)` is also `"c"` — the same
answer arrived at from opposite ends, which is why writing `-1` is the robust way to say "after the
last one".

The sixth argument is the one people forget exists. `if_not_found` removes the need to wrap the
call in `IFERROR`, and unlike `IFERROR` it only catches the no-match case: an argument-domain error
still surfaces.

## Result and edge cases

The return kind is `Text` — a scalar string, including possibly the empty string. This function
is not itself array-producing; it is a scalar kernel that the engine lifts. The lifting rules,
including how a per-element failure stays element-local rather than collapsing the whole result,
are in [coercion and lifting](../model/02-coercion-and-lifting.md); the spill mechanics are the
host-side adaptation step described in [the call pipeline](../model/03-call-pipeline.md).

What is specific to this function is *which* positions lift. An upstream OxFunc defect record
(`BUG-FUNC-008`) reports a live-Excel replay in which `TEXTAFTER` spilled over an array-valued
`text` argument and over an array-valued `instance_num` argument, and records that widening the
lift to an array-valued *delimiter* is not admitted by that evidence. So the shape of
`TEXTAFTER("a-b-c", "-", SEQUENCE(3))` is a column, while an array delimiter is a separate and
unsettled question. The Handbook repeats that as an upstream observation under one Excel build,
not as its own claim.

Other boundaries worth naming explicitly:

- **Numbers and logicals as `text` or `delimiter`.** They convert to text by the ordinary rules in
  [coercion and lifting](../model/02-coercion-and-lifting.md); `TRUE` becomes the string `TRUE`.
  Nothing here is special-cased.
- **An empty referenced cell** delivers `Empty`, which is distinct from an omitted argument. See
  [the value universe](../model/01-value-universe.md) for why the two are kept apart.
- **Error inputs propagate.** Coercion never discards a worksheet error.
- **Empty result versus no result.** These are different outcomes and should not be conflated by a
  caller: an empty string means the cut succeeded and nothing followed it; `#N/A` (or
  `if_not_found`) means no such occurrence existed.

## Errors

Microsoft's page documents the following (see Sources):

| Error | Documented condition |
|---|---|
| `#N/A` | The delimiter is not found, or `instance_num` exceeds the number of occurrences. |
| `#VALUE!` | `instance_num` is `0`, or its magnitude exceeds the length of the text. |

`#N/A` is suppressible by supplying `if_not_found`; `#VALUE!` is not. Errors arriving in any
argument propagate ahead of all of this.

## Relationships

- [`TEXTBEFORE`](FUNC.TEXTBEFORE.md) — the mirror function, same arguments, opposite side of the
  cut. The two share one implementation module upstream, which is why their behavioural questions
  are usually settled together.
- [`TEXTSPLIT`](FUNC.TEXTSPLIT.md) — the same delimiter idea taken to its conclusion: instead of
  selecting one occurrence, split at all of them and return the pieces as an array.
- `MID`, `LEFT`, `RIGHT` — positional slicing. Reach for these when you know the character index;
  reach for `TEXTAFTER` when you know the marker.
- `FIND` / `SEARCH` combined with `MID` is the pre-2022 idiom this function replaces. `FIND` is
  case-sensitive and `SEARCH` is not, which is the same distinction `match_mode` now expresses in
  one argument.
- Nothing is superseded by `TEXTAFTER`; the older idiom remains fully supported.

## Notes for implementers

- **Occurrence enumeration for multi-character delimiters needs an overlap policy.** Scanning
  `"aaa"` for `"aa"` yields two occurrences if you advance by one character and one occurrence if
  you advance past the match. The documentation does not state which Excel does. Pick one, write
  it down, and treat it as a probe target rather than an assumption.
- **Negative `instance_num` should be resolved against the same occurrence list**, not by
  re-scanning right-to-left. A right-to-left scan can disagree with a left-to-right scan for
  overlapping delimiters, and the difference is exactly the overlap policy above.
- **Case-insensitive matching is not one rule.** The upstream slice contract records only ASCII
  case folding as observed; Unicode case folding, and locale-sensitive folding in particular, are
  outside what that evidence covers. An implementation that folds with a full Unicode table may
  find more occurrences than Excel does.
- **Indices are UTF-16 code units** in the model this Handbook uses. Delimiters and text
  containing astral characters make the distinction between code units and Unicode scalars
  observable; see [the value universe](../model/01-value-universe.md).
- **Lift only the positions you have evidence for.** Lifting the delimiter position because it
  seems symmetric is precisely the kind of invented behaviour this Handbook is trying not to ship.

## What has not been checked

There is no Handbook vector suite for `TEXTAFTER`, and no Handbook evidence record comparing any
implementation against Excel for this function. Nothing on this page is a measurement.

The specific questions that are open, and the probes that would close them:

1. **Overlapping multi-character delimiters.** `TEXTAFTER("aaaa", "aa", 2)` distinguishes the two
   overlap policies in one cell. Run it with `instance_num` from `1` to `4` and with the negative
   mirror.
2. **`match_end` semantics.** OxFunc's contract reads the synthetic terminal delimiter as applying
   only when the delimiter is otherwise absent. `TEXTAFTER("a-b", "-", 2, 0, 1)` and
   `TEXTAFTER("a-b", "x", 1, 0, 1)` separate that reading from the broader one where the end of
   text is always an extra occurrence.
3. **Whether `match_end` also affects the start of the text**, which matters only for negative
   `instance_num`. Probe `TEXTAFTER("a-b", "-", -2, 0, 1)`.
4. **Case-insensitive folding beyond ASCII.** `TEXTAFTER("straße", "SS", 1, 1)` and a Turkish
   dotted/dotless `i` pair are the standard separators between ASCII folding, simple Unicode
   folding, and full folding.
5. **Array-valued `delimiter`.** The upstream record explicitly declines to admit it. A single
   probe of `TEXTAFTER("a-b_c", {"-","_"})` settles whether Excel lifts that position.
6. **The `#VALUE!` boundary for large `instance_num`.** Microsoft documents the error when the
   magnitude exceeds the text length, which is a surprising rule — it means the error depends on
   the text, not on the occurrence count. `TEXTAFTER("ab", "x", 3)` versus `TEXTAFTER("abcd", "x",
   3)` tests whether the documented rule is the observed one.

Until those are recorded, treat every statement above that is not attributed to Microsoft's page
or to a named upstream record as a model, not a finding.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| occurrence list | The left-to-right enumeration of delimiter match positions that `instance_num` indexes into |
| polarity | The direction-dependent answer for a zero-width (empty) delimiter |
| lift position | An argument position over which the engine broadcasts this scalar kernel |
| `if_not_found` | The sixth argument; replaces the no-match `#N/A` only, not other errors |
| upstream observation | A behaviour recorded by OxFunc under a named Excel build; not a Handbook claim |

## Sources

- Microsoft Support, TEXTAFTER function —
  <https://support.microsoft.com/en-us/office/textafter-function-c8db2546-5b51-416a-9690-c7e6722e90b4>
  (retrieved for this page; source of the signature, argument meanings, and the documented error
  table).
- OxFunc `docs/function-lane/FUNCTION_SLICE_TEXT_DELIM_FAMILY_CONTRACT_PRELIM.md` — the admitted
  current-baseline slice, empty-delimiter polarity, and the `match_end` reading. Provisional by its
  own statement.
- OxFunc `docs/bugs/streams/BUG-FUNC-008_text_scalar_and_delimiter_array_support_gap.md` — the
  live-Excel replay of which argument positions spill, and the explicit non-admission of an
  array-valued delimiter.
- Handbook `content/model/01-value-universe.md`, `02-coercion-and-lifting.md`,
  `03-call-pipeline.md` — the shared value, coercion, lifting, and publication model this page
  refers to rather than restating.
