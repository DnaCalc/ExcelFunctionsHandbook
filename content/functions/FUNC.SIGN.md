---
schema: efh.function-page/v1
function_id: FUNC.SIGN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — SIGN function"
    locator: "https://support.microsoft.com/en-us/office/sign-function-109c932d-fcdc-4023-91f1-2dd0e916a1d8"
    role: "documented signature, the three-valued result, and the worked examples"
  - work: "IEEE 754-2019, Standard for Floating-Point Arithmetic"
    locator: "clauses on signed zero and on the copySign/isSignMinus operations"
    role: "why signum and sign-bit extraction are different functions, and where they differ"
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
family: sign_fn
role_in_family: >-
  The sole member: the three-valued signum, and the category's cleanest example of a function
  whose only difficulty is the boundary case at zero.
---

## What it computes

`SIGN(number)` is the **signum** function:

    sgn(x) =  1  for x > 0
              0  for x = 0
             -1  for x < 0

Microsoft's page states exactly that: it "Returns 1 if the number is positive, zero (0) if the
number is 0, and -1 if the number is negative", with `=SIGN(10)` giving 1, `=SIGN(4-4)` giving
0, and `=SIGN(-0.00001)` giving -1.

- **Domain**: all real numbers; the documentation says "any real number".
- **Range**: the three-element set `{-1, 0, 1}`.
- **Parity**: odd, `sgn(-x) = -sgn(x)`.
- **Structure**: `sgn` is the derivative of `|x|` away from the origin, and `x = sgn(x) * |x|`
  is the polar decomposition of a real number. `sgn` is discontinuous at `0` — the only
  discontinuity, a jump of size 2 — and that single point is where every implementation question
  lives.
- **Identities worth having**: `SIGN(x) * ABS(x) = x`; `SIGN(x)^2` is `1` except at zero;
  `ABS(SIGN(a) - SIGN(b))` is a sign-change detector, which is the classic bisection guard.

The three-valued convention is worth naming explicitly because it is not universal. Some
languages define `sign` as two-valued (`±1`, with zero going to `+1`), and hardware sign-bit
extraction is two-valued by construction. Excel's documented function is the three-valued
mathematical signum.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | Any real number. Required. | — |

One argument; the reference engine records an arity of exactly one, an ordinary numeric slot
under the shared coercion rules — see
[Coercion and lifting](../model/02-coercion-and-lifting.md). Logicals convert (`TRUE` is 1) and
text that reads as a number converts.

There is no tolerance argument and no epsilon. `SIGN` compares against exact zero, which is the
source of essentially every surprise it produces (see *Numerical notes*).

## Result and edge cases

Returns `Number`, one of `-1`, `0`, `1`.

- **Zero.** The documented result is `0`. Writing `-0` (`0x8000000000000000`) to a cell via
  `Range.Value2` stores `+0` (`0x0000000000000000`), and `SIGN` returns `0`
  ([sweep-questions-1-ANSWERS.md](../notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md),
  `C:\Work\DnaCalc\codex-runs\sweep-questions-1\Q1-subnormal-inputs.json`).
- **Subnormals.** Writing a positive or negative subnormal binary64 value (`±5e-324` through
  `±(2^-1022 - 5e-324)`) to a cell via `Range.Value2` flushes the cell to `+0`, so
  `SIGN(±5e-324)` on a cell-written input returns `0` ([FN-ABS-002](../notes/FN-ABS-002.json),
  `Q1-subnormal-inputs.json`). When `SIGN` receives a true computed subnormal in a formula on
  Excel 16.0 build 20527 (`x` from `EXP(-745) = 0x0000000000000001`,
  `EXP(-730) = 0x00000000001c7ea3`, or `EXP(-709) = 0x0008bfe58468888d`), negative subnormals
  `0 - x` return `-1` at all three magnitudes, whereas positive subnormals transition inside the
  subnormal range: `SIGN(EXP(-745)) = 0` and `SIGN(EXP(-730)) = 0`, while `SIGN(EXP(-709)) = 1`
  (`Q1-subnormal-inputs.json`). Normal extremes `±2^-1022` and `±DBL_MAX` (`0x7fefffffffffffff` /
  `0xffefffffffffffff`) return `±1`.
