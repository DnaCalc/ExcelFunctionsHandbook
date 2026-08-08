---
schema: efh.function-page/v1
function_id: FUNC.STOCKHISTORY
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
family: null
role_in_family: null
---

`STOCKHISTORY` retrieves a time series of market data for a financial instrument and spills it
as a table. It is the first Excel worksheet function whose result is a **licensed data feed** —
its answer is not a computation at all but a purchase, and Microsoft documents a subscription
requirement to match.

The reference engine (OxFunc) records it as **deferred**, with the reason **"Deferred
external/provider function."** There is no implementation module and no live registry entry —
the projection is catalog-only. The deferral is as structural as a deferral gets: the values
belong to a data provider, are licensed, and change with the market.

## What it computes

Given an instrument, a date range, a sampling interval and a choice of columns,
`STOCKHISTORY` returns a rectangular array: one row per period in the range, one column per
requested property, optionally preceded by header rows.

Three things about that description carry more weight than they appear to.

**The result is an ordinary array, not a rich value.** Unlike the Stocks *data type* — which
produces a linked rich value with a `#FIELD!`-addressable payload — `STOCKHISTORY` spills plain
numbers and dates. That makes it far easier to compute with, and it is why the function exists
alongside the data type rather than being subsumed by it.

**The date you ask for is not necessarily the date you get.** Microsoft states that if
`interval` is not daily, the first data point may precede the supplied `start_date`. The
function samples periods, not days: a weekly interval returns week-aligned rows, so a start
date mid-week yields a row anchored to that week's start. Any workbook that assumes row 1
corresponds to `start_date` is relying on something the documentation explicitly denies.

**Column order follows your request, not the table above.** The `property` arguments are
positional: whatever you list first is the first column. Getting `Date` in the output at all
requires asking for property `0`.

The properties available:

| Value | Column |
|---|---|
| 0 | Date |
| 1 | Close |
| 2 | Open |
| 3 | High |
| 4 | Low |
| 5 | Volume |

That list is the whole vocabulary. There is no adjusted close, no dividend, no split factor —
so a series spanning a corporate action cannot be reconciled from what this function returns.
Whether the returned prices are themselves adjusted is not stated in the documentation, and the
Handbook does not know.

## Arguments

`STOCKHISTORY(stock, start_date, [end_date], [interval], [headers], [property0], …, [property5])`

- **`stock`** (required) — a ticker symbol in double quotes, or a reference to a cell holding a
  Stocks data type value. Microsoft documents an optional disambiguating form: a four-character
  ISO market identifier code, a colon, then the ticker (`"XNAS:MSFT"`). A bare ticker is
  ambiguous across exchanges, so the qualified form is the one to prefer in anything
  reproducible.
- **`start_date`** (required) — the earliest date for which data is retrieved. See the caveat
  above about non-daily intervals.
- **`end_date`** (optional) — the latest date; **defaults to `start_date`**. That default is
  worth stating loudly: omitting `end_date` does not mean "up to today", it means a single
  period. This is the argument position most often misread.
- **`interval`** (optional) — 0 daily, 1 weekly, 2 monthly; default 0.
- **`headers`** (optional) — 0 no headers, 1 show headers, 2 show the instrument identifier and
  headers; default 1. Note that the default *includes* a header row, so the spilled range is
  one row taller than the data unless you say otherwise — a common off-by-one in formulas that
  index into the result.
- **`property0` … `property5`** (optional) — the columns to retrieve, from the table above, in
  the order given.

## Result and edge cases

A spilled array. Two documented behaviours shape how it must be consumed:

- **"The STOCKHISTORY function does not stamp a format on the cells that it spills into."** So
  the Date column arrives as serial numbers and displays as numbers unless the cells are
  already date-formatted. This is a deliberate departure from functions like `TODAY`, which the
  call-pipeline chapter describes as carrying a presentation hint (see
  [the call pipeline](../model/03-call-pipeline.md)); `STOCKHISTORY` publishes plain values and
  leaves presentation entirely to the workbook.
- **Availability of the instrument is not availability of its history.** Microsoft notes that
  some instruments available as Stocks data types have no historical information. Being able to
  look something up does not mean you can chart it.

Two further edges follow from the shape being data-dependent: the number of rows depends on how
many trading periods the provider has, and the spilled range therefore changes size when the
range or the market calendar changes. Anything downstream must handle a resizing spill — that
is a `#SPILL!` risk on every recalculation, not just at entry.

Microsoft also notes that historical data generally updates only after trading days complete,
so a range extending to today is not a live quote.

## Errors

Microsoft's page as fetched does not present a tabulated error list for `STOCKHISTORY`. What it
does state, and what functions as the hardest gate, is a **licensing requirement**: the
function "requires a Microsoft 365 Personal, Microsoft 365 Family, Microsoft 365 Business
Standard, or Microsoft 365 Business Premium subscription."

That is a category of precondition no other function in this Handbook carries. A formula can be
perfectly written, on a supported build, with a live network, and still not work because of the
licence attached to the signed-in account. The Handbook does not know which error value an
unlicensed account produces, and will not guess; `#BUSY!`, `#CONNECT!` and `#BLOCKED!` all
exist in the value-universe registry and any of them is plausible.

