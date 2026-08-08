---
schema: efh.function-page/v1
function_id: FUNC.AVERAGEA
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - Arguments
  - The admission table, side by side with AVERAGE
  - Result and edge cases
  - Errors
  - Documentation divergences
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: averagea_fn
role_in_family: >-
  The wide-admission mean: the same arithmetic as AVERAGE, but text and logicals inside ranges
  become data instead of being skipped, which changes the denominator as well as the numerator.
---

# AVERAGEA

## What it computes

`AVERAGEA(value1, [value2], …)` returns the arithmetic mean

    AVERAGEA = (1/n) · Σ_{i=1..n} v_i

over a **wider set of admitted values** than [AVERAGE](FUNC.AVERAGE.md). The arithmetic is
identical; the function is defined by its admission rule, not by its formula.

The rule, as the reference engine implements it at commit `473efa3`: numbers count as
themselves; logical values count as 1 or 0; **text inside a range or array counts as 0**,
including the empty string; empty cells and omitted slots still contribute nothing. Microsoft's
page states the same rule twice over — "Arguments that contain TRUE evaluate as 1; arguments
that contain FALSE evaluate as 0 (zero)" and "Array or reference arguments that contain text
evaluate as 0 (zero). Empty text ("") evaluates as 0 (zero)" — and then, in the very next
bullet, states the opposite. See the divergences section.

The consequence is worth stating baldly, because it is the entire reason the function exists
and the entire reason it surprises people: text does not merely join the sum as zero, it joins
the **count**. A column holding `10`, `20` and the word `"n/a"` averages to 15 under `AVERAGE`
and to 10 under `AVERAGEA`. Neither result is a bug. `AVERAGEA` is answering "what is the mean
if every non-empty entry is worth something, and unusable entries are worth nothing", which is
a legitimate question and a different one.

Range: as with `AVERAGE`, the result of finite inputs lies between the smallest and largest
admitted value — where the admitted values now include the zeros contributed by text.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `value1` | The first value, range or array. Required. | — |
| `value2 …` | Further values, ranges or arrays. Optional, repeating. | — |

The reference engine declares 1 to 255 slots, matching Microsoft's "1 to 255 cells, ranges of
cells, or values for which you want the average". The parameter is named `value`, not `number`,
and that naming is the signature's own signal that non-numeric kinds are expected rather than
tolerated — Microsoft's argument list admits "numbers; names, arrays, or references that
contain numbers; text representations of numbers; or logical values, such as TRUE and FALSE, in
a reference."

## The admission table, side by side with AVERAGE

| Value | `AVERAGE`, direct | `AVERAGE`, in range | `AVERAGEA`, direct | `AVERAGEA`, in range |
|---|---|---|---|---|
| Number | counted | counted | counted | counted |
| Numeric text `"2"` | converted | skipped | converted | **counted as 0** |
| Non-numeric text `"x"` | `#VALUE!` | skipped | `#VALUE!` | **counted as 0** |
| Empty string `""` | `#VALUE!` | skipped | `#VALUE!` | **counted as 0** |
| `TRUE` / `FALSE` | 1 / 0 | skipped | 1 / 0 | **1 / 0** |
| Empty cell | — | skipped | — | skipped |
| Error | propagates | propagates | propagates | propagates |

Two rows repay attention.

**Numeric-looking text in a range counts as 0, not as its numeric value.** The reference
engine's `averagea_argument_value` helper converts text only when it arrived as a direct scalar
argument; text reaching it from a reference returns `0.0` without any conversion attempt. So a
range cell holding the *text* `"2"` contributes zero, while the same `"2"` typed into the
formula contributes two. This is the sharpest edge on the function and the one that turns an
imported CSV — where numbers often arrive as text — into a silently deflated average.