- **Non-finite inputs.** The reference engine's projected `real_result_policy` for `SIGN` reads
  `arg_domain_guard=none` with `non_finite=allow`. Since `SIGN`'s output is always one of three
  small integers, `non_finite=allow` cannot describe the output; it describes the policy slot
  being unused. An infinite *input* is not reachable from a worksheet cell, so the case is
  theoretical on this surface.

The reference engine classifies `SIGN` as a unary numeric scalar-or-array kernel
(`UnaryNumericScalarOrArrayElementwise`) with `kernel_signature_class: NumToNum` and
`error_collapse_profile: None`, so arrays lift elementwise and element failures stay
element-local.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | The argument does not convert to a number | Shared coercion rules |
| propagated | An error value in the argument is returned unchanged | Shared coercion rules |

Microsoft's page lists no error conditions for `SIGN`, and no domain restriction exists — the
function is total on the reals.

## Relationships

- **`ABS`** — the other half of the decomposition `x = SIGN(x) * ABS(x)`. The two are almost
  always used together.
- **`IF(x>0, 1, IF(x<0, -1, 0))`** — the hand-written equivalent, and worth mentioning because
  it is the form readers write when they have forgotten `SIGN` exists. It also makes the
  comparison semantics explicit, which `SIGN` hides.
- **`GESTEP` and `DELTA`** (engineering category) — the other threshold functions. `DELTA(a, b)`
  is an equality indicator and `GESTEP(x, step)` a step function; between them and `SIGN` there
  are three different conventions for what happens exactly at the boundary, and they do not
  agree with each other.
- **`TANH`** — the smooth saturating analogue. Where `SIGN` is used to steer a computation,
  `TANH(k*x)` for large `k` is the differentiable stand-in, which is why it turns up in
  optimization work.
- **`TRUNC`, `INT`, `ROUND`** — the rounding family shares `SIGN`'s sensitivity to which side of
  zero a value falls on; `TRUNC` in particular is "round toward zero", i.e. rounding in the
  direction opposite to `SIGN`.
- **Confused with**: `ABS` (magnitude, not direction) and with a two-valued sign, which is what
  most programming languages' bit-level helpers return.

## Numerical notes

`SIGN` performs no arithmetic, so it has no rounding error of its own. Everything that goes
wrong with it goes wrong upstream, and there are three recurring patterns worth naming.

**1. The zero case is almost never the zero you mean.** `SIGN(0.1 + 0.2 - 0.3)` is not `0`,
because `0.1 + 0.2 - 0.3` is not `0` in binary64. Any workflow that uses `SIGN` to test "did
this quantity come out to zero" is testing an exact bit condition and will get `±1` on values
that are zero to any reasonable tolerance. The remedy is not to fix `SIGN`; it is to test
`ABS(x) < tol` explicitly, so that the tolerance is visible in the formula rather than implied.

**2. `SIGN` is a discontinuity amplifier.** Its condition number is infinite at the origin: an
arbitrarily small perturbation of the input changes the output by 1 or 2. That is inherent to
the mathematics, not a defect, but it means `SIGN` should never sit downstream of a long
accumulation whose last bits are noise. The classic failure is a sign-change test on the
residual of an iterative calculation, where the residual's own error exceeds its value near
convergence — the bisection loop then wanders because `SIGN` is reporting the sign of the
rounding error.

**3. Sign-bit extraction and signum are different functions.** They agree everywhere except at
zero, where signum gives `0` for both zeros and sign-bit extraction gives `+1`/`-1`
respectively. An implementation written with `copysign` or a bit mask gets the negative-zero
case wrong relative to the documented three-valued definition; an implementation written with
two comparisons gets it right. Since the difference shows up on exactly one input class, it
survives casual testing indefinitely. This is the reason the negative-zero probe below is worth
running even though the case looks exotic.

**What a careful implementation does**: two comparisons against literal zero, in the order
`x > 0`, then `x < 0`, then `0`. Under IEEE comparison semantics this handles both zeros
correctly and needs no special case. For Excel compatibility on true computed subnormals
(`crates/calc-func/src/catalogue/math.rs`, commit `e042619e35426ffac3d8cfaaa2dd2961abe3d02c`),
negative computed subnormals return `-1`, while positive computed subnormals transition from `0`
at `0x00000000001c7ea3` (`EXP(-730)`) to `1` at `0x0008bfe58468888d` (`EXP(-709)`); DNA Calc
declines positive computed subnormals with a typed not-modelled code until that interior
threshold is pinned, and matches all 1,226 of 1,226 recorded `SIGN` bank rows
([sweep-questions-1-ANSWERS.md](../notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md),
`C:\Work\DnaCalc\codex-runs\sweep-report-2.md`).

