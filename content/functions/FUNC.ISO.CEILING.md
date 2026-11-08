---
schema: efh.function-page/v1
function_id: FUNC.ISO.CEILING
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
  - Page vocabulary
  - Sources
family: ceiling_floor_family
role_in_family: >-
  The sign-blind ceiling: it takes the absolute value of significance and so returns the
  mathematical ceiling for every sign combination, unlike the legacy CEILING it was
  introduced to correct.
---

## What it computes

`ISO.CEILING(number, [significance])` rounds *number* **upward** — toward positive infinity — to
the nearest multiple of *significance*.

    ISO.CEILING(x, s) = ⌈ x / |s| ⌉ · |s|

with *s* defaulting to 1, in which case the function is the plain mathematical ceiling ⌈x⌉.

The defining property, and the reason the function exists, is stated on Microsoft's page:

> "The absolute value of the multiple is used, so that the ISO.CEILING function returns the
> mathematical ceiling irrespective of the signs of number and significance."

So the direction of rounding is **always toward +∞**, for every combination of signs. The
documented examples make the shape explicit: 4.3 rises to 5, −4.3 rises to −4, and 4.3 with a
significance of either 2 or −2 gives 6, while −4.3 with either sign of 2 gives −4. Negating the
significance changes nothing.

Ceiling is the reflection of floor: ⌈x⌉ = −⌊−x⌋, so everything true of
[INT](FUNC.INT.md) has a mirror image here. Domain: all reals for *number*; any nonzero real for
*significance*. Range: the multiples of |*s*|. The function is non-decreasing in *number* and
discontinuous at every multiple of |*s*|, with a jump of exactly |*s*| — this time from the
*right*, since ceiling is right-continuous where floor is left-continuous.

The name records its origin: this is the rounding rule of ISO/IEC 29500 (the OOXML spreadsheet
formula specification), added so that a spreadsheet could express the mathematical ceiling
without inheriting the sign-dependent behaviour of the older `CEILING`.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | "The value to be rounded." Required. | — |
| `significance` | "The optional multiple to which number is to be rounded." | 1 |

The declared arity is one to two, which agrees with the documented optional second argument.

**A projection gap worth naming.** The Handbook's mechanical projection of this entry carries no
signature at all — the signature field is marked as a placeholder rather than filled — so the
generator has nothing to render there. The signature above is taken from Microsoft's page, not
from the projection. This is a gap in the Handbook's own data, published rather than papered
over.

Both slots are numeric and subject to ordinary to-number coercion
([coercion and lifting](../model/02-coercion-and-lifting.md)).

## Result and edge cases

Returns `Number`.

The reference engine's published battery is rendered beside this page. Qualitatively, it shows:

- **Zero** returns zero, and **−1** returns −1: an argument already on a multiple is left alone.
  Ceiling is idempotent, ⌈⌈x⌉⌉ = ⌈x⌉, and that is the visible consequence.
- **A logical argument** and **numeric text** both convert and round. Microsoft's page addresses
  neither.
- **The smallest positive subnormal** — on Excel 16.0 build 20527, writing a subnormal binary64
  value (`±5e-324` through `±(2^-1022 - 5e-324)`) or `-0` to a cell via `Range.Value2` stores
  `+0`, so a cell-written subnormal `number` or `significance` returns `0`
  ([sweep-questions-1-ANSWERS.md](../notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md),
  `C:\Work\DnaCalc\codex-runs\sweep-questions-1\Q1-subnormal-inputs.json`). When a true computed
  subnormal reaches `ISO.CEILING` in a formula (`x` from `EXP(-745)`, `EXP(-730)`, or
  `EXP(-709)`), `ISO.CEILING(x, 1)` rounds up to `1` and `ISO.CEILING(0 - x, 1)` returns `0`,
  while a subnormal quotient `number / |s|` flushes to `0` ([FN-ABS-002](../notes/FN-ABS-002.json)).
- **The largest finite double** is returned unchanged, because every double at or above 2^52 is
  already an integer and therefore already a multiple of 1.
- **An inline array** produces an array of the same shape. Note that the module implementing this
  family carries an open upstream defect stream concerning math scalar/array lifting
  (`BUG-FUNC-017`), so array-shaped arguments in this family are an area with recorded
  historical trouble rather than a settled one.
- **An empty range** produces `#VALUE!`.

