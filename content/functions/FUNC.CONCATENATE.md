---
schema: efh.function-page/v1
function_id: FUNC.CONCATENATE
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
role_in_family: "The legacy member: scalar-only joining, retained for workbook compatibility."
---

## What it computes

`CONCATENATE` joins two or more text strings into one string, in argument order, with nothing
inserted between them.

It is the legacy member of its family. Microsoft's article states that it "has been replaced
with the CONCAT function", recommends `CONCAT` going forward, and warns that `CONCATENATE` "may
not be available in future versions of Excel". Excel keeps the name so that existing workbooks
keep calculating; new formulas should use `CONCAT`, `TEXTJOIN`, or `&`.

The substantive difference from `CONCAT` is not the name. `CONCATENATE` is **scalar-shaped**:
each argument contributes one value, not a flattened range. A multi-cell range handed to
`CONCATENATE` does not join its cells.

## Arguments

`text1` — required, the first item to join; may be text, a number, or a cell reference. `text2,
…` — optional. The article states the ceiling as up to 255 items totalling a maximum of 8,192
characters.

Note that those are two different limits from the ones `CONCAT` carries (253 slots, 32,767
characters), and the 8,192-character figure is documented for `CONCATENATE` specifically. If
you are migrating formulas between the two functions, both limits change.

The argument position that most often surprises readers is any position holding a multi-cell
range. `CONCAT` accepts it and flattens; `CONCATENATE` does not consume it as a range at all,
and what happens instead depends on the shared array/implicit-intersection machinery rather
than on this function.

## Result and edge cases

The return kind is `Text`.

- Numbers and logicals convert to text by the ordinary rules; a number contributes its General
  rendering, not its displayed format.
- Empty cells contribute empty text.
- **Array lifting is asymmetric across argument positions.** The axis chip this page renders
  records `CONCATENATE` as a by-index scalar-array lift over positions 0, 1 and 2 — that is,
  only the first three arguments broadcast over an array — whereas `CONCAT` lifts natively. The
  OxFunc registry annotates that entry as verified against live Excel 16.0 build 20026; the
  Handbook reports that as an upstream record rather than as its own measurement. It is unusual
  enough to be worth stating twice: with `CONCATENATE`, whether an array argument spreads
  depends on *which slot it is in*.

## Errors

- An error value in any argument propagates
  ([chapter 02](../model/02-coercion-and-lifting.md)).
- A value kind with no text conversion surfaces `#VALUE!` under the shared coercion rules.
- Microsoft's article documents `#NAME?` as the usual symptom of a common authoring mistake —
  missing quotation marks around a text argument. That is a formula-entry problem rather than a
  behaviour of the function, and it is listed here because the article lists it and readers
  arrive looking for it.

The article's other two "common problems" are also authoring issues rather than function
semantics: quotation marks appearing in the result usually mean a missing comma between
arguments, and words running together mean no space argument was supplied. `CONCATENATE`
inserts nothing on its own.

## Relationships

- **`CONCAT`** is the modern replacement, named as such by Microsoft. It accepts ranges and
  carries different limits. See [`CONCAT`](FUNC.CONCAT.md).
- **`TEXTJOIN`** adds a delimiter and an ignore-empty flag, and is usually the better migration
  target for a `CONCATENATE` chain that interleaves separator literals.
- **`&` (`FUNC.OP_CONCAT`)** is the operator form; the article itself recommends it as the
  simpler syntax. It is not identical in shape to either function — it is a binary operator
  with the operator pipeline's own lifting behaviour.
- **`TEXT`** is what the article recommends composing with when the joined pieces need
  formatting; `CONCATENATE` will not format anything for you.

Despite its status, `CONCATENATE` is **not** in Excel's Compatibility category. It sits in the
Text category alongside `CONCAT`, and Microsoft's deprecation statement is in prose rather than
in the category taxonomy.

## Notes for implementers

1. **Do not implement `CONCATENATE` as `CONCAT` with a different arity.** The range behaviour
   and the position-dependent lifting are genuinely different, and the difference is observable
   in ordinary workbooks.
2. **Carry both documented limits separately** (255 slots, 8,192 characters here; 253 and
   32,767 there). They are not derived from a shared constant.
3. **The lift positions are irreducible structure.** The `[0,1,2]` position list is not a rule
   with a reason; it is a fact to be copied, and the call-pipeline chapter says so about this
   whole axis.
4. The OxFunc reference engine at commit `473efa3` prepares each argument as a single value and
   rejects a multi-cell range argument as an unsupported value kind, in contrast to its own
   `CONCAT`, which expands. Implementation fact about OxFunc only.

## What has not been checked

No Handbook vector suite exists for `CONCATENATE`, and no Excel-comparison evidence record
names it. The Handbook has verified nothing about this function against Excel. Worth probing
first:

- **`CONCATENATE(A1:A3)`** with three filled cells, entered in a cell in the same rows and in a
  cell outside them, which separates implicit intersection from an outright error;
- **the same call in slot 1, slot 3 and slot 4**, which is the direct test of the recorded
  `[0,1,2]` lift-position list;
- **the 8,192-character limit** — whether it is enforced, and with which error, since the
  documented figure is far below the text cap that `CONCAT` errors at;
- **the 255th and 256th argument slots**, and whether over-arity is refused at formula entry;
- whether a number argument's text conversion follows the workbook locale.

## Page vocabulary

| Term | Meaning as used here |
|---|---|
| by-index lift | Broadcasting over arrays only at a named list of argument positions |
| scalar-shaped | Each argument contributes one value; ranges are not flattened |
| retained for compatibility | Kept in the function surface so existing workbooks keep calculating |

## Sources

- Microsoft, "CONCATENATE function" —
  <https://support.microsoft.com/en-us/office/concatenate-function-8f8ae884-2ca8-4f7a-b093-75d702bea31d>
  (that it has been replaced by `CONCAT` and may not be available in future versions; up to 255
  items totalling 8,192 characters; the `&` and `TEXT` recommendations; the `#NAME?` and
  missing-comma common problems). Retrieved for this page.
- Handbook `content/model/03-call-pipeline.md` (the `ByIndexScalarArrayLift` axis and its
  empirically-pinned status) and `content/model/02-coercion-and-lifting.md` (to-text
  conversion, error propagation).
- OxFunc `crates/oxfunc_core/src/functions/concat_family.rs` at commit `473efa3` — read for the
  reference engine's scalar preparation and for the registry annotation recording the `[0,1,2]`
  lift positions as verified against live Excel 16.0 build 20026. Upstream record, quoted as
  such.
