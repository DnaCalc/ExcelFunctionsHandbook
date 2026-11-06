---
schema: efh.function-page/v1
function_id: FUNC.GEOMEAN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.GeoMean method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.geomean"
    role: "documented description, the direct-argument versus array scan rules, the #NUM! condition for any data point ≤ 0, and the equation reference"
  - work: "Microsoft Support — GEOMEAN function"
    locator: "https://support.microsoft.com/en-us/office/geomean-function-db1ac48d-25a5-40a0-ab83-0b38980e40d5"
    role: "the worksheet-surface documentation page; not retrievable at curation time (the host refused the request)"
  - work: "Hardy, Littlewood and Pólya, Inequalities"
    locator: "chapter 2 — the means and the AM–GM inequality"
    role: "the inequality that orders this function against HARMEAN and AVERAGE"
  - work: "N. J. Higham, Accuracy and Stability of Numerical Algorithms"
    locator: "chapters on summation and on products"
    role: "the error analysis behind the log-sum versus scaled-product choice"
episodes: []
body_sections:
  - What it does
  - Try it
  - Quirks
  - How to implement
  - Version differences
  - Alternatives
  - Notes
  - Sources
family: geomean_fn
role_in_family: "The multiplicative mean: a stored product followed by exponent normalization and stored LN, reciprocal and EXP stages; the product fallback remains open."
---

# GEOMEAN

## What it does

`GEOMEAN(number1, [number2], ...)` computes the geometric mean of positive numbers: the nth root of their product. For growth rates, supply growth factors such as `1.05`, then subtract one from the result if a rate is wanted.

## Try it

`=GEOMEAN(2,8)` returns `4`. Both observations can be replaced by four without changing their product. `=GEOMEAN(1.1,0.9)` is about `0.9949874371`, showing that equal percentage gains and losses do not cancel multiplicatively.

The retained build 20430 controls give `GEOMEAN(1,2)=1.414213562373095` and `SQRT(2)=1.4142135623730951`, adjacent binary64 values. [FN-GEOMEAN-001](../notes/FN-GEOMEAN-001.json) retains the exact cell inputs and both answers. Their mathematical values agree, but the recorded calculation routes do not publish the same bits.

## Quirks

An admitted zero or negative value gives `#NUM!`. References ignore text, logical values and blanks; direct logical values and numeric text can be admitted. A product may overflow or underflow even when its geometric mean is representable. The current fallback for that case is not an identified Excel route.

## How to implement

1. Collect positive numbers. The current code returns the common input immediately when all admitted values are equal; a singleton is also an identity.
2. Fold the product in input order, storing **each** multiplication with `R(op)` from the [variance recipe](FUNC.VAR.S.md#how-to-implement). Call the positive normal result `P` and count `n`.
3. Extract the integer binary exponent `e=floor(log2(P))` from the encoded exponent. Compute the integer `q=trunc(e/n)`, toward zero. Scale by an exact power of two: `M=P*2^(-q*n)`. Do not use floor for negative exponent division.
4. Store the measured public `LN(M)` result. Separately compute and store `R(1/n)`, then store `R(LN(M)*reciprocal)`. A direct `LN(M)/n` removes an observed rounding stage.
5. Call the measured public EXP on that stored argument, store its result, and scale by `2^q`.

Worked identity reduction: `2*8=16`, `e=4`, `n=2`, `q=2`, `M=1`; LN gives zero, EXP gives one and scaling returns four. A nontrivial reduction, `2*4=8`, uses `e=3`, `q=1`, `M=2`; stored LN times the stored half is about `0.34657359027997265`, EXP is about `1.414213562373095`, then scaling gives about `2.82842712474619`.

The normalized composition and product stores are isolated on the cited scalar controls. LN/EXP depend on their measured instruction models; substituting a language’s libm is a numerical alternative with different possible last bits. Private root internals, global branch boundaries and the overflow/underflow product fallback remain open. The code’s sum-of-logs fallback is an explicit candidate, not an Excel identification.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,212 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.GEOMEAN.json).

Retained controls from the indexed captures:

