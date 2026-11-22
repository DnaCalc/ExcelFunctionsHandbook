---
schema: efh.function-page/v1
function_id: FUNC.UNICODE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — UNICODE function"
    locator: "https://support.microsoft.com/en-us/office/unicode-function-adb74aaa-a2a5-4dde-aff6-966e4e81f16f"
    role: "documented signature, description and error conditions"
  - work: "OxFunc — FUNCTION_SLICE_TEXT_CORE_AND_COMPATIBILITY_FAMILY_CONTRACT_PRELIM.md"
    locator: "docs/function-lane/FUNCTION_SLICE_TEXT_CORE_AND_COMPATIBILITY_FAMILY_CONTRACT_PRELIM.md"
    role: "upstream admitted-slice rule: first Unicode scalar, not first raw code unit"
  - work: "OxFunc — STRING_BEHAVIOR_RESEARCH_NOTES.md"
    locator: "docs/function-lane/STRING_BEHAVIOR_RESEARCH_NOTES.md"
    role: "upstream local-baseline observations of the round trip and of a dangling-surrogate input"
episodes: []
body_sections:
  - "What it computes"
  - "Arguments"
  - "Result and edge cases"
  - "Errors"
  - "Relationships"
  - "Notes for implementers"
  - "What has not been checked"
  - "Notes"
  - "Page vocabulary"
  - "Sources"
family: text_unicode_fn
role_in_family: "The text-to-code-point direction of the Unicode pair."
---

## What it computes

`UNICODE` returns the Unicode code point of the **first character** of a string.

The whole content of the function is in what "first character" means, and this is a place where a
careless implementation and a careful one give different answers for real inputs. Worksheet text
is a sequence of UTF-16 code units (see [the value universe](../model/01-value-universe.md)). If
the first code unit is a high surrogate and the second is a low surrogate, they jointly encode one
character in the supplementary planes. The choices are therefore:

- return the first *code unit* — which for an emoji returns a surrogate value in `0xD800`–`0xDBFF`,
  a number that is not a code point at all; or
- decode the first *Unicode scalar* — which for an emoji returns the real code point, above
  `0xFFFF`.

OxFunc's upstream text-core contract records the second reading for its admitted slice: `UNICODE`
returns the first Unicode scalar, not merely the first raw code unit, and rejects an invalid
leading surrogate structure. An OxFunc research note separately records observing
`UNICODE(UNICHAR(128512))` returning `128512` on its local Excel baseline, and — the sharper
observation — that taking the last code unit of an over-length string that had been truncated
mid-pair yields `#VALUE!` from `UNICODE` while `LEN` still reports one unit. Both are upstream
observations under a named baseline, not Handbook claims, and both point the same way: `UNICODE`
decodes, and refuses to decode nonsense.

So the definition is: decode the first Unicode scalar of the argument text and return its code
point as a number. Everything after the first character is ignored.

## Arguments

`UNICODE(text)`

| Argument | Required | Meaning |
|---|---|---|
| `text` | yes | The text whose first character's code point is returned. |

One argument, and one trap: the argument is *text*, so a number arrives as its general-format
rendering and `UNICODE(0)` asks for the code point of the character `"0"` — that is, `48`, not `0`.
The generated battery on this page shows exactly that shape of result, and it is a coercion
consequence described in [coercion and lifting](../model/02-coercion-and-lifting.md), not a
function-specific rule. Likewise `UNICODE(TRUE)` asks about the string `TRUE` and therefore about
the letter `T`.

Only the first character matters. `UNICODE("Hello")` and `UNICODE("H")` agree.

## Result and edge cases

The return kind is `Number` — an integer-valued double, the code point.

`UNICODE` is a scalar kernel and lifts elementwise over array arguments, per
[coercion and lifting](../model/02-coercion-and-lifting.md); `UNICODE({"A","B"})` is a two-element
numeric array. That makes `UNICODE` the natural companion to `MID` for dumping a string's code
points: `UNICODE(MID(t, SEQUENCE(LEN(t)), 1))`, with the caveat that slicing a supplementary-plane
character with `MID` may hand `UNICODE` half a pair — which is precisely the input the upstream
note observed erroring.

