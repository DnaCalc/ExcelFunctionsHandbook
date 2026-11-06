---
schema: efh.function-page/v1
function_id: FUNC.BESSELK
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MISC-0003
open_problems: []
references:
  - work: "Microsoft Support — BESSELK function"
    locator: "https://support.microsoft.com/en-us/office/besselk-function-606d11bc-06d3-4d53-9ecb-2803e2b90b70"
    role: "syntax, the truncation of n, the #VALUE! and #NUM! remarks and the worked example; its defining equation is an image"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 9"
    locator: "9.8.5 to 9.8.8 — polynomial approximations to K0 and K1"
    role: "the published seven-digit polynomials the recorded answers follow"
  - work: "NIST Digital Library of Mathematical Functions, chapter 10"
    locator: "https://dlmf.nist.gov/10.25"
    role: "definitions, recurrence, cross-product and limiting forms of the modified Bessel functions"
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
family: bessel_convert_family
role_in_family: >-
  The decaying partner of BESSELI, defined only for positive x, and the member whose recurrence
  runs upward, in the stable direction, from two polynomial starting values.
---

# BESSELK

## What it computes

`BESSELK(x, n)` returns **K_n(x)**, the modified Bessel function of the second kind, of integer
order n at the real point x > 0. It is the second solution of the modified Bessel equation

    x² w″ + x w′ − (x² + n²) w  =  0

the one that decays as x grows, where [BESSELI](FUNC.BESSELI.md) is the one that stays finite at
the origin. Microsoft's page defines it through the J and Y Bessel functions at an imaginary
argument, in an equation given as an image. The standard form of that relation is

    K_n(x)  =  (π/2) · i^(n+1) · [ J_n(i·x) + i · Y_n(i·x) ]

For real x > 0 a more useful form is the integral

    K_n(x)  =  ∫₀^∞  e^(−x cosh t) cosh(n t) dt

which makes three things visible at once: K_n(x) is positive, it falls as x grows, and it rises as
the order rises.

- **Decay.** For large x, K_n(x) ≈ √(π/(2x)) · e^(−x). As with I_n, every order looks alike at
  that scale. K_0(100) is already about 4.7E-45.
- **The singularity at 0.** K_0(x) ≈ −ln x as x → 0, and K_n(x) ≈ ½ (n−1)! (2/x)^n for n ≥ 1. The
  function goes to infinity at the origin, faster at higher order, and has no value there.
- **Negative x.** K_n has a branch point at 0 and is not real on the negative axis. There is no
  parity rule of the kind I_n has, which is why this function has a smaller domain than `BESSELI`.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `x` | The value at which to evaluate the function. Required. | — |
| `n` | The order of the function. Required. Truncated if not an integer. | — |

Two arguments, both required; the projection declares an arity of exactly 2.

**`x` must be positive, and the documentation does not say so.** Microsoft's remarks cover only a
nonnumeric argument and `n < 0`. The recorded answers show `#NUM!` on all 24 calls with `x ≤ 0` and
a non-negative order, including `x = 0` at orders 0, 1, 2 and 10 (Excel 16.0 build 20430, Windows
64-bit, CPU not recorded).

**Truncation and the negative test.** As with `BESSELI`, the sign test sees `n` before truncation.
All 51 recorded calls with `x > 0` and `n` strictly between −1 and 0 returned `#NUM!`, although
truncation would give order 0. Positive fractions truncate as documented, and `BESSELK(x, 0.9)`
reads the same as `BESSELK(x, 0)` at the grid points where both were asked.

`BESSELK` is not a function of real order, even though K_ν is perfectly well defined for real ν.
`BESSELK(x, 1.5)` is K_1(x), not the closed-form K_(3/2)(x).

The slots are numeric. No reading of text, a logical, an empty cell or an array in either slot is
on record for this function; see *What has not been checked*.

## Result and edge cases

Returns `Number`. Unless stated otherwise, the behaviour below is read from the 1,236 recorded
answers on build 20430.

