---
schema: efh.function-page/v1
function_id: FUNC.COUNTBLANK
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft 365 support: COUNTBLANK function"
    locator: "https://support.microsoft.com/en-us/office/countblank-function-6a92d772-675c-4bee-b346-24af6bd3ac22"
    role: "retrieved for this pass; the one-argument syntax, the rule that formulas returning empty text are counted, and the rule that zero values are not"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - The signature divergence
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
  - Page vocabulary
  - Sources
family: countblank_fn
role_in_family: >-
  Its own module, and the counting cluster's odd member: range-only where its neighbours accept
  arrays, and the only one whose notion of "blank" includes a cell that demonstrably contains a
  formula.
---

# COUNTBLANK

## What it computes

`COUNTBLANK(range)` counts the cells in a range that are **blank**, where blank has a specific
documented meaning that is not the same as "empty".

Microsoft's page states the rule in two sentences: cells with formulas that return `""` (empty
text) are also counted, and cells with zero values are not counted. So:

| Cell content | Counted as blank? |
|---|---|
| Nothing at all | yes |
| A formula returning `""` | yes |
| A literal empty string | yes, on the same rule |
| `0` | no |
| Any other number, text, or logical | no |
| An error value | not documented — see below |

The definition is therefore *display-shaped* rather than *content-shaped*: a cell counts as blank if
it shows nothing, even when it contains a formula. That is a defensible choice for the job the
function exists to do — finding gaps in a data-entry region — and it is the reason `COUNTBLANK` is
not the complement of [COUNTA](FUNC.COUNTA.md).

**The arithmetic that surprises people.** `COUNTA` counts a `""` cell (it contains information) and
`COUNTBLANK` counts the same cell (it displays nothing). So for a range `r`,

    COUNTA(r) + COUNTBLANK(r)  ≥  ROWS(r) * COLUMNS(r)

with equality only when no cell holds empty text. Two functions, two definitions of blank, both
documented, both correct on their own terms. Anyone reconciling a count should decide which
definition they mean before choosing a function.

## Arguments

`COUNTBLANK(range)` — one argument.

| Argument | Meaning (as documented) |
|---|---|
| `range` | "The range from which you want to count the blank cells" |

Microsoft's page states plainly that the function accepts one argument. The argument is a *range*,
not a value: this is a reference-sensitive surface, and the distinction is load-bearing rather than
stylistic — see the next section.

The projection records `arg_preparation_profile: RefsVisibleInAdapter`, meaning the reference
reaches the function as a reference rather than being resolved to values first
([chapter 02](../model/02-coercion-and-lifting.md)). That is what lets the function count cells that
have never been touched: it can ask the range for its declared extent and subtract the cells that
actually hold something, rather than iterating over values that do not exist.

## The signature divergence

Microsoft's page documents one argument. **The reference engine's registry records an arity of
minimum 1, maximum 255**, and its evaluation loop sums blank counts across every argument supplied.

The Handbook records this as a divergence between the documentation and the reference engine's
classification, and does not resolve it. Three readings are available and the Handbook has evidence
for none of them:

1. Excel accepts only one argument, and the reference engine's registered arity is too wide.
2. Excel accepts several ranges undocumented, and the documentation is narrow.
3. Excel accepts several arguments but the documentation describes the supported usage.

`=COUNTBLANK(A1:A3, C1:C3)` in a live Excel decides it in one cell. It is the first probe listed
below, and it is the kind of question this Handbook exists to publish rather than guess at.

## Result and edge cases

Returns `Number` — a non-negative integer count.

- **Array arguments are rejected.** OxFunc's `BUG-FUNC-011` stream records a live Excel replay
  pinning `COUNTBLANK` over an inline array as `#VALUE!` while a true range still counts blanks,
  with `COUNT`, `COUNTA`, `ROWS` and `COLUMNS` recorded on the same replay as array-permissive
  contrast controls. The reference engine was narrowed to match. This is the best-anchored fact on
  the page — it is an upstream live-Excel observation with a named date — but it is OxFunc's
  observation, not the Handbook's, and it is a single replay rather than a sweep.
