---
schema: efh.function-page/v1
function_id: FUNC.T.INV.2T
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
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
family: chi_f_t_family
role_in_family: >-
  The two-tail Student-t critical value: the surface the textbooks mean by "the t table", the
  modern spelling of the legacy TINV, and the only member of the t quartet whose documentation
  admits that it is computed by iterating on its own sibling.
---

# T.INV.2T

## What it computes

`T.INV.2T(probability, deg_freedom)` returns the **two-tailed critical value** of Student's
*t*-distribution with ν degrees of freedom. Microsoft states the definition directly: it returns
"that value `t`, such that `P(|X| > t) = probability` where `X` is a random variable that
follows the t-distribution".

So it is the exact inverse of [T.DIST.2T](FUNC.T.DIST.2T.md), and it is related to the left-tail
quantile of [T.INV](FUNC.T.INV.md) by

    T.INV.2T(p, ν) = T.INV(1 − p/2, ν) = − T.INV(p/2, ν)

This is the number printed in every statistics textbook's t table. Asking for the 5% two-sided
critical value at 10 degrees of freedom is `T.INV.2T(0.05, 10)`.

Because `P(|T| > t)` is continuous and strictly decreasing from 1 at `t = 0` to 0 as `t → ∞`,
the inverse exists and is unique on `p ∈ (0, 1]`, with

    T.INV.2T(1, ν) = 0        exactly, for every ν
    T.INV.2T(p, ν) → +∞       as p → 0⁺

The range is `[0, ∞)`: this surface never returns a negative number. That is worth stating
plainly, because it is the fastest way to tell it apart from `T.INV`, which returns negative
values for `p < ½`.

Closed forms at the two small integer degrees of freedom, obtained by inverting the elementary
two-tail formulas:

| ν | `T.INV.2T(p, ν)` |
|---|---|
| 1 | `cot(πp/2)` = `tan(π(1 − p)/2)` |
| 2 | `sqrt( 2(1 − p²) / p² )` = `sqrt(2)·sqrt(1 − p²)/p` |
| ν → ∞ | `NORM.S.INV(1 − p/2)` |

Both are exact and elementary — a full parameter slice each on which any implementation can be
checked against arithmetic instead of against another approximation.

Deep-tail growth: as `p → 0⁺` the critical value grows like `p^{−1/ν}`, so it explodes as a power
law, violently for small ν (`ν = 1` gives growth proportional to `1/p`) and gently for large ν.
An implementation whose starting bracket is derived from the normal quantile — which grows only
like `sqrt(−2 ln p)` — will start catastrophically low in the small-ν tail.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `probability` | The two-tail probability `P(\|T\| > t)`. | Yes |
| `deg_freedom` | ν, the number of degrees of freedom. Non-integer values are documented as truncated. | Yes |

Exactly two arguments; the projection records an arity of exactly two.

Documented admissible range: `#NUM!` when `probability ≤ 0` or `probability > 1`, so
`probability = 1` is admitted — and here, unlike on `T.INV`, that is mathematically fine: the
answer is exactly 0. The two inverse surfaces have opposite behaviour at their shared documented
boundary, which is a small but real reason to keep them straight.

Microsoft documents that a non-integer `deg_freedom` is truncated, and that one-tailed values can
be recovered by passing `2 * probability`.

Numeric slots take ordinary to-number coercion; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`, a non-negative critical value.

- **`probability = 1`** is exactly 0. Free and exact.
- **`probability` near 0** is where the critical value is large and where the problem is badly
  conditioned: `dt/dp = −1/(2 f(t; ν))`, and the density in the tail is tiny, so a one-ulp change
  in `p` moves `t` a long way. No algorithm can recover accuracy the input does not contain.
  This is why a tail critical value is a fundamentally harder request than a central one.
- **`probability` near 1** is the opposite corner: `t` is near 0 and the answer has plenty of
  absolute room, but computing it from `T.INV(1 − p/2, ν)` would form `1 − p/2` for `p` near 1
  and lose relative precision in the small quantity. A direct two-tail search does not have that
  problem, which is an argument for this surface existing rather than being a wrapper.
- **Round trip.** `T.DIST.2T(T.INV.2T(p, ν), ν)` should return `p`. Microsoft's own remark makes
  this the defining relationship rather than a nice property: the value is *found* by seeking it.
- **Arrays.** Lift axis projected as `surface_native` with `default-unexamined` provenance; the
  reference-engine battery beside this page refuses an inline array argument. Unsettled.

## Errors

As documented on Microsoft's `T.INV.2T` page:

| Error | Documented condition |
|---|---|
| `#VALUE!` | Either argument is nonnumeric. |
| `#NUM!` | `probability ≤ 0` or `probability > 1`. |
| `#NUM!` | `deg_freedom < 1`. |

