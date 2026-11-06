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
  - "What it does"
  - "Try it"
  - "Quirks"
  - "How to implement"
  - "Version differences"
  - "Alternatives"
  - "Sources"
family: pi
role_in_family: "The whole family: a nullary constant with its own module, no arguments and no domain."
---

# PI

## What it does

`PI()` returns Excel's floating-point approximation to π, the ratio of a circle's circumference
to its diameter. It takes no arguments. Use `PI()*radius^2` for a circle's area and `2*PI()*radius`
for its circumference when ordinary floating-point arithmetic is appropriate.

In the recorded Excel 16.0.20527.20088 capture, the returned binary64 is
`0x400921fb54442d18`, approximately 3.141592653589793. This is the nearest binary64 value to π;
it is still a finite approximation to an irrational number.

## Try it

Calculate `=PI()` to see the decimal and raw binary64 bits. Compare `=PI()-3.14159265358979` to
see why a decimal copied from a documentation display is not the same stored constant.

## Quirks

Microsoft's description gives π to fifteen significant decimal digits. Parsing that shorter
decimal, 3.14159265358979, produces binary64 `0x400921fb54442d11`, seven representable steps below
the recorded PI result. The description is not an instruction to store that truncated string.

Identities involving exact π need not hold exactly when evaluated with a rounded constant and
rounded arithmetic. Multiplying or dividing this normal binary64 constant by 2 is exact in
binary arithmetic; dividing by 3 or 180 generally introduces another rounding.

The order of operations also matters: `x*PI()/180` and `x*(PI()/180)` round at different places.
Their mathematical equality does not imply identical numeric results.

## How to implement

For the recorded constant, construct binary64 from the 64-bit pattern `0x400921fb54442d18` or
the equivalent hexadecimal floating-point literal `0x1.921fb54442d18p+1`. Return a numeric value
and reject an argument-bearing call at the formula boundary. Computing π through a platform
inverse-trigonometric function adds an unnecessary dependency on that function's rounding.

Keep a worksheet PI constant separate from a high-precision constant used inside trigonometric
argument reduction. Reducing a large angle with only a rounded binary64 π loses information;
many-word reduction methods address a different problem from returning this worksheet constant.

## Version differences

The raw constant below was captured on Excel 16.0.20527.20088 for Windows. It is not a comparison
of every build. The documentation's fifteen-digit wording describes accuracy at that decimal
scale and does not specify all of the returned bits on every platform.

## Alternatives

[RADIANS](FUNC.RADIANS.md) and [DEGREES](FUNC.DEGREES.md) convert angle units; they have their
own arithmetic and rounding order. [SQRTPI](FUNC.SQRTPI.md) computes a function involving π.
Use a higher-precision representation when the surrounding calculation needs more precision
than binary64 can retain.

## Sources

- [Microsoft: PI function](https://support.microsoft.com/en-us/excel/functions/pi-function).
- Payne and Hanek, *Radian reduction for trigonometric functions* (1983).
