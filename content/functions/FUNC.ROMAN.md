---
schema: efh.function-page/v1
function_id: FUNC.ROMAN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-STRUCT-0007
open_problems: []
references:
  - work: "Microsoft — WorksheetFunction.Roman method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.roman"
    role: "the documented form table (0-4, TRUE, FALSE) and the two stated #VALUE! conditions"
  - work: "Microsoft — Excel JavaScript API, Excel.Functions.roman"
    locator: "https://learn.microsoft.com/en-us/javascript/api/excel/excel.functions"
    role: "confirms the return kind is a string and the form argument accepts number, string or boolean"
  - work: "Microsoft Support — ROMAN function"
    locator: "https://support.microsoft.com/en-us/office/roman-function-d6b0b99e-de46-4704-a518-b45a0f8b56f5"
    role: "the worksheet-surface documentation page named in the projection; not retrievable for this pass"
  - work: "Ifrah, The Universal History of Numbers"
    locator: "chapters on Roman numeration"
    role: "the historical numeral system against which Excel's \"concise\" forms are non-standard"
episodes: []
body_sections:
  - "What it computes"
  - "Arguments"
  - "The five forms"
  - "Result and edge cases"
  - "Errors"
  - "Relationships"
  - "Notes for implementers"
  - "What has not been checked"
  - "Page vocabulary"
  - "Sources"
family: roman_fn
role_in_family: "The Arabic-to-Roman renderer; a text-producing function filed under maths, and the only surface in the batch whose output is a string."
---

# ROMAN

## What it computes

`ROMAN(number, [form])` converts an Arabic numeral to a Roman numeral, returned as **text**.

The classical system is additive with a subtractive shorthand: the letters `I V X L C D M` have
values 1, 5, 10, 50, 100, 500, 1000, and a smaller letter placed before a larger one is subtracted.
Standard rendering decomposes the number by decimal place and emits the canonical group for each:

| Place | Groups |
|---|---|
| thousands | `""`, `M`, `MM`, `MMM` |
| hundreds | `""`, `C`, `CC`, `CCC`, `CD`, `D`, `DC`, `DCC`, `DCCC`, `CM` |
| tens | `""`, `X`, `XX`, `XXX`, `XL`, `L`, `LX`, `LXX`, `LXXX`, `XC` |
| units | `""`, `I`, `II`, `III`, `IV`, `V`, `VI`, `VII`, `VIII`, `IX` |

The domain ceiling of 3999 follows directly: the thousands row runs out at `MMM`, and the classical
system has no letter for 5000 — the overbar notation for larger values is not available in plain
text. The floor is 0, which the classical system also has no letter for.

`form` selects how aggressively the rendering may abbreviate beyond the canonical groups. That is
where the function stops being mathematics and becomes a table of conventions — see below.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | The Arabic numeral to convert. Required. | — |
| `form` | How concise the Roman numeral should be. Optional. | 0 (classic) |

Microsoft's VBA reference documents `form` as follows, verbatim:

| Form | Type |
|---|---|
| 0 or omitted | Classic. |
| 1 | More concise. |
| 2 | More concise. |
| 3 | More concise. |
| 4 | Simplified. |
| TRUE | Classic. |
| FALSE | Simplified. |

Note that `TRUE` maps to form 0 and `FALSE` maps to form 4 — a logical argument selects the two
*extremes*, not true/false in any ordinary sense. The reference engine implements exactly that
mapping, and treats any other value in the slot as a number truncated toward zero.

**What the documentation does not say is the whole difficulty.** Forms 1, 2 and 3 are all
described as "More concise", with no statement of how they differ, no examples, and no rule. A
reader cannot determine from Microsoft's documentation what `ROMAN(499, 2)` should be. The only
way to know is to observe it.

## The five forms

The reference engine implements a specific ladder of abbreviations, each form admitting everything
the previous one did plus more. The abbreviations it introduces are, in order:

- **Form 1** adds `VL` for 45–49 and `VC` for 95–99 in the tens/units, and `LD` for 450–499,
  `LM` for 950–999 in the hundreds.
