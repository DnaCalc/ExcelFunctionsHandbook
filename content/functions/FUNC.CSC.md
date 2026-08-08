---
schema: efh.function-page/v1
function_id: FUNC.CSC
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0004
open_problems: []
references:
  - work: "Microsoft Support — CSC function"
    locator: "https://support.microsoft.com/en-us/office/csc-function-07379361-219a-4398-8675-07ddc4f135c1"
    role: "documented signature, the magnitude constraint, the #NUM! and #VALUE! conditions, and the CSC(n) = 1/SIN(n) statement"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 4.3 (Circular Functions), 4.3.68 for the cosecant expansion"
    role: "definition, pole structure and the Laurent expansion at the origin"
  - work: "Payne & Hanek, Radian reduction for trigonometric functions (SIGNUM Newsletter, 1983)"
    locator: null
    role: "the argument reduction that survives large arguments"
  - work: "Muller, Elementary Functions: Algorithms and Implementation"
    locator: "the argument-reduction chapters"
    role: "the error analysis behind a documented magnitude bound"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: csc
role_in_family: >-
  The circular cosecant — the reciprocal of the sine, and the surface whose own documentation
  pins its implementation identity while leaving its pole at the origin unstated.
---

# CSC

## What it computes

`CSC(number)` returns the cosecant of an angle in radians. Microsoft's page states the
identity outright: `CSC(n)` equals `1/SIN(n)`.

    csc x = 1 / sin x

| Property | Statement |
|---|---|
| Domain | all real `x` except `x = kπ`, `k ∈ ℤ` |
| Range | `(-∞, -1] ∪ [1, ∞)` — `ABS(csc x) ≥ 1` everywhere |
| Parity | odd: `csc(-x) = -csc(x)` |
| Period | `2π` |
| Poles | simple, at `x = kπ`, residue `(-1)^k` |
| Extrema | `csc x = ±1` at `x = π/2 + kπ`; these are the local extrema, not zeros |
| Laurent series at 0 | `csc x = 1/x + x/6 + 7x³/360 + …` (A&S 4.3.68) |
| Derivative | `d/dx csc x = -csc x · cot x` |
| Identity | `csc²x - cot²x = 1` |
| Zeros | none — the cosecant is never zero |

That last row is the reader's most useful fact: **`CSC` has no zeros**, so a `CSC` result near
zero is impossible and a `CSC` result of small magnitude is a bug. The function's values are
bounded away from the origin and unbounded near every multiple of `π`.

As with [COT](FUNC.COT.md), the poles matter less than they look. `π` is irrational, so the
only pole that is a binary64 value is `x = 0`; at every other multiple of `π` the function
returns a large finite number whose magnitude is roughly the reciprocal of the distance from
the argument to the true pole.

## Arguments

| Argument | Meaning | Constraint (as documented) |
|---|---|---|
| `number` | The angle in radians. Required. | `ABS(number) < 2^27` |

Microsoft documents the magnitude constraint and `#NUM!` beyond it. The reference engine
carries `arg_domain_guard=circular_trig_overflow`, consistent with the documented bound — the
same axis value as [COS](FUNC.COS.md) and [COT](FUNC.COT.md), and unlike the hyperbolic
[COTH](FUNC.COTH.md) and [CSCH](FUNC.CSCH.md), whose pages document the same bound while their
axes record no guard.

