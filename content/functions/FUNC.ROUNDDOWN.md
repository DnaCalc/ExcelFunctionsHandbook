---
schema: efh.function-page/v1
function_id: FUNC.ROUNDDOWN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft — WorksheetFunction.RoundDown method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.rounddown"
    role: "the documented \"rounds a number down, toward 0 (zero)\" definition and the three num_digits remarks"
  - work: "Microsoft Support — ROUNDDOWN function"
    locator: "https://support.microsoft.com/en-us/office/rounddown-function-2ec94c73-241f-4b01-8c6f-17e6d7968f53"
    role: "the worksheet-surface documentation page named in the projection; not retrievable for this pass"
  - work: "Gay, Correctly rounded binary-decimal and decimal-binary conversions"
    locator: "AT&T Bell Labs Numerical Analysis Manuscript 90-10, 1990"
    role: "the reference algorithm for deciding a decimal digit exactly rather than by scaling"
  - work: "IEEE 754-2019, clause 4.3.2"
    locator: "roundTowardZero"
    role: "the directed rounding attribute this function implements at a decimal place"
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
  - "Excel versions, platforms and CPUs"
  - "Page vocabulary"
  - "Sources"
family: rounddown_fn
role_in_family: "The toward-zero member of the directed-rounding trio; ROUND's floor-side sibling, and TRUNC by another name."
---

# ROUNDDOWN

## What it computes

`ROUNDDOWN(number, num_digits)` rounds toward zero to `num_digits` decimal places.

    ROUNDDOWN(x, d) = sign(x) · ⌊|x| · 10^d⌋ / 10^d

Microsoft's VBA reference defines it by difference: "**RoundDown** behaves like **Round**, except
that it always rounds a number down", where "down" is immediately qualified in the summary line as
"down, toward 0 (zero)". The distinction is essential and is the most common misreading on this
page: **`ROUNDDOWN` is not the floor.** It is truncation. For positive values the two agree; for
negative values they do not. `ROUNDDOWN(-1.7, 0)` is `-1`, while the floor of `-1.7` is `-2`.

The same three remarks as [ROUND](FUNC.ROUND.md) apply to `num_digits`: greater than zero rounds
to that many decimal places, zero rounds to an integer, less than zero rounds to the left of the
decimal point.

**Monotonicity, and the property this function has that `ROUND` does not.** Truncation toward zero
is idempotent and never increases magnitude:

    |ROUNDDOWN(x, d)| ≤ |x|,   and   ROUNDDOWN(ROUNDDOWN(x, d), d) = ROUNDDOWN(x, d)

so a `ROUNDDOWN` result is always a *conservative* estimate of magnitude. That is why it is the
correct choice for anything that must not overstate — available inventory, billable whole units,
capacity that must not be exceeded.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | Any real number to be rounded down. Required. | — |
| `num_digits` | The number of decimal places. Required. | — |

Both are numeric slots under ordinary to-number coercion — see
[Coercion and lifting](../model/02-coercion-and-lifting.md). As with `ROUND`, `num_digits` is
**required**, and the reference engine truncates a non-integral `num_digits` toward zero. Neither
fact is documented.

## Result and edge cases

Returns `Number`.

- **Sign is preserved exactly.** `ROUNDDOWN(-1.7, 0)` is `-1`, not `-2`. This is the whole
  difference from `INT` and `FLOOR.MATH`.
- **There are no ties.** Because the direction is fixed, `ROUNDDOWN` never has to break one — the
  entire tie-rule question that dominates [ROUND](FUNC.ROUND.md) simply does not arise. That makes
  this the *better-behaved* function of the two, and the one whose only remaining hazard is the
  representation issue below.
- **Large and small `num_digits`.** The reference engine returns `number` unchanged once
  `num_digits` reaches 308, and a signed zero once it reaches −308. Implementation limits, not
  documented behaviour.
- **Negative zero.** Rounding a small negative number down to a coarse place produces `−0` in the
  reference engine. It displays as `0`; it is not the same bit pattern. See
  [The value universe](../model/01-value-universe.md).
- **Arrays.** The projection declares a `Custom` coercion profile with `surface_native` lifting.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | Either argument does not convert to a number | shared coercion model |
| propagated | An error value in either argument | shared coercion model |
| `#VALUE!` | Wrong argument count (`num_digits` omitted) | arity, refused at entry |

Microsoft's VBA page states no error condition for this method.

