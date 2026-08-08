---
schema: efh.function-page/v1
function_id: FUNC.REGEXTEST
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — REGEXTEST function"
    locator: "https://support.microsoft.com/en-us/office/regextest-function-7d38200b-5e5c-4196-b4e6-9bff73afbd31"
    role: "documented signature, argument meanings, and the documented PCRE2 regex flavour"
  - work: "OxFunc — FUNCTION_SLICE_REGEX_TRIAD_CONTRACT_PRELIM.md"
    locator: "docs/function-lane/FUNCTION_SLICE_REGEX_TRIAD_CONTRACT_PRELIM.md"
    role: "upstream admitted local-slice boundary and its explicit non-claim over full Excel regex"
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
role_in_family: "The predicate member of the regex triad: matches without extracting."
---

## What it computes

`REGEXTEST` answers one question: does any part of the text match the pattern?

Formally, it returns `TRUE` if there exists any substring of `text` — including the empty substring,
where the pattern permits it — that the regular expression `pattern` matches, and `FALSE` otherwise.
It is a *search*, not a *full match*: the pattern does not have to describe the entire string. To
demand a full match you anchor the pattern yourself with `^` and `$`, or with `\A` and `\z`.

The regex dialect is documented. Microsoft's page states that this function, together with
`REGEXEXTRACT` and `REGEXREPLACE`, uses the PCRE2 flavour of regular expressions. That single
sentence is the most valuable thing on the documentation page, because it converts a large open
question ("which regex is this?") into a citation. It does not, however, mean everything PCRE2 can
express is reachable from a worksheet cell — see below.

`REGEXTEST` returns a `Logical`, which makes it the natural condition for `IF`, `FILTER` and
conditional formatting, and the natural building block for validation columns. Because it returns
`TRUE`/`FALSE` rather than a value or an error, it composes better than the older
`ISNUMBER(SEARCH(...))` idiom it replaces.

## Arguments

`REGEXTEST(text, pattern, [case_sensitivity])`

| Argument | Required | Meaning (as documented by Microsoft) |
|---|---|---|
| `text` | yes | The text, or reference to a cell containing text, to match against. |
| `pattern` | yes | The regular expression describing the pattern to match. |
| `case_sensitivity` | no | `0` (default) case-sensitive; `1` case-insensitive. |

The argument that surprises people is the third, and in two ways. First its **polarity**: `0` means
case-*sensitive*. The name reads like a switch that turns sensitivity on, and it is the opposite —
`1` turns matching case-*insensitive*. Second, it duplicates something the pattern can already say:
PCRE2 supports the inline `(?i)` flag, so there are two mechanisms for the same effect and nothing
consulted here states how they interact when they disagree.

Note also what is *not* an argument: there is no way to ask for the match position or the matched
text. That is `REGEXEXTRACT`'s job.

## Result and edge cases

The return kind is `Logical`.

- **Non-text arguments coerce to text.** `REGEXTEST(0, 0)` asks whether the string `"0"` matches
  the pattern `"0"`, which is a coercion consequence described in
  [coercion and lifting](../model/02-coercion-and-lifting.md), not a regex behaviour. The generated
  battery on this page shows several rows of exactly this shape and they should be read that way.
- **A pattern that can match the empty string makes the answer trivially `TRUE`** for any text,
  which is a common way to write a predicate that silently always passes. `REGEXTEST(x, "a*")` is
  `TRUE` for every `x`.
- **The empty text argument** is a real edge and is not obviously `FALSE`: an empty-matching pattern
  should still succeed against it. This Handbook has not established Excel's behaviour there.
- **Errors propagate** ahead of any matching.
- **An empty referenced cell** delivers `Empty`, distinct from an omitted argument; see
  [the value universe](../model/01-value-universe.md).
- **Array arguments.** Whether `REGEXTEST` lifts over an array-valued `text` — which would be the
  natural way to validate a column — is not asserted here. The general lifting rules are in
  [coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

Microsoft's page publishes no explicit error table for this function (confirmed on the retrieved
page). What can be said:

