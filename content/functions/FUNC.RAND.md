---
schema: efh.function-page/v1
function_id: FUNC.RAND
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft — Excel JavaScript API, Excel.Functions.rand"
    locator: "https://learn.microsoft.com/en-us/javascript/api/excel/excel.functions"
    role: "the documented range and volatility, verbatim: \"a random number greater than or equal to 0 and less than 1, evenly distributed (changes on recalculation)\""
  - work: "Microsoft Support — RAND function"
    locator: "https://support.microsoft.com/en-us/office/rand-function-4cbfa695-8869-4788-8d90-021ea9f5be73"
    role: "the worksheet-surface documentation page named in the projection; not retrievable for this pass"
  - work: "L'Ecuyer & Simard, TestU01: A C library for empirical testing of random number generators"
    locator: "ACM TOMS 33(4), 2007"
    role: "the standard battery any claim about a generator's quality has to survive"
  - work: "McCullough & Wilson, On the accuracy of statistical procedures in Microsoft Excel"
    locator: "Computational Statistics & Data Analysis, series of papers 1999-2008"
    role: "the published external critique of Excel's random number generation across versions"
  - work: "Wichmann & Hill, Algorithm AS 183: An efficient and portable pseudo-random number generator"
    locator: "Applied Statistics 31(2), 1982"
    role: "the combined-congruential generator historically associated with spreadsheet RAND implementations"
episodes: []
body_sections:
  - "What it computes"
  - "Arguments"
  - "Result and edge cases"
  - "Errors"
  - "Relationships"
  - "Numerical notes"
  - "What has not been checked"
  - "Notes"
  - "Page vocabulary"
  - "Sources"
family: rand_fn
role_in_family: "The unit-interval generator; the source every other random surface draws from."
---

# RAND

## What it computes

`RAND()` returns a pseudo-random number in the half-open interval **[0, 1)** — at least zero and
strictly less than one — intended to be uniformly distributed, and recomputed on every
recalculation.

Microsoft's Excel JavaScript API reference states the contract in one sentence: "Returns a random
number greater than or equal to 0 and less than 1, evenly distributed (changes on recalculation)."
Both endpoints matter. The lower end is *attainable* and the upper end is not, which is what makes
`RAND()*(b-a)+a` a correct sampler for `[a, b)` and `1/RAND()` a formula that can divide by zero.

This is the only function in this batch that is not a function. It has no argument, so it has no
input to be a function *of*; the projection classifies it `PseudoRandom`, `VolatileFull`,
`HostSerialized`, with a `RandomProvider` dependency. It is a *stream*, and the value you see is a
position in that stream. Everything difficult about it follows from that.

## Arguments

None. Arity is exactly zero and the parentheses are mandatory syntax.

The absence of a seed argument is the single most consequential design decision on this page:
there is no worksheet-level way to reproduce a draw, and therefore no worksheet-level way to make
a model deterministic. Workbooks that need reproducibility freeze values (paste-as-values) or move
the generator outside Excel.

## Result and edge cases

Returns `Number` in `[0, 1)`.

- **Volatility.** `VolatileFull` means the cell recomputes on every recalculation of the workbook,
  not only when a precedent changes. A sheet full of `RAND()` re-randomises when you edit an
  unrelated cell, press F9, or open the file. This is a property of the *engine's* dependency
  handling, not of the function's mathematics, and it is why a `RAND()` column cannot be used as a
  stable key.
- **Thread serialisation.** `HostSerialized` records that the surface cannot be evaluated freely in
  parallel: a shared stream has to be advanced under a lock, or two threads get the same draw. Any
  implementation that parallelises recalculation has to decide whether the *sequence* of draws is
  part of the observable behaviour. If it is, parallel evaluation changes results; if it is not,
  nothing is reproducible anyway.
- **Zero is attainable in principle.** The documented interval includes 0, so a formula that
  divides by `RAND()` has a genuine — if astronomically unlikely, and generator-dependent — failure
  mode. Whether any real implementation ever emits exactly 0 depends on how the bits are assembled;
  see the numerical notes.
- **The reference engine does not generate anything.** It takes a value from an injected
  `RandomProvider` and validates it: finite and within `[0, 1)`, otherwise a `#VALUE!`-mapped
  failure. So the reference engine pins the *contract* of `RAND` and deliberately does not pin its
  *stream*. The probe battery rendered beside this page reflects that: every row is recorded as not
  dispatchable because the surface is declared non-deterministic.

## Errors

No documented error condition. Nothing in the argument list can fail, because there is no argument
list.

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | Any argument supplied (arity refusal) | arity, refused at entry |
| `#VALUE!` | The host's random provider yields a value outside `[0, 1)` or a non-finite value | reference engine only; not a worksheet-observable condition |