## Relationships

- **`TRUNC`** — the same operation. `TRUNC(x, d)` and `ROUNDDOWN(x, d)` compute the same thing;
  `TRUNC`'s second argument is optional and defaults to zero, while `ROUNDDOWN`'s is required.
  Excel therefore ships two functions for one operation, differing only in argument admission.
  That is worth knowing when reading someone else's workbook: a `TRUNC` and a `ROUNDDOWN` in
  adjacent columns are not doing different things.
- **`INT`** — floors. `INT(-1.7)` is `-2`; `ROUNDDOWN(-1.7, 0)` is `-1`. The two agree on
  non-negative input and diverge on every negative non-integer, which is the classic silent bug in
  a model that changes sign.
- **`FLOOR.MATH`** — rounds toward negative infinity by default, with an explicit `mode` argument
  that switches negative numbers to toward-zero behaviour. `FLOOR.MATH` is the function to reach
  for when the direction must be *stated*; `ROUNDDOWN` is the one where it is baked in.
- **[ROUND](FUNC.ROUND.md)** and **[ROUNDUP](FUNC.ROUNDUP.md)** — the other two directions.
  `ROUNDDOWN(x,d) ≤ ROUND(x,d) ≤ ROUNDUP(x,d)` in magnitude, always, with equality when `x` is
  already a multiple of `10^-d`.
- **`ROUNDDOWN(x, 0) + ROUNDUP(x, 0)`** is not generally `2x`, and `ROUNDDOWN(-x, d)` is
  `-ROUNDDOWN(x, d)` — the function is odd, which `INT` is not. Symmetry about zero is the
  cleanest way to remember which member of the family does what.

## Numerical notes

**The tie problem disappears; the representation problem does not.** `ROUNDDOWN` still has to
decide whether the stored value is above or below a decimal boundary, and the naive
scale-and-truncate route still gets that wrong when the scaling rounds across the boundary. If
`x·10^d` rounds *up* to exactly an integer that the exact product falls short of, truncation keeps
that integer and the result is one unit too large — which for a function whose entire purpose is to
never overstate is the worst possible failure direction. The failure is rare and completely silent.

**The remedy is the same as for `ROUND` and is cheaper here.** Compute the candidate, then check
the sign of the exact residual `|x| − candidate` using a fused multiply-add; if it is negative, the
scaling overshot and the candidate must be decremented by one unit in the last decimal place. There
is no tie case to worry about, so a single correction step is provably sufficient. An implementation
that wants the mathematically correct answer should do exactly this; an implementation targeting
compatibility should not, until Excel's own behaviour on the boundary is pinned.

**Negative `num_digits` is the well-behaved half.** Powers of ten up to 10²² are exactly
representable in binary64, so for `d ≤ 0` the scaling factor `10^-d` is exact and only the division
and truncation can misbehave. For `d > 0` the factor `10^d` is exact too (same reason, up to 22),
but the *target grid* — multiples of `10^-d` — is not representable at all, so the returned value
is the nearest binary64 to the truncated decimal rather than the truncated decimal itself. The
function cannot return what its own specification names, and this is unavoidable in binary
arithmetic.

**Repeated application is safe.** Unlike the round-to-nearest case, where chained rounding at
different precisions can drift upward, truncation is monotone and idempotent, so
`ROUNDDOWN(ROUNDDOWN(x, 4), 2) = ROUNDDOWN(x, 2)` exactly. That property makes `ROUNDDOWN`
composable in a way `ROUND` is not, and it is worth exploiting in staged calculations.

## What has not been checked

No Handbook vector suite exists for `ROUNDDOWN`, and **no evidence record lists this surface among
its subjects**. `ROUNDDOWN` appears inside the group membership of an upstream array-lift
measurement (`EV-STRUCT-0011`) whose subjects are other surfaces; that record's reader warning
forbids reading its group total as any individual surface's rate. The honest statement is
therefore: **the rounding family was measured as a group for structural array admission; this
surface was not measured separately, and its arithmetic was not measured at all.**

The presence projection records the defect stream `BUG-FUNC-017` against this module.

Inputs I would probe first:

1. **`ROUNDDOWN(-1.7, 0)` beside `INT(-1.7)` and `TRUNC(-1.7)`** — three cells that settle the
   toward-zero-versus-floor question and confirm the `TRUNC` identity in the same glance.
