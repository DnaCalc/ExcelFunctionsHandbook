---
schema: efh.function-page/v1
function_id: FUNC.ASINH
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0009
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Asinh method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.asinh"
    role: "documented description, the 'any real number' argument statement, and the Asinh(Sinh(x)) = x round trip"
  - work: "Microsoft Support — ASINH function"
    locator: "https://support.microsoft.com/en-us/office/asinh-function-4e00475a-067a-43cf-926a-765b0249717c"
    role: "the canonical worksheet article; not retrieved for this pass (fetch returned HTTP 403)"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 4 (Elementary Transcendental Functions), section 4.6"
    locator: "4.6.20, the arcsinh logarithmic form"
    role: "the closed form, the series, and the relations to the other inverse hyperbolic functions"
  - work: "fdlibm, s_asinh.c"
    locator: null
    role: "the published three-branch reference implementation: log1p near zero, closed form in the middle, ln(2x) at the top"
  - work: "Higham, Accuracy and Stability of Numerical Algorithms"
    locator: "chapter 1, the log1p discussion"
    role: "why ln(1 + t) must not be evaluated by forming 1 + t"
  - work: "Muller, Elementary Functions: Algorithms and Implementation"
    locator: "chapters on inverse hyperbolic functions and on avoiding spurious overflow"
    role: "the general treatment of intermediate overflow in closed-form evaluations"
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
family: asinh
role_in_family: >-
  The inverse hyperbolic sine on all of R: the only unrestricted member of the inverse hyperbolic
  set, and the surface whose identified substrate turns an intermediate overflow into a documented
  domain that the function does not actually cover.
---

## What it computes

`ASINH(number)` is the inverse hyperbolic sine.

    asinh x  =  ln( x + sqrt(x^2 + 1) )

- **Domain**: all real numbers. `sinh` is a strictly increasing bijection from `R` onto `R`, so
  its inverse needs no branch choice and has no domain restriction. Microsoft's Learn reference
  says exactly this: the argument is "any real number".
- **Range**: all real numbers.
- **Parity**: odd, exactly. `asinh(-x) = -asinh(x)`. Because the square root is always at least
  `|x|`, the closed form above never cancels for positive `x` — but it cancels badly for negative
  `x`, which is why every implementation folds the sign. See the numerical notes.
- **Monotonicity**: strictly increasing everywhere.
- **Derivative**: `d/dx asinh x = 1 / sqrt(x^2 + 1)` — bounded by 1, positive, and smooth on the
  whole line. There are no singularities of any kind. `ASINH` is the best-conditioned function in
  its family and its difficulties are entirely about representation.
- **Series about zero**: `asinh x = x - x^3/6 + 3x^5/40 - ...`, so `asinh x -> x` for small `x` and
  subnormals pass through unchanged.
- **Asymptotics**: `asinh x = ln(2x) + 1/(4x^2) - ...` as `x -> +infinity`. Growth is logarithmic:
  every argument in the upper half of the double range maps into a narrow band of results, and at
  the top of binary64 the answer is a little over 710.
- **Relations**: `asinh x = sign(x) * acosh(sqrt(x^2 + 1))` and
  `asinh x = atanh( x / sqrt(x^2 + 1) )`; also `asinh(sinh t) = t` for every real `t`, which is
  the round trip Microsoft's Learn reference states.
- **Complex continuation**: branch cuts along the imaginary axis outside `[-i, i]`. Nothing on the
  real line is cut, which is the precise reason the real domain is unrestricted.

Abramowitz & Stegun give the logarithmic closed form in chapter 4 section 4.6.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `number` | Any real number. Required. | — |

That is Microsoft's own wording from the Learn reference. **The Handbook flags it as the most
consequential documented statement on this page**, because what is on record upstream about the
implementation contradicts it — see *Result and edge cases* below.

One argument; the reference engine records an arity of exactly one, a `NumToNum` kernel signature
and a unary numeric scalar-or-array lift profile, so arrays lift elementwise. Ordinary numeric slot
under the shared coercion rules — see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

There are no units here. `ASINH` takes a number, not an angle; the `RADIANS`/`DEGREES` apparatus
that attaches to the circular inverses is meaningless for it.

## Result and edge cases

Returns `Number`.

- **Zero** returns zero, and an odd implementation preserves the sign of zero.
- **Subnormals and very small arguments** pass through unchanged.
- **Negative arguments** are the case the closed form handles badly and the oddness handles for
  free.
- **Very large arguments** are the interesting case, and the reason this page carries an evidence
  record.

### The overflow boundary — the substantive finding on this page

Mathematically, `asinh` of the largest finite double is a modest number just above 710: finite,
representable, and nowhere near any limit. But the closed form
`ln(x + sqrt(x^2 + 1))` forms `x^2` first, and `x^2` overflows binary64 as soon as `|x|` exceeds
the square root of the largest finite double — approximately `1.34e154`. Above that point the
intermediate is infinite while the answer is small.

