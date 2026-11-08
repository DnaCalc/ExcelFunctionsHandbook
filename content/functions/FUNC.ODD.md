---
schema: efh.function-page/v1
function_id: FUNC.ODD
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
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
family: odd_fn
role_in_family: >-
  Sole member of its module; rounds away from zero to an odd integer, and is one of the few
  Excel functions whose defining postcondition provably fails on part of its own domain.
---

## What it computes

`ODD(number)` rounds *number* **away from zero** to the nearest odd integer.

Microsoft's two remarks give the rule completely:

> "Regardless of the sign of number, a value is rounded up when adjusted away from zero."
>
> "If number is an odd integer, no rounding occurs."

So the direction is outward from zero in both half-planes — 1.5 rises to 3, and −2 falls to −3 —
and an argument already on an odd integer is returned unchanged. The documented examples pin all
five cases: 1.5 → 3, 3 → 3, 2 → 3, −1 → −1, −2 → −3.

Written out:

    ODD(x) = sign(x) · ( 2·⌈ (|x| − 1)/2 ⌉ + 1 )

**Domain and range.** Domain: all reals. Range: the odd integers — with two qualifications, one
at zero and one at the top of the floating-point range, both below.

**Shape.** A staircase with steps of width 2 and jump 2, symmetric under *x* ↦ −*x* (the function
is odd in the algebraic sense as well as in the arithmetic one: ODD(−x) = −ODD(x) away from zero).
It is idempotent — ODD(ODD(x)) = ODD(x) — and it is the away-from-zero counterpart of `EVEN`,
which does the same thing to an even lattice.

**Zero is the interesting point.** Zero is an even integer, so `ODD(0)` must move — but "away from
zero" has no direction at zero. The two candidates are ±1, equidistant and equally defensible.
**Microsoft's page does not state which one is returned**; the documented example list stops at
−1. The reference engine returns +1. That is the first probe below.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `number` | "The value to round." | Yes |

Exactly one argument; the declared arity is one to one. The slot is numeric and subject to
ordinary to-number coercion ([coercion and lifting](../model/02-coercion-and-lifting.md)). The
declared coercion profile is the unary numeric scalar-or-array-elementwise one, so `ODD` is a
scalar kernel that lifts over arrays.

## Result and edge cases

Returns `Number` — an integer value.

The reference engine's published battery is rendered beside this page. Qualitatively:

- **Zero** returns +1, as discussed above. Undocumented.
- **−1** is returned unchanged, matching the documented example.
- **A logical argument** converts and rounds; **numeric text** converts and rounds.
- **The smallest positive subnormal** — writing any positive or negative subnormal binary64
  value (`±5e-324` through `±(2^-1022 - 5e-324)`) or `-0` to a cell via `Range.Value2` flushes
  the stored cell value to `+0` (`0x0000000000000000`) on Excel 16.0 build 20527, so both
  `ODD(5e-324)` and `ODD(-5e-324)` on cell-written inputs evaluate `ODD(0) = +1`
  ([sweep-questions-1-ANSWERS.md](../notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md),
  `C:\Work\DnaCalc\codex-runs\sweep-questions-1\Q1-subnormal-inputs.json`). When a true computed
  subnormal reaches `ODD` in a formula (`x` from `EXP(-745) = 0x0000000000000001`,
  `EXP(-730) = 0x00000000001c7ea3`, or `EXP(-709) = 0x0008bfe58468888d`), `ODD(x)` returns `+1`
  (`0x3ff0000000000000`) and `ODD(0 - x)` returns `-1` (`0xbff0000000000000`)
  ([FN-ABS-002](../notes/FN-ABS-002.json)).
- **An inline array** lifts elementwise.
- **An empty range** produces `#VALUE!`.
- **The largest finite double** (`±1.7976931348623157e+308`, `0x7fefffffffffffff` /
  `0xffefffffffffffff`) is returned unchanged on Excel 16.0 build 20527 (`Q1-subnormal-inputs.json`)
  — and it is an **even** integer. See below; this is not a rounding decision, it is a
  representability wall.

### The postcondition fails above 2^53

`ODD` promises an odd result. Above 2^53 it cannot deliver one, and the reason is arithmetic
rather than implementation.

Binary64 spaces consecutive representable values 2 apart in [2^53, 2^54), 4 apart in
[2^54, 2^55), and so on. **Every finite double of magnitude at or above 2^53 is an even integer**,
because it is a multiple of at least 2. There is no odd double above that point. So for any
argument in that range:

- returning the argument unchanged gives an **even** answer, violating the function's defining
  property; and
- returning "the next odd integer" is impossible, because that integer is not representable —
  the nearest doubles either side are both even.

