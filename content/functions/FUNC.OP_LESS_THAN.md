---
schema: efh.function-page/v1
function_id: FUNC.OP_LESS_THAN
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
family: operator_compare_concat_family
role_in_family: "Strict less-than: the operator that makes the family's cross-type ordering visible, because ordering only means something when the operands differ."
---

## What it computes

`A < B` returns `TRUE` when `A` precedes `B` in Excel's comparison order, and `FALSE`
otherwise.

The three shared comparison rules are stated in full on
[`FUNC.OP_EQUAL`](FUNC.OP_EQUAL.md). This operator is where the first of them stops being an
abstraction, because ordering across kinds is only observable through the strict
inequalities:

```
   any Number   <   any Text   <   any Logical
```

OxFunc's provisional compare/concat contract records this as "empirically observed Excel type
ordering rather than numeric-text coercion", with the current-baseline findings `="10">2`
yielding `TRUE` and `=FALSE<TRUE` yielding `TRUE`. Read as an ordering, the first says the
text `"10"` outranks the number 2 *because it is text*, not because of any numeric
comparison; a reader who expects `"10" < 2` to be false for numeric reasons gets the right
answer for the wrong reason, and will be wrong the moment the numbers change. The second
says the logicals are ordered among themselves with `FALSE` before `TRUE`.

Within a kind:

- **Numbers** order numerically, but through the family's normalization lane rather than
  exact IEEE comparison — the contract records `=0.1+0.2<0.3` as `FALSE`, which exact
  arithmetic would make `TRUE`. See [`FUNC.OP_EQUAL`](FUNC.OP_EQUAL.md) rule 3.
- **Text** orders by a case-insensitive comparison. The collation beyond that — accents,
  non-Latin scripts, locale-specific sequences — is explicitly out of the contract's slice,
  and the Handbook has measured only the cases in FN-OP_EQUAL-001, FN-OP_LESS_THAN-001 and
  FN-SORT-001.
- **Logicals** order `FALSE` before `TRUE`.

## Arguments

| Position | Name | Meaning |
|---|---|---|
| 0 | `A` | Left operand: the candidate lesser value. Required. |
| 1 | `B` | Right operand: the candidate greater value. Required. |

Arity is exactly 2; no optional arguments, no defaults. Order is load-bearing — this is the
first comparison page where swapping operands changes the answer.

## Result and edge cases

Returns a `Logical` (`KernelSignatureClass::Custom`).

- **Equal operands** give `FALSE`; `<` is strict.
- **Different kinds** give the kind ordering, always, with no numeric interpretation of text.
- **Blank operands.** The context-sensitive blank rule applies: blank behaves as `0` against
  a number, `""` against text, `FALSE` against a logical. So a blank cell is not less than
  zero, and is not less than `""` — but it *is* less than any positive number and less than
  `TRUE`.
- **Text ordering of numeric strings.** `"10" < "9"` is `TRUE` under text ordering while
  `10 < 9` is `FALSE`; the difference is entirely in whether the operands are text. Sorted
  columns of numbers stored as text are the standard way this reaches a real workbook.
- **Arrays.** Ordinary broadcast with `#N/A` for unsupplied coordinates.

## Errors

| Error | Condition |
|---|---|
| any incoming error | Propagates unchanged; `ErrorCollapseProfile::None`. |
| `#N/A` | Inside an array result only: a coordinate neither operand supplies. |

`data/functions/FUNC.OP_LESS_THAN.json` records no Microsoft documentation URL (`docs` is
`null`).

## Relationships

- [`FUNC.OP_LESS_EQUAL`](FUNC.OP_LESS_EQUAL.md) — the non-strict form.
- [`FUNC.OP_GREATER_THAN`](FUNC.OP_GREATER_THAN.md) — the mirror. `A<B` and `B>A` should
  agree; that they do is an assumption worth testing rather than a theorem.
- [`FUNC.OP_EQUAL`](FUNC.OP_EQUAL.md) — the rules; also the third leg of trichotomy.
- `MIN`, `SMALL`, `SORT`, `RANK` — functions that order values. They do not necessarily use
  this operator's rules; `MIN` over a range ignores text entirely, where `<` ranks it above
  every number. Do not reason from one to the other.
- `MATCH` with match type `1` or `-1`, and `VLOOKUP`/`HLOOKUP` in approximate mode, depend on
  a data ordering that must agree with the comparison ordering. Mixed-kind columns are where
  that assumption breaks.
