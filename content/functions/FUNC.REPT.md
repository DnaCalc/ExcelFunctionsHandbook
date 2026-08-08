---
schema: efh.function-page/v1
function_id: FUNC.REPT
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
family: text_scalar_misc
role_in_family: The only length-multiplying member; the family's one route to the text cap.
---

# REPT

## What it computes

`REPT(text, number_times)` returns `text` concatenated with itself `number_times` times:

```
REPT(t, n)  =  t · t · … · t     (n copies)
REPT(t, 0)  =  ""
```

`number_times` is truncated to an integer before use — Microsoft states this explicitly — so
`REPT("ab", 2.9)` is `REPT("ab", 2)`.

The interesting property of this function is not the concatenation. It is that `REPT` is the
cheapest way in the whole worksheet language to manufacture a string longer than Excel will
hold, which makes it the function against which the text cap is most often observed. The
Handbook's value-universe chapter records that observation: producing an over-cap string in a
formula yields `#VALUE!`, and `REPT` is the function it was observed with.

The result length is `LEN(text) × TRUNC(number_times)` code units, and the documented ceiling
is 32,767.

## Arguments

| Argument | Meaning |
|---|---|
| `text` | The string to repeat. Required. |
| `number_times` | How many copies. Required; documented as a positive number, truncated to an integer. |

Both arguments are required. The Handbook's projected signature for this entry is a
placeholder — the metadata layer carries the arity but not a rendered parameter list — so the
argument names above come from Microsoft's documentation rather than the reference engine's
registry.

`number_times` is a numeric slot and takes ordinary to-number coercion; see
[Coercion and lifting](../model/02-coercion-and-lifting.md). `text` may be any value that
converts to text, so `REPT(0, 3)` is a to-text question before it is a `REPT` question.

## Result and edge cases

Returns `Text`.

Documented:

- `number_times` of 0 returns the empty string `""`.
- Non-integer `number_times` is truncated.
- A result longer than 32,767 characters returns `#VALUE!`.

Not documented, and therefore not stated as fact here: what a **negative** `number_times`
does. Microsoft's page describes the argument as "a positive number" and states the zero case,
but does not state the negative case. `#VALUE!` is the obvious expectation and the obvious
thing to probe.

Also undocumented: what `REPT("", n)` does for large `n` — the result is empty regardless, but
whether the cap check runs before or after the empty-string shortcut decides whether
`REPT("", 10^9)` is instant or is an error.

Empty, missing and error arguments follow the shared call model. The implementing module named
in the presence projection carries an open upstream defect stream on scalar and
delimiter-array support (`BUG-FUNC-008`), so array-shaped arguments are a known soft spot.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | The result would exceed 32,767 characters. | Documented on Microsoft's `REPT` page. |
| `#VALUE!` | Non-numeric text in `number_times`, or a value that cannot convert to text in `text`. | Shared coercion rules, not `REPT`-specific. |

Error values in either argument propagate. Whether a negative `number_times` produces
`#VALUE!` or `#NUM!` is unverified — see above.

The over-cap rule deserves emphasis because the same cap behaves differently on a different
path. The Handbook's value-universe chapter records that assigning an over-cap string through
the automation interface **truncates silently**, while producing one in a formula errors. Same
limit, two enforcement behaviours, decided by how the string arrives. `REPT` is on the erroring
side.

## Relationships

- **`CONCAT` / `TEXTJOIN` / the `&` operator** are the general concatenation route; `REPT` is
  the special case where all the pieces are the same and the count is computed.
- **[TEXTJOIN](FUNC.TEXTJOIN.md)** shares the 32,767-character result ceiling and the `#VALUE!`
  on exceeding it, and is the other text function most likely to hit it.
- **`LEN`** is the natural companion, both for predicting the cap and for the classic in-cell
  bar-chart idiom `REPT("|", value)`.
- There is no compatibility-replacement pair here: `REPT` is neither superseded nor a retained
  legacy name.
- Readers occasionally confuse `REPT` with `SUBSTITUTE` when trying to pad strings. Padding is
  `REPT` plus `LEN`; `SUBSTITUTE` has nothing to do with it.

## Notes for implementers

The cap check must precede the allocation, not follow it. `REPT("x", 10^9)` is a documented
`#VALUE!`, and an implementation that discovers this by building the string first has a
denial-of-service surface where Excel has an error value. Compute `LEN(text) ×
TRUNC(number_times)`, compare against 32,767, and only then allocate.

That multiplication itself needs care: `number_times` arrives as a double and can be
astronomically large, so the length computation has to be done in a form that cannot overflow
before the comparison happens.

Truncation is toward zero, which is what makes the negative case a real question rather than a
formality — `TRUNC(-0.5)` is 0, so a naive truncate-then-loop implementation would return the
empty string for `REPT("a", -0.5)` and something else for `REPT("a", -1)`. Whatever Excel does,
an implementation should not arrive at two different answers for those two inputs by accident.

Counting is in UTF-16 code units against the cap, consistent with `LEN`.

## What has not been checked

No Handbook vector suite exists for `REPT`, and no Excel-comparison evidence record names it.
Nobody has checked this function against Excel within the Handbook's record. The one empirical
statement on this page — that the over-cap formula path publishes `#VALUE!` — comes from the
Handbook's value-universe chapter, which records it as an observed probe result rather than a
documented one; it is not a suite result and it is not scoped to a named Excel build here.

Inputs worth probing first:

1. **`REPT("a", -1)` and `REPT("a", -0.5)`** — the undocumented negative case, and the pair
   that separates "negative is an error" from "negative truncates to zero".
2. **The cap boundary exactly**: `REPT("a", 32767)` and `REPT("a", 32768)`, then
   `REPT("ab", 16383)` and `REPT("ab", 16384)`. Two string lengths pin whether the check is on
   the product or on the count.
3. **`REPT("", 10^9)`** — whether the empty-string case short-circuits the cap check.
4. **An astral character**: `REPT("😀", 16384)`, which is 32,768 code units from 16,384
   characters. This decides whether the cap counts code units, as the value-universe chapter
   says text is measured, or characters, as Microsoft's page says.
5. **Fractional and very large `number_times`**, including values beyond the range of a 64-bit
   integer, to confirm the truncation rule does not stop applying.
6. **Array `text` or `number_times`**, given the open `BUG-FUNC-008` stream on this module.

Probe 4 is the one that would teach the Handbook the most, because it tests the documentation
against the model rather than merely filling a gap.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| text cap | The 32,767-unit ceiling on worksheet text |
| formula path / interop path | The two routes by which an over-cap string can arise; they enforce the cap differently |
| code unit | One UTF-16 unit; the unit `LEN` counts and the cap is measured in |

## Sources

- Microsoft, "REPT function" —
  <https://support.microsoft.com/en-us/office/rept-function-04c4d778-e712-43b4-9c15-d656582bb061>
  (signature, the zero case, the truncation rule, and the 32,767-character `#VALUE!`).
- Handbook, [The value universe](../model/01-value-universe.md) — the text cap in UTF-16 code
  units, the observed `#VALUE!` on the over-cap formula path (observed with `REPT`), and the
  contrasting silent truncation on the interop path.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — to-number and to-text
  coercion of the two argument slots.
- Handbook projections `data/functions/FUNC.REPT.json` (placeholder signature) and
  `data/presence/FUNC.REPT.json` (implementing module; `BUG-FUNC-008`).