## What has not been checked

No Handbook evidence record lists `FUNC.SIGN` in its subjects, and no Handbook vector suite
exists for `SIGN`. **Nobody has checked this function against Excel within the Handbook's
record.** The three-valued definition above is documented by Microsoft; everything else on this
page is mathematics, the shared call model, or the reference engine's own classification.

Probes worth running first:

1. **Negative zero.** `SIGN(-1*0)`, `SIGN(0*-1)`, and a `-0` produced by underflow from below.
   This is the one input that distinguishes a comparison-based implementation from a
   sign-bit-based one, and the documentation is silent on it. Highest value per probe on this
   page by a wide margin.
2. **Subnormal inputs of both signs**, to confirm that magnitude plays no part.
3. **Text and logical arguments** — `SIGN("−5")` with a Unicode minus, `SIGN("1e-400")` (which
   underflows on parse), `SIGN(TRUE)`, `SIGN(FALSE)`. The parse boundary is where the shared
   coercion rules meet this function, and the underflowing-text case is the one that could
   plausibly disagree between implementations.
4. **An empty referenced cell** versus an omitted argument, to confirm the Empty/Missing split
   documented in the call model behaves here as elsewhere.
5. **Array arguments**, to confirm elementwise lifting and element-local `#VALUE!`.
6. **`SIGN(x) * ABS(x) = x`** as a metamorphic identity across a broad sample — cheap, needs no
   oracle, and would catch a two-valued implementation immediately at zero.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `SIGN` that an implementer matching Excel, or a reader explaining an
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

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.SIGN.json; edits here are overwritten -->

Readings of SIGN with a recorded Excel build: 1,266 input(s) over 2 recorded environment(s),
Excel 16.0 build(s) 19929, 20430, all on Windows; none of these captures records the CPU. 1
same-input comparison(s) between environments over 2 input-pair(s): no result differs. No
reading from a Mac, Excel for the web, 32-bit Excel or an Intel CPU is recorded.

Readings of `SIGN` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 40 | date not recorded |
| 16.0 build 20430 | Windows, 64-bit | CPU not recorded | 1,226 | 2026-09-29 |

Compared on the same inputs:

| Check | Differs in | Environments | Inputs compared | Inputs with different results |
|---|---|---|---:|---:|
| XC-SIGN-001 | excel-build | 16.0 build 19929 vs 16.0 build 20430 | 2 | 0 |

Differences observed: none.

Evidence for every row: [`content/evidence/environments/FUNC.SIGN.json`](../../content/evidence/environments/FUNC.SIGN.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| signum | The three-valued sign function returning `-1`, `0` or `1` |
| negative zero | The IEEE 754 value `-0`, which compares equal to `+0` but carries a sign bit |
| sign-bit extraction | A two-valued alternative that reads the sign bit and never returns zero |
| discontinuity amplifier | A function whose output changes by a fixed amount under an arbitrarily small input change |

## Sources

- Microsoft, "SIGN function" —
  <https://support.microsoft.com/en-us/office/sign-function-109c932d-fcdc-4023-91f1-2dd0e916a1d8>
  (signature, "any real number", the three-valued result, and the examples `SIGN(10)`,
  `SIGN(4-4)`, `SIGN(-0.00001)`; no error conditions are listed).
- IEEE 754-2019 — signed zero, comparison semantics, and the distinction between `isSignMinus`
  and a mathematical signum.
- Handbook projections `data/functions/FUNC.SIGN.json` (arity,
  `kernel_signature_class: NumToNum`, `error_collapse_profile: None`, `real_result_policy` with
  `arg_domain_guard=none` and `non_finite=allow`) and `data/presence/FUNC.SIGN.json`.
- Handbook [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- [sweep-questions-1-ANSWERS.md](../notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md)
  (`C:\Work\DnaCalc\codex-runs\sweep-questions-1\ANSWERS.md`, `Q1-subnormal-inputs.json`) and
  `C:\Work\DnaCalc\codex-runs\sweep-report-2.md` — Excel 16.0 build 20527 subnormal cell-write
  flushing, true computed subnormal `SIGN` transition probes, and DNA Calc bank comparison
  (`c0b94f88ab`, `e042619e35`).
