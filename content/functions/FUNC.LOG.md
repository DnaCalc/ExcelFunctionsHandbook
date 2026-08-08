---
schema: efh.function-page/v1
function_id: FUNC.LOG
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
family: log_fn
role_in_family: >-
  Sole member of its module; the general-base logarithm, and — on the upstream sweep's own
  finding — a quotient of two natural logarithms for every base, with no special case.
---

## What it computes

`LOG(number, [base])` returns the logarithm of *number* to the given *base*: the power to which
the base must be raised to obtain the number. Microsoft's own gloss of the first example is
exactly that — "the power to which the base must be raised to equal 10."

    LOG(x, b) = log_b x = ln x / ln b

The change-of-base identity in the second form is not one implementation choice among many; it is
the *definition* of a logarithm to an arbitrary base once the natural logarithm exists. Every
other property follows:

- **Domain.** *x* > 0 and *b* > 0 with *b* ≠ 1. The base must be positive because b^y is not
  real-valued for negative *b* at fractional *y*, and it must differ from 1 because 1^y is 1 for
  every *y* — there is no power of 1 that gives 2, and log₁ is not a function.
- **Range.** All of ℝ.
- **The pole in the base.** As *b* → 1, ln *b* → 0 and the quotient diverges. This is a genuine
  pole of the two-variable function, and it is the numerical hazard specific to `LOG` — see
  "Numerical notes".
- **Monotonicity flips.** For *b* > 1 the function increases in *x*; for 0 < *b* < 1 it
  decreases. log_b x and log_{1/b} x are negatives of one another.
- **Fixed points.** LOG(b, b) = 1 and LOG(1, b) = 0 for every admissible base.

Microsoft's three documented examples cover the shape: base omitted gives base 10, an integer
base gives the expected integer answer for an exact power, and a base of approximately e
reproduces the natural logarithm.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | "The positive real number for which you want the logarithm." Required. | — |
| `base` | "The base of the logarithm. If base is omitted, it is assumed to be 10." | 10 |

The declared arity is one to two, matching. Both slots are numeric and subject to ordinary
to-number coercion ([coercion and lifting](../model/02-coercion-and-lifting.md)).

**The default is 10, not e.** This is the one thing about `LOG` that costs people real money:
in most programming languages and in most mathematical writing outside engineering, an unadorned
`log` means the natural logarithm. In Excel it means base 10. A formula ported from Python, R,
C or MATLAB that reads `LOG(x)` is wrong by a constant factor of ln 10 ≈ 2.302585…, and it is
wrong *quietly*, because the result is still a plausible number. The Excel spelling of the
natural logarithm is [LN](FUNC.LN.md).

## Result and edge cases

Returns `Number`.

The reference engine's published battery is rendered beside this page, and several of its
numeric rows are marked **host-scoped** — their bits depend on the CPU as well as on the input.
Qualitatively, the battery shows:

- **Zero and negative first arguments** produce `#NUM!`.
- **`LOG(TRUE)`** converts the logical to 1 and returns zero. Undocumented conversion.
- **Numeric text** converts and evaluates.
- **The largest finite double** and **the smallest positive subnormal** produce finite results,
  both host-scoped.
- **An inline array** in the first slot lifts elementwise.
- **An empty range** produces `#VALUE!`.

The battery does not probe the base argument at all — every row exercises the default. The whole
of `LOG`'s distinctive behaviour therefore sits outside the published battery, which is worth
knowing before drawing any conclusion from it.

## Errors

| Error | Condition | Basis |
|---|---|---|
| `#NUM!` | *number* is zero or negative | Not stated on Microsoft's `LOG` page; reference engine behaviour |
| `#NUM!` or `#DIV/0!` | *base* is 1, so the denominator ln *b* is zero | **Neither documented nor observed here** |
| `#NUM!` | *base* is zero or negative | Not stated on Microsoft's `LOG` page |
| `#VALUE!` | An argument does not convert to a number | Shared call model |
| propagated | An error value in either argument | Shared call model |

**A documentation gap, recorded as a finding.** Microsoft's `LOG` page as retrieved states the
domain in adjectives — "the positive real number" for the first argument — and documents **no
error value at all**, for any argument, including the base. Every error row above is therefore
either the reference engine's declaration or, in the base-of-1 row, genuinely unknown. The
base-of-1 case is the sharpest gap: it is a true mathematical pole, a one-cell probe, and nothing
in the Handbook's record says what Excel returns.

