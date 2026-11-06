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
  - Sources
family: geomean_fn
role_in_family: "The multiplicative mean: a stored product followed by exponent normalization and stored LN, reciprocal and EXP stages; the product fallback remains open."
---

# GEOMEAN

## What it does

`GEOMEAN(number1, [number2], ...)` computes the geometric mean of positive numbers: the nth root of their product. For growth rates, supply growth factors such as `1.05`, then subtract one from the result if a rate is wanted.

## Try it

`=GEOMEAN(2,8)` returns `4`. Both observations can be replaced by four without changing their product. `=GEOMEAN(1.1,0.9)` is about `0.9949874371`, showing that equal percentage gains and losses do not cancel multiplicatively.

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

## Alternatives

[AVERAGE](FUNC.AVERAGE.md) suits additive quantities; [HARMEAN](FUNC.HARMEAN.md) suits rates over equal-sized portions. A scaled or logarithmic product is useful for a mathematical implementation if its different rounding history is acceptable.

## Sources

- [Microsoft: GEOMEAN function](https://support.microsoft.com/en-us/excel/functions/geomean-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/excel_numeric/x87_chains.rs); implementation steps and the limits identified in the text.
- [Product and fallback](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/statistics.rs).
- [Numerical fingerprint scope](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/docs/generated/numeric-fingerprints.md).
