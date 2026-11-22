---
schema: efh.function-page/v1
function_id: FUNC.RIGHT
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
family: text_slice_family
role_in_family: The tail-anchored slice; the mirror of LEFT, and the member whose count argument is optional.
---

# RIGHT

## What it computes

`RIGHT(text, [num_chars])` returns the last `num_chars` characters of `text`.

With `n = LEN(text)`:

```
RIGHT(t, k)  =  characters at positions n - k + 1 … n      for 0 <= k <= n
RIGHT(t, k)  =  t                                          for k > n
RIGHT(t, 0)  =  ""
```

The anchoring is what distinguishes it from [MID](FUNC.MID.md): `RIGHT` counts from the end,
so the window moves when `text` changes length. `MID(t, LEN(t) - k + 1, k)` is the same slice
written from the other end, and the arithmetic in that expression is precisely the arithmetic
`RIGHT` exists to avoid.

"Character" means one UTF-16 code unit — the unit `LEN` counts and the unit the Handbook's
value universe defines worksheet text in. See
[The value universe](../model/01-value-universe.md), "Text, exactly".

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `text` | The string to slice. Required. | — |
| `num_chars` | How many characters to take from the end. Optional; must be zero or greater. | 1 |

The optional second argument is the family's least uniform feature and worth flagging: `RIGHT`
and `LEFT` default their count to 1, while `MID` requires all three of its arguments. A bare
`RIGHT(A1)` is a legal one-character slice; a bare `MID(A1, 2)` is not a legal call at all.

`num_chars` is a numeric slot subject to ordinary to-number coercion; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Text`.

The boundary rules Microsoft documents on the `RIGHT` page:

- `num_chars` must be greater than or equal to zero.
- `num_chars` greater than the length of `text` returns all of `text` — no error, no padding.
- `num_chars` omitted is treated as 1.

Numbers arriving in `text` are converted to text first, which is the standard trap for anyone
slicing formatted values: `RIGHT` sees the general-format rendering of the number, not what the
cell displays. The cell's number format is presentation and does not reach the function. The
to-text rules are shared and only outlined in
[Coercion and lifting](../model/02-coercion-and-lifting.md); this is a real gap, not a
simplification.

Error values propagate. The implementing module named in the presence projection carries an
open upstream defect stream on array positions and count arguments in this family
(`BUG-FUNC-007`), so array-shaped arguments are unsettled here.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | `num_chars` is negative. | Implied by the documented "must be greater than or equal to zero"; the page states the requirement rather than the error value. |
| `#VALUE!` | Non-numeric text in `num_chars`, or a value that cannot convert to text in `text`. | Shared coercion rules, not `RIGHT`-specific. |

Note the honest weakness in the first row: Microsoft's page states a requirement on the
argument, not the error code produced when it is violated. `#VALUE!` is what the sibling
pages document for the analogous violation, and it is what the Handbook expects — but it is an
inference here, and it is on the probe list below.

## Relationships

- **`LEFT`** is the mirror image, with the same optional-count-defaults-to-1 signature.
- **[MID](FUNC.MID.md)** is the unanchored slice; `RIGHT(t, k)` equals `MID(t, LEN(t)-k+1, k)`
  within the valid range, and the equivalence breaks at `k > LEN(t)`, where `RIGHT` clamps and
  the `MID` expression computes a `start_num` below 1 and errors.
- **[RIGHTB](FUNC.RIGHTB.md)** is the byte-counting sibling, documented by Microsoft on the
  same page and published by the Handbook as its own entry.
- **`TEXTAFTER`** is the modern alternative when the tail is delimited rather than counted —
  `TEXTAFTER(a, "@")` instead of `RIGHT(a, LEN(a) - FIND("@", a))`. `RIGHT` is not superseded;
  fixed-width tails are still its job.
- Readers confuse `RIGHT` with `TRIM` when stripping trailing spaces. `RIGHT` cannot strip
  anything — it takes a fixed count from the end regardless of content.

## Notes for implementers

The clamp is one-sided and asymmetric with `MID`: `RIGHT` clamps an over-long count silently,
while `MID` errors on an under-1 start. An implementation that routes `RIGHT` through a shared
`MID` kernel has to clamp *before* computing the start position, or it will turn a documented
clamp into an error.

Because `num_chars` counts code units, `RIGHT` can split a surrogate pair and return a
dangling low surrogate. The value-universe chapter records that ill-formed UTF-16 is a
reachable state in this model, so downstream text handling has to survive it rather than
assume well-formedness.

`RIGHT` on a number is a to-text conversion followed by a slice, and the to-text step is the
part that is not pinned. Any implementation matching Excel here is really matching Excel's
general-format number rendering.

## What has not been checked

No Handbook vector suite exists for `RIGHT`, and no Excel-comparison evidence record names it.
Nobody has checked this function against Excel within the Handbook's record.

Inputs worth probing first:

1. **`RIGHT("abc", -1)`** — the negative count, whose error code this page infers rather than
   cites. One cell settles it.
2. **Fractional counts**: `RIGHT("abcdef", 2.9)`. Truncation toward zero is assumed across
   this family and verified nowhere.
3. **`RIGHT(1234.5)` and `RIGHT(TRUE)`** — the to-text step, which is the least pinned part of
   the shared coercion model and the part most likely to differ between implementations.
4. **`RIGHT("😀ab", 3)` and `RIGHT("ab😀", 1)`** — astral characters at and across the window
   edge, confirming the code-unit count and showing what a split surrogate publishes.
5. **`RIGHT("", 5)`** and `RIGHT` on an empty referenced cell, which are two different inputs
   under the shared model (Empty versus empty text) and may or may not be two different
   results.
6. **Array arguments in both positions**, given the open `BUG-FUNC-007` stream on array
   positions and count arguments in this family.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `RIGHT` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

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
| tail-anchored | The slice is measured from the end of the string, so it moves as the string changes |
| clamp | An over-long count yields the whole string rather than an error |
| code unit | One UTF-16 unit; the unit `LEN` and `num_chars` count in |

## Sources

- Microsoft, "RIGHT, RIGHTB functions" —
  <https://support.microsoft.com/en-us/office/right-function-240267ee-9afa-4639-a02b-f19e1786cf2f>
  (signature, the non-negative requirement, the over-long clamp, and the default of 1).
- Handbook, [The value universe](../model/01-value-universe.md) — text as UTF-16 code units,
  the text cap, and reachable ill-formed UTF-16.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — to-number coercion of
  `num_chars`, the outline-level status of to-text, and the Empty/Missing distinction.
- Handbook projection `data/presence/FUNC.RIGHT.json` — implementing module, sibling set, and
  the `BUG-FUNC-007` defect stream.
