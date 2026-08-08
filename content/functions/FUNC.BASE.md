---
schema: efh.function-page/v1
function_id: FUNC.BASE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-STRUCT-0011
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Base method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.base"
    role: "documented description, the three parameters, the String return type, and the statement that leading zeros are not added when min_length is omitted; notable for stating no constraint on number or radix"
  - work: "Microsoft Support — BASE function"
    locator: "https://support.microsoft.com/en-us/office/base-function-2ef61411-aee9-4f29-a811-1c42456c6342"
    role: "the canonical worksheet article; not retrieved for this pass (fetch returned HTTP 403)"
  - work: "Knuth, The Art of Computer Programming, volume 2 (Seminumerical Algorithms)"
    locator: "section 4.1, positional number systems; section 4.4, radix conversion"
    role: "the uniqueness theorem for positional representation and the standard conversion algorithms"
  - work: "IEEE Std 754-2019, Standard for Floating-Point Arithmetic"
    locator: "binary64 significand width"
    role: "the 2^53 boundary beyond which consecutive integers are not all representable"
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
family: base_fn
role_in_family: >-
  Sole member of its module: the general positional radix formatter, DECIMAL's inverse, and the
  surface where the exactly-representable-integer boundary of binary64 becomes a user-visible
  domain limit.
---

## What it computes

`BASE(number, radix, [min_length])` writes a non-negative integer in a positional numeral system
of the given radix, and returns the result as **text**.

For an integer `n >= 0` and a radix `b >= 2`, the positional representation is the unique finite
sequence of digits `d_k d_(k-1) ... d_1 d_0` with

    n = sum over i of d_i * b^i,     0 <= d_i < b,     d_k != 0 (for n > 0)

Uniqueness is the theorem that makes the function well defined: for every `b >= 2` and every
`n >= 0` there is exactly one such sequence. It fails for `b = 1` (no positional system exists) and
for `b = 0` (nothing exists), which is why those radices must be rejected rather than given some
convention.

The digits are extracted by the standard recurrence

    d_i = floor(n / b^i) mod b,

equivalently by repeated division: divide by `b`, record the remainder, repeat on the quotient
until it is zero, and reverse. The number of digits is `floor(log_b n) + 1` for `n >= 1`, and `1`
for `n = 0`.

**Digit alphabet.** Radices above 10 need symbols beyond `0`-`9`. The universal convention, and the
one the reference engine's outputs are consistent with, is `0`-`9` then `A`-`Z`, which caps the
usable radix at 36. That cap is arithmetic, not arbitrary: 10 digits plus 26 letters is 36 symbols.

**`min_length` pads on the left with zeros.** Microsoft's Learn reference documents the parameter as
"the minimum length of the returned string" and states that if it is omitted, leading zeros are not
added. Padding never truncates: a representation longer than `min_length` is returned in full.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `number` | The value to convert | yes |
| `radix` | The base to convert into | yes |
| `min_length` | Minimum width of the returned text, left-padded with zeros | no |

The reference engine records an arity of 2 to 3, with `min_length` optional — matching Microsoft's
Learn parameter table, which marks `Arg3` optional and the other two required.

**The documentation gap on this page is large and worth stating plainly.** Microsoft's Learn
reference gives **no constraint at all** on any argument: no minimum or maximum radix, no bound on
`number`, no statement that `number` must be a non-negative integer, no bound on `min_length`, and
no error condition. The reference engine rejects radices that admit no positional system, and it
accepts non-integer inputs by truncating them. Neither of those behaviours is documented on
anything the Handbook retrieved. Microsoft's worksheet article — which is the natural home for such
constraints — was not retrievable for this pass.

The misunderstood position is the **return kind**: `BASE` returns `Text`, not a number. `BASE(255,
16)` is the string `"FF"`, and `BASE(9, 10)` is the string `"9"`, which will not compare equal to
the number `9` under `=`. Feeding a `BASE` result into arithmetic re-coerces it, and for radices
other than 10 that coercion means something entirely different from the digits you wrote.

## Result and edge cases

Returns `Text`.

- **Zero** is representable and its representation is the single digit `0` in every radix.
- **Non-integer `number`.** The reference engine truncates towards zero, so a fractional argument
  is converted as its integer part. Whether Excel truncates, rounds, or errors is not documented on
  anything retrieved. Note that a positional representation of a *fraction* would require a radix
  point and an infinite expansion in general; truncation is the only finite answer, but rejecting
  the input would also have been defensible.
- **Negative `number`.** There is no sign convention in the mathematics above, and the reference
  engine's battery rejects negative inputs. This is the single biggest behavioural difference from
  the fixed-radix conversion family — see *Relationships*.
