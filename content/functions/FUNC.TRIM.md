---
schema: efh.function-page/v1
function_id: FUNC.TRIM
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — TRIM function"
    locator: "https://support.microsoft.com/en-us/office/trim-function-410388fa-c5df-49c6-b16c-9e5630b479f9"
    role: "documented description and scope of the space characters removed"
  - work: "OxFunc — FUNCTION_SLICE_TEXT_CORE_AND_COMPATIBILITY_FAMILY_CONTRACT_PRELIM.md"
    locator: "docs/function-lane/FUNCTION_SLICE_TEXT_CORE_AND_COMPATIBILITY_FAMILY_CONTRACT_PRELIM.md"
    role: "upstream admitted-slice rule on ASCII spaces versus non-breaking spaces, and array lift"
  - work: "OxFunc — BUG-FUNC-008 text scalar and delimiter array-support gap"
    locator: "docs/bugs/streams/BUG-FUNC-008_text_scalar_and_delimiter_array_support_gap.md"
    role: "upstream live-Excel observation that TRIM spills over an array-valued argument"
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
family: text_scalar_misc
role_in_family: "The whitespace normaliser of the scalar text group: collapses runs of spaces and
  strips the ends."
---

## What it computes

`TRIM` is not the `trim` you know from other languages. Most languages' `trim` removes leading and
trailing whitespace and leaves the interior alone. Excel's `TRIM` also **collapses interior runs**.

The rule, stated exactly: take the text, delete every leading space, delete every trailing space,
and replace each remaining run of consecutive spaces with a single space. Equivalently — and this
is the formulation to implement — split the text on spaces, discard empty pieces, and rejoin with
one space between pieces.

Two things follow that catch people out:

1. **`TRIM` is not idempotent-by-accident, it is idempotent by construction.** `TRIM(TRIM(x))` is
   `TRIM(x)`, because the output contains no leading, trailing, or doubled spaces by definition.
2. **`TRIM` changes the interior of your data.** If your text is fixed-width or contains
   deliberate alignment padding, `TRIM` destroys it. `SUBSTITUTE(x, " ", "")` removes spaces;
   `TRIM` normalises them; they are different tools and neither is a general "clean up" function.

The word "space" is doing a great deal of work in that rule, and it is the whole difficulty of this
function. Microsoft's page is the authority on which characters count, and it is
linked below; the Handbook could not retrieve it while this entry was written, so nothing from it
is quoted or paraphrased here. What can be stated with a named basis is the implementation-side
reading: OxFunc's upstream text-core contract records that on its admitted slice `TRIM` collapses
ASCII spaces and *preserves* non-breaking spaces. That is the single most
practically important fact on this page, because text pasted from a web page is full of
non-breaking spaces, and `TRIM` will not touch them. `CLEAN` will not help either — it targets
control characters. The usual repair is `TRIM(SUBSTITUTE(x, UNICHAR(160), " "))`.

## Arguments

`TRIM(text)`

| Argument | Required | Meaning |
|---|---|---|
| `text` | yes | The text to normalise. |

There is exactly one argument and no options. There is no way to ask `TRIM` to strip the ends
without collapsing the interior, and no way to widen its notion of whitespace.

Non-text arguments convert to text first, by the ordinary rules in
[coercion and lifting](../model/02-coercion-and-lifting.md): a number arrives as its general-format
rendering, `TRUE` as the string `TRUE`. Since none of those renderings contains a space that a
naïve reading would expect to survive, `TRIM` of a non-text scalar is generally that scalar's text
form unchanged.

## Result and edge cases

The return kind is `Text`. `TRIM` of the empty string is the empty string; `TRIM` of a string of
nothing but spaces is the empty string.

`TRIM` is a scalar kernel, so array arguments are handled by the engine's lifting — the elementwise
rules, including element-local failures, are in
[coercion and lifting](../model/02-coercion-and-lifting.md). Specific to this function: an upstream
OxFunc defect record (`BUG-FUNC-008`) reports a live-Excel replay in which `TRIM` spilled over a
single array-valued argument, which is why `TRIM({"  a  "," b "})` is a two-element array rather
than an error. That is an upstream observation under one Excel build, not a Handbook claim.

Other boundaries:

- **An empty referenced cell** delivers `Empty`, distinct from an omitted argument; see
  [the value universe](../model/01-value-universe.md).
- **Errors propagate.** `TRIM` has no error-masking policy.
- **Astral characters are unaffected**, but note that the model's text is a sequence of UTF-16
  code units, so a `TRIM` implemented over code units and one implemented over Unicode scalars will
  agree here only because no space character is astral. Do not generalise from that to the rest of
  the text family.

## Errors

Microsoft's page documents no error conditions for `TRIM` beyond the ordinary ones (see Sources).
In practice the reachable errors are:

| Error | Condition |
|---|---|
| any error value | Propagated from the argument; coercion does not discard worksheet errors. |
| `#VALUE!` | An argument whose kind has no text conversion in this context. |

The Handbook has not established which argument kinds fall in that second row for `TRIM`
specifically.

