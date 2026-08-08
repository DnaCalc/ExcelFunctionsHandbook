---
schema: efh.function-page/v1
function_id: FUNC.STANDARDIZE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — STANDARDIZE function"
    locator: "https://support.microsoft.com/en-us/office/standardize-function-81d66554-2d54-40ec-ba83-6437108ee775"
    role: "documented signature, the stated equation, and the documented #NUM! condition on standard_dev"
  - work: "Abramowitz, M. and Stegun, I. A., Handbook of Mathematical Functions"
    locator: "chapter 26, section 26.2 (the normal distribution and its standardised form)"
    role: "the standardising substitution that reduces a general normal to the standard normal"
  - work: "OxFunc — standardize_fn.rs"
    locator: "crates/oxfunc_core/src/functions/standardize_fn.rs"
    role: "reference-engine kernel: the stdev <= 0 guard and the elementwise lift"
episodes: []
body_sections:
  - "What it computes"
  - "Arguments"
  - "Result and edge cases"
  - "Errors"
  - "Relationships"
  - "Numerical notes"
  - "What has not been checked"
  - "Page vocabulary"
  - "Sources"
family: standardize_fn
role_in_family: >-
  The affine standardising map: shifts by the mean and scales by the standard deviation, turning
  a value on any scale into a z-score.
---

# STANDARDIZE

## What it computes

`STANDARDIZE(x, mean, standard_dev)` returns the **z-score** of `x` relative to a distribution
with the stated centre and scale:

    STANDARDIZE(x, mu, sigma)  =  ( x - mu ) / sigma        for sigma > 0

It is the affine map `T(x) = (x - mu)/sigma`, and everything about it follows from that being an
affine map with positive slope:

- It is a **bijection of the real line onto itself**, strictly increasing, with inverse
  `x = mu + sigma*z`. Nothing is lost and nothing is folded.
- It is the **unique** affine map sending `mu` to `0` and `mu + sigma` to `1`. That is what fixes
  the convention: `STANDARDIZE(mu, mu, sigma) = 0` and a value one standard deviation above the
  mean maps to exactly `1`.
- Applied to a whole data set with its own mean and standard deviation, the image has mean `0`
  and standard deviation `1`. Applied to a random variable `X` with `E[X] = mu` and
  `SD(X) = sigma`, the image `Z` has `E[Z] = 0` and `SD(Z) = 1` — for *any* distribution, not
  only the normal one. Standardisation is a change of units, not a normality assumption.

Its role in the statistical category is as the **argument reduction** for the normal family. The
substitution `z = (x - mu)/sigma` is what reduces a general normal density and distribution
function to the standard ones — it is the substitution behind Abramowitz & Stegun's treatment in
chapter 26, where every general-normal quantity is tabulated through its standard form. In Excel
terms the mathematical identity is

    NORM.DIST(x, mu, sigma, TRUE)  =  NORM.S.DIST( STANDARDIZE(x, mu, sigma), TRUE )

and the corresponding one for the density carries an extra `1/sigma` from the Jacobian:

    NORM.DIST(x, mu, sigma, FALSE)  =  NORM.S.DIST( STANDARDIZE(x, mu, sigma), FALSE ) / sigma

Both are exact in real arithmetic. Neither is a good way to compute the left-hand side in
floating point — see Numerical notes.

The name is slightly misleading in one respect worth stating: `STANDARDIZE` does not compute
`mean` or `standard_dev` from data. It is a three-argument scalar transform, and the caller
supplies the parameters. Standardising a column against its own statistics means writing
`STANDARDIZE(A1, AVERAGE($A$1:$A$100), STDEV.S($A$1:$A$100))`, which is where the population
versus sample choice actually enters.

## Arguments

| Argument | Meaning | Admissible |
|---|---|---|
| `x` | The value to standardise. Required. | any finite number |
| `mean` | The centre of the distribution. Required. | any finite number |
| `standard_dev` | The scale. Required. | strictly positive |

