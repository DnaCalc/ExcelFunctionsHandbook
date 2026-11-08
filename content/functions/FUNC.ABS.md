---
schema: efh.function-page/v1
function_id: FUNC.ABS
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — ABS function"
    locator: "https://support.microsoft.com/en-us/office/abs-function-3420200f-5628-4e8c-99da-c99d7c87713c"
    role: "the canonical article; not retrieved for this pass (fetch returned HTTP 403)"
  - work: "IEEE Std 754-2019, Standard for Floating-Point Arithmetic"
    locator: "clause 5.5.1, sign-bit operations"
    role: "the statement that absolute value is a sign-bit operation, exact and never signalling"
  - work: "Higham, Accuracy and Stability of Numerical Algorithms"
    locator: "chapter 2, Sterbenz's lemma"
    role: "the band in which a floating-point subtraction is exact, which is what governs the ABS(a-b) idiom"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Notes
  - Excel versions, platforms and CPUs
  - Page vocabulary
  - Sources
family: abs
role_in_family: >-
  The magnitude projection: the one arithmetic surface in the math category that is exact by
  construction, because it changes a sign bit and nothing else.
---

## What it computes

`ABS(number)` is the absolute value — the distance of *number* from zero on the real line:

    |x|  =   x   if x >= 0
            -x   if x <  0

- **Domain**: all real numbers.
- **Range**: the non-negative reals, `[0, +infinity)`.
- **Parity**: even. `|-x| = |x|`, and this is the defining symmetry.
- **Smoothness**: continuous everywhere, differentiable everywhere *except* at zero, where the
  left and right derivatives are `-1` and `+1`. The corner at the origin is the whole reason
  `ABS` shows up in optimisation problems as the awkward term.
- **Algebra**: `|xy| = |x||y|`, `|x/y| = |x|/|y|`, and the triangle inequality
  `|x + y| <= |x| + |y|` with equality exactly when `x` and `y` have the same sign. Absolute
  value is the standard norm on `R`; that is what makes `ABS(a - b)` the natural distance.

The identity that ties it to its sibling is `x = SIGN(x) * ABS(x)`, exact for every `x` except
that `SIGN(0)` is `0` and so contributes nothing at the origin.

In binary64 terms `ABS` is not an approximation of anything. The magnitude of a representable
number is representable, so the correctly rounded result *is* the exact result: `ABS` clears the
sign bit and leaves every other bit alone. Under IEEE 754 that makes it one of a very small set
of operations — sign-bit operations — that are exact, cannot overflow, cannot underflow, and
raise no exception on any input.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The real number whose magnitude you want. Required. | — |

One argument; the reference engine records an arity of exactly one, and classifies the surface as
a unary numeric scalar-or-array kernel, so an array in the single slot lifts elementwise. Ordinary
numeric slot under the shared coercion rules — see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

There is no commonly misunderstood position here, because there is only one. The misunderstanding
that does occur is upstream of the call: readers reach for `ABS` to make a *result* positive when
what they wanted was to make an *argument* positive, and the two differ whenever anything
non-monotonic sits in between.

## Result and edge cases

Returns `Number`.

- **Zero.** `|0| = 0`. The interesting sub-case is negative zero: IEEE 754 distinguishes `-0`
  from `+0`, and absolute value maps `-0` to `+0`. Writing `-0` (`0x8000000000000000`) to a
  worksheet cell via `Range.Value2` stores `+0` (`0x0000000000000000`) in Excel 16.0 build
  20527 ([sweep-questions-1-ANSWERS.md](../notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md),
  `C:\Work\DnaCalc\codex-runs\sweep-questions-1\Q1-subnormal-inputs.json`).
