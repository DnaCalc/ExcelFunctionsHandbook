---
schema: efh.function-page/v1
function_id: FUNC.RANDARRAY
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — RANDARRAY function"
    locator: "https://support.microsoft.com/en-us/office/randarray-function-21261e55-3bec-4885-86a6-8b0a47fd4d33"
    role: "the only documentation page for this surface; retrieval refused during this pass, so it is cited as a locator and not quoted"
  - work: "Microsoft — Excel JavaScript API, Excel.Functions"
    locator: "https://learn.microsoft.com/en-us/javascript/api/excel/excel.functions"
    role: "checked for a randArray entry; the API surface documented there does not include one"
  - work: "L'Ecuyer & Simard, TestU01: A C library for empirical testing of random number generators"
    locator: "ACM TOMS 33(4), 2007"
    role: "the standard battery for the stream this surface draws from"
  - work: "Lemire, Fast random integer generation in an interval"
    locator: "ACM TOMEACS 4(1), 2019"
    role: "the bias analysis for the whole-number mode's floor(u*count) construction"
episodes: []
body_sections:
  - "What it does"
  - "Try it"
  - "Quirks"
  - "How to implement"
  - "Version differences"
  - "Alternatives"
  - "Sources"
family: misc_conversion_family
role_in_family: "The odd one out: a volatile array generator co-located in a module of deterministic conversion surfaces, sharing only the module and not the semantics."
---

# RANDARRAY

## What it does

`RANDARRAY([rows], [columns], [min], [max], [whole_number])` returns an array of random values.
The defaults are one row, one column, minimum 0, maximum 1 and decimal results. Set
`whole_number` to TRUE to request integers.

`=RANDARRAY(2,3)` requests two rows and three columns. `=RANDARRAY(2,3,1,6,TRUE)` requests a grid
of whole numbers in the stated range. The array changes when Excel recalculates it; copy and
paste values if a particular sample must be retained.

## Try it

The example is editable, but OneCalc does not supply a random stream in this preview. Calculate
reports that limit explicitly. It does not substitute a fixed sample or a different generator.

## Quirks

An array needs enough empty worksheet cells to spill. A blocked spill and invalid function
arguments are separate problems. A formula used as a random sort key can change on recalculation,
so do not treat a recalculated random array as a persistent identifier.

Microsoft says the minimum must be less than the maximum and documents `#VALUE!` otherwise.
The OneCalc kernel accepts equal endpoints and returns that value. There is no measured Excel
comparison here resolving the equal-endpoint case; the two accounts must remain distinct.

The continuous distribution, integer endpoint handling and treatment of non-integral dimensions
need separate checks. A real-number interval formula does not by itself prove which binary64
endpoints are attainable. Drawing successive values from a pseudorandom stream also does not
prove statistical independence.

## How to implement

Validate dimensions before allocation and check their product for overflow. Make the source of
randomness an explicit service so that recalculation, stream ownership and replay are under the
caller's control. Fill one two-dimensional result using a declared traversal order.

For a chosen continuous sampler, transforming a unit draw with `min + u*(max-min)` is a common
starting point, but the arithmetic can overflow or round to an endpoint. For uniform bounded
integers, rejection sampling avoids the bias that a naive modulo or floor construction can
introduce when the source range is not a multiple of the target range.

Those are choices for a new sampler. Reproducing Excel additionally requires observations of
its generator, seed or stream state, integer mapping and draw order. Matching the requested
range and array shape does not reproduce Excel's random sequence. Keep missing randomness a
refusal rather than silently returning cached or deterministic values.

## Version differences

Microsoft lists RANDARRAY for Microsoft 365 and Excel 2021/2024, including the listed mobile
editions. No comparison of generated streams or endpoint rules between named builds is
recorded here. Product availability is separate from the random algorithm used by a build.

## Alternatives

[RAND](FUNC.RAND.md) returns one unit-interval value. [RANDBETWEEN](FUNC.RANDBETWEEN.md) requests
one bounded integer. [SEQUENCE](FUNC.SEQUENCE.md) creates a deterministic grid. A sampler with an
explicit seed is more suitable for reproducible experiments, but its stream need not be Excel's.

## Sources

- [Microsoft: RANDARRAY function](https://support.microsoft.com/en-us/excel/functions/randarray-function).
- Lemire, *Fast random integer generation in an interval* (2019), for bounded-integer bias.
- L'Ecuyer and Simard, *TestU01* (2007), for testing random streams.
