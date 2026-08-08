---
schema: efh.function-page/v1
function_id: FUNC.FREQUENCY
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Frequency method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.frequency"
    role: "documented description, the two empty-argument rules, the one-more-than-bins rule, and the ignore-blanks-and-text rule"
  - work: "Microsoft Support — FREQUENCY function"
    locator: "https://support.microsoft.com/en-us/office/frequency-function-44e3be2b-eca0-42cd-a3f7-fd9ea898fdb9"
    role: "the worksheet-surface documentation page; not retrievable at curation time (the host refused the request)"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - The bin convention
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: lookup_prob_frequency_family
role_in_family: >-
  The counting member of a module otherwise devoted to lookup and probability: the only one that
  returns a vertical array of counts rather than a selected or reduced value.
---

## What it computes

`FREQUENCY(data_array, bins_array)` counts how many values of `data_array` fall into each
interval defined by `bins_array`, and returns those counts as a **vertical array with one more
element than `bins_array` has**.

Given m bin boundaries b₁ ≤ b₂ ≤ … ≤ b_m, the returned counts c₁ … c_{m+1} are

    c₁      = #{ v : v ≤ b₁ }
    c_k     = #{ v : b_{k−1} < v ≤ b_k }        for 2 ≤ k ≤ m
    c_{m+1} = #{ v : v > b_m }

Three properties define the function and all three are documented.

**The intervals are half-open, closed on the right.** A value exactly equal to a boundary falls
into the interval *ending* at that boundary, not the one starting there. This is the opposite
convention from most histogram tooling — NumPy, R's `hist`, and most plotting libraries default
to left-closed, right-open bins — and it is the single most common source of off-by-one
disagreement when a spreadsheet histogram is reproduced elsewhere. Microsoft's own worked
description is the "count of test scores in ranges of scores" framing; the boundary rule
follows from the extra-element description quoted below.

**The result has one more element than there are bins.** Microsoft states this directly, and
explains the extra element: it "returns the count of any values above the highest interval".
The m boundaries cut the real line into m + 1 pieces, and the function returns all m + 1
counts. Entering `FREQUENCY` over the same number of cells as bins silently discards the
overflow count.

**The result is a vertical array.** Not a row, not a scalar, whatever the shapes of the
inputs. Microsoft's wording — "returns a vertical array of numbers" — is a shape guarantee, and
it means an implementation cannot echo the orientation of `bins_array`.

Counts are returned as numbers; the sum of all m + 1 counts equals the number of *admitted*
values in `data_array`, which is not the same as the number of cells in it — see the
ignore rule under **Arguments**.

## Arguments

Two arguments, both required; the reference engine declares an arity of exactly 2.

| Argument | Meaning |
|---|---|
| `data_array` | The values whose frequencies are counted. Documented: if it "contains no values, FREQUENCY returns an array of zeros." |
| `bins_array` | The interval boundaries. Documented: if it "contains no values, FREQUENCY returns the number of elements in data_array." |

**`FREQUENCY` ignores blank cells and text.** That is Microsoft's sentence, verbatim in
substance, and it is the scan policy for both arguments. Note what it does not say: it says
nothing about logical values, and nothing about text that looks like a number. Those are
unsettled here and are on the probe list.

Both arguments are consumed by scanning. This is not a lift kernel and there is no elementwise
broadcast; an array argument is data, not a call to repeat the function.

Microsoft's remark that `FREQUENCY` "must be entered as an array formula" is a pre-dynamic-array
statement. On a build with dynamic arrays the result spills from a single cell. The Handbook has
not checked how the two entry modes differ on this function, and the difference is exactly the
kind that changes what a reader sees without changing what the function computes.

## The bin convention

Worth isolating, because it is where readers get the wrong answer while believing they got the
right one.

| Given bins | Interval for c₁ | for c₂ | … | for c_{m+1} |
|---|---|---|---|---|
| b₁, b₂, …, b_m | (−∞, b₁] | (b₁, b₂] | … | (b_m, +∞) |

Consequences:

- A data value equal to b₁ is counted in the **first** bin.
- A data value equal to b_m is counted in bin m, **not** in the overflow bin.
- Duplicate boundaries produce an empty interval between them, whose count is necessarily zero.
- Reversing a "greater than or equal" intuition by shifting boundaries down by one unit works
  for integers and fails for continuous data. There is no option to switch the convention.