- **Form 2** adds `IL` for 49 and `IC` for 99, and `XD` for 490–499, `XM` for 990–999.
- **Form 3** adds `VD` for 495–499 and `VM` for 995–999.
- **Form 4** adds `ID` for 499 and `IM` for 999.

None of these is classical. `IL` for 49 and `IM` for 999 violate the ordinary subtractive rule
(which permits subtracting only `I` from `V` and `X`, `X` from `L` and `C`, `C` from `D` and `M`);
they are Excel-specific contractions with no standing in Roman epigraphy. This page states them as
the *reference engine's* ladder, not as Excel's: the mapping above is what OxFunc implements, and
the Handbook has not compared it against Excel.

The right way to read the ladder is as a nesting: `ROMAN(n, k)` is never longer than
`ROMAN(n, k−1)`, and the forms differ only on numbers close to 50, 100, 500 and 1000 from below.
For every other input all five forms agree.

## Result and edge cases

Returns `Text` — a string, not a number. This is the only surface in this batch whose result kind
is not `Number`, and it matters downstream: the result does not participate in arithmetic without
coercion, and it sorts as text.

- **`ROMAN(0)`** returns the **empty string** in the reference engine — the natural consequence of
  a place-value decomposition in which every group is empty. Microsoft's documentation does not
  mention zero, stating only the negative and greater-than-3999 conditions. An empty string is a
  defensible reading and so is `#VALUE!`; nothing documented decides it.
- **Non-integral `number`** is truncated toward zero in the reference engine before conversion, so
  `ROMAN(2.9)` is `II`. Undocumented.
- **Non-integral `form`** is likewise truncated.
- **Missing or empty `number`** is treated as 0 in the reference engine, and therefore yields the
  empty string rather than an error — a consequence of the empty-cell-to-zero mapping described in
  [Coercion and lifting](../model/02-coercion-and-lifting.md), colliding with the zero case above.
- **Arrays.** The projection declares a `Custom` coercion profile with `surface_native` lifting,
  and `EV-STRUCT-0007` is a structural array-admission record naming this surface.
- **Text length.** The longest result is `MMMDCCCLXXXVIII` for 3888 in form 0 — fifteen
  characters, comfortably inside every text limit.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | `number` is negative | Microsoft VBA page, verbatim: "If number is negative, the #VALUE! error value is returned" |
| `#VALUE!` | `number` is greater than 3999 | Microsoft VBA page, verbatim |
| `#VALUE!` | `form` is outside 0–4 after truncation | reference engine; **not documented** |
| `#VALUE!` | An argument does not convert | shared coercion model |
| propagated | An error value in either argument | shared coercion model |

## Relationships

- **`ARABIC`** — the inverse direction, parsing a Roman numeral back to a number. The pair is not
  a clean bijection: `ARABIC` must accept the concise forms that `ROMAN` can emit, and it accepts
  many strings `ROMAN` would never produce. Round-tripping `ROMAN` then `ARABIC` is the natural
  consistency test and is on the probe list below.
- **`TEXT`** — the general number-to-string formatter, which has no Roman format code. `ROMAN` is
  the only route.
- **`BASE` and `DECIMAL`** — the other pair of numeral-system converters, for radix 2–36. They are
  the structural analogue of `ROMAN`/`ARABIC` for positional systems, and the contrast is
  instructive: `BASE` is total on its domain and mechanical, while `ROMAN` needs a five-way style
  argument because the target notation is not positional and admits genuine ambiguity.
- **Confused with `ROUND`** by autocomplete alone. They share four letters and nothing else.

## Notes for implementers

**The place-value decomposition is the whole algorithm for form 0**, and it should be written as
four table lookups rather than a greedy subtraction loop. The greedy loop happens to produce the
same answer for form 0 and cannot express forms 1–4 at all, because those introduce contractions
that cross place boundaries (`VL` spans tens and units; `LD` spans hundreds and tens).

**Forms are ranges, not points.** The concise contractions apply to *bands* — 45–49, 95–99,
450–499, 490–499, 495–499, 499 — each of which is rendered as a prefix plus the standard rendering
of the remainder. Getting a band boundary wrong produces a wrong answer on exactly one or two
inputs out of four thousand, which no casual test will find. The full domain is only 4000 × 5
inputs; an implementation should simply enumerate all 20,000 and compare.

