---
schema: efh.function-page/v1
function_id: FUNC.EUROCONVERT
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
family: misc_conversion_family
role_in_family: The legacy currency member — a fixed-rate, add-in-owned conversion with its own rounding regime, distinct from the unit conversions its module neighbours perform.
---

`EUROCONVERT` is a historical artifact with unusually precise semantics. It converts between
the euro and the currencies it replaced, at the **irrevocably fixed** conversion rates set when
each currency joined — not at market rates, which is why the function can be deterministic and
offline at all. It exists to implement the conversion *rules* that came with those rates,
including a mandated rounding regime and a mandated way of routing one legacy currency to
another.

Its status: the reference engine (OxFunc) records it as **deferred**, with the reason
**"Deferred add-in-owned surface."** That reason is specific and worth separating from the
other deferrals in this batch. `EUROCONVERT` is not deferred because it needs a network or a
server — it is pure arithmetic over a fixed table. It is deferred because in Excel the function
does not belong to the core function set at all: it ships in the **Euro Currency Tools**
add-in, and if that add-in is not installed and loaded, the name does not resolve. Microsoft's
category for it is literally "User defined functions that are installed with add-ins".

OxFunc nonetheless carries an implementation module for it (`misc_conversion_family.rs`), a
registry-backed signature, and classification axes marking it `Deterministic`, `SafePure` and
`NonVolatile`. The deferral is about ownership of the surface, not about tractability.

## What it computes

Three conversion directions, with different arithmetic:

1. **National → euro.** Divide by that currency's fixed rate.
2. **Euro → national.** Multiply by that currency's fixed rate.
3. **National → national (triangulation).** Convert the source to euro, round that intermediate
   euro amount to `triangulation_precision` significant digits, then convert the rounded
   intermediate to the target currency. The mandated intermediate step is what "triangulation"
   names, and it is the reason this function cannot be replaced by a single multiplication: a
   direct source-to-target rate would give a different, non-conforming answer.

And one identity case Microsoft states explicitly: **if the source ISO code is the same as the
target ISO code, the original number is returned.**

The rate table covers fourteen ISO codes:

`BEF` `LUF` `DEM` `ESP` `FRF` `IEP` `ITL` `NLG` `ATS` `PTE` `FIM` `GRD` `SIT` `EUR`

The rates themselves are not Excel's invention — they are the fixed conversion rates adopted in
EU law when each currency's euro entry was settled, and they never change. The Handbook does
not transcribe them on this page; the authoritative source is named under Sources, and any
implementation should take them from there rather than from a spreadsheet.

### The rounding regime

`full_precision` selects between two very different results. When it is FALSE (Microsoft
documents FALSE as the default), the result is rounded according to **currency-specific**
rules; when TRUE, all significant digits from the calculation are kept.

Microsoft's documented precision table:

| ISO codes | Calculation precision | Display precision |
|---|---|---|
| `BEF`, `LUF`, `ESP`, `ITL` | 0 | 0 |
| `DEM`, `FRF`, `IEP`, `NLG`, `ATS`, `FIM`, `SIT`, `EUR` | 2 | 2 |
| `PTE`, `GRD` | 0 | 2 |

The third row is the interesting one and is not a typo: for the escudo and the drachma,
calculation precision and display precision differ. Those currencies had subunits that were no
longer in practical use, so amounts were computed to whole units but presented with two
decimals. Any implementation that collapses the two columns into one will be wrong for exactly
two of the fourteen currencies — a defect that is invisible for most test data.

Note also that these are *currency* precisions, not a single function-wide rounding rule. The
same formula rounds differently depending on which target currency you name.

## Arguments

`EUROCONVERT(number, source, target, [full_precision], [triangulation_precision])`

- **`number`** (required) — the currency value to convert, or a reference to a cell containing
  it.
- **`source`** (required) — a three-letter ISO code from the list above, as text or a cell
  reference.
- **`target`** (required) — likewise, the ISO code to convert to.
- **`full_precision`** (required per Microsoft; optional per the reference engine's signature —
  see below) — a logical. FALSE applies the currency-specific rounding above; TRUE keeps all
  significant digits.
