---
schema: efh.function-page/v1
function_id: FUNC.MINA
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - Arguments
  - The coercion table, and a contradiction in it
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: mina_fn
role_in_family: >-
  MIN's text-and-logical-counting variant; the member where converting text to zero is most
  damaging, because most spreadsheet data is positive.
---

# MINA

## What it computes

`MINA(value1, [value2], …)` returns the smallest value in a set, where **logical values and
text are converted to numbers rather than skipped**.

The mathematics is identical to [MIN](FUNC.MIN.md): the bottom order statistic of the admitted
set. The difference is entirely in admission and conversion, which Microsoft's `MINA` page
states as:

- "Arguments can be the following: numbers; names, arrays, or references that contain numbers;
  text representations of numbers; or logical values, such as TRUE and FALSE, in a reference."
- "Arguments that contain TRUE evaluate as 1; arguments that contain text or FALSE evaluate as
  0 (zero)."
- "If an argument is an array or reference, only values in that array or reference are used.
  Empty cells and text values in the array or reference are ignored."
- "If the arguments contain no values, MINA returns 0."
- "Arguments that are error values or text that cannot be translated into numbers cause
  errors."

**The consequence that dominates this function.** Because text and `FALSE` convert to `0`
rather than being skipped, `MINA` can never exceed `0` once any text or `FALSE` is present. And
because most spreadsheet data is positive — prices, quantities, durations, distances, counts —
that is not an edge case, it is the ordinary case. A column of positive measurements with one
`"n/a"` typed into it has `MINA` equal to `0`, and the `0` appears nowhere in the data.

`MINA` is therefore **not a selector** in the sense `MIN` is: its result need not be one of its
inputs. It is the mirror of [MAXA](FUNC.MAXA.md)'s zero clamp, and it is the worse of the two,
because the direction of the clamp points into the region where real data lives.

## Arguments

| Argument | Meaning | Notes |
|---|---|---|
| `value1` | The first value | "Value1 is required" |
| `value2, …` | Further values | Optional; "1 to 255 values" |

Note the parameter name — `value`, not `number` — which the `A`-suffix family uses throughout
precisely because non-numeric values participate. The reference engine declares the matching
1-to-255 arity.

Note also that `MINA`'s page says "Value1 is required" while the [MIN](FUNC.MIN.md) page says
"Number1 is optional" in the corresponding sentence. That inconsistency is recorded as a
finding on the `MIN` page; `MINA`'s wording is the one that agrees with its own syntax line and
with the reference engine's declared arity.

Argument boundaries carry no meaning; all slots pool into one reduction.

## The coercion table, and a contradiction in it

| Value | Typed **directly** into the argument list | Reached inside an **array or reference** |
|---|---|---|
| Number | counted | counted |
| `TRUE` | counted as 1 | counted as 1 |
| `FALSE` | counted as 0 | counted as 0 |
| Text that reads as a number | counted as its numeric value | **see below** |
| Text that does not read as a number | **error** | **see below** |
| Empty cell | — | ignored |
| Error value | error | error |

**The two "see below" cells are a documented contradiction, and the Handbook publishes it as a
finding rather than choosing a side.** Microsoft's `MINA` page asserts both of the following
about text inside a reference:

1. "Arguments that contain text or FALSE evaluate as 0 (zero)."
2. "Empty cells and text values in the array or reference are ignored."

These cannot both hold. If text evaluates as `0` it is counted and drags the minimum down to
zero on all-positive data; if it is ignored, the minimum of an all-positive column stays
positive. The two readings differ on exactly the case that distinguishes `MINA` from `MIN`, and
they can differ by the entire magnitude of the data.

The same contradiction appears verbatim on the [MAXA](FUNC.MAXA.md) page, which is evidence that
it is a family-wide editorial defect in the documentation rather than a per-function slip.

What can be said without observing Excel:

- Rule 1 is the one that gives `MINA` a reason to exist as a separate function. Under rule 2
  applied to text, `MINA` and `MIN` would differ only on logicals in references.
- Microsoft's own `MIN` page points at `MINA` as the way to "include logical values and text
  representations of numbers in a reference as part of the calculation", which is consistent
  with rule 1.
- Nothing in the Handbook's record settles it. The probe is one cell wide and is first on the
  list below.

## Result and edge cases

Returns `Number`. The result need not be one of the inputs, because text and `FALSE` are
converted rather than selected.

