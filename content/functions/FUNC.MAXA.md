---
schema: efh.function-page/v1
function_id: FUNC.MAXA
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
family: maxa_fn
role_in_family: >-
  MAX's text-and-logical-counting variant; the member whose documented admission rules
  contradict themselves on the text row.
---

# MAXA

## What it computes

`MAXA(value1, [value2], …)` returns the largest value in a set, where **logical values and
text are converted to numbers rather than skipped**.

The mathematics is identical to [MAX](FUNC.MAX.md): the top order statistic of the admitted
set, a selector that performs no arithmetic. The whole difference between the two functions is
the admission and conversion policy, and Microsoft's `MAXA` page states it as a set of
conversion rules:

- "Arguments that contain TRUE evaluate as 1; arguments that contain text or FALSE evaluate as
  0 (zero)."
- "Logical values and text representations of numbers that you type directly into the list of
  arguments are counted."
- "If an argument is an array or reference, only values in that array or reference are used.
  Empty cells and text values in the array or reference are ignored."
- "If the arguments contain no values, MAXA returns 0 (zero)."
- "Arguments that are error values or text that cannot be translated into numbers cause
  errors."

The consequence that matters, and the reason the function exists: because text and `FALSE`
convert to `0` rather than being skipped, **`MAXA` can never return a negative number when any
text or `FALSE` is present in the data.** A column of negative measurements with one "n/a"
typed into it has `MAXA` equal to `0`, and the `0` is not in the data. `MAXA` is therefore not
a selector in the sense `MAX` is: its result need not be one of its inputs.

That is a feature when the zeros are meaningful — "no reading" genuinely means zero — and a
trap otherwise. It is the reason `MAXA` should be chosen deliberately rather than reached for
as "MAX but more inclusive".

## Arguments

| Argument | Meaning | Notes |
|---|---|---|
| `value1` | The first value | Required |
| `value2, …` | Further values | Optional; "Number arguments 2 to 255" |

Note the parameter name: Microsoft calls these `value`, not `number`, throughout the `A`-suffix
family, precisely because non-numeric values participate. The reference engine's projection
declares the same 1-to-255 arity.

Argument boundaries carry no meaning; all slots pool into one reduction.

## The coercion table, and a contradiction in it

Read as a table, Microsoft's rules give:

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
finding rather than choosing a side.** Microsoft's own page says both of the following about
text inside a reference:

1. "Arguments that contain text or FALSE evaluate as 0 (zero)."
2. "Empty cells and text values in the array or reference are ignored."

These cannot both hold for text in a reference. If text evaluates as `0` it is counted and
drags the maximum up toward zero; if it is ignored it contributes nothing and the maximum of an
all-negative column stays negative. The two readings differ on exactly the case that
distinguishes `MAXA` from `MAX`, and they differ in the sign of the answer.

The Handbook has not observed which reading Excel implements, and does not assert one from
memory. What can be said:

- Rule 1 is the one that gives `MAXA` a reason to exist as a separate function; under rule 2
  applied to text, `MAXA` and `MAX` would differ only on logicals in references.
- The `A`-suffix family's stated purpose — Microsoft's `MAX` page points at `MAXA` as the way
  to "include logical values and text representations of numbers in a reference as part of the
  calculation" — is consistent with rule 1.
- Nothing in the Handbook's record settles it, and the probe is one cell wide. It is first on
  the list below.

## Result and edge cases

Returns `Number`. Unlike `MAX`, the result need not be one of the input values, because text
and `FALSE` are converted rather than selected.

- **No values at all.** Microsoft documents "If the arguments contain no values, MAXA returns 0
  (zero)." The reference engine's own battery — OxFunc's answers, no Excel involved — returns
  zero for its blank-argument row, which matches the documented rule.

  This is worth putting beside the [MAX](FUNC.MAX.md) page: on the *same* battery row, the
  reference engine returns zero for `MAXA` and an error for `MAX`, while Microsoft documents
  the zero for both. **The `MAXA` side matches the documentation and the `MAX` side does not.**
  That asymmetry is recorded on both pages as a divergence to be settled against Excel.

