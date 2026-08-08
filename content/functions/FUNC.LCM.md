---
schema: efh.function-page/v1
function_id: FUNC.LCM
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
  - Numerical notes
  - What has not been checked
  - Page vocabulary
  - Sources
family: lcm_fn
role_in_family: >-
  Sole member of its module; the join operation of the divisibility lattice, and the one
  member of the integer pair whose documented failure ceiling is on the result rather than
  on any argument.
---

## What it computes

`LCM(number1, [number2], ...)` returns the least common multiple of its arguments after each
has been truncated to an integer.

For positive integers *a*, *b*, lcm(*a*, *b*) is the smallest positive integer that both
divide. As with gcd, the *n*-argument function is the associative fold:

    lcm(a, b, c) = lcm(lcm(a, b), c)

The structural facts worth carrying:

- **lcm(*a*, 0) = 0**, and lcm(*a*, 1) = *a*. Zero is the absorbing element of the join, one is
  its identity — exactly the mirror of gcd, where zero is the identity.
- **The two-argument identity** gcd(*a*, *b*) · lcm(*a*, *b*) = *a* · *b* holds for
  non-negative integers. **It does not generalize to three or more arguments**: the product of
  gcd and lcm of a triple is not the product of the triple. The correct three-argument identity
  runs through the inclusion–exclusion of prime exponents, not through a single product, and
  every attempt to compute a three-way lcm as "product over gcd" is wrong. This is the single
  most common error in worksheet code that reimplements `LCM`.
- **In prime-exponent form**, lcm takes the pointwise maximum of the exponent vectors and gcd
  the pointwise minimum. That is the whole theory: gcd is the meet, lcm the join, of the
  divisibility lattice.
- Microsoft's page gives the practical motivation: the least common multiple is what you need to
  add fractions with unlike denominators.

Domain: non-negative integers after truncation. Range: non-negative integers.

## Arguments

| Argument | Meaning | Required |
|---|---|---|
| `number1` | The first value. | Yes |
| `number2`, … | Further values. | No |

Microsoft's page states the count as "1 to 255 values" and the truncation rule as: "If any
value is not an integer, it is truncated." The declared arity in the projection agrees.

Truncation happens before the arithmetic. `LCM(2.9, 3)` is the lcm of 2 and 3. On the
admissible non-negative domain, truncation and floor agree, so the negative-number ambiguity
that separates truncation from flooring elsewhere in this category cannot arise here.

Argument slots are numeric and subject to ordinary to-number coercion
([coercion and lifting](../model/02-coercion-and-lifting.md)). Range and array arguments are
scanned and reduced to a single result; `LCM` is declared with a custom coercion profile, not
the scalar-lift profile, so it does not apply elementwise.

## Result and edge cases

Returns `Number` — a non-negative integer value.

The reference engine's published battery is rendered beside this page by the generator. The
behaviours it exhibits, stated qualitatively:

- **Zero alone** yields zero, consistent with zero as the absorbing element.
- **A logical argument** is converted and accepted; **numeric text** is converted and then
  truncated. Microsoft's page addresses neither case, speaking only of "nonnumeric".
- **An inline array** is reduced to one result rather than lifted elementwise.
- **An empty range** produces `#VALUE!`.
- **A subnormal magnitude** truncates to zero, and zero absorbs — so a single tiny positive
  argument turns the whole result into zero. That is mathematically right and practically
  surprising, and it is the failure mode to watch for when `LCM` is fed computed values rather
  than typed constants.
- **The largest finite double** does not produce `#NUM!` in the reference engine, although
  Microsoft documents `#NUM!` once the result reaches 2^53. See "Errors".

## Errors

As documented on Microsoft's `LCM` page:

| Error | Documented condition |
|---|---|
| `#VALUE!` | "If any argument is nonnumeric, LCM returns the #VALUE! error value." |
| `#NUM!` | "If any argument is less than zero, LCM returns the #NUM! error value." |
| `#NUM!` | "If LCM(a,b) >=2^53, LCM returns the #NUM! error value." |

Error values in any argument propagate under the shared coercion discipline.

Note the wording of the third row and compare it with [GCD](FUNC.GCD.md)'s: `GCD`'s ceiling is
tested on any **parameter**, `LCM`'s on the **result**. The asymmetry is not an editorial
accident — a gcd is bounded above by its smallest argument and can never overflow inputs that
were themselves admissible, whereas an lcm of two modest coprime arguments is their product and
reaches 2^53 quickly. `LCM` is therefore the member of the pair that must test *after* computing,
which means it must either detect the overflow during the fold or compute in a wider type.

**Documented-versus-reference-engine divergence, recorded here as a finding.** The reference
engine's published battery row for the largest finite double returns a finite number rather than
the documented `#NUM!`. The Handbook has not observed which behaviour Excel produces. That probe
is first on the list below.

## Relationships

- **[GCD](FUNC.GCD.md)** — the dual operation. Separate modules in the reference engine, and
  separate XLL slots. The two-argument identity gcd·lcm = product is the standard route to
  computing one from the other; the safe form is `a / GCD(a,b) * b`, which divides before
  multiplying and so avoids an intermediate that overflows when the answer would not.
- **[MOD](FUNC.MOD.md)** — the divisibility primitive underneath both.
- **[MULTINOMIAL](FUNC.MULTINOMIAL.md)** — the other variadic exact-integer function in this
  category, sharing `LCM`'s group structural evidence record and its 2^53-scale
  representability problem.
