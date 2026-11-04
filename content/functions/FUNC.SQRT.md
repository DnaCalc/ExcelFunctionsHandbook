---
schema: efh.function-page/v1
function_id: FUNC.SQRT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — SQRT function"
    locator: "https://support.microsoft.com/en-us/office/sqrt-function-654975c2-05c4-4831-9a24-2c65e4040fdf"
    role: "documented signature, the #NUM! condition for negative arguments, and the ABS workaround"
  - work: "IEEE 754-2019, Standard for Floating-Point Arithmetic"
    locator: "clause 5.4.1 (squareRoot)"
    role: "the requirement that square root be correctly rounded, which is what makes this function special"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 3"
    locator: "3.7 (roots and powers)"
    role: "elementary algebraic identities and the series used in cancellation-avoiding rewrites"
  - work: "Muller et al., Handbook of Floating-Point Arithmetic"
    locator: "chapter on square root and division algorithms"
    role: "Newton-Raphson and Goldschmidt square root, and the last-bit rounding argument"
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
  - Excel versions, platforms and CPUs
  - Page vocabulary
  - Sources
family: sqrt_fn
role_in_family: >-
  The sole member: the non-negative square root, and the one elementary function in the category
  that IEEE 754 requires to be correctly rounded.
---

## What it computes

`SQRT(number)` is the **principal** (non-negative) square root: the unique `y >= 0` with
`y^2 = x`.

- **Domain**: `[0, +infinity)` over the reals. The equation `y^2 = x` has no real solution for
  `x < 0`, which is the mathematical reason the negative case is an error and not a convention.
- **Range**: `[0, +infinity)`.
- **Branch**: the principal branch. Every positive `x` has two real square roots, `+sqrt(x)` and
  `-sqrt(x)`; `SQRT` returns the positive one. Extended to the complex plane, the square root
  has a branch point at `0` and the conventional branch cut along the negative real axis — which
  is exactly the ray on which `SQRT` reports `#NUM!`. Excel's real-only surface makes the cut
  visible as an error condition.
- **Monotonicity and shape**: strictly increasing, concave, with `sqrt(0) = 0`, `sqrt(1) = 1`,
  and derivative `1/(2 sqrt x)` — unbounded at the origin, which is why `SQRT` is
  ill-conditioned in *absolute* terms near zero and perfectly conditioned in relative terms
  everywhere.
- **Relative conditioning**: the relative condition number is exactly `1/2` for all `x > 0`.
  Square root *halves* relative error. It is one of the very few operations that improves the
  accuracy of its input, and that fact is worth knowing when assembling a formula.
- **Identities**: `sqrt(x*y) = sqrt(x)*sqrt(y)` for non-negative arguments; `sqrt(x) = x^(1/2)`;
  `sqrt(x^2) = |x|`, not `x`.

Microsoft's page states the signature and one remark: "If number is negative, SQRT returns the
#NUM! error value", with the documented workaround `=SQRT(ABS(A2))`.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The number whose square root is wanted. Required, must be non-negative. | — |

One argument; the reference engine records an arity of exactly one. Ordinary numeric slot under
the shared coercion rules — see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

The commonly misunderstood position is the negative case, and specifically the `ABS` workaround
Microsoft documents. `SQRT(ABS(x))` is **not** the square root of `x`; it is the modulus of the
principal complex square root. If the negative sign is meaningful, the workaround silently
destroys it. Where a genuine complex root is wanted, the engineering category's `IMSQRT` is the
function that computes one.

## Result and edge cases

Returns `Number`.

- **Zero.** `SQRT(0)` is `0`. In IEEE 754 arithmetic the square root of negative zero is
  negative zero — a sign that a worksheet will not display and that may or may not survive to
  the published result. Whether Excel preserves it is unverified here and is on the probe list.
- **Exact squares.** The root of an exactly representable perfect square is itself representable,
  so neither rounding of FN-SQRT-001 moves it: `SQRT(4)` is exactly `2`, and there is no
  last-bit wobble in the answer. This is not true of `POWER(x, 0.5)` in general, and that
  difference is the subject of the *Relationships* and *Numerical notes* sections below.
- **Very large arguments.** `SQRT` cannot overflow: the square root of the largest finite double
  is comfortably in range. Mathematically it also cannot underflow to zero from a non-zero input,
  because the square root of the smallest subnormal is a normal number, yet `SQRT(5e-324)` returns
  0 on the one recorded row (FN-SQRT-001). The other hazard on the positive axis is the double
  rounding of that note.
- **Negative arguments.** `#NUM!`, as documented.

The reference engine classifies `SQRT` as a unary numeric scalar-or-array kernel
(`UnaryNumericScalarOrArrayElementwise`) with `kernel_signature_class: Custom`, so arrays lift
elementwise.

