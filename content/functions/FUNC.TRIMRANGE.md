---
schema: efh.function-page/v1
function_id: FUNC.TRIMRANGE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - Arguments
  - A documented divergence
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
  - Page vocabulary
  - Sources
family: trimrange_fn
role_in_family: >-
  Strips the blank margin from a range or array, returning only the populated rectangle — the
  function form of the trim-reference operators.
---

## What it computes

`TRIMRANGE` removes blank rows and columns from the **edges** of a range or array and returns
what is left.

The operation scans inward from each edge until it meets a non-blank cell, then keeps the
rectangle bounded by those four positions. Interior blanks are untouched — a blank row in the
middle of populated data stays exactly where it is. `TRIMRANGE` is a *margin* operation, not a
compaction; anything that removes interior blanks (`TOCOL(…, 1)`, `FILTER`) is a different
function.

The point of it is whole-column references. `SUM(A:A)` is convenient and, on a large sheet,
expensive and fragile; `SUM(TRIMRANGE(A:A))` is convenient and bounded. Before `TRIMRANGE` the
same effect required `OFFSET` with a `COUNTA`-computed height — which is volatile — or an
`INDEX`-built range, which is correct but unreadable. `TRIMRANGE` gives the idiom a name and,
per its projected metadata, a non-volatile one.

Which edges are trimmed is controlled per axis: rows and columns have independent settings for
whether leading blanks, trailing blanks, both, or neither are removed.

## Arguments

Microsoft documents `TRIMRANGE(range, [trim_rows], [trim_cols])`:

| Argument | Documented default | Meaning |
|---|---|---|
| `range` | required | The range (or array) to be trimmed |
| `trim_rows` | `3` | `0` none · `1` leading blank rows · `2` trailing blank rows · `3` both |
| `trim_cols` | `3` | `0` none · `1` leading blank cols · `2` trailing blank cols · `3` both |

So the documented default trims blank margins on all four sides.

The argument most likely to be misread is the code itself, because `1` and `2` are not
symmetric-looking and there is nothing in the call site to remind you which is which. `0` and
`3` are safe; the intermediate values are worth spelling out in a comment.

## A documented divergence

The Handbook's projected signature for this function is
`TRIMRANGE(array, [trim_rows], [trim_cols], [headers])` — **four** arguments — and the
reference engine's declared contract states a different code table and a different default:
`trim_rows` and `trim_cols` defaulting to `1` with `1` meaning *trailing*, `2` meaning
*leading*, and a fourth `headers_count` argument protecting a number of leading rows from
trimming.

Microsoft's page, retrieved for this entry, documents three arguments, `1` meaning *leading*,
`2` meaning *trailing*, and a default of `3`.

The Handbook does not resolve this here, and does not silently prefer either side. What can be
said honestly:

- The two accounts disagree about the **default**, about the **meaning of the codes `1` and
  `2`**, and about the **existence of a fourth argument**.
- A disagreement of this shape usually has one of three causes: the documentation changed after
  the reference engine's contract was written, the reference engine's contract was drafted
  against a preview or a different source, or the function's surface genuinely differs across
  Excel channels. `TRIMRANGE` is recent enough for all three to be plausible.
- Nobody has run the experiment that would settle it. It is a single afternoon's work: enter
  `TRIMRANGE` with each code in each position against a range with known leading and trailing
  blanks, and record what comes back.

Until that happens, treat Microsoft's table as the documented behaviour and treat any
implementation's code mapping as an implementation detail to verify rather than to trust.

## Result and edge cases

The return kind is an array containing the trimmed contents.

- **A fully blank input** has nothing left after trimming. The reference engine's contract maps
  that to `#CALC!`, which is consistent with Excel's general convention for an empty array
  result ([the value universe](../model/01-value-universe.md)); the Handbook has not verified
  it for this function.
- **What counts as blank** is the pivotal question, exactly as for `TOCOL`: an empty cell
  certainly, but a cell holding `""`, or a formula returning `""`, is a different matter. A
  column of formulas that return `""` below the data is the single most common real input to
  `TRIMRANGE`, and the documentation does not say what happens to it.
- **A single-cell result stays an array.** The reference engine's `BUG-FUNC-026` explicitly
  lists `trimrange_fn::trimrange_kernel` among the places where a single-cell scalarization was
  removed, after Excel probes showed that a nested `TAKE` result remains array-typed even
  though the anchor cell publishes a bare value.
- **Interior blanks survive**, always.
- Dynamic-array publication and `#SPILL!` are host-side adaptation.

## Errors

The reference engine's contract declares `#VALUE!` for a trim-type value outside the admitted
set and `#CALC!` when trimming leaves nothing. Microsoft's page, as retrieved, does not
enumerate error conditions for `TRIMRANGE`.

