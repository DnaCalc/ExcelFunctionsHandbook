---
schema: efh.function-page/v1
function_id: FUNC.ISLOGICAL
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
family: is_predicates_family
role_in_family: The Logical-kind test; the member that makes TRUE/FALSE visible as a kind rather
  than as a number in disguise.
---

## What it computes

`ISLOGICAL(value)` returns `TRUE` when the value it receives has kind `Logical` — that is, when
it is the boolean `TRUE` or the boolean `FALSE` — and `FALSE` for every other kind.

Microsoft's condition: "Value refers to a logical value."

`Logical` is a **distinct kind**, not a number
([the value universe](../model/01-value-universe.md)).
It converts to 1 or 0 on demand in arithmetic contexts, which is why `TRUE+0` is `1` and
`SUM(TRUE)` is `1`, but the conversion is a coercion performed by the consuming function, not a
property of the value. `ISLOGICAL` classifies before any of that happens, so:

- `ISLOGICAL(TRUE)` is `TRUE`
- `ISLOGICAL(1)` is `FALSE`
- `ISLOGICAL("TRUE")` is `FALSE` — the string, not the boolean
- `ISLOGICAL(1=1)` is `TRUE` — comparison operators produce `Logical` values

The last line is the useful one: `ISLOGICAL` is the way to confirm that a comparison, a
`NOT`/`AND`/`OR` result, or an `ISxxx` result really is a boolean and not a number that
happens to be 1 or 0.

## Arguments

`value` — required, exactly one.

Not converted. Microsoft's remark: "The value arguments of the IS functions are not converted."
The to-logical primitive described in
[coercion and lifting](../model/02-coercion-and-lifting.md) — under which zero becomes `FALSE`,
nonzero becomes `TRUE`, and in some contexts the strings `"TRUE"`/`"FALSE"` are accepted — is
exactly what `ISLOGICAL` does not do. Whether text that names a logical is accepted at all is
per-family policy elsewhere in Excel; here it is settled, because no conversion is attempted.

The argument is a values position: a reference is resolved before the function runs
(`ArgPreparationProfile::ValuesOnlyPreAdapter`).

## Result and edge cases

Returns a `Logical`, or an array of `Logical` when the argument is an array.

- **`FALSE` is a logical.** `ISLOGICAL(FALSE)` is `TRUE`. The predicate is about the kind, not
  the truth value, and an implementation that special-cases falsiness will get this wrong.
- **`0` and `1` are `FALSE`.** Numbers, whatever their value.
- **The strings `"TRUE"` and `"FALSE"` are `FALSE`.** They are `Text`.
- **`Empty` is `FALSE`.** A blank cell is not a logical, even though most boolean contexts treat
  it as `FALSE`.
- **`Error` is `FALSE` and does not propagate**; `ISLOGICAL` is one of the declared per-family
  exceptions to the propagation discipline in
  [coercion and lifting](../model/02-coercion-and-lifting.md).
- **Logicals in scanned ranges are ignored by some aggregates** even though they are present as
  values. That is a scan-policy fact about `SUM` and friends, not about `ISLOGICAL`, but it is
  the same underlying kind distinction seen from the other side, and it is why a column of
  `TRUE`/`FALSE` can be simultaneously "logical" to `ISLOGICAL` and invisible to `SUM`.
- **Arrays** are classified elementwise into a same-shaped mask.

## Errors

`ISLOGICAL` returns no error of its own for any value it can be given. The only failure
available to it is **arity**: zero arguments, or two. `ISLOGICAL()` is expected to be refused at
formula entry rather than evaluated ([the call pipeline](../model/03-call-pipeline.md)); the
reference engine, having no entry-time surface, reports `#VALUE!` for both the too-few and the
too-many case.

Microsoft's IS-functions page documents no error return for `ISLOGICAL`.

## Relationships