**A divergence worth recording.** The reference engine's projected `real_result_policy` for
`SQRT` reads `arg_domain_guard=none` with `non_finite=allow`. Microsoft documents a domain
restriction — negative arguments are `#NUM!` — so the declared axis does not, on its face,
express the documented behaviour. The most likely reading is that the guard lives inside the
`Custom` kernel rather than in the declared axis vocabulary, in which case the axis value is a
classification gap rather than a behavioural claim. The Handbook records the mismatch and does
not resolve it: the documentation says there is a domain restriction, and the projected axis
says there is no argument-domain guard.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | `number` is negative | Documented by Microsoft |
| `#VALUE!` | The argument does not convert to a number | Shared coercion rules |
| propagated | An error value in the argument is returned unchanged | Shared coercion rules |

## Relationships

- **`POWER(x, 0.5)` and `x^0.5`** — mathematically the same value, computationally not the same
  function. IEEE 754 requires `squareRoot` to be correctly rounded and does not require it of a
  general `pow`, which in practice is not; Excel's `SQRT` is rounded twice, so it too misses the
  correctly rounded value on some inputs (FN-SQRT-001). Two expressions that look interchangeable
  can differ in the last bit. Prefer `SQRT` whenever the exponent is literally one half.
- **`SQRTPI`** — `sqrt(number * pi)`. Related in name, and the Handbook's evidence for `SQRTPI`
  makes precisely the `SQRT`-versus-`pow` point above; see that page.
- **`ABS`** — the documented workaround for negative input, with the caveat stated above.
- **`IMSQRT`** — the complex square root, for when the negative case is meaningful rather than
  a mistake. Note it returns text in the engineering complex-number format, not a number.
- **`EXP` / `LN`** — `SQRT` is emphatically *not* to be computed as `exp(ln(x)/2)`; that
  composition loses roughly half the significand and can overflow the intermediate.
- **Confused with**: `POWER` with a fractional exponent, and with `SUMSQ`'s inverse (there is
  no such function; the Euclidean norm is `SQRT(SUMSQ(...))`).

## Numerical notes

`SQRT` is the best-behaved elementary function in the entire category, and it is worth being
explicit about why, because the reasons are unusual.

**IEEE 754 mandates correct rounding.** Clause 5.4.1 of the standard lists `squareRoot` among
the operations that must return the correctly rounded result — the same tier as addition,
multiplication and division, and a tier that `exp`, `log`, `pow` and every trigonometric
function do *not* occupy. There is exactly one right answer for every input, every conforming
platform produces it, and there is no "quality of implementation" question. Excel's worksheet
`SQRT` is not that operation: the root is rounded to the x87 unit's 64-bit significand and then
stored to binary64, and where that double rounding lands differently Excel takes the double-rounded
value (FN-SQRT-001).

**Why correct rounding is achievable here and not for `exp`.** The table-maker's dilemma — the
possibility that the true value sits so close to a rounding boundary that arbitrarily many bits
are needed to decide — does not bite for square root. The worst case is bounded: because the
square of a `p`-bit number has at most `2p` bits, the true square root of a binary64 number can
never be exactly halfway between two representable values (except in the exact-square case,
where it is representable outright), and the number of extra bits needed to round correctly is
small and known. That structural fact is what puts `sqrt` in the correctly rounded tier.

**How it is computed.** Modern hardware has a square-root instruction; where it does not, the
standard software route is a Newton-Raphson or Goldschmidt iteration on the *reciprocal* square
root — `y_{n+1} = y_n (3 - x y_n^2)/2`, which converges quadratically and uses no division —
followed by a multiplication by `x` and a final correctly rounding step. The exponent is halved
by integer arithmetic first so that the iteration runs on a significand in `[1, 4)`. Muller et
al. give the full treatment including the residual-based final rounding.

**Where `SQRT` participates in someone else's cancellation.** `SQRT` itself does not cancel,
but it appears in two classic formulas that do, and a page about square roots should name them:

1. **The quadratic formula.** `(-b + sqrt(b^2 - 4ac)) / (2a)` cancels catastrophically when
   `b^2 >> 4ac` and `b > 0`. The standard fix is to compute the root of larger magnitude with
   the sign-matched numerator and obtain the other from `x1 * x2 = c/a`.
2. **`sqrt(x^2 + 1) - x`** for large `x`, which appears inside `ASINH` and in vector-length
   work. Rewrite as `1 / (sqrt(x^2 + 1) + x)`.

In both cases the defect is in the surrounding expression, not in `SQRT` — which is exactly the
point: because on normal inputs `SQRT` is off by at most its double rounding (FN-SQRT-001), nearly
all the error you see around it came from somewhere else.