- **Zero is not blank.** Documented. This is the rule that makes `COUNTBLANK` usable on numeric
  columns where `0` is a real observation.
- **Never-touched cells inside the range count.** They have no value at all, which is the
  uncontroversial half of the definition.
- **Cells outside the used range.** A whole-column reference nominally contains a million cells.
  What `COUNTBLANK(A:A)` returns on a sheet with three filled cells is not addressed by the
  documentation, and the answer depends on whether the declared extent or the used extent is
  counted. The reference engine counts the declared extent.
- **An error value in a cell.** Undocumented. The reference engine treats an error cell as a
  preparation failure and surfaces that error as the function's result rather than counting the
  cell as non-blank. That is a strong behaviour — one `#N/A` anywhere in the range turns the whole
  count into `#N/A` — and it is exactly the kind of thing that should not be inferred from an
  implementation. It is on the probe list.

## Errors

The documentation states no error conditions for this function.

Reference-engine behaviour, recorded as implementation fact and not as documented behaviour:

| Error | Condition |
|---|---|
| `#VALUE!` | An array-valued argument is supplied instead of a range |
| propagated | An error value in a scanned cell surfaces as the function's result |
| `#VALUE!` | Argument-count or preparation failure |

The array-rejection row has the live-Excel anchor described above. The error-propagation row has no
anchor at all.

## Relationships

- **[COUNTA](FUNC.COUNTA.md)** — the near-complement that is not a complement, because both count
  empty-text cells. The pair is the clearest illustration in Excel that "blank" is not one concept.
- **[COUNT](FUNC.COUNT.md)** — counts numbers only; `0` counts there and does not count here, which
  is the one place all three functions agree in spirit.
- **[COUNTIF](FUNC.COUNTIF.md)** — `COUNTIF(range,"")` and `COUNTIF(range,"<>")` are the usual
  attempted substitutes. They run the criteria matcher's own blank-and-empty-text rules, which are
  a third definition again, so they are not reliable stand-ins. If you need a specific definition of
  blank, write it explicitly with `ISBLANK` or `LEN`.
- **[ISBLANK](FUNC.ISBLANK.md)** — the per-cell predicate, and a *fourth* definition: `ISBLANK` is
  false for a formula returning `""`, where `COUNTBLANK` counts it. `SUMPRODUCT(--ISBLANK(range))`
  and `COUNTBLANK(range)` therefore disagree exactly on the empty-text cells, and that disagreement
  is the cleanest available diagnostic for how many such cells a range holds.
- **`AREAS`, `ISFORMULA`, `FORMULATEXT`, `SUBTOTAL`, `AGGREGATE`** — named in the upstream record as
  the same-direction reference-sensitive surfaces that also reject array substitutes. They are the
  family `COUNTBLANK` belongs to structurally, even though it is filed with the counting functions.

## Notes for implementers

1. **Take the reference, not the values.** Resolving to values first destroys the information the
   function needs: you cannot count cells that never existed as values. This is why the surface is
   declared reference-visible at the adapter.
2. **Count by extent minus defined cells, then re-add the empty-text ones.** That is the shape of
   the reference engine's sparse path and it is the only formulation that is correct and fast on a
   whole-column reference.
3. **Reject arrays deliberately.** It is tempting to reuse the aggregate expansion helper that
   serves `COUNT` and `COUNTA`; that is precisely the mistake the upstream bug stream records, and
   the root-cause note calls it inheriting an over-permissive shape rather than a regression.
4. **Decide the error policy explicitly.** Counting an error cell as non-blank and propagating it
   are both defensible; the reference engine propagates. Whichever you choose, it should be a
   declared per-family policy rather than a side effect of the coercion helper you called
   ([chapter 02](../model/02-coercion-and-lifting.md)).
5. **Empty text is a Text value.** The predicate is "is this Text of zero length", not "does this
   render as nothing". A number formatted to display nothing is not blank.

