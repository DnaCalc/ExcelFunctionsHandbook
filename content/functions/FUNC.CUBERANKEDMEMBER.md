---
schema: efh.function-page/v1
function_id: FUNC.CUBERANKEDMEMBER
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
  - Notes
  - Page vocabulary
  - Sources
family: null
role_in_family: null
---

`CUBERANKEDMEMBER` is the family's indexer: given a set that lives on the server, it retrieves
the item at one position. Microsoft's framing is "the top sales performer or the top 10
students", and the "top" in that phrase is doing quiet but load-bearing work — see below.

The reference engine (OxFunc) carries it in its catalog and defers it, with the recorded reason
**"Deferred cube-context function."** There is no implementation module and no live registry
entry — catalog-only. The deferral is structural: the set being indexed is server-side.

## What it computes

The member at position `rank` of a set, where position 1 is the first item.

The word to be careful about is *ranked*. `CUBERANKEDMEMBER` does not rank anything. It reads a
position out of an order that already exists — the order the set was built in. If the set came
from `CUBESET` with `sort_order` 2 and a `sort_by` measure, then rank 1 genuinely is "the top
performer by that measure". If the set was built with `sort_order` 0 (`SortNone`, the default),
rank 1 is simply whichever item the cube happened to return first, and the formula
`CUBERANKEDMEMBER(conn, mySet, 1)` will read like a top-N query while meaning nothing of the
kind.

So the function's real semantics are: **position lookup in a server-defined sequence**, with
all the ranking meaning imported from the `CUBESET` that produced the sequence. The pairing is
inseparable, and this is the single most useful thing to understand about the function.

The standard idiom is a top-N block: one `CUBESET` cell defining and sorting the set, a column
of `CUBERANKEDMEMBER` cells for ranks 1…N, and a parallel column of `CUBEVALUE` cells reading
the measure at each of those coordinates. `CUBESETCOUNT` supplies N when it is not fixed.

Like `CUBEMEMBER`, the resulting cell is dual: its display value is a caption, and its identity
is the member, which is what `CUBEVALUE` consumes when the cell is referenced.

## Arguments

`CUBERANKEDMEMBER(connection, set_expression, rank, [caption])`

- **`connection`** (required) — text; the name of a connection stored in the workbook, used as
  a lookup key. Note that `CUBERANKEDMEMBER` takes a connection even when `set_expression` is a
  reference to a `CUBESET` cell that already has one; `CUBESETCOUNT`, by contrast, takes no
  connection at all. The asymmetry between the two accessors is not explained by the
  documentation.
- **`set_expression`** (required) — text holding an MDX set expression such as
  `"{[Item1].children}"`, *or* a `CUBESET` call, *or* a reference to a cell containing one.
  Three quite different things share one argument slot.
- **`rank`** (required) — an integer position. Microsoft: "If rank is a value of 1, it returns
  the top value, if rank is a value of 2, it returns the second most top value, and so on."
  One-based, and the documentation says nothing about zero, negative, fractional, or
  out-of-range values.
- **`caption`** (optional) — text displayed instead of the cube's own caption.

## Result and edge cases

The result is a member: displayed as a caption, carrying member identity for downstream CUBE
functions. It is a single cell — asking for ranks 1…10 requires ten formulas, not one spilling
call. `CUBERANKEDMEMBER` predates dynamic arrays and has no array form.

While the request is outstanding the cell displays `#GETTING_DATA…` — a transient
extended-family placeholder catalogued in [the value universe](../model/01-value-universe.md),
not a failure.

