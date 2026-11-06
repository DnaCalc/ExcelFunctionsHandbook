---
schema: efh.function-page/v1
function_id: FUNC.INT
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
  - Excel versions, platforms and CPUs
  - Sources
family: int_fn
role_in_family: >-
  Sole member of its module; the floor function, and the one rounding primitive in the math
  category that rounds toward negative infinity rather than toward zero.
---

# INT

## What it does

`INT(number)` rounds down to an integer, toward negative infinity. This makes negative fractions more negative: it is a different direction from TRUNC. The recorded compatibility route also has a preliminary decimal reading near integer boundaries.

## Try it

`=INT(-6.8)` returns `-7`, while `=TRUNC(-6.8)` returns `-6`. `=INT(6.8)` returns `6`. For an ordinary positive amount, subtracting its INT result extracts the fractional part.

A boundary control from the source is different from mathematical floor of the incoming binary value: an exact cell value `2.9999999999999982` gives `3` on the recorded build. Entering a decimal formula literal can change the value before INT sees it; retain the exact cell input when reproducing that control.

## Quirks

The fifteen-digit reading can cross an integer. Calling a language's floor on the incoming double therefore misses recorded answers. The current implementation applies this reading only below magnitude `1e15`; larger values use binary64 floor directly. For example, the recorded `4503599627370495.5` floors to `4503599627370495`, while its fifteen-digit decimal reading would have crossed upward.

INT has no digit-count argument. The direction for ordinary negative fractions is toward negative infinity; ROUNDDOWN and TRUNC use toward zero. Numeric conversion, errors and array handling belong to the worksheet wrapper, separately from this finite scalar recipe.

## How to implement

The current code separates decimal reading from binary floor:

1. If `abs(x)>=1e15`, floor the exact binary64 value. A value already integral is unchanged; for a negative value with a fractional part, the result is one less than its toward-zero truncation.
2. Below that threshold, obtain the shortest decimal that converts back to the input binary64. If it has at most fifteen significant digits, use that exact decimal; otherwise decode the exact binary64 as a rational or exact decimal.
3. Round that reading to fifteen significant decimal digits, choosing the larger magnitude at an exact decimal half. When this did not change the reading, the current code falls back to floor of the incoming binary64.
4. When the reading changed, truncate its decimal fraction toward zero and convert the whole value to binary64. Also convert the full reading. If the full reading is below the truncated whole value, subtract one; otherwise retain the whole value.

For `2.9999999999999982`, the shortened fifteen-digit reading is `3`, so the answer is three. For `-6.8`, the integral truncation is minus six and the number lies below it, so floor gives minus seven. Do not reuse ROUND's preliminary x87 scale operation: INT's current decimal helper is a separate route.

The calculation source reports 109 COM controls on build 20527, including both sides of small integers and the large-value exception. Those controls support this implementation on their stated inputs; they do not identify every conversion or workbook route. The [stored-arithmetic chapter](../model/09-staged-arithmetic.md) explains exact input decoding when a host decimal representation is unsuitable.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 40 |
| 20430 | Windows, 64-bit, CPU not recorded | 124,403 |

- XC-INT-001: build 19929 versus build 20430; 2 same-input pairs, 0 differing answers. Compared axes: excel-build. Numeric answers were compared by their recorded binary64 bits; other answers by typed value.

Agreement on these compared inputs does not establish agreement on unmeasured inputs.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.INT.json).


The current source also retains a later build-20527 COM group of 109 INT readings from 30 October 2026. It records `INT(2.9999999999999982)=3` and the large-input exception `INT(4503599627370495.5)=4503599627370495`. These are additional controls, not a matched-input comparison with the earlier environments.

## Alternatives

[TRUNC](FUNC.TRUNC.md) removes a fraction toward zero. [FLOOR.MATH](FUNC.FLOOR.MATH.md) chooses a multiple with explicit negative-number mode, and [ROUND](FUNC.ROUND.md) chooses a nearest decimal position. [MOD](FUNC.MOD.md) has a documented floor identity but a separately recovered numerical route.

## Excel versions, platforms and CPUs

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.INT.json; edits here are overwritten -->

Readings of INT with a recorded Excel build: 124,443 input(s) over 2 recorded environment(s),
Excel 16.0 build(s) 19929, 20430, all on Windows; none of these captures records the CPU. 1
same-input comparison(s) between environments over 2 input-pair(s): no result differs. No
reading from a Mac, Excel for the web, 32-bit Excel or an Intel CPU is recorded.

Readings of `INT` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 40 | date not recorded |
| 16.0 build 20430 | Windows, 64-bit | CPU not recorded | 124,403 | 2026-09-29 |

Compared on the same inputs:

| Check | Differs in | Environments | Inputs compared | Inputs with different results |
|---|---|---|---:|---:|
| XC-INT-001 | excel-build | 16.0 build 19929 vs 16.0 build 20430 | 2 | 0 |

Differences observed: none.

Evidence for every row: [`content/evidence/environments/FUNC.INT.json`](../../content/evidence/environments/FUNC.INT.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Sources

- [Microsoft: INT function](https://support.microsoft.com/en-us/excel/functions/int-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/rounding.rs); implementation steps and the limits identified in the text.
- [Exact decimal reading and integer parts](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/decimal.rs).
- [Recorded acceptance control](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/tests/acceptance.rs), INT at cell A5 in the rounding tests.