| Error | Basis |
|---|---|
| any error value arriving in an argument | propagates, per the universal rule in the coercion chapter |
| `#VALUE!` for a pattern the engine will not compile | expected — every regex surface must reject malformed patterns somehow — but the exact rejected set is **not** established, and this is the single largest open question on this page |

The reason that second row is left deliberately vague is worth stating plainly rather than hiding.
Upstream OxFunc carries a closed defect record, `BUG-FUNC-041`, whose subject is exactly this
boundary: its local regex engine had been mapping **unrecognised escape sequences to the literal
letter**, so a pattern like `\n` silently matched the letter `n` instead of a newline or an error.
That is the worst possible failure mode for a predicate — a wrong `TRUE`/`FALSE` with no error to
warn you. The record's repair replaced silent fallthrough with rejection, and a subsequent
live-Excel comparison recorded that the first repair had then swung too far: it rejected a range of
escapes that Excel admitted, in one direction only (OxFunc refusing what Excel accepted). The
escape sets that record ended up admitting and rejecting are reproduced below as an upstream
observation.

## Relationships

- [`REGEXEXTRACT`](FUNC.REGEXEXTRACT.md) — same engine, returns the matched text instead of a
  verdict. `REGEXTEST(t,p)` is close to `NOT(ISNA(REGEXEXTRACT(t,p)))`, though the two differ in
  how they report a pattern error.
- [`REGEXREPLACE`](FUNC.REGEXREPLACE.md) — same engine, rewrites the matches.
- `SEARCH` and `FIND` — the predecessors. `ISNUMBER(SEARCH(needle, hay))` is the classic containment
  test; `SEARCH` supports only `*`, `?` and the `~` escape, and is case-insensitive while `FIND` is
  case-sensitive. `REGEXTEST` does not supersede either; both remain current, and for a plain
  substring test `SEARCH` is clearer and faster.
- `COUNTIF` with wildcards, and `IFS`/`SWITCH` chains, are the other idioms `REGEXTEST` displaces
  in validation work.
- `EXACT` — for whole-string equality, which does not need a regex.
- `ISNUMBER(SEARCH(...))` returns `TRUE`/`FALSE` too, but silently treats a missing needle as
  `FALSE`; `REGEXTEST` distinguishes "no match" from "bad pattern" if the engine reports the latter.

## Notes for implementers

- **Do not implement a regex subset with a silent fallthrough.** This is the concrete lesson of
  `BUG-FUNC-041` and it generalises: if your parser meets a construct it does not understand,
  *fail*, never guess. A subset engine that errors on what it cannot do is honest and usable; one
  that quietly reinterprets is worse than useless, because its wrong answers look like right ones.
- **The escape set is a specification, not an implementation detail.** The upstream record's
  live-Excel comparison lists, as admitted on the build it tested: the shorthand classes
  `\d \D \w \W \s \S`; the zero-width assertions `\A \Z \z \b \B`; the character escapes
  `\n \t \r \f \v \e \h`; and the escaped literal metacharacters
  `\. \* \+ \? \[ \] \( \) \| \^ \$ \/ \\`. It lists unknown letter escapes such as
  `\q \k \m \g \p \c \x \y \j \o` as rejected. Treat that as an upstream observation under one
  Excel build, not as a Handbook claim, and re-probe before relying on it — `\p` in particular is a
  Unicode-property escape in full PCRE2, so its appearance in the rejected list is a signal about
  the build tested rather than about PCRE2.
- **Zero-width assertions must be assertion atoms, not literals.** Modelling `\b` as a character is
  how the original defect arose.
- **Anchor semantics need a line-mode decision.** Whether `^` and `$` match at embedded newlines
  depends on multiline mode; PCRE2 has both, and nothing consulted here says which Excel uses.
- **Guard against catastrophic backtracking.** A cell formula that never returns is a worse user
  experience than a `#VALUE!`. PCRE2 has match limits; use them, and decide what error a limit hit
  produces.
