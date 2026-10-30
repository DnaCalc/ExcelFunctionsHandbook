---
schema: efh.function-page/v1
function_id: FUNC.LN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0001
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
  - Excel versions, platforms and CPUs
  - Page vocabulary
  - Sources
family: ln_fn
role_in_family: >-
  Sole member of its module; the natural logarithm, and the kernel the rest of the log family
  is defined through — LOG divides by it and LOG10 is its scaled companion.
---

## What it computes

`LN(number)` returns the natural logarithm — the logarithm to base
e = 2.718281828459045… — of a positive real number.

The cleanest definition is the integral one, because it needs no prior notion of exponentiation:

    ln x = ∫₁ˣ dt / t,   x > 0

From it everything follows. The function is the inverse of `EXP`, which Microsoft's page states
directly: "LN is the inverse of the EXP function", with the worked example `LN(EXP(3))` returning
3.

**Domain and range.** The domain is the positive reals (0, ∞); the range is all of ℝ. `ln` is
strictly increasing, concave, and unbounded in both directions — but it grows more slowly than
any positive power of *x*, which is why the logarithm of the largest finite double is a
three-digit number.

**The singularity.** At *x* → 0⁺ the function tends to −∞. This is a logarithmic singularity, not
a pole: it is unbounded but integrable, and ln x → −∞ slowly enough that x·ln x → 0.

**The branch cut.** Over the complex plane the logarithm is multivalued —
log z = ln|z| + i(arg z + 2πk) — and the principal branch is made single-valued by cutting along
the negative real axis, where the imaginary part jumps by 2π. `LN` is a real function and simply
refuses the cut and everything left of it. Readers who need the other branch want
`IMLN`, which returns the principal complex logarithm.

**The identities**, all of which a reader may want and none of which the documentation states:

    ln(ab) = ln a + ln b            ln(a/b) = ln a − ln b
    ln(xᵖ) = p · ln x               ln(1/x) = −ln x
    ln x = 2 · artanh((x−1)/(x+1))
    ln(1+z) = z − z²/2 + z³/3 − …   for |z| < 1

The last two are the ones that matter numerically, and they reappear below. Abramowitz & Stegun
chapter 4, section 4.1 is the standard tabulation of the logarithmic function's series,
continued fractions and functional relations.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `number` | "The positive real number for which you want the natural logarithm." | Yes |

Exactly one argument; the declared arity is one to one. The slot is numeric and subject to
ordinary to-number coercion ([coercion and lifting](../model/02-coercion-and-lifting.md)). The
declared coercion profile is the unary numeric scalar-or-array-elementwise one, so `LN` is a
scalar kernel that lifts over arrays.

Microsoft's wording carries the whole domain restriction in one adjective — *positive*. It does
not say what happens when the argument is not positive; see "Errors".

## Result and edge cases

Returns `Number`.

The reference engine's published battery is rendered beside this page. Note that the generator
marks several of its numeric rows **host-scoped**: their bits depend on the CPU the reference
engine ran on, not only on the input. That flag is not decoration — see "Numerical notes".

Qualitatively, the battery shows:

- **Zero and negative arguments** produce `#NUM!`.
- **`LN(TRUE)`** converts the logical to 1 and returns zero. Undocumented conversion.
- **Numeric text** converts and evaluates.
- **The largest finite double** and **the smallest positive subnormal** both produce finite
  results — the range of `ln` over the entire positive double range fits in three decimal digits
  either side of zero. Both rows are host-scoped.
- **An inline array** lifts elementwise.
- **An empty range** produces `#VALUE!`.

**A divergence worth recording.** The upstream live-Excel sweep behind this page's evidence
record explicitly *excludes* one subnormal-domain edge, describing it as a case where Excel
flushes the smallest positive subnormal to zero and therefore returns `#NUM!`. The reference
engine's battery row for that same input returns a finite number. Two different behaviours are
on record for the same input, from two parts of the same evidence base, and neither has been
re-observed by the Handbook. It is probe 1 below.

## Errors

| Error | Condition | Basis |
|---|---|---|
| `#NUM!` | The argument is zero or negative | Not stated on Microsoft's `LN` page; this is the reference engine's behaviour |
| `#VALUE!` | The argument does not convert to a number | Shared call model |
| propagated | An error value in the argument | Shared call model |

