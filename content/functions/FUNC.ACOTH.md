---
schema: efh.function-page/v1
function_id: FUNC.ACOTH
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0006
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Acoth method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.acoth"
    role: "documented description and argument description; notable for stating no domain constraint at all"
  - work: "Microsoft Support — ACOTH function"
    locator: "https://support.microsoft.com/en-us/office/acoth-function-cc49480f-f684-4171-9fc5-73e4e852300f"
    role: "the canonical worksheet article; not retrieved for this pass (fetch returned HTTP 403)"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 4 (Elementary Transcendental Functions), section 4.6"
    locator: "4.6.22 and the arccoth branch conventions"
    role: "the logarithmic closed form, the excluded interval, and the relation acoth(x) = atanh(1/x)"
  - work: "Kahan, Branch Cuts for Complex Elementary Functions"
    locator: null
    role: "the branch structure of the inverse hyperbolic cotangent and why its cut is the closed unit interval"
  - work: "Higham, Accuracy and Stability of Numerical Algorithms"
    locator: "chapter 1, the log1p discussion"
    role: "the cancellation in log of a ratio near one, which is this function's large-argument hazard"
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
family: acoth
role_in_family: >-
  The inverse hyperbolic cotangent on |x| > 1: the only member of the inverse hyperbolic set whose
  domain is the complement of an interval rather than an interval, and one of the two surfaces in
  the family carrying an open discrepancy record.
---

## What it computes

`ACOTH(number)` is the inverse hyperbolic cotangent.

    acoth x  =  (1/2) * ln( (x + 1) / (x - 1) )  =  atanh(1/x),     |x| > 1

- **Domain**: `|x| > 1`, that is `(-infinity, -1) ∪ (1, +infinity)`. The closed interval
  `[-1, 1]` is excluded: `coth` maps `R \ {0}` onto exactly `|y| > 1`, so no real argument in the
  unit interval has a real inverse hyperbolic cotangent. This is the *complement* of `ATANH`'s
  domain, and the two functions partition the real line between them but for the two endpoints.
- **Range**: `(-infinity, 0) ∪ (0, +infinity)` — every nonzero real, with zero itself unattained.
  `acoth` never returns zero, because `coth` never equals zero.
- **Parity**: odd. `acoth(-x) = -acoth(x)`. This is exact mathematics and, as the identification
  work in this family has shown for its sibling, not something an implementation gets for free.
- **Poles**: logarithmic singularities at `x = 1+` (where `acoth -> +infinity`) and `x = -1-`
  (where `acoth -> -infinity`). Approach from *inside* the interval is not possible; there is
  nothing there.
- **Asymptotics**: `acoth(x) = 1/x + 1/(3x^3) + 1/(5x^5) + ...` for `|x| > 1`. As
  `|x| -> infinity` the function decays like `1/x` and reaches into the subnormals. This is the
  half of the domain where all the numerical difficulty lives.
- **Near the poles**: with `x = 1 + t`, `acoth(1 + t) = (1/2) * ln(2/t) + O(t)`. The growth is
  logarithmic, so even an argument one ulp above `1` produces only a modest number — around 18.
  The function is far better behaved at its poles than its poles suggest.
- **Derivative**: `d/dx acoth x = 1/(1 - x^2)`, which is negative on the whole domain (so `acoth`
  is decreasing on each branch) and finite everywhere except at the excluded endpoints.
- **Complex continuation**: the branch cut is the closed interval `[-1, 1]` — exactly the excluded
  real domain. `ATANH`'s cuts are the two outer rays. The two functions have complementary cuts,
  which is the precise statement of the complementary-domain observation above.

Abramowitz & Stegun give the closed form in chapter 4 section 4.6; Kahan's branch-cut paper is the
standard treatment of why the cut sits where it does.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The hyperbolic cotangent of the value you want. Required. | — |

That description is Microsoft's, from the Learn reference: "the hyperbolic cotangent of the angle
that you want".

**A documentation gap the Handbook records.** Microsoft's Learn reference for `ACOTH` states *no
constraint whatsoever* on the argument — no mention of `|x| > 1`, no mention of the excluded
interval, no error condition. The reference engine rejects `|x| <= 1`. That is documentation and
implementation disagreeing by omission on the single most important fact about this function, and
it is the kind of divergence this Handbook exists to publish. Microsoft's worksheet article, which
may well state the constraint, was not retrieved for this pass.

