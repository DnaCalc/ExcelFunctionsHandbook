---
schema: efh.function-page/v1
function_id: FUNC.IRR
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-FIN-0016
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
family: cashflow_rate_family
role_in_family: >-
  The equally-spaced root finder: the rate at which a periodic cash-flow series has zero present
  value. XIRR's undated sibling and the family member whose answer is, by construction, the output
  of an iterative solver.
---

# IRR

## What it computes

`IRR(values, [guess])` returns the **internal rate of return** of a series of cash flows occurring
at **regular, equally spaced intervals**: the periodic rate `r` at which the series' net present
value is zero.

    NPV(r) = Σ_{k=0}^{n} CF_k / (1 + r)^k = 0

Note the indexing: **the first value is at time zero and is not discounted.** This is the opposite
convention from `NPV`, whose first argument is discounted by one period, and the mismatch between
the two is a classic source of off-by-one-period errors — see *Relationships*.

The rate is per *period*, and the period is whatever interval separates consecutive entries. Monthly
cash flows give a monthly rate; annualizing it is your job, and it is a compounding conversion
(`(1+r)^12 − 1`), not a multiplication.

### The mathematics: a polynomial, and why the answer may not be unique

Substituting `v = 1/(1 + r)` turns the equation into a polynomial of degree `n` in `v`:

    CF_0 + CF_1·v + CF_2·v² + … + CF_n·vⁿ = 0

so **`IRR` is a polynomial root-finding problem**, and everything the theory of polynomials says
applies. In particular, by Descartes' rule of signs, the number of positive real roots is at most
the number of sign changes in the coefficient sequence — that is, in the cash-flow sequence itself.

The consequences are not edge cases; they are the function's real behaviour:

- **A conventional series** — one sign change, outflows first and then inflows — has **exactly one**
  economically meaningful root. This is the case the function is designed for and the only case in
  which "the" internal rate of return is well defined.
- **Multiple sign changes** may admit **several** real roots, each a legitimate solution of the
  equation. `IRR` returns one of them: the one its iteration converged to from `guess`. Changing
  `guess` can change the answer to a *different correct answer*. This is mathematics, not a defect,
  and no implementation can fix it.
- **No sign change at all** — all inflows or all outflows — has no root in the admissible range, and
  the function reports an error.

The classic pathology is a project with a large terminal cleanup cost (mine reclamation, nuclear
decommissioning): outflow, inflows, outflow. Two sign changes, two IRRs, both real, both useless.
[MIRR](FUNC.MIRR.md) exists precisely because of this.

Domain: the solution is sought over `r > −1`; at `r = −1` the discount factors are undefined, and
below it the odd-power terms change sign in ways that carry no financial reading.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `values` | An array or reference of cells containing the cash flows, **in the order in which they occur**. Required. | — |
| `guess` | A starting estimate for the iteration. Optional. | 0.1 (10%) |

Three things about `values` that matter more than the formula does:

1. **Order is meaning.** `IRR` has no dates; position in the array *is* the time index. Sorting the
   range, inserting a row, or reading a two-dimensional block in an unexpected order changes the
   answer silently.
2. **It must contain at least one positive and one negative value.** Otherwise there is no root; see
   Errors.
3. **Non-numeric entries are skipped, not zeroed.** The reference engine's commentary at commit
   `473efa3` records that text, logical values and blank cells inside the array are **ignored**,
   identically for array constants and ranges, and attributes that reading to live Excel 16.0 build
   20026. This is a consequential rule: a skipped blank does not consume a period, so a gap in the
   column *shortens* the series rather than inserting a zero cash flow. A model that leaves a blank
   row for a period with no cash flow gets a different answer than one that writes `0`.

`guess` affects convergence and, when there are several roots, which one is found. It does not
affect the equation.

## Result and edge cases

Returns `Number` — a periodic rate as a decimal fraction.

- **The answer is the output of an iterative solver.** It is an approximation to a root, not a
  closed form, and it carries the solver's tolerance rather than the arithmetic's. Two
  implementations that both "work" can return visibly different digits.
- **`guess` changes the answer when there are multiple roots**, as above.
- **Non-convergence returns an error** rather than a partial answer.
- **A single sign change does not guarantee an economically sensible rate** — it guarantees
  uniqueness, which is a different thing. A series that returns barely more than it costs over
  twenty periods has a tiny IRR that is correct and uninformative.
- **Two cash flows** is the smallest admissible series and has a closed-form answer,
  `r = −CF_1/CF_0 − 1`; it is the natural accuracy probe.
