---
schema: efh.function-page/v1
function_id: FUNC.ATANH
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0005
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Atanh method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.atanh"
    role: "documented description, the open (-1, 1) domain statement, and the Atanh(Tanh(x)) = x round trip"
  - work: "Microsoft Support — ATANH function"
    locator: "https://support.microsoft.com/en-us/office/atanh-function-3cd65768-0de7-4f1d-b312-d01c8c930d90"
    role: "the canonical worksheet article; not retrieved for this pass (fetch returned HTTP 403)"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 4 (Elementary Transcendental Functions), section 4.6"
    locator: "4.6.22, the arctanh logarithmic form and series"
    role: "the closed form, the series, the poles, and the relation to the other inverse hyperbolic functions"
  - work: "fdlibm, e_atanh.c"
    locator: null
    role: "the published two-branch reference implementation using log1p on 2t/(1-t)"
  - work: "Higham, Accuracy and Stability of Numerical Algorithms"
    locator: "chapter 1, the log1p discussion"
    role: "why the logarithm of a ratio near one is the canonical accuracy failure, and what log1p fixes"
  - work: "Fisher, On the probable error of a coefficient of correlation deduced from a small sample (1921)"
    locator: null
    role: "the variance-stabilising transform that makes ATANH a statistical tool as well as a hyperbolic one"
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
family: atanh
role_in_family: >-
  The inverse hyperbolic tangent on the open interval (-1, 1): the family's pole-bounded member,
  the substrate of FISHER, and the surface carrying a per-surface substrate-identification record
  with open rows in its switch band.
---

## What it computes

`ATANH(number)` is the inverse hyperbolic tangent.

    atanh x  =  (1/2) * ln( (1 + x) / (1 - x) ),     -1 < x < 1

- **Domain**: the *open* interval `(-1, 1)`. `tanh` maps `R` bijectively onto `(-1, 1)`, so the
  inverse is defined exactly there and needs no branch choice. Microsoft's Learn reference states
  the domain as "between -1 and 1 (excluding -1 and 1)".
- **Range**: all real numbers.
- **Parity**: odd, exactly, as mathematics. `atanh(-x) = -atanh(x)`. Whether an *implementation*
  preserves that bit for bit is a separate question, and on this surface it is a live one — see the
  numerical notes.
- **Monotonicity**: strictly increasing on the whole domain.
- **Poles**: logarithmic singularities at both endpoints. `atanh(x) -> +infinity` as `x -> 1-` and
  `-> -infinity` as `x -> -1+`. The growth is slow: even one ulp inside an endpoint the value is
  only around 18.4, so *nothing overflows anywhere in this function*. The poles are real and
  numerically harmless.
- **Derivative**: `d/dx atanh x = 1/(1 - x^2)`, which is at least 1 everywhere and unbounded at the
  endpoints.
- **Series about zero**: `atanh x = x + x^3/3 + x^5/5 + x^7/7 + ...` for `|x| < 1`. For small `x`,
  `atanh x -> x`, so subnormals pass through unchanged.
- **Near the poles**: with `x = 1 - t`, `atanh(1 - t) = (1/2) ln(2/t) + O(t)`.
- **Relations**: `atanh x = acoth(1/x)` (complementary domain — see [ACOTH](FUNC.ACOTH.md));
  `atanh x = asinh( x / sqrt(1 - x^2) )`; and `atanh(tanh t) = t` for every real `t`, which is the
  round trip Microsoft's Learn reference states.
- **Complex continuation**: branch cuts along `(-infinity, -1]` and `[1, +infinity)` — the exact
  complement of the real domain, and the mirror image of `ACOTH`'s cut.

Abramowitz & Stegun give the closed form in chapter 4 section 4.6.

### Why this function matters beyond hyperbolic geometry