2. **A boundary case engineered to expose scaling overshoot**, such as `ROUNDDOWN(0.29, 1)` and
   `ROUNDDOWN(4.35, 1)` and `ROUNDDOWN(1.4999999999999998, 0)` — inputs whose exact product with
   the scale sits just below an integer. If any of them returns a value one unit high, the
   implementation is scaling without correction.
3. **`ROUNDDOWN(x, d)` against `TRUNC(x, d)` swept over a few thousand random inputs and
   `d ∈ [-5, 5]`** — the cheapest way to establish, or refute, that the two are the same function.
4. **`ROUNDDOWN(-0.4, 0)`** and its reciprocal — the negative-zero question.
5. **`ROUNDDOWN(1.5, 0.9)`** — non-integral `num_digits`, truncated in the reference engine and
   undocumented.
6. **`ROUNDDOWN(x, 400)` and `ROUNDDOWN(x, -400)`** — the clamp boundaries.
7. **Array arguments in both slots**, given the array-lift defect stream on this module.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `ROUNDDOWN` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-ROUND-002: The rounding functions read 15 digits first, so ROUNDDOWN can round up

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.* Applies to ROUND, ROUNDDOWN, ROUNDUP, TRUNC.

Before choosing a digit these functions read the number to 15 significant digits on the x87
unit: with e the decimal exponent and n = 14 - e, y = RN64(|x| * 10^n), or RN64(|x| / 10^-n)
when n is negative, while 10^|n| is exact in 64 bits (|n| <= 27). A half in y goes up for
ROUNDDOWN, ROUNDUP and TRUNC and down for ROUND. So digits beyond the 15th are lost even when
the requested precision keeps them: ROUND(-1594628480062905,14) loses its last digit, and
ROUNDDOWN(283025097725723.5,0) returns 283025097725724, larger than its input.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `ROUND(-1594628480062905, 14)` | number `-1594628480062905` (`0xc316a939806956e4`); num_digits `14` (`0x402c000000000000`) (cells) | `-1594628480062900` (`0xc316a939806956d0`) | `-1594628480062905` (`0xc316a939806956e4`) | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |
| `ROUNDDOWN(283025097725723.5, 0)` | number `283025097725723.5` (`0x42f0168ea69971b8`); num_digits `0` (`0x0000000000000000`) (cells) | `283025097725724` (`0x42f0168ea69971c0`) | `283025097725723` (`0x42f0168ea69971b0`) | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |

Correct value: An integer rounded to 14 places is itself.
Correct value: ROUNDDOWN drops the fraction .5.

Basis (measured): Under the family rules the recorded rows match ROUND 36,821 of 36,945,
ROUNDUP 52,022 of 52,841, ROUNDDOWN 59,182 of 59,995 and TRUNC 50,504 of 51,317, with no row
lost and each remaining failure classified in the write-up.

- Open: How the scale is formed for |n| > 27: a single correctly rounded 64-bit power and the chained multiplications tried do not fit; 123 ROUND, 548 ROUNDUP, 556 ROUNDDOWN and 555 TRUNC rows remain.
- Open: The reading step itself: one control at n = -10 misses the single RN64 step by just over half a unit, so that step is an approximation of the real one even for |n| <= 27. It stays because it gains rows and loses none.

Evidence:

- DnaCalc `crates/calc-func/src/catalogue/rounding.rs` at `ab94fb91a5` — the rules in the product; the commit message carries the row counts
- [`content/notes/evidence/2026-11-04/rounding-EVIDENCE.md`](../../content/notes/evidence/2026-11-04/rounding-EVIDENCE.md) — research write-up: full-bank result, the rules, what stays open
- OxFunc-Evidence `cache/oracle-20430/build-20430/ROUND.jsonl.gz`, line 3758, key ROUND|0xc316a939806956e4|0x402c000000000000 at `e26b7bddb8` (readings)
- OxFunc-Evidence `cache/oracle-20430/build-20430/ROUNDDOWN.jsonl.gz`, line 753, key ROUNDDOWN|0x42f0168ea69971b8|0x0000000000000000 at `e26b7bddb8` (readings)

### FN-ROUND-003: Near DBL_MAX, ROUND returns NaN bit patterns that Excel treats as numbers

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.* Applies to ROUND, ROUNDDOWN, ROUNDUP, TRUNC.

