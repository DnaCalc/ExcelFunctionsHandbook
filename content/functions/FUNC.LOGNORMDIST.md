---
schema: efh.function-page/v1
function_id: FUNC.LOGNORMDIST
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

# LOGNORMDIST

## What it does

`LOGNORMDIST(x,mean,standard_dev)` returns the probability that a positive lognormal variable is no greater than `x`. Its mean and standard deviation arguments describe **ln(X)**, not X. This legacy function always returns a cumulative probability; it has no density switch.

## Try it

`=LOGNORMDIST(1,0,1)` returns `0.5`: ln(1) is the centre of the specified normal distribution. With the same positive spread, values below one have probabilities below one half and values above one have probabilities above one half.

`=LOGNORMDIST(EXP(1),1,0.5)` is approximately `0.5`. Mathematically, the median is `exp(mean)`; in a worksheet composition, the stored EXP result and the later logarithm can introduce their own rounding.

## Quirks

Nonpositive `x` or nonpositive standard deviation gives `#NUM!`; nonnumeric arguments give `#VALUE!`. A negative log-space mean is allowed. All three arguments are required, and adding a fourth cumulative flag does not turn this spelling into LOGNORM.DIST.

The two tails are asymmetric in the original units even though ln(X) is normal. A CDF near one can round to one, so subtracting it from one may lose a small upper-tail probability. The standardization must use the log-space parameters, not statistics calculated directly from untransformed observations.

## How to implement

The current shared kernel performs these steps:

1. Validate positive finite `x` and positive finite `sigma`.
2. Obtain the internal CRT logarithm `L=crt_log(x)`.
3. Store the ordinary binary64 subtraction and division `z=(L-mean)/sigma`.
4. Evaluate the shared standard-normal CDF at `z`.

At `x=1,mean=0`, the logarithm and standardized value are zero and the CDF is one half. The standard-normal CDF has its own stored argument transformation and complementary-error dependency; its upper ERFC body remains open.

The code retains a 45-control public-composition observation on the modern spelling, but a later larger bank distinguishes the internal CRT log from worksheet LN at a named control. Thus `NORM.DIST(LN(x),mean,sigma,TRUE)` is a useful mathematical composition, not permission to replace the internal log on every input. This wrapper is shared by the current legacy implementation; the modern observation alone does not measure every legacy call.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 3,299 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.LOGNORMDIST.json).

## Alternatives

[LOGNORM.DIST](FUNC.LOGNORM.DIST.md) is the modern spelling. [NORM.DIST](FUNC.NORM.DIST.md) describes the normal variable in log space.

## Sources

- [Microsoft: LOGNORMDIST function](https://support.microsoft.com/en-us/excel/functions/lognormdist-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/normal_log_family.rs); implementation steps and the limits identified in the text.
