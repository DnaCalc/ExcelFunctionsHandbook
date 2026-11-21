---
schema: efh.function-page/v1
function_id: FUNC.OP_CONCAT
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
family: operator_compare_concat_family
role_in_family: "The only text-producing operator: it shares the family's broadcast machinery with the six comparisons but coerces to text where they order kinds."
---

## What it computes

`A & B` converts both operands to text and returns their concatenation, left operand first.

OxFunc's provisional compare/concat contract states the rule exactly that way — "`OP_CONCAT`
coerces operands to text and concatenates them in order" — with current-baseline findings
`="a"&1` yielding `"a1"` and `=1&TRUE` yielding `"1TRUE"`.

The whole difficulty of this operator is hidden inside "converts to text", because that
conversion is a *rendering*, and rendering is where locale and formatting live:

- **Numbers.** Rendered by general formatting rules (chapter 02). This is not the cell's
  display format: a cell showing `1 234,50 €` concatenates as whatever general formatting
  makes of the underlying number, not as what the cell shows. The decimal separator used in
  that rendering is locale-dependent, and chapter 02 flags the locale-dependent surface as an
  open area — so `=1.5&""` is not guaranteed to be `"1.5"` everywhere.
- **Logicals.** Render as the text `TRUE` and `FALSE` — in English, on the observed
  baseline. Whether that rendering localizes is not recorded here.
- **Dates and times.** A date is a number wearing a format (chapter 01), so `&` renders the
  serial number, not the date. `=A1&""` on a date cell gives the serial, which is the single
  most common surprise on this operator and the reason `TEXT(A1,"yyyy-mm-dd")` exists.
- **Empty.** Concatenating a blank contributes nothing, which is why `=A1&""` is the standard
  idiom for "give me the text of this cell, or an empty string".

Chapter 01's text model applies to the result: worksheet text is a sequence of UTF-16 code
units capped at 32,767 units, and it records that producing an over-cap string in a *formula*
path yields `#VALUE!` where the interop path truncates. `&` is the easiest way in the whole
function surface to reach that cap.

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | Left operand; rendered first. Required. |
| 1 | `B` | Right operand; rendered second. Required. |

Arity is exactly 2; no optional arguments, no defaults. Order is load-bearing — concatenation
is not commutative. A chain `a & b & c` is two applications of this two-argument operator,
and because concatenation *is* associative, the grouping does not change the result.

## Result and edge cases

Returns `Text` (`KernelSignatureClass::TextToText`).

- **Blank operands.** Contribute the empty string.
- **Error operands.** Propagate; `&` does not render an error as its display text. `=NA()&""`
  is `#N/A`, not `"#N/A"`.
- **Arrays.** The family's broadcast rule applies, and the contract's findings show it
  clearly: `={"a","b"}&{"x";"y"}` produces the 2×2 outer product, and
  `={"a","b"}&{"x","y","z"}` produces two joined values plus `#N/A` at the coordinate the
  left operand cannot supply. Concatenating a row against a column is therefore a compact way
  to build a full cross-product of labels.
- **The 32,767-unit cap.** Reachable by repeated concatenation. See chapter 01 for the
  formula-path-versus-interop-path split, which is an observed distinction rather than a
  documented one.

## Errors

| Error | Condition |
|---|---|
| any incoming error | Propagates unchanged; `ErrorCollapseProfile::None`. |
| `#VALUE!` | Documented in chapter 01 for the formula path when a produced string exceeds the 32,767-code-unit cap (observed with `REPT`; not re-observed for `&` here). |
| `#N/A` | Inside an array result only: a coordinate neither operand supplies. |

`data/functions/FUNC.OP_CONCAT.json` records no Microsoft documentation URL (`docs` is
`null`).

## Relationships