All three are ordinary numeric slots subject to
[coercion and lifting](../model/02-coercion-and-lifting.md). Unlike most of the statistical
category, `STANDARDIZE` is a **scalar kernel**: the reference engine runs it through the
elementwise lifted path, so an array in any of the three positions produces an array of results,
with element failures staying element-local. That makes
`STANDARDIZE(A1:A100, AVERAGE(A1:A100), STDEV.S(A1:A100))` a single spilling formula, which is
the idiomatic use.

**`standard_dev` must be strictly positive.** Zero is rejected, not treated as a limit. That is
the documented rule and the reference engine implements exactly it, with a `<= 0` test.

## Result and edge cases

Returns `Number`, dimensionless — the units of `x` and `mu` cancel against those of `sigma`.

- **`x = mean`.** Exactly `0`, and exactly `+0` for `mu` finite, since `x - mu` is exactly zero
  and `0/sigma` is `+0` for positive `sigma`.
- **`standard_dev = 0`.** `#NUM!`, not `#DIV/0!` and not an infinity. This is worth a second look:
  arithmetically the natural failure of a division by zero is `#DIV/0!`, and `STANDARDIZE`
  deliberately reports a *domain* failure instead. The choice says that a distribution with zero
  spread is not a distribution whose z-scores are meaningful, rather than that a division went
  wrong. The reference engine's guard is `stdev <= 0`, so the same `#NUM!` covers negative
  scales.
- **`standard_dev` negative.** `#NUM!`. A negative scale would give a well-defined
  order-reversing affine map, and it is refused rather than accepted — the function insists on
  the orientation convention.
- **Very large `x - mean` relative to `standard_dev`.** The result overflows to an infinity if the
  quotient exceeds the representable range. The reference engine's declared real-result policy
  permits non-finite results rather than converting them to `#NUM!`, so a genuinely enormous
  z-score is returned as an infinity rather than diagnosed. Whether Excel does the same is not
  something this page claims.
- **Very small `standard_dev`.** Subnormal scales are accepted by the `<= 0` test and produce
  correspondingly enormous or infinite results.
- **Errors in any argument** propagate as themselves; under the lifted path an error in one
  element of an array argument stays in that element.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | `standard_dev <= 0` | documented and implemented |
| `#VALUE!` | Any argument is text that does not parse as a number | shared coercion rule |
| propagated | An error value in any argument surfaces as that error | shared coercion rule |

There is no `#DIV/0!` reachable from this function: the only division is by a value the guard has
already established to be strictly positive.

## Relationships

- **[NORM.S.DIST](FUNC.NORM.S.DIST.md) and [NORM.DIST](FUNC.NORM.DIST.md)** are the functions
  `STANDARDIZE` feeds. The composition identity above is exact mathematically, and
  [NORM.DIST](FUNC.NORM.DIST.md) exists as a separate function precisely so that callers do not
  have to route through it.
- **[NORM.S.INV](FUNC.NORM.S.INV.md) and [NORM.INV](FUNC.NORM.INV.md)** run the inverse map:
  `NORM.INV(p, mu, sigma) = mu + sigma * NORM.S.INV(p)`, which is `STANDARDIZE` read backwards.
- **[AVERAGE](FUNC.AVERAGE.md), [STDEV.S](FUNC.STDEV.S.md) and [STDEV.P](FUNC.STDEV.P.md)** are
  the usual sources of the two parameters. Choosing between the sample and population standard
  deviation is a statistical decision that `STANDARDIZE` does not make and cannot see.
- **[Z.TEST](FUNC.Z.TEST.md)** is the hypothesis test built on the same standardisation, with the
  standard error `sigma/sqrt(n)` in place of `sigma`.
- **[PERCENTRANK.INC](FUNC.PERCENTRANK.INC.md)** is the non-parametric alternative for "where
  does this value sit": it uses the empirical distribution instead of a location-scale model, and
  needs no parameters.