- **Subnormals.** Writing any positive or negative subnormal binary64 value (`±5e-324` through
  `±(2^-1022 - 5e-324)`) to a cell via `Range.Value2` flushes the stored cell value to `+0`,
  and `*`, `/`, and probed `^` underflows in the subnormal range also flush to `+0`, whereas `+`,
  `-`, and `EXP(-745)` (`0x0000000000000001`) keep true subnormals in formula cells
  ([FN-ABS-002](../notes/FN-ABS-002.json), `Q1-subnormal-inputs.json`). When `ABS` receives a
  true computed subnormal `±x` from `EXP(-745)` (`0x0000000000000001`), `EXP(-730)`
  (`0x00000000001c7ea3`), or `EXP(-709)` (`0x0008bfe58468888d`) (with `0 - x` for the negative
  input), it returns `x` unchanged.
- **The largest finite double.** `±1.7976931348623157e+308` (`0x7fefffffffffffff` /
  `0xffefffffffffffff`) reads back intact from cells and passes through `ABS` as
  `0x7fefffffffffffff` (`Q1-subnormal-inputs.json`).
- **Logicals.** Convert by the shared rule (`TRUE` to 1, `FALSE` to 0) before the kernel sees
  them.
- **Text that reads as a number.** Converts as a direct argument under the shared to-number rule;
  text in a *scanned range* is a different question and does not arise for a unary kernel. A zero
  mantissa with an out-of-range exponent, such as `"0e-309"`, does not convert: `#VALUE!`
  (FN-ABS-001).
- **Arrays.** Lift elementwise. The implementing module carries an open upstream defect stream on
  exactly this point (`BUG-FUNC-022`, named in the presence projection as an array-lift gap on the
  unary `ABS` surface), so array-shaped arguments are the unsettled part of this surface, not the
  arithmetic.

The reference engine's projected `real_result_policy` for `ABS` records `non_finite=allow` — the
kernel is permitted to hand back a non-finite value. For `ABS` specifically this is inert, since
no finite input can produce one; it matters on the pages of functions that can.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | The argument does not convert to a number | Shared coercion rules, not an `ABS`-specific condition |
| propagated | An error value in the argument is returned unchanged | Shared coercion rules |

There is no domain error and no overflow error, because `ABS` has neither a restricted domain nor
an arithmetic step. Microsoft's article for `ABS` was not retrieved for this pass, so no error
condition on this page is stated on its authority.

## Relationships

- **`SIGN`** — the other half of the polar decomposition of a real number. `SIGN(x) * ABS(x) = x`
  away from the origin.
- **`IMABS`** — the complex modulus, which is the same idea one dimension up and *not* the same
  computation: it is `hypot(a, b)`, an operation with real overflow and cancellation questions,
  where the real `ABS` has none.
- **Unary minus** — `ABS` is not "negate if negative" implemented as a comparison plus a
  negation; it is a single sign-bit clear. The distinction is invisible in results and visible in
  performance.
- **`SUMSQ`, `SUMXMY2`, `DEVSQ`** — squaring is the other standard way to discard a sign, and it
  is the one that overflows. Where a formula can use either, `ABS` has the wider range.
- **`AVEDEV`** — the mean absolute deviation, which is `ABS` under an aggregate, and the reason
  the function exists in the statistical category at all.
- **Confused with**: `INT` and `TRUNC`, which also "simplify" a number but change its magnitude;
  `ABS` never does.

## Numerical notes

`ABS` is the rare function whose numerical note is that it has no numerical error. Everything
interesting happens on either side of it.

**The comparison idiom.** `ABS(a - b) < tol` is the standard floating-point equality test, and the
error in it belongs entirely to the subtraction. Sterbenz's lemma gives the good case: if `a` and
`b` are within a factor of two of each other, `a - b` is computed exactly, so `ABS(a - b)` is the
exact distance. Outside that band the subtraction rounds, and for widely separated magnitudes it
can round to something with no correct digits at all. `ABS` faithfully reports whatever the
subtraction produced; readers who see a surprising result from this idiom are looking at the wrong
function.

