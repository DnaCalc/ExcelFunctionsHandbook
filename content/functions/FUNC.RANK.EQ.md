---
schema: efh.function-page/v1
function_id: FUNC.RANK.EQ
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — RANK.EQ function"
    locator: "https://support.microsoft.com/en-us/office/rank-eq-function-284858ce-8ef6-450e-b662-26245be04a40"
    role: "documented signature, the order-flag convention, the tie behaviour, and the #N/A condition"
  - work: "OxFunc — rank_common.rs"
    locator: "crates/oxfunc_core/src/functions/rank_common.rs"
    role: "reference-engine kernel: the strict-count rank, the exact equality match, and the order coercion"
episodes: []
body_sections:
  - "What it computes"
  - "Arguments"
  - "Result and edge cases"
  - "Errors"
  - "Relationships"
  - "Numerical notes"
  - "What has not been checked"
  - "Page vocabulary"
  - "Sources"
family: rank_eq_fn
role_in_family: >-
  Competition ranking: tied values all take the best rank of their block and the following
  ranks are skipped, so the ranks of a list with ties are not a permutation of 1..n.
---

# RANK.EQ

## What it computes

`RANK.EQ(number, ref, [order])` returns the position `number` would occupy in `ref` if `ref` were
sorted, counting from the best end.

Let `S = {v_1, …, v_n}` be the numeric values in `ref` (a multiset — repeats are kept). Then

    order = 0 or omitted (descending):   RANK.EQ  =  #{ i : v_i >  number }  +  1
    order nonzero      (ascending):      RANK.EQ  =  #{ i : v_i <  number }  +  1

The rank is a **strict count plus one**. That single formula is the whole function, and it
determines the tie behaviour without any separate rule: because the comparison is strict, every
member of a tied block counts zero of its own peers, so all of them receive the same rank — the
best one the block occupies.

This is **competition ranking**, the `1, 2, 2, 4` convention. It is what a scoreboard does: two
silver medals and no bronze. Two properties follow, and the second is the one that surprises
people:

1. **Ties share the best rank of their block.** Three values tied at what would be positions
   `4, 5, 6` all rank `4`.
2. **The ranks are not a permutation of `1..n`.** After a block of `m` ties at rank `r`, the next
   distinct value ranks `r + m`, not `r + 1`. Ranks are skipped, and the sum of the ranks over
   all of `ref` is not `n(n+1)/2`. Anything downstream that assumes a rank vector is a
   permutation — a Spearman correlation computed by hand, for instance — is wrong on tied data
   and needs [RANK.AVG](FUNC.RANK.AVG.md) instead.

`number` must **occur** in `ref`. `RANK.EQ` is not an interpolating position function: it does
not tell you where a value *would* go, it tells you where a value that is present *is*. A
`number` absent from `ref` yields `#N/A`, not a fractional or bracketing rank. That is the
behaviour that distinguishes `RANK.EQ` from [PERCENTRANK.INC](FUNC.PERCENTRANK.INC.md), which
does interpolate.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The value whose rank is wanted. Required. | — |
| `ref` | The list of numbers to rank within. Required. | — |
| `order` | `0` or omitted for descending; any nonzero value for ascending. Optional. | `0` (descending) |

**`order` is a flag, not a direction constant.** Only the zero/nonzero distinction is read: `1`,
`-1`, `TRUE` and `2` all select ascending. The reference engine coerces `order` to a number and
truncates it before the zero test, so `0.5` truncates to `0` and selects *descending* — an
undocumented consequence of the truncation that a caller writing a computed `order` could
plausibly hit.

The default is **descending**, which is worth stating explicitly because it is the opposite of
the default a reader coming from a sort dialog expects. `RANK.EQ(x, A)` answers "how high does
`x` place", with `1` meaning largest.