## Relationships

- **[LN](FUNC.LN.md)** — the natural logarithm, and (on the upstream finding below) the routine
  `LOG` calls twice. `LOG(x, EXP(1))` and `LN(x)` are the same value mathematically; whether they
  agree in the last bit is a different question and is not settled here.
- **[LOG10](FUNC.LOG10.md)** — the dedicated base-10 logarithm. `LOG(x)` with the base omitted and
  `LOG10(x)` compute the same mathematical quantity **through different code**: `LOG10` is its own
  module in the reference engine, while `LOG` reaches base 10 through the general path. That they
  are separate paths is visible in the reference engine's own published batteries — on shared
  inputs, the `LOG` and `LOG10` rows agree to nearly full precision and **disagree in their final
  bits**, which is the signature of a quotient against a scaled kernel. Whether Excel's two
  surfaces disagree in the same way is unobserved, and the comparison is the sharpest available
  test of the "no special-casing" finding below.
- **`POWER` and `^`** — the inverse relation: `POWER(b, LOG(x, b))` is *x*, up to rounding.
- **[IMLOG10](FUNC.IMLOG10.md) / [IMLOG2](FUNC.IMLOG2.md)** — the complex-valued counterparts.
  Excel publishes a complex base-2 logarithm but **no real one**: there is no `LOG2`. Base 2 on
  the worksheet goes through `LOG(x, 2)`, which is exactly the case the finding below is about.

**A retracted upstream claim, recorded here because it is instructive.** One of the reference
engine's own documents asserted that Excel special-cases base 10 and base 2, using dedicated
`log10`/`log2` routines that are more accurate than the naive quotient. The live-Excel sweep
behind this page's evidence record **refuted that**: on the sweep's own finding, `LOG(x, base)` is
ln(x)/ln(base) for *every* base, and the special-casing was removed. The two upstream documents
contradict each other and the record carries the retraction explicitly. The Handbook publishes
both sides rather than silently preferring one, and notes that the refuting evidence is the more
recent and the more direct.

## Numerical notes

`LOG` is the clearest case in the math category of a function whose difficulty is **not** in its
kernel. The kernel is [LN](FUNC.LN.md), and its analysis belongs on that page. What belongs here
is what the division does.

**Two logarithms and a divide.** If each `ln` is correctly rounded to within half an ULP and the
division adds another half, the quotient carries about 1.5 ULP of error in the best case — before
any conditioning is considered. That is why a dedicated `log10` (which folds the 1/ln10 scaling
into the polynomial) beats `ln(x)/ln(10)`, and it is the reason the upstream claim of
special-casing was plausible enough to be believed for a while. On the sweep's finding Excel does
*not* take the more accurate route, which means a compatibility implementation must reproduce the
quotient, including its extra rounding, rather than improve on it. This is one of the recurring
shapes in this Handbook: matching Excel and being accurate are different objectives, and here
they point in different directions.

**The base-near-1 pole is the real hazard.** The condition number of the quotient with respect to
the base is proportional to 1/ln *b*. As *b* → 1 the denominator goes to zero, so the relative
error in the *result* is amplified without bound:

- At *b* = 1.0001, ln *b* is about 10⁻⁴, and every ULP of error in the denominator is amplified
  by four orders of magnitude in the answer.
- Worse, ln *b* for *b* near 1 is itself the ln(1+x) cancellation problem described on the `LN`
  page: the base *as a double* has already lost the low bits of *b* − 1.

So `LOG(x, 1.0000001)` is a compounded disaster — an inaccurate denominator, divided into,
amplified. There is a correct way to compute it (evaluate the denominator as `log1p(b-1)` from the
exact perturbation, which needs the perturbation rather than the perturbed value), and there is no
way to reach it from the worksheet, because Excel publishes no `log1p`. A reader whose base is
near 1 — a growth rate, a per-period interest factor — should restructure the formula rather than
trust the function. The honest statement is that this is a domain where the *mathematics* is
ill-conditioned, not a domain where any implementation is at fault.