- `COUNTIF(range,"<5")` — criteria strings again: parsed by the consuming function, not this
  operator.

## Notes for implementers

- Implement one comparison core returning a three-way result (less, equal, greater, plus an
  error lane), and derive all six operators from it. Six independent implementations will
  disagree somewhere, and the disagreement will be invisible until a user finds it.
- The kind ordering must be applied before any within-kind comparison, and must be total.
- Case-insensitive text comparison and locale collation are not the same problem. Record
  which one you implemented; the contract's own slice stops at the installed baseline.
- Do not reduce `A<B` to `NOT(A>=B)` in the presence of errors — the error must reach the
  result, not become a logical.

## What has not been checked

No Handbook vector suite covers `<`, and no Excel-comparison evidence record is attached to
this page. Apart from the notes below, nothing here is a measurement.

Probes worth running first:

1. **Totality of the kind order.** All ordered pairs drawn from {number, text, logical,
   blank}, in both directions, checked with `<` and `>`. Three published findings imply the
   ordering; a complete matrix would establish it.
2. **Mirror consistency.** `=A<B` against `=B>A` over the same corpus. Any disagreement means
   the six operators are not derived from one core.
3. **Trichotomy.** For each pair, exactly one of `<`, `=`, `>` should be `TRUE`. Under a
   normalization comparison model this should hold; under a tolerance model near the boundary
   it might not.
4. **Text collation.** `="a"<"B"` (case-insensitive ordering puts `a` before `B`; a
   code-point ordering would not), plus accented and further non-Latin pairs under two locales.
   Hangul jamo, Japanese repeat marks and the Han iteration mark are read in the notes below; the
   collation rule behind them is not identified, and further Hangul comparisons are not yet
   published.
5. **Numeric boundary.** The near-15-digit pairs from the `=` page, run through `<` as well,
   to confirm the same normalization is in force for ordering as for equality.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `OP_LESS_THAN` that an implementer matching Excel, or a reader explaining
an unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's
answer and where it was recorded; "measured" means the statement is read off recorded Excel
answers, "inferred" marks what goes beyond them. Where older text on this page says a behaviour
was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

### FN-OP_EQUAL-001: Conjoining Hangul jamo do not compare equal to the precomposed syllable

*Quirk; measured; recorded 2026-11-04.* Applies to OP_EQUAL, OP_LESS_THAN.

U+1100 U+1161 is canonically equivalent to the syllable U+AC00 in Unicode, but Excel's
comparison says they are not equal, and the jamo sequence sorts first. The same holds with a
final consonant, U+1100 U+1161 U+11A8 against U+AC01.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `="가"="가"` | as typed | `FALSE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `="가"<"가"` | as typed | `TRUE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `="각"="각"` | as typed | `FALSE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): 32 recorded readings on the fn3_collation sheet.

- Open: Twelve further comparisons (other leading consonants, a trailing consonant, a compound vowel, compatibility jamo) were read on 2026-11-04. They are not yet in a published capture, so this note does not rest on them.

Evidence:

- DnaCalc `crates/calc-func/tests/captures/fn3_questions/oracle.capture.json`, sheet fn3_collation, rows 290 to 321 at `eceb809985`
- DnaCalc `crates/calc-func/tests/captures/fn3_questions/oracle.capture.json`, sheet fn3_collation, row 290, column 4 at `eceb809985` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn3_questions/oracle.capture.json`, sheet fn3_collation, row 291, column 4 at `eceb809985` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn3_questions/oracle.capture.json`, sheet fn3_collation, row 296, column 4 at `eceb809985` (readings)

### FN-OP_LESS_THAN-001: Punctuation before a Japanese repeat mark sorts one way after a Latin letter, the other after Han

*Quirk; measured; recorded 2026-11-04.* Applies to OP_GREATER_THAN, OP_LESS_THAN.

Comparing a string with punctuation before a repeat or prolonged-sound mark against the same
string without the punctuation, the order depends on the character at the start. After a Latin
letter the punctuated string is smaller: "a-ゝx" < "aゝx". After a Han character it is larger:
"時-ゝx" > "時ゝx". This held for every pair read, with the marks ゝ, 々 and ー and with hyphen-minus,
U+2212 and U+2019 as the punctuation. In all 66 readings the two strings are never equal, and a
string with the mark added sorts after the string without it, after Latin and after Han alike:
"a−ゝ" > "a−".

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `="a-ゝx"<"aゝx"` | as typed | `TRUE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `="a-ゝx">"aゝx"` | as typed | `FALSE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `="時-ゝx"<"時ゝx"` | as typed | `FALSE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `="時-ゝx">"時ゝx"` | as typed | `TRUE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `="a−ゝ"<"aゝ"` | as typed | `TRUE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `="時−ゝ">"時ゝ"` | as typed | `TRUE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `="a−ゝ">"a−"` | as typed | `TRUE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `="時−ゝ">"時−"` | as typed | `TRUE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): 66 recorded readings of =, < and > on 22 pairs of strings, 11 pairs after 時
and 11 after a.

- Open: The collation rule behind this is not identified, and none is claimed here.

Evidence:

- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4.capture.json`, sheets q001 to q066 at `2d0eddcac8` — 66 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4.capture.json`, sheet q059, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4.capture.json`, sheet q060, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4.capture.json`, sheet q050, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4.capture.json`, sheet q051, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4.capture.json`, sheet q029, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4.capture.json`, sheet q006, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4.capture.json`, sheet q027, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4.capture.json`, sheet q003, row 20, column 10 at `2d0eddcac8` (readings)

