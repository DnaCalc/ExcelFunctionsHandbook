---
schema: efh.function-page/v1
function_id: FUNC.QUOTIENT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-STRUCT-0007
open_problems: []
references:
  - work: "Microsoft — WorksheetFunction.Quotient method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.quotient"
    role: "documented signature, the \"integer portion of a division\" wording, and the single stated error condition"
  - work: "Microsoft Support — QUOTIENT function"
    locator: "https://support.microsoft.com/en-us/office/quotient-function-9f7bf099-2a18-4282-8fa4-65290cc99dee"
    role: "the worksheet-surface documentation page named in the projection; not retrievable for this pass"
  - work: "Boute, The Euclidean definition of the functions div and mod"
    locator: "ACM TOPLAS 14(2), 1992"
    role: "the taxonomy of truncated, floored and Euclidean division that this page's sibling mismatch turns on"
  - work: "OxFunc — EXCEL_MATH_DEVIATION_CATALOG.md"
    locator: "docs/EXCEL_MATH_DEVIATION_CATALOG.md entry XMD-006"
    role: "the observed quotient-magnitude limit on MOD, which QUOTIENT does not share"
episodes: []
body_sections:
  - "What it computes"
  - "Arguments"
  - "Result and edge cases"
  - "Errors"
  - "Relationships"
  - "Numerical notes"
  - "What has not been checked"
  - "Notes"
  - "Page vocabulary"
  - "Sources"
family: quotient_fn
role_in_family: "The truncating integer division; the only member of its module, and deliberately not the partner of MOD."
---

# QUOTIENT

## What it computes

`QUOTIENT(numerator, denominator)` returns the integer part of `numerator / denominator`,
discarding the remainder.

    QUOTIENT(n, d) = trunc(n / d)

**Truncation, not floor.** `trunc` rounds toward zero, so the sign of the numerator survives:
`trunc(-7/3) = -2`, whereas `⌊-7/3⌋ = -3`. This single choice is the whole content of the
function, and it is the source of the trap in the next section.

Domain: all real `n`, and `d ≠ 0`. Range: the integers representable as binary64 — which is to
say, exact integers up to 2⁵³ and even integers thereafter.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `numerator` | The dividend. Required. | — |
| `denominator` | The divisor. Required. | — |

Both are ordinary numeric slots — see
[Coercion and lifting](../model/02-coercion-and-lifting.md). Microsoft's VBA page names both as
`Variant`, which is consistent with text and logicals converting; the same page states the single
documented error condition, quoted under *Errors* below.

Nothing in the documentation requires either argument to be an integer, and neither is truncated
before the division: the truncation happens once, to the quotient.

## Result and edge cases

Returns `Number`, always an integral value (or an error).

- **Signs.** All four sign combinations truncate toward zero: `QUOTIENT(7,3)` and
  `QUOTIENT(-7,-3)` are `2`; `QUOTIENT(-7,3)` and `QUOTIENT(7,-3)` are `-2`.
- **`denominator = 0`** → `#DIV/0!`. On Excel 16.0 build 20527, writing a subnormal binary64
  value (`±5e-324` through `±(2^-1022 - 5e-324)`) to a cell via `Range.Value2` flushes the cell
  to `+0`, so a cell-written subnormal `denominator` returns `#DIV/0!` (including `0 / 0`), while
  a cell-written subnormal `numerator` with nonzero `denominator` returns `0`
  ([sweep-questions-1-ANSWERS.md](../notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md),
  `C:\Work\DnaCalc\codex-runs\sweep-questions-1\Q1-subnormal-inputs.json`). When a true computed
  subnormal reaches `QUOTIENT` as the numerator in a formula (`x` from `EXP(-745)`, `EXP(-730)`,
  or `EXP(-709)`), `QUOTIENT(±x, 1)` truncates to `0` ([FN-ABS-002](../notes/FN-ABS-002.json)).
- **Non-integer arguments** are accepted and are not pre-rounded: `QUOTIENT(7.9, 2)` is `3`,
  because `7.9/2 = 3.95` truncates to `3`.
- **Very large quotients.** Once `|n/d| ≥ 2⁵³` every binary64 value is already integral, so
  truncation is a no-op and the result is whatever the division rounded to. The reference engine
  declares no domain guard here, which is a deliberate contrast worth recording — see
  *Relationships*.
