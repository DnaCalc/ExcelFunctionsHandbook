---
schema: efh.function-page/v1
function_id: FUNC.TYPE
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
family: type_fn
role_in_family: Sole member of its module; the whole IS family collapsed into one numeric answer.
---

## What it computes

`TYPE(value)` returns a number naming the value's kind. Microsoft's table, verbatim:

| If value is | `TYPE` returns |
|---|---|
| Number | 1 |
| Text | 2 |
| Logical value | 4 |
| Error value | 16 |
| Array | 64 |
| Compound data | 128 |

The codes are **powers of two** — 1, 2, 4, 16, 64, 128 — with 8 and 32 unused in the published
table. That is not decoration: the numbering descends from a bit-flag encoding of admissible
argument types in Excel's macro-sheet heritage, which is also why the documentation's own
examples of the function's usefulness are the macro-era `ARGUMENT` and `INPUT`. Nothing in
`TYPE`'s worksheet behaviour requires you to know that, but it explains why the codes skip.

`TYPE` answers the same question as the IS-family classifiers, in one call instead of five, and
in a form you can `CHOOSE` or `SWITCH` on. Where `ISNUMBER` is a boolean, `TYPE` is a
discriminator.

Microsoft states one limit explicitly and it is the one users trip over: "You cannot use TYPE to
determine whether a cell contains a formula. TYPE only determines the type of the resulting, or
displayed, value. If value is a cell reference to a cell that contains a formula, TYPE returns
the type of the formula's resulting value." `ISFORMULA` is the function for that question.

## Arguments

`value` — required, exactly one. The published signature is `TYPE(value)`.

The argument is a values position: a reference is resolved before `TYPE` sees it
(`ArgPreparationProfile::ValuesOnlyPreAdapter`). So `TYPE(A1)` reports the kind of `A1`'s
*contents*, never `Reference`, and there is no ordinary way to make `TYPE` report on a reference
as such. `ISREF` is the function that can.

The values-only resolution is also why a single-cell reference and the value inside it are
indistinguishable to `TYPE` — see [the call pipeline](../model/03-call-pipeline.md) on the 1x1
collapse.

## Result and edge cases

Returns a `Number` — one of the codes above.

- **An error input returns `16`; it does not propagate.** `TYPE` is a classifier, so an error is
  a subject, not a failure. `TYPE(#DIV/0!)` is `16`, and so is `TYPE(#N/A)`. `TYPE` cannot tell
  you *which* error; `ERROR.TYPE` can.
- **An array input returns `64` as a single scalar.** This is the behaviour that distinguishes
  `TYPE` from every IS-family classifier: they lift elementwise over an array and return a mask,
  while `TYPE` looks at the argument as a whole and reports "this is an array". The reference
  engine implements exactly that. It makes `TYPE` the only cheap way to ask "did this expression
  produce an array?".
- **An empty cell returns `1`.** Not a separate code — `Empty` is not in Microsoft's table at all.
  The reference engine maps `Empty` (and `Missing`) to `1`, the number code, which is consistent
  with the way emptiness normalizes to numeric zero at the published-result boundary
  ([the value universe](../model/01-value-universe.md)). It means `TYPE` cannot distinguish a
  blank cell from a cell containing `0`, and `ISBLANK` is the function that can.
- **Code 128, "compound data", is in the documentation and is not reachable in the reference
  engine**, whose declared code map has no 128 case. What produces a 128 in current Excel — if
  anything still does — the Handbook does not know. This is the page's clearest documented /
  implemented gap and is probe 1 below.
- **Codes 8 and 32 do not appear** in Microsoft's table.
- **Rich values.** Whether a linked data type reports 128, or the code of its core projection,
  is not established here.

## Errors

`TYPE` returns no error of its own for any value it can be given — every kind has a code. The
only failure available to it is **arity**: zero arguments, or two. `TYPE()` is expected to be
refused at formula entry rather than evaluated
([the call pipeline](../model/03-call-pipeline.md)); the reference engine, having no entry-time
surface, reports `#VALUE!` for both.

## Relationships

- **The IS family** answers the same questions one bit at a time. The correspondence is exact
  where both exist: `TYPE=1` matches `ISNUMBER`, `TYPE=2` matches `ISTEXT`, `TYPE=4` matches
  `ISLOGICAL`, `TYPE=16` matches `ISERROR`. There is no IS function for `64`, which is why
  `TYPE(x)=64` is the standard array test.
