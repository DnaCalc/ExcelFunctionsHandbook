---
schema: efh.function-page/v1
function_id: FUNC.ISODD
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
family: is_predicates_family
role_in_family: The odd one out in two senses — the only member of the module that coerces its
  argument to a number instead of classifying its kind.
---

## What it computes

`ISODD(number)` truncates `number` toward zero to an integer and returns `TRUE` if that integer
is odd, `FALSE` if it is even.

Microsoft's IS-functions table gives the condition as "Value refers to an odd number." The
companion `ISEVEN` page states the shared mechanics that apply to both: the argument "is
truncated" if it is not an integer, and a nonnumeric argument yields `#VALUE!`.

**Truncation is toward zero.** `ISODD(3.9)` tests 3 and is `TRUE`; `ISODD(-3.9)` tests −3 and is
`TRUE`; `ISODD(2.5)` tests 2 and is `FALSE`. Floor and truncate disagree on negative
non-integers, and parity makes the disagreement visible, so this is not a detail an
implementation can guess at.

**`ISODD` is not a kind test**, despite living in the IS family and sharing an implementation
module with `ISBLANK`, `ISTEXT` and the error predicates. It is a numeric function: it converts
its argument. The family remark that IS-function arguments "are not converted" is written about
the classifiers, and `ISODD` sits outside it — a documentation seam that this page flags rather
than smooths over.

## Arguments

`number` — required, exactly one.

Admissible values are numbers and whatever else Excel's to-number conversion accepts. The
documented failure condition is "nonnumeric", which is about convertibility, not kind.

What the reference engine does with the ambiguous kinds, as declared in its own contract:

| Argument | Reference engine |
|---|---|
| A number | truncate, test parity |
| Numeric text, e.g. `"2.5"` | converted, truncated to 2 — `FALSE` |
| Non-numeric text, e.g. `""` | `#VALUE!` |
| An empty cell | treated as `0` — `FALSE` |
| An omitted slot (`Missing`) | treated as `0` |
| `TRUE` / `FALSE` | **rejected** — `#VALUE!` |

The logical row is the odd one: most numeric contexts in Excel convert `TRUE` to 1, which would
make `ISODD(TRUE)` return `TRUE`. The reference engine instead declines the conversion and
errors. **Excel's answer is not known to the Handbook.**

## Result and edge cases

Returns a `Logical`, or an array of `Logical` when the argument is an array — `ISODD` is a
scalar kernel lifted elementwise, and an element that fails to convert carries its own error
without collapsing the array ([coercion and lifting](../model/02-coercion-and-lifting.md)).

- **Zero is even, so `ISODD(0)` is `FALSE`** — and zero arrives from an empty cell and an
  omitted slot as well as from arithmetic.
- **Sign does not affect parity.** −3 is odd. Sign affects only which way truncation moves.
- **Subnormals and tiny magnitudes** truncate to 0 and are therefore not odd.
- **Very large magnitudes are unresolved.** Every finite double at or above 2^53 is an exact
  even integer, so the mathematically correct answer for the largest finite double is `FALSE`.
  The reference engine's published battery row for that input reads `TRUE`, which is what a
  64-bit integer conversion produces once the value exceeds the integer range — the saturated
  maximum is odd. The Handbook does not know what Excel returns. Recorded here as an open
  behaviour question, not as a defect claim.
- **Errors propagate.** `ISODD` is an ordinary numeric function, so an error argument comes out
  as that error rather than as a `Logical`. This is the sharpest behavioural difference between
  `ISODD` and its module siblings, which classify errors instead.

## Errors

- **`#VALUE!`** for a nonnumeric argument, as documented on the `ISEVEN` page for the pair.
- **`#VALUE!`** for an arity failure — zero arguments or two. `ISODD()` is expected to be refused
  at formula entry rather than evaluated ([the call pipeline](../model/03-call-pipeline.md));
  the reference engine, having no entry-time surface, reports `#VALUE!` for both.
- **Any incoming error value**, propagated unchanged.

## Relationships

- **`ISEVEN`** is the complement over the convertible domain: for any argument that converts,
  exactly one of the two is `TRUE`. Outside that domain they do not complement — both return the
  same error. The two have adjacent XLL identities (`xlfIsodd` 421, `xlfIseven` 420) but live in
  different modules in the reference engine, so they need independent evidence.
