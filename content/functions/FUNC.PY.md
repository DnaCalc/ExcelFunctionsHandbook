---
schema: efh.function-page/v1
function_id: FUNC.PY
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "PY function"
    locator: "https://support.microsoft.com/en-us/excel/functions/py-function"
    role: "public source read for this page"
  - work: "Troubleshoot Python in Excel errors"
    locator: "https://support.microsoft.com/en-us/office/troubleshoot-python-in-excel-errors-7736520d-47ef-43a8-b640-d826afb63249"
    role: "public source read for this page"
  - work: "Data security and Python in Excel"
    locator: "https://support.microsoft.com/en-us/office/data-security-and-python-in-excel-33cc88a4-4a87-485e-9ff9-f35958278327"
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

# PY

## What it does

`PY` runs Python code and puts the result in a cell. The code does not run on your computer: it runs in a
container in Microsoft's cloud, and Excel shows what comes back.

`PY(python_code, return_type)` has two arguments, both required and both fixed at entry.

- **`python_code`** is the program, as static text. It cannot be a cell reference or a calculated value.
- **`return_type`** is the number 0 or 1. 0 returns an Excel value; 1 returns a Python object held in the cell.

Worksheet data reaches the code through a helper, `xl()`, which reads ranges, names, tables and Power Query
connections, with an optional `headers` setting for the first row:

```
=PY("xl(""Table1[#All]"", headers=True)",1)
```

You rarely type this formula. Entering `=PY` and choosing it from AutoComplete opens a Python editor, which
writes the formula into the cell.

## Try it

## Quirks

- **It cannot be nested.** Excel rejects a formula that combines `PY` with other functions. Other cells may
  read its result, but the call stands alone in its cell.
- **The formula is hidden.** It does not appear in the formula bar. `FORMULATEXT` still shows it.
- **Both arguments are frozen.** `PY(A1,0)` and `PY("1+1",B1)` are not accepted, so the code cannot depend on a
  cell except through `xl()`.
- **A result can be an object.** With `return_type` 1 the cell holds a Python object, not a worksheet value.

### When the service is unavailable

Microsoft states that the calculation needs internet access, because it happens in its cloud and not in the
local application. Its troubleshooting article gives these outcomes, in place of a value:

| Error | Meaning given by Microsoft | Suggested action |
|---|---|---|
| `#BUSY!` | Still calculating in the cloud | Reset the runtime if it lasts past 60 seconds |
| `#CONNECT!` | The connection to the cloud needs refreshing | Reset the runtime |
| `#TIMEOUT!` | The formula passed the maximum execution time | Rerun, reset, or raise the Python Formula Timeout option |
| `#BLOCKED!` | Connected services are not available to this account | Check access; device-based subscriptions and shared computer activation are not supported |

Reset is on Formulas > Python, or Ctrl+Alt+Shift+F9. Microsoft's data security article says the container stays
online while the workbook is open or until a timeout. What a saved result shows when a workbook opens offline
is not stated in these sources and has not been measured.

## How to implement

DNA Calc does not run this function yet. It is deferred by policy, because the answer depends on a back end
Microsoft hosts and controls. An implementation needs:

1. **A Python runtime.** A sandboxed interpreter with no access to the host, one per workbook, which keeps its
   state between cells. Cells run in a defined order, so that order has to be reproduced too.
2. **The `xl()` bridge.** It must turn references, tables and connections into the runtime's objects, and convert
   results back. `return_type` 1 needs a cell value that holds a foreign object.
3. **The library set.** Results depend on the versions of the packages the runtime provides.
4. **Asynchronous calculation.** A cell can be busy, so recalculation needs states like `#BUSY!` and a timeout.
5. **Grammar rules.** Refuse non-static arguments and any nesting at entry, as Excel does.

A local runtime would be a different function from Microsoft's: it would need its own name, or a clear statement
that answers can differ.

## Version differences

No Excel answers for this function are recorded here, so no comparison between Excel builds exists. Microsoft's page lists Excel for Microsoft 365 on Windows and Mac. Its troubleshooting
article says iPad, iPhone and Android are not supported.

## Alternatives

- Add-ins that run Python on your own machine, for local control of libraries and data.
- Power Query or worksheet functions, when the calculation does not need Python.

## Sources

- [PY function](https://support.microsoft.com/en-us/excel/functions/py-function)
- [Troubleshoot Python in Excel errors](https://support.microsoft.com/en-us/office/troubleshoot-python-in-excel-errors-7736520d-47ef-43a8-b640-d826afb63249)
- [Data security and Python in Excel](https://support.microsoft.com/en-us/office/data-security-and-python-in-excel-33cc88a4-4a87-485e-9ff9-f35958278327)
