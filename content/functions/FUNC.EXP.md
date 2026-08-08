---
schema: efh.function-page/v1
function_id: FUNC.EXP
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0001
  - EV-MATH-0008
open_problems: []
references:
  - work: "Microsoft Support — EXP function"
    locator: "https://support.microsoft.com/en-us/office/exp-function-c578f034-2c45-4c37-bc8c-329660a63abe"
    role: "documented signature, the value of e, the inverse relationship to LN, and the pointer to ^ for other bases"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 4.2 (Exponential Function), 4.2.1-4.2.45"
    role: "definition, series, identities, and the classical polynomial approximations"
  - work: "Tang, Table-driven implementation of the exponential function in IEEE floating-point arithmetic (ACM TOMS, 1989)"
    locator: null
    role: "the table-driven reduce-approximate-reconstruct algorithm that most modern libms use"
  - work: "Cody & Waite, Software Manual for the Elementary Functions"
    locator: "the EXP chapter"
    role: "the classical two-part reduction x = k*ln2 + r and its error analysis"
  - work: "fdlibm (Sun Microsystems freely distributable libm)"
    locator: "e_exp.c and s_expm1.c"
    role: "the reference branch structure, and the expm1 companion that cancellation demands"
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
family: exp_fn
role_in_family: >-
  The natural exponential — the substrate under most of the hyperbolic and financial surfaces,
  and the surface whose evaluation route has been identified most precisely of any in this batch.
---

# EXP

## What it computes

`EXP(number)` returns `e` raised to the power of its argument. Microsoft's page gives the
constant to fifteen digits and states the two relationships that matter to a reader: `EXP` is
the inverse of `LN`, and powers of other bases go through the `^` operator.

    exp(x) = e^x = Σ_{n≥0} x^n / n!

| Property | Statement |
|---|---|
| Domain | all real `x` |
| Range | `(0, ∞)` — strictly positive, never zero, never negative |
| Series | `1 + x + x²/2! + x³/3! + …`, convergent for every `x` |
| Defining property | the unique solution of `f' = f` with `f(0) = 1` |
| Functional equation | `exp(x+y) = exp(x)·exp(y)` |
| Inverse | `ln`, on `(0, ∞)` |
| Fixed point | none on the reals; `exp(x) > x` everywhere |
| Entire | no poles, no branch cuts; periodic with period `2πi` in the complex plane |
| Overflow threshold | `ln(DBL_MAX) ≈ 709.782712893384` |
| Underflow to zero | below about `-745.133` the true value is under half the smallest subnormal |
| Gradual underflow | between about `-708.396` and `-745.133` the result is subnormal and loses bits |

The three thresholds in the last rows are the whole edge-case story, and they are properties of
binary64 rather than of any implementation. Note the asymmetry: the overflow cliff is sharp —
one argument returns a finite value, the next does not — while the underflow side degrades
gradually through the subnormal range before reaching zero.

## Arguments

| Argument | Meaning | Notes |
|---|---|---|
| `number` | The exponent applied to base `e`. Required. | Microsoft documents no constraint |

Ordinary to-number coercion applies; the reference engine declares the surface a scalar kernel
that lifts elementwise over arrays. See
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`, strictly positive.

- **`EXP(0)`** — exactly 1. This is the only argument whose result is exact for a reason.
- **`EXP(1)`** — the double nearest `e`. Microsoft's page prints `e` to fifteen digits, which is
  the value's decimal shadow rather than its bits.
- **Overflow** — beyond `ln(DBL_MAX)` the true value is not representable. The reference
  engine records `non_finite=num` for this surface, and `EV-MATH-0008` records the convention
  that a non-finite real result surfaces as an error rather than as an infinity, with a large
  `EXP` argument among its named witnesses. That record publishes **no count**; it establishes
  where an error code appears, with witnesses and no denominator.
- **Underflow** — the result becomes subnormal and then zero. Zero is outside the mathematical
  range of the function, so `EXP` returning `0` is the format speaking, not the function.
  Whether Excel returns `0` or an error there is not documented and is on the probe list.
- **Never negative, never zero for moderate arguments.** A negative result would be a defect,
  and the invariant costs nothing to test.
- **Small arguments** — `EXP(x)` for tiny `x` returns a value very close to 1, and the
  *interesting* quantity `EXP(x) - 1` is then destroyed by cancellation. See the numerical
  notes: this is the single most consequential accuracy issue on the worksheet surface, and it
  is not a defect in `EXP`.

## Errors

Microsoft's `EXP` page documents no error conditions.

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | Non-numeric argument | Shared call model, not this page |
| `#NUM!` | The true result is not finite | `EV-MATH-0008`, which names `EXP` — a kind and error-code convention, no count |