The reference engine's battery row for the largest finite double takes the first option. Any
implementation must take it or must error; there is no third choice. The honest statement of the
function's range is therefore: **the odd integers below 2^53, and whatever convention the
implementation adopts above it.** No documentation addresses this, and it is not a defect in
anyone's code — it is the domain running out.

The same wall governs `EVEN` in mirror image, where the story is happier: above 2^53 every double
already *is* even, so `EVEN`'s postcondition survives exactly where `ODD`'s fails.

## Errors

| Error | Condition | Basis |
|---|---|---|
| `#VALUE!` | The argument is nonnumeric | **Documented**: "If number is nonnumeric, ODD returns the #VALUE! error value." |
| propagated | An error value in the argument | Shared call model |

The reference engine additionally reports `#VALUE!` for arity failures; in Excel a missing
required argument is expected to be refused at formula entry
([the call pipeline](../model/03-call-pipeline.md)).

There is no documented domain error, and none is needed: `ODD` is total on the reals.

## Relationships

- **`EVEN`** — the twin, rounding away from zero to an even integer. The pair are not inverses and
  do not compose usefully; they partition nothing. Note the asymmetry recorded above: `EVEN`'s
  postcondition survives the 2^53 wall and `ODD`'s does not.
- **[ISEVEN](FUNC.ISEVEN.md) / `ISODD`** — the parity *tests*, which truncate toward zero. `ODD`
  is a rounding function, not a predicate, and `ISODD(ODD(x))` is true by construction only below
  the wall.
- **[MROUND](FUNC.MROUND.md)** — rounds to the nearest multiple. `ODD` is not `MROUND(x, 2)` with
  an offset: `MROUND` rounds to *nearest* while `ODD` rounds *away*, so they agree only when the
  argument is already past the midpoint.
- **[INT](FUNC.INT.md)** and **`TRUNC`** — the other integer-valued roundings, each with a
  different convention. This category has floor, truncate-toward-zero, away-from-zero and
  half-away-from-zero all in play, and the names do not disclose which is which.
- **[ROUNDUP](FUNC.ROUNDUP.md)** — rounds away from zero to a number of decimal places;
  `ROUNDUP(x, 0)` is the away-from-zero integer rounding without the parity constraint.
- The most common real use is column and row banding, and generating alternating series — both of
  which stay in the small-integer range where none of this page's edge cases can arise.

## Numerical notes

`ODD` performs no transcendental work and, below the representability wall, no rounding of its
own. Its numerical character comes from three places.

**Discontinuity amplification.** Like [INT](FUNC.INT.md), `ODD` is a step function, so a sub-ULP
error in its argument becomes a full step — here a jump of **2**, not 1 — whenever the argument
straddles a step boundary. The boundaries are the odd integers themselves: an argument that ought
to be exactly 3 but arrives as the next double above 3 is rounded to 5. Anything computed and then
passed to `ODD` should be rounded to an integer first if the intent was an integer.

**The representability wall**, described in full above. An implementation that computes
`2*ceil((|x|-1)/2)+1` in floating point does not merely return an even number above 2^53 — the
intermediate `|x|-1` is itself a no-op there (subtracting 1 from a double spaced 2 apart returns
the same double), so the formula degenerates to the identity. That the naive formula and the
correct-by-convention answer coincide is a coincidence, not a design, and an implementation that
converts through a fixed-width integer instead will produce something different — or wrap — at the
same inputs.

**The sign of zero and subnormals.** Writing `-0.0` or a negative subnormal to a cell via
`Range.Value2` stores `+0` (`0x0000000000000000`), which `ODD` maps to `+1`. On true computed
subnormals in a formula (`0 - EXP(-745)`), `ODD` preserves the negative direction and returns
`-1`. In DNA Calc (`crates/calc-func/src/catalogue/math.rs`, commit
`e042619e35426ffac3d8cfaaa2dd2961abe3d02c`, and `crates/calc-func/src/witness_exec.rs`, commit
`c0b94f88ab4e3df5b518329d08ea03cf481abdbe`), `ODD` rounds true positive and negative subnormals
to `±1` and returns `±DBL_MAX` unchanged above `2^53`, matching all 1,227 of 1,227 recorded
`ODD` bank rows (`C:\Work\DnaCalc\codex-runs\sweep-report-2.md`).

The relevant background is not a special-function reference — there is no series to sum here — but
the general treatment of rounding to an integer lattice and the exactness of the operations
involved: IEEE 754-2019's `roundToIntegral*` family, and Muller et al., *Handbook of
Floating-Point Arithmetic*, on the spacing of binary64 and on rounding to a grid.

## What has not been checked

No Handbook vector suite exists for `ODD`; `vectors/` publishes nothing for this function, and
**no evidence record names `ODD` at all** — not as a subject, and not inside any group's
membership. Nobody has checked this function against Excel within the Handbook's record. The
implementing module carries no open upstream defect stream, which means only that nothing has been
filed, not that anything has been confirmed.

