---
schema: efh.function-page/v1
function_id: FUNC.EXPAND
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
  Pads an array out to a requested rectangle with a fill value; the family's only growing
  reshaper, and the standard way to make mismatched arrays conformable.
---

## What it computes

`EXPAND(array, rows, [columns], [pad_with])` returns an array of the requested dimensions whose
top-left region is `array` and whose remaining cells are all `pad_with`.

The operation is one-directional: it grows, it never shrinks. The original elements keep their
positions relative to the top-left corner, and every new cell — to the right, below, and in the
bottom-right corner block — receives the same pad value. Microsoft documents the default
`pad_with` as `#N/A`, which is a deliberate choice rather than a convenience: `#N/A` is the
value model's "not available" marker, so an unpadded `EXPAND` produces a result whose synthetic
cells announce themselves rather than masquerading as zeros or blanks.

`EXPAND` is the growing complement of `TAKE`/`DROP` and the shape-fixing tool for the whole
dynamic-array family: when two arrays must be conformable — for `HSTACK` alignment, for an
elementwise operation, for a fixed-size output block — `EXPAND` is how you make them so
explicitly instead of relying on broadcasting.

## Arguments

| Argument | Meaning |
|---|---|
| `array` | The array or range to pad. Required. |
| `rows` | The number of rows the result should have. Required. |
| `columns` | The number of columns the result should have. Optional; omitted keeps the array's current column count. |
| `pad_with` | The value to fill new cells with. Optional; Microsoft documents the default as `#N/A`. |

Two positions are commonly misread.

**`rows` and `columns` are target dimensions, not increments.** `EXPAND(a, 3)` means "make it
three rows tall", not "add three rows". This is the opposite convention from `DROP` and `TAKE`,
whose counts *are* increments, and the inconsistency is the most reliable source of error when
the two are used in the same formula.

**Omitting `columns` is not the same as passing the current width** in one respect: it is
shorter and it tracks a changing input, but the documented effect is the same — the column axis
is left as it is. Passing an explicit width smaller than the current width is an error, not a
truncation.

## Result and edge cases

Return kind: `Array`, of exactly the requested dimensions. The result spills; spilling is
host-side adaptation ([the call pipeline](../model/03-call-pipeline.md)).

- **`rows` or `columns` equal to the current extent** is a no-op on that axis.
- **`pad_with` may be any scalar**: a number, text, a logical, or an error value. Passing `""`
  is the common way to get visually blank padding, and it produces empty *text*, not the Empty
  kind — [the value universe](../model/01-value-universe.md) keeps those distinct, and
  downstream `ISBLANK` will disagree with your eyes.
- **Whether `pad_with` may be an array** is not established here. If it can, the padding
  semantics are no longer "one value everywhere".
- **`rows` and `columns` lift.** The projection records
  `by_index_scalar_array_lift(positions=1|2)`: the dispatch layer broadcasts those two argument
  positions. What an array of target dimensions produces is a shape question the Handbook has
  not settled.
- **Element errors** in `array` pass through unchanged; nothing collapses
  (`error_collapse_profile: None`).

## Errors

Microsoft's page documents that `EXPAND` returns `#VALUE!` when `rows` or `columns` is smaller
than the array's current row or column count. That is the characteristic error of this function
and it follows from its one-directional design: there is no shrinking mode to fall back on.

The documented default `pad_with` of `#N/A` means an ordinary successful `EXPAND` routinely
*contains* error values without *being* an error. That distinction matters downstream: an
aggregate scanning the padded result will meet `#N/A` and, under the propagation rule in
[coercion and lifting](../model/02-coercion-and-lifting.md), surface it. Supplying an explicit
`pad_with` — `0`, `""` — is usually what a formula that feeds an aggregate actually wants.

An error value supplied as `rows` or `columns` propagates by the same universal rule.

## Relationships

- **`TAKE` and `DROP`** shrink where `EXPAND` grows, but note the argument-convention difference
  described above: their counts are increments, `EXPAND`'s are targets.
- **`HSTACK` and `VSTACK`** pad implicitly with `#N/A` when the pieces do not conform. `EXPAND`
  makes that padding explicit and lets you choose the fill value — which is the usual reason to
  reach for it before stacking.
- **`IFERROR`** is the standard partner when the `#N/A` default is not wanted after the fact;
  supplying `pad_with` is the cheaper alternative.
- **`SEQUENCE`** builds an array of a target shape from nothing; `EXPAND` reshapes one you have.
- **`MAKEARRAY`** and the `LAMBDA` helpers construct padded results generatively, which is the
  route to take when the pad value depends on position rather than being constant.

## Notes for implementers

- **The pad region is an L, not a rectangle.** Growing both axes creates three new regions
  (right, below, bottom-right). Implementations that fill row-major over the whole output and
  copy the original in afterwards get this right by construction; implementations that fill
  "the new rows" and "the new columns" separately double-fill or miss the corner.
- **Validate before allocating.** The documented `#VALUE!` condition — a target smaller than the
  current extent — must be checked before any output buffer is sized, or a shrink request
  becomes a truncation bug.
- **`pad_with` must be copied as a value of any kind**, including error values, without
  coercion. An implementation that types the fill as a number cannot express the documented
  default.
- **Large targets are a resource question.** `EXPAND` is the easiest function in the family to
  ask for an enormous result from, since the size is an argument rather than derived from the
  input. Whatever ceiling the host imposes is the host's, not this function's, and the Handbook
  has not established what Excel does at that ceiling.
- The module is shared with the rest of the dynamic-array reshapers.

## What has not been checked

No Handbook vector suite exists for `EXPAND`, and no Excel-comparison evidence record is
recorded for it. Nobody has checked this function against Excel here.

Probes worth running first:

1. **`rows` or `columns` one less than the current extent**, confirming the documented `#VALUE!`
   boundary in both directions, and **exactly equal**, confirming the no-op.
2. **The default pad**, verified as `#N/A` rather than assumed, including what an aggregate over
   the result then returns.
3. **`pad_with` as text, as a logical, as an error value, and as an array.** The array case is
   the genuine unknown; the others establish that the fill is kind-preserving.
4. **Array-valued `rows` or `columns`**, testing the declared lift positions and the resulting
   shape.
5. **Very large targets**, to find the host ceiling and its failure mode.
6. **Omitted `columns` with an input whose width changes**, confirming that the axis tracks the
   input rather than being fixed at first evaluation.

Item 3's array case and item 4 are the two that could change the description above.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| target dimension | An absolute result size, as opposed to an increment |
| pad value | The value written into every synthesized cell |
| conformable | Two arrays having shapes that an elementwise or stacking operation accepts |
| lift position | An argument index the dispatch layer broadcasts over arrays |

## Sources

- Microsoft, EXPAND function —
  <https://support.microsoft.com/en-us/office/expand-function-7433fba5-4ad1-41da-a904-d5d95808bc38>
  (target `rows` and `columns`, the `#N/A` default for `pad_with`, and the `#VALUE!` condition
  when a target is smaller than the current extent).
- Handbook `content/model/01-value-universe.md` (error values as first-class values; empty text
  versus Empty).
- Handbook `content/model/02-coercion-and-lifting.md` (error propagation).
- Handbook `content/model/03-call-pipeline.md` (`ByIndexScalarArrayLift`; host-side spill
  adaptation).
- Handbook `data/functions/FUNC.EXPAND.json` and `data/presence/FUNC.EXPAND.json` (arity,
  classification axes, shared reshape-family module).