`ATANH` is **Fisher's z-transform**. For a sample correlation coefficient `r`, `z = atanh(r)` is
approximately normally distributed with variance `1/(n-3)` — the variance-stabilising transform
Fisher introduced in 1921, and the standard route to a confidence interval for a correlation.
Excel exposes it twice: as `ATANH`, and as `FISHER`, whose entire content is the same formula.
That makes `ATANH` a statistics function wearing a hyperbolic name, and it means its accuracy near
`|x| = 1` has consequences for confidence intervals on strong correlations.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | Any real number strictly between -1 and 1. Required. | — |

Microsoft's Learn reference gives both the description and the constraint: the number must be
between -1 and 1, excluding the endpoints.

One argument; the reference engine records an arity of exactly one and a unary numeric
scalar-or-array lift profile. Ordinary numeric slot under the shared coercion rules — see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

The misunderstood position is the openness of the interval. Readers who arrive with a correlation
coefficient of exactly `1` — which happens whenever a column is regressed against itself — get an
error, and that error is mathematically correct: the transform of a perfect correlation is
infinite.

## Result and edge cases

Returns `Number`.

- **`x = 1` and `x = -1`** are **not** in the domain. These are the poles; there is no value.
- **Just inside the endpoints** the answer is finite and modest. There is no overflow in this
  function at all, which is worth stating because a function with poles usually has one.
- **Zero** returns zero, and the sign of zero should survive an odd implementation.
- **Subnormals and very small arguments** pass through unchanged: `atanh(x) = x` to within rounding.
  This is not automatic — it is *emergent* from the correct small-argument branch and *destroyed*
  by the naive one, which is precisely what makes it a diagnostic. See the numerical notes.
- **Anything with `|x| >= 1`**, including large magnitudes and anything a text conversion is likely
  to produce, is a domain failure.
- **Arrays** lift elementwise, with element-local errors on out-of-domain elements.

Two rows of the battery rendered beside this page are marked **host-scoped** — their outcome is
tied to the machine that produced them rather than being portable. That flag says the last bits of
this surface are not platform-independent in the reference engine, which is consistent with the
substrate identification described below.

The projected `real_result_policy` records `arg_domain_guard=none` and `non_finite=allow`; the
`|x| < 1` test lives in the kernel.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | `|number| >= 1`, including `±1` themselves | Reference engine's kernel; Microsoft's Learn page states the constraint and names no error code |
| `#VALUE!` | The argument does not convert to a number | Shared coercion rules |
| propagated | An error value in the argument is returned unchanged | Shared coercion rules |

## Relationships

- **`TANH`** — the forward function. `ATANH(TANH(t)) = t` for every real `t`, the cleanest round
  trip in the family, and the best oracle-free test on this page.
- **`FISHER`** — the same formula under a statistical name. `FISHER(r)` and `ATANH(r)` should be
  identical; if they are not, one of them is a separate implementation and that is worth knowing.
  `FISHERINV` is `TANH`.
- **`ACOTH`** — the complementary-domain partner, `atanh(x) = acoth(1/x)`. Upstream identification
  work treats the two together, and both carry evidence records here.
- **`ACOSH`** — the third inverse hyperbolic primary. Between `ATANH` on `(-1,1)`, `ACOSH` on
  `[1,infinity)` and `ASINH` on `R`, the family covers the line several times over with different
  shapes.
- **`ATAN`** — the circular namesake, and structurally the inverse arrangement: `ATAN` has an
  unbounded domain and a bounded range, `ATANH` a bounded domain and an unbounded range.
- **`CORREL`, `PEARSON`, `RSQ`** — the producers of the arguments that make this function
  statistically interesting.
- **`LN`** — the substrate of the closed form.
- **Confused with**: `ATAN`, by name; and with `1/TANH`, which is `COTH`.

## Numerical notes

`ATANH`'s hazard is at the *origin*, not at the poles — the opposite of what the graph suggests,
and the same inversion seen on the `ACOTH` page.

