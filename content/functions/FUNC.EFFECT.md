---
schema: efh.function-page/v1
function_id: FUNC.EFFECT
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
  The compounding-frequency translator in the nominal-to-effective direction; NOMINAL's inverse and
  the family's smallest closed-form member.
---

# EFFECT

## What it computes

`EFFECT(nominal_rate, npery)` converts a **nominal annual rate compounded `npery` times a year**
into the **effective annual rate** — the single annual rate that produces the same year-end value
with one compounding.

    EFFECT = (1 + nominal_rate/npery)^npery − 1

The identity behind it: growing at `r/n` per period for `n` periods multiplies principal by
`(1 + r/n)^n`, and the effective rate is that multiplier minus one. A 10% nominal rate compounded
monthly is not 10% a year — it is about 10.47%, and the gap is the whole reason the function
exists. Consumer-credit disclosure regimes worldwide are built on this conversion.

Domain and range: for `nominal_rate > 0` and integer `npery ≥ 1`, `EFFECT` is strictly increasing in
`npery` and strictly greater than `nominal_rate` for every `npery > 1`. At `npery = 1` it is
exactly `nominal_rate`.

The limiting case is the interesting one. As `n → ∞`,

    (1 + r/n)^n → e^r        so        EFFECT → e^r − 1

which is continuous compounding. That limit is approached but never reached through this function,
because `npery` is truncated to an integer — there is no "continuous" setting. `EXP(r) − 1` is the
formula for that, and `EFFECT` with a very large `npery` approximates it from below.

The function is **not** an annualization of a period rate in the simple-interest sense. `EFFECT`
compounds; `INTRATE` and `DISC` do not. Feeding a money-market discount quote to `EFFECT` produces
a number with no financial meaning.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `nominal_rate` | The nominal annual interest rate, as a decimal fraction. Required. | — |
| `npery` | The number of compounding periods per year. Required. | — |

Two documented rules:

1. **`npery` is truncated to an integer.** `npery = 12.9` compounds twelve times, not 12.9 times.
2. **`npery` must be at least 1** and `nominal_rate` must be greater than zero; both violations are
   `#NUM!`.

`npery` is not restricted to the conventional 1, 2, 4, 12 — 365 (daily) and 52 (weekly) are
accepted and are exactly what the function is for.

Both slots are numeric and follow the shared coercion rules; in the reference engine at commit
`473efa3` an omitted-slot Missing marker and a blank cell both arrive as `0`, which for `npery`
means the `#NUM!` guard fires.

## Result and edge cases

Returns `Number` — an effective annual rate as a decimal fraction.

- **`npery = 1`** returns `nominal_rate` — up to floating-point rounding. It is not guaranteed to
  return the input's exact bits, because the computation still forms `(1 + r)^1 − 1`.
- **`nominal_rate ≤ 0` is an error, not a computation.** This is a *documented restriction*, not a
  mathematical boundary: `(1 + r/n)^n − 1` is perfectly well defined for `r = 0` (giving 0) and for
  small negative `r`. Excel refuses both. A model that needs the effective rate of a zero or
  negative nominal rate cannot use this function and must write the formula out.
- **Large `npery`** approaches `EXP(nominal_rate) − 1` from below and stays finite; there is no
  overflow for any realistic rate.
- **Non-integer `npery`** truncates toward zero, so `0.9` becomes `0` and therefore `#NUM!`.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

As documented on Microsoft's `EFFECT` page:

| Error | Condition |
|---|---|
| `#VALUE!` | `nominal_rate` or `npery` is non-numeric |
| `#NUM!` | `nominal_rate ≤ 0` or `npery < 1` |

The reference engine raises the same pair, applying the `npery < 1` test **after** truncation, so
`npery = 0.9` is `#NUM!` by way of becoming `0`.

## Relationships

- **[NOMINAL](FUNC.NOMINAL.md)** — the exact inverse. `NOMINAL(EFFECT(r, n), n)` should return `r`
  up to floating-point rounding, and the round trip is the natural self-test for both. The two
  functions carry identical domain restrictions.
- **`RRI`** — the compound rate implied by a present value, a future value and a period count.
  Where `EFFECT` translates a *quoted* rate between conventions, `RRI` extracts a rate from
  observed values.
- **`FV` / `FVSCHEDULE`** — the growth these rates describe. `FV(EFFECT(r,n), 1, 0, −P)` and
  `FV(r/n, n, 0, −P)` agree by construction; that agreement is the definition of the effective
  rate, and it makes a good check.
- **`EXP`** — the continuous-compounding limit, `EXP(r) − 1`, which `EFFECT` approaches as `npery`
  grows and never reaches.
- **`XIRR` / `IRR`** — internal rates of return, which are already effective rates over their own
  period; do not pass them through `EFFECT` a second time.
- **Confused with**: APR versus APY in consumer lending. `EFFECT` maps APR to APY. Which of the two
  a jurisdiction requires you to disclose is a legal question, not a spreadsheet one.

## Numerical notes

The formula `(1 + r/n)^n − 1` is the textbook expression and it is the **worst** way to evaluate the
quantity when `r/n` is small.