- **Radix 0 and radix 1** are rejected: no positional system exists for either.
- **Radix above 36** exceeds the digit alphabet.
- **Non-integer `radix`.** The reference engine truncates, so a radix of 2.5 behaves as radix 2.
  Undocumented.
- **Very large `number`.** The hard limit is `2^53`, and the reason is the argument type rather
  than the algorithm — see the numerical notes.
- **`min_length` larger than the representation** pads; smaller has no effect; zero or omitted
  means no padding.
- **Arrays.** All argument positions broadcast. The evidence record attached to this page concerns
  exactly that: `BASE` is a named subject of an array-lift tranche, with its own witness row about
  a `radix` argument supplied as an array producing a text-valued array result.

The reference engine classifies this surface with a `Custom` coercion-lift profile and a `Custom`
kernel signature — the standard numeric shapes do not fit a function that returns text.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | The radix admits no positional representation, or *number* is outside the admissible range | Reference engine; **no error condition is documented on Microsoft's Learn page** |
| `#VALUE!` | An argument does not convert to a number | Shared coercion rules |
| propagated | An error value in any argument is returned unchanged | Shared coercion rules |

The condition column is deliberately imprecise about the exact bounds, because the exact bounds are
undocumented in every source the Handbook retrieved. Sharpening it is what the probe list is for.

## Relationships

- **`DECIMAL`** — the inverse. `DECIMAL(BASE(n, b), b)` should recover `n` for every admissible `n`
  and `b`, and that round trip is the best oracle-free test for the pair. `DECIMAL(text, radix)`
  reads what `BASE` writes.
- **`DEC2BIN`, `DEC2HEX`, `DEC2OCT`** — the fixed-radix conversions, and they are **not** special
  cases of `BASE`. Three differences matter:
  1. They accept **negative** numbers and encode them in **two's complement** over a fixed width;
     `BASE` has no sign convention at all.
  2. They have **narrow ranges** (a small number of places) where `BASE` runs to the
     exactly-representable-integer limit.
  3. They take a `places` argument that behaves like `min_length` but is bounded.
  A formula that swaps `DEC2HEX` for `BASE(x, 16)` changes behaviour on every negative input and on
  every large one.
- **`BIN2DEC`, `HEX2DEC`, `OCT2DEC`** — the fixed-radix readers, `DECIMAL`'s specialised cousins,
  with the same two's-complement asymmetry.
- **`BITAND` and the bit-operation family** — the other place the `2^48`-and-beyond integer limits
  of Excel's numeric model surface.
- **`ROMAN` / `ARABIC`** — the non-positional pair, and the useful contrast: `BASE` and `DECIMAL`
  are a clean bijection on their domain because positional representation is unique, whereas
  `ROMAN` and `ARABIC` are not, because Roman notation is not.
- **`TEXT`** — the general formatter, which cannot do radix conversion.
- **Confused with**: `DEC2HEX`, constantly, and with the idea that `BASE` returns a number.

## Numerical notes

`BASE` has no rounding error — every digit it produces is exact — but it sits on top of a
floating-point argument, and that is where all of its numerical content lives.

**The `2^53` boundary.** `number` arrives as a binary64 double. Doubles represent every integer up
to `2^53` exactly; above that, only even integers, then only multiples of 4, and so on. So for
arguments above `2^53` the *value the function receives is not the integer the user wrote*, and any
representation it produces is the correct representation of a different number. There is no way for
the function to detect this — the information is gone before the call. That is why `2^53` is the
natural upper bound for a radix formatter over doubles, and why an implementation that quietly
accepts larger arguments is producing digits that look authoritative and are not.

This is the same boundary that limits exact integer arithmetic everywhere in the worksheet, made
unusually visible because `BASE`'s whole output is digits.

**The digit-count trap.** The tempting way to size the output buffer, or to compute a digit, is
`floor(log(n) / log(b)) + 1`. This is wrong at and near exact powers of the radix: `LOG10(1000)`
computed in floating point can land just below `3`, and the formula then reports three digits where
four are needed. Every careful implementation either computes the digit count by the repeated
division itself, or computes it by logarithm and then *verifies* against a power. Knuth's treatment
of radix conversion in *TAOCP* volume 2 section 4.4 is the standard reference; the general lesson
is that a logarithm should never be the sole source of an integer count.

**Repeated division versus repeated multiplication.** Extracting digits least-significant-first by
`n mod b` then `n = floor(n/b)` is exact in integer arithmetic and, because every intermediate stays
below the starting value, exact in doubles too as long as the start is below `2^53`. Extracting
most-significant-first by dividing by `b^k` requires forming `b^k`, which is exactly representable
only for small `k` in most radices, and is therefore the worse algorithm on a floating-point
substrate.

