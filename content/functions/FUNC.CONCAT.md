---
schema: efh.function-page/v1
function_id: FUNC.CONCAT
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
family: concat_family
role_in_family: "The modern member: joins text across ranges as well as scalars, with no delimiter."
---

## What it computes

`CONCAT` joins text end to end and returns the result as one text value. Nothing is inserted
between the pieces, nothing is skipped, and the order is the order the arguments are written.

The property that makes it a new function rather than a rename of `CONCATENATE` is how it
consumes a **range**. `CONCAT` flattens a multi-cell argument and joins every cell in it;
`CONCATENATE` does not accept one. `CONCAT(A1:A3)` is a legal, meaningful call.

Stated as an operation: let each argument be expanded to a sequence of scalar values — an array
or range expands in row-major order, a scalar expands to itself — and let each scalar be
converted to text by the ordinary to-text rules. `CONCAT` returns the concatenation of all
those texts, in argument order, with the expansions inlined in place. Empty cells contribute
empty text, which is the same as contributing nothing.

Microsoft's article states the replacement relationship plainly: `CONCAT` "replaces the
CONCATENATE function", though `CONCATENATE` remains available for backward compatibility.

## Arguments

`text1` — required; `text2, …` — optional, up to 253 text arguments in total per the article.
Each argument may be a string or an array such as a cell range.

Two argument-level points worth stating:

- **There is no delimiter argument and no ignore-empty argument.** The article says so
  explicitly, and the projected description carried in `data/functions/FUNC.CONCAT.json` says
  the same. If you need either, the function you want is `TEXTJOIN`.
- **The 253 limit is on argument *slots*, not on cells.** A single range argument can
  contribute far more than 253 values. This is the point readers most often get backwards when
  migrating from `CONCATENATE`, whose 255 slots each had to be written out.

## Result and edge cases

The return kind is `Text`.

- **The 32,767-character cap is a real boundary, and it errors.** Microsoft's article states
  that a result longer than 32,767 characters returns `#VALUE!`. That is the cell text limit
  from [chapter 01](../model/01-value-universe.md), surfacing here as an error rather than as
  truncation — consistent with what that chapter records for the formula path generally.
- **Empty cells inside a range contribute nothing**, so `CONCAT` over a sparse column silently
  closes the gaps. There is no way to tell afterwards how many cells were empty.
- **Numbers and logicals convert to text.** A number contributes its General rendering, so a
  value displayed as `1 234,57` by a cell format contributes the unformatted digits. Wrap in
  `TEXT` when the rendering matters.
- Reference and array arguments are resolved and expanded before conversion, per
  [chapter 03](../model/03-call-pipeline.md).

## Errors

- `#VALUE!` when the joined result would exceed 32,767 characters (documented).
- An error value anywhere in the input propagates
  ([chapter 02](../model/02-coercion-and-lifting.md)) — including an error sitting in one cell
  of a range argument, which is the case where `CONCAT` differs most sharply from an aggregate
  that could choose to skip it.
- A value kind with no text conversion surfaces `#VALUE!` under the shared coercion rules.

## Relationships

- **`CONCATENATE`** is the function `CONCAT` replaces; Microsoft retains it for workbook
  compatibility and its own article warns that it may not be available in future versions. The
  two differ in range handling, in argument limits, and — per the axis chip this page renders —
  in array lifting. See [`CONCATENATE`](FUNC.CONCATENATE.md).
- **`TEXTJOIN`** is the third member of the family and the one to reach for by default: it adds
  the delimiter and ignore-empty arguments that `CONCAT` deliberately lacks.
- **`&` (`FUNC.OP_CONCAT`)** is the operator form. For two or three scalars it is shorter and
  equivalent in intent; it has no range behaviour.
- **`TEXTSPLIT`** is the modern inverse direction.

## Notes for implementers

1. **Expansion order is part of the contract.** Row-major within each argument, arguments in
   written order. Any other order is a different function, and the difference only shows on
   multi-row, multi-column ranges — which is exactly the shape casual tests omit.
2. **Check the length cap incrementally.** A single append that crosses 32,767 must produce the
   error rather than a truncated string; building the whole result and testing afterwards is
   correct but can allocate absurdly on adversarial input.
3. **The cap counts UTF-16 code units**, so a result made of astral characters reaches it at
   half the number of visible characters.
4. The OxFunc reference engine at commit `473efa3` enforces the cap during accumulation, but
   maps the over-length condition to `#CALC!` rather than to the `#VALUE!` the documentation
   names. That divergence is recorded here as a difference between the reference engine and
   Microsoft's documentation; the Handbook has not observed which one live Excel agrees with.

## What has not been checked

No Handbook vector suite exists for `CONCAT`, and no Excel-comparison evidence record names it.
The open items, most valuable first:

- **The over-length result.** Build a call whose result is exactly 32,767 and one whose result
  is 32,768 code units, and record what Excel returns for each. This settles the
  documentation-versus- reference-engine divergence noted above and pins whether the boundary
  is inclusive.
- **Astral characters at the cap** — whether the limit counts code units or characters, and
  whether a result can be truncated mid-surrogate.
- **A range argument containing an error value**, to confirm propagation rather than skipping.
- **A multi-row, multi-column range argument**, to pin the flattening order.
- **The 253rd and 254th argument slots**, to confirm where admission fails and whether it fails
  at formula entry or at evaluation.
- Whether `CONCAT` broadcasts at all when given several arrays of different shapes, which the
  axis chip records as surface-native behaviour but which no probe here has exercised.

## Page vocabulary

| Term | Meaning as used here |
|---|---|
| flattening | Expanding an array or range argument into its scalar values in row-major order |
| argument slot | One comma-separated position in the call, as distinct from one contributed value |
| text cap | The 32,767-UTF-16-code-unit limit on a text value |

## Sources

- Microsoft, "CONCAT function" —
  <https://support.microsoft.com/en-us/office/concat-function-9b1a9a3f-94ff-41af-9736-694cbd6b4ca2>
  (that it replaces `CONCATENATE`; up to 253 text arguments; the 32,767-character `#VALUE!`
  boundary; the absence of delimiter and IgnoreEmpty arguments; the `TEXTJOIN` recommendation).
  Retrieved for this page.
- Handbook `content/model/01-value-universe.md` (the text cap and the formula-path error) and
  `content/model/03-call-pipeline.md` (argument preparation and expansion).
- OxFunc `crates/oxfunc_core/src/functions/concat_family.rs` at commit `473efa3` — read for the
  reference engine's expansion order and over-length mapping. Implementation fact only.
