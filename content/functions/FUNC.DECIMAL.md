---
schema: efh.function-page/v1
function_id: FUNC.DECIMAL
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-STRUCT-0009
open_problems: []
references:
  - work: "Microsoft Support — DECIMAL function"
    locator: "https://support.microsoft.com/en-us/office/decimal-function-ee554665-6176-46ef-82de-0a283658da2e"
    role: "documented signature, the radix range, the 255-character and 2^53 limits, the digit alphabet, and the error conditions"
  - work: "Knuth, The Art of Computer Programming, volume 2 (Seminumerical Algorithms)"
    locator: "section 4.4 (Radix Conversion)"
    role: "the standard treatment of positional radix conversion and where exactness is lost"
  - work: "Muller et al., Handbook of Floating-Point Arithmetic"
    locator: "the radix-conversion chapter"
    role: "when an integer-valued conversion is exact in binary64 and when it is not"
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
family: decimal_fn
role_in_family: >-
  The general radix parser: text in an arbitrary base from 2 to 36 into a worksheet number, and
  the documented inverse of BASE.
---

# DECIMAL

## What it computes

`DECIMAL(text, radix)` reads *text* as a numeral written in base *radix* and returns its value
as a worksheet number. It is a parser, not an arithmetic function — the "decimal" in the name
refers to the output being an ordinary number rather than a digit string, not to base 10 having
any special role in the computation.

The definition is positional notation. For a digit string `d_{n-1} … d_1 d_0` in base `b`:

    value = Σ_{i=0}^{n-1} d_i · b^i

with the digit alphabet `0`–`9` then `A`–`Z` supplying values `0`–`35`, and the reading
case-insensitive.

| Property | Statement |
|---|---|
| Radix range (documented) | `2 ≤ radix ≤ 36` |
| Text length (documented) | at most 255 characters |
| Value range (documented) | the text must resolve to a number `≥ 0` and `< 2^53` |
| Digit alphabet | `0`–`9`, then `A`–`Z`, case-insensitive |
| Result kind | `Number` |
| Exactness | every value below `2^53` is exactly representable, so the conversion is exact within the documented range |
| Inverse | `BASE(number, radix)` — the digit-string producer |

The documented value bound is the interesting one: `2^53` is exactly the point at which binary64
stops being able to represent every integer. Microsoft's page draws the boundary there and adds
that text resolving to values above it may lose precision. That is not a limitation of radix
conversion; it is a limitation of the result type, stated honestly on the documentation page.

## Arguments

| Argument | Meaning | Constraint (as documented) |
|---|---|---|
| `text` | The numeral to read. Required. | at most 255 characters; alphanumeric characters valid for the radix |
| `radix` | The base to read it in. Required. | an integer, at least 2 and at most 36 |

Both slots take the shared coercion rules, with the wrinkle that the first slot is a *text*
slot that will often receive a number: `DECIMAL(1010, 2)` passes a number where a numeral was
asked for, and the number must be rendered to text before it can be parsed. Which rendering —
general format, full precision, locale-dependent — is not stated on Microsoft's page, and the
Handbook has not checked. That is a real question, not a pedantic one: a large numeric first
argument could render in scientific notation and then contain characters that are not digits in
any radix.