- **The documented example** `BESSELK(1.5, 1)` is printed by Microsoft as `0.277387804`. The true
  value is 0.2773878005, so the printed value differs from it in the ninth significant digit. This
  is the function's normal accuracy, not an error in the documentation.
- **Small `x`, ordinary order** behaves correctly and the numbers grow quickly. The smallest
  positive `x` read is 0.001, where order 10 is about 1.9E38, the largest value in the recording.
- **Large `x`** decays toward the bottom of the range. The largest `x` read is 100, where order 0
  is about 4.7E-45. In the recording no call with `x > 0` and a non-negative order raised an error,
  and neither end of the range was reached.
- **Overflow and underflow.** K_n is not representable for high order at small `x`, or at `x` beyond
  about 740, where K_0 rounds to 0 below the smallest subnormal. How Excel reports either has not
  been read. `BESSELI` raises `#NUM!` well before its own limit, so a silent 0 cannot be assumed
  here.

## Errors

As documented on Microsoft's page:

| Error | Condition |
|---|---|
| `#VALUE!` | `x` is nonnumeric |
| `#VALUE!` | `n` is nonnumeric |
| `#NUM!` | `n < 0` |

As recorded, in addition to the documented `#NUM!` for `n ≤ −1` (build 20430):

| Error | Condition | Rows |
|---|---|---|
| `#NUM!` | `x ≤ 0` at any non-negative order | 24 |
| `#NUM!` | `n` strictly between −1 and 0, with `x > 0` | 51 |

The documentation states neither row. The first is forced by the mathematics and is what any
reader would expect; it is still a gap in the page. The second is a choice, and an implementation
that truncates first gets it wrong.

The reference engine does exactly that at the commit this page was written against. It rejects
`x ≤ 0` as recorded, but it truncates the order before testing its sign, so an `n` between −1 and 0
is evaluated as order 0 rather than raising `#NUM!`. It raises `#NUM!` when the computed result is
not finite. Those are read from its source, and they are facts about that engine, not about Excel.

## Relationships

- **[BESSELI](FUNC.BESSELI.md)** — the growing solution of the same equation. The two are tied by
  the exact cross-product I_n(x)·K_(n+1)(x) + I_(n+1)(x)·K_n(x) = 1/x, a useful check that needs no
  oracle. In the form described under *Numerical notes*, K_0 and K_1 use I_0 and I_1 for x ≤ 2, so
  the two functions are not independent there. Their accuracy note is shared.
- **`BESSELJ` and `BESSELY`** — the ordinary Bessel functions, through which Microsoft defines this
  one. Y_n has the same singularity at the origin. All four share the implementing module in the
  reference engine; module sharing is an implementation fact and implies nothing about Excel's code.
- **The recurrence.** K_(n+1)(x) = K_(n−1)(x) + (2n/x)·K_n(x). Every term is positive, which is why
  stepping upward in n is safe here and unsafe for I_n.
- **Where readers meet it.** K_0 is the kernel of the two-dimensional screened Poisson equation and
  the steady-state well function of a leaky aquifer, and K_1 appears in the density of the
  normal-inverse-Gaussian distribution.

## Numerical notes

**Low-order polynomials, by design.** The notes below record that `BESSELK` is correctly rounded on
0 of 1,155 recorded rows, with a largest relative error of 1.7E-7. Treat it as a seven-digit
function: the same order of accuracy as `BESSELI`, and for the same reason.

**The form the recorded answers follow.** The notes do not give a staging for this function. The
published candidates are the polynomials of Abramowitz & Stegun 9.8.5 to 9.8.8: for x ≤ 2 a
polynomial in (x/2)² plus a ln(x/2) term multiplied by the polynomial I_0 or I_1, and for x > 2 a
polynomial in 2/x multiplied by e^(−x)/√x. Orders 2 and above then step upward from K_0 and K_1.
While writing this page that recipe was transcribed in plain binary64 and compared with the
recorded rows: it gave the same double on 1,141 of the 1,155 numeric rows and came within 10 units
on the other 14 (Excel 16.0 build 20430, Windows 64-bit, CPU not recorded). That is strong evidence
for the form, and it is a spot check on the same rows, not a held-out measurement. It says nothing
about the units in the remaining 14, which is where a staging difference such as the x87 storage
the notes describe for `BESSELI` would show.