**What is not documented, and is not invented here:** what happens when *significance* is zero.
Every multiple of zero is zero, so "the least multiple of 0 that is at least x" exists only for
x ≤ 0. Microsoft's page does not state a rule; the neighbouring `CEILING.MATH` documentation
states that a zero significance yields zero, but that is a different function's page and this
page will not transfer it. It is the first probe below.

## Errors

Microsoft's `ISO.CEILING` page documents **no error return**. Errors reachable here come from the
shared call model:

| Error | Condition |
|---|---|
| `#VALUE!` | An argument does not convert to a number under the shared to-number rules |
| propagated | An error value in either argument surfaces as that error |

The reference engine additionally reports `#VALUE!` for arity failures. The absence of a
documented `#NUM!` is a real difference from the legacy `CEILING`, whose
documentation *does* carry a sign-mismatch error — because `ISO.CEILING` takes the absolute
value of significance, it has no sign mismatch to reject.

## Relationships

This is the most crowded neighbourhood in the math category, and the differences are entirely
about **what happens to negatives**.

- **`CEILING.PRECISE`** — documented with the same rule: absolute value
  of significance, mathematical ceiling regardless of signs. On the documented behaviour the two
  are the same function under two names, one carrying the ISO lineage and one the Excel-native
  naming scheme. This page does **not** assert that they share a code path or that they agree in
  every bit; it asserts only that the two published rules coincide.
- **`CEILING`** — the legacy function `ISO.CEILING` was introduced to correct.
  Its documented rule is sign-dependent, and it is the one that surprises people.
- **`CEILING.MATH`** — the modern general form, with an explicit third
  argument choosing the direction for negative numbers. `ISO.CEILING` is the fixed-mode case.
- **`FLOOR.PRECISE`**, **[FLOOR.MATH](FUNC.FLOOR.MATH.md)**,
  **`FLOOR`** — the downward mirrors, one for one.
- **[INT](FUNC.INT.md)** — the plain floor. `ISO.CEILING(x)` and `-INT(-x)` are the same number.
- **[MROUND](FUNC.MROUND.md)** — rounds to the *nearest* multiple rather than up.
- All seven of the ceiling/floor surfaces are implemented in one shared module in the reference
  engine, so they are a family by construction there as well as by documentation.

## Numerical notes

Like floor, ceiling introduces no rounding error of its own and inherits every hazard from the
discontinuity. The two additions specific to the *significance* form are worth stating.

**The division is where the error enters.** `⌈x/|s|⌉·|s|` performs a division, a ceiling and a
multiplication. Neither the division nor the multiplication is exact in general, so:

1. **`x/|s|` may round to the wrong side of an integer.** If *x* is an exact multiple of *s* but
   the quotient rounds to just above the integer, the ceiling adds a whole step and the answer is
   one multiple too high. This is not hypothetical: it is the standard failure mode of every
   naive round-to-multiple implementation, and it is why `ISO.CEILING(4.2, 0.1)` and its
   relatives are the classic test cases. Decimal significances like 0.1, 0.05 and 0.01 are not
   representable in binary64, so *every* such call is asking for the ceiling of a quotient that
   was already inexact.
2. **The final multiplication may not land back on a representable multiple.** `n·|s|` rounds,
   so the returned value can fail to be exactly divisible by *s* even when *n* is correct.

The standard remedies are the ones used throughout careful decimal-rounding work: compute the
quotient with a fused multiply-add or a double-double residual to determine the correct side of
the integer boundary before rounding; or scale by a power of ten and work in exact integers when
the significance is a decimal fraction; or apply a deliberate, documented tolerance so that a
quotient within a few ULP of an integer is treated as on it. Each of these is a *decision*, and
what matters for a compatibility implementation is which decision the target made — a question
this page cannot answer for Excel.

For the default significance of 1 none of this arises: the operation is IEEE 754's
`roundToIntegralTowardPositive` (C99 `ceil`, Rust `f64::ceil`), exact for every finite double,
correctly rounded by the standard, and with no intermediate at all. The gap in difficulty between
`ISO.CEILING(x)` and `ISO.CEILING(x, 0.1)` is the whole story of this function. In DNA Calc
(`crates/calc-func/src/catalogue/math.rs`, commit `e042619e35426ffac3d8cfaaa2dd2961abe3d02c`,
and `crates/calc-func/src/witness_exec.rs`, commit `c0b94f88ab4e3df5b518329d08ea03cf481abdbe`),
`ISO.CEILING` accepts true subnormal `number` inputs while flushing subnormal quotients and
cell-written subnormal `Range.Value2` inputs to `+0`, matching all 1,335 of 1,335 recorded
`ISO.CEILING` bank rows (`C:\Work\DnaCalc\codex-runs\sweep-report-2.md`).