**Hypotenuse.** `SQRT(x^2 + y^2)` overflows when `x` alone is large, even though the result is
representable. There is no `HYPOT` in Excel; the scaled form `SQRT((x/m)^2 + (y/m)^2) * m` with
`m = MAX(ABS(x), ABS(y))` is the workaround, and it also fixes the underflow case.

## What has not been checked

No Handbook evidence record lists `FUNC.SQRT` in its subjects, and no Handbook vector suite
exists for `SQRT`. **The checks against Excel in the Handbook's record are the Notes below:**
FN-SQRT-001 matches 41,897 of 41,897 distinct normal positive rows, and FN-VALUE-001 reads text
arguments, with other digit blocks and any dependence on the Windows or workbook locale still open.
The `#NUM!` condition for negative arguments is documented by Microsoft and has not been observed
in Excel by the Handbook.

Probes worth running first:

1. **`SQRT` against `POWER(x, 0.5)` and `x^0.5`** over a broad sample, especially near the top
   and bottom of the exponent range. If they ever differ, the two are separate routes; `SQRT`'s own
   is the double-rounded one of FN-SQRT-001, not the correctly rounded one. The `SQRTPI` evidence
   records make exactly this distinction for a neighbouring surface, which raises the prior that it
   holds here too.
2. **Perfect squares** across the exponent range — `4`, `1e100` when exactly representable,
   `2^2k` — where the correct answer is exact and any deviation is a defect rather than a
   rounding choice.
3. **Negative zero.** `SQRT(-0)` and whether a signed zero survives to the published result, and
   whether `-0` takes the documented `#NUM!` branch (it should not; `-0` is not negative under
   IEEE comparison).
4. **The boundary of the negative branch**: the largest negative subnormal, and text and logical
   arguments that coerce to a negative number.
5. **Subnormal inputs**, whose square roots are normal — a place where a poorly written
   scaling step fails. The one recorded row, `SQRT(5e-324)`, returns 0 (FN-SQRT-001).
6. **Array arguments**, to confirm elementwise lifting and element-local `#NUM!`.
7. **The reference engine's `arg_domain_guard=none` classification** against actual behaviour on
   a negative argument, which is the divergence recorded above.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `SQRT` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-SQRT-001: SQRT is rounded twice, once to 64 bits and once to 53

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.*