**Upward is the stable direction.** K_n grows with n, so stepping up amplifies nothing dangerous.
There is no starting point to choose and no analogue of `BESSELI`'s large-`x` defect. The relative
error at orders 2 and above in the recording comes from K_0 and K_1 and thins out as the order
rises: the worst row at each order falls from about 1.7E-7 at order 1 to between 1E-8 and 2.4E-8 at
orders 12 and above.

**Where the largest errors sit.** The worst rows in the recording are at order 1, and then order 0,
with x near 38 to 40. That is a swing of the large-`x` polynomial's own error curve, with nothing
to suggest a defect.

**There is a seam at x = 2.** Evaluating the printed coefficients on both sides of 2 gives a jump
of about 6.7E-8 relative in K_0 and 1.1E-8 in K_1. That is arithmetic on the published coefficients:
no recorded row sits within 0.0003 of 2. The matching step in `BESSELI` is visible in Excel's
answers at adjacent doubles, so there is every reason to expect this one too.

**Reference engine.** At the commit named under Sources, the reference engine implements the same
polynomials and the same upward recurrence in plain binary64.

## What has not been checked

No Handbook vector suite exists for `BESSELK`; `vectors/` publishes nothing at this revision, so no
suite-scoped claim exists for it.

The numeric rows are recorded on one Excel build, 20430, on 64-bit Windows with the CPU not
recorded. Unlike `BESSELI`, this function has no later readings on record. No reading from a Mac,
Excel for the web, 32-bit Excel or an Intel CPU is on record. The evidence record `EV-MISC-0003` is
older and narrower: one joint scan of `BESSELI` and `BESSELK` over the band x ≥ 8, scored against
the reference engine, with no split between the two functions. Its figure renders beside this page
and is not a rate for this surface.

What is still unmeasured:

1. **The last units.** No staging is on record for this function, and the 14 rows that a plain
   binary64 transcription misses are not explained. The x87 storage that the notes establish for
   `BESSELI` is the first thing to try, since x ≤ 2 uses the same polynomial I_0 and I_1.
2. **The range ends.** No row has `x` above 100, or `n` at or above 20. Behaviour at the underflow
   end, around x = 700 to 745, and whether a cut like `BESSELI`'s 709.7 exists here, is unknown.
3. **Overflow at small `x` and high order.** The largest value read is about 1.9E38. Whether
   overflow gives `#NUM!` or something else has not been read.
4. **The seam at 2.** Adjacent doubles around 2, at orders 0 and 1, would show the jump and which
   branch owns x = 2.
5. **Non-numeric arguments.** `BESSELK("1.5",1)`, `BESSELK(TRUE,1)`, an empty cell in either slot,
   and an array or range argument have no recorded reading. An empty `x` is doubly interesting: if
   it reads as 0, as an empty cell does in `DELTA` and `ERF`, the result should be `#NUM!`, not
   `#VALUE!`.
6. **Which error wins** when both arguments are bad, including the case `x ≤ 0` with `n < 0`, which
   gives `#NUM!` either way, and the case of a nonnumeric `x` with `n < 0`, which does not.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `BESSELK` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-BESSELI-002: BESSELI and BESSELK are low-order approximations, rarely correctly rounded

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.* Applies to BESSELI, BESSELK.

