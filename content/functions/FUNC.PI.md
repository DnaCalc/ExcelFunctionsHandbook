---
schema: efh.function-page/v1
function_id: FUNC.PI
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft — WorksheetFunction.Pi method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.pi"
    role: "the only documented statement of what the constant is: \"the mathematical constant pi, accurate to 15 digits\""
  - work: "Microsoft Support — PI function"
    locator: "https://support.microsoft.com/en-us/office/pi-function-264199d0-a3ba-46b8-975a-c4a04608989b"
    role: "the worksheet-surface documentation page named in the projection; not retrievable for this pass"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 4 (elementary transcendental functions); table of pi to high precision"
    role: "the reference decimal expansion against which a stored constant is judged"
  - work: "Payne & Hanek, Radian reduction for trigonometric functions"
    locator: "SIGNUM Newsletter 18 (1983)"
    role: "why a rounded pi is not enough for large-argument trigonometric reduction"
episodes: []
body_sections:
  - "What it computes"
  - "Arguments"
  - "Result and edge cases"
  - "Errors"
  - "Relationships"
  - "Numerical notes"
  - "What has not been checked"
  - "Page vocabulary"
  - "Sources"
family: pi
role_in_family: "The whole family: a nullary constant with its own module, no arguments and no domain."
---

# PI

## What it computes

`PI()` returns the circle constant π — the ratio of a circle's circumference to its diameter,
equivalently the least positive zero of `sin`, equivalently `4·arctan(1)`.

    π = 3.14159265358979323846264338327950288...

π is irrational (Lambert, 1761) and transcendental (Lindemann, 1882). Neither property is
decorative here: **no floating-point number is π**. What a worksheet can hand back is the
binary64 value nearest to π, which is the exact dyadic rational

    7074237752028440 / 2^51

and which differs from π by δ ≈ 1.2246467991473532×10⁻¹⁶ — the number that shows up whenever
someone asks why the sine of a right angle is not zero.

Microsoft's VBA reference describes the return as "the mathematical constant pi, accurate to 15
digits". Fifteen digits is a statement about the decimal rendering, not about the stored value:
the stored double carries the full ~17 significant decimal digits of the nearest binary64.

## Arguments

None. The projection records an arity of exactly zero and the kernel class `NullaryConst`: the
function takes no arguments, has no optional slots, and admits no coercion. The parentheses are
mandatory syntax, not an empty argument list.

Because there is no argument, none of the shared coercion machinery in
[Coercion and lifting](../model/02-coercion-and-lifting.md) applies to this surface.

## Result and edge cases

Returns `Number`. The projection classifies it `Deterministic`, `NonVolatile`, `SafePure` — the
same value on every call, every recalculation and every thread. There is no edge case in the
argument domain, because there is no argument domain.

The two edges that exist are elsewhere:

1. **`PI()` is not π.** Any identity that holds exactly for π holds only approximately for
   `PI()`. `SIN(PI())` is not zero; mathematically `sin(fl(π)) = sin(π − δ) = sin δ ≈ δ`, so the
   result is the tiny residual above rather than zero. This is correct arithmetic on a correct
   constant, and it is the most common "bug report" the constant attracts.
2. **`PI()/2` is exact, `PI()*2` is exact, `PI()/3` is not.** Halving and doubling a binary
   float are exact; dividing by three is not. Formulas that need π/2 or 2π lose nothing;
   formulas that need π/180 (see [RADIANS](FUNC.RADIANS.md)) round twice.

## Errors

No documented error condition and no domain to violate. The only refusal available is an
argument count other than zero, which is an admission failure at formula entry rather than a
runtime error value — see the entry-time rejection rule in
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Relationships

- **[RADIANS](FUNC.RADIANS.md)** and **`DEGREES`** — the two functions that exist so you do not
  have to write `x*PI()/180`, and which round differently from that expression.
- **`SQRTPI`** — returns `SQRT(number·π)`. OxFunc's math-deviation catalogue records `SQRTPI` as
  evaluated through a general power rather than a correctly rounded square root, which makes it a
  poorer route to `√π` than the name suggests.
- **The trigonometric family** (`SIN`, `COS`, `TAN`, `SEC`, `CSC`, `COT`) — all of which reduce
  their argument modulo 2π internally, using a π that is not `PI()` and not visible from the
  worksheet.
