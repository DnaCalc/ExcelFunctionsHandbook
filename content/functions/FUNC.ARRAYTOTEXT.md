---
schema: efh.function-page/v1
function_id: FUNC.ARRAYTOTEXT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — ARRAYTOTEXT function"
    locator: "https://support.microsoft.com/en-us/office/arraytotext-function-9cdcad46-2fa5-4c6b-ac92-14e7bc862b8b"
    role: "documented signature, the two format modes, worked output examples, and the #VALUE! condition"
  - work: "OxFunc — FUNCTION_SLICE_ARRAY_TEXT_SPLIT_FAMILY_CONTRACT_PRELIM.md"
    locator: "docs/function-lane/FUNCTION_SLICE_ARRAY_TEXT_SPLIT_FAMILY_CONTRACT_PRELIM.md"
    role: "upstream admitted-slice rendering rules for concise and strict modes"
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
family: array_text_split_family
role_in_family: "The array-to-text half of the split/render pair: flattens a grid into one string."
---

## What it computes

`ARRAYTOTEXT` renders a whole array — or a range — as a single string.

It has the same two registers as its scalar sibling [`VALUETOTEXT`](FUNC.VALUETOTEXT.md), and the
difference between them is sharper here because an array has structure to lose.

**Concise (`format = 0`, the default)** walks the array in row-major order and joins the elements'
General-format renderings with `", "`. Row structure is *not* preserved: a 3×2 array and a 1×6
array with the same elements in the same order render identically. Microsoft's page shows this
directly — a two-column, three-row range renders as a single comma-separated run.

**Strict (`format = 1`)** renders an array literal: braces around the whole thing, commas between
columns, semicolons between rows, text elements quoted, and Booleans, numbers and errors left
unquoted. This preserves shape and kind, and it reparses. Microsoft's page shows a 3×2 range
becoming `{TRUE,#VALUE!;1234.01234,"Seattle";"Hello","1,123"}` — which is worth studying, because
it contains the two facts that matter most: an error appears as bare `#VALUE!` (a rendered error,
not a propagated one) and the number `1123` formatted with a thousands separator appears as the
*quoted string* `"1,123"`, because concise/strict rendering follows the cell's displayed text, not
its underlying number.

That last point is the trap. `ARRAYTOTEXT` renders what the cells *show*, so a formatted number can
come back as text that no longer parses as that number.

Upstream OxFunc's contract for this family records the same shape from the implementation side:
row-major concise rendering; strict rendering with braces, comma and semicolon separators, quoted
text cells and unquoted logical, error and number cells; and scalar inputs promoted to 1×1 arrays.

## Arguments

`ARRAYTOTEXT(array, [format])`

| Argument | Required | Meaning (as documented by Microsoft) |
|---|---|---|
| `array` | yes | The array or range to render as text. |
| `format` | no | `0` (default) concise; `1` strict. Anything else returns `#VALUE!`. |

Two things about the first position. It accepts a **range**, not only an array value — which is the
usual way it is used, and which means the reference is resolved to values before the function sees
it (see [the call pipeline](../model/03-call-pipeline.md)). And a **scalar is accepted**, promoted
to a 1×1 array; upstream records that promotion explicitly, and it is why `ARRAYTOTEXT("abc")` is
not an error.

The `format` argument admits exactly `0` and `1`; Microsoft documents anything else as `#VALUE!`.

## Result and edge cases

The return kind is `Text` — always a single scalar string, never an array. `ARRAYTOTEXT` is a
reducer: it is the one place in this part of the catalogue where a grid goes in and a scalar comes
out.

- **The 32,767-code-unit cap is genuinely reachable.** Rendering a large range produces a long
  string, and strict mode adds quoting overhead. The cap and its two enforcement paths — truncation
  on the interop path, `#VALUE!` on the formula path — are described in
  [the value universe](../model/01-value-universe.md). Which one `ARRAYTOTEXT` meets has not been
  probed.
