---
schema: efh.function-page/v1
function_id: FUNC.ISEVEN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-STRUCT-0007
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
family: iseven_fn
role_in_family: Sole member of its module; the even half of the parity pair, and one of only two
  IS-named functions that coerce their argument.
---

## What it computes

`ISEVEN(number)` truncates `number` toward zero to an integer and returns `TRUE` if that integer
is even, `FALSE` if it is odd.

Microsoft's own wording, verbatim: "Returns TRUE if number is even, or FALSE if number is odd",
with the argument described as "The value to test. If number is not an integer, it is
truncated", and the remark "If number is nonnumeric, ISEVEN returns the #VALUE! error value."

Two things follow that are worth stating precisely.

**Truncation is toward zero, not rounding.** `ISEVEN(2.5)` tests 2 and is `TRUE`;
`ISEVEN(3.9)` tests 3 and is `FALSE`; `ISEVEN(-2.5)` tests −2 and is `TRUE`. Truncation toward
zero and floor disagree on negatives, and parity is exactly where that disagreement becomes
visible, so an implementation that floors will be wrong on half the negative non-integers.

**Zero is even.** `ISEVEN(0)` is `TRUE`, which Microsoft's page states explicitly and which
matters because zero arrives from more directions than any other value: an empty cell, an
omitted slot, a subtraction that cancelled.

**`ISEVEN` is not a kind test.** Despite the name, it does not belong with `ISNUMBER` and
`ISTEXT`. It is a numeric function with a boolean result. The IS-family remark that arguments
"are not converted" does not describe `ISEVEN`'s behaviour, and the two documentation pages sit
in tension on this point — see "What has not been checked".

## Arguments

`number` — required, exactly one. The published signature is `ISEVEN(number)`.

Admissible values are numbers, and — this is the part the documentation leaves open — whatever
else Excel's to-number conversion accepts. The documented failure condition is stated as
"nonnumeric", which is a statement about convertibility rather than about kind.

What the reference engine does with the ambiguous kinds, as declared in its own contract:

| Argument | Reference engine |
|---|---|
| A number | truncate, test parity |
| Numeric text, e.g. `"2.5"` | converted, then truncated — `TRUE` |
| Non-numeric text, e.g. `""` | `#VALUE!` |
| An empty cell | treated as `0` — `TRUE` |
| An omitted slot (`Missing`) | treated as `0` |
| `TRUE` / `FALSE` | **rejected** — `#VALUE!` |

The logical row is the surprising one. Most numeric contexts in Excel convert `TRUE` to 1; this
function's reference implementation declines to, which would make `ISEVEN(TRUE)` an error rather
than `FALSE`. **Excel's answer here is not known to the Handbook**, and it is the first probe
listed below.

## Result and edge cases

Returns a `Logical`, or an array of `Logical` when the argument is an array — `ISEVEN` is a
scalar kernel lifted elementwise, so `ISEVEN({1,2;3,4})` yields `{FALSE,TRUE;FALSE,TRUE}` and an
element that fails to convert carries its own error without collapsing the array
([coercion and lifting](../model/02-coercion-and-lifting.md)).

- **Negative numbers.** Parity is sign-independent: −4 is even. The sign matters only through
  the truncation direction.
- **Subnormal and tiny magnitudes** truncate to 0 and are therefore even.
- **Very large magnitudes are an unresolved case.** Every finite double at or above 2^53 is an
  exact even integer, so the mathematically correct answer for the largest finite double is
  `TRUE`. The reference engine's published battery row for that input reads `FALSE`, which is
  what a 64-bit integer conversion produces once the value exceeds the integer range. The
  Handbook does not know what Excel returns. This is recorded as an open behaviour question, not
  as a defect claim against either party, and it is probe 2 below.
- **Errors propagate.** Unlike the kind-test IS functions, `ISEVEN` is an ordinary numeric
  function: an error argument comes out as that error, not as a `Logical`.

## Errors

- **`#VALUE!`** — documented: "If number is nonnumeric, ISEVEN returns the #VALUE! error value."
- **`#VALUE!`** for an arity failure — zero arguments or two. `ISEVEN()` is expected to be
  refused at formula entry rather than evaluated
  ([the call pipeline](../model/03-call-pipeline.md)); the reference engine, having no
  entry-time surface, reports `#VALUE!` for both.
- **Any incoming error value**, propagated unchanged.

Microsoft's page documents no other error return.

## Relationships

- **`ISODD`** is the complement over the convertible domain: for any argument that converts,
  exactly one of `ISEVEN` and `ISODD` is `TRUE`. Where the argument does not convert, both
  return the same error rather than complementing. Excel implements the two in different
  internal groupings — `ISEVEN` has its own XLL slot (`xlfIseven`, 420) next to `ISODD` (421) —
  and the reference engine keeps them in separate modules, so they are worth testing as two
  functions rather than one.
