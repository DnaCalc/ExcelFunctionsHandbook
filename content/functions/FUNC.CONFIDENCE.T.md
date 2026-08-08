---
schema: efh.function-page/v1
function_id: FUNC.CONFIDENCE.T
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0012
  - EV-DIST-0017
open_problems: []
references:
  - work: "Microsoft 365 support: CONFIDENCE.T function"
    locator: "https://support.microsoft.com/en-us/office/confidence-t-function-e8eca395-6c3a-4ba9-9003-79ccc61d3c53"
    role: "retrieved for this pass; syntax, argument descriptions, the size-truncation rule, and the documented error conditions including the size = 1 case"
  - work: "Microsoft 365 support: CONFIDENCE.NORM function"
    locator: "https://support.microsoft.com/en-us/office/confidence-norm-function-7cec58a6-85bb-488d-91c3-63828d4fbfd4"
    role: "retrieved for this pass; the comparison partner, and the source of the argument wording that the CONFIDENCE.T page appears to have inherited"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 26 (Probability Functions), 26.7 (Student's t-distribution)"
    role: "the t-distribution, its relation to the incomplete beta function, and its percentage points"
  - work: "Welinder, notes on Excel's statistical functions (Gnumeric)"
    locator: "Gnumeric documentation, statistical accuracy appendix"
    role: "the external tradition of auditing Excel's statistical lane; named as tradition, not as evidence about this surface"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - A documentation defect on the Microsoft page
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: confidence_test_family
role_in_family: >-
  The interval half-width of the pair; it consumes the two-tailed Student's t quantile at n-1
  degrees of freedom, which makes it the only member whose accuracy is inherited wholesale from
  the chi/F/t substrate rather than from the normal one.
---

# CONFIDENCE.T

## What it computes

`CONFIDENCE.T(alpha, standard_dev, size)` returns the **half-width** of a two-sided confidence
interval for a population mean when the standard deviation was *estimated from the sample* rather
than known in advance.

Writing `t_{q,ν}` for the `q`-quantile of Student's t-distribution with `ν` degrees of freedom,

    CONFIDENCE.T(α, s, n)  =  t_{1 − α/2, n − 1} · s / √n

and the interval is `x̄ ± CONFIDENCE.T(α, s, n)`.

The degrees-of-freedom count is `n − 1`, not `n`: one degree of freedom was spent estimating the
mean before the spread was measured. That single subtraction is the whole difference between this
function and [CONFIDENCE.NORM](FUNC.CONFIDENCE.NORM.md) in structure, and it is the reason the
smallest usable sample here is `n = 2` rather than `n = 1`.

The t-distribution itself. Its density on `ν > 0` degrees of freedom is

    f(t; ν)  =  Γ((ν+1)/2) / ( √(νπ) · Γ(ν/2) ) · (1 + t²/ν)^(−(ν+1)/2)

symmetric about zero, with tails heavier than the normal's — `f(t; ν) ~ C·|t|^(−(ν+1))`, so only
moments below order `ν` exist, and the variance `ν/(ν−2)` exists only for `ν > 2`. As `ν → ∞` the
density converges to the standard normal's, and correspondingly `t_{q,ν} → z_q` from above. Because
`t_{q,ν} > z_q` for every finite `ν` and every `q > 1/2`, **`CONFIDENCE.T` is always at least as
wide as `CONFIDENCE.NORM` on the same three arguments**, dramatically so for small `n`.

The CDF is an incomplete beta function. With `I_x(a,b)` the regularized incomplete beta,

    P(T ≤ t; ν)  =  1 − ½ · I_{ν/(ν+t²)}(ν/2, ½)     for t ≥ 0

which is the identity the reference engine's t-family kernels are written against, and it is why
the accuracy of every Student's t surface in Excel — and hence of this function — is really the
accuracy of one incomplete beta routine.

Domain and range: `α ∈ (0,1)`, `s > 0`, `n ≥ 2`, and the value is positive throughout. It diverges
as `α → 0⁺`, tends to `0` as `α → 1⁻`, is linear in `s`, and falls roughly like `n^(−1/2)` with a
second, faster contribution from the shrinking t quantile as the degrees of freedom grow.

## Arguments

`CONFIDENCE.T(alpha, standard_dev, size)` — three arguments, all required; the registry records an
arity of exactly 3.

| Argument | Meaning | Admissible values |
|---|---|---|
| `alpha` | The significance level; the confidence level is `100·(1 − alpha)%` | `0 < alpha < 1` |
| `standard_dev` | The standard deviation of the data (see the defect note below) | `> 0` |
| `size` | The sample size; truncated if not an integer | `≥ 2` in practice — see the error discussion |

The misunderstood positions are the same two as on the normal sibling, plus one specific to this
function:

- `alpha` is the significance level, not the confidence level. `0.05` means 95%.
- `size` is documented as truncated when non-integer.
- The **degrees of freedom are derived, not supplied**. A reader who has already computed
  `n − 1` elsewhere and passes it here gets an interval one degree of freedom too wide.

All three are numeric slots under ordinary to-number coercion
([chapter 02](../model/02-coercion-and-lifting.md)).

## Result and edge cases

Returns `Number` — a positive half-width.

Documented boundary behaviour, from Microsoft's page:

- Any nonnumeric argument → `#VALUE!`.
- `alpha ≤ 0` or `alpha ≥ 1` → `#NUM!`.
- `standard_dev ≤ 0` → `#NUM!`.
- `size` non-integer → truncated.
- **`size = 1` → `#DIV/0!`.** This is the one documented error condition with no counterpart on
  `CONFIDENCE.NORM`, and it is mathematically the right shape: at `n = 1` there are zero degrees of
  freedom and the t quantile does not exist.

What the documentation does **not** say: what `size < 1` does. The normal sibling documents
`size < 1 → #NUM!`; this page documents only the `size = 1` case. So `CONFIDENCE.T(0.05, 1, 0)` is
undocumented behaviour.

**The reference engine disagrees with the documented `size = 1` row.** OxFunc's
`confidence_t_kernel` rejects every truncated size below `2` with `#NUM!` — so on the reference
engine `size = 1` yields `#NUM!` where Microsoft's page documents `#DIV/0!`. The Handbook records
this as a divergence between documentation and the reference engine and does not resolve it: nobody
has run `CONFIDENCE.T(0.05, 1, 1)` in Excel within this record. It is the first probe on the list
below.

A second, smaller reference-engine divergence from the sibling: `confidence_t_kernel` maps
non-finite arguments to `#VALUE!`, while `confidence_norm_kernel` maps them to `#NUM!`. The two
functions live in different modules, and the difference looks like module drift rather than a
modelled distinction. Whether Excel distinguishes them is unchecked.

The projection records this surface with an explicit by-index lift over its first three argument
positions, carrying an in-repo note that the profile was verified against live Excel 16.0 build
20026 — one of the few structural facts on this page with a named observation context.

## Errors

As documented on Microsoft's `CONFIDENCE.T` page:

| Error | Condition |
|---|---|
| `#VALUE!` | Any argument is nonnumeric |
| `#NUM!` | `alpha ≤ 0` or `alpha ≥ 1` |
| `#NUM!` | `standard_dev ≤ 0` |
| `#DIV/0!` | `size` equals 1 |

Reference-engine rows that are **not** documented, recorded here as implementation facts about
OxFunc rather than as claims about Excel: truncated `size < 2` → `#NUM!` (which contradicts the
`#DIV/0!` row above at `size = 1`), non-finite argument → `#VALUE!`, and empty or missing arguments
treated as `0` before the domain checks — which routes an empty `alpha` to `#NUM!` rather than to
the `#VALUE!` its sibling produces on the same shaped call.

## A documentation defect on the Microsoft page

Microsoft's `CONFIDENCE.T` page describes `standard_dev` as "The population standard deviation for
the data range and is assumed to be known" — word for word the sentence on the `CONFIDENCE.NORM`
page.

For `CONFIDENCE.T` that sentence is self-contradicting. If the population standard deviation were
known, the correct interval would use the normal quantile and there would be no reason for this
function to exist; the t-distribution enters *precisely because* `s` is a sample estimate with its
own sampling variability. The wording appears to have been copied from the normal page when the
2010 dotted pair was created.

The Handbook records this as a documentation defect, not as a behaviour claim: nothing about what
the function *computes* is in doubt, only what the page says the argument is. Readers deciding
between the two functions should use the operative rule — known `σ` takes `CONFIDENCE.NORM`,
sample `s` takes `CONFIDENCE.T` — and not the argument prose.

## Relationships

- **[CONFIDENCE.NORM](FUNC.CONFIDENCE.NORM.md)** — the known-`σ` sibling. Same shape, `z_{1−α/2}`
  in place of `t_{1−α/2,n−1}`. Not a legacy/modern pair: both are current, and both were introduced
  in Excel 2010 alongside the legacy `CONFIDENCE`.
- **[CONFIDENCE](FUNC.CONFIDENCE.md)** — the pre-2010 spelling, which corresponds to the *normal*
  function. There is no legacy spelling of `CONFIDENCE.T`; it is new function surface, not a
  rename. This matters when reading migration tables that treat every dotted name as a rename.
- **[T.INV.2T](FUNC.T.INV.2T.md)** — the two-tailed t quantile.
  `CONFIDENCE.T(α, s, n)` and `T.INV.2T(α, n-1) * s / SQRT(n)` are the same expression, and the
  reference engine literally implements the first by calling its `t_inv_2t` kernel. Whether Excel
  composes them the same way is unchecked, and the two-route comparison is a good probe.
- **[T.TEST](FUNC.T.TEST.md)** — the hypothesis-test counterpart; **[Z.TEST](FUNC.Z.TEST.md)** is
  the other member of this reference-engine module.
- **[STDEV.S](FUNC.STDEV.S.md)** — the usual supplier of `standard_dev`. Feeding `STDEV.P` here
  instead is a common and silent error: it estimates with the wrong divisor and then pairs it with
  a t quantile that assumes the other one.

## Numerical notes

Three sources of error stack in this function, and only the first is specific to it.

1. **The incomplete beta.** The t quantile is obtained by inverting an incomplete beta function.
   That routine — Didonato & Morris's `BRATIO` (TOMS 708) is the canonical one, and OxFunc's
   research record identifies a `BRATIO`-shaped substrate under Excel's beta-side surfaces — is
   where nearly all the error lives. Its accuracy varies sharply by branch: continued fraction,
   series, and asymptotic regions do not agree in their last bits, and which branch a given
   `(a, b, x)` lands in is an implementation choice, not a mathematical one.
2. **The inversion.** There is no closed form for `t_{q,ν}`. Implementations either use a
   root-finder over the CDF or a dedicated quantile series (A&S 26.7.5 gives the classical
   Cornish–Fisher-style expansion in `z` and `ν`). A root-finder inherits the forward function's
   error and adds its own stopping-criterion error; a series is fast and has its own accuracy
   envelope. The reference engine uses bisection over its forward kernel, which is robust and
   slow and cannot be more accurate than the forward evaluation it brackets.
3. **The composition.** `t · s / √n` — one multiplication, one square root, one division, and a
   choice of association that changes the last bit.

The specific hazard worth calling out: `1 − α/2` again. For very small `α` the quantile's argument
saturates against `1`, and the answer is then determined entirely by how the implementation handles
the tail. A careful implementation works with `α/2` in the upper tail directly rather than forming
the complement — the same principle OxFunc records as decisive for the chi-square and F inverses,
where it inverts the published right-tail surface at `p` instead of the CDF at `1 − p` because the
complement staging carried a systematic small-`p` bias.

Standard references for the routines involved: Abramowitz & Stegun chapter 26.7 (Student's t, its
incomplete-beta form, and the percentage-point expansions); Didonato & Morris, ACM TOMS 708
(`BRATIO`); Cephes `incbet`/`incbi`; Boost's `students_t` quantile; Numerical Recipes' `betai` and
`betacf` (readable, and explicitly less accurate than the TOMS routine).

## What has not been checked

No Handbook vector suite exists for `CONFIDENCE.T`; `vectors/` publishes nothing at this revision.

Two Excel-comparison evidence records list this surface among their subjects, and both must be read
with their own warnings:

- **`EV-DIST-0012`** — a sampled cell-reference re-sweep that produced a per-surface figure for
  sixteen distribution surfaces. For `CONFIDENCE.T` this is the *only* per-surface figure that has
  ever been published anywhere in the upstream record, because the later campaign explicitly did
  not probe this surface. The record carries a reader warning that these are sampled counts on
  small denominators with an internally inconsistent column, so the figure is weak evidence and
  the record says so itself.
- **`EV-DIST-0017`** — the standing residual register for the statistical distributions. It
  publishes run totals over a fifteen-function family and **no per-surface figures at all**; its
  reader warning forbids attributing any of it to one surface. It tells you this family has been
  measured; it tells you nothing specific about `CONFIDENCE.T`.

So: this surface has been touched by Excel-comparison work, has one weak sampled figure of its own,
and has never had a dedicated sweep. The numbers themselves are rendered mechanically by the
evidence layer beside this page, with their scopes and warnings attached; read them there.

Inputs worth probing first:

1. **`CONFIDENCE.T(0.05, 1, 1)`** — the documented `#DIV/0!` against the reference engine's
   `#NUM!`. This is a documented-behaviour-versus-implementation disagreement with a one-cell
   resolution, and it is the highest-value probe on this page.
2. **`CONFIDENCE.T(0.05, 1, 0)` and `(0.05, 1, 1.9)`** — sizes below one and sizes that truncate to
   one. Undocumented on this page and directly adjacent to the disagreement above.
3. **`CONFIDENCE.T(0.05, 1, 2)`** — the smallest working sample, whose value is `t_{0.975,1}`, the
   Cauchy quantile `tan(π·0.475) ≈ 12.706…`, divided by `√2`. A closed-form target with no
   incomplete-beta subtlety, which makes it a clean read of the composition arithmetic.
4. **`CONFIDENCE.T(α, 1, 2)` and `T.INV.2T(α, 1)/SQRT(2)` swept together** — whether Excel composes
   this function from its own public t quantile, bit for bit, or runs a separate path.
5. **Tiny `α`** (`1e-10` down to `1e-17`) at small and large `n` — the complement-saturation
   regime, and the place where an implementation that never forms `1 − α/2` separates from one
   that does.
6. **Large `n`** (`1e6`, `1e9`) — where `t_{q,n−1}` should approach `z_q`; a comparison against
   `CONFIDENCE.NORM` at the same arguments measures how well the t tail degrades to normal.
7. **Non-finite and empty arguments in each position** — the two reference-engine divergences from
   `CONFIDENCE.NORM` noted above are both in this region.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| half-width | The single number returned; the interval is the mean plus and minus it |
| degrees of freedom | `n − 1` here, derived from `size`, never supplied |
| regularized incomplete beta | `I_x(a,b)`; the function the t CDF is expressed through |
| complement staging | Evaluating a tail quantity directly rather than as `1 −` its complement |
| documentation defect | A statement on the vendor page that is wrong or inconsistent, independent of behaviour |

## Sources

- Microsoft, "CONFIDENCE.T function" —
  <https://support.microsoft.com/en-us/office/confidence-t-function-e8eca395-6c3a-4ba9-9003-79ccc61d3c53>
  (retrieved for this pass: syntax, argument descriptions including the copied `standard_dev`
  wording discussed above, the size-truncation rule, and the four documented error conditions
  including `size = 1 → #DIV/0!`).
- Microsoft, "CONFIDENCE.NORM function" —
  <https://support.microsoft.com/en-us/office/confidence-norm-function-7cec58a6-85bb-488d-91c3-63828d4fbfd4>
  (retrieved for this pass; the comparison partner and the origin of the shared argument wording).
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 26.7 — Student's
  t-distribution, its incomplete-beta representation, and percentage points.
- Didonato & Morris, "Algorithm 708: Significant Digit Computation of the Incomplete Beta Function
  Ratio", ACM TOMS 18 (1992) — the `BRATIO` routine underlying t and F evaluation.
- Handbook evidence records `EV-DIST-0012` and `EV-DIST-0017` (`content/evidence/records/`); read
  their reader warnings, which bound what may be attributed to this surface.
- Handbook call-model chapter [02 Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.CONFIDENCE.T.json` and
  `data/presence/FUNC.CONFIDENCE.T.json`.
- OxFunc `docs/function-lane/FUNCTION_SLICE_CONFIDENCE_TEST_FAMILY_CONTRACT_PRELIM.md` and
  `crates/oxfunc_core/src/functions/confidence_test_family.rs` at commit `473efa3` — the
  `t_inv_2t` dependence, the `n < 2 → #NUM!` guard, the non-finite `#VALUE!` mapping, and the
  empty-to-zero scalar preparation, read as implementation facts about the reference engine.