## What has not been checked

No Handbook vector suite exists for `ISO.CEILING`; `vectors/` publishes nothing for it, and **no
evidence record names `ISO.CEILING` among its subjects**. `ISO.CEILING` does appear inside the
group membership of one structural array-lift record without being one of that record's
subjects, and that record's counts carry an explicit warning against per-surface attribution —
so the honest statement is that the family was measured on an array-shape axis and **this surface
was not measured separately**. Nobody has checked this function's values against Excel.

Everything above marked as documented comes from Microsoft's `ISO.CEILING` page: the syntax, the
two argument descriptions, the default significance of 1, the absolute-value-of-significance
rule, and the six worked examples.

Inputs I would probe first:

1. **`ISO.CEILING(4.3, 0)` and `ISO.CEILING(-4.3, 0)`.** The undocumented zero-significance case.
   Two cells, and it is the only outright gap in the documented rule.
2. **`ISO.CEILING(4.2, 0.1)`, `ISO.CEILING(0.3, 0.1)`, `ISO.CEILING(1.4, 0.2)`.** The inexact-
   significance cases described above, where an implementation's tolerance policy becomes
   visible. These are the cells that separate implementations; a value that is mathematically
   already on the multiple but not in binary64 is exactly the input a naive kernel over-rounds.
3. **`ISO.CEILING(-4.3, -2)` against `CEILING.PRECISE(-4.3, -2)` and `CEILING(-4.3, -2)`**, in
   one workbook. This is the cheapest demonstration of what the three names actually differ on,
   and it would settle whether the two "precise" surfaces agree to the last bit or only in
   documentation.
4. **`ISO.CEILING(TRUE)` and `ISO.CEILING("2.5")`** — undocumented conversions the reference
   engine accepts.
5. **`ISO.CEILING(-0)` and the sign of the returned zero.**
6. **An array in each argument slot**, given the open lifting defect stream on this family's
   module.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `ISO.CEILING` that an implementer matching Excel, or a reader explaining
an unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's
answer and where it was recorded; "measured" means the statement is read off recorded Excel
answers, "inferred" marks what goes beyond them. Where older text on this page says a behaviour
was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

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
| ceiling | The least integer not below the argument; rounds toward +∞ |
| significance | The multiple to round to; its absolute value is used |
| sign-blind | The rounding direction does not depend on the sign of either argument |
| inexact significance | A multiple such as 0.1 that has no exact binary64 representation |
| projection gap | A field the Handbook's mechanical data does not carry, published as absent |

## Sources

- Microsoft, "ISO.CEILING function" —
  <https://support.microsoft.com/en-us/office/iso-ceiling-function-e587bb73-6cc2-4113-b664-ff5b09859a83>.
  Retrieved for this page: the syntax, both argument descriptions, the default significance of 1,
  the absolute-value remark quoted above, and the six worked examples.
- ISO/IEC 29500 (OOXML) spreadsheet formula specification — the lineage the function's name
  records.
- IEEE 754-2019, `roundToIntegralTowardPositive` — the exact ceiling for the default-significance
  case.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — to-number outcomes,
  lifting, error propagation.
- `data/functions/FUNC.ISO.CEILING.json` — arity 1–2, the declared axes, and the empty signature
  placeholder recorded above, as projected at OxFunc `473efa3`;
  `data/presence/FUNC.ISO.CEILING.json` — the shared `ceiling_floor_family` module, its six
  sibling surfaces, and the `BUG-FUNC-017` lifting defect stream.
- [sweep-questions-1-ANSWERS.md](../notes/evidence/2026-11-08/sweep-questions-1-ANSWERS.md)
  (`C:\Work\DnaCalc\codex-runs\sweep-questions-1\ANSWERS.md`, `Q1-subnormal-inputs.json`) and
  `C:\Work\DnaCalc\codex-runs\sweep-report-2.md` — Excel 16.0 build 20527 subnormal cell-write
  flushing, true computed subnormal `ISO.CEILING(EXP(-745), 1) = 1` probes, and DNA Calc bank
  comparison (`c0b94f88ab`, `e042619e35`).
