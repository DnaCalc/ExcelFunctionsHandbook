# Implementing stored numerical steps

The [variance](../functions/FUNC.VAR.S.md#how-to-implement),
[small-argument ERF](../functions/FUNC.ERF.PRECISE.md#how-to-implement),
[MOD](../functions/FUNC.MOD.md#how-to-implement),
[GEOMEAN](../functions/FUNC.GEOMEAN.md#how-to-implement) and
[PMT](../functions/FUNC.PMT.md#how-to-implement) recipes use a stored operation written `R`:

```text
R(a op b) = binary64_store(round_to_64_significant_bits(exact(a op b)))
```

Both roundings choose the nearest value and break a tie toward an even final significand bit.
The first uses the extended exponent range. It is a 64-bit **significand**, not a binary64
format. The second uses binary64's 53-bit significand and its normal/subnormal limits.
Keep every assignment named in a recipe; combining two of them can change an answer.

This chapter gives one way to implement these finite arithmetic steps using large integers.
It translates the operations in the cited calculation source. It does not identify Microsoft's
private code, provide a transcendental instruction model, or establish agreement for unmeasured
Excel calls. The function guides state which stages are measured and which remain open.

## Decode the input exactly

Read the binary64 bits, rather than converting the displayed decimal text. Let `s` be the sign
bit, `E` the eleven-bit exponent field and `F` the 52-bit fraction field.

| Exponent field | Exact value |
|---|---|
| `1 <= E <= 2046` | `(-1)^s * (2^52+F) * 2^(E-1023-52)` |
| `E=0, F!=0` | `(-1)^s * F * 2^-1074` |
| `E=0, F=0` | signed zero |
| `E=2047` | infinity or NaN; handle separately |

Represent a finite value as a signed rational `A/B`, with arbitrary-size integers and `B>0`.
For addition, subtraction, multiplication and nonzero division, form the exact rational result
by integer arithmetic. Reduction by the greatest common divisor saves space but does not change
the result. This avoids losing the remainder before rounding.

## Round a rational to a binary significand

For a nonzero rational `z`, save its sign and use `a=abs(z)`:

1. Find the integer `e` for which `2^e <= a < 2^(e+1)`. Start with the difference between the
   numerator's and denominator's bit lengths, then correct by exact comparison. Do not use a
   floating-point logarithm near an exponent boundary.
2. For a `p`-bit significand, set `h=2^(e-p+1)`. Divide `a/h` as integers to obtain quotient `q`
   and remainder `r`, with denominator `d`: `a/h=q+r/d`, `0<=r<d`.
3. Increment `q` when `2*r>d`, or when `2*r==d` and `q` is odd. Otherwise keep it.
4. Return the signed exact value `q*h`. If `q=2^p`, the carry advances the exponent by one;
   normalize it if your representation requires a fixed-width significand.

For the first rounding, use `p=64`. Every nonzero exact result of one of the four basic
operations on finite binary64 operands fits comfortably inside the extended normal exponent
range, provided the divisor is nonzero. A chain that retains extended operands for later
operations still needs the extended format's own overflow/underflow handling.

For the binary64 store, round that already rounded rational again. In the normal range use
`p=53`. Below the minimum normal value, use the fixed spacing `h=2^-1074` and the same
quotient/remainder rule. A zero quotient produces zero; a carry of `2^52` reaches the minimum
normal value. A rounded normal significand whose exponent exceeds `1023` produces infinity
under nearest/even storage. The worksheet surface may then turn that into an error or refuse
it; the store and publication are separate steps.

Keep zero's sign separately if it matters to the caller. Rational zero alone cannot distinguish
`+0` from `-0`. Likewise, implement NaNs, infinities, division by zero and invalid operations as
explicit classes. For example, the variance guard's `0/0` is NaN, so its `>0.99` comparison is
false. Do not make an arbitrary-precision division exception decide that branch.

## A control that distinguishes the two roundings

These inputs are constructed arithmetic controls, rather than Excel observations:

```text
a = 1 + 2^-27 + 2^-51   bits 0x3ff0000002000002
b = 1 + 2^-26           bits 0x3ff0000004000000

exact a*b = 1 + 2^-26 + 2^-27 + 2^-51 + 2^-53 + 2^-77
```

The first rounding drops the `2^-77` term. What remains is exactly halfway between two
binary64 values, and the lower one has an even last significand bit. The second rounding
therefore chooses it. Rounding the original product directly to binary64 sees the positive
`2^-77` above that midpoint and chooses the upper value.

| Evaluation | Binary64 result | Bits |
|---|---:|---|
| Round once to binary64 | `1.0000000223517425` | `0x3ff0000006000003` |
| Round to 64 significant bits, then store | `1.0000000223517422` | `0x3ff0000006000002` |

The values above were checked with exact rational arithmetic and an x87 multiplication with
precision 64 and nearest/even rounding, followed by a binary64 store. This checks the primitive;
it is not an Excel comparison or a published Handbook vector suite.

## Square root without an extended host type

STDEV's staged root also needs two roundings. For a positive finite binary64 input `v`, let
`e=floor(floor(log2(v))/2)`, finding the inner exponent from bits or exact comparisons.
This is the exponent of `sqrt(v)`; division of a negative exponent must round downward.
Set `h=2^(e-63)`.

1. Find `q=floor(sqrt(v/h^2))`. An integer square root of `floor(v/h^2)` gives this integer.
2. Compare `4*v` with `(2*q+1)^2*h^2`, using exact integer/rational arithmetic. This compares
   the root with the midpoint between `q*h` and `(q+1)*h` without approximating the root.
3. Increment `q` when the left side is larger, or when they are equal and `q` is odd.
4. Store the exact value `q*h` to binary64 with the storage rule above.

For `v=2`, the rounded 64-bit significand is `0xb504f333f9de6484` and `h=2^-63`.
The binary64 store is `1.4142135623730951`, bits `0x3ff6a09e667f3bcd`. This example checks
the mechanics; the STDEV notes carry the recorded controls that distinguish the staged root
from a single binary64 square root.

## Where a store belongs

In ERF's Horner loop, `R(R(v*t)+c)` has a store after the multiply and another after the add.
A fused multiply-add has one rounding history and is a different candidate. In contrast,
ROUND's preliminary decimal reading rounds its scale operation to a 64-bit significand and
then makes an integer decision **before** any binary64 store. Reusing `R` there inserts an
extra stage. PMT's logarithm series uses ordinary binary64 steps, which also differ from `R`.

The public LN and EXP chains retain extended intermediates and use approximate transcendental
instructions. The rational arithmetic procedure above implements their basic arithmetic,
but does not supply `FYL2X` or `F2XM1`. See the [LN](../functions/FUNC.LN.md#how-to-implement)
and [EXP](../functions/FUNC.EXP.md#how-to-implement) guides for that remaining dependency.

## Sources

- [DnaCalc software x87 format, rounding and stores](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/excel_numeric/soft87.rs), especially `ext_from_f64`, `round_shift`, `round_pack` and `ext_to_f64`.
- [Stored arithmetic and staged square root](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/excel_numeric/mod.rs), `excel_x87_add`, `excel_x87_sub`, `excel_x87_mul`, `excel_x87_div` and `excel_sqrt_staged`.
- [Variance's assignment order and comparisons](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/variance_common.rs).
- [Recorded variance and root controls](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/tests/f1_stats.rs), `variance_uses_raw_moments_and_stores_pc64_steps` and `stdev_stores_the_extended_square_root`.
- [Rounding's preliminary reading and publication rules](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/rounding.rs).
