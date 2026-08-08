---
schema: efh.function-page/v1
function_id: FUNC.POWER
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0002
  - EV-MATH-0008
  - EV-STRUCT-0013
  - EV-MATH-0003
open_problems: []
references:
  - work: "Microsoft — WorksheetFunction.Power method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.power"
    role: "documented signature, the \"can be any real number\" wording for the base, and the statement that ^ may be used instead"
  - work: "Microsoft Support — POWER function"
    locator: "https://support.microsoft.com/en-us/office/power-function-d3f2908b-56f4-4c3f-895a-07fb519c362a"
    role: "the worksheet-surface documentation page named in the projection; not retrievable for this pass"
  - work: "OxFunc — EXCEL_MATH_DEVIATION_CATALOG.md"
    locator: "docs/EXCEL_MATH_DEVIATION_CATALOG.md entries XMD-004, XMD-008, XMD-009"
    role: "live-Excel observations of the integer-exponent path, the no-infinities error convention, and the negative-base odd-root path"
  - work: "OxFunc — BUG-FUNC-005 defect stream"
    locator: "docs/bugs/streams/BUG-FUNC-005_power_zero_to_zero_diverges_from_excel.md"
    role: "the live-Excel zero-to-zero observation and the reference engine's correction to it"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 4, sections 4.1-4.2 (logarithm and exponential)"
    role: "the defining identity x^y = exp(y ln x) and its domain"
  - work: "Cody & Waite, Software Manual for the Elementary Functions"
    locator: "chapter on POWER"
    role: "the classical accurate-power algorithm and its extra-precision logarithm"
  - work: "Muller, Elementary Functions: Algorithms and Implementation"
    locator: "chapters on correct rounding and the table maker's dilemma"
    role: "why a correctly rounded pow is hard and what the error amplification is"
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
family: power_fn
role_in_family: "The named power surface; it shares its kernel with the ^ operator and with the financial growth helper, so a change here moves three surfaces."
---

# POWER

## What it computes

`POWER(number, power)` returns `number` raised to `power` — written `x^y` below.

The mathematics is not one function but a family of cases, and every implementation difficulty on
this page comes from the seams between them.

**For `x > 0`, all real `y`:**

    x^y = exp(y · ln x)

This is the definition, not merely an evaluation strategy (A&S 4.2.1). It is smooth in both
arguments and is the only case with no discussion.

**For `x = 0`:**

    0^y = 0        for y > 0
    0^y            undefined (a pole) for y < 0
    0^0            indeterminate

`0^0` is the famous one. Combinatorics and the binomial theorem want `1`; the limit of `x^y`
along the path `y = 1/ln x` is `e`; the limit along `x = 0` is `0`. The two-variable limit does
not exist, so the value is a *convention*, and different systems choose differently. IEEE 754 and
C99 `pow` both specify `pow(0,0) = 1`. Excel does not — see below, and note that this is one of
the very few places where a spreadsheet knowingly departs from the floating-point standard.

**For `x < 0`:** `x^y` has no real value for general real `y`. It has a real value exactly when
`y` is an integer, and — by the odd-root convention — when `y = p/q` in lowest terms with `q`
odd, in which case `x^y = −(|x|^y)`. Off those points the principal complex value
`exp(y·(ln|x| + iπ))` is not real, and the principal branch of the complex logarithm has its cut
along the negative real axis, which is precisely where the base lives. So the negative-base case
is not a bug to be worked around; it is a branch-cut question that every implementation has to
answer explicitly.

**Range.** For `x > 0` the result is positive; for `x < 0` and admissible `y` the result carries
the sign of `(−1)^y`. Overflow and underflow arrive fast: `x^y` exceeds binary64 range as soon as
`y·log₁₀|x| > 308.25`.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The base. Microsoft's VBA page says "it can be any real number". Required. | — |
| `power` | The exponent. Required. | — |

Both are numeric slots under ordinary to-number coercion — see
[Coercion and lifting](../model/02-coercion-and-lifting.md). Text that reads as a number converts;
logicals convert to 1 and 0; an error in either slot propagates.

