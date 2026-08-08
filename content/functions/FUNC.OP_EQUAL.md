---
schema: efh.function-page/v1
function_id: FUNC.OP_EQUAL
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
family: operator_compare_concat_family
role_in_family: "The anchor comparison: the family's fullest statement of the cross-type ordering, the case-insensitive text rule and the numeric normalization the other five comparisons inherit."
---

## What it computes

`A = B` returns the logical `TRUE` or `FALSE` according to whether the two operands are equal
under Excel's comparison rules — which are not the rules of IEEE-754, not the rules of
ordinary string equality, and not the rules of a type-strict language.

Three rules do all the work. They are shared by all six comparison operators, and this page
is where they are stated in full.

### Rule 1 — cross-type ordering, not cross-type coercion

When the operands are of different kinds, comparison does **not** coerce one to the other.
It orders the kinds. OxFunc's provisional compare/concat contract records the admitted lane
as "empirically observed Excel type ordering rather than numeric-text coercion", with these
current-baseline findings: `=1="1"` yields `FALSE`, `="10">2` yields `TRUE`, and `=TRUE>0`
yields `TRUE`. Read together, those three place every number below every text, and every text
below every logical:

```
   any Number   <   any Text   <   any Logical
```

So the number 10 is less than the text `"2"`, and `FALSE` is greater than any number. Under
this rule `A = B` between operands of different kinds is always `FALSE`, no matter how
convertible they look. This is the single most important thing on the page: `1 = "1"` is
false, and every "why doesn't my lookup match?" question in a spreadsheet forum eventually
arrives here.

Note the contrast with arithmetic: [`+`](FUNC.OP_ADD.md) *does* coerce `"1"` to 1. Comparison
and arithmetic have opposite doctrines about text, in the same formula language.

### Rule 2 — text comparison is case-insensitive

The contract records comparison text lanes as case-insensitive on the admitted slice, with
`="a"="A"` yielding `TRUE`. `=` is therefore not string identity. `EXACT` is the
case-sensitive alternative.

