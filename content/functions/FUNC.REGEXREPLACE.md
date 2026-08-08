---
schema: efh.function-page/v1
function_id: FUNC.REGEXREPLACE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — REGEXREPLACE function"
    locator: "https://support.microsoft.com/en-us/office/regexreplace-function-9c030bb2-5e47-4efc-bad5-4582d7100897"
    role: "documented signature, argument meanings and error conditions"
  - work: "Microsoft Support — REGEXTEST function"
    locator: "https://support.microsoft.com/en-us/office/regextest-function-7d38200b-5e5c-4196-b4e6-9bff73afbd31"
    role: "documented statement that this family uses the PCRE2 regex flavour"
  - work: "OxFunc — FUNCTION_SLICE_REGEX_TRIAD_CONTRACT_PRELIM.md"
    locator: "docs/function-lane/FUNCTION_SLICE_REGEX_TRIAD_CONTRACT_PRELIM.md"
    role: "upstream admitted local-slice boundary and the recorded occurrence-argument behaviour"
  - work: "OxFunc — W24_BATCH10_REGEX_EXECUTION_RECORD.md"
    locator: "docs/function-lane/W24_BATCH10_REGEX_EXECUTION_RECORD.md"
    role: "upstream native-Excel replay rows for the regex triad"
  - work: "OxFunc — BUG-FUNC-041 regex silent escape fallthrough"
    locator: "docs/bugs/streams/BUG-FUNC-041_regex_silent_escape_fallthrough_and_dead_translate_phrasebook.md"
    role: "upstream defect record and live-Excel escape-admission observations"
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
family: number_regex_translate_family
role_in_family: "The rewriting member of the regex triad: substitutes matched text."
---

## What it computes

`REGEXREPLACE` finds matches of a regular expression in a string and substitutes replacement text
for them, returning the rewritten string.

The operation is: scan `text` for non-overlapping matches of `pattern` from the left; for the
match or matches selected by the `occurrence` argument, splice `replacement` in place of the
matched substring; leave everything else untouched; return the result. The unmatched parts of the
string pass through verbatim, which is what distinguishes a *replace* from an *extract*.

The dialect is PCRE2, per the family statement on Microsoft's `REGEXTEST` page.

There is one first-order question about this function that the sources consulted here **do not
settle**, and it is the most consequential thing on the page: **when `occurrence` is omitted, does
`REGEXREPLACE` replace every match or only the first?** Both are defensible designs — most
programming-language `replace_all` functions replace everything, most spreadsheet `SUBSTITUTE`-style
functions replace everything unless told otherwise, and a function with an occurrence selector
could reasonably default to the first. Upstream OxFunc's slice contract records the omitted case as
"the first or only match under the admitted current packet", and the native Excel replay rows behind
that packet include `REGEXREPLACE("abc123def", "\d+", "X") -> abcXdef` — a case where the pattern
has exactly one match, so it cannot distinguish the two readings at all. The evidence available
here is genuinely silent, and this page will not guess. It is the first probe listed below.

What the same replay does pin, as an upstream observation, is the selective form:
`REGEXREPLACE("abc123def", "[a-z]+", "X", 2) -> abc123X` — a positive `occurrence` of `2` rewriting
the second match and leaving the first alone.

## Arguments

`REGEXREPLACE(text, pattern, replacement, [occurrence], [case_sensitivity])`

The projection records an arity of three to five. Microsoft's page is the authority on the argument
names and meanings and is linked below; it was not retrievable while this entry was written, so the
fourth and fifth positions are described here from upstream OxFunc's contract, which names them
`occurrence` and `case_sensitivity`.

| Argument | Required | Meaning |
|---|---|---|
| `text` | yes | The text to rewrite. |
| `pattern` | yes | The regular expression identifying what to replace. |
| `replacement` | yes | The text substituted for each selected match. |
| `occurrence` | no | Which match to replace. Upstream records a positive `n` as replacing the `n`-th match; the omitted default is unresolved — see above. |
| `case_sensitivity` | no | `0` case-sensitive, `1` case-insensitive, matching the family's polarity. |

The third position is the one with hidden depth. In full PCRE2 usage, replacement text is itself a
small language: `$1`, `\1` or `${name}` insert capture groups, and `$$` escapes the dollar sign.
Whether Excel's `replacement` supports any of that is **not established here**, and it changes what
the function can do more than any other single fact. Upstream OxFunc's local slice explicitly
admits *literal replacement text only* — but that is a statement about the reference implementation,
not about Excel.

