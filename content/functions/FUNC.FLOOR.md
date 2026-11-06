---
schema: efh.function-page/v1
function_id: FUNC.FLOOR
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — FLOOR function"
    locator: "https://support.microsoft.com/en-us/office/floor-function-14bb497c-24f2-4e04-b327-b0b4de5a8886"
    role: "syntax, the two sign remarks, the #VALUE! and #NUM! conditions and the worked examples"
  - work: "Microsoft Learn — WorksheetFunction.Floor method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.floor"
    role: "the fuller statement of the sign cases, the exact-multiple remark, and the pointer to Floor_Precise as the replacement"
  - work: "IEEE 754-2019, Standard for Floating-Point Arithmetic"
    locator: "the roundToIntegral operations"
    role: "why the floor step itself is exact and the division and multiplication around it are not"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Notes
  - Page vocabulary
  - Sources
family: ceiling_floor_family
role_in_family: >-
  The legacy two-argument round-down: it counts multiples of a signed significance, so the sign of
  the significance sets the direction for negative numbers, and a positive number with a negative
  significance is an error. FLOOR.MATH and FLOOR.PRECISE exist to remove that restriction.
---

# FLOOR

## What it computes

`FLOOR(number, significance)` rounds `number` down to a multiple of `significance`. The
mathematical definition is one line:

    FLOOR(x, s) = s × floor(x / s)

The floor is taken on the **quotient**, not on the number, and that one fact sets the direction
for every sign combination. A quotient of 1.25 floors to 1 and a quotient of −1.25 floors to −2,
so the same rule rounds toward zero in some cases and away from it in others:

| `number` | `significance` | Quotient | Direction of the result | Example |
|---|---|---|---|---|
| positive | positive | positive | toward zero | `FLOOR(3.7, 2)` is `2` |
| negative | negative | positive | toward zero | `FLOOR(-2.5, -2)` is `-2` |
| negative | positive | negative | away from zero | `FLOOR(-2.5, 2)` is `-4` by the rule |
| positive | negative | negative | none: `#NUM!` | `FLOOR(2.5, -2)` |

The first, second and fourth rows are Microsoft's own examples. The third follows from the
definition and from Microsoft's VBA reference, which says a positive significance rounds a
negative number away from zero; all 292 recorded readings with that sign pattern fit it.

That definition is not quite what Excel evaluates. In recorded readings Excel rounds the quotient
to 15 significant decimal digits **before** it takes the floor:

    FLOOR(x, s) = s × floor( x / s rounded to 15 significant digits )

The difference is small in size and large in effect, because a quotient that lands just under an
integer is treated as that integer. `FLOOR(1938.84, 9.06)` is `1938.8400000000001` in Excel, where
the plain definition in binary64 gives `1929.7800000000002`, one whole step lower. *Numerical
notes* below gives the counts and the limits of that statement.

Domain and range: all real pairs except those in *Errors*. The result is a multiple of
`significance` as far as the final multiplication can represent one. It is usually at or below
`number` for a positive quotient, but not always, as the example above shows.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The numeric value to round. Required. | none |
| `significance` | The multiple to round to. Required. | none |

- **Both arguments are required.** The function has an arity of exactly two. Its modern relatives
  take `significance` as optional with a default of 1; `FLOOR` does not.
- **`significance` is a multiple, not a number of digits.** `FLOOR(1.58, 0.1)` is `1.5`.
- The projection places `FLOOR` in the Compatibility category. Microsoft's VBA reference says it
  has been replaced and points to `Floor_Precise`, while keeping it available.
- Both slots take ordinary to-number coercion in the reference engine; see
  [Coercion and lifting](../model/02-coercion-and-lifting.md). The reference engine broadcasts
  array arguments cell by cell. Array behaviour is not covered by the readings used here.

## Result and edge cases

Returns `Number`.

- **`number` is zero.** The notes below record `FLOOR(0, 0)` and `FLOOR(-0, 0)` as `+0`, and a zero
  number with a negative significance as `+0` rather than `#NUM!`. A zero number is settled before
  either error can arise.
- **`significance` is zero with a nonzero `number`** gives `#DIV/0!`.
- **`number` is already a multiple.** Microsoft's VBA reference says no rounding occurs. For
  ordinary sizes that holds, but it is not true at every size: see the large-quotient readings
  in *Numerical notes*, where `FLOOR(4503599627370496, 2)` moves the number.
