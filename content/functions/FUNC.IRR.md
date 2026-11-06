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
  - What it does
  - Try it
  - Quirks
  - How to implement
  - Version differences
  - Alternatives
  - Notes
  - Sources
family: cashflow_rate_family
role_in_family: >-
  The equally-spaced root finder: the rate at which a periodic cash-flow series has zero present
  value. XIRR's undated sibling and the family member whose answer is, by construction, the output
  of an iterative solver.
---

# IRR

## What it does

`IRR(values,[guess])` returns a periodic rate that makes an ordered cash-flow series’ present value zero. The first admitted flow occurs at time zero; later flows occur at equally spaced periods. Amounts can vary. Guess defaults to 0.1.

The mathematical equation is `sum(c_i/(1+r)^i)=0`, with zero-based i. A series with several sign changes can have multiple acceptable roots.

## Try it

`=IRR({-1000,1250})` returns about `0.25`: one period turns a thousand outflow into 1250 inflow.

`=IRR({-1000,0,1250})` is about `0.1180339887`. The explicit zero preserves a second period, giving `(1+r)^2=1.25`. An ignored blank would remove that position.

`=IRR({-100,230,-132},0.05)` is about `0.1`; a guess of `0.3` leads to about `0.2` in the current route. Both mathematical roots solve `100*(1+r)^2-230*(1+r)+132=0`.

## Quirks

Microsoft requires at least one positive and one negative value. It ignores text, logicals and empty cells in an array/reference, so a missing amount and numeric zero have different timing effects. Supply the flows in their intended sequence.

The current collector accepts a row or column vector and gives `#REF!` for a rectangle with both dimensions above one. A contained error becomes `#VALUE!`, whereas a whole-argument scalar error preserves its code. These are current source rules, not a claim about every Excel input route.

Microsoft documents twenty tries, accuracy within 0.00001 percent and `#NUM!` on nonconvergence. The current candidate has distinct two-flow and general routes; its later refinement and publication rules should not be mistaken for that entire private iteration schedule.

## How to implement

Collect admitted numbers in order and validate their signs. The current internal residual forms `base=1+r` with ordinary binary64 arithmetic, evaluates the runtime `crt_pow(base,i)` for each index, divides the corresponding flow and adds forward. Its domain excludes rates at/below `-0.999999999`. This residual is not the public NPV wrapper’s repeated-factor loop.

### Two admitted flows

The current route iterates in `q=1+r`, rather than returning the closed-form ratio immediately:

1. Set `current=1+guess`, `previous=1`, and `previous_value=F(0)`.
2. Evaluate `value=F(current-1)`. If it differs from previous_value, use secant step `value*(current-previous)/(value-previous_value)`. If equal, use the analytic derivative and step `value/Fprime(current-1)`; a zero derivative fails.
3. Set `next=current-step`. If its stored bits equal current, return `next-1`. Otherwise move the previous state forward and repeat, for at most twenty iterations. Failure gives `#NUM!`.

For `-1000,1250`, the mathematical q is 1.25 and returned rate one quarter. The q-space iteration and final subtraction still matter to less convenient low-bit results.

### More than two flows

The current bounded candidate starts from zero and guess, tries secant steps, then analytic Newton steps when a secant is unavailable, and otherwise halves a positive rate or restarts at 0.1. Its search permits 100 iterations with residual magnitude at most `1e-8`. A successful seed gets up to 64 analytic Newton refinements, stopping at a stored-rate fixpoint or noncontracting step.

Publication then scans sixteen adjacent binary64 rates on each side for a smaller absolute residual. It locates both ends of the contiguous equal-absolute-residual plateau by doubling the distance in ordered-rate encodings and bisecting after leaving the plateau. If that plateau spans zero it returns zero; otherwise it returns the midpoint **encoding**, decoded as a rate. This is distinct from averaging the endpoint numbers.

For negative rates, the current ordered key complements all bits of a negative binary64 value; for a nonnegative value it sets the high bit. This gives a monotone key and keeps the two zero representations adjacent. Decode with the inverse transformation. Plateau search has bounded distance probes and bisections and can fail rather than scan indefinitely.

The residual producer, root-selection basins, return plateaus and remaining recorded numerical differences remain open. Matching a small residual or a mathematical closed form alone does not identify Excel’s private solver or certify this function.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 217 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IRR.json).

## Alternatives

[XIRR](FUNC.XIRR.md) uses actual dates. [RATE](FUNC.RATE.md) assumes one repeated payment; [NPV](FUNC.NPV.md) evaluates a supplied periodic rate. [MIRR](FUNC.MIRR.md) uses separate financing and reinvestment rates.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `IRR` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-IRR-001: IRR with a direct non-number gives #NUM!, and the guess is still checked

*Quirk; measured; recorded 2026-11-04.*

Text or a logical typed directly as the values argument of IRR is not an error of type: "",
"2", "x", "0", "-2", TRUE and FALSE all give #NUM!, never #VALUE!. The guess is read anyway,
and its error wins: with any of those values, a guess of "x" gives #VALUE! and a guess of NA()
gives #N/A, while 0.1 or an empty guess leaves #NUM!. A direct error in values wins over the
guess: IRR(NA(),"x") is #N/A.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=IRR("2")` | as typed | `#NUM!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=IRR(TRUE)` | as typed | `#NUM!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=IRR("2","x")` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=IRR("2",NA())` | as typed | `#N/A` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=IRR("2",)` | as typed | `#NUM!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=IRR(NA())` | as typed | `#N/A` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=IRR(NA(),"x")` | as typed | `#N/A` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): 40 recorded readings: seven direct non-numbers and NA() as values, each with
five guesses (none, 0.1, "x", NA(), empty).

- Open: A reference to one cell holding the same values answers differently. Those readings are not in a published capture, so this note covers direct values only.

Evidence:

- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-list-controls.capture.json`, sheets ql001 to ql040 at `2d0eddcac8` — 40 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-list-controls.capture.json`, sheet ql006, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-list-controls.capture.json`, sheet ql026, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-list-controls.capture.json`, sheet ql008, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-list-controls.capture.json`, sheet ql009, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-list-controls.capture.json`, sheet ql010, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-list-controls.capture.json`, sheet ql036, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-list-controls.capture.json`, sheet ql038, row 20, column 10 at `2d0eddcac8` (readings)

<!-- efh:notes end -->

## Sources

- [Microsoft: IRR function](https://support.microsoft.com/en-us/excel/functions/irr-function), opened 7 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/cashflow_rate_family.rs); implementation steps and the limits identified in the text.
- [Remaining solver failures](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/docs/generated/problem-functions.md); source-specific counts, without a whole-function equality claim.
