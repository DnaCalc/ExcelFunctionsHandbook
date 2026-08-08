---
schema: efh.function-page/v1
function_id: FUNC.FISHERINV
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.FisherInv method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.fisherinv"
    role: "documented description, the round-trip statement, and the single #VALUE! condition"
  - work: "Microsoft Support — FISHERINV function"
    locator: "https://support.microsoft.com/en-us/office/fisherinv-function-62504b39-415a-4284-a285-19c8e82f86bb"
    role: "the worksheet-surface documentation page; not retrievable at curation time (the host refused the request)"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 4"
    locator: "elementary transcendental functions; the hyperbolic tangent"
    role: "the mathematics of tanh, which is what FISHERINV computes"
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
family: fisherinv_fn
role_in_family: >-
  Sole member of its module; the inverse half of the Fisher pair, total on the reals, and the
  member whose only recorded live-Excel observation is a saturation at the top of its range.
---

## What it computes

`FISHERINV(y)` is the hyperbolic tangent:

    FISHERINV(y)  =  tanh(y)  =  (e^{2y} − 1) / (e^{2y} + 1)
                              =  (e^{y} − e^{−y}) / (e^{y} + e^{−y})

- **Domain:** the whole real line. This function has no interior domain restriction, which is
  the structural difference between it and its partner.
- **Range:** the open interval (−1, 1). The endpoints are approached but never attained
  mathematically.
- **Symmetry:** odd. tanh(−y) = −tanh(y), tanh(0) = 0 exactly.
- **Derivative:** 1 − tanh²(y) = sech²(y); it is 1 at the origin and decays like 4e^{−2|y|}.
- **Asymptotics:** tanh(y) → ±1 exponentially fast. By |y| ≈ 19 the difference from 1 is below
  the double-precision spacing at 1, so every implementation saturates well before the argument
  gets large.
- **Series at the origin:** tanh(y) = y − y³/3 + 2y⁵/15 − … , so for small y the function is y
  to first order.

Microsoft states the relationship rather than the formula: "If y = FISHER(x), then
FISHERINV(y) = x." That is the defining property, and it is exact as mathematics — tanh and
artanh are genuine mutual inverses on (−1, 1) ↔ ℝ. It is *not* exact as floating point, and
the round trip is one of the more useful probes this pair offers; see **Numerical notes**.

The statistical use is the return leg of the Fisher z-transformation: build a confidence
interval on the transformed scale, where the sampling distribution is approximately normal,
then map the interval endpoints back to correlations with `FISHERINV`. Because tanh is
monotone, the mapped interval is still an interval, and because it is nonlinear, the mapped
interval is asymmetric about the point estimate — which is the whole point.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `y` | The value to transform back. Required. | — |

One argument; the reference engine declares an arity of exactly 1. `y` is a numeric slot under
ordinary to-number coercion, so numeric text and logicals convert by the shared rules in
[Coercion and lifting](../model/02-coercion-and-lifting.md). Unlike `FISHER`, there is no
value of `y` that is out of domain, so the `TRUE` → 1 conversion here is unremarkable rather
than a trap.

## Result and edge cases

Returns `Number`, always in [−1, 1] once floating point is accounted for.

- **At the origin**, tanh(0) = 0 exactly.
- **Small `y`**, including subnormals, returns `y` to within a rounding, because
  tanh(y) = y + O(y³).
- **Large positive `y`** saturates at exactly 1, and large negative `y` at exactly −1. This is
  a floating-point fact, not a mathematical one: the true value never reaches the endpoint,
  but the nearest double to it is the endpoint once |y| is beyond roughly 19.
- **The saturation is where the implementations differ**, and this one has a recorded
  observation behind it. OxFunc's kernel carries an explicit guard, attributed to the upstream
  defect stream `BUG-FUNC-027` (class A6), for the case where e^{2y} overflows to +∞ and the
  direct quotient becomes ∞/∞ — a NaN. The upstream note records a live Excel observation that
  Excel saturates to 1 there rather than producing a non-number, and the guard exists to match
  that. Two things follow. First, this is a real behavioural fact with a named source, and it
  is the only such fact this page has. Second, it is upstream's observation, on a build named
  in upstream's record, not a Handbook verification — the Handbook has not reproduced it.
- **The negative side needs no guard**: e^{2y} underflows to 0 and the direct quotient gives
  −1 without special handling. The asymmetry in the guard is a consequence of the asymmetry of
  the formula, not a modelling choice.
- **Array arguments** lift elementwise; the surface is classified as a unary numeric
  scalar-or-array kernel.

## Errors

As documented by Microsoft:

| Error | Condition |
|---|---|
| `#VALUE!` | `y` is nonnumeric |

That is the entire documented error surface — one row. There is **no documented `#NUM!`
condition**, which is unusual for a Statistical-category function and is the clearest
structural contrast with `FISHER`. Error values arriving in the argument propagate under the
shared coercion discipline.

What the documentation does not say, and this page will not invent: what happens for an
argument so large that the intermediate exponential overflows. The documented error table has
no row for it, the mathematical answer is a value inside the range, and the observed answer in
the upstream record is saturation — but "documented" and "observed once, upstream" are
different warrants and the Handbook keeps them apart.

## Relationships

- **[FISHER](FUNC.FISHER.md)** — the forward transform, artanh. Microsoft documents the round
  trip. Note the asymmetry: `FISHER` guards its domain with `#NUM!`; `FISHERINV` has no domain
  to guard and documents no `#NUM!` at all.