**Whether the boundaries must be sorted is not documented, and the reference engine takes a
position.** Microsoft's page says nothing about the ordering of `bins_array`. OxFunc's kernel
requires the boundaries to be non-decreasing and returns `#NUM!` if they are not. That is a
domain restriction with no documented counterpart, and it is recorded here as a
documentation-versus-reference-engine divergence: the documentation is silent, the reference
engine refuses. What Excel does with unsorted bins — reject, sort, or count against the
boundaries in the order given, which for an unsorted list gives a different and stranger answer
than either — is unverified and is the first thing on the probe list.

## Result and edge cases

Returns `Array` — a vertical array of numbers, of length `LEN(bins_array) + 1`.

- **No admitted values in `data_array`**: documented as an array of zeros. The array still has
  one element more than the bin count.
- **No admitted values in `bins_array`**: documented as "the number of elements in data_array",
  which is the degenerate m = 0 case — a single-element array holding the total count. Note the
  documented wording says *elements of data_array*, while the same page says text and blanks are
  ignored; whether an ignored cell is an "element" for this rule is not stated.
- **Both empty**: falls under both rules at once, which the documentation does not disambiguate.
- **Duplicate boundaries** give a zero count for the collapsed interval.
- **The counts are exact integers** held as doubles, so there is no accuracy question about the
  counts themselves. The only comparison performed is `value ≤ boundary`, which is exact IEEE
  comparison — and that is where the numerical content of this function actually lives; see
  **Numerical notes**.
