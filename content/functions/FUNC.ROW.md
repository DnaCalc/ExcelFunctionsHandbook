---
schema: efh.function-page/v1
function_id: FUNC.ROW
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
family: row_fn
role_in_family: >-
  Reads a reference's row coordinate, or — with no argument — the calling cell's own row: one
  of the few functions whose answer depends on where the formula lives.
---

## What it computes

`ROW` reports a **row coordinate**, in one of two quite different ways depending on whether it
is given an argument.

1. **`ROW()`** — no argument — returns the 1-based row index of **the cell containing the
   formula**. It reads nothing from the sheet's contents; it reads the evaluation context. This
   makes `ROW()` one of the small set of *caller-aware* functions, alongside `COLUMN()` and
   `CELL` with an omitted reference. Two identical formulas in two cells return different
   answers.
2. **`ROW(reference)`** — returns the 1-based row index of the reference. For a single cell,
   that is one number. For an area spanning several rows, it is a **vertical array of the row
   indices from top to bottom**: `ROW(A5:A8)` is `{5;6;7;8}`.

That second behaviour is the one worth internalizing, because it is the engine behind a large
family of array idioms. `ROW(INDIRECT("1:"&n))` is the classic way to manufacture the sequence
`1..n` in a pre-`SEQUENCE` Excel, and `ROW(range) - ROW(top_of_range) + 1` is the classic way
to get positions relative to a range's top. Both are consequences of `ROW` of an area being an
array rather than a scalar.

## Arguments

**`reference`** — optional. A reference whose row index is wanted.

Three points about this single argument:

- **Omission is meaningful, not a default.** Omitting the argument does not substitute a value;
  it switches `ROW` into caller-aware mode. This is the `Missing`-versus-`Empty` distinction of
  [the value universe](../model/01-value-universe.md) doing real work: an omitted argument and
  an argument that resolves to a blank cell are not the same call.
- **The argument is consumed as a reference, not as a value.** `ROW` is declared
  `RefsVisibleInAdapter` in [the call pipeline](../model/03-call-pipeline.md): it inspects the
  address, never the contents. The reference engine's contract records that non-reference
  arguments are refused at worksheet ingress rather than evaluated to an error.
- **A multi-row area gives an array, not the top row.** Readers expecting a scalar and getting
  a spill (or, historically, an implicit intersection) are meeting this rule.

## Result and edge cases

The return kind is a number for the no-argument and single-cell forms, and a vertical array of
numbers for a multi-row area.

- **Whole-column references produce very large arrays.** `ROW(A:A)` is semantically the
  complete column of row indices. That is a legitimate function result; whether the *worksheet*
  can host it is a separate, host-level question, and the reference engine's contract records
  the observed consequence: a whole-column `ROW` can publish as `#SPILL!` when the anchor cell
  cannot accommodate the shape. The function did not fail; the publication did.
- **Multi-area references** (`ROW((A1:A2,C1:C2))`) are not described by the documentation and
  the Handbook has not verified their behaviour.
- **What `ROW()` means outside a cell** — in a defined name, in a conditional-format rule, in
  an add-in call — depends on the evaluation context the host supplies, and is host behaviour
  rather than function semantics.

## Errors

Microsoft's documentation for `ROW` names no error values, which is consistent with a function
whose only input is a reference and whose failure modes are admission-time rather than
runtime. In practice the observable failures are:

- an argument that is not a reference, refused at entry rather than returning an error value;
- `#REF!` propagating from a reference that has been invalidated (deleted rows), which is
  reference resolution failing before `ROW` runs;
- `#SPILL!` at publication for an array result that cannot be placed — a host outcome, not a
  `ROW` outcome.

## Relationships

- **`COLUMN`** is the exact transpose: same two forms, same caller-aware behaviour, horizontal
  array instead of vertical.
- **`ROWS`** answers the adjacent question — *how many* rows, rather than *which* — and always
  returns a scalar. Confusing the two is the single most common mistake here:
  `ROW(A5:A8)` is `{5;6;7;8}`; `ROWS(A5:A8)` is `4`.
- **`SEQUENCE`** replaces the `ROW(INDIRECT(...))` idiom for generating `1..n`, without
  volatility and without abusing a coordinate function as a generator.
- **`CELL("row", ref)`** is another route to the same number, through a different and much
  broader function.
- **`OFFSET`, `INDEX`, `INDIRECT`, `ADDRESS`** are the rest of the reference family; `ROW` is
  a reference *reader*.

## Notes for implementers

- The two forms are different functions sharing a name. The no-argument form needs the caller's
  address from the evaluation context; the one-argument form needs no context at all. An engine
  that supplies caller context only when an argument is absent has to keep that seam explicit.
- The area form returns an array whose length is the reference's row span — including for
  whole-column references. Materializing that eagerly is a performance decision with visible
  consequences; refusing to materialize it changes the semantics.
- `ROW` must not dereference. Its answer is a property of the address, and a cell's contents —
  including whether it is blank — is irrelevant.
- The result of the area form is a *column* array. Getting the orientation wrong is invisible
  in a 1×1 case and wrong everywhere else.
- Caller-aware functions interact with anything that evaluates a formula away from a cell:
  defined names, array contexts, and add-in entry points each need a decided answer.

## What has not been checked

No Handbook vector suite exists for `ROW`, and no Handbook evidence record is attached to this
page. Nothing here says any implementation agrees with Excel.

`ROW` is awkward to test with a value-comparison harness for the same reason `OFFSET` is: half
its behaviour depends on *where the formula is*, which a stateless vector suite does not model.
A suite has to place formulas at known addresses and record the address alongside the result.

First probes:

1. **The caller-aware form at several addresses**, including row 1 and the last row of both
   the default grid and an `.xls`-compatibility workbook.
2. **The area form's shape**: `ROW` of a single cell, a vertical area, a horizontal area (does
   a 1-row, many-column reference give one number or many?), and a rectangular area — the
   rectangular case is the one the documentation does not describe.
3. **Whole-column and whole-row references**, and where the `#SPILL!` boundary falls.
4. **Multi-area and 3-D references.**
5. **`ROW` inside a defined name** and inside a conditional-format rule, to pin what "the
   caller" means when there is no ordinary cell.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| caller-aware | The result depends on the calling cell's position, not on any argument |
| area form | `ROW(reference)` over a multi-row area; returns a vertical array of indices |
| `RefsVisibleInAdapter` | Argument-preparation profile: the function receives the live reference |
| `#SPILL!` | Publication-time failure when an array result cannot be placed; a host outcome |

## Sources

- Microsoft, *ROW function* —
  <https://support.microsoft.com/en-us/office/row-function-3a63b74a-c4d0-4093-b49a-e76eb49a6d8d>
  (syntax, the optional reference, and the caller-row behaviour when it is omitted). Not
  retrieved for this page; the behaviour above is stated as documented behaviour and should be
  re-checked against the page.
- Handbook `content/model/01-value-universe.md` (reference shapes, `Missing` versus `Empty`)
  and `content/model/03-call-pipeline.md` (reference-aware preparation, caller-aware
  functions).
- OxFunc `docs/function-lane/FUNCTION_SLICE_ROW_CONTRACT_PRELIM.md` — the reference engine's
  declared contract: caller-row scalar for the omitted-argument form, scalar for a single-cell
  reference, vertical array of distinct row indices for an area, and the observed whole-column
  `#SPILL!` edge at worksheet publication. Bounded empirical baseline: Excel 16.0 build 19725,
  default and `.xls`-compatibility workbook lanes.
- Handbook `data/functions/FUNC.ROW.json` (signature, arity, classification axes).