Errors in either argument propagate under the universal coercion rule. The Handbook has not
verified any of this against Excel.

## Relationships

- **[T.DIST.2T](FUNC.T.DIST.2T.md)** is the forward function this inverts, and Microsoft's page
  names it as such.
- **[T.INV](FUNC.T.INV.md)** is the *left-tail* quantile, a different function. The conversion is
  `T.INV.2T(p, ν) = T.INV(1 − p/2, ν)`. Using `T.INV(p, ν)` where `T.INV.2T(p, ν)` was meant
  gives a number of the wrong sign and the wrong magnitude, and it will not look obviously wrong
  in a spreadsheet.
- **`TINV` is this surface's legacy spelling** — not `T.INV`'s. `TINV(p, ν)` and
  `T.INV.2T(p, ν)` are documented to compute the same quantity. **Whether they compute it the
  same way, and publish the same bits, is not established.** The Handbook's alias-pairing record
  for the legacy/modern collapse covers five *forward* CDF pairs and states in terms that no
  inverse pair — `GAMMAINV`/`GAMMA.INV`, `CHIINV`/`CHISQ.INV.RT`, `BETAINV`/`BETA.INV`,
  `FINV`/`F.INV.RT`, and `TINV`/`T.INV.2T` — has ever been probed for identity. Treating the
  legacy and modern inverse spellings as interchangeable is an assumption, and this Handbook
  names it as one.
- **[T.DIST.RT](FUNC.T.DIST.RT.md)**: there is no `T.INV.RT`. The right-tail critical value is
  spelled `T.INV(1 − p, ν)` or `T.INV.2T(2p, ν)`; the second form is preferable, because it does
  not form `1 − p`.
- **`F.INV.RT(p, 1, ν) = T.INV.2T(p, ν)²`**, from `T² ~ F(1, ν)` — a cross-family identity over
  two surfaces the reference engine implements in one module.
- **`CONFIDENCE.T`** is the packaged consumer: a t-based confidence half-width is
  `T.INV.2T(alpha, n−1) · s / sqrt(n)`.
- **`T.TEST`** goes the other way — it returns a `p`-value, not a critical value.

## Numerical notes

Microsoft is unusually forthcoming here, and what it says shapes the whole section: the page
states that `T.INV.2T` uses an iterative seek for the `x` at which the forward two-tail function
equals `probability`, and that **the precision of `T.INV.2T` depends on the precision of
`T.DIST.2T`**. That is a documented statement about an algorithm, and it is rare enough in
Excel's documentation to be worth taking seriously: it says the inverse is not independently
accurate, and that any error in the forward surface is inherited.

Three consequences follow.

**Error inheritance is not neutral — it is amplified by the conditioning.** If the forward
surface has relative error ε at `t`, the induced error in the recovered `t` is roughly
`ε · p / f(t; ν)`. In the tail, where `f` is tiny, that factor is large. An inverse built by
iterating on a forward function that is good to a few ulp can still be poor in absolute terms far
out, and no amount of iteration fixes it.

**The stopping rule has to be stated in the right variable.** Iterating to a fixed tolerance on
`t` over-solves in the centre and under-solves in the tail; iterating to a fixed tolerance on `p`
does the reverse. A defensible implementation iterates on `p` with a *relative* tolerance and
polishes with one Newton step whose derivative is the density — which is available in closed
form from `T.DIST(·, ν, FALSE)`.

**There is a better route than iteration.** The two-tail critical value follows analytically from
the inverse regularized incomplete beta: with `y = I⁻¹_p(ν/2, ½)`, `t = sqrt(ν(1 − y)/y)`. Cephes'
`incbi`, Boost.Math's `ibeta_inv`, and the DCDFLIB/TOMS lineage all provide the inverse beta with
stated error bounds, so a `natural-best` implementation should take this route and reserve
iteration for the polish step. Hill's classic t-quantile algorithm and the expansion in A&S
26.7.5 give the starting approximation when iteration is used; both are stated in terms of the
normal deviate and both need a tail correction for small ν, for exactly the power-law reason
given above.

Nothing here is a claim about what Excel actually does beyond what its own page says.

## What has not been checked

No Handbook vector suite exists for `T.INV.2T`, and no Handbook evidence record lists this
surface among its subjects. **Nobody has checked `T.INV.2T` against Excel within the Handbook's
record.**