- **There is no `E()`.** Excel publishes π as a function and Euler's number only as `EXP(1)`.
  The asymmetry is historical, not principled.

## Numerical notes

The interesting numerics of a constant are entirely about what happens downstream.

**Argument reduction is the reason a rounded π is not enough.** Computing `sin(x)` for large `x`
requires `x mod 2π` — and the modulus must be taken against *π*, not against `fl(π)`. Reducing
with a 53-bit π loses one bit of the reduced argument for every bit by which `x` exceeds π, so a
naive reduction is worthless well before the exponent range is exhausted. The standard remedy is
Payne–Hanek reduction, which multiplies by a many-word representation of 2/π and keeps only the
bits that survive; fdlibm's `__ieee754_rem_pio2` is the canonical implementation. Excel's own
circular-trigonometric surfaces sidestep the problem instead: OxFunc's math-deviation catalogue
records live Excel refusing arguments of magnitude 2²⁷ or greater with `#NUM!` — see
[SEC](FUNC.SEC.md).

**Writing `3.14159265358979` in a cell is not the same as writing `PI()`.** The literal parses to
a different double: the decimal shown in the documentation is π truncated, not π rounded to 15
digits, and truncation and round-to-nearest disagree here. Spreadsheets that hard-code the
constant from the documentation string are working with a different number from the ones that
call the function.

**Where π appears inside a formula, association matters.** `x*PI()/180` and `x*(PI()/180)` are
two different computations: the first rounds `x·fl(π)` and then divides, the second rounds
`fl(π)/180` once and then multiplies. Neither is uniformly better; they differ, and the
difference is the whole subject of [RADIANS](FUNC.RADIANS.md).

## What has not been checked

No Handbook vector suite exists for `PI`, and no evidence record in
`content/evidence/records/` lists this surface among its subjects. Nobody has checked this
function against Excel within the Handbook's record. The probe battery rendered beside this page
is a reference-engine measurement, not an Excel measurement.

The whole question for this surface is a single bit pattern, so the probe list is short and
decisive:

1. **The returned bits.** Whether Excel's `PI()` is the binary64 nearest to π or a different
   nearby double. `=PI()-3.14159265358979` and `=PI()*2^51` are the two cheapest discriminators
   that survive display rounding; the second is exact and integral if and only if the value is
   the nearest double.
2. **`SIN(PI())`.** If the constant is the nearest double, the result is the residual δ above.
   Any other value means either the constant or `SIN` is not what this page assumes.
3. **`PI()` under a non-English locale and under Compatibility Version 1**, to establish that the
   constant is not one of the surfaces that shifts with workbook mode.
4. **`PI()` inside an add-in call across the C API boundary**, which is where a value that is
   normalized on the worksheet would show its raw form.

Until those exist, the honest statement is that the Handbook knows what π is and does not know
what Excel returns for it.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| nullary | Takes no arguments; the parentheses are syntax, not an empty argument list |
| nearest double | The binary64 value closest to the exact real number, under round-to-nearest-even |
| δ (delta) | The gap π − fl(π); the value `SIN(PI())` should return |
| argument reduction | Reducing a trigonometric argument modulo 2π before evaluating a kernel |

## Sources

- Microsoft, "WorksheetFunction.Pi method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.pi> (the "accurate to
  15 digits" wording, and the absence of any statement about the stored bits).
- Microsoft Support, "PI function" —
  <https://support.microsoft.com/en-us/office/pi-function-264199d0-a3ba-46b8-975a-c4a04608989b>
  (the worksheet page named in `data/functions/FUNC.PI.json`; the Handbook's retrieval of this
  page was refused with HTTP 403 during this pass, so nothing on this page is sourced from it).
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 4 — the elementary
  transcendental functions and the high-precision value of π.
- Payne & Hanek, "Radian reduction for trigonometric functions", SIGNUM Newsletter 18 (1983);
  fdlibm `__ieee754_rem_pio2` — the standard treatment of reduction against an exact π.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.PI.json` (arity 0, `NullaryConst`, `Deterministic`,
  `NonVolatile`, `SafePure`) and `data/presence/FUNC.PI.json` (implementing module, no sibling
  surfaces, no upstream defect streams).
