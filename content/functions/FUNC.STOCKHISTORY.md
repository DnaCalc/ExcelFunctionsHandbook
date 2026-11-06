---
schema: efh.function-page/v1
function_id: FUNC.STOCKHISTORY
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "STOCKHISTORY function"
    locator: "https://support.microsoft.com/en-us/excel/functions/stockhistory-function"
    role: "public source read for this page"
  - work: "How to correct a #CONNECT! error"
    locator: "https://support.microsoft.com/en-us/office/how-to-correct-a-connect-error-584f7de9-b238-480c-8e8f-de0ba2c1aac9"
    role: "public source read for this page"
  - work: "About the Stocks financial data sources"
    locator: "https://support.microsoft.com/en-us/excel/about-the-stocks-financial-data-sources?ui=en-us&rs=en-us&ad=us"
    role: "public source read for this page"
  - work: "Microsoft Q&A: resolving the #CONNECT! error"
    locator: "https://learn.microsoft.com/en-us/answers/questions/5861729/how-to-resolve-the-connect-error"
    role: "public source read for this page"
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

# STOCKHISTORY

## What it does

`STOCKHISTORY` fetches historical prices for one instrument and spills them as a table of ordinary numbers and
dates. The values come from Microsoft's data provider, not from the workbook.

`STOCKHISTORY(stock, start_date, [end_date], [interval], [headers], [property0], ..., [property5])`

- **`stock`** is a ticker in double quotes, or a cell holding a Stocks data type. To name an exchange, put its
  four-character market identifier code and a colon first, as in `"XNAS:MSFT"`.
- **`start_date`** is the first date requested.
- **`end_date`** defaults to `start_date`, so leaving it out asks for one period, not for data up to today.
- **`interval`** is 0 daily (default), 1 weekly or 2 monthly.
- **`headers`** is 0 none, 1 headers (default), or 2 instrument identifier and headers.
- **`property0`** to **`property5`** choose the columns, in the order you list them:

| Code | Column |
|---|---|
| 0 | Date |
| 1 | Close |
| 2 | Open |
| 3 | High |
| 4 | Low |
| 5 | Volume |

The default columns are Date and Close.

## Try it

## Quirks

- **The first row can precede `start_date`.** Non-daily intervals report the first day of the period that holds
  the date. Daily and weekly rows use the first valid trading day; monthly rows use the first calendar day.
- **Each column is a period summary.** Close is the last trading day's close, Open the first day's open, High and
  Low the period's extremes, Volume the total.
- **Today is missing until the market closes.** Data generally updates after the trading day ends.
- **Some instruments have no history.** Microsoft notes that many index funds and some other items offered as
  Stocks data types have none.
- **Column order is yours.** Date appears only if requested, or by default.
- **Headers add a row.** The default output is one row taller than the data, which matters to any formula indexing
  into the result.
- **No formatting is applied**, so cells go back to General if the formula is removed.

### When the service is unavailable

Microsoft's function page does not say what happens when the service cannot be reached. Its general article
says `#CONNECT!` means a formula could not reach a resource it needs. A Microsoft Q&A answer, which is not official
documentation, reports that error for this function. With volatile functions such as `TODAY` and automatic
calculation, data refreshes in the background when the workbook opens, and editing continues meanwhile. A
subscription is also needed: Microsoft lists Microsoft 365 Personal, Family, Business Standard and Business Premium.
No failure of this function has been recorded here.

## How to implement

DNA Calc does not run this function yet. It is deferred by policy, because the data comes from a back end Microsoft
hosts and controls. An implementation needs:

1. **A licensed data feed** with the same instruments, exchanges and adjustment rules.
2. **Period logic:** trading calendars per exchange, for the first valid day of each period.
3. **Column assembly:** an array built from `property` order, with `headers` rows.
4. **Asynchronous calculation** with a result that can spill late, and a failure outcome such as `#CONNECT!`.
5. **Argument rules** for tickers, data types, exchange prefixes and date coercion.

Answers would depend on the provider chosen, so another feed could not promise Excel's values.

## Version differences

No Excel answers for this function are recorded here, so no comparison between Excel builds exists. Microsoft's page lists Excel for Microsoft 365 on Windows and Mac. Past prices can also
change at the provider, so an old answer is not a stable reference.

## Alternatives

- The Stocks data type with `FIELDVALUE` for current fields.
- Power Query against a data source you license, for a stored and repeatable history.
- Recording a result as values, when a report has to stay fixed.

## Sources

- [STOCKHISTORY function](https://support.microsoft.com/en-us/excel/functions/stockhistory-function)
- [How to correct a #CONNECT! error](https://support.microsoft.com/en-us/office/how-to-correct-a-connect-error-584f7de9-b238-480c-8e8f-de0ba2c1aac9)
- [About the Stocks financial data sources](https://support.microsoft.com/en-us/excel/about-the-stocks-financial-data-sources?ui=en-us&rs=en-us&ad=us)
- [Microsoft Q&A: resolving the #CONNECT! error](https://learn.microsoft.com/en-us/answers/questions/5861729/how-to-resolve-the-connect-error)