- **`TYPE`** returns `4` for exactly the values `ISLOGICAL` calls `TRUE`. `4` is the odd number
  out in the `TYPE` code table — 1, 2, 4, 16, 64 — and the gap at 8 is a reminder that these are
  bit flags by origin.
- **`ISNUMBER`** is the predicate people reach for when they want "is it 1 or 0 or TRUE or
  FALSE?" and it will not give them that; `ISNUMBER(TRUE)` is `FALSE` and `ISLOGICAL(1)` is
  `FALSE`, so neither predicate alone covers the union. `OR(ISNUMBER(x), ISLOGICAL(x))` does.
- **`N`** converts a logical to 1 or 0 — the conversion `ISLOGICAL` declines to perform. `N` is
  the standard way to turn a `Logical` mask into a summable numeric mask, as is the double unary
  `--`.
- **`TRUE()` and `FALSE()`** manufacture the values this predicate tests for.
- **`ISNONTEXT`** is `TRUE` for logicals too, but for six other kinds as well; `ISLOGICAL` is the
  precise test.

## Notes for implementers

1. **Model `Logical` as its own kind.** An engine that represents booleans as numbers 1 and 0
   cannot implement `ISLOGICAL` at all, and cannot implement `TYPE` either. This is the single
   design decision the function polices.
2. **Do not accept `"TRUE"`/`"FALSE"` text.** The to-logical primitive's text acceptance is
   per-family policy elsewhere; it must not leak into a kind test.
3. **`ISLOGICAL(FALSE)` must be `TRUE`.** Worth an explicit test, because a naive "is it truthy?"
   shortcut passes every other case.
4. **The kind must survive comparison operators.** `1=1` has to produce a `Logical`, not a
   number, or `ISLOGICAL` will report `FALSE` on the most natural way of producing a boolean.

## What has not been checked

No Handbook vector suite exists for `ISLOGICAL`; `vectors/` publishes nothing for this function.
No Excel-comparison evidence record names `ISLOGICAL` as a subject — **nobody has checked this
function's answers against Excel inside the Handbook's record.** The reference engine's own
answers appear on the battery panel under their own label; they are not Excel's answers.

The probes that would settle the open questions:

1. **Logicals from every producer** — a typed `TRUE`, `TRUE()`, `1=1`, `NOT(0)`, `AND(1,1)`, an
   `ISNUMBER` result, and a logical stored in a cell and read back — all through `ISLOGICAL`. If
   any producer yields a number rather than a `Logical`, that is a genuine finding about Excel's
   value model, and this is the cheapest place to detect it.
2. **A logical returned by an add-in through the C API**, since the raw-return boundary is
   broader than the published-result boundary
   ([the value universe](../model/01-value-universe.md)).
3. **A logical inside an array literal versus a logical in a scanned range**, read through
   `ISLOGICAL` after `INDEX`, to confirm that range scanning does not change the kind on the way
   through.
4. **A checkbox control's linked cell** on a modern build, which is a newer route to a `Logical`
   in a cell and has never been checked here.
5. **Arity at entry** — whether Excel refuses `=ISLOGICAL()` at entry or evaluates it.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `ISLOGICAL` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-ISNUMBER-001: ISNUMBER, ISTEXT, ISNONTEXT, and ISLOGICAL perform no coercion, treating numeric/logical text as text and rich images as ISNONTEXT TRUE

*Quirk; measured; recorded 2026-11-22.* Applies to ISLOGICAL, ISNONTEXT, ISNUMBER, ISTEXT.