**The small-argument failure.** Evaluate `(1/2) ln((1+x)/(1-x))` for small `x`. The ratio is close
to `1`, and the answer — which is close to `x` — lives entirely in the digits that the division has
just rounded away. Concretely: once `|x|` falls below about `2^-53`, the doubles `1+x` and `1-x` are
both exactly `1`, the ratio is exactly `1`, its logarithm is exactly `0`, and the naive form returns
zero for an argument whose `atanh` is the argument itself. The collapse is complete, and it starts
degrading long before it becomes total.

This is why the subnormal pass-through is a *diagnostic*: an implementation that returns the input
unchanged for a subnormal argument is not using the naive ratio there, and one that returns zero
is.

**The remedies**, both standard:

1. **`log1p` on a rearranged argument.** `atanh(x) = (1/2) * log1p( 2x / (1 - x) )`. Nothing near
   `1` is formed and nothing is subtracted from `1` except `x`, which is exact. `fdlibm`'s
   `e_atanh.c` uses this form.
2. **The `log1p` difference.** `atanh(x) = (1/2) * ( log1p(x) - log1p(-x) )`. Symmetric, and it
   makes the oddness manifest — the two terms swap under `x -> -x`, so the result negates exactly.
3. **The series** for very small `|x|`: `x + x^3/3`, truncated as soon as the cubic term falls below
   the rounding of `x`. Below a threshold, `x` itself is correctly rounded.

Higham's discussion of `log(1+x)` is the standard statement of why the `log1p` primitive exists.
Excel does not expose `log1p` as a worksheet function, which is why a worksheet-level workaround is
awkward and why an implementer of this surface must supply it internally.

**Near the poles the naive form is fine.** `(1+x)/(1-x)` is large there, `ln` of a large number is
well conditioned, and `1 - x` is exact for doubles near `1` by Sterbenz's lemma. So the correct
branch structure is: **naive ratio-log in the outer band, `log1p` form near zero** — the reverse of
the intuitive ordering, exactly as for `ACOTH`.

**What is on record upstream.** `EV-MATH-0005` is a **substrate-identification** record for this
surface. The identification it carries is a piecewise shape of that kind: a direct ratio-logarithm
over the main band including the near-pole rows, and a small-argument logarithm-of-one-plus pair
below a threshold, with the threshold placed inside a gap between the two regions' clean bands and
with residual rows still open in the switch band itself. The record publishes a per-surface count
and names the open rows; **its figures are rendered mechanically beside this page and are not
restated in this prose**, and its own status text is the authority on what the count does and does
not establish. The Handbook publishes it as an upstream identification — a hypothesis with
supporting evidence and open residuals — not as a settled fact about Excel.

**The oddness question, which is genuinely open.** The upstream production note for this surface
states that the *signed* ratio is evaluated directly rather than by computing on `|x|` and
restoring the sign, with the consequence that the negative argument rounds independently and exact
oddness is not guaranteed by construction. A retained test in the same module asserts exact odd
symmetry at a specific near-pole witness. Those two statements are not formally contradictory — a
non-odd-by-construction implementation can still be odd at particular points — but together they
leave the general question unresolved, and the Handbook records it as unresolved rather than
picking a side. It matters: `FISHER`'s statistical use assumes `atanh(-r) = -atanh(r)`, and a
one-ulp asymmetry there is a real, if small, defect.

**Why switch placement dominates.** As with `ACOTH`, two forms that are each individually excellent
will disagree in the last bits over the band between their switch points. An implementation that
places its threshold differently from Excel's will differ from Excel on every argument in the gap.
That is why identification work here concentrates on locating switches rather than on improving
accuracy, and it is why the residual rows recorded upstream cluster in exactly that band.

## What has not been checked

The evidence attached to this page is **`EV-MATH-0005`**, a **substrate-identification** record
whose subject is `FUNC.ATANH`. It carries a per-surface count, an identified piecewise substrate,
and open residual rows in the switch band; it also carries its own careful notes about which
sub-figures are independent measurements and which are not, and about the corpus being the
identification's own rather than a held-out one. **The figures belong to the record and are
rendered with it; nothing here restates them, and no general agreement claim follows.**

