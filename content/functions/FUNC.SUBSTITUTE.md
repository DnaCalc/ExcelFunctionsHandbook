---
schema: efh.function-page/v1
function_id: FUNC.SUBSTITUTE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - SUBSTITUTE versus REPLACE
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
  - Notes
  - Page vocabulary
  - Sources
family: text_search_replace_family
role_in_family: The by-content editor — matches literal text and rewrites every occurrence, or one chosen occurrence.
---

# SUBSTITUTE

## What it computes

`SUBSTITUTE(text, old_text, new_text, [instance_num])` returns `text` with occurrences of
`old_text` rewritten as `new_text`.

Two rules define it, and both are documented:

1. **`old_text` is matched by content, literally.** The match is **case sensitive** and there
   is **no wildcard language** — a `*` in `old_text` is an asterisk.
2. **Without `instance_num`, every occurrence is replaced. With `instance_num`, only that one
   occurrence is.** Microsoft states this directly: if you specify `instance_num`, only that
   instance of `old_text` is replaced; otherwise every occurrence in `text` is changed.

`SUBSTITUTE` never fails to find something. If `old_text` does not occur in `text`, the result
is `text` unchanged — not an error, not an empty string. This is the opposite of `FIND` and
`SEARCH`, which raise `#VALUE!` when the target is absent, and it is why `SUBSTITUTE` is safe
to apply speculatively while a `SEARCH` needs an `IFERROR` guard.

Note the case rule carefully, because it points the wrong way from most readers' intuition:
`SEARCH` — the *finding* function most people reach for — is case-**in**sensitive, while
`SUBSTITUTE` — the *editing* function — is case-**sensitive**. A formula that tests with
`SEARCH` and edits with `SUBSTITUTE` is applying two different matching rules to the same
string, and will happily confirm that a substring is present and then decline to replace it.

## SUBSTITUTE versus REPLACE

The most-confused pair in the text category. Microsoft draws the line on the `SUBSTITUTE` page
itself:

> Use `SUBSTITUTE` when you want to replace specific text in a text string; use `REPLACE` when
> you want to replace any text that occurs in a specific location in a text string.

| | `SUBSTITUTE` | `REPLACE` |
|---|---|---|
| Addresses the target by | **content** — `old_text`, matched literally | **position** — `start_num`, `num_chars` |
| Number of edits | all occurrences, or one chosen by `instance_num` | exactly one window |
| Target absent | returns the input unchanged | not applicable — nothing is searched for |
| Case sensitivity | case sensitive | not applicable |
| Wildcards | none | not applicable |
| Byte sibling | none | `REPLACEB` |

The absence of a `SUBSTITUTEB` is itself informative: by-content editing never needs a byte
coordinate, so the DBCS variant that the positional family requires has no counterpart here.
Under DBCS, `SUBSTITUTE` is generally the safer of the two for exactly that reason.

The one-line test: if your formula computes `start_num` from a `FIND` or `SEARCH` result, you
wanted `SUBSTITUTE`. If your positions come from a fixed-width record layout, you wanted
[REPLACE](FUNC.REPLACE.md).

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `text` | The string to edit. Required. | — |
| `old_text` | The literal substring to find. Required. | — |
| `new_text` | What to put in its place. Required. | — |
| `instance_num` | Which occurrence to replace, counted from the left. Optional. | all occurrences |

`instance_num` is the position most often misread. It is an **occurrence index**, not a
character position and not a count of replacements to make. `instance_num` of 2 replaces the
second occurrence and leaves the first and third alone.

`instance_num` is a numeric slot subject to ordinary to-number coercion; the other three are
text slots. See [Coercion and lifting](../model/02-coercion-and-lifting.md).

The Handbook's projected signature for this entry is a placeholder — the metadata layer carries
the arity (three required, one optional) but not a rendered parameter list — so the argument
names above come from Microsoft's documentation rather than the reference engine's registry.

## Result and edge cases

Returns `Text`.

Documented: the `instance_num` semantics above, and the by-content matching rule. Microsoft's
page does not enumerate boundary conditions, so the following are open rather than settled:

- **Empty `old_text`.** `SUBSTITUTE("abc", "", "X")` has no obviously correct answer — an empty
  match occurs everywhere. Unverified.
- **Empty `new_text`.** Deletion, by the obvious reading, and the idiomatic way to strip a
  substring. Not stated on the page.
