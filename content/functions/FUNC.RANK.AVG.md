---
schema: efh.function-page/v1
function_id: FUNC.RANK.AVG
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — RANK.AVG function"
    locator: "https://support.microsoft.com/en-us/office/rank-avg-function-bd406a6f-eb38-4d73-aa8e-6d1c3c72e83a"
    role: "documented signature, the order-flag convention, the averaged-tie behaviour, and the #N/A condition"
  - work: "OxFunc — rank_common.rs"
    locator: "crates/oxfunc_core/src/functions/rank_common.rs"
    role: "reference-engine kernel: the midrank formula built on the competition rank"
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
family: rank_avg_fn
role_in_family: >-
  Fractional ranking: a tied block receives the average of the ranks it occupies, so the ranks
  of a list always sum to n(n+1)/2 and are usable as inputs to further statistics.
---

# RANK.AVG

## What it computes

`RANK.AVG(number, ref, [order])` returns the **midrank** of `number` within `ref`: the average of
the positions the tied block containing `number` occupies in the sorted order.

Let `S` be the multiset of numeric values in `ref`, let `r` be the competition rank

    order = 0 or omitted (descending):   r  =  #{ i : v_i >  number }  +  1
    order nonzero      (ascending):      r  =  #{ i : v_i <  number }  +  1

and let `m = #{ i : v_i == number }` be the multiplicity of `number`. The tied block occupies
positions `r, r+1, …, r+m-1`, whose mean is

    RANK.AVG  =  r  +  (m - 1) / 2

This is **fractional ranking**, the `1, 2.5, 2.5, 4` convention. When `m = 1` the correction term
vanishes and `RANK.AVG` agrees exactly with [RANK.EQ](FUNC.RANK.EQ.md); the two functions differ
only on values that repeat.

The reason midranks exist is a conservation property that competition ranks lack. Summing the
midrank over every element of `ref` gives

    sum of midranks  =  1 + 2 + … + n  =  n(n+1)/2

for **every** data set, tied or not, because averaging within a block redistributes the block's
positions without changing their total. Competition ranking skips positions and does not conserve
the sum. That conservation is exactly what the rank-based statistics need: Spearman's rank
correlation, the Wilcoxon rank-sum and signed-rank tests, the Mann–Whitney U statistic and the
Kruskal–Wallis test are all defined on midranks, and all of them lose their null distributions if
fed competition ranks on tied data. **If the ranks are going into further arithmetic, `RANK.AVG`
is the correct function and `RANK.EQ` is not.**

Two consequences of the formula:

1. **The result need not be an integer.** A block of even size lands on a half-integer; a block
   of size `m` shifts the rank by `(m-1)/2`, which is a multiple of `1/2` and nothing finer.
   `RANK.AVG` returns exactly `r` or exactly `r + k/2`, never anything more exotic.
2. **The result is unchanged by which member of the block you ask about.** Every element of the
   tied block gets the same midrank, because `r` and `m` depend on the *value*, not on its
   position in `ref`.

Like [RANK.EQ](FUNC.RANK.EQ.md), `RANK.AVG` requires `number` to **occur** in `ref`. It reports
where a present value sits; it does not interpolate a position for an absent one. An absent
`number` yields `#N/A`.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The value whose rank is wanted. Required. | — |
| `ref` | The list of numbers to rank within. Required. | — |
| `order` | `0` or omitted for descending; any nonzero value for ascending. Optional. | `0` (descending) |

`order` is read only as zero versus nonzero. The reference engine coerces it and **truncates
toward zero** before that test, so any `order` in the open interval `(-1, 1)` selects
*descending* — an undocumented consequence that a computed fractional flag would hit.

The default is descending: `1` is the largest value.