- **Errors inside the array are rendered, not propagated**, in the documented example. That is a
  striking exception to the usual discipline in
  [coercion and lifting](../model/02-coercion-and-lifting.md), where coercion never silently
  discards a worksheet error — here the error is neither discarded nor propagated but *displayed*.
  Whether an error arriving as the whole `array` argument behaves the same way is a separate
  question and is not settled here.
- **Empty cells inside a range** have no obvious rendering. Whether they contribute an empty
  element, a zero, or nothing at all to the concise join is not established.
- **Row-major order is the documented traversal**, so concise mode reading order matches how you
  read the range on screen.
- **Concise mode loses shape irreversibly.** If you need the shape back, use strict mode.

## Errors

| Error | Basis |
|---|---|
| `#VALUE!` when `format` is anything other than `0` or `1` | documented on Microsoft's page (retrieved for this entry) |
| `#VALUE!` if the rendered result exceeds the worksheet text cap on the formula path | expected from the value-universe chapter's account of the cap; not probed for this function |
| any error value arriving as the `array` argument itself | behaviour not established here — see below |

## Relationships

- [`VALUETOTEXT`](FUNC.VALUETOTEXT.md) — the scalar counterpart, same `format` argument, same two
  registers. The pair splits on input shape.
- [`TEXTSPLIT`](FUNC.TEXTSPLIT.md) — the closest thing to an inverse, and its sibling in the same
  upstream implementation module. `TEXTSPLIT` turns a delimited string into a grid; concise
  `ARRAYTOTEXT` turns a grid into a delimited string. They do not compose cleanly, because concise
  mode drops row structure and quotes nothing.
- `TEXTJOIN` — the older and more controllable flattener: you choose the delimiter and whether to
  ignore empties, but you get no shape and no quoting. `TEXTJOIN(", ", FALSE, range)` is the
  hand-rolled approximation of concise mode, and it differs at least in how it renders errors and
  formatted numbers.
- `CONCAT` — flattens a range with no delimiter at all.
- `TEXT` — formats a single number with a supplied format string; unrelated except by name
  confusion.
- Strict mode is the function to reach for when you want to *see* what a dynamic-array formula
  actually produced, which makes it a debugging tool as much as a data function.

## Notes for implementers

- **Reuse the engine's General-format renderer**, exactly as for `VALUETOTEXT`. Concise mode is
  defined by reference to cell display, and the documented example shows that cell *formatting*
  reaches the output. An implementation that renders from the underlying double will disagree with
  Excel on every formatted cell.
- **Strict mode's separators are formula-language separators**, which are locale-dependent in
  Excel. A strict rendering that is meant to reparse must use the separators of the locale it will
  be pasted into. This is a real portability hazard and deserves a test.
- **Quote only what needs quoting.** The documented example quotes text and leaves Booleans,
  numbers and errors bare; a uniform-quoting implementation is visibly wrong.
- **Escape inner quotes** by doubling, and test with a string that contains one.
- **Traverse row-major** and do not assume square.
- **Check the result against the text cap** and decide which failure mode you implement.
- **Decide what an empty cell renders as** and write it down; it is the most likely silent
  divergence.

## What has not been checked

There is no Handbook vector suite for `ARRAYTOTEXT`, and no Handbook evidence record comparing any
implementation against Excel. Nothing on this page is a measurement. The rendering rules above come
from Microsoft's documented examples and from upstream OxFunc's provisional family contract, which
states its own boundary: its packet proves core rendering semantics for a seeded baseline and does
not claim broader locale or collation closure.

The probes that would settle the open points:

1. **Empty cells.** `ARRAYTOTEXT(A1:B1, 0)` and `(…, 1)` with one cell empty. Three plausible
   renderings — nothing, an empty element, a zero — and they are distinguishable in one cell each.
2. **An error as the whole argument.** `ARRAYTOTEXT(NA())` — rendered as text, or propagated?
   `ISTEXT` around it answers it, and it is the same question flagged on the `VALUETOTEXT` page.
