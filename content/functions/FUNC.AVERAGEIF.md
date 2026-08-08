---
schema: efh.function-page/v1
function_id: FUNC.AVERAGEIF
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - Arguments
  - The criteria mechanism
  - The average_range anchoring rule
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
  The single-criterion conditional mean, and the only member whose value range may be a
  different shape from its criteria range — the anchoring rule that AVERAGEIFS explicitly drops.
---

# AVERAGEIF

## What it computes

`AVERAGEIF(range, criteria, [average_range])` builds a selection mask over `range` by testing
each cell against `criteria`, and returns the arithmetic mean of the numeric cells of
`average_range` at the selected positions:

    M      = { i : criteria(range[i]) }
    result = ( Σ_{i ∈ M, average_range[i] numeric} average_range[i] )
             / #{ i ∈ M : average_range[i] numeric }

Two counts are in play and they are not the same. The mask counts *matching positions*; the
divisor counts *matching positions holding a number*. A matched cell that is empty, text, or
logical contributes to neither numerator nor divisor. That is why `AVERAGEIF` can return
`#DIV/0!` on a range where the criterion matched plenty of cells.

The function decomposes cleanly into three independent stages, and every difficult question
about it belongs to exactly one of them:

1. **Criteria parsing** — turning the `criteria` argument into a comparison operator plus an
   operand.
2. **Cell matching** — applying that comparison to each cell of `range`.
3. **Filtered reduction** — averaging the selected cells of `average_range`.

Stages 1 and 2 are shared verbatim with `COUNTIF`, `SUMIF`, `MAXIFS`, `MINIFS` and the `…IFS`
plural forms; the reference engine implements all of them in one module. Stage 3 is the only
part that is `AVERAGEIF`'s own.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `range` | The cells the criterion is tested against. Required. | — |
| `criteria` | The test. A number, expression, cell reference, or text. Required. | — |
| `average_range` | The cells actually averaged. Optional. | `range` |

Microsoft describes `range` as "One or more cells to average, including numbers or names,
arrays, or references that contain numbers", `criteria` as "The criteria in the form of a
number, expression, cell reference, or text that defines which cells are averaged", and
`average_range` as "The actual set of cells to average. If omitted, range is used."

The reference engine declares an arity of 2 to 3 and, unusually for a statistical function,
takes its arguments with references still visible (`RefsVisibleInAdapter`) rather than resolved
to values — which is exactly what the anchoring rule below requires.

## The criteria mechanism

`criteria` is not a value; it is a small expression language. The reference engine parses a
text criterion into an operator prefix and a right-hand side:

| Criterion form | Parsed as |
|---|---|
| `5` or a cell holding `5` | equality against the number 5 |
| `TRUE` | equality against the logical TRUE |
| `">5"` | greater-than against the number 5 |
| `"<>abc"` | not-equal against the text `abc` |
| `"<="`, `"<"`, `">"`, `">="`, `"<>"`, `"="` prefixes | the corresponding comparison |
| `"apple"` (no prefix) | equality against text |
| `"=" `with nothing after it | equality against **blank** |
| `"a*"`, `"a?c"` | text equality with wildcards |

Microsoft documents the wildcard set on the `AVERAGEIF` page: `?` matches any single character,
`*` matches any sequence, and `~` escapes a literal `?` or `*`. The reference engine enables
wildcard interpretation only for the equality and inequality operators, not for the ordering
comparisons — a detail Microsoft's page does not state.

**Matching is kind-aware.** A numeric criterion matches numeric cells, and also text cells whose
content parses as that number; it does not match logicals. A text criterion matches only text
cells. A logical criterion matches logical cells, and text that reads as a logical. This means
`AVERAGEIF(A:A, 1)` and `AVERAGEIF(A:A, TRUE)` select different cells even though `TRUE`
coerces to 1 in arithmetic.

