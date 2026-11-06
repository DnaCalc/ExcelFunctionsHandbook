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

`=SERIESSUM(2,0,1,{1,3,2})` returns `15`:

| Coefficient index | Exponent | Term |
|---:|---:|---:|
| 0 | 0 | `1*2^0=1` |
| 1 | 1 | `3*2^1=6` |
| 2 | 2 | `2*2^2=8` |

Changing m to two uses exponents zero, two and four: `=SERIESSUM(2,0,2,{1,3,2})` returns `45`. The exponent step belongs to the powers, rather than changing the coefficient values.

## Quirks

Microsoft documents `#VALUE!` for nonnumeric arguments. The current first-three-argument reader rejects logical values. Its coefficient grid accepts numeric cells, propagates a worksheet error and returns `#VALUE!` for other coefficient kinds; blank or text coefficients are not skipped as they can be in some aggregates.

The current graph evaluates a power before multiplying by its coefficient. A zero coefficient therefore does not erase an invalid power: the current route for `SERIESSUM(-1,0.5,1,{0})` reaches `#NUM!`. Negative bases and fractional exponents need the real power-domain policy. Overflow and underflow of terms can matter even when the mathematical sum would be finite.

## How to implement

1. Read numeric `x,n,m`, then the coefficient grid. Traverse coefficient cells in row order, retaining their positions.
2. For zero-based coefficient index i, form exponent `e=n+i*m` with a binary64 multiplication followed by an addition.
3. Obtain the internal runtime power `crt_pow(x,e)`. This is the current code’s leaf, not the public worksheet POWER wrapper.
4. Multiply by coefficient `a_i`, then add the term to a binary64 accumulator initialized to zero. Preserve the term and sum order and apply worksheet result publication afterward.

For the first example, the accumulator visits one, seven, then fifteen. With a two-row coefficient array `{1,3;2,4}`, the same row traversal at x=2, n=0, m=1 gives `1+6+8+32=47`.

Horner evaluation can avoid repeated powers for an ordinary polynomial. It can also change rounding, overflow and invalid-power behaviour. Treat it as a separate numerical implementation choice until compared; the series definition alone does not identify Excel’s internal power algorithm or all its storage stages.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[SUMPRODUCT](FUNC.SUMPRODUCT.md) can combine explicit powers and coefficients; [POWER](FUNC.POWER.md) evaluates one term’s power. Horner evaluation is a mathematical alternative for polynomial series.

## Sources

- [Microsoft: SERIESSUM function](https://support.microsoft.com/en-us/excel/functions/seriessum-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/math.rs); implementation steps and the limits identified in the text.
