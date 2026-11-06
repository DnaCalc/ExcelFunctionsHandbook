---
schema: efh.function-page/v1
function_id: FUNC.BESSELI
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MISC-0003
open_problems: []
references:
  - work: "Microsoft Support — BESSELI function"
    locator: "https://support.microsoft.com/en-us/office/besseli-function-8d33855c-9a8d-444b-98e0-852267b1c0df"
    role: "syntax, the truncation of n, the #VALUE! and #NUM! remarks and the worked example; its defining equation is an image"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 9"
    locator: "9.8.1 to 9.8.4 — polynomial approximations to I0 and I1"
    role: "the published seven-digit polynomials and their stated error bounds"
  - work: "NIST Digital Library of Mathematical Functions, chapter 10"
    locator: "https://dlmf.nist.gov/10.25"
    role: "definitions, recurrence, cross-product and asymptotic behaviour of the modified Bessel functions"
  - work: "Insight Toolkit (ITK), itkGaussianOperator.hxx"
    locator: "commit 311b7060ef39e371f3cd209ec135284ff5fde735"
    role: "the published body the function notes identify; Apache-2.0"
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
  The exponentially growing partner of BESSELK, and the only member of the Bessel quartet whose
  higher orders are built by a backward recurrence from a starting point that ignores x.
---

# BESSELI

## What it computes

`BESSELI(x, n)` returns **I_n(x)**, the modified Bessel function of the first kind, of integer
order n at the real point x. Microsoft describes it as the Bessel function evaluated for a purely
imaginary argument, which is the standard relation

    I_n(x)  =  i^(−n) · J_n(i·x)

I_n is the solution that stays finite at the origin of the modified Bessel equation

    x² w″ + x w′ − (x² + n²) w  =  0

It has an everywhere-convergent power series and, for integer order, a short integral form:

    I_n(x)  =  Σ_{k≥0}  (x/2)^(2k+n) / ( k! · (k+n)! )
    I_n(x)  =  (1/π) ∫₀^π  e^(x cos θ) cos(nθ) dθ

Three facts about its shape matter below.

- **Growth.** For large x, I_n(x) ≈ e^x / √(2πx), and at that scale every order looks alike: the
  order only appears as a factor close to e^(−n²/(2x)). I_0 is still representable in binary64
  up to about x = 713.9.
- **The origin.** I_0(0) = 1 and I_n(0) = 0 for n ≥ 1, with I_n(x) ≈ (x/2)^n / n! for small x.
- **Parity.** I_n(−x) = (−1)^n · I_n(x): even orders are even functions and odd orders odd.

For integer order the function is real on the whole line, positive for x > 0, and has no poles and
no branch points. Everything difficult about `BESSELI` in Excel comes from how it is approximated.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `x` | The value at which to evaluate the function. Required. | — |
| `n` | The order of the function. Required. Truncated if not an integer. | — |

Two arguments, both required; the projection declares an arity of exactly 2.

**Truncation and the negative test.** Microsoft states both that a non-integer `n` is truncated
and that `n < 0` is `#NUM!`, and does not say which comes first. The recorded answers do: the
sign test sees the value **before** truncation. All 64 recorded calls with `n` strictly between
−1 and 0 returned `#NUM!` (Excel 16.0 build 20430, Windows 64-bit, CPU not recorded), although
truncating any of them gives 0. Positive fractions behave as documented: `BESSELI(-1, 0.9)` and
`BESSELI(-1, 0)` read the same.

`BESSELI` is not a function of real order. The notes below compute their "correct value" at the
integer part of `n` for that reason, and `BESSELI(x, 3.99)` is order 3.

The slots are numeric. Beyond Microsoft's statement that a nonnumeric argument gives `#VALUE!`, no
reading of text, a logical, an empty cell or an array in either slot is on record for this
function; see *What has not been checked*.

## Result and edge cases

Returns `Number`. Unless stated otherwise the behaviour below is read from recorded Excel answers
(builds 20430 and 20527, Windows 64-bit).