**Numeric comparison is not exact IEEE comparison.** The criteria family compares numbers
through a shared truncation-style 15-significant-digit helper, not through `==` on doubles.
The reference engine's defect stream `BUG-FUNC-004` records this lane as closed and signed off
against a named live Excel build; the practical consequence is that a criterion of `0.3`
matches a cell computed as `0.1+0.2`, where an exact-match function like `MATCH(...,0)` would
not. If your criterion sits on a boundary produced by arithmetic, this is the rule that decides
the answer.

## The average_range anchoring rule

Microsoft states it directly: "The average_range does not need matching dimensions as range;
calculations begin from the top-left cell and correspond to range dimensions."

So `AVERAGEIF(A1:A10, ">5", B1)` averages `B1:B10`, not the single cell `B1`. The third
argument behaves as an *anchor*, not as a range: only its top-left corner is load-bearing, and
the shape is taken from `range`.

The reference engine implements exactly this. It first flattens `average_range` as given; if
the flattened shape differs from the criteria range's shape and the argument is a reference, it
re-anchors that reference to the criteria range's shape; if it cannot (an inline array, for
instance), it keeps the literal shape and then requires the shapes to match, failing with
`#VALUE!` if they do not.

This is a genuine trap, and it is the reason `AVERAGEIFS` — which requires exact shape
agreement — is not simply "`AVERAGEIF` with more criteria". Moving a formula from one to the
other can change which cells are averaged without changing any argument text.

## Result and edge cases

Returns `Number`.

- **No matching cells** → `#DIV/0!`, documented by Microsoft: "If no cells in the range meet
  the criteria, AVERAGEIF returns the #DIV/0! error value."
- **Matches exist but none is numeric** → `#DIV/0!` in the reference engine, by the same
  zero-divisor path. Microsoft documents a related but differently-worded condition: "If range
  is a blank or text value, AVERAGEIF returns the #DIV0! error value."
- **Logical cells in the averaged range** are ignored by the reference engine. Microsoft
  documents this for `AVERAGEIF` — "Cells in the range containing TRUE or FALSE values are
  ignored" — and documents the *opposite* for `AVERAGEIFS`. See the divergences section.
- **Empty cells in `average_range`** are ignored; Microsoft states this.
- **Text cells in `average_range`** are ignored by the reference engine — not counted as zero.
  Contrast [AVERAGEA](FUNC.AVERAGEA.md), where range text is data.
- **`range` and `average_range` overlapping or identical** is ordinary and is the default.
- **Arrays.** The surface lifts natively (`lift_broadcast_profile: surface_native`).

## Errors

| Error | Condition | Source |
|---|---|---|
| `#DIV/0!` | No cell meets the criteria | Microsoft, `AVERAGEIF` page |
| `#DIV/0!` | `range` is blank or a text value | Microsoft, `AVERAGEIF` page |
| `#DIV/0!` | Matches exist but no matched cell of `average_range` holds a number | reference engine at `473efa3` |
| `#VALUE!` | The shapes of `range` and `average_range` disagree and cannot be anchored | reference engine at `473efa3` |
| `#VALUE!` | `criteria` is an array or a reference that did not resolve to a scalar | reference engine at `473efa3` |
| propagated | An error value in the criteria argument, or in a matched cell of `average_range`, becomes the result | reference engine at `473efa3` |

## Documentation divergences

The Handbook exists partly to publish these. Three are visible on this surface without running
Excel at all.

1. **Microsoft's own two pages disagree about logicals.** The `AVERAGEIF` page says "Cells in
   the range containing TRUE or FALSE values are ignored." The `AVERAGEIFS` page says "Cells in
   range that contain TRUE evaluate as 1; cells in range that contain FALSE evaluate as 0."
   These are the same family, the same kind of range, and opposite rules. The reference engine
   implements the `AVERAGEIF` reading for **both** functions: its shared
   `numeric_cell_for_aggregate` helper returns nothing for a logical cell. So either
   Microsoft's `AVERAGEIFS` page is wrong, or the reference engine is wrong about `AVERAGEIFS`.
   Nobody has resolved this against Excel; it is the first probe listed below.