3. **Locale of strict separators.** Produce a strict rendering on a machine whose formula argument
   separator is `;` and see whether the column separator changes with it.
4. **Round trip.** Paste a strict rendering back into the formula bar and compare with `EXACT` and
   with the original array. Any failure is a bug in either the renderer or your understanding of
   the grammar, and it is worth knowing which.
5. **The text cap.** Render a range large enough to exceed 32,767 code units and record which of
   the two documented cap behaviours occurs on the formula path.
6. **Formatted numbers.** Confirm the documented behaviour that a thousands-separated display value
   renders as quoted text in strict mode, then check dates, percentages and currency, which are the
   other formats most likely to appear in a real range.
7. **Rich values** — a linked data type in the range — to see whether the core projection is
   rendered.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `ARRAYTOTEXT` that an implementer matching Excel, or a reader explaining
an unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's
answer and where it was recorded; "measured" means the statement is read off recorded Excel
answers, "inferred" marks what goes beyond them. Where older text on this page says a behaviour
was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

### FN-CONCAT-001: Number-to-text emits 15 significant digits via an exact Q16.64 split on [1, 65536) and x87 low-nibble scaling into Q0.64 elsewhere

*Detail for matching Excel's bits; inferred; recorded 2026-11-22.* Applies to ARRAYTOTEXT, CONCAT, CONCATENATE, DOLLAR, FIXED, OP_CONCAT, TEXT, TEXTJOIN, VALUETOTEXT.

Every worksheet consumer of general 15-significant-digit number-to-text conversion (&, CONCAT,
CONCATENATE, VALUETOTEXT, ARRAYTOTEXT, FIXED, DOLLAR, TEXT, TEXTJOIN, and general cell
formatting) shares one routine (first_precision). For 1.0 <= |x| < 65,536.0 (0 <= exp2 < 16),
Branch 1 splits the binary64 significand directly into an integer part w (1..=65535, d digits)
and an exact Q0.64 fraction f with no FPU scaling, then adds the truncated half-ulp constant
floor(2^63 / 10^(15 - d)); because floor(2^63 / 10^k) < 2^63 / 10^k, exact binary ties below
65,536 never round up, and [1, 10) rounds up only past +0.07204 ulp64(v). For |x| < 1.0 or |x|
>= 65,536.0, Branch 2 computes p = ceil((exp2 + 1) / 3.3219) (0 for [0.5, 1)), multiplies x by
10^-p using the low-nibble-first x87 PC64 table chain (10^+-1..15, 10^+-16..240, 10^+-256) with
one 10^-1 step if scaled v >= 1.0, shifts the 64-bit significand mv into Q0.64 by a truncating
right shift, and adds floor(2^63 / 10^k) (k in 15..=17). That explains why [0.5, 1) and
below-2^k values require +0.372036854775808 ulp64(v) to round up, while above-2^k values
exhibit a period-8 oscillation +3.37203685 + 1.25 * (mv & 7) ulp64(10v). See
https://github.com/DnaCalc/DnaCalc/blob/344d70dcea3ea77cbbeee1595f96694630a9610a/docs/articles/last-bit-number-text.md.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `CONCAT(3.645252540996335, "")` | text1 `3.645252540996335` (`0x400d297a2a09ebd8`); text2 `` (cells) | `3.64525254099634` | Excel 16.0.20527.20196, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics |
| `CONCAT(86497.03926400005, "")` | text1 `86497.03926400005` (`0x40f51e10a0d349c2`); text2 `` (cells) | `86497.0392640001` | Excel 16.0.20527.20196, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics |
| `CONCAT(8.594243319945645e-293, "")` | text1 `8.594243319945645e-293` (`0x034b71c2dc7a8ed6`); text2 `` (cells) | `8.5942433199457E-293` | Excel 16.0.20527.20196, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics |

