---
schema: efh.function-page/v1
function_id: FUNC.AVERAGEIFS
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Differences from AVERAGEIF that are not just "more criteria"
  - Result and edge cases
  - Errors
  - Documentation divergences
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: criteria_family
role_in_family: >-
  The multi-criterion conditional mean: an AND-conjunction of independent criteria ranges over a
  value range that must match every one of them in size and shape.
---

# AVERAGEIFS

## What it computes

`AVERAGEIFS(average_range, criteria_range1, criteria1, [criteria_range2, criteria2], …)`
averages the numeric cells of `average_range` at the positions where **every** criterion holds:

    M      = { i : criteria₁(criteria_range₁[i]) ∧ … ∧ criteria_m(criteria_range_m[i]) }
    result = ( Σ_{i ∈ M, average_range[i] numeric} average_range[i] )
             / #{ i ∈ M : average_range[i] numeric }

The conjunction is positional, not relational: position \(i\) in every criteria range refers to
the same row of the same conceptual table as position \(i\) in `average_range`. This is why the
shapes must agree, and it is why `AVERAGEIFS` cannot express a join.

Microsoft states the rule in one sentence: "Each cell in average_range is used in the average
calculation only if all of the corresponding criteria specified are true for that cell."

There is no OR form. Disjunction has to be built as a sum of `AVERAGEIFS` results — which is
wrong for a mean, because means do not add — or by moving to `FILTER` or `SUMPRODUCT`. That
asymmetry (AND is a function, OR is not) is a real limitation of the whole `…IFS` family.