BESSELI and BESSELK are low-order polynomial approximations. Over the recorded numeric rows
with non-negative order, BESSELI gives the correctly rounded value on 2 of 1163 rows and
BESSELK on 0 of 1155, with largest relative errors 5.0e-04 and 1.7e-07. BESSELI(-1,0) is
1.2660658480342601 where I0(1) is 1.2660658777520084, about 2.3E-8 relative.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `BESSELI(-1, 0)` | x `-1` (`0xbff0000000000000`); n `0` (`0x0000000000000000`) (cells) | `1.2660658480342601` (`0x3ff441ce433e3c05`) | `1.2660658777520084` (`0x3ff441ce4b386c2d`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `BESSELI(12.514410892637624, 3.987394155038235)` | x `12.514410892637624` (`0x40290760dd512b66`); n `3.987394155038235` (`0x400fe62ee82134c8`) (cells) | `21360.000787344507` (`0x40d4dc000ce65cba`) | `21360.00150112302` (`0x40d4dc0018982a92`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `BESSELK(1.8447401994787955, 0.15119840195885237)` | x `1.8447401994787955` (`0x3ffd840e4ca60c0e`); n `0.15119840195885237` (`0x3fc35a781fcf7430`) (cells) | `0.1380029049573409` (`0x3fc1aa1445c5bb34`) | `0.13800289523526038` (`0x3fc1aa1430e4f602`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |

Correct value: mpmath besseli at 800 bits, order the integer part of n, correctly rounded.
Correct value: mpmath besselk at 800 bits, order the integer part of n, correctly rounded.

Basis (measured): Every numeric row of the extended capture of each function, compared with
mpmath at 200 bits.

Evidence:

- DnaCalc-Research `round22-result.json`, product; bank.own_BESSELI_gains at `130ec21b3d` — research round 22 result
- CalcFunc-Evidence `cache/w111-extended-regenerated-20260929/answers-besseli.json.gz`, w111extended-2026092903-BESSELI-00001 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `cache/w111-extended-regenerated-20260929/answers-besseli.json.gz`, w111extended-2026092903-BESSELI-00633 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `cache/w111-extended-regenerated-20260929/answers-besselk.json.gz`, w111extended-2026092903-BESSELK-00698 at `4d5320dfdc` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| K_n(x) | The modified Bessel function of the second kind, of integer order n |
| order | The argument `n` after truncation |
| seam | The join between two approximating polynomials, here at x = 2 |
| staging | The order of operations and the precision each result is held in |
| unit | One unit in the last place of a binary64 result |

## Sources

- Microsoft, "BESSELK function" —
  <https://support.microsoft.com/en-us/office/besselk-function-606d11bc-06d3-4d53-9ecb-2803e2b90b70>
  (syntax, argument meanings, truncation of `n`, the `#VALUE!` and `#NUM!` remarks and the example
  value; the page's defining equation is an image and was not read as text; **no condition on the
  sign of `x` is stated**).
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, 9.8.5 to 9.8.8 — the polynomial
  approximations.
- NIST Digital Library of Mathematical Functions, chapter 10, sections 10.25 to 10.32, 10.34 and
  10.40 — <https://dlmf.nist.gov/10.25> (definitions, the relation to J_n and Y_n, recurrence,
  cross-product, integral form, limiting forms and the negative axis).
- The recorded Excel answers the notes below draw on: 1,236 `BESSELK` calls on Excel 16.0 build
  20430, Windows 64-bit, CPU not recorded, captured 2026-09-29. Statements on this page that go
  beyond the notes (the domain and sign-test counts, the spot check, and the per-order errors) were
  read from that same file and compared with mpmath at 300 bits while this page was written.
- Handbook evidence record `EV-MISC-0003` (the joint scan with `BESSELI`).
- Handbook, [The value universe](../model/01-value-universe.md),
  [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [Claim language and honesty](../model/06-claim-language.md).
- Handbook projections `data/functions/FUNC.BESSELK.json` and `data/presence/FUNC.BESSELK.json`
  (arity, classification axes, implementing module and sibling set).
- OxFunc `crates/oxfunc_core/src/functions/bessel_convert_family.rs` at commit `473efa3` (the
  reference engine's `besselk_kernel`, `bessk0`, `bessk1` and order guard).
