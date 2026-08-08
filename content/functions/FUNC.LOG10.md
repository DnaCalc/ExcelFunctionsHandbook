---
schema: efh.function-page/v1
function_id: FUNC.LOG10
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
family: log10_fn
role_in_family: >-
  Sole member of its module; the dedicated base-10 logarithm, kept separate from LOG's
  general-base path in the reference engine even though the two compute the same quantity.
---

## What it computes

`LOG10(number)` returns the base-10 logarithm of a positive real number — "the power to which
10 must be raised to equal the given number", in the documentation's own gloss.

    LOG10(x) = log₁₀ x = ln x / ln 10 = log₁₀(e) · ln x,   x > 0

with log₁₀(e) = 0.4342944819032518…, the constant conventionally called M_LOG10E.

**Domain and range.** Domain (0, ∞); range all of ℝ. Strictly increasing, concave, with the same
logarithmic singularity at 0⁺ and the same absence of any finite pole as [LN](FUNC.LN.md) — the
two differ only by a positive constant factor, so every structural property transfers, including
the branch cut of the complex extension along the negative real axis.

**What makes base 10 special is not the mathematics — it is the exactness readers expect.** For
integer *k*, log₁₀(10^k) = *k* exactly, and Microsoft's documented examples assert exactly that:
`LOG10(100000)` and `LOG10(10^5)` are both shown as 5, and `LOG10(10)` as 1. Those are the
easiest values to state and among the harder ones to compute, because 10^k for *k* > 22 is not
exactly representable in binary64 and because a quotient of two inexact logarithms lands on an
integer only by luck. See "Numerical notes".

The useful identities:

    log₁₀(ab) = log₁₀ a + log₁₀ b       log₁₀(xᵖ) = p · log₁₀ x
    log₁₀ x = LN(x) / LN(10)            ⌊log₁₀ n⌋ + 1 = the number of decimal digits of n ≥ 1

The last is the reason `LOG10` appears in so many worksheets, and it is also the reason this page
has a warning in it.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `number` | "The positive real number for which you want the base-10 logarithm." | Yes |

Exactly one argument; the declared arity is one to one, and there is no base parameter — that is
what [LOG](FUNC.LOG.md) is for. The slot is numeric and subject to ordinary to-number coercion
([coercion and lifting](../model/02-coercion-and-lifting.md)); the declared coercion profile is
the unary numeric scalar-or-array-elementwise one, so `LOG10` is a scalar kernel that lifts over
arrays.

## Result and edge cases

Returns `Number`.

The reference engine's published battery is rendered beside this page, and several of its numeric
rows are marked **host-scoped** — their bits depend on the CPU as well as on the input.
Qualitatively, the battery shows:

- **Zero and negative arguments** produce `#NUM!`.
- **`LOG10(TRUE)`** converts the logical to 1 and returns zero. Undocumented conversion.
- **Numeric text** converts and evaluates.
- **The largest finite double** and **the smallest positive subnormal** produce finite results,
  both host-scoped. Note that the base-10 logarithm compresses the entire positive double range
  into roughly the interval from −324 to +309, which is why `LOG10` is the natural function for
  talking about magnitude.
- **An inline array** lifts elementwise.
- **An empty range** produces `#VALUE!`.

**An observation worth recording as a finding.** For the same inputs, the reference engine's
published battery rows for `LOG10` and for [LOG](FUNC.LOG.md) with its base defaulted to 10 —
mathematically the identical quantity — **do not agree in their final bits**. They agree to
nearly full precision and diverge at the end. That is exactly what one expects when one surface
folds the base-10 scaling into its kernel and the other computes a quotient of two logarithms,
and it is direct evidence, inside the reference engine, that the two are separate code paths
rather than one wrapping the other. Whether Excel's own `LOG10` and `LOG` differ in the same
places is unobserved, and it is the most informative probe on this page.

## Errors

| Error | Condition | Basis |
|---|---|---|
| `#NUM!` | The argument is zero or negative | Not stated on Microsoft's `LOG10` page; reference engine behaviour |
| `#VALUE!` | The argument does not convert to a number | Shared call model |
| propagated | An error value in the argument | Shared call model |

