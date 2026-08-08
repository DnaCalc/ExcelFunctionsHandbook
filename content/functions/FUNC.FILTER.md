---
schema: efh.function-page/v1
function_id: FUNC.FILTER
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
  - Notes for implementers
  - What has not been checked
family: dynamic_array_reshape_family
role_in_family: >-
  Selects the rows or columns of an array for which a parallel boolean array is true; the
  family's only predicate-driven selector, and the one member with an explicit empty-result
  argument.
---

## What it computes

`FILTER(array, include, [if_empty])` keeps the slices of `array` for which the corresponding
element of `include` is true, and drops the rest. Order is preserved: the surviving slices come
back in their original sequence.

The word doing the work is *slices*. `FILTER` does not select individual cells; it selects whole
rows or whole columns, and **which axis it filters is decided by the shape of `include`, not by
an argument**:

- `include` shaped like a single column with one entry per row of `array` — rows are filtered;
- `include` shaped like a single row with one entry per column of `array` — columns are
  filtered.

That is the single most important thing to understand about this function, and it is the source
of most `#VALUE!` results people meet with it: the criterion array must be parallel to the axis
you mean to filter, and getting it transposed does not filter the other way, it fails.

`include` is an array of truth values, typically produced by a comparison over a range —
`(A2:A100="North")` — which is itself an elementwise lifted operation
([coercion and lifting](../model/02-coercion-and-lifting.md)). Combining criteria is arithmetic
on those arrays: `*` for AND, `+` for OR. `FILTER` supplies no criteria language of its own,
which is why it composes so freely and why it has no equivalent of the `COUNTIF` family's
criteria strings.

## Arguments

| Argument | Meaning |
|---|---|
| `array` | The array or range to filter. Required. |
| `include` | A boolean array parallel to the filtered axis of `array`. Required. |
| `if_empty` | The value to return when nothing passes the filter. Optional. |

Notes on each:

- **`array`** is prepared under `ValuesOnlyPreAdapter`, so a reference is resolved to its values
  before the function runs; the result is values and carries no reference structure.
- **`include`** must have exactly one entry per row (or per column) of `array`. Non-logical
  values are interpreted by ordinary to-logical rules — zero is false, non-zero true — but which
  contexts accept text logicals is per-family policy in the shared model rather than a universal
  rule, and the Handbook has not established `FILTER`'s policy.
- **`if_empty`** is the argument people omit and then wish they had not. Without it, an empty
  result is an error (below). With it, `FILTER(…, …, "")` or `FILTER(…, …, "none")` degrades
  gracefully. It accepts any scalar; whether it accepts an array is not established here.

## Result and edge cases

Return kind: `Array` — the surviving rows (or columns), spilled. The result's width equals
`array`'s width when filtering rows, and its height equals `array`'s height when filtering
columns.

- **Every row passes** — the result equals the input.
- **Exactly one row passes** — a one-row array, not a scalar. Downstream code that expects a
  scalar should not assume; use `@` or a reducer.
- **No row passes** — the empty case, handled by `if_empty` or by `#CALC!` (below).
- **Errors inside `array`** in surviving rows are carried through as values; the projection
  records `error_collapse_profile: None`.
- **Errors inside `include`** are a different matter: an error in the criterion array has no
  truth value. What `FILTER` does with one is not established here, and it is a case real
  workbooks hit constantly, because criterion arrays are usually computed.
- **A `FILTER` over a whole-column reference** carries the whole column's blanks into the
  criterion, which is why the idiom is usually paired with a non-blank test.

## Errors

Microsoft's page documents two conditions:

- `#CALC!` when no entries in `include` are true and `if_empty` is omitted. This is the
  empty-array case: the value model admits no empty array at the published boundary
  ([value universe](../model/01-value-universe.md)), so "nothing matched" has to surface as an
  error unless you supplied a substitute.
- `#VALUE!` when `include` is not of a dimension compatible with `array`.

The `#CALC!` case is the one that changes how formulas must be written: any `FILTER` whose
criterion depends on data should either carry an `if_empty` or be wrapped. Note that `FILTER`
is unusual in the reshape family for offering `if_empty` at all — `DROP` and its siblings give
you `#CALC!` with no substitute.

An error value supplied as an argument propagates under the universal rule in
[coercion and lifting](../model/02-coercion-and-lifting.md).

## Relationships

- **`SORT` and `UNIQUE`** are the other two members of the classic dynamic-array trio;
  `FILTER` then `SORT` then `UNIQUE` is the canonical query pipeline, and all three live in the
  same reference-implementation module.
