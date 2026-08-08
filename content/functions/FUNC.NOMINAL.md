---
schema: efh.function-page/v1
function_id: FUNC.NOMINAL
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
family: financial_time_value_family
role_in_family: >-
  The compounding-frequency translator in the effective-to-nominal direction; EFFECT's inverse, and
  the member whose root extraction is the family's sharpest small-rate cancellation site.
---

# NOMINAL

## What it computes

`NOMINAL(effect_rate, npery)` converts an **effective annual rate** into the **nominal annual rate
compounded `npery` times a year** that would produce it.

    NOMINAL = npery × ( (1 + effect_rate)^(1/npery) − 1 )

The construction reads directly: `(1 + effect_rate)^(1/npery)` is the per-period growth factor whose
`npery`-fold repetition gives one year's growth; subtracting 1 turns it into a per-period rate; and
multiplying by `npery` restates it as an annual figure by the nominal convention — that is, by
simple multiplication, ignoring the compounding it has just accounted for. That deliberate
inconsistency *is* the nominal convention, and it is why nominal rates are always at or below their
effective counterparts.

`NOMINAL` is the exact inverse of [EFFECT](FUNC.EFFECT.md):

    NOMINAL(EFFECT(r, n), n) = r        and        EFFECT(NOMINAL(e, n), n) = e

in exact arithmetic. In binary64 the round trip is accurate but not bitwise idempotent; see
*Numerical notes*.

Domain and range: for `effect_rate > 0` and integer `npery ≥ 1`, the result is positive, strictly
less than `effect_rate` for every `npery > 1`, and strictly decreasing in `npery`. At `npery = 1`
it equals `effect_rate`.

The limit as `npery → ∞` is `ln(1 + effect_rate)` — the continuously-compounded equivalent, the
force of interest. As with `EFFECT`, that limit is approached and never reached, because `npery` is
truncated to an integer.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `effect_rate` | The effective annual interest rate, as a decimal fraction. Required. | — |
| `npery` | The number of compounding periods per year. Required. | — |

Two documented rules, identical to `EFFECT`'s:

1. **`npery` is truncated to an integer.**
2. **`npery` must be at least 1** and `effect_rate` must be greater than zero; both violations are
   `#NUM!`.

`npery` is not restricted to the conventional 1, 2, 4, 12; weekly (52) and daily (365) are ordinary
inputs.

Both slots are numeric and follow the shared coercion rules; in the reference engine at commit
`473efa3` an omitted-slot Missing marker and a blank cell both arrive as `0`, so a blank `npery`
trips the `#NUM!` guard.

## Result and edge cases

Returns `Number` — a nominal annual rate as a decimal fraction.

- **`npery = 1`** returns `effect_rate`, up to floating-point rounding; the computation still forms
  `(1 + e)^1 − 1` rather than short-circuiting, so exact bit recovery is not guaranteed.
- **`effect_rate ≤ 0` is an error, not a computation.** As with `EFFECT`, this is a *documented
  restriction* rather than a mathematical one: the expression is well defined at `e = 0` (giving 0)
  and for small negative `e`. Excel refuses both. Negative-rate environments therefore need the
  formula written out.
- **Large `npery`** approaches `LN(1 + effect_rate)` from above and stays finite.
- **Non-integer `npery`** truncates toward zero, so `0.9` becomes `0` and is `#NUM!`.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

As documented on Microsoft's `NOMINAL` page:

| Error | Condition |
|---|---|
| `#VALUE!` | `effect_rate` or `npery` is non-numeric |
| `#NUM!` | `effect_rate ≤ 0` or `npery < 1` |

The reference engine raises the same pair, applying the `npery < 1` test after truncation.

## Relationships

- **[EFFECT](FUNC.EFFECT.md)** — the exact inverse. The two carry identical arguments, identical
  domain restrictions and identical error conditions; only the direction differs. Round-tripping
  through both is the natural self-test.
- **`LN`** — the continuous limit, `LN(1 + effect_rate)`, which `NOMINAL` approaches as `npery`
  grows.
- **`RRI`** — the compound rate implied by observed values rather than translated from a quote.
  `RRI(npery, 1, 1 + effect_rate)` computes exactly the per-period rate `NOMINAL` divides out,
  which makes it an independent route to the same intermediate.
- **`RATE`** — the annuity solver. Unrelated in mechanism (`RATE` iterates, `NOMINAL` is closed
  form) but frequently confused because both return "a rate".
- **Confused with**: APR versus APY. `NOMINAL` maps APY back to APR. Which one a disclosure regime
  requires is a legal question.
- **`FV` / `PMT` / `PV`** — the annuity functions consume a *periodic* rate. `NOMINAL(e, 12)/12`,
  not `NOMINAL(e, 12)`, is what belongs in a monthly `FV` call. Passing the annual nominal rate to a
  monthly annuity is one of the most common financial-modelling errors in spreadsheets, and no
  function in the family can detect it.

## Numerical notes

The evaluation `npery × ((1 + effect_rate)^(1/npery) − 1)` has a cancellation problem that is worse
than `EFFECT`'s, because the root extraction drives the intermediate toward 1 as `npery` grows.