The criteria parsing and cell-matching stages are shared verbatim with
[AVERAGEIF](FUNC.AVERAGEIF.md#the-criteria-mechanism), `COUNTIF(S)`, `SUMIF(S)`, `MAXIFS` and
`MINIFS`; that section is the single description, and it is not repeated here.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `average_range` | The cells actually averaged. Required, and **first**. | — |
| `criteria_range1` | The first range tested. Required. | — |
| `criteria1` | The first criterion. Required. | — |
| `criteria_range2, criteria2 …` | Further range/criterion pairs. Optional, in pairs. | — |

Microsoft documents "1 to 127 ranges in which to evaluate the associated criteria" and
correspondingly "1 to 127 criteria". The reference engine at `473efa3` declares an arity of 3
to 255 slots and additionally requires the argument count after the first to be **even** —
an unpaired trailing argument is rejected structurally rather than treated as a criterion with
a default range.

The argument order is reversed relative to `AVERAGEIF`. In `AVERAGEIF` the tested range comes
first and the averaged range is an optional third argument; in `AVERAGEIFS` the averaged range
comes first and is mandatory. Rewriting one as the other by adding a criterion pair silently
produces a different formula.

## Differences from AVERAGEIF that are not just "more criteria"

| | `AVERAGEIF` | `AVERAGEIFS` |
|---|---|---|
| Averaged range | third argument, optional | first argument, required |
| Shape rule | anchored: only the top-left of `average_range` matters | **exact**: every range must be the same size and shape |
| Criteria count | exactly one | one or more, conjoined |
| Argument structure | fixed 2–3 | first argument plus range/criterion pairs |
| Documented logical rule | TRUE/FALSE cells are **ignored** | TRUE evaluates as 1, FALSE as 0 |

Microsoft draws the shape distinction explicitly: "Unlike the range and criteria arguments in
the AVERAGEIF function, in AVERAGEIFS each criteria_range must be the same size and shape as
average_range." The reference engine enforces exactly that — it flattens every range and
rejects the call with `#VALUE!` if any shape differs, with no anchoring attempt.

The documented logical rule is the row where the two Microsoft pages contradict each other; see
the divergences section.

## Result and edge cases

Returns `Number`.

- **No position satisfies every criterion** → `#DIV/0!`, documented: "If there are no cells
  that meet all the criteria, AVERAGEIFS returns the #DIV/0! error value."
- **Positions match but none holds a number** → `#DIV/0!` in the reference engine, by the same
  zero-divisor path. Microsoft documents a related condition: "If cells in average_range cannot
  be translated into numbers, AVERAGEIFS returns the #DIV0! error value."
- **A single criterion pair.** `AVERAGEIFS(B1:B10, A1:A10, ">5")` is the exact-shape analogue
  of `AVERAGEIF(A1:A10, ">5", B1:B10)`, and is the safer of the two to write precisely because
  no anchoring can occur.
- **Empty cells in `average_range`** contribute to neither sum nor count.
- **Text and logical cells in `average_range`** are skipped by the reference engine. This is
  where the documentation dispute bites.
- **Overlapping ranges**, including using `average_range` as its own criteria range, are
  ordinary.
- **Arrays.** The surface lifts natively (`lift_broadcast_profile: surface_native`); inline
  array arguments are accepted but must match shape exactly, since an array carries no
  reference to re-anchor.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#DIV/0!` | No position satisfies all criteria | Microsoft, `AVERAGEIFS` page |
| `#DIV/0!` | `average_range` is blank or a text value; matched cells cannot be translated into numbers | Microsoft, `AVERAGEIFS` page |
| `#DIV/0!` | Positions match but no matched `average_range` cell holds a number | reference engine at `473efa3` |
| `#VALUE!` | Any criteria range differs in size or shape from `average_range` | reference engine at `473efa3` |
| `#VALUE!` | An unpaired trailing argument (an even count after the first) | reference engine at `473efa3` |
| `#VALUE!` | A criterion is an array or an unresolved reference | reference engine at `473efa3` |
| propagated | An error value in a criterion, or in a matched `average_range` cell | reference engine at `473efa3` |

## Documentation divergences

1. **Logicals inside the averaged range.** Microsoft's `AVERAGEIFS` page states "Cells in range
   that contain TRUE evaluate as 1; cells in range that contain FALSE evaluate as 0." The
   `AVERAGEIF` page states the opposite — "Cells in the range containing TRUE or FALSE values
   are ignored" — for the same family and the same kind of range. The reference engine
   implements the *ignore* reading for both functions, through a single shared reduction
   helper. At most one of the three descriptions is right, and the Handbook has not observed
   Excel. This is recorded here as an open documentation-versus-implementation divergence, not
   as a finding about Excel.
2. **Empty cells in a criteria range.** Microsoft states "If a cell in a criteria range is
   empty, AVERAGEIFS treats it as a 0 value." The reference engine does not: an empty cell is
   matched only by a blank-equality criterion, and a numeric criterion of `0` does not select
   it. On a table with blank cells in a criteria column, the two readings give different means.
3. **The `#DIV0!` spelling** appears in the remark text on both pages where `#DIV/0!` is meant;
   noted so a reader searching for the code finds the remark.
4. **Criteria count.** Microsoft documents a limit of 127 criteria ranges; the reference engine
   caps the total argument list at 255 slots, which is 127 pairs plus the averaged range — the
   same limit expressed the other way. Recorded as agreement, since the two numbers reconcile
   exactly and a reader may otherwise think they conflict.

## Relationships

- **[AVERAGEIF](FUNC.AVERAGEIF.md)** — the singular form, with reversed argument order and the
  anchoring rule this function drops. The shared criteria language is documented there.
- **`SUMIFS` / `COUNTIFS` / `MAXIFS` / `MINIFS`** — the rest of the plural family, same module,
  same conjunction, same shape rule. `AVERAGEIFS` is `SUMIFS/COUNTIFS` only when every matched
  cell is numeric; when a matched cell holds text the divisors differ, because `COUNTIFS`
  counts matched positions and `AVERAGEIFS` counts matched numeric cells.
- **[AVERAGE](FUNC.AVERAGE.md)** — the unconditional mean, in a different module with a
  different admission rule.
- **`DAVERAGE`** — the database-family conditional mean. It expresses OR natively, through
  multiple criteria rows, which is exactly what `AVERAGEIFS` cannot do; see
  [DAVERAGE](FUNC.DAVERAGE.md).
- **`FILTER` + `AVERAGE`**, or `AVERAGE(IF(...))` as an array formula — the routes that can
  express disjunction and relational conditions. They use ordinary worksheet comparison, so
  they do not inherit the wildcard language or the tolerant numeric comparison.
- **`GROUPBY` / `PIVOTBY`** — where a whole table of conditional means is wanted rather than
  one cell.

## Numerical notes

The reduction is the same plain sequential `f64` accumulation and single division as
[AVERAGE](FUNC.AVERAGE.md#numerical-notes), applied to the masked subset in row-major order.
Its error analysis carries over unchanged: a bound proportional to \(\sum|x_i|\) over the
selected cells, and order sensitivity.

Two things are specific to this surface.

**The mask dominates the arithmetic.** As on `AVERAGEIF`, the selection is decided by the
family's truncation-style 15-significant-digit comparison rather than by exact double
equality — the lane recorded in the reference engine's `BUG-FUNC-004` stream, closed and signed
off against a named live Excel build. One wrongly selected cell moves the answer by a full
value; a summation rounding moves it by an ulp. Audit the mask first.

**Conjunction is short-circuited per position.** The reference engine evaluates criteria in
argument order and stops at the first failure for each position. That is invisible in the
result but visible in cost, and it means the *order* of criteria pairs affects how many
comparisons run — worth knowing when a criteria range is expensive to resolve, and worth
knowing when reasoning about which error surfaces if two different criteria ranges both contain
error values at different positions.

## What has not been checked

**No Handbook vector suite exists for `AVERAGEIFS`, and no evidence record lists this surface
among its subjects.** The implementing module carries `BUG-FUNC-004` and `BUG-FUNC-011` in the
reference engine's record; neither is an `AVERAGEIFS` measurement. The documented statements
above were fetched from Microsoft's `AVERAGEIFS` page while this page was written; everything
else is the reference engine at commit `473efa3`, named as such.

Inputs worth probing first:

1. **A `TRUE` cell inside `average_range`.** Microsoft's `AVERAGEIFS` page says it counts as 1;
   its `AVERAGEIF` page says it is ignored; the reference engine ignores it in both. One probe
   settles a three-way disagreement, and the same probe run through `AVERAGEIF` on the same
   data tells you whether the two functions really differ or whether one page is simply wrong.
2. **An empty cell in a criteria range with a criterion of `0`.** This separates the documented
   "treated as a 0 value" reading from the reference engine's blank-only matching.
3. **Mismatched shapes** — `AVERAGEIFS(B1:B10, A1:A5, ">5")` — to confirm the exact-shape rule
   really produces `#VALUE!` rather than silently truncating or anchoring as `AVERAGEIF` does.
4. **A transposed criteria range** of the same cardinality but different orientation, which is
   the case where "same size" and "same shape" come apart.
5. **The tolerant-comparison boundary**: a criterion of `0.3` against a cell computed as
   `0.1+0.2`, and the arithmetic boundary pair recorded in `BUG-FUNC-004`.
6. **An unpaired trailing argument**, to see whether Excel rejects the structure the way the
   reference engine does or interprets it some other way.
7. **Two criteria ranges holding different error values at different positions**, to see which
   error surfaces and whether short-circuit order is observable.
8. **127 criteria pairs and 128**, the documented boundary.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| conjunction | The AND over all criteria at a single position; there is no OR form |
| positional alignment | Position \(i\) means the same row in every range; the reason shapes must agree |
| exact-shape rule | Every criteria range must match `average_range` in size and shape; no anchoring |
| selection mask | The per-position boolean vector produced by the conjunction |
| tolerant comparison | The truncation-style 15-significant-digit numeric comparison the criteria family uses |
| reference engine | OxFunc at commit `473efa3` |

## Sources

- Microsoft, "AVERAGEIFS function" —
  <https://support.microsoft.com/en-us/office/averageifs-function-48910c45-1fc0-4389-a028-f7c5c3001690>
  (syntax and argument text; the 1-to-127 range and criteria limits; the blank-or-text `#DIV0!`
  remark; the empty-criteria-cell-as-zero remark; the TRUE-as-1/FALSE-as-0 remark; the
  all-criteria-true rule; the same-size-and-shape requirement contrasted with `AVERAGEIF`; the
  cannot-be-translated-into-numbers remark; the no-cells-meet-criteria `#DIV/0!`; and the
  wildcard set).
- Microsoft, "AVERAGEIF function" —
  <https://support.microsoft.com/en-us/office/averageif-function-faec8e2e-0dec-4308-af69-f5576d8ac642>
  (quoted here only for the contradicting ignored-logicals remark and the anchoring statement).
- OxFunc `crates/oxfunc_core/src/functions/criteria_family.rs` at commit `473efa3` — the
  pair-structure check, `ensure_same_shape`, the short-circuited `matching_mask`, and the
  `numeric_cell_for_aggregate` reduction that skips text and logicals.
- OxFunc `docs/bugs/streams/BUG-FUNC-004_numeric_comparison_tolerance_family_split.md` and
  `docs/bugs/streams/BUG-FUNC-011_countblank_range_only_parity_gap.md` — the module's two
  streams, neither of which measures this surface.
- Handbook projections `data/functions/FUNC.AVERAGEIFS.json` and
  `data/presence/FUNC.AVERAGEIFS.json`.
- Handbook, [AVERAGEIF](FUNC.AVERAGEIF.md) (the shared criteria language),
  [AVERAGE](FUNC.AVERAGE.md) (the reduction's error analysis),
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md`.