- **A tiny negative number with a positive significance.** The rule gives one step below zero, and
  `FLOOR(-1E-300, 2)` is `-2` in the recorded readings. One reading is an exception:
  `FLOOR(-2.2250738585072014E-308, 2)` is `0`, where the quotient has fallen into the subnormal
  range.
- **The largest double.** `FLOOR(1.7976931348623157E+308, 0.1)` and the same number with
  significance `2` both give `#NUM!`, and so does its negative at significance `0.1`, `2`, `-0.1`
  and `-2`.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | Either argument is non-numeric | Microsoft |
| `#NUM!` | `number` positive and `significance` negative | Microsoft; 349 recorded readings |
| `#DIV/0!` | `significance` zero and `number` nonzero | 23 recorded readings and the notes below; not on either Microsoft page read here |
| `#NUM!` | The quotient or the final product leaves the number range | 6 recorded readings at the largest double |

Neither Microsoft page read for this entry says what a zero significance does. The readings above
are Excel 16.0 build 20430, Windows 64-bit, with the CPU not recorded.

Microsoft's support page explains the `#NUM!` example as "different signs", and its remarks say a
negative number is rounded away from zero. Neither sentence is general. `FLOOR(-2.5, 2)` has
different signs and returns a number, and `FLOOR(-2.5, -2)` in the same page's example table
rounds toward zero. The VBA reference states the cases correctly, and the table under *What it
computes* follows it.

The reference engine at commit `473efa3` tests `significance == 0` first, so it answers `#DIV/0!`
for `FLOOR(0, 0)`. Excel's recorded answer is `0`; the notes below are the record.

## Relationships

- **[FLOOR.MATH](FUNC.FLOOR.MATH.md)** takes the absolute value of the significance, so signs can
  no longer disagree, and adds a `mode` argument for negative numbers. Moving a workbook from
  `FLOOR` to `FLOOR.MATH` changes answers: `FLOOR(-2.5, -2)` is `-2`, and the default reading of
  `FLOOR.MATH` as Microsoft describes it moves the same arguments further from zero.
- **`FLOOR.PRECISE`** also ignores the sign of the significance, and is the replacement Microsoft's
  VBA reference names.
- **`CEILING`** is the upward mirror in the same implementing module.
- **[MROUND](FUNC.MROUND.md)** rounds to the nearest multiple. The notes record that it too treats
  a quotient very close to a half as that half, so the worksheet's multiple functions share the
  idea of cleaning up a quotient, though the notes do not establish a shared routine.
- **[INT](FUNC.INT.md)** is the floor to integers, toward minus infinity. `FLOOR(x, 1)` is the same
  idea, but do not assume the same answer next to an integer: `FLOOR(1*(1-2^-51), 1)` is `1`.
- **[TRUNC](FUNC.TRUNC.md)** and **[ROUNDDOWN](FUNC.ROUNDDOWN.md)** round toward zero at a decimal
  digit position rather than at a multiple.

## Numerical notes

The notes below cover zero. This section covers what the plain definition misses everywhere else.

**The plain route fails visibly.** `s × floor(x / s)` in binary64 rounds twice around one exact
step. `x / s` rounds, and when the true quotient is an integer the rounded one can land just
below it, which costs a whole step. The floor is exact. The product `s × q` rounds again, so the
answer need not be a multiple of the represented `s`.

**Excel does not take the exact floor of the stored values either.** A careful implementation
might compute the true floor of the two binary64 values. Excel does not. `0.1` is stored slightly
above one tenth, so the exact quotient of `0.5` by it is slightly below 5 and its exact floor is 4.
Excel's `FLOOR(0.5, 0.1)` is `0.5`.

**The 15-digit quotient.** Rounding `x / s` to 15 significant decimal digits, then taking the floor,
then multiplying by `s` in binary64, matches Excel on more rows than either alternative:

| Readings | Plain route | Exact floor | 15-digit quotient |
|---|---|---|---|
| 945 rows with nonzero arguments and a number as the answer, Excel 16.0 build 20430, Windows 64-bit, CPU not recorded | 857 | 848 | 945 |
| 120 calls with two literal arguments, Excel 16.0 build 20527, Windows 64-bit | 114 | not computed | 120 |

The 945 in the last column counts a subnormal quotient or `number` as zero. Those rows are part of
a set of 1,335 readings, and the rule matches all 1,335 once the error rows of *Errors* and the
zero rows of the notes below are added to it. All of these counts were recomputed for this page
from the recorded inputs. None is a held-out result: the rule was first stated from readings of
this family, and the same readings are used here. The two sets hold different inputs, so they say
nothing about a difference between the two builds.