For a 5% effective rate compounded daily, `(1.05)^(1/365) ≈ 1.000133...`, and subtracting 1 from a
value that close to 1 discards roughly four significant decimal digits before the multiplication by
365 restores the magnitude. The relative error of the answer is the relative error of the
subtraction, and no amount of care in the multiplication recovers it. The larger `npery` is — that
is, the closer the function gets to its continuous limit — the more accuracy the naive form loses.

The stable form uses the same primitive pair as the rest of this family:

    NOMINAL = npery × expm1( log1p(effect_rate) / npery )

`log1p` computes `ln(1 + e)` without forming `1 + e`, and `expm1` computes `e^x − 1` without forming
the near-1 intermediate that cancels. This holds full relative accuracy for every admissible
`npery`, and it makes the continuous limit visible directly: as `npery → ∞`, `npery·expm1(L/npery)`
→ `L = log1p(effect_rate)`. The `expm1`/`log1p` treatment descends from the `fdlibm` lineage that
modern libms inherit.

The reference engine at commit `473efa3` uses the naive form: `(1 + effect_rate)` raised to
`1/npery` by a plain floating-point power, minus 1, times `npery`. Two implementation facts about
that engine are worth recording, because they are exactly where a disagreement would live:

1. It calls the **plain floating-point power**, whereas its inverse `EFFECT` calls the worksheet
   `POWER` kernel. Two functions that are exact inverses of one another routed through different
   power implementations will not round-trip cleanly, and the discrepancy will look like noise
   rather than like a bug.
2. The exponent is formed as `1.0/npery`, which is inexact for every `npery` that is not a power of
   two — so `npery = 12` and `npery = 365` both introduce a rounding before the power is even
   evaluated.

The Handbook does not claim what Excel does internally.

## What has not been checked

No Handbook vector suite exists for `NOMINAL`; `vectors/` publishes nothing at this revision, so no
suite-scoped claim exists for it. No Excel-comparison evidence record names `NOMINAL` in its
subjects; the financial evidence records that exist for this implementing module name other
surfaces, and the Handbook does not attribute a group measurement to a surface a record does not
list. **The family containing `NOMINAL` has been measured against live Excel; this surface has not
been measured separately.** Nobody has checked `NOMINAL`'s values against Excel within the
Handbook's record.

The argument meanings, the truncation rule and the two error rows are Microsoft's documented
statements; the evaluation form, the power-routine asymmetry with `EFFECT` and the reciprocal
exponent are read from the reference engine's source at commit `473efa3`.

Inputs worth probing first:

1. **A modest rate at a high frequency** — `NOMINAL(0.05, 365)` and `NOMINAL(0.0001, 365)` —
   compared against `npery·expm1(log1p(e)/npery)` computed to higher precision. This is the probe
   the cancellation analysis above exists to motivate.
2. **`EFFECT(NOMINAL(e, n), n)` round trips** across a grid of `e` and `n`. If the two functions use
   different power routines the residual will be systematic in `n`, not random — which is the
   signature to look for.
3. **`NOMINAL(e, 1)`**, which should return `e`; whether it returns `e`'s exact bits reveals whether
   the trivial case is special-cased.
4. **`NOMINAL(e, n)` against `n × (RRI(n, 1, 1 + e))`** — the same per-period rate by a different
   function in the same family, needing no external oracle.
5. **Powers of two versus non-powers of two for `npery`** — `n = 2, 4, 8` against `n = 12, 365` —
   which isolates the `1/npery` rounding from everything else.
6. **`effect_rate = 0` and a small negative `effect_rate`**, confirming the documented `#NUM!`.
7. **`npery = 0.9`, `1.9`, `0`** — the truncation boundary and the side the guard sits on.
8. **A very large `npery`** — `NOMINAL(0.05, 1E9)` — against `LN(1.05)`, testing the continuous
   limit where the naive form is at its worst.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| nominal rate | An annual rate obtained by multiplying a periodic rate by the number of periods |
| effective annual rate | The single annual rate giving the same year-end value with one compounding |
| compounding frequency | `npery`, the number of compounding periods per year, truncated to an integer |
| force of interest | The `npery → ∞` limit, `LN(1 + effect_rate)`, unreachable through this function |
| `expm1` / `log1p` | The accurate primitives for `e^x − 1` and `ln(1 + x)` near zero |

## Sources

- Microsoft, "NOMINAL function" —
  <https://support.microsoft.com/en-us/office/nominal-function-7f1ae29b-6b92-435e-b950-ad8b190ddd2b>
  (syntax, argument meanings, the truncation of `npery`, the equation, and the documented `#VALUE!`
  and `#NUM!` conditions).
- Handbook, [EFFECT](FUNC.EFFECT.md) — the inverse conversion, the shared domain restrictions and
  the companion numerical discussion.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- OxFunc `crates/oxfunc_core/src/functions/financial_time_value_family.rs` at commit `473efa3` —
  the `nominal` kernel, its guards, its plain floating-point power and its reciprocal exponent,
  read as implementation facts about that engine.
- Handbook projections `data/functions/FUNC.NOMINAL.json` and `data/presence/FUNC.NOMINAL.json`.
