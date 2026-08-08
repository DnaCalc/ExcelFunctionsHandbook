---
schema: efh.function-page/v1
function_id: FUNC.MODE.SNGL
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: mode_sngl_fn
role_in_family: >-
  The single-valued mode: it returns one most-frequent value from a data set, and is the member
  that must silently resolve the ambiguity that MODE.MULT refuses to resolve.
---

# MODE.SNGL

## What it computes

`MODE.SNGL(number1, [number2], …)` returns a most frequently occurring value in the data.

Write the admitted values as a multiset `S` over the reals, and let `m(v)` be the multiplicity
of `v` in `S`. Define

    M = max { m(v) : v in S }
    A = { v in S : m(v) = M }        the set of modes

Then:

- if `M = 1` — every value occurs once — the mode is not defined and the function reports
  `#N/A`;
- if `M > 1` and `|A| = 1`, the answer is the unique element of `A`;
- if `M > 1` and `|A| > 1`, **the mathematics does not pick a winner and the function must**.

That third line is the whole substance of this page. The mode is the one classical measure of
central tendency that is not a function of the data in the ordinary sense: it is
set-valued. `MEDIAN` resolves its even-`n` ambiguity by a stated rule (average the two middle
values). `MODE.SNGL` resolves a genuine multiplicity of maxima by an unstated one, and the
selection rule is the fact a reader most needs and is least likely to find written down.

The mode is also not a statistic of the underlying distribution in any stable sense. For a
continuous population the sample mode is almost surely `#N/A` (no value repeats), and where it
is not, it is an artefact of rounding or of a measurement grid rather than of the distribution.
`MODE.SNGL` is a function about the *recorded* data, not about the process that produced it.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number1` | The first argument: a number, a reference, an array, or a name. Required. | — |
| `number2`, … | Further arguments of the same kinds. Optional, repeating. | — |

The reference engine records an arity of one to 255 arguments and a `NumsToNum` kernel
signature — the function reduces a numeric multiset to a single number. It also records the
`Custom` coercion/lift profile rather than a standard aggregate scan policy, which is a signal
that the admission of text, logicals and empty cells is decided by this function rather than
inherited.

Which values reach the multiset is the usual worksheet asymmetry, not something this page
should restate: direct scalar arguments and values reached by scanning a range obey different
rules. See [Coercion and lifting](../model/02-coercion-and-lifting.md), which states the
asymmetry once and explains why there is deliberately no global precedence rule.

## Result and edge cases

Returns `Number`.

- **No repeated value.** The documented result is `#N/A`. This is the common case for
  measured data and it is not an error in the sense of a mistake; it is the correct report
  that the sample has no mode.
- **A single value.** A one-element data set has no repeat, so the no-mode branch applies.
  The reference engine's battery renders exactly this case and its result is shown beside
  this page.
- **Ties among several modes.** The selection rule is not stated on Microsoft's page and this
  Handbook does not know it. The two natural candidates are *smallest tied value* and *first
  tied value in argument order*, and they are distinguishable with a two-argument probe. See
  the probe list below.
- **Equality of numbers.** Counting multiplicities requires deciding when two doubles are
  "the same value". Excel applies a tolerant, truncation-style comparison in some families and
  exact comparison in others; OxFunc's `BUG-FUNC-004` records that split explicitly for the
  lookup and comparison families. Which side `MODE.SNGL` falls on has not been established
  here, and it is directly observable: under exact comparison `0.1+0.2` and `0.3` are two
  distinct values with multiplicity one each, and under the tolerant comparison they are one
  value with multiplicity two.
- **Arrays.** `MODE.SNGL` is an aggregate, not a lift kernel: an array argument is consumed by
  scanning, and the function returns a scalar.

## Errors

| Error | Condition |
|---|---|
| `#N/A` | The data set contains no value that occurs more than once |
| `#VALUE!` | A direct argument does not convert to a number |
| propagated | An error value among the admitted values surfaces as that error |

The `#N/A` row is documented by Microsoft on the page cited in Sources. Retrieval of that page
was refused by the upstream host while this page was written, so the row is published as
documented behaviour with its source named and should be re-checked against the page. The
other two rows are the ordinary coercion rules of
[Coercion and lifting](../model/02-coercion-and-lifting.md) rather than anything specific to
this function, and the Handbook has not observed either in Excel.

## Relationships

- **`MODE`** is the legacy spelling. Both carry the same argument count in the reference
  engine's registry — one to 255 — so unlike several other modernized statistical pairs, this
  rename does not change the signature. **That is not the same as saying the two compute the
  same bits.** Microsoft's compatibility functions are separately dispatched surfaces; whether
  `MODE` and `MODE.SNGL` share a code path inside Excel is an empirical question, and the
  reference engine's own presence projection puts them in different implementation modules.
  Proving identity requires evidence, and none is attached here.
- **[MODE.MULT](FUNC.MODE.MULT.md)** is the honest sibling: it returns the whole set `A` as a
  vertical array, which is what the mathematics actually produces. Where `MODE.SNGL` hides a
  tie, `MODE.MULT` exposes it. If you care whether your data has one mode or several, that is
  the function to use, and `COUNT(MODE.MULT(…))` is the direct test.
