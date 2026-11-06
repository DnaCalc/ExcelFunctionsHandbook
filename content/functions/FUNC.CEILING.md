---
schema: efh.function-page/v1
function_id: FUNC.CEILING
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

# CEILING

## What it does

`CEILING(number,significance)` chooses a multiple of a signed step. For positive inputs with positive significance it rounds outward. For negative inputs, positive significance moves toward zero and negative significance moves away from zero. Both arguments are required. Significance is a step size, such as three or `0.05`, rather than a count of decimal places.

## Try it

`=CEILING(-10.3,3)` returns `-9`, while `=CEILING(-10.3,-3)` returns `-12`. The magnitude quotient is about 3.4333: selecting three or four steps gives the two directions.

`=CEILING(31.2,3)` returns `33`. A positive input with negative significance, such as `=CEILING(31.2,-3)`, gives `#NUM!` in the current route.

## Quirks

The significance sign changes the negative-input direction; a positive number with negative significance gives `#NUM!`. The current wrapper returns zero if either operand is zero, before that sign guard. Nonnumeric arguments are documented as `#VALUE!`.

Near an integer, the current function counts from a fifteen-significant-digit quotient reading. It can therefore return a multiple below the original positive input. The cited source retains `CEILING(1*(1+2^-48),1)=1`, while `CEILING(1*(1+2^-47),1)=2`. These are recorded boundary controls, separate from the ordinary signed examples above.

Do not substitute ROUNDUP: CEILING chooses multiples, and its negative-input sign rule is different. Extreme quotient and result publication cases also belong to the scalar wrapper’s numerical policy.

## How to implement

Let `D` mean one nearest/even binary64 rounding and `R` the [stored 64-bit-significand operation](../model/09-staged-arithmetic.md).

1. Coerce the two arguments in order. Return positive zero if either is zero; reject a positive number with negative significance.
2. The current native wrapper first obtains `R(number/significance)` to check the arithmetic site. A nonfinite quotient eventually gives `#NUM!`; a zero quotient keeps its zero route. For a finite nonzero quotient, the shared reading helper recomputes `q=D(number/significance)`.
3. Round q to fifteen significant decimal digits, then convert that decimal reading back to nearest binary64. The source implements this as scientific formatting with one leading digit and fourteen fractional digits, followed by parsing.
4. Take the mathematical ceiling of that stored reading and return `R(significance*count)`. Keep the count as a floating integer or a sufficiently large integer; narrowing every possible count to a signed 64-bit integer introduces an unrelated limit. Normalize zero; the current number publisher rejects nonfinite results and refuses unsupported subnormal publication.

The decimal-reading step can be coded without a formatter. Decode the exact q, find `e` with `10^e<=abs(q)<10^(e+1)`, and let `h=10^(e-14)`. Round `abs(q)/h` to the nearest integer, with an exact half choosing an even integer; restore sign, multiply by h exactly and store once to binary64. Use integer quotient/remainder comparisons as in the stored-arithmetic chapter. The Rust formatter’s nearest/even rule is documented publicly. This step acts on the binary64 quotient; ROUND’s preliminary input scaling is a different recipe.

For `number=-10.3,significance=3`, the quotient reading is about `-3.43333333333333`; its ceiling is `-3`, giving `-9`. A significance of `-3` makes the quotient positive, its ceiling four, and the product `-12`.

The reading is a measured reconstruction on retained controls, not a recovery of private source. Underflow, extreme counts and whole-domain agreement still need separately scoped evidence.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,335 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.CEILING.json).

## Alternatives

[MROUND](FUNC.MROUND.md) chooses a nearest multiple; [ROUNDUP](FUNC.ROUNDUP.md) and [ROUNDDOWN](FUNC.ROUNDDOWN.md) address decimal positions. [FLOOR.MATH](FUNC.FLOOR.MATH.md) provides the other directional family.

## Sources

- [Microsoft: CEILING function](https://support.microsoft.com/en-us/excel/functions/ceiling-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/rounding.rs); implementation steps and the limits identified in the text.
- [Quotient reading and sign helpers](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/ceiling_floor_family.rs).
- [Rust formatting precision and nearest/even rounding](https://doc.rust-lang.org/std/fmt/index.html#precision), opened 6 November 2026; semantics of the formatter used by the current quotient helper.
