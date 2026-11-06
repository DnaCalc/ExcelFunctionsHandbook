---
schema: efh.function-page/v1
function_id: FUNC.SERIESSUM
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it does
  - Try it
  - Quirks
  - How to implement
  - Version differences
  - Alternatives
  - Sources
family: null
role_in_family: null
---

# SERIESSUM

## What it does

`SERIESSUM(x,n,m,coefficients)` adds terms `a_i*x^(n+i*m)`, starting with coefficient index i=0. N is the initial exponent and m the exponent step; the coefficient count determines the number of terms.

## Try it

`=SERIESSUM(2,0,1,{1,3,2})` returns `15`: `1*2^0+3*2^1+2*2^2=1+6+8`. Choosing m=2 would instead use powers zero, two and four.

## Quirks

Microsoft documents `#VALUE!` for nonnumeric arguments. The current coefficient-grid route also propagates worksheet errors and rejects nonnumeric coefficient cells. A zero coefficient does not necessarily excuse an invalid power operation: the current graph computes each power before multiplying. Negative x with noninteger exponents needs the power function’s real-domain policy.

## How to implement

Walk coefficient cells in the established row order. For index i compute exponent `n+i*m`, evaluate the internal runtime power of x, multiply by the coefficient and add sequentially. The current route calls `crt_pow`, rather than the worksheet POWER wrapper. The example has exact small integer powers and additions. A polynomial Horner or a repeated-power update can be faster, but changes intermediate rounding and exceptional values; publish it as a different implementation choice until compared.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[SUMPRODUCT](FUNC.SUMPRODUCT.md) can combine explicit powers and coefficients; [POWER](FUNC.POWER.md) evaluates one term’s power. Horner evaluation is a mathematical alternative for polynomial series.

## Sources

- [Microsoft: SERIESSUM function](https://support.microsoft.com/en-us/excel/functions/seriessum-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/math.rs); implementation steps and the limits identified in the text.