What the rule explains, each from a recorded reading:

| Call | Excel | What happened |
|---|---|---|
| `FLOOR(1938.84, 9.06)` | `1938.8400000000001` | Quotient just under 214 counts as 214; the product lands above `number` |
| `FLOOR(-1990.8, -8.4)` | `-1990.8000000000002` | The same with both signs negative |
| `FLOOR(0.9999999999999999, 0.1)` | `1` | Result greater than `number` |
| `FLOOR(1*(1-2^-51), 1)` | `1` | The quotient rounds to 1 at 15 digits |
| `FLOOR(1*(1-2^-50), 1)` | `0` | One step further from 1, and it does not |
| `FLOOR(4503599627370496, 2)` | `4503599627370500` | An exact multiple, moved, because the quotient has 16 digits |

**The last row is the unintuitive one.** Once the quotient has more than 15 significant digits the
rounding step moves it by far more than one unit, and the answer changes though the number was
already a multiple. A power-of-two significance gives no protection, although both the division
and the multiplication are exact there. Among the recorded readings the rule also holds at very
large quotients, where the answer can differ from `number` in the sixteenth digit.

**Everyday consequence.** For prices and quantities in ordinary ranges the 15-digit step does what
a user wants: a quotient of 28.999999999999996 counts as 29. The price is that the result can sit
one unit above `number`, so `FLOOR(x, s) <= x` is not safe as a test in a formula.

## What has not been checked

No Handbook vector suite exists for `FLOOR`; `vectors/` publishes nothing at this revision. No
evidence record lists this surface as a subject. It appears by name inside the group listing of
the structural record `EV-STRUCT-0011`, whose reader warning does not allow a figure to be
attributed to a single surface, so nothing is taken from it here.

The notes below settle the zero cases. Still unmeasured or unsettled after them:

1. **The tie rule of the 15-digit step.** Ties-to-even and ties-away-from-zero both match all 1,335
   readings, so the readings do not separate them. Truncation to 15 digits, and rounding to 14 or
   16 digits, do not match.
2. **Held-out inputs for the 15-digit rule.** See *Numerical notes*. Inputs built to sit on either
   side of a 15-digit boundary, with significances other than `1`, would test it harder than the
   recorded sets do.
3. **Non-numeric arguments.** Every one of the 1,335 readings has two numeric cells. Text, logical
   values, blank cells and error values are unread here, and the `#VALUE!` row above rests on
   Microsoft's page alone.
4. **Arrays and ranges.** The reference engine's broadcast is an implementation fact, not an Excel
   observation.
5. **Subnormal arguments.** The only subnormal `number` values read are `5E-324` and its negative,
   and they behave as zero: `FLOOR(5E-324, -2)` is `0`, not `#NUM!`. A companion capture from the
   same date records that this route changes subnormals and negative zero as they enter a cell, so
   those rows may describe the cell and not `FLOOR`. The same doubt applies to `-0` in the notes
   below: Excel's answer stands, but whether the function received a negative zero is not known.
6. **Other environments.** No input has been read in two environments. The Mac, Excel for the web,
   32-bit Excel and Intel CPUs are unread.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `FLOOR` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-FLOOR-001: FLOOR of zero is +0, even with significance 0

*Quirk; measured; recorded 2026-11-04.*

FLOOR(0,0) and FLOOR(-0,0) return +0. A zero number with a negative significance also returns
+0, not #NUM!. Only a nonzero number with significance 0 gives #DIV/0!.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `FLOOR(0, 0)` | number `0` (`0x0000000000000000`); significance `0` (`0x0000000000000000`) (cells) | `0` (`0x0000000000000000`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `FLOOR(-0, -2)` | number `-0` (`0x8000000000000000`); significance `-2` (`0xc000000000000000`) (cells) | `0` (`0x0000000000000000`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `FLOOR(1, 0)` | number `1` (`0x3ff0000000000000`); significance `0` (`0x0000000000000000`) (cells) | `#DIV/0!` | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |

Basis (measured): 12 recorded readings.

Evidence:

- DnaCalc `apps/dna-onecalc/tests/acceptance.rs`, recorded_floor_zero_identity_precedes_its_multiple_error at `1474b041c8` — recorded readings pinned
- CalcFunc-Evidence `runs/w111-broad-20260929/answers/answers-floor.json.gz`, w111broad-20260929-FLOOR-00002 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `runs/w111-broad-20260929/answers/answers-floor.json.gz`, w111broad-20260929-FLOOR-00005 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `runs/w111-broad-20260929/answers/answers-floor.json.gz`, w111broad-20260929-FLOOR-00062 at `4d5320dfdc` (readings)