2. **"Empty criteria cells are treated as zero values"** (Microsoft, `AVERAGEIF` page) does not
   describe what the reference engine does. Given an empty criteria argument, the reference
   engine builds a *blank-equality* criterion — it selects cells that are themselves blank —
   rather than an equality against the number zero. The two readings select disjoint sets on
   any range that holds both blanks and zeros.
3. **The `#DIV0!` spelling.** Microsoft's remark text renders the error as `#DIV0!` on both the
   `AVERAGEIF` and `AVERAGEIFS` pages, while the same pages use `#DIV/0!` elsewhere. This is a
   typographical defect in the documentation rather than a behavioural claim, but it is worth
   recording so that a reader searching for the error code finds the remark.

## Relationships

- **[AVERAGEIFS](FUNC.AVERAGEIFS.md)** — the plural form. Not a superset: it requires exact
  shape agreement, so it cannot express the anchoring shorthand, and its documentation states
  the opposite logical rule. Argument order is also reversed — the averaged range comes first.
- **`COUNTIF` / `SUMIF`** — the same criteria mechanism with a different reduction.
  `AVERAGEIF` is mathematically `SUMIF/COUNTIF` **only** when every matched cell is numeric;
  otherwise the divisors differ, because `COUNTIF` counts matched cells while `AVERAGEIF`
  counts matched *numeric* cells.
- **`MAXIFS` / `MINIFS`** — same module, same criteria mechanism, different reduction; note
  they return `0` rather than an error when nothing matches, which `AVERAGEIF` does not.
- **[AVERAGE](FUNC.AVERAGE.md)** — the unconditional mean, with a different admission rule
  (the direct-versus-range dual policy) implemented in a different module. `AVERAGEIF` is not
  `AVERAGE` with a filter.
- **`DAVERAGE`** — the database-family conditional mean, with a criteria *range* rather than a
  criteria expression; see [DAVERAGE](FUNC.DAVERAGE.md) for that mechanism.
- **`FILTER` + `AVERAGE`** — the modern dynamic-array route, which uses ordinary worksheet
  comparison rather than the criteria language and therefore does not inherit the wildcard or
  the tolerant-numeric rules.

## Numerical notes