The worksheet SQRT is RN53(RN64(sqrt(x))): the root is rounded to the x87 unit's 64-bit
significand and then stored to binary64. On inputs where that double rounding lands differently
from a single rounding, Excel takes the double-rounded value: SQRT(0.9999999999999999) is 1.
The smallest subnormal input returns 0.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `SQRT(0.9999999999999999)` | number `0.9999999999999999` (`0x3fefffffffffffff`) (cells) | `1` (`0x3ff0000000000000`) | `0.9999999999999999` (`0x3fefffffffffffff`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `SQRT(1.7976931348623157e+308)` | number `1.7976931348623157e+308` (`0x7fefffffffffffff`) (cells) | `1.3407807929942597e+154` (`0x5ff0000000000000`) | `1.3407807929942596e+154` (`0x5fefffffffffffff`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `SQRT(5e-324)` | number `5e-324` (`0x0000000000000001`) (cells) | `0` (`0x0000000000000000`) | `2.2227587494850775e-162` (`0x1e60000000000000`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |

Correct value: Exact square root at 800 bits, correctly rounded.

Basis (measured): 41,897 of 41,897 distinct normal positive bank rows match. On the 164 rows
where double and correct rounding differ, Excel follows double rounding every time. The
subnormal result rests on 1 row.

Evidence:

- DnaCalc `crates/calc-func/src/ported/sqrt_fn.rs`, excel_sqrt_staged at `b5ce0759bc` — the kernel in the product
- OxFunc-Evidence `runs/w111-broad-20260929/answers/answers-sqrt.json.gz`, w111broad-20260929-SQRT-00016 at `e26b7bddb8` (readings)
- OxFunc-Evidence `runs/w111-broad-20260929/answers/answers-sqrt.json.gz`, w111broad-20260929-SQRT-00025 at `e26b7bddb8` (readings)
- OxFunc-Evidence `runs/w111-broad-20260929/answers/answers-sqrt.json.gz`, w111broad-20260929-SQRT-00002 at `e26b7bddb8` (readings)

### FN-VALUE-001: Text to number reads Arabic, Persian, Devanagari and Thai digits, but not fullwidth digits

*Quirk; measured; recorded 2026-11-04.* Applies to DATEVALUE, SQRT, VALUE.

In the recorded en-ZA settings, converting text to a number accepts Arabic-Indic (U+0660 to
U+0669), extended Arabic-Indic (U+06F0 to U+06F9), Devanagari (U+0966 to U+096F) and Thai
(U+0E50 to U+0E59) digits. They work alone, in runs, with a decimal point, in the exponent,
with a sign or percent, and mixed with ASCII digits or with each other: VALUE("1٢3") is 123,
and SQRT of a four-digit string using four scripts is the root of 1234. Fullwidth digits
(U+FF10 to U+FF19) give #VALUE! alone or mixed. DATEVALUE reads the four accepted scripts in a
date written year-month-day.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=VALUE("١")` | as typed | `1` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("١٢٣")` | as typed | `123` (`0x405ec00000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("1٢3")` | as typed | `123` (`0x405ec00000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("١e2")` | as typed | `100` (`0x4059000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("१")` | as typed | `1` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("๑")` | as typed | `1` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("１")` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=VALUE("1２3")` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=SQRT("۱٢३๔")` | as typed | `35.12833614050059` (`0x4041906d51932c26`) | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `=DATEVALUE("٢٠٢٤-٠١-٠٢")` | as typed | `45293` (`0x40e61da000000000`) | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |

Basis (measured): 66 recorded VALUE readings (all ten digits of each of the five scripts, then
composed forms), 10 SQRT and 4 DATEVALUE readings.

Inferred, not measured: That other text-to-number routes, such as arithmetic operators, treat
digits the same way is inferred from VALUE and SQRT agreeing.

- Open: Other digit blocks (Bengali, Tibetan, Khmer and so on) have not been read.
- Open: Whether the set depends on the Windows or workbook locale has not been read.

Evidence:

- DnaCalc `apps/dna-onecalc/tests/acceptance.rs`, delivered_fn4_digit_scripts_and_numeric_date_widths at `55697ab4f7` — recorded answers pinned
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheets qt031 to qt096 at `2d0eddcac8` — 66 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/SQRT.reading.json` at `2d0eddcac8` — 10 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/DATEVALUE.reading.json` at `2d0eddcac8` — 6 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt042, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt081, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt084, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt088, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt062, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt072, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt032, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-text-controls.capture.json`, sheet qt093, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/SQRT.reading.json`, fn7-SQRT-digits-6 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/DATEVALUE.reading.json`, fn7-DATEVALUE-digits-1 at `2d0eddcac8` (readings)

<!-- efh:notes end -->

## Excel versions, platforms and CPUs

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.SQRT.json; edits here are overwritten -->

Readings of SQRT with a recorded Excel build: 42,845 input(s) over 2 recorded environment(s),
Excel 16.0 build(s) 19929, 20430, all on Windows; none of these captures records the CPU. 1
same-input comparison(s) between environments over 2 input-pair(s): no result differs. No
reading from a Mac, Excel for the web, 32-bit Excel or an Intel CPU is recorded.

Readings of `SQRT` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 42 | date not recorded |
| 16.0 build 20430 | Windows, 64-bit | CPU not recorded | 42,803 | 2026-09-29 |

Compared on the same inputs:

| Check | Differs in | Environments | Inputs compared | Inputs with different results |
|---|---|---|---:|---:|
| XC-SQRT-001 | excel-build | 16.0 build 19929 vs 16.0 build 20430 | 2 | 0 |

Differences observed: none.

Evidence for every row: [`content/evidence/environments/FUNC.SQRT.json`](../../content/evidence/environments/FUNC.SQRT.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| principal square root | The non-negative root; the branch `SQRT` returns |
| correctly rounded | The result is the true value rounded once to the nearest double; required by IEEE 754 for square root |
| table-maker's dilemma | The problem of not knowing in advance how many bits are needed to round a transcendental correctly |
| relative condition number | Amplification of relative input error into relative output error; exactly `1/2` here |

## Sources

- Microsoft, "SQRT function" —
  <https://support.microsoft.com/en-us/office/sqrt-function-654975c2-05c4-4831-9a24-2c65e4040fdf>
  (signature; "If number is negative, SQRT returns the #NUM! error value"; the `ABS` workaround).
- IEEE 754-2019, clause 5.4.1 — `squareRoot` among the correctly rounded operations.
- Muller et al., *Handbook of Floating-Point Arithmetic* — Newton-Raphson and Goldschmidt square
  root, the bounded worst case, and the final rounding step.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 3 — elementary algebraic
  identities used in the cancellation-avoiding rewrites.
- Handbook page [SQRTPI](FUNC.SQRTPI.md) — where the `sqrt`-versus-`pow` distinction is on the
  record for a neighbouring surface.
- Handbook projections `data/functions/FUNC.SQRT.json` (arity, `kernel_signature_class: Custom`,
  `real_result_policy` with `arg_domain_guard=none`) and `data/presence/FUNC.SQRT.json`.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md).
