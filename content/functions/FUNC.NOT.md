---
schema: efh.function-page/v1
function_id: FUNC.NOT
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
family: not_fn
role_in_family: "The unary complement: the only member of the logical group that is not a variadic scan."
---

# NOT

## What it computes

`NOT` returns the logical complement of the truthiness of its single argument.

The rule is two steps, and both are worth naming:

1. **Truthiness.** A `Logical` is itself. A `Number` is false when it is exactly 0 and true
   otherwise. Anything that is neither must first coerce to a number, and text that does not read
   as a number fails.
2. **Complement.** Return `TRUE` if step 1 produced false, `FALSE` if it produced true.

`NOT` is the only function in Excel's logical group with a fixed arity of exactly one. It is not a
reducer, so none of the scan and skip machinery of `AND`, `OR`, and `XOR` applies to it: there is
no "ignore text in a range" rule, because there is no range scan.

## Arguments

`NOT(logical)`

- `logical` is required, and the registry records arity exactly 1 — minimum 1, maximum 1. There is
  nothing optional and nothing to default.
- The argument is *a value to test for truthiness*, not specifically a logical value. Numbers are
  admissible and are tested against zero.

The commonly misunderstood point is that `NOT` is not a filter or a set complement. `NOT(A1:A3)`
is not "everything except those cells"; it is a single-value test whose behaviour on a
multi-cell reference is a shape question, addressed below.

## Result and edge cases

Returns a `Logical`, or an `Error`.

- **Zero.** `NOT(0)` is `TRUE`. This is the case people rely on for "is this blank-or-zero".
- **Any nonzero number.** `FALSE`, including negatives and very large magnitudes. There is no
  tolerance band around zero.
- **Empty string.** The reference engine surfaces `#VALUE!` for `NOT("")`; empty text is not zero
  and is not skipped, because there is no skip rule here.
- **Empty cell.** The reference engine surfaces `#VALUE!` for a scanned empty argument, which is a
  genuine divergence in shape from the `AND`/`OR` family and one of the more surprising facts on
  this page. Whether Excel agrees is listed as unchecked below.
- **Error input.** Propagates.
- **Arrays.** `NOT` is a scalar-shaped function; how a multi-cell argument is handled is governed
  by the lifting rules in [Coercion and lifting](../model/02-coercion-and-lifting.md) rather than
  by anything specific to `NOT`.

## Errors

| Error | Condition |
|---|---|
| `#VALUE!` | The argument is text that does not read as a number, or is empty. |
| any incoming error | Propagates unchanged. |

Microsoft's `NOT` page is the documented source for the argument contract; it was not re-fetched
at this revision. The empty-and-empty-string outcomes above are reference-engine behaviours.

## Relationships

- `AND`, `OR`, `XOR` are the variadic members of the same group; `NOT` is the unary one, and the
  asymmetry in blank handling between them is real, not an editorial simplification.
- De Morgan's laws relate `NOT` to `AND` and `OR` for the well-classified cases. They do **not**
  extend to the family's `#VALUE!`-on-empty-scan behaviour, which has no algebraic counterpart:
  `NOT(AND(range))` and `OR(NOT-of-each)` can differ when the range contains no truth values.
- `<>` (`FUNC.OP_NOT_EQUAL`) is a comparison, not a complement, and is what readers usually want
  when they write `NOT(a = b)`.
- `IFERROR` and `ISERROR` are what readers want when they write `NOT(ISERROR(...))`; the
  double-negation idiom `NOT(ISERROR(x))` is exactly `ISNUMBER`-style inspection and is often
  better written with the positive test.

## Notes for implementers

1. **Do not share the reducer's item classifier.** `AND`'s classifier maps blank and skipped text
   to "contributes nothing"; `NOT` has no such bucket and must fail instead. Reusing one function
   for both is the natural refactor and it silently changes `NOT`'s blank behaviour.
2. The zero test must be an exact IEEE comparison against `0.0`. Negative zero compares equal to
   zero and therefore yields `TRUE` — worth a vector when a suite exists.
3. `NOT` has no non-finite path: its result is always a logical or an error, so the publication
   policy is the default one (`NonFinite::Allow` in the classification axes) and there is no
   `#NUM!` route.

## What has not been checked

There is no Handbook vector suite for `NOT`; `vectors/` publishes nothing at this revision. No
Excel-comparison evidence record names `NOT`.

Probes worth running first:

1. `=NOT(A1)` with `A1` empty — does Excel return `TRUE` (treating blank as zero) or `#VALUE!`?
   The reference engine surfaces `#VALUE!`, and this is the single most consequential divergence
   candidate on the page, because blank-versus-zero is the everyday case.
2. `=NOT("")` — the same question for empty text.
3. `=NOT("TRUE")` — whether logical-looking text is accepted here, and whether the answer matches
   whatever `AND("TRUE")` does.
4. `=NOT(-0)` and a cell computed to negative zero — confirms the exact-zero test.
5. `=NOT(A1:A3)` entered normally and as a dynamic array — establishes the lift behaviour of a
   scalar-shaped logical function, which the classification axes record as surface-native.

## Page vocabulary

| Term | Meaning |
|---|---|
| truthiness | The mapping of a value to true/false before the complement is applied |
| unary complement | The one-argument negation, distinct from the family's variadic scans |
| `NonFinite::Allow` | Publication policy: the kernel cannot produce a non-finite result |

## Sources

- Microsoft, NOT function —
  <https://support.microsoft.com/en-us/office/not-function-9cfc6011-a054-40c7-a140-cd4ba2d87d77>
  (documented source for the argument contract; not re-fetched at this revision).
- Handbook call-model chapters
  [01 The value universe](../model/01-value-universe.md) and
  [02 Coercion and lifting](../model/02-coercion-and-lifting.md).
- `data/functions/FUNC.NOT.json`, `data/presence/FUNC.NOT.json`.
- OxFunc `crates/oxfunc_core/src/functions/not_fn.rs` at commit 473efa3 — the truthiness test and
  the empty/empty-string outcomes, read as implementation facts about the reference engine.