ROUND of a number near 1.8E308 can return a binary64 with an all-ones exponent field. A decimal
result of 2^1024 or more keeps its 53-bit significand, and its exponent is carried into the
exponent field without a check, so the bits are a NaN. ROUND, ROUNDDOWN and TRUNC return those
bits; ROUNDUP gives #NUM!. The recorded rows hold 430 such ROUND rows, 102 ROUNDDOWN and 95
TRUNC, and all 627 follow this. Even ROUND(DBL_MAX,0), whose answer is DBL_MAX, returns
0x7ff000000000000a. The cell shows 2E+308 or 1.8E+308, and a pattern with its sign bit set
shows 2E+308 too, with no minus. ISNUMBER gives TRUE, ISERROR FALSE, TYPE 1, ERROR.TYPE #N/A;
A1=A1 and A1>0 are TRUE. =A1, N(A1), MAX(A1), ABS(A1) and IFERROR(A1,...) pass the pattern on,
as does =-A1 with its sign unchanged. Arithmetic on it (A1+0, A1*1, A1-A1, SUM, ROUND) gives
#NUM!, and TEXT(A1,"0") gives #VALUE!. Saved to .xlsx, each such cell is written as the error
#NUM!; reopened, the cells read #NUM! until a full recalculation brings the patterns back.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `ROUND(1.7976931348623157E+308, 0)` | number `1.7976931348623157e+308` (`0x7fefffffffffffff`); num_digits `0` (`0x0000000000000000`) (cells) | `NaN bit pattern, shown as 1.8E+308` (`0x7ff000000000000a`) | `1.7976931348623157e+308` (`0x7fefffffffffffff`) | Excel 16.0 build 20527, Windows; bitness and CPU not recorded in this reading |
| `ROUND(1.7976931348623157E+308, -307)` | number `1.7976931348623157e+308` (`0x7fefffffffffffff`); num_digits `-307` (`0xc073300000000000`) (cells) | `NaN bit pattern, shown as 1.8E+308` (`0x7ff005419221015d`) | `#NUM!` | Excel 16.0 build 20527, Windows; bitness and CPU not recorded in this reading |
| `ROUND(1.7976931348623157E+308, -308)` | number `1.7976931348623157e+308` (`0x7fefffffffffffff`); num_digits `-308` (`0xc073400000000000`) (cells) | `NaN bit pattern, shown as 2E+308` (`0x7ff1ccf385ebc8a0`) | `#NUM!` | Excel 16.0 build 20527, Windows; bitness and CPU not recorded in this reading |

Correct value: DBL_MAX is an integer, so rounding it to 0 places returns it.
Correct value: The rounded value exceeds the largest binary64; Excel reports overflow in its
own arithmetic as #NUM!.

Basis (measured): One Excel workbook read cell by cell through Value2 on 2026-11-04, saved,
reopened and recalculated; rows from the rounding write-up, 627 of 627.

Inferred, not measured: That one decimal-to-binary conversion routine is the source of all
these patterns is inferred from the exponent-overflow encoding.

- Open: How these patterns sort among themselves and with other numbers, and display width, are not recorded.

Evidence:

- DnaCalc `crates/calc-func/src/catalogue/rounding.rs` at `ab94fb91a5` — the rules in the product; the commit message carries the row counts
- [`content/notes/evidence/2026-11-04/round-nan-downstream.json`](../../content/notes/evidence/2026-11-04/round-nan-downstream.json), cells A1..A4, B1..B24, reopened, recalculated — cell-by-cell Excel reading of the NaN patterns
- [`content/notes/evidence/2026-11-04/round-nan-downstream.ps1`](../../content/notes/evidence/2026-11-04/round-nan-downstream.ps1) — the script that took the reading
- [`content/notes/evidence/2026-11-04/round-nan-downstream-sheet1.xml`](../../content/notes/evidence/2026-11-04/round-nan-downstream-sheet1.xml) — the sheet as Excel saved it
- [`content/notes/evidence/2026-11-04/rounding-EVIDENCE.md`](../../content/notes/evidence/2026-11-04/rounding-EVIDENCE.md) — research write-up: full-bank result, the rules, what stays open
- [`content/notes/evidence/2026-11-04/round-nan-downstream.json`](../../content/notes/evidence/2026-11-04/round-nan-downstream.json), cells.A3 (readings)
- [`content/notes/evidence/2026-11-04/round-nan-downstream.json`](../../content/notes/evidence/2026-11-04/round-nan-downstream.json), cells.A2 (readings)
- [`content/notes/evidence/2026-11-04/round-nan-downstream.json`](../../content/notes/evidence/2026-11-04/round-nan-downstream.json), cells.A1 (readings)