The Handbook therefore states the error surface as **unsettled**: there is a declared
implementation mapping and no documented counterpart, and the two have not been compared.

## Relationships

- **The trim-reference operators** are the operator form of the same idea, and are modeled in
  this Handbook as functions: `FUNC.OP_TRIM_REF_LEADING`, `FUNC.OP_TRIM_REF_TRAILING` and
  `FUNC.OP_TRIM_REF_BOTH` (see [the call pipeline](../model/03-call-pipeline.md), "Operators are
  functions"). `A.:A` and its relatives are the terse spelling of `TRIMRANGE(A:A, …)`.
- **`OFFSET` with `COUNTA`** is the idiom `TRIMRANGE` replaces, and it is volatile.
- **`INDEX`-built ranges** (`A1:INDEX(A:A, n)`) are the non-volatile pre-`TRIMRANGE` idiom.
- **`TOCOL(…, 1)`** removes *all* blanks and flattens; `TRIMRANGE` removes only the margin and
  keeps the rectangle.
- **`FILTER`** removes rows by predicate.
- **`ROWS`/`COUNTA`** are what people reach for when they want the *size* of the populated
  region rather than its contents.

## Notes for implementers

- Trimming is per axis and per edge: four independent decisions driven by two codes. Scanning
  in from each edge is the natural implementation and is `O(m + n)` in the populated case, not
  `O(mn)` — worth getting right, because whole-column references are the intended input.
- Interior blanks must survive. A compaction pass is a different function.
- The blank predicate must be stated and shared with the rest of the engine.
- A single-cell trimmed result must stay array-shaped; this is a recorded, deliberately-undone
  scalarization in the reference engine.
- The code table and default are exactly the place to *not* guess. Whatever mapping is
  implemented should be traceable to a source, and the divergence recorded above should be
  resolved by observation before either mapping is trusted.

## What has not been checked

No Handbook vector suite exists for `TRIMRANGE`, and no Handbook evidence record is attached to
this page. Nothing here claims agreement with Excel for any implementation. `TRIMRANGE` is one
of the newest functions in the catalogue and the thinnest in published behavioural detail,
which is why this page carries an explicit divergence section instead of a confident summary.

First probes, in order:

1. **The code table.** `trim_rows` and `trim_cols` each set to `0`, `1`, `2`, `3` against a
   range with known leading *and* trailing blank margins, recorded independently per axis. This
   single experiment settles the divergence above.
2. **The default.** Two- and three-argument calls against the same range, compared with the
   explicit `3, 3` call.
3. **A fourth argument.** Whether `TRIMRANGE(range, 3, 3, 1)` is accepted at all, and what it
   does if it is — this decides whether the projected four-argument signature reflects Excel or
   an implementation extension.
4. **The blank predicate**: empty cells, `""` literals, formula-produced `""`, spaces, and
   zeros in the margin.
5. **The fully blank input**, against `#CALC!`.
6. **Whole-column and whole-row references**, which are the intended use and the case where
   performance and correctness meet.
7. **Interior blanks**, to confirm they are never removed.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| margin | The blank rows and columns at a range's edges; the only blanks `TRIMRANGE` removes |
| trim code | The `0`–`3` value selecting which edges of an axis are trimmed |
| trim-reference operator | The operator spelling of the same idea (`.:`, `:.`, `.:.`) |
| divergence | A recorded disagreement between documentation and an implementation contract, published rather than resolved by preference |

## Sources

- Microsoft, *TRIMRANGE function* —
  <https://support.microsoft.com/en-us/office/trimrange-function-d7812248-3bc5-4c6b-901c-1afa9564f999>
  (syntax `=TRIMRANGE(range,[trim_rows],[trim_cols])`, the `0`/`1`/`2`/`3` code tables for both
  axes, and the default of `3` for each). Retrieved for this page.
- OxFunc `docs/function-lane/FUNCTION_SLICE_TRIMRANGE_CONTRACT_PRELIM.md` — the reference
  engine's declared contract, including the four-argument admission policy, the differing code
  meanings and defaults, `headers_count` row protection, `#CALC!` on an empty result and
  `#VALUE!` on an invalid trim type. The source of the divergence recorded above.
- OxFunc bug stream `docs/bugs/streams/BUG-FUNC-026_take_1x1_scalar_publication_mismatch.md` —
  names `trimrange_fn::trimrange_kernel` among the single-cell scalarizations that were removed
  after Excel probes.
- Handbook `content/model/01-value-universe.md` (the `#CALC!` empty-array convention) and
  `content/model/03-call-pipeline.md` (the trim-reference operator identities).
- Handbook `data/functions/FUNC.TRIMRANGE.json` (the projected four-argument signature and the
  classification axes).