Other edges:

- **Empty text is an error, not zero.** There is no first character to report.
- **An empty referenced cell** delivers `Empty`, distinct from an omitted argument; see
  [the value universe](../model/01-value-universe.md).
- **Errors propagate** from the argument.
- **The result is not bounded by `0xFFFF`.** Any implementation or consumer that assumes a
  16-bit result is wrong for emoji and for most of the CJK extensions.

## Errors

Microsoft's page is the authority on the documented error conditions and is linked below; it was
not retrievable while this entry was written, so this section states expectations to be checked
against it rather than quoting it.

| Error | Status |
|---|---|
| `#VALUE!` for empty text | expected; the function has no first character to report |
| `#VALUE!` for an ill-formed leading surrogate | recorded upstream as the admitted-slice behaviour, and separately observed on an OxFunc local Excel baseline; not a Handbook claim |
| any error value, propagated from the argument | follows from the universal error-propagation rule |

## Relationships

- [`UNICHAR`](FUNC.UNICHAR.md) — the inverse direction and the sibling in the same implementation
  module upstream. `UNICODE(UNICHAR(n)) = n` is the round trip.
- `CODE` — the older counterpart. Upstream's text-core contract records `CODE` as reading only the
  first UTF-16 code unit and rejecting empty text; `UNICODE` decodes a full scalar. Over the ASCII
  range the two agree, which is why the difference goes unnoticed until it matters. `CODE` is not
  superseded; it remains available and is the right function when you genuinely want the legacy
  character-set number.
- `CHAR` — `CODE`'s inverse, and the narrower counterpart to `UNICHAR`.
- `LEN` and `LENB` — the other place where the scalar-versus-code-unit question is observable.
  Upstream records these two as having diverged for supplementary-plane characters under a current
  Excel compatibility version, with `LENB` retaining code-unit counting. Anyone reasoning about
  `UNICODE` should read that fork note before assuming a consistent counting model across the
  text family.
- Confused with `VALUE`, which converts a numeral string to the number it denotes — a completely
  different operation. `UNICODE("5")` is `53`; `VALUE("5")` is `5`.

## Notes for implementers

- **Decode, do not index.** Reading `s[0]` in a UTF-16 buffer gives a code unit. Decode the first
  scalar, and handle the unpaired-surrogate case deliberately rather than by whatever your string
  type does when it panics.
- **Do not route through UTF-8.** A conversion step will reject or replace exactly the inputs whose
  behaviour is in question.
- **The result is a `Number`, so it is a double.** Every code point is exactly representable, but
  do not let a 16-bit integer type into the signature.
- **Empty text must be checked before decoding**, otherwise the failure surfaces as a panic or an
  out-of-range read rather than as `#VALUE!`.
- **Lift over arrays**, which is what makes the code-point-dump idiom work.

## What has not been checked

There is no Handbook vector suite for `UNICODE`, and no Handbook evidence record comparing any
implementation against Excel. Nothing on this page is a measurement. The scalar-decoding reading is
carried here on the strength of two upstream OxFunc records, each scoped to a named local baseline.

The probes that would settle the rest:

1. **The lone-surrogate input.** Build one deliberately — a truncated pair, or `UNICHAR` of a
   surrogate value if that turns out to be admitted — and check whether `UNICODE` errors, returns
   the surrogate value, or returns a replacement. Pair each result with `LEN` of the same string.
2. **The supplementary-plane round trip at scale.** `UNICODE(UNICHAR(n))` swept across the planes,
   which would seed a vector suite for both functions at once.
3. **`UNICODE(MID(emoji, 1, 1))`** — whether `MID` slices a pair and, if so, what `UNICODE` then
   says. This depends on an open `LEFT`/`RIGHT`/`MID` surrogate question that upstream has
   explicitly flagged as unresolved under the current compatibility version.
