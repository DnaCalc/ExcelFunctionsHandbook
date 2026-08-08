---
schema: efh.function-page/v1
function_id: FUNC.FLOOR.MATH
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — FLOOR.MATH function"
    locator: "https://support.microsoft.com/en-us/office/floor-math-function-c302b599-fbdb-4177-ba19-2c2b1249a2f5"
    role: "documented signature, the default positive and negative behaviour, the mode paragraph, and the significance rule"
  - work: "IEEE 754-2019, Standard for Floating-Point Arithmetic"
    locator: "the roundToIntegral operations and the exactness of division by a power of two"
    role: "which steps of the quotient-floor-scale op-graph are exact and which are not"
  - work: "Muller et al., Handbook of Floating-Point Arithmetic"
    locator: "the chapters on exact operations and on the accuracy of a*b and a/b"
    role: "why s*floor(x/s) is not the mathematical floor-to-a-multiple"
  - work: "Kahan, lecture notes on floating-point arithmetic and rounding"
    locator: null
    role: "the general argument against composing roundings and expecting an exact grid"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: ceiling_floor_family
role_in_family: >-
  The most general downward rounding: to a multiple of an arbitrary significance, with an
  explicit mode argument selecting the direction taken for negative numbers.
---

# FLOOR.MATH

## What it computes

`FLOOR.MATH(number, [significance], [mode])` rounds *number* **down** to a multiple of
*significance*, with *mode* selecting what "down" means for negative numbers.

The mathematics is one line. For a nonzero significance `s`, the mathematical floor to a
multiple is

    floor_to_multiple(x, s) = ABS(s) · floor( x / ABS(s) )

which is the largest multiple of `ABS(s)` not exceeding `x`. That is the toward-`-∞` reading,
and it is what Microsoft documents as the default: a positive number with a fractional part
loses it, and a negative number moves further from zero.

The mode argument selects the other reading for negative arguments — the toward-zero
truncation:

    trunc_to_multiple(x, s) = sign(x) · ABS(s) · floor( ABS(x) / ABS(s) )

The two agree on every non-negative argument and on every exact multiple. They differ only on
negative arguments that are not multiples, where one gives the mathematical floor and the other
gives truncation toward zero.

| Property | Statement (for the default toward-`-∞` reading) |
|---|---|
| Domain | all real `x`, all real `s` |
| Range | multiples of `ABS(s)` |
| Idempotent | applying it twice changes nothing |
| Monotone | non-decreasing in `x` |
| Bound | `result ≤ x`, with equality exactly at the multiples |
| Sign of `s` | ignored — the absolute value of the significance is used |
| Fixed points | the multiples of `ABS(s)` |
| Relation to `INT` | `FLOOR.MATH(x)` with default significance is `INT(x)` in the toward-`-∞` reading |

## Arguments

| Argument | Meaning | Default (as documented) |
|---|---|---|
| `number` | The value to round down. Required. | — |
| `significance` | The multiple to round to. Optional. | 1 |
| `mode` | The direction, toward or away from zero, used for negative numbers. Optional. | see the note below |

The reference engine records an arity of one to three arguments, matching the documented
signature. It also carries `signature_placeholder: true` for this surface, meaning the
Handbook's own projection holds **no real signature string** for it — the argument names above
come from Microsoft's page, not from the projection, and the projection's absence is shown
rather than faked.

**A documentation inconsistency, recorded as observed at curation.** Microsoft's page states
the default for negative numbers as rounding *away* from zero, and then, in the paragraph on
`mode`, describes using zero or a negative number as the `mode` argument to *change* the
direction for negative numbers, with a worked example in which a `mode` of `-1` rounds toward
zero. Those two statements do not sit together: if `mode` of `0` changes the direction, then
`mode` omitted and `mode` of `0` are not the same thing, which is unusual for an optional
argument, and the page does not say what `mode` omitted corresponds to. The usual reading in
the wild is that a nonzero `mode` selects toward-zero and zero-or-omitted selects
away-from-zero, which is the reading that makes the default paragraph and the mode paragraph
consistent — but that reading is **not** what the page says, and the Handbook does not assert
Excel's behaviour from anywhere but the page. This is on the probe list, and the page wording
should be re-read against the live document when it is next fetched.

