---
schema: efh.function-page/v1
function_id: FUNC.TEXTJOIN
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
family: textjoin
role_in_family: Sole member — the delimiter-aware concatenator, implemented in its own module.
---

# TEXTJOIN

## What it computes

`TEXTJOIN(delimiter, ignore_empty, text1, [text2], …)` flattens its text arguments into one
sequence of items and concatenates them with `delimiter` between consecutive items.

Two things distinguish it from `CONCAT` and from the `&` operator, and both are in the
signature:

1. **The delimiter goes *between* items, not after them.** `n` items produce `n − 1`
   separators. Getting this right by hand — trimming a trailing comma off a `&` chain — is the
   chore the function exists to remove.
2. **`ignore_empty` decides whether empty items count as items.** With `TRUE`, empty cells
   contribute nothing and produce no separator; with `FALSE`, they contribute an empty item and
   therefore still produce a separator, yielding runs of adjacent delimiters. This is the whole
   reason the argument is mandatory rather than optional: there is no defensible default.

Ranges and arrays are flattened rather than treated as single values, so `TEXTJOIN(", ", TRUE,
A1:A10)` is a ten-item join, not a one-item one. That flattening is the function's real power
and the reason it replaced the `&`-chain idiom.

## Arguments

| Argument | Meaning |
|---|---|
| `delimiter` | The separator placed between items. Required. Documented as text — empty, one or more characters in quotes, or a reference to text; a number supplied here is treated as text. |
| `ignore_empty` | `TRUE` to skip empty items, `FALSE` to keep them. Required. |
| `text1`, `text2`, … | The items to join: text values, ranges, or arrays. `text1` required; the rest optional and repeating. |

Microsoft documents an upper bound of 252 text arguments. The Handbook's projected arity for
this entry records a maximum of 255 argument slots in total, which is the same limit counted
with `delimiter` and `ignore_empty` included — the two figures agree rather than conflict, and
the difference is what is being counted.

An empty `delimiter` reduces the function to plain concatenation, which Microsoft names as a
supported use.

## Result and edge cases

Returns `Text`.

Documented: the 32,767-character ceiling on the result, and `#VALUE!` when it is exceeded — the
same cap and the same enforcement side as [REPT](FUNC.REPT.md), and the reason those two are
the text functions most likely to hit it in practice. `TEXTJOIN` over a large range reaches the
cap easily.

Open rather than settled:

- **The Empty versus empty-text distinction.** `ignore_empty` is documented in terms of empty
  *cells*. Whether an empty *string* argument, or an empty string inside an array literal, is
  also skipped is a different question under the shared model, which keeps Empty and text
  distinct ([The value universe](../model/01-value-universe.md)). Unverified.
- **Non-text items.** Numbers, dates and logicals in the item list must be rendered to text,
  and which rendering is used — general format, or something else — is a to-text question the
  shared model states only in outline.
- **Error items.** Whether an error in one of the joined cells propagates or is skipped.
  Propagation is the default under the shared rules, but `TEXTJOIN` scans ranges, and range
  scanning is where per-family policy lives.
- **An array `delimiter`.** Whether a multi-item delimiter is cycled across the joins, or only
  its first element is used, is a behaviour readers report and the Handbook has not checked.

The presence projection for this entry records no open upstream defect stream — this is one of
the few functions on this page with a clean sheet in that respect, which says something about
the defect record and nothing about correctness.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | The result exceeds 32,767 characters. | Documented on Microsoft's `TEXTJOIN` page. |
| — | Error values among the joined items. | Shared propagation rule; unverified for this function. |

## Relationships

- **`CONCAT`** is the same flattening concatenation without a delimiter and without
  `ignore_empty`. **`CONCATENATE`** is the Compatibility-category ancestor of `CONCAT`:
  Microsoft has replaced it with `CONCAT` and retains the old name so existing workbooks keep
  working. `TEXTJOIN` is not part of that supersession chain — it is a distinct, later
  function, not a renamed one, and nothing it does is deprecated.
- **The `&` operator** is the two-item primitive. Everything above is sugar over it, and
  `TEXTJOIN` is the sugar that also solves the trailing-separator and empty-item problems.