### FN-SORT-001: A Han iteration mark is not read as a repeat of the character

*Quirk; measured; recorded 2026-11-04.* Applies to OP_LESS_THAN, SORT.

In text comparison and SORT, after a Han character the iteration mark 々 sorts before Latin
letters and before the doubled character: 時 < 時々 < 時a < 時b < 時時. Latin letters sort before Han
here, and 時々 is not treated as 時時.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `="時々"<"時時"` | as typed | `TRUE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `="時々"<"時a"` | as typed | `TRUE` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=TEXTJOIN("\|",FALSE,SORT({"時々";"時時";"時";"時a";"時b"}))` | as typed | `時\|時々\|時a\|時b\|時時` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): 77 recorded controls over three bases (時, 人, 漢).

Evidence:

- DnaCalc `crates/calc-fml/tests/acceptance/f3.rs`, fn1_han_iteration_comparison_and_lookup_order at `44db028d72` — recorded order pinned
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/fn1_collation.capture.json`, sheet fn1_collation, row 2, column 4 at `f3275d997a` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/fn1_collation.capture.json`, sheet fn1_collation, row 3, column 4 at `f3275d997a` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/fn1_collation.capture.json`, sheet fn1_collation, row 229, column 4 at `f3275d997a` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Machine name | Meaning |
|---|---|
| `Arity { min: 2, max: 2 }` | Exactly two operands |
| `ArgPreparationProfile::ValuesOnlyPreAdapter` | References resolved to values before the operator runs |
| `CoercionLiftProfile::Custom` | Operator-specific comparison rules |
| `KernelSignatureClass::Custom` | Kernel shape is function-specific |
| `LiftBroadcastProfile::SurfaceNative` | The operator does its own array lifting |
| `ErrorCollapseProfile::None` | Error operands propagate; no precedence folding |
| `Logical` | The return kind |

## Sources

- `data/functions/FUNC.OP_LESS_THAN.json` at OxFunc `473efa3` — identity, arity, signature
  `A < B`, classification. `docs` is `null`: **no Microsoft documentation URL is recorded for
  this entry.** Microsoft's account of the comparison operators lives in the support article
  *Calculation operators and precedence in Excel*, not yet linked from the data projection.
- `data/presence/FUNC.OP_LESS_THAN.json` — implementing module
  `crates/oxfunc_core/src/functions/operator_compare_concat_family.rs`.
- OxFunc `docs/function-lane/FUNCTION_SLICE_OPERATOR_COMPARE_CONCAT_FAMILY_CONTRACT_PRELIM.md`
  — the type-ordering lane, case-insensitivity, blank lanes, the normalization model and the
  `=FALSE<TRUE`, `="10">2`, `=0.1+0.2<0.3` findings; provisional by its own header, with
  locale collation explicitly out of slice.
- Handbook `content/model/01-value-universe.md`, `02-coercion-and-lifting.md`,
  `03-call-pipeline.md`, and [`FUNC.OP_EQUAL`](FUNC.OP_EQUAL.md) for the three rules.