`ref` is consumed by scanning; text, logical values and blank cells in a scanned range are
skipped rather than converted, so `n` counts numeric cells only. See
[coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number` — an exact integer or exact half-integer.

- **`number` not present in `ref`.** `#N/A`. The reference engine reaches this by computing the
  competition rank first, which itself fails when the multiplicity is zero.
- **`ref` empty of numbers.** `#N/A`.
- **`number` is text, a logical or a blank cell.** The reference engine returns `#N/A` rather than
  `#VALUE!`. Microsoft's page does not state a rule for this; `#VALUE!` is the equally defensible
  alternative and is on the probe list.
- **A block spanning the whole list.** If every value in `ref` equals `number`, then `r = 1` and
  `m = n`, so the answer is `(n+1)/2` — the midpoint. Every element is equally ranked, which is
  the correct statement about a constant sample.
- **Errors in `ref`** propagate as themselves.
- **`ref` given as a rectangular grid** is flattened.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#N/A` | `number` does not occur in `ref` | documented |
| `#N/A` | `ref` contains no numeric values | reference engine |
| `#N/A` | `number` is non-numeric | reference engine; `#VALUE!` is the alternative reading and is not ruled out |
| `#VALUE!` | `order` is non-numeric text | shared coercion rule |
| propagated | An error value in `number`, `ref` or `order` surfaces as that error | shared coercion rule |

## Relationships

- **[RANK.EQ](FUNC.RANK.EQ.md)** is the competition-ranking sibling and the base this function is
  built on: the reference engine computes `RANK.EQ` and adds `(m-1)/2`. They agree exactly when
  `number` is unique in `ref`, and differ by `(m-1)/2` otherwise. `RANK.EQ` is for scoreboards;
  `RANK.AVG` is for statistics.
- **[RANK](FUNC.RANK.md)**, the legacy Compatibility-category name, corresponds to
  [RANK.EQ](FUNC.RANK.EQ.md), **not** to this function. `RANK.AVG` has no legacy predecessor: it
  arrived with the dotted names because averaged ranking was not previously available in Excel at
  all. That asymmetry matters when reading migration guidance — the `.EQ`/`.AVG` pair is one old
  function plus one genuinely new one, not a split of one old function into two.
- **[CORREL](FUNC.CORREL.md)** over two `RANK.AVG` columns is Spearman's rank correlation
  coefficient, and is the reason `RANK.AVG` exists in a spreadsheet. Excel publishes no
  `SPEARMAN`; this composition is the standard route, and it is only correct with averaged ranks.
- **[COUNTIF](FUNC.COUNTIF.md)** reproduces the definition:
  `COUNTIF(ref,">"&number) + (COUNTIF(ref,"="&number)+1)/2` is the descending midrank, without
  the presence check.
- **[PERCENTRANK.INC](FUNC.PERCENTRANK.INC.md)** answers the same question on a `[0, 1]` scale and
  does interpolate for absent values.
- **[SMALL](FUNC.SMALL.md) and [LARGE](FUNC.LARGE.md)**: the inverse identity that holds for
  `RANK.EQ` fails here, because a midrank on tied data is not a valid order-statistic index.

## Numerical notes

`RANK.AVG` does one division, by two, and nothing else. That makes it about as numerically clean
as a worksheet function can be, and it is worth saying why precisely.

**The correction term is exact.** `(m - 1) / 2` with `m` a small integer is a division by a power
of two, which is exact in binary64 for every `m` up to `2^53`. Adding it to the integer-valued
competition rank is also exact, because `r + (m-1)/2` needs at most one bit below the binary
point and `r` is far from the top of the significand. So `RANK.AVG` returns the mathematically
exact midrank, with no rounding, for any list Excel can hold. There is no accuracy question here
at all — which is unusual enough in the statistical category to be worth recording.

**Every numerical question is a comparison question.** As with [RANK.EQ](FUNC.RANK.EQ.md), the
multiplicity `m` and the strict count `r` both come from comparisons of raw binary64 values. The
reference engine uses exact `==` and `>` / `<`. Whether Excel applies its truncation-style
fifteen-significant-digit tolerant comparison here instead is undocumented and unobserved, and
the consequence is larger for `RANK.AVG` than for `RANK.EQ`: a tolerant comparison does not merely
change a match into a miss, it changes the *size of the tied block*, and therefore moves the rank
by a half-integer for every member of that block. `RANK.AVG` over data with near-duplicate values
— the ordinary case for measured data — is exactly where this would show.

**The conservation identity is a usable self-check.** Because the midranks of a complete list sum
to `n(n+1)/2` exactly, and because that sum is computed in exact arithmetic as argued above, a
column of `RANK.AVG` over its own `ref` must sum to exactly `n(n+1)/2` with no tolerance. If it
does not, the tie detection disagreed with itself somewhere. This is a cheap and strong invariant,
and it is the one a vector suite for this function should assert first.

**Cost.** Two passes over `ref` per call — one for the strict count, one for the multiplicity —
so a column of `RANK.AVG` formulas over a shared `ref` is `O(n^2)`.

## What has not been checked

No Handbook vector suite exists for `RANK.AVG`, and no Handbook evidence record names it as a
subject. Nobody has checked this function against Excel within the Handbook's record. The
implementing module carries no tests of its own in the reference engine; the shared rank kernel
it calls carries a small number, which speaks to that kernel rather than to this surface.

Everything above marked as documented comes from Microsoft's `RANK.AVG` page. **Retrieval of that
page was blocked by the upstream host on this pass**, so those statements are recorded as
documented behaviour with the source named and should be re-read against the live page.

Inputs worth probing first:

1. **The conservation identity.** Rank a list of ten values containing a triple tie and a double
   tie against itself, and sum the result. It must be exactly `55`. This single probe exercises
   the strict count, the multiplicity count and the half-integer arithmetic simultaneously, and
   any disagreement with Excel shows up in it.
2. **`RANK.AVG(0.1+0.2, {0.3, 0.3, 1})`** — the tolerant-versus-exact comparison probe. Under
   exact equality this is `#N/A`; under a tolerant comparison it is a midrank over a block of
   three. This is the highest-value unknown on the page.
3. **Even and odd tied blocks**: `RANK.AVG(2, {3,2,2,1})` and `RANK.AVG(2, {3,2,2,2,1})`,
   confirming the half-integer and integer cases respectively.
4. **A constant `ref`**: `RANK.AVG(5, {5,5,5,5})`, which must be `2.5`.
5. **`order` values `0.5`, `-0.5`, `TRUE`, `-1`**, pinning whether truncation precedes the zero
   test.
6. **A non-numeric `number`**, against the `#N/A`-versus-`#VALUE!` split.
7. **`RANK.AVG` against `RANK.EQ`** on unique data, compared bitwise — the identity `m = 1`
   predicts they agree exactly, and that is a cheap consistency check on both.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| midrank | The average of the positions a tied block occupies; what `RANK.AVG` returns |
| fractional ranking | The `1, 2.5, 2.5, 4` convention |
| multiplicity `m` | The number of elements of `ref` equal to `number` |
| competition rank `r` | The [RANK.EQ](FUNC.RANK.EQ.md) value this function corrects |
| conservation identity | Midranks over a complete list sum to `n(n+1)/2` |
| order flag | `order`, read only as zero versus nonzero, after truncation |

## Sources

- Microsoft, *RANK.AVG function* —
  <https://support.microsoft.com/en-us/office/rank-avg-function-bd406a6f-eb38-4d73-aa8e-6d1c3c72e83a>
  (signature, the descending default and the zero/nonzero `order` convention, the averaged-rank
  tie behaviour, and the `#N/A` condition when `number` is absent). Retrieval was blocked by the
  upstream host for this page; the documented behaviour above is stated as documented behaviour
  and should be re-checked against the page.
- OxFunc `crates/oxfunc_core/src/functions/rank_avg_fn.rs` and
  `crates/oxfunc_core/src/functions/rank_common.rs` at commit `473efa3` — the
  `rank_eq + (m-1)/2` construction, the exact-equality multiplicity count, and the `order`
  truncation.
- OxFunc bug stream `docs/bugs/streams/BUG-FUNC-004_numeric_comparison_tolerance_family_split.md`
  — the tolerant-versus-exact numeric comparison split whose applicability here is the open
  question above.
- Handbook [coercion and lifting](../model/02-coercion-and-lifting.md) — the range-scan policy
  governing what `ref` contributes.
- Handbook `data/functions/FUNC.RANK.AVG.json`, `data/presence/FUNC.RANK.AVG.json`.
