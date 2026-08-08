---
schema: efh.function-page/v1
function_id: FUNC.ISO.CEILING
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
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: ceiling_floor_family
role_in_family: >-
  The sign-blind ceiling: it takes the absolute value of significance and so returns the
  mathematical ceiling for every sign combination, unlike the legacy CEILING it was
  introduced to correct.
---

## What it computes

`ISO.CEILING(number, [significance])` rounds *number* **upward** — toward positive infinity — to
the nearest multiple of *significance*.

    ISO.CEILING(x, s) = ⌈ x / |s| ⌉ · |s|

with *s* defaulting to 1, in which case the function is the plain mathematical ceiling ⌈x⌉.

The defining property, and the reason the function exists, is stated on Microsoft's page:

> "The absolute value of the multiple is used, so that the ISO.CEILING function returns the
> mathematical ceiling irrespective of the signs of number and significance."

So the direction of rounding is **always toward +∞**, for every combination of signs. The
documented examples make the shape explicit: 4.3 rises to 5, −4.3 rises to −4, and 4.3 with a
significance of either 2 or −2 gives 6, while −4.3 with either sign of 2 gives −4. Negating the
significance changes nothing.

Ceiling is the reflection of floor: ⌈x⌉ = −⌊−x⌋, so everything true of
[INT](FUNC.INT.md) has a mirror image here. Domain: all reals for *number*; any nonzero real for
*significance*. Range: the multiples of |*s*|. The function is non-decreasing in *number* and
discontinuous at every multiple of |*s*|, with a jump of exactly |*s*| — this time from the
*right*, since ceiling is right-continuous where floor is left-continuous.

The name records its origin: this is the rounding rule of ISO/IEC 29500 (the OOXML spreadsheet
formula specification), added so that a spreadsheet could express the mathematical ceiling
without inheriting the sign-dependent behaviour of the older `CEILING`.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | "The value to be rounded." Required. | — |
| `significance` | "The optional multiple to which number is to be rounded." | 1 |

The declared arity is one to two, which agrees with the documented optional second argument.

**A projection gap worth naming.** The Handbook's mechanical projection of this entry carries no
signature at all — the signature field is marked as a placeholder rather than filled — so the
generator has nothing to render there. The signature above is taken from Microsoft's page, not
from the projection. This is a gap in the Handbook's own data, published rather than papered
over.

Both slots are numeric and subject to ordinary to-number coercion
([coercion and lifting](../model/02-coercion-and-lifting.md)).

## Result and edge cases

Returns `Number`.

The reference engine's published battery is rendered beside this page. Qualitatively, it shows:

- **Zero** returns zero, and **−1** returns −1: an argument already on a multiple is left alone.
  Ceiling is idempotent, ⌈⌈x⌉⌉ = ⌈x⌉, and that is the visible consequence.
- **A logical argument** and **numeric text** both convert and round. Microsoft's page addresses
  neither.
- **The smallest positive subnormal** rounds up to 1 — the smallest strictly positive value
  becomes the smallest positive integer. This is correct and is the cleanest demonstration that
  the function rounds up rather than to-nearest.
- **The largest finite double** is returned unchanged, because every double at or above 2^52 is
  already an integer and therefore already a multiple of 1.
- **An inline array** produces an array of the same shape. Note that the module implementing this
  family carries an open upstream defect stream concerning math scalar/array lifting
  (`BUG-FUNC-017`), so array-shaped arguments in this family are an area with recorded
  historical trouble rather than a settled one.
- **An empty range** produces `#VALUE!`.

**What is not documented, and is not invented here:** what happens when *significance* is zero.
Every multiple of zero is zero, so "the least multiple of 0 that is at least x" exists only for
x ≤ 0. Microsoft's page does not state a rule; the neighbouring `CEILING.MATH` documentation
states that a zero significance yields zero, but that is a different function's page and this
page will not transfer it. It is the first probe below.

## Errors

Microsoft's `ISO.CEILING` page documents **no error return**. Errors reachable here come from the
shared call model:

| Error | Condition |
|---|---|
| `#VALUE!` | An argument does not convert to a number under the shared to-number rules |
| propagated | An error value in either argument surfaces as that error |

The reference engine additionally reports `#VALUE!` for arity failures. The absence of a
documented `#NUM!` is a real difference from the legacy [CEILING](FUNC.CEILING.md), whose
documentation *does* carry a sign-mismatch error — because `ISO.CEILING` takes the absolute
value of significance, it has no sign mismatch to reject.

## Relationships

This is the most crowded neighbourhood in the math category, and the differences are entirely
about **what happens to negatives**.

- **[CEILING.PRECISE](FUNC.CEILING.PRECISE.md)** — documented with the same rule: absolute value
  of significance, mathematical ceiling regardless of signs. On the documented behaviour the two
  are the same function under two names, one carrying the ISO lineage and one the Excel-native
  naming scheme. This page does **not** assert that they share a code path or that they agree in
  every bit; it asserts only that the two published rules coincide.
- **[CEILING](FUNC.CEILING.md)** — the legacy function `ISO.CEILING` was introduced to correct.
  Its documented rule is sign-dependent, and it is the one that surprises people.
- **[CEILING.MATH](FUNC.CEILING.MATH.md)** — the modern general form, with an explicit third
  argument choosing the direction for negative numbers. `ISO.CEILING` is the fixed-mode case.
- **[FLOOR.PRECISE](FUNC.FLOOR.PRECISE.md)**, **[FLOOR.MATH](FUNC.FLOOR.MATH.md)**,
  **[FLOOR](FUNC.FLOOR.md)** — the downward mirrors, one for one.
