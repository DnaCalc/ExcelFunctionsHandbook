---
schema: efh.function-page/v1
function_id: FUNC.ROUNDUP
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft — WorksheetFunction.RoundUp method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.roundup"
    role: "the documented \"rounds a number up, away from 0 (zero)\" definition and the three num_digits remarks"
  - work: "Microsoft Support — ROUNDUP function"
    locator: "https://support.microsoft.com/en-us/office/roundup-function-f8bc9b23-e795-47db-8703-db171d0c42a7"
    role: "the worksheet-surface documentation page named in the projection; not retrievable for this pass"
  - work: "Gay, Correctly rounded binary-decimal and decimal-binary conversions"
    locator: "AT&T Bell Labs Numerical Analysis Manuscript 90-10, 1990"
    role: "the reference algorithm for deciding a decimal digit exactly rather than by scaling"
  - work: "IEEE 754-2019, clause 4.3.2"
    locator: "roundTowardPositive / roundTowardNegative"
    role: "the directed attributes this function combines into an away-from-zero rule"
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
family: roundup_fn
role_in_family: "The away-from-zero member of the directed-rounding trio; the conservative-overstatement counterpart of ROUNDDOWN."
---

# ROUNDUP

## What it computes

`ROUNDUP(number, num_digits)` rounds away from zero to `num_digits` decimal places.

    ROUNDUP(x, d) = sign(x) · ⌈|x| · 10^d⌉ / 10^d

Microsoft's VBA reference summarises it as "Rounds a number up, away from 0 (zero)" and then
defines it by difference: "**RoundUp** behaves like **Round**, except that it always rounds a
number up". The parenthetical is doing real work — **`ROUNDUP` is not the ceiling.** For positive
values the two agree; for negative values they do not. `ROUNDUP(-1.2, 0)` is `-2`, while the
ceiling of `-1.2` is `-1`.

The same three `num_digits` remarks apply as on [ROUND](FUNC.ROUND.md): greater than zero rounds to
that many decimal places, zero rounds to an integer, less than zero rounds to the left of the
decimal point.

**The properties that make it useful.** Rounding away from zero never decreases magnitude and is
idempotent:

    |ROUNDUP(x, d)| ≥ |x|,   and   ROUNDUP(ROUNDUP(x, d), d) = ROUNDUP(x, d)

so a `ROUNDUP` result is a *safe overestimate* of magnitude. It is the correct choice for anything
that must not fall short — boxes needed, staff required, buffer to allocate. It is also, for the
same reason, the wrong choice for anything summed over many rows, because the bias accumulates: a
column of `ROUNDUP(x, 2)` overstates its total by up to one cent per row, systematically and in one
direction.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | Any real number to be rounded up. Required. | — |
| `num_digits` | The number of decimal places. Required. | — |

Both are numeric slots under ordinary to-number coercion — see
[Coercion and lifting](../model/02-coercion-and-lifting.md). As with the rest of the family,
`num_digits` is **required**, and the reference engine truncates a non-integral `num_digits` toward
zero. Neither fact is documented.

## Result and edge cases

Returns `Number`.

- **Sign is preserved and magnitude grows.** `ROUNDUP(-1.2, 0)` is `-2`. This is the whole
  difference from `CEILING.MATH`'s default direction.
- **Zero is a fixed point.** `ROUNDUP(0, d)` is `0` for every `d` — the reference engine special-
  cases it, and it has to, because the sign extraction has no meaningful answer at zero.
- **There are no ties.** As with [ROUNDDOWN](FUNC.ROUNDDOWN.md), the direction is fixed, so the
  tie-rule question that dominates [ROUND](FUNC.ROUND.md) does not arise.
- **Any nonzero fraction promotes.** `ROUNDUP(1.0000000000000002, 0)` is `2`. The function is
  extremely sensitive to the last bit: a value that is one ulp above an integer rounds up a whole
  unit. Since arithmetic routinely produces values one ulp off an intended integer, `ROUNDUP` on a
  computed quantity is a well-known source of off-by-one results — the value that "should be" 3.0
  is stored as 3.0000000000000004 and comes back as 4.
- **Large and small `num_digits`.** The reference engine returns `number` unchanged once
  `num_digits` reaches 308, and a signed zero once it reaches −308. Implementation limits, not
  documented behaviour.
- **Arrays.** The projection declares a `Custom` coercion profile with `surface_native` lifting.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | Either argument does not convert to a number | shared coercion model |
| propagated | An error value in either argument | shared coercion model |
| `#VALUE!` | Wrong argument count (`num_digits` omitted) | arity, refused at entry |

Microsoft's VBA page states no error condition for this method. In particular it does not say what
happens when rounding up at a coarse place would exceed the representable range — `ROUNDUP` of a
value near the top of the double range at negative `num_digits` has to either saturate or fail, and
nothing documents which.