- Readers confuse `STANDARDIZE` with normalisation to `[0, 1]`, which is `(x - min)/(max - min)`
  — a different affine map with different invariances. `STANDARDIZE` centres on the mean and
  scales by spread; min-max scaling pins the extremes. Neither is "the" normalisation.

## Numerical notes

The function is one subtraction and one division, both correctly rounded, so the *implementation*
contributes at most one ulp of relative error to each. All of the interesting behaviour is in the
conditioning of the map itself.

**Catastrophic cancellation at the centre.** `x - mu` is a subtraction of two nearby quantities
whenever `x` is close to `mu`, and that is exactly the region where users spend most of their
time. If `x` and `mu` are each around `10^6` and differ by around `10^{-6}`, the subtraction is
exact in binary64 (Sterbenz's lemma covers the case where the two operands are within a factor of
two of each other), but the *inputs* generally are not: `mu` came from an
[AVERAGE](FUNC.AVERAGE.md) that carried its own rounding, and that error is transmitted to the
result multiplied by `1/sigma`. The relative error of the z-score is unbounded near `x = mu` even
though its absolute error is fine. This matters for anything downstream that cares about the
relative accuracy of a small z-score, and it does not matter at all for a tail probability, where
the absolute accuracy is what governs.

The general statement: the condition number of `z` with respect to `mu` is `|mu|/|x - mu|`, which
blows up at the centre; the condition number with respect to `sigma` is `1`, uniformly. Errors in
the mean are dangerous; errors in the standard deviation are not.

**Why the composition identity is not a computational recipe.** `NORM.DIST` is mathematically
`NORM.S.DIST(STANDARDIZE(...))`, but computing it that way costs accuracy twice. First, the
standardisation rounds `z`, and then the distribution function amplifies that rounding by its own
derivative — which in the far tail is the density, so the *relative* error of the tail probability
grows like `|z|` times the relative error in `z`. Second, the composition throws away any
opportunity for the distribution routine to use a better-conditioned form of its argument. This
is the standard reason that libraries expose a general-parameter entry point rather than telling
callers to standardise first, and it is the reason [NORM.DIST](FUNC.NORM.DIST.md) is a separate
function rather than a wrapper. Do not use `STANDARDIZE` as a preprocessing step for a tail
probability if accuracy in the tail matters.

**The guard is exact, not tolerant.** `sigma <= 0` is an exact comparison. A subnormal `sigma`
passes it and produces an enormous quotient, possibly an infinity. There is no tolerance and no
scaling, which is a defensible choice — any threshold would be arbitrary — but it means the
function has no "nearly degenerate" diagnostic, only a degenerate one.

**Overflow and underflow.** `(x - mu)/sigma` can overflow when the numerator is large and the
scale is subnormal, and can underflow to zero when the reverse holds. Neither is diagnosed. A
scale-aware implementation would compute the quotient by rescaling both operands, as `hypot`
does; this one does not need to for ordinary data and would need to for adversarial data.

**Lifting.** Because `STANDARDIZE` is a genuine scalar kernel routed through the elementwise
lifted path, the accuracy analysis above holds per element and there is no accumulation across an
array — unlike every aggregate in this category. Whether the lifting itself matches Excel's is a
separate question, and the reference engine's implementing module is named in an upstream defect
stream on text, date, array-lift and coercion gaps, which is a reason to treat the lifting and
coercion edges as unsettled rather than assumed.

## What has not been checked

No Handbook vector suite exists for `STANDARDIZE`, and **no Handbook evidence record names
`STANDARDIZE` as a subject**. Nobody has checked this function against Excel within the
Handbook's record.

One qualification, stated because the opposite would be easy to imply: `STANDARDIZE` is listed
among the candidate surfaces of a large upstream structural sweep whose results are recorded in
the Handbook's evidence layer as a **group** figure. That group was measured; this surface was
not measured separately. The record carries a reader warning forbidding per-surface attribution
from a group count, and the Handbook honours it — no figure from that sweep attaches to this
page and none is quoted here. "A structural comparison exists for the group" and "`STANDARDIZE`
has been checked" are different statements, and only the first is true.