**A documentation note worth recording.** The VBA page's "can be any real number" is true of the
*argument admission* and false of the *result domain*: a negative base with, say, `power = 0.5`
admits fine and then has no real value. The documentation states no restriction, states no error
condition, and does not mention `0^0` at all. Everything this page says about those cases is
sourced from observation recorded upstream, and is labelled as such.

## Result and edge cases

Returns `Number`.

The reference engine's declared classification for this surface, from
`data/functions/FUNC.POWER.json`, includes one axis that no other surface in this batch carries:

    precision_rounding_profile: integer_exponent_publication

That axis is the projection of OxFunc math-deviation entry **XMD-004**, which records live Excel
evaluating an exact-integer exponent by **repeated multiplication (binary exponentiation)** rather
than through `exp(y·ln x)`. The two routes give different last bits. The consequence for a reader
is concrete: `POWER(1.05, 10)` and `EXP(10*LN(1.05))` are not the same computation, and the first
is the one Excel is reported to perform. The catalogue's evidence line names live Excel 16.0 build
20026.

Boundary cases, each with its source named:

- **`POWER(0, 0)`** — OxFunc's defect stream `BUG-FUNC-005` records live Excel COM replay
  observing `#NUM!` for both `=0^0` and `=POWER(0,0)`, on Excel 16.0 build 19929. The reference
  engine had returned `1` and was corrected to `#NUM!`. Microsoft's documentation says nothing
  about this case. **This is a divergence from IEEE 754 and from C99 `pow`, both of which specify
  1**, and the Handbook records it as such: an Excel-specific convention, observed rather than
  documented.
- **`POWER(0, negative)`** — the same stream records `=0^-1 → #DIV/0!`. The pole is reported as a
  division error rather than a domain error, which is the reading that treats `x^-n` as `1/x^n`.
- **Overflow** — OxFunc catalogue entry **XMD-008** records that Excel never publishes `±Inf` or
  `NaN`: a non-finite real result becomes `#NUM!`, *except* that in `POWER` and `^` an infinity
  arising from a negative exponent over a sub-unit base becomes `#DIV/0!` instead, consistent with
  the `0^negative` rule. `EV-MATH-0008` carries that entry and publishes no count for it.
- **Negative base, reciprocal odd root** — catalogue entry **XMD-009** records Excel routing
  `x < 0` with `y ≈ 1/q` (odd `q`) through the transcendental composite `−exp(y·ln(−x))` rather
  than through a root, so the clean value is not what comes back. The reference engine reproduces
  that route, and additionally reproduces an acceptance test performed on `1/y` rounded to 15
  significant decimal digits — a decimal-shaped tolerance that no binary tolerance reproduces.
- **Arrays** — the projection declares `UnaryNumericScalarOnly` coercion with `surface_native`
  lifting; array arguments are handled by the surface, not by a lift kernel declared on the
  function.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | Either argument does not convert to a number | shared coercion model |
| propagated | An error value in either argument | shared coercion model |
| `#NUM!` | `POWER(0, 0)` | observed live Excel, `BUG-FUNC-005` |
| `#DIV/0!` | Base zero with a negative exponent | observed live Excel, `BUG-FUNC-005` |
| `#NUM!` | Negative base with an exponent having no real root | reference engine; documentation silent |
| `#NUM!` | The real result is not finite (overflow), positive-exponent arm | catalogue `XMD-008` |
| `#DIV/0!` | The real result is not finite, negative-exponent-over-sub-unit-base arm | catalogue `XMD-008` |

**Microsoft's documentation states none of these rows.** The VBA reference lists no error
condition for `Power` at all. Every row above is either shared-model machinery or an upstream
live-Excel observation, and the Handbook has not re-observed any of it.

## Relationships

- **The `^` operator** is the same function by a different name, and Microsoft's page says so
  outright: "The '^' operator can be used instead of **Power**". OxFunc binds both to one kernel.
  But `EV-STRUCT-0013` and `EV-MATH-0003` exist precisely to stop that identity being read as
  shared evidence: the records state that no operator-side measurement exists anywhere upstream,
  that the operator's only surface-specific witnesses sit on a *different Excel build* from the
  function's sign-off build, and that the planned operator sweep was never run. Whether `^` and
  `POWER` publish identical results on a single build is an open question, not a settled one.