Both numeric slots take ordinary to-number coercion. The reference engine classifies the
coercion-and-lift profile as `Custom`, and the presence projection attaches an open
array-lift defect stream to the implementing module, so array-shaped arguments are unsettled.
See [Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`.

- **`significance` omitted** — documented default 1, so the function is a plain floor.
- **`significance` negative** — the absolute value is used; the sign of the significance does
  not flip the direction. This is the design difference from the legacy `FLOOR`, which errors
  when the two signs disagree.
- **`significance` zero** — not documented on this page. The mathematical expression divides by
  it. `FLOOR.PRECISE`'s page documents that a zero in either position gives zero; whether
  `FLOOR.MATH` shares that convention is unchecked.
- **`number` zero** — zero is a multiple of everything, so the answer is zero.
- **Exact multiples** — unchanged, in both modes.
- **Very large `number`** — above `2^53` the spacing of doubles exceeds 1, so with the default
  significance the function is the identity; with a small significance it still is, because no
  multiple of a small `s` lies strictly between two adjacent doubles up there.
- **Non-integer `significance`** — the interesting case, and the one where the answer depends on
  arithmetic rather than on the rule. See the numerical notes.

## Errors

Microsoft's `FLOOR.MATH` page documents no error conditions. That is itself notable: the legacy
`FLOOR` documents `#NUM!` for mismatched signs and `#DIV/0!` for a zero significance, and
`FLOOR.MATH` was introduced precisely to remove the first of those restrictions. Whether it
also removed the second — that is, what a zero significance does — the page does not say.

Non-numeric arguments surface `#VALUE!` under the shared call model, and error values
propagate. The mechanically rendered battery beside this page shows the boundary probes.

## Relationships

- **[FLOOR.PRECISE](FUNC.FLOOR.PRECISE.md)** — the same function without the `mode` argument,
  documented as always returning the mathematical floor irrespective of both signs. It should
  therefore equal `FLOOR.MATH` with `mode` omitted, on every input. Whether it does bitwise —
  including at a zero significance, where only `FLOOR.PRECISE`'s page documents an answer — is
  the natural cross-surface probe.
- **`FLOOR`** — the legacy member. It rounds toward zero for negative numbers with negative
  significance and errors when the signs of the two arguments differ. Replacing `FLOOR` with
  `FLOOR.MATH` in an existing workbook changes results on negative arguments and removes an
  error; that is a migration hazard, not a bug fix.
- **`CEILING.MATH`, `CEILING.PRECISE`, `CEILING`, `ISO.CEILING`** — the upward mirror of the
  same family, sharing the implementing module. `ISO.CEILING` and `CEILING.PRECISE` are the
  toward-`+∞` pair.
- **`INT`** — `INT(x)` is the floor to integers, toward `-∞`, and coincides with
  `FLOOR.MATH(x)` at the default significance in the default mode. `TRUNC(x)` is the
  toward-zero reading and coincides with the other mode.
- **`MROUND`** — nearest multiple rather than lower multiple; it disagrees with this function
  on the upper half of every step.
- **`ROUNDDOWN`** — toward zero to a *digit position* rather than to a multiple; the same
  direction convention as the non-default mode, on a decimal grid.
- **[EVEN](FUNC.EVEN.md)** — a fixed-step rounding away from zero, for contrast: the worksheet's
  rounding functions do not share one direction convention, and this family is where the
  convention is made an explicit argument.

## Numerical notes

The rule is exact arithmetic; the implementation is not, and the gap between them is this
page's real content.

**The naive op-graph fails, and it fails visibly.** Computing `s · floor(x / s)` in binary64
performs two roundings around an exact operation:

1. `x / s` rounds. If the true quotient is an integer but the rounded quotient falls a hair
   below it, `floor` drops an entire step.
2. `floor` is exact — an IEEE 754 `roundToIntegral` operation.
3. `s · q` rounds again, so the result need not be an exact multiple of `s` even when the
   mathematics says it is.

Step 1 is the damaging one, and it is not a rare edge case. Working the naive op-graph through
in binary64 (this is arithmetic, not a claim about Excel):

| Input | True quotient | Computed quotient | Naive result | Mathematical answer |
|---|---|---|---|---|
| `x = 0.29, s = 0.01` | 29 | just below 29 | `0.28` | `0.29` |
| `x = 6.6, s = 1.1` | 6 | just below 6 | `5.5` | `6.6` |
| `x = 4.35, s = 0.05` | 87 | just below 87 | `4.3` | `4.35` |

In each row the naive route returns a value **one whole step too low**, because the decimal
significance is not a binary64 value and the quotient of two rounded decimals lands just under
the integer it should be. A user who thinks of the significance as "cents" or "tenths" sees the
function skip a tread. This is the classic complaint about every floor-to-a-multiple function in
every spreadsheet, and it is arithmetic rather than a defect in the rule.

**What a careful implementation does about it.** There is no single accepted answer, and the
choice is a decision to record rather than a neutral one:

- **Exactness-preserving route**: use a fused multiply-add to compute the residual
  `x - s·q` exactly and correct `q` by one when the residual has the wrong sign. This makes the
  result the true floor of the *represented* values `x` and `s`, which is the defensible
  mathematical position, and it still returns `0.28` for the first row above — because
  `0.29/0.01` genuinely is less than 29 in the values the worksheet holds.
- **Intent-preserving route**: apply a relative tolerance, treating a quotient within a few ulp
  of an integer as that integer. This returns `0.29` and matches what users expect, at the cost
  of no longer being a mathematical function of its inputs — there are then arguments where
  `FLOOR.MATH(x, s) > x`.

These two routes disagree on a set of inputs that is easy to enumerate, and **the Handbook has
not determined which one Excel takes.** That is the most valuable open question on this page,
because the answer changes results in the second decimal place of ordinary financial formulas,
not in the last bit.

**Where nothing goes wrong**: when `s` is a power of two, `x/s` and `s·q` are both exact, and
the naive op-graph is the correct answer. Rounding to halves, quarters or eighths is safe;
rounding to tenths is not.

**A note on `mode` and negative zero.** In the toward-zero reading, a negative argument in
`(-s, 0)` maps to zero. Whether that zero carries a sign is not documented and is visible
through `1/FLOOR.MATH(-0.5, 1, 1)`.

## What has not been checked

**No evidence record in the Handbook lists this surface as a subject.** `FLOOR.MATH` appears
inside a group listing in the structural array-lift record `EV-STRUCT-0011`, which is a group
total across many surfaces with no per-surface split and whose reader warning forbids
attributing any of its figures to an individual surface. The honest statement is therefore:
**the family was measured in a group; this surface was not measured separately**, and nothing
in that record may be read as evidence about `FLOOR.MATH`.

No Handbook vector suite exists for `FLOOR.MATH`. The default behaviours, the significance rule
and the mode paragraph are Microsoft's; the internal inconsistency in that page's mode
description is recorded above as observed at curation; the zero-significance behaviour is
documented nowhere; and the tolerance question is unresolved.

Inputs I would probe first:

1. **The `mode` truth table**: `FLOOR.MATH(-6.3, 1, m)` for `m` omitted, `0`, `1`, `-1`, `2`,
   `TRUE`, `""`. The documentation's mode paragraph and its default paragraph cannot both be
   right, and seven cheap probes produce the actual mapping. This is the largest documented
   ambiguity in this batch.
2. **The tolerance question**: `FLOOR.MATH(0.29, 0.01)`, `FLOOR.MATH(6.6, 1.1)`,
   `FLOOR.MATH(4.35, 0.05)`. Each has a naive answer and an intent answer that differ by a
   whole step, so the results are unmistakable and need no bit comparison. Three probes decide
   which route Excel takes.
3. **Zero significance**: `FLOOR.MATH(5, 0)` and `FLOOR.MATH(-5, 0)`, against
   `FLOOR.PRECISE(5, 0)` — where zero *is* documented to give zero. If the two surfaces
   disagree, that is a finding about a documented-versus-undocumented pair.
4. **Negative significance**: `FLOOR.MATH(-6.3, -1)` and `FLOOR.MATH(6.3, -1)`, testing the
   documented absolute-value rule, and the same inputs on legacy `FLOOR`, which is documented to
   error on mismatched signs.
5. **The `FLOOR.PRECISE` equivalence**: sweep and assert
   `FLOOR.MATH(x, s) = FLOOR.PRECISE(x, s)` with `mode` omitted. Any disagreement means the two
   surfaces do not share a kernel despite sharing a module.
6. **Power-of-two significances** (`0.5`, `0.25`, `2^-20`) against decimal ones, which separates
   the arithmetic hazard from the rule.
7. **Above `2^53`**, where the function must be the identity, and **subnormal significances**,
   where `x/s` can overflow.
8. **Array arguments in each position**, given the open array-lift defect stream on this module.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| significance | The step size; the result is a multiple of its absolute value |
| mode | The argument selecting toward-zero or away-from-zero rounding for negatives |
| toward `-∞` | The mathematical floor: the largest multiple not exceeding the argument |
| toward zero | Truncation: the multiple nearest zero on the same side as the argument |
| naive op-graph | `s · floor(x/s)` evaluated directly, with two roundings around one exact step |
| tolerance route | Snapping a near-integer quotient to that integer, matching user intent |
| tread | One step of the staircase: an interval mapped to a single multiple |

## Sources

- Microsoft, "FLOOR.MATH function" —
  <https://support.microsoft.com/en-us/office/floor-math-function-c302b599-fbdb-4177-ba19-2c2b1249a2f5>
  (fetched at curation: signature with two optional arguments, the default significance of 1,
  the default away-from-zero behaviour for negatives, the mode paragraph quoted in substance
  above, and the significance/remainder rules. No error conditions are documented there).
- Handbook evidence record `EV-STRUCT-0011` — named here only to state that this surface appears
  inside its group listing and **not** as a subject; its figures may not be attributed here.
- IEEE 754-2019 — `roundToIntegral` and the exactness of power-of-two scaling.
- Muller et al., *Handbook of Floating-Point Arithmetic*; Kahan's lecture notes — composing
  roundings and the failure of naive grid arithmetic.
- Handbook, [FLOOR.PRECISE](FUNC.FLOOR.PRECISE.md) and [EVEN](FUNC.EVEN.md);
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.FLOOR.MATH.json` (arity 1–3, `signature_placeholder`
  true, `Custom` coercion and kernel classifications) and `data/presence/FUNC.FLOOR.MATH.json`
  (the shared `ceiling_floor_family` module and the open array-lift defect stream).
