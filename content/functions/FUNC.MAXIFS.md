---
schema: efh.function-page/v1
function_id: FUNC.MAXIFS
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - Arguments
  - The criteria language
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: criteria_family
role_in_family: >-
  The conditional maximum: the criteria family's selection stage followed by a max reduction,
  sharing the family's comparison predicate rather than MAX's.
---

# MAXIFS

## What it computes

`MAXIFS(max_range, criteria_range1, criteria1, [criteria_range2, criteria2], …)` returns the
largest value among the cells of `max_range` whose correspondingly-positioned cells in every
criteria range satisfy the matching criterion.

Written out, with `i` ranging over positions in the common shape:

    S = { i : P₁(criteria_range1[i]) ∧ P₂(criteria_range2[i]) ∧ … ∧ P_k(criteria_range_k[i]) }
    MAXIFS = max { max_range[i] : i ∈ S, max_range[i] is numeric }

Two structural facts:

1. **The conditions are combined with AND, positionally.** Criteria pairs intersect; there is
   no OR form and no way to write one inside a single call. Position, not value, is what links
   a criteria cell to a `max_range` cell — which is why the shapes must match.
2. **`MAXIFS` is a selection stage followed by a reduction.** Everything about the selection is
   shared with the rest of the criteria family; everything about the reduction is shared with
   [MAX](FUNC.MAX.md). The two halves are worth thinking about separately, because they carry
   different risks — the selection carries a comparison-tolerance question, the reduction
   carries the empty-set question.

Microsoft documents the size requirement directly: "The size and shape of the max_range and
criteria_rangeN arguments must be the same, otherwise these functions return the #VALUE!
error."

**Domain and result.** `max_range` may hold anything; only its numeric cells participate in the
maximum. The result is a real number. When `S` is empty — or when no cell of `max_range` at a
selected position is numeric — Microsoft documents that the function returns `0`.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `max_range` | "The actual range of cells in which the maximum will be determined." | Yes |
| `criteria_range1` | "Is the set of cells to evaluate with the criteria." | Yes |
| `criteria1` | "Is the criteria in the form of a number, expression, or text that defines which cells will be evaluated as maximum." | Yes |
| `criteria_range2, criteria2, …` | "Additional ranges and their associated criteria. You can enter up to 126 range/criteria pairs." | No |

The reference engine declares an arity of 3 to 255 and classifies the surface as
`RefsVisibleInAdapter` — it receives live references rather than only resolved values, which is
what positional correspondence over ranges requires. Note that arity 255 and "126 range/criteria
pairs" are the same statement counted differently: `1 + 2×126 = 253`, plus headroom.

**Ranges must match in shape, not merely in count.** A `3×1` `max_range` against a `1×3`
criteria range is a shape mismatch even though both hold three cells.

**One identity fact worth recording.** The Handbook's projection carries **no XLL entry point**
for `MAXIFS` — the `xlcall` field is absent, where every classic statistical function on
neighbouring pages carries one (`xlfMax` is 7, `xlfMedian` 227, and so on). That is consistent
with `MAXIFS` being one of the post-2007 additions reached through the `_xlfn.` future-function
mechanism rather than through a legacy function code, and it is why the function name appears
as `_xlfn.MAXIFS` when a file is opened in a version that does not have it. Microsoft's page
gives the availability list: Excel for Microsoft 365, Excel 2024, Excel 2021 and Excel 2019.

## The criteria language

The criteria argument is the family's shared mini-language, and Microsoft's page says so
explicitly: the same criteria format works across `MINIFS`, `SUMIFS` and `AVERAGEIFS`. The
Handbook documents the language once, on the criteria-family pages; the parts that bear on
`MAXIFS` specifically are:

- A bare number or text is an equality test.
- A text criterion may begin with a comparison operator — `">5"`, `"<>x"`, `">="&A1` — and the
  operator is parsed out of the string.
