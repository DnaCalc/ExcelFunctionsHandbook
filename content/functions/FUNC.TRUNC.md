---
schema: efh.function-page/v1
function_id: FUNC.TRUNC
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — TRUNC function"
    locator: "https://support.microsoft.com/en-us/office/trunc-function-8b86a64c-3127-43db-ba14-aa5ceb292721"
    role: "documented signature, the num_digits default of 0, and the stated contrast with INT"
  - work: "IEEE 754-2019, Standard for Floating-Point Arithmetic"
    locator: "roundToIntegralTowardZero; the five rounding-direction attributes"
    role: "the standard name for this rounding mode and its distinction from the others"
  - work: "Goldberg, What Every Computer Scientist Should Know About Floating-Point Arithmetic"
    locator: "sections on rounding and on decimal conversion"
    role: "why scaled decimal truncation is not exact and where the representable-decimal problem comes from"
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
  - Page vocabulary
  - Sources
family: trunc_fn
role_in_family: >-
  The sole member: rounding toward zero, with an optional decimal-place argument that turns a
  clean integral operation into a scaled one.
---

## What it computes

`TRUNC(number, [num_digits])` discards digits beyond a given decimal position, rounding **toward
zero**.

With `num_digits = 0` — the documented default — the mathematical statement is

    TRUNC(x) = sgn(x) * floor(|x|)

which is IEEE 754's `roundToIntegralTowardZero`. With a general `d`, the intended value is

    TRUNC(x, d) = sgn(x) * floor(|x| * 10^d) / 10^d

so positive `d` keeps `d` decimal places and negative `d` zeroes out digits to the left of the
decimal point.

- **Domain**: all real numbers, for both arguments.
- **Range**: for `d = 0`, the integers; in general, the multiples of `10^-d` — except that most
  of those multiples are not representable in binary64, which is the whole numerical content of
  this page.
- **Shape**: a monotone non-decreasing step function, constant on each half-open interval
  between consecutive multiples of `10^-d`, with the intervals oriented **away from zero on both
  sides** — `[0, s)` maps to `0` and `(-s, 0]` maps to `0`, so the step at the origin is twice
  as wide as every other step. That double-width step at zero is the geometric statement of
  "toward zero" and it is what distinguishes this function from `INT` and from `FLOOR`.
- **Identities**: `TRUNC(x) = INT(x)` for `x >= 0`; `TRUNC(-x) = -TRUNC(x)` (odd, unlike `INT`);
  `x = TRUNC(x) + frac(x)` where `frac` has the same sign as `x`.

Microsoft's page states the contrast directly: "TRUNC simply removes the fractional part of the
number, whereas INT returns the nearest lower integer (not necessarily smaller in magnitude)",
with the worked case that `TRUNC(-4.3)` is `-4` while `INT(-4.3)` is `-5`.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The number to truncate. Required. | — |
| `num_digits` | The precision of the truncation. Optional. | `0` |

The reference engine records an arity of one to two, consistent with the documented optional
second argument.

The commonly misunderstood position is `num_digits`, in two ways:

1. **Negative values are meaningful.** `TRUNC(1234, -2)` truncates to hundreds. Readers who
   assume the argument is a count of decimals in the ordinary sense often do not realise the
   left-hand side is reachable.
2. **It is not a rounding argument.** `ROUND` and `TRUNC` take the same-shaped second argument
   and do different things with it; a formula ported from one to the other changes behaviour
   silently on exactly half the inputs.

Whether `num_digits` is itself truncated when non-integral is not stated on Microsoft's page.
Excel takes the integer part of `|num_digits|`, with its sign, and reads only the low 16 bits of
that (FN-ROUNDUP-001).

## Result and edge cases

Returns `Number`.

- **Negative numbers** truncate toward zero — the documented `TRUNC(-4.3) = -4`. This is the
  single most important edge and the reason the function exists alongside `INT`.
- **Values already integral** are returned unchanged up to 15 significant digits; digits beyond
  the 15th are lost in the reading that comes first (FN-ROUND-002).
- **Zero** returns zero. Whether a negative input in `(-1, 0)` returns `+0` or `-0` is
  unspecified by the documentation and is on the probe list; the mathematical answer is `0` and
  IEEE's `roundToIntegralTowardZero` preserves the sign of the operand, which would give `-0`.
- **Very large numbers.** Above `2^52` every binary64 value is already an integer, so `TRUNC`
  with `d = 0` is mathematically the identity there, subject to that reading in Excel. For positive
  `d` the scaling `10^d` can overflow for large
  `d`; for negative `d` and large `|x|` the same is true in the other direction.