- **An all-text column.** Under rule 1 above the answer is `0`; under rule 2 it is the
  no-values case, which is also `0`. The two readings happen to coincide here, which is why
  this input does *not* discriminate between them. The discriminating input is an all-negative
  numeric column with one text cell in it.
- **A directly-passed empty string.** The reference engine's battery returns `#VALUE!` for
  that row. Microsoft's page documents that directly-passed text which cannot be translated to
  a number causes an error, so an error is the expected reading — but it sits oddly beside the
  rule that text "evaluates as 0". Another face of the same contradiction, on the direct side.
- **`TRUE` in a reference** is counted as `1` — this part of the family's purpose is not in
  dispute, and it is the cleanest observable difference from `MAX`.
- **Error values** propagate; the reference engine declares a `ReductionFold` error-collapse
  profile with a canonical legacy error algebra, so which error survives when several are
  present is a defined but unverified rule.
- **Arrays** are consumed by scanning, not lifted elementwise.

## Errors

| Error | Condition | Basis |
|---|---|---|
| `#VALUE!` | A directly-passed argument is text that cannot be translated into a number | Documented on Microsoft's `MAXA` page |
| propagated | An argument is an error value | Documented ("arguments that are error values … cause errors") |

There is no documented `#NUM!` or `#DIV/0!` condition, and no documented empty-input error —
the documented empty-input answer is `0`.

## Relationships

- **[MAX](FUNC.MAX.md)** — the numbers-only sibling, and the function whose own documentation
  names `MAXA` as the remedy for wanting logicals and numeric text counted. Read both coercion
  tables side by side before choosing; they differ on four rows.
- **[MINA](FUNC.MINA.md)** — the mirror. Note that the sign asymmetry is worse for `MINA`: text
  and `FALSE` converting to `0` puts a *ceiling* of zero on `MINA` over all-positive data,
  which is the more frequently encountered version of the trap.
- **The rest of the `A` suffix family**: [AVERAGEA](FUNC.AVERAGEA.md),
  [COUNTA](FUNC.COUNTA.md), [STDEVA](FUNC.STDEVA.md), [STDEVPA](FUNC.STDEVPA.md),
  [VARA](FUNC.VARA.md), [VARPA](FUNC.VARPA.md). These share an *intent* — count more kinds of
  value — and do **not** share one uniform rule. `COUNTA` in particular counts anything
  non-empty including text, without converting it. Each page states its own table; do not
  generalise from this one.
- **[MAXIFS](FUNC.MAXIFS.md)** — the conditional maximum, which selects the population by
  criteria and then applies `MAX`-style numeric admission, not `MAXA`-style conversion.
- **[LARGE](FUNC.LARGE.md)** — the order-statistic selector, numbers only.
- **Confused with:** `MAX`, in both directions. A workbook that switched from `MAX` to `MAXA`
  to "make it handle the text cells" has changed the answer on every column containing text,
  usually by clamping it at zero.

## Numerical notes

There is no floating-point arithmetic in `MAXA` beyond the conversions, so there is no
rounding error to analyse. Three implementation observations are worth recording anyway.

**The conversion happens before the comparison, and it is lossy.** `TRUE → 1`, `FALSE → 0`,
`text → 0` collapses an entire value kind onto a single number. Once collapsed, the maximum
cannot tell a genuine zero from a converted `FALSE` from a converted string. Any implementation
that wants to *report* which value won — and some UI layers do — has to carry the provenance
alongside the number rather than reconstructing it afterwards.

**The empty-set identity is `0`, and it is not `−∞`.** The same associativity trap described on
the [MAX](FUNC.MAX.md) page applies here and is *worse*, because `MAXA` reaches `0` through two
independent routes — the no-values rule and the text-converts-to-zero rule. A hierarchical
computation that takes `MAXA` of column `MAXA`s over all-negative data will report `0` if any
column is empty *or* contains any text. An implementer should track "have I seen an admissible
value" as a separate flag from the running best, and must not initialise the accumulator to
`0`.