**Absolute versus relative tolerance.** A fixed `tol` in the idiom above is an *absolute*
tolerance, and it is meaningful only over a narrow range of magnitudes. The relative form
`ABS(a - b) <= tol * ABS(b)` scales correctly but fails when `b` is zero. The mixed form
`ABS(a - b) <= tol * MAX(ABS(a), ABS(b), 1)` is the usual compromise, and it is worth writing out
rather than reaching for, because the choice is a modelling decision and not a numerical one.

**Why implementations should not branch.** The obvious implementation — compare against zero,
negate if negative — is correct but has two defects: it is a branch on data, and it maps `-0` to
`-0` rather than to `+0` unless the comparison is written carefully. Clearing the sign bit has
neither problem. Any implementation claiming `ABS` should be checked at `-0` for exactly this
reason.

**`ABS` as a guard.** A common pattern wraps a domain-restricted function in `ABS` to keep its
argument admissible — `SQRT(ABS(x))`, `LN(ABS(x))`. This silently changes the function being
computed. The Handbook notes it because it turns a `#NUM!` into a plausible wrong answer, which is
the worse of the two failures.

**Subnormal and cell-input boundary in DNA Calc.** In `crates/calc-func/src/catalogue/math.rs`
(commit `e042619e35426ffac3d8cfaaa2dd2961abe3d02c`) and `crates/calc-func/src/witness_exec.rs`
(commit `c0b94f88ab4e3df5b518329d08ea03cf481abdbe`), `ABS` clears the sign bit (`x.abs()`),
preserving true computed subnormals such as `ABS(0 - EXP(-745)) = 5e-324` (`0x0000000000000001`),
while cell-input replay flushes subnormal `Range.Value2` cell writes to `+0`. DNA Calc matches
all 1,227 of 1,227 recorded `ABS` bank rows (`C:\Work\DnaCalc\codex-runs\sweep-report-2.md`).

## What has not been checked

**No evidence record in the Handbook names `FUNC.ABS`**, and no Handbook vector suite exists for
it. Beyond the text-to-number readings of FN-ABS-001, nobody has checked this function against
Excel within the Handbook's record. The battery rendered beside this page is the reference engine's
own answers with no Excel involved, and it is
labelled as such.

Microsoft's `ABS` support article was not retrieved for this pass — the fetch returned HTTP 403 —
so no statement on this page rests on it. The mathematics above is stated from the standard
literature; the classification facts are read from the Handbook's own projections.

Inputs worth probing first, in order of how much each would settle:

1. **`ABS(-0)` and its round trip** — enter a negative zero (for example as `-1 * 0` or via a
   formula that underflows to it) and test whether the result is distinguishable from `+0` by
   `1/ABS(x)`. This is the single probe that tells you whether Excel's value model preserves the
   IEEE sign of zero at all, and the answer is load-bearing for several other pages.
2. **Array arguments in the single slot**, given the open `BUG-FUNC-022` array-lift stream on this
   module. A `2x2` inline array and a range reference are different paths and both need a probe.
3. **The largest finite double and the smallest subnormal**, to confirm that `ABS` is a pure
   pass-through at both ends rather than routing through any arithmetic.
4. **`ABS("2.5")` versus `ABS(A1)` with `"2.5"` in `A1`** — the direct-argument versus range-scan
   asymmetry described in the coercion chapter. `ABS` is a clean instrument for it, precisely
   because the arithmetic cannot confound the result.
5. **`ABS(TRUE)` and `ABS("")`** — the logical and empty-text conversions, which are shared-model
   behaviour but have never been observed in Excel for this surface.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `ABS` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-ABS-001: Text to number checks the exponent range even when the mantissa is zero

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.* Applies to ABS, VALUE.

