---
schema: efh.function-page/v1
function_id: FUNC.RANDARRAY
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — RANDARRAY function"
    locator: "https://support.microsoft.com/en-us/office/randarray-function-21261e55-3bec-4885-86a6-8b0a47fd4d33"
    role: "the only documentation page for this surface; retrieval refused during this pass, so it is cited as a locator and not quoted"
  - work: "Microsoft — Excel JavaScript API, Excel.Functions"
    locator: "https://learn.microsoft.com/en-us/javascript/api/excel/excel.functions"
    role: "checked for a randArray entry; the API surface documented there does not include one"
  - work: "L'Ecuyer & Simard, TestU01: A C library for empirical testing of random number generators"
    locator: "ACM TOMS 33(4), 2007"
    role: "the standard battery for the stream this surface draws from"
  - work: "Lemire, Fast random integer generation in an interval"
    locator: "ACM TOMEACS 4(1), 2019"
    role: "the bias analysis for the whole-number mode's floor(u*count) construction"
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
family: misc_conversion_family
role_in_family: "The odd one out: a volatile array generator co-located in a module of deterministic conversion surfaces, sharing only the module and not the semantics."
---

# RANDARRAY

## What it computes

`RANDARRAY([rows], [columns], [min], [max], [whole_number])` returns a grid of pseudo-random
numbers.

It is the dynamic-array generalisation of the two older random surfaces, and it subsumes both:

    RANDARRAY()                      ≡  RAND()
    RANDARRAY(1, 1, a, b, TRUE)      ≡  RANDBETWEEN(a, b)

Two sampling modes, and the distinction is the whole function:

**Continuous mode** (`whole_number` false, the default) draws from the interval between `min` and
`max`. The reference engine forms `min + u·(max − min)` from a unit draw `u ∈ [0,1)`, which makes
the interval half-open at the top: `max` is a supremum, not a maximum.

**Whole-number mode** (`whole_number` true) draws uniformly from the **integers** in the range.
The reference engine takes `lo = ⌈min⌉`, `hi = ⌊max⌋`, and returns `lo + ⌊u·(hi−lo+1)⌋` — so this
mode is inclusive at both ends, exactly the opposite endpoint convention from the continuous mode
sitting in the same argument list. A function whose upper bound is attainable or not depending on
a Boolean flag is a genuine usability hazard, and it is worth stating before anyone builds a
sampler on it.

Each cell is an independent draw from the host's stream; the array has no correlation structure by
construction. Everything about the underlying generator is discussed on [RAND](FUNC.RAND.md) and is
not repeated here.

## Arguments

All five arguments are optional. The projection records an arity of 0 to 5 and a `Custom`
coercion profile; the signature projection carries no parameter list, so the argument order below
is taken from the reference engine's own reading of the surface, not from a documentation quote.

| Argument | Meaning | Default in the reference engine |
|---|---|---|
| `rows` | Number of rows to fill | 1 |
| `columns` | Number of columns to fill | 1 |
| `min` | Lower bound of the draw | 0 |
| `max` | Upper bound of the draw | 1 |
| `whole_number` | Integers if true, decimals if false | FALSE |

`rows` and `columns` are **truncated toward zero** in the reference engine rather than rejected
when non-integral, and a value below 1 after truncation is refused. `whole_number` accepts a
logical directly and otherwise coerces through to-number with the usual nonzero-is-true rule.