**The comparison predicate and the fold order** are the same questions as for `MAX`: whether
`+0` and `−0` are distinguished in the returned bits, and which error wins in a multi-error
reduction. The reference engine records `SequentialLeftFold` for this surface, which fixes both
answers without documenting them. Because `MAXA` manufactures zeros from text and `FALSE`, the
`±0` question is slightly more likely to be observable here than in `MAX` — a converted `FALSE`
is `+0`, and a data cell could hold `−0`.

## What has not been checked

No evidence record lists `FUNC.MAXA` among its subjects, and no count in the Handbook's
evidence layer touches this surface. Nobody has checked `MAXA` against Excel within the
Handbook's record. No Handbook vector suite exists.

Microsoft's `MAXA` page was retrieved for this curation pass, so the quoted rules above are
documentation rather than recollection — which is how the contradiction in them was found. The
Handbook publishes that contradiction as a finding: **the documented rules for text inside a
reference are mutually inconsistent, and the two readings differ in the sign of the answer on
all-negative data.**

Inputs I would probe first, and why:

1. **An all-negative numeric column with one text cell in it** — for example `{−5, −3, "n/a"}`
   as a range, not as literals. Under "text evaluates as 0" the answer is `0`; under "text
   values in the reference are ignored" it is `−3`. **One cell settles the documented
   contradiction**, and no other probe on this page matters as much.
2. **The same range with `FALSE` in place of the text**, which the documentation treats
   unambiguously as `0`, to confirm the logical row and isolate the text row.
3. **`MAXA` against `MAX` on the identical range**, which turns the difference into a single
   observable rather than two readings.
4. **A blank range**, comparing `MAXA` and `MAX` — the divergence recorded on the
   [MAX](FUNC.MAX.md) page, where the reference engine's two answers differ and the
   documentation gives one.
5. **A directly-passed empty string and a directly-passed non-numeric string**, against the
   same values in cells — the direct-versus-scan asymmetry, on the row where the documentation
   is least clear.
6. **`TRUE` and `FALSE` in a reference, mixed with numbers between 0 and 1**, which is the only
   range where the logical conversion changes the answer rather than being dominated.
7. **`+0` and `−0` in a range together with a text cell**, reading the sign of the result
   through `1/MAXA(...)`, to see whether a manufactured zero or a stored `−0` wins.
8. **Two different error values in one call, in both orders**, to pin the error-fold rule.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| `A`-suffix family | `MAXA`, `MINA`, `AVERAGEA`, `COUNTA`, `STDEVA`, `STDEVPA`, `VARA`, `VARPA` — shared intent, different rules |
| conversion, not selection | Text and logicals become numbers, so the result need not be an input value |
| the text row | The contradictory documented treatment of text inside a reference |
| empty-set identity | The documented `0` returned when no values are present |
| zero clamp | The effect of text or `FALSE` converting to `0` on all-negative data |
| `ReductionFold` | The reference engine's error-collapse profile for this surface |

## Sources

- Microsoft, "MAXA function" —
  <https://support.microsoft.com/en-us/office/maxa-function-814bda1e-3840-4bff-9365-2f59ac2ee62d>
  — retrieved for this curation pass. Source of the syntax, the 1-to-255 argument count, every
  conversion rule quoted above, the "returns 0 (zero)" rule for the no-values case, and the
  error rule. Both halves of the documented contradiction are on this one page.
- Microsoft, "MAX function" —
  <https://support.microsoft.com/en-us/office/max-function-e0012414-9ac8-4b34-9a47-73e662c08098>
  — retrieved for this pass; the sentence directing readers to `MAXA` when they want logicals
  and numeric text in a reference counted.
- Handbook [MAX](FUNC.MAX.md) — the numbers-only sibling and the shared empty-set divergence
  finding.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md) — the direct-argument
  versus range-scan asymmetry.
- Handbook projections `data/functions/FUNC.MAXA.json` (arity 1–255, `xlfMaxa` code 362,
  `AggregateDirectAndRangeDualPolicy`, `ReductionFold`, `SequentialLeftFold`) and
  `data/presence/FUNC.MAXA.json` (module `maxa_fn.rs`, unshared, no defect stream named).