Converting text to a number, measured through ABS and VALUE, rejects a zero mantissa with an
out-of-range exponent: ABS("0e-309") is #VALUE! though the text denotes 0, and ABS("0e-308") is
0. The exponent counts as written, less the fraction digits, less 2 for a percent sign, so
ABS("0.0e309") is 0, ABS("0e309") is #VALUE!, and ABS("0e-307%") is #VALUE!.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `=ABS("0e-309")` | as typed | `#VALUE!` | `0` (`0x0000000000000000`) | Excel build not recorded in this run; bulk captures of the same date (2026-09-29) record Excel 16.0 build 20430, Windows 64-bit, workbook compatibility 2 |
| `=ABS("0e-308")` | as typed | `0` (`0x0000000000000000`) | — | Excel build not recorded in this run; bulk captures of the same date (2026-09-29) record Excel 16.0 build 20430, Windows 64-bit, workbook compatibility 2 |
| `=ABS("0e-307%")` | as typed | `#VALUE!` | — | Excel build not recorded in this run; bulk captures of the same date (2026-09-29) record Excel 16.0 build 20430, Windows 64-bit, workbook compatibility 2 |
| `=ABS("0e309")` | as typed | `#VALUE!` | `0` (`0x0000000000000000`) | Excel build not recorded in this run; bulk captures of the same date (2026-09-29) record Excel 16.0 build 20430, Windows 64-bit, workbook compatibility 2 |
| `=ABS("0.0e309")` | as typed | `0` (`0x0000000000000000`) | — | Excel build not recorded in this run; bulk captures of the same date (2026-09-29) record Excel 16.0 build 20430, Windows 64-bit, workbook compatibility 2 |
| `=VALUE("0e-307")` | as typed | `0` (`0x0000000000000000`) | — | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("0e-307%")` | as typed | `#VALUE!` | — | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("0e309")` | as typed | `#VALUE!` | — | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("0.0e309")` | as typed | `0` (`0x0000000000000000`) | — | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("0e309%")` | as typed | `0` (`0x0000000000000000`) | — | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("0e311%")` | as typed | `#VALUE!` | — | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Correct value: The text denotes zero: a zero mantissa is 0 at any exponent.

Basis (measured): About 120 recorded zero-mantissa cases through ABS, and 72 through VALUE. All
72 VALUE readings fit a limit of -308 to 308 on the exponent as counted here.

Inferred, not measured: The exponent formula is inferred, though it fits every reading; that
functions other than ABS and VALUE share this conversion is inferred.

Evidence:

- DnaCalc `apps/dna-onecalc/tests/acceptance.rs`, recorded_zero_percentage_exponents_are_validated at `344c166787` — recorded readings pinned
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheets qt097 to qt168 at `2d0eddcac8` — 72 recorded VALUE readings
- CalcFunc-Evidence `runs/w111-numeric-text-percent-bounds-20260929/outcomes/excel.jsonl.gz`, w111textbounds-00008-abs-boundary-2-0 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `runs/w111-numeric-text-percent-bounds-20260929/outcomes/excel.jsonl.gz`, w111textbounds-00012-abs-boundary-3-0 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `runs/w111-numeric-text-percent-bounds-20260929/outcomes/excel.jsonl.gz`, w111textbounds-00017-abs-boundary-4-1 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `runs/w111-numeric-text-percent-bounds-20260929/outcomes/excel.jsonl.gz`, w111textbounds-00028-abs-boundary-7-0 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `runs/w111-numeric-text-percent-bounds-20260929/outcomes/excel.jsonl.gz`, w111textbounds-00108-abs-boundary-27-0 at `4d5320dfdc` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt099, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt100, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt107, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt143, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt108, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt112, row 20, column 10 at `2d0eddcac8` (readings)

### FN-ABS-002: Subnormal cell writes and *, /, ^ underflows flush to +0, while +, - and EXP(-745) keep subnormals

*Quirk; measured; recorded 2026-11-08.* Applies to ABS, CEILING, CEILING.MATH, CEILING.PRECISE, DELTA, EVEN, EXP, FLOOR, FLOOR.MATH, FLOOR.PRECISE, GESTEP, INT, ISEVEN, ISO.CEILING, ISODD, ODD, QUOTIENT, SIGN, SQRT.