## Relationships

- `CLEAN` — the neighbouring "tidy the text" function, and the one `TRIM` is most often confused
  with. `CLEAN` removes non-printable control characters; `TRIM` normalises spaces. Neither does
  the other's job, and `TRIM(CLEAN(x))` is the common pairing.
- `SUBSTITUTE` — removes or replaces a specified character. This is the escape hatch for
  whitespace `TRIM` will not touch, notably `UNICHAR(160)`.
- [`TEXTSPLIT`](FUNC.TEXTSPLIT.md) with `ignore_empty` performs the same "drop empty pieces"
  operation on a general delimiter, which is `TRIM`'s rule in generalised form.
- `TEXTJOIN` with `ignore_empty` set is the rejoining half of the same idea.
- `TRIMRANGE` and the trim-reference operators are *not* related despite the name: they trim empty
  rows and columns from a reference, not spaces from a string.
- `EXACT` and `=` comparisons are where untrimmed data usually announces itself, which is why
  `TRIM` shows up so often in lookup keys.

## Notes for implementers

- **The character set is the specification.** Implement `TRIM` against an explicit list of code
  points, not against a library's `is_whitespace` predicate. A Unicode-aware `is_whitespace`
  matches tab, newline, non-breaking space, en quad, ideographic space and more; the evidence
  recorded upstream says the non-breaking space at least survives Excel's `TRIM`. Using the
  language's default whitespace class here is the single most likely way to diverge.
- **Collapse, don't just strip.** A port that reuses a host-language `trim()` produces wrong
  interior results for every string with a double space.
- **Tabs and newlines are a separate question** from the non-breaking space, and this Handbook
  does not have an answer for them. Do not assume they behave like the space just because the
  language you are porting from groups them together.
- **The 32,767-code-unit text cap** applies to the result as to any string; see
  [the value universe](../model/01-value-universe.md). `TRIM` only ever shortens, so it cannot
  create a cap violation, but it can be the step that makes an over-long input fit.
- **Lift a single array-valued argument** rather than rejecting arrays outright — that is the
  behaviour the upstream record pins.

## What has not been checked

There is no Handbook vector suite for `TRIM`, and no Handbook evidence record comparing any
implementation against Excel. Nothing on this page is a measurement. The one behavioural fact
carried here beyond Microsoft's own description — that non-breaking spaces survive — comes from
OxFunc's provisional contract for its admitted slice, not from a Handbook verification.

The probes that would settle the rest, in the order that would matter most to a user:

1. **The whitespace inventory.** Run `TRIM` over a one-character-per-cell sweep of the plausible
   candidates: `CHAR(9)` (tab), `CHAR(10)` and `CHAR(13)` (line breaks), `UNICHAR(160)`
   (non-breaking space), `UNICHAR(8194)`/`UNICHAR(8195)` (en/em space), `UNICHAR(8239)` (narrow
   no-break space), `UNICHAR(12288)` (ideographic space), each wrapped as `"a" & c & "b"` and as
   `c & "a" & c`. This one sweep converts the vaguest part of this page into a table.
2. **Interior collapse with mixed whitespace.** `TRIM("a" & CHAR(9) & CHAR(9) & "b")` shows whether
   a non-space whitespace run is collapsed, preserved, or treated as a word boundary.
3. **Whether a run of mixed space and non-breaking space collapses around the survivor.**
4. **Behaviour at the text cap**, by trimming a string near 32,767 code units.
5. **Locale sensitivity.** `TRIM` should have none, but nothing has confirmed that; a sweep on a
   non-Latin UI locale would close it cheaply.

Until those are recorded, the whitespace inventory in particular should be read as "documented and
upstream-observed", not as established.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| collapse | Replacing a run of consecutive spaces with exactly one |
| whitespace inventory | The explicit set of code points a `TRIM` implementation treats as a space |
| non-breaking space | `UNICHAR(160)`; recorded upstream as surviving Excel's `TRIM` |
| lift | Elementwise application of this scalar kernel over an array argument |

## Sources

- Microsoft Support, TRIM function —
  <https://support.microsoft.com/en-us/office/trim-function-410388fa-c5df-49c6-b16c-9e5630b479f9>
  (the authority on the documented description and on which space characters are in scope; the
  page was not retrievable at the time this entry was written, so it is cited by reference only and
  nothing on this page paraphrases it).
- OxFunc `docs/function-lane/FUNCTION_SLICE_TEXT_CORE_AND_COMPATIBILITY_FAMILY_CONTRACT_PRELIM.md`
  — admitted-slice rule that `TRIM` collapses ASCII spaces and preserves non-breaking spaces, and
  that it spills over a single array-valued argument. Provisional by its own statement.
- OxFunc `docs/bugs/streams/BUG-FUNC-008_text_scalar_and_delimiter_array_support_gap.md` — the
  live-Excel replay that pinned the array lift.
- Handbook `content/model/01-value-universe.md` (text as UTF-16 code units, the 32,767 cap),
  `02-coercion-and-lifting.md` (to-text coercion, lifting, error propagation).