## Relationships

- **[RANDBETWEEN](FUNC.RANDBETWEEN.md)** — the integer sampler, built from the same stream, with
  an inclusive upper bound where `RAND` has an exclusive one. The endpoint difference is the most
  common source of off-by-one sampling errors in spreadsheets.
- **[RANDARRAY](FUNC.RANDARRAY.md)** — the dynamic-array generalisation, which subsumes both:
  `RANDARRAY()` with no arguments is `RAND()`, and `RANDARRAY(1,1,a,b,TRUE)` is
  `RANDBETWEEN(a,b)`.
- **`RAND()*(b-a)+a`** — the documented idiom for a uniform draw on `[a, b)`. It inherits `RAND`'s
  half-open interval, and it loses resolution when `a` and `b` are far apart in magnitude.
- **`INDEX(range, RANDBETWEEN(1, ROWS(range)))`** — sampling *with* replacement. There is no
  built-in sampler without replacement; `SORTBY(range, RANDARRAY(ROWS(range)))` is the modern
  idiom, and it is a permutation only if the draws are distinct.
- **`NORM.INV(RAND(), μ, σ)`** — the inverse-transform route to a normal deviate, and the place
  where the quality of `RAND`'s low-order bits and the accuracy of the inverse CDF in the tails
  both stop being academic.

## Numerical notes

**Resolution.** A generator producing a `[0,1)` double can offer anywhere from 2⁻³² to 2⁻⁵³
spacing. Two constructions dominate: `k/2⁵³` with `k` a 53-bit integer (uniform on a lattice,
never returns exactly 1, can return exactly 0), and `(k+½)/2⁵³` (never returns either endpoint).
A generator built by dividing a 32-bit integer by 2³² gives only about 4×10⁹ distinct values,
which is visible as ties in any sample of a few hundred thousand rows — the birthday bound bites
around 65,536 draws. If a workbook's Monte Carlo shows duplicate draws, this is why.

**Period and equidistribution matter more than "randomness".** A combined multiplicative
congruential generator in the Wichmann–Hill family (AS 183) has a period around 7×10¹² and fails
modern spectral tests; a Mersenne Twister has period 2¹⁹⁹³⁷−1 and 623-dimensional
equidistribution but is not cryptographic and fails linear-complexity tests; a modern counter-based
or `xoshiro`-family generator does better in less state. For a spreadsheet, the practical
questions are (a) how many draws before the sequence repeats, (b) whether consecutive draws are
independent when used as coordinates of a point — the classic lattice failure that makes a pair of
`RAND()` cells trace visible planes in a scatter plot — and (c) whether the low bits are as good
as the high bits. L'Ecuyer & Simard's TestU01 is the standard instrument for all three.

**Streams and parallelism.** Once a generator is shared across cells, the *assignment of draws to
cells* becomes part of the answer. The two disciplined solutions are a single serialised stream
(what `HostSerialized` describes) and per-cell counter-based generation, where the draw is a
keyed function of the cell address and a workbook counter — reproducible, parallel-safe, and
compatible with nothing that already exists.

**On Excel's actual generator, this page says nothing.** Microsoft has characterised it in
different terms across versions, and external authors — McCullough & Wilson most persistently —
have published critiques of specific releases. The Handbook has not tested any Excel build's
generator and does not restate anyone's account of it. What can be said without evidence is only
the contract: half-open, uniform by declaration, volatile.

## What has not been checked

No Handbook vector suite exists for `RAND`, and no evidence record in
`content/evidence/records/` lists this surface among its subjects. The presence projection records
no upstream defect stream touching this module.

`RAND` is also the surface where the Handbook's usual method does not apply, and that deserves to
be said plainly: **a value-comparison suite is impossible for a function with no arguments and no
seed.** There is nothing to compare against a reference implementation. Any evidence for `RAND`
has to be statistical or structural, and none exists here.

What would actually settle something:

1. **Resolution.** Draw a large sample and examine `RAND()*2^53`, `RAND()*2^32` and
   `MOD(RAND()*2^53, 1)` for integrality. This identifies the construction — 32-bit, 53-bit, or
   something else — in one pass, and it is the single most informative experiment available.
2. **Endpoint attainment.** Whether exactly `0` ever appears, and whether `1-RAND()` ever returns
   exactly `1`. The documented interval says the first is possible; most constructions make it
   unreachable in practice.
3. **Repeat distance.** Whether a long column of draws contains duplicates at a rate consistent
   with the birthday bound for the resolution found in (1). Excess duplicates mean lower resolution
   than advertised.