- Wildcards `?` and `*` apply to text criteria, with `~` as the escape.
- Microsoft documents that a criteria argument referring to an empty cell is **treated as 0**.
  That is a sharp edge: `MAXIFS(B:B, A:A, C1)` with `C1` blank does not mean "no filter", it
  means "equal to zero".

The comparison used to decide whether a criteria cell matches is **not** raw IEEE equality.
The reference engine's presence projection names the upstream defect stream `BUG-FUNC-004`
(numeric comparison tolerance family split) against this module, which records a shared
truncation-style 15-significant-digit comparison helper for Excel's tolerant comparison
families. The consequence for `MAXIFS` is concrete: whether a cell holding `0.1+0.2` satisfies
the criterion `0.3` decides whether its value enters the maximum at all. The Handbook has not
verified which side of that split `MAXIFS` falls on; the defect stream is the reason to think
the question is live rather than academic.

## Result and edge cases

Returns `Number`.

- **No cell meets the criteria.** Documented: the function returns `0`. The reference engine's
  own battery — OxFunc's answers, no Excel involved — returns zero for its blank and
  no-match rows, consistent with the documented rule. Note the trap this creates on
  all-negative data: a `MAXIFS` of `0` means either "the maximum matching value is zero" or
  "nothing matched", and the two are indistinguishable. Guarding with
  `COUNTIFS(...)` on the same criteria is the standard remedy.
- **Non-numeric cells at selected positions** are not part of a numeric maximum. Whether they
  are skipped or cause an error is not stated on Microsoft's page for this function, and the
  Handbook has not checked.
- **Shape mismatch** → `#VALUE!`, documented.
- **Whole-column ranges** (`A:A` against `B:B`) match in shape and are the common idiom;
  performance is an engine matter the Handbook does not assert on.
- **Inline array literals in the range positions.** The reference engine's battery returns an
  *array* of `#VALUE!` values for a two-dimensional inline array literal — that is, the call
  is lifted elementwise over the array rather than treating it as a range, and each element
  then fails. Microsoft's page describes the arguments as ranges of cells throughout and does
  not document array-literal behaviour, so this is a **documentation gap rather than a
  documented divergence**, but it is a behaviour a caller can reach and it is recorded here as
  a finding.
- **Error values** in `max_range` or a criteria range propagate; the reference engine declares
  a `ReductionFold` error-collapse profile with a canonical legacy error algebra, so which
  error survives among several is a defined but unverified rule.
- **Criteria referring to an empty cell** are treated as `0`, documented — see above.

## Errors

| Error | Condition | Basis |
|---|---|---|
| `#VALUE!` | `max_range` and any `criteria_rangeN` differ in size or shape | Documented on Microsoft's `MAXIFS` page |
| propagated | An error value in a participating cell | Shared coercion rule, chapter 02; fold rule from the reference engine's classification |

Microsoft's page documents exactly one error condition. It does not document a `#NUM!`, a
`#N/A`, or an error for the no-match case — the no-match answer is `0`.

## Relationships

- **[MINIFS](FUNC.MINIFS.md)** — the exact mirror, same arguments, same criteria language, same
  documented `#VALUE!` and same documented `0` for no match. They share an implementing module
  in the reference engine.
- **[MAX](FUNC.MAX.md)** — the unconditional maximum. The two differ in more than the filter:
  `MAX` takes a variadic value list and applies the direct-versus-scan coercion split, while
  `MAXIFS` takes ranges only and applies the criteria family's comparison rules. They are not
  the same reduction wearing a filter.
- **[SUMIFS](FUNC.SUMIFS.md)**, **[AVERAGEIFS](FUNC.AVERAGEIFS.md)**,
  **[COUNTIFS](FUNC.COUNTIFS.md)** — the rest of the multi-criteria family, sharing the
  selection stage. `COUNTIFS` with the same criteria is the standard way to distinguish "no
  match" from "the maximum is zero".