### FN-FLOOR-002: FLOOR rounds its quotient to 15 digits first, so the result can pass the number

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.*

FLOOR behaves as s times the floor of number/s, where the quotient is first rounded to 15
significant decimal digits and the final product is taken in binary64. A quotient just under an
integer therefore counts as that integer, and the result can exceed the number:
FLOOR(0.9999999999999999,0.1) is 1. Once the quotient has more than 15 significant digits the
step moves it by more than one unit, so an exact multiple can change: FLOOR(4503599627370496,2)
is 4503599627370500.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `FLOOR(0.9999999999999999, 0.1)` | number `0.9999999999999999` (`0x3fefffffffffffff`); significance `0.1` (`0x3fb999999999999a`) (cells) | `1` (`0x3ff0000000000000`) | `0.9` (`0x3feccccccccccccd`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |
| `FLOOR(4503599627370496, 2)` | number `4503599627370496` (`0x4330000000000000`); significance `2` (`0x4000000000000000`) (cells) | `4503599627370500` (`0x4330000000000004`) | `4503599627370496` (`0x4330000000000000`) | Excel 16.0 build 20430, Windows 64-bit, CPU not recorded in this capture |

Correct value: Exact rational floor of number/significance, times significance, correctly
rounded.

Basis (measured): All 931 recorded rows with normal nonzero arguments and a number answer,
Excel build 20430, recomputed for this note from the recorded inputs; the 1,335 rows of that
capture also hold errors, zero and subnormal cases. On the 945 rows that include subnormal
quotients, the plain route matches 857 and an exact floor 848. A separate 120 literal calls in
build 20527 agree. The rule was first stated from readings of this family, so these counts are
in sample.

Inferred, not measured: That Excel's code rounds to 15 digits this way is inferred from the
fit. Whether FLOOR.MATH and FLOOR.PRECISE take the same step has not been read.

Evidence:

- CalcFunc-Evidence `runs/w111-broad-20260929/answers/answers-floor.json.gz` at `4d5320dfdc` — the 1,335 readings
- CalcFunc-Evidence `runs/w111-broad-20260929/answers/answers-floor.json.gz`, w111broad-20260929-FLOOR-00083 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `runs/w111-broad-20260929/answers/answers-floor.json.gz`, w111broad-20260929-FLOOR-00109 at `4d5320dfdc` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| significance | The multiple being rounded to, signed for this function |
| quotient | `number / significance`, on which the floor is taken |
| plain route | `s × floor(x / s)` in binary64 with no other step |
| 15-digit quotient | The quotient rounded to 15 significant decimal digits before the floor |
| exact floor | The floor of the true quotient of the two stored binary64 values |
| subnormal | A binary64 value below 2.2250738585072014E-308 in magnitude |

## Sources

- Microsoft Support, "FLOOR function" —
  <https://support.microsoft.com/en-us/office/floor-function-14bb497c-24f2-4e04-b327-b0b4de5a8886>
  (syntax, the two sign remarks, the `#VALUE!` and `#NUM!` conditions and the examples). Read
  through a fetch that returned a summary with quoted text, not the raw page.
- Microsoft Learn, "WorksheetFunction.Floor method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.floor> (the sign cases,
  the exact-multiple remark and the replacement notice).
- The recorded `FLOOR` readings the notes below cite: 1,335 inputs, Excel 16.0 build 20430, Windows
  64-bit. The counts and the table in *Numerical notes* were recomputed from them for this page.
- The catalogue capture cited by the notes below for this family: Excel 16.0 build 20527, Windows
  64-bit, of which 120 rows are `FLOOR` calls with two literal arguments. The `2^-51` and `2^-50`
  rows come from the same capture.
- Handbook, [Function notes](../notes/NOTES.md), for the zero cases.
- Handbook, [The value universe](../model/01-value-universe.md),
  [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [Claim language and honesty](../model/06-claim-language.md).
- OxFunc `crates/oxfunc_core/src/functions/ceiling_floor_family.rs` at commit `473efa3` — the
  `floor_kernel` guard order, its plain quotient and its array broadcast, read as implementation
  facts about that engine.
- Handbook projections `data/functions/FUNC.FLOOR.json` and `data/presence/FUNC.FLOOR.json` (arity,
  category, implementing module and sibling set).
