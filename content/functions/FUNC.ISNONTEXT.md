---
schema: efh.function-page/v1
function_id: FUNC.ISNONTEXT
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
role_in_family: The negation member — TRUE for every kind except Text, blank cells included.
---

## What it computes

`ISNONTEXT(value)` returns `TRUE` when the value it receives does **not** have kind `Text`, and
`FALSE` when it does.

Microsoft states it as "Value refers to any item that is not text", and adds a parenthetical
that carries more weight than its size suggests: "(Note that this function returns TRUE if the
value refers to a blank cell.)"

That note is the page. `ISNONTEXT` is a negation, so it inherits everything the `Text` kind
excludes:

| Value kind | `ISNONTEXT` |
|---|---|
| `Number` (including dates, times, currency) | `TRUE` |
| `Logical` | `TRUE` |
| `Error` (any code, including `#N/A`) | `TRUE` |
| `Empty` — a blank cell | `TRUE` |
| `Reference` | `TRUE` |
| `Text` (including the zero-length string) | `FALSE` |

The trap is reading `ISNONTEXT` as "has a proper value in it". It does not mean that. It is
`TRUE` for a blank cell and `TRUE` for `#REF!`, which are two of the least value-bearing things
a cell can hold.

## Arguments

`value` — required, exactly one.

Not converted. Microsoft's remark: "The value arguments of the IS functions are not converted."
Since `ISNONTEXT` is defined by negating a kind test, non-conversion means the negation is over
the *kind* delivered, not over any parse attempt.

The argument is a values position: a reference is resolved before the function runs
(`ArgPreparationProfile::ValuesOnlyPreAdapter`).

## Result and edge cases

Returns a `Logical`, or an array of `Logical` when the argument is an array.

- **A blank cell is `TRUE`.** Documented by Microsoft in the function table itself, so this is
  not an inference.
- **The zero-length string is `FALSE`**, because it is `Text`. `ISNONTEXT("")` and
  `ISNONTEXT(<blank cell>)` therefore disagree — the pair is a compact way to distinguish
  `Empty` from `Text("")` without using `ISBLANK`.
- **Errors are `TRUE` and do not propagate.** `ISNONTEXT` is one of the declared per-family
  exceptions to the propagation discipline in
  [coercion and lifting](../model/02-coercion-and-lifting.md). An error input is classified, not
  forwarded — which means `ISNONTEXT(#VALUE!)` is the `Logical` `TRUE`, not `#VALUE!`.
- **Arrays** are classified elementwise into a same-shaped mask.
- **Rich values.** A linked data type's core projection is typically text, which would make
  `ISNONTEXT` `FALSE` for a stock cell. That is not established here.

## Errors

`ISNONTEXT` returns no error of its own for any value it can be given. The only failure
available to it is **arity**: zero arguments, or two. `ISNONTEXT()` is expected to be refused at
formula entry rather than evaluated ([the call pipeline](../model/03-call-pipeline.md)); the
reference engine, having no entry-time surface, reports `#VALUE!` for both the too-few and the
too-many case.

Microsoft's IS-functions page documents no error return for `ISNONTEXT`.

## Relationships

- **`ISTEXT`** is the exact complement, and the two should always be thought of as one function
  with a sign. If you find yourself writing `NOT(ISTEXT(x))`, write `ISNONTEXT(x)`; if you find
  yourself writing `NOT(ISNONTEXT(x))`, write `ISTEXT(x)`.
- **`ISBLANK`** overlaps but is far narrower: `ISBLANK` is `TRUE` only for `Empty`, whereas
  `ISNONTEXT` is `TRUE` for `Empty` *and* five other kinds. `ISNONTEXT` is not a blank test,
  despite the parenthetical that makes it look like one.
- **`ISNUMBER`** is what people usually mean when they reach for `ISNONTEXT`. If the question is
  "is this a usable number?", `ISNUMBER` is the predicate; `ISNONTEXT` will happily say `TRUE`
  about `#DIV/0!`.
- **`T(value)`** returns the empty string for exactly the values `ISNONTEXT` calls `TRUE`.
- **`TYPE`** distinguishes what `ISNONTEXT` lumps together: `1` number, `4` logical, `16` error,
  `64` array. If you need to know *which* non-text thing you have, `TYPE` is the function.

## Notes for implementers

1. **Derive it from `ISTEXT`.** One predicate, one negation. Writing `ISNONTEXT` independently
   guarantees an eventual divergence on some kind added later — and the divergence will be
   silent, because both functions return a plausible boolean either way.
2. **Include `Empty` on the `TRUE` side deliberately**, with the Microsoft parenthetical cited in
   the code comment. It looks like a bug to anyone reading the implementation cold, and someone
   will eventually "fix" it.
3. **Errors must reach the kernel**, or `ISNONTEXT` will forward the error instead of returning
   `TRUE`.
4. **Reference arguments.** `ISNONTEXT` sees values, not references, so the `Reference` row above
   is reachable only through a path that hands the function an unresolved reference — which
   values-only preparation is supposed to prevent. If it happens, something upstream is wrong.

## What has not been checked

No Handbook vector suite exists for `ISNONTEXT`; `vectors/` publishes nothing for this function.
No Excel-comparison evidence record names `ISNONTEXT` as a subject — **nobody has checked this
function's answers against Excel inside the Handbook's record.** The reference engine's own
answers appear on the battery panel under their own label; they are not Excel's answers.

The probes that would settle the open questions:

1. **`ISNONTEXT` and `ISTEXT` over one shared corpus of every value kind**, checked for exact
   complementarity in Excel. The Handbook asserts the pair is complementary; that is a model
   statement, not a measurement.
2. **The blank-cell row.** Microsoft documents `TRUE`; confirming it against a live build costs
   one cell and closes the page's most-cited claim.
3. **Linked data types and in-cell images** — the one input class where the answer could
   plausibly be either.
4. **An error value in each of the extended codes** (`#SPILL!`, `#CALC!`), expected `TRUE`,
   unmeasured.
5. **Arity at entry** — whether Excel refuses `=ISNONTEXT()` at entry or evaluates it.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| negation member | A predicate defined as the complement of another rather than by its own kind |
| blank cell | The `Empty` kind; documented `TRUE` under `ISNONTEXT` |
| complement pair | `ISTEXT` and `ISNONTEXT`: exactly one is `TRUE` for any value |
| mask | The same-shaped array of `Logical` produced from an array argument |

## Sources

- Microsoft, "IS functions" —
  <https://support.microsoft.com/en-us/office/is-functions-0f2d7971-6019-40a0-a171-f2d869135665>.
  Read for this page: the `ISNONTEXT` row including its blank-cell parenthetical, verbatim, and
  the non-conversion remark.
- Handbook, [the value universe](../model/01-value-universe.md) — the kind inventory this
  function negates over.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — the inspection
  functions' exemption from error propagation.
- Handbook, [the call pipeline](../model/03-call-pipeline.md) — values-only preparation and the
  admission boundary.
- `data/functions/FUNC.ISNONTEXT.json` — identity (`xlfIsnontext`, code 190), arity, declared
  axes, as projected at OxFunc `473efa3`.