The out-of-range case (`rank` greater than the set's cardinality) is the obvious boundary, and
the documentation does not state its result. Real workbooks hit it constantly, because a top-10
block over a set that turns out to hold six members is an everyday occurrence. The Handbook
does not know what Excel returns there; it is the first probe listed below.

## Errors

As documented by Microsoft on the page linked below:

- **`#NAME?`** — the connection name is not a valid workbook connection; or the OLAP server is
  not running, not available, or returns an error message.
- **`#N/A`** — the `set_expression` syntax is incorrect, or the set contains at least one
  member with a different dimension from the others.

Microsoft's `CUBERANKEDMEMBER` page documents no error for an out-of-range `rank`, and no
`#VALUE!` condition of the kind its siblings list. Whether that reflects the behaviour or the
documentation is unknown to the Handbook.

## Relationships

- **`CUBESET`** — supplies the set and, crucially, its order. `CUBERANKEDMEMBER` cannot be
  understood without it.
- **`CUBESETCOUNT`** — the other accessor on a set; gives the loop bound that keeps `rank` in
  range.
- **`CUBEMEMBER`** — the other coordinate producer. `CUBEMEMBER` names a member directly;
  `CUBERANKEDMEMBER` names one positionally. Both feed `CUBEVALUE`.
- **`CUBEVALUE`** — the consumer that turns the retrieved coordinate into a number.

Outside the family, readers reach for `LARGE`, `INDEX`, or `SORTBY`+`INDEX` by analogy. The
analogy is decent for intent and wrong for mechanism: those functions rank data present on the
sheet, whereas `CUBERANKEDMEMBER` never brings the set to the client.

## Notes for implementers

- The order is not the function's to define. Any implementation must treat rank as an index
  into a sequence fixed by the set constructor, and must therefore preserve set order
  faithfully across the boundary — including tie order.
- Tie handling is a genuine semantic question that neither this function nor `CUBESET`
  documents. If two members share a `sort_by` value, which is rank 1 — and is that answer
  stable across recalculations? An implementation that sorts with an unstable algorithm will
  produce a workbook that changes its answers without any input changing.
- Because `set_expression` accepts a literal string, a nested call, and a cell reference, the
  argument must be resolved with the same identity-versus-display rule the rest of the family
  uses: a referenced cell containing a CUBE function contributes its expression, not its text.
- The result is scalar-per-call. A modern reimplementation would be tempted to return an array
  for a range of ranks; that would be a different function, not this one.

## What has not been checked

No Handbook vector suite exists for `CUBERANKEDMEMBER`, and no Excel-comparison evidence is
recorded. Nobody has checked this function against Excel in the Handbook's record. Everything
behavioural above is Microsoft's documented behaviour or explicitly flagged as unknown.

Given an oracle with a live cube, the probes that would settle the most:

1. **`rank` out of range.** A set of known cardinality *n*, then `rank` = *n*+1. This is the
   most frequently hit undocumented boundary in the function.
2. **`rank` = 0, negative, and fractional.** Whether fractional ranks truncate, round, or
   error, and whether 0 is treated as an error or as rank 1.
3. **Unsorted sets.** The same set built with `sort_order` 0 and read at rank 1 across repeated
   recalculations and across two sessions, to see whether "existing order" is stable at all.
4. **Ties.** Two members with identical `sort_by` values, read at ranks 1 and 2 repeatedly.
5. **The connection argument's role.** A `connection` naming a *different* valid connection
   from the one behind a referenced `CUBESET` cell — is it ignored, validated, or does it
   change the resolution target?
6. **Identity downstream.** `CUBEVALUE(conn, rankedCell)` compared with
   `CUBEVALUE(conn, "<the member's MDX>")`, to confirm the identity passes through.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `CUBERANKEDMEMBER` that an implementer matching Excel, or a reader
explaining an unexpected Excel result, needs to know. Each note gives exact binary64 inputs,
Excel's answer and where it was recorded; "measured" means the statement is read off recorded
Excel answers, "inferred" marks what goes beyond them. Where older text on this page says a
behaviour was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

### FN-CUBERANKEDMEMBER-001: CUBERANKEDMEMBER accepts set expressions directly or via cell references, truncates rank (with logical/numeric-text coercion), and returns #VALUE! on invalid rank text before connection lookup

*Quirk; measured; recorded 2026-11-22.*

In Excel 16.0.20527, CUBERANKEDMEMBER(connection, set_expression, rank, [caption]) accepts
either a CUBESET cell reference or an inline set expression string, reads rank by truncating
numbers (1.9 -> 1), coercing logicals (TRUE -> 1) and numeric strings ("1" -> 1), rejects
non-numeric rank text ("hello", "1,234") with #VALUE! before connection lookup, and returns
#N/A for rank < 1 or exceeding the set size. All 4,287 full-bank CUBERANKEDMEMBER rows match.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=CUBERANKEDMEMBER("ThisWorkbookDataModel","[Table1].[Region].Children",1)` | as typed | `East` | Excel 16.0.20527 (64-bit), Windows, workbook data model, en-ZA |
| `=CUBERANKEDMEMBER("ThisWorkbookDataModel",CUBESET("ThisWorkbookDataModel","[Table1].[Product].Children","p",2,"[Measures].[TotalAmount]"),1)` | as typed | `A` | Excel 16.0.20527 (64-bit), Windows, workbook data model, en-ZA |
| `=CUBERANKEDMEMBER("ThisWorkbookDataModel",CUBESET("ThisWorkbookDataModel","[Table1].[Product].Children","p",2,"[Measures].[TotalAmount]"),0)` | as typed | `#N/A` | Excel 16.0.20527 (64-bit), Windows, workbook data model, en-ZA |
| `CUBERANKEDMEMBER(41, 44960, "hello", 1)` | connection `39.99999999999999` (`0x4043ffffffffffff`); set_expression `25569.0` (`0x40d8f84000000000`); rank `hello`; caption `1.0` (`0x3ff0000000000000`) (cells) | `#VALUE!` | Excel 16.0 build 20527, Windows, workbook compatibility 2, en-ZA; bitness and CPU not recorded |

Basis (measured): Measured in DnaCalc crates/calc-host/tests/fixtures/cube/measured.xlsx
(A104-A107), pivot_cube/oracle.xlsx (A131-A132), and CalcFunc-Evidence
admit2-answers-cuberankedmember-scalar.json; 4,287/4,287 full-bank rows match.

Evidence:

- DnaCalc `crates/calc-func/src/catalogue/cube_functions.rs`, cuberankedmember at `2e9be2b5b7` — rank parsing, truncation, and early text error
- DnaCalc `crates/calc-func/witness-status.tsv`, CUBERANKEDMEMBER at `2e9be2b5b7` — 4,287/4,287 full-bank matches
- DnaCalc `crates/calc-host/tests/fixtures/cube/measured.xlsx`, xl/worksheets/sheet1.xml A107 at `2e9be2b5b7` (readings)
- DnaCalc `crates/calc-host/tests/fixtures/pivot_cube/oracle.xlsx`, xl/worksheets/sheet1.xml A131 at `2e9be2b5b7` (readings)
- DnaCalc `crates/calc-host/tests/fixtures/pivot_cube/oracle.xlsx`, xl/worksheets/sheet1.xml A132 at `2e9be2b5b7` (readings)
- CalcFunc-Evidence `readings/excel-20527/admit2-answers-cuberankedmember-scalar.json`, admit2-scalar-CUBERANKEDMEMBER-0241 at `4ef2c244d7` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| reference engine: deferred | OxFunc intentionally defers the function; the recorded reason is "Deferred cube-context function." |
| catalog-only | The projection carries identity and documentation metadata but no implementation module or live registry entry |
| rank | A one-based position within a set's existing order — not a computed ranking |
| set order | The sequence fixed by the `CUBESET` call that produced the set |
| identity vs display | The cell's member identity (what CUBE functions consume) versus its caption (what a reader sees) |
| `#GETTING_DATA` | Extended-family transient placeholder while external data is retrieved |

## Sources

- Microsoft, "CUBERANKEDMEMBER function" —
  <https://support.microsoft.com/en-us/office/cuberankedmember-function-07efecde-e669-4075-b4bf-6b40df2dc4b3>
  (syntax, all four arguments including the one-based wording of `rank`, the `#GETTING_DATA`
  remark, and the `#NAME?` and `#N/A` conditions listed above).
- Microsoft, "CUBESET function" —
  <https://support.microsoft.com/en-us/office/cubeset-function-5b2146bd-62d6-4d04-9d8f-670e993ee1d9>
  (the `sort_order` table that gives `rank` its meaning).
- Handbook `data/functions/FUNC.CUBERANKEDMEMBER.json` — admission label and reason, category,
  the XLL identity `xlfCuberankedmember` (383), and the catalog-only metadata status.
- Handbook `data/presence/FUNC.CUBERANKEDMEMBER.json` — no implementation module, no dispatch
  entry.
- Handbook `content/model/01-value-universe.md` — the extended error-code family.