- **`instance_num` greater than the number of occurrences**, or zero, or negative. Unchanged
  input and `#VALUE!` are both plausible for the first; the last two are unaddressed.
- **Overlapping occurrences**, as in `SUBSTITUTE("aaa", "aa", "X")`. Left-to-right
  non-overlapping scanning is the usual convention and is not documented here.
- **Whether replacement is re-scanned.** If `new_text` contains `old_text`, a naive
  implementation can loop. Single-pass scanning is the sane behaviour and is not stated.

The implementing module named in the presence projection carries two open upstream
array-support defect streams (`BUG-FUNC-008`, `BUG-FUNC-016`), so array-shaped arguments are
unsettled here too.

## Errors

Microsoft's `SUBSTITUTE` page carries no error table. The reachable errors are the shared ones:
an error value in any argument propagates, non-numeric text in `instance_num` surfaces
`#VALUE!` under the shared coercion rules, and a result exceeding the 32,767 code-unit text cap
is a candidate — the Handbook's value-universe chapter records `#VALUE!` for the over-cap
formula path observed with `REPT`, but whether `SUBSTITUTE` publishes the same way is
unverified.

`SUBSTITUTE` can grow its input without limit — replacing `"a"` with a long string in a long
text — so the cap is genuinely reachable, not theoretical.

## Relationships

- **[REPLACE](FUNC.REPLACE.md)** — the by-position counterpart. See the comparison above.
- **[SEARCH](FUNC.SEARCH.md) and `FIND`** — the locators. Both differ from `SUBSTITUTE` on the
  matching axis: `SEARCH` is case-insensitive with wildcards, `FIND` is case-sensitive and
  literal, `SUBSTITUTE` is case-sensitive and literal. `FIND` is the locator whose matching
  rule actually agrees with `SUBSTITUTE`'s.
- **`REGEXREPLACE`** is the modern general-purpose alternative, with a real pattern language,
  case options and capture groups. `SUBSTITUTE` is not superseded — literal replacement is
  still its own job, and it is the cheaper and clearer tool when that is all you need.
- **`TRIM` and `CLEAN`** are the fixed-purpose cleaners; `SUBSTITUTE(text, CHAR(160), " ")` is
  the standard idiom for the non-breaking spaces that `TRIM` does not touch, which is probably
  this function's single most common real-world use.
- **`LEN(text) - LEN(SUBSTITUTE(text, x, ""))`** is the classic occurrence-counting idiom, and
  it works precisely because `SUBSTITUTE` returns the input unchanged when there is nothing to
  do.

## Notes for implementers

Scanning must be single-pass and left-to-right, with the scan position advancing past
`new_text` rather than re-entering it. Otherwise `SUBSTITUTE("a", "a", "aa")` does not
terminate.

`instance_num` counting has to be done on the *original* occurrences, established by the same
non-overlapping left-to-right scan. Counting occurrences in a partially rewritten string gives
different answers.

The case-sensitivity is a genuine, load-bearing difference from `SEARCH`, and it is the kind
of thing an implementation that shares a matcher between the two functions will silently get
wrong. They share a module in the reference engine; they must not share a comparison.

Matching is over UTF-16 code units. That makes the match byte-alignment-free but also means an
`old_text` that is half a surrogate pair can match, which is a state the value-universe chapter
records as reachable.

## What has not been checked

No Handbook vector suite exists for `SUBSTITUTE`, and no Excel-comparison evidence record
names it. Nobody has checked this function against Excel within the Handbook's record. Only
two things on this page are documented — the by-content-versus-by-location guidance and the
`instance_num` semantics. Everything in "Result and edge cases" is explicitly open.

Inputs worth probing first:

1. **`SUBSTITUTE("abc", "", "X")`** — the empty-match case, which has no convention to fall
   back on and is the cheapest way to learn how the scanner is written.
2. **`SUBSTITUTE("aaa", "aa", "X")`** — overlap handling. `"Xa"` means left-to-right
   non-overlapping; anything else is news.
3. **`SUBSTITUTE("a", "a", "aa")`** — whether the replacement is re-scanned.
4. **`SUBSTITUTE("aaa", "a", "X", 0)`, `(…, -1)` and `(…, 5)`** — the three out-of-range
   `instance_num` cases, none of which the documentation addresses.