The reference engine's implementing module is separately named in an upstream defect stream on
unswept conversion, text/date handling, array lifting and coercion. That is an upstream register
of open work, not a Handbook measurement.

Everything above marked as documented comes from Microsoft's `STANDARDIZE` page. **Retrieval of
that page was blocked by the upstream host on this pass**, so those statements are recorded as
documented behaviour with the source named and should be re-read against the live page.

Inputs worth probing first:

1. **`standard_dev = 0`**, against `#NUM!` (documented and implemented) and `#DIV/0!` (the
   arithmetically natural alternative). One cell, and it is the only documented error condition
   the function has.
2. **`standard_dev` negative**, confirming that the `<= 0` reading rather than a `= 0` reading is
   what Excel implements.
3. **A subnormal `standard_dev` with an ordinary `x - mean`**, checking whether an infinity is
   returned or `#NUM!` — the boundary between the declared non-finite-allowed policy and a
   domain guard.
4. **Cancellation**: `STANDARDIZE(1000000.0000001, 1000000, 0.0000001)`, whose exact answer is
   `1`, against the same relationship stated near zero. Any difference measures the transmitted
   error rather than the function.
5. **The composition identity**: `NORM.DIST(x, mu, sigma, TRUE)` against
   `NORM.S.DIST(STANDARDIZE(x, mu, sigma), TRUE)` in the far tail, bitwise. The identity is exact
   mathematically; the gap is the accuracy cost of standardising first, and it is the probe that
   demonstrates why the two functions are separate.
6. **Array arguments in each of the three positions**, singly and in combination, given the open
   lifting and coercion stream on this module. Mixed shapes are the interesting case.
7. **Numeric-looking text and a logical in each position**, against the coercion rules.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| z-score | The standardised value `(x - mu)/sigma` |
| affine map | `x -> a*x + b`; `STANDARDIZE` is the one sending `mu` to 0 and `mu+sigma` to 1 |
| argument reduction | Rewriting a general-parameter problem in standard form before evaluating |
| condition number | How much a relative error in an input is amplified in the output |
| scalar kernel | A function lifted elementwise over array arguments, per [coercion and lifting](../model/02-coercion-and-lifting.md) |
| non-finite allowed | The declared policy under which an overflowing result is returned rather than diagnosed |

## Sources

- Microsoft, *STANDARDIZE function* —
  <https://support.microsoft.com/en-us/office/standardize-function-81d66554-2d54-40ec-ba83-6437108ee775>
  (signature, the stated equation, and the `#NUM!` condition when `standard_dev` is not positive).
  Retrieval was blocked by the upstream host for this page; the documented behaviour above is
  stated as documented behaviour and should be re-checked against the page.
- M. Abramowitz and I. A. Stegun, *Handbook of Mathematical Functions*, chapter 26 section 26.2 —
  the standardising substitution that reduces a general normal to the standard normal, and the
  Jacobian factor in the density identity.
- N. J. Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd ed., chapter 2 (Sterbenz's
  lemma and the conditioning of subtraction) — the basis of the cancellation discussion.
- OxFunc `crates/oxfunc_core/src/functions/standardize_fn.rs` at commit `473efa3` — the
  reference-engine kernel: the `stdev <= 0` domain guard returning `#NUM!`, and the routing
  through the values-only elementwise lifted path.
- OxFunc defect stream
  `docs/bugs/streams/BUG-FUNC-028_unswept_conversion_text_date_array_lift_and_coercion_gap.md`
  — the upstream register naming this module's lifting and coercion edges as open.
- Handbook [coercion and lifting](../model/02-coercion-and-lifting.md) — the elementwise lift and
  the element-local error rule.
- Handbook `data/functions/FUNC.STANDARDIZE.json` (the declared real-result policy allowing
  non-finite results), `data/presence/FUNC.STANDARDIZE.json`.