- **`CHOOSEROWS` / `CHOOSECOLS`** select by index where `FILTER` selects by predicate. When the
  predicate is really "positions 1, 4 and 9", the index functions say it directly.
- **`TAKE` / `DROP`** select contiguous edge bands, which is the other common special case.
- **Advanced Filter and the database functions (`DSUM` and family)** are the pre-dynamic-array
  answers to the same question, with criteria expressed as a range of criteria rows rather than
  as a boolean array.
- **`XLOOKUP`** answers "the one matching row" where `FILTER` answers "all matching rows".
  Reaching for `FILTER` and then taking the first row is a common but weaker idiom, because it
  computes everything to use one.
- **`SUMIFS` / `COUNTIFS`** aggregate under criteria without materializing the matching rows;
  when you only want a total, they do less work and produce no spill.

## Notes for implementers

- **Axis inference from `include`'s shape is the crux.** The rule has to distinguish a
  column-shaped criterion from a row-shaped one, and to reject anything that is neither — with
  `#VALUE!`, per the documentation. A 1×1 criterion is ambiguous for a 1×1 array and needs a
  declared policy.
- **The empty result must be intercepted before the array is constructed**, exactly as in
  `DROP`: a zero-row array is not representable, so the `if_empty`-or-`#CALC!` decision belongs
  at the point where the survivor count is known.
- **Truth-value interpretation of `include` is per-family policy**, not engine law. The shared
  model is explicit that to-logical behaviour is distributed across functions rather than
  centralized, so an implementation must declare its rule for numbers, text, blanks and errors
  in the criterion rather than inheriting one.
- **Stability.** Preserving input order is part of the semantics, not an implementation
  accident; any parallel or partitioned implementation must be order-preserving.
- **Category oddity worth knowing about the data layer**: the projection records `FILTER`'s
  category as "Our 10 featured functions" rather than "Lookup and reference functions". That is
  Microsoft's own catalogue placement carried through the sync, not a Handbook classification —
  and it means category-driven navigation will not find `FILTER` where its siblings live.

## What has not been checked

No Handbook vector suite exists for `FILTER`, and no Excel-comparison evidence record is
recorded for it. Nobody has checked this function against Excel here.

The probes that matter most, because each one covers a case ordinary workbooks hit:

1. **An error value inside `include`** — for example a criterion array containing `#DIV/0!`.
   Does the whole call error, does that row drop, or does it survive? This is the largest
   practical unknown on the page.
2. **Non-logical criterion values**: numbers other than 0 and 1, text `"TRUE"`, empty cells, and
   the empty string. The to-logical policy is explicitly per-family in the shared model, so
   there is nothing to infer.
3. **Dimension mismatch in both directions** — a criterion one element too long, one too short,
   and correctly sized but transposed — to pin the documented `#VALUE!` condition.
4. **The empty result with and without `if_empty`**, and with `if_empty` given as an array
   rather than a scalar.
5. **Column filtering**, which is the less-used mode and therefore the less-exercised path:
   a row-shaped `include` over a wide array.
6. **A 1×1 `array` with a 1×1 `include`**, the ambiguous-axis case.

Items 1 and 2 would change the "Arguments" section; the rest confirm documented behaviour.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| criterion array | The `include` argument: a boolean array parallel to the filtered axis |
| slice | A whole row or whole column, the unit `FILTER` keeps or drops |
| axis inference | Deciding whether to filter rows or columns from `include`'s shape |
| empty result | No slice passes; surfaces as `if_empty` or `#CALC!` |
| order-preserving | Survivors are returned in their original sequence |

## Sources

- Microsoft, FILTER function —
  <https://support.microsoft.com/en-us/office/filter-function-f4f7cb66-82eb-4767-8f7c-4877ad80c759>
  (argument definitions, the `if_empty` substitute, the `#CALC!` empty-result condition, and the
  `#VALUE!` dimension-incompatibility condition).
- Handbook `content/model/01-value-universe.md` (`#CALC!`; no empty array at the published
  boundary).
- Handbook `content/model/02-coercion-and-lifting.md` (to-logical as per-family policy;
  elementwise lifting of comparisons; error propagation).
- Handbook `content/model/03-call-pipeline.md` (`ValuesOnlyPreAdapter`; host-side spill
  adaptation).
- Handbook `data/functions/FUNC.FILTER.json` and `data/presence/FUNC.FILTER.json` (arity,
  category placement, classification axes, shared reshape-family module).