Two nearby records exist and neither reaches this page. A per-surface histogram record carries a
row for the legacy `TINV` — that row belongs to `TINV`, and the alias-pairing record explicitly
forbids inheriting an inverse figure across a legacy/modern pair, so it does **not** travel here.
The alias-pairing record itself covers forward CDFs only. The reference-engine battery beside
this page is the engine answering its own questions, with no Excel involved.

One documentation discrepancy this page flags rather than resolves: in the retrieval used for
this page, the remark describing the iterative seek names a **three-argument** call to
`T.DIST.2T` — a function whose documented arity is two. That is either a stale remark carried
over from the legacy `TDIST(x, deg_freedom, tails)` signature or an artifact of retrieval. It
should be re-read against the live page before being cited as a defect, and it is recorded here
so that it is re-read rather than forgotten.

Inputs worth probing first:

1. **`T.INV.2T(1, ν)`** across ν — must be exactly 0, and it is the counterpart to `T.INV`'s
   much more interesting `probability = 1` case.
2. **`T.INV.2T(p, 1)` against `TAN(PI()*(1−p)/2)`**, and **`T.INV.2T(p, 2)` against
   `SQRT(2)*SQRT(1−p^2)/p`** — the two elementary slices, comparing Excel against arithmetic.
3. **`T.INV.2T(p, ν)` against `T.INV(1 − p/2, ν)`** — an exact identity between two Excel
   surfaces. No oracle needed; a mismatch proves the two have separate internals, and the size of
   the mismatch is a direct measurement of the `1 − p/2` cancellation the two-tail surface is
   supposed to avoid.
4. **`TINV(p, ν)` against `T.INV.2T(p, ν)`** — the legacy/modern inverse pairing that the
   Handbook's alias record says has never been probed for anyone. This is the probe that would
   turn an assumption into a finding, and it is cheap.
5. **`T.DIST.2T(T.INV.2T(p, ν), ν)` against `p`** — the round trip that the documentation's own
   description makes definitional, at `p` = 0.9, 0.05, 1e-8, 1e-100.
6. **`F.INV.RT(p, 1, ν)` against `T.INV.2T(p, ν)^2`** — the cross-family identity.
7. **Small ν deep tail**: `T.INV.2T(1e-300, 1)` and neighbours, where a normal-based starting
   bracket fails hardest and where an iteration-count limit would show as a wrong answer rather
   than as an error.
8. **Non-integer `deg_freedom`** — `T.INV.2T(0.05, 10.9)` against `T.INV.2T(0.05, 10)`, which
   confirms or refutes the documented truncation in one call.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| two-tail critical value | The `t ≥ 0` with `P(\|T\| > t) = p` — what this surface returns |
| iterative seek | The search on the forward function that Microsoft's page describes |
| error inheritance | The documented dependence of this surface's precision on `T.DIST.2T`'s |
| conditioning | `dt/dp = −1/(2 f(t; ν))`; large in the tail, so tail critical values are hard |
| inverse pair | A legacy/modern inverse spelling; no such pair has been probed for identity |

## Sources

- Microsoft, "T.INV.2T function" —
  <https://support.microsoft.com/en-us/office/t-inv-2t-function-ce72ea19-ec6c-4be7-bed2-b9baf2264f17>
  (syntax; the definition `P(|X| > t) = probability`; `#VALUE!` for nonnumeric arguments; `#NUM!`
  for `probability ≤ 0` or `> 1`; `#NUM!` for `deg_freedom < 1`; truncation of a non-integer
  `deg_freedom`; the one-tailed conversion by doubling the probability; and the statement that
  the precision of `T.INV.2T` depends on the precision of `T.DIST.2T`, found by an iterative
  seek). Retrieved for this page; see the arity discrepancy noted above.
- Handbook evidence record `EV-DIST-0011` — cited for its explicit statement that no legacy/
  modern *inverse* pair, `TINV`/`T.INV.2T` included, has ever been probed for identity, and that
  no inverse figure may be inherited across such a pair.
- M. Abramowitz and I. A. Stegun, *Handbook of Mathematical Functions*, section 26.7, including
  26.7.5 (t percentage points in terms of the normal deviate) and the percentage-point tables
  this surface replaces.
- Cephes `incbi` / `stdtri`; Boost.Math `ibeta_inv` and the `students_t_distribution` quantile;
  DCDFLIB and the TOMS incomplete-beta-inverse lineage — the analytic route to a t critical
  value.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.T.INV.2T.json`, `data/presence/FUNC.T.INV.2T.json`.