**Argument reduction and the kernel** are `LN`'s, and the substrate identification, the ruled-out
libm list and the host-CPU caveat on the last bit all carry across from the same evidence record.
For the standard treatment of the underlying kernel see Cody & Waite, *Software Manual for the
Elementary Functions*, and Muller, *Elementary Functions*; Abramowitz & Stegun chapter 4,
section 4.1 tabulates the change-of-base relations.

## What has not been checked

No Handbook vector suite exists for `LOG`; `vectors/` publishes nothing for this function.

One evidence record lists `LOG` among its subjects: **EV-MATH-0001**, a live-Excel sweep. Its
figures, build ambiguity and reader warning are rendered from the record itself and are not
restated here. What matters for calibration:

1. `LOG`'s figure in that record is per-surface, but two other figures in the same lane are
   **group** totals over `EXP` and `LN` only and **may not be attributed to `LOG` at all** — the
   record says so explicitly.
2. The record carries a **host-CPU caveat**: on the hardest inputs the last bit is a property of
   the machine.
3. The record carries the **retraction** of the base-10/base-2 special-casing claim described
   above.

`LOG` also appears inside the group membership of a second, structural record without being one
of its subjects; that record's counts are not claimed here.

Nobody has checked the *base* argument against Excel within the Handbook's record — and the base
is where this function differs from every other logarithm on the surface.

Inputs I would probe first:

1. **`LOG(2, 1)`.** The pole. One cell, and there is currently no statement anywhere in the
   Handbook's record about what it returns.
2. **`LOG(8, 2)` against `LOG(8, 2.0000000000000004)`**, and `LOG(1000, 10)` against
   `LOG10(1000)`. These are the cells that test the no-special-casing finding directly: if base 10
   is special-cased, `LOG(x)` and `LOG10(x)` will differ in the last bit for some *x*; if it is
   not, they will differ in a *different* pattern, because one is a quotient and the other is not.
3. **`LOG(x, b)` for *b* just above 1** — 1.1, 1.01, 1.001, 1.0001 — at a fixed *x*. This maps the
   amplification and shows a reader exactly where the function stops being usable.
4. **Negative and zero bases**, and a base between 0 and 1 (`LOG(8, 0.5)`, which should be −3).
   The fractional-base branch is entirely unprobed.
5. **`LOG(0)`, `LOG(-1)`** — the `#NUM!` that no documentation states.
6. **`LOG(TRUE)` and `LOG("2.5")`** — undocumented conversions the reference engine accepts.
7. **An array in the *base* slot**, which no published battery row touches.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| change of base | log_b x = ln x / ln b; the identity that defines the general-base logarithm |
| pole in the base | The divergence as base → 1, where ln base → 0 |
| condition number | How much a relative error in an input is amplified in the result |
| host-scoped | A result whose bits depend on the CPU as well as on the input |
| group figure | A count measured across several surfaces jointly; never a per-function rate |

## Sources

- Microsoft, "LOG function" —
  <https://support.microsoft.com/en-us/office/log-function-4e82f196-1ca9-4747-8fb0-6c4a3abb3280>.
  Retrieved for this page: the syntax, both argument descriptions, the statement that an omitted
  base is assumed to be 10, and the three worked examples. The page as retrieved documents no
  error value.
- Handbook evidence record `EV-MATH-0001` — live-Excel sweep naming `LOG` as a subject; carries
  the host-CPU caveat, the per-surface/group distinction, the ruled-out list, and the retraction
  of the base-10/base-2 special-casing claim.
- M. Abramowitz and I. A. Stegun, *Handbook of Mathematical Functions*, chapter 4, section 4.1 —
  logarithmic function relations including change of base.
- W. J. Cody and W. Waite, *Software Manual for the Elementary Functions*; J.-M. Muller,
  *Elementary Functions: Algorithms and Implementation*, 3rd edition — the kernel and its error
  analysis, and the accuracy argument for a dedicated base-10 routine over a quotient.
- Handbook, [LN](FUNC.LN.md) — the kernel this function is built on, including the ln(1+x)
  cancellation that governs the base-near-1 case.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — to-number outcomes,
  lifting, error propagation.
- `data/functions/FUNC.LOG.json` and `data/presence/FUNC.LOG.json` — identity, signature
  `LOG(number, [base])`, arity 1–2, declared axes, the implementing module and its Lean companion,
  as projected at OxFunc `473efa3`.