**Empty cells are still skipped.** `AVERAGEA` widens the admission of *content*; it does not
turn absence into zero. The distinction between empty and zero is a value-universe distinction;
see [The value universe](../model/01-value-universe.md#missing-versus-empty).

## Result and edge cases

Returns `Number`.

- **No admitted values** — every slot empty. The reference engine returns `#DIV/0!`, matching
  `AVERAGE`.
- **A range of only text.** Under `AVERAGE` this is `#DIV/0!` (nothing admitted); under
  `AVERAGEA` it is `0` (everything admitted, all as zero). The two functions differ between an
  error and a number on the same input, which makes this the best single discriminating probe.
- **`TRUE` cells.** They contribute 1, which is almost never what a reader intends and is the
  second most common source of a wrong `AVERAGEA`.
- **Arrays.** The surface lifts natively (`lift_broadcast_profile: surface_native`) and consumes
  arrays as a reduction.
- **Very large / very small magnitudes, negative values, non-integers.** Ordinary; no domain
  guard is recorded on this surface (`arg_domain_guard=none`, `non_finite=allow`).

## Errors

The reference engine at `473efa3` produces:

| Error | Condition |
|---|---|
| `#DIV/0!` | No value was admitted |
| `#VALUE!` | A **directly-supplied** text argument could not be converted to a number |
| propagated | An error value among the admitted data becomes the result |

The `#VALUE!` row deserves its emphasis: `AVERAGEA("x")` errors, while `AVERAGEA(A1)` with `x`
in `A1` returns `0`. Widened admission applies to range-derived text only; a direct text
argument is still coerced by the ordinary rules and still fails.

Microsoft documents the propagation row as "Arguments that are error values or text that cannot
be translated into numbers cause errors", without naming a code, and names no error for the
empty-data case. The `#DIV/0!` row is therefore a reference-engine statement.

`AVERAGEA` declares `error_collapse_profile: ReductionFold` in the projection, like `AVERAGE`.

## Documentation divergences

1. **Microsoft's `AVERAGEA` page contradicts itself about text inside a reference**, in two
   adjacent bullets. One says "Array or reference arguments that contain text evaluate as 0
   (zero). Empty text ("") evaluates as 0 (zero)." The next says "If an argument is an array or
   reference, only values in that array or reference are used. Empty cells and text values in
   the array or reference are ignored." *Evaluate as zero* and *ignored* are different
   functions: the first puts text in the denominator, the second does not. On a range of three
   numbers and one text cell, the two readings differ by a factor of 4/3. The reference engine
   implements the evaluate-as-zero reading. Nobody has checked which one Excel implements, and
   the contradiction means the documentation cannot settle it.
2. **Numeric-looking text inside a reference.** The reference engine counts it as `0`, not as
   its numeric value; Microsoft's page says such text "evaluate[s] as 0 (zero)", which agrees —
   but the page's own opening argument list admits "text representations of numbers" as a kind
   of argument, which reads as though they would be converted. The evaluate-as-zero rule is the
   stronger and more specific statement, and it is what this page describes; the tension is
   recorded because it is the single most common misreading of this function.
3. **The empty-data error is undocumented**, as it is for `AVERAGE` and `AVEDEV`. Three
   neighbouring functions, three undocumented empty-data behaviours, and the reference engine
   gives two different error codes among them.

## Relationships

- **[AVERAGE](FUNC.AVERAGEA.md#the-admission-table-side-by-side-with-average)** — see the table
  above; the pair differs only in admission. Full page: [AVERAGE](FUNC.AVERAGE.md).
- **`MAXA` / `MINA` / `STDEVA` / `VARA` / `STDEVPA` / `VARPA`** — the rest of the `…A` family,
  built on the same widened admission helper. The reference engine shares the logical/text rule
  across them through a small set of helpers in `aggregate_common.rs`, so a finding about
  `AVERAGEA`'s text rule is a candidate finding about all of them — a hypothesis, not a
  transferred result.
- **`COUNT` / `COUNTA`** — the same widening, applied to counting. `AVERAGEA`'s denominator is
  closer to `COUNTA` than to `COUNT`, which is a useful way to remember it.
- **Confused with**: `AVERAGEIF` with a criterion excluding text. That is a different function
  answering the `AVERAGE` question with extra selection, not the `AVERAGEA` question.

## Numerical notes

The floating-point analysis is exactly that of [AVERAGE](FUNC.AVERAGE.md#numerical-notes) —
sequential summation, a condition number of \(\sum|v_i| / |\sum v_i|\), and one final division —
with one addition that is specific to this surface.

The zeros injected by text are **exact** zeros, and adding an exact zero to a running sum is
exact in IEEE arithmetic (barring the signed-zero corner, which does not arise for a
non-negative count). So text contributes no rounding error to the numerator at all; its entire
effect is on \(n\). That has a pleasant consequence for anyone building a reference
implementation: `AVERAGEA` cannot be *less* accurate than `AVERAGE` over the same numeric
subset for reasons of summation — only for reasons of what was counted. Any observed
divergence between the two beyond the denominator is therefore evidence about admission, not
about arithmetic, which makes the pair a clean differential probe.

The logicals are exact too (1.0 and 0.0 are exactly representable). The only inexactness on
this surface is the same summation and division that `AVERAGE` has.

The reference engine accumulates left to right in a plain `f64` and divides once. Excel's
internal method is not asserted here.

## What has not been checked

**Nobody has checked this function against Excel within the Handbook's record.** No Handbook
vector suite exists for `AVERAGEA`, no evidence record lists this surface among its subjects,
and `data/presence/FUNC.AVERAGEA.json` records no mention of it in any defect stream,
discrepancy catalogue or exactness register. Microsoft's page was fetched while this page was
written; it supplies the admission rules quoted above, and it contradicts itself on the central
one. Everything else is mathematics or a named statement about the reference engine at commit
`473efa3`.

Inputs worth probing first:

1. **A range of three numbers and one text cell.** This settles divergence 1 — the
   evaluate-as-zero reading and the ignored reading differ by the denominator, and the answer
   is unambiguous. Highest-value probe on the page, because Microsoft's own documentation
   cannot answer it.
2. **A range holding the text `"2"`.** If the answer treats it as two rather than zero, the
   reference engine's model is wrong in a way that would propagate to every `…A` sibling.
3. **A range of only text.** `AVERAGE` should error and `AVERAGEA` should return `0` under the
   evaluate-as-zero reading; under the ignored reading both error. The pair discriminates the
   two admission rules in one shot.
3. **`AVERAGEA("x")` versus `AVERAGEA(A1)` with `x` in `A1`** — the direct-versus-range
   asymmetry of the `#VALUE!` branch.
4. **The empty string.** A cell holding `=""` and a cell holding a literal empty string are
   both non-empty content that displays as nothing; whether either counts as an admitted zero
   is the subtlest question here.
5. **`TRUE` and `FALSE` cells mixed with numbers**, to confirm 1/0 rather than skip.
6. **A genuinely empty cell inside the range**, to confirm the widening stops at content and
   does not reach absence.
7. **The same summation battery as `AVERAGE`** — mixed-magnitude and cancelling data — run
   through both functions, so that any difference is attributable to \(n\) alone.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| widened admission | The `…A` rule: range text counts as 0 and logicals count as 1/0 |
| admitted value | A value that entered both the sum and the count |
| direct scalar | A value written literally at the call site rather than reached through a reference |
| differential probe | A pair of calls whose difference isolates one rule (here, `AVERAGE` versus `AVERAGEA`) |
| reference engine | OxFunc at commit `473efa3` |

## Sources

- Microsoft, "AVERAGEA function" —
  <https://support.microsoft.com/en-us/office/averagea-function-f5f84098-d453-4f4c-bbba-3d2c66356091>
  (the one-line description; the `value1, value2, …` argument text and 1-to-255 limit; the
  admissible-argument list; the directly-typed-values-are-counted rule; TRUE-as-1 and
  FALSE-as-0; the array-or-reference-text-evaluates-as-0 and empty-text rules; the
  contradicting "text values … are ignored" bullet; the error-causing-arguments sentence; and
  the pointer to `AVERAGE`).
- Handbook projection `data/functions/FUNC.AVERAGEA.json` (signature, arity 1–255,
  `AggregateDirectAndRangeDualPolicy`, `ReductionFold`, the `microsoft-english-verbatim`
  one-line description, and the Microsoft documentation address) and
  `data/presence/FUNC.AVERAGEA.json`.
- OxFunc `crates/oxfunc_core/src/functions/averagea_fn.rs` and the
  `averagea_argument_value` helper in
  `crates/oxfunc_core/src/functions/aggregate_common.rs` at commit `473efa3` — the text-to-zero
  and logical-to-1/0 branches, and the direct-scalar exception that keeps `#VALUE!` alive.
- Handbook, [AVERAGE](FUNC.AVERAGE.md) — the shared arithmetic and its error analysis.
- Handbook, [The value universe](../model/01-value-universe.md) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md`.