- **`EXP` and `LN`** — the pair the fractional path is built from. `POWER(x,y)` and
  `EXP(y*LN(x))` are different computations even when both are defined.
- **`SQRT`** — for `y = 0.5` a hardware square root is correctly rounded and a general power is
  not, which is why `SQRT(x)` and `POWER(x, 0.5)` are worth distinguishing. `EV-MATH-0002` records
  that the exponent-`½`-is-`sqrt` behaviour was *discovered* during the upstream sweep rather than
  assumed.
- **`SQRTPI`**, **`FACT`**, **`GROWTH`/`RRI`/`FV`** — other surfaces whose results are powers.
  OxFunc's presence projection shows the financial growth helper sharing this module's kernel, so
  a change to the power path moves financial results too.
- **Confused with `EXP`.** `EXP(x)` is `e^x`, not `x^e`. The mistake is common enough that it is
  worth the sentence.

## Numerical notes

`pow` is the hardest of the elementary functions to get right, and the reasons are worth stating
because they explain every deviation above.

**Error amplification through the logarithm.** Writing `x^y = exp(y ln x)`, a relative error `ε`
in `ln x` becomes a relative error of about `|y ln x| · ε` in the result. Since `|y ln x|` can
reach 709 before the result overflows, a logarithm accurate to one unit in the last place buys a
result accurate only to about 2⁹·ulp. Every serious implementation therefore computes `ln x` in
extra precision — Cody & Waite use a split representation, fdlibm's `__ieee754_pow` carries about
2⁻⁵³ of extra headroom in a two-word product, and correctly rounded libraries (CRlibm, MPFR) go
further. This is the single fact that explains why "just call `exp` and `ln`" is not an
implementation.

**Integer exponents are a different algorithm, not an optimisation.** Binary exponentiation
performs about `log₂ n` roundings and has a relative error bound of roughly `log₂(n)·u`; the
transcendental route has the amplification above. Neither dominates the other everywhere, and
they disagree in the last bits — which is exactly what XMD-004 records Excel doing, and why
"rounder-looking" answers such as `1.05^10` come out of the multiplication path. An implementation
that wants to reproduce a specific system's answers must reproduce its *dispatch* as well as its
arithmetic: the reference engine's dispatch on exact-integer-ness is exact, so `POWER(2, 3+ulp)`
takes the other road entirely.

**Negative bases are a branch decision.** The odd-root convention is not a mathematical necessity;
it is a choice to prefer the real branch over the principal complex one. Once made, it raises the
question of *when* an exponent counts as a reciprocal odd integer — and the upstream record here
is instructive: the acceptance band was found to be decimal-shaped, widening by a factor of 100
around `1/101` relative to `1/3`, which is the signature of a test performed on the decimal
rendering of `1/y` rather than on its bits.

**Underflow and subnormals.** The integer path and the fractional path do not treat subnormal
results alike in the reference engine — the integer path flushes a subnormal magnitude to zero,
the positive-exponent fractional path publishes subnormals. Anyone porting this behaviour should
treat subnormal handling as a separate axis to pin, not as a consequence of the algorithm.

**Host-CPU sensitivity.** `EV-MATH-0002`'s reader warning records that the `0.5 → sqrt` and
integer paths are CPU-independent while the general fractional path is not, and carries the same
x87 microcode caveat as `EXP` and `LN`. A power result is therefore not necessarily portable
across machines even within one Excel build — which is exactly the distinction the Handbook's
`portable-reproducible` flavour exists to name; see
[About implementation options](../model/07-implementation-options.md).

## What has not been checked

No Handbook vector suite exists for `POWER`. Four evidence records list this surface among their
subjects, and it is worth being precise about what each does and does not establish:

- `EV-MATH-0002` is a live-Excel comparison of the fractional-exponent path. Its own status text
  states that **neither half of the corpus was cleanly held out** from the algorithm it scores —
  one half was the reverse-engineering ground truth, the other is where a second correction was
  discovered and then also scored. It is evidence that the reconstructed algorithm reproduces the
  rows it was built from, not evidence of behaviour on unseen inputs.
- `EV-MATH-0008` publishes the no-infinities convention with named witnesses and **no count at
  all**; its reader warning says any reading of it as numeric-bits evidence would be wrong.
