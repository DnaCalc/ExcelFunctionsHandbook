---
schema: efh.function-page/v1
function_id: FUNC.HYPGEOM.DIST
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
  - Notes
  - Sources
family: null
role_in_family: null
---

# HYPGEOM.DIST

## What it does

`HYPGEOM.DIST(sample_s, number_sample, population_s, number_pop, cumulative)` models a fixed-size sample drawn **without replacement** from a finite population. Every subset of the sample size has equal probability. FALSE returns the mass at exactly `sample_s` successes; TRUE returns the probability of at most that many.

Write the four counts as `k,n,K,N`: observed successes, sample size, population successes and population size. The success probability changes after each draw because the selected object is not replaced. A binomial model with one fixed probability answers a different sampling question.

## Try it

`=HYPGEOM.DIST(1,2,3,5,FALSE)` is about `0.6`. Drawing two objects from five, three marked successful, yields one success in `choose(3,1)*choose(2,1)=6` of `choose(5,2)=10` possible samples.

## Quirks

The feasible mathematical success interval is `max(0,n-(N-K))..min(n,K)`. Microsoft’s opened remarks specify `#NUM!` outside that interval and for nonpositive sample size, population success count or population size. Nonnumeric count arguments give `#VALUE!`.

Recorded build 20430 controls depart from those stated guards. `HYPGEOM.DIST(0,0,0,5,0)` returned one. With successes above the support, `HYPGEOM.DIST(5,4,2,5,0)` returned zero, while changing the cumulative flag from zero to one returned one. [FN-HYPGEOM.DIST-001](../notes/FN-HYPGEOM.DIST-001.json) retains all four calls, exact inputs and provenance.

The current code checks original negative signs before truncating the counts, so a negative fraction must not become an admitted zero merely through truncation. Its support shortcuts and mass body need separate tests; neither the probability definition nor the public error list settles every recorded numerical edge.

## How to implement

For a mathematical implementation, count the supported selections:

```text
mass(k) = choose(K,k) * choose(N-K,n-k) / choose(N,n)
```

The sample `k=1,n=2,K=3,N=5` has six favourable subsets out of ten, giving probability `0.6`. A recurrence between adjacent supported masses is a way to avoid constructing huge factorials, but it is an alternative algorithm.

The current compatibility candidate has a more specific composition:

1. Reject original negative counts, then truncate all four toward zero. Reject `n>N` or `K>N`.
2. For a mass, return zero for an unsupported `k`. Handle the degenerate `n=0` and `n=N` cases explicitly.
3. Otherwise form the binary64 values `p=n/N` and `q=(N-n)/N` separately. Do not assume the stored `q` is interchangeable with `1-p`.
4. Obtain shared binomial masses `a=b(k;K,p,q)`, `b=b(n-k;N-K,p,q)` and `c=b(n;N,p,q)`. Return `(a*b)/c` using the current binary64 multiplication and division order.
5. For cumulative calls, the current code returns one immediately when `k>=min(n,K)`; otherwise it adds masses from zero through `k` and caps the sum at one.

The shared binomial body, small-tail choices and publication behaviour retain unresolved differences in the problem report. The generated fingerprint report says no complete hypergeometric algorithm is adopted. Treat this as a codeable current candidate, with the named recorded endpoint controls, rather than a whole-domain Excel identification.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 3,653 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

Concrete controls from the 29 September 2026 build 20430 capture:

| Captured call | Excel result |
|---|---:|
| `HYPGEOM.DIST(0,0,0,5,0)` | 1 |
| `HYPGEOM.DIST(0,0,0,5,1)` | 1 |
| `HYPGEOM.DIST(5,4,2,5,0)` | 0 |
| `HYPGEOM.DIST(5,4,2,5,1)` | 1 |

Inputs were written as exact doubles through bulk `Value2` cells. The capture records Windows 64-bit and Compatibility Version 2; it does not record CPU. These controls do not compare builds.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.HYPGEOM.DIST.json).

## Alternatives

[HYPGEOMDIST](FUNC.HYPGEOMDIST.md) is the mass-only compatibility spelling. [BINOM.DIST](FUNC.BINOM.DIST.md) describes independent trials with one fixed probability, and [COMBIN](FUNC.COMBIN.md) counts selections.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `HYPGEOM.DIST` that an implementer matching Excel, or a reader explaining
an unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's
answer and where it was recorded; "measured" means the statement is read off recorded Excel
answers, "inferred" marks what goes beyond them. Where older text on this page says a behaviour
was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

### FN-HYPGEOM.DIST-001: Recorded hypergeometric calls admit zero counts and counts above the support

*Documentation differs from behaviour; measured; recorded 2026-11-06.*

Four retained build 20430 controls return numeric values where the opened documentation
specifies #NUM!: HYPGEOM.DIST(0,0,0,5,0) and its cumulative form both return 1;
HYPGEOM.DIST(5,4,2,5,0) returns 0 and its cumulative form returns 1. These observations cover
the listed inputs, not every out-of-support call.