`ref` is consumed by scanning: text, logical values and blank cells in a scanned range are
skipped rather than converted, so `n` counts numeric cells only. See
[coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number` — a positive integer, always an exact integer value.

- **`number` not present in `ref`.** `#N/A`.
- **`ref` empty of numbers.** `#N/A`.
- **`number` is text, a logical or a blank cell.** The reference engine returns `#N/A` rather
  than `#VALUE!`: a non-numeric `number` cannot match any collected value, and the code routes it
  to the not-found answer instead of to a coercion failure. Microsoft's page does not state a
  rule for this, and `#VALUE!` would be the equally defensible reading. This one is on the probe
  list.
- **`number` present more than once.** All occurrences receive the same rank; the multiplicity
  affects the ranks of *later* values, not this one.
- **`number` in `ref` but reached by a different route.** The match is exact IEEE-754 equality of
  the collected doubles — see Numerical notes. A value that displays identically but differs in
  the last bit does not match.
- **Errors in `ref`** propagate as themselves.
- **`ref` given as a rectangular grid** is flattened; `RANK.EQ` does not care about shape.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#N/A` | `number` does not occur in `ref` | documented |
| `#N/A` | `ref` contains no numeric values | reference engine |
| `#N/A` | `number` is non-numeric | reference engine; `#VALUE!` would be the alternative reading and is not ruled out |
| `#VALUE!` | `order` is non-numeric text | shared coercion rule |
| propagated | An error value in `number`, `ref` or `order` surfaces as that error | shared coercion rule |

## Relationships

- **[RANK.AVG](FUNC.RANK.AVG.md)** is the sibling that resolves ties by averaging. For a tied
  block of `m` values starting at competition rank `r`, `RANK.AVG` returns `r + (m-1)/2`, the
  midrank. The two functions agree exactly when `number` has no duplicates in `ref`. `RANK.AVG`
  is the one to use whenever the ranks feed further statistics, because its ranks always sum to
  `n(n+1)/2`.
- **[RANK](FUNC.RANK.md)** is the legacy Compatibility-category name. Microsoft's replacement
  guidance points `RANK` at `RANK.EQ`, and the two are documented as computing the same quantity.
  **The Handbook does not treat that as an identity.** A legacy alias and its modern name are two
  registered entry points, and Excel is free to route them to two different code paths; that they
  agree is a claim requiring evidence. No such evidence is in the Handbook's record. The one
  place the question has teeth is the `order` flag and the non-numeric-`number` case, where the
  older function's behaviour is even less documented.
- **[SMALL](FUNC.SMALL.md) and [LARGE](FUNC.LARGE.md)** are the inverses:
  `LARGE(ref, RANK.EQ(x, ref))` recovers `x` when `x` occurs in `ref`, because competition rank
  and the descending order-statistic index are the same number. The corresponding identity for
  `RANK.AVG` fails on ties, since its rank need not be an integer.
- **[PERCENTRANK.INC](FUNC.PERCENTRANK.INC.md)** answers the same question on a `[0, 1]` scale
  and *does* interpolate for values not present, which is exactly the behaviour `RANK.EQ` refuses.
- **[COUNTIF](FUNC.COUNTIF.md)** reproduces the definition directly:
  `COUNTIF(ref, ">"&number) + 1` is the descending rank, without the presence check. Readers who
  want a rank for an absent value write this, and should know they have given up the `#N/A`
  diagnostic in the process.

## Numerical notes

`RANK.EQ` performs no arithmetic beyond counting, so there is no rounding error anywhere in it.
Every numerical question about this function is a **comparison** question, and there are two.

**The match is exact equality, not tolerant comparison.** The reference engine tests
`v_i == number` on raw binary64 values. Excel is known to use a truncation-style
fifteen-significant-digit comparison in some of its tolerant-comparison families — the reference
engine records that helper for the lookup family, and deliberately records the contrast that the
exact-match lookup lane stays exact. Which side `RANK.EQ` falls on is not documented anywhere and
has not been observed. The stakes are concrete: `RANK.EQ(0.3, {0.1+0.2, 1, 2})` is `#N/A` under
exact equality and a rank under a tolerant one, and a workbook that computes its `number` by a
different formula from the one that filled `ref` will meet this. This is the single most
important open question about the function.

**Signed zero and the strict comparison.** `-0.0 == 0.0` is true and `-0.0 > 0.0` is false, so
the two zeros are one value for both the match test and the rank count. That is the right answer
and it is worth noting only because the sort-free formulation makes it automatic — an
implementation that sorted and then scanned would have to be careful, and this one does not sort
at all.

**The counting form is `O(n)` per call and exact.** Because the rank is computed as a count
rather than by sorting and locating, a column of `RANK.EQ` formulas over the same `ref` is
`O(n^2)` overall. That is a performance fact rather than a correctness one, but it is the reason
large rank columns are slow, and it is not fixable inside the function.

**The `order` truncation.** `trunc(order)` before the zero test means the ascending/descending
boundary is not at zero but across the open interval `(-1, 1)`, which all truncates to `0` and
therefore selects descending. Nothing documents this, and a caller passing a computed fraction
gets the opposite of what the "any nonzero value" phrasing promises.

## What has not been checked

No Handbook vector suite exists for `RANK.EQ`, and no Handbook evidence record names it as a
subject. Nobody has checked this function against Excel within the Handbook's record. The
implementing module carries no tests of its own in the reference engine; the shared rank kernel
it calls carries a small number, which is a statement about that kernel rather than about this
surface.

Everything above marked as documented comes from Microsoft's `RANK.EQ` page. **Retrieval of that
page was blocked by the upstream host on this pass**, so those statements are recorded as
documented behaviour with the source named and should be re-read against the live page.

Inputs worth probing first:

1. **`RANK.EQ(0.1+0.2, {0.3, 1, 2})`** — the tolerant-versus-exact comparison probe, and the one
   that decides whether `RANK.EQ` belongs to Excel's tolerant-comparison family or its exact one.
   Everything else on this list is cheaper to guess than this one.
2. **A three-way tie**: `RANK.EQ(2, {3,2,2,2,1})` and the rank of `1` in the same list,
   confirming the block takes the best rank and that the following rank skips by the block size.
3. **`order` values `0.5`, `-0.5`, `TRUE`, `-1`** — four probes that pin whether truncation
   happens before the zero test, which the documented "any nonzero value" wording does not cover.
4. **A non-numeric `number`**: text, `TRUE`, and a blank cell, against the `#N/A`-versus-`#VALUE!`
   split.
5. **`ref` containing text, logicals and blanks** alongside numbers, confirming they are skipped
   rather than counted as zeros.
6. **`RANK.EQ` against `RANK`** on the same tied data with the same `order`, compared bitwise —
   the probe that would turn the documented legacy-replacement relationship into an evidenced
   one.
7. **`LARGE(ref, RANK.EQ(x, ref))`** over tied data, confirming the inverse identity holds
   exactly.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| competition ranking | The `1, 2, 2, 4` convention: ties share the best rank, following ranks skip |
| strict count | The rank formula's `#{ v_i > number } + 1`; strictness is what produces the tie behaviour |
| midrank | The averaged rank [RANK.AVG](FUNC.RANK.AVG.md) returns instead |
| order flag | `order`, read only as zero versus nonzero, after truncation |
| exact equality match | The raw binary64 `==` test the reference engine uses to find `number` in `ref` |

## Sources

- Microsoft, *RANK.EQ function* —
  <https://support.microsoft.com/en-us/office/rank-eq-function-284858ce-8ef6-450e-b662-26245be04a40>
  (signature, the descending default and the zero/nonzero `order` convention, the shared-rank tie
  behaviour, and the `#N/A` condition when `number` is absent). Retrieval was blocked by the
  upstream host for this page; the documented behaviour above is stated as documented behaviour
  and should be re-checked against the page.
- OxFunc `crates/oxfunc_core/src/functions/rank_eq_fn.rs` and
  `crates/oxfunc_core/src/functions/rank_common.rs` at commit `473efa3` — the strict-count rank,
  the exact-equality presence test, the `#N/A` for a non-numeric `number`, and the `order`
  truncation.
- OxFunc bug stream `docs/bugs/streams/BUG-FUNC-004_numeric_comparison_tolerance_family_split.md`
  — the tolerant-versus-exact numeric comparison split whose applicability to `RANK.EQ` is the
  open question above.
- Handbook [coercion and lifting](../model/02-coercion-and-lifting.md) — the range-scan policy
  governing what `ref` contributes.
- Handbook `data/functions/FUNC.RANK.EQ.json`, `data/presence/FUNC.RANK.EQ.json`.
