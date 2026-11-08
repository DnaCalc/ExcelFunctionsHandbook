---
schema: efh.function-page/v1
function_id: FUNC.EVEN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — EVEN function"
    locator: "https://support.microsoft.com/en-us/office/even-function-197b5f06-c795-4c1e-8696-3c3b8a646cf9"
    role: "documented signature, the away-from-zero rule regardless of sign, the no-rounding rule for even integers, and the #VALUE! condition"
  - work: "IEEE 754-2019, Standard for Floating-Point Arithmetic"
    locator: "the roundToIntegral operations"
    role: "the exactness of ceiling and of scaling by a power of two"
  - work: "Muller et al., Handbook of Floating-Point Arithmetic"
    locator: "the chapters on exact operations and on integer-valued floating-point numbers"
    role: "why every double above 2^53 is already an even integer"
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
family: even_fn
role_in_family: >-
  The away-from-zero rounding to an even integer — the parity partner of ODD, and one of the
  few worksheet roundings whose step size is 2 rather than 1.
---

# EVEN

## What it computes

`EVEN(number)` rounds its argument to an even integer, moving **away from zero**. Microsoft's
page states the rule in exactly that form: regardless of the sign, the adjustment is away from
zero, and an argument that is already an even integer is not moved.

In closed form:

    EVEN(x) = 2 · sign(x) · ceil( ABS(x) / 2 )      for x ≠ 0
    EVEN(0) = 0

| Property | Statement |
|---|---|
| Domain | all real `x` |
| Range | the even integers representable as doubles |
| Parity of the map | odd: `EVEN(-x) = -EVEN(x)` |
| Idempotent | `EVEN(EVEN(x)) = EVEN(x)` |
| Monotone | non-decreasing |
| Step | 2 — the function is a staircase with treads of width 2, not 1 |
| Fixed points | exactly the even integers |
| Bound | `ABS(EVEN(x)) ≥ ABS(x)`, with equality only at the fixed points |
| Identity above `2^53` | `EVEN(x) = x` for every double with `ABS(x) ≥ 2^53` |

The last row is the one that surprises people, and it is pure format arithmetic rather than
function design. Above `2^53` the spacing between consecutive doubles is at least 2, so **every
representable value in that range is already an even integer**. `EVEN` is therefore the identity
map on the entire upper half of the format's range, and the function only does anything at all
below `2^53`.

The half-open-interval reading is worth keeping in mind: `EVEN` maps `(0, 2]` to `2`, `(2, 4]`
to `4`, and symmetrically on the negative side. The endpoints belong to the *lower* tread
because an even integer is not moved.

## Arguments

| Argument | Meaning | Notes |
|---|---|---|
| `number` | The value to round. Required. | Microsoft documents no constraint |

Ordinary to-number coercion applies — logicals convert, numeric-looking text converts, blanks
and errors follow the shared model. The reference engine declares the surface a scalar kernel
that lifts elementwise over arrays. See
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`, always an even integer.

- **Zero** — `EVEN(0)` is `0`. Zero is even, so no adjustment is required, and the closed form
  above handles it as the degenerate case rather than as an exception.
- **Negative arguments** — move away from zero: an argument between `-2` and `0` becomes `-2`.
  This is the documented rule and it is the opposite of what "round up" would mean if read as
  "toward `+∞`". Microsoft's own wording — a value is rounded *up* when adjusted *away* from
  zero — is a definition of "up" as "away from zero", and the whole page turns on it.
- **Already-even integers** — unchanged, documented explicitly.
- **Odd integers** — move by exactly 1, away from zero.
- **Values already above `2^53`** — unchanged, as above.
- **The largest finite double** (`±1.7976931348623157e+308`, `0x7fefffffffffffff` /
  `0xffefffffffffffff`) is already an even integer and returns `±DBL_MAX` unchanged on Excel
  16.0 build 20527 ([sweep-questions-1-ANSWERS.md](../notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md),
  `C:\Work\DnaCalc\codex-runs\sweep-questions-1\Q1-subnormal-inputs.json`).
- **Subnormals and signed zero** — writing any positive or negative subnormal binary64 value
  (`±5e-324` through `±(2^-1022 - 5e-324)`) or `-0` to a cell via `Range.Value2` flushes the
  stored cell value to `+0` (`0x0000000000000000`), so `EVEN(±5e-324)` on a cell-written input
  returns `0` ([FN-ABS-002](../notes/FN-ABS-002.json), `Q1-subnormal-inputs.json`). When a true
  computed subnormal reaches `EVEN` in a formula (`x` from `EXP(-745) = 0x0000000000000001`,
  `EXP(-730) = 0x00000000001c7ea3`, or `EXP(-709) = 0x0008bfe58468888d`), `EVEN(x)` rounds away
  from zero to `2` (`0x4000000000000000`) and `EVEN(0 - x)` rounds to `-2`
  (`0xc000000000000000`).

## Errors

As documented on Microsoft's `EVEN` page:

| Error | Condition |
|---|---|
| `#VALUE!` | `number` is non-numeric |