- `CONCATENATE` — the legacy function form. `data/functions/FUNC.CONCATENATE.json` places it
  in the **Compatibility** category: Microsoft's modern replacement is `CONCAT`, and the old
  name is retained so existing workbooks keep working. New formulas should use `CONCAT` or
  `TEXTJOIN`; `CONCATENATE` is kept for workbook compatibility, not recommended.
  (<https://support.microsoft.com/en-us/office/concatenate-function-8f8ae884-2ca8-4f7a-b093-75d702bea31d>)
- `CONCAT` — the modern replacement, which accepts ranges as well as scalars.
  (<https://support.microsoft.com/en-us/office/concat-function-9b1a9a3f-94ff-41af-9736-694cbd6b4ca2>)
- `TEXTJOIN` — the delimiter-aware, empty-skipping join.
  (<https://support.microsoft.com/en-us/office/textjoin-function-357b449a-ec91-49d0-80c3-0e8fc845691c>)
- `&` is not superseded by any of them. It remains the operator; the functions are
  alternatives with extra arguments, not replacements.
- `TEXT`, `VALUE`, `T`, `N` — explicit conversion. If the rendering matters, `TEXT` states
  the format instead of inheriting the general one.
- `EXACT` — because the natural follow-up to building text with `&` is comparing it, and `=`
  on text is case-insensitive.

## Notes for implementers

- The number-to-text rendering is the entire compatibility surface of this operator. Pin it
  explicitly: significant digits, when scientific notation kicks in, how integers and
  negative zero render, and what the decimal separator is. A "close enough" rendering
  produces text that differs from Excel's in ways that are trivially visible to users and
  fatal to comparisons.
- Do not render errors. An error operand short-circuits to the error value.
- Count the cap in UTF-16 code units, not characters and not bytes (chapter 01). Astral
  characters count double.
- Concatenation is associative, so chains may be evaluated in any grouping — but the
  *rendering* of each operand must happen before joining, not after, or a numeric operand
  could be rendered in a different context.

## What has not been checked

No Handbook vector suite covers `&`, and no Excel-comparison evidence record is attached to
this page. Nothing here is a measurement.

Probes worth running first:

1. **The number renderer.** `=x&""` over a corpus spanning integers, large and small
   magnitudes, values needing 15 versus 17 significant digits, negative zero, and values in
   the scientific-notation regime. This produces the rendering table that a compatibility
   implementation needs and that no chapter currently supplies.
2. **Locale.** The same corpus under at least two locales with different decimal separators,
   plus `=TRUE&""` under a non-English UI, to establish whether literal rendering localizes.
3. **The cap.** Build a string just under and just over 32,767 code units by concatenation,
   with BMP and astral characters, to confirm the formula-path `#VALUE!` behaviour for `&`
   specifically.
4. **Dates.** `=A1&""` on date, time and datetime cells with various cell formats, confirming
   the serial-number rendering.
5. **Broadcast padding.** Reproduce the row-against-column and mismatched-length findings, to
   confirm the `#N/A` coordinate rule holds for the text operator as it does for the
   comparisons.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `OP_CONCAT` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

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

| Machine name | Meaning |
|---|---|
| `Arity { min: 2, max: 2 }` | Exactly two operands |
| `ArgPreparationProfile::ValuesOnlyPreAdapter` | References resolved to values before the operator runs |
| `CoercionLiftProfile::Custom` | Operator-specific coercion (to text) |
| `KernelSignatureClass::TextToText` | Kernel maps text to text |
| `LiftBroadcastProfile::SurfaceNative` | The operator does its own array lifting |
| `ErrorCollapseProfile::None` | Error operands propagate unchanged |
| `Text` | UTF-16 code-unit string, capped at 32,767 units |

## Sources

- `data/functions/FUNC.OP_CONCAT.json` at OxFunc `473efa3` — identity, arity, signature
  `A & B`, `TextToText` kernel class. `docs` is `null`: **no Microsoft documentation URL is
  recorded for this operator entry.** Microsoft's account of `&` lives in the support article
  *Calculation operators and precedence in Excel*, not yet linked from the data projection.
- `data/presence/FUNC.OP_CONCAT.json` — implementing module
  `crates/oxfunc_core/src/functions/operator_compare_concat_family.rs`.
- `data/functions/FUNC.CONCATENATE.json` (category `Compatibility`),
  `data/functions/FUNC.CONCAT.json`, `data/functions/FUNC.TEXTJOIN.json` — the function-form
  relatives and their Microsoft URLs, quoted above.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OPERATOR_COMPARE_CONCAT_FAMILY_CONTRACT_PRELIM.md`
  — the to-text-and-join rule, the broadcast lanes, and the `="a"&1`, `=1&TRUE`,
  `={"a","b"}&{"x";"y"}`, `={"a","b"}&{"x","y","z"}` findings; provisional by its own header.
- Handbook `content/model/01-value-universe.md` (text as UTF-16 code units, the 32,767 cap,
  the formula-versus-interop split), `02-coercion-and-lifting.md` (to-text outline and the
  locale caveat), `03-call-pipeline.md`.