One argument; the reference engine records an arity of exactly one and a unary numeric
scalar-or-array lift profile, so arrays lift elementwise. Ordinary numeric slot under the shared
coercion rules — see [Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`.

- **`x = 1` and `x = -1`** are *not* in the domain and are rejected. The interval is open; these
  are the poles.
- **Anything strictly inside `(-1, 1)`**, including zero and every subnormal, is a domain failure.
  Readers arriving from `ATANH` — whose domain is exactly this interval — get this wrong in both
  directions.
- **Just outside `±1`** the answer is finite and modest. There is no overflow near the poles,
  because the growth is logarithmic: the largest magnitude reachable from a double argument is
  around 18.4, attained one ulp outside `1`.
- **Large magnitudes** decay towards zero like `1/x`, and the result reaches into the subnormal
  range at the top of the double domain. This is the region where relative accuracy is hard to
  keep, and where the two candidate evaluation forms visibly differ.
- **Arrays** lift elementwise, with out-of-domain elements erroring element-locally.

The reference engine's projected `real_result_policy` records `arg_domain_guard=none` and
`non_finite=allow`; the `|x| > 1` test lives in the kernel rather than in a declared guard axis.

Two rows of the battery rendered beside this page are marked **host-scoped**, meaning their
outcome is tied to the machine that produced them rather than being a portable fact. That flag is
itself informative: it says the last bits of this surface are not platform-independent in the
reference engine, which is consistent with the substrate identification described below.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | `|number| <= 1`, including `±1` themselves and everything in between | Reference engine's kernel; **not stated on Microsoft's Learn page**, which gives no constraint |
| `#VALUE!` | The argument does not convert to a number | Shared coercion rules |
| propagated | An error value in the argument is returned unchanged | Shared coercion rules |

## Relationships

- **`ATANH`** — the reciprocal partner and the complementary domain: `acoth(x) = atanh(1/x)`
  wherever both are defined, `ATANH` on `(-1, 1)` and `ACOTH` on `|x| > 1`. Upstream identification
  work treats the two together for exactly this reason, and both carry evidence records here.
- **`COTH`** — the forward function. `COTH(ACOTH(x)) = x` to rounding on the domain;
  `ACOTH(COTH(t)) = t` for every nonzero `t`.
- **`ACOSH`** — shares the exclusion of the unit interval, but only on one side: `ACOSH` takes
  `[1, infinity)`, `ACOTH` takes `|x| > 1`. They differ at exactly `x = 1`, where `ACOSH` is
  defined and zero and `ACOTH` is not defined at all.
- **`ACOT`** — the circular namesake, and unlike `ACOTH` it has no domain restriction whatsoever.
  The pair is a good illustration that a hyperbolic namesake is not a variant: it is a different
  function with a different domain and a different shape.
- **`LN`** — the substrate of the closed form, and the source of the large-argument hazard.
- **Confused with**: `1/COTH(x)`, and with `ATANH`, whose domain is the exact complement.

## Numerical notes

`ACOTH` has one serious hazard, and it is at the far end of the domain rather than at the poles —
the opposite of what the function's shape suggests.

**The large-argument cancellation.** Evaluate `(1/2) ln((x+1)/(x-1))` for large `x`. The ratio
`(x+1)/(x-1)` tends to `1`, and `ln` of a value near `1` is the canonical accuracy disaster: the
information about the answer lives entirely in the digits of the ratio that the division has just
rounded away. Concretely, once `x` exceeds about `2^53`, the doubles `x+1` and `x-1` are the same
number, the ratio is exactly `1`, and the naive form returns exactly zero — where the true answer
is approximately `1/x`, a perfectly representable positive number. The failure begins long before
that: relative accuracy degrades steadily from around `x = 10`.

**The remedies**, both standard:

1. **Reciprocate.** `acoth(x) = atanh(1/x)`, and `1/x` is small, so an accurate small-argument
   `atanh` delivers full relative accuracy right down to the subnormals. This is the single most
   effective change and it costs one division.
2. **Use `log1p` on the difference form.** `atanh(u) = (1/2)(log1p(u) - log1p(-u))` with
   `u = 1/x`, which never forms a quantity near `1` and never takes a logarithm of one. Higham's
   treatment of `log(1+x)` is the standard reference for why the `log1p` primitive exists at all.

Near the poles, by contrast, the naive form is fine: `(x+1)/(x-1)` is large there, `ln` of a large
number is well conditioned, and the only subtlety is that `x - 1` must be computed from a double
argument that is already close to `1` — which Sterbenz's lemma makes exact.

**So the correct branch structure is the reverse of the intuitive one**: use the direct ratio-log
*near* the poles, where the ratio is far from one, and the reciprocal `atanh` form *far* from
them, where the ratio is close to one. An implementation that uses one form everywhere is wrong at
one end or the other.

**What is on record upstream.** OxFunc's identification note for this surface records a hypothesis
of exactly that two-regime shape for Excel's `ACOTH` — a direct ratio-logarithm at small magnitude
and a reciprocal `log1p`-pair form above a switch magnitude, with the sign handled by oddness —
and states that the exact switch double is not yet pinned. That is an upstream identification, and
the Handbook publishes it as such: it is a hypothesis with supporting rows and open residuals, not
a settled fact, and the evidence record attached to this page carries its status, its maturity, and
its own reader warnings. The record's figures are rendered mechanically beside this page; this
prose does not restate them.

**Why the switch point matters more here than usual.** Because the two forms disagree in the last
bits over a whole band, any implementation that places its switch differently from Excel's will
differ from Excel on every argument in the gap between the two placements — even though both forms
are individually excellent. This is the characteristic signature of a piecewise elementary
function, and it is why identification work on this family concentrates on locating switches rather
than on improving accuracy.

## What has not been checked

The evidence attached to this page is **`EV-MATH-0006`**, an **open-discrepancy** record whose
subject list contains `FUNC.ACOTH`. It is a genuine per-surface record: it carries a count measured
for this surface, an upstream severity and maturity, and its own statement that the surface is not
signed off and that rows remain open. **The figures belong to the record and are rendered beside
this page mechanically; nothing in this prose restates them, and no agreement claim follows from
them.** The record's own status text is the authority on what it does and does not establish.

**No Handbook vector suite exists for `ACOTH`.** The Handbook has not itself observed this function
in Excel; what exists is an upstream identification with an attached record.

The documented statement above — the description and the argument wording — comes from Microsoft's
Learn `WorksheetFunction.Acoth` reference, which was retrieved and which states no domain
constraint. Microsoft's worksheet article was not retrieved (HTTP 403).

Probes worth running first:

1. **`ACOTH(1)`, `ACOTH(-1)`, `ACOTH(0)`** — the three points that pin the excluded interval and
   the error code that Microsoft's Learn page does not mention.
2. **One ulp outside each pole** — `1 + 2^-52` and its negative — which is where the largest
   magnitudes live and where the direct ratio form is at its best. A mismatch here would be
   surprising and therefore very informative.
3. **A logarithmic sweep of large `|x|`** from about `10` to the largest finite double, compared
   against `1/x` at the far end (where the two agree to well within a double). This is the probe
   that identifies which of the two forms is in use, and it needs no high-precision oracle above
   about `1e8`.
4. **A dense sweep across the switch magnitude** named in the upstream identification. The open
   residuals recorded there sit in exactly this band, and dense probing is what the upstream note
   itself says is required to pin the switch double.
5. **`ACOTH(x) + ACOTH(-x)`** across the domain — the oddness test. Whether it is exactly zero for
   every `x` distinguishes an implementation that computes on `|x|` and restores the sign from one
   that evaluates the signed ratio directly. The sibling surface `ATANH` is on record upstream as
   *not* being exactly odd, which makes this probe unusually worth running here.
6. **Array arguments mixing admissible and excluded elements**, for the element-local error policy.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| excluded interval | `[-1, 1]`, on which `acoth` has no real value; the complex branch cut |
| complementary domain | `ATANH` on `(-1, 1)`, `ACOTH` on `|x| > 1`; together the whole line but the endpoints |
| ratio-log form | `(1/2) ln((x+1)/(x-1))`, accurate near the poles and inaccurate far from them |
| reciprocal form | `atanh(1/x)`, accurate far from the poles |
| switch magnitude | The argument at which a piecewise implementation changes form; the target of identification work |
| host-scoped row | A battery row whose outcome is tied to the machine that produced it |

## Sources

- Microsoft Learn, "WorksheetFunction.Acoth method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.acoth> (retrieved: the
  description and argument wording; **no domain constraint and no error condition are stated
  there**).
