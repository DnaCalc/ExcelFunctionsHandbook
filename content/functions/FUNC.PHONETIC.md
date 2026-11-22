---
schema: efh.function-page/v1
function_id: FUNC.PHONETIC
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
  - Notes
  - Page vocabulary
  - Sources
family: null
role_in_family: null
---

## What it computes

`PHONETIC(reference)` returns the **furigana** stored alongside the text in a cell.

Microsoft's description: "Extracts the phonetic (furigana) characters from a text string."

*The quoted one-line description is Microsoft's text, from its support article <https://support.microsoft.com/en-us/office/phonetic-function-9a329dac-0c0f-42f8-9a55-639086988554>.*

Understanding this function requires one fact about Japanese text in Excel that has no analogue
in the rest of the worksheet surface: **a cell can carry a second string that is not its value.**
When Japanese text is typed through an IME, the IME knows the reading that produced the kanji,
and Excel stores that reading as *furigana* — a phonetic annotation attached to the cell. It is
not part of the cell's text value, it is not visible unless furigana display is switched on, and
no other function can see it.

`PHONETIC` is the only worksheet function that reads it.

This makes `PHONETIC` structurally unlike every other text function. `LEFT`, `UPPER`, `LEN` and
the rest are pure functions of a string: given the same input they compute the same output
anywhere, on any machine, in any locale. `PHONETIC` is not a computation on its argument at all —
it is a **retrieval of stored metadata**, and the metadata was created by the input method at the
time the text was typed. Two cells displaying identical kanji can return different furigana, or
one can return furigana and the other nothing, depending entirely on how each was entered. Text
produced by a formula, imported from a file, or pasted as values generally carries no furigana at
all.

Three further constraints, all documented:

1. **The system region must be set to a Far East language** — Japanese, Chinese or Korean — for
   the function to work.
2. **A multi-cell reference is reduced to its upper-left cell**; the function does not aggregate.
3. **A non-contiguous reference is an error**, not a reduction.

## Arguments

`reference` — required, exactly one. Microsoft's description: "Text string or a reference to a
single cell or a range of cells that contain a furigana text string."

The name `reference` is doing real work here. Furigana is a property of a *cell*, so the useful
form of the call is a reference; a literal text string has no furigana attached to it, because
there was never an IME session that produced it. The documentation admits a text string as an
argument form, but what a text literal could usefully return is not something this Handbook can
state.

The multi-cell form is admitted and reduced: with a range, the answer comes from the upper-left
cell only. This is the same reduction `CELL("contents", …)` performs, and it is worth knowing
because it is silent — a range argument does not error, it just quietly ignores everything but
one cell.

## Result and edge cases

Returns `Text` — the stored furigana, which may be an empty string when the cell has none.

- **No furigana means no result, not an error.** A cell of Latin text, a formula result, an
  imported value: none of these carries the annotation, and the expected answer is empty text.
  The Handbook has not confirmed this against a live build.
- **Formula results carry no furigana.** `PHONETIC` on a cell containing `=A1&B1` reads the
  concatenation's stored annotation, and a concatenation does not produce one. This is the most
  common disappointment with the function: it works on typed data and not on derived data.
- **The annotation can be edited independently of the text.** Excel's furigana editing UI lets a
  user change the reading without changing the kanji, which means `PHONETIC`'s answer is not
  derivable from the cell's value even in principle.
- **Katakana versus hiragana.** Excel has a furigana *type* setting; which script the stored
  annotation uses is a property of how the cell was configured. What `PHONETIC` returns for each
  setting is not established here.
- **Locale dependence is total.** Outside a Far East region setting, the documented requirement is
  not met and the function's behaviour is not something this page can state.

## Errors

- **`#N/A`** — documented, for a reference to non-adjacent cells (a multi-area reference).

The Handbook has read Microsoft's page and does not have a further documented error table to
reproduce. In particular, what happens when the system region requirement is unmet — an error, an
empty string, or a `#NAME?` because the function is not exposed — is **not established here**,
and it is the first thing this page would probe.

## Relationships

- **`ASC` and `DBCS`** are the other limited by locale East-Asian text functions in Excel's surface,
  converting between half-width and full-width forms. Unlike `PHONETIC`, they are pure functions
  of their argument; they are limited by locale but not by stored metadata.
- **`FINDB`, `LEFTB`, `LENB`, `MIDB`, `REPLACEB`, `RIGHTB`, `SEARCHB`** are the byte-variant text
  functions whose behaviour differs under DBCS locales — the other place in Excel where locale
  changes what a text function does. They are byte-counting variants, not metadata readers.
- **No other function reads furigana.** There is no `ISPHONETIC`, no way to test whether a cell
  has an annotation other than calling `PHONETIC` and checking for an empty result, and no way to
  *write* furigana from a formula. The read is one-way and the surface is one function wide.
- **`CELL`** is the nearest structural analogue in this Handbook: another function that returns
  stored workbook state rather than computing on a value.

## Notes for implementers

1. **This is not a text algorithm.** There is no rule that converts kanji to a reading — the
   mapping is ambiguous, which is exactly why the IME stores the reading rather than deriving it.
   Any implementation that tries to *generate* furigana is implementing a different function.
2. **It requires a cell metadata channel.** An engine whose cell model is "a value plus a format"
   cannot hold furigana and therefore cannot implement `PHONETIC` at all. Declining is the correct
   response.
