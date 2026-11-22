---
schema: efh.function-page/v1
function_id: FUNC.RANDBETWEEN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft — WorksheetFunction.RandBetween method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.randbetween"
    role: "documented signature, the bottom/top wording (\"the smallest integer\" / \"the largest integer\"), and the recalculation statement; no error conditions are stated"
  - work: "Microsoft Support — RANDBETWEEN function"
    locator: "https://support.microsoft.com/en-us/office/randbetween-function-4cc7f0d1-87dc-4eb7-987f-a469ab381685"
    role: "the worksheet-surface documentation page named in the projection; not retrievable for this pass"
  - work: "Lemire, Fast random integer generation in an interval"
    locator: "ACM TOMACS 4(1), 2019"
    role: "the bias analysis and the standard unbiased construction for an integer range"
  - work: "L'Ecuyer & Simard, TestU01"
    locator: "ACM TOMS 33(4), 2007"
    role: "the empirical battery for the underlying stream"
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
family: randbetween_fn
role_in_family: "The inclusive integer sampler; RAND's discrete counterpart and the surface where the endpoint convention flips."
---

# RANDBETWEEN

## What it computes

`RANDBETWEEN(bottom, top)` returns a pseudo-random **integer** between `bottom` and `top`,
**inclusive at both ends**, redrawn on every recalculation.

    P(X = k) = 1 / (top − bottom + 1)    for each integer k with bottom ≤ k ≤ top

Microsoft's VBA reference states the endpoints as inclusive without using the word: `bottom` is
"the smallest integer **RandBetween** will return" and `top` is "the largest integer
**RandBetween** will return". The same page states that "a new random integer number is returned
every time the worksheet is calculated".

**The inclusive upper bound is the fact to carry away.** [RAND](FUNC.RAND.md) returns a value in
`[0, 1)` — upper bound excluded. `RANDBETWEEN` includes both bounds. So `RANDBETWEEN(1, N)` picks
one of `N` items, while `INT(RAND()*N)+1` picks one of `N` items by a different route with a
different failure mode, and `RANDBETWEEN(0, N)` has `N+1` outcomes, not `N`. Off-by-one errors in
spreadsheet sampling almost always trace back to mixing the two conventions.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `bottom` | The smallest integer that may be returned. Required. | — |
| `top` | The largest integer that may be returned. Required. | — |

Both are numeric slots under ordinary to-number coercion — see
[Coercion and lifting](../model/02-coercion-and-lifting.md). Microsoft's page types both as
`Variant`, consistent with text and logicals converting.

**Non-integral bounds are not documented.** The VBA page describes both arguments as integers and
says nothing about what happens if they are not. The reference engine narrows inward: it takes
`⌈bottom⌉` and `⌊top⌋`, so `RANDBETWEEN(1.2, 3.8)` samples from `{2, 3}`. Narrowing inward is the
only choice that keeps every returned value inside the stated interval, but it is a choice, and
rounding outward or truncating toward zero are both defensible alternatives that would produce
different samples. Nothing documented settles it.

## Result and edge cases

Returns `Number`, always integral.

- **`bottom = top`** returns that value with probability 1 — a degenerate but legal draw.
- **`bottom > top`** is refused. The reference engine maps it to `#NUM!`. **Microsoft's VBA page
  states no error condition at all for this method**, so the code is undocumented on the source
  this page could read. `#NUM!` is the conventional expectation, and the Handbook has not observed
  it.
- **Non-integral bounds** narrow inward, as above; if `⌈bottom⌉ > ⌊top⌋` — for example
  `RANDBETWEEN(1.2, 1.8)`, where no integer lies in the interval — the same refusal applies. This
  is a case a reader is unlikely to anticipate: an interval that plainly contains numbers contains
  no *integers*, and the function errors.
- **Volatility.** `VolatileFull`, `PseudoRandom`, `HostSerialized`, `RandomProvider` — the whole
  discussion on [RAND](FUNC.RAND.md) applies unchanged. The value is not stable; a
  `RANDBETWEEN` cell cannot be used as a key.
- **Very wide ranges.** When `top − bottom + 1` approaches or exceeds 2⁵³, the arithmetic that
  produces the draw stops being exact; see the numerical notes.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | Either argument does not convert to a number | shared coercion model |
| propagated | An error value in either argument | shared coercion model |
| `#NUM!` | `⌈bottom⌉` exceeds `⌊top⌋` — including the case where the interval contains no integer | reference engine; **not documented on the VBA page** |
| `#VALUE!` | The host's random provider yields a value outside `[0, 1)` | reference engine only |

## Relationships

- **[RAND](FUNC.RAND.md)** — the continuous source. `RANDBETWEEN` is built from it, and inverts
  its endpoint convention.
- **[RANDARRAY](FUNC.RANDARRAY.md)** — with `whole_number` true, `RANDARRAY(1,1,a,b,TRUE)` is the
  same sampler, using the same `⌈min⌉`/`⌊max⌋` normalisation in the reference engine. `RANDARRAY`
  is the surface to reach for when more than one draw is wanted, because a spilled array is one
  volatile object rather than `n` of them.
- **`INT(RAND()*(b-a+1))+a`** — the manual construction. It is exactly what the reference engine
  does internally, with the same bias characteristics, and it is worth writing out once so that the
  bias is visible rather than hidden behind a function name.
- **`INDEX(range, RANDBETWEEN(1, ROWS(range)))`** — sampling with replacement, the idiom this
  function mostly exists to serve.
- **`CRITBINOM`/`BINOM.INV`** — the other route to a discrete draw, via an inverse CDF. Different
  distribution, same dependence on the quality of the underlying uniform.

## Numerical notes