- **`MEDIAN`** and **`AVERAGE`** are the other two classical centres. Only `MODE.SNGL` can fail
  to exist; only `MODE.SNGL` is unchanged by an arbitrary monotone relabelling of the values;
  only `AVERAGE` uses every observation.
- **Confused with**: `MAX` (largest value, not most frequent) and `COUNTIF` (how many times a
  *given* value occurs, which is `m(v)` for one `v` rather than the argmax over `v`).

## Numerical notes

There is no floating-point approximation here — no series, no argument reduction, no
cancellation. Everything difficult about `MODE.SNGL` is discrete.

1. **The equality predicate is the algorithm's only real decision.** Multiplicity counting is
   parameterized by "same value", and a tolerant comparison is not an equivalence relation on
   the reals: if `a ≈ b` and `b ≈ c` under a fixed absolute-or-relative tolerance, `a ≈ c` can
   fail. An implementation that groups by a tolerant predicate therefore has to define what a
   group *is* — chaining, or clustering around a representative — and different definitions
   give different multiplicities on the same data. An implementation that groups by exact bits
   has no such problem and a different one: values that a user regards as equal are counted
   apart.
2. **The tie-break falls out of the data structure, not out of a decision.** A sort-then-scan
   implementation naturally reports the smallest tied value; a hash-map implementation reports
   whichever key the iteration order surfaces; a first-pass-wins scan reports the earliest in
   argument order. All three are `O(n log n)` or better and all three are defensible, and they
   disagree. This is the classic case where an implementation detail becomes an observable
   semantic, and the only correct response is to pin the rule deliberately and test it.
3. **`-0.0` and `0.0`** compare equal under IEEE 754 `==` but have different bit patterns. A
   bitwise grouping separates them; a numeric grouping does not. The same question arises for
   values arriving from different cells with different formats.
4. **Stability under argument regrouping.** `MODE.SNGL(A, B)` and `MODE.SNGL(B, A)` should
   agree; under a first-encountered tie-break they need not. That is a cheap invariant to test
   and a cheap one to get wrong.

## What has not been checked

No Handbook vector suite exists for `MODE.SNGL`, and no Handbook evidence record names this
surface. Nobody has compared this function against Excel within the Handbook's record. The
`#N/A`-when-no-duplicate rule is documented by Microsoft; the tie-break rule and the equality
predicate are documented nowhere the Handbook has found, and they are the two facts that
determine the answer.

Inputs worth probing first, in order of what they settle:

1. **`MODE.SNGL({1,1,2,2})` and `MODE.SNGL({2,2,1,1})`.** Two modes, tied, presented in both
   orders. If both calls return `1`, the rule is smallest-wins; if they return `1` and `2`,
   the rule is first-encountered. Two cells settle the single most important undocumented
   fact about this function.
2. **`MODE.SNGL({-1,-1,1,1})`** — the same test with a sign, which distinguishes
   smallest-wins from smallest-in-magnitude-wins.
3. **`MODE.SNGL({0.3, 0.1+0.2, 0.3})` versus `MODE.SNGL({0.1+0.2, 0.3})`.** The tolerant
   versus exact comparison question, using the same witness pair that `BUG-FUNC-004` uses for
   the lookup families. If the second call returns a value rather than `#N/A`, the comparison
   is tolerant.
4. **`MODE.SNGL({-0,0})`** — whether negative zero is a distinct value for counting purposes.
5. **A range containing text that reads as a number, logicals, and empty cells**, against the
   same values passed directly as arguments. This is the direct-versus-scan asymmetry, and
   `MODE.SNGL`'s `Custom` coercion profile means it cannot be predicted from any other
   function's behaviour.
6. **An error value inside the scanned range** — whether it propagates or is skipped.
7. **`MODE.SNGL` against `MODE`** on every probe above, which is the only way to turn the
   legacy-alias question from an assumption into a finding.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| multiplicity | The number of times a value occurs in the admitted data |
| mode set | The set `A` of all values attaining the maximum multiplicity |
| tie-break rule | The undocumented rule by which one element of `A` is returned |
| equality predicate | The test by which two admitted numbers are counted as the same value |
| no-mode branch | The `#N/A` result when every admitted value occurs exactly once |

## Sources

- Microsoft, "MODE.SNGL function" —
  <https://support.microsoft.com/en-us/office/mode-sngl-function-f1267c16-66c6-4386-959f-8fba5f8bb7f8>
  (signature and the no-duplicate `#N/A` rule). Retrieval was refused by the upstream host
  when this page was written; documented behaviour above is stated as such and should be
  re-checked against the page.
- Handbook, [The value universe](../model/01-value-universe.md) — value kinds and the
  raw-versus-published boundary.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — the
  direct-argument versus range-scan asymmetry and error propagation.
- `data/functions/FUNC.MODE.SNGL.json` — arity one to 255, `NumsToNum` kernel signature,
  `Custom` coercion/lift profile, `ErrorCollapseProfile::None`, XLL symbol `xlfMode_sngl`.
- `data/presence/FUNC.MODE.SNGL.json` — implementing module
  `crates/oxfunc_core/src/functions/mode_sngl_fn.rs`, shared with no other surface.
- OxFunc bug stream `docs/bugs/streams/BUG-FUNC-004_numeric_comparison_tolerance_family_split.md`
  — the tolerant-versus-exact numeric comparison split, cited here as the source of the probe
  witness rather than as evidence about this function.