- **Arrays.** The projection declares `UnaryNumericScalarOnly` coercion with `surface_native`
  lifting; `EV-STRUCT-0007` is a structural array-admission record that names this surface among
  its subjects.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | Either argument is nonnumeric | Microsoft VBA page, verbatim: "If either argument is nonnumeric, **Quotient** returns the #VALUE! error value" |
| `#DIV/0!` | `denominator` is zero | reference engine; **not documented** |
| propagated | An error value in either argument | shared coercion model |

## Relationships

- **`MOD` — and this is the important one.** In most languages `div` and `mod` are a matched pair
  satisfying `n = d·div(n,d) + mod(n,d)`. **In Excel they are not a pair.** `QUOTIENT` truncates
  toward zero; Excel's `MOD` is documented to take the sign of the divisor, which is the *floored*
  convention. With `n = -7`, `d = 3`: `QUOTIENT` gives `-2`, `MOD` gives `2`, and
  `3·(-2) + 2 = -4 ≠ -7`. The identity fails on every input where the operands have opposite
  signs. Boute's TOPLAS paper is the standard taxonomy of exactly this mismatch; Excel is one of
  the systems it was written about. The matching partner of Excel's `MOD` is `INT(n/d)`, not
  `QUOTIENT(n,d)`.
- **`INT`** — `INT` floors, `TRUNC` truncates, and `QUOTIENT` truncates a quotient. For positive
  operands all three agree, which is why the difference is usually discovered in production.
- **`TRUNC`** — `QUOTIENT(n,d)` and `TRUNC(n/d)` are the same computation, with one difference
  that matters: `n/d` written as an expression yields `#DIV/0!` on a zero divisor from the
  operator, whereas `QUOTIENT` yields it from the function. There is no accuracy difference; both
  form the quotient in binary64 first.
- **A sibling asymmetry the projection makes visible.** OxFunc's catalogue entry **XMD-006**
  records live Excel refusing `MOD` once the *quotient* magnitude reaches about 1.1259×10¹²,
  returning `#NUM!` — a guard on the quotient, not on the operands. `QUOTIENT` forms the same
  quotient and the projection declares `arg_domain_guard=none` for it. Two functions computing the
  same intermediate, one guarded and one not, is exactly the kind of thing this Handbook exists to
  publish; whether Excel itself guards `QUOTIENT` is unknown here and is on the probe list.

## Numerical notes

`QUOTIENT` looks arithmetic-free and is not.

**The division rounds before the truncation.** `n/d` is computed in binary64 and rounded to
nearest; only then is the fractional part discarded. When the exact quotient sits just below an
integer, the rounding can carry it up to that integer, and truncation then returns a value one
larger than the true integer part. This is not hypothetical: it is the standard failure mode of
every `trunc(a/b)` integer division in floating point, and it is why languages with exact integer
types do not implement `div` this way. The magnitude of the exact quotient does not have to be
large — it only has to be within half an ulp of an integer, which happens for perfectly ordinary
decimal inputs whose binary representations are inexact.

**The remedy, for an implementation that wants the true integer part**, is to compute the
candidate `q = trunc(fl(n/d))` and then correct it using an exactly computed residual: the
product `d·q` is exact whenever `q` is small enough, and `fma(-d, q, n)` gives the residual
without rounding error, so the sign of the residual says whether `q` overshot. Two corrections
suffice. This is the same technique used to make `fmod` exact, and it is cheap.

**Above 2⁵³ the question changes character.** There, the quotient's binary64 representation has no
fractional bits at all, so "the integer part" is a property of the *rounded* quotient rather than
of the exact one, and no post-correction recovers what was lost in the operands. An implementation
should decide whether it is computing `trunc` of the *real* quotient or of the *floating-point*
quotient, and say which. The two are different functions.

**Exactness where it does hold.** When `d` is a power of two, or when `n` and `d` are both exact
integers below 2⁵³, `n/d` is either exact or has a well-behaved error and the naive route is
correct. The overwhelming majority of spreadsheet uses live in that region, which is why the
hazard above is rarely seen and, when seen, is disbelieved. In DNA Calc
(`crates/calc-func/src/catalogue/math.rs`, commit `e042619e35426ffac3d8cfaaa2dd2961abe3d02c`,
and `crates/calc-func/src/witness_exec.rs`, commit `c0b94f88ab4e3df5b518329d08ea03cf481abdbe`),
`QUOTIENT` accepts true subnormal numerators while cell-input replay flushes subnormal
`Range.Value2` cell writes to `+0`, matching all 1,362 of 1,362 recorded `QUOTIENT` bank rows
(`C:\Work\DnaCalc\codex-runs\sweep-report-2.md`).

