---
schema: efh.function-page/v1
function_id: FUNC.FISHER
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Fisher method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.fisher"
    role: "documented description, the #VALUE! and #NUM! conditions, and the statement that the equation is the Fisher transformation"
  - work: "Microsoft Support — FISHER function"
    locator: "https://support.microsoft.com/en-us/office/fisher-function-d656523c-5076-4f95-b87b-7741bf236c69"
    role: "the worksheet-surface documentation page; not retrievable at curation time (the host refused the request)"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 4"
    locator: "elementary transcendental functions; the inverse hyperbolic tangent and its series"
    role: "the mathematics of artanh, which is what FISHER computes"
  - work: "R. A. Fisher, 'Frequency distribution of the values of the correlation coefficient in samples from an indefinitely large population' (1915) and the 1921 z-transformation paper"
    locator: null
    role: "origin of the variance-stabilizing transformation this function is named for"
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
family: fisher_fn
role_in_family: >-
  Sole member of its module; the forward half of the Fisher/inverse-Fisher pair, and the member
  that carries a domain guard while its inverse carries none.
---

## What it computes

`FISHER(x)` is the inverse hyperbolic tangent:

    FISHER(x)  =  artanh(x)  =  ½ · ln( (1 + x) / (1 − x) )

- **Domain:** the open interval −1 < x < 1.
- **Range:** the whole real line.
- **Poles:** logarithmic singularities at x = −1 and x = +1, where the function tends to −∞
  and +∞ respectively. There is no branch cut inside the domain.
- **Symmetry:** odd. artanh(−x) = −artanh(x), and artanh(0) = 0 exactly.
- **Derivative:** 1 / (1 − x²), which is 1 at the origin and grows without bound at the ends.
- **Limiting form near zero:** artanh(x) = x + x³/3 + x⁵/5 + … , so for small x the function
  is x to first order.

Microsoft's documentation states the purpose rather than the formula in text: the
transformation "produces a function that is normally distributed rather than skewed", and it
is for hypothesis testing on the correlation coefficient. That is the statistical reason the
function exists. Pearson's sample correlation r is confined to [−1, 1] and its sampling
distribution is badly skewed near the ends; artanh(r) is approximately normal with variance
1/(n − 3) more or less independently of the true correlation, which is what makes confidence
intervals for a correlation tractable. This is R. A. Fisher's variance-stabilizing
z-transformation, and the function is a spreadsheet spelling of it.

The mathematics and the statistics are separable. `FISHER` is artanh; the normal-approximation
story is why Excel ships it under a statistician's name rather than as `ATANH` — which Excel
also ships, in the Math category. See **Relationships**.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `x` | The value to transform. Required. | — |

One argument, no options; the reference engine declares an arity of exactly 1. `x` is a numeric
slot subject to ordinary to-number coercion, so numeric text and logicals convert by the shared
rules — see [Coercion and lifting](../model/02-coercion-and-lifting.md).

The trap in that sentence is worth naming: **`TRUE` converts to 1, and 1 is outside the
domain.** `FISHER(TRUE)` is therefore a domain failure rather than a value, and so is
`FISHER("1")`. Nothing in the documentation warns about this, because the documentation
discusses the domain in terms of x and the coercion in terms of "nonnumeric".

## Result and edge cases

Returns `Number`.

- **At the origin**, artanh(0) = 0, and 0 is exactly representable, so an implementation that
  gets anything other than a signed zero here is doing something wrong.
- **Approaching the poles**, the true value diverges. The documented behaviour is not a
  saturating value and not an infinity: it is `#NUM!` for the whole closed complement
  |x| ≥ 1, which means the endpoints themselves error rather than returning ±∞.
- **Very small x**, including subnormals, should return x itself to within a rounding, because
  artanh(x) = x + O(x³).
- **Array arguments** lift elementwise: the reference engine classifies this surface as a
  unary numeric scalar-or-array kernel, so an array argument produces an array of results with
  element-local errors, following the lift baseline in
  [Coercion and lifting](../model/02-coercion-and-lifting.md).

The projected battery beside this page records the reference engine's own answers on a fixed
probe list. Those are OxFunc's answers; no Excel was involved in producing them.

## Errors

As documented by Microsoft:

| Error | Condition |
|---|---|
| `#VALUE!` | `x` is nonnumeric |
| `#NUM!` | `x` ≤ −1, or `x` ≥ 1 |

The `#NUM!` condition is stated with non-strict inequalities on both sides, so both endpoints
are errors. Error values arriving in the argument propagate under the shared coercion
discipline; the Handbook has not observed any of this in Excel itself.

## Relationships

- **[FISHERINV](FUNC.FISHERINV.md)** — the inverse, tanh. Microsoft states the round trip
  directly: if y = FISHER(x) then FISHERINV(y) = x. The pair is asymmetric in a way that
  matters to implementers: `FISHER` has a bounded domain and a documented `#NUM!` guard,
  while `FISHERINV` has an unbounded domain and no documented `#NUM!` condition at all.
- **`ATANH`** — the Math-category spelling of the same mathematical function. Excel publishes
  both. They are documented to compute the same thing; whether they compute it with the same
  code, and therefore return the same bits, is not established here and would need evidence.
  Treating `FISHER` and `ATANH` as interchangeable is an assumption, not a fact.
- **`CORREL` / `PEARSON`** — the usual producers of the r that gets fed to `FISHER`.
- **`NORM.S.INV`, `NORM.S.DIST`** — the usual next step: the transformed value is compared
  against a standard normal quantile, and `FISHERINV` maps the interval endpoints back.