- **`TANH`** — the Math-category spelling of the same mathematical function. Excel ships both.
  They are documented to compute the same thing; whether they are the same computation, and
  therefore agree in the last bit, is unestablished and would need evidence. Do not assume it.
- **`CORREL` / `PEARSON`** — the correlation estimators whose transformed confidence bounds
  are mapped back through this function.
- **`NORM.S.INV`** — the quantile that sets those bounds on the transformed scale.
- **`EXP`, `LN`** — the substrate. Any statement about `FISHERINV`'s last bits is ultimately a
  statement about the exponential the platform provides.

## Numerical notes

**Three formulas, three failure modes.** The direct form (e^{2y} − 1)/(e^{2y} + 1) is the one
Microsoft's equation image shows and the one the reference engine implements. It overflows for
large positive y — hence the guard above — and it *loses precision near the origin*, because
e^{2y} − 1 for small y is a subtraction of nearly equal quantities. The `expm1`-based form

    tanh(y) = expm1(2y) / (expm1(2y) + 2)

removes that cancellation entirely, which is why fdlibm's `tanh` is written that way. The
symmetric form (e^y − e^{−y})/(e^y + e^{−y}) costs two exponentials and still overflows.

**The standard treatment** — fdlibm, Cephes, Boost — is a three-region split: return y (or the
first series term) below a small threshold; use the `expm1` form in the middle; and return
±1 with a token dependence on y beyond the saturation threshold, so the sign and the
rounding mode behave. Argument reduction is not needed; tanh is its own reduction.

**The round trip is a good test and a bad guarantee.** `FISHERINV(FISHER(x))` recovers x
exactly only when the two roundings happen to cancel. Near x = 0 both functions are
approximately the identity and the round trip is faithful; near |x| = 1 the forward map
expands and the backward map contracts, so the round trip is catastrophically unstable there —
a change of one ulp in the transformed value moves the recovered correlation by far more than
one ulp. That instability is a property of the mathematics, not a defect, and it is precisely
why the transformation is useful: it spreads out the region where correlations are hard to
distinguish.

**What a careful implementation does.** Use `expm1`; special-case the origin so the signed zero
survives; pick the saturation threshold from the actual double spacing rather than a
hard-coded constant; and never let an intermediate infinity reach a quotient.

## What has not been checked

No Handbook vector suite exists for `FISHERINV`, and no evidence record in the Handbook's
collection names this surface as a subject. **The Handbook has not compared this function
against Excel.**

The one piece of behavioural evidence anywhere in reach is upstream's, not the Handbook's: the
`BUG-FUNC-027` class-A6 note recording a live-Excel saturation at large positive argument and
the guard OxFunc added because of it. That is a witness, from a build named in upstream's
record, for one qualitative behaviour at one end of the range. It is not a pass rate, it is not
a suite, and it does not license any statement about the interior of the range.

Inputs I would probe first, and why:

1. **A ladder of small `y`** — 2⁻²⁰ down to the subnormal edge — against correctly rounded
   tanh. This is the cancellation probe. The direct form and the `expm1` form separate here
   and only here, so the residual pattern identifies which formula Excel uses. It is the single
   most informative sweep available for this function.
2. **The saturation boundary from below**: the largest `y` for which the result is strictly
   less than 1, and the smallest for which it is exactly 1. This pins the threshold, and the
   threshold is an implementation fingerprint.
3. **Beyond the overflow point** for e^{2y} — the exact region the upstream guard exists for.
   Reproducing that observation in the Handbook's own record would convert an inherited
   witness into a Handbook one.
4. **The negative mirror of both boundaries**, to test whether saturation is symmetric.
   A guard written for one side only can leave the other side taking a different path.
5. **The round trip `FISHERINV(FISHER(x))`** at x near 0, near ±0.9, and near ±0.9999 —
   which measures the composition rather than either function, and shows the conditioning
   blow-up described above.
6. **`FISHERINV` against `TANH`** on the same arguments. If they ever differ, the two surfaces
   are not one computation, and that is a finding worth publishing on both pages.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| tanh | The hyperbolic tangent, the function FISHERINV computes |
| saturation | The point beyond which the nearest double to the true value is exactly ±1 |
| `expm1` form | tanh(y) = expm1(2y)/(expm1(2y)+2), the cancellation-free rearrangement |
| round trip | The composition FISHERINV(FISHER(x)), exact as mathematics and inexact in floating point |
| `BUG-FUNC-027` | Upstream defect stream whose class-A6 note records the large-argument saturation observation |

## Sources

- Microsoft Learn, "WorksheetFunction.FisherInv method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.fisherinv>
  (description, the round-trip statement, and the single documented `#VALUE!` condition). The
  worksheet-surface page at `support.microsoft.com` documents the same function; it was not
  retrievable at curation time, so nothing here is quoted from it.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 4 — the hyperbolic
  tangent, its series and asymptotics.
- fdlibm `s_tanh.c`; Cephes; Boost.Math — the `expm1` rearrangement and the three-region split.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.FISHERINV.json` and
  `data/presence/FUNC.FISHERINV.json`.
- OxFunc `crates/oxfunc_core/src/functions/fisherinv_fn.rs` at commit `473efa3` — the direct
  kernel form, the overflow guard, and the `BUG-FUNC-027` class-A6 attribution quoted in
  substance under **Result and edge cases**.
