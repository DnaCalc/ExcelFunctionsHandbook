---
schema: efh.function-page/v1
function_id: FUNC.PRODUCT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft — WorksheetFunction.Product method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.product"
    role: "the documented direct-argument versus array/reference asymmetry, stated in two sentences"
  - work: "Microsoft Support — PRODUCT function"
    locator: "https://support.microsoft.com/en-us/office/product-function-8e6b5b24-90ee-4650-aeec-80982a0512ce"
    role: "the worksheet-surface documentation page named in the projection; not retrievable for this pass"
  - work: "Higham, Accuracy and Stability of Numerical Algorithms"
    locator: "chapter 3 (basic error analysis); chapter on products and scaling"
    role: "the error bound for a sequential product and the intermediate-overflow problem"
  - work: "OxFunc — EXCEL_MATH_DEVIATION_CATALOG.md"
    locator: "docs/EXCEL_MATH_DEVIATION_CATALOG.md entry XMD-008"
    role: "the observed rule that Excel converts a non-finite real result to an error value"
episodes: []
body_sections:
  - "What it computes"
  - "Arguments"
  - "Result and edge cases"
  - "Errors"
  - "Relationships"
  - "Numerical notes"
  - "What has not been checked"
  - "Page vocabulary"
  - "Sources"
family: product
role_in_family: "The multiplicative reduction: SUM's counterpart, with the same two-policy admission rule and a very different overflow profile."
---

# PRODUCT

## What it computes

`PRODUCT(number1, [number2], ...)` multiplies its admitted arguments and returns the product.

    PRODUCT(a₁, …, a_k) = a₁ · a₂ · … · a_k

Mathematically the interesting part is the **empty case**. The empty product is 1 — that is the
convention that makes `∏` associative, makes `x⁰ = 1`, and makes the multiplicative identity the
identity. Excel does not use it. When nothing numeric is admitted, the reference engine returns
`0`, following the spreadsheet convention that an aggregate over nothing is zero, the same
convention `SUM` uses where it happens to coincide with the mathematics. `PRODUCT` is where the
two conventions come apart, and it is worth knowing which one you are getting: a product over an
empty selection collapses your formula to zero rather than leaving it unchanged.

Everything else about `PRODUCT` is admission policy, not arithmetic.

## Arguments

`number1` is required; up to 254 further arguments are optional and repeat. The projection records
an arity of 1 to 255. (Microsoft's VBA page shows 30 slots, which is the pre-2007 limit; the
worksheet surface takes more.)

The documented admission rule is the flagship asymmetry of
[Coercion and lifting](../model/02-coercion-and-lifting.md), and Microsoft's VBA page states both
halves of it in consecutive sentences:

> Arguments that are numbers, logical values, or text representations of numbers are counted;
> arguments that are error values or text that cannot be translated into numbers cause errors.

> If an argument is an array or reference, only numbers in the array or reference are counted.
> Empty cells, logical values, text, or error values in the array or reference are ignored.

So the same value multiplies or vanishes depending on how it arrived:

| Value | Typed directly | Reached through a range or array |
|---|---|---|
| `2` | multiplies | multiplies |
| `TRUE` | multiplies as 1 | ignored |
| `"3"` | multiplies as 3 | ignored |
| `"abc"` | causes an error | ignored |
| empty cell | — | ignored |
| `#N/A` | causes an error | **documented as ignored** |

The projection classifies the surface `AggregateDirectAndRangeDualPolicy`, which is the machine
name for exactly that two-column table.

## Result and edge cases

Returns `Number`.

- **No admitted values** → `0`, as above, not the mathematical empty product `1`.
- **A single admitted zero** → `0`, and it wins over everything, including values that would
  otherwise overflow.
- **Overflow.** The projection declares `real_result_policy … non_finite=allow` for this surface:
  a non-finite result is permitted to publish. OxFunc's own catalogue entry **XMD-008** records
  live Excel doing the opposite — converting every non-finite real result to `#NUM!`. `PRODUCT`
  is not in XMD-008's named function list, so the Handbook records this as an **unresolved
  divergence between the reference engine's declared axis and the catalogue's general statement
  about Excel**, not as a settled fact about either. `PRODUCT(1E300, 1E300)` is the one-cell probe.
- **Arrays.** `PRODUCT` is not a lift kernel; array arguments are consumed by scanning, under the
  right-hand column of the table above. It never returns an array.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | A **directly typed** argument is text that does not read as a number | Microsoft VBA page |
| propagated | A **directly typed** argument is an error value | Microsoft VBA page ("cause errors") |
| `#VALUE!` | Fewer than one or more than 255 arguments | arity, refused at entry |

The documentation's claim that error values **inside an array or reference are ignored** deserves
to be flagged rather than repeated quietly. It sits badly beside two other things in this
Handbook: the shared coercion chapter's rule that an error in a scanned range does surface from
`SUM` (skipping errors is not part of the ignore-text scan policy), and the reference engine's
declared `error_collapse_profile: ReductionFold` for `PRODUCT`, which is the classification of a
reduction that *folds* competing errors rather than one that discards them. **Documentation and
reference-engine classification disagree here, and the Handbook has not observed Excel to break
the tie.** `=PRODUCT(A1:A3)` with `#N/A` in `A2` is the whole experiment.

## Relationships

- **`SUM`** — the additive twin, with the same dual-policy admission and none of the overflow
  drama. Where `SUM`'s empty case agrees with the mathematics, `PRODUCT`'s does not.