5. **Fractional `instance_num`**: `SUBSTITUTE("aaa", "a", "X", 2.9)`.
6. **`SUBSTITUTE("ABC", "b", "X")`** — a one-cell confirmation of the documented case
   sensitivity, and the direct contrast with `SEARCH("b", "ABC")` succeeding.
7. **An over-cap result** built with `REPT`, to see whether `SUBSTITUTE` errors or truncates.
8. **Array arguments in each position**, given the two open defect streams on this module.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `SUBSTITUTE` that an implementer matching Excel, or a reader explaining
an unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's
answer and where it was recorded; "measured" means the statement is read off recorded Excel
answers, "inferred" marks what goes beyond them. Where older text on this page says a behaviour
was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

### FN-LEFT-001: In CompatibilityVersion 1, LEFT/RIGHT preserve astral pairs, MID slices UTF-16 units, and 32,767-unit overflow errors differ by function

*Quirk; measured; recorded 2026-11-22.* Applies to CONCATENATE, LEFT, LEN, MID, REPLACE, REPT, RIGHT, SUBSTITUTE.

In Excel 16.0.20527.20088 (CompatibilityVersion 1), LEFT and RIGHT preserve valid UTF-16
surrogate pairs when counting characters (LEFT(UNICHAR(128512),1) returns '😀', whose LEN is
2), whereas MID slices raw UTF-16 code units and can emit an unpaired surrogate
(MID(UNICHAR(128512),1,1) returns 0xd83d). Exceeding the 32,767 UTF-16 code-unit cell limit
gives #VALUE! in REPT, SUBSTITUTE, and REPLACE, #CALC! in CONCAT, and silent truncation to
32,767 units in CONCATENATE and TEXTJOIN (which can leave an unpaired high surrogate when an
astral character straddles index 32,767).

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=LEFT(UNICHAR(128512),1)` | as typed | `😀` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=LEN(LEFT(UNICHAR(128512),1))` | as typed | `2.0` (`0x4000000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=REPT("a",32768)` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=LEN(SUBSTITUTE(REPT("a",32767),"a","bb"))` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=LEN(REPLACE(REPT("a",32767),1,0,"b"))` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=LEN(CONCATENATE(REPT("a",32767),"b"))` | as typed | `32767.0` (`0x40dfffc000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Measured across all 76 rows of dcfl2_text in Excel 16.0.20527.20088 under
CompatibilityVersion 1.

Evidence:

- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcfl2_text.capture.json`, rows 1 to 76 at `4fb9b728df` — Measured surrogate and 32,767-unit text limits in Excel 16.0.20527.20088
- DnaCalc `crates/calc-func/src/catalogue/text_functions.rs`, count_surrogate_pairs at `4fb9b728df` — CompatibilityVersion 1 vs 2 counting and overflow policies
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcfl2_text.capture.json`, sheet dcfl2_text row 1 column 4 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcfl2_text.capture.json`, sheet dcfl2_text row 17 column 4 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcfl2_text.capture.json`, sheet dcfl2_text row 59 column 4 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcfl2_text.capture.json`, sheet dcfl2_text row 74 column 4 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcfl2_text.capture.json`, sheet dcfl2_text row 76 column 4 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcfl2_text.capture.json`, sheet dcfl2_text row 75 column 4 at `4fb9b728df` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| by-content | Addressing the edit by matching a substring rather than by coordinates |
| occurrence index | What `instance_num` counts: the *n*th match, not the *n*th character |
| non-overlapping scan | Matches are found left to right and consume their input |

## Sources

- Microsoft, "SUBSTITUTE function" —
  <https://support.microsoft.com/en-us/office/substitute-function-6434944e-a904-4336-a9b0-1e58df3bc332>
  (signature, the `instance_num` semantics, and the quoted by-content versus by-location
  guidance).
- Microsoft, "REPLACE, REPLACEB functions" —
  <https://support.microsoft.com/en-us/office/replace-function-8d799074-2425-4a8a-84bc-82472868878a>
  (the other side of the pair).
- Handbook, [The value universe](../model/01-value-universe.md) — the 32,767 code-unit text
  cap and the observed `#VALUE!` on the over-cap formula path.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — argument coercion and
  error propagation.
- Handbook projections `data/functions/FUNC.SUBSTITUTE.json` (placeholder signature) and
  `data/presence/FUNC.SUBSTITUTE.json` (implementing module; `BUG-FUNC-008`, `BUG-FUNC-016`).
