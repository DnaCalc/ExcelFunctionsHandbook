---
schema: efh.function-page/v1
function_id: FUNC.SWITCH
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-STRUCT-0001
  - EV-STRUCT-0006
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
family: misc_switch_info_family
role_in_family: "The value dispatcher: compares one expression against candidate values and returns the first match's result."
---

# SWITCH

## What it computes

`SWITCH` evaluates one expression once and compares it, in order, against a list of candidate
values, returning the result paired with the first candidate that matches. If nothing matches, it
returns an optional trailing default — and if there is no default, `#N/A`.

The rule:

1. Evaluate `expression` once.
2. Walk the `(valueN, resultN)` pairs left to right. For each, test `expression = valueN` under the
   function's equality predicate.
3. On the first match, return `resultN` and stop.
4. If no candidate matched, return `default` if one was supplied, otherwise `#N/A`.

Two things make `SWITCH` more interesting than "`IFS` with an implicit `=`".

**Parity decides the default.** There is no keyword marking the default argument. `SWITCH` counts:
the expression takes one slot, and candidate/result pairs take two each, so an *even* total
argument count means the last argument is unpaired and is therefore the default. An odd total
means every candidate has a result and there is no default. Nothing in the formula text says
"default" — the argument count says it. Miscount the pairs and a value you meant as a candidate
becomes the default, silently.

**Equality is not raw IEEE equality.** The reference engine's `SWITCH` compares two numbers through
a shared helper that truncates each operand to 15 significant decimal digits and then compares the
truncated values exactly. This is *not* an epsilon tolerance and does not behave like one: two
values astride a truncation-bucket boundary compare unequal no matter how close they are, and two
values far apart inside one bucket compare equal. Text candidates are compared
case-insensitively. The Handbook records this as evidence record `EV-STRUCT-0001`, which
documents the family split between surfaces routed through that helper — `SWITCH` among them —
and surfaces deliberately kept on raw IEEE equality (`MATCH`, `XMATCH`, `DELTA`) as the
experiment's control arm.

If you take one thing from this page: `SWITCH` is a *comparison* function wearing dispatch
clothing, and its comparison is the same tolerant-looking one the `=` operator and `COUNTIF` use,
not the exact one lookup functions use.

## Arguments

`SWITCH(expression, value1, result1, [value2, result2], ..., [default])`

- **`expression`** — required. Evaluated once. Any value kind.
- **`valueN` / `resultN`** — at least one pair is required; the registry records a minimum arity of
  3 and a maximum of 255.
- **`default`** — optional, and identified only by argument parity as described above.

The commonly misunderstood positions are the last two: readers assume a named default and assume
that a numeric candidate matches a numeric-looking *text* expression. It does not — the reference
engine matches text against text and numbers against numbers, and does not cross the kind
boundary.

## Result and edge cases

Returns whatever kind the selected result carries.

- **No match, no default.** `#N/A`.
- **No match, default present.** The default, verbatim.
- **Text matching.** Case-insensitive in the reference engine, so `"A"` matches `"a"`.
- **Numeric matching.** Through the 15-significant-digit truncation predicate described above,
  not raw IEEE equality.
- **Kind mismatch.** A number and numeric text do not match each other.
- **Blank expression and blank candidate.** Match each other in the reference engine; blank against
  a non-blank does not.
- **Error as the expression.** Propagates.
- **Selected result is an error.** Returned verbatim; `SWITCH` selects, it does not inspect.
- **Selected result is an omitted slot.** The reference engine returns `#N/A`; a blank *cell* as the
  selected result becomes numeric `0`.
- **Arrays.** The call model records `SWITCH` under `ByIndexScalarArrayLift` over positions 0, 1
  and 3 — the expression, the first candidate, and the first result
  ([chapter 03](../model/03-call-pipeline.md)). As with `IFS`, that list is irreducible structure
  rather than a derivable rule.

## Errors

| Error | Condition |
|---|---|
| `#N/A` | No candidate matched and no default was supplied. |
| `#VALUE!` | Argument-count or argument-preparation failure in the reference engine, including an array or reference reaching the comparison in an unsupported position. |
| any incoming error | An error in `expression` propagates; an error in the selected result is returned unchanged. |

The classification records `ErrorCollapseProfile::SelectorBranch` with
`ErrorAlgebra::CanonicalExcelLegacy`. Microsoft's `SWITCH` page is the documented source for the
syntax and the default behaviour; it was not re-fetched at this revision (the request was
refused by the server), so the syntax above is taken from the signature recorded in
`data/functions/FUNC.SWITCH.json`.

## Relationships

- **`IFS`** is the sibling for independent conditions. Use `IFS` when the tests differ in kind; use
  `SWITCH` when they are all equality against one expression. Both return `#N/A` on no match, but
  only `SWITCH` has a real default slot.
