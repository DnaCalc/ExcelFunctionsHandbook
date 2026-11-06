---
schema: efh.function-page/v1
function_id: FUNC.COPILOT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "COPILOT function"
    locator: "https://support.microsoft.com/en-us/excel/functions/copilot-function"
    role: "public source read for this page"
  - work: "Bring AI to your formulas with the COPILOT function in Excel"
    locator: "https://techcommunity.microsoft.com/blog/microsoft365insiderblog/bring-ai-to-your-formulas-with-the-copilot-function-in-excel/4443487"
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

# COPILOT

## What it does

`COPILOT` sent a prompt and optional cell data to an AI model and returned its reply in the sheet. Microsoft withdrew
it: its page says that from 14 September 2026 the function is no longer available in Excel, and points to Copilot in
Excel, opened from the Copilot icon, instead.

`COPILOT(prompt_part1, [context1], [prompt_part2], [context2], ...)`

- **`prompt_part`** is text describing the task.
- **`context`** is a cell or range that supplies data.

Prompt parts and context arguments alternate, so a formula can place data between pieces of a prompt:

```
=COPILOT("Classify sentiment",B2:B100)
```

## Try it

## Quirks

- **It was never a fixed mapping.** The model behind it could change, so one formula could give different results at
  different times. This handbook's method of comparing recorded answers does not apply to such a function.
- **Rate limits.** Microsoft's announcement gave 100 calls every 10 minutes and up to 300 an hour. One call with a large
  array counted once; filling across cells counted each cell.
- **It was a preview.** It was offered through Microsoft's Frontier and Insider programs.

### When the service is unavailable

Microsoft's page describes the withdrawal itself. Results already calculated stay in workbooks as cached values. Once
such a cell recalculates, it shows `#NAME?`, because Excel no longer knows the function. So a workbook can look intact
until a recalculation reaches it.

## How to implement

DNA Calc does not run this function yet, and it is deferred by policy: the answer depends on a back end Microsoft hosted
and controlled, and has since withdrawn. Reading a file that uses it is still worth doing correctly:

1. **Keep the cached value** on load, as Excel does, and do not recalculate it silently.
2. **Give `#NAME?` on recalculation**, which is Excel's documented outcome since the withdrawal.
3. **Offer the formula text** so a user can replace the call.

A new function with this shape would need an AI service, rate limiting, asynchronous calculation and a result whose
shape can change. It would not be this function.

## Version differences

Microsoft's page lists Excel for Microsoft 365 on Windows and Mac, the web app and Excel for iPhone, before the
withdrawal of 14 September 2026. No Excel answers for this function are recorded here, so no comparison between Excel builds exists.

## Alternatives

- Copilot in Excel, the replacement Microsoft names.
- `TRANSLATE` and `DETECTLANGUAGE` for those two tasks.
- Paste a useful result as values, so it survives recalculation.

## Sources

- [COPILOT function](https://support.microsoft.com/en-us/excel/functions/copilot-function)
- [Bring AI to your formulas with the COPILOT function in Excel](https://techcommunity.microsoft.com/blog/microsoft365insiderblog/bring-ai-to-your-formulas-with-the-copilot-function-in-excel/4443487)