**No Handbook vector suite exists for `ATANH`.** The Handbook has not itself observed this function
in Excel; what exists is an upstream identification with an attached record.

Two specific things are *not* established: the exact switch double (the upstream note says it is
placed in a gap rather than pinned), and whether the implementation is exactly odd.

The documented statements above come from Microsoft's Learn `WorksheetFunction.Atanh` reference,
which was retrieved. Microsoft's worksheet article was not (HTTP 403).

Probes worth running first:

1. **`ATANH(1)`, `ATANH(-1)`, `ATANH(0)`** — the excluded endpoints and the centre, which pin the
   open interval and the undocumented error code.
2. **A subnormal argument.** One probe. If it comes back unchanged, the small-argument branch
   exists; if it comes back zero, the naive ratio is in use all the way down. Nothing else on this
   page is settled so cheaply.
3. **A sweep from `1e-1` down to the subnormal floor** against a high-precision reference — the
   full cancellation profile, and the way to locate the switch threshold from outside.
4. **A dense sweep across the switch band** named in the upstream identification, which is where
   the open residual rows sit and where the upstream note itself says dense probing is required.
5. **`ATANH(x) + ATANH(-x)`** across the whole domain, which resolves the oddness question directly
   and is the probe the Handbook most wants run.
6. **One ulp inside each pole** — the largest magnitudes reachable, and the region where the naive
   form is at its best.
7. **`ATANH(TANH(t))` across `t`** — the exact round trip, no oracle needed.
8. **`ATANH(r)` against `FISHER(r)`** on the same arguments — an internal cross-check that would
   reveal whether the two surfaces share an implementation.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| Fisher's z-transform | `z = atanh(r)`, the variance-stabilising transform for a correlation coefficient |
| ratio-log form | `(1/2) ln((1+x)/(1-x))`, accurate near the poles and catastrophic near zero |
| `log1p` form | `(1/2) log1p(2x/(1-x))` or the `log1p` difference; accurate near zero |
| switch band | The interval between the clean regions of two forms, where implementations disagree |
| subnormal pass-through | `atanh(x) = x` for very small `x`; emergent from the good branch, absent from the naive one |
| host-scoped row | A battery row whose outcome is tied to the machine that produced it |

## Sources

- Microsoft Learn, "WorksheetFunction.Atanh method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.atanh> (retrieved: the
  description, the open `(-1, 1)` domain, and the round trip; **no error code is named there**).
- Microsoft, "ATANH function" —
  <https://support.microsoft.com/en-us/office/atanh-function-3cd65768-0de7-4f1d-b312-d01c8c930d90>
  — the canonical worksheet article. **Not retrieved for this pass** (HTTP 403).
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 4 section 4.6 — the closed
  form, series and poles.
- `fdlibm` `e_atanh.c` — the published `log1p`-based reference implementation.
- Higham, *Accuracy and Stability of Numerical Algorithms*, chapter 1 — the `log(1+x)` problem.
- Fisher (1921) — the z-transform, and the reason this surface is also a statistics function.
- Handbook evidence record `EV-MATH-0005` (subject `FUNC.ATANH`) — a substrate-identification
  record with a per-surface count, an identified piecewise form, and open rows in the switch band.
- Handbook projections `data/functions/FUNC.ATANH.json` and `data/presence/FUNC.ATANH.json`
  (implementing module; discrepancy and math-deviation catalogue entries; the `BUG-FUNC-027`
  scalar-invocation sweep).
- OxFunc `crates/oxfunc_core/src/functions/atanh.rs` at commit `473efa3` — the two-branch kernel,
  the note that the signed ratio is evaluated directly, and the retained odd-symmetry test.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md); sibling page
  [ACOTH](FUNC.ACOTH.md).