- **`IF`** nested is what both replace.
- **`CHOOSE`** dispatches on a 1-based index rather than on a value match.
- **`LOOKUP` / `XLOOKUP` / `MATCH`** are the table-driven alternatives, and they are on the *other*
  side of the equality split recorded in `EV-STRUCT-0001`: `MATCH` and `XMATCH` compare with raw
  IEEE equality. Replacing a `SWITCH` with an `XLOOKUP` over a two-column table therefore changes
  the comparison semantics, not just the notation. That is a genuinely surprising migration hazard
  and the most practical consequence of this page.
- `SWITCH` arrived in Excel 2016. It supersedes nothing and is not a Compatibility-category entry.

## Notes for implementers

1. **Compute the default by parity before scanning**, and treat "is there a default" as a
   structural fact of the call, not something discovered mid-loop.
2. **Do not evaluate candidates or results past the first match.** Lazy evaluation is observable
   whenever a later argument would error.
3. **Choose the equality predicate deliberately and document it.** If you share a comparison helper
   with `=`, `COUNTIF`, and `SUMIF`, `SWITCH` inherits their truncation semantics. If you share one
   with `MATCH`, it inherits raw IEEE equality. Both are defensible; silently inheriting whichever
   helper was nearest is not.
4. **Case-insensitive text comparison needs a stated collation.** ASCII-case folding and full
   Unicode case folding differ for non-ASCII text, and the reference engine's helper folds ASCII
   case. Locale-sensitive text is an open area
   ([chapter 02](../model/02-coercion-and-lifting.md)).

## What has not been checked

There is no Handbook vector suite for `SWITCH`; `vectors/` publishes nothing at this revision, so
no suite-scoped claim exists for it.

Two Excel-comparison records name `SWITCH`, both as a body-only subject:

- **`EV-STRUCT-0001`** — the comparison-tolerance family split. It establishes that `SWITCH` is on
  the truncation-helper arm and that lanes spanning both arms were scored against live Excel
  16.0 build 20026. Its counts are group totals across the whole lane set and cannot be
  decomposed onto `SWITCH`; the record also warns that the scored corpus was the repair's own
  fitting set.
- **`EV-STRUCT-0006`** — the scalar-parameter array-lift structural verification, again with
  group-scoped counts only.

So `SWITCH` is better evidenced than most functions on this shelf, and still has no per-function
result. Read both records for their scope and caveats.

Probes worth running, in priority order:

1. Two numbers that agree to 15 significant digits but differ in the 16th, as `expression` and
   `value1` — does Excel's `SWITCH` match them? This is the single claim on the page with the most
   riding on it, and the record's own caveat about a fitting set is a reason to re-probe it with
   fresh pairs.
2. The same pair through `MATCH` and through `=` — confirms the family split from both sides.
3. `=SWITCH("a","A",1)` and non-ASCII case pairs — pins the collation.
4. `=SWITCH(1,"1",2,3)` — confirms numbers and numeric text do not match.
5. `=SWITCH(A1, 1, "x")` with an even versus odd argument count around the same candidate list —
   confirms the parity rule for default detection.
6. `=SWITCH(A1, 1, "x")` with `A1` empty, and with `value1` an empty cell — the blank-matches-blank
   case.

## Page vocabulary

| Term | Meaning |
|---|---|
| parity-detected default | The trailing default identified by an even total argument count |
| truncation-style equality | Operands truncated to 15 significant decimal digits, then compared exactly |
| control arm | Surfaces deliberately kept on raw IEEE equality in `EV-STRUCT-0001` |
| `ByIndexScalarArrayLift` | Scalar kernel broadcast over a named list of argument positions |

## Sources

- Microsoft, SWITCH function —
  <https://support.microsoft.com/en-us/office/switch-function-47ab33c0-28ce-4530-8a45-d532ec4aa25e>
  (documented source for syntax and the default argument; the fetch was refused at this revision,
  so the signature here comes from the projected registry data instead).
- Handbook evidence records `EV-STRUCT-0001` and `EV-STRUCT-0006`
  (`content/evidence/records/`) — the comparison-tolerance family split and the array-lift
  structural verification; both group-scoped.
- Handbook call-model chapters
  [02 Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [03 The call pipeline](../model/03-call-pipeline.md).
- `data/functions/FUNC.SWITCH.json`, `data/presence/FUNC.SWITCH.json`.
- OxFunc `crates/oxfunc_core/src/functions/misc_switch_info_family.rs` and
  `crates/oxfunc_core/src/functions/excel_numeric_compare.rs` at commit 473efa3 — the parity
  default detection, case-insensitive text match, and the shared numeric-equality helper, read as
  implementation facts about the reference engine.