## Relationships

- **`LN`** — the documented inverse. `EV-MATH-0001` treats `EXP`, `LN`, `LOG10` and `LOG`
  together as one identification problem, which is the right way to read them.
- **`POWER` and `^`** — Microsoft's page routes other bases here. Note that `POWER(EXP(1), x)`
  and `EXP(x)` are different computations and need not agree in the last bit.
- **[COSH](FUNC.COSH.md), `SINH`, `TANH`, [COTH](FUNC.COTH.md), [CSCH](FUNC.CSCH.md), `SECH`** —
  every hyperbolic surface is built on this one. Their accuracy is bounded by its accuracy, and
  their branch structures exist to keep its overflow out of their expressions.
- **`LOG`, `LOG10`** — the other members of the log/exp identification.
- **Financial surfaces** — `PMT`, `RATE`, `PV`, `FV`, `NPER`, `PDURATION` and friends evaluate
  `(1+r)^n` compounding, which is exponential arithmetic in disguise; the presence projection
  attaches an annuity-exactness defect stream that mentions this module.
- **`GAMMALN`, and through it [COMBINA](FUNC.COMBINA.md)** — the `exp(lnΓ)` composition, whose
  final step is this function.
- **`EXPON.DIST`, `LOGNORM.DIST`, `NORM.DIST`, `POISSON.DIST`** — distributions whose kernels
  are exponentials, and where the cancellation issue below is the difference between a usable
  and an unusable tail.

## Numerical notes

The exponential is the best-studied function in numerical analysis, and it is one of the few
where the modern state of the art is well ahead of what most software actually ships.

**The standard algorithm**, in three stages, essentially unchanged since Cody & Waite and
refined by Tang:

1. **Reduce.** Write `x = k·ln2 + r` with `k = round(x/ln2)` and `ABS(r) ≤ ln2/2`. Because
   `ln2` is irrational, this needs `ln2` split into a high part exactly representable and a low
   correction, so that `x - k·ln2_hi - k·ln2_lo` is computed without losing bits.
2. **Approximate.** Evaluate `e^r` on the small interval by a minimax polynomial or rational —
   A&S 4.2.44–4.2.45 gives classical coefficient sets; Tang's table-driven variant reduces
   further onto a table of `2^{j/32}` values so the polynomial degree drops.
3. **Reconstruct.** Scale by `2^k`, exactly, by adding to the exponent field. The only care
   needed is at the boundaries, where `2^k` itself would overflow or where the result is
   subnormal and the scaling must be split in two.

fdlibm's `e_exp.c` is the reference implementation of this shape; every step's error budget is
documented there.

**What the evidence says about Excel's route.** `EV-MATH-0001` is a live-verification record
naming `EXP` as one of four log/exp surfaces, and its substrate identification is unusually
specific: the legacy Microsoft x87 CRT transcendental sequence — the `87tran.asm` `fFEXP`
routine, driven by the `F2XM1` and `FYL2X` instructions with a named control word, and one final
store to binary64. It records that **every modern libm is ruled out at the bit level** for these
surfaces, including UCRT, glibc, MKL-VML, SVML, and the correctly-rounded value itself. The
record also carries an honest structural caveat: the source calls its sweep "fresh" and never
uses the words "held out", so the held-out state is published as partial; and the per-surface
figures do not reconcile with the sweep total the source names, with the record publishing that
gap as an open question rather than smoothing it. Every one of the four surfaces inherits a
host-CPU microcode caveat on `F2XM1`/`FYL2X`: an answer produced through those instructions is a
property of the CPU as much as of the software. All figures live in the evidence layer and
render beside this page.

**The `expm1` problem, which is the practically important one.** For small `x`, `e^x` is close
to 1, so `EXP(x) - 1` loses every leading digit to cancellation: at `x = 10^-10` about ten
significant digits are gone, and at `x = 10^-16` the answer is zero. This matters constantly —
continuous compounding, hazard rates, small-return finance, `1 - EXP(-λt)` survival terms — and
**Excel has no `EXPM1` on the worksheet surface**. The remedies available in a worksheet:

- use the series `x + x²/2 + x³/6` for small `ABS(x)`, which is accurate and cheap;
- restructure algebraically so the subtraction never appears;
- for `1 - EXP(-x)`, use `-EXPM1(-x)` conceptually and implement it as the series.

The Handbook's "The Last Bit" series treats the `expm1` family in the financial surfaces at
length; the phenomenon is the same one, and the fact that the worksheet exposes `EXP` but not
`EXPM1` is a design gap rather than an accuracy defect.