- `EV-STRUCT-0013` and `EV-MATH-0003` are about the `^` operator's relationship to this surface,
  and both exist to record an *absence*: there is no operator-side measurement, and the operator's
  witnesses and the function's sign-off sit on different Excel builds.

So: the fractional path has been compared against Excel upstream under a stated and honest
caveat; the operator has not; and nothing on this page has been re-observed by the Handbook.

Inputs I would probe first:

1. **`POWER(0,0)` and `0^0` on the current build**, alongside `POWER(0,-1)`, `POWER(0,1)` and
   `POWER(1,0)`. The zero lane is the cheapest place where Excel is reported to depart from IEEE
   754, and it is a four-cell experiment.
2. **`POWER(1.05,10)` against `EXP(10*LN(1.05))` and against `1.05^10`**, which separates the
   integer-exponent dispatch from the transcendental path *and* tests the operator/function
   identity in the same formula.
3. **`POWER(2, 3)` versus `POWER(2, 3+2^-50)`** — the dispatch boundary. If the exponent test is
   exact, these two land on different algorithms and the results differ visibly in the last bits.
4. **`POWER(-8, 1/3)` and `POWER(-27, 1/3)`**, then `POWER(-8, 1/101)` and its immediate
   neighbours, which probe both the odd-root convention and the decimal-shaped acceptance band.
5. **`POWER(10,700)` and `POWER(0.001,-700)`** — the two arms of the overflow convention, which
   are documented nowhere and differ from each other in error code.
6. **`POWER(2,-1022)` and `POWER(2,-1023)`** — the subnormal boundary on the integer path.
7. **The largest-finite-double row in the probe battery rendered on this page.** Its outcome is a
   `NaN` published as a number rather than an error value; XMD-008 records Excel converting every
   non-finite real result to an error. The Handbook records this as an open divergence between the
   reference engine and the catalogue's own account of Excel, and it is worth an Excel probe of
   `POWER(1.7976931348623157E+308, 1.7976931348623157E+308)` to settle which side is wrong.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| integer-exponent publication | Evaluating `x^n` by repeated squaring rather than `exp(n ln x)` |
| odd-root convention | Giving `x^(1/q)` a real negative value for negative `x` and odd `q` |
| branch cut | The discontinuity of the complex logarithm along the negative real axis |
| error amplification | The factor `\|y ln x\|` by which a logarithm's error grows in the result |
| dispatch | The test that selects which of several algorithms evaluates a given input |

## Sources

- Microsoft, "WorksheetFunction.Power method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.power> (signature,
  "any real number" for the base, and the `^` equivalence; no error conditions are stated).
- Microsoft Support, "POWER function" —
  <https://support.microsoft.com/en-us/office/power-function-d3f2908b-56f4-4c3f-895a-07fb519c362a>
  (retrieval refused with HTTP 403 during this pass; nothing here is sourced from it).
- OxFunc `docs/EXCEL_MATH_DEVIATION_CATALOG.md`, entries XMD-004 (integer-exponent publication),
  XMD-008 (no infinities; the `#NUM!` / `#DIV/0!` split) and XMD-009 (negative-base reciprocal odd
  root), each carrying its own live-Excel evidence line.
- OxFunc `docs/bugs/streams/BUG-FUNC-005_power_zero_to_zero_diverges_from_excel.md` — the
  zero-to-zero observation and correction.
- Handbook evidence records `EV-MATH-0002`, `EV-MATH-0008`, `EV-STRUCT-0013`, `EV-MATH-0003`,
  rendered beside this page with their own reader warnings.
- Abramowitz & Stegun chapter 4; Cody & Waite, *Software Manual for the Elementary Functions*;
  Muller, *Elementary Functions: Algorithms and Implementation*; fdlibm `__ieee754_pow`.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md),
  [Claim language and honesty](../model/06-claim-language.md),
  [About implementation options](../model/07-implementation-options.md).
- Handbook projections `data/functions/FUNC.POWER.json` (the `integer_exponent_publication`
  precision axis) and `data/presence/FUNC.POWER.json` (module, shared kernel, defect streams
  `BUG-FUNC-005`, `BUG-FUNC-015`, `BUG-FUNC-027`, `BUG-FUNC-042`).
