---
schema: efh.function-page/v1
function_id: FUNC.AREAS
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
family: reference_metadata_family
role_in_family: >-
  Counts the rectangular areas inside a reference; the family's structural interrogator, the
  one member whose answer depends on how a reference was assembled rather than where it points.
---

## What it computes

`AREAS` counts the **rectangles** in a reference. A reference in the Excel value model is not
always one block: the union operator (the comma, in the default English locale) joins several
disjoint rectangles into a single multi-area reference, and `AREAS` reports how many rectangles
that union contains.

The count is structural, not geometric. `AREAS` does not merge rectangles that happen to be
adjacent, and it does not deduplicate rectangles that overlap — it reports how the reference was
built. Two rectangles that together tile a larger rectangle still count as two.

A single cell and a single rectangular range are both one area, so the answer is at least 1 for
any reference. The interesting inputs are all unions.

## Arguments

| Argument | Meaning |
|---|---|
| `reference` | A reference, or a name that resolves to one. Required, and it must genuinely be a reference — a value cannot be counted. |

The one thing every reader gets wrong once: **a union written inline needs a second set of
parentheses.** `AREAS(A1:C4,E5)` is not a one-argument call with a union in it; it parses as two
arguments, which is outside the function's arity of exactly 1. The union must be parenthesized
into a single argument — `AREAS((A1:C4,E5))` — so that the comma is read by the reference
algebra rather than by the argument list. Microsoft's page makes the same point in its examples.
This is a formula-grammar collision, not a quirk of `AREAS`: it applies to any function that
takes a reference in a position where a comma would otherwise separate arguments.

## Result and edge cases

Return kind: `Number` — a positive integer count.

`AREAS` is one of the reference-aware functions: the projection records
`arg_preparation_profile: RefsVisibleInAdapter`, meaning the live reference survives into the
function instead of being dereferenced to values first. That is not a detail; it is the whole
function. If references were resolved before the call, as they are for the majority profile
described in [the call pipeline](../model/03-call-pipeline.md), the multi-area structure would
be gone before `AREAS` could count it.

Boundary cases specific to this function, none of which the Handbook has established for Excel:

- **A whole-column or whole-row reference** (`A:A`, `3:3`) — one area, presumably, but the
  answer is worth recording rather than assuming.
- **A three-dimensional reference** (`Sheet1:Sheet3!A1`) — does the sheet span multiply the
  count, or is it one area spanning sheets? The [value universe](../model/01-value-universe.md)
  lists three-dimensional as its own reference shape, which is precisely why the answer is not
  derivable from the other shapes.
- **A spill anchor** (`B1#`) — a reference shape that postdates the function by decades.
- **A structured reference** (`Table1[Amount]`), including the multi-column and
  non-contiguous-column forms, which can designate more than one rectangle.
- **Overlapping and adjacent unions**, which test whether the count is structural (as stated
  above) or normalized.

## Errors

Microsoft's page documents the argument as a reference to a cell or range of cells that can
refer to multiple areas. It does not publish an error table. The expected failure is the
non-reference argument — `AREAS(5)`, `AREAS("A1:B2")` — for which `#VALUE!` is the conventional
code in this family; the Handbook records that as expected, not as verified. An error value
supplied as the argument propagates under the universal rule in
[coercion and lifting](../model/02-coercion-and-lifting.md).

Note that text that *looks* like a reference is not a reference. `AREAS("A1:B2")` asks the
function to count areas in a string. If you have address text, `INDIRECT` is the conversion
step, and it brings its own consequences.

## Relationships

- **`ROWS` and `COLUMNS`** measure a reference's extent; `AREAS` measures its cardinality. The
  three together describe a reference's shape.
- **`INDEX`'s reference form** is the main consumer of multi-area references: its fourth
  argument, `area_num`, selects which area to index into. `AREAS` is how you find out how many
  there are to choose from. The pairing `INDEX(ref, r, c, AREAS(ref))` — take the last area —
  is the idiom that makes `AREAS` more than a curiosity.
- **The union operator** (`FUNC.OP_UNION_REF` in the Handbook's operator catalog) is what
  creates the structure `AREAS` reports. The intersection operator can also produce references,
  and whether an intersection result ever has more than one area is an open question here.
- **`OFFSET` and `INDIRECT`** produce references at evaluation time; whether a reference so
  produced can carry multiple areas is a further open question.

## Notes for implementers

- **Reference structure must be preserved end to end.** Any implementation that normalizes a
  reference — sorting, merging adjacent rectangles, deduplicating overlaps — before `AREAS`
  sees it will produce a different, and probably wrong, count. This is the standard trap for
  engines that canonicalize references early for dependency tracking.
- **The comma is context-sensitive** in the formula grammar: argument separator at one level,
  union operator inside a parenthesized reference expression. The argument separator is
  locale-dependent (semicolon in many locales) while the union operator's spelling is tied to
  it. A formula parser has to get the interaction right or `AREAS((A1,B1))` will not survive a
  locale round trip. Formula grammar is out of the Handbook's declared scope, but the collision
  lands on this function's doorstep.
- The function is classified `fec_dependency_profile: RefOnly` — it needs the reference
  facility but nothing else about the workbook: no cell values are read, so no dereference of
  the areas is required to answer.

## What has not been checked

No Handbook vector suite exists for `AREAS`, and no Excel-comparison evidence record is recorded
for it. Nobody has checked this function against Excel here.

What to probe first, and why each one is a distinct question rather than a variation:

1. **`AREAS((A1:C4,E5,G7:H9))`** — the base case that establishes that inline unions count as
   expected, and that the double-parenthesis grammar behaves as Microsoft's example implies.
2. **A defined name bound to a multi-area reference**, passed by name. This separates
   "the parser built a union at this call site" from "the reference value carries multiple
   areas", which are different mechanisms with the same surface.
3. **Adjacent and overlapping unions** — `(A1:B2,C1:D2)` and `(A1:C3,B2:D4)`. These decide
   whether the count is structural or normalized, the single most consequential unknown on this
   page.
4. **Three-dimensional, structured, and spill-anchor references**, one probe each. Each is a
   distinct reference shape in the value model and none of them can be inferred from the
   others.
5. **Non-reference arguments**: a number, a text address, an array literal, an error. These pin
   the error surface.

Item 3 is the one that would change how the function is described, not merely what its edge
cases return.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| area | One rectangular block inside a reference |
| multi-area reference | A reference built by the union operator from two or more rectangles |
| union operator | The comma (locale-dependent) joining references into one value |
| reference-aware | The function receives the live reference, not its values (`RefsVisibleInAdapter`) |
| structural count | A count that reflects how the reference was built, not its merged geometry |

## Sources

- Microsoft, AREAS function —
  <https://support.microsoft.com/en-us/office/areas-function-8392ba32-7a41-43b3-96b0-3695d2ec6152>
  (argument definition, and the double-parenthesis requirement for inline unions).
- Handbook `content/model/01-value-universe.md` (reference shapes: area, multi-area,
  three-dimensional, structured, spill anchor).
- Handbook `content/model/02-coercion-and-lifting.md` (error propagation; references at the
  call boundary).
- Handbook `content/model/03-call-pipeline.md` (`RefsVisibleInAdapter`; operators as
  functions).
- Handbook `data/functions/FUNC.AREAS.json` and `data/presence/FUNC.AREAS.json` (arity,
  classification axes, implementing module).