**A documentation gap, recorded as a finding.** Microsoft's `LN` page states the domain — "The
positive real number for which you want the natural logarithm" — but the page as retrieved
states **no error value** for a non-positive argument. The `#NUM!` above is the reference
engine's declared behaviour and the conventional Excel answer for a domain failure; it is not
quoted from documentation, and this page will not present it as documented. Confirming it
against Excel is a one-cell probe.

## Relationships

- **`EXP`** — the inverse, and the other half of the pair. Both are named as subjects of the same
  evidence record, and the upstream work identified a single shared substrate for the pair.
- **[LOG](FUNC.LOG.md)** — the general-base logarithm. The upstream evidence record's ruled-out
  list contains a specific, useful finding: `LOG(x, base)` is `ln(x)/ln(base)` for **every** base,
  with no special-casing for base 10 or base 2. `LN` is therefore not merely related to `LOG` —
  on that finding it is the routine `LOG` calls twice.
- **[LOG10](FUNC.LOG10.md)** — the base-10 logarithm, and a separate module in the reference
  engine rather than a wrapper.
- **`POWER` and `^`** — `x^y` is conventionally computed as exp(y·ln x), which makes `LN`'s
  accuracy a lower bound on the accuracy of every non-integer power.
- **`IMLN`** — the complex principal logarithm, defined across the branch cut `LN`
  refuses.
- **What Excel does not publish: there is no `LN1P`.** The standard remedy for the cancellation
  described below — C99's `log1p`, computing ln(1+x) accurately for tiny *x* — has no worksheet
  surface. A reader who needs it must build it, and the usual worksheet-level trick is
  `x * LN(1+x) / ((1+x) - 1)`, which recovers most of the lost digits by dividing by the *actual*
  perturbation rather than the intended one. That expression is arithmetic, not a Handbook claim
  about Excel.

## Numerical notes

The logarithm is one of the two functions (with `EXP`) that every elementary-function library is
judged on, and its implementation is a well-trodden path.

**Argument reduction.** Write the argument as *x* = *m*·2^k with the significand *m* in
[√2/2, √2); then

    ln x = k · ln 2 + ln m

The exponent *k* comes out of the bit pattern for free, so the transcendental work is confined to
a significand near 1. Because k·ln2 can be much larger than ln m, ln 2 must be carried in two
pieces (a head that is exact in binary64 and a tail), or the reduction throws away the accuracy
the kernel just earned. This is the Cody–Waite two-part-constant technique, and it is the single
most common place a hand-rolled logarithm loses its last digits.

**The kernel.** With *f* = (*m*−1)/(*m*+1), the identity ln m = 2·artanh f gives an **odd** series
in *f*, so the polynomial has half as many terms as a direct series in (*m*−1) and no cancellation
between successive terms. This is the arrangement used in Sun's fdlibm `e_log.c`, in Cephes, and
in essentially every derivative of them; Cody and Waite's *Software Manual for the Elementary
Functions* is the original systematic treatment, and Muller's *Elementary Functions: Algorithms
and Implementation* is the modern reference for the error analysis.

**Where naive evaluation loses precision — the ln(1+x) problem.** This is the hazard a
spreadsheet reader will actually meet. For small *x*, ln(1+x) ≈ *x*, so the true result is tiny —
but the argument 1+*x* has already been rounded to binary64, destroying every bit of *x* below
the ULP of 1. Compute `LN(1 + 0.0000001)` and the answer is not wrong in absolute terms, yet
several of its significant digits are noise, and for *x* below about 10⁻¹⁶ the answer is exactly
zero. The loss is in the **argument**, before the logarithm is ever called, so no amount of care
inside the kernel can recover it. The standard remedy is a separate `log1p` entry point that
takes *x* rather than 1+*x* and evaluates the series directly; Excel publishes no such function.
Compound-interest, log-returns and survival-analysis formulas are where this bites, and it bites
silently.

