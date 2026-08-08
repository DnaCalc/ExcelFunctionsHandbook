---
schema: efh.function-page/v1
function_id: FUNC.LOOKUP
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - The two forms
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
family: lookup_prob_frequency_family
role_in_family: >-
  Finds the largest key not exceeding a search value in sorted data and returns the aligned
  result; the family's ordered-search member, and the oldest lookup in the catalog.
---

## What it computes

`LOOKUP` finds, among a sequence of keys, **the largest key that is less than or equal to the
value you are looking for**, and returns the value aligned with it. It has no exact-match mode
and no not-found argument: approximate is the only behaviour, and `#N/A` is the only miss.

Microsoft documents the requirement that makes it work: the keys must be in **ascending order**.
`LOOKUP` does not verify this. On unsorted data it returns a value — some value — without
complaint. Everything difficult about this function traces back to that one unchecked
precondition.

There is a second reason `LOOKUP` is difficult, and it is unusual in the catalog: **the
Handbook's projection carries no signature for it.** `data/functions/FUNC.LOOKUP.json` records
`signature_placeholder: true` with a null signature, so the generator suppresses the signature
rather than faking one. That is the honest rendering of a function whose two forms take
differently-named arguments in the same positions — and it is a fair warning about how much of
this page is documentation rather than measurement.

## The two forms

Microsoft's documentation presents `LOOKUP` as two forms sharing a name, distinguished by
whether the second argument is a vector or a rectangular array.

### Vector form — `LOOKUP(lookup_value, lookup_vector, [result_vector])`

`lookup_vector` is a single row or a single column of keys, in ascending order. The search finds
the largest key not exceeding `lookup_value`, and the answer is the correspondingly positioned
element of `result_vector`. Microsoft documents that `lookup_vector` and `result_vector` must be
the same size; where they are not, the behaviour is not something to rely on.

The two vectors need not be parallel in orientation or adjacent in the sheet — they can be a row
and a column, or live on different sheets. Alignment is by *position*, not by geometry, which is
what makes the vector form more flexible than `HLOOKUP` and `VLOOKUP` and also what makes a
size mismatch so easy to introduce.

Omitting `result_vector` returns the matched key itself, from `lookup_vector`.

### Array form — `LOOKUP(lookup_value, array)`

One rectangular argument serves as both keys and results. Microsoft documents the axis rule: if
`array` has more columns than rows, the search runs along the **first row** and the result comes
from the **last row**; otherwise — square, or taller than wide — the search runs along the
**first column** and the result comes from the **last column**.

That rule is shape-driven and therefore silent: an array that changes from wide to tall as data
is added changes which axis is searched, with no error and no visible cause. Microsoft's own
documentation says the array form is provided for compatibility with other spreadsheet programs
and recommends `HLOOKUP` or `VLOOKUP` instead. The Handbook agrees, and would put it more
strongly: the array form's shape rule is a trap, and there is no case where it is the clearest
way to express an intention.

## Arguments

| Argument | Meaning |
|---|---|
| `lookup_value` | The value searched for. May be a number, text, a logical, a name, or a reference to one. |
| `lookup_vector` / `array` | The keys. A single row or column in the vector form; a rectangle in the array form, where the shape rule decides the axis. |
| `result_vector` | Vector form only. The values to return, positionally aligned with `lookup_vector` and — per the documentation — the same size. Optional; omitted returns the key. |

The arity is 2 to 3, which is what makes the two forms indistinguishable from the signature
alone: a two-argument call is the array form or a vector form without `result_vector`, and only
the second argument's shape tells you which.

The position readers misjudge most is `result_vector`'s **alignment**. It is positional, not
geometric: the *n*th element of `result_vector` pairs with the *n*th element of
`lookup_vector`, in each vector's own reading order. A row paired with a column works; a
five-element key vector paired with a six-element result vector does not fail loudly.

## Result and edge cases

Return kind: whatever the aligned element holds — number, text, logical, or error.

