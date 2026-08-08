---
schema: efh.function-page/v1
function_id: FUNC.ISBLANK
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
  - Page vocabulary
  - Sources
family: is_predicates_family
role_in_family: The Empty-kind test; the only member whose answer turns on the value kind
  that never survives to a published formula result.
---

## What it computes

`ISBLANK(value)` answers exactly one question about a value's **kind**: is it `Empty`?

`Empty` is the value-universe kind defined in
[the value universe](../model/01-value-universe.md) as "the state of a cell that has no
content" — a value-shaped stand-in for *nothing is here*. `ISBLANK` returns `TRUE` when and
only when the value it receives carries that kind, and `FALSE` for every other kind: `Number`,
`Text`, `Logical`, `Error`, `Array`, `Reference`.

The distinction that makes this function worth a page is that **"blank" is a kind, not an
appearance**. Four things a spreadsheet user would call blank are not `Empty`:

| What is in the cell | Kind delivered | `ISBLANK` |
|---|---|---|
| Nothing — never typed into, or cleared | `Empty` | `TRUE` |
| The empty string typed as `'` or entered as `""` | `Text` (length 0) | `FALSE` |
| A formula `=""` that publishes an empty string | `Text` (length 0) | `FALSE` |
| A formula whose result is `0` shown as blank by a number format | `Number` | `FALSE` |

The third row is the one that costs people afternoons: a helper column of `=IF(x,"",y)` is a
column of zero-length text, and `ISBLANK` will disagree with the eye on every row of it.

Microsoft states the condition as "Value refers to an empty cell." The Handbook's restatement
in terms of the `Empty` kind is the same rule made precise enough to implement, and it is what
lets the page say something definite about `""`, which the one-line documentation does not
address.

## Arguments

`value` — required, exactly one. There is no optional second argument and no variadic tail.

The argument is **not converted**. Microsoft's IS-functions remark is explicit: "The value
arguments of the IS functions are not converted." `ISBLANK` inspects the kind that arrives and
nothing more; there is no coercion attempt whose failure could change the answer. See
[coercion and lifting](../model/02-coercion-and-lifting.md) for what "not converted" excludes.

The argument position is a values position, not a reference position: a reference is resolved
before `ISBLANK` sees it (`ArgPreparationProfile::ValuesOnlyPreAdapter`), so `ISBLANK` cannot
distinguish `ISBLANK(A1)` from `ISBLANK(<the value in A1>)`. This matters for the contrast with
`ISREF`, which is the family's one reference-visible member.

## Result and edge cases

Returns a `Logical`, or an array of `Logical` when the argument is an array.

- **Omitted versus empty.** `Empty` and `Missing` are different kinds
  ([value universe](../model/01-value-universe.md), "Missing versus Empty"). `ISBLANK`'s
  documented subject is the empty *cell*. What the function returns for a `Missing` marker
  delivered by an empty argument slot is not something the Handbook has established.
- **An error argument does not propagate.** `ISBLANK` is a kind test, so an `Error` value is a
  value it classifies rather than a failure it forwards; the answer for an error input is
  `FALSE`, not the error. This is the per-family exception to error propagation named in
  [coercion and lifting](../model/02-coercion-and-lifting.md).
- **Arrays.** The reference engine classifies an array argument elementwise and returns an array
  of the same shape. Whether Excel's spill result agrees element for element on every kind mix
  has not been checked here.
- **Zero-length text.** `FALSE`, per the table above. This is the single most consequential
  edge case on the page.

## Errors

