---
schema: efh.function-page/v1
function_id: FUNC.REGEXEXTRACT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — REGEXEXTRACT function"
    locator: "https://support.microsoft.com/en-us/office/regexextract-function-4b96c140-9205-4b6e-9fbe-6aa9e783ff57"
    role: "documented signature, the three return modes, and the text-valued result note"
  - work: "Microsoft Support — REGEXTEST function"
    locator: "https://support.microsoft.com/en-us/office/regextest-function-7d38200b-5e5c-4196-b4e6-9bff73afbd31"
    role: "documented statement that this family uses the PCRE2 regex flavour"
  - work: "OxFunc — FUNCTION_SLICE_REGEX_TRIAD_CONTRACT_PRELIM.md"
    locator: "docs/function-lane/FUNCTION_SLICE_REGEX_TRIAD_CONTRACT_PRELIM.md"
    role: "upstream admitted local-slice boundary and the recorded #N/A on no match"
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
role_in_family: "The extracting member of the regex triad: returns matched text, optionally as an
  array."
---

## What it computes

`REGEXEXTRACT` searches `text` for the regular expression `pattern` and returns what it matched.

Unlike its siblings it is not one function but three, selected by `return_mode`, and the three
differ in *shape* as well as in content:

| `return_mode` | What is returned | Shape |
|---|---|---|
| `0` (default) | The first match | scalar text |
| `1` | Every match, in order | array |
| `2` | The capturing groups of the first match | array |

That table is the whole function, and each row deserves a sentence.

**Mode 0** performs a leftmost search and returns the matched substring. Not the whole text, not
the position — the matched text itself.

**Mode 1** returns all non-overlapping matches. This is the mode that makes `REGEXEXTRACT` a
dynamic-array function: the result spills, and its length depends on the data, which is exactly the
case that dynamic arrays were introduced for. The scan order and the non-overlap rule are standard
regex semantics (each search resumes after the previous match), though nothing consulted here
states how a zero-width match is advanced past — a real question, since a pattern that can match
empty would otherwise loop.

**Mode 2** returns the capture groups of the first match. This is the mode that turns a regex into
a parser: one pattern with parenthesised groups splits a string into its parts in a single call.
Note what it does *not* say — whether group `0` (the whole match) is included, whether unmatched
optional groups appear as empty strings or `#N/A`, and what the shape is (row or column). None of
those is settled here.

The dialect is PCRE2, documented on the family's `REGEXTEST` page. And one further documented note
matters more than it looks: **`REGEXEXTRACT` always returns text values.** Extracting `"123"` gives
you the string `"123"`, and Microsoft's page explicitly points you at `VALUE` to convert it — which
drags in every locale question on the [`VALUE`](FUNC.VALUE.md) page. For numeric extraction,
`NUMBERVALUE` with explicit separators is the more portable second step.

## Arguments

`REGEXEXTRACT(text, pattern, [return_mode], [case_sensitivity])`

| Argument | Required | Meaning (as documented by Microsoft) |
|---|---|---|
| `text` | yes | The text, or reference to a cell containing text, to extract from. |
| `pattern` | yes | The regular expression describing what to extract. |
| `return_mode` | no | `0` first match (default), `1` all matches as an array, `2` capturing groups of the first match. |
| `case_sensitivity` | no | `0` case-sensitive, `1` case-insensitive. |

Two argument positions cause trouble.

`return_mode` is the position readers get wrong when porting from other spreadsheet dialects, where
the equivalent function's third argument means something else entirely. It is a *shape* selector:
changing it changes whether you get a scalar or an array, which changes whether the formula spills,
which changes whether it fits where you put it.

`case_sensitivity` has the same inverted-looking polarity as elsewhere in the family: `0` is
case-*sensitive*. It also overlaps with PCRE2's inline `(?i)` flag, and nothing consulted here
states which wins if they disagree.

## Result and edge cases

The return kind depends on `return_mode`: `Text` in mode `0`, `Array` of text in modes `1` and `2`.
Array results are published by the host-side adaptation step described in
[the call pipeline](../model/03-call-pipeline.md), which is also where `#SPILL!` comes from.