3. **The multi-cell reduction is upper-left, silently.** Not an error, not an aggregate.
4. **Non-contiguous is `#N/A`**, distinct from the reduction case, and the two must be told apart.
5. **Any behavioural claim must carry a locale axis**, in the same way `INFO`'s claims must carry
   platform and build ([claim language and honesty](../model/06-claim-language.md)).

## What has not been checked

This page is the most sparsely evidenced in the assignment, and the reasons are structural rather
than incidental.

- No Handbook vector suite exists for `PHONETIC`; `vectors/` publishes nothing for this function.
- No Excel-comparison evidence record names `PHONETIC` as a subject. **Nobody has checked this
  function's behaviour against Excel inside the Handbook's record.**
- **The reference engine has no entry for `PHONETIC` at all.** It is not registry-backed, it has
  no implementing module, and its catalogue admission is recorded as *deferred*, with the reason
  given as a deferred external/stored-metadata surface. Its battery rows all read
  `cannot-call: not-in-reference-catalog`. So unlike `CELL` and `INFO` — which are implemented but
  unanswerable without a host — `PHONETIC` is not implemented, and the deferral is honest: a
  function that reads a metadata channel the engine does not model cannot be implemented in it.
- Its declared axes are empty and its arity is unrecorded, because there is no implementation to
  read them from. Where other pages can point at declared structure, this one cannot.

What this page holds, therefore, is Microsoft's documentation and an explanation of why the
function is shaped the way it is. Everything below is open.

What a Japanese-locale harness would have to establish, in order:

1. **The behaviour outside a Far East region setting.** Documented as required; not documented as
   to what happens when it is absent. Error, empty string, or unavailable function — three
   plausible answers, and workbooks travel between locales constantly, so this is the question
   with the widest practical reach.
2. **A cell typed through a Japanese IME**, read through `PHONETIC` — the baseline that does not
   exist. Then the same kanji pasted as values into a second cell, which should have no furigana,
   to demonstrate that the annotation does not travel with the text.
3. **The furigana type setting** (hiragana, katakana, half-width katakana) against the returned
   string.
4. **Furigana edited by hand** to differ from the reading the IME produced, confirming that
   `PHONETIC` returns stored state rather than a derivation.
5. **The three reference forms**: single cell, contiguous range (upper-left reduction), and
   non-contiguous range (`#N/A`).
6. **A text literal argument** — `PHONETIC("...")` — which the documentation admits and whose
   result nobody here can predict.
7. **Chinese and Korean region settings**, since the documented requirement names all three but
   furigana is a Japanese concept.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `PHONETIC` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-PHONETIC-001: PHONETIC requires a reference operand at entry and returns #N/A under non-Japanese profiles

*Quirk; measured; recorded 2026-11-22.*

In Excel 16.0.20527 (en-ZA), PHONETIC(reference) accepts only an operand that can yield a
reference at formula entry (=PHONETIC(2), =PHONETIC({1,2}), and =PHONETIC(1+1) are refused with
0x800A03EC, while =PHONETIC(B1) and =PHONETIC(INDEX(B1:B3,0)) enter), and returns #N/A for both
text and blank cells under non-Japanese editing profiles. Matches 3,056/3,056 full-bank rows
under en-ZA.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=PHONETIC(Data!B1)` | as typed | `#N/A` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Measured in Excel 16.0.20527.20088 in prefix_metadata (row 305), 8 entry
probes in excel-entry.tsv, and 3,056/3,056 full-bank PHONETIC rows.

- Open: Phonetic furigana extraction under ja-JP locale remains unmodelled.

Evidence:

- DnaCalc `crates/calc-func/src/catalogue/text_format.rs`, lines 1002-1012 and 1376-1379 at `2e9be2b5b7` — PHONETIC #N/A under non-Japanese profiles and reference-only entry rule
- DnaCalc `crates/calc-func/src/catalogue/entry.rs`, line 385 at `2e9be2b5b7` — Measured reference-place census for PHONETIC
- DnaCalc `crates/calc-func/witness-status.tsv`, PHONETIC at `2e9be2b5b7` — 3,056/3,056 full-bank PHONETIC rows matched
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/prefix_metadata.capture.json`, sheet prefix_metadata row 305 column 1 at `2e9be2b5b7` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| furigana | The phonetic reading stored alongside a cell's text; not part of its value |
| stored metadata | State attached to a cell that no computation on its value can recover |
| limited by locale | Requires a specific system region setting to function |
| deferred | The reference engine intentionally does not implement this surface; the reason is published |

## Sources

- Microsoft, "PHONETIC function" —
  <https://support.microsoft.com/en-us/office/phonetic-function-9a329dac-0c0f-42f8-9a55-639086988554>.
  Read for this page: the description, the syntax, the `reference` argument description, the
  upper-left reduction for a range, the `#N/A` for non-adjacent cells, and the Far East region
  requirement.
- Handbook, [the value universe](../model/01-value-universe.md) — the `Text` kind as UTF-16 code
  units, and the reference shapes this function distinguishes.
- Handbook, [the execution context](../model/04-execution-context.md) — locale as an axis of the
  environment a function runs in.
- Handbook, [claim language and honesty](../model/06-claim-language.md) — why a locale-scoped
  behaviour cannot be published without its axis.
- `data/functions/FUNC.PHONETIC.json` — identity (`xlfPhonetic`, code 360), category (Text
  functions), and the recorded admission label *deferred* with its stated reason, as projected at
  OxFunc `473efa3`.
- `data/presence/FUNC.PHONETIC.json` — the absence record: no implementing module, no Lean
  source, not registry-backed.