- **`x = 0`** gives 1 at order 0 and 0 at every higher order read (1, 2 and 10), as the
  mathematics requires.
- **Negative `x`** is admitted. Even orders return the value at |x|, and odd orders return it
  negated, which was read at ±1 and at ±709.7.
- **The documented example** `BESSELI(1.5, 1)` is printed by Microsoft as `0.981666428`. Nine
  digits is about what the function offers: the answers are accurate to roughly seven or eight
  significant figures at ordinary arguments, not to fifteen. `BESSELI(-1, 0)` is the plainest
  example in the notes below.
- **`|x|` just above 709.7 is `#NUM!`** at every order read, although the true value is far inside
  the binary64 range there. The notes below record the boundary to the adjacent double. The
  failure is abrupt: `BESSELI(709.7, 0)` is a number near 2.48E306, and the next double up is an
  error.
- **Orders 2 and above can be badly wrong at large `x`.** At x = 709.7, order 3 is 53% below the
  true value. This is a real wrong answer with no error raised, recorded in the notes below and
  discussed under *Numerical notes*.
- **Very high orders can collapse to 0.** At x = ±709.7, orders 1568 to 1572 all read 0 (build
  20527), although the true values, from about 1.5E-299 down to 3.1E-302, are ordinary binary64
  numbers. Order 10000 also read 0, which there is the right rounded answer: the true value is
  near 2.9E-10154.

## Errors

As documented on Microsoft's page:

| Error | Condition |
|---|---|
| `#VALUE!` | `x` is nonnumeric |
| `#VALUE!` | `n` is nonnumeric |
| `#NUM!` | `n < 0` |

As recorded, in addition to the documented `#NUM!` for `n ≤ −1`:

| Error | Condition | Recorded on |
|---|---|---|
| `#NUM!` | `n` strictly between −1 and 0, where truncation would give 0 | 64 calls, build 20430 |
| `#NUM!` | \|x\| above 709.7, either sign, at every order read from 0 to 10000 | build 20527, AMD Ryzen 7 4800H |

The documentation mentions neither row. The reference engine differs from both rows at the commit this page was written against. Its order
check truncates first, so an `n` between −1 and 0 is evaluated as order 0 instead of raising
`#NUM!`; and it raises `#NUM!` only when the computed result is not finite, so it has no cut at
709.7. Those are read from its source, and they are facts about that engine, not about Excel.

## Relationships

- **[BESSELK](FUNC.BESSELK.md)** — the other modified Bessel function, decaying where this one
  grows. The two are tied by an exact cross-product,
  I_n(x)·K_(n+1)(x) + I_(n+1)(x)·K_n(x) = 1/x, which needs no oracle and is a useful sanity test on
  any pair of answers. `BESSELK` shares this page's accuracy note but none of the large-`x`
  trouble below, because it steps upward.
- **`BESSELJ` and `BESSELY`** — the ordinary Bessel functions, oscillating where the modified ones
  are monotone. All four share the reference engine's implementing module, which implies nothing
  about Excel's code.
- **`EXP`** — every answer at |x| ≥ 3.75 carries a factor e^|x|, which is the likely reason for the
  cut near 709.7.
- **The recurrence.** I_(n−1)(x) − I_(n+1)(x) = (2n/x)·I_n(x) links neighbouring orders. It gives a
  workaround for the large-`x` defect at low order: `BESSELI(x,0) - 2/x*BESSELI(x,1)` is I_2(x)
  from the two orders that do not use the recurrence. That identity is mathematics; it has not
  been read in Excel as a repair.
- **Where readers meet it.** I_0(κ) is the normalising constant of the von Mises distribution, and
  I_0 and I_1 appear in the Rician distribution and in radially symmetric diffusion problems.

## Numerical notes