`LOOKUP` is reference-aware (`arg_preparation_profile: RefsVisibleInAdapter`), so its vectors
arrive as live references and need not be materialized whole to be searched.

- **`lookup_value` below every key** — `#N/A`. This is the documented miss, and it is the *only*
  miss: a value above every key matches the last one, because the rule is "largest key not
  exceeding".
- **Duplicate keys** — which of several equal keys is selected is a consequence of the search
  strategy, and is not established here.
- **Mixed types among the keys.** "Largest not exceeding" needs a total order across numbers,
  text and logicals. Excel has one; where its type boundaries fall, and what that means for a
  mixed key vector, is not established here.
- **Empty cells among the keys** — skipped, treated as a key, or treated as zero — is a real
  question for partially filled vectors, and the answer interacts with the sortedness
  assumption.
- **Errors among the keys.** This case deserves its own paragraph, below.
- **Text keys** compare case-insensitively under Excel's usual convention, and the collation
  governing their order may be locale-dependent. Not established here.

**The `LOOKUP(2, 1/(condition), result)` idiom.** A widely used construction relies on `LOOKUP`
searching a vector that is deliberately full of `#DIV/0!` errors with a search value that
exceeds every non-error entry, to retrieve the *last* row satisfying a condition. Its mechanism
requires that errors be passed over rather than propagated, and that an out-of-range search
value settle on the last valid key even though the vector is not sorted. Microsoft's
documentation does not describe this behaviour, and the Handbook states plainly: this idiom is
folklore that works, its mechanism is not documented, and nothing in the Handbook has verified
it. It is the single most interesting open question on this page, because the idiom is load
bearing in real workbooks and rests on unspecified behaviour.

## Errors

Microsoft's page documents `#N/A` when `lookup_value` is smaller than the smallest key.

Beyond that the documented error surface is thin, and thin for a reason: `LOOKUP`'s failure mode
is usually not an error at all. Unsorted keys, mismatched vector sizes, and the array form's
shape rule all produce *values*. A function that returns the wrong number quietly is worse than
one that errors, and this is the function's defining characteristic.

An error value supplied as `lookup_value` propagates under the universal rule in
[coercion and lifting](../model/02-coercion-and-lifting.md). Errors *inside* the key vector are
the open question described above.

## Relationships

- **`XLOOKUP`** is the modern replacement and supersedes every use of `LOOKUP`: it defaults to
  exact match, takes an explicit not-found value, searches in either direction, supports a
  reverse search mode, and separates the search array from the return array — which is the
  vector form's flexibility with the sizes checked. New code should use it. Microsoft retains
  `LOOKUP` for workbook compatibility.
- **`VLOOKUP` / `HLOOKUP`** are the better-specified relatives, and are what Microsoft's own
  page recommends in place of `LOOKUP`'s array form. They also offer an exact-match mode, which
  `LOOKUP` does not.
- **`MATCH`** with `match_type` 1 implements the same "largest not exceeding" rule and returns
  the position instead of the value; `INDEX`/`MATCH` is the decomposed, checkable version of
  what `LOOKUP` does in one call.
- **`XMATCH`** adds a binary-search mode that makes the sortedness assumption explicit rather
  than implicit — arguably the honest version of `LOOKUP`'s contract.
- **Module siblings.** The reference implementation places `LOOKUP` with `FREQUENCY`, `PROB` and
  `MODE.MULT`. That grouping is about shared ordered-bin machinery, not about Excel's
  categories — `FREQUENCY`'s bin assignment is the same "largest not exceeding" question asked
  of a whole array at once.

## Notes for implementers

- **The search strategy is observable.** Because Excel does not validate sortedness, whatever
  the implementation does on unsorted data *is* the specification. A binary search and a linear
  scan agree on sorted input and disagree on unsorted input, so the choice is not free and
  cannot be made on performance grounds alone. Matching Excel here requires evidence, not
  reasoning.
- **The array form's axis rule uses `>`, not `>=`.** Wide means more columns than rows; square
  falls to the column branch. An implementation that writes the comparison the other way is
  wrong only for square arrays, which is exactly the case a hand-written test is least likely to
  include.