- **`SUMPRODUCT`** — despite the name, not a relative: it multiplies *across* arrays elementwise
  and then sums, and it applies its own coercion rules.
- **`FACT`**, **`FACTDOUBLE`**, **`MULTINOMIAL`** — closed forms for products that `PRODUCT` would
  compute term by term, and better conditioned for it.
- **`POWER`** — `PRODUCT(x,x,x)` and `POWER(x,3)` are the same mathematics by different
  algorithms; see [POWER](FUNC.POWER.md) on the integer-exponent path.
- **`AGGREGATE`** — the route to a product-like reduction that skips errors *by declaration*
  rather than by an undocumented scan rule. `AGGREGATE` has no product option, which is itself
  worth knowing before you plan around it.
- **`EXP(SUM(LN(range)))`** — the classical workaround for overflow, valid only for strictly
  positive data, and less accurate than a scaled product.

## Numerical notes

**A product is a left fold, and the order is part of the answer.** Floating-point multiplication is
commutative but not associative, so `(a·b)·c` and `a·(b·c)` can differ in the last bit. The
reference engine accumulates strictly left to right in argument order, then in row-major order
within each array — which is the only defensible choice for reproducibility, and worth knowing
before anyone "optimises" a product with a parallel reduction.

**The error bound is benign; the range is not.** For `k` factors the relative error of a sequential
product is bounded by roughly `(k−1)·u/(1−(k−1)u)` — linear in `k`, with no cancellation term,
because multiplication has no catastrophic cancellation (Higham, *ASNA*, ch. 3). Products are
*accurate*. What they are not is *safe*: the intermediate can overflow or underflow even when the
final result is perfectly representable. `1e300 · 1e300 · 1e-300` is `1e300` mathematically and
`+∞` or `#NUM!` by naive evaluation, and the same happens in the underflow direction, where the
loss is silent — a subnormal intermediate quietly discards bits and the final value comes back
wrong rather than erroring.

**The standard remedies**, in increasing order of effort:

1. Accumulate the exponent separately: multiply mantissas, track a scale factor, and reassemble
   once at the end (`frexp`/`ldexp` style). This makes intermediate range failures impossible
   while keeping the same rounding sequence as the naive fold on the mantissa.
2. Sort or pair the factors so that large and small alternate — cheap, effective, and it changes
   the result bits, which for a compatibility-targeting implementation is a reason not to.
3. Work in logarithms only if the data is strictly positive and accuracy is not critical; the
   round trip through `ln` and `exp` costs far more accuracy than the fold ever did.

A `natural-best` implementation should take route 1. An implementation targeting Excel
compatibility should take none of them until Excel's own intermediate behaviour is pinned, because
each of them changes which inputs produce an error.

## What has not been checked

No Handbook vector suite exists for `PRODUCT`, and no evidence record in
`content/evidence/records/` lists this surface among its subjects. The presence projection records
no upstream defect stream touching this module. Nobody has checked this function against Excel
within the Handbook's record.

Inputs I would probe first:

1. **`PRODUCT(A1:A3)` with an error value in the range**, against `PRODUCT(1, #N/A, 3)` typed
   directly. This is the documentation-versus-classification conflict above, and one pair of cells
   settles it.
2. **`PRODUCT(A1:A3)` on an empty range, and `PRODUCT("")`** — whether the no-admitted-values case
   really returns zero, and whether it returns zero or an error when the emptiness is total.
3. **`PRODUCT(1E300, 1E300)` and `PRODUCT(1E300, 1E300, 1E-300)`** — overflow at the end versus
   overflow in the middle. If Excel returns `#NUM!` for the first and a finite value for the
   second, it is doing something more careful than a naive fold, which would be a substantial
   finding.
4. **`PRODUCT(1E-300, 1E-300, 1E300)`** — the underflow-in-the-middle case, which fails silently
   rather than loudly and is therefore the more dangerous of the two.
5. **`PRODUCT(TRUE, "3")` against `PRODUCT(A1:A2)` with the same two values in cells** — the dual
   policy, in one line each.
6. **Argument-count behaviour beyond 255**, which the VBA page (written against the 30-argument
   era) cannot answer.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| dual policy | The rule that direct arguments and scanned ranges coerce by different rules |
| empty product | The mathematical convention that a product over nothing is 1 |
| left fold | Sequential accumulation in argument order, which fixes the rounding sequence |
| intermediate overflow | Range failure in a partial product whose final value is representable |

## Sources

- Microsoft, "WorksheetFunction.Product method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.product> (both
  admission sentences quoted above, including the claim that errors inside an array or reference
  are ignored).
- Microsoft Support, "PRODUCT function" —
  <https://support.microsoft.com/en-us/office/product-function-8e6b5b24-90ee-4650-aeec-80982a0512ce>
  (retrieval refused with HTTP 403 during this pass; nothing here is sourced from it).
- OxFunc `docs/EXCEL_MATH_DEVIATION_CATALOG.md` entry XMD-008 — the observed no-infinities rule
  this surface's declared axis does not follow.
- Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd ed., chapter 3 — the sequential
  product error bound and the scaling remedy.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — the direct-argument
  versus range-scan asymmetry and the error-propagation rule.
- Handbook projections `data/functions/FUNC.PRODUCT.json`
  (`AggregateDirectAndRangeDualPolicy`, `ReductionFold`, `numerical_reduction_policy=
  SequentialLeftFold`, `error_algebra=CanonicalExcelLegacy`, `non_finite=allow`) and
  `data/presence/FUNC.PRODUCT.json` (own module, no shared surfaces, no defect streams).