## Result and edge cases

The return kind is `Text`.

- **No match returns the input unchanged.** This is the natural reading and is how every replace
  function in common use behaves, but note that it differs from
  [`REGEXEXTRACT`](FUNC.REGEXEXTRACT.md), which returns `#N/A` on no match. If your formula needs
  to know whether anything was replaced, `REGEXREPLACE` will not tell you — pair it with
  [`REGEXTEST`](FUNC.REGEXTEST.md).
- **Zero-width matches need an advance rule.** A pattern that can match the empty string, replaced
  globally, would otherwise loop or insert the replacement between every character. Which of those
  Excel does is unprobed.
- **The result can grow past the text cap.** Replacing a short pattern with a long replacement in a
  long string is the easiest way in the text family to exceed 32,767 code units. The cap and its
  two enforcement paths are described in [the value universe](../model/01-value-universe.md).
- **Non-text arguments coerce to text** by the ordinary rules in
  [coercion and lifting](../model/02-coercion-and-lifting.md). The generated battery rows that pass
  a number in all three positions are coercion artefacts, not regex behaviour.
- **Errors propagate** ahead of matching.
- **An empty referenced cell** delivers `Empty`, distinct from an omitted argument.
- **Array arguments** are not asserted here.

## Errors

Microsoft's page is the authority on documented error conditions; it was not retrievable while this
entry was written, so this section states bases rather than quoting it.

| Error | Basis |
|---|---|
| any error value arriving in an argument | propagates, per the universal rule in the coercion chapter |
| `#VALUE!` for a pattern the engine will not compile | expected; the exact rejected set is **not** established |
| `#VALUE!` for an `occurrence` outside the admitted domain (`0`, negative, non-integer) | expected; unprobed |
| `#VALUE!` if the result exceeds the text cap on the formula path | expected from the value-universe chapter; unprobed for this function |

## Relationships

- [`REGEXTEST`](FUNC.REGEXTEST.md) — the predicate. Use it to find out whether a replace would do
  anything.
- [`REGEXEXTRACT`](FUNC.REGEXEXTRACT.md) — the extractor. Same engine; returns the match instead of
  rewriting around it.
- `SUBSTITUTE` — the direct predecessor and still the right tool for literal text. It takes an
  `instance_num` in the same spirit as `occurrence`, and upstream records its rules for the current
  baseline: an empty `old_text` leaves the source unchanged, and `instance_num < 1` is `#VALUE!`.
  `SUBSTITUTE` is not superseded and is clearer for any non-pattern replacement.
- `REPLACE` — positional replacement by start and length; a different job entirely despite the
  near-identical name. Readers confuse `REPLACE` and `REGEXREPLACE` constantly; the first takes
  character positions, the second takes a pattern.
- [`TRIM`](FUNC.TRIM.md) — worth mentioning because `REGEXREPLACE(x, "\s+", " ")` is the obvious
  "trim with a wider whitespace set" idiom, and is the standard workaround for `TRIM` not touching
  non-breaking spaces. Whether Excel's `\s` includes the non-breaking space is itself unprobed.
- `TEXTJOIN`, [`TEXTSPLIT`](FUNC.TEXTSPLIT.md) — the delimiter tools, usually a better fit than a
  regex when the structure really is a separator.

## Notes for implementers

- **Decide and document the default-occurrence question.** It is the single most visible behaviour
  of the function and the sources consulted here do not settle it. Do not copy a default from
  another language's `replace`.
- **Decide whether `replacement` is a template or a literal**, and be loud about it. If it is a
  template, `$` becomes a metacharacter and every user string containing a dollar sign is a latent
  bug; if it is literal, capture-group substitution is impossible and users will ask why.
- **Advance past zero-width matches** or the global replace does not terminate.
- **Splice, do not rebuild by repeated search.** Repeatedly searching the partially rewritten string
  can match text that the replacement itself introduced — a classic and silent defect.
- **Never let an unrecognised construct fall through to a literal.** Upstream OxFunc's closed defect
  record `BUG-FUNC-041` documents that exact failure in its own engine, where unrecognised escape
  sequences were mapped to the literal letter so `\n` matched `n`. Wrong output, no error. Reject
  what you cannot compile.
- **The escape set is part of the contract.** The upstream record's live-Excel comparison lists as
  admitted, on the build it tested: `\d \D \w \W \s \S`; `\A \Z \z \b \B`; `\n \t \r \f \v \e \h`;
  and the escaped literals `\. \* \+ \? \[ \] \( \) \| \^ \$ \/ \\`; and lists unknown letter
  escapes such as `\q \k \m \g \p \c \x \y \j \o` as rejected. Upstream observation on one Excel
  build, not a Handbook claim.