- **No match.** Upstream OxFunc's slice contract records `#N/A` as the no-match result in mode `0`.
  `#N/A` is the right answer semantically — the value is not available — and it means
  `REGEXEXTRACT` composes with `IFNA` rather than `IFERROR`. What modes `1` and `2` return when
  there is no match is not established here; an empty array is not a legal Excel value, so
  something else must happen.
- **Everything comes back as text.** Documented, and worth repeating because it is the most common
  source of downstream surprise.
- **Non-text arguments coerce to text** by the ordinary rules in
  [coercion and lifting](../model/02-coercion-and-lifting.md); the generated battery rows where a
  number is passed as both `text` and `pattern` are coercion artefacts, not regex behaviour.
- **Errors propagate** ahead of matching.
- **An empty referenced cell** delivers `Empty`, distinct from an omitted argument; see
  [the value universe](../model/01-value-universe.md).
- **Array-valued `text`** — the obvious way to extract from a column — is not asserted here, and it
  interacts badly with modes `1` and `2` in any case: an array of arrays is not a value this model
  admits (see [the value universe](../model/01-value-universe.md)), so lifting cannot be
  unconditional.

## Errors

Microsoft's page publishes no explicit error table for this function (confirmed on the retrieved
page). What can be stated:

| Error | Basis |
|---|---|
| `#N/A` on no match in mode `0` | recorded in upstream OxFunc's slice contract; not a Handbook claim |
| any error value arriving in an argument | propagates, per the universal rule in the coercion chapter |
| `#SPILL!` in modes `1` and `2` | an engine outcome when the array cannot be published; see the call-pipeline chapter |
| `#VALUE!` for a pattern the engine will not compile, or a `return_mode` outside `{0,1,2}` | expected, but the exact boundaries are **not** established |

## Relationships

- [`REGEXTEST`](FUNC.REGEXTEST.md) — the predicate version. Use it when you only need to know
  whether a match exists; it avoids the `#N/A`-handling that `REGEXEXTRACT` forces on you.
- [`REGEXREPLACE`](FUNC.REGEXREPLACE.md) — the rewriting version, same engine.
- [`TEXTAFTER`](FUNC.TEXTAFTER.md), [`TEXTBEFORE`](FUNC.TEXTBEFORE.md),
  [`TEXTSPLIT`](FUNC.TEXTSPLIT.md) — the delimiter-based extraction family. For anything that is
  genuinely delimiter-shaped, these are clearer, cheaper and far easier to audit than a regex.
  Reach for `REGEXEXTRACT` when the structure is a *pattern*, not a *separator*.
- `MID` with `FIND`/`SEARCH` — the legacy extraction idiom. Not superseded; still the right tool
  when the position is known.
- [`VALUE`](FUNC.VALUE.md) and `NUMBERVALUE` — the necessary second step whenever what you extracted
  was a number.
- `FILTER` and `IFNA` — the usual companions for mode-1 results.

## Notes for implementers

- **Mode 2 is where the design decisions live.** Group numbering, group `0`, unmatched optional
  groups, named groups, and result orientation are five separate choices, and this Handbook has
  evidence for none of them. Write down what you chose.
- **Advance past zero-width matches** in mode 1, or the all-matches loop does not terminate. The
  standard rule is to bump the search position by one when a match is empty; whether Excel does
  that, and whether the empty match is included in the output, is unprobed.
- **Never let an unrecognised construct fall through to a literal.** Upstream OxFunc's closed
  defect record `BUG-FUNC-041` documents exactly that failure in its own engine: unrecognised
  escape sequences were mapped to the literal letter, so `\n` matched `n`. The result is a wrong
  answer with no error — the worst outcome available. Reject what you cannot compile.
- **The escape set is part of the contract.** The upstream record's live-Excel comparison lists, as
  admitted on the build it tested: `\d \D \w \W \s \S`; `\A \Z \z \b \B`; `\n \t \r \f \v \e \h`;
  and the escaped literals `\. \* \+ \? \[ \] \( \) \| \^ \$ \/ \\`. It lists unknown letter escapes
  such as `\q \k \m \g \p \c \x \y \j \o` as rejected. That is an upstream observation on one Excel
  build, not a Handbook claim; re-probe before depending on it.
