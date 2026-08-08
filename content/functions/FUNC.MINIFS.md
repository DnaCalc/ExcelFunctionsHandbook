---
schema: efh.function-page/v1
function_id: FUNC.MINIFS
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
  The conditional minimum: the criteria family's selection stage followed by a min reduction,
  and the member where the documented zero-on-no-match is most likely to be mistaken for data.
---

# MINIFS

## What it computes

`MINIFS(min_range, criteria_range1, criteria1, [criteria_range2, criteria2], …)` returns the
smallest value among the cells of `min_range` whose correspondingly-positioned cells in every
criteria range satisfy the matching criterion.

With `i` ranging over positions in the common shape:

    S = { i : P₁(criteria_range1[i]) ∧ … ∧ P_k(criteria_range_k[i]) }
    MINIFS = min { min_range[i] : i ∈ S, min_range[i] is numeric }

Two structural facts:

1. **Criteria pairs combine with AND, positionally.** There is no OR form inside a single call.
   The link between a criteria cell and a `min_range` cell is position, which is why the shapes
   must agree. Microsoft's page adds a clarification worth quoting: the ranges "don't need to be
   aligned, only the same shape/size" — they may sit anywhere on the sheet, and correspondence
   is by offset within each range, not by row number.
2. **`MINIFS` is a selection stage followed by a reduction.** The selection is shared with the
   whole criteria family and carries a comparison-predicate question; the reduction is shared
   with [MIN](FUNC.MIN.md) and carries the empty-set question. The two halves fail differently
   and are worth separating.

**Domain and result.** `min_range` may hold anything; only its numeric cells participate. When
`S` is empty, Microsoft documents that the function returns `0`.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `min_range` | "The actual range of cells in which the minimum value will be determined." | Yes |
| `criteria_range1` | "Is the set of cells to evaluate with the criteria." | Yes |
| `criteria1` | "Is the criteria in the form of a number, expression, or text that defines which cells will be evaluated as minimum." | Yes |
| `criteria_range2, criteria2, …` | Additional range/criteria pairs; up to 126 pairs | No |

The reference engine declares an arity of 3 to 255 and classifies the surface as
`RefsVisibleInAdapter` — it receives live references, which positional correspondence over
ranges requires.

**Shape, not count.** A `3×1` `min_range` against a `1×3` criteria range is a mismatch even
though both hold three cells. Microsoft documents `#VALUE!` for that case.

**One identity fact worth recording.** The Handbook's projection carries **no XLL entry point**
for `MINIFS` — the `xlcall` field is absent, where the classic statistical functions on
neighbouring pages all carry one (`xlfMin` is 6, `xlfMina` 363). That is consistent with
`MINIFS` being a post-2007 addition reached through the `_xlfn.` future-function mechanism
rather than a legacy function code, and it is why the name appears as `_xlfn.MINIFS` when a file
is opened in a version that lacks it. Microsoft's page gives the availability list: Excel for
Microsoft 365, Excel 2024, Excel 2021 and Excel 2019, on Windows and Mac.

## The criteria language

The criteria argument is the family's shared mini-language, and Microsoft's page says the
matching works consistently with `MAXIFS`, `SUMIFS` and `AVERAGEIFS`. The parts that bear on
`MINIFS` specifically:

- A bare number or text is an equality test.
- A text criterion may begin with a comparison operator — `">5"`, `"<>x"`, `"<="&A1` — which is
  parsed out of the string.
- Wildcards `?` and `*` apply to text criteria, with `~` as the escape.
- **An empty criteria cell is treated as zero**, documented. `MINIFS(B:B, A:A, C1)` with `C1`
  blank does not mean "no filter"; it means "equal to zero".

The predicate used to decide a numeric match is **not** raw IEEE equality. The presence
projection names the upstream defect stream `BUG-FUNC-004` (numeric comparison tolerance family
split) against this surface's implementing module, which records a shared truncation-style
15-significant-digit comparison helper for Excel's tolerant comparison families. For `MINIFS`
the consequence is concrete: whether a cell holding `0.1+0.2` satisfies a criterion of `0.3`
decides whether its value enters the minimum at all. The Handbook has not verified which side
of that split `MINIFS` falls on; the defect stream is why the question is live.

## Result and edge cases

Returns `Number`.

- **No cell meets the criteria.** Documented: returns `0`. The reference engine's own battery —
  OxFunc's answers, no Excel involved — returns zero for its blank and no-match rows,
  consistent with the documented rule.

  **This convention is more dangerous on `MINIFS` than on [MAXIFS](FUNC.MAXIFS.md), for the
  same reason it is more dangerous on `MIN` than on `MAX`.** Most spreadsheet data is positive.
  A `MINIFS` of `0` over positive data means either "nothing matched" or "the smallest matching
  value is zero", and the two are indistinguishable — but the first is far more common, and the
  `0` is *smaller than every real value*, so it wins every downstream comparison, ranking and
  conditional format. The standard guard is
  `IF(COUNTIFS(<same criteria>)=0, "none", MINIFS(...))`, and it is worth writing every time
  the data are strictly positive.