## Relationships

- **`CEILING.MATH`** — rounds toward positive infinity by default, with a `mode` argument that
  switches negative numbers to away-from-zero behaviour. `CEILING.MATH(x, 10^-d, 1)` is the
  equivalent of `ROUNDUP(x, d)`; the default `mode` is *not*. Reach for `CEILING.MATH` when the
  direction must be explicit and for a significance that is not a power of ten.
- **`ISO.CEILING` and `CEILING.PRECISE`** — the always-toward-positive-infinity variants, which
  differ from `ROUNDUP` on exactly the negative half-line.
- **[ROUND](FUNC.ROUND.md)** and **[ROUNDDOWN](FUNC.ROUNDDOWN.md)** — the other two directions.
  `ROUNDDOWN(x,d) ≤ ROUND(x,d) ≤ ROUNDUP(x,d)` in magnitude, always.
- **`ROUNDUP(x/n, 0)`** — the idiomatic "how many containers" calculation, and the one that meets
  the last-bit sensitivity above head-on: when `x` is an exact multiple of `n`, the quotient may
  still land one ulp high and produce one container too many. Where the operands are integers,
  `QUOTIENT` plus a remainder test is the safer construction; see [QUOTIENT](FUNC.QUOTIENT.md).
- **`ROUNDUP` is odd:** `ROUNDUP(-x, d) = -ROUNDUP(x, d)`. `CEILING.MATH` in its default mode is
  not.

## Numerical notes

**No ties, but the sharpest boundary sensitivity in the family.** `ROUNDUP` promotes on *any*
excess, however small, which makes it the function most exposed to the accumulated error of
whatever computed its argument. `ROUND` needs an error of half a decimal unit to change its
answer; `ROUNDUP` needs one ulp. That is a difference of roughly fifteen orders of magnitude in
required perturbation, and it is why `ROUNDUP` results are so often described as "wrong" when the
arithmetic upstream is what moved.

**Scaling can overshoot in the harmless direction and undershoot in the harmful one.** With the
naive `⌈|x|·10^d⌉/10^d`, if the product rounds *up* across an integer that the exact product falls
short of, the ceiling is already that integer and the answer is correct. But if the product rounds
*down* onto an integer that the exact product slightly exceeds, the ceiling stops there and the
result is one unit too *small* — violating the function's defining guarantee that the result never
falls short. For a function whose entire contract is "never understate", that is the failure mode
to guard.

**The guard is one exact residual test.** Form the candidate, compute `|x| − candidate` exactly
with a fused multiply-add, and if it is positive, increment by one unit in the last decimal place.
There is no tie case, so one correction step is provably sufficient. As on the sibling pages: a
`natural-best` implementation should do this, and an `excel-bitexact` implementation should not
until Excel's own boundary behaviour is pinned — see
[About implementation options](../model/07-implementation-options.md).

**The target grid is still not representable.** For `d > 0` the multiples of `10^-d` do not exist
in binary64, so the returned value is the nearest double to the rounded-up decimal, and that
nearest double can be *below* the decimal it represents. Consequently `ROUNDUP(x, 2) ≥ x` holds as
a comparison of stored doubles in practice but is not guaranteed by the specification; the
guarantee lives in decimal, and the value lives in binary.

**Accumulated bias is a modelling concern, not an implementation one.** A column of `n` values each
rounded up at `d` places overstates its total by an expected `n·10^-d/2`. If the total matters,
round the total rather than the parts — the classic penny-allocation problem, and the reason
`ROUNDUP` should never be applied row-wise before a `SUM` that is meant to reconcile.

## What has not been checked

No Handbook vector suite exists for `ROUNDUP`, and **no evidence record lists this surface among
its subjects**. `ROUNDUP` appears inside the group membership of an upstream array-lift measurement
(`EV-STRUCT-0011`) whose subjects are other surfaces; that record's reader warning forbids reading
its group total as any individual surface's rate. The honest statement is therefore: **the rounding
family was measured as a group for structural array admission; this surface was not measured
separately, and its arithmetic was not measured at all.**

The presence projection records the defect stream `BUG-FUNC-017` against this module.

Inputs I would probe first:

1. **`ROUNDUP(-1.2, 0)` beside `CEILING.MATH(-1.2)` and `ISO.CEILING(-1.2)`** — three cells that
   settle the away-from-zero-versus-toward-positive-infinity question.
2. **`ROUNDUP(3.0000000000000004, 0)` and `ROUNDUP(0.1+0.2, 1)`** — the last-bit sensitivity, using
   values that arithmetic actually produces rather than values typed by hand.