- **Size mismatch between the vectors needs a declared policy**, since the documentation states
  a requirement rather than a behaviour. Silently truncating, silently extending, and erroring
  are all defensible and all different.
- **Errors in the key vector need a declared policy too**, and it is not the ordinary
  propagation rule if the well-known idiom works as reported.
- **Reference-awareness is a performance property here.** Searching a whole-column key vector
  should not materialize a million cells; the shared model records selective dereference as a
  function-local capability rather than a general pipeline one.
- **Do not "improve" `LOOKUP`.** Rejecting unsorted input, or checking vector sizes, would make
  a better function and a worse `LOOKUP` — workbooks that Excel computes would stop computing.

## What has not been checked

No Handbook vector suite exists for `LOOKUP`, and no Excel-comparison evidence record is
recorded for it. Nobody has checked this function's behaviour against Excel here. This page is
almost entirely documentation plus structural reading; the projection does not even carry a
signature.

The probes that would settle the most, in order:

1. **The search strategy, via unsorted keys.** Construct key vectors where a binary search and a
   linear scan give different answers, and record which answer Excel gives — at several
   lengths, since a strategy that switches on size would show up as an inconsistency. This is
   the master probe: the sortedness assumption is unchecked at runtime, so this is what
   determines everything the documentation leaves unsaid.
2. **The `LOOKUP(2, 1/(cond), result)` idiom, dissected.** Separate probes for: a key vector of
   pure errors; a mixed vector of errors and numbers; a search value above every valid key; and
   the full idiom. This establishes whether errors are skipped, and whether the "last match"
   behaviour is a consequence of the search strategy from item 1 or an independent rule.
3. **The array form's axis rule at the square boundary**: a 3×3 array, a 3×4, and a 4×3, with
   keys arranged so the two axes give different answers.
4. **Vector size mismatch**, both directions, with the shorter vector as keys and as results.
5. **Mixed-type key vectors** — numbers, text, logicals, blanks — establishing the order across
   type boundaries, and **case and collation** for text keys.
6. **Duplicate keys**, establishing which occurrence is returned.
7. **A row key vector paired with a column result vector**, confirming that alignment is
   positional rather than geometric.

Items 1 and 2 are the ones that would turn this page from a description of the documentation
into a description of the function.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| vector form | `LOOKUP(lookup_value, lookup_vector, [result_vector])`; keys and results in separate one-dimensional ranges |
| array form | `LOOKUP(lookup_value, array)`; one rectangle, axis chosen by shape |
| shape rule | Wider than tall searches the first row; otherwise the first column |
| largest not exceeding | The match rule: the greatest key ≤ `lookup_value` |
| positional alignment | The *n*th key pairs with the *n*th result, regardless of orientation |
| signature placeholder | The projection carries no signature for this entry and suppresses it rather than faking one |

## Sources

- Microsoft, LOOKUP function —
  <https://support.microsoft.com/en-us/office/lookup-function-446d94af-663b-451d-8251-369d5e3864cb>
  (the vector and array forms, the ascending-order requirement, the same-size requirement for
  the two vectors, the array form's shape rule, the `#N/A` condition when the search value is
  below every key, and the recommendation to prefer `HLOOKUP`/`VLOOKUP` over the array form).
- Handbook `content/model/01-value-universe.md` (value kinds; the error registry).
- Handbook `content/model/02-coercion-and-lifting.md` (error propagation; reference resolution).
- Handbook `content/model/03-call-pipeline.md` (`RefsVisibleInAdapter`; selective dereference as
  a function-local capability).
- Handbook `content/model/06-claim-language.md` (why an undocumented but widely relied-upon
  behaviour is published as an open question rather than as a fact).
- Handbook `data/functions/FUNC.LOOKUP.json` (arity 2–3; `signature_placeholder: true`) and
  `data/presence/FUNC.LOOKUP.json` (implementing module shared with `FREQUENCY`, `PROB` and
  `MODE.MULT`).
