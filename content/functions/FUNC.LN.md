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
[IMLN](FUNC.IMLN.md), which returns the principal complex logarithm.

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
- **[IMLN](FUNC.IMLN.md)** — the complex principal logarithm, defined across the branch cut `LN`
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
substrate as a legacy Microsoft x87 CRT transcendental sequence, and rules out every modern
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