3. **A boundary case engineered to expose scaling undershoot**, where the exact product `|x|·10^d`
   slightly exceeds an integer but rounds down onto it. If the result equals that integer divided
   by the scale, the implementation is violating its own never-understate contract.
4. **`ROUNDUP(0, 5)` and `ROUNDUP(-0, 5)`** — the fixed point and its sign.
5. **`ROUNDUP(1.7976931348623157E+308, -1)`** — the overflow question the documentation does not
   address: saturate, error, or wrap.
6. **`ROUNDUP(1.5, 0.9)`** — non-integral `num_digits`, truncated in the reference engine and
   undocumented.
7. **Array arguments in both slots**, given the array-lift defect stream on this module.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `ROUNDUP` that an implementer matching Excel, or a reader explaining an
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

Basis (measured): Under the family rules the OxFunc-Evidence bank matches ROUND 36,391 of
36,945 rows, ROUNDUP 49,013 of 52,841, ROUNDDOWN 59,080 of 59,995 and TRUNC 50,409 of 51,317,
with no row lost and each remaining failure classified in the write-up.

- Open: How the scale is formed for |n| > 27: a single correctly rounded 64-bit power and the chained multiplications tried do not fit; 123 ROUND, 548 ROUNDUP, 556 ROUNDDOWN and 555 TRUNC rows remain.

Evidence:

- DnaCalc `crates/calc-func/tests/f1_math.rs`, rounding_family_matches_recorded_bank_rows at `77873410e3` — ROUND family rules in the product; failed before the change
- [`content/notes/evidence/2026-11-04/rounding-EVIDENCE.md`](../../content/notes/evidence/2026-11-04/rounding-EVIDENCE.md) — research write-up: full-bank result, the rules, what stays open
- OxFunc-Evidence `cache/oracle-20430/build-20430/ROUND.jsonl.gz`, line 3758, key ROUND|0xc316a939806956e4|0x402c000000000000 at `e26b7bddb8` (readings)
- OxFunc-Evidence `cache/oracle-20430/build-20430/ROUNDDOWN.jsonl.gz`, line 753, key ROUNDDOWN|0x42f0168ea69971b8|0x0000000000000000 at `e26b7bddb8` (readings)

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

Basis (measured): Under the family rules the OxFunc-Evidence bank matches ROUND 36,391 of
36,945 rows, ROUNDUP 49,013 of 52,841, ROUNDDOWN 59,080 of 59,995 and TRUNC 50,409 of 51,317,
with no row lost and each remaining failure classified in the write-up. All 13,914 informative
ROUNDDOWN and TRUNC rows with large num_digits follow the rule.

Evidence:

- DnaCalc `crates/calc-func/tests/f1_math.rs`, rounding_family_matches_recorded_bank_rows at `77873410e3` — ROUND family rules in the product; failed before the change
- [`content/notes/evidence/2026-11-04/rounding-EVIDENCE.md`](../../content/notes/evidence/2026-11-04/rounding-EVIDENCE.md) — research write-up: full-bank result, the rules, what stays open
- OxFunc-Evidence `runs/w111-rounding-count-refinement-20260929/answers-roundup.json.gz`, w111roundcount-ROUNDUP-00041 at `e26b7bddb8` (readings)
- OxFunc-Evidence `runs/w111-rounding-count-refinement-20260929/answers-roundup.json.gz`, w111roundcount-ROUNDUP-00043 at `e26b7bddb8` (readings)
- OxFunc-Evidence `runs/w111-rounding-count-refinement-20260929/answers-roundup.json.gz`, w111roundcount-ROUNDUP-00033 at `e26b7bddb8` (readings)
- OxFunc-Evidence `runs/w111-rounding-count-refinement-20260929/answers-trunc.json.gz`, w111roundcount-TRUNC-00041 at `e26b7bddb8` (readings)

### FN-ROUNDUP-002: ROUNDUP adds its unit in binary64, one bit from the decimal answer

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.*

ROUNDUP forms the ROUNDDOWN result, converts it to binary64, converts one unit 10^-p to
binary64, and adds the two in binary64. The sum need not be the binary64 value nearest the
decimal answer, so ROUNDUP(-1.125,2) is -1.1300000000000001 and ROUNDUP(3.535,2) is
3.5399999999999996. The difference shows when the result is compared with a typed 3.54.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `ROUNDUP(-1.125, 2)` | number `-1.125` (`0xbff2000000000000`); num_digits `2` (`0x4000000000000000`) (cells) | `-1.1300000000000001` (`0xbff2147ae147ae15`) | `-1.13` (`0xbff2147ae147ae14`) | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |
| `ROUNDUP(3.535, 2)` | number `3.535` (`0x400c47ae147ae148`); num_digits `2` (`0x4000000000000000`) (cells) | `3.5399999999999996` (`0x400c51eb851eb851`) | `3.54` (`0x400c51eb851eb852`) | Excel 16.0 build 20430, Windows 64-bit, AMD Ryzen 7 4800H |