**Where relative accuracy is intrinsically hard.** Near *x* = 1 the result passes through zero,
so the *relative* error of any implementation is unbounded there no matter how good it is: a
half-ULP error in the argument becomes an arbitrarily large relative error in a result that is
approaching zero. Correctly-rounded logarithm implementations exist (the CRLIBM project and its
successors), but they cannot repair an argument that arrived already rounded. This distinction —
error in the function versus error inherited from the argument — is the one to keep straight when
reading any accuracy claim about `LN`.

**The substrate identified upstream.** The evidence record attached to this page identifies the
substrate as an x87 transcendental instruction sequence, and rules out every modern
libm — UCRT, glibc, MKL-VML, SVML — *and the correctly-rounded value* at the bit level. The
consequence for anyone implementing compatibility: the last bit of the x87 instructions involved
is CPU microcode, so on the hardest inputs agreement is a property of the **host CPU**, not of
the code. That is why the battery marks `LN`'s numeric rows host-scoped, and it means any
compatibility claim for this function carries a hardware axis in addition to the build and
platform axes the Charter already requires.

## What has not been checked

No Handbook vector suite exists for `LN`; `vectors/` publishes nothing for this function.

One evidence record lists `LN` among its subjects: **EV-MATH-0001**, a live-Excel sweep. Read the
record itself for its figures, its named build ambiguity and its own reader warning — this page
does not restate them. Three things about it are load-bearing for how much you should conclude:

1. The record's per-surface figure for `LN` is genuinely per-surface, but two other figures in the
   same lane are **group** totals over `LN` and `EXP` together, and one of those scores an
   out-of-repo reference implementation rather than the shipped kernel. They may not be read as
   `LN` pass rates.
2. The record carries a **host-CPU caveat** that every one of its surfaces inherits.
3. The record carries two **retractions** of upstream claims, both of which were wrong in the
   direction of overstating what was known.

The record is real evidence and it is more than most functions in this category have. It is not a
Handbook vector suite, it was not re-verified by the Handbook, and it does not license any
statement about inputs outside its corpus.

Inputs I would probe first:

1. **The smallest positive subnormal, and the few doubles above it.** The upstream sweep's own
   exclusion says Excel flushes it to zero and errors; the reference engine returns a number.
   This is a live contradiction inside the evidence base and the cheapest cell on the page.
2. **`LN(0)` and `LN(-1)`.** The `#NUM!` that no documentation states.
3. **`LN(1)`, and the doubles immediately either side of 1.** The zero-crossing, where relative
   accuracy is intrinsically hardest and where any implementation's worst relative error lives.
4. **`LN(1 + 2^-30)` and `LN(1 + 2^-52)`** — the cancellation described above, to establish how
   many digits a worksheet actually loses and whether the `x*LN(1+x)/((1+x)-1)` reconstruction
   recovers them in Excel's own arithmetic.
5. **`LN(EXP(x))` for a spread of x**, as a round-trip metamorphic test that needs no oracle
   table at all — it is the cheapest way to get a large number of informative cells.
6. **`LN(TRUE)`** — an undocumented conversion.
7. **The same corpus on two different CPU vendors**, given the host-CPU caveat. Without that,
   any bit-level result for this function is scoped to one machine and should be published as
   such.

## Excel versions, platforms and CPUs

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.LN.json; edits here are overwritten -->

Readings of LN with a recorded Excel build: 1,815 input(s) over 6 recorded environment(s),
Excel 16.0 build(s) 18429, 19929, 20131, 20228, 20430, all on Windows; recorded CPUs: AMD Zen
2, Intel Coffee Lake. 349 input(s) were also read in captures that record no build (some may
repeat inputs counted above); they are not counted. 2 same-input comparison(s) between
environments over 54 input-pair(s): 17 differ. A CPU dependence is observed (see below). The
Last Bit probes read LN on an Intel Core i7-9750H (Excel build 18429) and an AMD Ryzen 7 4800H
(Excel build 20131): 17 input(s) give results one unit apart in the last place; CPU, Excel
build and OS all differ between the two machines. No reading from a Mac, Excel for the web or
32-bit Excel is recorded.

Readings of `LN` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 45 | date not recorded |
| 16.0 build 20131 | Windows, bitness not recorded | AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | 349 | 2026-07-21 to 2026-07-23 |
| 16.0 build 18429 | Windows, 64-bit | Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | 52 | 2026-07-05 |
| 16.0 build 20131 | Windows, 64-bit | AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | 52 | 2026-07-03 |
| 16.0 build 20228 | Windows, 64-bit | CPU not recorded | 90 | 2026-08-09 |
| 16.0 build 20430 | Windows, 64-bit | CPU not recorded | 1,227 | 2026-09-29 |