Beyond case, text comparison raises collation questions — accents, ligatures, non-Latin
scripts, locale-specific ordering — which the contract explicitly puts out of slice ("locale
and collation-sensitive text ordering beyond the current installed baseline"). Nothing about
non-ASCII text ordering is settled here.

### Rule 3 — numeric equality is normalized, not exact

The rule that surprises engineers most. The contract records that numeric-vs-numeric
comparisons on the admitted operator slice do **not** use exact IEEE-double equality, but a
currently pinned normalization lane, with the current local model described as
"truncation-style normalization to 15 significant decimal digits on the tested compare
paths, not round-to-nearest". Its baseline finding is the classic one: `=0.1+0.2=0.3` yields
`TRUE`, together with the matching results for the other five comparisons.

This is why a spreadsheet appears not to suffer from floating-point equality problems while
the arithmetic underneath it plainly does. The comparison layer hides the residue that the
arithmetic layer creates.

Two consequences worth spelling out:

- **Equality here is not the same relation as bitwise equality.** Two distinct doubles can
  compare equal. Any implementation that reaches for `a == b` on doubles has implemented a
  different function.
- **Whether the relation is transitive depends on which model is right.** A *normalization*
  model (round or truncate each operand to 15 significant digits, then compare exactly) is
  transitive: it is equality of a function of the operands. A *tolerance* model (compare the
  difference against a scaled epsilon) is not transitive: `a≈b` and `b≈c` need not give
  `a≈c`. The contract names the normalization model as its current local one and marks the
  whole lane as currently pinned rather than settled — so transitivity of `=` on numbers is
  an open question with a decisive experiment behind it.

### The other `=`

The `=` a reader types first is not this function. A leading `=` introduces a formula: it is
grammar, telling the host that what follows is an expression rather than literal content.
This operator is the `=` that appears *inside* an expression, between two subexpressions.
`=A1=B1` contains both: the first `=` is the formula marker, the second is
`FUNC.OP_EQUAL`. Chapter 03 draws exactly this line — parse-only syntax carries no semantics
and gets no function identity, while every evaluable operator does.

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | Left operand. Required. |
| 1 | `B` | Right operand. Required. |

Arity is exactly 2; no optional arguments, no defaults. Equality is symmetric, so operand
order does not change the value — but it does set the broadcast orientation when the operands
are arrays of different shapes.

## Result and edge cases

Returns a `Logical`. The declared kernel class is `KernelSignatureClass::Custom`, which is
the right shape: this is not a numeric kernel.

- **Blank operands.** The contract records blank comparison as context-sensitive, with the
  blank taking the shape of the other operand: blank vs number behaves as `0`, blank vs text
  as `""`, blank vs logical as `FALSE`. Its baseline findings include a blank cell comparing
  equal to `0` and a blank cell comparing equal to `""` — which together mean `=A1=0` and
  `=A1=""` can both be `TRUE` for the same empty `A1`, even though `0` and `""` are not equal
  to each other. This is not a contradiction; it is what context-sensitive blank coercion
  means, and it is a genuine trap.
- **Empty text versus blank.** `""` is a text value of length zero; a blank cell is the
  `Empty` kind (chapter 01). They are different values that can compare equal to the same
  things.
- **Arrays.** The contract records the ordinary broadcast rule: singleton dimensions
  broadcast, row-vs-column combinations produce two-dimensional grids, and coordinates
  neither operand can supply return `#N/A`. Its findings include `={1,2}={1;2}` producing a
  2×2 grid of logicals.
- **Errors.** An error operand propagates rather than comparing;
  `ErrorCollapseProfile::None` means there is no precedence folding of the kind reductions
  perform.

## Errors

| Error | Condition |
|---|---|
| any incoming error | Propagates unchanged. Comparison has no error-collapse profile of its own. |
| `#N/A` | Inside an array result only: a coordinate neither operand supplies. |

`=` has no coercion-failure error lane of its own, because it does not coerce across kinds —
that is the direct consequence of rule 1. `data/functions/FUNC.OP_EQUAL.json` records no
Microsoft documentation URL (`docs` is `null`), so these conditions come from the shared
call-model chapters and OxFunc's provisional contract.

## Relationships

- [`FUNC.OP_NOT_EQUAL`](FUNC.OP_NOT_EQUAL.md) — the complement, with the error caveat noted
  on that page.
- [`FUNC.OP_LESS_THAN`](FUNC.OP_LESS_THAN.md),
  [`FUNC.OP_LESS_EQUAL`](FUNC.OP_LESS_EQUAL.md),
  [`FUNC.OP_GREATER_THAN`](FUNC.OP_GREATER_THAN.md),
  [`FUNC.OP_GREATER_EQUAL`](FUNC.OP_GREATER_EQUAL.md) — the ordering siblings; same three
  rules.
- `EXACT` — case-sensitive text equality. Where `=` says `"a"="A"` is true, `EXACT` says it
  is false. If case matters, `=` is the wrong operator.
- `IF`, `COUNTIF`, `SUMIF`, `SUMIFS`, `MATCH`, `XLOOKUP` — the consumers. Note that the
  criteria *strings* of the `*IF*` family (`"=5"`, `">3"`, `"<>"`) are a separate
  mini-language parsed by those functions; they are not this operator, and their matching
  rules — wildcards, in particular — do not follow the rules on this page.
- `DELTA` — the engineering-function equivalent for numbers, returning 1 or 0.

## Notes for implementers

- Do not implement `=` as `a == b` on doubles, and do not implement it as a type-coercing
  comparison. Both are different functions from the one described here.
- Implement the kind ordering as an explicit total order over kinds, then compare within
  kind. That structure makes rules 1 and 2 separable and testable.
- Put the numeric normalization in one place shared by all six comparisons. If `=` and `<`
  normalize differently, the operators stop being consistent with each other — and a reader
  can detect that with three cells.
- Case-insensitive comparison is not `to_lowercase` in general. For ASCII it is; for Unicode,
  case folding has locale-sensitive edges (Turkish dotless i is the standard example), and
  the contract puts that surface out of slice — so a compatibility implementation should
  record what it does rather than assume.
- Comparison is `SafePure`, `Deterministic`, `NonVolatile`.

## What has not been checked

No Handbook vector suite covers `=`, and no Excel-comparison evidence record is attached to
this page. Every rule above is reported from OxFunc's provisional contract or from the shared
chapters; none of it has been re-measured by the Handbook, and the contract itself marks its
comparison lanes as currently pinned rather than settled.

Probes worth running first:

1. **Transitivity of numeric equality.** Construct triples `a`, `b`, `c` that a
   15-significant-digit normalization model and a scaled-tolerance model classify
   differently, and check `=a=b`, `=b=c`, `=a=c`. This distinguishes the two models with a
   handful of cells and is the highest-value experiment on the page.
2. **The normalization boundary.** Pairs differing at the 15th and 16th significant digit,
   generated arithmetically rather than typed (typed literals are re-parsed and may not reach
   the same doubles), across several exponent ranges — including subnormals, where a
   significant-digit rule and a relative-tolerance rule must diverge.
3. **Truncation versus rounding.** Pairs whose 16th digit rounds up across the boundary,
   which separates "truncate to 15 digits" from "round to 15 digits".
4. **Kind ordering completeness.** Every ordered pair of kinds — number, text, logical, blank
   — compared in both directions, to confirm the ordering is total and antisymmetric rather
   than merely observed on three examples.
5. **Unicode case folding.** `="ß"="SS"`, `="İ"="i̇"`, and accented pairs, under at least two
   locales.
6. **Blank in both positions.** `=A1=B1` with both cells empty, and against `0`, `""`,
   `FALSE`, to confirm the context-sensitive blank rule where there is no context to take.

## Page vocabulary

| Machine name | Meaning |
|---|---|
| `Arity { min: 2, max: 2 }` | Exactly two operands |
| `ArgPreparationProfile::ValuesOnlyPreAdapter` | References resolved to values before the operator runs |
| `CoercionLiftProfile::Custom` | Operator-specific comparison rules, not the shared numeric coercion |
| `KernelSignatureClass::Custom` | Kernel shape is function-specific (not a numeric kernel) |
| `LiftBroadcastProfile::SurfaceNative` | The operator does its own array lifting |
| `ErrorCollapseProfile::None` | Error operands propagate; no precedence folding |
| `Logical` | The return kind: `TRUE` or `FALSE` |
| `default-unexamined` | Axis provenance: a projection default, not an examined fact |

## Sources

- `data/functions/FUNC.OP_EQUAL.json` at OxFunc `473efa3` — identity, arity, signature
  `A = B`, classification, axis provenance. `docs` is `null`: **no Microsoft documentation
  URL is recorded for this entry.** Microsoft's account of the comparison operators lives in
  the support article *Calculation operators and precedence in Excel*, not yet linked from
  the data projection.
- `data/presence/FUNC.OP_EQUAL.json` — implementing module
  `crates/oxfunc_core/src/functions/operator_compare_concat_family.rs`, the family slug for
  this page.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OPERATOR_COMPARE_CONCAT_FAMILY_CONTRACT_PRELIM.md`
  — the case-insensitivity lane, the type-ordering lane, the context-sensitive blank lanes,
  the 15-significant-digit normalization model, the broadcast rule, and the current-baseline
  findings quoted above. Marked provisional; its comparison lanes are described as currently
  pinned, and locale/collation ordering is explicitly out of slice.
- Handbook `content/model/01-value-universe.md` (value kinds, Empty versus text),
  `02-coercion-and-lifting.md` (why arithmetic coerces text and comparison does not),
  `03-call-pipeline.md` (operators are functions; parse-only syntax is not).
