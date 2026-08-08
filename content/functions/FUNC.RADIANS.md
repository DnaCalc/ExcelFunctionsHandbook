---
schema: efh.function-page/v1
function_id: FUNC.RADIANS
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft — WorksheetFunction.Radians method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.radians"
    role: "the complete documented content: signature, one-line description, no remarks and no error conditions"
  - work: "Microsoft Support — RADIANS function"
    locator: "https://support.microsoft.com/en-us/office/radians-function-ac409508-3d48-45f5-ac02-1497c92de5bf"
    role: "the worksheet-surface documentation page named in the projection; not retrievable for this pass"
  - work: "OxFunc — EXCEL_MATH_DEVIATION_CATALOG.md"
    locator: "docs/EXCEL_MATH_DEVIATION_CATALOG.md entry XMD-008"
    role: "the DEGREES overflow witness, which is the asymmetry this surface is defined against"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 4, section 4.3 (circular functions); the degree/radian conversion tables"
    role: "the conversion constant and the convention that trigonometric arguments are radians"
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
family: radians
role_in_family: "The degrees-to-radians scaling; the inverse direction of DEGREES, and the safer of the two."
---

# RADIANS

## What it computes

`RADIANS(angle)` converts an angle from degrees to radians.

    RADIANS(d) = d · π / 180

It is a linear map — multiplication by a single constant — and it exists because every
trigonometric function in the worksheet takes radians while every human angle is written in
degrees. The constant is

    π/180 = 0.01745329251994329576923690768488...

Domain: all real numbers. Range: all real numbers. The map is a bijection on the reals with
`DEGREES` as its exact mathematical inverse, and — this is the part that matters — *not* its
floating-point inverse.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `angle` | An angle in degrees. Required. | — |

One numeric slot under ordinary to-number coercion — see
[Coercion and lifting](../model/02-coercion-and-lifting.md). The projection classifies the
surface `UnaryNumericScalarOrArrayElementwise`: it is a scalar kernel, so an array argument lifts
elementwise and element failures stay element-local.

Microsoft's VBA page for this method is two sentences long. It states the signature, describes the
conversion, and stops: no remarks, no error conditions, no domain notes. Everything else on this
page is either mathematics or reference-engine classification, and is labelled.

## Result and edge cases

Returns `Number`.

- **`RADIANS(0)`** is zero, and `RADIANS(-0)` preserves the sign of zero in binary64 — a
  distinction with no worksheet consequence but a real one at the raw-return boundary; see
  [The value universe](../model/01-value-universe.md).
- **Large angles.** The projection declares `arg_domain_guard=none; non_finite=allow`. Because the
  constant is less than 1, multiplication by it cannot overflow: `RADIANS` of the largest finite
  double is still finite. This is the exact opposite of its sibling, and it is why `RADIANS` has no
  domain guard while `DEGREES` needs one.
- **Tiny angles.** The same multiplication *can* underflow: a subnormal input, multiplied by
  π/180, rounds to zero. The map is not injective at the bottom of the exponent range.
- **What comes next is where the surprises live.** `RADIANS(90)` is not π/2, because π/2 is not a
  double. `SIN(RADIANS(90))` is therefore not exactly 1 for the same reason `SIN(PI()/2)` is not —
  though in this case the error lands in a place where sine is flat, so it is invisible.
  `COS(RADIANS(90))` is the visible one: a small nonzero value rather than zero.

## Errors

No documented error condition, and none declared: the projection records
`arg_domain_guard=none`, so the only failures available are the shared ones.

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | The argument does not convert to a number | shared coercion model |
| propagated | The argument is an error value | shared coercion model |

## Relationships

- **`DEGREES`** — the other direction, `r·180/π`. The pair is not a floating-point round trip:
  `DEGREES(RADIANS(d))` returns `d` for many inputs and not for all, because two roundings by
  reciprocal-ish constants do not cancel. If a workbook needs the original degrees back, it should
  keep them rather than convert twice.
- **A projection asymmetry worth recording.** `DEGREES` declares `non_finite=num` and is named in
  OxFunc catalogue entry **XMD-008** with the witness `DEGREES(1E307)` — multiplying by 180/π
  overflows and Excel is recorded as publishing `#NUM!`. `RADIANS` declares `non_finite=allow` and
  appears in no such entry. The two surfaces are the same operation with reciprocal constants, and
  they sit on opposite sides of the engine's no-infinities convention purely because one constant
  is greater than 1 and the other is less. That is correct behaviour, not an inconsistency — but
  it is the kind of thing that looks like an inconsistency in a table of axes, so the Handbook
  states the reason.