**A documentation gap, recorded as a finding.** Microsoft's `LOG10` page as retrieved carries the
domain only as an adjective — "the positive real number" — and states **no error value** for a
non-positive argument. The `#NUM!` above is the reference engine's behaviour and the conventional
Excel answer for a domain failure; it is not quoted from documentation and is not presented here
as documented.

## Relationships

- **[LN](FUNC.LN.md)** — the natural logarithm; `LOG10` is `LN` scaled by log₁₀(e). Both are
  subjects of the same evidence record and share the substrate identified there.
- **[LOG](FUNC.LOG.md)** — the general-base form, whose default base *is* 10. Two surfaces, one
  mathematical function, two code paths in the reference engine, and observably different last
  bits. Note the tension worth keeping in view: the evidence record's ruled-out list reports that
  `LOG(x, base)` is a plain quotient for *every* base, with no base-10 special case — yet Excel
  publishes a separate `LOG10` surface, which is where a dedicated base-10 routine would live if
  one exists. Nothing in the Handbook's record settles whether `LOG10` and `LOG(x)` agree in
  Excel.
- **`POWER(10, y)`** — the inverse. `POWER(10, LOG10(x))` returning *x* exactly is a stronger
  requirement than either function meets in general.
- **There is no real `LOG2` in Excel.** Base 2 goes through `LOG(x, 2)`. Given how often base-2
  logarithms appear in sizing and information-theoretic calculations, this is a notable absence,
  and the complex-valued [IMLOG2](FUNC.IMLOG2.md) exists while the real one does not.
- **`INT(LOG10(n)) + 1` for the digit count** is the single most common use of this function and
  the single most common way to get a wrong answer from it — see below.

## Numerical notes

**The scaled-kernel advantage.** A dedicated base-10 logarithm folds the constant log₁₀(e) into
the polynomial evaluation instead of dividing by ln 10 afterwards, which removes one rounding and
— more importantly — removes the error in the *representation* of ln 10. The standard fdlibm-family
arrangement is the one described on the [LN](FUNC.LN.md) page (extract the exponent *k*, reduce the
significand to a neighbourhood of 1, evaluate an odd polynomial in the artanh variable) with the
final combination performed as

    log₁₀ x = k · log₁₀2_hi + (k · log₁₀2_lo + log₁₀(e) · ln m)

where log₁₀2 is carried as a two-part Cody–Waite constant so the exponent term stays exact. Sun's
fdlibm `e_log10.c` is the canonical source; Cephes and most current libm implementations descend
from it. Muller's *Elementary Functions* gives the error analysis; Abramowitz & Stegun chapter 4,
section 4.1 tabulates the underlying relations.

**Exactness on powers of ten is not free.** log₁₀(10^k) = *k* is trivially true in mathematics
and delicate in binary64:

1. For 0 ≤ *k* ≤ 22, 10^k is exactly representable, and a good implementation returns exactly *k*.
2. For larger *k*, 10^k is **not** exactly representable — the stored double is a near neighbour —
   so the mathematically correct answer for the *stored value* is not the integer *k*, and an
   implementation that returns *k* is arguably rounding to what the user meant rather than what
   they wrote. Both behaviours are defensible; they are different functions.
3. A quotient `ln(x)/ln(10)` will generally miss the integer even in case 1, because neither
   logarithm is exact and their ratio is not.

**The digit-count trap.** `INT(LOG10(n)) + 1` is the textbook decimal-digit count, and it fails
whenever `LOG10` of an exact power of ten comes back a hair below the integer: the floor then
drops a whole digit, and the formula reports that 1000 has three digits. This is the
[INT](FUNC.INT.md) amplification hazard in its most common concrete form — a sub-ULP error in the
argument of a discontinuous function becoming a whole-unit error in the answer. The robust
worksheet forms are `LEN(TEXT(n,"0"))` for integers, or `INT(LOG10(n) + 0.5*10^-9) + 1` with a
deliberate, documented tolerance. The fragile form is the one everybody writes.

**Relative accuracy near x = 1.** As with `LN`, the result passes through zero at *x* = 1, so
relative error is unbounded there for any implementation. Absolute accuracy remains excellent;
the two must not be confused when reading an accuracy claim.