**Uniformity is approximate, and the error is exactly `N/2⁵³`.** The construction
`lo + ⌊u·N⌋`, with `u` uniform on a lattice of `2⁵³` values and `N = hi − lo + 1`, distributes
`2⁵³` lattice points among `N` buckets. If `N` is not a power of two the buckets cannot receive
equal counts, and the largest and smallest bucket probabilities differ by a relative amount of
about `N/2⁵³`. For `RANDBETWEEN(1,6)` that is around `10⁻¹⁵` — undetectable in any experiment that
will ever be run. For `N` above `2⁴⁰` it becomes measurable, and for `N` near `2⁵³` the
distribution is visibly lumpy.

The unbiased alternatives are well known: rejection sampling (redraw when `u` falls in the
incomplete final block) and Lemire's multiply-shift method, which achieves the same at the cost of
one multiplication and a rarely taken branch. Both are strictly better mathematics. Neither may be
substituted into an implementation that is targeting compatibility, because each changes which
draw a given stream position produces — so this is a case where `natural-best` and
`excel-bitexact` must genuinely differ; see
[About implementation options](../model/07-implementation-options.md).

**Two floating-point details that bite at the extremes.** First, `hi − lo + 1` is computed in
binary64: when `hi` and `lo` straddle zero at large magnitude the subtraction can overflow, and
when the span exceeds `2⁵³` the `+1` is silently absorbed. Second, `u·N` can round *up* to exactly
`N` when `u` is the largest value below 1 and `N` is large, which would place the draw one past
`hi`; the reference engine guards this with an explicit clamp, and any implementation that omits
the clamp has a rare out-of-range bug that is essentially untestable by sampling.

**A wide range does not give a wide sample.** Because the source lattice has `2⁵³` points, a
`RANDBETWEEN` over a range wider than the generator's resolution cannot reach every integer in the
range — it reaches at most as many distinct values as the generator has. If the underlying stream
is 32-bit, that ceiling is around four billion regardless of the bounds requested. Anyone using
`RANDBETWEEN(1, 10^15)` as an identifier generator should know this.

## What has not been checked

No Handbook vector suite exists for `RANDBETWEEN`, and no evidence record in
`content/evidence/records/` lists this surface among its subjects. The presence projection records
no upstream defect stream touching this module. The probe battery rendered beside this page reports
every row as not dispatchable, because the surface is declared non-deterministic.

As with [RAND](FUNC.RAND.md), a value-comparison suite is impossible; but unlike `RAND`, this
surface has *arguments*, and its argument handling is fully checkable even though its output is
not. That is where the probes should go:

1. **`RANDBETWEEN(1.2, 3.8)` sampled many times** — whether the reachable set is `{2,3}` (narrow
   inward), `{1,2,3}` (round outward), or something else. This is the undocumented rule with the
   largest practical consequence.
2. **`RANDBETWEEN(1.2, 1.8)`** — an interval containing no integer. Error, or a value, or which
   value?
3. **`RANDBETWEEN(5, 4)`** — the reversed-bounds error code, undocumented on the page this
   Handbook could read.
4. **`RANDBETWEEN(-3, -1)` and `RANDBETWEEN(-1.5, 1.5)`** — negative and straddling ranges, where
   ceiling-and-floor and truncate-toward-zero give different answers.
5. **Endpoint attainment for a small range** — `RANDBETWEEN(1,2)` over a long column, checking
   that both values occur and in roughly equal proportion. The cheapest check that the bounds are
   inclusive as documented.
6. **`RANDBETWEEN(0, 2^53)` and `RANDBETWEEN(0, 2^53+2)`** — the resolution ceiling, where the
   count of distinct attainable values stops tracking the requested range.
7. **`RANDBETWEEN(TRUE, 3)` and `RANDBETWEEN("1", 3)`** — coercion in the bound slots, which the
   `Variant` typing suggests is permitted and which nothing documents.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `RANDBETWEEN` that an implementer matching Excel, or a reader explaining
an unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's
answer and where it was recorded; "measured" means the statement is read off recorded Excel
answers, "inferred" marks what goes beyond them. Where older text on this page says a behaviour
was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

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
| inclusive bounds | Both `bottom` and `top` are attainable outcomes |
| narrowing inward | Replacing non-integral bounds by `⌈bottom⌉` and `⌊top⌋` |
| modulo bias | The unequal bucket sizes produced by `⌊u·N⌋` when `N` is not a power of two |
| resolution ceiling | The limit on distinct outcomes imposed by the source stream, not by the bounds |

## Sources

- Microsoft, "WorksheetFunction.RandBetween method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.randbetween> (the
  smallest/largest-integer wording and the recalculation statement; the page states no error
  conditions and nothing about non-integral bounds).
- Microsoft Support, "RANDBETWEEN function" —
  <https://support.microsoft.com/en-us/office/randbetween-function-4cc7f0d1-87dc-4eb7-987f-a469ab381685>
  (retrieval refused with HTTP 403 during this pass; nothing here is sourced from it).
- Lemire, "Fast random integer generation in an interval", ACM TOMACS 4(1), 2019; L'Ecuyer &
  Simard, "TestU01", ACM TOMS 33(4), 2007.
- Handbook, [RAND](FUNC.RAND.md), [RANDARRAY](FUNC.RANDARRAY.md),
  [Coercion and lifting](../model/02-coercion-and-lifting.md),
  [About implementation options](../model/07-implementation-options.md).
- Handbook projections `data/functions/FUNC.RANDBETWEEN.json` (`PseudoRandom`, `VolatileFull`,
  `HostSerialized`, `RandomProvider`, `Custom` coercion) and
  `data/presence/FUNC.RANDBETWEEN.json` (own module, no shared surfaces, no defect streams).