| Build and recorded CPU | Captured call or arguments | Excel result |
|---|---|---|
| 20430, CPU not recorded | [`GEOMEAN(1,2)`](https://github.com/DnaCalc/CalcFunc-Evidence/blob/4287ba192d2e3a7fd54671ea1cee1c34cb2e84e2/cache/w111-broad-regenerated-20260929/answers-geomean.json.gz) | `1.414213562373095` (`0x3ff6a09e667f3bcc`) |

These are earlier recorded answers, not new Excel runs or an additional cross-build comparison. The function JSON retains each source locator, exact argument bits where available, and input-route status. Call notation does not imply that captured binary64 values were entered as decimal formula literals.

## Alternatives

[AVERAGE](FUNC.AVERAGE.md) suits additive quantities; [HARMEAN](FUNC.HARMEAN.md) suits rates over equal-sized portions. A scaled or logarithmic product is useful for a mathematical implementation if its different rounding history is acceptable.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `GEOMEAN` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-GEOMEAN-001: GEOMEAN(1,2) and SQRT(2) return adjacent values in retained build 20430 controls

*Excel differs from the mathematical answer; measured; recorded 2026-11-06.* Applies to GEOMEAN, SQRT.

In two retained build 20430 controls, GEOMEAN(1,2) returned 1.414213562373095
(0x3ff6a09e667f3bcc), while SQRT(2) returned 1.4142135623730951 (0x3ff6a09e667f3bcd). Both
functions mathematically return the positive square root of two; the SQRT result is the nearest
binary64 value. A fifteen-significant-digit display hides the difference.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `GEOMEAN(1,2)` | number1 `1` (`0x3ff0000000000000`); number2 `2` (`0x4000000000000000`) (cells) | `1.414213562373095` (`0x3ff6a09e667f3bcc`) | `1.4142135623730951` (`0x3ff6a09e667f3bcd`) | Excel 16.0 build 20430, Windows 64-bit (NT 10.00), Workbook Compatibility Version 2; CPU not recorded. Captured 29 September 2026. |
| `SQRT(2)` | number `2` (`0x4000000000000000`) (cells) | `1.4142135623730951` (`0x3ff6a09e667f3bcd`) | — | Excel 16.0 build 20430, Windows 64-bit (NT 10.00), Workbook Compatibility Version 2; CPU not recorded. Captured 29 September 2026. |

Correct value: Exact rational comparison: 2 lies strictly between the squares of the lower and
upper binary64 midpoints around 0x3ff6a09e667f3bcd. Therefore that value is the nearest
binary64 square root of 2.

Basis (measured): Two rows read directly from the landed regenerated W111 capture files. Both
record Excel build 20430, Windows 64-bit, Compatibility Version 2 and exact-double bulk Value2
inputs, on 29 September 2026. The calls are to different functions in the same recorded
environment class; this is not a comparison between builds and no fresh Excel calls were made.

- Open: Whether other inputs or environments retain the same difference; the two rows do not certify whole-function agreement or identify private Excel code.

Evidence:

- CalcFunc-Evidence `cache/w111-broad-regenerated-20260929/answers-geomean.json.gz`, w111broad-20260929-GEOMEAN-00003 at `4287ba192d` — Recorded GEOMEAN(1,2) answer and capture provenance.
- CalcFunc-Evidence `cache/w111-broad-regenerated-20260929/answers-sqrt.json.gz`, w111broad-20260929-SQRT-00014 at `4287ba192d` — Recorded SQRT(2) answer and capture provenance.
- DnaCalc `crates/calc-func/src/ported/excel_numeric/x87_chains.rs`, excel_root_x87 at `da3ecc4fb1` — Current normalized LN, reciprocal and EXP root reconstruction.
- CalcFunc-Evidence `cache/w111-broad-regenerated-20260929/answers-geomean.json.gz`, w111broad-20260929-GEOMEAN-00003 at `4287ba192d` — Capture provenance records exact-double bulk Value2 cell input. (readings)
- CalcFunc-Evidence `cache/w111-broad-regenerated-20260929/answers-sqrt.json.gz`, w111broad-20260929-SQRT-00014 at `4287ba192d` — Capture provenance records exact-double bulk Value2 cell input. (readings)

<!-- efh:notes end -->

## Sources

- [Microsoft: GEOMEAN function](https://support.microsoft.com/en-us/excel/functions/geomean-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/excel_numeric/x87_chains.rs); implementation steps and the limits identified in the text.
- [Product and fallback](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/statistics.rs).
- [Numerical fingerprint scope](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/docs/generated/numeric-fingerprints.md).
- [FN-GEOMEAN-001](../notes/FN-GEOMEAN-001.json): two retained build 20430 controls, with a rational check of the nearest square root.