- **A two-dimensional array** is rejected by the reference engine with `#REF!` when it has more than
  one row *and* more than one column; a single row or single column is accepted.
- **Any error value inside the array** is reported as `#VALUE!` in the reference engine rather than
  propagating the specific code — a departure from the usual propagation discipline in
  [Coercion and lifting](../model/02-coercion-and-lifting.md), and one the module's own commentary
  attributes to observed Excel behaviour.

## Errors

| Error | Condition |
|---|---|
| `#NUM!` | `values` contains no positive value, or no negative value |
| `#NUM!` | The iteration does not converge |
| `#NUM!` | Fewer than two usable cash flows |
| `#VALUE!` | An error value appears inside the `values` array (code not preserved) |
| `#REF!` | `values` is a genuinely two-dimensional array |
| `#VALUE!` | The call is made with more than two arguments |

The first two rows are the conditions every reference states — the sign requirement and
non-convergence. The remainder is the reference engine's behaviour at commit `473efa3` under the
shared call model. Microsoft's page documents the sign requirement and describes the iteration as
returning an error when it fails to converge within its iteration budget. The Handbook has not
observed any of this in Excel.

## Relationships

- **`XIRR`** — the dated sibling: arbitrary, irregular dates instead of equal periods, and an
  annualized rather than a per-period answer. If the cash flows are not equally spaced, `IRR` is the
  wrong function and will not say so.
- **`NPV`** — the same discounting, evaluated at a given rate instead of solved for one. **The
  indexing differs**: `NPV`'s first value is discounted one period, `IRR`'s is not. The idiomatic
  bridge is `CF_0 + NPV(IRR(values), values-without-CF_0) ≈ 0`, and writing
  `NPV(IRR(values), values)` instead is the standard mistake.
- **`XNPV`** — the dated `NPV`, and `XIRR`'s companion.
- **[MIRR](FUNC.MIRR.md)** — the modified internal rate of return: a **closed form**, always unique,
  with explicit finance and reinvestment rates. Where `IRR` implicitly assumes every interim inflow
  is reinvested at `IRR` itself — an assumption that is rarely true and is the deepest criticism of
  the measure — `MIRR` makes the reinvestment rate an argument. For any series with multiple sign
  changes, `MIRR` is the better tool.
- **`RATE`** — the annuity solver. Also iterative, also guess-sensitive, but it solves a *constant
  payment* equation rather than an arbitrary cash-flow polynomial.
- **`RRI`** — the compound rate between two values over `n` periods, closed form, no iteration. For
  a two-flow series it answers the same question `IRR` does.

## Numerical notes

`IRR` is a root-finder, and every difficulty in it is a root-finding difficulty.

**The objective is ill-conditioned near flat roots.** The derivative `dNPV/dr` can be small near the
root — for a long series at a low rate, the present value changes little over a wide band of `r`.
Newton-type iteration then takes large uncertain steps, and the *residual* being small does not
imply the *root* is accurate. This is the fundamental reason `IRR` answers from different
implementations disagree in more digits than ordinary arithmetic would explain: the problem, not the
code, is poorly conditioned.

**Convergence is not guaranteed and the failure mode is silent.** A Newton iteration on a polynomial
with several real roots can cycle, can diverge past `r = −1` out of the domain, or can converge to a
root far from the one the analyst had in mind. A bracketing method (bisection, Brent) is robust
where Newton is fast; a serious implementation uses both, which is what the reference engine does —
a bounded Newton solve with a bracketing fallback and a hard iteration cap.

**Publication is a separate decision from convergence.** The reference engine does something worth
knowing about: after its solver converges it scans a small neighbourhood of the root in ULP steps,
finds the plateau of representable rates over which `|NPV|` attains its minimum, and returns the
**midpoint of that plateau**. This is not part of the mathematical definition of an internal rate of
return. It is a publication rule, adopted because the residual near a flat root is genuinely
constant over many adjacent doubles and *something* must be chosen. Recording it here is the point:
when two implementations of `IRR` disagree in the last digits, the disagreement is at least as
likely to live in a publication rule as in the arithmetic.

**Accumulation order.** `Σ CF_k/(1+r)^k` is a sum of terms of decreasing magnitude, evaluated
forward. For long series with large early flows this loses low-order bits; a compensated summation
or a reverse-order accumulation would keep more of them, and would give different last digits.
Whether Excel accumulates forward is not something this page asserts.

The polynomial-root framing, Descartes' rule and the multiple-IRR pathology are standard
corporate-finance material; the root-finding treatment — Newton with bracketing fallback, and the
conditioning of flat roots — is standard numerical analysis, developed in *Numerical Recipes* among
many others.