No other error condition is documented, and none is structurally required: the function cannot
overflow and has no domain restriction.

## Relationships

- **`ODD`** — the parity partner, rounding away from zero to an odd integer. The pair is not
  symmetric: `ODD` has no fixed point at zero (`ODD(0)` is documented as 1) and its identity
  region above `2^53` does not exist, because no double above `2^53` is odd. That asymmetry
  makes `ODD` the more interesting of the two at the top of the range and `EVEN` the more
  interesting at the bottom.
- **`CEILING.MATH`** — the general away-from-zero-or-toward-`-∞` rounding.
  `CEILING.MATH(x, 2, 1)` is the away-from-zero reading and coincides with `EVEN` there;
  `CEILING.MATH(x, 2)` with the default mode rounds negatives toward `-∞` and does **not**
  match `EVEN`. Substituting one for the other on a column containing negatives changes results.
- **[FLOOR.MATH](FUNC.FLOOR.MATH.md), `FLOOR.PRECISE`** — the downward
  family, for contrast: they round toward `-∞`, where `EVEN` rounds away from zero. The
  worksheet's rounding functions do not share a single direction convention, and knowing which
  convention a given one uses is most of the skill in choosing between them.
- **`ROUNDUP`** — rounds away from zero to a digit position, the same *direction* convention as
  `EVEN` with a different *grid*.
- **`MROUND(x, 2)`** — rounds to the nearest multiple of 2 rather than away from zero, so it
  disagrees with `EVEN` on every argument in the lower half of each tread.
- **`ISEVEN`** — the predicate, which tests parity rather than producing it, and which
  truncates rather than rounding.

## Numerical notes

`EVEN` is one of the few functions in this batch that can be, and should be, **exact**.

Every step of the closed form is exact in binary64:

1. `ABS(x)` is exact — a sign-bit clear.
2. `ABS(x) / 2` is exact — division by a power of two only changes the exponent, and it cannot
   underflow into the subnormal range in a way that loses bits unless `x` was already subnormal,
   where the result is `0` or `2` anyway.
3. `ceil` is exact — it is an IEEE 754 `roundToIntegral` operation, which is by definition
   exactly rounded and, for a value already integral, the identity.
4. Multiplying by 2 is exact, and cannot overflow: the input to this step is at most
   `ceil(DBL_MAX/2)`, so the product is at most `DBL_MAX`.

So a correct implementation commits **no rounding error at all** on any argument, and any
disagreement between two implementations of `EVEN` is a disagreement about the *rule*, never
about the arithmetic. That is a rare and pleasant situation, and it means the interesting probes
here are all about tie behaviour and sign conventions rather than about ulp.

Two traps for implementers:

- **Do not use `2 * ceil(x / 2)` without the absolute value.** For negative `x`, `ceil` rounds
  toward `+∞`, i.e. *toward* zero, which is the opposite of the documented rule. The sign must
  be factored out first and restored afterwards.
- **Do not test "is already even" by `MOD(x, 2) = 0`** on large arguments. Above `2^53` this is
  true of everything, which happens to be correct, but the test itself involves a division that
  is exact only because of the power of two; the same pattern with a non-power-of-two step is
  not exact, and the habit transfers badly to `MROUND` and `CEILING.MATH`.

The identity region above `2^53` deserves one more sentence, because it is a fact about the
format that this function makes visible: a spreadsheet user who applies `EVEN` to a large
identifier — an account number stored as a number, say — will see it returned unchanged and may
conclude the function is broken. It is not; the value was already even, and had been since it
crossed `2^53`.

In DNA Calc (`crates/calc-func/src/catalogue/math.rs`, commit
`e042619e35426ffac3d8cfaaa2dd2961abe3d02c`, and `crates/calc-func/src/witness_exec.rs`, commit
`c0b94f88ab4e3df5b518329d08ea03cf481abdbe`), `EVEN` rounds true positive and negative subnormals
to `±2` while cell-input replay flushes subnormal `Range.Value2` cell writes to `+0`, matching
all 1,227 of 1,227 recorded `EVEN` bank rows (`C:\Work\DnaCalc\codex-runs\sweep-report-2.md`).

## What has not been checked

**No evidence record in the Handbook names `EVEN`.** The presence projection records no
discrepancy-catalogue entry, no math-deviation entry, no known-exactness-deviation entry and no
bug stream for this surface. Nobody has checked this function against Excel within the
Handbook's record.

