---
schema: efh.function-page/v1
function_id: FUNC.FVSCHEDULE
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
  The varying-rate counterpart to FV: a bare product of growth factors, with no payment stream, no
  timing switch and no closed-form inverse.
---

# FVSCHEDULE

## What it computes

`FVSCHEDULE(principal, schedule)` returns the future value of a `principal` after applying a
**series of compound interest rates**, one per period:

    FVSCHEDULE  =  principal × Π_{i=1}^{m} (1 + r_i)

where `r_1 … r_m` are the entries of `schedule` in order. That is the entire definition: a running
product of growth factors.

Its place in the family is defined by what it is *not*. [FV](FUNC.FV.md) assumes one **constant**
rate and adds a payment stream; `FVSCHEDULE` allows the rate to **vary** and has no payment stream
at all. When the rate changes over the life of the investment — a floating-rate note, a forecast
path of short rates, a scenario ladder — `FV` is structurally the wrong function and `FVSCHEDULE` is
the right one.

The relation to `FV` is exact in the degenerate case: a schedule of `n` identical rates `r` gives
`principal × (1 + r)^n`, which is `−FV(r, n, 0, −principal)`.

Two mathematical properties are worth stating because they are easy to assume wrongly:

1. **Order does not matter.** Multiplication is commutative, so shuffling the schedule leaves the
   answer unchanged in exact arithmetic. (In binary64 it changes the last bits — see *Numerical
   notes* — but never the value in any meaningful sense.) Contrast `NPV` or `IRR`, where order *is*
   meaning.
2. **The average that reproduces the answer is geometric, not arithmetic.** The single equivalent
   constant rate is `(Π(1 + r_i))^(1/m) − 1`, which is at or below the arithmetic mean of the `r_i`
   by the AM–GM inequality, with equality only when all rates are equal. Substituting the mean rate
   into `FV` therefore **overstates** the result, always. That gap is the whole reason volatility
   drags on compound returns, and `FVSCHEDULE` is the function that shows it.

Range: the product is positive whenever every `1 + r_i > 0`. A rate of exactly `−1` in any period
annihilates the whole thing to zero; a rate below `−1` flips the sign, which is arithmetically
consistent and financially meaningless.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `principal` | The present value — the amount to compound forward. Required. | — |
| `schedule` | An array or reference of interest rates to apply, one per period, **in order**. Required. | — |

- **The rates are per period, expressed as decimal fractions**, and the period is implicit: it is
  however long each schedule entry represents. There is no period-length argument and no unit
  checking.
- **The number of periods is the length of the schedule.** There is no separate `nper`, which
  removes one whole class of mismatch error that the `FV`-family functions are prone to.
- **`schedule` must be an array or reference of numbers.** Microsoft documents that non-numeric
  values in the schedule produce `#VALUE!`, and that **empty cells are treated as zero — no
  interest**.

That last documented rule is worth a moment. For a pure product, "treated as zero" and "skipped"
are the same thing: a factor of `1 + 0 = 1` changes nothing. So the two readings coincide here by
arithmetic accident, and `FVSCHEDULE` is one of the few functions in this family where the
blank-versus-zero question has no observable consequence. (In the reference engine at commit
`473efa3` the collector skips blanks outright, and the answer is identical either way.) A reader who
carries the habit over to `IRR`, where a skipped blank shortens the series and moves every later
cash flow one period earlier, will be badly surprised.

## Result and edge cases

Returns `Number` — a value in the same units as `principal`.

- **An empty schedule** leaves `principal` unchanged. The reference engine returns `principal` for a
  schedule that collects to nothing; whether Excel agrees is unverified.
- **A rate of exactly `−1`** produces zero, and every subsequent rate is irrelevant. The reference
  engine applies no guard here — no `#NUM!`, no `#DIV/0!` — it simply multiplies by zero.
- **A rate below `−1`** flips the sign of the result. Two such rates flip it back. This is
  arithmetic behaving correctly on input that has no financial reading, and no function in Excel
  will warn you.