- **No values at all.** Documented: "If the arguments contain no values, MINA returns 0." The
  reference engine's own battery — OxFunc's answers, no Excel involved — returns zero for its
  blank-argument row, matching the documented rule.

  Put this beside the [MIN](FUNC.MIN.md) page: on the *same* battery row the reference engine
  returns zero for `MINA` and an error for `MIN`, while Microsoft documents zero for both.
  **The `MINA` side matches the documentation and the `MIN` side does not.** That asymmetry is
  recorded on both pages as a divergence to be settled against Excel.

- **An all-text column.** Under rule 1 the answer is `0`; under rule 2 it is the no-values case,
  also `0`. The two readings coincide here, so this input does **not** discriminate between
  them. The discriminating input is an all-positive numeric column with one text cell.
- **A directly-passed empty string.** The reference engine's battery returns `#VALUE!` for that
  row, which matches the documented rule that directly-passed untranslatable text causes an
  error — and sits oddly beside "text … evaluates as 0". The direct-side face of the same
  contradiction.
- **`TRUE` in a reference** counts as `1`, which is the family's undisputed purpose and the
  cleanest observable difference from `MIN`. On data all greater than 1, a stray `TRUE` becomes
  the minimum.
- **Error values** propagate. The reference engine declares a `ReductionFold` error-collapse
  profile with a canonical legacy error algebra, so which error survives among several is a
  defined but unverified rule.
- **Arrays** are consumed by scanning, not lifted elementwise.

## Errors

| Error | Condition | Basis |
|---|---|---|
| `#VALUE!` | A directly-passed argument is text that cannot be translated into a number | Documented on Microsoft's `MINA` page |
| propagated | An argument is an error value | Documented ("arguments that are error values … cause errors") |

No documented `#NUM!`, `#DIV/0!` or empty-input error; the documented empty-input answer is `0`.

## Relationships

- **[MIN](FUNC.MIN.md)** — the numbers-only sibling, whose own documentation names `MINA` as
  the remedy for wanting logicals and numeric text counted, and whose page carries the same
  empty-set divergence finding. `MINA`'s page returns the favour: "To exclude logical values and
  text representations from calculations, use the MIN function instead."
- **[MAXA](FUNC.MAXA.md)** — the mirror, carrying the identical documented contradiction. The
  two pages together are the evidence that the defect is editorial and family-wide.
- **The rest of the `A` suffix family**: [AVERAGEA](FUNC.AVERAGEA.md),
  [COUNTA](FUNC.COUNTA.md), [STDEVA](FUNC.STDEVA.md), [STDEVPA](FUNC.STDEVPA.md),
  [VARA](FUNC.VARA.md), [VARPA](FUNC.VARPA.md). Shared *intent*, not one shared rule.
  `COUNTA` in particular counts non-empty values without converting them. Read each page.
- **[MINIFS](FUNC.MINIFS.md)** — the conditional minimum, which applies `MIN`-style numeric
  admission after criteria selection, not `MINA`-style conversion.
- **[SMALL](FUNC.SMALL.md)** — the order-statistic selector, numbers only, and the surface to
  reach for when "no data" must not silently become `0`.
- **Confused with:** `MIN`. A workbook switched from `MIN` to `MINA` to "handle the text cells"
  has changed the answer to `0` on every positive column containing text.

## Numerical notes

No floating-point arithmetic beyond the conversions, so no rounding error to analyse. Three
observations are worth recording.

**The conversion is lossy and happens before the comparison.** `TRUE → 1`, `FALSE → 0`,
`text → 0` collapses whole value kinds onto two numbers. After the collapse, the minimum cannot
distinguish a genuine zero from a converted `FALSE` from a converted string. Any layer that
wants to report *which cell won* must carry provenance alongside the number rather than
reconstruct it.

**The zero clamp is directional, and its direction is the bad one.** `MAXA`'s clamp only bites
on all-negative data; `MINA`'s bites on all-positive data, which is the common shape. Combined
with the no-values rule, `MINA` reaches `0` by two independent routes, so a hierarchical
`MINA`-of-`MINA`s over positive data reports `0` if **any** column is empty or contains **any**
text. An implementer must track "have I seen an admissible value" as a flag separate from the
running best and must not initialise the accumulator to `0`.