**These are low-order polynomials, by design.** Orders 0 and 1 are the polynomial approximations
printed in Abramowitz & Stegun as 9.8.1 to 9.8.4: one polynomial in (x/3.75)² for |x| < 3.75 and
one in 3.75/|x|, multiplied by e^|x|/√|x|, from 3.75 up. The coefficients carry seven or eight
decimal places, and the printed error bounds reach 1.9E-7 for I_0 and 2.2E-7 for I_1 on each
form's own scale. In the large-`x` form that scale is a factor near 0.4, so the relative error of
I_0 can reach about 2.5 times the bound; the largest recorded at order 0 is 4.7E-7, at x = 100. The
notes below measure the result: correctly rounded on 2 of 1,163 recorded rows. Treat the answer as a seven-digit figure.

**There is a seam at 3.75, and Excel shows it.** Two different polynomials meet there and they do
not agree exactly. Build 20527 reads I_0 at adjacent doubles: the double below 3.75 is on the small
polynomial, 3.75 itself is on the large one, and the answer steps up by 2.1E-8 relative between
them. A numerical derivative taken across 3.75 is not to be trusted. The printed coefficients for
I_1 imply a smaller step, about 1.1E-8, which has not been read.

**Orders 2 and above come from a backward recurrence.** Stepping upward in n from I_0 and I_1 is
unstable for this function, so the standard method, due to Miller, starts high with an arbitrary
value, runs the recurrence downward where it is stable, and rescales the whole sequence at the
end by the known I_0(x). The notes below give Excel's exact staging. Two consequences are worth
knowing:

1. **The error of I_0 is inherited.** The normalisation uses the polynomial value of I_0, so a
   healthy high-order answer carries order 0's relative error. The recorded rows show it plainly:
   at x = 12.51 orders 0, 2 and 7 are all low by 3.34E-8, and at x = 709.7 orders 0 and 100 are
   both low by 1.45E-7. The step at the seam reaches orders 2 and 17 by the same route.
2. **The starting point ignores x.** The recurrence starts at j = 2(n + ⌊√(40n)⌋), a function of
   the order alone. That start is high enough while x is small compared with it. When x is much
   larger, the sequence has too few steps to settle, and the answer is simply wrong.

**How wrong, and where.** The recorded values do not change smoothly with the order. At x near 709,
order 2 is high by 0.20%, order 3 is low by 53%, order 10 is high by 0.08%, and order 100 is as
good as order 0. The 1,236-row recording behind the notes stops at x = 50, apart from six rows at
x = 100. Its worst row at x ≤ 50 is about 9E-6 relative, at order 2, and the 5.0E-4 quoted in the
notes below is `BESSELI(100, 2)`. So order 2 degrades steadily with x over the range read. Order 3
is about 1.6E-6 off near x = 50 and 53% low at 709, and the path between them has not been read.

The large-`x` failure is the one defect on this page that matters to an ordinary user: it raises
no error, and the wrong answer looks entirely plausible.

**The collapse to 0** has a likely cause in the same recurrence. It computes the ratio I_n/I_0 and
only then multiplies by I_0. At x = 709.7 and high order that ratio is below the binary64 range
although the product is not, so the ratio underflows to 0 and the multiplication cannot recover it.
A plain binary64 transcription of the published body does this somewhere between orders 1100 and
1200 at this x. That is a computation on the published body, and the cause is an inference: Excel
itself has been read only at order 100 and then from order 1568.

**The cut at 709.7.** e^709.7 is about 1.65E308, just inside the binary64 range, and the exponential
itself overflows near 709.78. The polynomial's e^|x| factor is therefore the natural reason for a
guard at about this size, but the guard then gives up about four units of argument over which I_0
is still finite. The reason is an inference; only the boundary is measured.

**Where the body comes from.** The notes below identify the body with the routine published in
ITK, evaluated with every operation stored from the x87 unit at precision 64, and report that a
plain binary64 transcription misses by a few units in the last place. The reference engine's source
describes its Bessel code as following Numerical Recipes, which uses the same polynomials and the
same recurrence constants. A shared shape says nothing about where Excel's code came from. The
reference engine at the commit named under Sources computes in plain binary64, so it cannot show
the x87 behaviour.