## What has not been checked

No Handbook vector suite exists for `IRR`; `vectors/` publishes nothing at this revision, so no
suite-scoped claim exists for it.

One Excel-comparison record names `IRR` in its subjects: **EV-FIN-0016**. It is worth reading in
full, because what it records is mostly an *absence*. There is no production pass count for `IRR`
anywhere in the upstream source. What the catalogue publishes for `IRR` is a pair of diverging
witnesses and nothing else. The candidate scores that do exist rate simulators and mask fits raced
against corpora built to fit them — in-sample by construction, across inconsistent denominators —
and the record notes that an earlier identification in that lane was **retracted as an over-fit**.
Its reader warning is explicit that `IRR` must never inherit a figure from a sibling solver or from
a `PRICE` record.

The Handbook's reading of that record: `IRR`'s absence of a measurement is not a gap in extraction,
it is the state of the evidence. **Nobody has established what Excel's `IRR` returns, and nothing on
this page should be read as saying otherwise.**

The implementing module additionally carries open defect streams touching this surface —
`BUG-FUNC-009` on default-guess solver non-convergence, `BUG-FUNC-014` on solver precision drift in
the dated sibling, and `BUG-FUNC-028` on conversion, text, date and array-lift coercion gaps.

Inputs worth probing first:

1. **A two-flow series** — `IRR({−100, 121})`, whose exact answer is `0.21` — where the closed form
   is known, the conditioning is perfect and any disagreement is pure solver or publication
   behaviour. Nothing else on this list is as diagnostic.
2. **The same series with several different `guess` values**, which separates a guess-independent
   answer from a guess-dependent one and is the cheapest test of solver determinism.
3. **A two-sign-change series** — e.g. `{−100, 300, −220}`, which has two real roots — probed from
   guesses on either side. This is the multiple-root behaviour, and it is where implementations most
   visibly differ from one another.
4. **A blank cell in the middle of the range versus an explicit `0`**, which tests the
   skip-versus-zero rule that shortens the series. This one changes answers in real models.
5. **Text and logical entries inside the array**, and an error value, testing the ignore rule and
   the `#VALUE!` collapse that does not preserve the incoming code.
6. **A long, nearly-flat series** — twenty small inflows against one outflow at a rate near zero —
   which exercises the ill-conditioned regime and the ULP-plateau publication rule at the same time.
7. **A series with no sign change**, confirming `#NUM!`, and a single-element series, confirming the
   minimum-length behaviour.
8. **`CF_0 + NPV(IRR(v), v[1:])`**, which must be approximately zero and which pins the indexing
   convention that separates `IRR` from `NPV`.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| internal rate of return | The periodic rate at which the series' net present value is zero |
| conventional series | One sign change — outflows then inflows — the case with a unique root |
| multiple IRRs | Several real roots, arising when the cash-flow sequence changes sign more than once |
| iterative solver | The root-finding procedure whose output the returned rate is |
| ULP plateau | The band of adjacent representable rates over which the residual attains its minimum |
| reinvestment assumption | `IRR`'s implicit assumption that interim inflows earn `IRR` itself |

## Sources

- Microsoft, "IRR function" —
  <https://support.microsoft.com/en-us/office/irr-function-64925eaa-9988-495b-b290-3ad0c163c1bc>
  (syntax, the ordering requirement on `values`, the requirement for at least one positive and one
  negative value, the role of `guess`, and the description of the calculation as an iteration that
  errors when it does not converge).
- Handbook evidence record `EV-FIN-0016` — the `IRR` record, its two diverging witnesses, its
  in-sample candidate scores, its retracted identification and its reader warning.
- Handbook, [MIRR](FUNC.MIRR.md) — the closed-form alternative for series with multiple sign
  changes.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — the error-propagation
  discipline this function's array collector departs from, and
  [The value universe](../model/01-value-universe.md).
- OxFunc `crates/oxfunc_core/src/functions/cashflow_rate_family.rs` at commit `473efa3` — the
  `irr_kernel`, its bounded Newton solve with bracketing fallback, its ULP-plateau publication rule,
  its array collector and its documented live-Excel attribution for the skip rules; read as
  implementation facts about that engine.
- Handbook projections `data/functions/FUNC.IRR.json` and `data/presence/FUNC.IRR.json` (arity,
  classification axes, implementing module, the `XIRR`/`XNPV` siblings and the `BUG-FUNC-009`,
  `BUG-FUNC-014` and `BUG-FUNC-028` defect streams).