**`EV-MATH-0009` is an `excel-math-deviation` record whose subject is `FUNC.ASINH`**, and what it
identifies is exactly this: the substrate is the literal `ln(x + sqrt(x^2+1))`, the `x^2`
intermediate overflows, and an error is published well below where the mathematical result would
require one. The record states the mechanism and, in its own words, publishes **no count**; its
reader warning says that an exactness claim without a denominator is not a count and that this
record publishes none. The Handbook repeats the shape and not the figures, which are rendered
mechanically beside this page.

The reference engine reproduces that boundary deliberately rather than repairing it: its `ASINH`
kernel forms the same `x*x` intermediate and rejects when it is not finite, so that the flip
happens at the same argument rather than at a guessed constant. That is a compatibility decision,
visible in the source at commit `473efa3`, and it is the right one for an
Excel-compatibility flavour — but it means the reference engine is not evidence that the behaviour
is correct, only that it is faithful.

**The documentation-versus-record divergence, stated plainly.** Microsoft's Learn reference
documents the argument as "any real number" and lists no error condition. The identified substrate
makes an entire band of real numbers — more than a hundred decades of the double range, from about
`1.34e154` to the largest finite double — fail. Documentation and behaviour disagree, and the
Handbook publishes both.

**Contrast with `ACOSH`.** The sibling closed form differs by one sign under the square root and
has the identical overflow, yet the reference engine's `ACOSH` carries no such guard and its
projected `real_result_policy` (`non_finite=allow`) permits a non-finite value to leave the kernel.
Two adjacent surfaces, one hazard, two treatments. See the [ACOSH](FUNC.ACOSH.md) page.

- **Arrays** lift elementwise.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#NUM!` | `|number|` is large enough that the `x^2` intermediate of the closed form overflows | `EV-MATH-0009` (substrate identified, no count) and the reference engine's kernel — **not documented anywhere retrieved** |
| `#VALUE!` | The argument does not convert to a number | Shared coercion rules |
| propagated | An error value in the argument is returned unchanged | Shared coercion rules |

There is **no mathematical domain error for `ASINH`**. The `#NUM!` above is an artefact of an
evaluation form, not a statement about the function.

## Relationships

- **`SINH`** — the forward function. `ASINH(SINH(t)) = t` for every real `t`, which is the cleanest
  round trip in the family (no absolute value, no branch), and therefore the best oracle-free test
  on this page.
- **`ACOSH`** — the closed forms differ by one sign under the square root; that sign is the whole
  difference between an unrestricted domain and `[1, infinity)`. The pair also differ in how the
  reference engine treats their shared overflow, which is a finding recorded on both pages.
- **`ATANH`** — the third inverse hyperbolic primary. `asinh x = atanh(x/sqrt(x^2+1))`.
- **`ASIN`** — the circular namesake, and a completely different shape: bounded domain, bounded
  range, singular derivative at the endpoints. Sharing a name is the only thing they do.
- **`LN`** and **`SQRT`** — the substrate, and the source of both the cancellation and the
  overflow.
- **`LOG1P`** — which Excel does not expose as a worksheet function. Its absence is why a
  worksheet-level workaround for the small-argument hazard is awkward, and why implementers of this
  surface must supply it internally.
- **Confused with**: `ASIN`, by name; and with `1/SINH`, which is `CSCH`.

## Numerical notes

The closed form is correct and, taken literally, fails at three places. All three are standard, and
all three have one-line remedies.

**1. Cancellation for negative arguments.** For `x` large and negative, `x + sqrt(x^2 + 1)` is a
difference of two nearly equal magnitudes, and the relative accuracy of the sum collapses; the
logarithm then magnifies whatever survived. The remedy is oddness: compute on `|x|` and apply the
sign at the end. This costs nothing and additionally makes `ASINH(-x) = -ASINH(x)` hold bit for
bit. Any implementation of an odd function that does not do this has a symmetry defect that is
trivially detectable.

**2. Cancellation for small arguments.** For small `x`, `x + sqrt(x^2 + 1)` is near `1`, and `ln`
of a value near `1` loses exactly the digits that carry the answer. The remedy is `log1p`, applied
to a rearranged argument:

    asinh x  =  log1p( x + x^2 / (1 + sqrt(1 + x^2)) )

Here nothing near `1` is ever formed and nothing is subtracted. Higham's treatment of `log(1+x)`
is the standard statement of why this rearrangement is necessary rather than merely tidy. Below a
threshold the series `x - x^3/6` — or simply `x` itself — is correctly rounded and cheaper.

**3. Overflow in the intermediate for large arguments.** Described above. The remedy is a branch:
once `x` is large enough that `x^2 + 1` would round to `x^2` anyway,

    asinh x = ln(x) + ln(2)

computed without ever forming `x^2`. This extends the working range from the square root of the
largest finite double all the way to the largest finite double itself — a hundred-odd decades
recovered for the cost of one comparison.

`fdlibm`'s `s_asinh.c` is the published implementation of exactly this three-branch structure, and
it is what "a careful implementation" means for this function.