**A note on sourcing.** Microsoft's documentation for this surface exists only as the support page
named in the projection, and the Handbook's retrieval of that page was refused with HTTP 403
during this pass; the Excel JavaScript API reference, which does document `rand` and `randBetween`,
carries no `randArray` entry. **Nothing on this page is quoted from Microsoft.** The argument
meanings above are consistent with the function's own English description in the projection
("you can specify the number of rows and columns to fill, minimum and maximum values, and whether
to return whole numbers or decimal values") and with the reference engine; the defaults and the
truncation rule are reference-engine facts.

## Result and edge cases

Returns an `Array` — a spilled dynamic array on the worksheet.

- **Volatility.** `VolatileFull`, `PseudoRandom`, `HostSerialized`, with a `RandomProvider`
  dependency: the entire grid is redrawn on every recalculation. A `RANDARRAY` spill is not a
  stable range and must not be used as a lookup source, a sort key, or anything else that is read
  twice.
- **`min = max`** yields that value in every cell in continuous mode, and a single-valued draw in
  whole-number mode.
- **`min > max`** is refused in both modes in the reference engine.
- **Non-integral dimensions** are truncated, as above.
- **Grid limits.** The reference engine enforces no worksheet grid bound; on the worksheet a
  request larger than the available space spills into `#SPILL!` territory, which is an engine
  behaviour rather than a function behaviour and is not modelled here.

**A divergence between two siblings in the same engine.** [SEQUENCE](FUNC.SEQUENCE.md) — the other
dimension-taking generator, in the same category, written in the same reference engine — *rejects*
a non-integral `rows` with an error where `RANDARRAY` truncates it. Two functions that take the
same argument in the same position and disagree about what a decimal means is precisely the sort of
inconsistency this Handbook exists to surface. Which one matches Excel is unknown here; the pair
`RANDARRAY(2.7,1)` and `SEQUENCE(2.7)` decides it in two cells.

## Errors

No documented error condition is available to this page. The reference engine's conditions:

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | `rows` or `columns` truncates to less than 1 | reference engine |
| `#VALUE!` | `min` or `max` is not finite | reference engine |
| `#VALUE!` | An argument does not convert to a number | shared coercion model |
| `#NUM!` | `min` exceeds `max` (either mode) | reference engine |
| `#NUM!` | The requested cell count overflows the engine's own size arithmetic | reference engine |
| propagated | An error value in any argument | shared coercion model |

Every row above is a reference-engine statement. The Handbook has not observed Excel's error codes
for any of these cases, and has not been able to read what Microsoft documents.

## Relationships

- **[RAND](FUNC.RAND.md)** — the scalar continuous draw, and the same stream. `RANDARRAY()` with
  no arguments is the same thing.
- **[RANDBETWEEN](FUNC.RANDBETWEEN.md)** — the scalar integer draw. `RANDARRAY`'s whole-number
  mode reproduces it, including the inclusive-both-ends convention and the same
  `⌈min⌉`/`⌊max⌋` normalisation of non-integral bounds.
- **[SEQUENCE](FUNC.SEQUENCE.md)** — the deterministic twin: same shape arguments, no randomness.
  The two are usually learned together and, as noted above, do not agree on non-integral
  dimensions in the reference engine.
- **`SORTBY(range, RANDARRAY(ROWS(range)))`** — the idiomatic shuffle. It is a genuine random
  permutation only if the draws are distinct; at the resolution discussed on
  [RAND](FUNC.RAND.md), ties are reachable in large ranges and a tie makes the "shuffle" preserve
  the original relative order of the tied rows.
- **The module it lives in.** The presence projection places this surface in a module shared with
  `BAHTTEXT`, `CONVERT`, `EUROCONVERT` and `PERCENTOF` — deterministic conversion functions with
  nothing semantically in common with it. That is a co-location fact about the reference engine's
  source layout, not a family relationship, and the `family` field on this page should be read that
  way.

## Numerical notes

**The whole-number mode has a bias, and it is quantifiable.** `lo + ⌊u·N⌋` with `N = hi−lo+1` maps
a lattice of `2⁵³` unit values onto `N` buckets. Unless `N` divides `2⁵³` — that is, unless `N` is
a power of two — the buckets receive unequal numbers of lattice points, and the resulting
distribution is uniform only to within a relative error of about `N/2⁵³`. For spreadsheet-sized
`N` that is far below any detectable level; for `N` near `2⁵³` it is total. The clean remedies are
Lemire's multiply-and-reject method or plain rejection sampling, both of which cost an occasional
extra draw and remove the bias exactly. An implementation targeting compatibility must not apply
them, because rejection changes *which* draw lands in which cell.

**The continuous mode loses resolution asymmetrically.** `min + u·(max − min)` has absolute
spacing `(max−min)·2⁻⁵³` everywhere, so when `min` is large and the interval is small, the result
inherits `min`'s exponent and the draw's low bits are rounded away — in the extreme, every cell
returns `min`. The subtraction `max − min` can also overflow when the bounds straddle zero at
large magnitude. A more careful construction interpolates as `min·(1−u) + max·u`, which is
monotone in `u`, cannot overflow for finite bounds of the same sign, and returns exactly `min` at
`u = 0`.

**Grid-scale draws expose stream quality.** A single `RAND()` cell reveals nothing; a
`RANDARRAY(1000,1000)` is a million draws in one recalculation, which is well past the birthday
bound for a 32-bit-resolution generator and well within reach of the serial-correlation tests
discussed on [RAND](FUNC.RAND.md). If any Excel-generator anomaly is visible from the worksheet,
this is the surface that shows it.

## What has not been checked

No Handbook vector suite exists for `RANDARRAY`, and no evidence record in
`content/evidence/records/` lists this surface among its subjects. The presence projection records
no upstream defect stream touching this module. The probe battery rendered beside this page reports
every row as not dispatchable, because the surface is declared non-deterministic — so the usual
mechanical evidence is unavailable here by construction.

Additionally, and unusually for this Handbook: **the Handbook was unable to read Microsoft's
documentation for this function during this pass.** The support page returned HTTP 403 and no
alternative Microsoft reference for `RANDARRAY` was found. Every argument default, every error
code and every endpoint convention on this page is therefore a reference-engine statement awaiting
both an Excel probe and a documentation read.

Inputs I would probe first:

1. **`RANDARRAY(2.7, 1)` against `SEQUENCE(2.7)`** — the truncate-versus-reject divergence
   described above, in two cells.
2. **`RANDARRAY(1, 20, 1, 3, TRUE)`** — whether both `1` and `3` appear, which settles the
   inclusive-both-ends reading of whole-number mode.
3. **`RANDARRAY(1, 20, 0, 1, FALSE)` compared against `RAND()`** — whether `1` is ever attained in
   continuous mode, which settles the half-open reading.
4. **`RANDARRAY(1, 1, 5, 4)` and `RANDARRAY(1, 1, 5, 4, TRUE)`** — the `min > max` error code in
   each mode, which the reference engine gives as `#NUM!` and Microsoft may document differently.
5. **`RANDARRAY(0)` and `RANDARRAY(-1)`** — the zero and negative dimensions, where
   [SEQUENCE](FUNC.SEQUENCE.md) is recorded as splitting `#CALC!` from `#VALUE!`; whether
   `RANDARRAY` splits them the same way is unknown.
6. **`RANDARRAY(1, 1, 1E308, 1.0000001E308)`** — the overflow-in-the-subtraction case, which
   distinguishes the naive interpolation from the monotone one.
7. **A large grid subjected to the resolution and serial-correlation tests** listed on
   [RAND](FUNC.RAND.md), since this surface is the cheapest way to obtain a large sample.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| continuous mode | `whole_number` false: draws from the interval, upper bound not attained |
| whole-number mode | `whole_number` true: draws integers, both bounds attained |
| spill | The worksheet behaviour of writing an array result across neighbouring cells |
| modulo bias | The non-uniformity of `⌊u·N⌋` when `N` does not divide the lattice size |

## Sources

- Microsoft Support, "RANDARRAY function" —
  <https://support.microsoft.com/en-us/office/randarray-function-21261e55-3bec-4885-86a6-8b0a47fd4d33>
  (named in `data/functions/FUNC.RANDARRAY.json`; retrieval refused with HTTP 403 during this pass,
  so nothing on this page is quoted from it).
- Microsoft, Excel JavaScript API reference —
  <https://learn.microsoft.com/en-us/javascript/api/excel/excel.functions> (checked for a
  `randArray` entry; the documented surface there does not include one).
- L'Ecuyer & Simard, "TestU01", ACM TOMS 33(4), 2007; Lemire, "Fast random integer generation in
  an interval", ACM TOMACS 4(1), 2019 — the bias analysis for the whole-number construction.
- Handbook, [RAND](FUNC.RAND.md) and [SEQUENCE](FUNC.SEQUENCE.md) — the scalar generator and the
  deterministic twin.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md),
  [Claim language and honesty](../model/06-claim-language.md).
- Handbook projections `data/functions/FUNC.RANDARRAY.json` (arity 0–5, `PseudoRandom`,
  `VolatileFull`, `HostSerialized`, `RandomProvider`, no signature projection) and
  `data/presence/FUNC.RANDARRAY.json` (module shared with `BAHTTEXT`, `CONVERT`, `EUROCONVERT`,
  `PERCENTOF`; no defect streams).