Writing any positive or negative subnormal binary64 value or -0 to a cell via Range.Value2
stores +0 (0x0000000000000000), while normal extremes 2^-1022 (0x0010000000000000) and ±DBL_MAX
(0x7fefffffffffffff) read back intact. In the 430 recorded bank rows with subnormal or
maximum-finite cell arguments, all 179 subnormal-input rows read back as +0 and match the +0
control (SQRT(-5e-324) is 0, SIGN(±5e-324) is 0, ODD(±5e-324) is 1, EVEN(±5e-324) is 0,
ISEVEN(±5e-324) is TRUE, ISODD(±5e-324) is FALSE, GESTEP(±5e-324) is 1, DELTA(±5e-324) is 1,
and a subnormal divisor or significance in QUOTIENT, CEILING or FLOOR flushes to 0). Formula
literals below 2.2250738585072014E-308 are refused at entry or parsed to 0. Products (*),
quotients (/) and probed powers (^) that fall in the subnormal range flush to +0, whereas + and
- retain subnormals and EXP(-745) holds 0x0000000000000001 in a formula cell. When fed a true
computed subnormal x from EXP(-745), EXP(-730) or EXP(-709) (and 0-x for negative), ABS(±x) is
x, SQRT(+x) is 2^-537, SQRT(-x) is #NUM!, EVEN(±x) is ±2, ODD(±x) is ±1, ISEVEN(±x) is TRUE,
ISODD(±x) is FALSE, CEILING(+x,1) is 1, FLOOR(-x,1) is -1, QUOTIENT(±x,1) is 0, DELTA(x,0) is
0, DELTA(x,x) is 1, GESTEP(+x,0) is 1 and GESTEP(-x,0) is 0, while SIGN(+x) and INT(-x)
transition inside the subnormal range (0 at 0x00000000001c7ea3 and ±1 at 0x0008bfe58468888d).

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `SQRT(-5e-324)` | number `-5e-324` (`0x8000000000000001`) (cells) | `0` (`0x0000000000000000`) | Excel 16.0.20527.20150, Windows 64-bit, AMD Ryzen 7 4800H, en-ZA |
| `=EXP(-745)` | as typed | `5e-324` (`0x0000000000000001`) | Excel 16.0.20527.20150, Windows 64-bit, AMD Ryzen 7 4800H, en-ZA |
| `=SQRT(EXP(-745))` | as typed | `2.2227587494850775e-162` (`0x1e60000000000000`) | Excel 16.0.20527.20150, Windows 64-bit, AMD Ryzen 7 4800H, en-ZA |
| `=SQRT(0-EXP(-745))` | as typed | `#NUM!` | Excel 16.0.20527.20150, Windows 64-bit, AMD Ryzen 7 4800H, en-ZA |
| `=EVEN(EXP(-745))` | as typed | `2` (`0x4000000000000000`) | Excel 16.0.20527.20150, Windows 64-bit, AMD Ryzen 7 4800H, en-ZA |
| `=SIGN(EXP(-730))` | as typed | `0` (`0x0000000000000000`) | Excel 16.0.20527.20150, Windows 64-bit, AMD Ryzen 7 4800H, en-ZA |
| `=SIGN(EXP(-709))` | as typed | `1` (`0x3ff0000000000000`) | Excel 16.0.20527.20150, Windows 64-bit, AMD Ryzen 7 4800H, en-ZA |
| `=INT(0-EXP(-730))` | as typed | `0` (`0x0000000000000000`) | Excel 16.0.20527.20150, Windows 64-bit, AMD Ryzen 7 4800H, en-ZA |
| `=INT(0-EXP(-709))` | as typed | `-1` (`0xbff0000000000000`) | Excel 16.0.20527.20150, Windows 64-bit, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): 430 bank rows replayed with Value2 input read-back and +0 controls, 22
subnormal bit-pattern cell-write probes, 16 literal-acceptance probes, 45 arithmetic underflow
probes, and 180 true-subnormal function calls in Excel 16.0 build 20527 on 2026-11-08.