- **`TANH` / `ATANH`** — the hyperbolic pair `FISHER`/`FISHERINV` shadows.

## Numerical notes

The interesting part of `FISHER` is that the textbook formula is the wrong way to evaluate it.

**Cancellation near zero.** Written as ½·ln((1 + x)/(1 − x)), the quotient is a number close to
1 for small x, and taking a logarithm of something close to 1 discards the leading significant
digits: the information about x lives in the difference between the quotient and 1, and that
difference has already been rounded away by the division. The accurate rearrangement is

    artanh(x) = ½ · log1p( 2x / (1 − x) )

which keeps the small quantity small and hands it to a routine designed for arguments near 1.
This is what fdlibm's `atanh` does, and it is the standard treatment in the literature — the
same cancellation lesson that governs `LN(1+x)` throughout the Handbook. Cephes and Boost use
the equivalent form; Numerical Recipes discusses the general pattern under the heading of
avoiding subtractive cancellation.

**Overflow is not the failure mode; loss of the domain is.** As |x| → 1 the quotient grows, but
only to about 2/(1 − |x|), and the logarithm tames it, so a double-precision evaluation stays
finite until x is within a rounding of ±1. The practical hazard is the other side: 1 − x is
computed exactly only when x is not extremely close to 1 (Sterbenz), and once x is within half
an ulp of 1 the subtraction returns the rounded gap and the result is dominated by that single
rounding.

**What the reference engine does.** OxFunc's kernel evaluates the naive form —
`0.5 * ((1 + x)/(1 − x)).ln()` — behind an `|x| ≥ 1 → #NUM!` guard. That is the direct
transcription of the documented formula, and it is exposed to exactly the near-zero
cancellation described above. This page records that as an observation about the reference
implementation. It says nothing about what Excel does, which nobody in this record has
determined.

**What a careful implementation does.** Use the `log1p` form; return x unchanged (or x + x³/3)
below the threshold where the series is indistinguishable from the function at working
precision; preserve the sign of zero; and decide deliberately whether the endpoints raise or
saturate, since Excel's documented answer there is an error rather than an infinity.

## What has not been checked

No Handbook vector suite exists for `FISHER`, and no evidence record in the Handbook's
collection names this surface as a subject. **Nobody has checked `FISHER` against Excel within
this record.** There is no per-surface Excel comparison for it in the evidence organ at all —
not a pass count, not a single witness point.

What does exist is weaker than it looks: a projected battery of the reference engine's own
answers on a fixed probe list, with no Excel on either side of the comparison, and a module
with no unit tests of its own in the presence projection.

Inputs I would probe first, and why:

1. **`FISHER(0)`** — the one point where the exact answer is known and representable. If this
   is not an exact zero, everything downstream is suspect.
2. **A ladder of small x** — 2⁻²⁰, 2⁻³⁰, 2⁻⁴⁰ — against the correctly rounded artanh. This is
   the cancellation probe: the naive and `log1p` forms diverge here and nowhere else, so the
   residuals identify which formula Excel is using. That single fact would settle more about
   this function than a broad random sweep.
3. **`FISHER(0.5)` and `FISHER(-0.5)`** — a mid-range pair, to test the odd symmetry: an
   implementation that folds on |x| gives an exactly antisymmetric answer, one that does not
   may differ in the last bit between the two signs.
4. **The endpoints and just inside them**: x = 1, x = −1, x = nextafter(1, 0). The documented
   rule says the endpoints are `#NUM!`; the point just inside is the largest finite value the
   function can return and is where any guard-ordering bug shows up.
5. **`FISHER(TRUE)` and `FISHER("0.5")`** — the coercion path, including the `TRUE` → 1 → out
   of domain trap named above.
6. **An array argument**, to confirm elementwise lifting and element-local errors on a mixed
   array containing one in-domain and one out-of-domain element.

Until those exist, this page describes the mathematics with confidence and Excel's behaviour
only as documented.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| artanh | The inverse hyperbolic tangent, the function FISHER computes |
| variance-stabilizing transform | A change of variable making the sampling variance roughly constant; Fisher's reason for the z-transformation |
| cancellation | Loss of significant digits when nearly equal quantities are subtracted, or a logarithm is taken near 1 |
| `log1p` form | The rearrangement ½·log1p(2x/(1−x)) that avoids that cancellation |
| domain guard | The `#NUM!` test on \|x\| ≥ 1 that precedes evaluation |

## Sources

- Microsoft Learn, "WorksheetFunction.Fisher method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.fisher>
  (description, the `#VALUE!` nonnumeric condition, and the `#NUM!` condition for x ≤ −1 or
  x ≥ 1). The worksheet-surface page at `support.microsoft.com` documents the same function;
  it was not retrievable at curation time, so nothing on this page is quoted from it.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 4 — the inverse
  hyperbolic tangent, its series, and its relation to the logarithm.
- fdlibm `s_atanh.c`; Cephes; Boost.Math — the `log1p` rearrangement named under
  **Numerical notes**.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.FISHER.json` (arity, unary numeric scalar-or-array
  lift profile, deterministic and non-volatile classification) and
  `data/presence/FUNC.FISHER.json` (implementing module, no sibling surfaces).
- OxFunc `crates/oxfunc_core/src/functions/fisher_fn.rs` at commit `473efa3` — the kernel form
  described under **Numerical notes**.