- **`TEXTSPLIT`** is the inverse: delimiter-separated text back into an array. `TEXTJOIN` and
  `TEXTSPLIT` round-trip only when no item contains the delimiter — the classic escaping
  problem, which neither function addresses.
- **[REPT](FUNC.REPT.md)** shares the result cap and its `#VALUE!`.
- **[TEXT](FUNC.TEXT.md)** is the usual supplier of formatted items to a `TEXTJOIN` call.

Microsoft documents `TEXTJOIN` as a later addition to the function surface — it is not
available in the oldest supported desktop versions. The Handbook's projected metadata for this
entry carries no version marker, so no availability boundary is asserted here.

## Notes for implementers

The separator rule is "between", which in practice means the emit loop must track whether
anything has been emitted yet rather than appending a delimiter after each item and trimming.
The trim approach breaks as soon as a legitimate item ends with the delimiter.

`ignore_empty` interacts with that flag: a skipped item must not advance the "something has
been emitted" state, or the result gains leading or doubled separators.

The 32,767-unit cap has to be checked as the result grows, not after it is built. `TEXTJOIN`
over a large range can otherwise allocate far past the cap before discovering the documented
`#VALUE!`.

Counting for the cap is in UTF-16 code units, consistent with `LEN` and the value-universe
chapter.

## What has not been checked

No Handbook vector suite exists for `TEXTJOIN`, and no Excel-comparison evidence record names
it. Nobody has checked this function against Excel within the Handbook's record. Only the
signature, the argument meanings, the 252-argument bound and the result cap are documented;
every question in "Result and edge cases" is open.

Inputs worth probing first:

1. **`TEXTJOIN(",", FALSE, "a", "", "b")` versus `TEXTJOIN(",", TRUE, "a", "", "b")`** — the
   `ignore_empty` semantics on *empty text* rather than empty cells, which is the distinction
   the documentation does not draw. `a,,b` versus `a,b` settles it in two cells.
2. **The same pair with an empty *cell*** rather than an empty string, which under the shared
   model is a different input entirely.
3. **`TEXTJOIN({"-","+"}, TRUE, "a", "b", "c")`** — whether an array delimiter cycles.
4. **Non-text items**: `TEXTJOIN(",", TRUE, 1.5, TRUE, TODAY())`, to establish the to-text
   rendering, which is the least pinned part of the shared coercion model.
5. **An error among the items**: `TEXTJOIN(",", TRUE, "a", NA(), "b")`, for the range-scan
   error policy.
6. **The cap boundary**: a join whose result is exactly 32,767 characters and one that is
   32,768, built with `REPT`, probed alongside the same boundary on `REPT` so the two
   enforcement points can be compared.
7. **The argument-count bound**: a call with 252 and with 253 text arguments, to confirm that
   the documented figure is the one enforced and that it is an admission-time rather than a
   runtime boundary ([The call pipeline](../model/03-call-pipeline.md)).

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `TEXTJOIN` that an implementer matching Excel, or a reader explaining an
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

| Term | Meaning on this page |
|---|---|
| item | One value in the flattened sequence being joined; ranges contribute many |
| between-separator rule | `n` items produce `n − 1` delimiters |
| text cap | The 32,767 UTF-16 code-unit ceiling on the result |

## Sources

- Microsoft, "TEXTJOIN function" —
  <https://support.microsoft.com/en-us/office/textjoin-function-357b449a-ec91-49d0-80c3-0e8fc845691c>
  (signature, the delimiter and `ignore_empty` arguments, the number-treated-as-text rule, the
  252-argument bound, the empty-delimiter concatenation use, and the 32,767-character
  `#VALUE!`).
- Handbook, [The value universe](../model/01-value-universe.md) — the text cap in UTF-16 code
  units and the Empty versus text distinction.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — to-text conversion,
  error propagation, and the direct-argument versus range-scan policy split.
- Handbook, [The call pipeline](../model/03-call-pipeline.md) — arity and the admission
  boundary.
- Handbook projections `data/functions/FUNC.TEXTJOIN.json` (arity, no version marker) and
  `data/presence/FUNC.TEXTJOIN.json` (sole-member implementing module, no open defect stream).