- **[INT](FUNC.INT.md)** — the plain floor. `ISO.CEILING(x)` and `-INT(-x)` are the same number.
- **[MROUND](FUNC.MROUND.md)** — rounds to the *nearest* multiple rather than up.
- All seven of the ceiling/floor surfaces are implemented in one shared module in the reference
  engine, so they are a family by construction there as well as by documentation.

## Numerical notes

Like floor, ceiling introduces no rounding error of its own and inherits every hazard from the
discontinuity. The two additions specific to the *significance* form are worth stating.

**The division is where the error enters.** `⌈x/|s|⌉·|s|` performs a division, a ceiling and a
multiplication. Neither the division nor the multiplication is exact in general, so:

1. **`x/|s|` may round to the wrong side of an integer.** If *x* is an exact multiple of *s* but
   the quotient rounds to just above the integer, the ceiling adds a whole step and the answer is
   one multiple too high. This is not hypothetical: it is the standard failure mode of every
   naive round-to-multiple implementation, and it is why `ISO.CEILING(4.2, 0.1)` and its
   relatives are the classic test cases. Decimal significances like 0.1, 0.05 and 0.01 are not
   representable in binary64, so *every* such call is asking for the ceiling of a quotient that
   was already inexact.
2. **The final multiplication may not land back on a representable multiple.** `n·|s|` rounds,
   so the returned value can fail to be exactly divisible by *s* even when *n* is correct.

The standard remedies are the ones used throughout careful decimal-rounding work: compute the
quotient with a fused multiply-add or a double-double residual to determine the correct side of
the integer boundary before rounding; or scale by a power of ten and work in exact integers when
the significance is a decimal fraction; or apply a deliberate, documented tolerance so that a
quotient within a few ULP of an integer is treated as on it. Each of these is a *decision*, and
what matters for a compatibility implementation is which decision the target made — a question
this page cannot answer for Excel.

For the default significance of 1 none of this arises: the operation is IEEE 754's
`roundToIntegralTowardPositive` (C99 `ceil`, Rust `f64::ceil`), exact for every finite double,
correctly rounded by the standard, and with no intermediate at all. The gap in difficulty between
`ISO.CEILING(x)` and `ISO.CEILING(x, 0.1)` is the whole story of this function.

## What has not been checked

No Handbook vector suite exists for `ISO.CEILING`; `vectors/` publishes nothing for it, and **no
evidence record names `ISO.CEILING` among its subjects**. `ISO.CEILING` does appear inside the
group membership of one structural array-lift record without being one of that record's
subjects, and that record's counts carry an explicit warning against per-surface attribution —
so the honest statement is that the family was measured on an array-shape axis and **this surface
was not measured separately**. Nobody has checked this function's values against Excel.

Everything above marked as documented comes from Microsoft's `ISO.CEILING` page: the syntax, the
two argument descriptions, the default significance of 1, the absolute-value-of-significance
rule, and the six worked examples.

Inputs I would probe first:

1. **`ISO.CEILING(4.3, 0)` and `ISO.CEILING(-4.3, 0)`.** The undocumented zero-significance case.
   Two cells, and it is the only outright gap in the documented rule.
2. **`ISO.CEILING(4.2, 0.1)`, `ISO.CEILING(0.3, 0.1)`, `ISO.CEILING(1.4, 0.2)`.** The inexact-
   significance cases described above, where an implementation's tolerance policy becomes
   visible. These are the cells that separate implementations; a value that is mathematically
   already on the multiple but not in binary64 is exactly the input a naive kernel over-rounds.
3. **`ISO.CEILING(-4.3, -2)` against `CEILING.PRECISE(-4.3, -2)` and `CEILING(-4.3, -2)`**, in
   one workbook. This is the cheapest demonstration of what the three names actually differ on,
   and it would settle whether the two "precise" surfaces agree bit for bit or only in
   documentation.
4. **`ISO.CEILING(TRUE)` and `ISO.CEILING("2.5")`** — undocumented conversions the reference
   engine accepts.
5. **`ISO.CEILING(-0)` and the sign of the returned zero.**
6. **An array in each argument slot**, given the open lifting defect stream on this family's
   module.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| ceiling | The least integer not below the argument; rounds toward +∞ |
| significance | The multiple to round to; its absolute value is used |
| sign-blind | The rounding direction does not depend on the sign of either argument |
| inexact significance | A multiple such as 0.1 that has no exact binary64 representation |
| projection gap | A field the Handbook's mechanical data does not carry, published as absent |

## Sources

- Microsoft, "ISO.CEILING function" —
  <https://support.microsoft.com/en-us/office/iso-ceiling-function-e587bb73-6cc2-4113-b664-ff5b09859a83>.
  Retrieved for this page: the syntax, both argument descriptions, the default significance of 1,
  the absolute-value remark quoted above, and the six worked examples.
- ISO/IEC 29500 (OOXML) spreadsheet formula specification — the lineage the function's name
  records.
- IEEE 754-2019, `roundToIntegralTowardPositive` — the exact ceiling for the default-significance
  case.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — to-number outcomes,
  lifting, error propagation.
- `data/functions/FUNC.ISO.CEILING.json` — arity 1–2, the declared axes, and the empty signature
  placeholder recorded above, as projected at OxFunc `473efa3`;
  `data/presence/FUNC.ISO.CEILING.json` — the shared `ceiling_floor_family` module, its six
  sibling surfaces, and the `BUG-FUNC-017` lifting defect stream.
