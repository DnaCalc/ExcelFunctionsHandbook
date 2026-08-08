---
schema: efh.function-page/v1
function_id: FUNC.NOW
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: NOW function"
    locator: "https://support.microsoft.com/en-us/office/now-function-3337fd29-145a-4347-b2e6-20c904739c46"
    role: "documented syntax, volatility remark and formatting note"
  - work: "Microsoft 365 support: TODAY function"
    locator: "https://support.microsoft.com/en-us/office/today-function-5eb3078d-a82c-4736-8930-2f51a028fdd9"
    role: "the date-only sibling with the same volatility semantics"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
family: now_fn
role_in_family: "The current-instant reader: the category's canonical volatile, time-dependent
  function, implemented in its own module."
---

## What it computes

`NOW` returns the current date and time as a date serial: the whole part is today's date and the
fractional part is the time of day. It reads the host's clock, and it takes no arguments.

That makes it the one function in this assignment that is not a function in the mathematical sense
at all. Everything else here maps arguments to a value; `NOW` maps *nothing* to a value that
changes. Three consequences, and they are the substance of this page.

### It is volatile

Excel marks `NOW` **volatile**: any formula that references it is recalculated whenever the
workbook recalculates, not merely when one of its inputs changes. The reference engine's own
classification records this as `VolatileFull`, alongside `TimeDependent` determinism,
`ApplicationState` host interaction, `HostSerialized` thread safety and a `TimeProvider`
dependency — five separate axes all pointing at the same fact.

Volatility is not a performance footnote. A single `NOW` in a large model marks its whole dependent
subtree dirty on every recalculation, and volatility is contagious through the dependency graph. It
is also the reason `NOW` does *not* tick: the value changes when Excel recalculates, not
continuously. A cell showing `NOW` is a timestamp of the last recalculation, not a clock.

### It is not testable the way the rest of the category is

The Handbook's reference-engine battery **refuses to call `NOW`**, and the battery row on this page
says so in its own vocabulary: `cannot-call:nondeterministic-by-declaration:time-dependent`. That
refusal is correct and worth defending, because it is easy to mistake for a coverage gap.

A vector suite pairs an input with an expected output. `NOW` has no input, and its output is a
function of when you asked. There is nothing for a vector to hold. The refusal is a statement
about the function's nature, not about the harness's limitations — and it is exactly the kind of
typed missingness the Handbook prefers to a fabricated pass.

What *can* be characterized, given a controlled clock, are the properties around the value:

- the mapping from a known wall-clock instant to a serial (which is the
  [DATE](FUNC.DATE.md)/[TIME](FUNC.TIME.md) model, already testable without `NOW`);
- the **granularity** — how finely the fractional part is quantized, which is a host-clock property;
- the **stability within one recalculation** — whether two `NOW` calls in one pass return the same
  value;
- the **relationship to [TODAY](FUNC.TODAY.md)** — whether `TODAY()` equals `INT(NOW())` at every
  instant, including across a midnight boundary during a single recalculation.

None of those has been established here.

### It carries a formatting hint

`NOW` is one of the documented seed examples for the Handbook's presentation-hint model: it returns
a plain number, and Excel applies a date-time format to the cell when the formula is entered into a
General-formatted cell. The formatting is host-side adaptation at the worksheet boundary, not part
of the value — see [the value universe](../model/01-value-universe.md) and
[the call pipeline](../model/03-call-pipeline.md), stage 4.

## Arguments

`NOW()` — no arguments. The parentheses are required syntax, not an empty argument list in the
[value-universe](../model/01-value-universe.md) sense: there is no `Missing` marker involved,
because there is no parameter slot.

Everything that varies about `NOW`'s result comes from outside the call: the machine's clock, its
time zone, its daylight-saving state, and the workbook's date system. None of these is an argument,
and none of them is visible to a formula.

## Result and edge cases

Returns a `Number`: a date serial with a fractional part.

- **Local time, not UTC.** The value reflects the machine's local clock; the same workbook opened
  in two time zones shows different values. There is no argument and no worksheet function in this
  category to convert between them.
- **Daylight-saving transitions** make the local clock non-monotonic once a year. `NOW` inherits
  that: a value read before and after a backward transition can go down.
- **Granularity is a host property.** Whether the fractional part carries seconds, or something
  finer or coarser, is not something the function's contract states.
- **Representability.** The fraction is a double and 1/86400 is not exact, so the value is a nearby
  double rather than the exact time — the same caveat as on [TIME](FUNC.TIME.md), and it matters
  when `NOW` results are compared for equality or fed to [SECOND](FUNC.SECOND.md).
- **A static timestamp is not this function.** Excel's keyboard shortcuts for entering a fixed date
  or time insert a literal value, not a `NOW` formula; the two look identical in a cell and behave
  completely differently on the next recalculation.

## Errors

