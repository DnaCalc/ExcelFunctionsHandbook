---
schema: efh.function-page/v1
function_id: FUNC.ISODD
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - "What it does"
  - "Try it"
  - "Quirks"
  - "How to implement"
  - "Version differences"
  - "Alternatives"
  - "Sources"
family: is_predicates_family
role_in_family: The odd one out in two senses — the only member of the module that coerces its
  argument to a number instead of classifying its kind.
---

# ISODD

## What it does

`ISODD(number)` tests whether a number is odd, returning a logical value. Microsoft's
ordinary rule is to discard a fractional part before testing parity: `ISODD(2.5)` is FALSE.
Sign does not change parity; truncating a negative number moves toward zero.

Excel has an additional near-integer tolerance. On the recorded build,
`ISODD(2.99999999999)` is TRUE: the value counts as 3 even though truncating its exact stored
value alone would give 2. This is a numerical quirk, not the ordinary mathematical parity test.

## Try it

Compare `=ISODD(2.5)`, `=ISODD(2.99999999999)` and `=ISODD(-2.99999999999)`. Use
`=ISEVEN(2.99999999999)` to compare the paired predicate.

## Quirks

The documented description of truncation omits the tolerance visible near an integer. Recorded
ISODD calls distinguish 2.99999999991 (TRUE) from 2.99999999989 (FALSE). Those points constrain a
boundary; they do not establish one universal tolerance on every build or at every magnitude.

The pair also differs from strict classifiers such as ISNUMBER. The recorded ISEVEN accepts
numeric text `"2"` and `" 2 "`, while ISEVEN(TRUE) returns `#VALUE!`. An error input is propagated
rather than classified as FALSE. Do not apply a blanket rule that every IS-prefixed function
performs no conversion.

At magnitudes of 2^53 and above, all finite binary64 numbers are even integers. Converting a
large input to a signed machine integer can saturate or overflow and create a false odd answer.

## How to implement

Keep argument reading separate from the parity kernel: read a one-cell reference, handle blanks,
coerce permitted numeric text, reject logical values where the measured rule requires it, and
propagate errors. References spanning multiple cells need their own measured shape rule.

A compatibility candidate used by OneCalc is to add binary64 `1e-10` to the input's absolute
magnitude, round that addition to binary64, truncate the result, and take the remainder modulo 2.
The predicate returns whether that remainder is nonzero.
The rounded addition is part of the candidate; replacing it with a test on the original value
changes results near an integer.

Test both sides of each observed transition and both signs, direct versus referenced text,
logical arguments and magnitudes beyond signed-integer range. Keep this compatibility algorithm
separate from a mathematical integer-parity routine.

## Version differences

The examples below name Excel 16.0.20527.20088 on Windows. They show specific tolerance and
coercion behaviour on that build. Other builds and platforms need corresponding observations;
Microsoft's general truncation description does not settle those boundaries.

## Alternatives

[ISEVEN](FUNC.ISEVEN.md) tests the other parity. MOD and an explicitly specified
truncation rule can express mathematical parity, but they need not reproduce the tolerance.
[ISNUMBER](FUNC.ISNUMBER.md) classifies a value without performing a parity calculation.

## Sources

- [Microsoft: ISODD function](https://support.microsoft.com/en-us/excel/functions/isodd-function).
- [Microsoft: IS functions](https://support.microsoft.com/en-us/excel/functions/is-functions).