**The thousands prefix is form-independent.** All five forms render the thousands digit as
`""`/`M`/`MM`/`MMM` and apply their contractions only to the remainder below 1000. That is what
makes `IM` for 999 possible while `MIM` for 1999 comes out as `M` + `IM`.

**Preserve the value kind.** The result is text. An implementation that returns a number-shaped
value, or that lets the empty-string case become an empty cell, changes downstream behaviour — see
the raw-versus-published boundary in [The value universe](../model/01-value-universe.md).

## What has not been checked

No Handbook vector suite exists for `ROMAN`. One evidence record, `EV-STRUCT-0007`, lists this
surface among its subjects: a live-Excel structural resweep of array lift and coercion. Its reader
warning states plainly that the group figure is shared across roughly twenty surfaces, that each
contributed only one or two cases, and that the record **establishes nothing about any surface's
results** — it is about argument shape, coercion and error placement. The record's own open
question notes that its membership list is not exactly recoverable from the source.

So: argument shape has upstream evidence; the rendering tables have none. In particular **nobody
has checked the concise forms against Excel within the Handbook's record**, and Microsoft's
documentation does not define them, which makes this one of the few surfaces where the *documented*
specification is genuinely insufficient to implement the function.

Inputs I would probe first:

1. **`ROMAN(n, f)` for `n` in {45, 49, 95, 99, 450, 490, 495, 499, 950, 990, 995, 999} across all
   five forms** — sixty cells that pin every contraction band boundary at once. This is the single
   most valuable experiment on this page, because it is the part no documentation covers.
2. **`ROMAN(0)`** — empty string, error, or something else.
3. **`ROMAN(3.9)` and `ROMAN(-0.5)`** — truncation versus rounding, and whether a value that
   truncates to 0 from below counts as negative.
4. **`ROMAN(1, TRUE)` against `ROMAN(1, 0)`, and `ROMAN(499, FALSE)` against `ROMAN(499, 4)`** —
   confirming the documented logical mapping, which is counter-intuitive enough to be worth
   checking rather than assuming.
5. **`ROMAN(1, 5)` and `ROMAN(1, -1)`** — the out-of-range form, whose error is undocumented.
6. **`ROMAN(4000)` and `ROMAN(3999)`** — the documented ceiling, one cell either side.
7. **`ARABIC(ROMAN(n, f))` swept over the whole domain and all five forms** — the round-trip
   consistency test. Any input where it fails is either a `ROMAN` contraction `ARABIC` cannot
   parse, or an `ARABIC` bug, and either is a finding.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| subtractive shorthand | Writing a smaller letter before a larger one to mean the difference |
| place-value decomposition | Rendering thousands, hundreds, tens and units independently |
| contraction band | A range of numbers a concise form renders with a non-classical prefix |
| form ladder | The nesting in which each higher form admits every abbreviation of the lower ones |

## Sources

- Microsoft, "WorksheetFunction.Roman method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.roman> (the form table
  quoted above, and the two `#VALUE!` conditions; the page does not distinguish forms 1, 2 and 3
  from one another and does not mention zero or non-integral input).
- Microsoft, Excel JavaScript API reference —
  <https://learn.microsoft.com/en-us/javascript/api/excel/excel.functions> (`roman` returns a
  string; `form` accepts number, string or boolean).
- Microsoft Support, "ROMAN function" —
  <https://support.microsoft.com/en-us/office/roman-function-d6b0b99e-de46-4704-a518-b45a0f8b56f5>
  (retrieval refused with HTTP 403 during this pass; nothing here is sourced from it).
- Handbook evidence record `EV-STRUCT-0007`, rendered beside this page with its own reader
  warning.
- Handbook, [The value universe](../model/01-value-universe.md),
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.ROMAN.json` (arity 1–2, `Custom` coercion,
  `Deterministic`) and `data/presence/FUNC.ROMAN.json` (own module; defect stream
  `BUG-FUNC-028`).