- Microsoft, "ACOTH function" —
  <https://support.microsoft.com/en-us/office/acoth-function-cc49480f-f684-4171-9fc5-73e4e852300f>
  — the canonical worksheet article. **Not retrieved for this pass** (HTTP 403).
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 4 section 4.6 — the closed
  form, the excluded interval, and the relation to `atanh`.
- Kahan, *Branch Cuts for Complex Elementary Functions* — the branch structure of `acoth` and why
  its cut is the closed unit interval.
- Higham, *Accuracy and Stability of Numerical Algorithms*, chapter 1 — the `log(1+x)` problem
  behind the large-argument remedy.
- Handbook evidence record `EV-MATH-0006` (subjects include `FUNC.ACOTH`) — an open-discrepancy
  record with a per-surface count, an upstream severity and maturity, and open rows. Its figures
  and warnings are rendered with the record.
- Handbook projections `data/functions/FUNC.ACOTH.json` (arity, lift profile, `real_result_policy`
  with `arg_domain_guard=none;non_finite=allow`) and `data/presence/FUNC.ACOTH.json` (implementing
  module; entries in the discrepancy and math-deviation catalogues and in the `BUG-FUNC-027`
  scalar-invocation sweep).
- OxFunc `crates/oxfunc_core/src/functions/acoth.rs` and the W109 identification note at commit
  `473efa3` — the two-regime substrate hypothesis and the statement that the switch double is not
  yet pinned.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md); sibling page
  [ATANH](FUNC.ATANH.md).