- **`MOD(n,2)=0`** is the arithmetic equivalent for exact integers, but it does not truncate and
  it handles negatives differently, so the two are not interchangeable on real data.
- **`EVEN(n)`** rounds *up in magnitude* to the next even integer; it is a rounding function, not
  a predicate, and `ISEVEN(EVEN(n))` is `TRUE` by construction.
- **`ODD(n)`** is `EVEN`'s counterpart.
- The parity pair is most often seen in conditional formatting as `ISEVEN(ROW())` for banded
  rows — where the argument is always a positive integer and none of this page's edge cases can
  arise.

## Notes for implementers

1. **Truncate toward zero, then take parity.** Not floor, not round-half-even. The negative
   non-integers are the test cases that separate a correct implementation from a plausible one.
2. **Decide the integer-range question deliberately.** A `f64 → i64` conversion silently
   saturates or wraps above 2^63, which produces a parity answer determined by the conversion
   rather than by the number. If you route through a fixed-width integer, the domain where that
   is safe must be stated and guarded, and the behaviour above it must be a choice — an error, a
   mathematically correct answer computed from the exponent, or a documented match to whatever
   Excel does — never an accident.
3. **Decide the logical-argument question deliberately too**, and record which way and why. It
   is the one row where the reference engine departs from the coercion habit of the rest of the
   library.
4. **Lift elementwise, keeping element failures element-local** — a non-convertible element gets
   its own `#VALUE!`, the array survives.

## What has not been checked

No Handbook vector suite exists for `ISEVEN`; `vectors/` publishes nothing for this function.

One evidence record names `ISEVEN`: **EV-STRUCT-0007**, a structural-admission resweep against
live Excel 16.0 build 20026. Read it with its own reader warning attached, because the warning
is the point: the record's "46 of 47" is a *group* figure shared across roughly twenty surfaces,
each of which contributed one or two probe cases, and **it may not be rendered as a pass rate for
`ISEVEN`**. What it supports for this function is narrow and worth having anyway — the one or two
array-shape and coercion-placement cases `ISEVEN` contributed to that group matched. The record
establishes nothing about `ISEVEN`'s parity answers, and the source's member list ends in an
ellipsis, so even the membership is approximate.

So: argument shape has been touched once, in a group, at one build. Everything below is open.

1. **`ISEVEN(TRUE)` and `ISEVEN(FALSE)`.** Two cells. The reference engine says `#VALUE!`; the
   ordinary Excel habit would say `FALSE` and `TRUE`. This is the page's most likely divergence
   and the cheapest to settle.
2. **The large-magnitude domain.** `ISEVEN` of 2^53, 2^53+2, 2^63, 2^64 and the largest finite
   double. Mathematically all are even; a 64-bit conversion says otherwise for the top of the
   range. Finding where Excel's answer stops tracking the mathematics — if it does — would be a
   genuine result.
3. **Numeric text.** `ISEVEN("2")` and `ISEVEN("2.5")` as direct arguments and as cell contents.
   The IS-functions page's non-conversion remark and this function's own "nonnumeric" wording
   predict opposite answers, and the Handbook has no observation to choose between them.
4. **Negative non-integers** — `ISEVEN(-2.5)`, `ISEVEN(-3.5)` — to confirm truncation toward
   zero rather than floor.
5. **Empty and omitted.** `ISEVEN(A1)` with `A1` blank, and an omitted slot inside a `LAMBDA`.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| truncation toward zero | Dropping the fractional part; differs from floor on negatives |
| convertible domain | The set of arguments Excel's to-number conversion accepts |
| group figure | A count measured across several surfaces jointly; not a per-function rate |
| scalar kernel lift | The function is applied elementwise over array arguments |

## Sources

- Microsoft, "ISEVEN function" —
  <https://support.microsoft.com/en-us/office/iseven-function-aa15929a-d77b-4fbb-92f4-2f479af55356>.
  Read for this page: the description, the `number` argument description including truncation,
  the `#VALUE!` remark, and the statement that zero is even.
- Microsoft, "IS functions" —
  <https://support.microsoft.com/en-us/office/is-functions-0f2d7971-6019-40a0-a171-f2d869135665>.
  Read for the `ISEVEN` row and for the non-conversion remark that sits in tension with it.
- Handbook evidence record `EV-STRUCT-0007` — group structural resweep, live Excel 16.0 build
  20026; carries a reader warning against per-surface rates.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — to-number outcomes,
  the Empty/Missing distinction, element-local array failures.
- Handbook, [the call pipeline](../model/03-call-pipeline.md) — the admission boundary and the
  scalar-lift profile.
- `data/functions/FUNC.ISEVEN.json` — identity (`xlfIseven`, code 420), the published signature
  `ISEVEN(number)`, arity, declared axes, as projected at OxFunc `473efa3`.