**Why the naive form survives in the wild.** It is one line, it is correct on the range people
actually use, and its failure mode is an error rather than a wrong number. That is a defensible
engineering position and it is very likely how the behaviour on record came to exist. The Handbook
records it as a departure from the mathematics without implying carelessness.

**What a compatibility implementation must therefore decide.** An `excel-bitexact` flavour has to
reproduce the boundary, and the only way to place it exactly right is to form the same intermediate
in the same precision rather than to compare against a decimal constant — which is what the
reference engine does. A `natural-best` flavour should use the three-branch form and will
consequently *disagree* with Excel on that whole upper band, by returning a number where Excel
returns an error. Both are correct implementations of different specifications; the four-flavour
framework exists for precisely this situation. See
[About implementation options](../model/07-implementation-options.md).

## What has not been checked

The evidence attached to this page is **`EV-MATH-0009`**, an `excel-math-deviation` record whose
subject is `FUNC.ASINH`. What it establishes is a **mechanism**: the evaluation form and the
overflow boundary it implies. What it does not establish is any level of agreement, because — in
its own words — it publishes no count at all, and its reader warning says so explicitly. **Nothing
about numeric agreement for `ASINH` follows from it**, and this page makes no such claim.

**No Handbook vector suite exists for `ASINH`.** Beyond the identified boundary, the Handbook has
not observed this function in Excel. In particular the accuracy of the *ordinary* range — small
arguments, moderate arguments, the near-zero band where `log1p` matters — is entirely unmeasured
here.

The presence projection records that this surface's module appears in the math-deviation catalogue
and in no defect stream. The documented statements above come from Microsoft's Learn
`WorksheetFunction.Asinh` reference, which was retrieved; Microsoft's worksheet article was not
(HTTP 403).

Probes worth running first:

1. **Bisect the overflow boundary.** Find the exact largest argument returning a number. Comparing
   it against the square root of the largest finite double is a one-bit test of whether the
   intermediate really is `x*x`, and it is the probe that would confirm or refute the identified
   substrate independently.
2. **Small arguments across many decades**, from `1e-1` down to the subnormal floor, against a
   high-precision reference. This is the `log1p` probe and it is completely unmeasured. A naive
   implementation fails here visibly and a good one does not.
3. **`ASINH(x) + ASINH(-x)`**, which must be exactly zero for every `x` if the sign is folded.
4. **`ASINH(SINH(t))` across `t`** — the exact round trip, needing no oracle, and the fastest way
   to find the small-argument defect if there is one.
5. **`ASINH(x)` against `LN(x + SQRT(x^2 + 1))`** computed on the worksheet, at small `x` — a
   metamorphic probe that separates the two candidate algorithms without any external reference.
6. **The band between the square root of the largest finite double and the largest finite double**,
   to confirm the error is uniform across it rather than appearing at some other place.
7. **Array arguments spanning the boundary**, to confirm the element-local error policy.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| intermediate overflow | An overflow in a step of a formula whose final result is representable |
| overflow band | The arguments whose `asinh` is representable but whose `x^2` is not |
| `log1p` | The primitive computing `ln(1 + t)` accurately for small `t`; not exposed as an Excel function |
| substrate | The evaluation form an implementation actually uses, as distinct from the mathematics it computes |
| exact oddness | `f(-x) = -f(x)` holding bit for bit, obtained by computing on `|x|` and restoring the sign |

## Sources

- Microsoft Learn, "WorksheetFunction.Asinh method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.asinh> (retrieved: the
  description, the **"any real number"** argument statement, and the round trip; **no error
  condition is stated there**).
- Microsoft, "ASINH function" —
  <https://support.microsoft.com/en-us/office/asinh-function-4e00475a-067a-43cf-926a-765b0249717c>
  — the canonical worksheet article. **Not retrieved for this pass** (HTTP 403).
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, chapter 4 section 4.6 — the closed
  form and series.
- `fdlibm` `s_asinh.c` — the published three-branch reference implementation.
- Higham, *Accuracy and Stability of Numerical Algorithms*, chapter 1 — the `log(1+x)` problem.
- Muller, *Elementary Functions: Algorithms and Implementation* — spurious intermediate overflow
  and its avoidance.
- Handbook evidence record `EV-MATH-0009` (subject `FUNC.ASINH`) — the identified substrate and
  overflow boundary, with its own statement that **no count is published**.
- Handbook projections `data/functions/FUNC.ASINH.json` and `data/presence/FUNC.ASINH.json`.
- OxFunc `crates/oxfunc_core/src/functions/asinh.rs` at commit `473efa3` — the kernel that forms
  the same `x*x` intermediate in order to place the boundary without a guessed constant.
- Handbook [About implementation options](../model/07-implementation-options.md) — why an
  Excel-compatible and a mathematically-best implementation must disagree on the overflow band —
  and [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Sibling page [ACOSH](FUNC.ACOSH.md) — the same hazard, treated differently.