- **The general `d` case is not exact.** `10^-d` is not representable for any `d` except zero
  and small negative values, so `TRUNC(x, d)` for `d > 0` cannot in general return a number
  whose decimal expansion terminates where the caller asked. The returned double is the nearest
  representable neighbour of the intended decimal value. This is not an implementation defect;
  it is a property of binary floating point. It does mean that `TRUNC(x, 2)` followed by a
  comparison against a literal with two decimals is not a reliable equality test.

The reference engine classifies `TRUNC` with `kernel_signature_class: NumsToNum`,
`coercion_lift_profile: Custom` and `error_collapse_profile: None`, and its projected
`real_result_policy` reads `arg_domain_guard=none` with `non_finite=allow` — no domain
restriction, consistent with a function that is total on the reals.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | Either argument does not convert to a number | Shared coercion rules |
| propagated | An error value in either argument is returned unchanged | Shared coercion rules |

Microsoft's page lists no error conditions for `TRUNC`, and the function has no domain
restriction. What happens when the internal scaling by `10^d` overflows is not documented.

## Relationships

- **`INT`** — the documented contrast. `INT` rounds toward negative infinity (the floor);
  `TRUNC` rounds toward zero. They agree on non-negative numbers and differ on every negative
  non-integer. `INT` also has no second argument.
- **`ROUND`, `ROUNDDOWN`, `ROUNDUP`** — the decimal-place family. **`ROUNDDOWN` is the
  near-duplicate**: rounding "down" in Excel's naming means toward zero, not toward negative
  infinity, so `ROUNDDOWN(x, d)` and `TRUNC(x, d)` are the same operation under two names. The
  documented difference is that `TRUNC`'s second argument is optional and `ROUNDDOWN`'s is not.
  A reader choosing between them is choosing a name, not a behaviour.
- **`FLOOR.MATH` / `CEILING.MATH`** — the significance-based family, which rounds to a multiple
  of an arbitrary step rather than a power of ten, and which carries an explicit mode argument
  for the negative direction. Where `TRUNC`'s negative-number convention is fixed,
  `FLOOR.MATH`'s is a parameter.
- **`MOD` and `QUOTIENT`** — `QUOTIENT` is integer division and is `TRUNC(a/b)` in intent;
  `MOD`'s sign convention follows the floor, not the truncation, which is why `MOD` and
  `QUOTIENT` do not compose the way a C programmer expects.
- **`SIGN`** — the parity relation `TRUNC(-x) = -TRUNC(x)` is exactly the statement that `TRUNC`
  commutes with `SIGN`.
- **Confused with**: `INT` (the documented case), `ROUND` (which rounds to nearest, not toward
  zero), and cell *formatting*, which changes display without changing the stored value.

## Numerical notes

The `d = 0` case is trivial in IEEE 754: rounding toward zero is one of its five rounding
directions, hardware implements it directly, and there is no error. Excel does not take that route:
`TRUNC(DBL_MIN, 0)` returns a subnormal, not 0 (FN-TRUNC-001), and digits beyond the 15th are lost
at `d = 0` too (FN-ROUND-002). Everything below concerns the general case.

**The scaling problem.** The obvious implementation of `TRUNC(x, d)` is
`trunc(x * 10^d) / 10^d`. It has three separate defects:

1. **`10^d` is inexact for `d > 22`.** Powers of ten are exactly representable in binary64 only
   up to `10^22`; beyond that the constant itself is a rounded value, so the scaling introduces
   error before any truncation happens.
2. **The multiplication rounds.** Even with an exact `10^d`, `x * 10^d` rounds, and the rounding
   can cross an integer boundary. A value that is a hair below `k` mathematically can round *up*
   to exactly `k`, whereupon `trunc` keeps `k` and the caller sees a digit they asked to have
   discarded. The failure is one-sided and rare, which is what makes it a durable bug.
3. **The division rounds again.** The final quotient is the nearest double to the intended
   decimal, not the decimal itself.

**What careful implementations do.** The defensible approaches, in increasing order of effort:
scale with an exact power-of-ten table for the representable range and fall back to a
higher-precision path beyond it; or do the work in a decimal representation (`decimal128`, or a
big-integer significand with a decimal exponent) so that "keep `d` decimal digits" is exact by
construction; or use a correctly rounded decimal-conversion routine (the Steele-White / Ryu
line) to produce the digit string and rebuild from it. Spreadsheet implementations historically
take the first route. Excel's is recorded in FN-ROUND-002: a 15-digit reading on the x87 unit
before any digit is chosen.