Everything above marked as documented comes from Microsoft's `ODD` page: the syntax, the argument
description, the `#VALUE!` remark, the away-from-zero rule, the no-rounding-for-odd-integers rule,
and the five worked examples.

Inputs I would probe first:

1. **`ODD(0)`.** One cell. Microsoft's page does not say, the two candidate answers are equally
   defensible, and the reference engine has picked one. This is the page's cleanest open question.
2. **`ODD(-0.0)`** — reached as `ODD(-1*0)` or `ODD(0*-1)` — to see whether the sign bit steers the
   direction.
3. **The 2^53 boundary**: `ODD(2^53 - 1)`, `ODD(2^53)`, `ODD(2^53 + 2)` and the largest finite
   double. This locates where the postcondition stops holding and reveals whether Excel returns the
   argument, errors, or does something else. It is the most *interesting* probe on the page, and
   nothing in the Handbook's record predicts the answer.
4. **`ODD(0.5)`, `ODD(-0.5)`, `ODD(1)`, `ODD(-1)`** — the smallest-magnitude behaviour either side
   of zero, which is where the away-from-zero rule is easiest to get subtly wrong.
5. **`ODD(TRUE)`** — the reference engine converts the logical, and the documentation's
   "nonnumeric" wording does not obviously cover a logical.
6. **An array argument**, to confirm the elementwise lift and element-local failures.
7. **`ODD(3 + 2^-50)`**, an argument a hair above an odd integer, as the concrete form of the
   amplification hazard.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `ODD` that an implementer matching Excel, or a reader explaining an
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

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.ODD.json; edits here are overwritten -->

Readings of ODD with a recorded Excel build: 1,268 input(s) over 2 recorded environment(s),
Excel 16.0 build(s) 19929, 20430, all on Windows; none of these captures records the CPU. 1
same-input comparison(s) between environments over 2 input-pair(s): no result differs. No
reading from a Mac, Excel for the web, 32-bit Excel or an Intel CPU is recorded.

Readings of `ODD` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 41 | date not recorded |
| 16.0 build 20430 | Windows, 64-bit | CPU not recorded | 1,227 | 2026-09-29 |

Compared on the same inputs:

| Check | Differs in | Environments | Inputs compared | Inputs with different results |
|---|---|---|---:|---:|
| XC-ODD-001 | excel-build | 16.0 build 19929 vs 16.0 build 20430 | 2 | 0 |

Differences observed: none.

Evidence for every row: [`content/evidence/environments/FUNC.ODD.json`](../../content/evidence/environments/FUNC.ODD.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| away from zero | The rounding direction: outward in both half-planes |
| idempotent | Applying the function twice changes nothing after the first application |
| representability wall | 2^53, above which no odd integer is a binary64 value |
| postcondition | The property the result is supposed to have — here, being odd |
| amplification | A sub-ULP argument error becoming a full step in the result |

## Sources

- Microsoft, "ODD function" —
  <https://support.microsoft.com/en-us/office/odd-function-deae64eb-e08a-4c88-8b40-6d0b42575c98>.
  Retrieved for this page: the syntax, the argument description, the `#VALUE!` nonnumeric remark,
  the away-from-zero rule, the no-rounding-for-odd-integers remark, and the five worked examples.
  The page as retrieved says nothing about zero and nothing about large magnitudes.
- IEEE 754-2019 — binary64 value spacing and the `roundToIntegral*` operations;
  J.-M. Muller et al., *Handbook of Floating-Point Arithmetic*, 2nd edition — spacing of the
  binary64 grid and rounding to a lattice.
- Handbook, [INT](FUNC.INT.md) — the amplification hazard shared by every step function in this
  category; [ISEVEN](FUNC.ISEVEN.md) — the parity predicate and its own 2^53 discussion;
  [coercion and lifting](../model/02-coercion-and-lifting.md) — to-number outcomes, scalar-kernel
  lifting, error propagation.
- `data/functions/FUNC.ODD.json` and `data/presence/FUNC.ODD.json` — identity, signature
  `ODD(number)`, arity 1–1, the `UnaryNumericScalarOrArrayElementwise` coercion profile, the
  implementing module and its Lean companion, as projected at OxFunc `473efa3`.
- [sweep-questions-1-ANSWERS.md](../notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md)
  (`C:\Work\DnaCalc\codex-runs\sweep-questions-1\ANSWERS.md`, `Q1-subnormal-inputs.json`) and
  `C:\Work\DnaCalc\codex-runs\sweep-report-2.md` — Excel 16.0 build 20527 subnormal cell-write
  flushing, true computed subnormal `ODD(EXP(-745)) = 1` / `ODD(0-EXP(-745)) = -1`, `±DBL_MAX`
  pass-through, and DNA Calc bank comparison (`c0b94f88ab`, `e042619e35`).