- **Results are text, so do not helpfully convert.** Returning a number because the match looked
  numeric would be a divergence from the documented contract.
- **Decide the no-match shape for modes 1 and 2 explicitly**, since an empty array is not
  representable.
- **Bound the match effort.** A pathological pattern in a recalculating workbook is a hang.

## What has not been checked

There is no Handbook vector suite for `REGEXEXTRACT`, and no Handbook evidence record comparing any
implementation against Excel. Nothing on this page is a measurement.

Keep two boundaries apart. Microsoft documents **PCRE2** as the flavour; this Handbook has not
probed how much of PCRE2 is reachable. Separately, **OxFunc's local engine implements a bounded
subset**: its slice contract places grouping, alternation, lookarounds, backreferences and
capture-group extraction outside the admitted slice, and describes first-match extraction only.
That means the reference implementation cannot presently exercise `return_mode` `1` or `2` at all —
which is a property of the reference engine, **not** a statement about Excel, and it is the reason
modes 1 and 2 are the least-supported part of this page.

The probes that would settle the most, in priority order:

1. **Modes 1 and 2 at all.** `REGEXEXTRACT("a1b2c3", "\d", 1)` and
   `REGEXEXTRACT("2024-01-02", "(\d+)-(\d+)-(\d+)", 2)`. Two cells that establish shape,
   orientation, and whether group `0` is included.
2. **No match in modes 1 and 2.** The same formulas against text with no match.
3. **Unmatched optional groups.** `REGEXEXTRACT("ac", "(a)(b)?(c)", 2)` — empty strings, `#N/A`, or
   omitted entries.
4. **Zero-width matches in mode 1.** `REGEXEXTRACT("abc", "x*", 1)` — how many results, and what
   are they?
5. **The escape battery**, re-run per escape as described on the [`REGEXTEST`](FUNC.REGEXTEST.md)
   page. It is shared across the triad and is the highest-value sweep in this family.
6. **`return_mode` domain.** `3`, `-1`, `1.5`, `TRUE`, `"1"` — where `#VALUE!` starts and whether
   truncation happens first.
7. **Array-valued `text`** in each mode, which is where the shape rules will either compose or
   collapse.
8. **Unicode.** `REGEXEXTRACT("héllo", "\w+")` and an astral character against `.`, to see whether
   matching is scalar-aware or code-unit-aware.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| PCRE2 | The regex flavour Microsoft's page names for this family |
| return mode | The `return_mode` argument; selects first match, all matches, or capture groups |
| capture group | A parenthesised sub-pattern whose match mode `2` returns |
| zero-width match | A match of the empty string; requires an advance rule in all-matches scanning |
| local slice | The bounded subset OxFunc's engine implements; not a statement about Excel |

## Sources

- Microsoft Support, REGEXEXTRACT function —
  <https://support.microsoft.com/en-us/office/regexextract-function-4b96c140-9205-4b6e-9fbe-6aa9e783ff57>
  (retrieved for this page; source of the signature, the three `return_mode` values, and the note
  that results are always text and can be converted with `VALUE`. The page publishes no error
  table.)
- Microsoft Support, REGEXTEST function —
  <https://support.microsoft.com/en-us/office/regextest-function-7d38200b-5e5c-4196-b4e6-9bff73afbd31>
  (retrieved; carries the family-wide statement that the regex flavour is PCRE2).
- OxFunc `docs/function-lane/FUNCTION_SLICE_REGEX_TRIAD_CONTRACT_PRELIM.md` — first-match-only
  extraction, `#N/A` on no match, and the explicit out-of-slice list. Provisional by its own
  statement.
- OxFunc `docs/bugs/streams/BUG-FUNC-041_regex_silent_escape_fallthrough_and_dead_translate_phrasebook.md`
  — the silent-escape-fallthrough defect and the live-Excel escape-admission observations, scoped
  to Excel 16.0 build 20026, Windows 64-bit, as recorded there.
- Handbook `content/model/01-value-universe.md`, `02-coercion-and-lifting.md`,
  `03-call-pipeline.md`.