Two error sources compound. First, `1 + r/n` loses the low bits of `r/n` when the ratio is tiny —
for a 1% rate compounded daily, `r/n ≈ 2.7×10⁻⁵`, so roughly five significant digits of the input
are discarded in the addition. Second, the result of the power is close to 1, so the final
subtraction of 1 is a cancellation that exposes exactly those damaged bits. The two effects are
multiplicative in the relative error, and the damage grows with `n`.

The standard remedy is the `expm1`/`log1p` pair:

    EFFECT = expm1(npery × log1p(nominal_rate/npery))

`log1p` computes `ln(1 + x)` accurately for small `x` without forming `1 + x`, and `expm1` computes
`e^x − 1` without forming the intermediate that cancels. Together they hold full relative accuracy
across the whole admissible domain. This is the same pairing that appears throughout the annuity
functions in this family, and the standard reference treatment is in the `fdlibm`/Cody-and-Waite
lineage that every modern libm inherits.

The reference engine at commit `473efa3` does **not** take that route for `EFFECT`: it forms
`1 + nominal_rate/npery`, raises it through the worksheet `POWER` kernel, and subtracts 1 — the
naive form. That is an implementation fact about the reference engine, recorded here because it
locates precisely where a small-rate, high-frequency disagreement would come from. It is also worth
noting as a structural observation that its inverse, `NOMINAL`, does **not** call the same power
routine — it uses a plain floating-point power. Two functions that are exact inverses of each other
going through different power implementations is the kind of asymmetry that shows up as a failing
round trip long before it shows up as a wrong answer.

The Handbook does not claim what Excel does internally for either function.

## What has not been checked

No Handbook vector suite exists for `EFFECT`; `vectors/` publishes nothing at this revision, so no
suite-scoped claim exists for it. No Excel-comparison evidence record names `EFFECT` in its
subjects — the financial evidence records that exist for this module name other surfaces, and the
Handbook does not attribute a group measurement to a surface the record does not list. **The family
containing `EFFECT` has been measured against live Excel; this surface has not been measured
separately.** Nobody has checked `EFFECT`'s values against Excel within the Handbook's record.

The argument meanings, the truncation rule and the two error rows are Microsoft's documented
statements; the evaluation form and the power-routine asymmetry with `NOMINAL` are read from the
reference engine's source at commit `473efa3`.

Inputs worth probing first:

1. **A small rate at a high frequency** — `EFFECT(0.0001, 365)` and `EFFECT(1E-8, 365)` — compared
   against `expm1(n·log1p(r/n))` computed to higher precision. This is the probe the numerical
   discussion above exists to motivate, and it is where the naive form loses the most.
2. **`NOMINAL(EFFECT(r, n), n)` round trips** over a grid of `r` and `n`, which tests both members
   and would expose the power-routine asymmetry as a systematic bias rather than as noise.
3. **`EFFECT(r, 1)`**, which should return `r` — and whether it returns `r`'s exact bits or one ULP
   away tells you whether the trivial case is special-cased.
4. **`EFFECT(r, n)` against `FV(r/n, n, 0, −1) − 1`** — the same quantity by a different route
   inside the same family, a metamorphic check that needs no external oracle.
5. **`nominal_rate = 0` and a small negative `nominal_rate`**, to confirm the documented `#NUM!`
   rather than the mathematically sensible answer.
6. **`npery = 0.9`, `npery = 1.9`, `npery = 0`** — the truncation boundary and which side of it the
   guard sits on.
7. **A very large `npery`** — `EFFECT(0.1, 1E9)` — against `EXP(0.1) − 1`, testing the continuous
   limit and any overflow behaviour in the power routine.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| nominal rate | An annual rate quoted without regard to intra-year compounding |
| effective annual rate | The single annual rate giving the same year-end value with one compounding |
| compounding frequency | `npery`, the number of compounding periods per year, truncated to an integer |
| continuous compounding | The `npery → ∞` limit, `EXP(r) − 1`, unreachable through this function |
| `expm1` / `log1p` | The accurate primitives for `e^x − 1` and `ln(1 + x)` near zero |

## Sources

- Microsoft, "EFFECT function" —
  <https://support.microsoft.com/en-us/office/effect-function-910d4e4c-79e2-4009-95e6-507e04f11bc4>
  (syntax, argument meanings, the truncation of `npery`, the equation, and the documented `#VALUE!`
  and `#NUM!` conditions).
- Handbook, [NOMINAL](FUNC.NOMINAL.md) — the inverse conversion and the shared domain restrictions.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- OxFunc `crates/oxfunc_core/src/functions/financial_time_value_family.rs` at commit `473efa3` —
  the `effect` kernel, its guards, and its use of the worksheet `POWER` kernel (contrasted with
  `nominal`'s plain floating-point power), read as implementation facts about that engine.
- Handbook projections `data/functions/FUNC.EFFECT.json` and `data/presence/FUNC.EFFECT.json`
  (arity, classification axes, implementing module and the fifteen-surface family it shares).