- **`MOD(n,2)=1`** is the arithmetic form and is *not* equivalent on negatives: Excel's `MOD`
  takes the sign of the divisor, so `MOD(-3,2)` is 1 and the two happen to agree there, but the
  functions differ on non-integers because `MOD` does not truncate.
- **`ODD(n)`** rounds away from zero to the next odd integer; a rounding function, not a
  predicate. `ISODD(ODD(n))` is `TRUE` by construction.
- **`ROW()` / `COLUMN()`** are the usual arguments in practice — `ISODD(ROW())` for banded rows,
  where every edge case on this page is out of reach.
- **The kind-test siblings** (`ISBLANK`, `ISERR`, `ISERROR`, `ISLOGICAL`, `ISNA`, `ISNONTEXT`,
  `ISREF`, `ISTEXT`) share `ISODD`'s module and share almost nothing else. Do not reason from one
  to the other.

## Notes for implementers

1. **Truncate toward zero.** Negative non-integers are the discriminating tests.
2. **The integer-range boundary needs a decision, not a cast.** Converting `f64` to a fixed-width
   integer makes the answer above 2^63 a property of the conversion rather than of the number.
   Either compute parity from the exponent (everything at or above 2^53 is even), or guard the
   domain and state what happens outside it.
3. **The logical-argument decision must be explicit** and must match `ISEVEN`'s. Whatever the two
   do, they must do the same thing, or their complementarity breaks on a class of inputs where
   both answers look reasonable.
4. **Errors propagate here but are classified by the siblings.** Sharing a module makes it easy
   to route `ISODD` through the classifiers' error-exempt path by accident. It must not be.

## What has not been checked

No Handbook vector suite exists for `ISODD`; `vectors/` publishes nothing for this function. No
Excel-comparison evidence record names `ISODD` as a subject — **nobody has checked this
function's answers against Excel inside the Handbook's record.** (Its sibling `ISEVEN` appears in
one group structural record; nothing in it transfers to `ISODD`.) The reference engine's own
answers appear on the battery panel under their own label; they are not Excel's answers.

The probes that would settle the open questions:

1. **`ISODD(TRUE)` and `ISODD(FALSE)`.** Two cells, and the most likely divergence on the page.
2. **The large-magnitude domain** — 2^53, 2^53+1, 2^63, 2^64, the largest finite double. The
   mathematics says `FALSE` everywhere at or above 2^53; a 64-bit conversion says `TRUE` at the
   top. Where Excel's answer leaves the mathematics, if it does, is a real and findable fact.
3. **Numeric text** as a direct argument and as cell contents, since the IS-family
   non-conversion remark and the parity functions' own "nonnumeric" wording predict opposite
   answers.
4. **Negative non-integers** — `ISODD(-3.9)`, `ISODD(-2.5)` — to pin truncation direction.
5. **`ISODD` and `ISEVEN` over one shared input corpus**, checked for complementarity rather than
   tested separately, since the interesting failures are exactly the inputs where the two stop
   being complements.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| truncation toward zero | Dropping the fractional part; differs from floor on negatives |
| convertible domain | The set of arguments Excel's to-number conversion accepts |
| complementarity | The property that exactly one of `ISODD`/`ISEVEN` is `TRUE` |
| scalar kernel lift | The function is applied elementwise over array arguments |

## Sources

- Microsoft, "IS functions" —
  <https://support.microsoft.com/en-us/office/is-functions-0f2d7971-6019-40a0-a171-f2d869135665>.
  Read for this page: the `ISODD` and `ISEVEN` rows and the non-conversion remark.
- Microsoft, "ISEVEN function" —
  <https://support.microsoft.com/en-us/office/iseven-function-aa15929a-d77b-4fbb-92f4-2f479af55356>.
  Read for the truncation statement and the `#VALUE!` remark that the parity pair shares.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — to-number outcomes,
  the Empty/Missing distinction, error propagation, element-local array failures.
- Handbook, [the call pipeline](../model/03-call-pipeline.md) — the admission boundary and the
  scalar-lift profile.
- `data/functions/FUNC.ISODD.json` — identity (`xlfIsodd`, code 421), arity, declared axes, as
  projected at OxFunc `473efa3`.