- **[SUMIF](FUNC.SUMIF.md)**, **[AVERAGEIF](FUNC.AVERAGEIF.md)**, **[COUNTIF](FUNC.COUNTIF.md)**
  — the single-criterion ancestors, whose argument order differs (range first, then criterion,
  then the sum range last) from the `…IFS` order (result range first). That reversal is the
  most common editing mistake when converting between them, and it is silent when the ranges
  happen to be the same shape.
- **[DMAX](FUNC.DMAX.md)** — the database-family conditional maximum, using a criteria *range*
  with headers instead of range/criterion pairs. `DMAX` can express OR, which `MAXIFS` cannot.
- **`MAX(IF(…))`** — the legacy array-formula idiom for the same job, which uses ordinary
  worksheet equality rather than the criteria language and therefore does **not** always agree
  with `MAXIFS`. Where they disagree, the criteria language's comparison rules are usually the
  reason.
- **[FILTER](FUNC.FILTER.md)** with `MAX` — the modern dynamic-array route, which does support
  OR and arbitrary boolean expressions, and which uses ordinary comparison operators rather
  than criteria strings.

## Numerical notes

The reduction half of `MAXIFS` has no arithmetic and no rounding error, exactly as
[MAX](FUNC.MAX.md) does not. The numerical substance is entirely in the **selection** half, and
it is genuinely substantial.

**The comparison predicate is the whole story.** A numeric criterion has to decide whether a
stored double equals, exceeds or falls below a target double. Three predicates are plausible
and they disagree on real data:

1. **Raw IEEE comparison** — `0.1+0.2 = 0.3` is FALSE, and the cell is excluded.
2. **Truncation to 15 significant decimal digits before comparing** — the behaviour the
   upstream defect stream `BUG-FUNC-004` records for Excel's tolerant comparison families. Under
   this rule `0.1+0.2` and `0.3` are equal and the cell is included.
3. **A relative-epsilon comparison** — plausible-looking, and not what the record describes.

The difference is not a last-bit matter: it changes *which cells are in the population*, and
therefore can change the returned maximum by an arbitrary amount. A criterion `">=0.3"` applied
to a column containing `0.1+0.2` either admits a row or does not, and if that row holds the
largest value the two answers differ completely. **This is the reason `MAXIFS` belongs on a
numerical-behaviour page at all.**

A second-order point: because the criteria strings are *parsed*, a criterion built by
concatenation — `">="&A1` — goes through a number-to-text-to-number round trip. Excel's
general number formatting renders at most 15 significant digits, so a threshold that needs the
16th or 17th digit to be represented exactly cannot survive the round trip. The safe idiom
passes the comparison operator and the value separately where the function allows it, or uses
a helper column of booleans; the criteria language does not offer a typed alternative.

**The empty-set convention.** As on `MAX`, the documented answer for no match is `0`, which is
inside the range of plausible data rather than below it. Chained or hierarchical `MAXIFS`
computations over all-negative data inherit the same associativity trap described on the
[MAX](FUNC.MAX.md) page: the top-level answer can be `0` when every underlying value is
negative.

## What has not been checked

No evidence record lists `FUNC.MAXIFS` among its subjects, and no count in the Handbook's
evidence layer touches this surface. Nobody has checked `MAXIFS` against Excel within the
Handbook's record. No Handbook vector suite exists.

Microsoft's `MAXIFS` page was retrieved for this curation pass, so the syntax, the shape rule,
the `#VALUE!` condition, the no-match `0`, the empty-criteria-as-zero rule, the 126-pair limit
and the availability list above are quoted documentation rather than recollection.

The presence projection at commit `473efa3` places this surface in `criteria_family.rs`, shared
with `AVERAGEIF`, `AVERAGEIFS`, `COUNTIF`, `COUNTIFS`, `MINIFS`, `SUMIF` and `SUMIFS`, and
names the upstream defect stream `BUG-FUNC-004` on the module. Being named in a module-level
defect stream is not a per-surface finding: it says the family's comparison predicate is a
known live question, not that `MAXIFS` has been measured.

