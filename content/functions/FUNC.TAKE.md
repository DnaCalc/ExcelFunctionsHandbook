---
schema: efh.function-page/v1
function_id: FUNC.TAKE
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
family: dynamic_array_reshape_family
role_in_family: >-
  Slices a contiguous block from the start or the end of an array along either axis; the
  complement of DROP.
---

## What it computes

`TAKE` returns a contiguous corner block of an array.

Given an `m × n` array `A`, a row count `r` and a column count `c`, the result is the
sub-array formed by:

- the **first `r` rows** if `r > 0`, or the **last `|r|` rows** if `r < 0`;
- the **first `c` columns** if `c > 0`, or the **last `|c|` columns** if `c < 0`.

The sign is the direction, and the magnitude is the count. That is the whole rule, and it is
what makes `TAKE(A, -1)` — the last row — one of the most useful short formulas in modern
Excel.

Counts larger than the array are the interesting case: `TAKE(A, 100)` over a five-row array
returns five rows. `TAKE` clamps rather than padding or erroring, which is what makes it safe
to use with a computed count. (`EXPAND` is the function that pads.)

## Arguments

Microsoft documents `TAKE(array, rows, [columns])`.

| Argument | Required | Meaning |
|---|---|---|
| `array` | yes | The array to take from |
| `rows` | yes (but omissible as an empty slot — see below) | Number of rows; negative takes from the end |
| `columns` | no | Number of columns; negative takes from the end |

The subtlety readers most often miss is the **empty slot** in the second position.
`TAKE(A, , 2)` — with the row count written as nothing between two commas — means "all rows,
first two columns". The signature marks `rows` as required, and it is required in the sense that
you cannot write `TAKE(A)`; but the omitted-slot spelling is legal and meaningful, and it is the
idiomatic way to slice columns only.

Live Excel replay on 2026-04-10, recorded in OxFunc's `BUG-FUNC-012`, pins the four cases
directly:

| Formula | Excel result |
|---|---|
| `=TAKE({1,2;3,4},,1)` | `{1;3}` |
| `=TAKE({1,2;3,4},,-1)` | `{2;4}` |
| `=DROP({1,2;3,4},,1)` | `{2;4}` |
| `=DROP({1,2;3,4},,-1)` | `{1;3}` |

That record exists because an implementation treated the empty slot as a missing required
argument and returned an error. It is the clearest available illustration of why
[the value universe](../model/01-value-universe.md)'s `Missing`-versus-`Empty` distinction is
not academic.

## Result and edge cases

The return kind is an array — always, including when it has one element.

- **A count of zero** produces an empty array. Microsoft documents that Excel returns `#CALC!`
  to indicate an empty array when `rows` or `columns` is `0`.
- **Counts exceeding the array** clamp to the array's extent rather than padding.
- **1×1 results stay arrays.** This is the case OxFunc's `BUG-FUNC-026` characterizes with
  direct Excel probes: `=TAKE({1,2;3,4},1,1)` publishes the value `1` in its anchor cell, but
  `=TYPE(TAKE({1,2;3,4},1,1))` returns `64` (array), `=ROWS(...)` and `=COLUMNS(...)` both
  return `1`, and `=HSTACK(TAKE({1,2;3,4},1,1),9)` spills `{1,9}`. The function's result is a
  1×1 array; the *cell* publishes the single value inside it. Those are two different layers,
  and conflating them is a recorded defect class.
- **Non-integer counts** are expected to truncate; the Handbook has not verified the direction
  or the resulting error for a non-numeric count.
- Errors and blanks inside the array are carried through as values.

## Errors

Documented by Microsoft:

| Error | Documented condition |
|---|---|
| `#CALC!` | An empty array, when `rows` or `columns` is `0` |
| `#NUM!` | The array exceeds size limits |

`#SPILL!` arises at publication when the result cannot be placed — a host outcome rather than a
`TAKE` outcome. Error values arriving in `rows` or `columns` propagate under the ordinary
coercion rules.

## Relationships

