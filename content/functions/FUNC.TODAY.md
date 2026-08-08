---
schema: efh.function-page/v1
function_id: FUNC.TODAY
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: TODAY function"
    locator: "https://support.microsoft.com/en-us/office/today-function-5eb3078d-a82c-4736-8930-2f51a028fdd9"
    role: "documented syntax, volatility remark and formatting note"
  - work: "Microsoft 365 support: NOW function"
    locator: "https://support.microsoft.com/en-us/office/now-function-3337fd29-145a-4347-b2e6-20c904739c46"
    role: "the instant-reading sibling with the same volatility semantics"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
family: today_fn
role_in_family: "The current-date reader: NOW's date-only sibling, volatile and clock-dependent,
  implemented in its own module."
---

## What it computes

`TODAY` returns the current date as a whole date serial, with no time component. It reads the
host's clock and takes no arguments.

Like [NOW](FUNC.NOW.md), it is not a function in the mathematical sense: it maps nothing to a value
that changes. Three facts govern it, and they are the same three, with one difference that turns
out to matter.

### It is volatile

Excel marks `TODAY` volatile: formulas depending on it are recalculated on every workbook
recalculation. The reference engine's classification records `TimeDependent` determinism,
`VolatileFull` volatility, `ApplicationState` host interaction, `HostSerialized` thread safety and a
`TimeProvider` dependency — the same five-axis profile as `NOW`.

Here is the difference that matters: `TODAY`'s value is *constant for a day*, so the volatility is
almost always invisible. A `TODAY`-driven model recalculates constantly and produces the same
answer, right up until midnight, when everything changes at once. That combination — expensive and
silent, then abruptly different — is why `TODAY` in a large dependency tree is a well-known
performance and reproducibility hazard.

### The battery refuses it, correctly

The reference-engine battery on this page reports
`cannot-call:nondeterministic-by-declaration:time-dependent` for every probe row. That is not a
coverage gap; it is the harness declining to fabricate a fixture for a function whose output is a
function of when you asked. A vector pairs an input with an expected output, and `TODAY` has no
input.

This is worth stating plainly because `TODAY` looks so much more tractable than `NOW` — its value is
stable for twenty-four hours, so a naive suite could "pass" for a day and then fail. Stability is
not determinism.

### It carries a formatting hint

`TODAY` is, with `NOW`, one of the documented seed examples for the presentation-hint model: the
function returns a plain number and the host applies a date format when the formula is entered into
a General-formatted cell. The value itself is core; the formatting is host-side adaptation at the
worksheet boundary. See [the value universe](../model/01-value-universe.md) and
[the call pipeline](../model/03-call-pipeline.md), stage 4.

## Arguments

`TODAY()` — no arguments. The parentheses are syntax; there is no parameter slot and therefore no
`Missing` marker.

Everything that varies about the result comes from outside the call: the machine's clock and time
zone, and the workbook's date system.

## Result and edge cases

Returns a `Number`: a whole date serial, with the fractional part zero.

- **Local date, not UTC.** The same workbook opened in two time zones can show two different dates
  for several hours each day.
- **Midnight is a discontinuity**, and it is the only interesting moment in the function's
  behaviour. A recalculation that spans it, or two cells evaluated either side of it, can disagree.
- **Whole-day exactness.** Because the result is an integer-valued double, `TODAY()` is safe for
  equality comparison against other whole serials in a way [NOW](FUNC.NOW.md) is not. This is the
  practical reason to prefer `TODAY` whenever the time of day is not wanted: `A1 = TODAY()` works,
  `A1 = NOW()` almost never does.
- **A static date is not this function.** Excel's shortcut for entering today's date inserts a
  literal serial; it looks identical in the cell and never changes.

## Errors

`TODAY` takes no arguments and has no documented error return. Its failure modes are host-level — an
unavailable clock, or a system date outside the representable serial range — and are not observable
as worksheet error values.

## Relationships