- Open: The exact subnormal threshold between 0x00000000001c7ea3 (EXP(-730)) and 0x0008bfe58468888d (EXP(-709)) where SIGN(+x) changes from 0 to 1 and INT(-x) changes from 0 to -1 is not yet pinned.

Evidence:

- [`content/notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md`](../../content/notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md), Q1: Subnormal and maximum-finite inputs — summary of the build 20527 subnormal and max-finite measurements
- DnaCalc `crates/calc-func/src/witness_exec.rs`, flush_cell_value2_number at `c0b94f88ab` — cell Value2 input flushing in witness replay
- DnaCalc `crates/calc-func/src/catalogue/math.rs` at `e042619e35` — true-subnormal handling across math, rounding and comparison functions
- [`content/notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md`](../../content/notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md), Q1: Subnormal and maximum-finite inputs (readings)

<!-- efh:notes end -->

## Excel versions, platforms and CPUs

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.ABS.json; edits here are overwritten -->

Readings of ABS with a recorded Excel build: 1,267 input(s) over 2 recorded environment(s),
Excel 16.0 build(s) 19929, 20430, all on Windows; none of these captures records the CPU. 1
same-input comparison(s) between environments over 2 input-pair(s): no result differs. No
reading from a Mac, Excel for the web, 32-bit Excel or an Intel CPU is recorded.

Readings of `ABS` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 40 | date not recorded |
| 16.0 build 20430 | Windows, 64-bit | CPU not recorded | 1,227 | 2026-09-29 |

Compared on the same inputs:

| Check | Differs in | Environments | Inputs compared | Inputs with different results |
|---|---|---|---:|---:|
| XC-ABS-001 | excel-build | 16.0 build 19929 vs 16.0 build 20430 | 2 | 0 |

Differences observed: none.

Evidence for every row: [`content/evidence/environments/FUNC.ABS.json`](../../content/evidence/environments/FUNC.ABS.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| sign-bit operation | An operation that changes only the sign bit; exact, and never overflows or underflows |
| Sterbenz's lemma | If two positive numbers are within a factor of two, their difference is computed exactly |
| absolute tolerance | A fixed comparison threshold, meaningful only over a narrow magnitude range |
| negative zero | The IEEE 754 value `-0`, distinct in bits from `+0` and equal to it under `=` |

## Sources

- Microsoft, "ABS function" —
  <https://support.microsoft.com/en-us/office/abs-function-3420200f-5628-4e8c-99da-c99d7c87713c>
  — the canonical article. **Not retrieved for this pass** (HTTP 403), so nothing here is stated
  on its authority. The projection `data/functions/FUNC.ABS.json` carries Microsoft's English
  one-line description verbatim, and that is the only documented text this page relies on.
- IEEE Std 754-2019, clause 5.5.1 — absolute value as a sign-bit operation: exact, quiet, and
  total.
- Higham, *Accuracy and Stability of Numerical Algorithms*, chapter 2 — Sterbenz's lemma and the
  conditioning of subtraction, which is where the error in the `ABS(a - b)` idiom actually lives.
- Handbook projections `data/functions/FUNC.ABS.json` (arity, `NumToNum` kernel signature,
  `UnaryNumericScalarOrArrayElementwise` lift profile, `real_result_policy` with
  `non_finite=allow`) and `data/presence/FUNC.ABS.json` (implementing modules and the
  `BUG-FUNC-022` array-lift defect stream).
- Handbook [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- [sweep-questions-1-ANSWERS.md](../notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md)
  (`C:\Work\DnaCalc\codex-runs\sweep-questions-1\ANSWERS.md`, `Q1-subnormal-inputs.json`) and
  `C:\Work\DnaCalc\codex-runs\sweep-report-2.md` — Excel 16.0 build 20527 subnormal cell-write
  flushing, true computed subnormal probes, and DNA Calc bank comparison (`c0b94f88ab`,
  `e042619e35`).