- **`DROP`** is the exact complement: `DROP(A, k)` removes what `TAKE(A, k)` keeps, and the two
  share the same sign convention and the same empty-slot behaviour. `TAKE(A, 3)` and
  `DROP(A, 3)` partition `A`'s rows.
- **`CHOOSEROWS` / `CHOOSECOLS`** select arbitrary, non-contiguous indices; `TAKE` is the
  contiguous-from-an-edge special case.
- **`EXPAND`** is the growing counterpart: `TAKE` clamps down, `EXPAND` pads up.
- **`INDEX`** with omitted arguments can extract whole rows or columns and predates all of
  these; `TAKE` is the readable modern form.
- **`TRIMRANGE`** answers a different question — remove the blank margin, whatever size it is —
  where `TAKE` needs the count in advance.
- Readers confuse `TAKE(A, -1)` (the last row) with `TAKE(A, 1)` (the first). The sign carries
  a lot of meaning in a very small symbol.

## Notes for implementers

- Sign is direction; magnitude is count. Implementing negative counts as "count from the end,
  exclusive" is off by one and produces plausible output.
- Clamping is the documented behaviour for oversized counts — not an error, not padding.
- The empty-slot form in the *leading* required position must normalize to "all rows". This is
  a recorded defect class, verified against live Excel on 2026-04-10.
- A 1×1 result must remain an array. The reference engine's `BUG-FUNC-026` records an attempted
  function-level scalarization being *undone* after Excel probes showed the nested result is
  still array-typed; worksheet scalar publication belongs above the function, in the
  result-completion seam.
- The zero-count case has a specific outcome (`#CALC!`) and needs an explicit branch; producing
  a genuinely empty array and letting it leak downstream is not the same thing.

## What has not been checked

No Handbook vector suite exists for `TAKE`, and no Handbook evidence record is attached to this
page. The Excel observations quoted above come from named upstream OxFunc bug streams and are
cited so they can be re-run; they are not Handbook measurements, and nothing here says any
implementation agrees with Excel.

First probes:

1. **The sign × magnitude grid** for both axes, including counts equal to, one less than, and
   greater than the array's extent — this covers the clamping rule and the off-by-one that
   negative counts invite.
2. **Zero counts** in each position and both together, against the documented `#CALC!`.
3. **The empty-slot forms** `TAKE(A,,c)` and `TAKE(A,r)` with `columns` absent, against their
   explicit equivalents.
4. **Shape observation of 1×1 results** through `TYPE`, `ROWS`, `COLUMNS` and `HSTACK`, since
   the anchor cell's display hides the distinction.
5. **Non-integer, text-numeric and array-valued counts.**
6. **Size limits**, against the documented `#NUM!`.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| sign-as-direction | Positive counts take from the start, negative from the end |
| clamping | Counts larger than the array return the whole array rather than padding or erroring |
| empty slot | An omitted argument written between commas; means "all" in `TAKE`'s count positions |
| shape publication | The worksheet's collapse of a 1×1 array result to a displayed scalar, above the function |

## Sources

- Microsoft, *TAKE function* —
  <https://support.microsoft.com/en-us/office/take-function-25382ff1-5da1-4f78-ab43-f33bd2e4e003>
  (syntax, negative counts taking from the end, the `#CALC!` empty-array condition for a zero
  count, the `#NUM!` size condition, and the `=TAKE(A2:C4,,2)` column-slice example).
  Retrieved for this page.
- Handbook `content/model/01-value-universe.md` (`Missing` versus `Empty`, the `#CALC!`
  convention) and `content/model/03-call-pipeline.md`.
- OxFunc bug stream
  `docs/bugs/streams/BUG-FUNC-012_take_drop_omitted_leading_count_parity_gap.md` — live Excel
  replay of the omitted leading count, 2026-04-10.
- OxFunc bug stream `docs/bugs/streams/BUG-FUNC-026_take_1x1_scalar_publication_mismatch.md` —
  the `TYPE`/`ROWS`/`COLUMNS`/`HSTACK` probes separating function shape from cell publication.
- Handbook `data/functions/FUNC.TAKE.json` (signature, arity, classification axes).