- **[NOW](FUNC.NOW.md)** — the instant-reading sibling. `TODAY()` is expected to equal `INT(NOW())`;
  unverified here and interesting exactly at midnight.
- **[DATEDIF](FUNC.DATEDIF.md)** — `DATEDIF(birthdate, TODAY(), "Y")` is the canonical age formula,
  and the reason `TODAY`'s volatility is tolerated in so many workbooks.
- **[DATEVALUE](FUNC.DATEVALUE.md)** — shares an under-appreciated property: its documented
  year-omitted rule reads the system clock, making it clock-dependent too, though it is not declared
  volatile. `TODAY` is honest about depending on the clock; `DATEVALUE` is not.
- **[DATE](FUNC.DATE.md)** — the deterministic constructor. Any formula that can name its date
  explicitly should, because it then becomes reproducible and testable.
- **Volatile peers** — `NOW`, `RAND`, `RANDBETWEEN`, `OFFSET`, `INDIRECT`, `CELL`, `INFO`.

## Notes for implementers

1. **Inject the clock.** A time provider supplied through the execution context is what makes
   everything *around* this function testable, even though the function is not. The reference
   engine declares exactly such a dependency.
2. **Derive `TODAY` from the same instant as `NOW` within a recalculation pass.** If the two read
   the clock independently, a pass that spans midnight can produce a `TODAY` and a `NOW` that
   disagree about the date — an inconsistency no formula author can defend against.
3. **Truncate, do not round.** `TODAY` is the whole-day part of the current instant; rounding would
   make it flip at noon.
4. **Declare volatility** so the recalculation engine propagates it.
5. **Record the time zone with any captured observation.** A logged `TODAY` value without its zone
   is ambiguous for several hours a day.

## What has not been checked

There is no Handbook vector suite for `TODAY`, no Excel-comparison evidence is recorded, and the
reference-engine battery declines to call it, for the principled reason above.

What could be established with a controlled clock and a named Excel build, and has not been:

- **`TODAY() = INT(NOW())`**, sampled either side of local midnight and during a recalculation that
  spans it. This is the one genuinely subtle question the function poses.
- **The exact serial produced for a known system date**, with the clock set deliberately — the
  experiment that connects `TODAY` to the testable serial model of [DATE](FUNC.DATE.md).
- **Behaviour in a 1904-system workbook**, where the serial for the same date differs by 1,462.
- **Behaviour across a daylight-saving transition**, where local midnight is not 24 hours after the
  previous one.
- **Whether the fractional part is exactly zero**, rather than merely displaying as a date.

As with `NOW`, none of these yields a vector suite in the ordinary sense, and the honest published
state for this entry is that its correctness question is about the host clock and the serial
mapping, not about arithmetic.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| volatile | Recalculated on every workbook recalculation, regardless of input changes |
| time-dependent | The result depends on when the call happens; no input determines it |
| stability is not determinism | A value constant for a day still has no input to pair with in a vector |
| time provider | The injected clock an implementation should depend on instead of the system clock |
| presentation hint | A core value carrying a formatting suggestion applied by the host |

## Sources

- Microsoft 365 support, **TODAY function** —
  <https://support.microsoft.com/en-us/office/today-function-5eb3078d-a82c-4736-8930-2f51a028fdd9>.
- Microsoft 365 support, **NOW function** —
  <https://support.microsoft.com/en-us/office/now-function-3337fd29-145a-4347-b2e6-20c904739c46>.
- Handbook call-model chapters [01 value universe](../model/01-value-universe.md) and
  [03 call pipeline](../model/03-call-pipeline.md), which name `NOW` and `TODAY` as the documented
  host-side-adaptation seed examples.
- [FUNC.DATE](FUNC.DATE.md) — the serial model whose value `TODAY` reports.
- `data/functions/FUNC.TODAY.json` (the `TimeDependent` / `VolatileFull` / `ApplicationState` /
  `HostSerialized` / `TimeProvider` classification), `data/presence/FUNC.TODAY.json`,
  `data/battery/FUNC.TODAY.json` (the declined-call rows).