- **Check the result against the text cap** and decide which failure you implement.
- **Bound the match effort**, or a pathological pattern hangs a recalculation.

## What has not been checked

There is no Handbook vector suite for `REGEXREPLACE`, and no Handbook evidence record comparing any
implementation against Excel. Nothing on this page is a measurement. Two of this function's most
basic behaviours — the omitted-`occurrence` default, and whether `replacement` supports capture-group
references — are **unresolved here**, which is unusual for a function this widely used and is
stated plainly rather than filled in.

Keep the two boundaries apart, as on the sibling pages. Microsoft documents the flavour as PCRE2;
this Handbook has not probed how much of PCRE2 is reachable. OxFunc's local engine implements a
deliberately bounded subset — its contract places grouping, alternation, lookarounds and
backreferences outside the admitted slice and admits literal replacement text only. Those are
properties of the reference implementation, **not** statements about Excel.

The probes, in priority order:

1. **The default occurrence.** `REGEXREPLACE("a1b2c3", "\d", "X")`. If the answer is `aXbXcX` the
   default is global; if it is `aXb2c3` the default is first-only. One cell resolves the largest
   open question on this page.
2. **Capture-group references.** `REGEXREPLACE("2024-01-02", "(\d+)-(\d+)-(\d+)", "$3/$2/$1")`. If
   the result contains literal dollar signs, the replacement is literal text; if it is a
   rearranged date, it is a template. Follow with `"$$"` and with `"\1"` to pin the syntax.
3. **Negative and zero `occurrence`.** `REGEXREPLACE("a1b2", "\d", "X", -1)` and `…, 0)` — whether
   negative counts from the end as it does in the `TEXTAFTER` family, or errors.
4. **Zero-width matches.** `REGEXREPLACE("abc", "x*", "-")` — how many hyphens, and where.
5. **The escape battery**, shared with the rest of the triad; see the
   [`REGEXTEST`](FUNC.REGEXTEST.md) page.
6. **Whether `\s` includes the non-breaking space**, which decides whether
   `REGEXREPLACE(x, "\s+", " ")` is actually the `TRIM` replacement everyone assumes it is.
7. **The text cap**, by replacing a one-character pattern with a long replacement in a long string.
8. **Array-valued `text`**, in one cell.
9. **Interaction of `case_sensitivity` with an inline `(?i)`** flag in the pattern.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| PCRE2 | The regex flavour Microsoft's page names for this family |
| occurrence | The fourth argument; selects which match or matches are rewritten |
| replacement template | A replacement string in which `$1`-style references insert capture groups |
| literal replacement | A replacement string inserted verbatim, with no metacharacters |
| zero-width match | A match of the empty string; requires an advance rule in global replacement |
| local slice | The bounded subset OxFunc's engine implements; not a statement about Excel |

## Sources

- Microsoft Support, REGEXREPLACE function —
  <https://support.microsoft.com/en-us/office/regexreplace-function-9c030bb2-5e47-4efc-bad5-4582d7100897>
  (the authority for the signature, argument names and documented errors; not retrievable while
  this entry was written, so it is cited by reference and nothing here paraphrases it).
- Microsoft Support, REGEXTEST function —
  <https://support.microsoft.com/en-us/office/regextest-function-7d38200b-5e5c-4196-b4e6-9bff73afbd31>
  (retrieved; carries the family-wide PCRE2 statement).
- OxFunc `docs/function-lane/FUNCTION_SLICE_REGEX_TRIAD_CONTRACT_PRELIM.md` — the admitted local
  slice, literal-replacement-only rule, positive-`occurrence` behaviour, and the out-of-slice list.
  Provisional by its own statement.
- OxFunc `docs/function-lane/W24_BATCH10_REGEX_EXECUTION_RECORD.md` — the native-Excel replay rows
  quoted above, including the selective `occurrence = 2` row and the single-match row that cannot
  resolve the default question.
- OxFunc `docs/bugs/streams/BUG-FUNC-041_regex_silent_escape_fallthrough_and_dead_translate_phrasebook.md`
  — the silent-escape-fallthrough defect and the live-Excel escape-admission observations, scoped
  to Excel 16.0 build 20026, Windows 64-bit, as recorded there.
- Handbook `content/model/01-value-universe.md` (the text cap), `02-coercion-and-lifting.md`.