- **`triangulation_precision`** (required per Microsoft; optional per the reference engine's
  signature) — an integer **≥ 3**, the number of significant digits used for the intermediate
  euro value when converting between two national currencies. It has no effect when one side of
  the conversion is already the euro.

**A documented divergence, stated plainly.** Microsoft's page labels `full_precision` and
`triangulation_precision` as *Required*. The reference engine's projection records the
signature as `EUROCONVERT(number, source, target, [full_precision],
[triangulation_precision])` with arity 3–5, i.e. both optional, and Microsoft's own prose in
the same page describes FALSE as the *default* for `full_precision` — which only makes sense
if the argument can be omitted. The Handbook cannot resolve this from documentation and has not
observed Excel. Both readings are recorded here; the Handbook prefers neither. A three-argument
call is the first probe listed below.

## Result and edge cases

A number, with two documented behaviours that are easy to miss:

- **"Excel truncates any trailing zeros in the return value."** So the *value* is rounded to
  the currency's precision but the *representation* does not preserve trailing zeros — a
  conversion to a 2-decimal currency landing on a round amount does not come back as `12.50`.
- **"This function does not apply a number format."** The result is a bare number; currency
  formatting is the workbook's job. That separation is consistent with the Handbook's model, in
  which function pages describe values and not cell presentation (see
  [the call pipeline](../model/03-call-pipeline.md)).

And one structural restriction with no analogue elsewhere in this batch: **"This function
cannot be used in array formulas."** Almost every scalar function in Excel lifts over arrays;
this one is documented as refusing to. That makes `EUROCONVERT` an explicit exception to the
array-lifting baseline described in
[coercion and lifting](../model/02-coercion-and-lifting.md), and the manner of the refusal —
error, first-element-only, or entry-time rejection — is not stated.

## Errors

As documented by Microsoft:

- **`#NAME?`** — the Euro Currency Tools add-in is not installed and loaded. This is the
  characteristic failure of the whole add-in-owned category: the workbook is fine, the formula
  is fine, and the name simply does not exist in this session.
- **`#VALUE!`** — "Invalid parameters return #VALUE." Microsoft gives no finer breakdown, so an
  unrecognized ISO code, a non-numeric `number`, and a `triangulation_precision` below 3 are
  all documented as landing on the same code.

## Relationships

- **`CONVERT`** — the function readers reach for and the one that does *not* do currencies.
  `CONVERT` handles physical units (mass, distance, time, temperature). The two are neighbours
  in intent and disjoint in domain.
- **`CALL` and `REGISTER.ID`** — the other entries in the same "installed with add-ins"
  category, though they are a registration seam rather than a computation. What `EUROCONVERT`
  shares with them is the `#NAME?`-when-absent behaviour.
- **`STOCKHISTORY`** and the Currencies linked data type — the modern, *market-rate* path for
  currency conversion. They are not replacements for `EUROCONVERT` and must not be treated as
  such: `EUROCONVERT` implements a legal conversion at a frozen rate, not a market quote. Using
  a live rate to convert a legacy amount is a different and usually wrong operation.
- **`ROUND`** — worth naming because the temptation is to reimplement `EUROCONVERT` as a
  multiplication plus a `ROUND`. That reproduces neither the two-column precision table nor the
  triangulation intermediate.

## Notes for implementers

- **Triangulation is not associative with plain rounding.** Convert FRF → DEM directly by a
  composed rate and you get a different answer from the mandated route through a
  significant-digit-rounded euro intermediate. The intermediate rounding is the specification,
  not an implementation detail.
- **Significant digits, not decimal places.** `triangulation_precision` is documented as
  significant digits with a floor of 3. Implementing it as decimal places will agree on some
  magnitudes and diverge on others, which is the worst kind of bug.
- **Calculation precision and display precision are two columns.** See `PTE` and `GRD` above.
- **The rate table is data with legal provenance.** It should be sourced from the regulations,
  carry the accession dates for the later entrants (`GRD`, `SIT`), and never be edited.
- **The array-formula restriction is a real behavioural axis.** A modern reimplementation will
  naturally lift over arrays and will thereby accept formulas Excel refuses. Whether that
  matters depends on whether the goal is compatibility or utility — the Handbook's four-flavour
  framing is exactly the place to make that choice explicit (see
  [implementation options](../model/07-implementation-options.md)).