One behaviour recorded as a finding: the reference engine's own battery lifts the call
elementwise over a two-dimensional inline array literal in the range positions, producing an
array of errors. Microsoft's page describes these arguments as ranges of cells and documents no
array-literal behaviour, so this is a documentation gap; whether Excel does the same is
unchecked.

Inputs I would probe first, and why:

1. **A `max_range` column containing `0.3` and `0.1+0.2`, with criteria `"=0.3"`, `">=0.3"` and
   `"<0.3"` in turn.** This is the comparison-predicate probe and it is the single most
   consequential unknown on the page — it decides population membership, not a last bit.
2. **`MAXIFS` against `MAX(IF(...))`** on the same data and the same threshold. Where they
   disagree, the criteria language's predicate is exposed without needing to reason about it.
3. **A criterion built by concatenation from a cell holding a 17-significant-digit value**,
   against the same criterion typed literally — the text round-trip loss.
4. **No matching row, on all-negative data**, confirming the documented `0` and demonstrating
   that it is indistinguishable from a genuine zero maximum. Paired with `COUNTIFS` on the same
   criteria as the recommended guard.
5. **A criteria argument pointing at a blank cell**, confirming that it means "equal to zero"
   rather than "no filter" — documented, and the behaviour people are most often surprised by.
6. **Shape mismatch in three flavours**: same count different orientation, different count, and
   a criteria range one row longer than `max_range`. Only the documented `#VALUE!` is expected;
   confirming it for all three pins the rule.
7. **Non-numeric cells at matching positions** — text, logicals, blanks — to find whether they
   are skipped or fatal, which Microsoft's page does not say.
8. **Wildcards in a text criterion**, including a literal `*` escaped with `~`, since the
   criteria language is shared and any divergence would be family-wide.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| criteria pair | A `criteria_rangeN` / `criteriaN` pair; pairs combine with AND |
| positional correspondence | Criteria cells match `max_range` cells by position, which is why shapes must agree |
| criteria language | The shared mini-language of comparison prefixes, wildcards and literals |
| tolerant comparison | The truncation-style 15-significant-digit predicate the family is recorded as using |
| no-match answer | The documented `0` returned when the selected set is empty |
| future function | A post-2007 addition reached through `_xlfn.`, with no legacy XLL function code |

## Sources

- Microsoft, "MAXIFS function" —
  <https://support.microsoft.com/en-us/office/maxifs-function-dfd611e6-da2c-488a-919b-9b6376b28883>
  — retrieved for this curation pass. Source of the syntax, every argument description quoted
  above, the size-and-shape rule and its `#VALUE!`, the shared criteria format across `MINIFS`,
  `SUMIFS` and `AVERAGEIFS`, the no-match `0`, the empty-criteria-as-zero rule, the 126-pair
  limit, and the version availability list.
- Handbook [MAX](FUNC.MAX.md) — the unconditional reduction, the empty-set convention and its
  associativity trap.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md) — the family-policy
  treatment of comparison and scanning.
- OxFunc defect stream `BUG-FUNC-004` (numeric comparison tolerance family split), named on
  this surface's implementing module in the presence projection — the record that makes the
  comparison predicate a live question.
- Handbook projections `data/functions/FUNC.MAXIFS.json` (arity 3–255, **no `xlcall` entry**,
  `RefsVisibleInAdapter`, `ReductionFold`, `SequentialLeftFold`) and
  `data/presence/FUNC.MAXIFS.json` (module `criteria_family.rs`, shared with `AVERAGEIF`,
  `AVERAGEIFS`, `COUNTIF`, `COUNTIFS`, `MINIFS`, `SUMIF`, `SUMIFS`).
