---
schema: efh.function-page/v1
function_id: FUNC.CONCAT
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
family: concat_family
role_in_family: "The modern member: joins text across ranges as well as scalars, with no delimiter."
---

## What it computes

`CONCAT` joins text end to end and returns the result as one text value. Nothing is inserted
between the pieces, nothing is skipped, and the order is the order the arguments are written.

The property that makes it a new function rather than a rename of `CONCATENATE` is how it
consumes a **range**. `CONCAT` flattens a multi-cell argument and joins every cell in it;
`CONCATENATE` does not accept one. `CONCAT(A1:A3)` is a legal, meaningful call.

Stated as an operation: let each argument be expanded to a sequence of scalar values — an array
or range expands in row-major order, a scalar expands to itself — and let each scalar be
converted to text by the ordinary to-text rules. `CONCAT` returns the concatenation of all
those texts, in argument order, with the expansions inlined in place. Empty cells contribute
empty text, which is the same as contributing nothing.

Microsoft's article states the replacement relationship plainly: `CONCAT` "replaces the
CONCATENATE function", though `CONCATENATE` remains available for backward compatibility.

## Arguments

`text1` — required; `text2, …` — optional, up to 253 text arguments in total per the article.
Each argument may be a string or an array such as a cell range.

Two argument-level points worth stating:

- **There is no delimiter argument and no ignore-empty argument.** The article says so
  explicitly, and the projected description carried in `data/functions/FUNC.CONCAT.json` says
  the same. If you need either, the function you want is `TEXTJOIN`.
- **The 253 limit is on argument *slots*, not on cells.** A single range argument can
  contribute far more than 253 values. This is the point readers most often get backwards when
  migrating from `CONCATENATE`, whose 255 slots each had to be written out.

## Result and edge cases

The return kind is `Text`.

- **The 32,767-character cap is a real boundary, and it errors.** Microsoft's article states
  that a result longer than 32,767 characters returns `#VALUE!`. That is the cell text limit
  from [chapter 01](../model/01-value-universe.md), surfacing here as an error rather than as
  truncation — consistent with what that chapter records for the formula path generally.
- **Empty cells inside a range contribute nothing**, so `CONCAT` over a sparse column silently
  closes the gaps. There is no way to tell afterwards how many cells were empty.
- **Numbers and logicals convert to text.** A number contributes its General rendering, so a
  value displayed as `1 234,57` by a cell format contributes the unformatted digits. Wrap in
  `TEXT` when the rendering matters.
- Reference and array arguments are resolved and expanded before conversion, per
  [chapter 03](../model/03-call-pipeline.md).

## Errors

- `#VALUE!` when the joined result would exceed 32,767 characters (documented).
- An error value anywhere in the input propagates
  ([chapter 02](../model/02-coercion-and-lifting.md)) — including an error sitting in one cell
  of a range argument, which is the case where `CONCAT` differs most sharply from an aggregate
  that could choose to skip it.
- A value kind with no text conversion surfaces `#VALUE!` under the shared coercion rules.

## Relationships

- **`CONCATENATE`** is the function `CONCAT` replaces; Microsoft retains it for workbook
  compatibility and its own article warns that it may not be available in future versions. The
  two differ in range handling, in argument limits, and — per the axis chip this page renders —
  in array lifting. See [`CONCATENATE`](FUNC.CONCATENATE.md).
- **`TEXTJOIN`** is the third member of the family and the one to reach for by default: it adds
  the delimiter and ignore-empty arguments that `CONCAT` deliberately lacks.
- **`&` (`FUNC.OP_CONCAT`)** is the operator form. For two or three scalars it is shorter and
  equivalent in intent; it has no range behaviour.
- **`TEXTSPLIT`** is the modern inverse direction.

## Notes for implementers

1. **Expansion order is part of the contract.** Row-major within each argument, arguments in
   written order. Any other order is a different function, and the difference only shows on
   multi-row, multi-column ranges — which is exactly the shape casual tests omit.
2. **Check the length cap incrementally.** A single append that crosses 32,767 must produce the
   error rather than a truncated string; building the whole result and testing afterwards is
   correct but can allocate absurdly on adversarial input.
3. **The cap counts UTF-16 code units**, so a result made of astral characters reaches it at
   half the number of visible characters.
4. The OxFunc reference engine at commit `473efa3` enforces the cap during accumulation, but
   maps the over-length condition to `#CALC!` rather than to the `#VALUE!` the documentation
   names. That divergence is recorded here as a difference between the reference engine and
   Microsoft's documentation; the Handbook has not observed which one live Excel agrees with.

## What has not been checked

No Handbook vector suite exists for `CONCAT`, and no Excel-comparison evidence record names it.
The open items, most valuable first:

- **The over-length result.** Build a call whose result is exactly 32,767 and one whose result
  is 32,768 code units, and record what Excel returns for each. This settles the
  documentation-versus- reference-engine divergence noted above and pins whether the boundary
  is inclusive.
- **Astral characters at the cap** — whether the limit counts code units or characters, and
  whether a result can be truncated mid-surrogate.
- **A range argument containing an error value**, to confirm propagation rather than skipping.
- **A multi-row, multi-column range argument**, to pin the flattening order.
- **The 253rd and 254th argument slots**, to confirm where admission fails and whether it fails
  at formula entry or at evaluation.
- Whether `CONCAT` broadcasts at all when given several arrays of different shapes, which the
  axis chip records as surface-native behaviour but which no probe here has exercised.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `CONCAT` that an implementer matching Excel, or a reader explaining an
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

| Term | Meaning as used here |
|---|---|
| flattening | Expanding an array or range argument into its scalar values in row-major order |
| argument slot | One comma-separated position in the call, as distinct from one contributed value |
| text cap | The 32,767-UTF-16-code-unit limit on a text value |

## Sources

- Microsoft, "CONCAT function" —
  <https://support.microsoft.com/en-us/office/concat-function-9b1a9a3f-94ff-41af-9736-694cbd6b4ca2>
  (that it replaces `CONCATENATE`; up to 253 text arguments; the 32,767-character `#VALUE!`
  boundary; the absence of delimiter and IgnoreEmpty arguments; the `TEXTJOIN` recommendation).
  Retrieved for this page.
- Handbook `content/model/01-value-universe.md` (the text cap and the formula-path error) and
  `content/model/03-call-pipeline.md` (argument preparation and expansion).
- OxFunc `crates/oxfunc_core/src/functions/concat_family.rs` at commit `473efa3` — read for the
  reference engine's expansion order and over-length mapping. Implementation fact only.