In Excel 16.0 builds 20430 and 20527, ISNUMBER, ISTEXT, ISNONTEXT, and ISLOGICAL test the
runtime value type without implicit conversion: ISNUMBER("19") and ISNUMBER(TRUE) are FALSE,
ISLOGICAL("TRUE") and ISLOGICAL(0) are FALSE, ISTEXT("") is TRUE, and ISNONTEXT is TRUE for
numbers, logicals, blank cells, worksheet errors, and _webimage pictures (while ISTEXT,
ISNUMBER, and ISLOGICAL are FALSE for _webimage pictures). Each matches 1,227/1,227 full-bank
and 120/120 pinned rows.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=ISNUMBER("19")` | as typed | `FALSE` | Excel 16.0.20527.20196 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=ISTEXT("")` | as typed | `TRUE` | Excel 16.0.20527.20196 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=ISNONTEXT(A1)` | as typed | `TRUE` | Excel 16.0.20527.20196 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=ISLOGICAL(A11)` | as typed | `FALSE` | Excel 16.0.20527.20196 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `ISNUMBER(0)` | value `0.0` (`0x0000000000000000`) (cells) | `TRUE` | Excel 16.0 build 20430, Windows 64-bit, cell_value2_bulk; CPU not recorded |

Basis (measured): Measured across Excel 16.0.20527.20196 (dc_nzpw rows 39-61, 152-155,
177-180), Excel 16.0.20527.20088 (jnt_logical_info rows 18-21, 115-130), Excel 16.0.20527
(fetch.rs rows 516-517), and 1,227/1,227 full-bank and 120/120 pinned rows for each function.

Evidence:

- DnaCalc `crates/calc-func/src/catalogue/information.rs`, isnumber; istext; isnontext; islogical (lines 154-192) at `c08fee0a47` — Strict runtime type checks without coercion
- DnaCalc `crates/calc-func/witness-status.tsv`, ISLOGICAL; ISNONTEXT; ISNUMBER; ISTEXT at `c08fee0a47` — 1,227/1,227 full-bank and 120/120 pinned rows matched for each
- DnaCalc `crates/calc-func/tests/captures/dc_nzpw/oracle.capture.json`, sheet information_functions row 42 column 4 at `c08fee0a47` (readings)
- DnaCalc `crates/calc-func/tests/captures/dc_nzpw/oracle.capture.json`, sheet information_functions row 48 column 4 at `c08fee0a47` (readings)
- DnaCalc `crates/calc-func/tests/captures/dc_nzpw/oracle.capture.json`, sheet information_functions row 51 column 4 at `c08fee0a47` — A1 is blank; #N/A in B7 (row 54) is also TRUE (readings)
- DnaCalc `crates/calc-func/tests/captures/dc_nzpw/oracle.capture.json`, sheet information_functions row 59 column 4 at `c08fee0a47` — A11 holds text "TRUE" (readings)
- DnaCalc `crates/calc-func/tests/witnesses/ISNUMBER.json`, w111broad-20260929-ISNUMBER-00000 at `c08fee0a47` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| `Logical` | The TRUE/FALSE value kind; distinct from `Number`, convertible to 1/0 on demand |
| to-logical | The coercion primitive `ISLOGICAL` deliberately does not perform |
| mask | The same-shaped array of `Logical` produced from an array argument |
| kind test | A predicate that classifies the delivered kind instead of converting it |

## Sources

- Microsoft, "IS functions" —
  <https://support.microsoft.com/en-us/office/is-functions-0f2d7971-6019-40a0-a171-f2d869135665>.
  Read for this page: the `ISLOGICAL` row and the non-conversion remark.
- Microsoft, "TYPE function" —
  <https://support.microsoft.com/en-us/office/type-function-45b4e688-4bc3-48b3-a105-ffa892995899>.
  Read for the code table, which assigns 4 to a logical value.
- Handbook, [the value universe](../model/01-value-universe.md) — `Logical` as a distinct kind
  and the boundary admission matrix.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — the to-logical
  primitive, its per-family text acceptance, and the range-scan treatment of logicals.
- Handbook, [the call pipeline](../model/03-call-pipeline.md) — values-only preparation and the
  admission boundary.
- `data/functions/FUNC.ISLOGICAL.json` — identity (`xlfIslogical`, code 198), arity, declared
  axes, as projected at OxFunc `473efa3`.