**Comparison predicate and fold order** are the same questions as on [MIN](FUNC.MIN.md):
whether `+0` and `−0` are distinguished in the returned bits, and which error wins in a
multi-error reduction. The reference engine records `SequentialLeftFold`, which fixes both
answers without documenting them. Because `MINA` manufactures zeros from text and `FALSE`, the
`±0` question is more likely to be reachable here than in `MIN` — a manufactured zero is `+0`,
and a data cell may hold `−0`, so a range mixing the two exposes the tie-break.

## What has not been checked

No evidence record lists `FUNC.MINA` among its subjects, and no count in the Handbook's
evidence layer touches this surface. Nobody has checked `MINA` against Excel within the
Handbook's record. No Handbook vector suite exists.

Microsoft's `MINA` page was retrieved for this curation pass, so the quoted rules above are
documentation rather than recollection — which is how the contradiction in them was found.

Findings from this pass:

1. **The documented rules for text inside a reference are mutually inconsistent**, and the two
   readings differ by the whole magnitude of the data on all-positive columns. The identical
   contradiction appears on the [MAXA](FUNC.MAXA.md) page.
2. **The `MINA` and `MIN` pages disagree about whether the first argument is required.**
   `MINA` says required; `MIN` says optional in the same sentence that says "1 to 255". Recorded
   on the [MIN](FUNC.MIN.md) page.
3. **The reference engine's `MIN` and `MINA` disagree on the empty case**, with `MINA` matching
   the documented `0` and `MIN` returning an error. Recorded on both pages.

Inputs I would probe first, and why:

1. **An all-positive numeric column with one text cell in it** — for example `{5, 3, "n/a"}` as
   a range, not as literals. Under "text evaluates as 0" the answer is `0`; under "text values
   in the reference are ignored" it is `3`. **One cell settles the documented contradiction**,
   and nothing else on this page matters as much.
2. **The same range with `FALSE` in place of the text**, which the documentation treats
   unambiguously as `0`, isolating the logical row from the text row.
3. **`MINA` against `MIN` on the identical range**, turning the difference into one observable.
4. **A blank range, comparing `MINA` and `MIN`** — the engine disagreement, against Excel.
5. **A directly-passed empty string and a directly-passed non-numeric string**, against the
   same values in cells — the direct-versus-scan asymmetry on the least clear row.
6. **`TRUE` in a reference alongside numbers all greater than 1**, the only configuration where
   the logical conversion determines the answer rather than being dominated.
7. **`+0` and `−0` in a range together with a text cell**, read through `1/MINA(...)`, to see
   whether a manufactured zero or a stored `−0` wins.
8. **Two different error values in one call, in both orders**, to pin the declared error fold.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| `A`-suffix family | `MINA`, `MAXA`, `AVERAGEA`, `COUNTA`, `STDEVA`, `STDEVPA`, `VARA`, `VARPA` — shared intent, different rules |
| conversion, not selection | Text and logicals become numbers, so the result need not be an input value |
| the text row | The contradictory documented treatment of text inside a reference |
| zero clamp | The effect of text or `FALSE` converting to `0`; on `MINA` it bites all-positive data |
| empty-set identity | The documented `0` returned when no values are present |
| `ReductionFold` | The reference engine's error-collapse profile for this surface |

## Sources

- Microsoft, "MINA function" —
  <https://support.microsoft.com/en-us/office/mina-function-245a6f46-7ca5-4dc7-ab49-805341bc31d3>
  — retrieved for this curation pass. Source of the syntax, the "Value1 is required" wording,
  the 1-to-255 count, every conversion rule quoted above, the "returns 0" rule for the
  no-values case, the error rule, and the pointer back to `MIN`. Both halves of the documented
  contradiction are on this one page.
- Microsoft, "MIN function" —
  <https://support.microsoft.com/en-us/office/min-function-61635d12-920f-4ce2-a70f-96f202dcc152>
  — retrieved for this pass; the pointer to `MINA` and the conflicting "Number1 is optional"
  sentence.
- Handbook [MIN](FUNC.MIN.md) — the numbers-only sibling and the shared empty-set divergence
  finding.
- Handbook [MAXA](FUNC.MAXA.md) — the mirror, carrying the identical documented contradiction.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md) — the direct-argument
  versus range-scan asymmetry.
- Handbook projections `data/functions/FUNC.MINA.json` (arity 1–255, `xlfMina` code 363,
  `AggregateDirectAndRangeDualPolicy`, `ReductionFold`, `SequentialLeftFold`) and
  `data/presence/FUNC.MINA.json` (module `mina_fn.rs`, unshared, no defect stream named).