**Argument reduction here is easy, unlike the trigonometric case.** The reduction constant is
`ln2`, and `k` is bounded by about 1024, so `k·ln2` never needs many extra bits. That is why
`EXP` has no analogue of [COS](FUNC.COS.md)'s large-argument catastrophe: the exponential's
domain is bounded by overflow long before reduction precision becomes an issue.

## What has not been checked

Two evidence records name this surface. `EV-MATH-0001` carries a per-surface live-Excel count
for `EXP` and a specific substrate identification, together with a reader warning: two further
figures in the same upstream lane are **not** surface pass rates — one scores a backend module
over two surfaces jointly, the other scores an out-of-repo reference implementation — and none
of them may be attributed to any single surface. The record also publishes the unreconciled
sweep-total gap as an open question, and attaches the host-CPU microcode caveat.
`EV-MATH-0008` names `EXP` in the FINITE error-code convention family and publishes no count at
all.

No Handbook vector suite exists for `EXP`. Microsoft's page documents the inverse relationship
to `LN` and the pointer to `^`, and no error conditions or thresholds.

Inputs I would probe first:

1. **The overflow boundary, bisected**: `EXP(709.78)`, `EXP(709.7827128933839)`,
   `EXP(709.7827128933841)`, `EXP(710)`. The last finite argument locates the boundary exactly,
   and the answer changes kind rather than value, so it settles without any bit comparison.
2. **The underflow ladder**: `EXP(-708)`, `EXP(-745)`, `EXP(-745.2)`, `EXP(-746)`. Three
   distinct behaviours are possible — normal, subnormal, zero — and a fourth, `#NUM!`, would be
   a finding, since zero is a perfectly good finite value and the FINITE convention would not
   require an error.
3. **The subnormal band**, where results lose significand bits gradually. An implementation
   that scales by `2^k` in one step returns zero across the whole band; one that splits the
   scaling keeps the digits. The two are distinguishable by the *ratio* `EXP(-720)/EXP(-721)`,
   which should be close to `e`.
4. **`EXP(x) · EXP(-x) - 1`** across the range — a metamorphic probe requiring no oracle, and
   one whose residual pattern reflects the reduction's rounding directly.
5. **`LN(EXP(x)) - x`** and `EXP(LN(y)) - y` — the documented inverse relation, and the natural
   joint probe for the four surfaces `EV-MATH-0001` identifies together.
6. **Small arguments**: `EXP(2^-30)`, `EXP(2^-52)`, `EXP(2^-60)`. The first argument for which
   `EXP(x)` returns exactly 1 marks where the worksheet's missing `EXPM1` starts costing users
   everything, and it is worth publishing for that reason alone.
7. **The same vectors on a second CPU vendor**, given the explicit `F2XM1`/`FYL2X` microcode
   caveat. Any difference is a platform axis rather than a defect.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| argument reduction | Writing `x = k·ln2 + r` so the polynomial only sees a small `r` |
| table-driven | Tang's refinement: an extra reduction onto a table of `2^{j/N}` values |
| reconstruction | Scaling the approximated `e^r` by `2^k` through the exponent field |
| `expm1` | `e^x - 1` computed without cancellation; absent from the worksheet surface |
| gradual underflow | The subnormal band where results lose significand bits before reaching zero |
| FINITE convention | The recorded convention that a non-finite real result surfaces as an error |
| host-scoped | A result whose last bits depend on the CPU executing it |

## Sources

- Microsoft, "EXP function" —
  <https://support.microsoft.com/en-us/office/exp-function-c578f034-2c45-4c37-bc8c-329660a63abe>
  (fetched at curation: signature, the printed value of `e`, the statement that `EXP` is the
  inverse of `LN`, and the pointer to the `^` operator for other bases. No error conditions are
  documented there).
- Handbook evidence records `EV-MATH-0001` (the four-surface log/exp live verification, the x87
  CRT substrate identification, the ruled-out modern libms, the partial held-out state, the
  unreconciled sweep total, and the microcode caveat) and `EV-MATH-0008` (the FINITE error-code
  convention, no count). Read both reader warnings.
- Abramowitz & Stegun, chapter 4.2 — the exponential function and its approximations.
- Tang, *Table-driven implementation of the exponential function in IEEE floating-point
  arithmetic*, ACM TOMS (1989).
- Cody & Waite, *Software Manual for the Elementary Functions* — the classical reduction.
- fdlibm `e_exp.c` and `s_expm1.c`.
- Handbook, [COSH](FUNC.COSH.md) and [COMBINA](FUNC.COMBINA.md) — two surfaces that inherit this
  one's accuracy; [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.EXP.json` (the `non_finite=num` axis value) and
  `data/presence/FUNC.EXP.json` (implementing module and the annuity-exactness defect stream
  that mentions it).