- **The trigonometric family** (`SIN`, `COS`, `TAN`, `SEC`, `CSC`, `COT`) — the consumers.
  `SEC(RADIANS(x))` inherits the argument-magnitude guard described on [SEC](FUNC.SEC.md), but at
  a *different* input threshold, because `RADIANS` shrinks the argument by a factor of about 57.3
  before the guard sees it.
- **`PI`** — writing `x*PI()/180` by hand is the common alternative, and it is not the same
  computation; see the numerical notes.

## Numerical notes

**One rounding or two.** The reference engine computes `d · fl(π/180)`: the constant is rounded
once at compile time, and the multiplication rounds once. That is two roundings total, with a
worst-case relative error near 1 ulp. The handwritten alternative `d*PI()/180` also rounds twice —
once in `d·fl(π)`, once in the division — but the roundings fall in different places, so the two
expressions disagree on some inputs. A third form, `d/180*PI()`, is *better* than both when `d` is
an exact multiple of a power of two times 180, because division by 180 is exact more often than
one expects and the remaining multiplication is a single rounding.

None of the three is uniformly best. The `natural-best` route is a two-term (double-double)
representation of π/180: multiply by the high word, then correct with the low word, giving a
result correctly rounded on all but a vanishing set of inputs. The cost is one extra multiply-add.

**Why anyone should care.** The conversion is the *first* operation in a trigonometric pipeline,
so its error is the input error for everything downstream, and the trigonometric functions amplify
argument error by the derivative. For `TAN` and `SEC` near a pole that amplification is unbounded
(see [SEC](FUNC.SEC.md)), so a half-ulp difference in the converted angle becomes an arbitrarily
large difference in the result. Angles near 90° are exactly where engineering spreadsheets live.

**The exactness that is *not* available.** Degrees are a decimal-friendly unit and radians are not;
`RADIANS(90)` cannot be exact, cannot be reduced exactly against π, and cannot be recovered. A
library that wants exact quadrant behaviour has to keep the angle in degrees all the way to the
kernel and use degree-argument trigonometric functions (`sinpi`/`cospi`-style, with the argument
scaled by 1/180 exactly inside the kernel). Excel provides no such function, which is a real gap
worth naming: there is no `SIND`, and `SIN(RADIANS(30))` does not return exactly `0.5`.

## What has not been checked

No Handbook vector suite exists for `RADIANS`, and no evidence record in
`content/evidence/records/` lists this surface among its subjects. The presence projection records
no upstream defect stream touching this module. Nobody has checked this function against Excel
within the Handbook's record.

The whole surface is one constant and one multiplication, so a small probe set is decisive:

1. **`RADIANS(180)` compared bit-for-value against `PI()`**, via `=RADIANS(180)=PI()` and
   `=RADIANS(180)-PI()`. If they differ, the constant or the association differs from this page's
   assumption, and everything downstream shifts.
2. **`RADIANS(1)` against `PI()/180` and against `1/180*PI()`**, three cells that expose which
   association Excel uses.
3. **`RADIANS(90)` fed to `COS`** — the classic residual, and the cheapest downstream amplifier.
4. **`DEGREES(RADIANS(d))` swept over integer `d` from 1 to 360**, counting how many round-trip
   exactly. The answer characterises both functions at once.
5. **`RADIANS(1.7976931348623157E+308)` and `RADIANS(5E-324)`** — the two ends of the range,
   testing that no guard exists at the top and where the underflow to zero begins at the bottom.
6. **An array argument**, given that this surface is declared elementwise-lifting while several
   neighbouring math surfaces carry upstream array-support defect streams.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| conversion constant | π/180, rounded once to binary64 before use |
| double rounding | Two successive roundings (constant, then product) whose errors do not cancel |
| round trip | `DEGREES(RADIANS(d))`, which recovers `d` on many but not all inputs |
| amplification | Growth of an argument's error through a downstream function's derivative |

## Sources

- Microsoft, "WorksheetFunction.Radians method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.radians> (signature and
  description; the page states no remarks and no error conditions).
- Microsoft Support, "RADIANS function" —
  <https://support.microsoft.com/en-us/office/radians-function-ac409508-3d48-45f5-ac02-1497c92de5bf>
  (retrieval refused with HTTP 403 during this pass; nothing here is sourced from it).
- OxFunc `docs/EXCEL_MATH_DEVIATION_CATALOG.md` entry XMD-008 — the `DEGREES(1E307)` overflow
  witness that defines the asymmetry described above.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, section 4.3 — circular functions and
  the radian convention.
- Handbook, [The value universe](../model/01-value-universe.md),
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.RADIANS.json`
  (`UnaryNumericScalarOrArrayElementwise`, `arg_domain_guard=none`, `non_finite=allow`) and
  `data/presence/FUNC.RADIANS.json` (own module, no shared surfaces, no defect streams).