## What has not been checked

No Handbook vector suite exists for `QUOTIENT`. One evidence record, `EV-STRUCT-0007`, lists this
surface among its subjects: it is a structural array-lift and coercion resweep against live Excel.
Its own reader warning is explicit that the record is about argument shape, coercion and error
placement and **establishes nothing about this surface's numeric results** — and that the group
total is shared across roughly twenty surfaces, so no per-surface figure exists. The record's
membership list is open-ended in its source, which the record also says.

So: argument shape has upstream evidence, the arithmetic has none, and the zero-divisor error code
is undocumented.

Inputs I would probe first:

1. **All four sign combinations** of `QUOTIENT(±7, ±3)`, alongside `MOD(±7, ±3)` in adjacent
   cells. One block of eight cells settles both the truncation convention and the fact that the
   two functions do not reconstruct their operand.
2. **`QUOTIENT(1, 0)` and `QUOTIENT(0, 0)`** — the undocumented error code, and whether the second
   differs from the first.
3. **A quotient engineered to sit just below an integer**, such as `QUOTIENT(0.1+0.2, 0.1)` and
   `QUOTIENT(3*0.1, 0.1)`, where the exact answer is 2 or 3 depending on which side the rounding
   lands. This is the cheapest test of whether Excel post-corrects.
4. **`QUOTIENT(1125900000000, 1)` and its immediate neighbour**, the boundary at which `MOD` is
   recorded to fail. If `QUOTIENT` is guarded at the same place, the guard belongs to the shared
   quotient formation rather than to `MOD`.
5. **`QUOTIENT(2^53+1, 1)`** and nearby, where the operand itself is no longer representable.
6. **Text and logical arguments** in each slot, against the documented "either argument is
   nonnumeric" rule — in particular whether `"7"` counts as nonnumeric, which the wording leaves
   open.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `QUOTIENT` that an implementer matching Excel, or a reader explaining an
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

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| truncating division | Integer division rounding toward zero |
| floored division | Integer division rounding toward negative infinity; the convention Excel's `MOD` uses |
| division identity | `n = d·div(n,d) + mod(n,d)`, which `QUOTIENT` and `MOD` do not jointly satisfy |
| residual correction | Recovering the exact remainder with a fused multiply-add to fix a rounded quotient |

## Sources

- Microsoft, "WorksheetFunction.Quotient method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.quotient> (the
  "integer portion of a division" description and the single `#VALUE!` condition; no zero-divisor
  case is stated).
- Microsoft Support, "QUOTIENT function" —
  <https://support.microsoft.com/en-us/office/quotient-function-9f7bf099-2a18-4282-8fa4-65290cc99dee>
  (retrieval refused with HTTP 403 during this pass; nothing here is sourced from it).
- OxFunc `docs/EXCEL_MATH_DEVIATION_CATALOG.md` entry XMD-006 — the observed `MOD` quotient-
  magnitude limit that `QUOTIENT` does not declare.
- Boute, "The Euclidean definition of the functions div and mod", ACM TOPLAS 14(2), 1992 — the
  truncated/floored/Euclidean taxonomy.
- Handbook evidence record `EV-STRUCT-0007`, rendered beside this page with its own reader
  warning.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.QUOTIENT.json` (`UnaryNumericScalarOnly`,
  `arg_domain_guard=none`) and `data/presence/FUNC.QUOTIENT.json` (own module; defect stream
  `BUG-FUNC-028`).
- [sweep-questions-1-ANSWERS.md](../notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md)
  (`C:\Work\DnaCalc\codex-runs\sweep-questions-1\ANSWERS.md`, `Q1-subnormal-inputs.json`) and
  `C:\Work\DnaCalc\codex-runs\sweep-report-2.md` — Excel 16.0 build 20527 subnormal cell-write
  flushing, true computed subnormal `QUOTIENT(±EXP(-745), 1) = 0`, and DNA Calc bank comparison
  (`c0b94f88ab`, `e042619e35`).
