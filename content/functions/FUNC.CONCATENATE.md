---
schema: efh.function-page/v1
function_id: FUNC.CONCATENATE
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
role_in_family: "The legacy member: scalar-only joining, retained for workbook compatibility."
---

## What it computes

`CONCATENATE` joins two or more text strings into one string, in argument order, with nothing
inserted between them.

It is the legacy member of its family. Microsoft's article states that it "has been replaced
with the CONCAT function", recommends `CONCAT` going forward, and warns that `CONCATENATE` "may
not be available in future versions of Excel". Excel keeps the name so that existing workbooks
keep calculating; new formulas should use `CONCAT`, `TEXTJOIN`, or `&`.

The substantive difference from `CONCAT` is not the name. `CONCATENATE` is **scalar-shaped**:
each argument contributes one value, not a flattened range. A multi-cell range handed to
`CONCATENATE` does not join its cells.

## Arguments

`text1` — required, the first item to join; may be text, a number, or a cell reference. `text2,
…` — optional. The article states the ceiling as up to 255 items totalling a maximum of 8,192
characters.

Note that those are two different limits from the ones `CONCAT` carries (253 slots, 32,767
characters), and the 8,192-character figure is documented for `CONCATENATE` specifically. If
you are migrating formulas between the two functions, both limits change.

The argument position that most often surprises readers is any position holding a multi-cell
range. `CONCAT` accepts it and flattens; `CONCATENATE` does not consume it as a range at all,
and what happens instead depends on the shared array/implicit-intersection machinery rather
than on this function.

## Result and edge cases

The return kind is `Text`.

- Numbers and logicals convert to text by the ordinary rules; a number contributes its General
  rendering, not its displayed format.
- Empty cells contribute empty text.
- **Array lifting is asymmetric across argument positions.** The axis chip this page renders
  records `CONCATENATE` as a by-index scalar-array lift over positions 0, 1 and 2 — that is,
  only the first three arguments broadcast over an array — whereas `CONCAT` lifts natively. The
  OxFunc registry annotates that entry as verified against live Excel 16.0 build 20026; the
  Handbook reports that as an upstream record rather than as its own measurement. It is unusual
  enough to be worth stating twice: with `CONCATENATE`, whether an array argument spreads
  depends on *which slot it is in*.

## Errors

- An error value in any argument propagates
  ([chapter 02](../model/02-coercion-and-lifting.md)).
- A value kind with no text conversion surfaces `#VALUE!` under the shared coercion rules.
- Microsoft's article documents `#NAME?` as the usual symptom of a common authoring mistake —
  missing quotation marks around a text argument. That is a formula-entry problem rather than a
  behaviour of the function, and it is listed here because the article lists it and readers
  arrive looking for it.

The article's other two "common problems" are also authoring issues rather than function
semantics: quotation marks appearing in the result usually mean a missing comma between
arguments, and words running together mean no space argument was supplied. `CONCATENATE`
inserts nothing on its own.

## Relationships

- **`CONCAT`** is the modern replacement, named as such by Microsoft. It accepts ranges and
  carries different limits. See [`CONCAT`](FUNC.CONCAT.md).
- **`TEXTJOIN`** adds a delimiter and an ignore-empty flag, and is usually the better migration
  target for a `CONCATENATE` chain that interleaves separator literals.
- **`&` (`FUNC.OP_CONCAT`)** is the operator form; the article itself recommends it as the
  simpler syntax. It is not identical in shape to either function — it is a binary operator
  with the operator pipeline's own lifting behaviour.
- **`TEXT`** is what the article recommends composing with when the joined pieces need
  formatting; `CONCATENATE` will not format anything for you.

Despite its status, `CONCATENATE` is **not** in Excel's Compatibility category. It sits in the
Text category alongside `CONCAT`, and Microsoft's deprecation statement is in prose rather than
in the category taxonomy.

## Notes for implementers

1. **Do not implement `CONCATENATE` as `CONCAT` with a different arity.** The range behaviour
   and the position-dependent lifting are genuinely different, and the difference is observable
   in ordinary workbooks.
2. **Carry both documented limits separately** (255 slots, 8,192 characters here; 253 and
   32,767 there). They are not derived from a shared constant.
3. **The lift positions are irreducible structure.** The `[0,1,2]` position list is not a rule
   with a reason; it is a fact to be copied, and the call-pipeline chapter says so about this
   whole axis.
4. The OxFunc reference engine at commit `473efa3` prepares each argument as a single value and
   rejects a multi-cell range argument as an unsupported value kind, in contrast to its own
   `CONCAT`, which expands. Implementation fact about OxFunc only.

## What has not been checked

No Handbook vector suite exists for `CONCATENATE`, and no Excel-comparison evidence record
names it. The Handbook has verified nothing about this function against Excel. Worth probing
first:

- **`CONCATENATE(A1:A3)`** with three filled cells, entered in a cell in the same rows and in a
  cell outside them, which separates implicit intersection from an outright error;
- **the same call in slot 1, slot 3 and slot 4**, which is the direct test of the recorded
  `[0,1,2]` lift-position list;
- **the 8,192-character limit** — whether it is enforced, and with which error, since the
  documented figure is far below the text cap that `CONCAT` errors at;
- **the 255th and 256th argument slots**, and whether over-arity is refused at formula entry;
- whether a number argument's text conversion follows the workbook locale.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `CONCATENATE` that an implementer matching Excel, or a reader explaining
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

| Term | Meaning as used here |
|---|---|
| by-index lift | Broadcasting over arrays only at a named list of argument positions |
| scalar-shaped | Each argument contributes one value; ranges are not flattened |
| retained for compatibility | Kept in the function surface so existing workbooks keep calculating |

## Sources

- Microsoft, "CONCATENATE function" —
  <https://support.microsoft.com/en-us/office/concatenate-function-8f8ae884-2ca8-4f7a-b093-75d702bea31d>
  (that it has been replaced by `CONCAT` and may not be available in future versions; up to 255
  items totalling 8,192 characters; the `&` and `TEXT` recommendations; the `#NAME?` and
  missing-comma common problems). Retrieved for this page.
- Handbook `content/model/03-call-pipeline.md` (the `ByIndexScalarArrayLift` axis and its
  empirically-pinned status) and `content/model/02-coercion-and-lifting.md` (to-text
  conversion, error propagation).
- OxFunc `crates/oxfunc_core/src/functions/concat_family.rs` at commit `473efa3` — read for the
  reference engine's scalar preparation and for the registry annotation recording the `[0,1,2]`
  lift positions as verified against live Excel 16.0 build 20026. Upstream record, quoted as
  such.