### FN-ROUNDUP-001: ROUNDUP, ROUNDDOWN and TRUNC read num_digits from its low 16 bits

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.* Applies to ROUNDDOWN, ROUNDUP, TRUNC.

ROUNDUP, ROUNDDOWN and TRUNC take the digit count from the low 16 bits of the integer part of
|num_digits|, with the sign of num_digits. If |num_digits| is 2^32 or more, or those low bits
are 32768 or more, the truncation happens at 100 places (10^-100, or 10^100 for a negative
count). So 65536 acts as 0, 65537 as 1, 32768 to 65535 as 100, and -2147483748, whose low 16
bits are 100, as -100. In the out-of-range case ROUNDUP adds no unit for a positive count, and
gives #NUM! for a negative count when a digit is dropped.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `ROUNDUP(1.234567890123456e-200, 65536)` | number `1.234567890123456e-200` (`0x166e3d71f622be8f`); num_digits `65536` (`0x40f0000000000000`) (cells) | `1` (`0x3ff0000000000000`) | `1.234567890123456e-200` (`0x166e3d71f622be8f`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `ROUNDUP(1.234567890123456e-200, 65537)` | number `1.234567890123456e-200` (`0x166e3d71f622be8f`); num_digits `65537` (`0x40f0001000000000`) (cells) | `0.1` (`0x3fb999999999999a`) | `1.234567890123456e-200` (`0x166e3d71f622be8f`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `ROUNDUP(1.234567890123456e-200, 32768)` | number `1.234567890123456e-200` (`0x166e3d71f622be8f`); num_digits `32768` (`0x40e0000000000000`) (cells) | `0` (`0x0000000000000000`) | `1.234567890123456e-200` (`0x166e3d71f622be8f`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `TRUNC(1.234567890123456e-200, 65536)` | number `1.234567890123456e-200` (`0x166e3d71f622be8f`); num_digits `65536` (`0x40f0000000000000`) (cells) | `0` (`0x0000000000000000`) | `1.234567890123456e-200` (`0x166e3d71f622be8f`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |

Correct value: The exact decimal expansion of this binary64 number has fewer than 32768 places,
so a truncation at num_digits places returns the number itself.

Basis (measured): Under the family rules the recorded rows match ROUND 36,821 of 36,945,
ROUNDUP 52,022 of 52,841, ROUNDDOWN 59,182 of 59,995 and TRUNC 50,504 of 51,317, with no row
lost and each remaining failure classified in the write-up. All 13,914 informative ROUNDDOWN
and TRUNC rows with large num_digits follow the rule.

Evidence:

- DnaCalc `crates/calc-func/src/catalogue/rounding.rs` at `ab94fb91a5` — the rules in the product; the commit message carries the row counts
- [`content/notes/evidence/2026-11-04/rounding-EVIDENCE.md`](../../content/notes/evidence/2026-11-04/rounding-EVIDENCE.md) — research write-up: full-bank result, the rules, what stays open
- OxFunc-Evidence `runs/w111-rounding-count-refinement-20260929/answers-roundup.json.gz`, w111roundcount-ROUNDUP-00041 at `e26b7bddb8` (readings)
- OxFunc-Evidence `runs/w111-rounding-count-refinement-20260929/answers-roundup.json.gz`, w111roundcount-ROUNDUP-00043 at `e26b7bddb8` (readings)
- OxFunc-Evidence `runs/w111-rounding-count-refinement-20260929/answers-roundup.json.gz`, w111roundcount-ROUNDUP-00033 at `e26b7bddb8` (readings)
- OxFunc-Evidence `runs/w111-rounding-count-refinement-20260929/answers-trunc.json.gz`, w111roundcount-TRUNC-00041 at `e26b7bddb8` (readings)

### FN-TRUNC-001: TRUNC of DBL_MIN returns a subnormal, not 0

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.* Applies to ROUNDDOWN, ROUNDUP, TRUNC.