**The host-CPU caveat** attached to this page's evidence record applies here as it does to `LN`
and `LOG`: the substrate identified upstream is a legacy x87 transcendental sequence whose last
bit is microcode, so bit-level agreement is scoped to a machine as well as to a build.

## What has not been checked

No Handbook vector suite exists for `LOG10`; `vectors/` publishes nothing for this function.

One evidence record lists `LOG10` among its subjects: **EV-MATH-0001**, a live-Excel sweep. Its
figures and reader warning are rendered from the record itself. Two points govern how much you may
conclude: the record's `LOG10` figure is per-surface, but two other figures in the same lane are
**group** totals over `EXP` and `LN` only and the record states explicitly that they **may not be
attributed to `LOG10`**; and every surface in the record inherits the host-CPU caveat.

Nobody has checked `LOG10` against `LOG` in Excel, and the reference engine's own battery says
they differ.

Inputs I would probe first:

1. **`LOG10(10^k)` for k from 0 to 22, then 23 and beyond**, compared cell by cell against the
   integer *k*. This is the exactness question, it is where the two defensible behaviours in
   "Numerical notes" diverge, and the crossover at *k* = 23 is a sharp, cheap boundary.
2. **`LOG10(x)` against `LOG(x)` on the same *x***, over a spread of magnitudes. If they agree
   everywhere, Excel routes one through the other; if they differ, they are separate routines and
   the pattern of differences identifies which is the quotient. This settles a question the
   evidence record leaves open and needs no external oracle table.
3. **`INT(LOG10(n))+1` for n = 10, 100, 1000, 10^6, 10^15** — the digit-count trap, as it actually
   behaves in Excel's own arithmetic rather than in a library's.
4. **`LOG10(0)` and `LOG10(-1)`** — the `#NUM!` that no documentation states.
5. **The doubles immediately either side of 1**, where relative accuracy is intrinsically hardest.
6. **`LOG10(TRUE)`** — an undocumented conversion.
7. **The same corpus on two CPU vendors**, given the host-CPU caveat.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| scaled kernel | Folding log₁₀(e) into the polynomial instead of dividing afterwards |
| Cody–Waite constant | A constant carried as head plus tail so the exponent term stays exact |
| exactly representable power of ten | 10^k for 0 ≤ k ≤ 22 in binary64; beyond that the stored value is a neighbour |
| digit-count trap | `INT(LOG10(n))+1` losing a digit when the logarithm falls a hair short |
| host-scoped | A result whose bits depend on the CPU as well as on the input |
| group figure | A count measured across several surfaces jointly; never a per-function rate |

## Sources

- Microsoft, "LOG10 function" —
  <https://support.microsoft.com/en-us/office/log10-function-c75b881b-49dd-44fb-b6f4-37e3486a0211>.
  Retrieved for this page: the syntax, the "positive real number" argument description, the gloss
  of the result as the power 10 is raised to, and the four worked examples including the two that
  assert an exact 5. The page as retrieved states no error value.
- Handbook evidence record `EV-MATH-0001` — live-Excel sweep naming `LOG10` as a subject; carries
  the host-CPU caveat, the per-surface/group distinction with an explicit prohibition on
  attributing the group figures here, and the substrate identification.
- M. Abramowitz and I. A. Stegun, *Handbook of Mathematical Functions*, chapter 4, section 4.1.
- Sun Microsystems fdlibm, `e_log10.c` — the canonical scaled-kernel arrangement described above;
  W. J. Cody and W. Waite, *Software Manual for the Elementary Functions*, for the two-part
  constant technique; J.-M. Muller, *Elementary Functions*, 3rd edition, for the error analysis.
- Handbook, [LN](FUNC.LN.md) and [INT](FUNC.INT.md) — the shared kernel, and the amplification
  hazard behind the digit-count trap.
- `data/functions/FUNC.LOG10.json` and `data/presence/FUNC.LOG10.json` — identity, signature
  `LOG10(number)`, arity 1–1, declared axes, implementing module and Lean companion, as projected
  at OxFunc `473efa3`.