- Readers most often reach for `LCM` for common denominators and for period alignment ("both
  events recur together every `LCM(m, n)` steps"). Both usages sit far below the documented
  ceiling, which is why the ceiling is rarely met in practice and rarely tested.

## Numerical notes

Like `GCD`, `LCM` computes exactly in its intended domain — there is no rounding error to
analyse — and like `GCD` its real hazard is representability. Unlike `GCD`, the hazard is
reached from *inside* the admissible domain, because the lcm of admissible arguments is not
itself guaranteed admissible.

**Compute in the order that does not overflow.** The naive binary step
lcm(*a*, *b*) = *a* · *b* / gcd(*a*, *b*) forms the product first, and that product can exceed
2^53 — or exceed the machine word — even when the final answer is comfortably small. The
standard remedy is to divide first:

    lcm(a, b) = (a / gcd(a, b)) * b

The division is exact because gcd(*a*, *b*) divides *a*, so no precision is lost, and the
largest intermediate is the answer itself rather than the product of the inputs. Any
implementation that forms `a*b` before dividing has a smaller usable domain than the one it
advertises, and its failures appear as silently wrong values rather than as errors when the
arithmetic wraps.

**The fold accumulates.** With many arguments the running lcm grows monotonically, so an
overflow check belongs *inside* the fold, tested after each step, not once at the end. Checking
only the final value cannot detect an intermediate that already wrapped.

**The 2^53 wall.** Above 2^53 consecutive integers are no longer distinct doubles, so an lcm
returned as a binary64 above that point is not the integer it claims to be — it is the nearest
representable neighbour, and it may not even be a multiple of the inputs. Microsoft's documented
`#NUM!` is a refusal to hand back a value whose defining property has stopped holding. That is
the right instinct, and it is worth stating plainly: above the wall the *postcondition* fails,
not merely the precision.

Knuth, *The Art of Computer Programming*, volume 2, section 4.5.2 covers the gcd algorithms
this function's inner loop rests on; the divide-before-multiply discipline is standard advice in
every treatment of exact integer arithmetic.

## What has not been checked

No Handbook vector suite exists for `LCM`; `vectors/` publishes nothing for this function.

One evidence record lists `LCM` among its subjects: **EV-STRUCT-0007**, a structural-admission
resweep against a single named live Excel build. Read it with its own reader warning attached —
the record's figure is a **group** total shared across roughly twenty surfaces, each of which
contributed one or two probe cases, and it may not be rendered as a pass rate for `LCM`. What it
supports is narrow: the array-shape and coercion-placement case or cases `LCM` contributed to
that group matched. It says nothing about `LCM`'s values. `LCM` also appears inside a second
structural record's group membership without being a subject there; that record's counts are not
claimed on this page.

Nobody has checked this function's results against Excel.

Inputs I would probe first:

1. **A pair whose lcm straddles 2^53** — for example two large coprime integers whose product is
   just below the ceiling, and a second pair just above. This locates the documented result
   ceiling, and it is the page's live documentation-versus-implementation contradiction.
2. **A pair whose *product* exceeds 2^53 but whose *lcm* does not** — two large arguments sharing
   a large factor. This is the probe that distinguishes a divide-first implementation from a
   multiply-first one, and it is the single most informative cell on this page: a multiply-first
   implementation fails here while passing everything else.
3. **`LCM(0)` and `LCM(0, 5)`.** Zero as the absorbing element. Worth confirming rather than
   assuming, since "least common multiple of zero and five" has a defensible alternative reading
   as an error.
4. **`LCM(TRUE)` and `LCM("2.5")`.** Undocumented conversions that the reference engine accepts.
5. **`LCM(A1)` with `A1` blank, and an empty range**, against the reference engine's `#VALUE!`.
6. **Many arguments whose running lcm crosses the ceiling mid-fold** — say a dozen small primes.
   Whether the error appears depends on where the check sits, and this probe distinguishes an
   in-fold check from a final-value check.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| join | The least-upper-bound operation of the divisibility lattice; here, lcm |
| absorbing element | A value that forces the result regardless of the others; zero, for lcm |
| divide-before-multiply | Computing `a/gcd(a,b)*b` so the largest intermediate is the answer |
| the 2^53 wall | The magnitude above which consecutive integers are no longer distinct doubles |
| group figure | A count measured across several surfaces jointly; never a per-function rate |

## Sources

- Microsoft, "LCM function" —
  <https://support.microsoft.com/en-us/office/lcm-function-7152b67a-8bb5-4075-ae5c-06ede5563c94>.
  Retrieved for this page: the syntax, the "1 to 255 values" count, the truncation rule, the
  `#VALUE!` nonnumeric remark, the `#NUM!` negative remark, the `#NUM!` on the result at 2^53,
  and the common-denominator motivation.
- Handbook evidence record `EV-STRUCT-0007` — group structural-admission resweep at one named
  Excel build; carries a reader warning against per-surface rates.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — to-number outcomes,
  the aggregate-versus-lift distinction, error propagation.
- D. E. Knuth, *The Art of Computer Programming*, volume 2, section 4.5.2 — gcd algorithms and
  exact integer reduction.
- `data/functions/FUNC.LCM.json` — identity, signature, arity 1–255 and declared axes as
  projected at OxFunc `473efa3`; `data/presence/FUNC.LCM.json` — implementing module, Lean
  companion, and the `BUG-FUNC-028` defect stream.