**Why this matters more here than for `ROUND`.** Round-to-nearest is *self-correcting* about
boundary crossings: if the scaled product rounds slightly, the nearest integer is usually still
the right one, because the decision boundary is at the half-way point, far from where the
product lands. Truncation puts the decision boundary exactly at the integer, which is precisely
where the scaling error can push a value across. **Truncation is the more fragile of the two
operations under scaling**, and that is not obvious from the definitions.

**Repeated truncation is not idempotent across precisions.** `TRUNC(TRUNC(x, 4), 2)` need not
equal `TRUNC(x, 2)` once the scaled values are inexact. Chained rounding in financial worksheets
is where this surfaces.

## What has not been checked

No Handbook evidence record lists `FUNC.TRUNC` in its **subjects**. `FUNC.TRUNC` appears in the
group membership of `EV-STRUCT-0011`, an array-lift tranche whose counting is at group scope —
so **the family was measured and this surface was not measured separately**. The Handbook does
not attribute that record's figures to `TRUNC`, and this page claims nothing from it.

No Handbook vector suite exists for `TRUNC`. Beyond the group-scope record just named, the checks
against Excel are those of the Notes below: under the family rules the recorded rows match 50,504
of 51,317. Still open there: how the scale is formed for `|n| > 27`, the reading step itself, how
the NaN patterns sort and display, and why DBL_MIN converts 3 steps below nearest.

Probes worth running first:

1. **The scaling-boundary cases.** Values engineered to sit a fraction of an ULP below a
   `d`-decimal boundary — for example a value just under `1.005` truncated to two places, and
   the analogous constructions at several `d`. FN-ROUND-002 reads Excel as scaling in binary on
   the x87 unit, with a 15-digit reading first, so this now tests that reading.
2. **`d > 22`**, where the power of ten stops being representable in binary64. FN-ROUND-002 has
   its scale exact in 64 bits up to `|n| = 27`, and how it is formed beyond that is open.
3. **Negative `d`** across magnitudes, including `d` large enough that the answer is zero, and
   `d` negative enough that the scaling underflows.
4. **`TRUNC` against `ROUNDDOWN`** at the same inputs across a broad sample. If they ever
   differ, they are not the same code path, which would be a finding in itself given how
   interchangeable the documentation makes them look.
5. **Negative-zero production**: `TRUNC(-0.5)` — whether the published result carries a sign.
6. **Non-integer `num_digits`**, e.g. `TRUNC(1.23456, 2.7)`, which the documentation does not
   cover. Answered: Excel takes the integer part (FN-ROUNDUP-001).
7. **Very large `number` with positive `d`**, where the internal scaling would overflow.
8. **Array arguments in both positions**, given the group-scope array-lift record named above.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `TRUNC` that an implementer matching Excel, or a reader explaining an
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

Basis (measured): Recorded rows of the broad capture; counts from the rounding write-up.

- Open: Why this one input converts 3 steps below nearest.

Evidence:

- [`content/notes/evidence/2026-11-04/rounding-EVIDENCE.md`](../../content/notes/evidence/2026-11-04/rounding-EVIDENCE.md) — research write-up: full-bank result, the rules, what stays open
- OxFunc-Evidence `runs/w111-broad-20260929/answers/answers-trunc.json.gz`, w111broad-20260929-TRUNC-00026 at `e26b7bddb8` (readings)
- OxFunc-Evidence `runs/w111-broad-20260929/answers/answers-trunc.json.gz`, w111broad-20260929-TRUNC-00027 at `e26b7bddb8` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| toward zero | The rounding direction that discards magnitude; IEEE's `roundToIntegralTowardZero` |
| `num_digits` | The decimal position at which truncation happens; negative values act left of the point |
| scaling | Multiplying by `10^d` before the integral operation and dividing after |
| double-width step | The interval `(-s, s)` that all maps to zero, unique to toward-zero rounding |

## Sources

- Microsoft, "TRUNC function" —
  <https://support.microsoft.com/en-us/office/trunc-function-8b86a64c-3127-43db-ba14-aa5ceb292721>
  (signature, `num_digits` default of zero, and the stated contrast with `INT` including the
  `-4.3` example; no error conditions are listed).
- IEEE 754-2019 — `roundToIntegralTowardZero` and the rounding-direction attributes.
- Goldberg, *What Every Computer Scientist Should Know About Floating-Point Arithmetic* — why
  scaled decimal operations are inexact.
- Handbook evidence record `EV-STRUCT-0011` — cited only to record that `FUNC.TRUNC` appears
  there in group membership, which forbids per-surface attribution.
- Handbook projections `data/functions/FUNC.TRUNC.json` (arity 1-2,
  `kernel_signature_class: NumsToNum`, `coercion_lift_profile: Custom`, `real_result_policy`
  with `arg_domain_guard=none`) and `data/presence/FUNC.TRUNC.json`.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md).