## What has not been checked

No Handbook vector suite exists for `BESSELI`; `vectors/` publishes nothing at this revision, so no
suite-scoped claim exists for it.

The numeric rows and the boundary are recorded in the notes below, on Excel builds 20430 and 20527
on 64-bit Windows. No reading from a Mac, Excel for the web, 32-bit Excel or an Intel CPU is on
record. The evidence record `EV-MISC-0003` is older and narrower: one joint scan of `BESSELI` and
`BESSELK` over the band x ≥ 8, scored against the reference engine, with no split between the two
functions. Its figure renders beside this page and is not a rate for this surface.

What is still unmeasured after the notes:

1. **The map of the recurrence error.** Rows exist at x ≤ 50, at x = 100 and from 709 to 709.7,
   and nothing between 100 and 709. Sweeping orders 2 to 20 across that gap would show where the
   error starts and why it is not monotone in the order. The notes below record this as open.
2. **The cut at other orders.** It is recorded at orders 0, 1, 2, 3, 10, 100, 1570 and 10000, at
   both signs. That it applies at every order is inferred.
3. **The collapse to 0.** Where between order 100 and order 1568 the answer at x = 709.7 falls
   away, and at what order it does so at smaller x, has not been read.
4. **Non-numeric arguments.** `BESSELI("1.5",1)`, `BESSELI(TRUE,1)`, an empty cell in either slot,
   and an array or range argument have no recorded reading. This is a live question and not a
   formality: `ERF` and `DELTA`, in the same Engineering category, reject logicals, while the
   reference engine lets `BESSELI` convert them and gives it no elementwise array lift, which it
   does give `ERF`.
5. **Which error wins** when both arguments are bad. `DELTA` reads its second argument first and
   `ERF` its first, so the answer cannot be assumed.
6. **The largest admissible order.** The largest order read is 10000. The reference engine rejects
   a truncated order above 2³¹ − 1, and whether Excel has a limit, or how long it takes to reach
   one, is unknown.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `BESSELI` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-BESSELI-001: BESSELI is ITK's published polynomial and recurrence, on the x87 unit

*Detail for matching Excel's bits; measured; recorded 2026-11-04.*