The reduction is a plain sequential `f64` accumulation followed by one division, so the error
analysis is exactly [AVERAGE's](FUNC.AVERAGE.md#numerical-notes): a bound proportional to
\(\sum|x_i|\), a condition number of \(\sum|x_i|/|\sum x_i|\), and sensitivity to summation
order. Nothing about the criteria stage improves it.

What the criteria stage adds is a **second** numerical question that has nothing to do with
summation: the tolerant comparison. Because `compare_excel_numbers` works on a
15-significant-digit truncation rather than the full 53-bit significand, the selection mask
itself is a function of a rounding decision. Two numbers that differ in the sixteenth digit
select together. For a conditional average this is a bigger effect than any summation error:
a mis-selected cell changes the answer at full magnitude, while a summation error changes it in
the last bits. Anyone auditing an `AVERAGEIF` should look at the mask before looking at the
arithmetic.

The ordering of the scan is row-major over the flattened range, and the reference engine
accumulates in that order, so the answer depends on the range's orientation. A column and its
transpose can differ in the last bits.

## What has not been checked

**No Handbook vector suite exists for `AVERAGEIF`, and no evidence record lists this surface
among its subjects.** The criteria module carries two defect streams in the reference engine's
own record — `BUG-FUNC-004` (numeric comparison tolerance, closed and signed off against a
named live Excel build) and `BUG-FUNC-011` (a `COUNTBLANK` range-only parity gap) — but neither
is an `AVERAGEIF` measurement, and neither is evidence about this surface. The documented
behaviour above comes from Microsoft's `AVERAGEIF` and `AVERAGEIFS` pages, fetched while this
page was written; the rest is the reference engine at commit `473efa3`, named as such.

Inputs worth probing first:

1. **A logical cell inside `average_range`**, run through `AVERAGEIF` and `AVERAGEIFS` on the
   same data. This is the highest-value probe in this batch: Microsoft's two pages predict
   different answers, and the reference engine predicts one answer for both. Whatever Excel
   does, one of the three is wrong, and the result is publishable either way.
2. **An empty criteria argument** — `AVERAGEIF(A1:A5, B1)` with `B1` empty — over a range
   containing both blanks and zeros. This separates the documented "treated as zero" reading
   from the reference engine's blank-equality reading.
3. **`AVERAGEIF(A1:A10, ">5", B1)`** — the anchoring rule with a single-cell third argument. If
   only `B1` is averaged, the anchoring rule is not what Microsoft's page describes.
4. **`AVERAGEIF(A1:A10, ">5", C1:D5)`** — an anchor whose own shape is neither one cell nor the
   criteria shape, which is where "begin from the top-left cell" stops being unambiguous.
5. **The tolerant-comparison boundary**: a criterion of `0.3` against cells holding `0.3` and
   `=0.1+0.2`, and the arithmetic boundary pair recorded in `BUG-FUNC-004`.
6. **Numeric text in the criteria range**: a cell holding the text `"5"` with a criterion of
   `5`, and the mirror case.
7. **Wildcards under an ordering operator** — `">a*"` — which the reference engine treats as a
   literal and Microsoft's page does not discuss.
8. **`AVERAGEIF` versus `SUMIF/COUNTIF`** on a range where a matched cell holds text, which is
   the cheapest demonstration that the two divisors differ.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| criteria language | The operator-prefix-plus-operand mini-expression a `criteria` argument is parsed into |
| selection mask | The per-position boolean vector produced by testing `range` against `criteria` |
| anchoring | Taking only the top-left corner of `average_range` and the shape of `range` |
| tolerant comparison | The truncation-style 15-significant-digit numeric comparison the criteria family uses |
| matched numeric cell | A selected position whose `average_range` cell holds a number; the divisor counts these |
| reference engine | OxFunc at commit `473efa3` |

## Sources

- Microsoft, "AVERAGEIF function" —
  <https://support.microsoft.com/en-us/office/averageif-function-faec8e2e-0dec-4308-af69-f5576d8ac642>
  (syntax and argument text; the ignored-logicals remark; ignored empty cells in
  `average_range`; the blank-or-text `#DIV0!` remark; the empty-criteria-as-zero remark; the
  no-matching-cells `#DIV/0!`; the wildcard set; and the top-left anchoring statement).
- Microsoft, "AVERAGEIFS function" —
  <https://support.microsoft.com/en-us/office/averageifs-function-48910c45-1fc0-4389-a028-f7c5c3001690>
  (quoted here only for the contradicting TRUE-evaluates-as-1 remark and the same-size-and-shape
  requirement).
- OxFunc `crates/oxfunc_core/src/functions/criteria_family.rs` at commit `473efa3` — criteria
  parsing, kind-aware matching, wildcard gating, the anchoring branch, and the
  `numeric_cell_for_aggregate` reduction that skips text and logicals.
- OxFunc `docs/bugs/streams/BUG-FUNC-004_numeric_comparison_tolerance_family_split.md` — the
  tolerant numeric-comparison lane for this family, closed and signed off; and
  `docs/bugs/streams/BUG-FUNC-011_countblank_range_only_parity_gap.md`, the module's other
  stream.
- Handbook projections `data/functions/FUNC.AVERAGEIF.json` and
  `data/presence/FUNC.AVERAGEIF.json`.
- Handbook, [AVERAGE](FUNC.AVERAGE.md) (the shared reduction and its error analysis),
  [The value universe](../model/01-value-universe.md),
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook `CHARTER.md` sections 1, 3 and 7; `content/model/06-claim-language.md`.
