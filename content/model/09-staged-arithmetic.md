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

## Decimal-family packing exceptions

The ROUND family has measured output encodings that are separate from the ordinary binary64
store above. Carry a tagged bit pattern if your value model must represent them; treating every
such pattern as an ordinary host floating-point number loses Excel's worksheet behavior.

For an exact decimal result whose nearest binary64 conversion overflows, the current ROUND,
ROUNDDOWN and TRUNC model converts **half** the decimal to nearest binary64. If that half is
finite, add `1<<52` to its unsigned 64-bit encoding. This advances the encoded exponent without
a range check. The result can have an all-ones exponent field and a NaN fraction. If the half
also overflows, return `#NUM!`. ROUNDUP returns `#NUM!` for the overflow result instead of
publishing that tagged number.

For example, the exact decimal `2e308` gives `0x7ff1ccf385ebc8a0` by this rule. The recorded
rounding output is held as a worksheet number although an ordinary binary64 consumer calls
those bits NaN. The note records its display, predicates, subsequent arithmetic errors and
save/reopen behavior. This is an output model over recorded cases, not a claim to have seen an
unchecked exponent write in Microsoft's code.

The directed functions have an early underflow return. After forming the fifteen-digit reading,
and **before** rounding at the requested decimal place, check a normal input with magnitude
below twice the minimum normal value. If its changed reading converts to a subnormal, convert
twice the exact reading instead. Subtract `1<<52` from that normal value's encoding and return
the resulting bits immediately. No digit rounding follows that return. Other directed-path
final subnormals still flush to zero; ROUND has its own zero-flush rule.

At input `0x0010000000000000`, the fallback decimal reading is `2.22507385850720e-308`.
The early packing gives `0x000ffffffffffffa`. FN-TRUNC-001 records that result for TRUNC,
ROUNDDOWN and ROUNDUP at every sampled digit count, including zero. Converting the reading as
an ordinary subnormal would give `0x000ffffffffffffd`, and rounding at zero places would give
zero. Those are distinct operations.

The reading's large-power scaling remains open, as the function guides explain. Reproducing
these observed output encodings does not settle that earlier stage.

## Retain an extended intermediate when the recipe requires it

A stored operation R and a resident extended operation E have different meanings. Write
`E(a op b)=round_to_64_significant_bits(exact(a op b))`, retaining its extended exponent and
64-bit significand without an intervening binary64 store. Write D for ordinary binary64
storage. Thus R is `D(E(...))`, but an extended chain can feed E’s result directly into its
next E operation.

Use the same integer quotient/remainder rounding construction with p=64 for each E operation.
Keep that rounded value as an exact rational until an explicit D store. Loading a binary64
constant into the extended representation is exact; a coefficient written as a fraction in a
source table can nevertheless require an initial D rounding before that load. The recipe must
say which it means. Apply extended overflow and underflow rules when the chain can reach them.

The [POISSON mass helpers](../functions/FUNC.POISSON.DIST.md#how-to-implement) retain Horner
intermediates in E for one Stirling series, but store other products and sums with R. Its
compensated exponential retains the part of an extended subtraction lost by D. Using R at
every line would discard that correction. The operations describe the current public source;
they do not turn a mathematical logarithm or exponential into an x87 instruction model.

## Sources

- [DnaCalc software x87 format, rounding and stores](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/excel_numeric/soft87.rs), especially `ext_from_f64`, `round_shift`, `round_pack` and `ext_to_f64`.
- [Stored arithmetic and staged square root](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/excel_numeric/mod.rs), `excel_x87_add`, `excel_x87_sub`, `excel_x87_mul`, `excel_x87_div` and `excel_sqrt_staged`.
- [Variance's assignment order and comparisons](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/variance_common.rs).
- [Recorded variance and root controls](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/tests/f1_stats.rs), `variance_uses_raw_moments_and_stores_pc64_steps` and `stdev_stores_the_extended_square_root`.
- [Rounding's preliminary reading and publication rules](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/rounding.rs).
- [FN-ROUND-003](../notes/FN-ROUND-003.json) and [FN-TRUNC-001](../notes/FN-TRUNC-001.json): recorded overflow and initial-reading encodings, with their worksheet scope.
- [R31 resident series and compensated exponentials](https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/crates/calc-func/src/ported/excel_numeric/x87_chains.rs), `stirling_series`, `deviance_series`, `exp_compensated` and `exp_running`.