4. **Whether `UNICODE` reads past a leading combining sequence.** `UNICODE("e" & UNICHAR(769))`
   should be `101` under a first-scalar reading; a grapheme-cluster reading would be different, and
   nothing has ruled that out.
5. **Empty-string versus empty-cell versus omitted**, to confirm the three are distinguished as the
   value-universe chapter says they should be.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `UNICODE` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-CHAR-001: CHAR and CODE use the Windows-1252 code page (CHAR(128) is U+20AC, astral characters give CODE 63), while UNICHAR/UNICODE use Unicode scalar values

*Quirk; measured; recorded 2026-11-22.* Applies to CHAR, CODE, UNICHAR, UNICODE.

In Excel 16.0.20527.20088 (UI language 1033 / en-ZA), CHAR maps 1..=255 through Windows-1252
rather than ISO-8859-1, so CHAR(128) is '€' (U+20AC) and CHAR(159) is 'Ÿ' (U+0178). CODE("")
returns #VALUE!, and characters outside Windows-1252 such as an astral emoji
(LEFT(UNICHAR(128512),1)) return 63 ('?'), whereas UNICHAR and UNICODE preserve full Unicode
scalar values (UNICODE(LEFT(UNICHAR(128512),1)) = 128512).

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=CHAR(1)&CHAR(127)&CHAR(128)&CHAR(159)&CHAR(255)` | as typed | `€Ÿÿ` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=CODE("")` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=CODE(LEFT(UNICHAR(128512),1))` | as typed | `63.0` (`0x404f800000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=UNICODE(LEFT(UNICHAR(128512),1))` | as typed | `128512.0` (`0x40ff600000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Measured in Excel 16.0.20527.20088 across f2_rows (rows 1, 4) and dcfl2_text
(rows 12-14) and matched across 1,056/1,056 CHAR, 2,109/2,109 CODE, 1,068/1,068 UNICHAR, and
2,087/2,087 UNICODE full-bank rows.

Evidence:

- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/f2_rows.capture.json`, rows 1 and 4 at `4fb9b728df` — Windows-1252 CHAR mapping and CODE empty-text #VALUE!
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcfl2_text.capture.json`, rows 12 to 14 at `4fb9b728df` — Astral character CODE 63 vs UNICODE 128512
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/f2_rows.capture.json`, sheet f2_rows row 1 column 4 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/f2_rows.capture.json`, sheet f2_rows row 4 column 4 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcfl2_text.capture.json`, sheet dcfl2_text row 14 column 4 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcfl2_text.capture.json`, sheet dcfl2_text row 12 column 4 at `4fb9b728df` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| code point | The Unicode scalar value returned |
| code unit | A UTF-16 16-bit unit; worksheet text is a sequence of these |
| first character | The first Unicode scalar, which may occupy two code units |
| surrogate | A code unit in `0xD800`–`0xDFFF`; half of a pair, not a character |
| round trip | `UNICODE(UNICHAR(n)) = n` |

## Sources

- Microsoft Support, UNICODE function —
  <https://support.microsoft.com/en-us/office/unicode-function-adb74aaa-a2a5-4dde-aff6-966e4e81f16f>
  (the authority for signature, description, and errors; not retrievable while this entry was
  written, so it is cited by reference and nothing here paraphrases it).
- OxFunc `docs/function-lane/FUNCTION_SLICE_TEXT_CORE_AND_COMPATIBILITY_FAMILY_CONTRACT_PRELIM.md`
  — the admitted-slice rule that `UNICODE` returns the first Unicode scalar and rejects invalid
  leading surrogate structures, the `CODE` first-code-unit rule, and the `LEN`/`LENB`
  compatibility-version fork note. Provisional by its own statement.
- OxFunc `docs/function-lane/STRING_BEHAVIOR_RESEARCH_NOTES.md` — local-baseline observations of
  the supplementary-plane round trip and of a dangling-surrogate tail erroring under `UNICODE`.
- Handbook `content/model/01-value-universe.md` (UTF-16 code units, ill-formed UTF-16 as a
  reachable state), `02-coercion-and-lifting.md` (to-text coercion of numeric arguments, lifting,
  error propagation).