Likewise undocumented, and listed below as probes: an unknown ticker, a date range with no
trading days, and `start_date` later than `end_date`.

## Relationships

- **Stocks and Currencies linked data types** — the same provider, a different return shape.
  The data type gives you a rich value with fields; `STOCKHISTORY` gives you an array of
  numbers. Use the data type for "what is the current price", this function for "what was the
  price every day last year".
- **`FIELDVALUE`** — extracts a field from a linked data type value, and is the function
  readers reach for when they actually want history. It cannot do that.
- **`IMAGE`** — the other modern provider-dependent function, and a useful contrast:
  `IMAGE` produces a rich value, `STOCKHISTORY` produces a plain array. Two different answers
  to "what should a provider-backed function return".
- **`WEBSERVICE`** — the do-it-yourself alternative, still available on Windows Excel, with no
  licensing gate and no provider. Also with no data quality guarantees, a 32,767-character
  ceiling, and manual parsing.
- **`EUROCONVERT`** — worth naming as a contrast in *kind*. `EUROCONVERT` converts currency at
  legally fixed rates and is deterministic forever; anything involving market rates, including
  currency history from this function, is not.

## Notes for implementers

- There is no kernel. Everything interesting is provider protocol, licence checking, market
  calendar handling and array shaping.
- The period-alignment rule (the first row may precede `start_date`) is real semantics and must
  be reproduced, not smoothed over. It is where a naive implementation will silently differ.
- The absence of a format stamp is a deliberate publication choice and should be preserved: an
  implementation that returns formatted dates is doing something the documented function
  explicitly does not.
- Result shape depends on data, so an implementation must handle the spill-resize case, and any
  consumer must be written against a variable-height range.
- The licensing gate is not something a reference implementation can or should reproduce. It
  should be named as a host precondition, which is precisely the reason for the deferral.

## What has not been checked

No Handbook vector suite exists for `STOCKHISTORY`, and no Excel-comparison evidence is
recorded. Nobody has checked this function against Excel in the Handbook's record. Every
behavioural statement above is Microsoft's documented behaviour, cited as such, or explicitly
flagged as unknown.

A value-comparison suite is impossible in principle here: the returned prices are a licensed
third-party dataset that changes, so there is no fixed correct answer to compare against. What
*is* characterizable is the **shape and argument surface**, which is independent of the values,
and which is where every real workbook bug lives anyway. The probes, in order:

1. **Result shape as a function of arguments.** For each combination of `headers` ∈ {0, 1, 2},
   `interval` ∈ {0, 1, 2} and property-count 1 through 6, record only the spilled range's
   dimensions — not its values. This is fully reproducible and settles the off-by-one questions
   that dominate real use.
2. **Period alignment.** A `start_date` mid-week and mid-month at weekly and monthly intervals,
   recording only the first returned date relative to the requested one.
3. **`end_date` omitted.** Confirming the documented single-period default rather than
   "through today".
4. **Degenerate ranges.** `start_date` after `end_date`; a range containing only weekend and
   holiday dates; a range entirely in the future.
5. **Unknown and ambiguous tickers.** A nonsense symbol, a valid symbol on two exchanges given
   bare, and the same given with an ISO market identifier prefix.
6. **The licence gate.** The same formula on an account without a qualifying subscription, to
   record which error value appears — the single most useful undocumented fact about this
   function.
7. **Repeated properties and out-of-range values.** Property `1` requested twice; property `6`;
   a fractional property value.
8. **Recalculation and staleness.** Whether the function refetches on recalculation, on file
   open, or on a timer, and what a dependent formula sees while a fetch is outstanding
   (`#BUSY!` is the candidate).

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| reference engine: deferred | OxFunc intentionally defers the function; the recorded reason is "Deferred external/provider function." |
| catalog-only | The projection carries identity and documentation metadata but no implementation module or live registry entry |
| linked data type | A rich value backed by a provider, with an addressable field payload |
| ISO market identifier code | The four-character exchange prefix that disambiguates a ticker |
| period alignment | The rule by which non-daily intervals may return a first row earlier than `start_date` |
| spill | The host-side adaptation that lays an array result across neighbouring cells |

## Sources

- Microsoft, "STOCKHISTORY function" —
  <https://support.microsoft.com/en-us/office/stockhistory-function-1ac8b5b3-5f62-4d94-8ab8-7504ec7239a8>
  (syntax, all arguments and their defaults, the ISO market identifier form, the `interval`,
  `headers` and property tables, the licensing requirement quoted above, the no-format-stamp
  remark, the period-alignment caveat, and the note that some instruments have no historical
  data).
- Handbook `data/functions/FUNC.STOCKHISTORY.json` — admission label and reason, category
  "Information functions", and the catalog-only metadata status.
- Handbook `data/presence/FUNC.STOCKHISTORY.json` — no implementation module, no dispatch entry.
- Handbook `content/model/01-value-universe.md` and `03-call-pipeline.md` — the extended
  error-code registry (`#BUSY!`, `#CONNECT!`, `#BLOCKED!`, `#SPILL!`) and the presentation-hint
  behaviour this function is documented not to have.