The slot takes ordinary to-number coercion; the surface is declared an elementwise scalar
kernel. See [Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number` with absolute value at least 1.

- **`CSC(0)`** — `1/SIN(0)` is a division by zero. Microsoft's page **does not state the
  resulting error**, where [COT](FUNC.COT.md)'s page states `#DIV/0!` for the analogous case.
  `#DIV/0!` follows from the documented identity, but the identity is a definition, not an
  error specification, and the Handbook has not checked it. Recorded as a documentation gap.
- **Near other poles** — a large finite value, sign alternating with the parity of `k`.
- **`ABS` result below 1** — impossible mathematically. A returned value in `(-1, 1)` would be
  a defect, and this is the cheapest invariant to test.
- **Beyond `2^27`** — documented `#NUM!`.
- **Small arguments** — `csc x → 1/x`, a huge finite number, overflowing only in the subnormal
  range.

## Errors

As documented on Microsoft's `CSC` page:

| Error | Condition |
|---|---|
| `#NUM!` | `ABS(number)` is not less than `2^27` |
| `#VALUE!` | `number` is non-numeric |

The zero case is not documented on that page. See the note above.

## Relationships

- **`SIN`** — the reciprocal, named by the documentation itself. `EV-MATH-0004` records that
  the three reciprocal surfaces are built as `excel_x87_recip` of the *published* primaries,
  which is exactly what the documented identity `CSC(n) = 1/SIN(n)` describes: the reciprocal
  of the already-rounded binary64 sine, not an independently computed cosecant.
- **`SEC`, [COT](FUNC.COT.md)** — the other two reciprocals from the same 2013 batch, sharing
  the documentation template and the `2^27` bound.
- **[COS](FUNC.COS.md), `TAN`** — the primaries. `csc x = sec(π/2 - x)`.
- **`ASIN`** — there is no `ACSC` on the worksheet surface; `ASIN(1/x)` is the substitute.
- **[CSCH](FUNC.CSCH.md)** — the hyperbolic partner, `csch x = -i·csc(ix)`, sharing the `1/x`
  leading term and the pole at the origin.
- **`IMCSC`** — the complex version.

## Numerical notes

The documented identity is unusually informative, because it tells you what the error looks
like without needing an oracle.

**Double rounding is structural here.** If `CSC(x)` really is `1/SIN(x)` evaluated on the
published binary64 sine, then the cosecant's error is the sine's *relative* error, inverted,
plus the rounding of one division. Two consequences follow. The good one: the relative error of
`CSC` is bounded by the relative error of `SIN` plus half an ulp, so `CSC` is never much worse
than `SIN` in relative terms. The bad one: it is never *better* either, and near the poles the
sine's own relative error is at its worst, because that is where `sin` is small and its
argument reduction dominates.

**The reduction problem, inherited.** `EV-MATH-0004` identifies the substrate as the legacy CRT
`fFSIN`/`fFPTAN` chain with `FPREM1` reduction against the 64-bit ROM `π`, rules out Cody–Waite
reduction against extended `π`, and attaches a host-CPU microcode caveat to all six circular
surfaces. The record states each of the six was scored individually, so `CSC` is a measured
surface rather than one whose status was inherited from `SIN`. Its figures render in the
evidence layer beside this page.

**Where the amplification lives.** Near a pole `kπ`, write `x = kπ + δ`. Then
`csc x ≈ (-1)^k / δ`. The reduced argument `δ` is computed by subtracting a stored multiple of
`π` from `x`, so its *absolute* error is roughly `ABS(x) · 2^-p` for a `p`-bit reduction
constant. The relative error of the result is therefore about `ABS(x)·2^-p / ABS(δ)` — it grows
with the argument and blows up as the argument approaches a pole. This is the analysis that
makes a documented `2^27` bound a reasonable engineering decision rather than an arbitrary one:
past that magnitude the reduced argument carries no reliable bits at all.

**Near zero**, use `1/x + x/6 + 7x³/360` (A&S 4.3.68) rather than a division, both for speed and
because it gives the deviation from `1/x` directly.

**A `natural-best` implementation** would not use the published sine. It would perform one
reduction, evaluate `sin r` in extended precision, and reciprocate before rounding — one
rounding instead of two, and the pole located consistently with the reduction. That is a
different function from the documented one, which is exactly why the flavour distinction in
[About implementation options](../model/07-implementation-options.md) exists.

## What has not been checked

`EV-MATH-0004` names this surface as one of six scored individually. Its reader warning is
important here: the figure the upstream catalogue restates is a **group total** across all six
circular functions and is not a per-surface rate; the per-surface figures live in the record's
identification note. All six inherit the host-CPU microcode caveat on the reducing and
evaluating instructions.

No Handbook vector suite exists for `CSC`. The magnitude constraint, the two error conditions
and the `CSC(n) = 1/SIN(n)` identity are documented by Microsoft. The `CSC(0)` case is not
documented anywhere, and the Handbook has not observed it.

Inputs I would probe first:

1. **`CSC(0)`, and `CSC(-0)`.** The undocumented pole, and the cheapest gap on this page to
   close. `#DIV/0!` is expected from the documented identity; expectation is not evidence.
2. **`CSC(x) · SIN(x) - 1`** across the range. If the documented reciprocal construction is
   literal, this is the rounding of a single division and is tightly bounded everywhere,
   including near the poles where both factors are extreme. A larger residual would mean `CSC`
   is not simply `1/SIN` of the published value, and would contradict the documentation.
3. **The documented bound**: `CSC(2^27 - 1)`, `CSC(2^27)`, `CSC(-2^27)` — strictness and sign
   symmetry of the guard, answered in error codes.
4. **The range invariant**: sweep and assert `ABS(CSC(x)) ≥ 1`. Any violation is a defect that
   needs no oracle to detect.
5. **Pole approach**: the doubles nearest `π`, `2π`, `100π`, and near `2^26`. The magnitude
   returned reads off the residual precision of the reduction directly.
6. **`CSC(x)` against `1/SIN(x)` typed as a worksheet formula.** If these ever differ, the
   documentation's identity is not the implementation, which would be a first-order finding.
7. **The same vectors on a second CPU vendor**, given the microcode caveat.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| reciprocal construction | Forming `CSC` as `1/SIN` of the already-rounded published sine |
| double rounding | Two roundings — the sine's, then the division's — in one published value |
| published primary | The binary64 `SIN` value Excel returns, before any reciprocal |
| amplification near a pole | Growth of relative error as the argument approaches `kπ` |
| range invariant | `ABS(csc x) ≥ 1`, testable without any oracle |
| host-scoped | A result whose last bits depend on the CPU executing it |

## Sources

- Microsoft, "CSC function" —
  <https://support.microsoft.com/en-us/office/csc-function-07379361-219a-4398-8675-07ddc4f135c1>
  (fetched at curation: signature, the `2^27` constraint, the `#NUM!` and `#VALUE!` conditions,
  and the statement that `CSC(n)` equals `1/SIN(n)`. The zero case is not documented there).
- Handbook evidence record `EV-MATH-0004` — the six-surface circular identification, the
  `FPREM1`-against-ROM-`π` substrate, the reciprocal construction from published primaries, the
  ruled-out Cody–Waite reduction, and the host-CPU microcode caveat. Read its reader warning.
- Abramowitz & Stegun, chapter 4.3, in particular 4.3.68 — the cosecant expansion.
- Payne & Hanek (1983); Muller, *Elementary Functions* — reduction and its error analysis.
- Handbook, [COS](FUNC.COS.md) and [COT](FUNC.COT.md) — the shared reduction discussion;
  [About implementation options](../model/07-implementation-options.md);
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.CSC.json` (the
  `arg_domain_guard=circular_trig_overflow` and `non_finite=num` axis values) and
  `data/presence/FUNC.CSC.json`.