Documentation: The opened HYPGEOM.DIST remarks specify #NUM! for a sample size or population
success count at or below zero, and for sample successes above min(sample size,population
successes). (https://support.microsoft.com/en-us/excel/functions/hypgeom-dist-function)

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `HYPGEOM.DIST(0,0,0,5,0)` | sample_s `0` (`0x0000000000000000`); number_sample `0` (`0x0000000000000000`); population_s `0` (`0x0000000000000000`); number_pop `5` (`0x4014000000000000`); cumulative `0` (`0x0000000000000000`) (cells) | `1` (`0x3ff0000000000000`) | Excel 16.0 build 20430, Windows 64-bit (NT 10.00), Workbook Compatibility Version 2; CPU not recorded. Captured 29 September 2026. |
| `HYPGEOM.DIST(0,0,0,5,1)` | sample_s `0` (`0x0000000000000000`); number_sample `0` (`0x0000000000000000`); population_s `0` (`0x0000000000000000`); number_pop `5` (`0x4014000000000000`); cumulative `1` (`0x3ff0000000000000`) (cells) | `1` (`0x3ff0000000000000`) | Excel 16.0 build 20430, Windows 64-bit (NT 10.00), Workbook Compatibility Version 2; CPU not recorded. Captured 29 September 2026. |
| `HYPGEOM.DIST(5,4,2,5,0)` | sample_s `5` (`0x4014000000000000`); number_sample `4` (`0x4010000000000000`); population_s `2` (`0x4000000000000000`); number_pop `5` (`0x4014000000000000`); cumulative `0` (`0x0000000000000000`) (cells) | `0` (`0x0000000000000000`) | Excel 16.0 build 20430, Windows 64-bit (NT 10.00), Workbook Compatibility Version 2; CPU not recorded. Captured 29 September 2026. |
| `HYPGEOM.DIST(5,4,2,5,1)` | sample_s `5` (`0x4014000000000000`); number_sample `4` (`0x4010000000000000`); population_s `2` (`0x4000000000000000`); number_pop `5` (`0x4014000000000000`); cumulative `1` (`0x3ff0000000000000`) (cells) | `1` (`0x3ff0000000000000`) | Excel 16.0 build 20430, Windows 64-bit (NT 10.00), Workbook Compatibility Version 2; CPU not recorded. Captured 29 September 2026. |

Basis (measured): Read directly from four rows of the landed W111 domain capture. Its
provenance names Excel build 20430, Windows 64-bit, Compatibility Version 2, exact-double bulk
cell input and no cache. This harvest made no new Excel calls and did not compare builds.

- Open: Whether every out-of-support and zero-count combination follows the same rule in other builds.

Evidence:

- CalcFunc-Evidence `runs/w111-distribution-domain-20260929/answers-hypgeom.dist.json.gz`, hypgeom.dist-domain-01118-integer-support, hypgeom.dist-domain-01119-integer-support, hypgeom.dist-domain-01404-integer-support, hypgeom.dist-domain-01405-integer-support at `4287ba192d` — Recorded answers and capture provenance.
- DnaCalc `crates/calc-func/src/ported/discrete_dist.rs`, hypergeom_mass and hypergeom_dist_kernel at `da3ecc4fb1` — Current support checks, degenerate masses and cumulative endpoint shortcut.
- CalcFunc-Evidence `runs/w111-distribution-domain-20260929/answers-hypgeom.dist.json.gz`, hypgeom.dist-domain-01118-integer-support at `4287ba192d` — Original capture provenance records cell_value2_bulk input plumbing and no cache use. (readings)
- CalcFunc-Evidence `runs/w111-distribution-domain-20260929/answers-hypgeom.dist.json.gz`, hypgeom.dist-domain-01119-integer-support at `4287ba192d` — Original capture provenance records cell_value2_bulk input plumbing and no cache use. (readings)
- CalcFunc-Evidence `runs/w111-distribution-domain-20260929/answers-hypgeom.dist.json.gz`, hypgeom.dist-domain-01404-integer-support at `4287ba192d` — Original capture provenance records cell_value2_bulk input plumbing and no cache use. (readings)
- CalcFunc-Evidence `runs/w111-distribution-domain-20260929/answers-hypgeom.dist.json.gz`, hypgeom.dist-domain-01405-integer-support at `4287ba192d` — Original capture provenance records cell_value2_bulk input plumbing and no cache use. (readings)

<!-- efh:notes end -->

## Sources

- [Microsoft: HYPGEOM.DIST function](https://support.microsoft.com/en-us/excel/functions/hypgeom-dist-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/discrete_dist.rs); implementation steps and the limits identified in the text.
- [Public recorded domain bank](https://github.com/DnaCalc/CalcFunc-Evidence/blob/4287ba192d2e3a7fd54671ea1cee1c34cb2e84e2/runs/w111-distribution-domain-20260929/answers-hypgeom.dist.json.gz); download and decompress the gzip JSON.
- [Unresolved numerical scope](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/docs/generated/numeric-fingerprints.md) and [problem-function report](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/docs/generated/problem-functions.md).