No Handbook vector suite exists for `EVEN`. The away-from-zero rule, the no-rounding rule for
even integers and the `#VALUE!` condition are Microsoft's; the mathematics and the exactness
argument above are the Handbook's; and none of it has been observed in Excel here.

Inputs I would probe first:

1. **The negative side, at the tread boundaries**: `EVEN(-1)`, `EVEN(-2)`, `EVEN(-2.0000001)`,
   `EVEN(-0.5)`. The documented rule says these go to `-2`, `-2`, `-4`, `-2`. This is where a
   `ceil`-without-`abs` implementation fails, and it fails visibly rather than in the last bit.
2. **`EVEN(0)` and `EVEN(-0)`.** Zero is a fixed point; the signed-zero question is not
   documented anywhere, and the answer is visible through `1/EVEN(-0)`.
3. **The `2^53` frontier**: `EVEN(2^53 - 1)`, `EVEN(2^53)`, `EVEN(2^53 + 2)`. Below the
   frontier odd integers exist and must move; at and above it nothing can. A surprise here would
   mean the implementation is not working in binary64.
4. **The top of the range**: `EVEN(DBL_MAX)`, and the largest odd-behaving argument. The
   mathematics says no overflow is possible; an implementation that multiplies before halving
   would overflow anyway, and this probe catches it.
5. **Just-below-integer arguments**: `EVEN(1.9999999999999998)`, `EVEN(2.0000000000000004)` —
   the doubles adjacent to a fixed point, which separate "already even, leave alone" from
   "round up one tread".
6. **Subnormal arguments**: `EVEN(5E-324)` should be `2`, since the tiniest positive value is
   still positive and the tread above zero ends at 2.
7. **Coercion probes**: `EVEN(TRUE)`, `EVEN("3")`, `EVEN("")` and an array argument, since the
   surface is declared an elementwise scalar kernel and that declaration is unverified.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `EVEN` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

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

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.EVEN.json; edits here are overwritten -->

Readings of EVEN with a recorded Excel build: 1,270 input(s) over 2 recorded environment(s),
Excel 16.0 build(s) 19929, 20430, all on Windows; none of these captures records the CPU. 1
same-input comparison(s) between environments over 2 input-pair(s): no result differs. No
reading from a Mac, Excel for the web, 32-bit Excel or an Intel CPU is recorded.

Readings of `EVEN` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 43 | date not recorded |
| 16.0 build 20430 | Windows, 64-bit | CPU not recorded | 1,227 | 2026-09-29 |

Compared on the same inputs:

| Check | Differs in | Environments | Inputs compared | Inputs with different results |
|---|---|---|---:|---:|
| XC-EVEN-001 | excel-build | 16.0 build 19929 vs 16.0 build 20430 | 2 | 0 |

Differences observed: none.

Evidence for every row: [`content/evidence/environments/FUNC.EVEN.json`](../../content/evidence/environments/FUNC.EVEN.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| away from zero | The documented direction: negatives get more negative, positives more positive |
| tread | One step of the staircase: a half-open interval mapped to a single even integer |
| fixed point | An argument the function returns unchanged; here, exactly the even integers |
| identity region | Arguments at or above `2^53`, where every double is already even |
| `roundToIntegral` | The IEEE 754 family of exact integer-rounding operations |
| exact operation | One that introduces no rounding error for any representable input |

## Sources

- Microsoft, "EVEN function" —
  <https://support.microsoft.com/en-us/office/even-function-197b5f06-c795-4c1e-8696-3c3b8a646cf9>
  (fetched at curation: signature, the away-from-zero rule stated as holding regardless of sign,
  the rule that an even integer is not rounded, the `#VALUE!` condition, and the worked
  examples).
- IEEE 754-2019 — `roundToIntegral` operations and the exactness of scaling by a power of two.
- Muller et al., *Handbook of Floating-Point Arithmetic* — integer-valued doubles and the
  spacing argument behind the identity region.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [The value universe](../model/01-value-universe.md).
- Handbook projections `data/functions/FUNC.EVEN.json` (arity, the elementwise lift profile, the
  `non_finite=allow` axis value) and `data/presence/FUNC.EVEN.json` (which records no
  discrepancy, deviation or defect-stream mention for this surface).
- [sweep-questions-1-ANSWERS.md](../notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md)
  (`C:\Work\DnaCalc\codex-runs\sweep-questions-1\ANSWERS.md`, `Q1-subnormal-inputs.json`) and
  `C:\Work\DnaCalc\codex-runs\sweep-report-2.md` — Excel 16.0 build 20527 subnormal cell-write
  flushing, true computed subnormal `EVEN(EXP(-745)) = 2` / `EVEN(0-EXP(-745)) = -2`, and DNA
  Calc bank comparison (`c0b94f88ab`, `e042619e35`).
