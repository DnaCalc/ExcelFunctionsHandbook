---
schema: efh.function-page/v1
function_id: FUNC.POISSON
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it does
  - Try it
  - Quirks
  - How to implement
  - Version differences
  - Alternatives
  - Sources
family: null
role_in_family: null
---

# POISSON

## What it does

`POISSON(x,mean,cumulative)` evaluates a Poisson event count with the supplied expected count mean. FALSE returns the mass for exactly x events; TRUE returns the probability of zero through x events, inclusive. POISSON is the compatibility spelling. The mathematical mass at nonnegative integer k is `exp(-mean)*mean^k/k!`.

## Try it

`=POISSON(0,2,FALSE)` is about `0.1353352832`, the probability of no events when two are expected. The cumulative call at zero has the same value. `=POISSON(4,3,FALSE)` is about `0.1680313557`; the corresponding cumulative call also includes counts zero through three.

## Quirks

The count is truncated, but an original negative fraction is rejected before it can become zero. The mean must be nonnegative in Microsoft’s documented domain. Nonnumeric count or mean gives `#VALUE!`.

Mean zero is a point mass: zero events has probability one and positive counts have mass zero. For a positive mean, a cumulative result includes the requested count. “More than k” therefore uses `1-CDF(k)`, while “at least k” uses the CDF at k-1.

The current k=0 branch runs before its negative-mean check and can return EXP of a positive number for that otherwise invalid argument. This is the code’s branch order, separate from Microsoft’s stated negative-mean error. It also retains a subnormal exponential where the positive-count routes flush below normal.

## How to implement

Use [D, E and R](../model/09-staged-arithmetic.md#retain-an-extended-intermediate-when-the-recipe-requires-it) for binary64 storage, resident 64-bit-significand arithmetic and a 64-bit operation followed by a binary64 store. The following finite-input recipe describes the newer cited implementation.

1. Reject an original negative count before truncation. Set k to its truncated value. If k=0, return the measured public `EXP(-mean)` result before the positive-count mean checks. This route retains its subnormal exponential.
2. For k>0 reject a negative mean. At mean zero, return zero for a mass or one for a cumulative probability.
3. For cumulative probability evaluate the generic upper gamma ratio `Q(k+1,mean)`. It is the sum through k, despite being an upper gamma ratio. This avoids starting a recurrence at a mass that has already underflowed.
4. For a mass with k=1 or `mean<1.1`, form `t=R(k*LN(mean))` and obtain the stored internal/public GAMMALN result G at k+1. Subtract mean and G with the compensated-exponential recipe below.
5. For other positive-count masses, compute Stirling correction s(k) and deviance d(k,mean). Store `exponent=R(-s-d)`, obtain stored public EXP(exponent), and divide it with R by `D(sqrt(D(D(2*pi)*k)))`, using the stored binary64 pi. The source uses an ordinary binary64 square root at this normalizer.
6. The current positive-count publisher flushes subnormal results to zero and rejects nonfinite results. Keep this policy distinct from the k=0 return.

### Exponential of two subtractions

Start `a=t`. For each term in the ordered list `[mean,G]`, form the resident difference `e=E(a-term)`, store `a=D(e)`, store `lost=D(E(e-a))`, and keep `factor=R(1+lost)`. After both subtractions, obtain public EXP(a), then multiply successively by the two factors using R at each multiplication. A single call to EXP of an algebraically combined difference loses these stores and corrections. GAMMALN’s complete inner approximation remains a separate open leaf.

### Stirling correction

For integer k=0 through six, the current table is:

| k | Stored decimal coefficient |
|---:|---:|
| 0 | 0 |
| 1 | 0.0810614667953272 |
| 2 | 0.0413406959554092 |
| 3 | 0.0276779256849983 |
| 4 | 0.0207906721037650 |
| 5 | 0.0166446911898211 |
| 6 | 0.0138761288230707 |

Convert each decimal to nearest binary64. The source describes these as values cut to sixteen decimal places; its observed k=4 value is one representable step above the table entry and remains unexplained.

For k>=7, first round each of these ten coefficients to binary64, in ascending power order:

```text
1/12, -1/360, 1/1260, -1/1680, 1/1188,
-691/360360, 1/156, -3617/122400, 43867/244188, -174611/125400
```

Form `u=E(1/E(k*k))`. Starting with the last coefficient, run Horner with resident multiplication and addition, `v=E(E(v*u)+coefficient)`. Return `D(E(v/k))`. Intermediate polynomial values are retained in E; storing each one to binary64 is a different model.

### Deviance

The mathematical quantity is `d(x,m)=x*ln(x/m)+m-x`. Outside the current series condition `abs(D(x-m))<D(0.2*D(x+m))`, form `ratio=R(x/m)`, obtain stored LN(ratio), then return `R(R(x*LN(ratio))+R(m-x))`.

Inside that condition, the current finite reconstruction is:

```text
delta = R(x-m)
v = E(delta / R(x+m))
w = E(v*v)
power = v; tail = 0; j = 1
repeat:
    power = E(power*w)
    next = E(tail + E(power/(2*j+1)))
    if next == tail: stop
    tail = next; j = j+1
V = D(v)
S = D(E(V+tail))
twice_x = D(2*x)
A = E(R(m-x) + R(twice_x*S))
A = E(A - R(twice_x*V))
A = E(D(A) + delta)
return D(E(A + R(delta*V)))
```

The series condition is the implementation’s floating comparison. Its mathematical ratio interval is `(2/3,3/2)`, wider than the published starting method’s interval. Do not change the threshold or intermediate stores solely from that algebraic equivalence.

Worked mass at k=4, mean three: the deviance reconstruction gives `0.15072828980712366`; the table supplies s(4) about `0.020790672103765`. The stored exponent is about `-0.17151896191088867`, and the normalizer is the binary64 square root of `8*pi`. This recipe gives about `0.16803135574154085`. These are worked reconstruction values, rather than additional Excel captures.

The mathematical starting point is Loader’s published saddle-point method; the project then isolates arithmetic sites from recorded answers. The source records 1,000 of 1,000 series controls at x=100 and 1,199 of 1,200 at x=1024. The latter remaining disagreement, the small-table exception, GAMMALN and other report failures are still open. The 1.1 split is recorded at x=2,3,7 and does not establish every global route boundary. The separately pinned interactive calculator can predate these later stages. No published Handbook suite establishes whole-function agreement.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[BINOM.DIST](FUNC.BINOM.DIST.md) models a fixed number of independent trials. [CHISQ.DIST.RT](FUNC.CHISQ.DIST.RT.md) exposes the related gamma-tail family; [EXP](FUNC.EXP.md) supplies the zero-count stage.

## Sources

- [Microsoft: POISSON function](https://support.microsoft.com/en-us/excel/functions/poisson-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [R31 discrete distribution implementation](https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/crates/calc-func/src/ported/discrete_dist.rs); guards, route split, Stirling table, deviance and mass composition.
- [Resident series and compensated exponential](https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/crates/calc-func/src/ported/excel_numeric/x87_chains.rs); explicit operation and store order.
- [Loader, Fast and Accurate Computation of Binomial Probabilities](https://www.r-project.org/doc/reports/CLoader-dbinom-2002.pdf), opened 6 November 2026; published saddle-point starting method and Poisson extension. The implementation’s measured stages are a separate claim.
- [Remaining numerical failures](https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/docs/generated/problem-functions.md); source-specific counts, including still unresolved Poisson answers.