- **Errors inside the scanned ranges** propagate under the shared coercion discipline; see
  [Coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

Microsoft's documentation for this function lists **no error conditions at all**. The Remarks
describe entry mode, the extra element, and the ignore rule; there is no error table.

That is a documentation gap rather than a guarantee, and the Handbook records two ways the
reference engine produces errors that no documentation covers:

| Error | Condition, as observed in the reference engine only |
|---|---|
| `#NUM!` | `bins_array` is not non-decreasing |
| `#REF!` | the projected battery records `#REF!` for an inline two-dimensional array literal in both argument positions |

Neither has a documented counterpart, and neither has been compared against Excel by the
Handbook. The `#REF!` is the more surprising of the two: `#REF!` is the invalid-reference error,
and nothing in the documented behaviour of `FREQUENCY` suggests a route to it from
well-formed array literals.

## Relationships

- **`COUNTIF` / `COUNTIFS`** — the per-bin alternative. One `COUNTIFS` per interval reproduces
  `FREQUENCY`, and doing so makes the boundary convention explicit instead of implicit, which is
  why it is the usual debugging move when a histogram looks wrong by one.
- **`COUNTIF` with `">"` and `"<="`** — the direct spelling of a single half-open bin.
- **`MATCH` with match type 1** — the same "largest boundary not exceeding" search that a
  sorted-bin implementation performs internally, exposed as a lookup.
- **`HISTOGRAM` (Analysis ToolPak)** and Excel's chart histogram — separate machinery with its
  own binning rules; agreement with `FREQUENCY` is not to be assumed.
- **`GROUPBY` / `PIVOTBY`** — the modern aggregation route, which supersedes `FREQUENCY` for
  most reporting uses without superseding it as a function.
- **Module siblings**: `LOOKUP`, `PROB`, `MODE.MULT` share the implementing module in the
  reference engine. Sharing a module is an implementation fact and carries no semantic
  relationship.

## Numerical notes

`FREQUENCY` has no arithmetic in it, and that makes its numerical behaviour easier to state and
no less consequential.

**The whole function is a comparison predicate.** Every count depends only on `value ≤ boundary`
evaluated in IEEE double precision. Two things follow. First, values that a user thinks are
equal to a boundary may not be: `0.1 + 0.2` is not `0.3`, and a data point produced by
arithmetic lands on the wrong side of a boundary typed as a literal. This is not a defect in
`FREQUENCY`; it is the boundary convention meeting binary floating point, and it is why bin
edges chosen midway between achievable data values are the robust choice. Second, the
comparison is exact, so the function is *deterministic and reproducible* in a way almost no
other member of the Statistical category is — there is no summation order, no cancellation, no
transcendental.

**Complexity.** The reference engine's kernel scans, for each data value, the boundary list from
the start and stops at the first boundary the value does not exceed — O(|data| × |bins|), and it
depends on the boundaries being sorted for its first-match rule to mean anything. A binary
search over sorted boundaries gives O(|data| · log|bins|) with identical results; a
sort-and-merge gives O(|data| log|data|). For the range sizes spreadsheets reach, the difference
is real but rarely decisive; the correctness dependence on sortedness is the more important
observation, and it is what makes the undocumented `#NUM!` guard above intelligible.

**Counts are exact.** Integers up to 2⁵³ are exactly representable, and no spreadsheet range
reaches that, so the returned counts are exact integers with no accumulation error. Any
disagreement between two implementations of `FREQUENCY` is therefore a *semantic* disagreement —
about the boundary convention, the ignore rule, or the sortedness requirement — never a rounding
one. That is unusual in this category and it makes disagreements here unusually diagnostic.

## What has not been checked

No Handbook vector suite exists for `FREQUENCY`, and no evidence record in the Handbook's
collection names this surface as a subject. **Nobody has checked `FREQUENCY` against Excel
within this record.**

The two behaviours this page reports beyond the documentation — the `#NUM!` on unsorted bins and
the `#REF!` on inline two-dimensional array literals — come from the reference engine and its
projected battery. No Excel was involved in producing either.

Inputs I would probe first, and why:

1. **Unsorted `bins_array`**, for instance boundaries given as 3, 1, 2 against data spanning
   all three. This is the top of the list because the documentation is silent, the reference
   engine returns `#NUM!`, and there are at least three plausible Excel behaviours that give
   three different visible answers. One probe distinguishes all of them.
2. **A value exactly on a boundary**, both as a typed literal and as the result of arithmetic
   (`0.1+0.2` against a boundary of `0.3`). This pins the closed-on-the-right convention and
   simultaneously demonstrates the floating-point trap to any reader who doubts it.
3. **`bins_array` with duplicate boundaries**, confirming the empty interval yields zero rather
   than a repeat count.
4. **Empty `data_array`, empty `bins_array`, and both empty** — the two documented degenerate
   rules and the case where they collide, which the documentation does not resolve.
5. **Text, numeric text, logicals and errors in each argument** — the documented ignore rule
   covers blanks and text only, and says nothing about logicals or about text that parses as a
   number. Two probes settle a rule that affects every real data range.
6. **An inline two-dimensional array literal in each position**, to reproduce or refute the
   `#REF!` the reference engine's battery records.
7. **Legacy array entry versus dynamic-array spill**, on the same inputs, to check that the
   entry mode changes only the presentation and not the counts.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| bin boundary | An element of `bins_array`; the closed upper end of one interval |
| half-open, closed right | The interval convention (b_{k−1}, b_k] that FREQUENCY uses |
| overflow bin | The extra returned element counting values above the highest boundary |
| ignore rule | The documented policy that blanks and text in the scanned ranges are skipped |
| sortedness requirement | The reference engine's undocumented `#NUM!` guard on non-monotone bins |

## Sources

- Microsoft Learn, "WorksheetFunction.Frequency method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.frequency>
  (the vertical-array result, the one-more-than-bins rule and the overflow element, the two
  empty-argument rules, the ignore-blanks-and-text rule, and the array-entry requirement). The
  worksheet-surface page at `support.microsoft.com` was not retrievable at curation time.
- Handbook, [The value universe](../model/01-value-universe.md) — array as a value kind and the
  raw-versus-published boundary.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — scan policy versus
  elementwise lift, and error propagation.
- Handbook projections `data/functions/FUNC.FREQUENCY.json` (arity, custom coercion and kernel
  classes, reference-visible argument preparation) and `data/presence/FUNC.FREQUENCY.json` (the
  `lookup_prob_frequency_family` module, shared with `LOOKUP`, `PROB` and `MODE.MULT`).
- OxFunc `crates/oxfunc_core/src/functions/lookup_prob_frequency_family.rs` at commit `473efa3` —
  the `frequency_kernel`, its non-decreasing-bins guard, and its first-match linear scan.