4. **Serial correlation and the lattice test.** Plot consecutive pairs and triples; a
   congruential generator shows planes. This is a five-minute experiment that has embarrassed more
   than one spreadsheet.
5. **Recalculation semantics.** Whether all `RAND()` cells in a workbook advance one shared stream
   or draw independently, and whether the assignment is stable under recalculation order. Two
   `RAND()` cells that ever return the same value in the same recalculation would be decisive.
6. **Cross-platform and cross-build stability.** Whether the same workbook, reopened on a
   different platform, produces a statistically distinguishable stream. Version-to-version changes
   in the generator are exactly the sort of thing that invalidates an archived model, and no
   Handbook record covers it.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `RAND` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-RAND-001: RAND, RANDARRAY, and RANDBETWEEN set PtgAttrSemi syntactically, and RANDARRAY requires _xlfn.

*Quirk; measured; recorded 2026-11-22.* Applies to RAND, RANDARRAY, RANDBETWEEN.

In Excel 16.0.20527, RAND(), RANDARRAY([rows], [columns], [min], [max], [whole_number]), and
RANDBETWEEN(bottom, top) all mark the enclosing formula syntactically volatile (PtgAttrSemi)
regardless of whether the call is in a taken branch, and refuse reference-place or call-postfix
uses at entry. RANDARRAY requires the _xlfn. prefix in OOXML (bare @RANDARRAY gives #NAME?),
whereas RAND and RANDBETWEEN need no prefix; degenerate bounds RANDBETWEEN(1,1) and
RANDARRAY(1,1,1,1,TRUE) deterministically return 1.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=RAND()*0` | as typed | `0.0` (`0x0000000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=@RANDARRAY(1,1,1,1,TRUE)` | as typed | `1.0` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=RANDBETWEEN(1,1)` | as typed | `1.0` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Measured in Excel 16.0.20527.20088 in prefix_metadata (rows 191-193),
excel-entry.tsv (14 entry checks), and volatility probes in VOLATILITY.md.

Evidence:

- DnaCalc `crates/calc-func/src/catalogue/random.rs`, lines 1-82 at `2e9be2b5b7` — RAND, RANDBETWEEN, and RANDARRAY implementations
- DnaCalc `docs/architecture/VOLATILITY.md`, sections 2 and 3 at `2e9be2b5b7` — Syntactic PtgAttrSemi volatility for RAND, RANDBETWEEN, and RANDARRAY
- DnaCalc `crates/calc-fml/tests/fixtures/entry/excel-entry.tsv`, RAND, RANDARRAY, and RANDBETWEEN rows at `2e9be2b5b7` — Formula-entry arity and reference-place checks
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/prefix_metadata.capture.json`, sheet prefix_metadata row 191 column 1 at `2e9be2b5b7` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/prefix_metadata.capture.json`, sheet prefix_metadata row 192 column 1 at `2e9be2b5b7` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/prefix_metadata.capture.json`, sheet prefix_metadata row 193 column 1 at `2e9be2b5b7` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| half-open interval | `[0, 1)`: zero is attainable, one is not |
| volatile | Recomputes on every recalculation, not only when a precedent changes |
| stream | The ordered sequence of draws a generator produces; the shared state behind every cell |
| resolution | The spacing of the lattice of values a generator can actually return |
| equidistribution | How uniformly consecutive draws fill a `k`-dimensional cube |

## Sources

- Microsoft, Excel JavaScript API, `Excel.Functions.rand` —
  <https://learn.microsoft.com/en-us/javascript/api/excel/excel.functions> (the documented range
  and the "changes on recalculation" statement).
- Microsoft Support, "RAND function" —
  <https://support.microsoft.com/en-us/office/rand-function-4cbfa695-8869-4788-8d90-021ea9f5be73>
  (retrieval refused with HTTP 403 during this pass; nothing here is sourced from it).
- L'Ecuyer & Simard, "TestU01", ACM TOMS 33(4), 2007; Wichmann & Hill, Algorithm AS 183, *Applied
  Statistics* 31(2), 1982; McCullough & Wilson, *Computational Statistics & Data Analysis*
  (1999–2008 series on Excel's statistical procedures).
- Handbook, [The value universe](../model/01-value-universe.md),
  [Claim language and honesty](../model/06-claim-language.md).
- Handbook projections `data/functions/FUNC.RAND.json` (`PseudoRandom`, `VolatileFull`,
  `HostSerialized`, `RandomProvider`, `ApplicationState`) and `data/presence/FUNC.RAND.json` (own
  module, no shared surfaces, no defect streams).