## What has not been checked

No Handbook vector suite exists for `EUROCONVERT`, and no Excel-comparison evidence is
recorded. Nobody has checked this function against Excel in the Handbook's record. OxFunc has
an implementation module for it, but the Handbook publishes no measurement of how that
implementation compares with Excel, and this page makes no claim about agreement in either
direction.

`EUROCONVERT` is one of the more rewarding suite targets in this batch: it is pure, offline,
and has a small closed input domain (fourteen codes, two directions, one flag, one integer), so
a suite could be close to exhaustive. It needs an Excel with the Euro Currency Tools add-in
loaded, which is an availability condition rather than an oracle problem. The probes, in order:

1. **Argument optionality.** A three-argument call, and a four-argument call, on a real Excel.
   This settles the documented/projected divergence recorded above, and it is the only open
   question on this page about the function's *shape* rather than its arithmetic.
2. **The `PTE` and `GRD` precision split.** Conversions into and out of both currencies with
   `full_precision` FALSE, at amounts whose 0-precision and 2-precision results differ. This
   is the behaviour most likely to be wrong in any reimplementation.
3. **Triangulation precision as significant digits.** The same FRF → DEM conversion at
   `triangulation_precision` 3, 6 and 15, across amounts spanning several orders of magnitude,
   which distinguishes significant-digit rounding from decimal-place rounding.
4. **`triangulation_precision` below 3.** The documented floor — error, clamp, or ignore?
5. **Identity conversion.** `source` equal to `target`, with `full_precision` FALSE, to confirm
   the documented "returns the original value" and in particular whether it bypasses rounding.
6. **The array-formula refusal.** The function applied to a multi-cell range, entered normally
   and as an array formula, to record what "cannot be used" produces.
7. **Rounding mode at the halfway point.** Amounts that land exactly on `.005` for a 2-decimal
   currency, to determine half-up versus half-even. Nothing in the documentation states this,
   and it changes results on real money.
8. **Add-in absence.** The same formulas with the add-in unloaded, to confirm `#NAME?` and to
   see whether a saved workbook's cached values survive.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| reference engine: deferred | OxFunc intentionally defers the function; the recorded reason is "Deferred add-in-owned surface." |
| add-in-owned surface | The name exists only when a specific Excel add-in is installed and loaded |
| irrevocable rate | The fixed euro conversion rate adopted in law for a currency; never revised |
| triangulation | Legacy-to-legacy conversion routed through a rounded euro intermediate |
| calculation precision / display precision | The two separate rounding columns in the documented currency table |
| significant digits | The unit of `triangulation_precision`; not decimal places |

## Sources

- Microsoft, "EUROCONVERT function" —
  <https://support.microsoft.com/en-us/office/euroconvert-function-79c8fd67-c665-450c-bb6c-15fc92f8345c>
  (syntax, all five arguments and their stated requiredness, the fourteen supported ISO codes,
  the calculation/display precision table, the identity rule, the trailing-zero and
  no-number-format remarks, the array-formula restriction, and the `#NAME?` and `#VALUE!`
  conditions).
- Council Regulation (EC) No 2866/98 of 31 December 1998 on the conversion rates between the
  euro and the currencies of the Member States adopting the euro, together with the later
  regulations covering the currencies that joined afterwards — the authoritative source of the
  fixed rates this function applies. Named here as where an implementation should obtain the
  table; the Handbook does not transcribe it on this page.
- Handbook `data/functions/FUNC.EUROCONVERT.json` — admission label and reason, category "User
  defined functions that are installed with add-ins", arity 3–5, the signature with
  `full_precision` and `triangulation_precision` bracketed as optional, and the classification
  axes quoted above.
- Handbook `data/presence/FUNC.EUROCONVERT.json` — the implementing module
  `crates/oxfunc_core/src/functions/misc_conversion_family.rs` and its surface-dispatch entry.
- Handbook `content/model/02-coercion-and-lifting.md`, `03-call-pipeline.md` and
  `07-implementation-options.md` — the array-lifting baseline this function is documented to
  refuse, the value-versus-presentation split, and the flavour framing referenced above.