Compared on the same inputs:

| Check | Differs in | Environments | Inputs compared | Inputs with different results |
|---|---|---|---:|---:|
| XC-LN-001 | excel-build | 16.0 build 19929 vs 16.0 build 20430 | 2 | 0 |
| XC-LN-002 | cpu, excel-build, os | 16.0 build 20131 vs 16.0 build 18429 | 52 | 17 |

Differences observed:

| Difference | Inputs | Excel | Result | Rechecked |
|---|---|---|---|---|
| XD-LN-001 | `3.677168983357276e-222` (`0x11f544676cf1bf2f`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `-509.8717474865596` (`0xc07fddf2ad7e124d`) | not-rechecked |
| XD-LN-001 | `3.677168983357276e-222` (`0x11f544676cf1bf2f`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `-509.87174748655957` (`0xc07fddf2ad7e124c`) | not-rechecked |
| XD-LN-002 | `8.703064031041396e-161` (`0x1eb393c13e56f40a`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `-368.5525248209083` (`0xc07708d724444078`) | not-rechecked |
| XD-LN-002 | `8.703064031041396e-161` (`0x1eb393c13e56f40a`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `-368.5525248209082` (`0xc07708d724444077`) | not-rechecked |
| XD-LN-003 | `2.5665223849584096e-91` (`0x2d20badbb5556728`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `-208.59269163749926` (`0xc06a12f75473f581`) | not-rechecked |
| XD-LN-003 | `2.5665223849584096e-91` (`0x2d20badbb5556728`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `-208.59269163749923` (`0xc06a12f75473f580`) | not-rechecked |
| XD-LN-004 | `6.926860213817972e-91` (`0x2d3693905c941f14`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `-207.59983682352345` (`0xc069f331dcfe7f08`) | not-rechecked |
| XD-LN-004 | `6.926860213817972e-91` (`0x2d3693905c941f14`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `-207.59983682352342` (`0xc069f331dcfe7f07`) | not-rechecked |
| XD-LN-005 | `3.884270428873988e-85` (`0x2e68258fb8991289`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `-194.362797730105` (`0xc0684b9c09fc3ba6`) | not-rechecked |
| XD-LN-005 | `3.884270428873988e-85` (`0x2e68258fb8991289`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `-194.36279773010497` (`0xc0684b9c09fc3ba5`) | not-rechecked |
| XD-LN-006 | `14971650289.99094` (`0x420be30a078fed72`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `23.429424269114342` (`0x40376deebfb7f46e`) | not-rechecked |
| XD-LN-006 | `14971650289.99094` (`0x420be30a078fed72`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `23.42942426911434` (`0x40376deebfb7f46d`) | not-rechecked |
| XD-LN-007 | `3.945299342281314e+26` (`0x4574658f6970c8b6`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `61.23973724816965` (`0x404e9eafb5cc42c6`) | not-rechecked |
| XD-LN-007 | `3.945299342281314e+26` (`0x4574658f6970c8b6`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `61.23973724816964` (`0x404e9eafb5cc42c5`) | not-rechecked |
| XD-LN-008 | `5.825300971601942e+29` (`0x461d6906690fdadb`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `68.53717836388884` (`0x40512261215c415e`) | not-rechecked |
| XD-LN-008 | `5.825300971601942e+29` (`0x461d6906690fdadb`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `68.53717836388883` (`0x40512261215c415d`) | not-rechecked |
| XD-LN-009 | `1.92097381483287e+65` (`0x4d7d2f67e7585ef8`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `150.3208632972981` (`0x4062ca44831b0c3a`) | not-rechecked |
| XD-LN-009 | `1.92097381483287e+65` (`0x4d7d2f67e7585ef8`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `150.32086329729808` (`0x4062ca44831b0c39`) | not-rechecked |
| XD-LN-010 | `9.251441304921035e+75` (`0x4fb4742155e8cc52`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `174.91866133068783` (`0x4065dd65ac726cee`) | not-rechecked |
| XD-LN-010 | `9.251441304921035e+75` (`0x4fb4742155e8cc52`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `174.9186613306878` (`0x4065dd65ac726ced`) | not-rechecked |
| XD-LN-011 | `8.014279760257424e+77` (`0x501baf60a3904e63`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `179.3802770810877` (`0x40666c2b3ad75615`) | not-rechecked |
| XD-LN-011 | `8.014279760257424e+77` (`0x501baf60a3904e63`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `179.38027708108768` (`0x40666c2b3ad75614`) | not-rechecked |
| XD-LN-012 | `2.6144394572616708e+104` (`0x559d2eada7837182`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `240.42989938923228` (`0x406e0dc1bc5d2a56`) | not-rechecked |
| XD-LN-012 | `2.6144394572616708e+104` (`0x559d2eada7837182`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `240.42989938923225` (`0x406e0dc1bc5d2a55`) | not-rechecked |
| XD-LN-013 | `3.076124606476124e+231` (`0x6fffb3b90594a629`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `533.020827041705` (`0x4080a82aa75e37f7`) | not-rechecked |
| XD-LN-013 | `3.076124606476124e+231` (`0x6fffb3b90594a629`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `533.0208270417049` (`0x4080a82aa75e37f6`) | not-rechecked |
| XD-LN-014 | `1.7339261820228413e-07` (`0x3e8745b9995cbfbe`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `-15.567707344427221` (`0xc02f22aa897c0721`) | not-rechecked |
| XD-LN-014 | `1.7339261820228413e-07` (`0x3e8745b9995cbfbe`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `-15.56770734442722` (`0xc02f22aa897c0720`) | not-rechecked |
| XD-LN-015 | `2615289.4664142975` (`0x4143f3fcbbb376b5`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `14.776885343968463` (`0x402d8dc3ea722c2a`) | not-rechecked |
| XD-LN-015 | `2615289.4664142975` (`0x4143f3fcbbb376b5`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `14.77688534396846` (`0x402d8dc3ea722c29`) | not-rechecked |
| XD-LN-016 | `5.98342654462436e-107` (`0x29e190bf0a3bd3c8`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `-244.58761154572932` (`0xc06e92cdb6ba751a`) | not-rechecked |
| XD-LN-016 | `5.98342654462436e-107` (`0x29e190bf0a3bd3c8`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `-244.5876115457293` (`0xc06e92cdb6ba7519`) | not-rechecked |
| XD-LN-017 | `4.5244421323604415e-95` (`0x2c58290e4b16f55a`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `-217.23608955054897` (`0xc06b278e0bac5120`) | not-rechecked |
| XD-LN-017 | `4.5244421323604415e-95` (`0x2c58290e4b16f55a`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `-217.23608955054894` (`0xc06b278e0bac511f`) | not-rechecked |

- XD-LN-001: The results differ by one unit in the last place; the AMD result is the correctly rounded one (0xc07fddf2ad7e124d). Intel's result equals the idealised x87 model (0xc07fddf2ad7e124c). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-002: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0xc07708d724444077). Intel's result equals the idealised x87 model (0xc07708d724444077). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-003: The results differ by one unit in the last place; the AMD result is the correctly rounded one (0xc06a12f75473f581). Intel's result equals the idealised x87 model (0xc06a12f75473f580). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-004: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0xc069f331dcfe7f07). Intel's result equals the idealised x87 model (0xc069f331dcfe7f07). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-005: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0xc0684b9c09fc3ba5). Intel's result equals the idealised x87 model (0xc0684b9c09fc3ba5). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-006: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0x40376deebfb7f46d). Intel's result equals the idealised x87 model (0x40376deebfb7f46d). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-007: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0x404e9eafb5cc42c5). Intel's result equals the idealised x87 model (0x404e9eafb5cc42c5). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-008: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0x40512261215c415d). Intel's result equals the idealised x87 model (0x40512261215c415d). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-009: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0x4062ca44831b0c39). Intel's result equals the idealised x87 model (0x4062ca44831b0c39). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-010: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0x4065dd65ac726ced). Intel's result equals the idealised x87 model (0x4065dd65ac726ced). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-011: The results differ by one unit in the last place; the AMD result is the correctly rounded one (0x40666c2b3ad75615). Intel's result equals the idealised x87 model (0x40666c2b3ad75614). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-012: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0x406e0dc1bc5d2a55). Intel's result equals the idealised x87 model (0x406e0dc1bc5d2a55). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-013: The results differ by one unit in the last place; the AMD result is the correctly rounded one (0x4080a82aa75e37f7). Intel's result equals the idealised x87 model (0x4080a82aa75e37f6). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-014: The results differ by one unit in the last place; the AMD result is the correctly rounded one (0xc02f22aa897c0721). Intel's result equals the idealised x87 model (0xc02f22aa897c0720). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-015: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0x402d8dc3ea722c29). Intel's result equals the idealised x87 model (0x402d8dc3ea722c29). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-016: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0xc06e92cdb6ba7519). Intel's result equals the idealised x87 model (0xc06e92cdb6ba7519). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-017: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0xc06b278e0bac511f). Intel's result equals the idealised x87 model (0xc06b278e0bac511f). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).

Observed (XS-LN-001, cpu): Handbook evidence record EV-MATH-0001 marks 6 of its counts for this
function as CPU-scoped: the counted result depends on how the host CPU executes x87
instructions. Every reading recorded here was taken on an AMD Zen 2 processor or on a host
whose CPU was not recorded; the one Intel reading (The Last Bit probes) differs from AMD on
some inputs; see the differences. OxFunc found Excel's last bit here follows the x87
F2XM1/FYL2X microcode on the hardest inputs (about 1 in 30). A dump of those x87 primitives on
an AMD Zen 2 and an AMD Zen 1 host (Excel only on the first) differed on none of 3,157 rows
(2026-08-23). The Handbook's model chapter 05 (version axes) states the same mechanism, that on
about one in two thousand of the hardest inputs the last bit tracks the microcode of the CPU
Excel runs on; The Last Bit (July 2026) measured it.

Evidence for every row: [`content/evidence/environments/FUNC.LN.json`](../../content/evidence/environments/FUNC.LN.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| logarithmic singularity | The unbounded but integrable behaviour of ln x as x → 0⁺ |
| branch cut | The negative real axis, where the complex logarithm's principal branch is severed |
| argument reduction | Splitting x = m·2^k so the kernel only sees a significand near 1 |
| Cody–Waite constant | A constant carried as head plus tail so the reduction stays exact |
| the ln(1+x) problem | Loss of significance in the *argument*, before the function is called |
| host-scoped | A result whose bits depend on the CPU as well as on the input |
| group figure | A count measured across several surfaces jointly; never a per-function rate |

## Sources

- Microsoft, "LN function" —
  <https://support.microsoft.com/en-us/office/ln-function-81fe1ed7-dac9-4acd-ba1d-07a142c6118f>.
  Retrieved for this page: the syntax, the "positive real number" argument description, the
  statement that `LN` is the inverse of `EXP`, and the three worked examples. The page as
  retrieved states no error value.
- Handbook evidence record `EV-MATH-0001` — live-Excel sweep naming `LN` as a subject; carries a
  host-CPU caveat, a per-surface/group distinction, two upstream retractions, and a ruled-out
  list covering the modern libm implementations and the correctly-rounded value.
- M. Abramowitz and I. A. Stegun, *Handbook of Mathematical Functions*, chapter 4, section 4.1 —
  the logarithmic function: series, continued fractions, functional relations.
- W. J. Cody and W. Waite, *Software Manual for the Elementary Functions* — the two-part-constant
  argument reduction and the artanh-form kernel.
- Sun Microsystems fdlibm, `e_log.c` — the reference implementation of the reduction and kernel
  described above; inherited by most current libm implementations and by Cephes.
- J.-M. Muller, *Elementary Functions: Algorithms and Implementation*, 3rd edition — the modern
  error analysis, and the correctly-rounded-logarithm literature (CRLIBM and successors).
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — to-number outcomes,
  scalar-kernel lifting, error propagation.
- `data/functions/FUNC.LN.json` and `data/presence/FUNC.LN.json` — identity, signature
  `LN(number)`, arity 1–1, declared axes, the implementing module and its Lean companion, as
  projected at OxFunc `473efa3`.