- **Non-numeric cells at selected positions.** Whether they are skipped or fatal is not stated
  on Microsoft's page for this function, and the Handbook has not checked.
- **Shape mismatch** → `#VALUE!`, documented.
- **Whole-column ranges** match in shape and are the common idiom.
- **Inline array literals in the range positions.** The reference engine's battery returns an
  *array* of `#VALUE!` values for a two-dimensional inline array literal — the call is lifted
  elementwise over the array rather than treated as a range, and each element then fails.
  Microsoft's page describes the arguments as ranges of cells throughout and documents no
  array-literal behaviour, so this is a **documentation gap rather than a documented
  divergence**; it is a behaviour a caller can reach, and it is recorded here as a finding.
- **Error values** in `min_range` or a criteria range propagate; the reference engine declares a
  `ReductionFold` error-collapse profile with a canonical legacy error algebra, so which error
  survives among several is a defined but unverified rule.
- **Criteria referring to an empty cell** are treated as `0`, documented.

## Errors

| Error | Condition | Basis |
|---|---|---|
| `#VALUE!` | `min_range` and any `criteria_rangeN` differ in size or shape | Documented on Microsoft's `MINIFS` page |
| propagated | An error value in a participating cell | Shared coercion rule, chapter 02; fold rule from the reference engine's classification |

Microsoft's page documents exactly one error condition. There is no documented `#NUM!`, no
`#N/A`, and no error for the no-match case — that answer is `0`.

## Relationships

- **[MAXIFS](FUNC.MAXIFS.md)** — the exact mirror: same arguments, same criteria language, same
  documented `#VALUE!` and `0`. The two share an implementing module in the reference engine,
  so any behavioural difference beyond the comparator is a defect in one of them.
- **[MIN](FUNC.MIN.md)** — the unconditional minimum. The two differ in more than the filter:
  `MIN` takes a variadic value list and applies the direct-versus-scan coercion split, while
  `MINIFS` takes ranges only and applies the criteria family's comparison rules.
- **[SUMIFS](FUNC.SUMIFS.md)**, **[AVERAGEIFS](FUNC.AVERAGEIFS.md)**,
  **[COUNTIFS](FUNC.COUNTIFS.md)** — the rest of the multi-criteria family, sharing the
  selection stage. `COUNTIFS` on the same criteria is the standard way to tell "no match" from
  "the minimum is zero".
- **[SUMIF](FUNC.SUMIF.md)**, **[AVERAGEIF](FUNC.AVERAGEIF.md)**, **[COUNTIF](FUNC.COUNTIF.md)**
  — the single-criterion ancestors, with the *opposite* argument order: criteria range first,
  result range last, where the `…IFS` functions put the result range first. Converting between
  them is the most common silent editing mistake in the family.
- **[DMIN](FUNC.DMIN.md)** — the database-family conditional minimum, using a headed criteria
  range instead of range/criterion pairs. `DMIN` can express OR; `MINIFS` cannot.
- **`MIN(IF(…))`** — the legacy array-formula idiom, which uses ordinary worksheet equality
  rather than the criteria language and therefore does not always agree with `MINIFS`.
- **[FILTER](FUNC.FILTER.md)** with `MIN` — the modern dynamic-array route, which supports OR
  and arbitrary boolean expressions and uses ordinary comparison operators.

## Numerical notes

The reduction half has no arithmetic and no rounding error, exactly as [MIN](FUNC.MIN.md) does
not. The numerical substance is entirely in the **selection** half.

**The comparison predicate is the whole story.** A numeric criterion must decide whether a
stored double equals, exceeds or falls below a target. Three predicates are plausible and they
disagree on real data:

1. **Raw IEEE comparison** — `0.1+0.2 = 0.3` is FALSE and the cell is excluded.
2. **Truncation to 15 significant decimal digits before comparing** — the behaviour the upstream
   defect stream `BUG-FUNC-004` records for Excel's tolerant comparison families. Under this
   rule the two are equal and the cell is included.
3. **A relative-epsilon comparison** — plausible-looking, and not what the record describes.

The difference is not a last-bit matter. It changes **which cells are in the population**, and
therefore can change the returned minimum by an arbitrary amount — including changing it from a
real value to the no-match `0`, which is the worst outcome available. A criterion `">=0.3"`
applied to a column containing `0.1+0.2` either admits a row or does not, and if that row held
the only match, the answer flips from a number to a zero that looks like data.

**The text round trip.** Criteria strings are parsed, so a criterion built by concatenation —
`">="&A1` — passes the threshold through number-to-text-to-number. Excel's general number
formatting renders at most 15 significant digits, so a threshold needing the 16th or 17th digit
cannot survive. The criteria language offers no typed alternative; a helper column of booleans
does.

**The empty-set convention.** As on `MIN`, the documented no-match answer is `0`, which is a
*lower* bound for most real data rather than an upper one. Chained or hierarchical `MINIFS`
computations inherit the associativity trap described on the [MIN](FUNC.MIN.md) page in its
sharpest form: a top-level minimum of `0` over strictly positive data almost always means an
empty selection somewhere below, and almost never means what it appears to mean.