`ISBLANK` has no domain over which it can fail: every value kind has an answer. The only error
the Handbook can attribute to the function itself is an **arity** failure — a call with zero
arguments or with two. `ISBLANK()` is expected to be refused at formula entry rather than
evaluated (see [the call pipeline](../model/03-call-pipeline.md), "Arity and the admission
boundary"); the reference engine, which has no entry-time surface, reports `#VALUE!` for both
the too-few and the too-many case.

Microsoft's IS-functions page documents no error return for `ISBLANK`.

## Relationships

- **`ISNONTEXT`** also returns `TRUE` for an empty cell — Microsoft says so in the function
  table itself ("Note that this function returns TRUE if the value refers to a blank cell").
  `ISBLANK` and `ISNONTEXT` therefore agree on `Empty` and disagree on every non-text non-empty
  kind. If you want "nothing here", `ISBLANK` is the narrow test; `ISNONTEXT` is not.
- **`COUNTBLANK`** counts cells over a range using its own blank predicate, which is documented
  to include formulas returning `""`. `ISBLANK` does not. Two functions, two definitions of
  blank, in the same workbook.
- **`ISOMITTED`** is the `Missing`-side counterpart, but only inside a `LAMBDA`; it is not a
  general "was this argument left out" test at the worksheet level.
- **`N`** maps both `Empty` and `Text` to `0`, which is why `N(A1)=0` cannot be used as a blank
  test.

## Notes for implementers

1. **Do not fold `Empty` into `Text("")` or into `Number(0)` upstream of this function.** An
   engine that normalizes empties early will get `ISBLANK` wrong and will not be able to get it
   right again, because the information is gone. The `Empty` kind must survive argument
   preparation intact.
2. **`Empty` is not admitted at the published-result boundary** but *is* admitted at the
   call-argument boundary (the admission matrix in
   [the value universe](../model/01-value-universe.md)). `ISBLANK` is one of the functions that
   makes that asymmetry observable from the sheet.
3. **Reference resolution happens first.** A single-cell reference materializes as a 1x1 array
   and is collapsed to the scalar inside it; a reference to an untouched cell must collapse to
   `Empty`, not disappear.
4. **Arrays are classified, not coerced.** The element-local rule from
   [coercion and lifting](../model/02-coercion-and-lifting.md) applies: an element that is an
   error is classified `FALSE`, it does not collapse the array.

## What has not been checked

No Handbook vector suite exists for `ISBLANK`; `vectors/` publishes nothing for this function,
so no suite-scoped claim is available. No Excel-comparison evidence record names `ISBLANK` as a
subject, which means **nobody has checked this function's answers against Excel inside the
Handbook's record**. The reference engine's own answers are shown on this page's battery panel
under their own label; they are not Excel's answers and are not offered as agreement.

The probes that would settle the open questions, in the order the Handbook would run them:

1. `ISBLANK` of a cell holding `=""` versus a genuinely untouched cell, in the same workbook —
   this is the claim the page rests on and it has not been re-observed here.
2. `ISBLANK` of an omitted argument slot inside a `LAMBDA` body, to pin the `Missing` answer
   that the documentation does not state.
3. `ISBLANK` over a multi-cell reference under dynamic arrays versus under implicit
   intersection, since the spill answer and the legacy answer need not be the same value.
4. `ISBLANK` of a cell containing a linked data type or an in-cell image, to find out which
   side of the predicate a rich value's core projection lands on.
5. `ISBLANK` of a deleted-then-restored cell and of a cell cleared with Delete versus with a
   space, to confirm that "cleared" really produces `Empty`.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| `Empty` | The value kind of a cell with no content; the only kind `ISBLANK` answers `TRUE` for |
| `Missing` | The omitted-argument marker; a different kind, live only at the call boundary |
| zero-length text | `Text` whose length is 0; not `Empty`, and `ISBLANK` returns `FALSE` for it |
| kind test | A predicate that classifies the value kind it receives instead of converting it |

## Sources

- Microsoft, "IS functions" —
  <https://support.microsoft.com/en-us/office/is-functions-0f2d7971-6019-40a0-a171-f2d869135665>.
  Read for this page: the function table row ("Value refers to an empty cell") and the remark
  that IS-function arguments are not converted.
- Handbook, [the value universe](../model/01-value-universe.md) — the `Empty` kind, the
  Missing/Empty distinction, and the boundary admission matrix.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — non-conversion,
  error propagation as a per-family policy, element-local array failures.
- Handbook, [the call pipeline](../model/03-call-pipeline.md) — values-only argument
  preparation and the admission-versus-runtime boundary.
- `data/functions/FUNC.ISBLANK.json` — identity, arity, declared axes, and the
  Microsoft-verbatim short description, as projected at OxFunc `473efa3`.