- **Negative `principal`** carries through; the family sign convention is not enforced here, because
  there is no equation to balance — `FVSCHEDULE` is a product, not a solved equation, and it returns
  the same sign it was given.
- **Logical values in the schedule** are converted to 1 and 0 by the reference engine's collector, so
  a stray `TRUE` becomes a 100% return for that period.
- **Numeric text in the schedule** is coerced; non-numeric text produces `#VALUE!`.
- **Overflow**: a long schedule of large rates overflows, and the reference engine converts a
  non-finite result to `#NUM!` rather than publishing an infinity.
- Empty, missing and error arguments follow the shared call model; see
  [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | A schedule entry is non-numeric (documented) |
| `#VALUE!` | `principal` receives non-numeric text or an unsupported value kind |
| propagated | An error value in the schedule or in `principal` surfaces as that error |
| `#NUM!` | The product is not representable |
| `#VALUE!` | The call is made with any number of arguments other than two |

The first row is Microsoft's documented condition. The remainder is the reference engine's behaviour
at commit `473efa3` under the shared call model. The Handbook has not observed any of it in Excel.

There is deliberately **no** row for a rate at or below `−1`: the reference engine has no guard
there, and that absence is itself worth probing.

## Relationships

- **[FV](FUNC.FV.md)** — the constant-rate, payment-bearing sibling. `FVSCHEDULE(P, {r,r,…,r})` and
  `−FV(r, n, 0, −P)` compute the same quantity by different routes and are a good cross-check.
- **`PV`** — there is **no `PVSCHEDULE`**. To discount through a varying rate path you divide, or
  you build the reciprocal schedule by hand. The asymmetry is a real gap in the function set.
- **`RRI`** — the equivalent constant rate: `RRI(m, principal, FVSCHEDULE(principal, schedule))`
  returns the geometric mean growth rate of the schedule. This pair is the cleanest way to express
  the AM–GM point above.
- **`PRODUCT`** — what `FVSCHEDULE` does, one step removed. `principal * PRODUCT(1 + schedule)` as an
  array formula is the hand-built equivalent, and it is worth knowing because it makes the treatment
  of blanks and text explicit rather than implicit.
- **`GEOMEAN`** — the geometric mean, on `1 + r` values, which relates directly to the equivalent
  constant rate.
- **[EFFECT](FUNC.EFFECT.md)** — a *within-year* compounding conversion at a single rate, not a
  path of rates. Different question.
- **Confused with**: applying an average rate. See the AM–GM note; the arithmetic mean always
  overstates.

## Numerical notes

`FVSCHEDULE` is a product of `m` factors and its error analysis is the classical one for repeated
multiplication.

**Error accumulates linearly, and there is no cancellation.** Each multiplication introduces at most
half an ULP of relative error, and the errors compound multiplicatively — so after `m` steps the
relative error is bounded by roughly `m·u/(1 − m·u)` with `u` the unit roundoff. For any schedule a
spreadsheet would realistically hold, that is a handful of ULPs at worst. There is no subtraction
anywhere in the computation, and therefore no cancellation and no ill-conditioning. Compared with
almost everything else in this family, `FVSCHEDULE` is benign.

**Order changes the last bits.** Floating-point multiplication is commutative but not associative,
so the product of the same factors in a different order can differ by an ULP or two. The reference
engine multiplies strictly left to right in schedule order. An implementation that sorted the
factors, or that used pairwise multiplication for parallelism, would return slightly different bits
for the same schedule — which is worth knowing before treating a last-digit disagreement as a bug.

**The `1 + r_i` addition is the accuracy floor.** For a small rate — a daily rate of `1e-5`, say —
forming `1 + r` discards the low bits of `r` before any multiplication happens, and no amount of
care in the product recovers them. A schedule of many small rates therefore loses more precision in
its additions than in its multiplications. The accurate alternative accumulates
`Σ log1p(r_i)` and exponentiates once:

    FVSCHEDULE  =  principal × exp( Σ log1p(r_i) )

which keeps full relative accuracy per term at the cost of one transcendental per entry and a
different rounding profile. That form is strictly better for long schedules of small rates and is
not what the reference engine does — it multiplies. The Handbook does not claim what Excel does.

**Overflow, not gradual underflow, is the representable-range risk.** A product of growth factors
greater than 1 grows geometrically; a long schedule at a high rate overflows to infinity, which the
reference engine converts to `#NUM!`. Underflow toward zero requires rates near `−1` and is a
degenerate case rather than a numerical one.

## What has not been checked

No Handbook vector suite exists for `FVSCHEDULE`; `vectors/` publishes nothing at this revision, so
no suite-scoped claim exists for it. No Excel-comparison evidence record names `FVSCHEDULE` in its
subjects; the financial records covering this implementing module name other surfaces, and the
Handbook does not attribute a group measurement to a surface a record does not list. **The family
containing `FVSCHEDULE` has been measured against live Excel; this surface has not been measured
separately.** Nobody has checked `FVSCHEDULE` against Excel within the Handbook's record.

The argument meanings, the `#VALUE!` condition on non-numeric schedule entries and the
empty-cells-treated-as-zero rule are Microsoft's documented statements. The collector's handling of
blanks, logicals and numeric text, the absence of a guard at `rate = −1`, the left-to-right product
and the empty-schedule result are read from the reference engine's source at commit `473efa3`.

Inputs worth probing first:

1. **A schedule containing exactly `−1`, and one containing a value below `−1`.** No guard exists in
   the reference engine, so the questions are whether Excel returns zero, returns a sign-flipped
   number, or errors. These are the only genuinely undefined-looking inputs the function admits.
2. **A schedule containing `TRUE`**, which the reference engine turns into a 100% return for that
   period. If Excel skips logicals instead — as its sibling `IRR` is recorded as doing for its own
   array — that is a divergence with a large observable effect.
3. **An empty schedule and an all-blank schedule**, testing whether `principal` comes back unchanged
   and whether "treated as zero" and "skipped" really do coincide.
4. **A long schedule of small rates** — 365 entries of `1e-5` — against
   `principal × exp(Σ log1p(r_i))` at higher precision. This quantifies the `1 + r` accuracy floor
   and is the only probe here with a numerical rather than a definitional target.
5. **The same schedule shuffled**, which must not change the value and may change the last bits;
   the size of that change measures the accumulation order.
6. **A constant schedule against `FV`** — `FVSCHEDULE(P, n copies of r)` versus `−FV(r, n, 0, −P)` —
   a metamorphic check crossing two kernels in the same family that must agree mathematically.
7. **`RRI(m, P, FVSCHEDULE(P, schedule))` against `GEOMEAN(1 + schedule) − 1`**, pinning the
   geometric-mean identity.
8. **Overflow**: a schedule long enough to exceed the double range, to see `#NUM!` rather than an
   infinity.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| schedule | The ordered array of per-period rates applied in sequence |
| growth factor | `1 + r_i`, the multiplier for one period |
| equivalent constant rate | The geometric mean growth rate reproducing the same terminal value |
| AM–GM gap | The amount by which an arithmetic mean rate overstates compound growth |
| accumulation order | The left-to-right sequence in which the factors are multiplied |

## Sources

- Microsoft, "FVSCHEDULE function" —
  <https://support.microsoft.com/en-us/office/fvschedule-function-bec29522-bd87-4082-bab9-a241f3fb251d>
  (syntax, the two argument descriptions, the `#VALUE!` condition on non-numeric schedule values,
  and the remark that empty cells are treated as zero — no interest).
- Handbook, [FV](FUNC.FV.md) — the constant-rate sibling and the shared family equation.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- OxFunc `crates/oxfunc_core/src/functions/financial_time_value_family.rs` at commit `473efa3` —
  the `fvschedule` kernel, its left-to-right product, its non-finite guard and the
  `numeric_sequence_from_args` collector governing blanks, logicals and text; read as implementation
  facts about that engine.
- Handbook projections `data/functions/FUNC.FVSCHEDULE.json` and
  `data/presence/FUNC.FVSCHEDULE.json`.