Correct value: The decimal answer has a nearest binary64 value; rounding away from zero at 2
places gives that value.

Basis (measured): Under the family rules the OxFunc-Evidence bank matches ROUND 36,391 of
36,945 rows, ROUNDUP 49,013 of 52,841, ROUNDDOWN 59,080 of 59,995 and TRUNC 50,409 of 51,317,
with no row lost and each remaining failure classified in the write-up.

Evidence:

- DnaCalc `crates/calc-func/tests/f1_math.rs`, rounding_family_matches_recorded_bank_rows at `77873410e3` — ROUND family rules in the product; failed before the change
- [`content/notes/evidence/2026-11-04/rounding-EVIDENCE.md`](../../content/notes/evidence/2026-11-04/rounding-EVIDENCE.md) — research write-up: full-bank result, the rules, what stays open
- OxFunc-Evidence `cache/oracle-20430/build-20430/ROUNDUP.jsonl.gz`, line 1863, key ROUNDUP|0xbff2000000000000|0x4000000000000000 at `e26b7bddb8` (readings)
- OxFunc-Evidence `cache/oracle-20430/build-20430/ROUNDUP.jsonl.gz`, line 1809, key ROUNDUP|0x400c47ae147ae148|0x4000000000000000 at `e26b7bddb8` (readings)

### FN-TRUNC-001: TRUNC of DBL_MIN returns a subnormal, not 0

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.* Applies to ROUNDDOWN, ROUNDUP, TRUNC.

With DBL_MIN (0x0010000000000000) as number, TRUNC, ROUNDDOWN and ROUNDUP return the subnormal
0x000ffffffffffffa at each num_digits recorded, so TRUNC(DBL_MIN,0) is not 0. That value is 6
steps below DBL_MIN and 3 steps below 0x000ffffffffffffd, the binary64 nearest the 15-digit
reading 2.22507385850720E-308. The bank holds 256 such rows for each of the three functions.

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

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.ROUNDUP.json; edits here are overwritten -->

Readings of ROUNDUP with a recorded Excel build: 52,996 input(s) over 3 recorded
environment(s), Excel 16.0 build(s) 19929, 20430, all on Windows; recorded CPUs: AMD Zen 2. 1
same-input comparison(s) between environments over 1 input-pair(s): no result differs. No
reading from a Mac, Excel for the web, 32-bit Excel or an Intel CPU is recorded.

Readings of `ROUNDUP` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 155 | date not recorded |
| 16.0 build 20430 | Windows, 64-bit | AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | 3,454 | 2026-09-25 |
| 16.0 build 20430 | Windows, 64-bit | CPU not recorded | 49,387 | 2026-09-29 |

Compared on the same inputs:

| Check | Differs in | Environments | Inputs compared | Inputs with different results |
|---|---|---|---:|---:|
| XC-ROUNDUP-001 | excel-build | 16.0 build 19929 vs 16.0 build 20430 | 1 | 0 |

Differences observed: none.

Evidence for every row: [`content/evidence/environments/FUNC.ROUNDUP.json`](../../content/evidence/environments/FUNC.ROUNDUP.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| away from zero | The magnitude never decreases and the sign never changes |
| ceiling | Rounding toward positive infinity, which this function does **not** do |
| promotion | Advancing to the next multiple because of any nonzero excess, however small |
| scaling undershoot | The rounded product landing on an integer the exact product exceeds |
| accumulated bias | The systematic one-direction error of rounding many values up before summing |

## Sources

- Microsoft, "WorksheetFunction.RoundUp method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.roundup> ("Rounds a
  number up, away from 0 (zero)", the behaves-like-Round sentence, and the three `num_digits`
  remarks; no error conditions are stated).
- Microsoft Support, "ROUNDUP function" —
  <https://support.microsoft.com/en-us/office/roundup-function-f8bc9b23-e795-47db-8703-db171d0c42a7>
  (retrieval refused with HTTP 403 during this pass; nothing here is sourced from it).
- Gay, "Correctly rounded binary-decimal and decimal-binary conversions", Bell Labs NAM 90-10,
  1990; IEEE 754-2019 clause 4.3.2.
- Handbook, [ROUND](FUNC.ROUND.md) — the shared decimal-rounding analysis;
  [ROUNDDOWN](FUNC.ROUNDDOWN.md); [QUOTIENT](FUNC.QUOTIENT.md);
  [About implementation options](../model/07-implementation-options.md);
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.ROUNDUP.json` (`Custom` coercion,
  `precision_rounding_profile=default`) and `data/presence/FUNC.ROUNDUP.json` (own module; defect
  stream `BUG-FUNC-017`).