`NOW` takes no arguments, so it has no argument-derived error conditions. There is no documented
error return.

The interesting failure modes are not error *values* at all: they are host-level — a clock outside
the representable serial range, or an unavailable time provider — and lie outside what a worksheet
formula can observe.

## Relationships

- **[TODAY](FUNC.TODAY.md)** — the date-only sibling with the same volatility and the same host
  dependency. `INT(NOW())` and `TODAY()` are expected to agree; unverified here, and interesting
  precisely at midnight.
- **[HOUR](FUNC.HOUR.md), [MINUTE](FUNC.MINUTE.md), [SECOND](FUNC.SECOND.md)** — the usual
  consumers of `NOW`'s fractional part, and where its granularity becomes visible.
- **[DATE](FUNC.DATE.md) and [TIME](FUNC.TIME.md)** — the deterministic constructors. Any formula
  that can be written with them instead of `NOW` should be, because it becomes testable.
- **`RAND`, `RANDBETWEEN`, `OFFSET`, `INDIRECT`, `CELL`, `INFO`** — the other volatile functions.
  `NOW` is not alone in its recalculation behaviour, though it is alone with `TODAY` in reading a
  clock.
- **Confusable.** `NOW()` versus a pasted static timestamp; and `NOW()` versus `TODAY()` where only
  the date is wanted, since `NOW`'s fractional part silently breaks equality comparisons against
  whole-day serials.

## Notes for implementers

1. **Inject the clock; never call the system clock from the kernel.** A time provider passed in
   through the execution context is what makes the surrounding machinery testable even though the
   function is not. The reference engine models this as a declared `TimeProvider` dependency, which
   is the right shape.
2. **Freeze the instant for the duration of a recalculation pass.** Two `NOW` calls in one
   evaluation must agree, or formulas comparing them can observe a moment that never existed —
   `NOW() - NOW()` should be zero.
3. **Declare volatility explicitly** so the recalculation engine can propagate it. A `NOW` that is
   not declared volatile produces a workbook that quietly shows stale times.
4. **Do the local-time conversion outside the serial arithmetic**, and record the time zone and
   daylight-saving state as part of any captured observation. A recorded `NOW` value without its
   zone is uninterpretable.
5. **Do not attempt an `excel-bitexact` claim for this function.** The claim would have to be about
   the clock, not the computation.

## What has not been checked

There is no Handbook vector suite for `NOW`, no Excel-comparison evidence is recorded, and the
reference-engine battery declines to call it at all — for the principled reason set out above.

What *could* be established, with a controlled clock and a named Excel build, and has not been:

- **Granularity** — sample `NOW` in a tight recalculation loop and inspect the distinct values. This
  says how many bits of the fractional part are meaningful, which nothing in the documentation
  states.
- **Within-pass stability** — `NOW() = NOW()` and `NOW() - NOW()` in one formula, and across several
  cells in one recalculation.
- **`TODAY() = INT(NOW())`**, sampled either side of local midnight.
- **The exact serial produced for a known instant**, with the machine clock set deliberately: this
  is the only experiment that connects `NOW` to the testable serial model, and it turns an
  untestable function into a testable one plus a clock.
- **Behaviour across a daylight-saving transition**, and in a 1904-system workbook.

Note what none of these produce: a vector suite in the ordinary sense. `NOW` will never carry one,
and the honest published state for this entry is that its correctness question is about the host,
not about arithmetic.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| volatile | Recalculated on every workbook recalculation, regardless of whether inputs changed |
| time-dependent | The result depends on when the call happens; no input determines it |
| time provider | The injected clock an implementation should depend on instead of the system clock |
| not dispatchable | Battery outcome: the harness declines to call a declared-nondeterministic function |
| presentation hint | A core value carrying a formatting suggestion; applied by the host, not part of the value |

## Sources

- Microsoft 365 support, **NOW function** —
  <https://support.microsoft.com/en-us/office/now-function-3337fd29-145a-4347-b2e6-20c904739c46>.
- Microsoft 365 support, **TODAY function** —
  <https://support.microsoft.com/en-us/office/today-function-5eb3078d-a82c-4736-8930-2f51a028fdd9>.
- Handbook call-model chapters [01 value universe](../model/01-value-universe.md) (presentation
  hints, boundary model) and [03 call pipeline](../model/03-call-pipeline.md) (host-side
  adaptation, which names `NOW` and `TODAY` as its documented seed examples).
- [FUNC.DATE](FUNC.DATE.md) and [FUNC.TIME](FUNC.TIME.md) — the serial model whose value `NOW`
  reports.
- `data/functions/FUNC.NOW.json` (the `TimeDependent` / `VolatileFull` / `ApplicationState` /
  `HostSerialized` / `TimeProvider` classification), `data/presence/FUNC.NOW.json`,
  `data/battery/FUNC.NOW.json` (the declined-call rows).