BESSELI truncates n to an integer and evaluates the body published in ITK with every operation
stored from the x87 unit at precision 64: the I0 and I1 polynomials, and for higher orders
Miller backward recurrence from j = 2*(n + int(sqrt(40*n))), rescaling by 1E-10 whenever |q| >
1E10, then normalising by I0(x). The same body in plain binary64 misses by 1 to 4 units in the
last place. The published ITK body is itkGaussianOperator.hxx at ITK
311b7060ef39e371f3cd209ec135284ff5fde735 (Apache-2.0).

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `BESSELI(12.514410892637624, 3.987394155038235)` | x `12.514410892637624` (`0x40290760dd512b66`); n `3.987394155038235` (`0x400fe62ee82134c8`) (cells) | `21360.000787344507` (`0x40d4dc000ce65cba`) | `21360.00150112302` (`0x40d4dc0018982a92`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `BESSELI(1.8661547234119729, 7.047660373731416)` | x `1.8661547234119729` (`0x3ffddbc50e25491c`); n `7.047660373731416` (`0x401c30cde189f75e`) (cells) | `0.0001361367103021099` (`0x3f21d7fd6fe380e0`) | `0.00013613670961869657` (`0x3f21d7fd6e62c6a2`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `BESSELI(13.469754413420809, 10.617422640816173)` | x `13.469754413420809` (`0x402af083a685968c`); n `10.617422640816173` (`0x40253c1ed2043b18`) (cells) | `1957.477427686333` (`0x409e95e8e2cdac05`) | `1957.477488938039` (`0x409e95e8f2dc3451`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |

Correct value: mpmath besseli at 800 bits, order the integer part of n, correctly rounded.

Basis (measured): All 1,318 recorded BESSELI recorded rows match (825 gained, no row lost in
the recorded rows).

Evidence:

- DnaCalc `crates/calc-func/tests/f1_engineering.rs`, lines 354-468 at `d62b1283bb` — recorded answers pinned; the ITK body in the product
- DnaCalc-Research `round22-result.json`, product; bank.own_BESSELI_gains at `130ec21b3d` — research round 22 result
- CalcFunc-Evidence `cache/w111-extended-regenerated-20260929/answers-besseli.json.gz`, w111extended-2026092903-BESSELI-00633 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `cache/w111-extended-regenerated-20260929/answers-besseli.json.gz`, w111extended-2026092903-BESSELI-00440 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `cache/w111-extended-regenerated-20260929/answers-besseli.json.gz`, w111extended-2026092903-BESSELI-00674 at `4d5320dfdc` (readings)

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

### FN-BESSELI-003: At large x the fixed recurrence start makes BESSELI badly wrong

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.*

Excel's recurrence for orders above 1 starts at a fixed j = 2*(n + int(sqrt(40*n))), which is
far too low at large x. At x = 709.7 BESSELI of order 2 is 0.20% high, and order 3 is 53% low,
where I3(709.7) is 2.4631E306. Orders 0 and 1, which use the polynomial and not the recurrence,
are within about 1.6E-7 at the same x.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `BESSELI(709.7, 3)` | x `709.7` (`0x40862d999999999a`); n `3` (`0x4008000000000000`) (cells) | `1.1560227990465618e+306` (`0x7f7a56f66ecd12a0`) | `2.4631300544524576e+306` (`0x7f8c0f97bbe572f6`) | Excel 16.0 build 20527, Windows 64-bit, AMD Ryzen 7 4800H |
| `BESSELI(709.7, 2)` | x `709.7` (`0x40862d999999999a`); n `2` (`0x4000000000000000`) (cells) | `2.4767103124440567e+306` (`0x7f8c3732e70ce6f9`) | `2.47182813908566e+306` (`0x7f8c28f5d197b1dd`) | Excel 16.0 build 20527, Windows 64-bit, AMD Ryzen 7 4800H |
| `BESSELI(709.7, 0)` | x `709.7` (`0x40862d999999999a`); n `0` (`0x0000000000000000`) (cells) | `2.4788083687166675e+306` (`0x7f8c3d51558ee541`) | `2.4788087273156452e+306` (`0x7f8c3d519a191ac9`) | Excel 16.0 build 20527, Windows 64-bit, AMD Ryzen 7 4800H |

Correct value: mpmath besseli at 800 bits, order the integer part of n, correctly rounded.

Basis (measured): Readings at x = 709.7 for orders 0, 1, 2 and 3, compared with mpmath.

Inferred, not measured: That the fixed start is the cause is inferred: a binary64 transcription
of the published body gives the same bits for orders 0, 2 and 3 at this x.

- Open: How far the error extends at other x is not measured.

Evidence:

- DnaCalc-Research `round22-result.json`, product; bank.own_BESSELI_gains at `130ec21b3d` — research round 22 result
- DnaCalc-Research `evidence/round18/r16-published-score/observations.jsonl`, line 177, id r16-besseli-021 at `2df115b951` (readings)
- CalcFunc-Evidence `readings/excel-20527/BESSELI.json`, r11-struct-besseli-09 at `dec750f162` (readings)
- CalcFunc-Evidence `readings/excel-20527/BESSELI.json`, r11-struct-besseli-01 at `dec750f162` (readings)

### FN-BESSELI-004: BESSELI gives #NUM! just above 709.7, where the function is still finite

*Quirk; measured; recorded 2026-11-04.*

BESSELI returns a number at |x| = 709.7 and #NUM! at the next binary64 above it, although I0
there is about 2.48E306, well inside the binary64 range. Recorded for orders 0, 1, 2, 3, 10 and
100, and for negative x at orders 0 to 2.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `BESSELI(709.7, 0)` | x `709.7` (`0x40862d999999999a`); n `0` (`0x0000000000000000`) (cells) | `2.4788083687166675e+306` (`0x7f8c3d51558ee541`) | Excel 16.0 build 20527, Windows 64-bit, AMD Ryzen 7 4800H |
| `BESSELI(709.7000000000002, 0)` | x `709.7000000000002` (`0x40862d999999999b`); n `0` (`0x0000000000000000`) (cells) | `#NUM!` | Excel 16.0 build 20527, Windows 64-bit, AMD Ryzen 7 4800H |
| `BESSELI(709.7000000000002, 1)` | x `709.7000000000002` (`0x40862d999999999b`); n `1` (`0x3ff0000000000000`) (cells) | `#NUM!` | Excel 16.0 build 20527, Windows 64-bit, AMD Ryzen 7 4800H |

Basis (measured): Adjacent-input readings at the boundary.

Inferred, not measured: That the same cut-off applies at every other order is inferred.

Evidence:

- DnaCalc `crates/calc-func/tests/f1_engineering.rs`, line 108 at `f2e72693a5` — the boundary in the product
- CalcFunc-Evidence `readings/excel-20527/BESSELI.json`, r11-struct-besseli-01 at `dec750f162` (readings)
- CalcFunc-Evidence `readings/excel-20527/BESSELI.json`, r11-struct-besseli-02 at `dec750f162` (readings)
- CalcFunc-Evidence `readings/excel-20527/BESSELI.json`, r11-struct-besseli-06 at `dec750f162` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| I_n(x) | The modified Bessel function of the first kind, of integer order n |
| order | The argument `n` after truncation |
| Miller recurrence | Computing a sequence by running its recurrence downward from an arbitrary start, then rescaling by a known value |
| seam | The join between two approximating polynomials, here at \|x\| = 3.75 |
| unit | One unit in the last place of a binary64 result |
| x87 unit | The floating-point unit of x86 processors, which computes in an 80-bit format |

## Sources

- Microsoft, "BESSELI function" —
  <https://support.microsoft.com/en-us/office/besseli-function-8d33855c-9a8d-444b-98e0-852267b1c0df>
  (syntax, argument meanings, truncation of `n`, the `#VALUE!` and `#NUM!` remarks and the example
  value; the page's defining equation is an image and was not read as text).
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, 9.8.1 to 9.8.4 — the polynomial
  approximations and their printed error bounds.
- NIST Digital Library of Mathematical Functions, chapter 10, sections 10.25 to 10.32, 10.34 and
  10.40 — <https://dlmf.nist.gov/10.25> (definitions, the relation to J_n, recurrence,
  cross-product, integral form, parity and asymptotics).
- Press et al., *Numerical Recipes in C*, routines `bessi0`, `bessi1` and `bessi` — the
  backward-recurrence presentation the reference engine's source names.
- The recorded Excel answers the notes below draw on: 1,236 `BESSELI` calls on Excel 16.0 build
  20430, Windows 64-bit, CPU not recorded, captured 2026-09-29; and the endpoint, seam and
  high-order readings on build 20527, Windows 64-bit, AMD Ryzen 7 4800H, captured in November 2026.
  Statements on this page that go beyond the notes (the sign test, the seam, the rows at orders
  1568 to 1572, and relative errors other than those the notes quote) were read from those same
  files and compared with mpmath at 300 bits while this page was written.
- Handbook evidence record `EV-MISC-0003` (the joint scan with `BESSELK`).
- Handbook, [The value universe](../model/01-value-universe.md),
  [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [Claim language and honesty](../model/06-claim-language.md).
- Handbook projections `data/functions/FUNC.BESSELI.json` and `data/presence/FUNC.BESSELI.json`
  (arity, classification axes, implementing module and sibling set).
- OxFunc `crates/oxfunc_core/src/functions/bessel_convert_family.rs` at commit `473efa3` (the
  reference engine's `besseli_kernel`, `bessi0`, `bessi1` and order guard).