## What has not been checked

No Handbook vector suite exists for `COUNTBLANK`; `vectors/` publishes nothing at this revision. No
Excel-comparison evidence record names this surface in its subjects. The Handbook has not itself
compared `COUNTBLANK` to Excel.

The one nearby anchor is OxFunc's `BUG-FUNC-011`, a closed stream containing a dated live Excel
replay of the array-versus-range distinction with explicit contrast controls. It is good evidence
for the one question it asked and says nothing about the rest of this page.

Inputs worth probing first:

1. **`=COUNTBLANK(A1:A3, C1:C3)`** — the multi-argument question raised by the signature divergence
   above. One cell settles whether the documentation or the registered arity is right, and the
   answer changes what the Handbook publishes as this function's signature.
2. **A range containing one `#N/A`** — does `COUNTBLANK` propagate the error, as the reference
   engine does, or count the cell as non-blank and return a number? This is an undocumented and
   high-impact behaviour; a single stray error turning a count into an error is very visible.
3. **A formula returning `""` alongside a literal `""` and a never-touched cell** — three routes to
   "looks empty", and the documentation only names the first. Run `COUNTA`, `COUNTBLANK` and
   `ISBLANK` over the same three cells and the four definitions of blank separate cleanly.
4. **A cell containing `0` with a number format that hides zeros** — confirms the rule is about the
   value and not the display.
5. **`COUNTBLANK(A:A)` on a sheet with a few filled cells** — declared extent versus used extent,
   entirely undocumented, and the answer determines whether the function is usable on whole columns
   at all.
6. **`=LET(d,{"";1},COUNTBLANK(d))`** — re-runs the upstream array-rejection replay from the
   Handbook's own side rather than inheriting it.
7. **A merged-cell region, and a range spanning a hidden row** — neither is addressed anywhere, and
   both are common in the spreadsheets where this function gets used.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| blank (COUNTBLANK's sense) | Nothing at all, or a formula result of empty text |
| empty text | A zero-length string, `""` |
| declared extent | The full cell count a reference designates, including never-touched cells |
| reference-visible | The argument reaches the function as a reference rather than as resolved values |
| array substitute | An inline array or `LET`-bound array passed where a range is expected |

## Sources

- Microsoft, "COUNTBLANK function" —
  <https://support.microsoft.com/en-us/office/countblank-function-6a92d772-675c-4bee-b346-24af6bd3ac22>
  (retrieved for this pass: the one-argument syntax `COUNTBLANK(range)`, the statement that the
  function accepts one argument, and the two remarks — formulas returning `""` are counted, cells
  with zero values are not).
- Handbook call-model chapters [01 The value universe](../model/01-value-universe.md) (Empty as a
  value kind; the raw-versus-published boundary) and
  [02 Coercion and lifting](../model/02-coercion-and-lifting.md) (reference resolution as an
  explicit step; error propagation as a per-family declaration).
- Handbook projections `data/functions/FUNC.COUNTBLANK.json` — which records the documented
  `COUNTBLANK(range)` display signature alongside an arity of minimum 1 and maximum 255, the
  divergence discussed above — and `data/presence/FUNC.COUNTBLANK.json`.
- OxFunc `docs/bugs/streams/BUG-FUNC-011_countblank_range_only_parity_gap.md` — the dated live Excel
  replay pinning array-substitute rejection against true-range blank counting, with `COUNT`,
  `COUNTA`, `ROWS` and `COLUMNS` as contrast controls and `AREAS`, `ISFORMULA`, `FORMULATEXT`,
  `SUBTOTAL`, `AGGREGATE` as same-direction policy neighbours. An upstream observation with its own
  scope, not a Handbook measurement.
- OxFunc `crates/oxfunc_core/src/functions/countblank_fn.rs` at commit `473efa3` — the
  extent-minus-defined-cells sparse count, the empty-text predicate, the array-substitute
  rejection, and the error-cell propagation, read as implementation facts about the reference
  engine.