Basis (inferred): Measured on 21,377 probe and W111 rows (21,377 of 21,377 matched), 29
formerly failing oracle-corpus cells (29 of 29 matched across dc_q9rb, catalogue, and
fn7_format_context), and three prospective frozen proofs in nx8 on Excel 16.0.20527.20196: 600
of 600 on frozen_cb9_predictions.jsonl, 32 of 32 on the 4-binade discriminator (exp2 in {-971,
-486, +484, +969}, settling ceil(frexp_k / 3.3219) 32/32 vs ceil(frexp_k * log10(2)) 0/32), and
20 of 20 on the 10^-1 sliver step. Post-study witness banks: CONCAT 3,578 of 3,578, CONCATENATE
3,133 of 3,133, FIXED 3,783 of 3,783, VALUETOTEXT 2,571 of 2,571.

Inferred, not measured: Whether Excel implements Branch 1 via integer bit-shifts or x87
fistp+fsub is unobservable from outputs and inferred; both give identical (w, f) for every
binary64 in [1.0, 65536.0).

- Open: Nothing remains open in first_precision itself. 132 post-study TEXT bank rows (0.00000000000000E+00 / General) return NotImplemented due to format-string/locale handling owned by dc-j9k.65 and dc-0f1k.8.

Evidence:

- DnaCalc `crates/calc-func/src/number_text.rs`, first_precision; scale_nibble_pc64; Q64_HALF_POW10 at `86427f7ab2` — Q0.64 two-branch number-to-text implementation (nx8)
- DnaCalc `docs/articles/last-bit-number-text.md` at `344d70dcea` — Last-bit article on Excel number-to-text
- CalcFunc-Evidence `readings/excel-20527/studies/cb9.json.gz` at `4ef2c244d7` — 632 scored cb9/nx8 holdout and binade-4 rows
- CalcFunc-Evidence `readings/excel-20527/studies/nx8.json.gz` at `4ef2c244d7` — 20 nx8 10^-1 sliver proof rows
- [`content/notes/evidence/2026-11-22/frozen-holdouts-provenance.tsv`](../../content/notes/evidence/2026-11-22/frozen-holdouts-provenance.tsv), cb9 — Pre-ask SHA-256 hashes and frozen holdout scores
- CalcFunc-Evidence `readings/excel-20527/studies/cb9.json.gz`, scored_600[295] at `4ef2c244d7` (readings)
- CalcFunc-Evidence `readings/excel-20527/studies/cb9.json.gz`, scored_600[178] at `4ef2c244d7` (readings)
- CalcFunc-Evidence `readings/excel-20527/studies/cb9.json.gz`, scored_binade4[6] at `4ef2c244d7` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| concise mode | `format = 0`; row-major join of General-format renderings, shape discarded |
| strict mode | `format = 1`; array-literal rendering with braces, separators and quoting |
| row-major | Traversal order: across each row, then down |
| reducer | A function that takes an array and returns a scalar |
| core projection | The traditional-gamut value a rich value presents to legacy surfaces |

## Sources

- Microsoft Support, ARRAYTOTEXT function —
  <https://support.microsoft.com/en-us/office/arraytotext-function-9cdcad46-2fa5-4c6b-ac92-14e7bc862b8b>
  (retrieved for this page; source of the signature, both format-mode descriptions, the worked
  concise and strict examples quoted above, and the documented `#VALUE!` for a `format` outside
  `{0,1}`).
- OxFunc `docs/function-lane/FUNCTION_SLICE_ARRAY_TEXT_SPLIT_FAMILY_CONTRACT_PRELIM.md` — the
  admitted-slice rendering rules, scalar-to-1×1 promotion, and the family's stated scope boundary.
  Provisional by its own statement.
- Handbook `content/model/01-value-universe.md` (arrays, the text cap and its two enforcement
  paths, rich values), `02-coercion-and-lifting.md` (error discipline), `03-call-pipeline.md`
  (reference resolution before the function runs).