- **Case-insensitivity is a folding choice**, and the upstream contract records only ASCII folding
  in its local slice. Full Unicode folding will match strings Excel may not.

## What has not been checked

There is no Handbook vector suite for `REGEXTEST`, and no Handbook evidence record comparing any
implementation against Excel. Nothing on this page is a measurement.

Two boundaries must be kept apart, and conflating them is the easiest mistake to make here:

- **What Excel's PCRE2 surface supports.** Documented as PCRE2; largely unprobed by this Handbook.
- **What OxFunc's local engine supports.** A deliberately bounded subset. Its own slice contract
  places grouping, alternation, lookarounds, backreferences and capture-group extraction *outside*
  the admitted slice, and rejects unescaped `(` `)` `|` `^` `$` `{` `}` as slice boundaries. Those
  rejections are properties of the reference implementation, **not** statements about Excel. Any
  chip or battery row on this page that reflects the local engine should be read in that light.

The probes that would settle the most:

1. **The escape battery, re-run.** A one-cell-per-escape sweep of `REGEXTEST("a1.B z", "\X")` for
   every letter and punctuation escape, on a current Excel build. This is the probe the upstream
   record ran, it is cheap, and it converts the largest open area into a table.
2. **Grouping, alternation and lookaround.** `REGEXTEST("ab", "(a)(b)")`, `REGEXTEST("ab","a|c")`,
   `REGEXTEST("ab","a(?=b)")` — three cells that establish whether the full PCRE2 surface is
   actually reachable, which nothing here has confirmed.
3. **Inline flags.** `REGEXTEST("A","(?i)a")` and the interaction with `case_sensitivity = 0`.
4. **Anchors and newlines.** `REGEXTEST("a" & CHAR(10) & "b", "^b")` distinguishes multiline mode.
5. **Empty text and empty pattern.** `REGEXTEST("", "")`, `REGEXTEST("", "a*")`, `REGEXTEST("a","")`.
6. **Unicode.** `REGEXTEST("é","\w")` and `REGEXTEST("😀",".")` — whether the engine is
   Unicode-aware, whether `.` matches a whole astral character or half a surrogate pair, and
   whether `\w` includes accented letters.
7. **Malformed patterns.** `REGEXTEST("a","[")` and `REGEXTEST("a","a{2,1}")` — which error, and
   whether it is `#VALUE!`.
8. **Pathological patterns**, to find the match limit and the error it produces.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| PCRE2 | The regular-expression flavour Microsoft's page names for this function family |
| search versus full match | This function succeeds on any substring match; anchoring is the caller's job |
| escape admission | Which `\X` sequences the engine accepts, rejects, or silently reinterprets |
| silent fallthrough | Treating an unrecognised escape as a literal; the defect `BUG-FUNC-041` records |
| local slice | The bounded subset OxFunc's own engine implements; not a statement about Excel |

## Sources

- Microsoft Support, REGEXTEST function —
  <https://support.microsoft.com/en-us/office/regextest-function-7d38200b-5e5c-4196-b4e6-9bff73afbd31>
  (retrieved for this page; source of the signature, the argument meanings, the `case_sensitivity`
  polarity, and the statement that this family uses the PCRE2 flavour. The page publishes no error
  table.)
- OxFunc `docs/function-lane/FUNCTION_SLICE_REGEX_TRIAD_CONTRACT_PRELIM.md` — the admitted local
  slice, its explicit out-of-slice list, and its statement that it does not claim the full Excel
  regex surface. Provisional by its own statement.
- OxFunc `docs/bugs/streams/BUG-FUNC-041_regex_silent_escape_fallthrough_and_dead_translate_phrasebook.md`
  — the silent-escape-fallthrough defect, its repair, and the live-Excel escape-admission
  observations reproduced above. Scoped to Excel 16.0 build 20026, Windows 64-bit, as recorded
  there.
- Handbook `content/model/01-value-universe.md`, `02-coercion-and-lifting.md`.