- **`ERROR.TYPE`** refines `TYPE`'s `16` into a code 1–8. The two functions are complementary and
  are often used together: `TYPE` to branch, `ERROR.TYPE` to diagnose.
- **`ISBLANK`** covers the distinction `TYPE` erases — `Empty` versus `Number` — because both
  report as `1`.
- **`ISFORMULA`** covers the distinction Microsoft's remark says `TYPE` cannot make.
- **`ISREF`** covers the reference case `TYPE` never sees.
- **`CELL("type", ref)`** is a different function with a confusingly similar name and a much
  coarser answer: it reports `"b"`, `"l"` or `"v"` (blank, label, value) for a cell, is
  host-dependent, and is not a substitute for `TYPE`.
- **`N` and `T`** are the projections that pair with `TYPE`'s classification.

## Notes for implementers

1. **`TYPE` of an array is `64`, not an array of codes.** This is the axis on which `TYPE`
   diverges from the entire IS family, and routing `TYPE` through a shared elementwise lift
   helper silently breaks it. It is the single most likely implementation error on this function.
2. **`Empty` and `Missing` need an explicit mapping.** Microsoft's table names neither. Whatever
   an engine chooses should be a recorded decision awaiting evidence, not a fall-through.
3. **Decide about 128 deliberately.** An engine that cannot produce compound data has no 128
   case, which is defensible — but it should be recorded as "unreachable here", not as
   "documented code omitted".
4. **The codes are stable numbers, not an enum's ordinal.** They must be emitted as the literal
   published values; there is no reordering freedom.
5. **Errors are classified, not propagated.** As with the IS classifiers, the error has to reach
   the kernel.

## What has not been checked

No Handbook vector suite exists for `TYPE`; `vectors/` publishes nothing for this function. No
Excel-comparison evidence record names `TYPE` as a subject — **nobody has checked this
function's answers against Excel inside the Handbook's record.** The reference engine's own
answers appear on the battery panel under their own label; they are not Excel's answers.

The probes that would settle the open questions, in priority order:

1. **What produces `TYPE` = 128 in current Excel?** Candidates worth trying: a linked data type
   (stock, geography), an in-cell image, an uncalled `LAMBDA`, a cell containing an OLE or
   embedded object, and a value returned from an add-in as a rich value. The documentation
   publishes the code; the Handbook cannot currently name a single input that yields it, and
   finding one — or establishing that none does on a modern build — is the most interesting open
   question on the page.
2. **`TYPE` of an empty cell versus `TYPE(0)`.** Expected `1` for both; unmeasured. If Excel
   returns something else for the blank, the value model's Empty-normalization story gains a
   visible witness.
3. **`TYPE` of an array**, both as a literal `{1,2}` and as a spilled range reference `B1#`, to
   confirm the single `64` rather than a lifted result — and to see whether a spill anchor
   behaves as an array or as a reference.
4. **`TYPE` of an omitted argument slot** inside a `LAMBDA`.
5. **`TYPE` of each error code**, including the extended family, expected `16` throughout.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| type code | The number `TYPE` returns; a power of two, from a bit-flag heritage |
| compound data | Microsoft's name for code 128; no input is known here to produce it |
| discriminator | A single value that selects among kinds, as opposed to a boolean test |
| whole-argument classification | `TYPE` inspects the argument as one value; it does not lift |

## Sources

- Microsoft, "TYPE function" —
  <https://support.microsoft.com/en-us/office/type-function-45b4e688-4bc3-48b3-a105-ffa892995899>.
  Read for this page: the full code table verbatim, including the 128 row, and both remarks —
  the `ARGUMENT`/`INPUT` usage note and the statement that `TYPE` cannot detect a formula.
- Microsoft, "IS functions" —
  <https://support.microsoft.com/en-us/office/is-functions-0f2d7971-6019-40a0-a171-f2d869135665>.
  Read for the classifier correspondences named above.
- Handbook, [the value universe](../model/01-value-universe.md) — the kind inventory, the
  Empty-at-publication rule, and rich values as an extension layer.
- Handbook, [the call pipeline](../model/03-call-pipeline.md) — values-only preparation, the
  1x1 collapse, and the admission boundary.
- `data/functions/FUNC.TYPE.json` — identity (`xlfType`, code 86), the published signature
  `TYPE(value)`, arity, declared axes, as projected at OxFunc `473efa3`.