**Truncation is a decision.** Truncating a non-integer `number` towards zero is one of at least
three defensible choices (round-half-even and reject being the others), and it produces different
results for negative inputs — which this function does not accept anyway, so the question only
bites on the fractional part. The reference engine truncates; nothing documents what Excel does.

**Padding is not formatting.** `min_length` pads with the *digit* zero, which is a semantically
meaningful character in a positional system, not a cosmetic space. Padding a base-16 value to eight
places produces a string that reads correctly as a base-16 numeral; the `TEXT` function's padding
would not.

## What has not been checked

The evidence attached to this page is **`EV-STRUCT-0011`**, a **structural-verification** record
whose subject list contains `FUNC.BASE`. It records an array-lift tranche: `BASE` is a named
subject with its own witness row about an array-valued `radix` argument, and text-returning
broadcast support was added for this surface as part of the fix. **But the record's counts are
group totals across many surfaces, with no per-surface split**, and the record says so and warns
against per-surface attribution. So the array-lift behaviour of `BASE` was measured *as part of a
group*; it was not measured separately, and no per-surface figure exists.

Beyond that: **no numeric or structural per-surface count exists for `BASE`, and no Handbook vector
suite exists for it.** The whole of the function's actual content — the admissible radix range, the
upper bound on `number`, the treatment of non-integers, the digit alphabet, the padding rules —
is unmeasured and, on everything the Handbook retrieved, undocumented.

The documented statements above come from Microsoft's Learn `WorksheetFunction.Base` reference,
which was retrieved and which documents the parameters and the padding remark and nothing else.
Microsoft's worksheet article was not retrieved for this pass (HTTP 403).

Probes worth running first, ordered by how much each settles:

1. **Radix sweep 0, 1, 2, 36, 37** with a fixed small `number`. Five probes that pin the entire
   admissible radix range and confirm the 36-symbol alphabet, all of which is undocumented.
2. **`BASE(255, 16)` and `BASE(255, 2)`** — the digit alphabet and case (`"FF"` versus `"ff"`),
   which nothing documents.
3. **The upper bound on `number`**: `2^53 - 1`, `2^53`, `2^53 + 2`. Whether the function errors at
   the exactly-representable boundary or keeps going past it is the single most consequential
   undocumented fact about it.
4. **`BASE(-1, 2)`** — the sign question, and the one that most distinguishes this function from
   `DEC2BIN`.
5. **`BASE(2.5, 10)` and `BASE(10, 2.5)`** — truncation in both slots.
6. **`min_length` of 0, of exactly the natural width, of a large value, and negative** — four
   probes covering a parameter with no documented bounds.
7. **`DECIMAL(BASE(n, b), b)` over a grid of `n` and `b`** — the round trip, requiring no oracle,
   and the fastest route to a disagreement between the pair.
8. **Array arguments in each of the three positions**, given the array-lift record attached here.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| positional representation | Writing `n` as a sum of digits times powers of a radix; unique for every radix `>= 2` |
| radix | The base of the positional system; here bounded above by the digit alphabet |
| digit alphabet | `0`-`9` then `A`-`Z`, which caps the usable radix at 36 |
| `min_length` padding | Left-padding with the digit zero to a minimum width; never truncates |
| exactly-representable-integer boundary | `2^53`, above which consecutive integers are not all binary64 values |
| group total | An evidence count spanning many surfaces, from which no per-surface figure may be read |

## Sources

- Microsoft Learn, "WorksheetFunction.Base method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.base> (retrieved: the
  description, the three parameters with `Arg3` optional, the `String` return type, and the
  statement that leading zeros are not added when `min_length` is omitted; **no constraint on
  `number` or `radix` and no error condition are stated there**).
- Microsoft, "BASE function" —
  <https://support.microsoft.com/en-us/office/base-function-2ef61411-aee9-4f29-a811-1c42456c6342>
  — the canonical worksheet article. **Not retrieved for this pass** (HTTP 403).
- Knuth, *The Art of Computer Programming*, volume 2, sections 4.1 and 4.4 — positional number
  systems, the uniqueness theorem, and radix-conversion algorithms including the digit-count
  pitfall.
- IEEE Std 754-2019 — the binary64 significand width, hence the `2^53` boundary.
- Handbook evidence record `EV-STRUCT-0011` (subjects `FUNC.ATAN2`, `FUNC.BASE`) — the array-lift
  tranche, with a per-surface witness for `BASE` and counts that are group totals carrying a warning
  against per-surface attribution.
- Handbook projections `data/functions/FUNC.BASE.json` (arity 2-3, `Custom` coercion-lift and
  kernel signature) and `data/presence/FUNC.BASE.json` (implementing module; the `BUG-FUNC-017`
  array-lift defect stream).
- Handbook [The value universe](../model/01-value-universe.md) (text as UTF-16 code units) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