The reference engine classifies this surface with a `Custom` coercion-and-lift profile and a
`Custom` kernel signature, which is the projection's way of saying the argument handling is not
one of the standard shapes. See [Coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

Returns `Number`.

- **Empty text** — not documented. An empty numeral has no value; whether Excel returns `0` or
  an error is unchecked.
- **Leading and trailing spaces** — not documented.
- **Digits out of range for the radix** — e.g. `DECIMAL("2", 2)`, where `2` is not a binary
  digit. Microsoft's page says arguments outside the stated constraints may return `#NUM!` or
  `#VALUE!`, without saying which applies here.
- **Non-integer radix** — the page documents *radix* as an integer; whether a fractional radix
  truncates or errors is not stated.
- **Values at and above `2^53`** — documented as possible precision loss rather than as an
  error. Note the asymmetry with the *lower* bound, which is documented as a constraint: a
  negative value is out of range, an over-large value is merely imprecise.
- **A leading minus sign** — the documented range starts at `0`, so signed input is outside the
  documented domain. What happens is not stated.

Every one of these is a boundary the documentation leaves open, and together they are why this
page's probe list is longer than its behaviour section.

## Errors

Microsoft's page states the error conditions **jointly and loosely**: arguments outside the
stated constraints may return `#NUM!` or `#VALUE!`. It does not map individual violations to
individual error values.

| Error | Condition (as documented, loosely) |
|---|---|
| `#NUM!` or `#VALUE!` | an argument is outside its documented constraints |

That is an unusually weak error specification for a function with five separate documented
constraints, and it is recorded here as a documentation gap rather than paraphrased into a
precision the source does not have.

## Relationships

- **`BASE`** — the documented inverse: `BASE` writes a number as a digit string in a given
  radix, `DECIMAL` reads one back. `DECIMAL(BASE(n, b), b) = n` is the round trip, and it is
  the natural metamorphic test for both surfaces.
- **`BIN2DEC`, `OCT2DEC`, `HEX2DEC`** — the fixed-radix predecessors from the Analysis
  ToolPak, for bases 2, 8 and 16. They differ from `DECIMAL` in a way that matters: they
  interpret their input as a **two's-complement** value of fixed width, so a leading digit can
  mean a negative number, while `DECIMAL` is documented over non-negative values only. Replacing
  `HEX2DEC` with `DECIMAL(…, 16)` silently changes the meaning of every input whose top bit is
  set.
- **`DEC2BIN`, `DEC2OCT`, `DEC2HEX`** — the corresponding writers, and `BASE`'s predecessors.
- **`VALUE`, `NUMBERVALUE`** — the base-10 text-to-number parsers, locale-aware where `DECIMAL`
  is not documented to be.
- **`ARABIC`** — the other non-positional numeral parser on the worksheet surface, and
  `DECIMAL`'s co-subject in the evidence record attached to this page.

## Numerical notes

Radix conversion is exact arithmetic dressed as parsing, and its accuracy story is short but
not empty.

**Within the documented range, the answer is exact and there is no excuse for anything less.**
Every integer below `2^53` is a binary64 value, so Horner's rule over the digits —
`acc ← acc·b + d` — computes the value with no rounding at all, provided each intermediate stays
below `2^53`. Since the intermediates of a correct parse are prefixes of the final value, they
do. Knuth TAOCP volume 2 §4.4 is the standard treatment.

**Outside it, the choices start to matter.** For a numeral whose value exceeds `2^53`,
Horner-in-binary64 rounds at every step, and the accumulated result can differ from the
correctly rounded value of the numeral by several ulp. The correct approach — the one a
`math-correct` flavour would take — accumulates in an exact integer type and rounds **once** at
the end. Microsoft's page documents that precision may be lost without saying which discipline
applies, so the difference between "rounds once" and "rounds every step" is unspecified and
observable. It is the sharpest numerical question on this page.

**Powers of two are the special case.** When the radix is 2, 4, 8, 16 or 32, digits map onto
fixed bit-fields and the conversion is a shift-and-or with no multiplication at all. An
implementation that special-cases them is exact for longer and faster; one that does not is
neither. Whether Excel distinguishes them is unchecked, and the discriminating probe is a long
numeral in base 16 against the same value in base 10.

**Length versus value.** The documented 255-character limit and the documented `2^53` value
limit are not the same constraint: a 255-character base-2 numeral of all zeros followed by a one
has a tiny value, and a 12-character base-36 numeral exceeds `2^53`. Which limit is checked
first, and whether the length limit is on the text or on its significant digits, is not
documented.

## What has not been checked

`EV-STRUCT-0009` names this surface. It records that `DECIMAL` was inside a structural sweep
against Excel — so "no structural comparison record exists" would be false — while stating in
its own words that **no per-surface count exists**: the run's figures are a group total across
many surfaces with no split, and the record's reader warning forbids rendering any of them as a
pass rate for this function. The record also notes that the Excel build is not stated in it and
has deliberately not been inherited from a later resweep. The honest summary the record itself
offers: a structural comparison is on record, and no row count was extracted.

That record is about **structure** — argument admission and result shape — not about values. No
value-level comparison of `DECIMAL` against Excel exists in the Handbook's record, and no
Handbook vector suite exists for it. The presence projection also attaches an open defect stream
covering text, date, array-lift and coercion gaps to this module, so the argument-handling
questions raised above are unsettled upstream as well.

Inputs I would probe first:

1. **The round trip against `BASE`**: `DECIMAL(BASE(n, b), b) = n` for `n` sweeping across the
   `2^53` boundary and `b` over all documented radixes. This needs no oracle for the values
   below the boundary — the identity must hold exactly — and above it, the *pattern* of failure
   distinguishes round-once from round-every-step.
2. **The same value in two radixes**: a large integer written in base 10 and in base 16, both
   passed to `DECIMAL`. If they disagree, a power-of-two fast path exists.
3. **Digit-alphabet boundaries**: `DECIMAL("Z", 36)`, `DECIMAL("Z", 35)`, `DECIMAL("z", 36)`,
   `DECIMAL("2", 2)`, `DECIMAL("", 10)`, `DECIMAL(" 10 ", 10)`. Five probes that map the
   accept/reject frontier the documentation states only in outline, and each answers with a
   kind rather than a number.
4. **Radix bounds**: `radix` of 1, 2, 36, 37, `36.9`, and `-2`. Which error value each produces
   resolves the loose `#NUM!`-or-`#VALUE!` specification.
5. **A numeric first argument**: `DECIMAL(1010, 2)` and `DECIMAL(1E+20, 10)`. The second forces
   the number-to-text rendering question into the open, because a scientific-notation rendering
   contains characters no radix accepts.
6. **The length limit**: numerals of 254, 255 and 256 characters, in base 2 with small value and
   in base 36 with large value, separating the length check from the value check.
7. **Signed input**: `DECIMAL("-10", 10)`, which is outside the documented non-negative range.
8. **Array arguments in both positions**, given the open coercion-and-array-lift defect stream
   on this module.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| radix | The base a numeral is written in; here documented from 2 to 36 |
| digit alphabet | The ordered symbol set `0`–`9`, `A`–`Z` supplying digit values `0`–`35` |
| Horner's rule | The accumulation `acc ← acc·b + d` used to evaluate a positional numeral |
| round once | Accumulating exactly and rounding a single time at the end |
| two's complement | The fixed-width signed reading used by `HEX2DEC` and peers, not by `DECIMAL` |
| structural comparison | A check of argument admission and result shape rather than of values |

## Sources

- Microsoft, "DECIMAL function" —
  <https://support.microsoft.com/en-us/office/decimal-function-ee554665-6176-46ef-82de-0a283658da2e>
  (fetched at curation: signature, the 2-to-36 radix range, the 255-character text limit, the
  non-negative and below-`2^53` value range, the case-insensitive alphanumeric digit alphabet,
  the precision note above `2^53`, and the jointly-stated `#NUM!`/`#VALUE!` condition).
- Handbook evidence record `EV-STRUCT-0009` — the structural sweep naming `ARABIC` and
  `DECIMAL`, with no per-surface split and an explicit prohibition on deriving one. Read its
  reader warning.
- Knuth, *TAOCP* volume 2 §4.4 — radix conversion.
- Muller et al., *Handbook of Floating-Point Arithmetic* — exactness of integer conversion in
  binary64.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [The value universe](../model/01-value-universe.md).
- Handbook projections `data/functions/FUNC.DECIMAL.json` (the `Custom` coercion and kernel
  classifications) and `data/presence/FUNC.DECIMAL.json` (implementing module and the open
  conversion/coercion defect stream).