With DBL_MIN (0x0010000000000000) as number, TRUNC, ROUNDDOWN and ROUNDUP return the subnormal
0x000ffffffffffffa at each num_digits recorded, so TRUNC(DBL_MIN,0) is not 0. That value is 6
steps below DBL_MIN and 3 steps below 0x000ffffffffffffd, the binary64 nearest the 15-digit
reading 2.22507385850720E-308. The recorded rows hold 256 such cases for each of the three
functions. ROUND gives 0 on the same inputs.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `TRUNC(2.2250738585072014e-308, 0)` | number `2.2250738585072014e-308` (`0x0010000000000000`); num_digits `0` (`0x0000000000000000`) (cells) | `2.2250738585071984e-308` (`0x000ffffffffffffa`) | `0` (`0x0000000000000000`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `TRUNC(2.2250738585072014e-308, 1)` | number `2.2250738585072014e-308` (`0x0010000000000000`); num_digits `1` (`0x3ff0000000000000`) (cells) | `2.2250738585071984e-308` (`0x000ffffffffffffa`) | `0` (`0x0000000000000000`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |

Correct value: DBL_MIN is below 1, so truncating it at 0 or 1 places gives 0.

Basis (measured): Recorded rows of the broad W111 capture; counts from the rounding write-up.

- Open: Why this one input converts 3 steps below nearest.

Evidence:

- [`content/notes/evidence/2026-11-04/rounding-EVIDENCE.md`](../../content/notes/evidence/2026-11-04/rounding-EVIDENCE.md) — research write-up: full-bank result, the rules, what stays open
- OxFunc-Evidence `runs/w111-broad-20260929/answers/answers-trunc.json.gz`, w111broad-20260929-TRUNC-00026 at `e26b7bddb8` (readings)
- OxFunc-Evidence `runs/w111-broad-20260929/answers/answers-trunc.json.gz`, w111broad-20260929-TRUNC-00027 at `e26b7bddb8` (readings)

<!-- efh:notes end -->

## Excel versions, platforms and CPUs

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.ROUNDDOWN.json; edits here are overwritten -->

Readings of ROUNDDOWN with a recorded Excel build: 60,107 input(s) over 3 recorded
environment(s), Excel 16.0 build(s) 19929, 20430, all on Windows; recorded CPUs: AMD Zen 2. 1
same-input comparison(s) between environments over 3 input-pair(s): no result differs. No
reading from a Mac, Excel for the web, 32-bit Excel or an Intel CPU is recorded.

Readings of `ROUNDDOWN` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 112 | date not recorded |
| 16.0 build 20430 | Windows, 64-bit | AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | 3,454 | 2026-09-25 |
| 16.0 build 20430 | Windows, 64-bit | CPU not recorded | 56,541 | 2026-09-29 |

Compared on the same inputs:

| Check | Differs in | Environments | Inputs compared | Inputs with different results |
|---|---|---|---:|---:|
| XC-ROUNDDOWN-001 | excel-build | 16.0 build 19929 vs 16.0 build 20430 | 3 | 0 |

Differences observed: none.

Evidence for every row: [`content/evidence/environments/FUNC.ROUNDDOWN.json`](../../content/evidence/environments/FUNC.ROUNDDOWN.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| toward zero | Truncation: the magnitude never increases and the sign never changes |
| floor | Rounding toward negative infinity, which this function does **not** do |
| idempotent | Applying the operation twice gives the same result as applying it once |
| scaling overshoot | The rounded product `x·10^d` crossing an integer the exact product does not reach |

## Sources

- Microsoft, "WorksheetFunction.RoundDown method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.rounddown> ("Rounds a
  number down, toward 0 (zero)", the behaves-like-Round sentence, and the three `num_digits`
  remarks; no error conditions are stated).
- Microsoft Support, "ROUNDDOWN function" —
  <https://support.microsoft.com/en-us/office/rounddown-function-2ec94c73-241f-4b01-8c6f-17e6d7968f53>
  (retrieval refused with HTTP 403 during this pass; nothing here is sourced from it).
- Gay, "Correctly rounded binary-decimal and decimal-binary conversions", Bell Labs NAM 90-10,
  1990; IEEE 754-2019 clause 4.3.2 (`roundTowardZero`).
- Handbook, [ROUND](FUNC.ROUND.md) — the shared decimal-rounding analysis;
  [The value universe](../model/01-value-universe.md);
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.ROUNDDOWN.json` (`Custom` coercion,
  `precision_rounding_profile=default`) and `data/presence/FUNC.ROUNDDOWN.json` (own module;
  defect stream `BUG-FUNC-017`).