## What has not been checked

No evidence record lists `FUNC.MINIFS` among its subjects, and no count in the Handbook's
evidence layer touches this surface. Nobody has checked `MINIFS` against Excel within the
Handbook's record. No Handbook vector suite exists.

Microsoft's `MINIFS` page was retrieved for this curation pass, so the syntax, the shape rule
and its `#VALUE!`, the no-match `0`, the empty-criteria-as-zero rule, the "need not be aligned"
clarification, the 126-pair limit and the availability list are quoted documentation rather
than recollection.

The presence projection at commit `473efa3` places this surface in `criteria_family.rs`, shared
with `AVERAGEIF`, `AVERAGEIFS`, `COUNTIF`, `COUNTIFS`, `MAXIFS`, `SUMIF` and `SUMIFS`, and names
the upstream defect stream `BUG-FUNC-004` on the module. Being named in a module-level defect
stream is not a per-surface finding: it says the family's comparison predicate is a known live
question, not that `MINIFS` has been measured.

One behaviour recorded as a finding: the reference engine's own battery lifts the call
elementwise over a two-dimensional inline array literal in the range positions, producing an
array of errors. Microsoft's page describes these arguments as ranges of cells and documents no
array-literal behaviour, so this is a documentation gap; whether Excel does the same is
unchecked.

Inputs I would probe first, and why:

1. **A `min_range` column containing `0.3` and `0.1+0.2`, with criteria `"=0.3"`, `">=0.3"` and
   `"<0.3"` in turn.** This is the comparison-predicate probe and the single most consequential
   unknown on the page: it decides population membership, and on `MINIFS` an empty population
   returns a zero that outranks every real value.
2. **`MINIFS` against `MIN(IF(...))`** on the same data and threshold — exposes the predicate
   without needing to reason about it.
3. **No matching row, on strictly positive data**, confirming the documented `0` and
   demonstrating that it is indistinguishable from a genuine zero minimum. Paired with
   `COUNTIFS` on the same criteria as the recommended guard.
4. **A criteria argument pointing at a blank cell**, confirming it means "equal to zero" rather
   than "no filter".
5. **A criterion built by concatenation from a cell holding a 17-significant-digit value**,
   against the same criterion typed literally — the text round-trip loss.
6. **Shape mismatch in three flavours**: same count different orientation, different count, and
   a criteria range one row longer. Only the documented `#VALUE!` is expected; confirming all
   three pins the rule.
7. **Ranges that are the same shape but in different sheet positions**, testing the documented
   "need not be aligned" clarification, which is the one rule here that a naive row-indexed
   implementation would get wrong.
8. **Non-numeric cells at matching positions** — text, logicals, blanks — which Microsoft's page
   does not address.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| criteria pair | A `criteria_rangeN` / `criteriaN` pair; pairs combine with AND |
| positional correspondence | Criteria cells match `min_range` cells by offset within each range, not by sheet row |
| criteria language | The shared mini-language of comparison prefixes, wildcards and literals |
| tolerant comparison | The truncation-style 15-significant-digit predicate the family is recorded as using |
| no-match answer | The documented `0` returned when the selected set is empty |
| dominating zero | The reason that `0` is worse here than on `MAXIFS`: it undercuts all-positive data |
| future function | A post-2007 addition reached through `_xlfn.`, with no legacy XLL function code |

## Sources

- Microsoft, "MINIFS function" —
  <https://support.microsoft.com/en-us/office/minifs-function-6ca1ddaa-079b-4e74-80cc-72eef32e6599>
  — retrieved for this curation pass. Source of the syntax, every argument description quoted
  above, the size-and-shape rule and its `#VALUE!`, the "need not be aligned" clarification, the
  shared criteria format across `MAXIFS`, `SUMIFS` and `AVERAGEIFS`, the no-match `0`, the
  empty-criteria-as-zero rule, the 126-pair limit, and the version availability list.
- Handbook [MAXIFS](FUNC.MAXIFS.md) — the mirror surface, with the same criteria language and
  the same findings.
- Handbook [MIN](FUNC.MIN.md) — the unconditional reduction, the empty-set convention and its
  directional hazard.
- Handbook [Coercion and lifting](../model/02-coercion-and-lifting.md) — the family-policy
  treatment of comparison and scanning.
- OxFunc defect stream `BUG-FUNC-004` (numeric comparison tolerance family split), named on
  this surface's implementing module in the presence projection.
- Handbook projections `data/functions/FUNC.MINIFS.json` (arity 3–255, **no `xlcall` entry**,
  `RefsVisibleInAdapter`, `ReductionFold`, `SequentialLeftFold`) and
  `data/presence/FUNC.MINIFS.json` (module `criteria_family.rs`, shared with `AVERAGEIF`,
  `AVERAGEIFS`, `COUNTIF`, `COUNTIFS`, `MAXIFS`, `SUMIF`, `SUMIFS`).
